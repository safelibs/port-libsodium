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

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

collect_needed_libraries() {
  local library="$1"

  readelf -d "$library" \
    | awk -F'[][]' '/NEEDED/ { print $2 }' \
    | sort -u
}

check_shared_object_contract() {
  local safe_lib="$safe_dir/target/release/libsodium.so"
  local upstream_lib="$repo_dir/original/src/libsodium/.libs/libsodium.so"
  local runtime_deb
  local soname

  [[ -f "$safe_lib" ]] || die "missing release library artifact: $safe_lib"
  [[ -f "$upstream_lib" ]] || die "missing upstream library artifact: $upstream_lib"

  soname="$(readelf -d "$safe_lib" | awk -F'[][]' '/SONAME/ { print $2; exit }')"
  [[ "$soname" == "libsodium.so.23" ]] \
    || die "expected SONAME libsodium.so.23, found ${soname:-<none>}"
  echo "Confirmed the shared object still has SONAME libsodium.so.23."

  mapfile -t safe_needed < <(collect_needed_libraries "$safe_lib")
  mapfile -t upstream_needed < <(collect_needed_libraries "$upstream_lib")
  mapfile -t missing_needed < <(
    comm -23 \
      <(printf '%s\n' "${upstream_needed[@]}") \
      <(printf '%s\n' "${safe_needed[@]}")
  )
  mapfile -t extra_needed < <(
    comm -13 \
      <(printf '%s\n' "${upstream_needed[@]}") \
      <(printf '%s\n' "${safe_needed[@]}")
  )

  [[ ${#missing_needed[@]} -eq 0 ]] \
    || die "safe shared object is missing upstream dynamic dependencies: ${missing_needed[*]}"

  if [[ ${#extra_needed[@]} -eq 0 ]]; then
    echo "Confirmed the shared object does not introduce unexpected dynamic dependencies relative to upstream."
    return
  fi

  if [[ ${#extra_needed[@]} -ne 1 || ${extra_needed[0]} != "libgcc_s.so.1" ]]; then
    die "safe shared object introduced unexpected dynamic dependencies: ${extra_needed[*]}"
  fi

  runtime_deb="$(find "$repo_dir" -maxdepth 1 -type f -name 'libsodium23_*.deb' | sort | tail -n1)"
  [[ -n "$runtime_deb" ]] || die "unable to locate the built libsodium23 package"
  dpkg-deb -f "$runtime_deb" Depends | grep -Eq '(^|, )libgcc-s1([ (]|,|$)' \
    || die "runtime package does not declare libgcc-s1 for libgcc_s.so.1"

  echo "Documented unavoidable dynamic dependency relative to upstream: libgcc_s.so.1, packaged via libgcc-s1."
}

check_cve_fix() {
  (
    cd "$safe_dir"
    cargo test --release --test cve_2025_69277
  )
  echo "Confirmed CVE-2025-69277 remains fixed."
}

log_step "Building Debian packages"
"$safe_dir/tools/build-deb.sh"

log_step "Checking shared object contract"
check_shared_object_contract

log_step "Checking exported symbols"
"$safe_dir/tools/check-symbols.sh" full

log_step "Running CVE regression guard"
check_cve_fix

log_step "Running original C source-compat suite"
"$safe_dir/tools/run-original-c-tests.sh" --all

log_step "Running original object relink suite"
"$safe_dir/tools/relink-original-objects.sh" --all

log_step "Running safe-mode dependent smoke tests"
"$repo_dir/test-original.sh" --mode safe "${only_args[@]}"
