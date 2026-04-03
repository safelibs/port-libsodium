#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
safe_dir=$(cd -- "$script_dir/.." && pwd)
repo_dir=$(cd -- "$safe_dir/.." && pwd)
orig_test_dir="$repo_dir/original/test/default"
quirks_dir="$repo_dir/original/test/quirks"
target_dir="$safe_dir/target/release"

foundation_subset=(
  codecs
  randombytes
  sodium_core
  sodium_utils
  sodium_utils2
  sodium_utils3
  sodium_version
  verify1
)

active_test_inventory() {
  awk '
    function emit(line,    count, i, parts) {
      gsub(/\\/, "", line)
      gsub(/^[[:space:]]+|[[:space:]]+$/, "", line)
      if (line == "") {
        return
      }
      count = split(line, parts, /[[:space:]]+/)
      for (i = 1; i <= count; i++) {
        if (parts[i] != "") {
          print parts[i]
        }
      }
    }
    function active(    i) {
      for (i = 1; i <= depth; i++) {
        if (!conds[i]) {
          return 0
        }
      }
      return 1
    }
    $1 == "if" {
      depth++
      conds[depth] = ($2 == "!EMSCRIPTEN" || $2 == "!MINIMAL")
      next
    }
    $1 == "endif" {
      if (depth > 0) {
        depth--
      }
      next
    }
    active() && $1 == "TESTS_TARGETS" && ($2 == "=" || $2 == "+=") {
      capture = ($0 ~ /\\[[:space:]]*$/)
      if ($3 != "\\") {
        emit(substr($0, index($0, $3)))
      }
      next
    }
    capture && active() {
      emit($0)
      capture = ($0 ~ /\\[[:space:]]*$/)
    }
  ' "$repo_dir/original/test/default/Makefile.am"
}

select_tests() {
  if [[ ${1:-} == "--all" ]]; then
    shift
    active_test_inventory | sort -u
    return
  fi

  if [[ $# -gt 0 ]]; then
    printf '%s\n' "$@"
    return
  fi

  printf '%s\n' "${foundation_subset[@]}"
}

[[ -f "$target_dir/libsodium.so" ]] || {
  echo "missing $target_dir/libsodium.so. build the release artifact first" >&2
  exit 1
}

tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT
runtime_libdir="$tmpdir/lib"
mkdir -p "$runtime_libdir"
ln -sf "$target_dir/libsodium.so" "$runtime_libdir/libsodium.so"
ln -sf "$target_dir/libsodium.so" "$runtime_libdir/libsodium.so.23"

mapfile -t selected < <(
  select_tests "$@" \
    | sort -u \
    | while IFS= read -r name; do
        [[ -f "$orig_test_dir/$name.c" ]] || continue
        printf '%s\n' "$name"
      done
)

for name in "${selected[@]}"; do
  cc \
    -std=c11 \
    -O2 \
    -DTEST_SRCDIR='"'"$orig_test_dir"'"' \
    -DHAVE_CATCHABLE_SEGV=1 \
    -I"$safe_dir/include" \
    -I"$quirks_dir" \
    "$orig_test_dir/$name.c" \
    -Wl,-rpath,"$runtime_libdir" \
    -L"$target_dir" \
    -lsodium \
    -o "$tmpdir/$name"

  (
    cd "$orig_test_dir"
    LD_LIBRARY_PATH="$runtime_libdir${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}" "$tmpdir/$name"
  )
done
