//! Category counts and pages reuse the report's existing page shell and tables.
use super::*;

pub(super) struct CategoryRow {
    pub name: String,
    pub href: String,
    pub count: usize,
    pub kinds: usize,
}
#[derive(Template)]
#[template(path = "category.html")]
struct CategoryView {
    count: usize,
    lints: Vec<CountLink>,
}

fn groups(report: &Report) -> BTreeMap<&str, Vec<&Finding>> {
    let mut groups = BTreeMap::<_, Vec<_>>::new();
    for finding in &report.findings {
        if let Some(category) = super::super::paths::category_key(finding) {
            groups.entry(category).or_default().push(finding);
        }
    }
    groups
}
pub(super) fn rows(report: &Report, pages: &PageMap, from: &Path) -> Vec<CategoryRow> {
    groups(report)
        .into_iter()
        .map(|(key, findings)| CategoryRow {
            name: if key == "unknown" {
                "Unknown".into()
            } else {
                key.into()
            },
            href: relative_href(from, &pages.category_pages[key], None),
            count: findings.len(),
            kinds: findings
                .iter()
                .map(|f| group_key(f))
                .collect::<std::collections::BTreeSet<_>>()
                .len(),
        })
        .collect()
}
pub(super) fn write(report: &Report, pages: &PageMap, destination: &Path) -> Result<(), String> {
    for (key, findings) in groups(report) {
        let path = &pages.category_pages[key];
        let mut kinds = BTreeMap::<_, Vec<_>>::new();
        for finding in &findings {
            kinds.entry(group_key(finding)).or_default().push(*finding);
        }
        let lints: Vec<_> = kinds
            .iter()
            .map(|(key, fs)| CountLink {
                link: Link {
                    href: relative_href(path, &pages.lint_pages[key], None),
                    text: fs[0]
                        .code
                        .clone()
                        .unwrap_or_else(|| "General Clippy diagnostic".into()),
                },
                count: fs.len(),
            })
            .collect();
        let sidebar = lints
            .iter()
            .map(|row| Link {
                href: row.link.href.clone(),
                text: row.link.text.clone(),
            })
            .collect();
        write_page(
            destination,
            path,
            Page {
                title: if key == "unknown" { "Unknown" } else { key },
                section: "Diagnostic kinds",
                root: relative_href(path, Path::new(""), None),
                sidebar,
                content: render(&CategoryView {
                    count: findings.len(),
                    lints,
                })?,
                crate_context: crate_context(report, &findings),
                crates: crate_infos(report, &findings),
                heading_kind: "Category",
                source_page: false,
                standalone_page: false,
                source_tree: String::new(),
            },
        )?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn unknown_and_non_clippy_findings_have_distinct_reachable_groups() {
        let cargo_metadata::Message::CompilerMessage(message) =
            serde_json::from_value(crate::diagnostic::sample()).unwrap()
        else {
            panic!()
        };
        let mut known = crate::diagnostic::normalize::normalize(&message, Path::new("/demo"));
        let mut other_target = known.clone();
        other_target.targets[0].name = "another-target".into();
        let mut unknown = known.clone();
        unknown.code = Some("clippy::new_future_lint".into());
        let mut compiler = known.clone();
        compiler.origin = "Compiler".into();
        compiler.code = Some("E0308".into());
        let mut general = compiler.clone();
        general.origin = "General".into();
        general.code = None;
        let mut findings = crate::diagnostic::dedup::merge_findings(vec![
            known.clone(),
            other_target,
            unknown,
            compiler,
            general,
        ]);
        for finding in &mut findings {
            if finding.code.as_deref() == known.code.as_deref() {
                finding.category = Some("complexity".into());
            }
        }
        known = findings
            .iter()
            .find(|f| f.code == known.code)
            .unwrap()
            .clone();
        assert_eq!(known.targets.len(), 2);
        let report = Report {
            category_metadata: None,
            project: ProjectInfo {
                workspace_root: "/demo".into(),
                target_dir: "/demo/target".into(),
                packages: vec![],
            },
            execution: Execution {
                code: Some(101),
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
            raw_count: 5,
            findings,
        };
        let sources = BTreeMap::from([(
            known.spans[0].path.clone(),
            SourceFile {
                display_path: "src/lib.rs".into(),
                text: Some("abc\n".into()),
                reason: None,
            },
        )]);
        let pages = super::super::super::paths::build_page_map(&report, &sources);
        assert_eq!(pages.category_pages.len(), 2);
        let rows = rows(&report, &pages, Path::new("index.html"));
        assert_eq!(rows.iter().map(|r| r.count).sum::<usize>(), 2);
        assert!(rows.iter().all(|r| r.count == 1 && r.kinds == 1));
        let output = tempfile::tempdir().unwrap();
        render_report(&report, &sources, &pages, output.path()).unwrap();
        validate_report(output.path()).unwrap();
        let unknown = std::fs::read_to_string(output.path().join("category/unknown.html")).unwrap();
        assert!(unknown.contains("new_future_lint"));
        assert!(!unknown.contains("E0308"));
        assert!(!unknown.contains("General diagnostics"));
    }
}
