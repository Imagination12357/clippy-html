# Circle/clip favicon and packaging — 2026-11-14

## Approved change

User approved the black circle / white paperclip SVG from
target/favicon-globe-draft/circle-clip.svg. The production SVG changes only its
draft title; geometry, transform, stroke colors and viewBox are unchanged.
The globe-grid concept was not selected. No Rust gear geometry is retained in
the active favicon. Project artwork is distributed as MIT OR Apache-2.0.

## Tracking and local preservation

Before this change, git ls-files found only these favicon assets: favicon.svg,
favicon.ico, README.md and CC-BY-4.0.txt under assets/favicon. Earlier draft
images/scripts in target/favicon-draft and target/favicon-globe-draft were
already ignored and untracked. Historical design/validation Markdown records
are kept as history; no Git history rewrite was requested or performed.

The four previous production files were copied to
target/favicon-archive/rust-gear/ before replacement. The canonical SVG/ICO now
contain the selected circle/clip artwork. git rm --cached removed the retired
assets/favicon/CC-BY-4.0.txt index entry while retaining its local file; a
specific .gitignore entry keeps it untracked. target/ remains ignored.

Cargo.toml explicitly excludes /target/** and /assets/favicon/CC-BY-4.0.txt.
The old CC notice is no longer embedded in the executable or emitted into new
reports. FAVICON.md, README, THIRD-PARTY-NOTICES and the report license page now
describe the current artwork; other third-party notices were preserved.

## Verification

- Existing integration assertions first failed on the old SVG. After updating,
  the report contains one black circle and one white clip path with no old
  gear group/mask/metadata. SVG and ICO report bytes equal production assets;
  retired favicon CC notice/link is absent.
- sharp/libvips rendered the production SVG at 16/32/48px; Pillow assembled the
  ICO. All three decoded ICO RGBA images exactly match the corresponding PNGs.
  Production SVG geometry equals the approved draft after ignoring title text.
- `cargo build --example cargo_probe --locked`, `cargo test --locked`:
  71 unit + 14 integration tests passed. fmt, strict all-target Clippy and
  Rust 1.88 all-target locked compilation passed.
- `cargo package --list --allow-dirty --offline` contains only README.md,
  favicon.svg and favicon.ico under assets/favicon; no target/draft/archive or
  retired CC license is listed.
- `cargo package --allow-dirty --offline` produced an actual .crate and passed
  its extracted-source build verification (142 files, 2.1 MiB uncompressed at
  this checkpoint). Direct tar inspection verified the same exclusions and
  byte equality of all three current favicon files. This is local package
  verification, not a crates.io publication.
- `git check-ignore -v` confirms target draft/archive paths and the locally
  retained CC file are ignored. No production draft/artwork file was deleted
  without first preserving a local copy.
- The existing target/help-shortcuts-preview report was regenerated to replace
  its previous embedded assets and remove retired license output. Internal
  HTML/link/asset validation applies before report publication.

## Limits

Fresh-context final review approved the change without material findings.
It independently inspected the current tracking state, preserved files,
approved SVG geometry, notices and actual archive exclusions; it did not
repeat the recorded test/raster runs. Historical documents and subjective
artwork preferences were not treated as new defects. No legal-policy approval
is asserted by this engineering review.

Historical commits still contain their original logo and notices. Existing
previously published binaries/reports are not changed by this local update.
No push, Pages publication or registry publication occurred in this task.
Browser tab icon caching/appearance is separate from byte-level verification.
