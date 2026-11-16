//! Stage navigation JSON inside the report or atomically replace an external file.
//! Report and external JSON are separate publications; this is not a transaction.
use crate::output::{OutputTarget, resolve_path};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

pub struct MetadataTarget {
    path: PathBuf,
    relative: Option<PathBuf>,
}

pub enum PreparedMetadata {
    Report {
        relative: PathBuf,
        bytes: Vec<u8>,
    },
    External {
        path: PathBuf,
        temporary: tempfile::NamedTempFile,
    },
}
fn check_file(path: &Path) -> Result<(), String> {
    match fs::symlink_metadata(path) {
        Ok(m) if !m.is_file() => Err(format!(
            "Metadata output must be a regular file: {}",
            path.display()
        )),
        Ok(_) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(format!("Cannot inspect metadata output: {e}")),
    }
}
impl MetadataTarget {
    pub fn prepare(requested: &Path, report: &OutputTarget) -> Result<Self, String> {
        let path = resolve_path(requested)?;
        if path.components().any(|c| {
            c.as_os_str()
                .to_string_lossy()
                .to_ascii_lowercase()
                .starts_with(".clippy-html-")
        }) {
            return Err("Metadata output conflicts with a reserved publication path".into());
        }
        if report.path.starts_with(&path) {
            return Err(
                "Metadata output must not replace the report directory or its ancestors".into(),
            );
        }
        check_file(&path)?;
        let relative = path.strip_prefix(&report.path).ok().map(Path::to_owned);
        Ok(Self { path, relative })
    }
    pub fn stage(self, bytes: Vec<u8>) -> Result<PreparedMetadata, String> {
        if let Some(relative) = self.relative {
            return Ok(PreparedMetadata::Report { relative, bytes });
        }
        let parent = self.path.parent().ok_or("Metadata path has no parent")?;
        fs::create_dir_all(parent).map_err(|e| format!("Cannot create metadata directory: {e}"))?;
        if resolve_path(&self.path)? != self.path {
            return Err("Metadata output path changed".into());
        }
        check_file(&self.path)?;
        let mut temporary = tempfile::NamedTempFile::new_in(parent)
            .map_err(|e| format!("Cannot stage metadata: {e}"))?;
        temporary
            .write_all(&bytes)
            .map_err(|e| format!("Cannot write metadata: {e}"))?;
        temporary
            .flush()
            .map_err(|e| format!("Cannot flush metadata: {e}"))?;
        Ok(PreparedMetadata::External {
            path: self.path,
            temporary,
        })
    }
}
impl PreparedMetadata {
    pub fn write_report(&self, root: &Path) -> Result<(), String> {
        if let Self::Report { relative, bytes } = self {
            let path = root.join(relative);
            fs::create_dir_all(path.parent().ok_or("Metadata path has no parent")?)
                .map_err(|e| format!("Metadata path conflicts with generated report: {e}"))?;
            let mut file = fs::OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .map_err(|e| format!("Metadata path conflicts with generated report: {e}"))?;
            file.write_all(bytes)
                .map_err(|e| format!("Cannot write report metadata: {e}"))?;
        }
        Ok(())
    }
    pub fn publish_external(self) -> Result<(), String> {
        self.publish_with(|temporary, path| {
            temporary
                .persist(path)
                .map(|_| ())
                .map_err(|e| format!("Cannot publish metadata {}: {}", path.display(), e.error))
        })
    }
    fn publish_with(
        self,
        persist: impl FnOnce(tempfile::NamedTempFile, &Path) -> Result<(), String>,
    ) -> Result<(), String> {
        if let Self::External { path, temporary } = self {
            if resolve_path(&path)? != path {
                return Err("Metadata output path changed before publication".into());
            }
            check_file(&path)?;
            persist(temporary, &path)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn tempdir() -> tempfile::TempDir {
        tempfile::tempdir_in(std::env::temp_dir().canonicalize().unwrap()).unwrap()
    }
    #[test]
    fn external_staging_and_failed_publication_keep_old_metadata_and_remove_temporary_files() {
        let dir = tempdir();
        let path = dir.path().join("navigation.json");
        fs::write(&path, "old metadata").unwrap();
        let report = OutputTarget {
            path: dir.path().join("report"),
        };
        let staged = MetadataTarget::prepare(&path, &report)
            .unwrap()
            .stage(b"new metadata".to_vec())
            .unwrap();
        assert_eq!(fs::read(&path).unwrap(), b"old metadata");
        assert!(
            staged
                .publish_with(|_, _| Err("injected replacement failure".into()))
                .is_err()
        );
        assert_eq!(fs::read(&path).unwrap(), b"old metadata");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
        let staged = MetadataTarget::prepare(&path, &report)
            .unwrap()
            .stage(b"new metadata".to_vec())
            .unwrap();
        assert!(
            crate::output::publish_report(&report, |_| Err("injected render failure".into()))
                .is_err()
        );
        drop(staged);
        assert_eq!(fs::read(&path).unwrap(), b"old metadata");
        assert_eq!(fs::read_dir(dir.path()).unwrap().count(), 1);
    }
    #[test]
    fn publication_rechecks_destination_and_does_not_replace_a_directory() {
        let dir = tempdir();
        let path = dir.path().join("navigation.json");
        let report = OutputTarget {
            path: dir.path().join("report"),
        };
        let staged = MetadataTarget::prepare(&path, &report)
            .unwrap()
            .stage(b"new metadata".to_vec())
            .unwrap();
        fs::create_dir(&path).unwrap();
        fs::write(path.join("unrelated"), "preserve").unwrap();
        assert!(staged.publish_external().is_err());
        assert_eq!(fs::read(path.join("unrelated")).unwrap(), b"preserve");
    }
    #[test]
    fn report_staging_refuses_existing_generated_files() {
        let dir = tempdir();
        let report = OutputTarget {
            path: dir.path().join("report"),
        };
        let staged = MetadataTarget::prepare(&report.path.join("index.html"), &report)
            .unwrap()
            .stage(b"json".to_vec())
            .unwrap();
        let stage = tempdir();
        fs::write(stage.path().join("index.html"), "generated html").unwrap();
        assert!(staged.write_report(stage.path()).is_err());
        assert_eq!(
            fs::read(stage.path().join("index.html")).unwrap(),
            b"generated html"
        );
    }
    #[cfg(unix)]
    #[test]
    fn refuses_symlink_files_and_parent_directories() {
        let dir = tempdir();
        let report = OutputTarget {
            path: dir.path().join("report"),
        };
        let original = dir.path().join("original");
        fs::write(&original, "preserve").unwrap();
        let link = dir.path().join("linked.json");
        std::os::unix::fs::symlink(&original, &link).unwrap();
        assert!(MetadataTarget::prepare(&link, &report).is_err());
        let parent = dir.path().join("linked-parent");
        std::os::unix::fs::symlink(dir.path(), &parent).unwrap();
        assert!(MetadataTarget::prepare(&parent.join("new.json"), &report).is_err());
        assert_eq!(fs::read(original).unwrap(), b"preserve");
    }
    #[cfg(windows)]
    #[test]
    fn refuses_junction_parent_directories_and_generated_case_aliases() {
        let dir = tempdir();
        let report = OutputTarget {
            path: dir.path().join("report"),
        };
        let original = dir.path().join("original");
        fs::create_dir(&original).unwrap();
        fs::write(original.join("existing.json"), "preserve").unwrap();
        let junction = dir.path().join("junction");
        let created = std::process::Command::new("cmd")
            .args(["/C", "mklink", "/J"])
            .arg(&junction)
            .arg(&original)
            .output()
            .unwrap();
        assert!(
            created.status.success(),
            "{}",
            String::from_utf8_lossy(&created.stderr)
        );
        assert!(MetadataTarget::prepare(&junction.join("existing.json"), &report).is_err());
        assert_eq!(
            fs::read(original.join("existing.json")).unwrap(),
            b"preserve"
        );
        let staged = MetadataTarget::prepare(&report.path.join("INDEX.HTML"), &report)
            .unwrap()
            .stage(b"json".to_vec())
            .unwrap();
        let stage = tempdir();
        fs::write(stage.path().join("index.html"), "generated html").unwrap();
        assert!(staged.write_report(stage.path()).is_err());
        assert_eq!(
            fs::read(stage.path().join("index.html")).unwrap(),
            b"generated html"
        );
    }
}
