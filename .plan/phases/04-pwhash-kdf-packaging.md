# Phase 04 Pwhash Kdf Packaging

## Phase Name
Password hashing, KDFs, AES-GCM, and Debian package build

## Implement Phase ID
`impl_safe_pwhash_packaging`

## Preexisting Inputs
- Phase 1 outputs expected to exist: `safe/Cargo.toml`, `safe/build.rs`, `safe/src/lib.rs`, `safe/src/abi/mod.rs`, `safe/src/abi/types.rs`, `safe/src/ffi/mod.rs`, `safe/src/ffi/helpers.rs`, `safe/src/foundation/core.rs`, `safe/src/foundation/runtime.rs`, `safe/src/foundation/randombytes.rs`, `safe/src/foundation/utils.rs`, `safe/src/foundation/codecs.rs`, `safe/src/foundation/version.rs`, `safe/src/foundation/verify.rs`, `safe/cabi/weak_runtime.c`, `safe/cabi/libsodium.map`, `safe/cabi/expected/foundation.symbols`, `safe/cabi/expected/through_symmetric.symbols`, `safe/cabi/expected/through_public_key.symbols`, `safe/cabi/expected/full.symbols`, `safe/cabi/expected/upstream-kinds.tsv`, `safe/tools/sync-upstream-interface.sh`, `safe/tools/check-symbols.sh`, `safe/tools/run-original-c-tests.sh`, `safe/tools/relink-original-objects.sh`, `safe/tests/abi_layout.rs`, `safe/tests/abi_symbols.rs`, `safe/tests/ported_foundation.rs`, and `safe/include/sodium.h`
- Phase 2 outputs expected to exist: `safe/src/hash/mod.rs`, `safe/src/hash/sha256.rs`, `safe/src/hash/sha512.rs`, `safe/src/generichash/mod.rs`, `safe/src/auth/mod.rs`, `safe/src/auth/hmacsha256.rs`, `safe/src/auth/hmacsha512.rs`, `safe/src/auth/hmacsha512256.rs`, `safe/src/onetimeauth/poly1305.rs`, `safe/src/stream/chacha20.rs`, `safe/src/stream/salsa20.rs`, `safe/src/stream/salsa2012.rs`, `safe/src/stream/salsa208.rs`, `safe/src/stream/xsalsa20.rs`, `safe/src/stream/xchacha20.rs`, `safe/src/core/hsalsa20.rs`, `safe/src/core/hchacha20.rs`, `safe/src/core/salsa.rs`, `safe/src/secretbox/mod.rs`, `safe/src/secretstream.rs`, `safe/src/shorthash.rs`, `safe/src/aead/chacha20poly1305.rs`, `safe/src/aead/xchacha20poly1305.rs`, and `safe/tests/ported_symmetric.rs`
- Phase 3 outputs expected to exist: `safe/src/scalarmult/mod.rs`, `safe/src/scalarmult/curve25519.rs`, `safe/src/scalarmult/ed25519.rs`, `safe/src/scalarmult/ristretto255.rs`, `safe/src/core/ed25519.rs`, `safe/src/core/ristretto255.rs`, `safe/src/sign/mod.rs`, `safe/src/sign/ed25519.rs`, `safe/src/sign/legacy_edwards25519sha512batch.rs`, `safe/src/box_api/mod.rs`, `safe/src/box_api/curve25519xsalsa20poly1305.rs`, `safe/src/box_api/curve25519xchacha20poly1305.rs`, `safe/src/kx.rs`, `safe/tests/ported_public_key.rs`, and `safe/tests/cve_2025_69277.rs`
- Upstream pwhash, KDF, and packaging authorities: `original/src/libsodium/include/sodium/crypto_pwhash.h`, `original/src/libsodium/include/sodium/crypto_pwhash_argon2i.h`, `original/src/libsodium/include/sodium/crypto_pwhash_argon2id.h`, `original/src/libsodium/include/sodium/crypto_pwhash_scryptsalsa208sha256.h`, `original/src/libsodium/include/sodium/crypto_kdf.h`, `original/src/libsodium/include/sodium/crypto_kdf_blake2b.h`, `original/src/libsodium/include/sodium/crypto_aead_aes256gcm.h`, `original/src/libsodium/crypto_pwhash/`, `original/src/libsodium/crypto_kdf/`, `original/src/libsodium/crypto_aead/aes256gcm/`, `original/debian/control`, `original/debian/rules`, `original/debian/libsodium23.install`, `original/debian/libsodium-dev.install`, `original/debian/libsodium23.symbols`, `original/libsodium.pc.in`, `original/src/libsodium/.libs/libsodium.so`, `original/test/default/Makefile.am`, and `original/test/default/hash2.exp`
- Existing pwhash and keygen test artifacts that must be consumed in place: `original/test/default/aead_aes256gcm.c`, `original/test/default/aead_aes256gcm.exp`, `original/test/default/aead_aes256gcm.o`, `original/test/default/aead_aes256gcm2.c`, `original/test/default/aead_aes256gcm2.exp`, `original/test/default/aead_aes256gcm2.o`, `original/test/default/kdf.c`, `original/test/default/kdf.exp`, `original/test/default/kdf.o`, `original/test/default/pwhash_argon2i.c`, `original/test/default/pwhash_argon2i.exp`, `original/test/default/pwhash_argon2i.o`, `original/test/default/pwhash_argon2id.c`, `original/test/default/pwhash_argon2id.exp`, `original/test/default/pwhash_argon2id.o`, `original/test/default/pwhash_scrypt.c`, `original/test/default/pwhash_scrypt.exp`, `original/test/default/pwhash_scrypt.o`, `original/test/default/pwhash_scrypt_ll.c`, `original/test/default/pwhash_scrypt_ll.exp`, `original/test/default/pwhash_scrypt_ll.o`, `original/test/default/keygen.c`, `original/test/default/keygen.exp`, and `original/test/default/keygen.o`

## New Outputs
- Rust password-hashing and KDF implementations
- A packageable shared library, static library, headers, and pkg-config file
- Debian packaging under `safe/debian/` that produces `libsodium23` and `libsodium-dev`
- Rust tests covering pwhash/KDF behavior and AES-GCM edge cases

## File Changes
- Create `safe/src/pwhash/mod.rs`
- Create `safe/src/pwhash/argon2.rs`
- Create `safe/src/pwhash/scrypt.rs`
- Create `safe/src/kdf.rs`
- Create `safe/src/aead/aes256gcm.rs`
- Create `safe/tests/ported_pwhash.rs`
- Create `safe/debian/control`
- Create `safe/debian/rules`
- Create `safe/debian/changelog`
- Create `safe/debian/source/format`
- Create `safe/debian/libsodium23.install`
- Create `safe/debian/libsodium-dev.install`
- Create `safe/debian/libsodium23.symbols` as a synced derivative of the upstream symbol file
- Create `safe/packaging/libsodium.pc.in`
- Create `safe/tools/build-deb.sh`

## Implementation Details
- Header/API coverage for this phase is every export from `sodium/crypto_pwhash*.h`, `sodium/crypto_kdf*.h`, and `sodium/crypto_aead_aes256gcm.h`.
- Preserve the consume-existing-artifacts contract from earlier phases: continue using the phase-1-synced symbol manifests and headers in place instead of regenerating them here.
- Implement the `crypto_pwhash_argon2i_*`, `crypto_pwhash_argon2id_*`, `crypto_pwhash_*`, and `crypto_pwhash_scryptsalsa208sha256_*` families, including raw KDF APIs, encoded password string generation and verification, `*_needs_rehash`, algorithm selectors, and opslimit/memlimit accessors.
- Preserve the exact upstream string and selector behavior, including the non-obvious `crypto_pwhash_primitive()` constant from `original/src/libsodium/include/sodium/crypto_pwhash.h`, which still returns `"argon2i"` even though the default algorithm is Argon2id.
- Implement `crypto_kdf_*` and `crypto_kdf_blake2b_*` using the same subkey derivation rules and context-size checks as upstream.
- Implement `crypto_aead_aes256gcm_*`, including the 512-byte precomputation state with 16-byte alignment. Preserve upstream semantics for `crypto_aead_aes256gcm_is_available()` and the `ENOSYS` failure path when the algorithm is unavailable on the current CPU, unless a deliberate compatibility review proves that widening availability is safe for all callers.
- Treat `original/test/default/keygen.c` as a cross-family gate for phases 2 through 4. It first becomes satisfiable here because it spans `crypto_auth_*`, `crypto_generichash_*`, `crypto_kdf_*`, `crypto_onetimeauth_*`, `crypto_aead_*`, `crypto_secretbox_*`, `crypto_secretstream_*`, `crypto_shorthash_*`, and `crypto_stream_*` keygen helpers.
- `safe/debian/libsodium23.symbols` must remain a synced derivative of `original/debian/libsodium23.symbols`, and `safe/packaging/libsodium.pc.in` must remain a synced derivative of `original/libsodium.pc.in`. Do not hand-maintain divergent exported-name, `-lsodium`, include-path, or private-lib contracts.
- Package build requirements:
- the `.deb` output must install `libsodium.so.23.3.0`, a `libsodium.so.23` symlink, a development `libsodium.so` symlink, `libsodium.a`, all public headers, and `pkgconfig/libsodium.pc`
- the binary package names must remain `libsodium23` and `libsodium-dev`
- `safe/debian/changelog` must use version `1.0.18-1ubuntu0.24.04.1+safelibs1` so the locally built packages sort above the archive Ubuntu package and can be installed with `dpkg -i` as an in-place upgrade during safe-mode harness runs
- the multiarch install paths must match the current Debian packaging contract from `original/debian/libsodium23.install` and `original/debian/libsodium-dev.install`
- keep the exported-name list sourced from `original/debian/libsodium23.symbols` and the symbol binding/type checks sourced from `original/src/libsodium/.libs/libsodium.so`
- The Debian rules file should use `cargo build --release` plus explicit install steps; do not depend on a custom external packaging workflow.
- `safe/tools/build-deb.sh` must delete any preexisting repo-root `libsodium23_*.deb`, `libsodium-dev_*.deb`, `libsodium-dbgsym_*.deb`, `*.buildinfo`, and `*.changes` outputs before invoking `dpkg-buildpackage`, so the verifier inspects artifacts produced by the current phase rather than stale package files.
- Prefer not to introduce new shared-library `NEEDED` dependencies beyond the upstream `libc.so.6` unless they are unavoidable and explicitly handled by the produced Debian package metadata.

## Verification Phases
### `check_safe_pwhash_packaging`
Type: `check`

Bounce Target: `impl_safe_pwhash_packaging`

Purpose: Verify Argon2, scrypt, KDF, AES-GCM semantics, cross-family keygen coverage, pkg-config surface, and Debian package assembly.

Commands:
- `cargo build --manifest-path safe/Cargo.toml --release`
- `cargo test --manifest-path safe/Cargo.toml --test ported_pwhash`
- `./safe/tools/run-original-c-tests.sh aead_aes256gcm aead_aes256gcm2 kdf pwhash_argon2i pwhash_argon2id pwhash_scrypt pwhash_scrypt_ll keygen`
- `./safe/tools/relink-original-objects.sh aead_aes256gcm aead_aes256gcm2 kdf pwhash_argon2i pwhash_argon2id pwhash_scrypt pwhash_scrypt_ll keygen`
- `./safe/tools/build-deb.sh`
- `readelf -d safe/target/release/libsodium.so | rg 'SONAME|NEEDED'`
- `dpkg-deb -c ./libsodium23_1.0.18-1ubuntu0.24.04.1+safelibs1_amd64.deb | rg 'libsodium\\.so\\.23|libsodium\\.so\\.23\\.3\\.0'`
- `dpkg-deb -c ./libsodium-dev_1.0.18-1ubuntu0.24.04.1+safelibs1_amd64.deb | rg 'include/sodium|pkgconfig/libsodium\\.pc|libsodium\\.a|libsodium\\.so$'`
- `./safe/tools/check-symbols.sh --expected safe/cabi/expected/full.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so`

## Success Criteria
- All pwhash, KDF, AES-GCM, and keygen upstream C tests pass from source and from relinked object files.
- `dpkg-buildpackage` produces installable `libsodium23` and `libsodium-dev` packages.
- The generated `.deb` contents expose the same library names, symlinks, header paths, and pkg-config path that the current dependent matrix expects.
- `safe/debian/libsodium23.symbols` and `safe/packaging/libsodium.pc.in` remain traceable synced derivatives of `original/debian/libsodium23.symbols` and `original/libsodium.pc.in`.
- The release artifact now matches the full `safe/cabi/expected/full.symbols` manifest exactly, with kind classification matching `safe/cabi/expected/upstream-kinds.tsv`.
- AES-GCM keeps the upstream availability contract: `crypto_aead_aes256gcm_is_available()` and its `ENOSYS` failure path stay intact unless a deliberate compatibility review explicitly justifies widened availability for all callers.

## Git Commit Requirement
The implementer must commit phase 4 work to git before yielding.
