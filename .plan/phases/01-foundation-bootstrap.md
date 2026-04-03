# Phase 01 Foundation Bootstrap

## Phase Name
Foundation, ABI, and compatibility harness bootstrap

## Implement Phase ID
`impl_safe_foundation`

## Preexisting Inputs
- `original/configure.ac`
- `original/src/libsodium/Makefile.am`
- `original/src/libsodium/include/Makefile.am`
- `original/src/libsodium/include/sodium.h`
- `original/src/libsodium/include/sodium/export.h`
- `original/src/libsodium/include/sodium/core.h`
- `original/src/libsodium/include/sodium/randombytes.h`
- `original/src/libsodium/include/sodium/randombytes_internal_random.h`
- `original/src/libsodium/include/sodium/randombytes_sysrandom.h`
- `original/src/libsodium/include/sodium/runtime.h`
- `original/src/libsodium/include/sodium/utils.h`
- `original/src/libsodium/include/sodium/version.h`
- `original/src/libsodium/include/sodium/crypto_verify_16.h`
- `original/src/libsodium/include/sodium/crypto_verify_32.h`
- `original/src/libsodium/include/sodium/crypto_verify_64.h`
- `original/src/libsodium/include/sodium/*.h`
- `original/src/libsodium/sodium/core.c`
- `original/src/libsodium/randombytes/randombytes.c`
- `original/src/libsodium/randombytes/internal/randombytes_internal_random.c`
- `original/src/libsodium/randombytes/sysrandom/randombytes_sysrandom.c`
- `original/src/libsodium/sodium/runtime.c`
- `original/src/libsodium/sodium/utils.c`
- `original/src/libsodium/sodium/codecs.c`
- `original/src/libsodium/sodium/version.c`
- `original/debian/libsodium23.symbols`
- `original/libsodium.pc.in`
- `original/src/libsodium/.libs/libsodium.so`
- `original/test/default/Makefile.am`
- `original/test/default/*.c`
- `original/test/default/*.o`
- `original/test/default/*.exp`
- `original/test/default/test-suite.log`

## New Outputs
- A buildable Rust package at `safe/` that emits `libsodium.so`, `libsodium.a`, and an `rlib`
- `safe/cabi/libsodium.map` as a synced derivative of `original/debian/libsodium23.symbols`
- `safe/cabi/expected/foundation.symbols`
- `safe/cabi/expected/through_symmetric.symbols`
- `safe/cabi/expected/through_public_key.symbols`
- `safe/cabi/expected/full.symbols`
- `safe/cabi/expected/upstream-kinds.tsv` as a synced derivative of `original/src/libsodium/.libs/libsodium.so`
- Reusable symbol-check, C-test, and object-relink scripts under `safe/tools/`
- Rust ABI/layout tests and Rust-ported foundation tests
- Mirrored install headers under `safe/include/` generated from upstream, not hand-maintained

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
- Generate `safe/include/sodium.h` and `safe/include/sodium/*.h` only through the sync script

## Implementation Details
- Consume the listed upstream headers, source files, symbol inventories, test inventories, test objects, and expected-output files in place. Do not refetch, rediscover, or regenerate those upstream artifacts elsewhere.
- Cover every export from `sodium/core.h`, `sodium/randombytes.h`, `sodium/randombytes_internal_random.h`, `sodium/randombytes_sysrandom.h`, `sodium/runtime.h`, `sodium/utils.h`, `sodium/version.h`, `sodium/crypto_verify_16.h`, `sodium/crypto_verify_32.h`, and `sodium/crypto_verify_64.h`.
- Set `[lib] name = "sodium"` and `crate-type = ["cdylib", "staticlib", "rlib"]` so Cargo emits `libsodium.so` and `libsodium.a`.
- In `build.rs`, pass Linux linker arguments for `-Wl,-soname,libsodium.so.23` and `-Wl,--version-script=<generated map>`.
- Generate `safe/cabi/libsodium.map` directly from `original/debian/libsodium23.symbols`.
- Generate `safe/cabi/expected/foundation.symbols`, `safe/cabi/expected/through_symmetric.symbols`, `safe/cabi/expected/through_public_key.symbols`, and `safe/cabi/expected/full.symbols` once in this phase by intersecting the explicit phase header scopes from the plan with `original/debian/libsodium23.symbols`.
- Generate `safe/cabi/expected/upstream-kinds.tsv` from `readelf --dyn-syms --wide original/src/libsodium/.libs/libsodium.so`.
- Run `safe/tools/sync-upstream-interface.sh` in this phase so `safe/include/`, `safe/cabi/libsodium.map`, and `safe/cabi/expected/*` become synced derivatives. Later phases must consume those derivatives unchanged unless work explicitly bounces back to `impl_safe_foundation`.
- Mirror the public headers from `original/src/libsodium/include/` into `safe/include/`. Do not hand-maintain translated or Rust-generated public headers.
- Export only the foundation symbols in this phase. Do not add placeholder exports for later families to satisfy a future full-ABI count early.
- Implement an FFI helper layer that centralizes raw-pointer validation, optional-null handling, overlap-safe copies, integer bounds checks, `errno` handling where upstream sets it, and panic containment so no Rust panic crosses the C ABI boundary.
- Implement `sodium_init`, `sodium_set_misuse_handler`, `sodium_misuse`, the weak `sodium_runtime_has_*` probes, the public `randombytes_*` API plus `randombytes_internal_implementation` and `randombytes_sysrandom_implementation`, the public `sodium_*` memory and codec helpers, the version exports, and `crypto_verify_16`, `crypto_verify_32`, `crypto_verify_64`, plus their `*_bytes()` accessors.
- Preserve upstream quirks: `sodium_init()` returns `0` on first initialization and `1` later, calls `randombytes_stir()`, keeps later `_pick_best_implementation()` insertion points stable, `randombytes_stir()` stays exported, `randombytes_buf()` skips the vtable call for zero length, `randombytes_close()` does not clear the selected implementation, `randombytes_uniform(upper_bound < 2)` returns `0`, and `sodium_misuse()` aborts after invoking the registered handler.
- Make `randombytes_buf_deterministic()` use nonce `"LibsodiumDRG"`, include a private ChaCha20-IETF keystream helper in phase 1 so deterministic randombytes is already correct, and call `sodium_misuse()` on builds where `SIZE_MAX > 0x4000000000ULL` and `size > 0x4000000000ULL`.
- Keep the runtime probe exports weak and preserve the two public `randombytes_*_implementation` data symbols as data exports, not functions.
- Add ABI/layout tests covering every public concrete or aliased state type exposed by the installed headers: `crypto_aead_aes256gcm_state`, `crypto_auth_hmacsha256_state`, `crypto_auth_hmacsha512_state`, `crypto_auth_hmacsha512256_state`, `crypto_generichash_blake2b_state`, `crypto_generichash_state`, `crypto_onetimeauth_poly1305_state`, `crypto_onetimeauth_state`, `crypto_hash_sha256_state`, `crypto_hash_sha512_state`, `crypto_secretstream_xchacha20poly1305_state`, `crypto_sign_ed25519ph_state`, `crypto_sign_state`, and `randombytes_implementation`.
- Make the symbol checker inspect only the dynamic symbol table and compare both names and kind classification against the generated manifests.
- Make `run-original-c-tests.sh --all` and `relink-original-objects.sh --all` derive the runnable inventory from `original/test/default/Makefile.am`, resolve the active `!EMSCRIPTEN` and `!MINIMAL` additions, and intersect that with the extant `.c` and `.o` files. Do not fabricate a `hash2` runnable test from `hash2.exp`.

## Verification Phases
- `Phase ID`: `check_safe_foundation`
- `Type`: `check`
- `bounce_target`: `impl_safe_foundation`
- `Purpose`: Verify that the Rust crate skeleton builds as `libsodium`, exports the correct foundation ABI surface, preserves the weak and data symbol contract, and passes the low-level upstream C tests plus the initial Rust ABI/layout tests.
- `Commands`:
```bash
cargo build --manifest-path safe/Cargo.toml --release
cargo test --manifest-path safe/Cargo.toml --test abi_layout
cargo test --manifest-path safe/Cargo.toml --test abi_symbols
cargo test --manifest-path safe/Cargo.toml --test ported_foundation
./safe/tools/check-symbols.sh --expected safe/cabi/expected/foundation.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so
./safe/tools/run-original-c-tests.sh sodium_core sodium_utils sodium_utils2 sodium_utils3 sodium_version codecs randombytes verify1
./safe/tools/relink-original-objects.sh sodium_core sodium_utils sodium_utils2 sodium_utils3 sodium_version codecs randombytes verify1
```

## Success Criteria
- `cargo build --manifest-path safe/Cargo.toml --release` produces `safe/target/release/libsodium.so` with SONAME `libsodium.so.23`
- The release artifact matches `safe/cabi/expected/foundation.symbols` exactly, with weak and data classification matching `safe/cabi/expected/upstream-kinds.tsv`
- The listed foundation C tests compile against the synced safe headers and pass
- The same subset of prebuilt upstream `.o` files relinks against the safe library and passes at runtime
- The synced interface artifacts and cumulative expected-export manifests are generated once and are ready for later phases to consume unchanged

## Git Commit Requirement
The implementer must commit work to git before yielding.
