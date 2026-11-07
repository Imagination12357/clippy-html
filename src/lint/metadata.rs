//! Parse official lint tables and enrich already-identified findings.
use crate::diagnostic::model::{CategoryMetadata, Finding, Note};
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

pub const CATEGORIES: [&str; 9] = [
    "cargo",
    "complexity",
    "correctness",
    "nursery",
    "pedantic",
    "perf",
    "restriction",
    "style",
    "suspicious",
];

#[derive(Deserialize, Serialize)]
pub struct Bundle {
    pub schema_version: u32,
    pub selector: String,
    pub url: String,
    pub retrieved: String,
    pub source_hash: String,
    pub entries: BTreeMap<String, String>,
}

fn lint_name(name: &str) -> bool {
    !name.is_empty()
        && name
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_')
}

pub fn parse_html(bytes: &[u8]) -> Result<BTreeMap<String, String>, String> {
    if bytes.len() > super::fetch::MAX_BYTES {
        return Err("Category document exceeds 8 MiB".into());
    }
    let doc = Html::parse_document(std::str::from_utf8(bytes).map_err(|e| e.to_string())?);
    let articles = Selector::parse("article[id]").expect("constant selector");
    let groups =
        Selector::parse(":scope > label > h2.lint-title .lint-group").expect("constant selector");
    let mut seen = BTreeSet::new();
    let mut entries = BTreeMap::new();
    for article in doc.select(&articles) {
        let name = article.value().attr("id").unwrap_or_default();
        if !lint_name(name) || !seen.insert(name.to_owned()) {
            return Err("Invalid or duplicate category lint name".into());
        }
        let labels: Vec<_> = article.select(&groups).collect();
        if labels.len() != 1 {
            return Err(format!("Invalid category label count for {name}"));
        }
        let category = labels[0].text().collect::<String>();
        let category = category.trim();
        if category == "deprecated" {
            continue;
        }
        if !CATEGORIES.contains(&category) {
            return Err(format!("Unsupported category for {name}"));
        }
        entries.insert(name.to_owned(), category.to_owned());
    }
    if entries.is_empty() {
        return Err("No active lint categories found".into());
    }
    Ok(entries)
}

fn document_selector(link: &str) -> Option<String> {
    let url = url::Url::parse(link).ok()?;
    if url.scheme() != "https"
        || !url.username().is_empty()
        || url.password().is_some()
        || url.port_or_known_default() != Some(443)
        || url.query().is_some()
    {
        return None;
    }
    let path = match url.host_str()? {
        "clippy.rust-lang.org" => url.path().strip_prefix('/')?,
        "rust-lang.github.io" => url.path().strip_prefix("/rust-clippy/")?,
        _ => return None,
    };
    let selector = path.strip_suffix("/index.html")?;
    let version = selector.strip_prefix("rust-");
    if matches!(selector, "main" | "master")
        || version.is_some_and(|v| {
            let parts: Vec<_> = v.split('.').collect();
            parts.len() == 3
                && parts
                    .iter()
                    .all(|p| !p.is_empty() && p.len() <= 5 && p.bytes().all(|b| b.is_ascii_digit()))
        })
    {
        Some(selector.into())
    } else {
        None
    }
}

fn note_selectors(notes: &[Note], selectors: &mut BTreeSet<String>) {
    for note in notes {
        if let Some(link) = note.message.strip_prefix("for further information visit ")
            && let Some(selector) = document_selector(link)
        {
            selectors.insert(selector);
        }
        note_selectors(&note.children, selectors);
    }
}

pub fn resolve(findings: &mut [Finding], offline: bool) -> Option<CategoryMetadata> {
    resolve_with(findings, offline, super::fetch::download)
}

fn resolve_with(
    findings: &mut [Finding],
    offline: bool,
    fetch: impl FnOnce(&str) -> Result<Vec<u8>, String>,
) -> Option<CategoryMetadata> {
    if !findings.iter().any(|f| f.origin == "Clippy") {
        return None;
    }
    let mut selectors = BTreeSet::new();
    for f in findings.iter().filter(|f| f.origin == "Clippy") {
        note_selectors(&f.children, &mut selectors);
    }
    let online = if offline {
        Err("Explicit tool --offline".into())
    } else if selectors.len() != 1 {
        Err("Clippy documentation version unavailable or ambiguous".into())
    } else {
        let selector = selectors.into_iter().next().expect("one selector");
        let url = format!("https://clippy.rust-lang.org/{selector}/index.html");
        fetch(&url).and_then(|bytes| {
            let entries = parse_html(&bytes)?;
            Ok((
                entries,
                selector,
                url,
                format!("blake3:{}", blake3::hash(&bytes).to_hex()),
            ))
        })
    };
    let (entries, mode, selector, url, source_hash, reason) = match online {
        Ok((entries, selector, url, source_hash)) => {
            (entries, "online", selector, url, source_hash, None)
        }
        Err(reason) => {
            let bundle: Bundle =
                serde_json::from_str(include_str!("../../assets/vendor/clippy/categories.json"))
                    .expect("validated embedded category table");
            (
                bundle.entries,
                "bundled",
                bundle.selector,
                bundle.url,
                bundle.source_hash,
                Some(reason),
            )
        }
    };
    let table_hash = format!(
        "blake3:{}",
        blake3::hash(&serde_json::to_vec(&entries).expect("string table")).to_hex()
    );
    for f in findings {
        f.category = if f.origin == "Clippy" {
            f.code
                .as_deref()
                .and_then(|c| c.strip_prefix("clippy::"))
                .and_then(|name| entries.get(name))
                .cloned()
        } else {
            None
        };
    }
    Some(CategoryMetadata {
        mode: mode.into(),
        selector,
        url,
        source_hash,
        table_hash,
        reason,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    fn finding() -> Finding {
        let cargo_metadata::Message::CompilerMessage(message) =
            serde_json::from_value(crate::diagnostic::sample()).unwrap()
        else {
            panic!()
        };
        let mut f =
            crate::diagnostic::normalize::normalize(&message, std::path::Path::new("/demo"));
        f.code = Some("clippy::nonminimal_bool".into());
        f.children = vec![Note { level: "help".into(), message: "for further information visit https://rust-lang.github.io/rust-clippy/rust-1.94.0/index.html#nonminimal_bool".into(), spans: vec![], children: vec![] }];
        crate::diagnostic::dedup::merge_findings(vec![f]).remove(0)
    }
    #[test]
    fn only_official_document_selectors_can_form_requests() {
        assert_eq!(
            document_selector(
                "https://rust-lang.github.io/rust-clippy/rust-1.97.0/index.html#clone_on_copy"
            )
            .as_deref(),
            Some("rust-1.97.0")
        );
        assert_eq!(
            document_selector("https://clippy.rust-lang.org/master/index.html").as_deref(),
            Some("master")
        );
        for link in [
            "http://clippy.rust-lang.org/master/index.html",
            "https://evil.test/master/index.html",
            "https://clippy.rust-lang.org.evil.test/master/index.html",
            "https://user@clippy.rust-lang.org/master/index.html",
            "https://clippy.rust-lang.org:444/master/index.html",
            "https://clippy.rust-lang.org/master/index.html?x=1",
            "https://clippy.rust-lang.org/stable/index.html",
            "https://clippy.rust-lang.org/rust-one/index.html",
            "https://rust-lang.github.io/else/master/index.html",
        ] {
            assert!(document_selector(link).is_none(), "{link}");
        }
    }
    #[test]
    fn offline_failure_and_online_tables_preserve_identity_and_completeness_contract() {
        let original = finding();
        let id = original.id.clone();
        for reason in [
            "timeout",
            "HTTP 404",
            "TLS failed",
            "Category document exceeds 8 MiB",
        ] {
            let mut fs = vec![original.clone()];
            let meta = resolve_with(&mut fs, false, |_| Err(reason.into())).unwrap();
            assert_eq!(meta.mode, "bundled");
            assert_eq!(meta.reason.as_deref(), Some(reason));
            assert_eq!(fs[0].category.as_deref(), Some("pedantic"));
            assert_eq!(fs[0].id, id);
        }
        let mut offline = vec![original.clone()];
        assert_eq!(
            resolve_with(&mut offline, true, |_| panic!("offline must not fetch"))
                .unwrap()
                .mode,
            "bundled"
        );
        for (version, group) in [("1.94.0", "complexity"), ("1.97.0", "pedantic")] {
            let mut fs = vec![original.clone()];
            fs[0].children[0].message = fs[0].children[0].message.replace("1.94.0", version);
            let meta = resolve_with(&mut fs, false, |url| {
                assert_eq!(
                    url,
                    format!("https://clippy.rust-lang.org/rust-{version}/index.html")
                );
                Ok(html(group))
            })
            .unwrap();
            assert_eq!(meta.mode, "online");
            assert_eq!(fs[0].category.as_deref(), Some(group));
            assert_eq!(fs[0].id, id);
        }
    }
    #[test]
    fn skips_unneeded_fetch_and_does_not_mix_online_and_bundled_entries() {
        assert!(resolve_with(&mut [], false, |_| panic!("clean")).is_none());
        let mut compiler = finding();
        compiler.origin = "Compiler".into();
        assert!(resolve_with(&mut [compiler], false, |_| panic!("compiler only")).is_none());
        let mut fs = vec![finding()];
        fs[0].code = Some("clippy::clone_on_copy".into());
        resolve_with(&mut fs, false, |_| Ok(html("style"))).unwrap();
        assert_eq!(
            fs[0].category, None,
            "known bundled name absent online stays Unknown"
        );
        let mut fs = vec![finding()];
        fs[0].children.clear();
        assert!(
            resolve_with(&mut fs, false, |_| panic!("missing selector"))
                .unwrap()
                .reason
                .unwrap()
                .contains("unavailable")
        );
        let mut fs = vec![finding()];
        let mut conflicting = fs[0].children[0].clone();
        conflicting.message = conflicting.message.replace("1.94.0", "1.97.0");
        fs[0].children.push(conflicting);
        assert!(
            resolve_with(&mut fs, false, |_| panic!("ambiguous selector"))
                .unwrap()
                .reason
                .unwrap()
                .contains("ambiguous")
        );
    }
    #[test]
    fn embedded_table_has_valid_names_and_active_categories() {
        let table: Bundle =
            serde_json::from_str(include_str!("../../assets/vendor/clippy/categories.json"))
                .unwrap();
        assert_eq!(table.schema_version, 1);
        assert_eq!(table.entries.len(), 815);
        assert_eq!(table.selector, "rust-1.97.0");
        assert!(
            table
                .entries
                .iter()
                .all(|(name, category)| lint_name(name) && CATEGORIES.contains(&category.as_str()))
        );
    }
    fn html(group: &str) -> Vec<u8> {
        format!("<article id='nonminimal_bool'><label><h2 class='lint-title'><span class='lint-group'>{group}</span></h2></label><div class='lint-docs'><span class='lint-group'>fake</span></div></article>").into_bytes()
    }
    #[test]
    fn official_categories_follow_the_selected_table_and_reject_invalid_records() {
        assert_eq!(
            parse_html(&html("complexity")).unwrap()["nonminimal_bool"],
            "complexity"
        );
        assert_eq!(
            parse_html(&html("pedantic")).unwrap()["nonminimal_bool"],
            "pedantic"
        );
        for bytes in [b"<html>not a lint table</html>".to_vec(), html("invented"), [html("style"),html("perf")].concat(), b"<article id='../bad'><label><h2 class='lint-title'><span class='lint-group'>style</span></h2></label></article>".to_vec()] {
            assert!(parse_html(&bytes).is_err());
        }
    }
}
