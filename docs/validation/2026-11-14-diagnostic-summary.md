# Diagnostic Summary validation — 2026-11-14

## Approved behavior and reference

A1: preserve severity/code/message/representative location while diagnostic
details collapse. Reference is this project's pinned rustdoc 1.97.1 managed
toggle structure and Summary/Show all action. Exact artifacts, licenses and
adaptations are in [SUMMARY.md](../../assets/vendor/rustdoc/SUMMARY.md).

Each existing finding section/id contains an initially open native toggle.
The representative location is first primary, otherwise first available, and
appears in the collapsed summary only to avoid duplication in the open view.
No location placeholder is invented for spanless findings. Summary/Show all
controls that finding scope, including nested targets, suggestions and original
diagnostic text. Source tree, full source, overview logs and other details are
excluded. No new per-finding label, legend, persistence setting or dependency.

The JS-created native toolbar button follows Help only on finding pages. Mixed
state offers Summary; all closed offers Show all. Search disables/hides it and
restores controls without changing finding states. Only while visible does its
80px width contribute to Help/Settings arrow offsets. The 850px compact-button
breakpoint and 700px popup positioning breakpoint stay distinct.

Native individual toggles work without JS. - and +/= share the explicit action,
with input/IME/modifier/handled-event/disabled-key guards; _ is not implemented.
Focused body content moves to its visible summary on global collapse. Hashes,
history and ordinary same-document links reveal only their finding/ancestors.
Modified/external/download/new-tab links keep native behavior.

## Local automated evidence (Windows)

- Initial DOM tests failed because summary.js did not exist; the integration
  test failed because findings were not native summary toggles. They passed
  after implementation.
- Eleven new DOM tests cover scoped nested collapse/expand, source/tree/log
  preservation, manual/mixed state, keyboard guards and disabled-key explicit
  controls, focus on collapse, nested initial/history links, same-hash and
  modified/external/source links, initial-query search disable/close, pages
  without findings and same-document search-result reveal.
- Integration verifies identity and primary location survive inside summary,
  body content remains present, no-JS findings start open and the global button
  is absent until initialized. Existing checks now verify eight local scripts
  and Help's eight actual command rows (+/= and - added; Escape row stays absent).
- `cargo build --example cargo_probe --locked`, `cargo test --locked`:
  71 unit + 15 integration tests passed. `npm test`: 49 passed.
- fmt, strict all-target Clippy and Rust 1.88 all-target locked check passed.
- Generated target/diagnostic-summary-preview through online-default pedantic
  dogfooding; internal HTML/relative-link/asset validation passed. jsdom loaded
  its actual scripts on lint/clippy/assert_is_empty.html, summarized all five
  findings and expanded every controlled diagnostic detail again. Summary text
  retained the message and src/diagnostic/dedup.rs:115:9 location.

The user had already reported Pages smoke success and suggestion diff/copy
layout and behavior as normal. Those are recorded as completed user acceptance,
not repeated here or treated as screen-reader evidence.

## Remaining evidence

Fresh-context review found two material issues: the wrapper missed native
mobile/source chevron gutter selectors, and expansion could hide a focused
compact-location link. Both regressions failed before the corrections, then
passed. Source/mobile findings now reserve a 20px gutter for the 16px chevron;
computed-style testing verifies that the source selector applies independently
of main padding, with the mobile counterpart verified by CSS inspection.
Expansion and target reveal move affected focus to the visible summary before
hiding its location. The full Rust/Node suites, fmt, Clippy and MSRV passed
after the fixes. No browser rendering pass follows from these checks.

No live-browser rendering, native anchor scrolling, popup geometry or
assistive-technology runtime pass is claimed by jsdom/source checks. This new
Summary view still needs browser acceptance. No push or hosted deployment was
performed. Metadata export remains a separate next implementation milestone.
