# Rustdoc source tree correction plan

**Goal:** Apply the vendored rustdoc source-tree CSS through its native HTML structure, preserving finding counts and separate findings navigation.

**Design:** Replace custom `ul/li` tree markup with `#src-sidebar`, `details.dir-entry`, a summary, `div.folders` followed by `div.files`, and `a.selected` for the active file. Emit folders before files. Generate this HTML statically; do not introduce rustdoc JavaScript. Preserve the existing source paths, external-source namespace, open ancestors and per-file counts.

**Cause:** The previous tree used independent markup and custom spacing/selection CSS, so rustdoc's existing source sidebar rules did not match it.

1. [x] Add generated-report assertions for native directory/file containers and selected-file styling, and observe failures.
2. [x] Separate directory/file rendering, replace template wrappers, and remove redundant tree CSS. Keep only report-specific counts and collapsible section styling.
3. [x] Run the full tests, fmt, strict Clippy and Rust 1.94 check; regenerate previews, record results and commit.

**Validation limit:** Automated file:// browser access remains unavailable. DOM/link checks establish structure and navigation; visual confirmation remains manual.

## Results

- New native-structure assertions failed in both the actual warning report and external/workspace tree regression before implementation; both now pass.
- Folder and file HTML is emitted into separate containers in rustdoc order. The active file keeps aria-current and also receives rustdoc's selected class.
- Moved the source tree outside sidebar-elems so general report-navigation link padding cannot override source-tree spacing. Other report pages retain their sidebar structure.
- Removed custom tree list padding, link wrapping and active-file color/weight rules. Existing rustdoc CSS supplies these source-tree styles; counts and independent collapsible report sections remain additions.
- cargo test --locked: 37 unit + 7 integration passed. Strict Clippy and Rust 1.94 locked check passed. Final format/link checks and preview refresh run before commit.
