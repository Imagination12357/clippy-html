# Source navigation validation

## Behavior

- Workspace source pages preserve recognizable relative directories and filenames under `src/`. An initial workspace `src/` is omitted to avoid `src/src/`; workspace members retain their directory prefixes.
- Unsafe filename characters are escaped with `~HH`. Device names, trailing dots, long components and directories ending in `.html` are protected. External/traversal-like paths use a separate output namespace. Case aliases receive deterministic suffixes; a conservative Unicode case key can suffix more names than a particular filesystem requires.
- All hrefs use relative URL paths with percent-encoded non-ASCII bytes. Nested pages resolve home, assets, licenses and diagnostic anchors; publication validates links recursively.
- The repeated path paragraph is removed. The heading retains the source path; diagnostic locations still identify their own spans.
- Files and Findings in this file are independently collapsible native HTML sections. Files shows diagnostic source paths, per-file unique finding counts, and the current file. Current ancestors open by default; other directories remain collapsed. External sources are separately grouped to preserve file identity.
- Finding links jump to source lines when coordinates match. Missing/changed/external full source falls back to finding anchors and diagnostic snippets.

## Evidence

- The initial readable-source regression failed against hashed names; after path changes it reached the expected duplicate-paragraph and missing-tree failures, then passed.
- Source naming tests cover Unicode, reserved characters/devices, ASCII case collisions, file/directory conflicts, external and traversal-like paths.
- Independent review found Unicode Windows aliases (`σ` / `ς`), external/workspace sidebar leaf collision, and repeated count aggregation. The first two were reproduced by failing regressions and fixed. Counts are now aggregated once before the source-page loop; a behavioral test checks one count per finding per file despite repeated spans. The performance improvement is established by the loop structure, not a timing benchmark.
- Supplementary tests check nested-page broken anchors, escaped hostile tree labels, and missing-source navigation. They were added after their corresponding implementation and are not claimed as separate RED/GREEN cycles.
- Final commands/results are recorded in the implementation plan's Progress section.

## Artifacts and limits

- Project report: `target/rustdoc-style-preview/index.html`; example source: `target/rustdoc-style-preview/src/diagnostic/normalize.rs.html`.
- Workspace fixture report: `target/phase-1-preview/index.html`.
- Comparison docs: `target/doc/cargo_clippy_html/index.html`.
- Browser file URLs remain blocked by the available automation surface. HTML parsing and local link validation passed; visual layout and cross-browser behavior require manual confirmation. No server workaround was used.
- Source filenames become readable; lint-group HTML names and internal finding IDs remain hashed in this change.
- The tree lists files represented in diagnostics, rather than every workspace source file. No JavaScript or new dependencies were added.

## Rustdoc tree style correction

User feedback identified that the initial hierarchy used a custom list layout. Replaced it with rustdoc's `#src-sidebar`, `details.dir-entry`, `summary`, `div.folders`, `div.files` and `a.selected` structure. Folders precede files; native rustdoc CSS controls indentation, hover and selection. The tree is outside `.sidebar-elems` to avoid its general navigation-link padding. Counts, current-file accessibility, open current ancestors and independent findings navigation remain. Generated-report regressions failed before this correction and pass afterward; browser visual confirmation remains manual.

Screenshot follow-up: intentionally align the Files section with the combined report sidebar using `padding-left: var(--sidebar-elems-left-padding)` on `.source-files` only. This shares the existing sidebar inset without inheriting `.sidebar-elems` link rules; native tree indentation and selection remain intact.
