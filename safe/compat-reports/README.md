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
