# Pinned rustdoc Settings port

Current correction: the user compared docs.rs (rustdoc 1.102 nightly), which has
Appearance / Interface / Content tabs and a different checkbox order. Settings
alone now follows that actual grouped DOM and its extracted CSS; global CSS and
theme state stay at the prior reference. See SETTINGS.md for exact version/hashes.
The original 1.97.1 mapping below records the initial implementation, not a claim
that all rustdoc versions share that layout. Impeccable is prohibited for this
project by the user's instruction, recorded in root AGENTS.md.

User direction: include Ayu, port the pinned rustdoc structure as closely as
possible, add no speculative controls. Local main and autonomous commits remain
approved. No remote operation or browser-policy workaround.

## Static reference

This project's own rustdoc 1.97.1, Rust commit
8bab26f4f68e0e26f0bb7960be334d5b520ea452:

- target/doc/settings.html: standalone settings shell and Back link.
- settings-170eb4bf.js SHA256 7a5bc91b83a8c9e590f3ed56fe7a0ea5c6f3bdce5a37fd5c785fcaeab74b2b03
- storage-41dd4d93.js SHA256 41ae6c52806867b0888b909a815fd147c2b012b10211a772747f49c10f3fece9
- main-fcd733ba.js: toolbar activation, modal dismissal and search settings.
- Existing vendored rustdoc.css: Settings gear, popover, radio/checkbox styles,
  light/dark/ayu variables, layout and setting-dependent classes.

The settings DOM is built by JavaScript, not present as form controls in the
generated settings.html. Reuse #settings(.popover), .settings, .setting-line,
.setting-radio-name, .setting-radio-choices, label.setting-radio/input/span and
label.setting-check/input/span. Normal pages attach the popover to .settings-menu;
settings.html appends a section to #main-content. Preserve input names/values,
order, defaults, labels and preferred-theme visibility. No Save/reset control.

## Exact settings mapping

| Native name | Action |
|---|---|
| Theme | Port light, dark, ayu, system preference; default system preference |
| Preferred light theme | Port light/dark/ayu choices, default light |
| Preferred dark theme | Port light/dark/ayu choices, default dark; Ayu may be selected |
| Auto-hide item contents for large items | Omit: no API item-content model |
| Auto-hide item methods' documentation | Omit: no API method documentation |
| Auto-hide trait implementation documentation | Omit: no trait implementation documentation |
| Directly go to item in search if there is only one result | Port: exactly one report result across all four kinds |
| Show line numbers on code examples | Omit: no code-example UI; source line numbers are structural navigation |
| Hide persistent navigation bar | Port to report sidebar; Settings remains available to restore it |
| Hide table of contents | Port to existing #rustdoc-toc; file tree remains separate |
| Hide module navigation | Omit: no #rustdoc-modnav/module navigation model |
| Disable keyboard shortcuts | Port to document search/settings shortcuts; ordinary control activation remains |
| Use sans serif fonts | Port Fira Sans body mode; retain bundled Source Code Pro for code, not unbundled Fira Mono |
| Word wrap source code | Port native .word-wrap-source-code behavior |
| Hide deprecated items | Omit: no deprecated API-item model; do not hide diagnostics |

## State, integration and necessary adaptations

An ordered synchronous local settings-storage.js in the head applies the stored
theme before styles/paint; all three themes already exist in rustdoc.css. Match
native use-system-theme, preferred-light-theme and preferred-dark-theme semantics
and follow prefers-color-scheme changes. Use clippy-html- storage keys to avoid
changing rustdoc preferences for unrelated documentation. Invalid stored values
fall back to defaults. Storage denial still allows current-page changes using
in-memory values; persistence across file URLs is browser-dependent.

Deferred settings.js builds the original form structure and popup/page behavior.
Button toggles, outside click/focus dismissal, Escape and ordinary modified-click
navigation to settings.html remain. Search and settings share the existing moving
toolbar. Escape follows the native document shortcut policy. Settings and Search
controls activate independently only when their implementation is ready.
Direct-result navigation removes the search query before following the link to
avoid returning to an automatic navigation loop.

The report intentionally has a unified sidebar and no native mobile topbar,
Help/Summary buttons or separate source-sidebar toggle. Keep that approved shell;
scope only needed CSS corrections for the Settings button's visibility, popup
arrow/viewport sizing, sidebar hide specificity and absent sidebar-toggle offset.
No new visual component or theme palette. Existing diagnostic range colors use
theme variables and must remain readable in dark and Ayu. The severity badge's
foreground may use existing theme text to avoid fixed light-theme colors there.

The standalone page uses the existing report shell, native settings heading/Back
placement and noscript explanation. Back has a valid report-index href and uses
history only for a verified same-report predecessor; no javascript: URL.

## Validation

Node/jsdom: all three fixed themes, system/default and preferred Ayu, media changes,
reload/nested-page storage, denial/invalid values; exact control labels/order/DOM;
popup toggle/outside/focus/Escape and modified clicks; settings.html; search popup
interaction, disabled shortcuts and single-result behavior. Rust integration:
settings page, script order/local paths, synchronous boot, no inline scripts,
license notices and broken-asset publication rejection. Full Cargo + Node suites,
fmt, strict Clippy and a real offline report. Fresh final review. Static DOM/CSS
checks do not establish real-browser rendering/file-URL/OS theme behavior.
