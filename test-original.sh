#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
IMAGE_TAG="${LIBSODIUM_DEPENDENT_IMAGE:-${LIBSODIUM_ORIGINAL_TEST_IMAGE:-libsodium-original-test:ubuntu24.04}}"
MODE="safe"
ONLY=""
FROM_LIST=""
REPORT_DIR=""
STRICT=0

usage() {
  cat <<'EOF'
usage: test-original.sh [--mode safe|original] [--only <package>] [--from-list <file>] [--report-dir <dir>] [--strict]

In safe mode (the default), builds the local safe Debian packages inside an
Ubuntu 24.04 Docker container, upgrades the installed libsodium23 and
libsodium-dev packages in place, and then smoke-tests the direct dependents
listed in dependents.json against that package install.

--mode original keeps the old /usr/local upstream build as a comparison path.
--only runs just one dependent entry from dependents.json.
--from-list reads a newline-delimited package selection file.
--report-dir writes results.tsv, failures.list, and logs/<package>.log.
--strict exits nonzero when any report row is FAIL or WARN.
EOF
}

die() {
  printf 'error: %s\n' "$*" >&2
  exit 1
}

resolve_host_dir() {
  local input="$1"
  local parent
  local base

  parent="$(dirname -- "$input")"
  base="$(basename -- "$input")"
  mkdir -p "$parent"
  parent="$(cd -- "$parent" && pwd)"
  printf '%s/%s\n' "$parent" "$base"
}

resolve_host_file() {
  local input="$1"
  local parent

  [[ -f "$input" ]] || die "missing package list: $input"
  parent="$(cd -- "$(dirname -- "$input")" && pwd)"
  printf '%s/%s\n' "$parent" "$(basename -- "$input")"
}

while (($#)); do
  case "$1" in
    --mode)
      MODE="${2:?missing value for --mode}"
      case "$MODE" in
        safe|original)
          ;;
        *)
          printf 'unknown mode: %s\n' "$MODE" >&2
          usage >&2
          exit 1
          ;;
      esac
      shift 2
      ;;
    --only)
      ONLY="${2:?missing value for --only}"
      shift 2
      ;;
    --from-list)
      FROM_LIST="${2:?missing value for --from-list}"
      shift 2
      ;;
    --report-dir)
      REPORT_DIR="${2:?missing value for --report-dir}"
      shift 2
      ;;
    --strict)
      STRICT=1
      shift
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

[[ -z "$ONLY" || -z "$FROM_LIST" ]] || die "--only and --from-list are mutually exclusive"

command -v docker >/dev/null 2>&1 || die "docker is required to run $0"
[[ -d "$ROOT/original" ]] || die "missing original source tree"
[[ -f "$ROOT/dependents.json" ]] || die "missing dependents.json"

if [[ -n "$REPORT_DIR" ]]; then
  REPORT_DIR="$(resolve_host_dir "$REPORT_DIR")"
  mkdir -p "$REPORT_DIR"
fi

if [[ -n "$FROM_LIST" ]]; then
  FROM_LIST="$(resolve_host_file "$FROM_LIST")"
fi

if [[ "${LIBSODIUM_SKIP_IMAGE_BUILD:-0}" != "1" ]]; then
  "$ROOT/safe/tools/build-dependent-image.sh" --tag "$IMAGE_TAG"
fi

docker_args=(
  --rm
  -i
  -e "LIBSODIUM_TEST_MODE=$MODE"
  -e "LIBSODIUM_TEST_ONLY=$ONLY"
  -e "LIBSODIUM_TEST_STRICT=$STRICT"
  -v "$ROOT":/work:ro
)

if [[ -n "$FROM_LIST" ]]; then
  docker_args+=(
    -e "LIBSODIUM_TEST_FROM_LIST=/selection/from-list"
    -v "$FROM_LIST":/selection/from-list:ro
  )
fi

if [[ -n "$REPORT_DIR" ]]; then
  docker_args+=(
    -e "LIBSODIUM_TEST_REPORT_DIR=/reports"
    -v "$REPORT_DIR":/reports
  )
fi

docker run "${docker_args[@]}" "$IMAGE_TAG" bash -s <<'CONTAINER_SCRIPT'
set -euo pipefail

export LANG=C.UTF-8
export LC_ALL=C.UTF-8

ROOT=/work
SRC_ROOT=/tmp/libsodium-original
SAFE_BUILD_ROOT=/tmp/libsodium-safe
ONLY_FILTER="${LIBSODIUM_TEST_ONLY:-}"
FROM_LIST_PATH="${LIBSODIUM_TEST_FROM_LIST:-}"
MODE="${LIBSODIUM_TEST_MODE:-safe}"
REPORT_DIR="${LIBSODIUM_TEST_REPORT_DIR:-}"
STRICT="${LIBSODIUM_TEST_STRICT:-0}"
MULTIARCH="$(gcc -print-multiarch)"
EXPECTED_LIBSODIUM_PATH=""
EXPECTED_LIBSODIUM_LIBDIR=""
DEPENDENTS_EXPECTED=16
DEPENDENTS_RUN=0
SELECTED_COUNT=0
PASS_COUNT=0
FAIL_COUNT=0
WARN_COUNT=0
RESULTS_FILE=""
FAILURES_FILE=""
LOG_DIR=""
declare -a selected_packages=()

log_step() {
  printf '\n==> %s\n' "$1"
}

die() {
  echo "error: $*" >&2
  exit 1
}

case "$MODE" in
  safe|original)
    ;;
  *)
    die "unsupported mode: $MODE"
    ;;
esac

case "$STRICT" in
  0|1)
    ;;
  *)
    die "unsupported strict flag: $STRICT"
    ;;
esac

if [[ "$MODE" == "original" ]]; then
  export LD_LIBRARY_PATH="/usr/local/lib:/usr/local/lib/$MULTIARCH${LD_LIBRARY_PATH:+:$LD_LIBRARY_PATH}"
  export PKG_CONFIG_PATH="/usr/local/lib/pkgconfig:/usr/local/lib/$MULTIARCH/pkgconfig:/usr/local/share/pkgconfig${PKG_CONFIG_PATH:+:$PKG_CONFIG_PATH}"
else
  unset LD_LIBRARY_PATH || true
  unset PKG_CONFIG_PATH || true
fi

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

pkgconfig_libdir() {
  pkg-config --variable=libdir libsodium
}

dpkg_libsodium_path() {
  dpkg -L libsodium23 \
    | awk '/\/libsodium\.so\.23(\.[0-9]+)*$/ { print }' \
    | sort -V \
    | tail -n1
}

assert_active_libsodium_resolution() {
  local libdir
  local ldconfig_path

  [[ -n "$EXPECTED_LIBSODIUM_PATH" ]] || die "expected libsodium path is not configured"
  [[ -n "$EXPECTED_LIBSODIUM_LIBDIR" ]] || die "expected libsodium libdir is not configured"

  libdir="$(pkgconfig_libdir)"
  [[ "$libdir" == "$EXPECTED_LIBSODIUM_LIBDIR" ]] \
    || die "pkg-config resolved libsodium to $libdir, expected $EXPECTED_LIBSODIUM_LIBDIR"

  ldconfig_path="$(readlink -f "$(get_library_path libsodium.so.23)")"
  [[ "$ldconfig_path" == "$EXPECTED_LIBSODIUM_PATH" ]] \
    || die "ldconfig resolved libsodium.so.23 to $ldconfig_path, expected $EXPECTED_LIBSODIUM_PATH"
}

assert_uses_selected_libsodium() {
  local target="$1"
  local resolved

  resolved="$(ldd "$target" | awk '/libsodium\.so\.23/ { print $3; exit }')"
  [[ -n "$resolved" ]] || die "ldd did not report libsodium for $target"
  resolved="$(readlink -f "$resolved")"
  [[ "$resolved" == "$EXPECTED_LIBSODIUM_PATH" ]] || {
    printf 'expected %s to resolve libsodium from %s, got %s\n' \
      "$target" "$EXPECTED_LIBSODIUM_PATH" "$resolved" >&2
    ldd "$target" >&2
    exit 1
  }
}

assert_dependents_inventory() {
  local expected actual actual_count
  expected=$'minisign\nshadowsocks-libev\nlibtoxcore2\nqtox\nfastd\ncurvedns\nnix-bin\nlibzmq5\nvim\nphp8.3-cli\npython3-nacl\nruby-rbnacl\nr-cran-sodium\nlibrust-libsodium-sys-dev\nlibtoxcore-dev\nlibzmq3-dev'
  actual="$(jq -r '.dependents[].package' "$ROOT/dependents.json")"
  actual_count="$(jq '.dependents | length' "$ROOT/dependents.json")"

  [[ "$actual_count" == "$DEPENDENTS_EXPECTED" ]] \
    || die "dependents.json contains $actual_count entries, expected $DEPENDENTS_EXPECTED"

  if [[ "$actual" != "$expected" ]]; then
    echo "dependents.json does not match the expected dependent matrix" >&2
    diff -u <(printf '%s\n' "$expected") <(printf '%s\n' "$actual") >&2 || true
    exit 1
  fi
}

validate_selected_package() {
  local package="$1"

  jq -e --arg package "$package" '.dependents[] | select(.package == $package)' \
    "$ROOT/dependents.json" >/dev/null || die "selection did not match any dependent: $package"
}

resolve_selected_packages() {
  local line
  local trimmed

  selected_packages=()

  if [[ -n "$ONLY_FILTER" && -n "$FROM_LIST_PATH" ]]; then
    die "--only and --from-list are mutually exclusive"
  fi

  if [[ -n "$ONLY_FILTER" ]]; then
    validate_selected_package "$ONLY_FILTER"
    selected_packages=("$ONLY_FILTER")
  elif [[ -n "$FROM_LIST_PATH" ]]; then
    [[ -f "$FROM_LIST_PATH" ]] || die "missing package list: $FROM_LIST_PATH"
    while IFS= read -r line || [[ -n "$line" ]]; do
      trimmed="${line#"${line%%[![:space:]]*}"}"
      trimmed="${trimmed%"${trimmed##*[![:space:]]}"}"
      [[ -n "$trimmed" ]] || continue
      validate_selected_package "$trimmed"
      selected_packages+=("$trimmed")
    done < "$FROM_LIST_PATH"
  else
    mapfile -t selected_packages < <(jq -r '.dependents[].package' "$ROOT/dependents.json")
  fi

  SELECTED_COUNT=${#selected_packages[@]}
}

setup_report_dir() {
  [[ -n "$REPORT_DIR" ]] || return 0

  RESULTS_FILE="$REPORT_DIR/results.tsv"
  FAILURES_FILE="$REPORT_DIR/failures.list"
  LOG_DIR="$REPORT_DIR/logs"

  mkdir -p "$REPORT_DIR"
  rm -rf "$LOG_DIR"
  mkdir -p "$LOG_DIR"
  printf 'package\tmode\tstatus\tlog_path\n' > "$RESULTS_FILE"
  : > "$FAILURES_FILE"
}

record_result() {
  local package="$1"
  local status="$2"
  local log_rel="$3"

  printf '%s\t%s\t%s\t%s\n' "$package" "$MODE" "$status" "$log_rel" >> "$RESULTS_FILE"

  case "$status" in
    PASS)
      PASS_COUNT=$((PASS_COUNT + 1))
      ;;
    FAIL)
      FAIL_COUNT=$((FAIL_COUNT + 1))
      printf '%s\n' "$package" >> "$FAILURES_FILE"
      ;;
    WARN)
      WARN_COUNT=$((WARN_COUNT + 1))
      printf '%s\n' "$package" >> "$FAILURES_FILE"
      ;;
    *)
      die "unsupported status: $status"
      ;;
  esac
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
  EXPECTED_LIBSODIUM_LIBDIR="$(pkgconfig_libdir)"
  [[ "$EXPECTED_LIBSODIUM_LIBDIR" == /usr/local/lib* ]] \
    || die "pkg-config did not resolve the /usr/local libsodium build"
  EXPECTED_LIBSODIUM_PATH="$(readlink -f "$(get_library_path libsodium.so.23)")"
  case "$EXPECTED_LIBSODIUM_PATH" in
    /usr/local/lib/*|/usr/local/lib/"$MULTIARCH"/*)
      ;;
    *)
      die "ldconfig did not resolve the /usr/local libsodium build"
      ;;
  esac
  assert_active_libsodium_resolution
}

build_safe_libsodium_packages() {
  local runtime_before
  local dev_before
  local runtime_after
  local dev_after
  local runtime_deb
  local dev_deb

  log_step "Building and installing safe libsodium packages"
  runtime_before="$(dpkg-query -W -f='${Version}' libsodium23)"
  dev_before="$(dpkg-query -W -f='${Version}' libsodium-dev)"

  rm -rf "$SAFE_BUILD_ROOT"
  cp -a "$ROOT" "$SAFE_BUILD_ROOT"
  "$SAFE_BUILD_ROOT/safe/tools/build-deb.sh" >/tmp/libsodium-safe-build.log 2>&1

  runtime_deb="$(find "$SAFE_BUILD_ROOT" -maxdepth 1 -type f -name 'libsodium23_*.deb' | sort | tail -n1)"
  dev_deb="$(find "$SAFE_BUILD_ROOT" -maxdepth 1 -type f -name 'libsodium-dev_*.deb' | sort | tail -n1)"
  [[ -n "$runtime_deb" ]] || die "missing built libsodium23 package"
  [[ -n "$dev_deb" ]] || die "missing built libsodium-dev package"

  dpkg -i "$runtime_deb" "$dev_deb" >/tmp/libsodium-safe-install.log 2>&1
  ldconfig

  runtime_after="$(dpkg-query -W -f='${Version}' libsodium23)"
  dev_after="$(dpkg-query -W -f='${Version}' libsodium-dev)"
  [[ "$runtime_after" != "$runtime_before" ]] \
    || die "libsodium23 was not upgraded in place"
  [[ "$dev_after" != "$dev_before" ]] \
    || die "libsodium-dev was not upgraded in place"
  [[ "$runtime_after" == *+safelibs1 ]] \
    || die "libsodium23 did not upgrade to the safe package build"
  [[ "$dev_after" == *+safelibs1 ]] \
    || die "libsodium-dev did not upgrade to the safe package build"

  EXPECTED_LIBSODIUM_PATH="$(readlink -f "$(dpkg_libsodium_path)")"
  EXPECTED_LIBSODIUM_LIBDIR="$(pkgconfig_libdir)"
  [[ "$EXPECTED_LIBSODIUM_LIBDIR" == "$(dirname "$EXPECTED_LIBSODIUM_PATH")" ]] \
    || die "pkg-config libdir does not match the package-installed libsodium path"
  assert_active_libsodium_resolution
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
  local dir
  local base
  local name
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
  assert_uses_selected_libsodium "$(command -v minisign)"

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
  assert_uses_selected_libsodium "$(command -v ss-server)"
  assert_uses_selected_libsodium "$(command -v ss-local)"

  (
    set -euo pipefail
    local_work="$(mktemp -d)"
    local http_pid
    local server_pid
    local client_pid

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
  assert_uses_selected_libsodium "$(get_library_path libtoxcore.so.2)"
  build_tox_smoke /tmp/tox-runtime-smoke
  /tmp/tox-runtime-smoke > /tmp/tox-runtime.log 2>&1
  require_contains /tmp/tox-runtime.log "TOX_OK"
}

test_qtox() {
  log_step "qtox"
  assert_uses_selected_libsodium "$(get_library_path libtoxcore.so.2)"

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
  assert_uses_selected_libsodium "$(command -v fastd)"
  fastd --generate-key > /tmp/fastd.log 2>&1
  require_contains /tmp/fastd.log "Secret:"
  require_contains /tmp/fastd.log "Public:"
}

test_curvedns() {
  log_step "curvedns"
  assert_uses_selected_libsodium "$(command -v curvedns)"

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
  assert_uses_selected_libsodium "$(command -v nix-store)"

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
  assert_uses_selected_libsodium "$(get_library_path libzmq.so.5)"
  build_zmq_curve_smoke /tmp/zmq-runtime-smoke
  /tmp/zmq-runtime-smoke > /tmp/zmq-runtime.log 2>&1
  require_contains /tmp/zmq-runtime.log "ZMQ_CURVE_OK"
}

test_vim() {
  log_step "vim"
  assert_uses_selected_libsodium "$(command -v vim)"

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
  local crate_dir
  local patch_table
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

dispatch_package() {
  local package="$1"

  case "$package" in
    minisign)
      test_minisign
      ;;
    shadowsocks-libev)
      test_shadowsocks_libev
      ;;
    libtoxcore2)
      test_libtoxcore2
      ;;
    qtox)
      test_qtox
      ;;
    fastd)
      test_fastd
      ;;
    curvedns)
      test_curvedns
      ;;
    nix-bin)
      test_nix_bin
      ;;
    libzmq5)
      test_libzmq5
      ;;
    vim)
      test_vim
      ;;
    php8.3-cli)
      test_php8_3_cli
      ;;
    python3-nacl)
      test_python3_nacl
      ;;
    ruby-rbnacl)
      test_ruby_rbnacl
      ;;
    r-cran-sodium)
      test_r_cran_sodium
      ;;
    librust-libsodium-sys-dev)
      test_librust_libsodium_sys_dev
      ;;
    libtoxcore-dev)
      test_libtoxcore_dev
      ;;
    libzmq3-dev)
      test_libzmq3_dev
      ;;
    *)
      die "unsupported dependent package: $package"
      ;;
  esac
}

run_package() {
  local package="$1"
  local log_rel
  local log_path
  local status
  local rc

  if [[ -n "$REPORT_DIR" ]]; then
    log_rel="logs/$package.log"
    log_path="$REPORT_DIR/$log_rel"

    set +e
    (dispatch_package "$package") >"$log_path" 2>&1
    rc=$?
    set -e

    if [[ "$rc" -eq 0 ]]; then
      status="PASS"
    else
      status="FAIL"
    fi

    record_result "$package" "$status" "$log_rel"
    DEPENDENTS_RUN=$((DEPENDENTS_RUN + 1))
    printf '%s\t%s\n' "$status" "$package"
    return 0
  fi

  dispatch_package "$package"
  DEPENDENTS_RUN=$((DEPENDENTS_RUN + 1))
}

assert_dependents_inventory
resolve_selected_packages
setup_report_dir

if [[ "$SELECTED_COUNT" -eq 0 ]]; then
  if [[ -n "$REPORT_DIR" ]]; then
    printf '\nNo dependent entries were selected. Wrote header-only results to %s.\n' "$REPORT_DIR"
  else
    printf '\nNo dependent entries were selected.\n'
  fi
  exit 0
fi

case "$MODE" in
  safe)
    build_safe_libsodium_packages
    ;;
  original)
    build_original_libsodium
    ;;
esac

for package in "${selected_packages[@]}"; do
  run_package "$package"
done

[[ "$DEPENDENTS_RUN" -eq "$SELECTED_COUNT" ]] \
  || die "expected $SELECTED_COUNT dependent checks to run, got $DEPENDENTS_RUN"

if [[ -n "$REPORT_DIR" ]]; then
  printf '\nCompleted %d dependent check(s): %d PASS, %d FAIL, %d WARN. Reports written to %s.\n' \
    "$SELECTED_COUNT" "$PASS_COUNT" "$FAIL_COUNT" "$WARN_COUNT" "$REPORT_DIR"
  if [[ "$STRICT" == "1" ]] && ((FAIL_COUNT > 0 || WARN_COUNT > 0)); then
    exit 1
  fi
elif [[ "$SELECTED_COUNT" -eq 1 ]]; then
  printf '\nConfirmed selected dependent entry %s passed through the modified Docker harness.\n' \
    "${selected_packages[0]}"
elif [[ "$SELECTED_COUNT" -eq "$DEPENDENTS_EXPECTED" ]]; then
  printf '\nConfirmed all 16 dependent entries in dependents.json passed through the modified Docker harness.\n'
else
  printf '\nConfirmed %d selected dependent entries passed through the modified Docker harness.\n' \
    "$SELECTED_COUNT"
fi
CONTAINER_SCRIPT
