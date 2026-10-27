//! Load only workspace-owned source files and validate diagnostic coordinates.
use crate::diagnostic::model::*;
use std::{collections::BTreeMap, path::PathBuf};
#[derive(Debug)]
pub struct SourceFile {
    pub display_path: String,
    pub text: Option<String>,
    pub reason: Option<String>,
}
pub fn all_spans(f: &Finding) -> Vec<&Span> {
    fn notes<'a>(n: &'a Note, out: &mut Vec<&'a Span>) {
        out.extend(&n.spans);
        for child in &n.children {
            notes(child, out);
        }
    }
    let mut out: Vec<_> = f.spans.iter().collect();
    for n in &f.children {
        notes(n, &mut out);
    }
    for s in &f.suggestions {
        out.extend(s.replacements.iter().map(|r| &r.span));
    }
    out
}
pub fn load_sources(project: &ProjectInfo, findings: &[Finding]) -> BTreeMap<PathBuf, SourceFile> {
    let roots: Vec<_> = project
        .packages
        .iter()
        .filter(|p| p.workspace_member)
        .filter_map(|p| p.manifest.parent()?.canonicalize().ok())
        .collect();
    let target = project.target_dir.canonicalize().ok();
    let member_ids: std::collections::BTreeSet<_> = project
        .packages
        .iter()
        .filter(|p| p.workspace_member)
        .map(|p| p.id.as_str())
        .collect();
    let member_manifests: std::collections::BTreeSet<_> = project
        .packages
        .iter()
        .filter(|p| p.workspace_member)
        .filter_map(|p| p.manifest.canonicalize().ok())
        .collect();
    let mut paths: BTreeMap<PathBuf, bool> = BTreeMap::new();
    for finding in findings {
        for span in all_spans(finding) {
            *paths.entry(span.path.clone()).or_default() |=
                member_ids.contains(finding.package_id.as_str());
        }
    }
    let mut sources = BTreeMap::new();
    for (source_path, eligible) in paths {
        sources.entry(source_path.clone()).or_insert_with(||{
            let loaded=source_path.canonicalize().map_err(|e|e.to_string()).and_then(|path|{
                let nearest_manifest=path.parent().and_then(|p|p.ancestors().map(|a|a.join("Cargo.toml")).find(|m|m.is_file())).and_then(|p|p.canonicalize().ok());
                if !eligible||!roots.iter().any(|r|path.starts_with(r))||target.as_ref().is_some_and(|r|path.starts_with(r))||!nearest_manifest.is_some_and(|m|member_manifests.contains(&m)){return Err("External, generated, or non-workspace source; diagnostic snippet shown instead".into());}
                std::fs::read_to_string(path).map_err(|e|e.to_string())
            });
            let (text,reason)=match loaded{Ok(s)=>(Some(s),None),Err(e)=>(None,Some(e))};
            SourceFile{display_path:source_path.strip_prefix(&project.workspace_root).unwrap_or(&source_path).to_string_lossy().replace('\\',"/"),text,reason}
        });
    }
    sources
}
pub fn span_matches(source: &SourceFile, span: &Span) -> bool {
    let Some(text) = &source.text else {
        return false;
    };
    if span.line_start == 0
        || span.line_end < span.line_start
        || span.byte_end < span.byte_start
        || span.byte_end > text.len()
        || !text.is_char_boundary(span.byte_start)
        || !text.is_char_boundary(span.byte_end)
    {
        return false;
    }
    let lines: Vec<_> = text.split('\n').collect();
    if span.line_end > lines.len() {
        return false;
    }
    if !span
        .text
        .iter()
        .zip(lines.iter().skip(span.line_start - 1))
        .all(|(snippet, line)| snippet.text == line.trim_end_matches('\r'))
    {
        return false;
    }
    let coordinate = |line: usize, column: usize| -> Option<usize> {
        if column == 0 {
            return None;
        }
        let prefix = lines
            .iter()
            .take(line - 1)
            .map(|l| l.len() + 1)
            .sum::<usize>();
        let value = lines[line - 1].trim_end_matches('\r');
        let offset = if column - 1 == value.chars().count() {
            value.len()
        } else {
            value.char_indices().nth(column - 1)?.0
        };
        Some(prefix + offset)
    };
    coordinate(span.line_start, span.column_start) == Some(span.byte_start)
        && coordinate(span.line_end, span.column_end) == Some(span.byte_end)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    #[test]
    fn unicode_crlf_and_source_mismatch() {
        let mut span = super::super::diagnostic::normalize::normalize(
            &match serde_json::from_value(crate::diagnostic::sample()).unwrap() {
                cargo_metadata::Message::CompilerMessage(m) => m,
                _ => panic!(),
            },
            Path::new("/demo"),
        )
        .spans
        .remove(0);
        let source = SourceFile {
            display_path: "a.rs".into(),
            text: Some("한글\r\nabc".into()),
            reason: None,
        };
        span.byte_start = 0;
        span.byte_end = 6;
        span.line_start = 1;
        span.line_end = 1;
        span.column_start = 1;
        span.column_end = 3;
        span.text[0].text = "한글".into();
        assert!(span_matches(&source, &span));
        span.byte_start = 1;
        assert!(!span_matches(&source, &span));
        span.byte_start = 0;
        span.text[0].text = "different".into();
        assert!(!span_matches(&source, &span));
    }
    #[test]
    fn missing_source_is_fallback() {
        let project = ProjectInfo {
            workspace_root: PathBuf::from("/missing"),
            target_dir: PathBuf::from("/missing/target"),
            packages: vec![],
        };
        let f = crate::diagnostic::normalize::normalize(
            &match serde_json::from_value(crate::diagnostic::sample()).unwrap() {
                cargo_metadata::Message::CompilerMessage(m) => m,
                _ => panic!(),
            },
            Path::new("/demo"),
        );
        let sources = load_sources(&project, &[f]);
        assert_eq!(sources.len(), 1);
        assert!(sources.values().next().unwrap().text.is_none());
    }
}
