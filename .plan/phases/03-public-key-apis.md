# Phase 03 Public Key APIs

## Phase Name
Curves, signatures, box, and other public-key APIs

## Implement Phase ID
`impl_safe_public_key`

## Preexisting Inputs
- Phase 1 outputs expected to exist: `safe/Cargo.toml`, `safe/build.rs`, `safe/src/lib.rs`, `safe/src/abi/mod.rs`, `safe/src/abi/types.rs`, `safe/src/ffi/mod.rs`, `safe/src/ffi/helpers.rs`, `safe/src/foundation/core.rs`, `safe/src/foundation/runtime.rs`, `safe/src/foundation/randombytes.rs`, `safe/src/foundation/utils.rs`, `safe/src/foundation/codecs.rs`, `safe/src/foundation/version.rs`, `safe/src/foundation/verify.rs`, `safe/cabi/weak_runtime.c`, `safe/cabi/libsodium.map`, `safe/cabi/expected/foundation.symbols`, `safe/cabi/expected/through_symmetric.symbols`, `safe/cabi/expected/through_public_key.symbols`, `safe/cabi/expected/full.symbols`, `safe/cabi/expected/upstream-kinds.tsv`, `safe/tools/sync-upstream-interface.sh`, `safe/tools/check-symbols.sh`, `safe/tools/run-original-c-tests.sh`, `safe/tools/relink-original-objects.sh`, `safe/tests/abi_layout.rs`, `safe/tests/abi_symbols.rs`, `safe/tests/ported_foundation.rs`, and `safe/include/sodium.h`
- Phase 2 outputs expected to exist: `safe/src/hash/mod.rs`, `safe/src/hash/sha256.rs`, `safe/src/hash/sha512.rs`, `safe/src/generichash/mod.rs`, `safe/src/auth/mod.rs`, `safe/src/auth/hmacsha256.rs`, `safe/src/auth/hmacsha512.rs`, `safe/src/auth/hmacsha512256.rs`, `safe/src/onetimeauth/poly1305.rs`, `safe/src/stream/chacha20.rs`, `safe/src/stream/salsa20.rs`, `safe/src/stream/salsa2012.rs`, `safe/src/stream/salsa208.rs`, `safe/src/stream/xsalsa20.rs`, `safe/src/stream/xchacha20.rs`, `safe/src/core/hsalsa20.rs`, `safe/src/core/hchacha20.rs`, `safe/src/core/salsa.rs`, `safe/src/secretbox/mod.rs`, `safe/src/secretstream.rs`, `safe/src/shorthash.rs`, `safe/src/aead/chacha20poly1305.rs`, `safe/src/aead/xchacha20poly1305.rs`, and `safe/tests/ported_symmetric.rs`
- Upstream public-key authorities: `original/src/libsodium/Makefile.am`, `original/test/default/Makefile.am`, `original/test/default/hash2.exp`, `original/src/libsodium/include/sodium/crypto_scalarmult.h`, `original/src/libsodium/include/sodium/crypto_scalarmult_curve25519.h`, `original/src/libsodium/include/sodium/crypto_scalarmult_ed25519.h`, `original/src/libsodium/include/sodium/crypto_scalarmult_ristretto255.h`, `original/src/libsodium/include/sodium/crypto_core_ed25519.h`, `original/src/libsodium/include/sodium/crypto_core_ristretto255.h`, `original/src/libsodium/include/sodium/crypto_sign.h`, `original/src/libsodium/include/sodium/crypto_sign_ed25519.h`, `original/src/libsodium/include/sodium/crypto_sign_edwards25519sha512batch.h`, `original/src/libsodium/include/sodium/crypto_box.h`, `original/src/libsodium/include/sodium/crypto_box_curve25519xsalsa20poly1305.h`, `original/src/libsodium/include/sodium/crypto_box_curve25519xchacha20poly1305.h`, `original/src/libsodium/include/sodium/crypto_kx.h`, `original/src/libsodium/crypto_scalarmult/`, `original/src/libsodium/crypto_core/ed25519/`, `original/src/libsodium/crypto_sign/`, `original/src/libsodium/crypto_box/`, `original/src/libsodium/crypto_kx/crypto_kx.c`, `relevant_cves.json`, and `original/debian/patches/CVE-2025-69277.patch`
- Existing public-key test artifacts that must be consumed in place: `original/test/default/scalarmult.c`, `original/test/default/scalarmult.exp`, `original/test/default/scalarmult.o`, `original/test/default/scalarmult2.c`, `original/test/default/scalarmult2.exp`, `original/test/default/scalarmult2.o`, `original/test/default/scalarmult5.c`, `original/test/default/scalarmult5.exp`, `original/test/default/scalarmult5.o`, `original/test/default/scalarmult6.c`, `original/test/default/scalarmult6.exp`, `original/test/default/scalarmult6.o`, `original/test/default/scalarmult7.c`, `original/test/default/scalarmult7.exp`, `original/test/default/scalarmult7.o`, `original/test/default/scalarmult8.c`, `original/test/default/scalarmult8.exp`, `original/test/default/scalarmult8.o`, `original/test/default/scalarmult_ed25519.c`, `original/test/default/scalarmult_ed25519.exp`, `original/test/default/scalarmult_ed25519.o`, `original/test/default/scalarmult_ristretto255.c`, `original/test/default/scalarmult_ristretto255.exp`, `original/test/default/scalarmult_ristretto255.o`, `original/test/default/core_ed25519.c`, `original/test/default/core_ed25519.exp`, `original/test/default/core_ed25519.o`, `original/test/default/core_ristretto255.c`, `original/test/default/core_ristretto255.exp`, `original/test/default/core_ristretto255.o`, `original/test/default/ed25519_convert.c`, `original/test/default/ed25519_convert.exp`, `original/test/default/ed25519_convert.o`, `original/test/default/box.c`, `original/test/default/box.exp`, `original/test/default/box.o`, `original/test/default/box2.c`, `original/test/default/box2.exp`, `original/test/default/box2.o`, `original/test/default/box7.c`, `original/test/default/box7.exp`, `original/test/default/box7.o`, `original/test/default/box8.c`, `original/test/default/box8.exp`, `original/test/default/box8.o`, `original/test/default/box_easy.c`, `original/test/default/box_easy.exp`, `original/test/default/box_easy.o`, `original/test/default/box_easy2.c`, `original/test/default/box_easy2.exp`, `original/test/default/box_easy2.o`, `original/test/default/box_seed.c`, `original/test/default/box_seed.exp`, `original/test/default/box_seed.o`, `original/test/default/box_seal.c`, `original/test/default/box_seal.exp`, `original/test/default/box_seal.o`, `original/test/default/sign.c`, `original/test/default/sign.exp`, `original/test/default/sign.o`, `original/test/default/kx.c`, `original/test/default/kx.exp`, `original/test/default/kx.o`, and `original/test/default/xchacha20.c`, `original/test/default/xchacha20.exp`, `original/test/default/xchacha20.o`

## New Outputs
- Rust implementations for Curve25519, Ed25519, Ristretto255, signatures, key exchange, and box APIs
- Dedicated Rust CVE regression tests for invalid subgroup points
- Ported Rust tests for the public-key families, including the box section of the XChaCha upstream test

## File Changes
- Create `safe/src/scalarmult/mod.rs`
- Create `safe/src/scalarmult/curve25519.rs`
- Create `safe/src/scalarmult/ed25519.rs`
- Create `safe/src/scalarmult/ristretto255.rs`
- Create `safe/src/core/ed25519.rs`
- Create `safe/src/core/ristretto255.rs`
- Create `safe/src/sign/mod.rs`
- Create `safe/src/sign/ed25519.rs`
- Create `safe/src/sign/legacy_edwards25519sha512batch.rs`
- Create `safe/src/box_api/mod.rs`
- Create `safe/src/box_api/curve25519xsalsa20poly1305.rs`
- Create `safe/src/box_api/curve25519xchacha20poly1305.rs`
- Create `safe/src/kx.rs`
- Create `safe/tests/ported_public_key.rs`
- Create `safe/tests/cve_2025_69277.rs`

## Implementation Details
- Header/API coverage for this phase is every export from `sodium/crypto_scalarmult*.h`, `sodium/crypto_core_ed25519.h`, `sodium/crypto_core_ristretto255.h`, `sodium/crypto_sign*.h`, `sodium/crypto_box*.h`, and `sodium/crypto_kx.h`.
- Keep consuming the phase-1-synced headers, version script, and expected-symbol manifests in place; the public-key phase extends the implementation without regenerating those synced derivatives unless work bounces back to `impl_safe_foundation`.
- Mirror the public-key layering in `original/src/libsodium/Makefile.am` so the Curve25519, Ed25519, Ristretto255, box, and kx modules remain aligned with the upstream family split.
- Implement `crypto_scalarmult_*` around a Curve25519 backend that preserves the upstream return-value rule: if the output point is all zeroes, return `-1`.
- Implement `crypto_scalarmult_ed25519_*` and `crypto_scalarmult_ristretto255_*`, including the `noclamp` and `base_noclamp` Ed25519 variants.
- Implement `crypto_kx_*` exactly as upstream composes it in `original/src/libsodium/crypto_kx/crypto_kx.c`: X25519 shared secret, BLAKE2b over `q || client_pk || server_pk`, and the correct RX/TX split order.
- Implement Ed25519 and Ristretto core operations, conversions, detached and attached signatures, `*_ph_*` streaming state, and the obsolete `crypto_sign_edwards25519sha512batch_*` exports so existing NaCl-era callers still link.
- Implement `crypto_sign_ed25519_pk_to_curve25519`, `crypto_sign_ed25519_sk_to_curve25519`, `crypto_sign_ed25519_sk_to_seed`, and `crypto_sign_ed25519_sk_to_pk`, not just the sign/verify entry points.
- Implement box APIs as the same compositions upstream uses:
- `crypto_box_curve25519xsalsa20poly1305_*`
- `crypto_box_curve25519xchacha20poly1305_*`
- high-level `crypto_box_*` wrappers and easy/detached/seal variants
- Treat `original/test/default/xchacha20.c` line 299 as the first point where the full original C XChaCha test becomes satisfiable. Phase 2 ports its stream and secretbox sections into Rust tests, but the original C `xchacha20` verifier is deferred here because `tv_box_xchacha20poly1305()` at lines 299-426 depends on `crypto_box_curve25519xchacha20poly1305_*`.
- Carry the CVE-2025-69277 fix into the Rust validation path: after multiplying by the subgroup order, accept only if `X == 0` and `Y == Z`, matching `original/src/libsodium/crypto_core/ed25519/ref10/ed25519_ref10.c`.
- Add the exact regression vector `not_main_subgroup_p` from `original/test/default/core_ed25519.c` to the Rust regression suite.
- Review every public entry point that accepts untrusted Ed25519 points, including validation, add/sub, scalar multiplication, and conversion helpers, so the subgroup fix is not isolated to one helper.

## Verification Phases
### `check_safe_public_key`
Type: `check`

Bounce Target: `impl_safe_public_key`

Purpose: Verify curve operations, Ed25519 and Ristretto correctness, box/kx/sign APIs, deprecated compatibility exports, the full upstream `xchacha20` test binary, and the CVE-2025-69277 regression.

Commands:
- `cargo build --manifest-path safe/Cargo.toml --release`
- `cargo test --manifest-path safe/Cargo.toml --test ported_public_key`
- `cargo test --manifest-path safe/Cargo.toml --test cve_2025_69277`
- `./safe/tools/run-original-c-tests.sh scalarmult scalarmult2 scalarmult5 scalarmult6 scalarmult7 scalarmult8 scalarmult_ed25519 scalarmult_ristretto255 core_ed25519 core_ristretto255 ed25519_convert box box2 box7 box8 box_easy box_easy2 box_seed box_seal sign kx xchacha20`
- `./safe/tools/relink-original-objects.sh scalarmult scalarmult2 scalarmult5 scalarmult6 scalarmult7 scalarmult8 scalarmult_ed25519 scalarmult_ristretto255 core_ed25519 core_ristretto255 ed25519_convert box box2 box7 box8 box_easy box_easy2 box_seed box_seal sign kx xchacha20`
- `./safe/tools/check-symbols.sh --expected safe/cabi/expected/through_public_key.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so`

## Success Criteria
- The public-key C tests listed above pass from source and from relinked object files.
- The checker explicitly confirms the CVE regression by exercising the invalid subgroup point and ensuring `crypto_core_ed25519_is_valid_point()` returns `0`.
- The checker confirms the deprecated `crypto_sign_edwards25519sha512batch_*` symbols still exist and link.
- The checker confirms the original `xchacha20` C test now passes, proving that the `crypto_box_curve25519xchacha20poly1305_*` family is wired into the same XChaCha coverage path as upstream.
- The release artifact matches `safe/cabi/expected/through_public_key.symbols` exactly, with no earlier-phase symbol removals or kind changes.

## Git Commit Requirement
The implementer must commit phase 3 work to git before yielding.
