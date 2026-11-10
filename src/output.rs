//! Stage complete reports and replace only owned output directories.
use crate::diagnostic::model::ProjectInfo;
use std::path::{Path, PathBuf};
pub struct OutputTarget {
    pub path: PathBuf,
}
pub struct PublishedReport {
    pub index: PathBuf,
    pub warnings: Vec<String>,
}
use std::{
    fs,
    io::{self, Write},
};
const MARKER: &str = ".clippy-html-owned";
fn canonical_future(path: &Path) -> Result<PathBuf, String> {
    if let Ok(m) = fs::symlink_metadata(path) {
        #[cfg(windows)]
        {
            use std::os::windows::fs::MetadataExt;
            if m.file_attributes() & 0x400 != 0 {
                return Err(format!(
                    "Refusing reparse-point output path: {}",
                    path.display()
                ));
            }
        }
        if m.file_type().is_symlink() {
            return Err(format!("Refusing symlink output path: {}", path.display()));
        }
    }
    let parent = path.parent().filter(|p| !p.as_os_str().is_empty());
    if let Some(parent) = parent {
        let base = canonical_future(parent)?;
        let name = path
            .file_name()
            .ok_or("Output path must name a directory")?;
        if path.exists() {
            path.canonicalize().map_err(|e| e.to_string())
        } else {
            Ok(base.join(name))
        }
    } else {
        path.canonicalize().map_err(|e| e.to_string())
    }
}
pub fn prepare_target(requested: &Path, project: &ProjectInfo) -> Result<OutputTarget, String> {
    let absolute = if requested.is_absolute() {
        requested.to_owned()
    } else {
        std::env::current_dir()
            .map_err(|e| e.to_string())?
            .join(requested)
    };
    let mut cleaned = PathBuf::new();
    for component in absolute.components() {
        match component {
            std::path::Component::ParentDir => {
                cleaned.pop();
            }
            std::path::Component::CurDir => {}
            c => cleaned.push(c),
        }
    }
    let path = canonical_future(&cleaned)?;
    let mut protected = vec![
        canonical_future(&project.workspace_root)?,
        canonical_future(&project.target_dir)?,
    ];
    protected.extend(
        project
            .packages
            .iter()
            .filter_map(|p| p.manifest.parent())
            .map(canonical_future)
            .collect::<Result<Vec<_>, _>>()?,
    );
    if protected.iter().any(|p| p.starts_with(&path)) {
        return Err("Output must not replace a project, package, or target root".into());
    }
    check_owned(&path)?;
    Ok(OutputTarget { path })
}
fn check_owned(path: &Path) -> Result<(), String> {
    if !path.exists() {
        return Ok(());
    }
    if !path.is_dir() || !fs::read_to_string(path.join(MARKER)).is_ok_and(|s| s == "1\n") {
        return Err(format!(
            "Refusing to overwrite non-report directory: {}",
            path.display()
        ));
    }
    Ok(())
}
struct Lock {
    path: PathBuf,
    _file: fs::File,
}
impl Drop for Lock {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}
pub fn publish_report(
    target: &OutputTarget,
    build: impl FnOnce(&Path) -> Result<(), String>,
) -> Result<PublishedReport, String> {
    publish_with_rename(target, build, |from, to| fs::rename(from, to))
}
fn publish_with_rename(
    target: &OutputTarget,
    build: impl FnOnce(&Path) -> Result<(), String>,
    mut rename: impl FnMut(&Path, &Path) -> io::Result<()>,
) -> Result<PublishedReport, String> {
    let parent = target.path.parent().ok_or("Output has no parent")?;
    fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    // Parent-scoped publication serializes case aliases without assuming that
    // every Windows directory uses the same case-sensitivity rules.
    let parent = parent.canonicalize().map_err(|e| e.to_string())?;
    let lock_path = parent.join(".clippy-html-publish.lock");
    let mut file=fs::OpenOptions::new().write(true).create_new(true).open(&lock_path).map_err(|e|format!("Cannot lock report output ({e}); if no generator is running, inspect and remove {}",lock_path.display()))?;
    writeln!(file, "{}", std::process::id()).map_err(|e| e.to_string())?;
    let lock = Lock {
        path: lock_path,
        _file: file,
    };
    // Recheck after acquiring the lock, before touching an existing report.
    let checked = canonical_future(&target.path)?;
    if checked != target.path {
        return Err("Output path changed during generation".into());
    }
    check_owned(&target.path)?;
    let backup = parent.join(format!(
        ".clippy-html-{}.backup",
        blake3::hash(target.path.to_string_lossy().as_bytes()).to_hex()
    ));
    if backup.exists() {
        return Err(format!(
            "Recovery backup already exists; inspect {}",
            backup.display()
        ));
    }
    let staging = tempfile::Builder::new()
        .prefix(".clippy-html-stage-")
        .tempdir_in(&parent)
        .map_err(|e| e.to_string())?;
    build(staging.path())?;
    crate::render::validate_report(staging.path())?;
    fs::write(staging.path().join(MARKER), "1\n").map_err(|e| e.to_string())?;
    // Rendering may take time: another application can create or replace the
    // destination after the initial check without using our publication lock.
    if canonical_future(&target.path)? != target.path {
        return Err("Output path changed during rendering".into());
    }
    check_owned(&target.path)?;
    let old = target.path.exists();
    if old {
        rename(&target.path, &backup)
            .map_err(|e| format!("Cannot preserve existing report: {e}"))?;
        if let Err(problem) = check_owned(&backup) {
            if !target.path.exists() && rename(&backup, &target.path).is_ok() {
                return Err(format!("Unexpected destination preserved: {problem}"));
            }
            return Err(format!(
                "Unexpected destination retained at {}: {problem}",
                backup.display()
            ));
        }
    }
    if let Err(e) = rename(staging.path(), &target.path) {
        if old && let Err(restore) = rename(&backup, &target.path) {
            return Err(format!(
                "Report replacement failed: {e}; restore failed: {restore}; preserved backup: {}",
                backup.display()
            ));
        }
        return Err(format!(
            "Report replacement failed; previous report restored: {e}"
        ));
    }
    let mut warnings = vec![];
    if old && let Err(e) = fs::remove_dir_all(&backup) {
        warnings.push(format!(
            "Report ready; backup cleanup failed ({e}): {}",
            backup.display()
        ));
    }
    drop(lock);
    Ok(PublishedReport {
        index: target.path.join("index.html"),
        warnings,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn tempdir() -> tempfile::TempDir {
        // macOS's default temp path goes through the /var symlink.
        let parent = std::env::temp_dir().canonicalize().unwrap();
        tempfile::tempdir_in(parent).unwrap()
    }
    fn project(root: &Path) -> ProjectInfo {
        ProjectInfo {
            workspace_root: root.into(),
            target_dir: root.join("target"),
            packages: vec![],
        }
    }
    #[test]
    fn build_failure_keeps_old_report() {
        let t = tempdir();
        let path = t.path().join("report");
        std::fs::create_dir(&path).unwrap();
        std::fs::write(path.join(".clippy-html-owned"), "1\n").unwrap();
        std::fs::write(path.join("index.html"), "old").unwrap();
        let target = prepare_target(&path, &project(t.path())).unwrap();
        assert!(publish_report(&target, |_| Err("failed render".into())).is_err());
        assert_eq!(
            std::fs::read_to_string(path.join("index.html")).unwrap(),
            "old"
        );
    }
    #[test]
    fn missing_font_assets_keep_old_report() {
        let t = tempdir();
        let path = t.path().join("report");
        std::fs::create_dir(&path).unwrap();
        std::fs::write(path.join(".clippy-html-owned"), "1\n").unwrap();
        std::fs::write(path.join("index.html"), "old").unwrap();
        let target = prepare_target(&path, &project(t.path())).unwrap();
        let result = publish_report(&target, |staging| {
            simple_report(staging)?;
            std::fs::create_dir(staging.join("static.files")).map_err(|e| e.to_string())?;
            std::fs::write(
                staging.join("static.files/fonts.css"),
                "@font-face { src: url(\"missing.woff2\"); }",
            )
            .map_err(|e| e.to_string())?;
            std::fs::write(
                staging.join("index.html"),
                "<link href=\"static.files/fonts.css\">",
            )
            .map_err(|e| e.to_string())
        });
        assert!(matches!(result, Err(message) if message.contains("Missing font resource")));
        assert_eq!(
            std::fs::read_to_string(path.join("index.html")).unwrap(),
            "old"
        );
    }
    #[test]
    fn refuses_unowned_or_project_directory() {
        let t = tempdir();
        let p = project(t.path());
        assert!(prepare_target(t.path(), &p).is_err());
        let other = t.path().join("notes");
        std::fs::create_dir(&other).unwrap();
        assert!(prepare_target(&other, &p).is_err());
        assert!(prepare_target(&t.path().join("new-report"), &p).is_ok());
    }
    fn simple_report(path: &Path) -> Result<(), String> {
        fs::create_dir(path.join("lint")).map_err(|e| e.to_string())?;
        fs::create_dir(path.join("src")).map_err(|e| e.to_string())?;
        fs::write(path.join("index.html"), "<!doctype html><title>new</title>")
            .map_err(|e| e.to_string())
    }
    #[test]
    fn rename_failure_restores_backup() {
        let t = tempdir();
        let path = t.path().join("report");
        fs::create_dir(&path).unwrap();
        fs::write(path.join(MARKER), "1\n").unwrap();
        fs::write(path.join("index.html"), "old").unwrap();
        let target = prepare_target(&path, &project(t.path())).unwrap();
        let mut count = 0;
        let result = publish_with_rename(&target, simple_report, |a, b| {
            count += 1;
            if count == 2 {
                Err(io::Error::other("injected publish failure"))
            } else {
                fs::rename(a, b)
            }
        });
        assert!(result.is_err());
        assert_eq!(fs::read_to_string(path.join("index.html")).unwrap(), "old");
    }
    #[test]
    fn rollback_failure_retains_backup() {
        let t = tempdir();
        let path = t.path().join("report");
        fs::create_dir(&path).unwrap();
        fs::write(path.join(MARKER), "1\n").unwrap();
        fs::write(path.join("index.html"), "old").unwrap();
        let target = prepare_target(&path, &project(t.path())).unwrap();
        let mut count = 0;
        let result = publish_with_rename(&target, simple_report, |a, b| {
            count += 1;
            if count >= 2 {
                Err(io::Error::other("injected rename failure"))
            } else {
                fs::rename(a, b)
            }
        });
        let e = result.err().unwrap();
        assert!(e.contains("preserved backup"));
        let backup = fs::read_dir(t.path())
            .unwrap()
            .map(|e| e.unwrap().path())
            .find(|p| p.extension().is_some_and(|s| s == "backup"))
            .unwrap();
        assert_eq!(
            fs::read_to_string(backup.join("index.html")).unwrap(),
            "old"
        );
    }
    #[test]
    fn concurrent_publish_has_one_owner() {
        let t = tempdir();
        let target = prepare_target(&t.path().join("report"), &project(t.path())).unwrap();
        let result = publish_report(&target, |dir| {
            assert!(publish_report(&target, simple_report).is_err());
            simple_report(dir)
        })
        .unwrap();
        assert!(result.index.is_file());
        assert!(result.warnings.is_empty());
    }
    #[test]
    fn unexpected_destination_created_during_render_is_preserved() {
        let t = tempdir();
        let path = t.path().join("report");
        let target = prepare_target(&path, &project(t.path())).unwrap();
        let result = publish_report(&target, |staging| {
            fs::create_dir(&path).unwrap();
            fs::write(path.join("important.txt"), "keep me").unwrap();
            simple_report(staging)
        });
        assert!(result.is_err());
        assert_eq!(
            fs::read_to_string(path.join("important.txt")).unwrap(),
            "keep me"
        );
    }
    #[test]
    fn unexpected_object_moved_to_backup_is_never_deleted() {
        let t = tempdir();
        let path = t.path().join("report");
        fs::create_dir(&path).unwrap();
        fs::write(path.join(MARKER), "1\n").unwrap();
        fs::write(path.join("index.html"), "old").unwrap();
        let target = prepare_target(&path, &project(t.path())).unwrap();
        let mut call = 0;
        let result = publish_with_rename(&target, simple_report, |from, to| {
            call += 1;
            if call == 1 {
                fs::rename(from, t.path().join("original"))?;
                fs::create_dir(from)?;
                fs::write(from.join("important.txt"), "keep me")?;
            }
            fs::rename(from, to)
        });
        assert!(result.is_err());
        assert!(
            fs::read_dir(t.path())
                .unwrap()
                .map(|e| e.unwrap().path())
                .any(|p| p.is_dir()
                    && fs::read_to_string(p.join("important.txt")).is_ok_and(|s| s == "keep me"))
        );
    }
    #[cfg(windows)]
    #[test]
    fn case_aliases_share_one_publication_lock() {
        let t = tempdir();
        fs::write(t.path().join("case-probe"), "probe").unwrap();
        if !t.path().join("CASE-PROBE").exists() {
            return;
        }
        let a = prepare_target(&t.path().join("Report"), &project(t.path())).unwrap();
        let b = prepare_target(&t.path().join("report"), &project(t.path())).unwrap();
        let entered = std::cell::Cell::new(false);
        publish_report(&a, |staging| {
            assert!(
                publish_report(&b, |_| {
                    entered.set(true);
                    Err("nested".into())
                })
                .is_err()
            );
            simple_report(staging)
        })
        .unwrap();
        assert!(
            !entered.get(),
            "case alias entered the protected publication section"
        );
    }
}
