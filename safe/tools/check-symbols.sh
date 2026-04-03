#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
safe_dir=$(cd -- "$script_dir/.." && pwd)

checkpoint=${1:-foundation}
library_path=${2:-$safe_dir/target/release/libsodium.so}

case "$checkpoint" in
  foundation|through_symmetric|through_public_key|full)
    manifest="$safe_dir/cabi/expected/${checkpoint}.symbols"
    ;;
  *)
    manifest=$checkpoint
    ;;
esac

kinds_manifest="$safe_dir/cabi/expected/upstream-kinds.tsv"

[[ -f "$manifest" ]] || {
  echo "missing expected-symbol manifest: $manifest" >&2
  exit 1
}
[[ -f "$kinds_manifest" ]] || {
  echo "missing upstream kinds manifest: $kinds_manifest" >&2
  exit 1
}
[[ -f "$library_path" ]] || {
  echo "missing library artifact: $library_path" >&2
  exit 1
}

tmpdir=$(mktemp -d)
trap 'rm -rf "$tmpdir"' EXIT

readelf --dyn-syms --wide "$library_path" \
  | awk '
      $1 ~ /^[0-9]+:$/ && $7 != "UND" && ($4 == "FUNC" || $4 == "OBJECT") {
        sub(/@.*/, "", $8)
        print $8 "\t" $5 "\t" $4
      }
    ' \
  | sort -u > "$tmpdir/actual.tsv"

cut -f1 "$tmpdir/actual.tsv" | sort -u > "$tmpdir/actual.names"
sort -u "$manifest" > "$tmpdir/expected.names"

diff -u "$tmpdir/expected.names" "$tmpdir/actual.names"

awk 'NR == FNR { want[$1] = 1; next } want[$1] { print }' \
  "$manifest" "$kinds_manifest" \
  | sort -u > "$tmpdir/expected.kinds"

awk 'NR == FNR { want[$1] = 1; next } want[$1] { print }' \
  "$manifest" "$tmpdir/actual.tsv" \
  | sort -u > "$tmpdir/actual.kinds"

if [[ $(wc -l < "$tmpdir/expected.kinds") -ne $(wc -l < "$tmpdir/expected.names") ]]; then
  echo "expected kinds manifest is missing one or more symbols from $manifest" >&2
  exit 1
fi

diff -u "$tmpdir/expected.kinds" "$tmpdir/actual.kinds"
