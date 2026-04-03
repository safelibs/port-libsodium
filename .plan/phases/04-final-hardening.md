# Phase 04 Final Hardening

## Phase Name
Final hardening and release closure

## Implement Phase ID
`impl_final_hardening`

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/build.rs`
- `safe/src/**`
- `safe/tests/**`
- `safe/cabi/**`
- `safe/include/**`
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
- `safe/compat-reports/dependents/results.tsv`
- `safe/compat-reports/dependents/failures.list`
- `safe/compat-reports/dependents/logs/`
- `safe/compat-reports/dependents/artifacts/`
- `safe/compat-reports/dependents-rerun/results.tsv`
- `safe/compat-reports/dependents-rerun/failures.list`
- `safe/compat-reports/dependents-rerun/logs/`
- `safe/compat-reports/dependents-rerun/artifacts/`
- `safe/compat-reports/issues.md`
- `original/src/libsodium/include/Makefile.am`
- `original/src/libsodium/include/sodium.h`
- `original/src/libsodium/include/sodium/*.h`
- `original/debian/libsodium23.symbols`
- `original/src/libsodium/.libs/libsodium.so`
- `original/test/default/Makefile.am`
- `original/test/default/*.c`
- `original/test/default/*.o`
- `original/test/default/*.exp`
- `original/test/default/test-suite.log`
- `dependents.json`
- `test-original.sh`
- `relevant_cves.json`

## New Outputs
- a release-candidate `safe/` tree with the compatibility workflow formalized and hardened
- a generated final report bundle under `safe/compat-reports/final/`
- a final safe-mode dependent report set for all 16 existing entries
- final package artifacts ready for Ubuntu 24.04 installation testing

## File Changes
- modify any remaining files under `safe/src/**`, `safe/tests/**`, `safe/tools/**`, `safe/debian/**`, `safe/packaging/**`, and `test-original.sh`

## Implementation Details
- Consume all outputs from phases 1 through 3 in place, including the tracked durable ledgers plus the generated evidence directories `safe/compat-reports/dependents/logs/`, `safe/compat-reports/dependents/artifacts/`, `safe/compat-reports/dependents-rerun/logs/`, and `safe/compat-reports/dependents-rerun/artifacts/`. Do not replace earlier ledgers or rerun evidence with alternate report roots.
- If phase 2 created tracked fixtures under `safe/testdata/dependents/`, consume those fixtures in place without promoting them to required preexisting inputs for this phase.
- The tracked durable files under `safe/compat-reports/` remain exactly `safe/compat-reports/.gitignore`, `safe/compat-reports/README.md`, `safe/compat-reports/issues.md`, `safe/compat-reports/dependents/results.tsv`, `safe/compat-reports/dependents/failures.list`, `safe/compat-reports/dependents-rerun/results.tsv`, and `safe/compat-reports/dependents-rerun/failures.list`.
- Treat `safe/compat-reports/final/` as generated workspace evidence rather than committed history. The final bundle must stay ignored in git even though reviewers consume it after the run.
- Make the final run durable and reviewable. `safe/tools/run-full-compat.sh --report-dir <dir>` must write at least:
- `summary.md`
- `cargo.log`
- `symbols.log`
- `source-tests.log`
- `relink.log`
- `packages.log`
- `dependents/results.tsv`
- `dependents/failures.list`
- `dependents/logs/<package>.log`
- `unsafe-audit.txt`
- `run-full-compat.sh` must treat the final dependent matrix as a gate. It may call `run-dependent-matrix.sh --strict` or perform an equivalent post-run ledger check, but the final full-compat command must fail if any dependent row is not `PASS`.
- `run-full-compat.sh --report-dir <dir>` must itself rebuild the release cdylib before `check-symbols.sh`, `run-original-c-tests.sh`, or `relink-original-objects.sh` consume `safe/target/release/libsodium.so`. The final verifier must not depend on a stale artifact left behind by an earlier phase.
- Preserve the current CVE regression coverage path around `relevant_cves.json` and `safe/tests/cve_2025_69277.rs`. Final hardening must keep using that existing path, and it must not refetch CVE data or re-infer the affected code.
- Preserve the existing authoritative interface sync path. Update `safe/include/`, `safe/cabi/libsodium.map`, `safe/cabi/expected/*.symbols`, and `safe/cabi/expected/upstream-kinds.tsv` only through `safe/tools/sync-upstream-interface.sh` if the upstream snapshot changes. Keep the authority split in which exported symbol names come from `original/debian/libsodium23.symbols` while symbol-kind and binding metadata come from `original/src/libsodium/.libs/libsodium.so`.
- Checkpoint manifests remain required repo artifacts throughout final hardening because `safe/build.rs` still validates them. Verifiers may confirm that workflow drivers default to the full ABI contract, but they must not require the strings `foundation`, `through_symmetric`, or `through_public_key` to disappear from the repository.
- Keep `unsafe` constrained to unavoidable FFI, OS, or low-level interoperability boundaries. Centralize new helper logic in `safe/src/ffi/helpers.rs` instead of spreading raw-pointer code further.
- Preserve the package names `libsodium23` and `libsodium-dev`, the SONAME `libsodium.so.23`, the package-upgrade behavior expected by the Docker harness, and the currently acceptable dynamic-dependency delta in which `libgcc_s.so.1` is the only tolerated extra `NEEDED` entry relative to upstream.
- Tighten package-build hygiene so reruns are deterministic. `safe/tools/build-deb.sh` should also clean stale `libsodium23-dbgsym_*.ddeb` outputs alongside `.deb`, `.buildinfo`, and `.changes` artifacts.
- Do not modify upstream `original/` tests merely to silence their warnings. Focus cleanup work on project-owned Rust warnings and actual compatibility defects.

## Verification Phases
### `check_final_hardening_tester`
Phase ID: `check_final_hardening_tester`

Type: `check`

Bounce Target: `impl_final_hardening`

Purpose: Run the full end-to-end matrix, including package build, ABI check, source and link compatibility, the final dependent-image run, CVE regression, and `unsafe` audit reporting.

Commands:
- `cargo test --manifest-path safe/Cargo.toml --all-targets`
- `cargo build --manifest-path safe/Cargo.toml --release`
- `./safe/tools/check-symbols.sh full`
- `./safe/tools/run-original-c-tests.sh --all`
- `./safe/tools/relink-original-objects.sh --all`
- `./safe/tools/build-deb.sh`
- `readelf -d safe/target/release/libsodium.so | rg 'SONAME|NEEDED'`
- `./safe/tools/run-full-compat.sh --report-dir safe/compat-reports/final`
- `test -f safe/compat-reports/final/summary.md`
- `test -f safe/compat-reports/final/unsafe-audit.txt`
- `rg -n "\\bunsafe\\b" safe/src safe/tests safe/build.rs safe/cabi || true`

### `check_final_hardening_senior`
Phase ID: `check_final_hardening_senior`

Type: `check`

Bounce Target: `impl_final_hardening`

Purpose: Review the final package contents and dependent ledger, and require every dependent row to be `PASS`.

Commands:
- `dpkg-deb -c ./libsodium23_*.deb | rg 'usr/lib/.*/libsodium\.so\.23$'`
- `dpkg-deb -c ./libsodium23_*.deb | rg 'usr/lib/.*/libsodium\.so\.23\.3\.0$'`
- `dpkg-deb -c ./libsodium-dev_*.deb | rg 'usr/include/sodium\.h$|usr/include/sodium/'`
- `dpkg-deb -c ./libsodium-dev_*.deb | rg 'pkgconfig/libsodium\.pc$|libsodium\.a$|libsodium\.so$'`
- `dpkg-deb -f ./libsodium23_*.deb Depends`
- `head -n 1 safe/compat-reports/final/dependents/results.tsv | rg '^package\tmode\tstatus\tlog_path$'`
- `test "$(tail -n +2 safe/compat-reports/final/dependents/results.tsv | wc -l)" -eq "$(jq '.dependents | length' dependents.json)"`
- `while IFS=$'\t' read -r package mode status log_path; do [[ "$package" == package ]] && continue; [[ "$mode" == safe ]] || exit 1; [[ "$log_path" == "logs/$package.log" ]] || exit 1; test -f "safe/compat-reports/final/dependents/$log_path"; done < safe/compat-reports/final/dependents/results.tsv`
- `awk -F'\t' 'NR > 1 && $3 != "PASS" {exit 1} END {exit NR > 1 ? 0 : 1}' safe/compat-reports/final/dependents/results.tsv`

## Success Criteria
- the full end-to-end matrix is green
- the final report directory exists and includes the summary, unsafe audit, and dependent ledger
- the final dependent ledger has one row per dependent and every row is `PASS`
- package contents and dependencies match the drop-in replacement contract
- the remaining `unsafe` footprint is documented and limited to necessary boundaries
- earlier ledgers, logs, and archived artifacts remain the authoritative in-place inputs for final closure work
- checkpoint manifests remain synced repo artifacts while workflow success continues to gate only on the full ABI contract
- the CVE coverage path remains on the existing metadata and regression test rather than refetching or re-inferring anything

## Git Commit Requirement
The implementer must commit work to git before yielding.
