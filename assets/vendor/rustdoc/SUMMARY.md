# Diagnostic Summary provenance

The reference is this project's rustdoc 1.97.1 output at Rust commit
`8bab26f4f68e0e26f0bb7960be334d5b520ea452`. It includes
`main-fcd733ba.js` (SHA-256
`baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db`),
`storage-41dd4d93.js`, and `struct.Finding.html`. The adaptation uses the MIT
option. The Rust Project Contributors' notice is in `LICENSE-MIT` and
`licenses/rustdoc-MIT.txt` in reports. [Upstream source](https://github.com/rust-lang/rust/tree/8bab26f4f68e0e26f0bb7960be334d5b520ea452/src/librustdoc/html/static)

Findings use rustdoc's `details.toggle > summary > section` structure. The
summary keeps severity, lint code, message, and the first primary location,
falling back to the first available location. When the finding is open, the
detailed locations take its place. Findings start open and can be toggled
individually without JavaScript.

On pages with findings, `summary.js` adds the toolbar button after Help. It
offers **Summary** when any finding is open and **Show all** when all are
closed. Show all opens nested details such as targets, suggestions, and the
original diagnostic text. The action is limited to findings and their nested
details. The sidebar, source listing, and overview logs have their own controls.

Manual toggles update the button. If an action hides the focused element,
focus moves to the finding's summary. A deep link opens its target finding
and any enclosing details. Press `-` to fold or `+` or `=` to expand.
These shortcuts respect editable fields, IME composition, modifier keys, and
the disable-shortcuts setting.

The report uses rustdoc's existing chevron. Source pages and views at or below
700px reserve a 20px gutter so the 16px chevron stays inside the content area.
When Summary is visible, the Help and Settings popup offsets include its 80px
width and button margin, including at the compact 850px breakpoint. Search
temporarily hides Summary and removes that offset.
