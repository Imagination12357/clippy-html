# Help and shortcut provenance

Reference: this project's generated rustdoc 1.97.1, Rust commit
8bab26f4f68e0e26f0bb7960be334d5b520ea452. Inspected artifacts:

- help.html: standalone main-content, heading and Back link.
- main-fcd733ba.js: buildHelpMenu, showHelp, handleShortcut and help dismissal.
  SHA-256 baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db.
- rustdoc-17e0aaed.css: .help-menu, #help.popover, .content, .side-by-side,
  .shortcuts dl/dt/dd/kbd and responsive layout. Existing Help icon is retained
  from the already-vendored CSS; no additional image asset was copied.

Source: https://github.com/rust-lang/rust/tree/8bab26f4f68e0e26f0bb7960be334d5b520ea452/src/librustdoc/html/static
under the MIT option. Original Rust Project Contributors notice and full
license remain in LICENSE-MIT / generated licenses/rustdoc-MIT.txt.

The report renders the shared Help partial into standalone help.html and an
inert template. help.js clones that template into the toolbar popover. A
normal Help link remains usable without JavaScript; modified clicks navigate
normally. Static Back falls back to the overview; JavaScript uses history only
for a previous page within the same report.

Keyboard list adapts only implemented commands: ?, S / /, search-result arrows
and ⏎. Escape behavior remains implemented without an additional Help row,
matching the reference. The native span.bottom > code structure displays the
generator's own cargo-clippy-html package version (CARGO_PKG_VERSION), not a
rustdoc/compiler or inspected-crate version. Search Tricks describes report data, AND terms and
case-insensitive matching. Rustdoc's type/API search syntax, handbook links,
rustdoc version text is omitted. The later diagnostic Summary control adds
+/= and - to Help; _ stays omitted because no impl blocks exist. No new report keyboard
commands for settings, copying or navigation were invented.

Help and Settings popovers exclude each other while search stays open beneath
them. Capture-phase popup Escape consumption plus defaultPrevented checks leave
search open until the next Escape. Search opening closes popovers. Form input,
IME composition and browser modifier keys are protected. Disable keyboard
shortcuts suppresses the report's custom global and arrow handlers; native
Tab/Enter and explicit buttons/links still work.

Toolbar order is Search, Settings, Help, following the pinned reference's
RustdocToolbarElement in storage-41dd4d93.js (SHA-256
41ae6c52806867b0888b909a815fd147c2b012b10211a772747f49c10f3fece9).
The following Summary control appears only on finding pages, as described in
SUMMARY.md. Popup offsets account for that control when visible and revert
during search, including the compact 850px Help/Settings breakpoint.
Popup positioning uses the existing 700px mobile breakpoint. The report overrides the
native mobile Help-link hiding rule because it does not have rustdoc-topbar.
Long popup content scrolls inside .content rather than clipping its arrow.
Actual layout and assistive-technology behavior require separate validation.

2026-11-12 follow-up: the user compared docs.rs rustdoc 1.101.0-nightly and
identified the unnecessary Escape row, the Enter label instead of ⏎ and the
missing generator version footer. The same conventions are present in the
pinned 1.97.1 artifact, so no global style migration is needed. The live
https://docs.rs/clap/latest/clap/ page checked on this date instead declares
1.102.0-nightly (f5cedb83a 2026-11-07); its
https://docs.rs/-/rustdoc.static/main-b2682d9c.js also confirms those conventions.
The exact user-viewed 1.101 artifact was not fetched; do not treat the live
1.102 artifact as that build.
