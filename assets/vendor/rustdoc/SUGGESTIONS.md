# Suggestion diffs and patch copying

The copy control follows rustdoc 1.97.1 at Rust commit
`8bab26f4f68e0e26f0bb7960be334d5b520ea452`. Its reference is
`main-fcd733ba.js` (SHA-256
`baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db`).
The adaptation uses the MIT option. The Rust Project Contributors' notice is in
`LICENSE-MIT` and ships as `licenses/rustdoc-MIT.txt` in reports.
[Upstream source](https://github.com/rust-lang/rust/tree/8bab26f4f68e0e26f0bb7960be334d5b520ea452/src/librustdoc/html/static/js/main.js)

`assets/suggestions.js` uses the same upper-right button placement with a
**Copy patch** label. It initializes on load so the button works from the
keyboard. It tries the Clipboard API first. The fallback selects text in a
temporary textarea, then writes the original patch in the copy event to
preserve CRLF. It restores focus and the previous selection. A failed copy
leaves the patch selectable and displays **Copy failed**.

The generator validates each replacement against the original source and builds
one unified patch for the entire suggestion, even when it spans several files.
`similar` 3.2.1 (Apache-2.0, Rust 1.85 minimum) computes the line changes.
The report escapes file headers, preserves Clippy's applicability, and uses
diff prefixes and theme colors to distinguish additions and removals.
