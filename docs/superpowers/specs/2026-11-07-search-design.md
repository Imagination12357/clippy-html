# Report search: pinned rustdoc interface, report-owned engine

## Scope and authority

Approved A1: follow rustdoc's in-place search flow; implement matching for report
data ourselves. The user specifically requested static analysis of generated
rustdoc output before adapting its UI. This extends the current codex/search
checkout; it adds no backend, build-time frontend toolchain or remote resource.

## Reference evidence

Reference: this project's own `cargo doc --no-deps --document-private-items`
output, rustdoc 1.97.1, Rust commit
`8bab26f4f68e0e26f0bb7960be334d5b520ea452`.

| Generated asset | SHA256 |
|---|---|
| rustdoc-17e0aaed.css | 264dc5a58dbfaf73bca6f1b5767413c775d1150c07855f6b399e8672ae3d9597 |
| main-fcd733ba.js | baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db |
| search-fd9372ac.js | f765706d62d88eb8f83ca79eaba8158512a0d8f9955190d1c4f5313c675109a3 |

Local inputs: target/doc/cargo_clippy_html/index.html,
target/doc/src/cargo_clippy_html/main.rs.html and target/doc/static.files/.
The generated HTML contains an initially empty rustdoc-toolbar; main.js builds
the search input/view and search.js builds the results. Reading only static
HTML would miss most of the interface.

| Reference operation | Generated structure / behavior | Report mapping |
|---|---|---|
| Toolbar | rustdoc-toolbar, #search-button > a, span.label; CSS supplies magnifier | Same structure and embedded icon; omit unrelated settings/help controls |
| Enter search | S or /; Search becomes Exit; same toolbar moves to search heading | Same, ignoring editable targets, modifiers and IME composition |
| Display | #main-content hidden; #alternative-display.content shown | Same in-place sibling section; original DOM remains intact |
| Input | .main-heading.search-results-main-heading > nav.sub > form.search-form > span + input.search-input | Same classes/structure; accessible report-specific label, no unsupported '?' hint |
| Heading | .search-switcher contains Search results | Same text; no API crate selector |
| Results | .search-out > #search-tabs + #results; selected buttons and counts | Same components, tabs Lints / Files / Findings / Categories |
| Row | direct anchor > span.result-name (span.typename + div.path), div.desc | Same visual structure; plain report kind/name/description |
| Submit | prevent default; execute current query | Same; Enter does not unexpectedly navigate |
| Keyboard | Down from input; Up/Down between rows; Left/Right switches tabs | Same; standard Tab and Enter on links remain available |
| Delay | input schedules query after 500ms | Same, Enter immediate; composition waits for completion |
| Close | Escape or Exit restores main content and label | Also explicitly restore saved scroll and focus |
| URL | ?search= query; push on entry, replace while typing; popstate restore | Same search parameter, preserve hash and unrelated parameters; history optional on file URLs |

The existing vendored stylesheet already contains the input, tab, count, row,
responsive and magnifier rules. Do not recreate their colors, spacing or fonts.
Native result containers are ul with direct anchor children; use div with the
same classes/direct anchors, and span.path inside span.result-name, to avoid
invalid content models without changing CSS. Initially choose the first nonempty
report tab so a file-only match is visible without an extra click.
Add only scoped corrections for omitted toolbar controls and the source input's
40px offset (the report has a unified sidebar, no rustdoc source-sidebar button).
No cards, dropdown search overlay, new filter bar, or speculative controls.

## Report data and matching

Generate one classic JavaScript index with four entry kinds:

- Lint: one per existing lint group, including compiler/general diagnostics;
  title is its code/label and description includes occurrence count.
- File: one per existing source page, including snippet fallback pages; use its
  display path rather than the generated filename.
- Finding: one per deduplicated diagnostic; title is its message, context includes
  code, level, package, category and primary file/location. Search child notes
  and suggestion messages too, but never index full source bodies/replacements.
- Category: one per existing category page, including Unknown when present.

All hrefs come from PageMap, relative to index.html, and finding links use
existing lint-page finding anchors. Do not change IDs or normalization.
Use NFC and lower case, split whitespace, require every term to occur across
entry fields. Rank exact name/path first, then prefix, substring, context-only;
ties keep deterministic generator order. No API type syntax or typo engine.
Empty input renders zero counts and a short prompt; unmatched input gets an
explicit no-results message. Show at most 100 rows per tab, retain total counts,
and say when the list is truncated. Never silently omit matching entries.

## Safety, portability and degradation

Emit search-index.js, search-engine.js and search.js as ordered deferred classic
scripts, using page-relative asset URLs. No fetch, modules, workers, CDN or
storage requirement. Data is JSON with script-sensitive characters escaped;
dynamic labels/messages/query are inserted only with textContent. Resolve only
generator-owned local hrefs against the report root. Validate index hrefs and
anchors plus script resources before staged publication.

The toolbar stays hidden until scripts and index initialize successfully;
without JavaScript or with missing assets, existing navigation stays usable.
Keep the main content DOM, sidebar, source tree and finding anchors. History API
failures cannot prevent opening/closing/searching. Native settings/help, crate
selector, type signatures, deprecated filters and Stringdex are out of scope.

## Verification boundary

Rust tests: index kinds/counts/dedup, safe serialization, Unicode/path links,
script presence, missing/escaping script resources, bad search anchor rejection.
Node tests: matching/ranking, Unicode, empty/no-match, counts/limit; DOM tests
where available for state, shortcuts, XSS-as-text, history and focus restoration.
Compare emitted classes/structure to the pinned reference rather than designing
new presentation. Run full Cargo tests, fmt and strict Clippy; generate a real
offline report and validate every search destination.

Static/DOM tests do not establish actual browser rendering, font metrics or
file:// acceptance. Local-file browser automation remains blocked; do not work
around it. Manual desktop/narrow-screen browser acceptance remains explicit.
