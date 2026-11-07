# Search interface provenance

Reference: this project's generated rustdoc 1.97.1 output, Rust commit
8bab26f4f68e0e26f0bb7960be334d5b520ea452. Source is
https://github.com/rust-lang/rust/tree/8bab26f4f68e0e26f0bb7960be334d5b520ea452/src/librustdoc/html/static
under the MIT option; original notice: LICENSE-MIT, copied to
licenses/rustdoc-MIT.txt in reports. Rust Project Contributors retain their
copyright. The generator's own license remains MIT OR Apache-2.0.

Generated reference files and SHA256:

- main-fcd733ba.js: baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db
- search-fd9372ac.js: f765706d62d88eb8f83ca79eaba8158512a0d8f9955190d1c4f5313c675109a3
- rustdoc-17e0aaed.css: 264dc5a58dbfaf73bca6f1b5767413c775d1150c07855f6b399e8672ae3d9597

Adapted: the toolbar/search form/result DOM vocabulary, in-place main/alternative
display, moving the toolbar, Search/Exit labels, S or / and Escape shortcuts,
500ms input scheduling, search query history, tab counts and arrow navigation.
The original input/tab/result/magnifier styles remain in the vendored stylesheet.
Search behavior is implemented in assets/search.js; the complete upstream
JavaScript files, Stringdex and rustdoc API search index are not distributed.

Changes for this report:

- In Lints, In Files, In Findings and In Categories replace API name/parameter/return tabs.
- Classic ordered local scripts replace upstream lazy script/engine loading.
- Report-owned search-engine.js performs NFC/case-normalized AND substring
  matching and deterministic ranking; it is not rustdoc's query/type engine.
- Results use textContent and generator-owned URLs, no trusted-HTML descriptions.
- div result panels and span.path preserve the same CSS selectors while avoiding
  ul/anchor and span/div content-model mismatches in the generated structure.
- No settings/help/crate selector/type signature/deprecation controls.
- Scoped CSS removes unused mobile toolbar slots and the source input offset
  associated with rustdoc's separate source-sidebar toggle.
- Explicit focus/scroll restoration, IME guards, optional history, plain empty
  states, first nonempty tab selection and a stated 100-row-per-tab display cap.
- Navigation remains available without scripts; toolbar activates after setup.
- Same-document main-content anchors close search before native navigation, so
  the unified report sidebar and same-page finding results reveal their targets.
- Empty/no-match panels reuse .search-failed and its centered layout, the native
  Example searches: list and No results :( text. Examples come from report lint,
  file and category names (representative queries for empty reports). External
  web-search/language-documentation suggestions are omitted; examples execute
  the report-owned search and preserve ordinary modified-click link behavior.

Detailed static-analysis mapping and verification limits are in the repository's
docs/superpowers/specs/2026-11-07-search-design.md and
docs/validation/2026-11-07-search.md. No pixel/browser equivalence is claimed.
