# Phase 04 Pwhash Kdf Packaging

## Phase Name
Password hashing, KDFs, AES-GCM, and Debian package build

## Implement Phase ID
`impl_safe_pwhash_packaging`

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
- `safe/src/core/*.rs`
- `safe/src/secretbox/mod.rs`
- `safe/src/secretstream.rs`
- `safe/src/shorthash.rs`
- `safe/src/aead/chacha20poly1305.rs`
- `safe/src/aead/xchacha20poly1305.rs`
- `safe/src/scalarmult/*.rs`
- `safe/src/sign/*.rs`
- `safe/src/box_api/*.rs`
- `safe/src/kx.rs`
- `safe/cabi/libsodium.map`
- `safe/cabi/expected/full.symbols`
- `safe/cabi/expected/upstream-kinds.tsv`
- `safe/tools/check-symbols.sh`
- `safe/tools/run-original-c-tests.sh`
- `safe/tools/relink-original-objects.sh`
- `safe/tests/ported_public_key.rs`
- `safe/tests/cve_2025_69277.rs`
- `safe/include/sodium.h`
- `safe/include/sodium/*.h`
- `original/src/libsodium/include/sodium/crypto_pwhash.h`
- `original/src/libsodium/include/sodium/crypto_pwhash_argon2i.h`
- `original/src/libsodium/include/sodium/crypto_pwhash_argon2id.h`
- `original/src/libsodium/include/sodium/crypto_pwhash_scryptsalsa208sha256.h`
- `original/src/libsodium/include/sodium/crypto_kdf.h`
- `original/src/libsodium/include/sodium/crypto_kdf_blake2b.h`
- `original/src/libsodium/include/sodium/crypto_aead_aes256gcm.h`
- `original/src/libsodium/crypto_pwhash/**/*`
- `original/src/libsodium/crypto_kdf/**/*`
- `original/src/libsodium/crypto_aead/aes256gcm/**/*`
- `original/debian/control`
- `original/debian/rules`
- `original/debian/libsodium23.install`
- `original/debian/libsodium-dev.install`
- `original/debian/libsodium23.symbols`
- `original/libsodium.pc.in`
- `original/test/default/keygen.c`
- `original/test/default/*.c`
- `original/test/default/*.o`
- `original/test/default/*.exp`

## New Outputs
- Rust password-hashing and KDF implementations
- A packageable shared library, static library, headers, and pkg-config file
- Debian packaging under `safe/debian/` that produces `libsodium23` and `libsodium-dev`
- Rust tests covering pwhash, KDF, and AES-GCM behavior

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
- Create `safe/debian/libsodium23.symbols`
- Create `safe/packaging/libsodium.pc.in`
- Create `safe/tools/build-deb.sh`

## Implementation Details
- Consume the carried-forward symbol manifests, synced headers, compatibility tooling, and previous-phase implementation artifacts in place.
- Cover every export from `sodium/crypto_pwhash*.h`, `sodium/crypto_kdf*.h`, and `sodium/crypto_aead_aes256gcm.h`.
- Implement the `crypto_pwhash_argon2i_*`, `crypto_pwhash_argon2id_*`, `crypto_pwhash_*`, and `crypto_pwhash_scryptsalsa208sha256_*` families, including raw KDF APIs, encoded password-string generation and verification, `*_needs_rehash`, algorithm selectors, and opslimit and memlimit accessors.
- Preserve the exact upstream string and selector behavior, including `crypto_pwhash_primitive()` returning `"argon2i"` even though the default algorithm is Argon2id.
- Implement `crypto_kdf_*` and `crypto_kdf_blake2b_*` with the same subkey-derivation rules and context-size checks as upstream.
- Implement `crypto_aead_aes256gcm_*`, including the 512-byte precomputation state with 16-byte alignment, and preserve upstream semantics for `crypto_aead_aes256gcm_is_available()` plus the `ENOSYS` failure path when the algorithm is unavailable.
- Treat `original/test/default/keygen.c` as the cross-family keygen gate for phases 2 through 4 and satisfy it here.
- Build Debian packages that install `libsodium.so.23.3.0`, the `libsodium.so.23` symlink, the development `libsodium.so` symlink, `libsodium.a`, all public headers, and `pkgconfig/libsodium.pc`.
- Keep the binary package names `libsodium23` and `libsodium-dev`.
- Use Debian version `1.0.18-1ubuntu0.24.04.1+safelibs1` in `safe/debian/changelog` so `dpkg -i` upgrades the archive package in place.
- Match the current multiarch install paths from `original/debian/libsodium23.install` and `original/debian/libsodium-dev.install`.
- Keep exported-name truth sourced from `original/debian/libsodium23.symbols` and symbol kind truth sourced from the phase-1 derivative `safe/cabi/expected/upstream-kinds.tsv`.
- `safe/debian/libsodium23.symbols` must be a synced derivative of `original/debian/libsodium23.symbols`, not a hand-maintained fork. If it needs to change, regenerate or sync it from the upstream symbol file rather than editing it as an independent contract.
- `safe/packaging/libsodium.pc.in` must be a synced derivative of `original/libsodium.pc.in`, not a hand-maintained rewrite. Preserve the upstream pkg-config surface and regenerate or sync from the upstream template if fields change.
- Use `cargo build --release` plus explicit install steps in the Debian rules file rather than a separate external packaging workflow.
- Make `safe/tools/build-deb.sh` delete any preexisting repo-root `libsodium23_*.deb`, `libsodium-dev_*.deb`, `libsodium-dbgsym_*.deb`, `*.buildinfo`, and `*.changes` before invoking `dpkg-buildpackage`.
- Prefer not to introduce new shared-library `NEEDED` dependencies beyond upstream `libc.so.6` unless they are unavoidable and handled by the Debian metadata.
- Reach the full ABI contract in this phase and preserve it unchanged in phases 5 and 6.

## Verification Phases
- `Phase ID`: `check_safe_pwhash_packaging`
- `Type`: `check`
- `bounce_target`: `impl_safe_pwhash_packaging`
- `Purpose`: Verify Argon2, scrypt, KDF, AES-GCM semantics, cross-family keygen coverage, pkg-config surface, and Debian package assembly.
- `Commands`:
```bash
cargo build --manifest-path safe/Cargo.toml --release
cargo test --manifest-path safe/Cargo.toml --test ported_pwhash
./safe/tools/run-original-c-tests.sh aead_aes256gcm aead_aes256gcm2 kdf pwhash_argon2i pwhash_argon2id pwhash_scrypt pwhash_scrypt_ll keygen
./safe/tools/relink-original-objects.sh aead_aes256gcm aead_aes256gcm2 kdf pwhash_argon2i pwhash_argon2id pwhash_scrypt pwhash_scrypt_ll keygen
./safe/tools/build-deb.sh
readelf -d safe/target/release/libsodium.so | rg 'SONAME|NEEDED'
dpkg-deb -c ./libsodium23_1.0.18-1ubuntu0.24.04.1+safelibs1_amd64.deb | rg 'libsodium\.so\.23|libsodium\.so\.23\.3\.0'
dpkg-deb -c ./libsodium-dev_1.0.18-1ubuntu0.24.04.1+safelibs1_amd64.deb | rg 'include/sodium|pkgconfig/libsodium\.pc|libsodium\.a|libsodium\.so$'
./safe/tools/check-symbols.sh --expected safe/cabi/expected/full.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so
```

## Success Criteria
- All listed pwhash, KDF, AES-GCM, and keygen C tests pass from source and from relinked upstream object files
- `dpkg-buildpackage` produces installable `libsodium23` and `libsodium-dev` packages
- The `.deb` contents expose the same library names, symlinks, header paths, and pkg-config path that the dependent matrix expects
- The release artifact matches `safe/cabi/expected/full.symbols` exactly, with kind classification matching `safe/cabi/expected/upstream-kinds.tsv`
- `safe/debian/libsodium23.symbols` and `safe/packaging/libsodium.pc.in` remain synced derivatives of the upstream artifacts rather than hand-maintained copies
- The full ABI contract is reached in this phase and remains unchanged in later phases

## Git Commit Requirement
The implementer must commit work to git before yielding.
