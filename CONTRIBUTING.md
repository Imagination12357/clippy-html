# Contributing

Development requires Rust with Clippy and rustfmt, Git, and Node.js 24 or newer.
The minimum supported Rust version is 1.88. The lockfiles pin Rust and Node test
dependencies. Install the 1.88.0 toolchain if you want to run the MSRV check.

```sh
rustup component add clippy rustfmt
npm ci --ignore-scripts
cargo test --locked
cargo fmt --check
cargo clippy --all-targets --locked -- -D warnings
cargo +1.88.0 check --all-targets --locked
npm test
```

The process-pipe test compiles `examples/cargo_probe.rs` into a temporary directory.
Integration tests copy fixture projects before running Clippy. Suggestion tests
apply patches with Git and compare the resulting bytes. Use a real browser to
check layout, clipboard permissions, accessibility, and `file://` behavior.

## Source packages

```sh
cargo package --list --locked
cargo package --locked
```

The manifest's `include` list ships the Rust source, Askama templates, report
assets and notices, README, metadata schema, and `examples/cargo_probe.rs`.
Cargo adds the manifest, lockfile, and packaging metadata. Check the archive
when changing any of these inputs.

Run the full test suite from a Git checkout. Cargo skips the nested fixture crates,
so the package also omits the integration tests that use them. The packaged unit
tests still run with `cargo test --locked`.

Keep the retired `assets/favicon/CC-BY-4.0.txt` explicitly excluded because
`include` overrides Git ignore rules. Ship the current notices and font licenses
with the source package and any standalone executable. [Third-party notices](THIRD-PARTY-NOTICES.md)

## CI and the self-report

CI runs Rust tests on Windows, Linux, and macOS. It also checks all targets with
Rust 1.88 on Linux and runs rustfmt, Clippy, Node tests, and the report script's
failure checks. [CI workflow](.github/workflows/ci.yml)

PRs and manual runs outside `main` install the checkout, run Clippy with pedantic
warnings enabled, and save the report and logs as `dogfood-report` for 14 days.
The report uses the default online category lookup. On `main`, CI generates and
deploys the report to Pages after the checks pass. Successful deployments use the
`github-pages` artifact. If report generation or deployment fails, look for
`dogfood-report` and its logs.

For a fork that should deploy its self-report, configure repository Settings →
Pages → Build and deployment → Source as **GitHub Actions**. Manual runs on
`main` can deploy too.

For a local self-report, give the script an absolute path that does not exist yet.

```sh
bash .github/scripts/dogfood.sh /absolute/path/to/a-fresh-directory
```

The script installs the tool there and saves the report and logs alongside it.

## Release archives for Binstall

Run the CI workflow manually with **Build archives for GitHub Releases and cargo
binstall** enabled. The existing test jobs will also build a release archive on
Windows x64 (MSVC), Linux x64 (glibc), and macOS arm64. The workflow saves the
archives as Actions artifacts and skips report generation and Pages deployment
for that run.

After the checks pass, download the three `.tar.gz` files. Attach them to the
GitHub Release for the same commit and version as the crates.io package, using
the tag `v<version>`. Keep the filenames as downloaded. For example, the Windows
asset for `v0.1.0` is `cargo-clippy-html-x86_64-pc-windows-msvc-v0.1.0.tar.gz`.
The archive contains the executable and notices in a directory of the same name
without `.tar.gz`. [Binstall's archive conventions](https://github.com/cargo-bins/cargo-binstall/blob/main/SUPPORT.md)

Once the crate and release assets are public, try the command below from an
environment with no existing installation. Selecting the repository strategy
lets you check that Binstall downloads our asset for that platform.

```sh
cargo binstall --strategies crate-meta-data cargo-clippy-html
cargo clippy-html --version
```

## Structure and reference behavior

The CLI collects Cargo diagnostics, merges duplicates, and loads the referenced
source files. The renderer uses one page map for HTML and optional navigation
JSON. It checks the generated destinations before publishing the report.

For interface changes, compare against the pinned rustdoc output and record the
adaptation under `assets/vendor/rustdoc/`. The main CSS comes from rustdoc 1.97.1.
The Settings tabs follow 1.102.0-nightly. Keep basic navigation usable without
JavaScript.

See what's planned and find past verification records in the [roadmap](ROADMAP.md).
