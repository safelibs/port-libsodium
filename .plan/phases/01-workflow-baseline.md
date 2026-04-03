# Phase 01 Workflow Baseline

## Phase Name
Workflow baseline, full-ABI default, and explicit dependent image

## Implement Phase ID
`impl_workflow_baseline`

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/build.rs`
- `safe/src/**`
- `safe/tests/**`
- `safe/cabi/**`
- `safe/include/**`
- `safe/debian/**`
- `safe/tools/check-symbols.sh`
- `safe/tools/run-original-c-tests.sh`
- `safe/tools/relink-original-objects.sh`
- `safe/tools/build-deb.sh`
- `safe/tools/run-full-compat.sh`
- `safe/tools/sync-upstream-interface.sh`
- `original/src/libsodium/.libs/libsodium.so`
- `original/src/libsodium/include/Makefile.am`
- `original/src/libsodium/include/sodium.h`
- `original/src/libsodium/include/sodium/*.h`
- `original/debian/libsodium23.symbols`
- `original/test/default/Makefile.am`
- `original/test/default/*.c`
- `original/test/default/*.o`
- `original/test/default/*.exp`
- `original/test/default/test-suite.log`
- `dependents.json`
- `test-original.sh`
- `relevant_cves.json`

## New Outputs
- `safe/docker/dependents.Dockerfile` as the checked-in Ubuntu 24.04 dependent-image authority
- `safe/tools/build-dependent-image.sh`
- `safe/tools/run-dependent-matrix.sh`
- `safe/compat-reports/.gitignore`
- `safe/compat-reports/README.md`
- workflow drivers that default to the full ABI contract instead of the obsolete staged-checkpoint default
- a report-root contract that separates tracked durable ledgers from ignored generated logs and artifacts

## File Changes
- modify `safe/tools/check-symbols.sh`
- modify `safe/tools/run-full-compat.sh`
- modify `test-original.sh`
- add `safe/docker/dependents.Dockerfile`
- add `safe/tools/build-dependent-image.sh`
- add `safe/tools/run-dependent-matrix.sh`
- add `safe/compat-reports/.gitignore`
- add `safe/compat-reports/README.md`

## Implementation Details
- Keep the generated workflow strictly linear, inline-only, and explicit-phase only. Do not introduce `parallel_groups`, `include`, `prompt_file`, `workflow_file`, `workflow_dir`, `checks`, or bounce-target lists.
- Do not derive workflow topology from `.plan/phases/*.md` or `.plan/workflow-structure.yaml`. `.plan/plan.md` plus the existing artifacts in `safe/`, `original/`, `dependents.json`, and `test-original.sh` remain the authorities.
- Preserve the consume-existing-artifacts contract by using the listed upstream headers, package metadata, built upstream shared object, source-test inventory, relink inventory, CVE metadata, and dependent inventory in place. Do not rediscover or regenerate those authorities in this phase.
- Keep `safe/cabi/expected/full.symbols` plus `safe/cabi/expected/upstream-kinds.tsv` as the only ABI gate for workflow success. The checkpoint manifests remain required synced repo artifacts because `safe/build.rs` validates them, but the workflow must not use them as staged success gates.
- Verifiers may confirm that workflow drivers now default to the full ABI contract, but they must not require the strings `foundation`, `through_symmetric`, or `through_public_key` to disappear from the repository because those manifests still need to exist for `safe/build.rs`.
- Preserve the current authority split used by `safe/tools/sync-upstream-interface.sh`: exported symbol names come from `original/debian/libsodium23.symbols`, while symbol-kind and binding metadata come from `original/src/libsodium/.libs/libsodium.so`.
- Preserve the current 16-entry dependent inventory in `dependents.json`. Do not replace it with a new discovery pass and do not shrink it just because the goal only said "a dozen applications."
- Keep `test-original.sh --mode original` available as a comparison path. Safe mode is the main target, but original mode remains a useful control.
- Preserve the current source-compat and relink contracts already encoded in `safe/tools/run-original-c-tests.sh` and `safe/tools/relink-original-objects.sh`: 77 runnable C tests, 77 relinkable object files, and `hash2.exp` as the lone orphan expected-output fixture.
- Make `safe/tools/check-symbols.sh` use `safe/cabi/expected/full.symbols` as its no-argument workflow default instead of `foundation.symbols`. Keep explicit checkpoint arguments for manual debugging.
- Extract the inline Dockerfile currently embedded in `test-original.sh:69-115` into `safe/docker/dependents.Dockerfile`. `test-original.sh` and `safe/tools/run-full-compat.sh` must consume that checked-in file through `safe/tools/build-dependent-image.sh`.
- Preserve the current read-only source mount in `test-original.sh`, but add a separate writable report mount such as `-v "$REPORT_DIR":/reports` so the container can emit reports without mutating `/work`.
- Extend `test-original.sh` so it can write per-package logs and a deterministic ledger under a caller-provided report directory while still supporting direct `--mode` and `--only` execution when no report directory is requested.
- Add `safe/tools/run-dependent-matrix.sh` as the stable CLI for matrix execution. It must accept at least `--mode`, `--report-dir`, `--only`, `--from-list`, and `--strict`, and it must orchestrate the existing package-specific logic in `test-original.sh` instead of forking a second copy of those tests.
- `run-dependent-matrix.sh` must always write each selected package's combined stdout and stderr to `<report-dir>/logs/<package>.log` and record that relative path verbatim in the `log_path` column. Data rows must stay in selection order; for the default full run that is the existing `dependents.json` order.
- `results.tsv` must contain exactly one header row with the fixed schema `package<TAB>mode<TAB>status<TAB>log_path`. Data rows may use only `PASS`, `FAIL`, or `WARN` in the `status` column, and no later phase may extend this schema.
- `failures.list` must contain one package name per line for every `FAIL` or `WARN` row, in the same order as the corresponding `results.tsv` rows. An empty file is valid and must still be created.
- `run-dependent-matrix.sh` must clear or recreate the target report directory before each run so stale logs and stale ledger rows cannot survive reruns. If `--from-list` resolves to zero packages, it must still create a header-only `results.tsv`, an empty `failures.list`, and exit successfully.
- In non-strict mode, the dependent matrix must finish and write its artifacts even when some individual packages fail. It should return nonzero only for harness-level failures such as image build failure, container launch failure, or an unreadable `dependents.json`.
- `safe/compat-reports/.gitignore` must enforce one explicit tracked-versus-generated policy. The only tracked files anywhere in `safe/compat-reports/` are `safe/compat-reports/.gitignore`, `safe/compat-reports/README.md`, `safe/compat-reports/issues.md`, `safe/compat-reports/dependents/results.tsv`, `safe/compat-reports/dependents/failures.list`, `safe/compat-reports/dependents-rerun/results.tsv`, and `safe/compat-reports/dependents-rerun/failures.list`. Baseline report directories, per-package logs, archived artifacts, and the entire `safe/compat-reports/final/` tree remain generated workspace artifacts and must stay ignored.
- `safe/compat-reports/README.md` must document the fixed TSV schema, the `logs/<package>.log` convention, the tracked-versus-generated policy, and the expectation that later phases consume the committed `dependents/` and `dependents-rerun/` ledgers in place.
- Extend `safe/tools/run-full-compat.sh` so it accepts `--report-dir` and runs its steps strictly serially. Any phase helper that reads `safe/target/release/libsodium.so` directly must run `cargo build --manifest-path safe/Cargo.toml --release` in the same phase first. Do not overlap `build-deb.sh` with source or relink test runs because `safe/debian/rules:30-43` runs `cargo clean`.
- Do not change `dependents.json` in this phase.

## Verification Phases
### `check_workflow_baseline_tester`
Phase ID: `check_workflow_baseline_tester`

Type: `check`

Bounce Target: `impl_workflow_baseline`

Purpose: Verify the baseline workflow against the current full ABI, package build, upstream source and relink suites, and representative runtime and compile-time dependents while using the checked-in image and report helpers.

Commands:
- `cargo test --manifest-path safe/Cargo.toml --all-targets`
- `cargo build --manifest-path safe/Cargo.toml --release`
- `./safe/tools/check-symbols.sh full`
- `./safe/tools/run-original-c-tests.sh --all`
- `./safe/tools/relink-original-objects.sh --all`
- `./safe/tools/build-deb.sh`
- `./safe/tools/run-dependent-matrix.sh --mode safe --report-dir safe/compat-reports/baseline-runtime --only minisign`
- `./safe/tools/run-dependent-matrix.sh --mode safe --report-dir safe/compat-reports/baseline-compile --only librust-libsodium-sys-dev`
- `test -f safe/compat-reports/baseline-runtime/failures.list`
- `test -f safe/compat-reports/baseline-compile/failures.list`
- `head -n 1 safe/compat-reports/baseline-runtime/results.tsv | rg '^package\tmode\tstatus\tlog_path$'`
- `head -n 1 safe/compat-reports/baseline-compile/results.tsv | rg '^package\tmode\tstatus\tlog_path$'`
- `test "$(tail -n +2 safe/compat-reports/baseline-runtime/results.tsv | wc -l)" -eq 1`
- `test "$(tail -n +2 safe/compat-reports/baseline-compile/results.tsv | wc -l)" -eq 1`
- `awk -F'\t' 'NR == 2 {exit !($1 == "minisign" && $2 == "safe" && $4 == "logs/minisign.log")} END {exit NR == 2 ? 0 : 1}' safe/compat-reports/baseline-runtime/results.tsv`
- `awk -F'\t' 'NR == 2 {exit !($1 == "librust-libsodium-sys-dev" && $2 == "safe" && $4 == "logs/librust-libsodium-sys-dev.log")} END {exit NR == 2 ? 0 : 1}' safe/compat-reports/baseline-compile/results.tsv`
- `test -f safe/compat-reports/baseline-runtime/logs/minisign.log`
- `test -f safe/compat-reports/baseline-compile/logs/librust-libsodium-sys-dev.log`

### `check_workflow_baseline_senior`
Phase ID: `check_workflow_baseline_senior`

Type: `check`

Bounce Target: `impl_workflow_baseline`

Purpose: Review that workflow drivers default to the full ABI, consume the checked-in dependent image, and no longer contain an inline Dockerfile heredoc, without treating the continued presence of checkpoint-manifest names as a failure.

Commands:
- `cargo build --manifest-path safe/Cargo.toml --release`
- `./safe/tools/check-symbols.sh`
- `rg -n 'safe/docker/dependents\.Dockerfile|build-dependent-image\.sh|run-dependent-matrix\.sh' safe/tools/run-full-compat.sh test-original.sh`
- `if rg -n "docker build -t .* - <<'DOCKERFILE'" test-original.sh; then exit 1; fi`
- `git diff --check`

## Success Criteria
- baseline Rust integration tests remain green
- the full 609-symbol ABI check remains green both explicitly and via the no-argument default
- all 77 upstream C source tests remain green
- all 77 upstream object relink tests remain green
- `.deb` packages still build
- one runtime dependent and one compile-time dependent still pass through the Docker harness
- the checked-in workflow drivers keep the topology linear, inline-only, and explicit-phase only while leaving `test-original.sh --mode original` available as a control path
- `safe/compat-reports/.gitignore` and `safe/compat-reports/README.md` codify the exact tracked-versus-generated file set and the fixed `results.tsv` plus `logs/<package>.log` contract
- checkpoint manifests remain synced repo artifacts, and verification focuses on the default full-ABI behavior rather than requiring `foundation`, `through_symmetric`, or `through_public_key` to disappear
- no workflow driver still owns an inline Dockerfile heredoc

## Git Commit Requirement
The implementer must commit work to git before yielding.
