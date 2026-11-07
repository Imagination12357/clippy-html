//! Build a report-local classic-script index from existing pages and diagnostics.
use super::paths::{PageMap, category_key, group_key, relative_href};
use crate::{diagnostic::model::*, source::SourceFile};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

const PREFIX: &str = "globalThis.CLIPPY_HTML_SEARCH_DATA=";
#[derive(Serialize, Deserialize)]
struct Entry {
    kind: String,
    name: String,
    context: String,
    description: String,
    href: String,
}

fn notes_text(notes: &[Note], output: &mut String) {
    for note in notes {
        output.push('\n');
        output.push_str(&note.message);
        notes_text(&note.children, output);
    }
}
fn script(entries: &[Entry]) -> Result<String, String> {
    let json = serde_json::to_string(entries).map_err(|e| e.to_string())?;
    let json = json
        .replace('&', "\\u0026")
        .replace('<', "\\u003c")
        .replace('>', "\\u003e")
        .replace('\u{2028}', "\\u2028")
        .replace('\u{2029}', "\\u2029");
    Ok(format!("{PREFIX}{json};\n"))
}
pub(super) fn write_index(
    report: &Report,
    sources: &BTreeMap<PathBuf, SourceFile>,
    pages: &PageMap,
    destination: &Path,
) -> Result<(), String> {
    let from = Path::new("index.html");
    let mut entries = vec![];
    for (key, path) in &pages.lint_pages {
        let findings: Vec<_> = report
            .findings
            .iter()
            .filter(|f| group_key(f) == *key)
            .collect();
        let Some(first) = findings.first() else {
            continue;
        };
        entries.push(Entry {
            kind: "lint".into(),
            name: first
                .code
                .clone()
                .unwrap_or_else(|| "General diagnostics".into()),
            context: first.origin.clone(),
            description: format!("{} findings", findings.len()),
            href: relative_href(from, path, None),
        });
    }
    for (key, path) in &pages.source_pages {
        entries.push(Entry {
            kind: "file".into(),
            name: sources[key].display_path.clone(),
            context: String::new(),
            description: sources[key].reason.clone().unwrap_or_default(),
            href: relative_href(from, path, None),
        });
    }
    for finding in &report.findings {
        let mut context = vec![
            finding
                .code
                .as_deref()
                .unwrap_or("General diagnostics")
                .to_owned(),
            finding.level.clone(),
        ];
        if let Some(package) = report
            .project
            .packages
            .iter()
            .find(|p| p.id == finding.package_id)
        {
            context.push(package.name.clone());
        }
        if let Some(category) = category_key(finding) {
            context.push(category.into());
        }
        if let Some(span) = finding
            .spans
            .iter()
            .find(|s| s.primary)
            .or_else(|| finding.spans.first())
        {
            let name = sources
                .get(&span.path)
                .map(|s| s.display_path.clone())
                .unwrap_or_else(|| span.path.to_string_lossy().replace('\\', "/"));
            context.push(format!("{name}:{}:{}", span.line_start, span.column_start));
        }
        let mut description = context.join(" · ");
        notes_text(&finding.children, &mut description);
        for suggestion in &finding.suggestions {
            description.push('\n');
            description.push_str(&suggestion.message);
        }
        entries.push(Entry {
            kind: "finding".into(),
            name: finding.message.clone(),
            context: context.join(" "),
            description,
            href: relative_href(
                from,
                &pages.lint_pages[&group_key(finding)],
                Some(&format!("finding-{}", finding.id)),
            ),
        });
    }
    for (key, path) in &pages.category_pages {
        entries.push(Entry {
            kind: "category".into(),
            name: if key == "unknown" {
                "Unknown".into()
            } else {
                key.clone()
            },
            context: String::new(),
            description: format!(
                "{} findings",
                report
                    .findings
                    .iter()
                    .filter(|f| category_key(f) == Some(key.as_str()))
                    .count()
            ),
            href: relative_href(from, path, None),
        });
    }
    std::fs::write(
        destination.join("static.files/search-index.js"),
        script(&entries)?,
    )
    .map_err(|e| e.to_string())
}
pub(super) fn read_hrefs(root: &Path) -> Result<Vec<String>, String> {
    let script = match std::fs::read_to_string(root.join("static.files/search-index.js")) {
        Ok(script) => script,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(vec![]),
        Err(e) => return Err(e.to_string()),
    };
    let json = script
        .strip_prefix(PREFIX)
        .and_then(|s| s.trim_end().strip_suffix(';'))
        .ok_or("Invalid search index script")?;
    let entries: Vec<Entry> = serde_json::from_str(json).map_err(|e| e.to_string())?;
    Ok(entries.into_iter().map(|entry| entry.href).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn script_roundtrips_untrusted_text_without_html_or_line_separators() {
        let text = "</script><script>alert('x')</script>&\u{2028}\u{2029}한글";
        let encoded = script(&[Entry {
            kind: "finding".into(),
            name: text.into(),
            context: "".into(),
            description: "".into(),
            href: "index.html".into(),
        }])
        .unwrap();
        assert!(!encoded.contains(['<', '>', '&', '\u{2028}', '\u{2029}']));
        let entries: Vec<Entry> = serde_json::from_str(
            encoded
                .strip_prefix(PREFIX)
                .unwrap()
                .trim_end()
                .strip_suffix(';')
                .unwrap(),
        )
        .unwrap();
        assert_eq!(entries[0].name, text);
    }
}
