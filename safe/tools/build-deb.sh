#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
safe_dir=$(cd -- "$script_dir/.." && pwd)
repo_dir=$(cd -- "$safe_dir/.." && pwd)

rm -f \
  "$repo_dir"/libsodium23_*.deb \
  "$repo_dir"/libsodium-dev_*.deb \
  "$repo_dir"/libsodium-dbgsym_*.deb \
  "$repo_dir"/*.buildinfo \
  "$repo_dir"/*.changes

(
  cd "$safe_dir"
  dpkg-buildpackage -us -uc -b
)
