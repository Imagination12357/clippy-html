# Suggestion diff and patch copying

Approved direction: A1 (single unified line diff) and B1 (copy one complete
suggestion as a unified patch). A2 (separate Before/After blocks) is only a
possible future display option; no toggle or second renderer is added now.

Each existing suggestion remains one group, with its message and original
applicability values. A generated patch contains every replacement across all
files, sorted deterministically, with three context lines. Render the same
patch as escaped, line-colored text. Do not add explanatory boilerplate to
every suggestion. Keep file headers and hunk coordinates visible.

Validate every replacement against loaded workspace source, including original
snippet/byte/line/Unicode coordinates. Reject overlapping edits and ambiguous
same-position insertions. A missing/stale source, unsafe patch path, or any
invalid replacement disables the whole suggestion's diff/copy; keep the
original replacements and one concrete fallback reason. Do not independently
offer partial patches. Preserve CRLF and missing final newline in patch content.
No-op replacements must not produce a misleading empty success.

Patch paths are workspace-relative, forward-slash paths with a/ and b/
prefixes. Quote/escape unusual UTF-8 paths using Git's quoted path syntax;
reject absolute/traversal paths and metadata control characters rather than
generating an unsafe patch. Source files are never modified by the generator.

Use the existing pinned rustdoc 1.97.1 example-wrap/button-holder/copy-button
structure. The generated main-fcd733ba.js creates a copy button at the code
block's upper right and copies code text; this report copies the entire patch.
Use a text label instead of separately licensed clipboard/checkmark SVG assets.
Show the control only after JavaScript initializes, expose it to keyboard users,
and report success only after the clipboard operation succeeds. Try Clipboard
API first, then the rustdoc-style selected-text fallback; if both fail, keep
the patch selectable and show a concise failure state. Without JS, diffs and
original replacements remain readable. Never run git apply or edit files.

The renderer uses similar 3.2.1 (Apache-2.0, declared Rust 1.85), with a bounded
diff computation. No browser dependency or extra runtime network access.
Deletion/addition colors are specific to diffs, adapted to Light/Dark/Ayu;
prefixes also convey meaning without color. The patch text is the copy source,
so markup, button labels and status text never enter the clipboard.

Validation: exact patch application to copies in tests (including multipart,
multiple files, CRLF, Unicode and EOF), hostile HTML/path handling, all-or-none
fallback, denied clipboard and no-JS behavior. Hosted/browser acceptance must
be distinguished from local parser/jsdom checks.
