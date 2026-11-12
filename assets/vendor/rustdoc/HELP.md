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

Keyboard list adapts only implemented commands: ?, S / /, search-result arrows,
Enter and Escape. Search Tricks describes report data, AND terms and
case-insensitive matching. Rustdoc's type/API search syntax, handbook links,
version footer and +/-/_ section commands are omitted. No new report keyboard
commands for settings, copying or navigation were invented.

Help and Settings popovers exclude each other while search stays open beneath
them. Capture-phase popup Escape consumption plus defaultPrevented checks leave
search open until the next Escape. Search opening closes popovers. Form input,
IME composition and browser modifier keys are protected. Disable keyboard
shortcuts suppresses the report's custom global and arrow handlers; native
Tab/Enter and explicit buttons/links still work.

Settings remains last in the toolbar. Help's scoped arrow offsets account for
that following button, including rustdoc's compact 850px button breakpoint.
Popup positioning uses the existing 700px mobile breakpoint. The report overrides the
native mobile Help-link hiding rule because it does not have rustdoc-topbar.
Long popup content scrolls inside .content rather than clipping its arrow.
Actual layout and assistive-technology behavior require separate validation.
