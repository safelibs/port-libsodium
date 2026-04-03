# Phase 02 Dependent Matrix

## Phase Name
Dependent matrix deepening and report artifacts

## Implement Phase ID
`impl_dependent_matrix`

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/build.rs`
- `safe/src/**`
- `safe/tests/**`
- `safe/include/**`
- `safe/cabi/**`
- `safe/debian/**`
- `safe/docker/dependents.Dockerfile`
- `safe/tools/check-symbols.sh`
- `safe/tools/run-original-c-tests.sh`
- `safe/tools/relink-original-objects.sh`
- `safe/tools/build-deb.sh`
- `safe/tools/build-dependent-image.sh`
- `safe/tools/run-dependent-matrix.sh`
- `safe/tools/run-full-compat.sh`
- `safe/tools/sync-upstream-interface.sh`
- `safe/compat-reports/.gitignore`
- `safe/compat-reports/README.md`
- `original/debian/libsodium23.symbols`
- `original/src/libsodium/.libs/libsodium.so`
- `dependents.json`
- `test-original.sh`
- `relevant_cves.json`

## New Outputs
- a complete safe-mode report for all 16 dependent entries
- stronger per-dependent behavior checks for the six currently weak runtime consumers
- `safe/compat-reports/dependents/results.tsv` and `safe/compat-reports/dependents/failures.list` as the committed durable inputs for phase 3
- per-dependent logs under `safe/compat-reports/dependents/logs/`
- archived side-effect or key-material artifacts under `safe/compat-reports/dependents/artifacts/`
- optional tracked fixtures under `safe/testdata/dependents/`

## File Changes
- modify `test-original.sh`
- modify `safe/tools/run-dependent-matrix.sh`
- modify `safe/tools/run-full-compat.sh`
- add fixtures under `safe/testdata/dependents/` as needed
- update `safe/compat-reports/README.md` to document the fixed deep-check marker and artifact contract

## Implementation Details
- Consume all phase-1 outputs in place: `safe/docker/dependents.Dockerfile`, `safe/tools/build-dependent-image.sh`, `safe/tools/run-dependent-matrix.sh`, `safe/compat-reports/.gitignore`, `safe/compat-reports/README.md`, and the full-ABI workflow defaults already landed in `safe/tools/check-symbols.sh`, `safe/tools/run-full-compat.sh`, and `test-original.sh`. Do not replace them with ad hoc image definitions, duplicate harness logic, or a second report policy.
- Keep the exact 16-package inventory from `dependents.json`. Do not replace it with a new discovery pass and do not shrink it just because the goal only said "a dozen applications."
- Keep `test-original.sh --mode original` available as a control path even while safe mode becomes the primary matrix target.
- Preserve the fixed report schema from phase 1. `results.tsv` must keep the exact header `package<TAB>mode<TAB>status<TAB>log_path`, every `log_path` value must be `logs/<package>.log`, and `failures.list` must remain the ordered filtered view of `FAIL` and `WARN` rows rather than a replacement ledger.
- Only the tracked durable files `safe/compat-reports/dependents/results.tsv` and `safe/compat-reports/dependents/failures.list` should be committed from this phase. Per-package logs, archived artifacts, and any baseline directories remain generated ignored outputs under the policy established in `safe/compat-reports/.gitignore`.
- If this phase adds tracked fixtures under `safe/testdata/dependents/`, later phases must consume those fixtures in place rather than regenerating them under a different path. Those fixtures are optional outputs from this phase, not mandatory preexisting inputs for later phases.
- Preserve the current strong behavior checks that already exercise real functionality:
- `minisign` signs and verifies a file
- `shadowsocks-libev` proxies live traffic using `xchacha20-ietf-poly1305`
- `libtoxcore2` and `libzmq5` compile and run runtime smoke binaries
- `vim` writes an encrypted file with `cryptmethod=xchacha20`
- `php8.3-cli` and `r-cran-sodium` already cover both encryption and signing paths
- `librust-libsodium-sys-dev`, `libtoxcore-dev`, and `libzmq3-dev` already compile and link against the installed safe packages
- Strengthen the weaker cases in `test-original.sh:600-829` so the matrix proves compatibility rather than mostly startup:
- `qtox`: keep the offscreen launch, require a real qtox-created config, profile, or settings side effect, copy at least one resulting qtox-owned file into `safe/compat-reports/dependents/artifacts/qtox/`, and emit the exact success marker `QTOX_PROFILE_OK` only after that side effect is verified
- `fastd`: parse the generated keypair, validate the key formats and lengths rather than only marker strings, and emit the exact success marker `FASTD_KEYPAIR_OK` only after those validations succeed
- `curvedns`: validate the generated DNSCurve key material formats and lengths, archive the generated files under `safe/compat-reports/dependents/artifacts/curvedns/`, and emit the exact success marker `CURVEDNS_KEYPAIR_OK`
- `nix-bin`: add a usable signing round trip beyond key generation and emit the exact success marker `NIX_SIGN_VERIFY_OK`
- `python3-nacl`: keep the secretbox round trip, add a detached-signature verify path, and emit the exact success marker `PYNACL_SIGN_VERIFY_OK`
- `ruby-rbnacl`: keep the secretbox round trip, add a detached-signature verify path, and emit the exact success marker `RBNACL_SIGN_VERIFY_OK`
- Preserve compile-time package checks as concrete compile and link exercises using installed headers and pkg-config metadata from the safe packages.
- Ensure `results.tsv` always lists every selected package exactly once and in selection order, even when the package test fails. Reruns must recreate the report directory rather than appending stale rows.
- Keep non-strict exit semantics for this phase: the matrix command should complete and write artifacts even if some packages are marked `FAIL` or `WARN`.
- Because phase-2 ledgers may intentionally contain `FAIL` or `WARN` rows that phase 3 will consume in place, review commands that inspect those rows must still succeed when zero rows match. Only the final hardening verifier may require every dependent row to be `PASS`.
- This phase's implementer must run the full safe-mode matrix once before yielding and commit the updated tracked `safe/compat-reports/dependents/results.tsv` and `safe/compat-reports/dependents/failures.list` files.

## Verification Phases
### `check_dependent_matrix_tester`
Phase ID: `check_dependent_matrix_tester`

Type: `check`

Bounce Target: `impl_dependent_matrix`

Purpose: Run the full safe-mode dependent matrix, persist the ledger and per-package logs, and sanity-check original-mode control behavior on one runtime and one compile-time consumer.

Commands:
- `./safe/tools/run-dependent-matrix.sh --mode safe --report-dir safe/compat-reports/dependents`
- `./test-original.sh --mode original --only minisign`
- `./test-original.sh --mode original --only librust-libsodium-sys-dev`
- `test -f safe/compat-reports/dependents/results.tsv`
- `test -f safe/compat-reports/dependents/failures.list`
- `head -n 1 safe/compat-reports/dependents/results.tsv | rg '^package\tmode\tstatus\tlog_path$'`
- `diff -u <(jq -r '.dependents[].package' dependents.json) <(tail -n +2 safe/compat-reports/dependents/results.tsv | cut -f1)`

### `check_dependent_matrix_senior`
Phase ID: `check_dependent_matrix_senior`

Type: `check`

Bounce Target: `impl_dependent_matrix`

Purpose: Review the fixed report contract, verify that the six previously weak dependents now emit concrete deep-check markers, and inspect interim `FAIL` or `WARN` rows without treating their presence as a verifier failure.

Commands:
- `while IFS=$'\t' read -r package mode status log_path; do [[ "$package" == package ]] && continue; [[ "$mode" == safe ]] || exit 1; [[ "$log_path" == "logs/$package.log" ]] || exit 1; test -f "safe/compat-reports/dependents/$log_path"; done < safe/compat-reports/dependents/results.tsv`
- `awk -F'\t' 'NR == 1 {next} NF == 4 && $3 ~ /^(PASS|FAIL|WARN)$/ {next} {exit 1}' safe/compat-reports/dependents/results.tsv`
- `rg -n 'QTOX_PROFILE_OK' safe/compat-reports/dependents/logs/qtox.log`
- `find safe/compat-reports/dependents/artifacts/qtox -type f | rg .`
- `rg -n 'FASTD_KEYPAIR_OK' safe/compat-reports/dependents/logs/fastd.log`
- `rg -n 'CURVEDNS_KEYPAIR_OK' safe/compat-reports/dependents/logs/curvedns.log`
- `rg -n 'NIX_SIGN_VERIFY_OK' safe/compat-reports/dependents/logs/nix-bin.log`
- `rg -n 'PYNACL_SIGN_VERIFY_OK' safe/compat-reports/dependents/logs/python3-nacl.log`
- `rg -n 'RBNACL_SIGN_VERIFY_OK' safe/compat-reports/dependents/logs/ruby-rbnacl.log`
- `awk -F'\t' 'NR > 1 && $3 ~ /^(FAIL|WARN)$/ {print $1 "\t" $3 "\t" $4}' safe/compat-reports/dependents/results.tsv`

## Success Criteria
- the safe-mode matrix completes and writes a 16-row ledger in the exact `dependents.json` order
- report artifacts exist and cover all 16 current dependents
- original-mode control still works for one representative runtime consumer and one representative compile-time consumer
- the six previously weak dependents now prove the strengthened behaviors via fixed log markers and archived artifacts
- interim `FAIL` or `WARN` rows remain committed, ordered, in-place inputs for phase 3 rather than ephemeral console output
- any tracked fixtures added under `safe/testdata/dependents/` are preserved as in-place inputs for later phases without becoming mandatory workflow preconditions

## Git Commit Requirement
The implementer must commit work to git before yielding.
