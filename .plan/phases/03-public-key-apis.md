# Phase 03 Public-Key APIs

## Phase Name
Curves, signatures, box, and other public-key APIs

## Implement Phase ID
`impl_safe_public_key`

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/build.rs`
- `safe/src/lib.rs`
- `safe/src/abi/mod.rs`
- `safe/src/abi/types.rs`
- `safe/src/ffi/mod.rs`
- `safe/src/ffi/helpers.rs`
- `safe/src/foundation/*.rs`
- `safe/src/hash/*.rs`
- `safe/src/generichash/mod.rs`
- `safe/src/auth/*.rs`
- `safe/src/onetimeauth/poly1305.rs`
- `safe/src/stream/*.rs`
- `safe/src/core/hsalsa20.rs`
- `safe/src/core/hchacha20.rs`
- `safe/src/core/salsa.rs`
- `safe/src/secretbox/mod.rs`
- `safe/src/secretstream.rs`
- `safe/src/shorthash.rs`
- `safe/src/aead/chacha20poly1305.rs`
- `safe/src/aead/xchacha20poly1305.rs`
- `safe/cabi/libsodium.map`
- `safe/cabi/expected/through_public_key.symbols`
- `safe/cabi/expected/upstream-kinds.tsv`
- `safe/tools/check-symbols.sh`
- `safe/tools/run-original-c-tests.sh`
- `safe/tools/relink-original-objects.sh`
- `safe/tests/ported_symmetric.rs`
- `safe/include/sodium.h`
- `safe/include/sodium/*.h`
- `original/src/libsodium/include/sodium/crypto_scalarmult.h`
- `original/src/libsodium/include/sodium/crypto_scalarmult_curve25519.h`
- `original/src/libsodium/include/sodium/crypto_scalarmult_ed25519.h`
- `original/src/libsodium/include/sodium/crypto_scalarmult_ristretto255.h`
- `original/src/libsodium/include/sodium/crypto_core_ed25519.h`
- `original/src/libsodium/include/sodium/crypto_core_ristretto255.h`
- `original/src/libsodium/include/sodium/crypto_sign.h`
- `original/src/libsodium/include/sodium/crypto_sign_ed25519.h`
- `original/src/libsodium/include/sodium/crypto_sign_edwards25519sha512batch.h`
- `original/src/libsodium/include/sodium/crypto_box.h`
- `original/src/libsodium/include/sodium/crypto_box_curve25519xsalsa20poly1305.h`
- `original/src/libsodium/include/sodium/crypto_box_curve25519xchacha20poly1305.h`
- `original/src/libsodium/include/sodium/crypto_kx.h`
- `original/src/libsodium/crypto_scalarmult/**/*`
- `original/src/libsodium/crypto_core/ed25519/**/*`
- `original/src/libsodium/crypto_sign/**/*`
- `original/src/libsodium/crypto_box/**/*`
- `original/src/libsodium/crypto_kx/crypto_kx.c`
- `original/debian/patches/CVE-2025-69277.patch`
- `relevant_cves.json`
- `original/test/default/core_ed25519.c`
- `original/test/default/xchacha20.c`
- `original/test/default/*.c`
- `original/test/default/*.o`
- `original/test/default/*.exp`

## New Outputs
- Rust implementations for Curve25519, Ed25519, Ristretto255, signatures, key exchange, and box APIs
- Dedicated Rust CVE regression tests for invalid subgroup points
- Rust-ported tests for the public-key families, including the box section of upstream `xchacha20.c`

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
- Consume the carried-forward manifests, synced headers, tooling, and symmetric implementation artifacts in place. Do not regenerate the manifest set or re-stage the synced header mirror here.
- Cover every export from `sodium/crypto_scalarmult*.h`, `sodium/crypto_core_ed25519.h`, `sodium/crypto_core_ristretto255.h`, `sodium/crypto_sign*.h`, `sodium/crypto_box*.h`, and `sodium/crypto_kx.h`.
- Implement `crypto_scalarmult_*` around a Curve25519 backend that preserves the upstream rule that an all-zero output returns `-1`.
- Implement `crypto_scalarmult_ed25519_*` and `crypto_scalarmult_ristretto255_*`, including the `noclamp` and `base_noclamp` Ed25519 variants.
- Implement `crypto_kx_*` exactly as upstream composes it in `original/src/libsodium/crypto_kx/crypto_kx.c`: X25519 shared secret, BLAKE2b over `q || client_pk || server_pk`, and the correct RX/TX split order.
- Implement Ed25519 and Ristretto core operations, conversions, detached and attached signatures, `*_ph_*` streaming state, and the obsolete `crypto_sign_edwards25519sha512batch_*` exports so existing NaCl-era callers still link.
- Implement `crypto_sign_ed25519_pk_to_curve25519`, `crypto_sign_ed25519_sk_to_curve25519`, `crypto_sign_ed25519_sk_to_seed`, and `crypto_sign_ed25519_sk_to_pk`.
- Implement `crypto_box_curve25519xsalsa20poly1305_*`, `crypto_box_curve25519xchacha20poly1305_*`, and the high-level `crypto_box_*` wrappers plus easy, detached, and seal variants using the same compositions upstream uses.
- Treat this as the first phase where the original C `xchacha20` binary becomes satisfiable. Phase 2 only ports the stream and secretbox sections into Rust tests; phase 3 must enable the box section and run the original C test.
- Carry the CVE-2025-69277 fix into the Rust validation path: after multiplying by the subgroup order, accept only if `X == 0` and `Y == Z`.
- Add the exact `not_main_subgroup_p` regression vector from `original/test/default/core_ed25519.c` to the Rust regression suite.
- Review every public entry point that accepts untrusted Ed25519 points so the subgroup-validation fix is not isolated to one helper.

## Verification Phases
- `Phase ID`: `check_safe_public_key`
- `Type`: `check`
- `bounce_target`: `impl_safe_public_key`
- `Purpose`: Verify curve operations, Ed25519 and Ristretto correctness, box, key-exchange, and signature APIs, deprecated compatibility exports, the full upstream `xchacha20` test binary, and the CVE-2025-69277 regression.
- `Commands`:
```bash
cargo build --manifest-path safe/Cargo.toml --release
cargo test --manifest-path safe/Cargo.toml --test ported_public_key
cargo test --manifest-path safe/Cargo.toml --test cve_2025_69277
./safe/tools/run-original-c-tests.sh scalarmult scalarmult2 scalarmult5 scalarmult6 scalarmult7 scalarmult8 scalarmult_ed25519 scalarmult_ristretto255 core_ed25519 core_ristretto255 ed25519_convert box box2 box7 box8 box_easy box_easy2 box_seed box_seal sign kx xchacha20
./safe/tools/relink-original-objects.sh scalarmult scalarmult2 scalarmult5 scalarmult6 scalarmult7 scalarmult8 scalarmult_ed25519 scalarmult_ristretto255 core_ed25519 core_ristretto255 ed25519_convert box box2 box7 box8 box_easy box_easy2 box_seed box_seal sign kx xchacha20
./safe/tools/check-symbols.sh --expected safe/cabi/expected/through_public_key.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so
```

## Success Criteria
- All listed public-key C tests pass from source and from relinked upstream object files
- The checker exercises the invalid subgroup point and confirms `crypto_core_ed25519_is_valid_point()` returns `0`
- The deprecated `crypto_sign_edwards25519sha512batch_*` symbols still exist and link
- The original `xchacha20` C test passes once the box family is present
- The release artifact matches `safe/cabi/expected/through_public_key.symbols` exactly, with no earlier-phase symbol removals or kind changes

## Git Commit Requirement
The implementer must commit work to git before yielding.
