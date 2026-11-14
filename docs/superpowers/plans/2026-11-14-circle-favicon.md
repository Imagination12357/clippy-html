# Circle and Paperclip Favicon Implementation Plan

> **For agentic workers:** Use superpowers:executing-plans inline, with one fresh final review.

**Goal:** Apply the approved black-circle/white-paperclip logo and keep old Rust-gear drafts out of the current source/package/report.

**Architecture:** Replace the two embedded canonical assets with the approved SVG and exact 16/32/48px ICO rasterizations. Preserve the previous assets under ignored target/, remove obsolete embedded notices, and explicitly exclude target/ and the retired asset license from Cargo packages.

**Tech Stack:** SVG, existing sharp/libvips and Pillow tooling (development only), Rust embedded assets, Cargo packaging.

**Spec:** User approved target/favicon-globe-draft/circle-clip.svg on 2026-11-14. Keep the black circle, white clip geometry and 32x32 viewBox unchanged. The user requests that old gear drafts not be tracked or packaged.

## Constraints and review focus

- Drafts under target/ are already untracked/ignored; only current assets/favicon SVG/ICO/readme/license are tracked. Do not rewrite history.
- Preserve old assets locally before replacing them. Keep the old CC license locally but remove its index entry, ignore and package-exclude it.
- New logo artwork uses the project's MIT OR Apache-2.0 license; update asset/report notices consistently. Do not modify other third-party notices/licenses.
- One approved SVG is the raster source for all sizes and previews. No alternate hand-drawn preview or geometry change.
- No old gear image or obsolete favicon CC notice in new report or Cargo source package.
- Work/commit on main; no push. Do not alter unrelated dependency-feature changes at base 1c882c7.

## Task 1: Replace embedded logo and metadata

- [x] Add failing integration assertions for the approved SVG/ICO and absent old notice.
- [x] Archive old SVG/ICO/readme/license under target/favicon-archive/rust-gear/.
- [x] Copy approved geometry to assets/favicon/favicon.svg, rasterize SVG to 16/32/48 PNGs, assemble and verify ICO byte equality.
- [x] Update favicon README, root README/notices and generated license page; stop embedding old favicon CC license.

## Task 2: Tracking, packaging and completion

- [x] Untrack obsolete CC file with git rm --cached; preserve it locally, ignore it and explicitly exclude target/** and that file in Cargo.toml.
- [x] Check cargo package --list and an actual .crate archive: new asset bytes present, drafts/archive/obsolete license absent.
- [x] Run relevant Rust tests/quality checks and generate a report with exact approved assets and valid notice links.
- [x] Record evidence, obtain one fresh final review, commit verified changes without pushing.
