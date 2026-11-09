#!/usr/bin/env bash
# Install this checkout and retain one online self-report plus command logs.
# Usage: bash .github/scripts/dogfood.sh /absolute/fresh/work-directory
set -euo pipefail

work=${1:?usage: dogfood.sh /absolute/fresh/work-directory}
if [ -e "$work" ]; then
  echo "Refusing existing dogfood directory: $work" >&2
  exit 1
fi
mkdir -p "$work/logs"
{
  printf 'Commit: %s\n' "${GITHUB_SHA:-local}"
  rustc --version --verbose
  cargo --version
  cargo clippy --version
} > "$work/logs/toolchain.log" 2>&1

cargo install --path . --locked --debug --root "$work/install" 2>&1 | tee "$work/logs/install.log"
export PATH="$work/install/bin:$PATH"
cargo clippy-html --output "$work/report" -- --locked --all-targets -- -W clippy::pedantic 2>&1 | tee "$work/logs/report.log"
test -f "$work/report/index.html"
