#[allow(dead_code)]
mod support;
use support::*;
#[test]
fn separate_build_directory_remains_snippet_only() {
    for separate in [false, true] {
        let f = Fixture::new("clean");
        std::fs::write(
            f.root.join("src/lib.rs"),
            "include!(concat!(env!(\"OUT_DIR\"), \"/generated.rs\"));\n",
        )
        .unwrap();
        let generated = "// FULL_GENERATED_SOURCE_SENTINEL\n\npub fn demo() { let x = 1_u32; let _ = x.clone(); }\n";
        std::fs::write(f.root.join("build.rs"), format!("fn main() {{ std::fs::write(std::path::PathBuf::from(std::env::var_os(\"OUT_DIR\").unwrap()).join(\"generated.rs\"), {generated:?}).unwrap(); }}")).unwrap();
        if separate {
            std::fs::create_dir(f.root.join(".cargo")).unwrap();
            std::fs::write(
                f.root.join(".cargo/config.toml"),
                "[build]\nbuild-dir = 'generated-cache'\n",
            )
            .unwrap();
        }
        assert_success(&f.run(&["--metadata", "meta.json"]));
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(f.root.join("meta.json")).unwrap()).unwrap();
        let finding = json["findings"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["code"] == "clippy::clone_on_copy")
            .unwrap();
        assert!(
            finding["location"]["url"]
                .as_str()
                .unwrap()
                .contains("#finding-"),
            "separate={separate}"
        );
        for page in html_files(&f.report()) {
            let html = std::fs::read_to_string(page).unwrap();
            assert!(
                !html.contains("FULL_GENERATED_SOURCE_SENTINEL"),
                "separate={separate}"
            );
            assert!(!html.contains("class=\"example-wrap suggestion-diff\""));
        }
    }
}
#[test]
fn first_line_bom_keeps_precise_ranges_and_applicable_patch() {
    for ending in ["\n", "\r\n"] {
        let f = Fixture::new("clean");
        let original =
            format!("\u{feff}pub fn demo() {{ let x = 1_u32; let _ = x.clone(); }}{ending}");
        std::fs::write(f.root.join("src/lib.rs"), &original).unwrap();
        assert_success(&f.run(&["--metadata", "meta.json"]));
        let json: serde_json::Value =
            serde_json::from_slice(&std::fs::read(f.root.join("meta.json")).unwrap()).unwrap();
        let finding = json["findings"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["code"] == "clippy::clone_on_copy")
            .unwrap();
        assert!(
            finding["location"]["url"]
                .as_str()
                .unwrap()
                .ends_with("#L1")
        );
        let html = f.html("src/lib.rs.html");
        let doc = scraper::Html::parse_document(&html);
        let marked: String = doc
            .select(&scraper::Selector::parse(".diagnostic-span.primary:not(.insertion)").unwrap())
            .flat_map(|e| e.text())
            .collect();
        assert_eq!(marked, "x.clone()", "highlight must not shift by the BOM");
        let lint = scraper::Html::parse_document(&f.html("lint/clippy/clone_on_copy.html"));
        let button = lint
            .select(&scraper::Selector::parse("[data-patch]").unwrap())
            .next()
            .expect("valid whole patch");
        let patch: String =
            serde_json::from_str(button.value().attr("data-patch").unwrap()).unwrap();
        let patch_file = f.root.join("patch.diff");
        std::fs::write(&patch_file, patch).unwrap();
        let applied = std::process::Command::new("git")
            .current_dir(&f.root)
            .args([
                "-c",
                "core.autocrlf=false",
                "-c",
                "core.eol=lf",
                "apply",
                "--no-index",
                "--whitespace=nowarn",
            ])
            .arg("patch.diff")
            .output()
            .unwrap();
        assert_success(&applied);
        assert_eq!(
            std::fs::read(f.root.join("src/lib.rs")).unwrap(),
            original.replace("x.clone()", "x").as_bytes()
        );
    }
}
