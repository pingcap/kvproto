#!/usr/bin/env bash
set -euo pipefail

SCRIPTS_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
KVPROTO_ROOT=$(cd "${SCRIPTS_DIR}/.." && pwd)

# Keep this pinned to ensure deterministic formatting across machines/CI.
BUF_VERSION="${BUF_VERSION:-1.64.0}"
BUF_BIN="${KVPROTO_ROOT}/bin/buf"
BUF_CURL_RETRY="${BUF_CURL_RETRY:-3}"

usage() {
    cat <<'EOF'
Usage:
  scripts/proto_format.sh --check   # verify proto files are formatted
  scripts/proto_format.sh --write   # format proto files in-place

This script downloads a pinned version of buf into ./bin (gitignored) if needed.

Optional env vars:
  BUF_VERSION    override pinned buf version
  BUF_CURL_RETRY retry count for downloads (default: 3)
EOF
}

resolve_buf_download_url() {
    local os arch
    os="${BUF_OS:-$(uname -s)}"
    arch="${BUF_ARCH:-$(uname -m)}"

    case "${os}" in
        Linux | Darwin) ;;
        *)
            echo "Unsupported OS for buf: ${os}" >&2
            return 1
            ;;
    esac

    # buf release asset names differ between Linux and Darwin for arm64.
    case "${os}:${arch}" in
        Linux:amd64 | Linux:x86_64 | Darwin:amd64 | Darwin:x86_64)
            arch="x86_64"
            ;;
        Linux:arm64 | Linux:aarch64)
            arch="aarch64"
            ;;
        Darwin:arm64)
            arch="arm64"
            ;;
        *)
            echo "Unsupported architecture for buf on ${os}: ${arch}" >&2
            return 1
            ;;
    esac

    printf '%s\n' "https://github.com/bufbuild/buf/releases/download/v${BUF_VERSION}/buf-${os}-${arch}"
}

install_buf() {
    local found_version url tmp_buf expected_sha actual_sha

    if [ -x "${BUF_BIN}" ]; then
        if found_version="$("${BUF_BIN}" --version 2>/dev/null)"; then
            found_version="${found_version%%[[:space:]]*}"
            found_version="${found_version#v}"
            if [ "${found_version}" = "${BUF_VERSION}" ]; then
                return 0
            fi
            echo "Found existing buf at ${BUF_BIN} with version ${found_version}, expected ${BUF_VERSION}. Please manually update or remove it." >&2
            return 1
        else
            echo "Found existing buf at ${BUF_BIN}, but failed to read its version; Please manually update or remove it." >&2
            return 1
        fi
    fi

    url="$(resolve_buf_download_url)"

    mkdir -p "$(dirname "${BUF_BIN}")"
    tmp_buf="${BUF_BIN}.tmp.$$"

    echo "Installing buf v${BUF_VERSION} to ${BUF_BIN}" >&2
    if ! curl -sSfL --retry "${BUF_CURL_RETRY}" --retry-delay 1 "${url}" -o "${tmp_buf}"; then
        echo "Failed to download buf from: ${url}" >&2
        rm -f "${tmp_buf}"
        return 1
    fi

    chmod +x "${tmp_buf}"
    mv "${tmp_buf}" "${BUF_BIN}"
}

run_format() {
    local -a cmd
    local include_proto

    cmd=("${BUF_BIN}" format . --path proto)
    # Only include top-level include/*.proto here. Nested include/google and
    # include/gogoproto are vendored third-party protos.
    for include_proto in include/*.proto; do
        [ -e "${include_proto}" ] || continue
        cmd+=(--path "${include_proto}")
    done

    "${cmd[@]}" "$@"
}

run_check() {
    if ! run_format --diff --exit-code; then
        echo >&2
        echo "Proto files are not formatted. Run: make proto-fmt" >&2
        return 1
    fi
}

run_write() {
    run_format -w
}

main() {
    if [ $# -ne 1 ]; then
        usage >&2
        return 2
    fi

    cd "${KVPROTO_ROOT}"
    install_buf

    case "$1" in
        --check) run_check ;;
        --write) run_write ;;
        *)
            usage >&2
            return 2
            ;;
    esac
}

main "$@"
