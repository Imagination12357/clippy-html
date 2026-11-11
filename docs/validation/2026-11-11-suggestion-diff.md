# Suggestion diff validation — 2026-11-11

Approved: A1 unified line diff and B1 whole-suggestion Copy patch. A2 separate
Before/After display remains a possible later option, with no UI toggle now.

## Implementation and limits

`src/render/suggestions.rs` validates every replacement using loaded workspace
source and existing `span_matches`, rejects overlaps/ambiguous insertions,
creates a deterministic per-file edited copy in memory, and emits one unified
patch with three context lines. It never writes the inspected source. Paths
are workspace-relative a/b paths, quoted for Git (including octal UTF-8 bytes).
Unsafe/control-character/non-UTF-8 paths and binary NUL data use fallback.
The whole group falls back if any member is invalid, unavailable or stale.
Patch generation additionally requires the diagnostic's original snippet to
cover every spanned line; empty or truncated snippets cannot verify source.

Limits: 1,024 replacements, 4 MiB combined source/replacement bytes, 100,000
newline characters per source file, 4 MiB patch output and a two-second
generation deadline per suggestion. No-op-only suggestions also fall back.
Fallback preserves original replacement text, applicability and a specific
reason, without a partial Copy patch control.

`pages.rs` provides escaped line spans and a JSON string in an escaped data
attribute. Displayed HTML normalization cannot change the exact copy payload.
The classic local script adds a rustdoc-shaped text button on initialization;
no-JS reports retain readable diffs without inactive buttons. Clipboard API is
preferred. A temporary selection plus a copy-event payload is the fallback:
the textarea's normalized value is never the patch source. Focus/selection are
restored; failures show Copy failed rather than false success.

similar 3.2.1 was checked through the official crates.io version metadata and
downloaded source: Apache-2.0, declared Rust 1.85 minimum; locked Rust 1.88
compilation also passed. It adds bstr 1.13.1 transitively. No other dependency
versions changed. Rustdoc copy provenance and deliberate adaptations are in
[SUGGESTIONS.md](../../assets/vendor/rustdoc/SUGGESTIONS.md).

## Local evidence (Windows)

- Initial patch tests failed to compile before build_patch existed, then passed.
- Tests actually ran `git -c core.autocrlf=false apply --whitespace=nowarn` on
  disposable copied files and compared resulting bytes for multipart changes,
  multiple files/Unicode-space paths, CRLF, insertions/deletions, missing EOF
  newline, separated hunks and multiline replacements changing later offsets.
- Group fallback tests cover stale snippets, missing files, overlaps, duplicate
  insertion positions, path traversal/header injection, no-ops, NUL and size
  limits. NUL regression failed before the guard, then passed.
- Initial DOM tests failed because the copy script did not exist. The first
  fallback then failed the exact CRLF assertion; the copy-event payload fix
  made it pass. Tests cover exact whole-patch data, idempotent independent
  initialization, Clipboard API denial, selected-text fallback/focus restoration,
  unsupported/throwing/false-returning copy APIs and malformed JSON.
- `cargo build --example cargo_probe --locked`, `cargo test --locked`:
  71 unit + 12 integration tests passed. Existing integration verifies generated
  escaped patch data, applicability, no-JS rendering and six local scripts.
- `npm test`: 30 passed (existing 26 + four suggestion clipboard tests).
- `cargo fmt --check`, strict all-target Clippy and
  `cargo +1.88.0 check --all-targets --locked --target-dir target/msrv-1.88`: passed.
- Generated `target/suggestion-diff-preview` with online-default pedantic
  self-dogfooding. The actual assert_is_empty suggestion for dedup.rs contains
  both edits as `assert!(!r[0].id.is_empty())` → `assert_ne!(r[0].id, "")` in one
  patch. jsdom loaded that generated data and confirmed the copied payload
  exactly matched the stored patch. Internal report link/asset validation passed.

## Remaining acceptance

Fresh-context final review found one material issue: the existing shared span
matcher accepted empty or partial snippets. A patch-local completeness guard
was added without changing unrelated source highlighting. Its missing/truncated
snippet regression failed before the guard and passed afterward, followed by
the entire Rust/Node suites, fmt, strict Clippy and MSRV checks above. No other
material findings or deferred minor findings were reported.

Actual browser layout, small-screen Light/Dark/Ayu appearance and file:// / Pages
clipboard policies are not proven by jsdom. They remain manual acceptance.
Screen-reader announcements also require actual assistive-technology checks.
No push, remote CI execution or Pages publication occurred in this task.
