#!/usr/bin/env bash
set -euo pipefail

script_dir=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)
safe_dir=$(cd -- "$script_dir/.." && pwd)
repo_dir=$(cd -- "$safe_dir/.." && pwd)

usage() {
  cat <<EOF
usage: $(basename "$0") --report-dir <dir> [--mode safe|original] [--only <package>] [--from-list <file>] [--strict]

Runs the dependent compatibility matrix through test-original.sh and writes a
deterministic report ledger under the requested report directory.
EOF
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

resolve_report_dir() {
  local input="$1"
  local parent
  local base

  parent="$(dirname -- "$input")"
  base="$(basename -- "$input")"
  mkdir -p "$parent"
  parent="$(cd -- "$parent" && pwd)"
  printf '%s/%s\n' "$parent" "$base"
}

mode="safe"
report_dir=""
only=""
from_list=""
strict=0

while (($#)); do
  case "$1" in
    --mode)
      mode="${2:?missing value for --mode}"
      shift 2
      ;;
    --report-dir)
      report_dir="${2:?missing value for --report-dir}"
      shift 2
      ;;
    --only)
      only="${2:?missing value for --only}"
      shift 2
      ;;
    --from-list)
      from_list="${2:?missing value for --from-list}"
      shift 2
      ;;
    --strict)
      strict=1
      shift
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      die "unknown option: $1"
      ;;
  esac
done

[[ -n "$report_dir" ]] || die "missing required --report-dir"
[[ -z "$only" || -z "$from_list" ]] || die "--only and --from-list are mutually exclusive"

case "$mode" in
  safe|original)
    ;;
  *)
    die "unknown mode: $mode"
    ;;
esac

report_dir="$(resolve_report_dir "$report_dir")"
rm -rf "$report_dir"
mkdir -p "$report_dir"

args=(
  --mode "$mode"
  --report-dir "$report_dir"
)

if [[ -n "$only" ]]; then
  args+=(--only "$only")
fi
if [[ -n "$from_list" ]]; then
  args+=(--from-list "$from_list")
fi
if [[ "$strict" == "1" ]]; then
  args+=(--strict)
fi

"$repo_dir/test-original.sh" "${args[@]}"
