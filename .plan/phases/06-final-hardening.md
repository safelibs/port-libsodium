# Phase 06 Final Hardening

## Phase Name
Final hardening, unsafe reduction, and full-matrix closure

## Implement Phase ID
`impl_safe_final_hardening`

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/build.rs`
- `safe/src/lib.rs`
- `safe/src/abi/*.rs`
- `safe/src/ffi/*.rs`
- `safe/src/foundation/*.rs`
- `safe/src/hash/*.rs`
- `safe/src/generichash/mod.rs`
- `safe/src/auth/*.rs`
- `safe/src/onetimeauth/*.rs`
- `safe/src/stream/*.rs`
- `safe/src/core/*.rs`
- `safe/src/secretbox/*.rs`
- `safe/src/secretstream.rs`
- `safe/src/shorthash.rs`
- `safe/src/aead/*.rs`
- `safe/src/scalarmult/*.rs`
- `safe/src/sign/*.rs`
- `safe/src/box_api/*.rs`
- `safe/src/kx.rs`
- `safe/src/pwhash/*.rs`
- `safe/src/kdf.rs`
- `safe/tests/abi_layout.rs`
- `safe/tests/abi_symbols.rs`
- `safe/tests/ported_foundation.rs`
- `safe/tests/ported_symmetric.rs`
- `safe/tests/ported_public_key.rs`
- `safe/tests/cve_2025_69277.rs`
- `safe/tests/ported_pwhash.rs`
- `safe/tests/ported_all.rs`
- `safe/debian/control`
- `safe/debian/rules`
- `safe/debian/changelog`
- `safe/debian/source/format`
- `safe/debian/libsodium23.install`
- `safe/debian/libsodium-dev.install`
- `safe/debian/libsodium23.symbols`
- `safe/packaging/libsodium.pc.in`
- `safe/tools/sync-upstream-interface.sh`
- `safe/tools/check-symbols.sh`
- `safe/tools/run-original-c-tests.sh`
- `safe/tools/relink-original-objects.sh`
- `safe/tools/build-deb.sh`
- `safe/tools/run-full-compat.sh`
- `safe/cabi/weak_runtime.c`
- `safe/cabi/libsodium.map`
- `safe/cabi/expected/foundation.symbols`
- `safe/cabi/expected/through_symmetric.symbols`
- `safe/cabi/expected/through_public_key.symbols`
- `safe/cabi/expected/full.symbols`
- `safe/cabi/expected/upstream-kinds.tsv`
- `safe/include/sodium.h`
- `safe/include/sodium/*.h`
- `original/src/libsodium/.libs/libsodium.so`
- `original/debian/libsodium23.symbols`
- `original/debian/control`
- `original/debian/rules`
- `original/debian/libsodium23.install`
- `original/debian/libsodium-dev.install`
- `original/debian/patches/CVE-2025-69277.patch`
- `original/src/libsodium/include/sodium.h`
- `original/src/libsodium/include/sodium/*.h`
- `original/test/default/Makefile.am`
- `original/test/default/*.c`
- `original/test/default/*.o`
- `original/test/default/*.exp`
- `original/test/default/test-suite.log`
- `relevant_cves.json`
- `dependents.json`
- `test-original.sh`

## New Outputs
- A release-candidate `safe/` tree with only necessary `unsafe` blocks remaining
- A full verification record that source, link, runtime, package, and CVE-mitigation requirements all pass

## File Changes
- Modify only the files that remain necessary after full-matrix failures are discovered
- Keep edits limited to `safe/src/**/*.rs`, `safe/tests/*.rs`, `safe/debian/*`, `safe/tools/*`, and `test-original.sh`

## Implementation Details
- Audit the remaining `unsafe` usage and move it behind narrow, documented boundaries for C ABI pointer translation, overlap-sensitive buffer handling, OS memory-management calls, CPU-feature intrinsics, and weak or data export shims.
- Ensure no `panic!` can cross the FFI boundary. Abort or catch-and-abort inside the export macros instead.
- Fix any remaining semantic mismatches discovered by the full matrix, especially around zeroization, error return codes, stateful API resets, package install paths, library resolution, or object relinking.
- Verify the final shared object does not introduce unexpected dynamic dependencies relative to upstream, or document and package any unavoidable ones.
- Defer targeted performance work until correctness is complete. Performance is not a reason to relax compatibility or safety requirements.
- Preserve the phase-4 full ABI contract, the synced `safe/debian/libsodium23.symbols`, and the synced `safe/packaging/libsodium.pc.in`. If either derivative needs adjustment, regenerate it from the upstream artifact rather than hand-editing it as a new source of truth.
- Keep the final `run-original-c-tests.sh --all` inventory tied to `original/test/default/Makefile.am` plus the extant `.c` and `.o` files so the orphan `hash2.exp` never becomes a fabricated runnable test.

## Verification Phases
- `Phase ID`: `check_safe_final_hardening`
- `Type`: `check`
- `bounce_target`: `impl_safe_final_hardening`
- `Purpose`: Run the full matrix end to end and ensure the remaining unsafe surface is limited to unavoidable FFI, OS, and CPU-feature boundaries.
- `Commands`:
```bash
cargo build --manifest-path safe/Cargo.toml --release
cargo test --manifest-path safe/Cargo.toml --all-targets
./safe/tools/check-symbols.sh --expected safe/cabi/expected/full.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so
./safe/tools/run-original-c-tests.sh --all
./safe/tools/relink-original-objects.sh --all
./safe/tools/build-deb.sh
./test-original.sh --mode safe
rg -n "\\bunsafe\\b" safe/src safe/tests safe/build.rs safe/cabi
readelf -d safe/target/release/libsodium.so | rg 'SONAME|NEEDED'
```

## Success Criteria
- The full verification command matrix is green end to end
- The final shared object exports exactly 609 symbols and still has SONAME `libsodium.so.23`
- `run-original-c-tests.sh --all` covers exactly the 77 runnable upstream C tests and excludes the orphan `hash2.exp`
- All 77 upstream C tests pass from source
- All 77 upstream prebuilt object files relink and run successfully
- All 16 dependents in `dependents.json` pass through the modified Docker harness
- CVE-2025-69277 remains fixed
- Remaining `unsafe` blocks are limited to unavoidable FFI, OS-memory, or CPU-feature boundaries

## Git Commit Requirement
The implementer must commit work to git before yielding.
