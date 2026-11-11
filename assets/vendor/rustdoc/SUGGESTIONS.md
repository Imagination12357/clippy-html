# Suggestion diff and copy provenance

Copy-button reference: this project's rustdoc 1.97.1 output, Rust commit
8bab26f4f68e0e26f0bb7960be334d5b520ea452. The generated
main-fcd733ba.js (SHA-256
baec8e8981b6e116315ea7ff1fed10b51352e001825228556ccd126317ed91db)
uses example-wrap > button-holder > button.copy-button at the upper right,
copies code text through a selected temporary textarea, and shows short success
feedback. Source: src/librustdoc/html/static/js/main.js in
https://github.com/rust-lang/rust/tree/8bab26f4f68e0e26f0bb7960be334d5b520ea452
under the MIT option. Original Rust Project Contributors notice and full license
remain in LICENSE-MIT / generated licenses/rustdoc-MIT.txt.

assets/suggestions.js adapts this structure for a whole suggestion patch,
initializing on load so keyboard access does not require mouse hover. It uses
a text label (Copy patch), not the separately licensed clipboard/checkmark SVG
assets. Success appears only after a successful copy. Clipboard API is tried
first; selected-text fallback intercepts the copy event to avoid textarea CRLF
normalization, then restores focus and selection. Failure remains visible as
Copy failed, while the patch stays readable/selectable. No runtime fetch or
automatic file changes are performed.

The unified diff itself is report-specific, not a rustdoc feature being copied.
similar 3.2.1 (Apache-2.0, Rust 1.85 minimum) computes line changes; the report
validates and groups source replacements and creates escaped a/b file headers.
Each generated suggestion keeps one patch, including multiple files. Prefixes
and theme-specific red/green backgrounds distinguish removals and additions.
Original applicability values are preserved. A2 (separate Before/After display)
is deferred as a possible future option, without an inactive control.
