#!/usr/bin/env bash
# Install apt packages and a rust toolchain for libsodium's safe build.
# autoconf/automake/libtool are required to autoreconf the upstream tree.
set -euo pipefail

repo_root="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"

toolchain="${SAFELIBS_RUST_TOOLCHAIN:-}"
if [[ -z "$toolchain" && -f "$repo_root/safe/rust-toolchain.toml" ]]; then
  toolchain="$(grep -oP '^channel\s*=\s*"\K[^"]+' "$repo_root/safe/rust-toolchain.toml" || true)"
fi
toolchain="${toolchain:-stable}"

export DEBIAN_FRONTEND=noninteractive
sudo apt-get update
sudo apt-get install -y --no-install-recommends \
  autoconf \
  automake \
  build-essential \
  ca-certificates \
  curl \
  devscripts \
  dpkg-dev \
  equivs \
  fakeroot \
  file \
  git \
  jq \
  libtool \
  python3 \
  rsync \
  xz-utils

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs \
  | sh -s -- -y --profile minimal --default-toolchain "$toolchain" --no-modify-path

# shellcheck source=/dev/null
. "$HOME/.cargo/env"
rustup default "$toolchain"
rustc --version
cargo --version

if [[ -n "${GITHUB_PATH:-}" ]]; then
  printf '%s\n' "$HOME/.cargo/bin" >> "$GITHUB_PATH"
fi
