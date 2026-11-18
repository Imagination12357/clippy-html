#!/usr/bin/env bash
# Build one native release archive. Print the archive filename for CI.
set -euo pipefail

target=${1:?usage: package-release.sh TARGET OUTPUT_DIR}
output=${2:?usage: package-release.sh TARGET OUTPUT_DIR}
name=cargo-clippy-html
case "$target" in
  x86_64-pc-windows-msvc) binary="$name.exe" ;;
  x86_64-unknown-linux-gnu|aarch64-apple-darwin) binary=$name ;;
  *) echo "Unsupported release target: $target" >&2; exit 1 ;;
esac

host=$(rustc -vV | sed -n 's/^host: //p')
if [ "$host" != "$target" ]; then
  echo "Expected native Rust host $target, got $host" >&2
  exit 1
fi

mkdir "$output"
cargo build --locked --release --target "$target" --target-dir target --bin "$name" >&2
executable="target/$target/release/$binary"
version_line=$("$executable" --version)
version=${version_line##* }
if [[ ! $version =~ ^[0-9]+\.[0-9]+\.[0-9]+([+-][0-9A-Za-z.-]+)*$ ]]; then
  echo "Unexpected version from $executable: $version_line" >&2
  exit 1
fi

bundle="$name-$target-v$version"
stage="$output/$bundle"
mkdir "$stage"
cp "$executable" "$stage/$binary"
chmod +x "$stage/$binary"

# Retain repository-relative notice paths so the inventory applies to the archive.
for notice in LICENSE-MIT LICENSE-APACHE THIRD-PARTY-NOTICES.md \
  assets/favicon/README.md assets/vendor/clippy/{LICENSE-MIT,README.md} \
  assets/vendor/rustdoc/{COPYRIGHT.txt,LICENSE-MIT,normalize-LICENSE-MIT} \
  assets/vendor/rustdoc/*.md assets/vendor/rustdoc/fonts/*-LICENSE.{txt,md}; do
  mkdir -p "$stage/$(dirname "$notice")"
  cp "$notice" "$stage/$notice"
done

archive="$bundle.tar.gz"
(
  cd "$output"
  tar -czf "$archive" "$bundle"
)
printf '%s\n' "$archive"
