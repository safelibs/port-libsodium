# Phase 02 Symmetric Secret-Key APIs

## Phase Name
Symmetric primitives and secret-key APIs

## Implement Phase ID
`impl_safe_symmetric`

## Preexisting Inputs
- `safe/Cargo.toml`
- `safe/build.rs`
- `safe/src/lib.rs`
- `safe/src/abi/mod.rs`
- `safe/src/abi/types.rs`
- `safe/src/ffi/mod.rs`
- `safe/src/ffi/helpers.rs`
- `safe/src/foundation/core.rs`
- `safe/src/foundation/runtime.rs`
- `safe/src/foundation/randombytes.rs`
- `safe/src/foundation/utils.rs`
- `safe/src/foundation/codecs.rs`
- `safe/src/foundation/version.rs`
- `safe/src/foundation/verify.rs`
- `safe/cabi/weak_runtime.c`
- `safe/cabi/libsodium.map`
- `safe/cabi/expected/foundation.symbols`
- `safe/cabi/expected/through_symmetric.symbols`
- `safe/cabi/expected/through_public_key.symbols`
- `safe/cabi/expected/full.symbols`
- `safe/cabi/expected/upstream-kinds.tsv`
- `safe/tools/sync-upstream-interface.sh`
- `safe/tools/check-symbols.sh`
- `safe/tools/run-original-c-tests.sh`
- `safe/tools/relink-original-objects.sh`
- `safe/tests/abi_layout.rs`
- `safe/tests/abi_symbols.rs`
- `safe/tests/ported_foundation.rs`
- `safe/include/sodium.h`
- `safe/include/sodium/*.h`
- `original/src/libsodium/Makefile.am`
- `original/src/libsodium/include/Makefile.am`
- `original/src/libsodium/include/sodium/crypto_hash.h`
- `original/src/libsodium/include/sodium/crypto_hash_sha256.h`
- `original/src/libsodium/include/sodium/crypto_hash_sha512.h`
- `original/src/libsodium/include/sodium/crypto_generichash.h`
- `original/src/libsodium/include/sodium/crypto_generichash_blake2b.h`
- `original/src/libsodium/include/sodium/crypto_auth.h`
- `original/src/libsodium/include/sodium/crypto_auth_hmacsha256.h`
- `original/src/libsodium/include/sodium/crypto_auth_hmacsha512.h`
- `original/src/libsodium/include/sodium/crypto_auth_hmacsha512256.h`
- `original/src/libsodium/include/sodium/crypto_onetimeauth.h`
- `original/src/libsodium/include/sodium/crypto_onetimeauth_poly1305.h`
- `original/src/libsodium/include/sodium/crypto_core_hsalsa20.h`
- `original/src/libsodium/include/sodium/crypto_core_hchacha20.h`
- `original/src/libsodium/include/sodium/crypto_core_salsa20.h`
- `original/src/libsodium/include/sodium/crypto_core_salsa2012.h`
- `original/src/libsodium/include/sodium/crypto_core_salsa208.h`
- `original/src/libsodium/include/sodium/crypto_stream.h`
- `original/src/libsodium/include/sodium/crypto_stream_chacha20.h`
- `original/src/libsodium/include/sodium/crypto_stream_salsa20.h`
- `original/src/libsodium/include/sodium/crypto_stream_salsa2012.h`
- `original/src/libsodium/include/sodium/crypto_stream_salsa208.h`
- `original/src/libsodium/include/sodium/crypto_stream_xsalsa20.h`
- `original/src/libsodium/include/sodium/crypto_stream_xchacha20.h`
- `original/src/libsodium/include/sodium/crypto_secretbox.h`
- `original/src/libsodium/include/sodium/crypto_secretbox_xsalsa20poly1305.h`
- `original/src/libsodium/include/sodium/crypto_secretbox_xchacha20poly1305.h`
- `original/src/libsodium/include/sodium/crypto_secretstream_xchacha20poly1305.h`
- `original/src/libsodium/include/sodium/crypto_shorthash.h`
- `original/src/libsodium/include/sodium/crypto_shorthash_siphash24.h`
- `original/src/libsodium/include/sodium/crypto_aead_chacha20poly1305.h`
- `original/src/libsodium/include/sodium/crypto_aead_xchacha20poly1305.h`
- `original/test/default/Makefile.am`
- `original/test/default/xchacha20.c`
- `original/test/default/*.c`
- `original/test/default/*.o`
- `original/test/default/*.exp`

## New Outputs
- Rust implementations for the symmetric primitive families and their high-level wrappers
- Rust-ported tests for the symmetric families, including the stream and secretbox portions of upstream `xchacha20.c`
- Preserved FFI-visible state layouts for hash, HMAC, BLAKE2b, Poly1305, and secretstream state

## File Changes
- Create `safe/src/hash/mod.rs`
- Create `safe/src/hash/sha256.rs`
- Create `safe/src/hash/sha512.rs`
- Create `safe/src/generichash/mod.rs`
- Create `safe/src/auth/mod.rs`
- Create `safe/src/auth/hmacsha256.rs`
- Create `safe/src/auth/hmacsha512.rs`
- Create `safe/src/auth/hmacsha512256.rs`
- Create `safe/src/onetimeauth/poly1305.rs`
- Create `safe/src/stream/chacha20.rs`
- Create `safe/src/stream/salsa20.rs`
- Create `safe/src/stream/salsa2012.rs`
- Create `safe/src/stream/salsa208.rs`
- Create `safe/src/stream/xsalsa20.rs`
- Create `safe/src/stream/xchacha20.rs`
- Create `safe/src/core/hsalsa20.rs`
- Create `safe/src/core/hchacha20.rs`
- Create `safe/src/core/salsa.rs`
- Create `safe/src/secretbox/mod.rs`
- Create `safe/src/secretstream.rs`
- Create `safe/src/shorthash.rs`
- Create `safe/src/aead/chacha20poly1305.rs`
- Create `safe/src/aead/xchacha20poly1305.rs`
- Create `safe/tests/ported_symmetric.rs`

## Implementation Details
- Consume the phase-1 manifests, synced headers, and compatibility scripts in place. Do not regenerate `safe/cabi/expected/*`, `safe/cabi/libsodium.map`, or `safe/include/*` here.
- Cover every export from `sodium/crypto_hash*.h`, `sodium/crypto_generichash*.h`, `sodium/crypto_auth*.h`, `sodium/crypto_onetimeauth*.h`, `sodium/crypto_core_h*`, `sodium/crypto_core_salsa*`, `sodium/crypto_stream*.h`, `sodium/crypto_secretbox*.h`, `sodium/crypto_secretstream_xchacha20poly1305.h`, `sodium/crypto_shorthash*.h`, `sodium/crypto_aead_chacha20poly1305.h`, and `sodium/crypto_aead_xchacha20poly1305.h`.
- Implement SHA-256 and SHA-512 so `crypto_hash_sha256_state` and `crypto_hash_sha512_state` remain concrete public C layouts.
- Implement BLAKE2b-backed `crypto_generichash_*` and `crypto_generichash_blake2b_*`, including exact `statebytes()` behavior and the aligned 384-byte public state object.
- Implement `crypto_auth`, `crypto_auth_hmacsha256`, `crypto_auth_hmacsha512`, and `crypto_auth_hmacsha512256`, including one-shot, init/update/final, `*_statebytes()`, `*_primitive()`, and keygen exports.
- Implement `crypto_onetimeauth_*` and `crypto_onetimeauth_poly1305_*`, preserving the 256-byte aligned public state layout.
- Implement `crypto_core_salsa20`, `crypto_core_salsa2012`, `crypto_core_salsa208`, `crypto_core_hsalsa20`, and `crypto_core_hchacha20`.
- Implement `crypto_stream`, `crypto_stream_xor`, `crypto_stream_keygen`, the full `crypto_stream_chacha20*` family including IETF and `_xor_ic` variants, the `crypto_stream_salsa20*`, `crypto_stream_salsa2012*`, `crypto_stream_salsa208*`, `crypto_stream_xsalsa20*`, and `crypto_stream_xchacha20*` families.
- Implement `crypto_secretbox_*`, `crypto_secretbox_xsalsa20poly1305_*`, `crypto_secretbox_xchacha20poly1305_*`, `crypto_secretstream_xchacha20poly1305_*`, `crypto_aead_chacha20poly1305_*`, `crypto_aead_chacha20poly1305_ietf_*`, and `crypto_aead_xchacha20poly1305_ietf_*`.
- Preserve overlap-tolerant `memmove` semantics in detached and open-detached secretbox entry points.
- Preserve exact public accessors and primitive-string selectors from the upstream headers.
- Port `tv_stream_xchacha20()` and `tv_secretbox_xchacha20poly1305()` from `original/test/default/xchacha20.c` into `safe/tests/ported_symmetric.rs`.
- Do not run the original C `xchacha20` binary yet. Its `tv_box_xchacha20poly1305()` section depends on the public-key box family that does not arrive until phase 3.
- Run the upstream `metamorphic` test in this phase because it is symmetric-only and exercises chunked-versus-one-shot behavior for generichash, onetimeauth, and HMAC APIs.

## Verification Phases
- `Phase ID`: `check_safe_symmetric`
- `Type`: `check`
- `bounce_target`: `impl_safe_symmetric`
- `Purpose`: Verify the symmetric primitive families, secret-key composite APIs, and the symmetric-only metamorphic tests against both the original C tests and the new Rust-ported tests.
- `Commands`:
```bash
cargo build --manifest-path safe/Cargo.toml --release
cargo test --manifest-path safe/Cargo.toml --test ported_symmetric
./safe/tools/run-original-c-tests.sh hash hash3 generichash generichash2 generichash3 auth auth2 auth3 auth5 auth6 auth7 onetimeauth onetimeauth2 onetimeauth7 chacha20 stream stream2 stream3 stream4 core1 core2 core3 core4 core5 core6 shorthash siphashx24 secretbox secretbox2 secretbox7 secretbox8 secretbox_easy secretbox_easy2 secretstream aead_chacha20poly1305 aead_chacha20poly13052 aead_xchacha20poly1305 metamorphic
./safe/tools/relink-original-objects.sh hash hash3 generichash generichash2 generichash3 auth auth2 auth3 auth5 auth6 auth7 onetimeauth onetimeauth2 onetimeauth7 chacha20 stream stream2 stream3 stream4 core1 core2 core3 core4 core5 core6 shorthash siphashx24 secretbox secretbox2 secretbox7 secretbox8 secretbox_easy secretbox_easy2 secretstream aead_chacha20poly1305 aead_chacha20poly13052 aead_xchacha20poly1305 metamorphic
./safe/tools/check-symbols.sh --expected safe/cabi/expected/through_symmetric.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so
```

## Success Criteria
- All listed symmetric-family C tests pass from source and from relinked upstream object files
- `safe/tests/ported_symmetric.rs` covers the XChaCha stream and XChaCha secretbox vectors from `original/test/default/xchacha20.c`
- The release artifact matches `safe/cabi/expected/through_symmetric.symbols` exactly
- No phase-1 exports disappear or change kind, and only the phase-2 symmetric exports are added

## Git Commit Requirement
The implementer must commit work to git before yielding.
