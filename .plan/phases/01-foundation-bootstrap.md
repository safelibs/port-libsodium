# Phase 01 Foundation Bootstrap

## Phase Name
Foundation, ABI, and compatibility harness bootstrap

## Implement Phase ID
`impl_safe_foundation`

## Preexisting Inputs
- Upstream interface and layout authorities: `original/configure.ac`, `original/src/libsodium/Makefile.am`, `original/src/libsodium/include/Makefile.am`, `original/src/libsodium/include/sodium.h`, `original/src/libsodium/include/sodium/export.h`, `original/src/libsodium/include/sodium/core.h`, `original/src/libsodium/include/sodium/randombytes.h`, `original/src/libsodium/include/sodium/randombytes_internal_random.h`, `original/src/libsodium/include/sodium/randombytes_sysrandom.h`, `original/src/libsodium/include/sodium/runtime.h`, `original/src/libsodium/include/sodium/utils.h`, `original/src/libsodium/include/sodium/version.h`, `original/src/libsodium/include/sodium/crypto_verify_16.h`, `original/src/libsodium/include/sodium/crypto_verify_32.h`, and `original/src/libsodium/include/sodium/crypto_verify_64.h`
- Upstream foundation implementations: `original/src/libsodium/sodium/core.c`, `original/src/libsodium/randombytes/randombytes.c`, `original/src/libsodium/randombytes/internal/randombytes_internal_random.c`, `original/src/libsodium/randombytes/sysrandom/randombytes_sysrandom.c`, `original/src/libsodium/sodium/runtime.c`, `original/src/libsodium/sodium/utils.c`, `original/src/libsodium/sodium/codecs.c`, and `original/src/libsodium/sodium/version.c`
- ABI and packaging authorities already present in the workspace: `original/debian/libsodium23.symbols`, `original/libsodium.pc.in`, and `original/src/libsodium/.libs/libsodium.so`
- Existing test inventory artifacts that must be consumed in place: `original/test/default/Makefile.am`, `original/test/default/hash2.exp`, `original/test/default/sodium_core.c`, `original/test/default/sodium_core.exp`, `original/test/default/sodium_core.o`, `original/test/default/sodium_utils.c`, `original/test/default/sodium_utils.exp`, `original/test/default/sodium_utils.o`, `original/test/default/sodium_utils2.c`, `original/test/default/sodium_utils2.exp`, `original/test/default/sodium_utils2.o`, `original/test/default/sodium_utils3.c`, `original/test/default/sodium_utils3.exp`, `original/test/default/sodium_utils3.o`, `original/test/default/sodium_version.c`, `original/test/default/sodium_version.exp`, `original/test/default/sodium_version.o`, `original/test/default/codecs.c`, `original/test/default/codecs.exp`, `original/test/default/codecs.o`, `original/test/default/randombytes.c`, `original/test/default/randombytes.exp`, `original/test/default/randombytes.o`, `original/test/default/verify1.c`, `original/test/default/verify1.exp`, and `original/test/default/verify1.o`

## New Outputs
- A buildable Rust package at `safe/` with a `cdylib` and `staticlib` named `libsodium`
- A linker version script derived from the upstream exported-name list
- A minimal C shim for weak runtime exports and any other unavoidable ABI-only symbol handling
- Cumulative expected-export manifests under `safe/cabi/expected/` for the foundation, through-symmetric, through-public-key, and full-ABI checkpoints, plus an upstream symbol-kind manifest derived from the current upstream dynamic symbol table
- Reusable compatibility scripts for symbol checking, legacy C test compilation, and legacy object relinking
- Rust ABI/layout tests and foundation Rust ports of the upstream test programs
- Synced public headers under `safe/include/`

## File Changes
- Create `safe/Cargo.toml`
- Create `safe/build.rs`
- Create `safe/src/lib.rs`
- Create `safe/src/abi/mod.rs`
- Create `safe/src/abi/types.rs`
- Create `safe/src/ffi/mod.rs`
- Create `safe/src/ffi/helpers.rs`
- Create `safe/src/foundation/core.rs`
- Create `safe/src/foundation/runtime.rs`
- Create `safe/src/foundation/randombytes.rs`
- Create `safe/src/foundation/utils.rs`
- Create `safe/src/foundation/codecs.rs`
- Create `safe/src/foundation/version.rs`
- Create `safe/src/foundation/verify.rs`
- Create `safe/cabi/weak_runtime.c`
- Create `safe/cabi/libsodium.map`
- Create `safe/cabi/expected/foundation.symbols`
- Create `safe/cabi/expected/through_symmetric.symbols`
- Create `safe/cabi/expected/through_public_key.symbols`
- Create `safe/cabi/expected/full.symbols`
- Create `safe/cabi/expected/upstream-kinds.tsv`
- Create `safe/tools/sync-upstream-interface.sh`
- Create `safe/tools/check-symbols.sh`
- Create `safe/tools/run-original-c-tests.sh`
- Create `safe/tools/relink-original-objects.sh`
- Create `safe/tests/abi_layout.rs`
- Create `safe/tests/abi_symbols.rs`
- Create `safe/tests/ported_foundation.rs`
- Create mirrored install headers under `safe/include/` only via the sync script, not by hand

## Implementation Details
- Header/API coverage for this phase is every export from `sodium/core.h`, `sodium/randombytes.h`, `sodium/randombytes_internal_random.h`, `sodium/randombytes_sysrandom.h`, `sodium/runtime.h`, `sodium/utils.h`, `sodium/version.h`, `sodium/crypto_verify_16.h`, `sodium/crypto_verify_32.h`, and `sodium/crypto_verify_64.h`.
- Treat `original/src/libsodium/include/Makefile.am` as the authoritative installed-header inventory when mirroring `safe/include/`, and treat `original/src/libsodium/Makefile.am` as the authoritative module and wrapper-layering inventory when laying out `safe/src/lib.rs` and the family modules that later phases extend.
- Set `[lib] name = "sodium"` and `crate-type = ["cdylib", "staticlib", "rlib"]` so Cargo emits `libsodium.so` and `libsodium.a`.
- In `build.rs`, pass Linux linker args for `-Wl,-soname,libsodium.so.23` and `-Wl,--version-script=<generated map>` so the resulting shared object preserves the upstream SONAME and export set.
- `safe/build.rs` must also validate that the phase-1-synced interface artifacts already exist before linking: `safe/include/sodium.h`, `safe/cabi/libsodium.map`, `safe/cabi/expected/foundation.symbols`, `safe/cabi/expected/through_symmetric.symbols`, `safe/cabi/expected/through_public_key.symbols`, `safe/cabi/expected/full.symbols`, and `safe/cabi/expected/upstream-kinds.tsv`. It must fail fast instead of silently regenerating or bypassing those synced derivatives.
- Generate `safe/cabi/libsodium.map` directly from `original/debian/libsodium23.symbols` so the linker allowlist contains all legal public names.
- Generate `safe/cabi/expected/foundation.symbols`, `safe/cabi/expected/through_symmetric.symbols`, `safe/cabi/expected/through_public_key.symbols`, and `safe/cabi/expected/full.symbols` as synced derivatives of the explicit phase header scopes in `.plan/plan.md` intersected with `original/debian/libsodium23.symbols`. Generate `safe/cabi/expected/upstream-kinds.tsv` from `readelf --dyn-syms --wide` on `original/src/libsodium/.libs/libsodium.so`. The Debian symbols file is the source of truth for names and versions; the upstream shared object is the source of truth for weak/global and function/object classification.
- Run `safe/tools/sync-upstream-interface.sh` in phase 1 to generate `safe/include/`, `safe/cabi/libsodium.map`, and the `safe/cabi/expected/*` manifests. Later phases must consume those generated artifacts unchanged unless work bounces back to `impl_safe_foundation`.
- Phase 1 must export only the symbols covered by its own header/API scope. Do not create placeholder exports for later families in phase 1. Later phases must add symbols monotonically until phase 4 reaches the full 609-name set.
- Mirror the public headers from `original/src/libsodium/include/` into `safe/include/`. Do not hand-maintain translated Rust-generated headers; source compatibility is safest if the installed headers remain the upstream headers.
- Implement an FFI helper layer that centralizes raw pointer validation, optional-null handling, overlap-aware copies, integer bound checks against `*_MESSAGEBYTES_MAX`, `errno` setting where upstream does so, and panic containment so no Rust panic crosses the C ABI boundary.
- Implement the foundation exports and symbols:
- `sodium_init`, `sodium_set_misuse_handler`, and `sodium_misuse`
- `sodium_runtime_has_neon`, `sodium_runtime_has_sse2`, `sodium_runtime_has_sse3`, `sodium_runtime_has_ssse3`, `sodium_runtime_has_sse41`, `sodium_runtime_has_avx`, `sodium_runtime_has_avx2`, `sodium_runtime_has_avx512f`, `sodium_runtime_has_pclmul`, `sodium_runtime_has_aesni`, and `sodium_runtime_has_rdrand`
- `randombytes_set_implementation`, `randombytes_implementation_name`, `randombytes_random`, `randombytes_uniform`, `randombytes_buf`, `randombytes_buf_deterministic`, `randombytes_seedbytes`, `randombytes_stir`, `randombytes_close`, and `randombytes`
- `randombytes_internal_implementation` and `randombytes_sysrandom_implementation`
- `sodium_memzero`, `sodium_stackzero`, `sodium_memcmp`, `sodium_compare`, `sodium_is_zero`, `sodium_increment`, `sodium_add`, `sodium_sub`, `sodium_malloc`, `sodium_allocarray`, `sodium_free`, `sodium_mlock`, `sodium_munlock`, `sodium_mprotect_noaccess`, `sodium_mprotect_readonly`, `sodium_mprotect_readwrite`, `sodium_pad`, and `sodium_unpad`
- `sodium_bin2hex`, `sodium_hex2bin`, `sodium_base64_encoded_len`, `sodium_bin2base64`, and `sodium_base642bin`
- `sodium_version_string`, `sodium_library_version_major`, `sodium_library_version_minor`, and `sodium_library_minimal`
- `crypto_verify_16`, `crypto_verify_32`, and `crypto_verify_64`, plus their `*_bytes()` accessors
- Preserve exact upstream quirks:
- `sodium_init()` returns `0` on first initialization and `1` on later calls
- `sodium_init()` must call `randombytes_stir()` during initialization just as `original/src/libsodium/sodium/core.c` does
- `randombytes_init_if_needed()` semantics from `original/src/libsodium/randombytes/randombytes.c` must be preserved, including the recursive path where the default implementation is installed and then `randombytes_stir()` is invoked
- `randombytes_stir()` must remain exported and must call the implementation vtable `stir` callback when present; `original/test/default/randombytes.c` explicitly exercises it
- `randombytes_buf()` must skip the vtable call when `size == 0`
- `randombytes_close()` must forward to the vtable `close` callback without clearing the chosen implementation pointer, matching upstream behavior
- `randombytes_buf_deterministic()` must use the fixed nonce `"LibsodiumDRG"`
- Because `randombytes_buf_deterministic()` is part of the phase-1 public surface and the upstream implementation depends on ChaCha20-IETF, phase 1 must include a private, non-exported ChaCha20-IETF keystream helper that phase 2 later reuses for the public `crypto_stream_chacha20_*` exports. Do not defer deterministic-randombytes correctness to the symmetric phase.
- On builds where `SIZE_MAX > 0x4000000000ULL`, `randombytes_buf_deterministic()` must call `sodium_misuse()` when `size > 0x4000000000ULL`, matching `original/src/libsodium/randombytes/randombytes.c` and the later `original/test/default/misuse.c` coverage.
- `randombytes_uniform(upper_bound < 2)` returns `0`
- `sodium_misuse()` aborts after invoking the registered handler
- `sodium_runtime_has_*` must remain weak exports
- Structure `sodium_init()` and its internal selector registry so later phases can add the family-specific `_pick_best_implementation()` hooks in the same order as `original/src/libsodium/sodium/core.c` without changing the public ABI or return-value contract established in phase 1.
- Add ABI layout tests that assert the Rust FFI types match current C ABI sizes and alignments, and where applicable the exported `*_statebytes()` accessors, for every public concrete or aliased state type exposed by the installed headers: `crypto_aead_aes256gcm_state`, `crypto_auth_hmacsha256_state`, `crypto_auth_hmacsha512_state`, `crypto_auth_hmacsha512256_state`, `crypto_generichash_blake2b_state`, `crypto_generichash_state`, `crypto_onetimeauth_poly1305_state`, `crypto_onetimeauth_state`, `crypto_hash_sha256_state`, `crypto_hash_sha512_state`, `crypto_secretstream_xchacha20poly1305_state`, `crypto_sign_ed25519ph_state`, `crypto_sign_state`, and `randombytes_implementation`. Do not stop at a representative subset.
- Make the symbol checker compare the current release artifact against the selected expected-name manifest plus `safe/cabi/expected/upstream-kinds.tsv`, using only the dynamic symbol table so later phases cannot silently drift from the upstream ABI contract.
- Make `run-original-c-tests.sh` and `relink-original-objects.sh` derive the `--all` inventory from `original/test/default/Makefile.am`, resolve the active `!EMSCRIPTEN` and `!MINIMAL` additions, and then intersect that inventory with the actual `.c` and `.o` files. Do not use handwritten ad hoc lists, and do not synthesize `hash2` just because `original/test/default/hash2.exp` exists.

## Verification Phases
### `check_safe_foundation`
Type: `check`

Bounce Target: `impl_safe_foundation`

Purpose: Verify that the Rust crate skeleton builds as `libsodium`, exports the correct foundation ABI surface, preserves the weak/data symbol contract, and passes the low-level upstream C tests plus the initial Rust ABI/layout tests.

Commands:
- `cargo build --manifest-path safe/Cargo.toml --release`
- `cargo test --manifest-path safe/Cargo.toml --test abi_layout`
- `cargo test --manifest-path safe/Cargo.toml --test abi_symbols`
- `cargo test --manifest-path safe/Cargo.toml --test ported_foundation`
- `./safe/tools/check-symbols.sh --expected safe/cabi/expected/foundation.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so`
- `./safe/tools/run-original-c-tests.sh sodium_core sodium_utils sodium_utils2 sodium_utils3 sodium_version codecs randombytes verify1`
- `./safe/tools/relink-original-objects.sh sodium_core sodium_utils sodium_utils2 sodium_utils3 sodium_version codecs randombytes verify1`

## Success Criteria
- `readelf -d safe/target/release/libsodium.so` reports `SONAME: libsodium.so.23`.
- The phase-1 export set matches `safe/cabi/expected/foundation.symbols` exactly, with weak/runtime and data-symbol classification matching `safe/cabi/expected/upstream-kinds.tsv`.
- `safe/build.rs` fails fast if the synced headers, version script, or expected-symbol manifests are missing, so later builds cannot proceed on stale or unsynced interface artifacts.
- The foundation C tests compile against the safe headers and library, and the same subset of prebuilt upstream `.o` objects relink against the safe library and pass at runtime.

## Git Commit Requirement
The implementer must commit phase 1 work to git before yielding.
