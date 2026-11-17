"""Safe audit probes: all projects/outputs live in an isolated temporary directory."""
import json
import os
from pathlib import Path
import subprocess
import tempfile

REPO = Path(__file__).resolve().parents[3]
BINARY = REPO / "target/debug/cargo-clippy-html.exe"

def run(args, cwd):
    result = subprocess.run([str(arg) for arg in args], cwd=cwd, capture_output=True, text=True, encoding="utf-8", errors="replace", timeout=90)
    return result

def make(root, source, extra=None):
    (root / "src").mkdir(parents=True)
    (root / "Cargo.toml").write_text('[package]\nname="audit_probe"\nversion="0.1.0"\nedition="2024"\n', encoding="utf-8")
    (root / "src/lib.rs").write_bytes(source)
    for name, content in (extra or {}).items():
        (root / "src" / name).write_bytes(content)

def inspect(root, label):
    result = run([BINARY, "--offline", "--metadata", "target/clippy-html/navigation.json", "--", "--offline"], root)
    metadata = root / "target/clippy-html/navigation.json"
    print(json.dumps({"probe":label,"exit":result.returncode,"stderr":result.stderr[-800:] if result.returncode else ""}, ensure_ascii=False))
    if not metadata.exists():
        return
    data = json.loads(metadata.read_text(encoding="utf-8"))
    for finding in data["findings"]:
        if finding["code"] in ["clippy::clone_on_copy", "unused_variables"]:
            loc = finding.get("location", {})
            print(json.dumps({"probe":label,"code":finding["code"],"location":loc}, ensure_ascii=False))
    if label == "extensionless-settings":
        page = root / "target/clippy-html/src/settings.html"
        result = run(["node", REPO / ".audit/full-audit/001/probe-settings.cjs", page], REPO)
        print(result.stdout.strip())
        if result.returncode:
            print(result.stderr[-800:])

with tempfile.TemporaryDirectory(prefix="clippy-html-audit-") as temporary:
    parent = Path(temporary)
    for label, content, extra in [
        ("lf", b"pub fn demo() {\n    let x = 1_u32;\n    let _ = x.clone();\n}\n", None),
        ("crlf", b"pub fn demo() {\r\n    let x = 1_u32;\r\n    let _ = x.clone();\r\n}\r\n", None),
        ("bom", b"\xef\xbb\xbfpub fn demo() {\n    let x = 1_u32;\n    let _ = x.clone();\n}\n", None),
        ("extensionless-settings", b'#[path = "settings"]\nmod settings;\n', {"settings": b"pub fn demo() { let x = 1_u32; let _ = x.clone(); }\n"}),
    ]:
        root = parent / label
        make(root, content, extra)
        inspect(root, label)
    if os.name == "nt":
        root = parent / "metadata-case-alias"
        make(root, b"pub fn demo() {}\n")
        result = run([BINARY, "--offline", "--output", "Report", "--metadata", "report/navigation.json", "--", "--offline"], root)
        print(json.dumps({"probe":"metadata-case-alias","exit":result.returncode,"stderr":result.stderr[-600:],"directoryLeft":(root/"Report").is_dir(),"indexExists":(root/"Report/index.html").exists()}, ensure_ascii=False))
        retry = run([BINARY, "--offline", "--output", "Report", "--metadata", "Report/navigation.json", "--", "--offline"], root)
        print(json.dumps({"probe":"metadata-case-alias-retry-identical-casing","exit":retry.returncode,"stderr":retry.stderr[-450:]}, ensure_ascii=False))
        control = run([BINARY, "--offline", "--output", "Control", "--metadata", "Control/navigation.json", "--", "--offline"], root)
        print(json.dumps({"probe":"metadata-case-control","exit":control.returncode,"indexExists":(root/"Control/index.html").exists()}, ensure_ascii=False))
