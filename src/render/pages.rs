use super::paths::PageMap;
use super::paths::{group_key, relative_href};
use crate::source::{all_spans, span_matches};
use crate::{diagnostic::model::*, source::SourceFile};
use askama::Template;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
struct Link {
    href: String,
    text: String,
}
struct CountLink {
    link: Link,
    count: usize,
}
struct PackageRow {
    name: String,
    observed: &'static str,
}
struct CrateInfo {
    name: String,
    version: String,
}
#[derive(Template)]
#[template(path = "base.html")]
struct Page<'a> {
    title: &'a str,
    section: &'a str,
    root: String,
    sidebar: Vec<Link>,
    content: String,
    crate_context: String,
    crates: Vec<CrateInfo>,
    heading_kind: &'a str,
    source_page: bool,
    source_tree: String,
}
#[derive(Template)]
#[template(path = "index.html")]
struct Index<'a> {
    workspace: String,
    status: &'static str,
    exit: String,
    collection: &'static str,
    count: usize,
    errors: usize,
    warnings: usize,
    raw_count: usize,
    args: String,
    issues: &'a [String],
    packages: Vec<PackageRow>,
    targets: Vec<String>,
    lints: Vec<CountLink>,
    files: Vec<Link>,
    stderr: &'a str,
    stderr_truncated: bool,
    stdout: &'a str,
    stdout_truncated: bool,
}
struct Location {
    kind: &'static str,
    href: String,
    label: String,
    note: String,
    snippet: String,
}
struct NoteView {
    level: String,
    message: String,
    link: Option<Link>,
}
fn note_view(note: &Note) -> NoteView {
    const PREFIX: &str = "for further information visit ";
    if let Some(text) = note.message.strip_prefix(PREFIX)
        && let Ok(url) = url::Url::parse(text)
        && url.scheme() == "https"
        && url.host_str().is_some()
        && url.username().is_empty()
        && url.password().is_none()
    {
        NoteView {
            level: note.level.clone(),
            message: PREFIX.into(),
            link: Some(Link {
                href: url.to_string(),
                text: text.into(),
            }),
        }
    } else {
        NoteView {
            level: note.level.clone(),
            message: note.message.clone(),
            link: None,
        }
    }
}
struct ReplacementView {
    location: String,
    applicability: String,
    text: String,
}
struct SuggestionView {
    message: String,
    replacements: Vec<ReplacementView>,
}
struct FindingView {
    id: String,
    level: String,
    code: String,
    message: String,
    origin: String,
    category: String,
    package: String,
    docs: String,
    targets: Vec<String>,
    locations: Vec<Location>,
    notes: Vec<NoteView>,
    suggestions: Vec<SuggestionView>,
    fallback: String,
}
#[derive(Template)]
#[template(path = "lint.html")]
struct Lint {
    findings: Vec<FindingView>,
}
struct SourceRow {
    number: usize,
    text: String,
    marked: bool,
    links: Vec<Link>,
}
#[derive(Template)]
#[template(path = "source.html")]
struct SourceView {
    reason: String,
    rows: Vec<SourceRow>,
    digits: usize,
}

fn source_rows(
    source: &SourceFile,
    key: &Path,
    path: &Path,
    findings: &[&Finding],
    pages: &PageMap,
    validate: &impl Fn(&SourceFile, &Span) -> bool,
) -> Vec<SourceRow> {
    let Some(text) = source.text.as_deref() else {
        return vec![];
    };
    let mut rows: Vec<_> = text
        .split('\n')
        .enumerate()
        .map(|(i, line)| SourceRow {
            number: i + 1,
            text: line.trim_end_matches('\r').into(),
            marked: false,
            links: vec![],
        })
        .collect();
    let mut assigned = std::collections::BTreeSet::new();
    for finding in findings {
        for span in display_spans(finding).iter().filter(|s| s.path == key) {
            if !validate(source, span) {
                continue;
            }
            for number in span.line_start..=span.line_end {
                if let Some(row) = rows.get_mut(number - 1)
                    && assigned.insert((number, &finding.id))
                {
                    row.marked = true;
                    row.links.push(Link {
                        href: relative_href(
                            path,
                            &pages.lint_pages[&group_key(finding)],
                            Some(&format!("finding-{}", finding.id)),
                        ),
                        text: format!(
                            "{}: {}",
                            finding.level,
                            finding.code.as_deref().unwrap_or("diagnostic")
                        ),
                    });
                }
            }
        }
    }
    rows
}
fn render(t: &impl Template) -> Result<String, String> {
    t.render().map_err(|e| e.to_string())
}
fn write_page(destination: &Path, path: &Path, page: Page<'_>) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(destination.join(parent)).map_err(|e| e.to_string())?;
    }
    std::fs::write(destination.join(path), render(&page)?).map_err(|e| e.to_string())
}
fn target_label(t: &TargetInfo) -> String {
    format!("{} ({})", t.name, t.kinds.join(", "))
}
fn crate_infos(report: &Report, findings: &[&Finding]) -> Vec<CrateInfo> {
    let mut crates: Vec<_> = report
        .project
        .packages
        .iter()
        .filter(|p| {
            p.workspace_member
                && (findings.is_empty() || findings.iter().any(|f| f.package_id == p.id))
        })
        .map(|p| CrateInfo {
            name: p.name.clone(),
            version: p.version.clone(),
        })
        .collect();
    crates.sort_by(|a, b| a.name.cmp(&b.name).then(a.version.cmp(&b.version)));
    crates
}
fn crate_context(report: &Report, findings: &[&Finding]) -> String {
    let crates = crate_infos(report, findings);
    let names: Vec<_> = crates.iter().map(|c| c.name.as_str()).collect();
    if names.is_empty() {
        report
            .project
            .workspace_root
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .into_owned()
    } else {
        names.join(", ")
    }
}
fn display_spans(f: &Finding) -> Vec<Span> {
    let mut seen: BTreeMap<(PathBuf, usize, usize), usize> = BTreeMap::new();
    let mut result: Vec<Span> = vec![];
    for span in all_spans(f) {
        let key = (span.path.clone(), span.byte_start, span.byte_end);
        if let Some(&index) = seen.get(&key) {
            if let Some(label) = &span.label {
                let existing = &mut result[index].label;
                if existing.as_deref() != Some(label) {
                    *existing = Some(
                        existing
                            .as_ref()
                            .map_or_else(|| label.clone(), |text| format!("{text}; {label}")),
                    );
                }
            }
        } else {
            seen.insert(key, result.len());
            result.push(span.clone());
        }
    }
    result
}
fn views(
    findings: &[&Finding],
    from: &Path,
    sources: &BTreeMap<PathBuf, SourceFile>,
    pages: &PageMap,
    packages: &[PackageInfo],
) -> Vec<FindingView> {
    fn flatten(n: &Note, out: &mut Vec<NoteView>) {
        out.push(note_view(n));
        for c in &n.children {
            flatten(c, out);
        }
    }
    findings
        .iter()
        .map(|f| {
            let locations = display_spans(f)
                .into_iter()
                .map(|s| {
                    let matched = sources
                        .get(&s.path)
                        .is_some_and(|src| span_matches(src, &s));
                    let anchor = if matched {
                        format!("L{}", s.line_start)
                    } else {
                        format!("finding-{}", f.id)
                    };
                    Location {
                        kind: if s.primary { "Primary" } else { "Secondary" },
                        href: relative_href(from, &pages.source_pages[&s.path], Some(&anchor)),
                        label: format!(
                            "{}:{}:{}",
                            sources[&s.path].display_path, s.line_start, s.column_start
                        ),
                        note: format!(
                            "{}{}",
                            s.label.as_deref().unwrap_or(""),
                            if matched {
                                ""
                            } else {
                                " [Diagnostic snippet; full source unavailable or changed]"
                            }
                        ),
                        snippet: s
                            .text
                            .iter()
                            .map(|t| t.text.as_str())
                            .collect::<Vec<_>>()
                            .join("\n"),
                    }
                })
                .collect();
            let mut notes = vec![];
            for n in &f.children {
                flatten(n, &mut notes);
            }
            let docs = f
                .code
                .as_deref()
                .and_then(|c| c.strip_prefix("clippy::"))
                .map(|name| {
                    let mut url = url::Url::parse(
                        "https://rust-lang.github.io/rust-clippy/master/index.html",
                    )
                    .expect("constant URL");
                    url.set_fragment(Some(name));
                    url.to_string()
                })
                .unwrap_or_default();
            FindingView {
                id: f.id.clone(),
                level: f.level.clone(),
                code: f
                    .code
                    .clone()
                    .unwrap_or_else(|| "General diagnostic".into()),
                message: f.message.clone(),
                origin: f.origin.clone(),
                category: if f.origin == "Clippy" {
                    "Category: Unknown"
                } else {
                    "Category: not applicable"
                }
                .into(),
                package: packages
                    .iter()
                    .find(|p| p.id == f.package_id)
                    .map_or_else(|| f.package_id.clone(), |p| p.name.clone()),
                docs,
                targets: f.targets.iter().map(target_label).collect(),
                locations,
                notes,
                suggestions: f
                    .suggestions
                    .iter()
                    .map(|s| SuggestionView {
                        message: s.message.clone(),
                        replacements: s
                            .replacements
                            .iter()
                            .map(|r| ReplacementView {
                                location: format!(
                                    "{}:{}:{}",
                                    r.span.path.display(),
                                    r.span.line_start,
                                    r.span.column_start
                                ),
                                applicability: r.applicability.clone(),
                                text: r.text.clone(),
                            })
                            .collect(),
                    })
                    .collect(),
                fallback: f.rendered.clone().unwrap_or_default(),
            }
        })
        .collect()
}
pub fn render_report(
    report: &Report,
    sources: &BTreeMap<PathBuf, SourceFile>,
    pages: &PageMap,
    destination: &Path,
) -> Result<(), String> {
    for dir in ["static.files", "licenses", "lint", "src"] {
        std::fs::create_dir_all(destination.join(dir)).map_err(|e| e.to_string())?;
    }
    for (path, bytes) in [
        (
            "static.files/rustdoc.css",
            include_str!("../../assets/vendor/rustdoc/rustdoc.css"),
        ),
        (
            "static.files/normalize.css",
            include_str!("../../assets/vendor/rustdoc/normalize.css"),
        ),
        (
            "static.files/clippy-html.css",
            include_str!("../../assets/clippy-html.css"),
        ),
        (
            "licenses/rustdoc-MIT.txt",
            include_str!("../../assets/vendor/rustdoc/LICENSE-MIT"),
        ),
        (
            "licenses/rustdoc-COPYRIGHT.txt",
            include_str!("../../assets/vendor/rustdoc/COPYRIGHT.txt"),
        ),
        (
            "licenses/normalize-MIT.txt",
            include_str!("../../assets/vendor/rustdoc/normalize-LICENSE-MIT"),
        ),
        (
            "licenses/UPSTREAM.md",
            include_str!("../../assets/vendor/rustdoc/UPSTREAM.md"),
        ),
    ] {
        std::fs::write(destination.join(path), bytes).map_err(|e| e.to_string())?;
    }
    let groups: BTreeMap<String, Vec<&Finding>> =
        report.findings.iter().fold(BTreeMap::new(), |mut map, f| {
            map.entry(group_key(f)).or_default().push(f);
            map
        });
    let index_path = Path::new("index.html");
    let lints = groups
        .iter()
        .map(|(key, fs)| CountLink {
            link: Link {
                href: relative_href(index_path, &pages.lint_pages[key], None),
                text: fs[0]
                    .code
                    .clone()
                    .unwrap_or_else(|| "General diagnostics".into()),
            },
            count: fs.len(),
        })
        .collect();
    let files: Vec<_> = sources
        .iter()
        .map(|(key, s)| Link {
            href: relative_href(index_path, &pages.source_pages[key], None),
            text: s.display_path.clone(),
        })
        .collect();
    let index = Index {
        workspace: {
            let members: Vec<_> = report
                .project
                .packages
                .iter()
                .filter(|p| p.workspace_member)
                .collect();
            if members.len() == 1 {
                members[0].name.clone()
            } else {
                report
                    .project
                    .workspace_root
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned()
            }
        },
        status: if report.execution.code == Some(0) {
            "Succeeded"
        } else {
            "Failed"
        },
        exit: report
            .execution
            .code
            .map_or("unavailable".into(), |n| n.to_string()),
        collection: if report.execution.complete {
            "Complete"
        } else {
            "Incomplete — inspect collection issues"
        },
        count: report.findings.len(),
        errors: report
            .findings
            .iter()
            .filter(|f| f.level == "error")
            .count(),
        warnings: report
            .findings
            .iter()
            .filter(|f| f.level == "warning")
            .count(),
        raw_count: report.raw_count,
        args: report
            .execution
            .requested_args
            .iter()
            .map(|a| format!("{a:?}"))
            .collect::<Vec<_>>()
            .join(" "),
        issues: &report.execution.issues,
        packages: report
            .project
            .packages
            .iter()
            .filter(|p| p.workspace_member)
            .map(|p| PackageRow {
                name: p.name.clone(),
                observed: if report.execution.observed_packages.contains(&p.id) {
                    "Yes"
                } else {
                    "Not observed; not inferred from warnings"
                },
            })
            .collect(),
        targets: report
            .execution
            .observed_targets
            .iter()
            .map(target_label)
            .collect(),
        lints,
        files,
        stderr: &report.execution.stderr,
        stderr_truncated: report.execution.stderr_truncated,
        stdout: &report.execution.text_stdout,
        stdout_truncated: report.execution.stdout_truncated,
    };
    let sidebar = vec![
        Link {
            href: "#packages".into(),
            text: "Workspace packages".into(),
        },
        Link {
            href: "#lints".into(),
            text: "Diagnostic kinds".into(),
        },
        Link {
            href: "#files".into(),
            text: "Source files".into(),
        },
    ];
    write_page(
        destination,
        index_path,
        Page {
            title: &index.workspace,
            section: "Overview",
            root: String::new(),
            sidebar,
            content: render(&index)?,
            crate_context: crate_context(report, &[]),
            crates: crate_infos(report, &[]),
            heading_kind: "Report",
            source_page: false,
            source_tree: String::new(),
        },
    )?;
    for (key, fs) in &groups {
        let path = &pages.lint_pages[key];
        let title = fs[0].code.as_deref().unwrap_or("General diagnostics");
        let sidebar = fs
            .iter()
            .map(|f| Link {
                href: format!("#finding-{}", f.id),
                text: format!(
                    "{} · {}",
                    f.level,
                    f.spans
                        .first()
                        .map_or("No source location".into(), |s| format!(
                            "{}:{}",
                            sources[&s.path].display_path, s.line_start
                        ))
                ),
            })
            .collect();
        write_page(
            destination,
            path,
            Page {
                title,
                section: "Occurrences",
                root: "../".into(),
                sidebar,
                content: render(&Lint {
                    findings: views(fs, path, sources, pages, &report.project.packages),
                })?,
                crate_context: crate_context(report, fs),
                crates: crate_infos(report, fs),
                heading_kind: "Lint",
                source_page: false,
                source_tree: String::new(),
            },
        )?;
    }
    let source_counts = super::navigation::source_counts(&report.findings);
    for (key, source) in sources {
        let path = &pages.source_pages[key];
        let fs: Vec<_> = report
            .findings
            .iter()
            .filter(|f| all_spans(f).iter().any(|s| &s.path == key))
            .collect();
        let rows = source_rows(source, key, path, &fs, pages, &span_matches);
        let mut content = render(&SourceView {
            reason: source.reason.clone().unwrap_or_default(),
            digits: rows.len().max(1).to_string().len(),
            rows,
        })?;
        content.push_str(&render(&Lint {
            findings: views(&fs, path, sources, pages, &report.project.packages),
        })?);
        let sidebar = fs
            .iter()
            .map(|f| {
                let spans = display_spans(f);
                let span = spans
                    .iter()
                    .find(|s| s.path == *key && s.primary)
                    .or_else(|| spans.iter().find(|s| s.path == *key));
                let href = span.filter(|s| span_matches(source, s)).map_or_else(
                    || format!("#finding-{}", f.id),
                    |s| format!("#L{}", s.line_start),
                );
                Link {
                    href,
                    text: format!(
                        "{} · {}{}",
                        f.level,
                        f.code.as_deref().unwrap_or("diagnostic"),
                        span.map_or_else(String::new, |s| format!(
                            " · {}:{}",
                            s.line_start, s.column_start
                        ))
                    ),
                }
            })
            .collect();
        write_page(
            destination,
            path,
            Page {
                title: &source.display_path,
                section: "File findings",
                root: relative_href(path, Path::new(""), None),
                sidebar,
                content,
                crate_context: crate_context(report, &fs),
                crates: crate_infos(report, &fs),
                heading_kind: "Source",
                source_page: true,
                source_tree: super::navigation::source_tree(
                    sources,
                    pages,
                    &source_counts,
                    &report.project.workspace_root,
                    key,
                    path,
                )?,
            },
        )?;
    }
    Ok(())
}

// Only inspect generated, double-quoted attributes. All variable text has already
// been escaped by Askama; HTML parser tests independently check this assumption.
fn attributes<'a>(html: &'a str, name: &str) -> Vec<&'a str> {
    let pattern = format!(" {name}=\"");
    html.split(&pattern)
        .skip(1)
        .filter_map(|s| s.split('"').next())
        .collect()
}
pub fn validate_report(destination: &Path) -> Result<(), String> {
    let root = destination.canonicalize().map_err(|e| e.to_string())?;
    let mut docs = BTreeMap::new();
    let mut directories = vec![root.clone()];
    while let Some(dir) = directories.pop() {
        for entry in std::fs::read_dir(dir).map_err(|e| e.to_string())? {
            let path = entry.map_err(|e| e.to_string())?.path();
            if path.is_dir() {
                directories.push(path);
            } else if path.extension().is_some_and(|x| x == "html") {
                let html = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
                let ids = attributes(&html, "id");
                let unique: std::collections::BTreeSet<_> = ids.iter().collect();
                if ids.len() != unique.len() {
                    return Err(format!("Duplicate anchor in {}", path.display()));
                }
                docs.insert(path, html);
            }
        }
    }
    if !docs.contains_key(&root.join("index.html")) {
        return Err("Missing report index".into());
    }
    for (path, html) in &docs {
        let base = url::Url::from_file_path(path).map_err(|_| "Invalid report URL")?;
        for href in attributes(html, "href") {
            if href.starts_with("https://") {
                continue;
            }
            let linked = base.join(href).map_err(|e| e.to_string())?;
            let target = linked
                .to_file_path()
                .map_err(|_| "Nonlocal internal link")?
                .canonicalize()
                .map_err(|e| format!("Broken report link {href}: {e}"))?;
            if !target.starts_with(&root) || !target.is_file() {
                return Err(format!("Broken report link: {href}"));
            }
            if let Some(id) = linked.fragment() {
                let content = docs.get(&target).ok_or("Anchor target is not HTML")?;
                if !attributes(content, "id").contains(&id) {
                    return Err(format!("Missing anchor: {href}"));
                }
            }
        }
    }
    Ok(())
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn validates_links_inside_nested_source_pages() {
        let t = tempfile::tempdir().unwrap();
        std::fs::create_dir_all(t.path().join("src/one/src")).unwrap();
        std::fs::write(t.path().join("index.html"), "<h1>Report</h1>").unwrap();
        std::fs::write(
            t.path().join("src/one/src/lib.rs.html"),
            "<a href=\"#missing\">broken</a>",
        )
        .unwrap();
        assert!(
            validate_report(t.path())
                .unwrap_err()
                .contains("Missing anchor")
        );
    }
    #[test]
    fn diagnostic_document_links_keep_version_and_reject_unsafe_schemes() {
        let make_note = |text: &str| Note {
            level: "help".into(),
            message: format!("for further information visit {text}"),
            spans: vec![],
            children: vec![],
        };
        let url = "https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#clone_on_copy";
        assert_eq!(note_view(&make_note(url)).link.unwrap().href, url);
        for text in [
            "javascript:alert(1)",
            "file:///C:/secret.txt",
            "https://user:password@example.com/",
            "not a URL",
        ] {
            let note = make_note(text);
            let view = note_view(&note);
            assert!(view.link.is_none());
            assert_eq!(view.message, note.message);
        }
    }
    #[test]
    fn long_source_validates_each_span_once_and_marks_the_right_row() {
        let cargo_metadata::Message::CompilerMessage(m) =
            serde_json::from_value(crate::diagnostic::sample()).unwrap()
        else {
            panic!()
        };
        let mut f = crate::diagnostic::normalize::normalize(&m, Path::new("/demo"));
        f.id = "one".into();
        let source = SourceFile {
            display_path: "src/lib.rs".into(),
            text: Some("abc\n".repeat(1000)),
            reason: None,
        };
        let pages = PageMap {
            lint_pages: BTreeMap::from([(group_key(&f), PathBuf::from("lint/a.html"))]),
            source_pages: BTreeMap::new(),
        };
        let calls = std::cell::Cell::new(0);
        let rows = source_rows(
            &source,
            &f.spans[0].path,
            Path::new("src/a.html"),
            &[&f],
            &pages,
            &|src, span| {
                calls.set(calls.get() + 1);
                span_matches(src, span)
            },
        );
        assert_eq!(rows.len(), 1001);
        assert!(rows[0].marked);
        assert_eq!(rows.iter().filter(|r| r.marked).count(), 1);
        assert_eq!(
            calls.get(),
            1,
            "source validation must not rescan the file for every output row"
        );
    }
    #[test]
    fn repeated_child_and_replacement_locations_are_presented_once() {
        let mut raw = crate::diagnostic::sample();
        let mut span = raw["message"]["spans"][0].clone();
        span["suggested_replacement"] = "fixed".into();
        span["suggestion_applicability"] = "MachineApplicable".into();
        raw["message"]["children"] = serde_json::json!([{"level":"help","message":"replace it","spans":[span],"children":[],"code":null,"rendered":null}]);
        let cargo_metadata::Message::CompilerMessage(m) = serde_json::from_value(raw).unwrap()
        else {
            panic!()
        };
        let f = crate::diagnostic::normalize::normalize(&m, Path::new("/demo"));
        let sources = BTreeMap::from([(
            f.spans[0].path.clone(),
            SourceFile {
                display_path: "src/lib.rs".into(),
                text: None,
                reason: Some("unavailable".into()),
            },
        )]);
        let pages = PageMap {
            lint_pages: BTreeMap::new(),
            source_pages: BTreeMap::from([(f.spans[0].path.clone(), PathBuf::from("src/a.html"))]),
        };
        let result = views(&[&f], Path::new("lint/a.html"), &sources, &pages, &[]);
        assert_eq!(result[0].locations.len(), 1);
        assert_eq!(result[0].notes.len(), 1);
        assert_eq!(result[0].suggestions.len(), 1);
        assert_eq!(result[0].suggestions[0].replacements.len(), 1);
    }
    #[test]
    fn empty_report_has_success_status() {
        let t = tempfile::tempdir().unwrap();
        let report = Report {
            project: ProjectInfo {
                workspace_root: PathBuf::from("/demo"),
                target_dir: PathBuf::from("/demo/target"),
                packages: vec![],
            },
            execution: Execution {
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
            raw_count: 0,
            findings: vec![],
        };
        let sources = BTreeMap::new();
        let pages = super::super::paths::build_page_map(&report, &sources);
        render_report(&report, &sources, &pages, t.path()).unwrap();
        validate_report(t.path()).unwrap();
        assert!(
            std::fs::read_to_string(t.path().join("index.html"))
                .unwrap()
                .contains("Succeeded")
        );
        let doc = scraper::Html::parse_document(
            &std::fs::read_to_string(t.path().join("index.html")).unwrap(),
        );
        assert_eq!(
            doc.select(
                &scraper::Selector::parse(
                    "main > .width-limiter > section#main-content > .main-heading"
                )
                .unwrap()
            )
            .count(),
            1
        );
    }
    #[test]
    fn hostile_text_and_multiple_findings_have_safe_unique_anchors() {
        let t = tempfile::tempdir().unwrap();
        let cargo_metadata::Message::CompilerMessage(m) =
            serde_json::from_value(crate::diagnostic::sample()).unwrap()
        else {
            panic!()
        };
        let mut f = crate::diagnostic::normalize::normalize(&m, Path::new("/demo"));
        f.message = "<script>alert('bad')</script> & \"quoted\"".into();
        f.spans[0].path = PathBuf::from("/demo/src/<script>&#.rs");
        let mut second = f.clone();
        second.level = "error".into();
        second.spans.push(second.spans[0].clone());
        let report = Report {
            project: ProjectInfo {
                workspace_root: PathBuf::from("/demo"),
                target_dir: PathBuf::from("/demo/target"),
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
            raw_count: 2,
            findings: crate::diagnostic::dedup::merge_findings(vec![f, second]),
        };
        let sources = crate::source::load_sources(&report.project, &report.findings);
        let pages = super::super::paths::build_page_map(&report, &sources);
        render_report(&report, &sources, &pages, t.path()).unwrap();
        validate_report(t.path()).unwrap();
        let html =
            std::fs::read_to_string(t.path().join(pages.lint_pages.values().next().unwrap()))
                .unwrap();
        let doc = scraper::Html::parse_document(&html);
        assert_eq!(
            doc.select(&scraper::Selector::parse("script,[onclick],[onerror]").unwrap())
                .count(),
            0
        );
        assert_eq!(
            doc.select(&scraper::Selector::parse(".finding").unwrap())
                .count(),
            2
        );
        assert!(
            doc.root_element()
                .text()
                .collect::<String>()
                .contains("<script>alert('bad')</script>")
        );
        let source =
            std::fs::read_to_string(t.path().join(pages.source_pages.values().next().unwrap()))
                .unwrap();
        let doc = scraper::Html::parse_document(&source);
        assert_eq!(
            doc.select(&scraper::Selector::parse("script,[onclick],[onerror]").unwrap())
                .count(),
            0
        );
        let active = doc
            .select(&scraper::Selector::parse(".source-tree [aria-current=\"page\"]").unwrap())
            .next()
            .unwrap();
        assert!(active.text().collect::<String>().contains("<script>&#.rs"));
    }
}
