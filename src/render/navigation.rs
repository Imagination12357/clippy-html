//! Static source hierarchy, rendered with escaped labels and relative links.
use super::paths::{PageMap, relative_href};
use crate::{
    diagnostic::model::Finding,
    source::{SourceFile, all_spans},
};
use askama::Template;
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

struct FileLink {
    href: String,
    count: usize,
    current: bool,
}
#[derive(Default)]
struct Node {
    children: BTreeMap<String, Node>,
    file: Option<FileLink>,
    open: bool,
}
#[derive(Template)]
#[template(path = "source-tree.html")]
struct TreeEntry<'a> {
    name: &'a str,
    file: &'a Option<FileLink>,
    children: TreeContents,
    open: bool,
}
#[derive(Default, Template)]
#[template(path = "source-tree-contents.html")]
struct TreeContents {
    folders: String,
    files: String,
}
fn render_children(node: &Node) -> Result<TreeContents, String> {
    let mut contents = TreeContents::default();
    for (name, child) in &node.children {
        if child.file.is_some() {
            contents.files.push_str(
                &TreeEntry {
                    name,
                    file: &child.file,
                    children: TreeContents::default(),
                    open: false,
                }
                .render()
                .map_err(|e| e.to_string())?,
            );
        }
        if !child.children.is_empty() {
            contents.folders.push_str(
                &TreeEntry {
                    name,
                    file: &None,
                    children: render_children(child)?,
                    open: child.open,
                }
                .render()
                .map_err(|e| e.to_string())?,
            );
        }
    }
    Ok(contents)
}
pub(super) fn source_counts(findings: &[Finding]) -> BTreeMap<PathBuf, usize> {
    let mut counts = BTreeMap::new();
    for finding in findings {
        let paths: std::collections::BTreeSet<_> =
            all_spans(finding).into_iter().map(|s| &s.path).collect();
        for path in paths {
            *counts.entry(path.clone()).or_default() += 1;
        }
    }
    counts
}
pub(super) fn source_tree(
    sources: &BTreeMap<PathBuf, SourceFile>,
    pages: &PageMap,
    counts: &BTreeMap<PathBuf, usize>,
    workspace_root: &Path,
    current: &Path,
    from: &Path,
) -> Result<String, String> {
    let mut root = Node::default();
    let mut external = Node::default();
    for (key, source) in sources {
        let active = key == current;
        let local = key.strip_prefix(workspace_root).is_ok_and(|relative| {
            relative
                .components()
                .all(|c| matches!(c, std::path::Component::Normal(_)))
        });
        let mut node = if local { &mut root } else { &mut external };
        node.open |= active;
        for part in source
            .display_path
            .split('/')
            .filter(|part| !part.is_empty())
        {
            node = node.children.entry(part.into()).or_default();
            node.open |= active;
        }
        node.file = Some(FileLink {
            href: relative_href(from, &pages.source_pages[key], None),
            count: counts.get(key).copied().unwrap_or(0),
            current: active,
        });
    }
    let mut contents = render_children(&root)?;
    if !external.children.is_empty() {
        contents.folders.push_str(
            &TreeEntry {
                name: "External sources",
                file: &None,
                children: render_children(&external)?,
                open: external.open,
            }
            .render()
            .map_err(|e| e.to_string())?,
        );
    }
    contents.render().map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::model::*;
    #[test]
    fn cached_counts_count_each_finding_once_per_file() {
        let cargo_metadata::Message::CompilerMessage(message) =
            serde_json::from_value(crate::diagnostic::sample()).unwrap()
        else {
            panic!()
        };
        let mut first = crate::diagnostic::normalize::normalize(&message, Path::new("/demo"));
        let path = first.spans[0].path.clone();
        first.spans.push(first.spans[0].clone());
        let mut second = first.clone();
        let other = PathBuf::from("/demo/other.rs");
        second.spans[0].path = other.clone();
        let counts = source_counts(&[first, second]);
        assert_eq!(counts[&path], 2);
        assert_eq!(counts[&other], 1);
    }
    #[test]
    fn external_and_workspace_paths_keep_distinct_tree_entries() {
        let report = Report {
            project: ProjectInfo {
                workspace_root: "/demo".into(),
                target_dir: "/demo/target".into(),
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
        let sources = BTreeMap::from([
            (
                PathBuf::from("/demo/dep/lib.rs"),
                SourceFile {
                    display_path: "dep/lib.rs".into(),
                    text: None,
                    reason: None,
                },
            ),
            (
                PathBuf::from("/dep/lib.rs"),
                SourceFile {
                    display_path: "/dep/lib.rs".into(),
                    text: None,
                    reason: None,
                },
            ),
        ]);
        let pages = super::super::paths::build_page_map(&report, &sources);
        for key in sources.keys() {
            let html = source_tree(
                &sources,
                &pages,
                &source_counts(&report.findings),
                &report.project.workspace_root,
                key,
                &pages.source_pages[key],
            )
            .unwrap();
            let doc = scraper::Html::parse_fragment(&html);
            assert_eq!(
                doc.select(
                    &scraper::Selector::parse("div.files > a.selected[aria-current=\"page\"]")
                        .unwrap()
                )
                .count(),
                1
            );
            assert!(
                doc.select(
                    &scraper::Selector::parse(
                        "details.dir-entry > summary + div.folders + div.files"
                    )
                    .unwrap()
                )
                .count()
                    >= 2
            );
            assert_eq!(
                doc.select(&scraper::Selector::parse("a").unwrap()).count(),
                2
            );
            assert_eq!(
                doc.select(&scraper::Selector::parse("a[aria-current=\"page\"]").unwrap())
                    .count(),
                1
            );
        }
    }
}
