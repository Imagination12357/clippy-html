"""Exercise the current non-Windows metadata branch on available case-insensitive storage.

This is branch emulation, NOT a macOS/Linux execution. All source copies and the
Cargo package are temporary. The existing output/publication module is imported
unchanged; report validation is a stub that is never reached by the failure.
"""
from pathlib import Path
import os
import subprocess
import tempfile

REPO = Path(__file__).resolve().parents[3]
if os.name != "nt":
    raise SystemExit("This emulator is intended for the available Windows filesystem")
text = (REPO/"src/metadata/output.rs").read_text(encoding="utf-8")
start = text.index("fn report_relative_path(")
end = text.index("\nimpl MetadataTarget", start)
function = text[start:end]
prefix = function[:function.index("    #[cfg(windows)]")]
assert "#[cfg(not(windows))]\n    Ok(None)" in function
emulated = text[:start] + prefix + "    Ok(None)\n}\n" + text[end:]

with tempfile.TemporaryDirectory(prefix="clippy-html-fa002-platform-") as tmp:
    root = Path(tmp)
    (root/"src").mkdir()
    manifest = (REPO/"Cargo.toml").read_text(encoding="utf-8")
    manifest = manifest.replace('name = "cargo-clippy-html"', 'name = "audit-fa002-platform-probe"', 1)
    (root/"Cargo.toml").write_text(manifest, encoding="utf-8")
    (root/"Cargo.lock").write_bytes((REPO/"Cargo.lock").read_bytes())
    (root/"src/metadata_output.rs").write_text(emulated, encoding="utf-8")
    output_path = (REPO/"src/output.rs").as_posix()
    code = r'''
#![allow(dead_code)]
mod diagnostic { pub mod model {
    pub struct PackageInfo { pub manifest: std::path::PathBuf }
    pub struct ProjectInfo { pub workspace_root: std::path::PathBuf, pub target_dir: std::path::PathBuf, pub packages: Vec<PackageInfo> }
}}
mod render { pub fn validate_report(_: &std::path::Path) -> Result<(),String> { panic!("failure must occur before report validation") } }
#[path = "OUTPUT_PATH"] mod output;
mod metadata_output;
fn main() {
    let tmp = tempfile::tempdir().unwrap();
    let root = tmp.path().canonicalize().unwrap();
    std::fs::write(root.join("case-probe"), "test").unwrap();
    assert!(root.join("CASE-PROBE").exists(), "emulation requires case-insensitive storage");
    let project = diagnostic::model::ProjectInfo { workspace_root: root.clone(), target_dir: root.join("target"), packages: vec![] };
    let target = output::prepare_target(&root.join("Report"), &project).unwrap();
    let metadata = metadata_output::MetadataTarget::prepare(&root.join("report/nav.json"), &target).unwrap().stage(b"{}".to_vec()).unwrap();
    let is_external = matches!(&metadata, metadata_output::PreparedMetadata::External { .. });
    let result = output::publish_report(&target, |_| panic!("failure must occur before rendering"));
    let error = result.err().expect("case alias must fail in the non-Windows branch");
    let retry = output::prepare_target(&root.join("Report"), &project).err().unwrap();
    println!("non_windows_branch_emulation external={is_external}; first_error={error}; report_dir_left={}; index_exists={}; retry_error={retry}", root.join("Report").is_dir(), root.join("Report/index.html").exists());
    drop(metadata);
}
'''.replace("OUTPUT_PATH", output_path)
    (root/"src/main.rs").write_text(code, encoding="utf-8")
    result = subprocess.run(["cargo", "+1.100.0", "run", "--quiet", "--offline", "--manifest-path", str(root/"Cargo.toml"), "--target-dir", str(REPO/"target")], cwd=root, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=90)
    print(result.stdout.strip())
    print(result.stderr.strip())
    raise SystemExit(result.returncode)
