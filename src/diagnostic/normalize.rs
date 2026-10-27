//! Preserve diagnostic trees and multipart suggestions as units.
use super::model::*;
use serde_json::Value;
use std::path::Path;
fn string(v: &Value, key: &str) -> String {
    v[key].as_str().unwrap_or_default().to_owned()
}
fn spans(v: &Value, root: &Path) -> Vec<Span> {
    v["spans"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|s| Span {
            path: root.join(string(s, "file_name")),
            byte_start: s["byte_start"].as_u64().unwrap_or(0) as usize,
            byte_end: s["byte_end"].as_u64().unwrap_or(0) as usize,
            line_start: s["line_start"].as_u64().unwrap_or(0) as usize,
            line_end: s["line_end"].as_u64().unwrap_or(0) as usize,
            column_start: s["column_start"].as_u64().unwrap_or(0) as usize,
            column_end: s["column_end"].as_u64().unwrap_or(0) as usize,
            primary: s["is_primary"].as_bool().unwrap_or(false),
            label: s["label"].as_str().map(str::to_owned),
            text: s["text"]
                .as_array()
                .into_iter()
                .flatten()
                .map(|t| SnippetLine {
                    text: string(t, "text"),
                    highlight_start: t["highlight_start"].as_u64().unwrap_or(0) as usize,
                    highlight_end: t["highlight_end"].as_u64().unwrap_or(0) as usize,
                })
                .collect(),
            expansion: if s["expansion"].is_null() {
                None
            } else {
                Some(s["expansion"].clone())
            },
        })
        .collect()
}
fn note(v: &Value, root: &Path, path: Vec<usize>, suggestions: &mut Vec<Suggestion>) -> Note {
    let normalized = spans(v, root);
    let replacements: Vec<_> = v["spans"]
        .as_array()
        .into_iter()
        .flatten()
        .zip(&normalized)
        .filter_map(|(raw, span)| {
            raw["suggested_replacement"]
                .as_str()
                .map(|text| Replacement {
                    span: span.clone(),
                    text: text.into(),
                    applicability: raw["suggestion_applicability"]
                        .as_str()
                        .unwrap_or("Unspecified")
                        .into(),
                })
        })
        .collect();
    if !replacements.is_empty() {
        suggestions.push(Suggestion {
            message: string(v, "message"),
            child_path: path.clone(),
            replacements,
        });
    }
    let children = v["children"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
        .map(|(i, c)| {
            let mut p = path.clone();
            p.push(i);
            note(c, root, p, suggestions)
        })
        .collect();
    Note {
        level: string(v, "level"),
        message: string(v, "message"),
        spans: normalized,
        children,
    }
}
pub fn normalize(message: &cargo_metadata::CompilerMessage, root: &Path) -> Finding {
    let v = serde_json::to_value(&message.message).expect("Cargo diagnostic is JSON serializable");
    let mut suggestions = vec![];
    let n = note(&v, root, vec![], &mut suggestions);
    let code = v["code"]["code"].as_str().map(str::to_owned);
    let origin = if code.as_deref().is_some_and(|s| s.starts_with("clippy::")) {
        "Clippy"
    } else if code.is_some() {
        "Rust compiler"
    } else {
        "General"
    };
    Finding {
        id: String::new(),
        package_id: message.package_id.to_string(),
        targets: vec![TargetInfo {
            name: message.target.name.clone(),
            kinds: message.target.kind.iter().map(|k| k.to_string()).collect(),
            source: message.target.src_path.clone().into_std_path_buf(),
        }],
        origin: origin.into(),
        code,
        level: n.level,
        category: None,
        message: n.message,
        spans: n.spans,
        children: n.children,
        suggestions,
        rendered: v["rendered"].as_str().map(str::to_owned),
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    pub fn finding() -> Finding {
        let msg: cargo_metadata::Message = serde_json::from_value(super::super::sample()).unwrap();
        let cargo_metadata::Message::CompilerMessage(m) = msg else {
            panic!()
        };
        normalize(&m, Path::new("/demo"))
    }
    #[test]
    fn preserves_diagnostic_kind_and_level() {
        let f = finding();
        assert_eq!(f.origin, "Clippy");
        assert_eq!(f.level, "warning");
        assert_eq!(f.spans.len(), 1);
    }
    #[test]
    fn multipart_replacements_stay_grouped() {
        let mut v = super::super::sample();
        let mut span = v["message"]["spans"][0].clone();
        span["suggested_replacement"] = "fixed".into();
        span["suggestion_applicability"] = "MachineApplicable".into();
        v["message"]["children"] = serde_json::json!([{"message":"replace both","level":"help","spans":[span.clone(),span],"children":[],"code":null,"rendered":null}]);
        let cargo_metadata::Message::CompilerMessage(m) = serde_json::from_value(v).unwrap() else {
            panic!()
        };
        let f = normalize(&m, Path::new("/demo"));
        assert_eq!(f.suggestions.len(), 1);
        assert_eq!(f.suggestions[0].replacements.len(), 2);
        assert_eq!(f.children.len(), 1);
    }
}
