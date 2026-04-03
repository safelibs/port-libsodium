# Phase 02 Symmetric Secret Key APIs

## Phase Name
Symmetric primitives and secret-key APIs

## Implement Phase ID
`impl_safe_symmetric`

## Preexisting Inputs
- Phase 1 Rust outputs expected to exist: `safe/Cargo.toml`, `safe/build.rs`, `safe/src/lib.rs`, `safe/src/abi/mod.rs`, `safe/src/abi/types.rs`, `safe/src/ffi/mod.rs`, `safe/src/ffi/helpers.rs`, `safe/src/foundation/core.rs`, `safe/src/foundation/runtime.rs`, `safe/src/foundation/randombytes.rs`, `safe/src/foundation/utils.rs`, `safe/src/foundation/codecs.rs`, `safe/src/foundation/version.rs`, and `safe/src/foundation/verify.rs`
- Phase 1 synced and verifier artifacts expected to exist: `safe/cabi/weak_runtime.c`, `safe/cabi/libsodium.map`, `safe/cabi/expected/foundation.symbols`, `safe/cabi/expected/through_symmetric.symbols`, `safe/cabi/expected/through_public_key.symbols`, `safe/cabi/expected/full.symbols`, `safe/cabi/expected/upstream-kinds.tsv`, `safe/tools/sync-upstream-interface.sh`, `safe/tools/check-symbols.sh`, `safe/tools/run-original-c-tests.sh`, `safe/tools/relink-original-objects.sh`, `safe/tests/abi_layout.rs`, `safe/tests/abi_symbols.rs`, `safe/tests/ported_foundation.rs`, and `safe/include/sodium.h`
- Upstream symmetric header and module authorities: `original/src/libsodium/Makefile.am`, `original/test/default/Makefile.am`, `original/test/default/hash2.exp`, `original/src/libsodium/include/sodium/crypto_hash.h`, `original/src/libsodium/include/sodium/crypto_hash_sha256.h`, `original/src/libsodium/include/sodium/crypto_hash_sha512.h`, `original/src/libsodium/include/sodium/crypto_generichash.h`, `original/src/libsodium/include/sodium/crypto_generichash_blake2b.h`, `original/src/libsodium/include/sodium/crypto_auth.h`, `original/src/libsodium/include/sodium/crypto_auth_hmacsha256.h`, `original/src/libsodium/include/sodium/crypto_auth_hmacsha512.h`, `original/src/libsodium/include/sodium/crypto_auth_hmacsha512256.h`, `original/src/libsodium/include/sodium/crypto_onetimeauth.h`, `original/src/libsodium/include/sodium/crypto_onetimeauth_poly1305.h`, `original/src/libsodium/include/sodium/crypto_core_hsalsa20.h`, `original/src/libsodium/include/sodium/crypto_core_hchacha20.h`, `original/src/libsodium/include/sodium/crypto_core_salsa20.h`, `original/src/libsodium/include/sodium/crypto_core_salsa2012.h`, `original/src/libsodium/include/sodium/crypto_core_salsa208.h`, `original/src/libsodium/include/sodium/crypto_stream.h`, `original/src/libsodium/include/sodium/crypto_stream_chacha20.h`, `original/src/libsodium/include/sodium/crypto_stream_salsa20.h`, `original/src/libsodium/include/sodium/crypto_stream_salsa2012.h`, `original/src/libsodium/include/sodium/crypto_stream_salsa208.h`, `original/src/libsodium/include/sodium/crypto_stream_xsalsa20.h`, `original/src/libsodium/include/sodium/crypto_stream_xchacha20.h`, `original/src/libsodium/include/sodium/crypto_secretbox.h`, `original/src/libsodium/include/sodium/crypto_secretbox_xsalsa20poly1305.h`, `original/src/libsodium/include/sodium/crypto_secretbox_xchacha20poly1305.h`, `original/src/libsodium/include/sodium/crypto_secretstream_xchacha20poly1305.h`, `original/src/libsodium/include/sodium/crypto_shorthash.h`, `original/src/libsodium/include/sodium/crypto_shorthash_siphash24.h`, `original/src/libsodium/include/sodium/crypto_aead_chacha20poly1305.h`, and `original/src/libsodium/include/sodium/crypto_aead_xchacha20poly1305.h`
- Existing symmetric test artifacts that must be consumed in place: `original/test/default/hash.c`, `original/test/default/hash.exp`, `original/test/default/hash.o`, `original/test/default/hash3.c`, `original/test/default/hash3.exp`, `original/test/default/hash3.o`, `original/test/default/generichash.c`, `original/test/default/generichash.exp`, `original/test/default/generichash.o`, `original/test/default/generichash2.c`, `original/test/default/generichash2.exp`, `original/test/default/generichash2.o`, `original/test/default/generichash3.c`, `original/test/default/generichash3.exp`, `original/test/default/generichash3.o`, `original/test/default/auth.c`, `original/test/default/auth.exp`, `original/test/default/auth.o`, `original/test/default/auth2.c`, `original/test/default/auth2.exp`, `original/test/default/auth2.o`, `original/test/default/auth3.c`, `original/test/default/auth3.exp`, `original/test/default/auth3.o`, `original/test/default/auth5.c`, `original/test/default/auth5.exp`, `original/test/default/auth5.o`, `original/test/default/auth6.c`, `original/test/default/auth6.exp`, `original/test/default/auth6.o`, `original/test/default/auth7.c`, `original/test/default/auth7.exp`, `original/test/default/auth7.o`, `original/test/default/onetimeauth.c`, `original/test/default/onetimeauth.exp`, `original/test/default/onetimeauth.o`, `original/test/default/onetimeauth2.c`, `original/test/default/onetimeauth2.exp`, `original/test/default/onetimeauth2.o`, `original/test/default/onetimeauth7.c`, `original/test/default/onetimeauth7.exp`, `original/test/default/onetimeauth7.o`, `original/test/default/chacha20.c`, `original/test/default/chacha20.exp`, `original/test/default/chacha20.o`, `original/test/default/stream.c`, `original/test/default/stream.exp`, `original/test/default/stream.o`, `original/test/default/stream2.c`, `original/test/default/stream2.exp`, `original/test/default/stream2.o`, `original/test/default/stream3.c`, `original/test/default/stream3.exp`, `original/test/default/stream3.o`, `original/test/default/stream4.c`, `original/test/default/stream4.exp`, `original/test/default/stream4.o`, `original/test/default/core1.c`, `original/test/default/core1.exp`, `original/test/default/core1.o`, `original/test/default/core2.c`, `original/test/default/core2.exp`, `original/test/default/core2.o`, `original/test/default/core3.c`, `original/test/default/core3.exp`, `original/test/default/core3.o`, `original/test/default/core4.c`, `original/test/default/core4.exp`, `original/test/default/core4.o`, `original/test/default/core5.c`, `original/test/default/core5.exp`, `original/test/default/core5.o`, `original/test/default/core6.c`, `original/test/default/core6.exp`, `original/test/default/core6.o`, `original/test/default/shorthash.c`, `original/test/default/shorthash.exp`, `original/test/default/shorthash.o`, `original/test/default/siphashx24.c`, `original/test/default/siphashx24.exp`, `original/test/default/siphashx24.o`, `original/test/default/secretbox.c`, `original/test/default/secretbox.exp`, `original/test/default/secretbox.o`, `original/test/default/secretbox2.c`, `original/test/default/secretbox2.exp`, `original/test/default/secretbox2.o`, `original/test/default/secretbox7.c`, `original/test/default/secretbox7.exp`, `original/test/default/secretbox7.o`, `original/test/default/secretbox8.c`, `original/test/default/secretbox8.exp`, `original/test/default/secretbox8.o`, `original/test/default/secretbox_easy.c`, `original/test/default/secretbox_easy.exp`, `original/test/default/secretbox_easy.o`, `original/test/default/secretbox_easy2.c`, `original/test/default/secretbox_easy2.exp`, `original/test/default/secretbox_easy2.o`, `original/test/default/secretstream.c`, `original/test/default/secretstream.exp`, `original/test/default/secretstream.o`, `original/test/default/aead_chacha20poly1305.c`, `original/test/default/aead_chacha20poly1305.exp`, `original/test/default/aead_chacha20poly1305.o`, `original/test/default/aead_chacha20poly13052.c`, `original/test/default/aead_chacha20poly13052.exp`, `original/test/default/aead_chacha20poly13052.o`, `original/test/default/aead_xchacha20poly1305.c`, `original/test/default/aead_xchacha20poly1305.exp`, `original/test/default/aead_xchacha20poly1305.o`, `original/test/default/metamorphic.c`, `original/test/default/metamorphic.exp`, `original/test/default/metamorphic.o`, and `original/test/default/xchacha20.c`

## New Outputs
- Rust implementations for the symmetric primitive families and their high-level wrappers
- Ported Rust tests for the same symmetric families, including the stream and secretbox portions of the upstream XChaCha test vectors
- Preserved FFI-visible state layouts for hash, HMAC, BLAKE2b state, Poly1305 state, and secretstream state

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
- Header/API coverage for this phase is every export from `sodium/crypto_hash*.h`, `sodium/crypto_generichash*.h`, `sodium/crypto_auth*.h`, `sodium/crypto_onetimeauth*.h`, `sodium/crypto_core_h*` and `sodium/crypto_core_salsa*`, `sodium/crypto_stream*.h`, `sodium/crypto_secretbox*.h`, `sodium/crypto_secretstream_xchacha20poly1305.h`, `sodium/crypto_shorthash*.h`, `sodium/crypto_aead_chacha20poly1305.h`, and `sodium/crypto_aead_xchacha20poly1305.h`.
- Consume the phase-1-synced interface artifacts in place: `safe/include/sodium.h`, `safe/cabi/libsodium.map`, and the `safe/cabi/expected/*` manifests remain synced derivatives created in phase 1 and must not be rediscovered or regenerated here unless work bounces back to `impl_safe_foundation`.
- Continue mirroring the wrapper layering in `original/src/libsodium/Makefile.am` so the Rust module layout follows the same family split as upstream.
- Implement SHA-256 and SHA-512 so the exported `crypto_hash_sha256_state` and `crypto_hash_sha512_state` remain exact C layouts rather than opaque heap handles.
- Implement BLAKE2b-backed `crypto_generichash_*` and `crypto_generichash_blake2b_*`, including exact `statebytes()` behavior and the aligned 384-byte public state object.
- Implement `crypto_auth`, `crypto_auth_hmacsha256`, `crypto_auth_hmacsha512`, and `crypto_auth_hmacsha512256`, including one-shot, init/update/final, `*_statebytes()`, `*_primitive()`, and keygen exports.
- Implement Poly1305-backed `crypto_onetimeauth_*` and `crypto_onetimeauth_poly1305_*`, preserving the 256-byte aligned public state layout.
- Implement `crypto_core_salsa20`, `crypto_core_salsa2012`, `crypto_core_salsa208`, `crypto_core_hsalsa20`, and `crypto_core_hchacha20`.
- Implement stream ciphers and wrappers:
- `crypto_stream`, `crypto_stream_xor`, `crypto_stream_keygen`
- `crypto_stream_chacha20*`, including IETF and `_xor_ic` variants
- `crypto_stream_salsa20*`, `crypto_stream_salsa2012*`, `crypto_stream_salsa208*`, `crypto_stream_xsalsa20*`, and `crypto_stream_xchacha20*`
- Implement secret-key composite APIs:
- `crypto_secretbox_*`
- `crypto_secretbox_xsalsa20poly1305_*`
- `crypto_secretbox_xchacha20poly1305_*`
- `crypto_secretstream_xchacha20poly1305_*`
- `crypto_aead_chacha20poly1305_*`
- `crypto_aead_chacha20poly1305_ietf_*`
- `crypto_aead_xchacha20poly1305_ietf_*`
- Preserve overlap behavior in APIs such as `crypto_secretbox_detached()`, `crypto_secretbox_open_detached()`, `crypto_secretbox_xchacha20poly1305_detached()`, and `crypto_secretbox_xchacha20poly1305_open_detached()`, which use `memmove`-style overlap tolerance upstream.
- Preserve exact accessors and string-returning primitive selectors from the headers, not normalized Rust names.
- Port the symmetric parts of `original/test/default/xchacha20.c` into `safe/tests/ported_symmetric.rs`: `tv_stream_xchacha20()` at lines 89-190 and `tv_secretbox_xchacha20poly1305()` at lines 199-297. Do not run the original C `xchacha20` binary yet; it also contains `tv_box_xchacha20poly1305()` at lines 299-426, which depends on phase-3 box APIs.
- Run the upstream C `metamorphic` test in this phase because it is symmetric-only and exercises chunked versus one-shot behavior for generichash, onetimeauth, and HMAC APIs.

## Verification Phases
### `check_safe_symmetric`
Type: `check`

Bounce Target: `impl_safe_symmetric`

Purpose: Verify the symmetric primitive families, secret-key composite APIs, and the symmetric-only metamorphic tests against both the original C tests and the new Rust-ported tests.

Commands:
- `cargo build --manifest-path safe/Cargo.toml --release`
- `cargo test --manifest-path safe/Cargo.toml --test ported_symmetric`
- `./safe/tools/run-original-c-tests.sh hash hash3 generichash generichash2 generichash3 auth auth2 auth3 auth5 auth6 auth7 onetimeauth onetimeauth2 onetimeauth7 chacha20 stream stream2 stream3 stream4 core1 core2 core3 core4 core5 core6 shorthash siphashx24 secretbox secretbox2 secretbox7 secretbox8 secretbox_easy secretbox_easy2 secretstream aead_chacha20poly1305 aead_chacha20poly13052 aead_xchacha20poly1305 metamorphic`
- `./safe/tools/relink-original-objects.sh hash hash3 generichash generichash2 generichash3 auth auth2 auth3 auth5 auth6 auth7 onetimeauth onetimeauth2 onetimeauth7 chacha20 stream stream2 stream3 stream4 core1 core2 core3 core4 core5 core6 shorthash siphashx24 secretbox secretbox2 secretbox7 secretbox8 secretbox_easy secretbox_easy2 secretstream aead_chacha20poly1305 aead_chacha20poly13052 aead_xchacha20poly1305 metamorphic`
- `./safe/tools/check-symbols.sh --expected safe/cabi/expected/through_symmetric.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so`

## Success Criteria
- All symmetric-family C tests listed above compile against the safe library and pass, and the same set of upstream object files relink against the safe library and pass.
- The Rust `ported_symmetric` test directly covers the XChaCha stream and XChaCha secretbox vectors from `original/test/default/xchacha20.c`.
- The release artifact matches `safe/cabi/expected/through_symmetric.symbols` exactly. No phase-1 exports disappear or change kind, and the only additions relative to phase 1 are the phase-2 symmetric-family names captured in that manifest.

## Git Commit Requirement
The implementer must commit phase 2 work to git before yielding.
