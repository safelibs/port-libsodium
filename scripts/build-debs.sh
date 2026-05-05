#!/usr/bin/env bash
# libsodium: regenerate the upstream autotools build so the safe layer
# can pick up its artifacts, then run the standard safe-debian build.
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
# shellcheck source=/dev/null
. "$repo_root/scripts/lib/build-deb-common.sh"

prepare_rust_env
prepare_dist_dir "$repo_root"

(
  cd "$repo_root/original"
  if [ -f Makefile ]; then
    make distclean || true
  fi
  autoreconf -ivf
  ./configure --disable-static
  make -j"$(nproc)"
)

cd "$repo_root/safe"
stamp_safelibs_changelog "$repo_root"
build_with_dpkg_buildpackage "$repo_root"
