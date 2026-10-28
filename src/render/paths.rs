//! Stable report names with safely encoded workspace source paths.
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
fn safe_component(text: &str, directory: bool) -> String {
    let mut name = String::new();
    for ch in text.chars() {
        if ch.is_alphanumeric() || matches!(ch, '_' | '-' | '.') {
            name.push(ch);
        } else {
            for byte in ch.to_string().bytes() {
                name.push_str(&format!("~{byte:02X}"));
            }
        }
    }
    let stem = name
        .split('.')
        .next()
        .unwrap_or_default()
        .to_ascii_uppercase();
    if matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL")
        || ["COM", "LPT"].iter().any(|prefix| {
            stem.strip_prefix(prefix).is_some_and(|n| {
                matches!(
                    n,
                    "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "¹" | "²" | "³"
                )
            })
        })
    {
        name.insert_str(0, "~00");
    }
    if name.is_empty() || name == "." || name == ".." {
        name = name.replace('.', "~2E");
        name.insert_str(0, "~00");
    }
    if directory && name.to_ascii_lowercase().ends_with(".html") {
        let dot = name.len() - 5;
        name.replace_range(dot..dot + 1, "~2E");
    }
    while name.ends_with('.') {
        name.pop();
        name.push_str("~2E");
    }
    // Keep single filenames well below common filesystem component limits.
    if name.len() > 120 {
        let hash = blake3::hash(text.as_bytes()).to_hex();
        let mut end = 80;
        while !name.is_char_boundary(end) {
            end -= 1;
        }
        name = format!("{}-{}", &name[..end], &hash[..16]);
    }
    name
}
fn source_candidate(root: &Path, path: &Path) -> PathBuf {
    if let Ok(relative) = path.strip_prefix(root)
        && relative
            .components()
            .all(|c| matches!(c, std::path::Component::Normal(_)))
        && relative.file_name().is_some()
    {
        let relative = if relative.components().count() > 1 {
            relative.strip_prefix("src").unwrap_or(relative)
        } else {
            relative
        };
        let parts: Vec<_> = relative.components().collect();
        let mut output = PathBuf::from("src");
        for (i, part) in parts.iter().enumerate() {
            let last = i + 1 == parts.len();
            let name = safe_component(&part.as_os_str().to_string_lossy(), !last);
            output.push(if last { format!("{name}.html") } else { name });
        }
        output
    } else {
        let name = safe_component(
            &path.file_name().unwrap_or_default().to_string_lossy(),
            false,
        );
        let digest = blake3::hash(path.as_os_str().as_encoded_bytes()).to_hex();
        PathBuf::from(format!("src/~external/{name}-{}.html", &digest[..16]))
    }
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
    let candidates: BTreeMap<_, _> = sources
        .keys()
        .map(|key| {
            (
                key.clone(),
                source_candidate(&report.project.workspace_root, key),
            )
        })
        .collect();
    let mut used: std::collections::BTreeSet<_> = candidates
        .values()
        .map(|p| p.to_string_lossy().to_lowercase().to_uppercase())
        .collect();
    let mut assigned = std::collections::BTreeSet::new();
    let source_pages = candidates
        .into_iter()
        .map(|(key, mut path)| {
            // Conservatively group Unicode case aliases, including final sigma
            // and Kelvin sign, before creating files on Windows.
            if !assigned.insert(path.to_string_lossy().to_lowercase().to_uppercase()) {
                let digest = blake3::hash(key.as_os_str().as_encoded_bytes()).to_hex();
                let stem = path
                    .file_stem()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .into_owned();
                let mut suffix = 0;
                loop {
                    let extra = if suffix == 0 {
                        String::new()
                    } else {
                        format!("-{suffix}")
                    };
                    path.set_file_name(format!("{stem}-{}{extra}.html", &digest[..16]));
                    if used.insert(path.to_string_lossy().to_lowercase().to_uppercase()) {
                        break;
                    }
                    suffix += 1;
                }
            }
            (key, path)
        })
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
        to.to_string_lossy()
            .replace('\\', "/")
            .bytes()
            .map(|b| {
                if b.is_ascii_alphanumeric() || matches!(b, b'/' | b'-' | b'_' | b'.' | b'~') {
                    char::from(b).to_string()
                } else {
                    format!("%{b:02X}")
                }
            })
            .collect::<String>()
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
    fn source_names_preserve_hierarchy_without_unsafe_or_colliding_paths() {
        use crate::diagnostic::model::*;
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
        let keys = [
            "/demo/src/diagnostic/한글.rs",
            "/demo/src/a #%.rs",
            "/demo/src/CON.rs",
            "/demo/src/Foo.rs",
            "/demo/src/foo.rs",
            "/demo/src/a.html/child.rs",
            "/demo/src/a",
            "/outside/lib.rs",
            "/demo/../escape.rs",
            "/demo/src/σ.rs",
            "/demo/src/ς.rs",
        ];
        let sources = keys
            .into_iter()
            .map(|key| {
                (
                    PathBuf::from(key),
                    SourceFile {
                        display_path: key.into(),
                        text: None,
                        reason: None,
                    },
                )
            })
            .collect();
        let pages = build_page_map(&report, &sources).source_pages;
        assert_eq!(
            pages[Path::new(keys[0])],
            PathBuf::from("src/diagnostic/한글.rs.html")
        );
        assert_eq!(
            pages[Path::new(keys[1])],
            PathBuf::from("src/a~20~23~25.rs.html")
        );
        let names: std::collections::BTreeSet<_> = pages
            .values()
            .map(|p| p.to_string_lossy().to_lowercase())
            .collect();
        assert_eq!(names.len(), keys.len());
        assert_ne!(
            pages[Path::new(keys[9])].to_string_lossy().to_uppercase(),
            pages[Path::new(keys[10])].to_string_lossy().to_uppercase(),
            "Windows aliases sigma and final sigma; their output files must differ"
        );
        assert!(
            !pages[Path::new(keys[2])]
                .file_name()
                .unwrap()
                .to_string_lossy()
                .to_uppercase()
                .starts_with("CON.")
        );
        for page in pages.values() {
            assert!(page.starts_with("src"));
            assert!(
                page.components()
                    .all(|c| matches!(c, std::path::Component::Normal(_)))
            );
            assert!(
                !pages
                    .values()
                    .any(|other| other != page && other.starts_with(page))
            );
        }
        assert_eq!(
            relative_href(
                Path::new("lint/a.html"),
                Path::new("src/한글.rs.html"),
                None
            ),
            "../src/%ED%95%9C%EA%B8%80.rs.html"
        );
    }
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
