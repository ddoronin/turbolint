#!/usr/bin/env bash
set -euo pipefail

if [ $# -ne 1 ]; then
  echo "Usage: $0 <version>"
  echo "Example: $0 0.2.0"
  exit 1
fi

VERSION="$1"
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

# Update all npm package.json files
for pkg_json in "$ROOT"/npm/*/package.json; do
  sed -i'' -e "s/\"version\": \"[^\"]*\"/\"version\": \"$VERSION\"/" "$pkg_json"
done

# Update optionalDependencies versions in the main package
sed -i'' -e "s/\"@turbolint\/cli-[^\"]*\": \"[^\"]*\"/&/" "$ROOT/npm/turbolint/package.json"
# More targeted: update each optional dep version
for dep in cli-darwin-arm64 cli-darwin-x64 cli-linux-x64 cli-linux-arm64 cli-linux-x64-musl cli-win32-x64; do
  sed -i'' -e "s|\"@turbolint/$dep\": \"[^\"]*\"|\"@turbolint/$dep\": \"$VERSION\"|" "$ROOT/npm/turbolint/package.json"
done

# Update workspace Cargo.toml version (add if missing)
if grep -q '^\[workspace\.package\]' "$ROOT/Cargo.toml"; then
  sed -i'' -e "/^\[workspace\.package\]/,/^\[/ s/^version = \"[^\"]*\"/version = \"$VERSION\"/" "$ROOT/Cargo.toml"
else
  # Insert [workspace.package] with version after [workspace]
  sed -i'' -e "/^\[workspace\]/a\\
\\
[workspace.package]\\
version = \"$VERSION\"" "$ROOT/Cargo.toml"
fi

echo "Bumped version to $VERSION"
