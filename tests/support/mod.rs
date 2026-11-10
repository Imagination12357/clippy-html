use std::{
    path::{Path, PathBuf},
    process::{Command, Output},
};
pub struct Fixture {
    pub temp: tempfile::TempDir,
    pub root: PathBuf,
}
fn copy(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).unwrap();
    for entry in std::fs::read_dir(from).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            copy(&path, &to.join(entry.file_name()));
        } else {
            std::fs::copy(&path, to.join(entry.file_name())).unwrap();
        }
    }
}
impl Fixture {
    pub fn new(name: &str) -> Self {
        // Keep explicit output paths clear of macOS's /var temp-path symlink.
        let parent = std::env::temp_dir().canonicalize().unwrap();
        let temp = tempfile::tempdir_in(parent).unwrap();
        let root = temp.path().join("프로젝트 space");
        copy(
            &Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures/projects")
                .join(name),
            &root,
        );
        Self { temp, root }
    }
    pub fn run(&self, args: &[&str]) -> Output {
        Command::new(env!("CARGO_BIN_EXE_cargo-clippy-html"))
            .current_dir(&self.root)
            .arg("--offline")
            .args(args)
            .output()
            .unwrap()
    }
    pub fn report(&self) -> PathBuf {
        self.root.join("target/clippy-html")
    }
    pub fn html(&self, path: &str) -> String {
        std::fs::read_to_string(self.report().join(path)).unwrap()
    }
}
pub fn assert_success(out: &Output) {
    assert!(
        out.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
pub fn html_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(dir).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files.extend(html_files(&path));
        } else if path.extension().is_some_and(|e| e == "html") {
            files.push(path);
        }
    }
    files.sort();
    files
}
