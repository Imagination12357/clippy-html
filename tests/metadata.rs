#[allow(dead_code)]
mod support;
use scraper::{Html, Selector};
use serde_json::Value;
use std::path::Path;
use support::*;

fn read(path: &Path) -> Value {
    serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
}
#[test]
fn new_report_case_alias_metadata_is_internal_and_repeated_runs_succeed() {
    let f = Fixture::new("clean");
    let probe = f.root.join("case-probe");
    std::fs::write(&probe, "probe").unwrap();
    if !f.root.join("CASE-PROBE").exists() {
        return;
    }
    for (report, metadata) in [
        ("Report", "report/navigation.json"),
        ("Report", "Report/navigation.json"),
        ("Report", "REPORT/navigation.json"),
        ("Nested/Report", "nested/REPORT/navigation.json"),
        ("Étage", "étage/navigation.json"),
    ] {
        assert_success(&f.run(&["--output", report, "--metadata", metadata]));
        assert!(f.root.join(report).join("index.html").is_file());
        assert_eq!(read(&f.root.join(metadata))["schema_version"], 1);
    }
}
#[test]
fn distinct_case_sensitive_output_paths_remain_external() {
    let f = Fixture::new("clean");
    std::fs::write(f.root.join("case-probe"), "probe").unwrap();
    if f.root.join("CASE-PROBE").exists() {
        return;
    }
    assert_success(&f.run(&["--output", "Report", "--metadata", "report/navigation.json"]));
    assert!(f.root.join("Report/index.html").is_file());
    assert!(!f.root.join("report/index.html").exists());
    assert_eq!(
        read(&f.root.join("report/navigation.json"))["schema_version"],
        1
    );
}
fn assert_url(root: &Path, href: &str) {
    assert!(!href.starts_with('/') && !href.contains('\\') && !href.contains("://"));
    let base = url::Url::from_directory_path(root).unwrap();
    let target = base.join(href).unwrap();
    let file = target.to_file_path().unwrap().canonicalize().unwrap();
    assert!(file.starts_with(root.canonicalize().unwrap()));
    let html = Html::parse_document(&std::fs::read_to_string(file).unwrap());
    if let Some(fragment) = target.fragment() {
        assert!(
            html.select(&Selector::parse("[id]").unwrap())
                .any(|e| e.value().id() == Some(fragment)),
            "{href}"
        );
    }
}
#[test]
fn metadata_is_optional_and_links_to_real_report_destinations() {
    let f = Fixture::new("warning");
    assert_success(&f.run(&[]));
    assert!(!f.report().join("metadata.json").exists());
    assert_success(&f.run(&["--metadata", "target/clippy-html/metadata.json"]));
    let json = read(&f.report().join("metadata.json"));
    assert_eq!(json["schema_version"], 1);
    assert_eq!(json["generator"]["name"], "cargo-clippy-html");
    assert_eq!(json["generator"]["version"], env!("CARGO_PKG_VERSION"));
    assert_eq!(json["execution"]["clippy_exit_code"], 0);
    assert_eq!(json["execution"]["collection_complete"], true);
    assert!(json.get("report_url").is_none() && json.get("toolchain").is_none());
    let pages = json["pages"].as_array().unwrap();
    for kind in ["overview", "lint", "category", "source"] {
        assert!(pages.iter().any(|p| p["kind"] == kind));
    }
    for page in pages {
        assert_url(&f.report(), page["url"].as_str().unwrap());
    }
    let findings = json["findings"].as_array().unwrap();
    assert!(!findings.is_empty());
    for finding in findings {
        assert_eq!(finding["code"], "clippy::clone_on_copy");
        assert_eq!(finding["level"], "warning");
        assert_eq!(finding["category"], "complexity");
        assert!(finding["message"].as_str().unwrap().contains("clone"));
        assert!(
            json["packages"]
                .as_array()
                .unwrap()
                .iter()
                .any(|p| p["id"] == finding["package_id"])
        );
        assert!(finding.get("suggestions").is_none() && finding.get("children").is_none());
        assert_url(&f.report(), finding["url"].as_str().unwrap());
        assert_eq!(finding["location"]["path"], "src/lib.rs");
        assert_eq!(finding["location"]["line"], 3);
        assert_url(&f.report(), finding["location"]["url"].as_str().unwrap());
    }
}
#[test]
fn external_metadata_replaces_the_selected_file_without_using_its_folder_as_url_base() {
    let f = Fixture::new("workspace");
    let path = f.root.join("수집 결과/nested/navigation.json");
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(&path, "old result").unwrap();
    assert_success(&f.run(&[
        "--metadata",
        "수집 결과/nested/navigation.json",
        "--",
        "--all-targets",
    ]));
    let json = read(&path);
    assert!(json["packages"].as_array().unwrap().len() >= 2);
    let ids: std::collections::BTreeSet<_> = json["findings"]
        .as_array()
        .unwrap()
        .iter()
        .map(|f| f["id"].as_str().unwrap())
        .collect();
    assert_eq!(ids.len(), json["findings"].as_array().unwrap().len());
    for page in json["pages"].as_array().unwrap() {
        assert_url(&f.report(), page["url"].as_str().unwrap());
    }
    for finding in json["findings"].as_array().unwrap() {
        assert_url(&f.report(), finding["url"].as_str().unwrap());
        if let Some(loc) = finding.get("location") {
            assert_url(&f.report(), loc["url"].as_str().unwrap());
        }
    }
    assert_success(&f.run(&[
        "--metadata",
        "수집 결과/nested/navigation.json",
        "--",
        "--all-targets",
    ]));
    assert_eq!(
        json,
        read(&path),
        "cached builds retain the same navigation index"
    );
}
#[test]
fn metadata_preserves_failed_and_clean_inspection_states() {
    for (fixture, success) in [("clean", true), ("compile-error", false)] {
        let f = Fixture::new(fixture);
        let output = f.run(&["--metadata", "navigation.json"]);
        assert_eq!(output.status.success(), success);
        let json = read(&f.root.join("navigation.json"));
        assert_eq!(
            json["execution"]["clippy_exit_code"],
            if success { 0 } else { 101 }
        );
        if success {
            assert!(json["findings"].as_array().unwrap().is_empty());
        }
        for finding in json["findings"].as_array().unwrap() {
            assert_url(&f.report(), finding["url"].as_str().unwrap());
        }
    }
}

#[test]
fn metadata_conflicts_and_invalid_external_paths_keep_the_previous_report() {
    let f = Fixture::new("warning");
    assert_success(&f.run(&["--metadata", "target/clippy-html/navigation.json"]));
    let original = std::fs::read(f.report().join("index.html")).unwrap();
    let old_metadata = std::fs::read(f.report().join("navigation.json")).unwrap();
    let external = f.root.join("external.json");
    std::fs::write(&external, "old external").unwrap();
    for path in [
        "target/clippy-html/index.html",
        "target/clippy-html/static.files/search-index.js",
        "target/clippy-html/static.files",
        "target/clippy-html/.clippy-html-owned",
        "target/.clippy-html-publish.lock",
        "target/clippy-html",
        "external.json/child.json",
    ] {
        let result = f.run(&["--metadata", path]);
        assert!(!result.status.success(), "accepted {path}");
        assert_eq!(
            original,
            std::fs::read(f.report().join("index.html")).unwrap()
        );
        assert_eq!(
            old_metadata,
            std::fs::read(f.report().join("navigation.json")).unwrap()
        );
        assert_eq!(std::fs::read(&external).unwrap(), b"old external");
    }
    let result = f.run(&["--output", "external.json", "--metadata", "external.json"]);
    assert!(!result.status.success());
    assert_eq!(std::fs::read(external).unwrap(), b"old external");
}

#[test]
fn report_local_metadata_replaces_with_the_report_and_is_absent_without_opt_in() {
    let f = Fixture::new("clean");
    for _ in 0..2 {
        assert_success(&f.run(&["--metadata", "target/clippy-html/索引 space/nested.json"]));
        assert_eq!(
            read(&f.report().join("索引 space/nested.json"))["schema_version"],
            1
        );
    }
    assert_success(&f.run(&[]));
    assert!(!f.report().join("索引 space/nested.json").exists());
}
