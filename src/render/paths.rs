//! Stable ASCII report names; user paths never become output paths.
use crate::{diagnostic::model::Report, source::SourceFile};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};
pub struct PageMap {
    pub lint_pages: BTreeMap<String, PathBuf>,
    pub source_pages: BTreeMap<PathBuf, PathBuf>,
}
fn assigned_names<K: Ord + Clone>(
    keys: impl IntoIterator<Item = K>,
    digest: impl Fn(&K) -> String,
) -> BTreeMap<K, String> {
    let keys: std::collections::BTreeSet<_> = keys.into_iter().collect();
    let mut counts: BTreeMap<String, usize> = BTreeMap::new();
    keys.into_iter()
        .map(|key| {
            let base = digest(&key);
            let count = counts.entry(base.clone()).or_default();
            let name = if *count == 0 {
                base
            } else {
                format!("{base}-{count}")
            };
            *count += 1;
            (key, name)
        })
        .collect()
}
pub fn group_key(f: &crate::diagnostic::model::Finding) -> String {
    format!("{}:{}", f.origin, f.code.as_deref().unwrap_or("general"))
}
pub fn build_page_map(report: &Report, sources: &BTreeMap<PathBuf, SourceFile>) -> PageMap {
    let lint_pages = assigned_names(report.findings.iter().map(group_key), |key| {
        if key == "General:general" {
            "general".into()
        } else {
            blake3::hash(key.as_bytes()).to_hex().to_string()
        }
    })
    .into_iter()
    .map(|(key, name)| (key, PathBuf::from(format!("lint/{name}.html"))))
    .collect();
    let source_pages = assigned_names(sources.keys().cloned(), |path| {
        blake3::hash(path.as_os_str().as_encoded_bytes())
            .to_hex()
            .to_string()
    })
    .into_iter()
    .map(|(key, name)| (key, PathBuf::from(format!("src/{name}.html"))))
    .collect();
    PageMap {
        lint_pages,
        source_pages,
    }
}
pub fn relative_href(from: &Path, to: &Path, anchor: Option<&str>) -> String {
    let depth = from.parent().map_or(0, |p| p.components().count());
    let mut result = format!(
        "{}{}",
        "../".repeat(depth),
        to.to_string_lossy().replace('\\', "/")
    );
    if let Some(id) = anchor {
        result.push('#');
        result.push_str(id);
    }
    result
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn relative_navigation_stays_portable() {
        assert_eq!(
            relative_href(
                Path::new("lint/a.html"),
                Path::new("src/b.html"),
                Some("L42")
            ),
            "../src/b.html#L42"
        );
        assert_eq!(
            relative_href(Path::new("index.html"), Path::new("lint/a.html"), None),
            "lint/a.html"
        );
    }
    #[test]
    fn colliding_names_are_distinct_and_order_independent() {
        let names = assigned_names(["b", "a"], |_| "same".into());
        assert_ne!(names["a"], names["b"]);
        assert_eq!(names, assigned_names(["a", "b"], |_| "same".into()));
    }
}
