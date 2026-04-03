# Phase 05 Compat Harness

## Phase Name
Full compatibility harness and Rust test port completion

## Implement Phase ID
`impl_safe_harness`

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
- `safe/debian/control`
- `safe/debian/rules`
- `safe/debian/changelog`
- `safe/debian/source/format`
- `safe/debian/libsodium23.install`
- `safe/debian/libsodium-dev.install`
- `safe/debian/libsodium23.symbols`
- `safe/packaging/libsodium.pc.in`
- `safe/tools/build-deb.sh`
- `safe/tools/check-symbols.sh`
- `safe/tools/run-original-c-tests.sh`
- `safe/tools/relink-original-objects.sh`
- `safe/cabi/expected/full.symbols`
- `safe/cabi/expected/upstream-kinds.tsv`
- `safe/include/sodium.h`
- `safe/include/sodium/*.h`
- `test-original.sh`
- `dependents.json`
- `original/test/default/Makefile.am`
- `original/test/default/misuse.c`
- `original/test/default/*.c`
- `original/test/default/*.o`
- `original/test/default/*.exp`

## New Outputs
- Completed Rust ports of the full upstream test suite
- A Docker harness that installs the safe Debian packages instead of building upstream from source
- A complete source-compat and link-compat runner for all 77 legacy tests

## File Changes
- Modify `test-original.sh`
- Create `safe/tests/ported_all.rs` or expand it if a partial stub already exists
- Create `safe/tools/run-full-compat.sh`

## Implementation Details
- Change the root harness from building upstream under `/usr/local` to building and installing safe `.deb` packages.
- Keep `--mode original` as an escape hatch for baseline comparisons, but make the safe package path the default verification target.
- Add `--mode safe|original` parsing and help text to `test-original.sh`. Do not create a separate harness.
- In `--mode safe`, build the local packages after the apt dependencies are installed, install them with `dpkg -i`, and prove that the archive `libsodium23` and `libsodium-dev` packages were upgraded in place.
- Do not rely on `/usr/local`, `LD_LIBRARY_PATH`, or `PKG_CONFIG_PATH` shadowing in safe mode.
- Replace the current `/usr/local`-specific assertions with checks against `dpkg -L libsodium23`, `pkg-config --variable=libdir libsodium`, and `ldconfig -p`.
- Add the Debian packaging prerequisites to the Docker image build in `test-original.sh`.
- Preserve the dependent inventory and ordering in `dependents.json`. Do not modify that file or replace it with a new discovery pass.
- Finish porting the remaining upstream C tests into Rust integration tests. This phase must explicitly port `original/test/default/misuse.c`.
- Make `run-original-c-tests.sh --all` and `relink-original-objects.sh --all` cover the actual 77 runnable tests and 77 prebuilt `.o` files, including `xchacha20`, `keygen`, `metamorphic`, and `misuse`, and excluding nonexistent artifacts such as `hash2`.
- Resolve the active `!EMSCRIPTEN` and `!MINIMAL` additions from `original/test/default/Makefile.am` before intersecting with the current `.c` and `.o` files.
- Build the safe package from a writable repo copy inside Docker, such as `/tmp/libsodium-safe` copied from `/work`, so `safe/` and `original/` remain available together if the safe build still consumes synced upstream interface artifacts.
- Reuse the existing dependent smoke-test functions in `test-original.sh`, updating only the build and install path plus library-resolution assertions rather than replacing the package-specific checks.
- Preserve the phase-4 full ABI, `safe/debian/libsodium23.symbols`, and `safe/packaging/libsodium.pc.in` unchanged unless work bounces back to regenerate those synced derivatives from the upstream artifacts.

## Verification Phases
- `Phase ID`: `check_safe_harness`
- `Type`: `check`
- `bounce_target`: `impl_safe_harness`
- `Purpose`: Verify that the root Docker harness now installs the Rust-built replacement package, that the ported Rust tests cover the full upstream suite, and that representative dependent compile-time and runtime smoke tests pass through the package-install path.
- `Commands`:
```bash
cargo build --manifest-path safe/Cargo.toml --release
cargo test --manifest-path safe/Cargo.toml --all-targets
./safe/tools/run-original-c-tests.sh --all
./safe/tools/relink-original-objects.sh --all
./safe/tools/check-symbols.sh --expected safe/cabi/expected/full.symbols --kinds safe/cabi/expected/upstream-kinds.tsv safe/target/release/libsodium.so
./test-original.sh --mode safe --only librust-libsodium-sys-dev
./test-original.sh --mode safe --only libtoxcore-dev
./test-original.sh --mode safe --only libzmq3-dev
./test-original.sh --mode safe --only minisign
./test-original.sh --mode safe --only libzmq5
./test-original.sh --mode safe --only python3-nacl
```

## Success Criteria
- The modified harness installs the safe package rather than using `/usr/local` shadowing
- Representative compile-time and runtime dependents succeed through the package-install path
- The full legacy C test suite passes both from source and from relinked upstream object files
- `misuse` is included in the `--all` suite and its abort-driven behavior passes under the safe library
- The release artifact still matches `safe/cabi/expected/full.symbols` exactly after the harness and final test-port changes

## Git Commit Requirement
The implementer must commit work to git before yielding.
