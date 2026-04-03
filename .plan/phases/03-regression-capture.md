# Phase 03 Regression Capture

## Phase Name
Regression capture and targeted compatibility fixes

## Implement Phase ID
`impl_regression_capture`

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/build.rs`
- `safe/src/**`
- `safe/tests/**`
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
- `safe/compat-reports/dependents/results.tsv`
- `safe/compat-reports/dependents/failures.list`
- `safe/compat-reports/dependents/logs/`
- `safe/compat-reports/dependents/artifacts/`
- `dependents.json`
- `test-original.sh`
- `relevant_cves.json`
- `original/debian/libsodium23.symbols`
- `original/src/libsodium/.libs/libsodium.so`
- `original/test/default/Makefile.am`
- `original/test/default/*.c`
- `original/test/default/*.o`
- `original/test/default/*.exp`
- `original/test/default/test-suite.log`

## New Outputs
- deterministic regression coverage for every dependent or harness issue found in phase 2
- `safe/compat-reports/issues.md` as the maintained issue ledger
- targeted code, packaging, or harness fixes in the Rust port and adjacent tooling
- reduced project-owned warning drift in the Rust implementation
- `safe/compat-reports/dependents-rerun/results.tsv` and `safe/compat-reports/dependents-rerun/failures.list` as committed post-fix rerun ledgers
- `safe/compat-reports/dependents-rerun/logs/` and `safe/compat-reports/dependents-rerun/artifacts/` as generated post-fix rerun evidence

## File Changes
- modify `safe/src/symmetric_impl.rs`
- modify `safe/src/public_key_impl.rs`
- modify `safe/src/foundation/**`
- modify `safe/src/ffi/helpers.rs`
- modify `safe/tests/ported_all.rs`
- modify the relevant `safe/tests/ported_*.rs` files
- add new regression tests under `safe/tests/` as needed
- modify `safe/tools/run-dependent-matrix.sh`
- modify `safe/tools/run-full-compat.sh`
- modify `test-original.sh`
- create or update `safe/compat-reports/issues.md`

## Implementation Details
- Consume the phase-2 report artifacts in place. Do not rediscover failures from scratch and do not replace the prepared dependent inventory.
- Only the tracked durable files `safe/compat-reports/issues.md`, `safe/compat-reports/dependents-rerun/results.tsv`, and `safe/compat-reports/dependents-rerun/failures.list` should be committed from this phase. Rerun logs and archived artifacts remain generated ignored outputs under the report-root policy established in phase 1.
- Preserve the current CVE regression coverage path around `relevant_cves.json` and `safe/tests/cve_2025_69277.rs`. Consume those existing artifacts in place, keep the regression green, do not refetch CVE data, and do not re-infer the affected code.
- If phase 2 created tracked fixtures under `safe/testdata/dependents/`, consume those fixtures in place. They remain optional carry-forward artifacts rather than mandatory preexisting inputs for this phase.
- For every package in `safe/compat-reports/dependents/failures.list`, add the narrowest stable reproducer possible:
- if the issue is a Rust-library semantic mismatch, add a Rust integration test under `safe/tests/`
- if the issue is a source, link, or package problem, add or strengthen a shell compatibility test under `safe/tools/`
- if the issue only reproduces inside the dependent container, strengthen the relevant package function in `test-original.sh` and archive supporting logs in `safe/compat-reports/issues.md`
- `safe/compat-reports/issues.md` must record each issue with at least the package or subsystem, status (`OPEN`, `FIXED`, or `COVERED`), reproducer location, and short resolution note or blocker.
- Do not allow this phase to be empty. If phase 2 produced no failing dependents, use this phase to turn the currently observed project-owned warnings into deterministic cleanup work:
- replace deprecated `aes_gcm::aead::AeadInPlace` usage in `safe/src/symmetric_impl.rs`
- remove or justify the dead constants in `safe/src/symmetric_impl.rs`
- add regression coverage for any dependent check that still proves only startup or linkage rather than actual behavior
- Do not special-case an empty `safe/compat-reports/dependents/failures.list` by skipping the rerun step. Rely on the header-only rerun-report contract from phase 1 so this phase still produces `safe/compat-reports/dependents-rerun/` even when the remaining work is warning cleanup rather than dependent breakage.
- Keep fixes localized. Likely touch points are `safe/src/symmetric_impl.rs`, `safe/src/public_key_impl.rs`, `safe/src/foundation/utils.rs`, `safe/src/ffi/helpers.rs`, packaging helpers, and container path logic.
- Review commands that enumerate remaining warning hotspots or `unsafe` usage must still succeed when zero matches remain, because this phase may legitimately eliminate every project-owned match.
- This phase's implementer must rerun the selected failure list once before yielding and commit the updated tracked `safe/compat-reports/issues.md`, `safe/compat-reports/dependents-rerun/results.tsv`, and `safe/compat-reports/dependents-rerun/failures.list` files.

## Verification Phases
### `check_regression_capture_tester`
Phase ID: `check_regression_capture_tester`

Type: `check`

Bounce Target: `impl_regression_capture`

Purpose: Rerun the library matrix plus the dependent failures recorded in phase 2, and require every repaired issue to have a deterministic reproducer or an explicit blocker entry.

Commands:
- `cargo test --manifest-path safe/Cargo.toml --all-targets`
- `cargo build --manifest-path safe/Cargo.toml --release`
- `./safe/tools/check-symbols.sh full`
- `./safe/tools/run-original-c-tests.sh --all`
- `./safe/tools/relink-original-objects.sh --all`
- `./safe/tools/run-dependent-matrix.sh --mode safe --from-list safe/compat-reports/dependents/failures.list --report-dir safe/compat-reports/dependents-rerun`
- `test -f safe/compat-reports/dependents-rerun/results.tsv`
- `test -f safe/compat-reports/dependents-rerun/failures.list`
- `head -n 1 safe/compat-reports/dependents-rerun/results.tsv | rg '^package\tmode\tstatus\tlog_path$'`
- `awk -F'\t' 'NR == 1 {next} NF == 4 && $2 == "safe" && $3 ~ /^(PASS|FAIL|WARN)$/ && $4 ~ /^logs\\/[^/]+\\.log$/ {next} {exit 1}' safe/compat-reports/dependents-rerun/results.tsv`
- `test -s safe/compat-reports/issues.md`

### `check_regression_capture_senior`
Phase ID: `check_regression_capture_senior`

Type: `check`

Bounce Target: `impl_regression_capture`

Purpose: Review the issue ledger, the targeted regressions, and the remaining warning and `unsafe` hotspots without assuming those hotspot strings must still exist.

Commands:
- `rg -n 'AeadInPlace|SECRETSTREAM_COUNTERBYTES|SECRETSTREAM_INONCEBYTES' safe/src/symmetric_impl.rs || true`
- `rg -n "\\bunsafe\\b" safe/src safe/tests safe/build.rs safe/cabi || true`
- `rg -n "\\b(FIXED|COVERED|OPEN)\\b" safe/compat-reports/issues.md`

## Success Criteria
- every issue in `safe/compat-reports/issues.md` is marked `FIXED`, `COVERED`, or explicitly `OPEN` with an explained blocker
- all repaired issues have deterministic reproducers
- the full internal library matrix stays green
- rerunning the phase-2 failure list produces an updated durable report set, even if the rerun is header-only
- if phase 2 produced no failing dependents, this phase still lands deterministic warning cleanup or stronger regression coverage
- phase-2 ledgers, logs, and archived artifacts remain the in-place inputs for issue triage and report-policy enforcement
- the CVE coverage path stays on the existing `relevant_cves.json` and `safe/tests/cve_2025_69277.rs` artifacts without refetching CVE data or re-inferring the affected code

## Git Commit Requirement
The implementer must commit work to git before yielding.
