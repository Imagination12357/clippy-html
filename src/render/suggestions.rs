//! Build one validated, workspace-relative patch for a whole suggestion.
use crate::{
    diagnostic::model::{Replacement, Suggestion},
    source::{SourceFile, span_matches},
};
use std::{
    collections::BTreeMap,
    fmt::Write,
    path::{Component, Path},
    time::{Duration, Instant},
};

const MAX_BYTES: usize = 4 * 1024 * 1024;

fn patch_path(root: &Path, path: &Path, prefix: &str) -> Result<String, String> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| "Source is outside the workspace")?;
    if relative.as_os_str().is_empty()
        || relative
            .components()
            .any(|c| !matches!(c, Component::Normal(_)))
    {
        return Err("Source path cannot be represented safely in a patch".into());
    }
    let mut name = prefix.to_owned();
    for component in relative.components() {
        let s = component
            .as_os_str()
            .to_str()
            .ok_or("Source path is not UTF-8")?;
        if s.chars().any(char::is_control) || s.contains('\\') {
            return Err("Source path cannot be represented safely in a patch".into());
        }
        name.push('/');
        name.push_str(s);
    }
    // Git's quoted path syntax handles spaces, quotes and UTF-8 octal bytes.
    let mut quoted = String::from("\"");
    for byte in name.bytes() {
        match byte {
            b'"' => quoted.push_str("\\\""),
            0x20..=0x7e => quoted.push(char::from(byte)),
            _ => {
                write!(&mut quoted, "\\{byte:03o}").expect("write string");
            }
        }
    }
    quoted.push('"');
    Ok(quoted)
}

pub(super) fn build_patch(
    suggestion: &Suggestion,
    sources: &BTreeMap<std::path::PathBuf, SourceFile>,
    root: &Path,
) -> Result<String, String> {
    if suggestion.replacements.is_empty() || suggestion.replacements.len() > 1024 {
        return Err("Suggestion has no replacements or exceeds the replacement limit".into());
    }
    let deadline = Instant::now() + Duration::from_secs(2);
    let mut files: BTreeMap<&Path, Vec<&Replacement>> = BTreeMap::new();
    for replacement in &suggestion.replacements {
        files
            .entry(&replacement.span.path)
            .or_default()
            .push(replacement);
    }
    let mut patch = String::new();
    let mut total = 0usize;
    for (path, mut edits) in files {
        let old_name = patch_path(root, path, "a")?;
        let new_name = patch_path(root, path, "b")?;
        let source = sources.get(path).ok_or("Source file is unavailable")?;
        let before = source.text.as_deref().ok_or_else(|| {
            source
                .reason
                .clone()
                .unwrap_or_else(|| "Source file is unavailable".into())
        })?;
        if before.contains('\0') || edits.iter().any(|edit| edit.text.contains('\0')) {
            return Err("Source or replacement contains binary NUL data".into());
        }
        total = total.saturating_add(before.len());
        for edit in &edits {
            total = total.saturating_add(edit.text.len());
        }
        if total > MAX_BYTES || before.bytes().filter(|&b| b == b'\n').count() > 100_000 {
            return Err("Suggestion exceeds the diff size limit".into());
        }
        edits.sort_by_key(|r| (r.span.byte_start, r.span.byte_end));
        let mut previous: Option<&Replacement> = None;
        for &edit in &edits {
            if Instant::now() >= deadline {
                return Err("Suggestion exceeded the diff time limit".into());
            }
            let original_lines = edit
                .span
                .line_end
                .checked_sub(edit.span.line_start)
                .and_then(|lines| lines.checked_add(1));
            if original_lines != Some(edit.span.text.len()) {
                return Err(
                    "Diagnostic does not include the complete original source snippet".into(),
                );
            }
            if !span_matches(source, &edit.span) {
                return Err(
                    "Source no longer matches the diagnostic coordinates or snippet".into(),
                );
            }
            if let Some(prev) = previous
                && (edit.span.byte_start < prev.span.byte_end
                    || edit.span.byte_start == prev.span.byte_start)
            {
                return Err("Suggestion contains overlapping or ambiguous replacements".into());
            }
            previous = Some(edit);
        }
        let mut after = before.to_owned();
        for edit in edits.into_iter().rev() {
            after.replace_range(edit.span.byte_start..edit.span.byte_end, &edit.text);
        }
        if before == after {
            continue;
        }
        let diff = similar::TextDiff::configure()
            .deadline(deadline)
            .diff_lines(before, &after);
        patch.push_str(
            &diff
                .unified_diff()
                .context_radius(3)
                .header(&old_name, &new_name)
                .to_string(),
        );
        if Instant::now() >= deadline {
            return Err("Suggestion exceeded the diff time limit".into());
        }
        if patch.len() > MAX_BYTES {
            return Err("Suggestion exceeds the patch size limit".into());
        }
    }
    if patch.is_empty() {
        return Err("Suggestion does not change the source".into());
    }
    Ok(patch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostic::model::{Replacement, SnippetLine, Span, Suggestion};
    use crate::source::SourceFile;
    use std::{collections::BTreeMap, path::Path};

    fn replacement(path: &Path, source: &str, start: usize, end: usize, text: &str) -> Replacement {
        let coordinate = |offset: usize| {
            let prefix = &source[..offset];
            let line = prefix.bytes().filter(|&b| b == b'\n').count() + 1;
            let column = prefix.rsplit('\n').next().unwrap().chars().count() + 1;
            (line, column)
        };
        let (line_start, column_start) = coordinate(start);
        let (line_end, column_end) = coordinate(end);
        Replacement {
            span: Span {
                path: path.into(),
                byte_start: start,
                byte_end: end,
                line_start,
                line_end,
                column_start,
                column_end,
                primary: true,
                label: None,
                expansion: None,
                text: source
                    .split('\n')
                    .skip(line_start - 1)
                    .take(line_end - line_start + 1)
                    .map(|s| SnippetLine {
                        text: s.trim_end_matches('\r').into(),
                        highlight_start: 0,
                        highlight_end: 0,
                    })
                    .collect(),
            },
            text: text.into(),
            applicability: "MachineApplicable".into(),
        }
    }
    fn suggestion(replacements: Vec<Replacement>) -> Suggestion {
        Suggestion {
            message: "fix".into(),
            child_path: vec![0],
            replacements,
        }
    }
    fn source(text: &str) -> SourceFile {
        SourceFile {
            display_path: "test.rs".into(),
            text: Some(text.into()),
            reason: None,
        }
    }
    fn apply(patch: &str, files: &[(&str, &str, &str)]) {
        let temp = tempfile::tempdir().unwrap();
        for (name, before, _) in files {
            let path = temp.path().join(name);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(path, before).unwrap();
        }
        let patch_path = temp.path().join("changes.patch");
        std::fs::write(&patch_path, patch).unwrap();
        let output = std::process::Command::new("git")
            .current_dir(temp.path())
            .args(["-c", "core.autocrlf=false", "apply", "--whitespace=nowarn"])
            .arg(&patch_path)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{}\n{patch}",
            String::from_utf8_lossy(&output.stderr)
        );
        for (name, _, after) in files {
            assert_eq!(
                std::fs::read(temp.path().join(name)).unwrap(),
                after.as_bytes()
            );
        }
    }
    #[test]
    fn multipart_and_multiple_files_apply_as_one_deterministic_patch() {
        let root = Path::new("/workspace");
        let a = root.join("src/a.rs");
        let b = root.join("src/한 글.rs");
        let before = "assert!(!items.is_empty());\n";
        let sources = BTreeMap::from([(a.clone(), source(before)), (b.clone(), source("old\n"))]);
        let mut s = suggestion(vec![
            replacement(&b, "old\n", 0, 3, "new"),
            replacement(&a, before, 6, 6, "_ne"),
            replacement(&a, before, 8, 25, "items, []"),
        ]);
        let patch = build_patch(&s, &sources, root).unwrap();
        s.replacements.reverse();
        assert_eq!(patch, build_patch(&s, &sources, root).unwrap());
        apply(
            &patch,
            &[
                ("src/a.rs", before, "assert_ne!(items, []);\n"),
                ("src/한 글.rs", "old\n", "new\n"),
            ],
        );
    }
    #[test]
    fn crlf_unicode_insertion_deletion_and_missing_final_newline_apply_exactly() {
        for (before, start, end, after) in [
            ("한글\r\nold\r\n", 8, 11, "한글\r\nnew\r\n"),
            ("old", 0, 3, "new"),
            ("", 0, 0, "new"),
            ("old\n", 0, 4, ""),
        ] {
            let root = Path::new("/workspace");
            let path = root.join("test.rs");
            let new = if after.is_empty() { "" } else { "new" };
            let s = suggestion(vec![replacement(&path, before, start, end, new)]);
            let patch = build_patch(&s, &BTreeMap::from([(path, source(before))]), root).unwrap();
            apply(&patch, &[("test.rs", before, after)]);
        }
    }
    #[test]
    fn bad_member_disables_the_entire_suggestion() {
        let root = Path::new("/workspace");
        let path = root.join("a.rs");
        let sources = BTreeMap::from([(path.clone(), source("abcdef\n"))]);
        let a = replacement(&path, "abcdef\n", 1, 4, "X");
        let b = replacement(&path, "abcdef\n", 3, 5, "Y");
        assert!(build_patch(&suggestion(vec![a.clone(), b]), &sources, root).is_err());
        let insert = replacement(&path, "abcdef\n", 2, 2, "X");
        assert!(build_patch(&suggestion(vec![insert.clone(), insert]), &sources, root).is_err());
        let mut stale = a.clone();
        stale.span.text[0].text = "stale".into();
        assert!(build_patch(&suggestion(vec![stale]), &sources, root).is_err());
        let mut missing = a.clone();
        missing.span.path = root.join("missing.rs");
        assert!(build_patch(&suggestion(vec![a, missing]), &sources, root).is_err());
        for unsafe_path in [
            "/outside.rs",
            "/workspace/../outside.rs",
            "/workspace/a\n--- injected.rs",
        ] {
            let p = Path::new(unsafe_path);
            let s = suggestion(vec![replacement(p, "old", 0, 3, "new")]);
            assert!(build_patch(&s, &BTreeMap::from([(p.into(), source("old"))]), root).is_err());
        }
    }
    #[test]
    fn missing_or_partial_original_snippets_cannot_patch_stale_source() {
        let root = Path::new("/workspace");
        let path = root.join("a.rs");
        for (original, current, keep) in [
            ("abcdef\n", "abcXYZ\n", 0),
            ("first\nmiddle\nlast\n", "first\nedited\nlast\n", 1),
        ] {
            let mut edit = replacement(&path, original, 0, original.len(), "replacement\n");
            edit.span.text.truncate(keep);
            let result = build_patch(
                &suggestion(vec![edit]),
                &BTreeMap::from([(path.clone(), source(current))]),
                root,
            );
            assert!(
                result.is_err(),
                "unverified original must not yield a patch: {result:?}"
            );
        }
    }
    #[test]
    fn distant_edits_with_multiline_replacement_keep_hunk_offsets() {
        let root = Path::new("/workspace");
        let path = root.join("a.rs");
        let before = (0..30).map(|i| format!("line{i}\n")).collect::<String>();
        let first = before.find("line2\n").unwrap();
        let second = before.find("line25\n").unwrap();
        let s = suggestion(vec![
            replacement(&path, &before, first, first + 5, "new2\nextra"),
            replacement(&path, &before, second, second + 6, "new25"),
        ]);
        let after = before
            .replace("line2\n", "new2\nextra\n")
            .replace("line25\n", "new25\n");
        let patch = build_patch(&s, &BTreeMap::from([(path, source(&before))]), root).unwrap();
        assert!(patch.matches("@@").count() >= 4);
        apply(&patch, &[("a.rs", &before, &after)]);
    }
    #[test]
    fn no_op_binary_and_oversized_suggestions_have_no_patch() {
        let root = Path::new("/workspace");
        let path = root.join("a.rs");
        for text in ["old", "old\0data", &"a".repeat(MAX_BYTES + 1)] {
            let unchanged = suggestion(vec![replacement(&path, text, 0, 3, &text[..3])]);
            assert!(
                build_patch(
                    &unchanged,
                    &BTreeMap::from([(path.clone(), source(text))]),
                    root
                )
                .is_err()
            );
        }
        let binary = "old\0data";
        let s = suggestion(vec![replacement(&path, binary, 0, 3, "new")]);
        assert!(build_patch(&s, &BTreeMap::from([(path.clone(), source(binary))]), root).is_err());
    }
}
