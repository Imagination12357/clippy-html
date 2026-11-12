# Help and shortcut validation — 2026-11-12

## Scope and reference

User approved ?/Help plus coordination of existing search/arrow/Enter/Escape
behavior. Summary/Show All and + / = / - / _ remain a separate decision.
Help follows this project's pinned rustdoc 1.97.1 generated help.html,
main-fcd733ba.js and CSS, with report-specific search text. See
[HELP.md](../../assets/vendor/rustdoc/HELP.md) for source/license and adaptations.

The same static Askama partial is rendered into help.html and an inert template
cloned by help.js. The Help link is visible without JS; search/settings controls
still wait for their respective scripts. Standalone Settings/Help share the
existing Back layout, now named standalone_page internally. No query engine,
new dependency, general shortcut registry or unrelated UI feature was added.

Help and Settings exclude each other. Their Escape handlers run before the
search listener and consume the event, so a second Escape is required to close
search. Search opening closes popovers. Composition state, isComposing/keyCode
229, handled events and browser modifiers guard custom keys. Editable controls
keep normal text; checkbox/radio globals follow the reference convention.
Disable keyboard shortcuts suppresses custom global and arrow behavior;
explicit buttons/links and browser-native Tab/Enter remain available.

## Automated evidence (Windows)

- Initial Help tests failed because the template/script did not exist.
- Composition regressions reproduced search activation and Settings Escape
  during composition; document-level composition tracking fixed them.
- A source page named help.html reproduced incorrect standalone detection;
  testing the static Help DOM instead of a URL suffix fixed it.
- Existing combined Settings/search test was updated to the approved one-layer
  Escape behavior. Generated script checks now verify seven local scripts.
- Eight Help DOM tests cover native structure and actual command list, absent
  search data, denied storage, layered Escape/focus, popup exclusion, search
  activation, modified clicks, internal focus transitions, outside dismissal,
  editable content/IME/modifiers/handled events, disabled keys, static standalone
  content and the source-name collision.
- The test harness observes native modified-link handling before suppressing
  jsdom's unimplemented navigation. It drains queued focusout microtasks before
  closing a window; callbacks on destroyed jsdom documents are not product bugs.
- Rust integration verifies static Help content, Back, nested Help links and
  inert templates; the normal internal validator checks relative links/assets.
- Local Rust unit/integration tests, Node tests, fmt, strict Clippy and the
  Rust 1.88 all-target locked check passed. Final totals and generated preview
  inspection are recorded after the final run below.

Final local totals: `cargo test --locked` passed 71 unit + 13 integration tests;
`npm test` passed 38 tests. `cargo fmt --check`, strict all-target Clippy and
`cargo +1.88.0 check --all-targets --locked --target-dir target/msrv-1.88` passed.
`target/help-shortcuts-preview` was generated with online-default pedantic
self-dogfooding and passed internal report validation. jsdom loaded all seven
actual generated scripts and verified search → Help → Escape (search stays
open) → Escape (search closes). This is DOM evidence, not a layout screenshot.

## Remaining acceptance

Fresh-context review found no new correctness blocker. It identified one
minor porting mismatch: native buttons become compact at 850px while the new
Help pointer initially changed at 700px. The new Help offset was corrected at
850px, keeping positioning at 700px; CSS parsing and upstream source inspection
confirm the declarations. No browser geometry claim follows from this check.
Pre-existing Settings breakpoint/source-name issues were kept outside this
change. Native popover behavior intentionally has no modal focus trap.

DOM checks do not establish popup geometry, responsive wrapping, actual browser
keyboard defaults, file:// policy or assistive-technology announcement behavior.
No live screen-reader claim is made. No push or hosted CI/Pages update was
performed in this task.

## Follow-up: native key labels and generator version

User compared docs.rs rustdoc 1.101.0-nightly. Removed the added Escape row
without removing Escape behavior, changed Enter's label to ⏎, and restored
span.bottom > code with cargo-clippy-html's own CARGO_PKG_VERSION. The shared
popup/standalone partial receives that value from the generator, independently
of project crate/toolchain/reference versions. These conventions were already
present in pinned 1.97.1; live docs.rs 1.102 also confirms them, as recorded in
HELP.md. The exact user-viewed 1.101 artifact was not retrieved.

Updated tests failed on the old key list/seven rows, then passed with six rows,
the glyph and real compiled package version. All eight Help DOM tests, the
generated Help integration test, fmt and strict all-target Clippy passed.

## Follow-up: toolbar order

User identified reversed Help/Settings buttons. Inspected RustdocToolbarElement
in pinned storage-41dd4d93.js: Search, Settings, Help, then Summary. Matched the
first three and kept Summary omitted. Both popup arrow offsets now account for
that order at desktop and the native 850px compact breakpoint. The integration
test reproduced Search/Help/Settings before the fix and verifies the corrected
order on overview, lint and source pages. No key or popup behavior was changed.
