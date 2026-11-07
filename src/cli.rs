//! Parse tool arguments without reinterpreting forwarded rustc arguments.
use clap::{Parser, error::ErrorKind};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
};

#[derive(Debug, Parser)]
#[command(
    name = "cargo clippy-html",
    version,
    about = "Generate offline HTML reports from Clippy diagnostics"
)]
pub struct RunOptions {
    #[arg(long, value_name = "DIR")]
    pub output: Option<PathBuf>,
    #[arg(long)]
    pub open: bool,
    /// Use bundled category data without fetching it (Cargo flags go after --).
    #[arg(long)]
    pub offline: bool,
    #[arg(last = true, value_name = "CLIPPY_ARGS")]
    pub clippy_args: Vec<OsString>,
}

pub fn parse_args(mut args: Vec<OsString>) -> Result<RunOptions, clap::Error> {
    if args.get(1).is_some_and(|a| a == "clippy-html") {
        args.remove(1);
    }
    let opts = RunOptions::try_parse_from(args)?;
    for arg in opts.clippy_args.iter().take_while(|a| *a != "--") {
        let text = arg.to_string_lossy();
        if text == "--fix"
            || text.starts_with("--fix=")
            || text == "--message-format"
            || text.starts_with("--message-format=")
        {
            return Err(clap::Error::raw(
                ErrorKind::ArgumentConflict,
                "--fix and --message-format cannot be forwarded to cargo clippy-html\n",
            ));
        }
    }
    Ok(opts)
}

pub struct CargoContext {
    pub cwd: PathBuf,
    pub cargo_exe: OsString,
    pub metadata_args: Vec<OsString>,
    pub target_dir_override: Option<PathBuf>,
}

pub fn cargo_context(
    opts: &RunOptions,
    cwd: &Path,
    cargo_exe: OsString,
) -> Result<CargoContext, String> {
    let mut context = CargoContext {
        cwd: cwd.to_owned(),
        cargo_exe,
        metadata_args: vec![],
        target_dir_override: None,
    };
    let args: Vec<_> = opts.clippy_args.iter().take_while(|a| *a != "--").collect();
    let mut i = 0;
    while i < args.len() {
        let text = args[i].to_string_lossy();
        let (name, inline) = text
            .split_once('=')
            .map_or((text.as_ref(), None), |(k, v)| (k, Some(v)));
        let has_value = matches!(
            name,
            "--manifest-path" | "--config" | "--features" | "-F" | "--target-dir"
        );
        if has_value {
            let value = if let Some(v) = inline {
                OsString::from(v)
            } else {
                i += 1;
                args.get(i)
                    .ok_or_else(|| format!("{name} requires a value"))?
                    .to_os_string()
            };
            if name == "--target-dir" {
                context.target_dir_override = Some(cwd.join(value));
            } else {
                context.metadata_args.extend([OsString::from(name), value]);
            }
        } else if matches!(
            name,
            "--locked" | "--offline" | "--frozen" | "--all-features" | "--no-default-features"
        ) {
            context.metadata_args.push(args[i].to_os_string());
        }
        i += 1;
    }
    Ok(context)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn tool_and_cargo_offline_flags_are_independent() {
        for command in [
            vec!["cargo-clippy-html"],
            vec!["cargo-clippy-html", "--offline"],
            vec!["cargo-clippy-html", "--", "--offline"],
            vec!["cargo-clippy-html", "--offline", "--", "--offline"],
        ] {
            let opts = parse_args(args(&command)).unwrap();
            assert_eq!(opts.offline, command.get(1) == Some(&"--offline"));
            let forwarded = command.contains(&"--");
            assert_eq!(
                opts.clippy_args,
                if forwarded {
                    vec![OsString::from("--offline")]
                } else {
                    vec![]
                }
            );
            let context = cargo_context(&opts, Path::new("/demo"), "cargo".into()).unwrap();
            assert_eq!(
                context.metadata_args.contains(&OsString::from("--offline")),
                forwarded
            );
        }
    }
    fn args(s: &[&str]) -> Vec<OsString> {
        s.iter().map(OsString::from).collect()
    }
    #[test]
    fn cargo_invocation_preserves_second_separator() {
        let result = parse_args(args(&[
            "cargo-clippy-html",
            "clippy-html",
            "--output",
            "report",
            "--",
            "--all-targets",
            "--",
            "-W",
            "clippy::pedantic",
        ]))
        .unwrap();
        assert_eq!(result.output, Some(PathBuf::from("report")));
        assert_eq!(
            result.clippy_args,
            args(&["--all-targets", "--", "-W", "clippy::pedantic"])
        );
    }
    #[test]
    fn rejects_mutating_and_message_format_args() {
        for forwarded in [
            &["--fix"][..],
            &["--message-format=json"],
            &["--message-format", "short"],
        ] {
            let mut input = args(&["tool", "--"]);
            input.extend(args(forwarded));
            assert!(parse_args(input).is_err());
        }
        assert!(parse_args(args(&["tool", "--metadata", "x.json"])).is_err());
        assert!(parse_args(args(&["tool", "--", "--", "--message-format=json"])).is_ok());
    }
    #[test]
    fn unicode_os_paths_survive() {
        let r = parse_args(args(&[
            "tool",
            "--output",
            "보고서 폴더",
            "--",
            "--manifest-path=한글/Cargo.toml",
        ]))
        .unwrap();
        assert_eq!(r.output, Some(PathBuf::from("보고서 폴더")));
        assert_eq!(r.clippy_args, args(&["--manifest-path=한글/Cargo.toml"]));
    }
    #[test]
    fn help_needs_no_project() {
        assert_eq!(
            parse_args(args(&["tool", "--help"]))
                .unwrap_err()
                .exit_code(),
            0
        );
        assert_eq!(
            parse_args(args(&["tool", "--version"]))
                .unwrap_err()
                .exit_code(),
            0
        );
    }
    #[test]
    fn context_respects_equals_and_separate_paths() {
        let opts = parse_args(args(&[
            "tool",
            "--",
            "--manifest-path=한글/Cargo.toml",
            "--target-dir",
            "cache",
            "--offline",
            "--",
            "--config=not-cargo",
        ]))
        .unwrap();
        let c = cargo_context(&opts, Path::new("root"), "cargo".into()).unwrap();
        assert_eq!(c.target_dir_override, Some(Path::new("root").join("cache")));
        assert_eq!(
            c.metadata_args,
            args(&["--manifest-path", "한글/Cargo.toml", "--offline"])
        );
    }
}
