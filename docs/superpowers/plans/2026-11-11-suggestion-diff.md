# Suggestion Diff Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans inline, with one fresh final review.

**Goal:** Show each complete suggestion as a unified diff and copy its whole patch.

**Architecture:** A render/suggestions module validates replacements and builds patch text; pages supplies escaped views. A small local classic script handles patch copying independently of search/settings. Existing fallback replacements remain available.

**Tech Stack:** Rust 1.88, similar 3.2.1, Askama, classic JavaScript, Node/jsdom.

**Spec:** docs/superpowers/specs/2026-11-11-suggestion-diff-design.md

## Global constraints

- A1/B1 only. Record A2 as deferred, without implementing it.
- Preserve multipart suggestions as one unit; never modify user source files.
- Validate all source ranges before generating any patch. Invalid/missing/stale/overlapping edits fall back as a group.
- Copy patch text only, including file headers, context, line endings and EOF markers.
- Pinned rustdoc structure; no Impeccable, no unrelated Help/Summary/shortcut work.
- Work/commit on local main; do not push.

## Review focus

- Ambiguous insertions/overlaps or one bad file must not yield a partial valid-looking patch.
- Path quoting and hostile content must not create extra patch headers or HTML.
- CRLF, Unicode and missing terminal newline must survive patch application.
- Clipboard denial/failure must not announce success or mutate visible patch text.
- Very large diffs must not exhaust time/memory; explicit fallback is acceptable.

## Task 1: Patch construction

Files: Cargo.toml/Cargo.lock, src/render/mod.rs, src/render/suggestions.rs.

- [x] Write tests and observe RED for multipart/multi-file patch, invalid source/overlap/path and CRLF/EOF cases.
- [x] Implement whole-group validation and deterministic unified patches using similar; three context lines, bounded computation.
- [x] Check actual git apply on temporary copied files, not the checkout.
- [x] Verify focused Rust tests and Rust 1.88 all-target compilation.

## Task 2: Presentation and copy

Files: src/render/pages.rs, templates/lint.html/base.html, assets/clippy-html.css/suggestions.js, src/render/assets.rs, tests/suggestions.test.cjs, package.json.

- [x] Write failing DOM tests for exact grouped patch copying, clipboard fallback/denial and keyboard-safe independent initialization.
- [x] Render escaped line-colored patch with native code-block/button structure; keep original fallback and applicability.
- [x] Add local copy script and asset validation coverage; remove per-suggestion boilerplate.
- [x] Run Rust/Node suites, fmt/Clippy/MSRV and generate a self-report with a multipart example.

## Task 3: Documentation and final review

Files: README.md, ROADMAP.md, provenance and validation records.

- [x] Record source adaptations, limitations, A2 deferral and actual evidence.
- [x] One fresh whole-change review; address material findings with regressions.
- [x] Commit verified work on main without pushing.
