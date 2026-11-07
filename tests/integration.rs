mod support;
use scraper::{Html, Selector};
use support::*;
#[test]
fn categories_link_to_lints_sources_and_record_the_bundled_origin() {
    let f = Fixture::new("warning");
    assert_success(&f.run(&[]));
    let category = f.report().join("category/complexity.html");
    assert!(
        category.is_file(),
        "known Clippy category should have a page"
    );
    let doc = Html::parse_document(&f.html("category/complexity.html"));
    assert!(
        doc.select(&Selector::parse("a").unwrap())
            .any(|a| a.value().attr("href") == Some("../lint/clippy/clone_on_copy.html"))
    );
    let lint = Html::parse_document(&f.html("lint/clippy/clone_on_copy.html"));
    assert!(
        lint.select(&Selector::parse("a").unwrap())
            .any(|a| a.value().attr("href") == Some("../../category/complexity.html"))
    );
    let index = f.html("index.html");
    assert!(index.contains("category/complexity.html"));
    assert!(index.contains("bundled"));
    assert!(index.contains("rust-1.97.0"));
    assert!(index.contains("Explicit tool --offline"));
    assert!(!f.report().join("category/unknown.html").exists());
    assert!(f.html("licenses/index.html").contains("Clippy category"));
}
#[test]
fn multiline_highlighting_preserves_source_text_without_creating_html() {
    let f = Fixture::new("warning");
    let source = "#![warn(clippy::clone_on_copy)]\n#[allow(dead_code)]\npub fn copy_number(value: i32) -> i32 {\n    let _ = r##\"</span><script>alert('bad')</script>&\nfn fake() {}\"##;\n    /* outer /* nested */\n       still a comment */\n    value.clone()\n}\n";
    std::fs::write(f.root.join("src/lib.rs"), source).unwrap();
    assert_success(&f.run(&[]));
    let path = html_files(&f.report().join("src")).pop().unwrap();
    let doc = Html::parse_document(&std::fs::read_to_string(path).unwrap());
    let text = doc
        .select(&Selector::parse(".line-text").unwrap())
        .map(|line| line.text().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(text, source);
    assert_eq!(
        doc.select(&Selector::parse("script,[onclick],[onerror]").unwrap())
            .count(),
        0
    );
    let keywords: Vec<_> = doc
        .select(&Selector::parse(".line-text .kw").unwrap())
        .map(|span| span.text().collect::<String>())
        .collect();
    assert_eq!(
        keywords.iter().filter(|text| text.as_str() == "fn").count(),
        1
    );
    assert!(
        doc.select(&Selector::parse(".line-text .string").unwrap())
            .any(|span| span.text().collect::<String>().contains("fn fake"))
    );
    assert!(
        doc.select(&Selector::parse(".line-text .comment").unwrap())
            .any(|span| span.text().collect::<String>().contains("still a comment"))
    );
    assert!(
        doc.select(&Selector::parse("#L8.diagnostic-line .diagnostic-marker").unwrap())
            .any(|link| link
                .text()
                .collect::<String>()
                .contains("clippy::clone_on_copy"))
    );
}
#[test]
fn reports_bundle_original_fonts_and_separate_licenses() {
    let f = Fixture::new("clean");
    assert_success(&f.run(&[]));
    let names = [
        "FiraSans-Regular-0fe48ade.woff2",
        "FiraSans-Italic-81dc35de.woff2",
        "FiraSans-Medium-e1aa3f0a.woff2",
        "FiraSans-MediumItalic-ccf7e434.woff2",
        "SourceSerif4-Regular-6b053e98.ttf.woff2",
        "SourceSerif4-It-ca3b17ed.ttf.woff2",
        "SourceSerif4-Semibold-457a13ac.ttf.woff2",
        "SourceSerif4-Bold-6d4fd4c0.ttf.woff2",
        "SourceCodePro-Regular-8badfe75.ttf.woff2",
        "SourceCodePro-It-fc8b9304.ttf.woff2",
        "SourceCodePro-Semibold-aa29a496.ttf.woff2",
        "NanumBarunGothic-13b3dcba.ttf.woff2",
    ];
    let mut total = 0;
    for name in names {
        let bytes =
            std::fs::read(f.report().join("static.files").join(name)).expect("bundled WOFF2");
        assert_eq!(&bytes[..4], b"wOF2");
        assert_eq!(
            bytes,
            std::fs::read(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/vendor/rustdoc/fonts")
                    .join(name)
            )
            .unwrap()
        );
        total += bytes.len();
    }
    assert_eq!(total, 1_386_044);
    let css = f.html("static.files/fonts.css");
    assert_eq!(css.matches("@font-face").count(), 12);
    assert!(!css.contains("Fira Mono"));
    for name in [
        "Fira-LICENSE.txt",
        "SourceSerif4-LICENSE.md",
        "SourceCodePro-LICENSE.txt",
        "NanumBarunGothic-LICENSE.txt",
    ] {
        let license = f.html(&format!("licenses/{name}"));
        assert!(license.contains("SIL OPEN FONT LICENSE Version 1.1"));
        assert!(license.contains("Reserved Font Name"));
        assert_eq!(
            license.as_bytes(),
            std::fs::read(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                    .join("assets/vendor/rustdoc/fonts")
                    .join(name)
            )
            .unwrap()
        );
    }
    let doc = Html::parse_document(&f.html("index.html"));
    assert!(
        doc.select(&Selector::parse("link[rel=stylesheet]").unwrap())
            .any(|link| link.value().attr("href") == Some("static.files/fonts.css"))
    );
    assert!(f.html("licenses/index.html").contains("OFL-1.1"));
    assert!(
        f.html("licenses/generator-MIT.txt")
            .contains("Imagination12357")
    );
    assert!(
        f.html("licenses/THIRD-PARTY-NOTICES.md")
            .contains("OFL-1.1")
    );
}
#[test]
fn excluded_nested_dependency_is_snippet_only() {
    let f = Fixture::new("nested-dependency");
    assert_eq!(f.run(&[]).status.code(), Some(101));
    let files = html_files(&f.report().join("src"));
    assert!(!files.is_empty());
    for path in files {
        let html = std::fs::read_to_string(path).unwrap();
        let doc = Html::parse_document(&html);
        assert_eq!(
            doc.select(&Selector::parse(".code-line").unwrap()).count(),
            0
        );
        assert!(!html.contains("DEPENDENCY_FULL_SOURCE_SENTINEL"));
        assert_eq!(
            doc.select(&Selector::parse(".source-tree a[aria-current=\"page\"]").unwrap())
                .count(),
            1
        );
        assert!(
            doc.select(&Selector::parse(".source-findings a").unwrap())
                .all(|a| a.value().attr("href").unwrap().starts_with("#finding-"))
        );
    }
}
#[test]
fn clean_cached_and_uncached_reports() {
    let f = Fixture::new("clean");
    for _ in 0..2 {
        assert_success(&f.run(&[]));
        let html = f.html("index.html");
        assert!(html.contains("Succeeded"));
        assert!(html.contains("0 unique"));
        assert!(html.contains("Complete"));
        assert!(html.contains("fixture-clean"));
    }
    assert!(!f.report().join("metadata.json").exists());
    assert!(f.report().join("licenses/rustdoc-MIT.txt").exists());
}
#[test]
fn warning_source_and_suggestion_are_escaped() {
    let f = Fixture::new("warning");
    assert_success(&f.run(&[]));
    let lint = html_files(&f.report().join("lint"));
    assert_eq!(lint.len(), 1);
    assert!(f.report().join("lint/clippy/clone_on_copy.html").is_file());
    let doc = Html::parse_document(&std::fs::read_to_string(&lint[0]).unwrap());
    assert_eq!(doc.select(&Selector::parse(".finding").unwrap()).count(), 1);
    assert!(
        doc.root_element()
            .text()
            .collect::<String>()
            .contains("MachineApplicable")
    );
    assert!(
        doc.select(&Selector::parse(".diagnostic-note a").unwrap())
            .any(|link| {
                link.value().attr("href").is_some_and(|href| {
                    href.starts_with("https://rust-lang.github.io/rust-clippy/rust-")
                        && href.ends_with("/index.html#clone_on_copy")
                })
            }),
        "Clippy's version-specific further-information URL must be clickable"
    );
    let src = html_files(&f.report().join("src"));
    assert_eq!(src.len(), 1);
    assert!(f.report().join("src/lib.rs.html").is_file());
    let doc = Html::parse_document(&std::fs::read_to_string(&src[0]).unwrap());
    assert_eq!(
        doc.select(
            &Selector::parse(
                "#src-sidebar .dir-entry > .files > a.selected[aria-current=\"page\"]"
            )
            .unwrap()
        )
        .count(),
        1
    );
    assert_eq!(
        doc.select(&Selector::parse("#src-sidebar ul, #src-sidebar li").unwrap())
            .count(),
        0
    );
    assert_eq!(
        doc.select(&Selector::parse("#main-content > p > code").unwrap())
            .count(),
        0
    );
    assert_eq!(
        doc.select(&Selector::parse(".sidebar-elems #src-sidebar").unwrap())
            .count(),
        0
    );
    let active = doc
        .select(&Selector::parse(".source-tree a[aria-current=\"page\"]").unwrap())
        .next()
        .expect("active source file");
    assert!(active.text().collect::<String>().contains("lib.rs"));
    assert!(active.text().collect::<String>().contains("1"));
    let finding = doc
        .select(&Selector::parse(".source-findings a").unwrap())
        .next()
        .expect("finding line navigation");
    assert_eq!(finding.value().attr("href"), Some("#L3"));
    assert_eq!(
        doc.select(&Selector::parse("script,[onclick],[onerror]").unwrap())
            .count(),
        0
    );
    assert!(
        doc.root_element()
            .text()
            .collect::<String>()
            .contains("<script>")
    );
    assert_eq!(
        doc.select(&Selector::parse(".code-line").unwrap()).count(),
        4
    );
    assert!(
        doc.select(&Selector::parse(".line-text .kw").unwrap())
            .any(|span| span.text().collect::<String>() == "fn")
    );
    assert!(
        doc.select(&Selector::parse(".line-text .doccomment").unwrap())
            .any(|span| span.text().collect::<String>().contains("<script>"))
    );
    let rendered_source = doc
        .select(&Selector::parse(".line-text").unwrap())
        .map(|line| line.text().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(
        rendered_source,
        std::fs::read_to_string(f.root.join("src/lib.rs"))
            .unwrap()
            .replace("\r\n", "\n")
    );
}
#[test]
fn lint_and_compile_failure_keep_reports_and_exit_codes() {
    let f = Fixture::new("warning");
    let out = f.run(&["--", "--", "-D", "warnings"]);
    assert_eq!(
        out.status.code(),
        Some(101),
        "{}",
        String::from_utf8_lossy(&out.stderr)
    );
    assert!(f.html("index.html").contains("Failed"));
    assert!(f.html("index.html").contains("1 errors"));
    let e = Fixture::new("compile-error");
    let out = e.run(&[]);
    assert_eq!(out.status.code(), Some(101));
    assert!(e.html("index.html").contains("Failed"));
    assert!(e.report().join("lint/rustc/E0308.html").is_file());
    assert!(!html_files(&e.report().join("lint")).is_empty());
}
#[test]
fn workspace_paths_unicode_and_multi_target_dedup() {
    let f = Fixture::new("workspace");
    assert_success(&f.run(&["--", "--workspace", "--all-targets"]));
    let html = f.html("index.html");
    assert!(html.contains("fixture-one"));
    assert!(html.contains("fixture-two"));
    let doc = Html::parse_document(&html);
    let crate_entries: Vec<_> = doc
        .select(&Selector::parse(".sidebar-crate .crate-info").unwrap())
        .map(|entry| entry.text().collect::<String>())
        .collect();
    assert!(
        crate_entries
            .iter()
            .any(|entry| entry.contains("fixture-one") && entry.contains("0.1.0"))
    );
    assert!(
        crate_entries
            .iter()
            .any(|entry| entry.contains("fixture-two") && entry.contains("0.2.0"))
    );
    assert_eq!(html_files(&f.report().join("src")).len(), 3);
    assert!(f.report().join("src/one/src/lib.rs.html").is_file());
    assert!(f.report().join("src/one/src/한글.rs.html").is_file());
    assert!(!html.contains("one/one/"));
    for file in html_files(&f.report().join("src")) {
        let source = std::fs::read_to_string(file).unwrap();
        let doc = Html::parse_document(&source);
        assert!(
            doc.select(
                &Selector::parse("#src-sidebar .dir-entry > summary + .folders + .files").unwrap()
            )
            .count()
                >= 2
        );
        assert_eq!(
            doc.select(&Selector::parse(".source-tree a[aria-current=\"page\"]").unwrap())
                .count(),
            1
        );
        assert!(
            doc.select(&Selector::parse(".source-tree details[open]").unwrap())
                .count()
                >= 2
        );
        assert!(
            doc.select(&Selector::parse(".source-tree details:not([open])").unwrap())
                .count()
                >= 1
        );
        assert!(
            doc.select(&Selector::parse(".code-line").unwrap()).count() > 0,
            "workspace source unexpectedly fell back"
        );
    }
    let lint = html_files(&f.report().join("lint"));
    assert_eq!(lint.len(), 1);
    let doc = Html::parse_document(&std::fs::read_to_string(&lint[0]).unwrap());
    assert_eq!(doc.select(&Selector::parse(".finding").unwrap()).count(), 5);
}
#[test]
fn output_and_target_dir_paths_follow_invocation_directory() {
    let f = Fixture::new("clean");
    assert_success(&f.run(&["--output", "report space", "--", "--target-dir", "cache"]));
    assert!(f.root.join("report space/index.html").is_file());
    assert!(!f.root.join("cache/clippy-html").exists());
    assert_success(&f.run(&["--", "--target-dir=cache"]));
    assert!(f.root.join("cache/clippy-html/index.html").is_file());
    let elsewhere = f.temp.path().join("elsewhere");
    std::fs::create_dir(&elsewhere).unwrap();
    let out = std::process::Command::new(env!("CARGO_BIN_EXE_cargo-clippy-html"))
        .current_dir(&elsewhere)
        .args(["--output", "relative-report", "--", "--manifest-path"])
        .arg(f.root.join("Cargo.toml"))
        .output()
        .unwrap();
    assert_success(&out);
    assert!(elsewhere.join("relative-report/index.html").exists());
}
#[test]
fn refuses_metadata_and_preserves_unowned_directory() {
    let f = Fixture::new("clean");
    assert_eq!(f.run(&["--metadata", "x.json"]).status.code(), Some(2));
    assert!(!f.root.join("x.json").exists());
    let folder = f.root.join("notes");
    std::fs::create_dir(&folder).unwrap();
    std::fs::write(folder.join("important.txt"), "keep").unwrap();
    assert!(!f.run(&["--output", "notes"]).status.success());
    assert_eq!(
        std::fs::read_to_string(folder.join("important.txt")).unwrap(),
        "keep"
    );
}
