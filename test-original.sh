#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
IMAGE_TAG="${LIBSODIUM_ORIGINAL_TEST_IMAGE:-libsodium-original-test:ubuntu24.04}"
ONLY=""

usage() {
  cat <<'EOF'
usage: test-original.sh [--only <package>]

Builds the original libsodium from ./original inside an Ubuntu 24.04 Docker
container, installs it into /usr/local, and then smoke-tests the direct
dependents listed in dependents.json against that build.

--only runs just one dependent entry from dependents.json.
EOF
}

while (($#)); do
  case "$1" in
    --only)
      ONLY="${2:?missing value for --only}"
      shift 2
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      printf 'unknown option: %s\n' "$1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

command -v docker >/dev/null 2>&1 || {
  echo "docker is required to run $0" >&2
  exit 1
}

[[ -d "$ROOT/original" ]] || {
  echo "missing original source tree" >&2
  exit 1
}

[[ -f "$ROOT/dependents.json" ]] || {
  echo "missing dependents.json" >&2
  exit 1
}

docker build -t "$IMAGE_TAG" - <<'DOCKERFILE'
FROM ubuntu:24.04

ARG DEBIAN_FRONTEND=noninteractive

RUN apt-get update \
 && apt-get install -y --no-install-recommends \
      autoconf \
      automake \
      build-essential \
      ca-certificates \
      cargo \
      curl \
      fastd \
      jq \
      librust-libc-dev \
      librust-libsodium-sys-dev \
      librust-pkg-config-dev \
      libtool \
      libtoxcore-dev \
      libzmq3-dev \
      minisign \
      netcat-openbsd \
      nix-bin \
      php8.3-cli \
      pkg-config \
      python3 \
      python3-nacl \
      qtox \
      r-base-core \
      r-cran-sodium \
      ruby \
      ruby-rbnacl \
      shadowsocks-libev \
      vim \
      curvedns \
 && rm -rf /var/lib/apt/lists/*
DOCKERFILE

docker run --rm -i \
  -e "LIBSODIUM_TEST_ONLY=$ONLY" \
  -v "$ROOT":/work:ro \
  "$IMAGE_TAG" \
  bash -s <<'CONTAINER_SCRIPT'
set -euo pipefail

export LANG=C.UTF-8
export LC_ALL=C.UTF-8

ROOT=/work
SRC_ROOT=/tmp/libsodium-original
ONLY_FILTER="${LIBSODIUM_TEST_ONLY:-}"
MULTIARCH="$(gcc -print-multiarch)"

export LD_LIBRARY_PATH="/usr/local/lib:/usr/local/lib/$MULTIARCH${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
export PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/local/lib/$MULTIARCH/pkgconfig:/usr/local/share/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"

log_step() {
  printf '\n==> %s\n' "$1"
}

die() {
  echo "error: $*" >&2
  exit 1
}

require_contains() {
  local path="$1"
  local needle="$2"

  if ! grep -F -- "$needle" "$path" >/dev/null 2>&1; then
    printf 'missing expected text in %s: %s\n' "$path" "$needle" >&2
    printf -- '--- %s ---\n' "$path" >&2
    cat "$path" >&2
    exit 1
  fi
}

require_nonempty_file() {
  local path="$1"

  if [[ ! -s "$path" ]]; then
    printf 'expected non-empty file: %s\n' "$path" >&2
    exit 1
  fi
}

get_library_path() {
  local soname="$1"
  local path

  path="$(ldconfig -p | awk -v soname="$soname" '$1 == soname { print $NF; exit }')"
  [[ -n "$path" ]] || die "unable to resolve library path for $soname"
  printf '%s\n' "$path"
}

assert_uses_original_libsodium() {
  local target="$1"
  local resolved

  resolved="$(ldd "$target" | awk '/libsodium\.so\.23/ { print $3; exit }')"
  [[ -n "$resolved" ]] || die "ldd did not report libsodium for $target"

  case "$resolved" in
    /usr/local/lib/*|/usr/local/lib/"$MULTIARCH"/*)
      ;;
    *)
      printf 'expected %s to resolve libsodium from /usr/local, got %s\n' "$target" "$resolved" >&2
      ldd "$target" >&2
      exit 1
      ;;
  esac
}

assert_dependents_inventory() {
  local expected actual
  expected=$'minisign\nshadowsocks-libev\nlibtoxcore2\nqtox\nfastd\ncurvedns\nnix-bin\nlibzmq5\nvim\nphp8.3-cli\npython3-nacl\nruby-rbnacl\nr-cran-sodium\nlibrust-libsodium-sys-dev\nlibtoxcore-dev\nlibzmq3-dev'
  actual="$(jq -r '.dependents[].package' "$ROOT/dependents.json")"

  if [[ "$actual" != "$expected" ]]; then
    echo "dependents.json does not match the expected dependent matrix" >&2
    diff -u <(printf '%s\n' "$expected") <(printf '%s\n' "$actual") >&2 || true
    exit 1
  fi

  if [[ -n "$ONLY_FILTER" ]]; then
    jq -e --arg package "$ONLY_FILTER" '.dependents[] | select(.package == $package)' \
      "$ROOT/dependents.json" >/dev/null || die "--only did not match any dependent: $ONLY_FILTER"
  fi
}

run_selected() {
  local package="$1"
  local fn="$2"

  if [[ -n "$ONLY_FILTER" && "$ONLY_FILTER" != "$package" ]]; then
    return 0
  fi

  "$fn"
}

build_original_libsodium() {
  log_step "Building original libsodium"
  cp -a "$ROOT/original" "$SRC_ROOT"
  cd "$SRC_ROOT"
  autoreconf -ivf >/tmp/libsodium-autogen.log 2>&1
  ./configure --prefix=/usr/local --disable-static >/tmp/libsodium-configure.log 2>&1
  make -j"$(nproc)" >/tmp/libsodium-make.log 2>&1
  make install >/tmp/libsodium-install.log 2>&1
  printf '/usr/local/lib\n/usr/local/lib/%s\n' "$MULTIARCH" > /etc/ld.so.conf.d/zz-libsodium-local.conf
  ldconfig
  cd /

  require_contains /tmp/libsodium-install.log "Libraries have been installed in:"
  [[ "$(pkg-config --variable=libdir libsodium)" == /usr/local/lib* ]] || die "pkg-config did not resolve the /usr/local libsodium build"
}

build_tox_smoke() {
  local output="$1"

  cat > "${output}.c" <<'EOF'
#include <stdio.h>
#include <tox/tox.h>

int main(void) {
    Tox_Err_Options_New opt_err;
    struct Tox_Options *options = tox_options_new(&opt_err);
    if (options == NULL) {
        fprintf(stderr, "tox_options_new failed: %d\n", opt_err);
        return 1;
    }

    tox_options_set_udp_enabled(options, false);
    tox_options_set_local_discovery_enabled(options, false);
    tox_options_set_hole_punching_enabled(options, false);

    Tox_Err_New new_err;
    Tox *tox = tox_new(options, &new_err);
    if (tox == NULL) {
        fprintf(stderr, "tox_new failed: %d\n", new_err);
        tox_options_free(options);
        return 1;
    }

    uint8_t address[TOX_ADDRESS_SIZE];
    uint8_t public_key[TOX_PUBLIC_KEY_SIZE];
    tox_self_get_address(tox, address);
    tox_self_get_public_key(tox, public_key);
    printf("TOX_OK %02x %02x\n", address[0], public_key[0]);

    tox_kill(tox);
    tox_options_free(options);
    return 0;
}
EOF

  cc "${output}.c" -o "$output" $(pkg-config --cflags --libs toxcore)
}

build_zmq_curve_smoke() {
  local output="$1"

  cat > "${output}.c" <<'EOF'
#include <stdio.h>
#include <string.h>
#include <unistd.h>
#include <zmq.h>

static void die(void *socket, const char *what) {
    fprintf(stderr, "%s: %s\n", what, zmq_strerror(zmq_errno()));
    if (socket != NULL) {
        zmq_close(socket);
    }
    _exit(1);
}

int main(void) {
    void *ctx = zmq_ctx_new();
    if (ctx == NULL) {
        die(NULL, "zmq_ctx_new");
    }

    char server_public[41], server_secret[41], client_public[41], client_secret[41];
    if (zmq_curve_keypair(server_public, server_secret) != 0) {
        die(NULL, "server keypair");
    }
    if (zmq_curve_keypair(client_public, client_secret) != 0) {
        die(NULL, "client keypair");
    }

    void *server = zmq_socket(ctx, ZMQ_REP);
    if (server == NULL) {
        die(NULL, "zmq_socket(server)");
    }

    int as_server = 1;
    if (zmq_setsockopt(server, ZMQ_CURVE_SERVER, &as_server, sizeof(as_server)) != 0) {
        die(server, "ZMQ_CURVE_SERVER");
    }
    if (zmq_setsockopt(server, ZMQ_CURVE_SECRETKEY, server_secret, 40) != 0) {
        die(server, "ZMQ_CURVE_SECRETKEY");
    }
    if (zmq_bind(server, "tcp://127.0.0.1:35557") != 0) {
        die(server, "zmq_bind");
    }

    void *client = zmq_socket(ctx, ZMQ_REQ);
    if (client == NULL) {
        die(server, "zmq_socket(client)");
    }
    if (zmq_setsockopt(client, ZMQ_CURVE_PUBLICKEY, client_public, 40) != 0) {
        die(client, "ZMQ_CURVE_PUBLICKEY");
    }
    if (zmq_setsockopt(client, ZMQ_CURVE_SECRETKEY, client_secret, 40) != 0) {
        die(client, "ZMQ_CURVE_SECRETKEY");
    }
    if (zmq_setsockopt(client, ZMQ_CURVE_SERVERKEY, server_public, 40) != 0) {
        die(client, "ZMQ_CURVE_SERVERKEY");
    }
    if (zmq_connect(client, "tcp://127.0.0.1:35557") != 0) {
        die(client, "zmq_connect");
    }

    usleep(200000);

    if (zmq_send(client, "ping", 4, 0) != 4) {
        die(client, "zmq_send(client)");
    }

    char buffer[16] = {0};
    if (zmq_recv(server, buffer, sizeof(buffer), 0) != 4) {
        die(server, "zmq_recv(server)");
    }
    if (strcmp(buffer, "ping") != 0) {
        fprintf(stderr, "unexpected request: %s\n", buffer);
        return 1;
    }

    if (zmq_send(server, "pong", 4, 0) != 4) {
        die(server, "zmq_send(server)");
    }
    memset(buffer, 0, sizeof(buffer));
    if (zmq_recv(client, buffer, sizeof(buffer), 0) != 4) {
        die(client, "zmq_recv(client)");
    }
    if (strcmp(buffer, "pong") != 0) {
        fprintf(stderr, "unexpected reply: %s\n", buffer);
        return 1;
    }

    puts("ZMQ_CURVE_OK");
    zmq_close(client);
    zmq_close(server);
    zmq_ctx_term(ctx);
    return 0;
}
EOF

  cc "${output}.c" -o "$output" $(pkg-config --cflags --libs libzmq)
}

write_cargo_patch_table() {
  local out="$1"
  local dir base name
  declare -A seen=()

  : > "$out"
  for dir in /usr/share/cargo/registry/*; do
    [[ -d "$dir" ]] || continue
    base="$(basename "$dir")"
    name="$(printf '%s\n' "$base" | sed -E 's/-[0-9][0-9A-Za-z.+~:-]*$//')"
    [[ -n "$name" ]] || continue
    [[ "$name" == "$base" ]] && continue
    [[ "$name" == "libsodium-sys" ]] && continue
    [[ -n "${seen[$name]:-}" ]] && continue
    seen["$name"]=1
    printf '%s = { path = "%s" }\n' "$name" "$dir" >> "$out"
  done
}

test_minisign() {
  log_step "minisign"
  assert_uses_original_libsodium "$(command -v minisign)"

  local work="/tmp/minisign-smoke"
  rm -rf "$work"
  mkdir -p "$work"
  cd "$work"

  printf 'smoke\n' > message.txt
  minisign -G -p pubkey -s seckey -W >/tmp/minisign-generate.log 2>&1
  minisign -S -s seckey -m message.txt -x signature.txt -t "smoke" >/tmp/minisign-sign.log 2>&1
  minisign -V -p pubkey -m message.txt -x signature.txt >/tmp/minisign-verify.log 2>&1

  require_nonempty_file "$work/pubkey"
  require_nonempty_file "$work/seckey"
  require_nonempty_file "$work/signature.txt"
  require_contains /tmp/minisign-verify.log "Signature and comment signature verified"
  cd /
}

test_shadowsocks_libev() {
  log_step "shadowsocks-libev"
  assert_uses_original_libsodium "$(command -v ss-server)"
  assert_uses_original_libsodium "$(command -v ss-local)"

  (
    set -euo pipefail
    local_work="$(mktemp -d)"
    local http_pid server_pid client_pid

    cleanup() {
      kill "${client_pid:-}" "${server_pid:-}" "${http_pid:-}" 2>/dev/null || true
      wait "${client_pid:-}" "${server_pid:-}" "${http_pid:-}" 2>/dev/null || true
      rm -rf "$local_work"
    }
    trap cleanup EXIT

    cd "$local_work"
    printf 'smoke through shadowsocks\n' > index.html

    python3 -m http.server 18080 --bind 127.0.0.1 >/tmp/shadowsocks-http.log 2>&1 &
    http_pid=$!

    cat > server.json <<'EOF'
{
  "server": "127.0.0.1",
  "server_port": 8388,
  "password": "test-password",
  "timeout": 60,
  "method": "xchacha20-ietf-poly1305"
}
EOF

    cat > local.json <<'EOF'
{
  "server": "127.0.0.1",
  "server_port": 8388,
  "local_address": "127.0.0.1",
  "local_port": 1080,
  "password": "test-password",
  "timeout": 60,
  "method": "xchacha20-ietf-poly1305"
}
EOF

    ss-server -c server.json >/tmp/ss-server.log 2>&1 &
    server_pid=$!
    ss-local -c local.json >/tmp/ss-local.log 2>&1 &
    client_pid=$!

    for port in 18080 8388 1080; do
      for _ in $(seq 1 20); do
        if nc -z 127.0.0.1 "$port" >/dev/null 2>&1; then
          break
        fi
        sleep 0.5
      done
      nc -z 127.0.0.1 "$port" >/dev/null 2>&1 || die "timed out waiting for port $port"
    done

    curl --silent --show-error --fail \
      --retry 10 \
      --retry-all-errors \
      --retry-delay 1 \
      --socks5-hostname 127.0.0.1:1080 \
      http://127.0.0.1:18080/index.html > out.txt
    require_contains out.txt "smoke through shadowsocks"
  )
}

test_libtoxcore2() {
  log_step "libtoxcore2"
  assert_uses_original_libsodium "$(get_library_path libtoxcore.so.2)"
  build_tox_smoke /tmp/tox-runtime-smoke
  /tmp/tox-runtime-smoke > /tmp/tox-runtime.log 2>&1
  require_contains /tmp/tox-runtime.log "TOX_OK"
}

test_qtox() {
  log_step "qtox"
  assert_uses_original_libsodium "$(get_library_path libtoxcore.so.2)"

  local work="/tmp/qtox-smoke"
  local status
  rm -rf "$work"
  mkdir -p "$work/home" "$work/config"

  set +e
  QT_QPA_PLATFORM=offscreen \
    HOME="$work/home" \
    XDG_CONFIG_HOME="$work/config" \
    timeout 15 qtox > /tmp/qtox.log 2>&1
  status=$?
  set -e

  if [[ "$status" != "0" && "$status" != "124" ]]; then
    cat /tmp/qtox.log >&2
    die "qtox exited with unexpected status $status"
  fi

  require_contains /tmp/qtox.log "Loading settings from :/conf/qtox.ini"
  require_contains /tmp/qtox.log "commit:"
}

test_fastd() {
  log_step "fastd"
  assert_uses_original_libsodium "$(command -v fastd)"
  fastd --generate-key > /tmp/fastd.log 2>&1
  require_contains /tmp/fastd.log "Secret:"
  require_contains /tmp/fastd.log "Public:"
}

test_curvedns() {
  log_step "curvedns"
  assert_uses_original_libsodium "$(command -v curvedns)"

  local work="/tmp/curvedns-smoke"
  rm -rf "$work"
  mkdir -p "$work"

  curvedns-keygen "$work" ns.example.com > /tmp/curvedns.log 2>&1
  require_nonempty_file "$work/env/CURVEDNS_PRIVATE_KEY"
  require_contains /tmp/curvedns.log "DNS public key:"
  require_contains /tmp/curvedns.log "Hex secret key:"
}

test_nix_bin() {
  log_step "nix-bin"
  assert_uses_original_libsodium "$(command -v nix-store)"

  local work="/tmp/nix-smoke"
  rm -rf "$work"
  mkdir -p "$work"
  cd "$work"

  nix-store --generate-binary-cache-key smoke.test cache.sec cache.pub >/tmp/nix-store.log 2>&1
  require_nonempty_file "$work/cache.sec"
  require_nonempty_file "$work/cache.pub"
  require_contains "$work/cache.pub" "smoke.test:"
  require_contains "$work/cache.sec" "smoke.test:"
  cd /
}

test_libzmq5() {
  log_step "libzmq5"
  assert_uses_original_libsodium "$(get_library_path libzmq.so.5)"
  build_zmq_curve_smoke /tmp/zmq-runtime-smoke
  /tmp/zmq-runtime-smoke > /tmp/zmq-runtime.log 2>&1
  require_contains /tmp/zmq-runtime.log "ZMQ_CURVE_OK"
}

test_vim() {
  log_step "vim"
  assert_uses_original_libsodium "$(command -v vim)"

  local work="/tmp/vim-smoke"
  rm -rf "$work"
  mkdir -p "$work"
  cd "$work"

  vim --version > /tmp/vim-version.log
  require_contains /tmp/vim-version.log "+sodium"

  cat > write.vim <<'EOF'
set nomore
set key=secret
set cryptmethod=xchacha20
call setline(1, ["alpha", "beta"])
wq! encrypted.txt
EOF

  vim -Nu NONE -n -es -S write.vim >/tmp/vim-write.log 2>&1
  require_nonempty_file "$work/encrypted.txt"
  if grep -F -- "alpha" "$work/encrypted.txt" >/dev/null 2>&1; then
    die "vim wrote plaintext into the encrypted file"
  fi

  head -c 12 "$work/encrypted.txt" > /tmp/vim-header.bin
  if [[ "$(cat /tmp/vim-header.bin)" != "VimCrypt~04!" ]]; then
    printf 'unexpected Vim encrypted file header: %s\n' "$(cat /tmp/vim-header.bin)" >&2
    exit 1
  fi
  cd /
}

test_php8_3_cli() {
  log_step "php8.3-cli"
  php8.3 <<'EOF' > /tmp/php-sodium.log
<?php
if (!extension_loaded('sodium')) {
    fwrite(STDERR, "sodium extension unavailable\n");
    exit(1);
}
$key = random_bytes(SODIUM_CRYPTO_SECRETBOX_KEYBYTES);
$nonce = random_bytes(SODIUM_CRYPTO_SECRETBOX_NONCEBYTES);
$cipher = sodium_crypto_secretbox('hello', $nonce, $key);
$plain = sodium_crypto_secretbox_open($cipher, $nonce, $key);
if ($plain !== 'hello') {
    fwrite(STDERR, "secretbox round-trip failed\n");
    exit(1);
}
$keypair = sodium_crypto_sign_keypair();
$signature = sodium_crypto_sign_detached('msg', sodium_crypto_sign_secretkey($keypair));
if (!sodium_crypto_sign_verify_detached($signature, 'msg', sodium_crypto_sign_publickey($keypair))) {
    fwrite(STDERR, "signature verification failed\n");
    exit(1);
}
echo "PHP_SODIUM_OK\n";
EOF
  require_contains /tmp/php-sodium.log "PHP_SODIUM_OK"
}

test_python3_nacl() {
  log_step "python3-nacl"
  python3 <<'EOF' > /tmp/python-nacl.log
from nacl.secret import SecretBox
from nacl.utils import random

box = SecretBox(random(SecretBox.KEY_SIZE))
message = b"hello"
nonce = random(SecretBox.NONCE_SIZE)
ciphertext = box.encrypt(message, nonce)
assert box.decrypt(ciphertext) == message
print("PYNACL_OK")
EOF
  require_contains /tmp/python-nacl.log "PYNACL_OK"
}

test_ruby_rbnacl() {
  log_step "ruby-rbnacl"
  ruby <<'EOF' > /tmp/ruby-rbnacl.log
require "rbnacl"

key = RbNaCl::Random.random_bytes(RbNaCl::SecretBox.key_bytes)
box = RbNaCl::SecretBox.new(key)
nonce = RbNaCl::Random.random_bytes(RbNaCl::SecretBox.nonce_bytes)
ciphertext = box.encrypt(nonce, "hello")
plaintext = box.decrypt(nonce, ciphertext)
abort "decrypt failed" unless plaintext == "hello"
puts "RBNACL_OK"
EOF
  require_contains /tmp/ruby-rbnacl.log "RBNACL_OK"
}

test_r_cran_sodium() {
  log_step "r-cran-sodium"
  Rscript -e 'library(sodium); key <- keygen(); cipher <- data_encrypt(charToRaw("hello"), key); plain <- rawToChar(data_decrypt(cipher, key)); stopifnot(plain == "hello"); kp <- sig_keygen(); sig <- sig_sign(charToRaw("msg"), kp); stopifnot(sig_verify(charToRaw("msg"), sig, sig_pubkey(kp))); cat("RSODIUM_OK\n")' \
    > /tmp/r-sodium.log
  require_contains /tmp/r-sodium.log "RSODIUM_OK"
}

test_librust_libsodium_sys_dev() {
  log_step "librust-libsodium-sys-dev"

  local work="/tmp/rust-libsodium-sys-smoke"
  local crate_dir patch_table
  rm -rf "$work"
  mkdir -p "$work/src"
  crate_dir="$(find /usr/share/cargo/registry -maxdepth 1 -mindepth 1 -type d -name 'libsodium-sys-*' | head -n1)"
  [[ -n "$crate_dir" ]] || die "unable to locate the installed libsodium-sys crate source"
  patch_table="$work/patches.toml"

  write_cargo_patch_table "$patch_table"

  cat > "$work/Cargo.toml" <<EOF
[package]
name = "libsodium-sys-smoke"
version = "0.1.0"
edition = "2021"

[dependencies]
libsodium-sys = { path = "$crate_dir", features = ["use-pkg-config"] }

[patch.crates-io]
$(cat "$patch_table")
EOF

  cat > "$work/src/main.rs" <<'EOF'
fn main() {
    unsafe {
        assert!(libsodium_sys::sodium_init() >= 0);
        let mut public_key = [0u8; libsodium_sys::crypto_box_PUBLICKEYBYTES as usize];
        let mut secret_key = [0u8; libsodium_sys::crypto_box_SECRETKEYBYTES as usize];
        assert_eq!(
            libsodium_sys::crypto_box_keypair(public_key.as_mut_ptr(), secret_key.as_mut_ptr()),
            0
        );
        println!("RUST_OK {} {}", public_key[0], secret_key[0]);
    }
}
EOF

  (cd "$work" && CARGO_NET_OFFLINE=true cargo run --quiet > /tmp/rust-libsodium-sys.log 2>&1)
  require_contains /tmp/rust-libsodium-sys.log "RUST_OK"
}

test_libtoxcore_dev() {
  log_step "libtoxcore-dev"
  build_tox_smoke /tmp/tox-dev-smoke
  /tmp/tox-dev-smoke > /tmp/tox-dev.log 2>&1
  require_contains /tmp/tox-dev.log "TOX_OK"
}

test_libzmq3_dev() {
  log_step "libzmq3-dev"
  build_zmq_curve_smoke /tmp/zmq-dev-smoke
  /tmp/zmq-dev-smoke > /tmp/zmq-dev.log 2>&1
  require_contains /tmp/zmq-dev.log "ZMQ_CURVE_OK"
}

assert_dependents_inventory
build_original_libsodium

run_selected minisign test_minisign
run_selected shadowsocks-libev test_shadowsocks_libev
run_selected libtoxcore2 test_libtoxcore2
run_selected qtox test_qtox
run_selected fastd test_fastd
run_selected curvedns test_curvedns
run_selected nix-bin test_nix_bin
run_selected libzmq5 test_libzmq5
run_selected vim test_vim
run_selected php8.3-cli test_php8_3_cli
run_selected python3-nacl test_python3_nacl
run_selected ruby-rbnacl test_ruby_rbnacl
run_selected r-cran-sodium test_r_cran_sodium
run_selected librust-libsodium-sys-dev test_librust_libsodium_sys_dev
run_selected libtoxcore-dev test_libtoxcore_dev
run_selected libzmq3-dev test_libzmq3_dev
CONTAINER_SCRIPT
