//! Public navigation index: basic selection fields and actual report destinations.
//! URLs use the report root supplied by consumers, independently of JSON location.
mod output;
pub use output::MetadataTarget;

use crate::{
    diagnostic::model::Report,
    render::paths::{PageMap, category_key, group_key, relative_href},
    source::{SourceFile, span_matches},
};
use serde::Serialize;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

#[derive(Serialize)]
pub struct NavigationMetadata {
    schema_version: u32,
    generator: Generator,
    execution: Execution,
    packages: Vec<Package>,
    pages: Vec<Page>,
    findings: Vec<Finding>,
}
#[derive(Serialize)]
struct Generator {
    name: &'static str,
    version: &'static str,
}
#[derive(Serialize)]
struct Execution {
    clippy_exit_code: Option<i32>,
    collection_complete: bool,
}
#[derive(Serialize)]
struct Package {
    id: String,
    name: String,
    version: String,
}
#[derive(Serialize)]
struct Page {
    kind: &'static str,
    name: String,
    url: String,
}
#[derive(Serialize)]
struct Finding {
    id: String,
    package_id: String,
    origin: String,
    code: Option<String>,
    level: String,
    category: Option<String>,
    message: String,
    url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    location: Option<Location>,
}
#[derive(Serialize)]
struct Location {
    path: String,
    line: usize,
    column: usize,
    url: String,
}

pub fn build(
    report: &Report,
    sources: &BTreeMap<PathBuf, SourceFile>,
    paths: &PageMap,
) -> NavigationMetadata {
    let root = Path::new("index.html");
    let mut pages = vec![Page {
        kind: "overview",
        name: "Report overview".into(),
        url: "index.html".into(),
    }];
    pages.extend(paths.lint_pages.iter().map(|(key, path)| {
        let finding = report
            .findings
            .iter()
            .find(|f| group_key(f) == *key)
            .unwrap();
        Page {
            kind: "lint",
            name: finding
                .code
                .clone()
                .unwrap_or_else(|| "General diagnostics".into()),
            url: relative_href(root, path, None),
        }
    }));
    pages.extend(paths.category_pages.iter().map(|(key, path)| Page {
        kind: "category",
        name: key.clone(),
        url: relative_href(root, path, None),
    }));
    pages.extend(paths.source_pages.iter().map(|(key, path)| Page {
        kind: "source",
        name: sources[key].display_path.clone(),
        url: relative_href(root, path, None),
    }));
    pages.sort_by(|a, b| (a.kind, &a.name, &a.url).cmp(&(b.kind, &b.name, &b.url)));
    let mut packages: Vec<_> = report
        .project
        .packages
        .iter()
        .map(|p| Package {
            id: p.id.clone(),
            name: p.name.clone(),
            version: p.version.clone(),
        })
        .collect();
    packages.sort_by(|a, b| a.id.cmp(&b.id));
    let mut findings: Vec<_> = report
        .findings
        .iter()
        .map(|f| {
            let anchor = format!("finding-{}", f.id);
            let location = f
                .spans
                .iter()
                .find(|s| s.primary)
                .or_else(|| f.spans.first())
                .map(|s| {
                    let source = &sources[&s.path];
                    let destination = if span_matches(source, s) {
                        format!("L{}", s.line_start)
                    } else {
                        anchor.clone()
                    };
                    Location {
                        path: source.display_path.clone(),
                        line: s.line_start,
                        column: s.column_start,
                        url: relative_href(root, &paths.source_pages[&s.path], Some(&destination)),
                    }
                });
            Finding {
                id: f.id.clone(),
                package_id: f.package_id.clone(),
                origin: f.origin.clone(),
                code: f.code.clone(),
                level: f.level.clone(),
                category: category_key(f).map(str::to_owned),
                message: f.message.clone(),
                url: relative_href(root, &paths.lint_pages[&group_key(f)], Some(&anchor)),
                location,
            }
        })
        .collect();
    findings.sort_by(|a, b| a.id.cmp(&b.id));
    NavigationMetadata {
        schema_version: 1,
        generator: Generator {
            name: "cargo-clippy-html",
            version: env!("CARGO_PKG_VERSION"),
        },
        execution: Execution {
            clippy_exit_code: report.execution.code,
            collection_complete: report.execution.complete,
        },
        packages,
        pages,
        findings,
    }
}

impl NavigationMetadata {
    pub fn json(&self) -> Result<Vec<u8>, String> {
        let mut bytes = serde_json::to_vec_pretty(self).map_err(|e| e.to_string())?;
        bytes.push(b'\n');
        Ok(bytes)
    }
    pub fn validate(&self, root: &Path) -> Result<(), String> {
        let root = root.canonicalize().map_err(|e| e.to_string())?;
        let base =
            url::Url::from_directory_path(&root).map_err(|_| "Invalid metadata report root")?;
        let mut anchors = BTreeMap::new();
        let id_selector = scraper::Selector::parse("[id]").unwrap();
        let hrefs = self
            .pages
            .iter()
            .map(|p| p.url.as_str())
            .chain(self.findings.iter().flat_map(|f| {
                std::iter::once(f.url.as_str()).chain(f.location.iter().map(|l| l.url.as_str()))
            }));
        for href in hrefs {
            let linked = base.join(href).map_err(|e| e.to_string())?;
            let path = linked
                .to_file_path()
                .map_err(|_| "Nonlocal metadata URL")?
                .canonicalize()
                .map_err(|e| format!("Broken metadata URL {href}: {e}"))?;
            if !path.starts_with(&root) || !path.is_file() {
                return Err(format!("Broken metadata URL: {href}"));
            }
            if let Some(id) = linked.fragment() {
                if !anchors.contains_key(&path) {
                    let html = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
                    let doc = scraper::Html::parse_document(&html);
                    let ids: std::collections::BTreeSet<String> = doc
                        .select(&id_selector)
                        .filter_map(|e| e.value().id().map(str::to_owned))
                        .collect();
                    anchors.insert(path.clone(), ids);
                }
                if !anchors[&path].contains(id) {
                    return Err(format!("Missing metadata anchor: {href}"));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::{self, model};
    fn report() -> (Report, BTreeMap<PathBuf, SourceFile>) {
        let cargo_metadata::Message::CompilerMessage(message) =
            serde_json::from_value(diagnostic::sample()).unwrap()
        else {
            panic!()
        };
        let mut finding = diagnostic::normalize::normalize(&message, Path::new("/demo"));
        finding.id = "sample".into();
        finding.message = "untrusted </script>&한글".into();
        finding.spans[0].path = PathBuf::from("/demo/src/한글 #.rs");
        let sources = BTreeMap::from([(
            finding.spans[0].path.clone(),
            SourceFile {
                display_path: "src/한글 #.rs".into(),
                text: Some("abc".into()),
                reason: None,
            },
        )]);
        (
            Report {
                category_metadata: None,
                project: model::ProjectInfo {
                    workspace_root: "/demo".into(),
                    target_dir: "/demo/target".into(),
                    packages: vec![],
                },
                execution: model::Execution {
                    code: Some(0),
                    complete: true,
                    requested_args: vec![],
                    observed_packages: vec![],
                    observed_targets: vec![],
                    stderr: String::new(),
                    stderr_truncated: false,
                    text_stdout: String::new(),
                    stdout_truncated: false,
                    issues: vec![],
                },
                raw_count: 1,
                findings: vec![finding],
            },
            sources,
        )
    }
    #[test]
    fn navigation_uses_validated_line_or_fallback_anchors_and_omits_absent_locations() {
        let (mut report, mut sources) = report();
        let pages = crate::render::paths::build_page_map(&report, &sources);
        let metadata = build(&report, &sources, &pages);
        assert!(
            metadata.findings[0]
                .location
                .as_ref()
                .unwrap()
                .url
                .ends_with("#L1")
        );
        let dir = tempfile::tempdir().unwrap();
        crate::render::render_report(&report, &sources, &pages, dir.path()).unwrap();
        metadata.validate(dir.path()).unwrap();
        assert!(
            metadata.findings[0]
                .location
                .as_ref()
                .unwrap()
                .url
                .contains("%")
        );
        for text in [None, Some("changed".into())] {
            sources.values_mut().next().unwrap().text = text;
            let metadata = build(&report, &sources, &pages);
            assert!(
                metadata.findings[0]
                    .location
                    .as_ref()
                    .unwrap()
                    .url
                    .ends_with("#finding-sample")
            );
            let dir = tempfile::tempdir().unwrap();
            crate::render::render_report(&report, &sources, &pages, dir.path()).unwrap();
            metadata.validate(dir.path()).unwrap();
        }
        report.findings[0].spans.clear();
        report.findings[0].origin = "Rustc".into();
        report.findings[0].code = None;
        report.execution.code = None;
        report.execution.complete = false;
        let paths = crate::render::paths::build_page_map(&report, &sources);
        let json: serde_json::Value =
            serde_json::from_slice(&build(&report, &sources, &paths).json().unwrap()).unwrap();
        assert!(json["findings"][0].get("location").is_none());
        assert!(json["findings"][0]["code"].is_null() && json["findings"][0]["category"].is_null());
        assert!(json["execution"]["clippy_exit_code"].is_null());
        assert_eq!(json["execution"]["collection_complete"], false);
    }
    #[test]
    fn navigation_is_deterministic_and_rejects_broken_destinations() {
        let (mut report, sources) = report();
        let mut extra = report.findings[0].clone();
        extra.id = "another".into();
        extra.category = Some("correctness".into());
        report.findings.push(extra);
        let paths = crate::render::paths::build_page_map(&report, &sources);
        let mut metadata = build(&report, &sources, &paths);
        assert_eq!(metadata.findings[0].id, "another");
        assert_eq!(metadata.findings[1].category.as_deref(), Some("unknown"));
        report.findings.reverse();
        assert_eq!(
            metadata.json().unwrap(),
            build(&report, &sources, &paths).json().unwrap()
        );
        let dir = tempfile::tempdir().unwrap();
        crate::render::render_report(&report, &sources, &paths, dir.path()).unwrap();
        metadata.validate(dir.path()).unwrap();
        metadata.findings[0].url = "index.html#not-present".into();
        assert!(
            metadata
                .validate(dir.path())
                .unwrap_err()
                .contains("Missing metadata anchor")
        );
        metadata.findings[0].url = "not-present.html".into();
        assert!(
            metadata
                .validate(dir.path())
                .unwrap_err()
                .contains("Broken metadata URL")
        );
    }
}
