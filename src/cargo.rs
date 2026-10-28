//! Run Cargo directly and drain both pipes while retaining bounded logs.
use crate::{
    cli::{CargoContext, RunOptions},
    diagnostic::{
        model::*,
        parse::{ParsedLine, parse_line},
    },
};
use std::{
    collections::BTreeSet,
    ffi::OsString,
    io::{self, BufRead, Read, Write},
    process::{Command, Stdio},
};
pub struct Collection {
    pub execution: Execution,
    pub messages: Vec<cargo_metadata::CompilerMessage>,
    build_success: Option<bool>,
}
pub fn load_project(ctx: &CargoContext) -> Result<ProjectInfo, String> {
    let output = Command::new(&ctx.cargo_exe)
        .current_dir(&ctx.cwd)
        .args(["metadata", "--format-version", "1", "--no-deps"])
        .args(&ctx.metadata_args)
        .output()
        .map_err(|e| format!("Cannot run Cargo metadata: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "Cargo metadata failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let m: cargo_metadata::Metadata = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("Invalid Cargo metadata: {e}"))?;
    let packages = m
        .packages
        .into_iter()
        .map(|p| PackageInfo {
            workspace_member: m.workspace_members.contains(&p.id),
            id: p.id.to_string(),
            name: p.name.to_string(),
            version: p.version.to_string(),
            manifest: p.manifest_path.into_std_path_buf(),
        })
        .collect();
    Ok(ProjectInfo {
        workspace_root: m.workspace_root.into_std_path_buf(),
        target_dir: ctx
            .target_dir_override
            .clone()
            .unwrap_or_else(|| m.target_directory.into_std_path_buf()),
        packages,
    })
}
pub fn collect(ctx: &CargoContext, opts: &RunOptions) -> Result<Collection, String> {
    let mut command = Command::new(&ctx.cargo_exe);
    command
        .current_dir(&ctx.cwd)
        .args(["clippy", "--message-format=json"])
        .args(&opts.clippy_args);
    collect_command(&mut command, opts.clippy_args.clone(), true)
}
const LOG_LIMIT: usize = 1024 * 1024;
fn append_bounded(output: &mut Vec<u8>, bytes: &[u8], truncated: &mut bool) {
    let count = bytes.len().min(LOG_LIMIT.saturating_sub(output.len()));
    output.extend_from_slice(&bytes[..count]);
    *truncated |= count < bytes.len();
}
fn collect_command(
    command: &mut Command,
    args: Vec<OsString>,
    relay: bool,
) -> Result<Collection, String> {
    let mut child = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Cannot start Clippy: {e}"))?;
    let mut stderr = child.stderr.take().expect("piped stderr");
    let drain = std::thread::spawn(move || {
        let mut output = vec![];
        let mut truncated = false;
        let mut chunk = [0; 8192];
        let mut error = None;
        loop {
            match stderr.read(&mut chunk) {
                Ok(0) => break,
                Ok(n) => {
                    if relay {
                        let _ = io::stderr().write_all(&chunk[..n]);
                    }
                    append_bounded(&mut output, &chunk[..n], &mut truncated);
                }
                Err(e) => {
                    error = Some(e.to_string());
                    break;
                }
            }
        }
        (
            String::from_utf8_lossy(&output).into_owned(),
            truncated,
            error,
        )
    });
    let mut result = read_stream(io::BufReader::new(
        child.stdout.take().expect("piped stdout"),
    ));
    if result
        .execution
        .issues
        .iter()
        .any(|s| s.starts_with("Read stdout:"))
    {
        let _ = child.kill();
    }
    let status = child
        .wait()
        .map_err(|e| format!("Cannot wait for Clippy: {e}"))?;
    let (stderr, truncated, error) = drain.join().map_err(|_| "stderr reader panicked")?;
    result.execution.stderr = stderr;
    result.execution.stderr_truncated = truncated;
    result.execution.code = status.code();
    if result
        .build_success
        .is_some_and(|success| success != status.success())
    {
        result.execution.complete = false;
        result
            .execution
            .issues
            .push("Cargo completion message disagrees with process exit status".into());
    }
    result.execution.requested_args = args;
    if let Some(e) = error {
        result.execution.complete = false;
        result.execution.issues.push(format!("Read stderr: {e}"));
    }
    Ok(result)
}
fn read_stream(mut input: impl BufRead) -> Collection {
    let mut result = Collection {
        execution: Execution {
            code: None,
            complete: false,
            requested_args: vec![],
            observed_packages: vec![],
            observed_targets: vec![],
            stderr: String::new(),
            stderr_truncated: false,
            text_stdout: String::new(),
            stdout_truncated: false,
            issues: vec![],
        },
        messages: vec![],
        build_success: None,
    };
    let mut finished = false;
    let mut packages = BTreeSet::new();
    let mut targets = BTreeSet::new();
    let mut logs = vec![];
    let mut bytes = vec![];
    loop {
        bytes.clear();
        match input.read_until(b'\n', &mut bytes) {
            Ok(0) => break,
            Err(e) => {
                result.execution.issues.push(format!("Read stdout: {e}"));
                break;
            }
            _ => {}
        }
        let line = match std::str::from_utf8(&bytes) {
            Ok(s) => s,
            Err(e) => {
                result
                    .execution
                    .issues
                    .push(format!("Invalid UTF-8 output: {e}"));
                continue;
            }
        };
        match parse_line(line) {
            ParsedLine::Message(message) => match *message {
                cargo_metadata::Message::CompilerMessage(m) => {
                    packages.insert(m.package_id.to_string());
                    targets.insert(TargetInfo {
                        name: m.target.name.clone(),
                        kinds: m.target.kind.iter().map(|k| k.to_string()).collect(),
                        source: m.target.src_path.clone().into_std_path_buf(),
                    });
                    result.messages.push(m);
                }
                cargo_metadata::Message::CompilerArtifact(a) => {
                    packages.insert(a.package_id.to_string());
                    targets.insert(TargetInfo {
                        name: a.target.name.clone(),
                        kinds: a.target.kind.iter().map(|k| k.to_string()).collect(),
                        source: a.target.src_path.into_std_path_buf(),
                    });
                }
                cargo_metadata::Message::BuildFinished(f) => {
                    finished = true;
                    result.build_success = Some(f.success);
                }
                _ => {}
            },
            ParsedLine::Malformed(e) => result
                .execution
                .issues
                .push(format!("Damaged Cargo JSON: {e}")),
            ParsedLine::Text(s) => {
                append_bounded(
                    &mut logs,
                    s.as_bytes(),
                    &mut result.execution.stdout_truncated,
                );
                append_bounded(&mut logs, b"\n", &mut result.execution.stdout_truncated);
            }
            ParsedLine::Extension => {}
        }
    }
    if !finished {
        result
            .execution
            .issues
            .push("No build-finished message was received".into());
    }
    result.execution.complete = finished && result.execution.issues.is_empty();
    result.execution.observed_packages = packages.into_iter().collect();
    result.execution.observed_targets = targets.into_iter().collect();
    result.execution.text_stdout = String::from_utf8_lossy(&logs).into_owned();
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parsing_damage_marks_incomplete() {
        let c = read_stream(io::Cursor::new(
            "{\"reason\":\n{\"reason\":\"build-finished\",\"success\":true}\n",
        ));
        assert!(!c.execution.complete);
        assert_eq!(c.execution.issues.len(), 1);
    }
    #[test]
    fn missing_build_finished_is_not_clean() {
        assert!(!read_stream(io::Cursor::new("log\n")).execution.complete);
    }
    #[test]
    fn clean_cached_artifacts_are_observed() {
        let mut v = crate::diagnostic::sample();
        v["reason"] = "compiler-artifact".into();
        v["profile"] = serde_json::json!({"opt_level":"0","debuginfo":0,"debug_assertions":true,"overflow_checks":true,"test":false});
        v["features"] = serde_json::json!([]);
        v["filenames"] = serde_json::json!([]);
        v["executable"] = serde_json::Value::Null;
        v["fresh"] = true.into();
        let c = read_stream(io::Cursor::new(format!(
            "{v}\n{{\"reason\":\"build-finished\",\"success\":true}}\n"
        )));
        assert!(c.execution.complete);
        assert_eq!(c.execution.observed_targets.len(), 1);
    }
    #[test]
    fn process_pipes_are_drained_and_failures_preserved() {
        let helper = std::env::current_exe()
            .unwrap()
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .join("examples")
            .join(format!("cargo_probe{}", std::env::consts::EXE_SUFFIX));
        assert!(helper.is_file(), "Build cargo_probe before running tests");
        let (send, recv) = std::sync::mpsc::channel();
        std::thread::spawn(move || {
            let mut result = vec![];
            for mode in [
                "flood",
                "early-failure",
                "malformed",
                "no-finish",
                "false-success",
            ] {
                let mut cmd = Command::new(&helper);
                cmd.arg(mode);
                result.push(collect_command(&mut cmd, vec![], false).unwrap());
            }
            let _ = send.send(result);
        });
        let r = recv
            .recv_timeout(std::time::Duration::from_secs(30))
            .expect("pipe drain timed out");
        assert_eq!(r[0].execution.exit_code(), 0);
        assert!(r[0].execution.stderr_truncated);
        assert!(r[0].execution.stdout_truncated);
        assert!(r[0].execution.stderr.len() <= LOG_LIMIT);
        assert!(r[0].execution.text_stdout.len() <= LOG_LIMIT);
        assert_eq!(r[1].execution.exit_code(), 101);
        assert!(!r[1].execution.complete);
        assert_eq!(r[2].execution.exit_code(), 1);
        assert!(!r[2].execution.complete);
        assert!(!r[3].execution.complete);
        assert_eq!(r[4].execution.exit_code(), 1);
        assert!(!r[4].execution.complete);
    }
}
