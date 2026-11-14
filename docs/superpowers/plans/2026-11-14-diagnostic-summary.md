# Diagnostic Summary Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans inline with one fresh final review.

**Goal:** Provide rustdoc-like diagnostic summaries, scoped expand/collapse and reliable deep-link reveal.

**Architecture:** Server renders native finding toggles and a compact representative location. A local summary.js manages only finding details and its dynamic toolbar button. Search publishes its close transition; existing independent controls remain independent.

**Tech Stack:** Rust 1.88 / Askama, existing rustdoc CSS, classic JS, Node/jsdom.

**Spec:** docs/superpowers/specs/2026-11-14-diagnostic-summary-design.md

## Constraints and review focus

- Preserve anchor IDs, source text, finding messages and availability of metadata/suggestions.
- Never collapse the source tree, full source, overview logs or search results.
- Native no-JS findings remain open; no new boilerplate labels.
- Deep links reveal only their target finding; same-hash/modified-link behavior and focus must remain usable.
- Search disables global collapse; editable/IME/modifier/disabled-key guards apply.
- Summary's extra toolbar width must not misplace Settings/Help pointers, including search mode.
- Work/commit on local main; no push, no Impeccable, no metadata export.

## Task 1: Finding summaries and scoped controls

Files: templates/lint.html, src/render/pages.rs, assets/summary.js, assets/clippy-html.css, templates/base.html, src/render/assets.rs, tests/summary.test.cjs, tests/integration.rs, package.json.

- [x] Add failing DOM/integration tests for preserved summary content, scope, mixed/manual state and no-JS defaults.
- [x] Implement native finding wrappers, representative location and dynamic native toolbar control.
- [x] Add keyboard/focus/deep-link reveal tests; preserve exact source text and existing clipboard behavior.

## Task 2: Search coordination and documentation

Files: assets/search.js, templates/help.html, existing Help/integration tests, README/ROADMAP, vendor provenance and validation record.

- [x] Reproduce hidden-search and same-document navigation cases, then implement close-event/state coordination.
- [x] Update Help's real command list, seven-to-eight local asset checks and native toolbar arrow offsets.
- [x] Run Node/Rust suites, fmt/Clippy/MSRV and generate a preview with multiple findings.
- [x] Record manual acceptance already supplied by the user for Pages and suggestion copying; do not claim screen-reader validation.
- [x] One fresh whole-change review, address material findings with regressions, commit without push.
