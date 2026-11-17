"""FA-002: run real current-binary boundary probes only in disposable projects."""
from pathlib import Path
import json
import os
import subprocess
import tempfile

REPO = Path(__file__).resolve().parents[3]
BINARY = REPO / "target/debug/cargo-clippy-html.exe"

def run(args, cwd):
    return subprocess.run([str(a) for a in args], cwd=cwd, capture_output=True, text=True,
                          encoding="utf-8", errors="replace", timeout=90)

def project(root, source):
    (root / "src").mkdir(parents=True)
    (root / "Cargo.toml").write_text('[package]\nname="audit_boundary"\nversion="0.1.0"\nedition="2024"\n', encoding="utf-8")
    (root / "src/lib.rs").write_bytes(source)

def inspect(root, label):
    result = run([BINARY, "--offline", "--metadata", "target/clippy-html/meta.json", "--", "--offline"], root)
    record = {"probe": label, "exit": result.returncode}
    path = root / "target/clippy-html/meta.json"
    if path.exists():
        data = json.loads(path.read_text(encoding="utf-8"))
        findings = [f for f in data["findings"] if f["code"] == "clippy::clone_on_copy"]
        record["locations"] = [f.get("location") for f in findings]
        lint = root / "target/clippy-html/lint/clippy/clone_on_copy.html"
        if lint.exists():
            text = lint.read_text(encoding="utf-8")
            record["hasDiff"] = 'class="example-wrap suggestion-diff"' in text
            record["hasFallback"] = 'Diff unavailable:' in text
        if label.startswith("first-line-"):
            html = (root/"target/clippy-html/src/lib.rs.html").read_text(encoding="utf-8")
            record["preciseRangeCount"] = html.count("diagnostic-span primary")
            collected = run(["cargo","clippy","--offline","--message-format=json"],root)
            for line in collected.stdout.splitlines():
                try:
                    item = json.loads(line)
                except ValueError:
                    continue
                message = item.get("message", {})
                if item.get("reason") == "compiler-message" and (message.get("code") or {}).get("code") == "clippy::clone_on_copy":
                    span = next(s for s in message["spans"] if s["is_primary"])
                    record["compilerSpan"] = {key:span[key] for key in ["byte_start","byte_end","column_start","column_end","text"]}
                    break
    else:
        record["stderr"] = result.stderr[-650:]
    print(json.dumps(record, ensure_ascii=False))

with tempfile.TemporaryDirectory(prefix="clippy-html-fa002-") as tmp:
    parent = Path(tmp)
    single = b"pub fn demo() { let x = 1_u32; let _ = x.clone(); }\n"
    for label, source in [("first-line-lf", single), ("first-line-bom", b"\xef\xbb\xbf"+single),
                          ("first-line-crlf", single.replace(b"\n", b"\r\n"))]:
        root = parent / label
        project(root, source)
        inspect(root, label)
    root = parent / "member-feature"
    project(root, b"pub fn demo() {}\n")
    with (root/"Cargo.toml").open("a", encoding="utf-8") as out:
        out.write('[workspace]\nmembers=["member"]\n')
    (root/"member/src").mkdir(parents=True)
    (root/"member/Cargo.toml").write_text('[package]\nname="member"\nversion="0.1.0"\nedition="2024"\n[features]\nspecial=[]\n', encoding="utf-8")
    (root/"member/src/lib.rs").write_text('pub fn member() {}\n', encoding="utf-8")
    for feature in ["special", "-Fspecial"]:
        args = [BINARY, "--offline", "--", "--offline", "-p", "member"]
        args += ["--features", feature] if feature == "special" else [feature]
        result = run(args, root)
        print(json.dumps({"probe":"member-feature-"+feature,"exit":result.returncode,"stderr":result.stderr[-650:] if result.returncode else ""}))
    for label, separate in [("default-build-dir", False), ("custom-build-dir", True)]:
        root = parent / label
        project(root, b'include!(concat!(env!("OUT_DIR"), "/generated.rs"));\n')
        if separate:
            (root/".cargo").mkdir()
            (root/".cargo/config.toml").write_text('[build]\nbuild-dir="generated-cache"\n',encoding="utf-8")
        generated = '// FULL_GENERATED_SOURCE_SENTINEL\n\npub fn demo() { let x = 1_u32; let _ = x.clone(); }\n'
        (root/"build.rs").write_text('fn main() { std::fs::write(std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("generated.rs"), '+json.dumps(generated)+').unwrap(); }\n',encoding="utf-8")
        inspect(root, label)
        report = root/"target/clippy-html"
        source_pages = list((report/"src").rglob('*.html')) if (report/"src").exists() else []
        copied = [p.relative_to(report).as_posix() for p in source_pages if "FULL_GENERATED_SOURCE_SENTINEL" in p.read_text(encoding="utf-8")]
        metadata = run(["cargo","metadata","--format-version","1","--offline","--no-deps"],root)
        cargo_data = json.loads(metadata.stdout) if metadata.returncode == 0 else {}
        directories = {key: str(Path(cargo_data[key]).relative_to(root)) for key in ["target_directory","build_directory"] if cargo_data.get(key)}
        print(json.dumps({"probe":label+"-full-copy","directories":directories,"pagesWithSentinel":copied},ensure_ascii=False))
    if os.name == "nt":
        root = parent/"case-directory"
        root.mkdir()
        changed = run(["fsutil", "file", "setCaseSensitiveInfo", root, "enable"], root)
        print(json.dumps({"probe":"case-sensitive-temp-directory-support","exit":changed.returncode,"stdout":changed.stdout.strip(),"stderr":changed.stderr.strip()}, ensure_ascii=False))
        if changed.returncode == 0:
            project(root/"project", b"pub fn demo() {}\n")
            result = run([BINARY,"--offline","--output","Report","--metadata","report/nav.json","--","--offline"],root/"project")
            print(json.dumps({"probe":"case-sensitive-windows-output","exit":result.returncode,"reportExists":(root/"project/Report/index.html").exists(),"externalExists":(root/"project/report/nav.json").exists(),"stderr":result.stderr[-650:] if result.returncode else ""},ensure_ascii=False))
