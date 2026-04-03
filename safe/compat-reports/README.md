# Compatibility Reports

`safe/compat-reports/` separates durable compatibility ledgers from generated run artifacts.

The fixed ledger schema for every `results.tsv` in this tree is:

```text
package<TAB>mode<TAB>status<TAB>log_path
```

- `status` may only be `PASS`, `FAIL`, or `WARN`.
- `log_path` is always report-root-relative and uses the `logs/<package>.log` convention.
- Data rows stay in the selected package order.
- `failures.list` contains one package name per line for every `FAIL` or `WARN` row, in the same order as the corresponding `results.tsv` rows.

The fixed deep-check success markers introduced for the phase-2 dependent
runtime cases are:

- `QTOX_PROFILE_OK`
- `FASTD_KEYPAIR_OK`
- `CURVEDNS_KEYPAIR_OK`
- `NIX_SIGN_VERIFY_OK`
- `PYNACL_SIGN_VERIFY_OK`
- `RBNACL_SIGN_VERIFY_OK`

Those markers only appear after the harness has completed the corresponding
behavior validation. The `dependents/` report also has a generated artifact
contract:

- `logs/<package>.log` always contains the complete per-package execution log.
- `artifacts/<package>/` may contain copied side effects or generated key
  material that the package produced during its check.
- Only `safe/compat-reports/dependents/results.tsv` and
  `safe/compat-reports/dependents/failures.list` are phase-2 durable inputs;
  `logs/`, `artifacts/`, and any rerun directories remain generated workspace
  state.

Tracked files in this tree are limited to:

- `safe/compat-reports/.gitignore`
- `safe/compat-reports/README.md`
- `safe/compat-reports/issues.md`
- `safe/compat-reports/dependents/results.tsv`
- `safe/compat-reports/dependents/failures.list`
- `safe/compat-reports/dependents-rerun/results.tsv`
- `safe/compat-reports/dependents-rerun/failures.list`

Everything else under `safe/compat-reports/` is generated workspace state and stays ignored, including baseline report directories, per-package `logs/`, archived artifacts, and the entire `safe/compat-reports/final/` tree.

Later phases must consume the committed `dependents/` and `dependents-rerun/` ledgers in place instead of inventing replacement ledgers elsewhere.
