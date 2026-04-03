# Phase 06 Final Hardening

## Phase Name
Final hardening, unsafe reduction, and full-matrix closure

## Implement Phase ID
`impl_safe_final_hardening`

## Preexisting Inputs
- Phase 1 through phase 5 outputs expected to exist: `safe/Cargo.toml`, `safe/build.rs`, `safe/src/lib.rs`, `safe/src/abi/mod.rs`, `safe/src/abi/types.rs`, `safe/src/ffi/mod.rs`, `safe/src/ffi/helpers.rs`, `safe/src/foundation/core.rs`, `safe/src/foundation/runtime.rs`, `safe/src/foundation/randombytes.rs`, `safe/src/foundation/utils.rs`, `safe/src/foundation/codecs.rs`, `safe/src/foundation/version.rs`, `safe/src/foundation/verify.rs`, `safe/src/hash/mod.rs`, `safe/src/hash/sha256.rs`, `safe/src/hash/sha512.rs`, `safe/src/generichash/mod.rs`, `safe/src/auth/mod.rs`, `safe/src/auth/hmacsha256.rs`, `safe/src/auth/hmacsha512.rs`, `safe/src/auth/hmacsha512256.rs`, `safe/src/onetimeauth/poly1305.rs`, `safe/src/stream/chacha20.rs`, `safe/src/stream/salsa20.rs`, `safe/src/stream/salsa2012.rs`, `safe/src/stream/salsa208.rs`, `safe/src/stream/xsalsa20.rs`, `safe/src/stream/xchacha20.rs`, `safe/src/core/hsalsa20.rs`, `safe/src/core/hchacha20.rs`, `safe/src/core/salsa.rs`, `safe/src/core/ed25519.rs`, `safe/src/core/ristretto255.rs`, `safe/src/secretbox/mod.rs`, `safe/src/secretstream.rs`, `safe/src/shorthash.rs`, `safe/src/aead/chacha20poly1305.rs`, `safe/src/aead/xchacha20poly1305.rs`, `safe/src/aead/aes256gcm.rs`, `safe/src/scalarmult/mod.rs`, `safe/src/scalarmult/curve25519.rs`, `safe/src/scalarmult/ed25519.rs`, `safe/src/scalarmult/ristretto255.rs`, `safe/src/sign/mod.rs`, `safe/src/sign/ed25519.rs`, `safe/src/sign/legacy_edwards25519sha512batch.rs`, `safe/src/box_api/mod.rs`, `safe/src/box_api/curve25519xsalsa20poly1305.rs`, `safe/src/box_api/curve25519xchacha20poly1305.rs`, `safe/src/kx.rs`, `safe/src/pwhash/mod.rs`, `safe/src/pwhash/argon2.rs`, `safe/src/pwhash/scrypt.rs`, `safe/src/kdf.rs`, `safe/cabi/weak_runtime.c`, `safe/cabi/libsodium.map`, `safe/cabi/expected/foundation.symbols`, `safe/cabi/expected/through_symmetric.symbols`, `safe/cabi/expected/through_public_key.symbols`, `safe/cabi/expected/full.symbols`, `safe/cabi/expected/upstream-kinds.tsv`, `safe/tools/sync-upstream-interface.sh`, `safe/tools/check-symbols.sh`, `safe/tools/run-original-c-tests.sh`, `safe/tools/relink-original-objects.sh`, `safe/tools/build-deb.sh`, `safe/tools/run-full-compat.sh`, `safe/tests/abi_layout.rs`, `safe/tests/abi_symbols.rs`, `safe/tests/ported_foundation.rs`, `safe/tests/ported_symmetric.rs`, `safe/tests/ported_public_key.rs`, `safe/tests/cve_2025_69277.rs`, `safe/tests/ported_pwhash.rs`, `safe/tests/ported_all.rs`, `safe/include/sodium.h`, `safe/debian/control`, `safe/debian/rules`, `safe/debian/changelog`, `safe/debian/source/format`, `safe/debian/libsodium23.install`, `safe/debian/libsodium-dev.install`, `safe/debian/libsodium23.symbols`, `safe/packaging/libsodium.pc.in`, and the updated `test-original.sh`
- Authoritative upstream closure inputs already present in the workspace: `original/configure.ac`, `original/src/libsodium/include/Makefile.am`, `original/src/libsodium/include/sodium.h`, `original/src/libsodium/Makefile.am`, `original/debian/libsodium23.symbols`, `original/libsodium.pc.in`, `original/debian/control`, `original/debian/rules`, `original/debian/libsodium23.install`, `original/debian/libsodium-dev.install`, `original/debian/patches/CVE-2025-69277.patch`, `original/src/libsodium/.libs/libsodium.so`, `original/test/default/Makefile.am`, `original/test/default/test-suite.log`, `original/test/default/hash2.exp`, `relevant_cves.json`, and `dependents.json`
- Existing full test corpus and relink corpus that must still be consumed in place: every concrete `.c`, `.exp`, and `.o` artifact already required by phase 5, including `original/test/default/xchacha20.c`, `original/test/default/xchacha20.exp`, `original/test/default/xchacha20.o`, `original/test/default/keygen.c`, `original/test/default/keygen.exp`, `original/test/default/keygen.o`, `original/test/default/metamorphic.c`, `original/test/default/metamorphic.exp`, `original/test/default/metamorphic.o`, and `original/test/default/misuse.c`, `original/test/default/misuse.exp`, `original/test/default/misuse.o`

## New Outputs
- A release-candidate `safe/` tree with only necessary `unsafe` blocks remaining
- A full verification record that every source-, link-, runtime-, package-, and CVE-mitigation requirement passes

## File Changes
- Modify only the files that remain necessary after full-matrix failures are discovered
- Expected touch points are limited to `safe/src/**/*`, `safe/tests/**/*`, `safe/debian/*`, `safe/tools/*`, and `test-original.sh`

## Implementation Details
- Audit the remaining `unsafe` usage and move it behind narrow, documented boundaries:
- C ABI pointer translation and buffer overlap handling
- `mmap`/`mprotect`/`mlock`/`munlock` and similar OS calls
- CPU feature intrinsics or any remaining SIMD-specific shims
- weak-symbol and data-symbol export shims
- Ensure no `panic!` can cross the FFI boundary. Either abort or catch-and-abort inside the export macros.
- Fix any remaining semantic mismatches discovered by the full matrix, especially in zeroization, error return codes, stateful API resets, package install paths, library resolution, or object relinking.
- Verify the final shared object does not introduce unexpected dynamic dependencies relative to upstream, or document and package any unavoidable ones.
- Keep the final package metadata, pkg-config surface, synced headers, and expected-symbol manifests aligned with the same upstream authorities used in earlier phases; hardening is not a license to fork those synced derivatives.
- Defer any targeted performance work for hot families such as Curve25519, ChaCha20/Poly1305, BLAKE2b, and Argon2 until correctness is complete. Performance is not a gating reason to relax compatibility or safety requirements.

## Verification Phases
### `check_safe_final_hardening`
Type: `check`

Bounce Target: `impl_safe_final_hardening`

Purpose: Run the full matrix end to end and ensure the remaining unsafe surface is limited to unavoidable FFI, OS, and CPU-feature boundaries.

Commands:
- `cargo build --manifest-path safe/Cargo.toml --release`
- `cargo test --manifest-path safe/Cargo.toml --all-targets`
- `./safe/tools/check-symbols.sh --expected safe/cabi/expected/full.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so`
- `./safe/tools/run-original-c-tests.sh --all`
- `./safe/tools/relink-original-objects.sh --all`
- `./safe/tools/build-deb.sh`
- `./test-original.sh --mode safe`
- `rg -n "\\bunsafe\\b" safe/src safe/tests safe/build.rs safe/cabi`
- `readelf -d safe/target/release/libsodium.so | rg 'SONAME|NEEDED'`

## Success Criteria
- The full command matrix above is green.
- The checker explicitly confirms 609 exported symbols exactly match the upstream ABI contract.
- The checker explicitly confirms the shared object still has SONAME `libsodium.so.23`.
- The checker explicitly confirms the final `run-original-c-tests.sh --all` path covers exactly the 77 runnable upstream C tests, not the orphan `hash2.exp`.
- The checker explicitly confirms all 77 upstream C tests pass from source.
- The checker explicitly confirms all 77 upstream prebuilt object files relink and run successfully.
- The checker explicitly confirms all 16 dependent entries in `dependents.json` pass through the modified Docker harness.
- The checker explicitly confirms CVE-2025-69277 remains fixed.

## Git Commit Requirement
The implementer must commit phase 6 work to git before yielding.
