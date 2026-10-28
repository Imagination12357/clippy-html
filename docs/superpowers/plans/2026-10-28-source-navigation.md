# Source navigation implementation plan

> **For agentic workers:** Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Replace opaque source page names and provide a hierarchical source sidebar alongside findings.

**Architecture:** PageMap remains the authority for paths. Preserve workspace-relative file names beneath src/, omitting an initial src/ to avoid src/src/. Encode unsafe filename characters, reserve external-source names, and resolve case-insensitive collisions deterministically. Render a native details/summary file tree, opening current ancestors; findings remain a separate collapsible section with line links. No JavaScript or dependency additions.

**Tech stack:** Rust, Askama, existing rustdoc CSS.

**Spec:** User's three source-view observations and the proposed Files / Findings in this file design, approved by the subsequent request to proceed.

## Constraints and alternatives

- Existing hashed names are safe but unreadable. Selected approach preserves recognizable paths with safe escaping; flat basenames would lose hierarchy and collide frequently.
- Only files already represented by diagnostic spans appear; this is not an all-workspace source browser.
- Current source path appears once in the main heading; diagnostic location labels remain necessary.
- Preserve file:// operation, escaped text, deterministic output, and common item-table alignment.
- External or traversal-like source names cannot control output placement; use an isolated external namespace.
- Existing Windows workspace is codex/phase-1. User authorizes autonomous commits; no push or merge.

## Tasks

1. [x] Paths: add regression tests for readable nested/Unicode names, reserved characters/devices, traversal, case collisions, and link escaping. Observe failures; implement safe assignment. Update directory creation and recursive link validation. Verify nested HTML link/anchor checks.
2. [x] Source navigation: add real report assertions for Files hierarchy, active file, counts, current ancestors, other collapsed folders, and finding line links. Observe failures; implement sidebar template/model and remove duplicate path paragraph. Verify whole suite.
3. [x] Validate with fmt, strict Clippy, MSRV check and regenerated local previews. Review changes and document limits/results. Commit after final artifact refresh.

## Review focus

- Windows reserved names and case aliases cannot overwrite one another.
- Relative links at arbitrary depths reach CSS, licenses, source and finding anchors.
- Unicode, percent/hash characters and HTML-special labels stay local and escaped.
- Recursive report validation catches missing nested anchors rather than skipping pages.
- Missing/external source still provides findings and a usable tree without copying full source.

## Progress

- Baseline: existing tracked style changes are present; retain them. Current branch has a Phase-1 commit, so the earlier uncommitted-only summary is stale.
- Prior no-auto-commit ruling is superseded by explicit user authorization.
- Tasks 1/2: regression failures observed for hashed names, duplicated source paragraph and absent tree/current file. Implemented safe paths, depth-aware links, recursive validation, and source navigation. Initial suite: 41/41.
- Independent final review: no Critical or minor findings; three Important findings. Windows sigma/final-sigma and external/workspace leaf collisions reproduced RED and fixed GREEN in one pass. Count aggregation moved outside the page loop; supplementary count semantics regression passes. No timing benchmark claim.
- Final automated checks: cargo test --locked, 37 unit + 7 integration = 44 passed; cargo clippy --all-targets --locked -- -D warnings passed; cargo +1.94.0 check --locked passed. cargo fmt applied; final fmt --check recorded at artifact refresh.
- Scope decisions: tree contains diagnostic files only, external files have a separate tree namespace, and Unicode aliases may receive conservative extra suffixes. Cost: this is not a complete source browser and some filesystems could have accepted the unsuffixed names.
- Validation limit: browser automation blocks file URLs. No visual-layout or cross-browser completion claim; local previews and HTML/link checks are available for manual comparison.
- Artifact refresh: cargo fmt --check and git diff --check passed; both local reports regenerated successfully with recursive link/anchor validation, and cargo doc --no-deps --document-private-items --locked succeeded. Keeping codex/phase-1 locally per user workflow; autonomous commit authorized, no push/merge.
