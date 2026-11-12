# Help and Shortcuts Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans inline, with one fresh-context final review.

**Goal:** Port rustdoc's Help structure and provide predictable keyboard commands for actual report features.

**Architecture:** Shared static help partial and generated help.html; local help.js handles popover/? independently of search. Existing search/settings handlers coordinate by events and defaultPrevented. No generic shortcut registry.

**Tech Stack:** Askama/Rust 1.88, classic JS, existing rustdoc CSS, Node/jsdom.

**Spec:** docs/superpowers/specs/2026-11-12-help-shortcuts-design.md

## Global constraints

- Port actual pinned rustdoc DOM/CSS; no Impeccable or new UI design.
- Only implemented shortcuts appear in Help. No expand/collapse or API search syntax.
- No-JS standalone Help and normal modified-link navigation remain usable.
- One Escape closes one layer; input/IME/browser commands and disable-shortcuts are respected.
- Main and autonomous verified commits are authorized. No push.

## Review focus

- Escape must not close the underlying search together with a popup.
- Missing search data/settings storage must not disable Help.
- Popover contents must keep working through focus transitions and toolbar moves.
- Search input / editable content / composition must retain normal characters.
- Static Help must use working relative links and stay readable without JS.

## Task 1: Shared Help and popover

Files: templates/help.html, templates/base.html, src/render/pages.rs/categories.rs/assets.rs, assets/help.js/clippy-html.css, tests/help.test.cjs, package.json.

- [x] Write tests and observe failure for ?/link initialization, static help, popup dismissal/exclusion and missing search data.
- [x] Implement shared static contents, help.html/Back and independently initialized native Help popover.
- [x] Verify no-JS links/content, edited/modified clicks and existing script validation.

## Task 2: Shortcut coordination

Files: assets/search.js/settings.js/help.js, tests/help.test.cjs, existing integration/DOM tests.

- [x] Reproduce layered Escape closing two layers and disabled/editable/IME shortcut conflicts.
- [x] Add only the event/guard changes needed to coordinate the existing handlers.
- [x] Run Node suite, Rust tests, fmt/Clippy/MSRV and generate a preview.

## Task 3: Provenance and review

Files: README/ROADMAP/notices, vendor Help provenance, validation record.

- [x] Record pinned reference, supported keys, adaptations and actual test evidence.
- [x] One fresh final review; fix material findings with regressions.
- [x] Commit on main without pushing.
