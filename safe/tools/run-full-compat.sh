#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
safe_dir=$(cd -- "$script_dir/.." && pwd)
repo_dir=$(cd -- "$safe_dir/.." && pwd)

usage() {
  cat <<EOF
usage: $(basename "$0") [--only <package>]

Builds the safe Debian packages, verifies the full exported symbol set,
re-runs all 77 legacy source-compat and relinked-object tests, and then
executes the dependent smoke harness in --mode safe.
EOF
}

only_args=()

while (($#)); do
  case "$1" in
    --only)
      only_args+=("$1" "${2:?missing value for --only}")
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf 'unknown option: %s\n' "$1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

log_step() {
  printf '\n==> %s\n' "$1"
}

log_step "Building Debian packages"
"$safe_dir/tools/build-deb.sh"

log_step "Checking exported symbols"
"$safe_dir/tools/check-symbols.sh" full

log_step "Running original C source-compat suite"
"$safe_dir/tools/run-original-c-tests.sh" --all

log_step "Running original object relink suite"
"$safe_dir/tools/relink-original-objects.sh" --all

log_step "Running safe-mode dependent smoke tests"
"$repo_dir/test-original.sh" --mode safe "${only_args[@]}"
