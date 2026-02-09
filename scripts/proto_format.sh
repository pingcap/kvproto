#!/usr/bin/env bash
set -euo pipefail

SCRIPTS_DIR=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
KVPROTO_ROOT=$(cd "${SCRIPTS_DIR}/.." && pwd)

# Keep this pinned to ensure deterministic formatting across machines/CI.
BUF_VERSION="${BUF_VERSION:-1.34.0}"
BUF_BIN="${KVPROTO_ROOT}/bin/buf"

usage() {
    cat <<'EOF'
Usage:
  scripts/proto_format.sh --check   # verify proto files are formatted
  scripts/proto_format.sh --write   # format proto files in-place

This script downloads a pinned version of buf into ./bin (gitignored) if needed.
EOF
}

install_buf() {
    if [ -x "${BUF_BIN}" ]; then
        return 0
    fi

    mkdir -p "$(dirname "${BUF_BIN}")"

    local os arch url
    os="$(uname -s)"
    arch="$(uname -m)"

    # buf release assets use these names.
    case "${arch}" in
        amd64) arch="x86_64" ;;
        x86_64 | arm64 | aarch64) ;;
        *)
            echo "Unsupported architecture for buf: ${arch}" >&2
            return 1
            ;;
    esac

    case "${os}" in
        Linux | Darwin) ;;
        *)
            echo "Unsupported OS for buf: ${os}" >&2
            return 1
            ;;
    esac

    url="https://github.com/bufbuild/buf/releases/download/v${BUF_VERSION}/buf-${os}-${arch}"
    echo "Installing buf v${BUF_VERSION} to ${BUF_BIN}" >&2
    curl -sSfL "${url}" -o "${BUF_BIN}"
    chmod +x "${BUF_BIN}"
}

run_check() {
    if ! "${BUF_BIN}" format . \
        --path proto \
        --path include/eraftpb.proto \
        --path include/rustproto.proto \
        --diff \
        --exit-code; then
        echo >&2
        echo "Proto files are not formatted. Run: make proto-fmt" >&2
        return 1
    fi
}

run_write() {
    "${BUF_BIN}" format . \
        --path proto \
        --path include/eraftpb.proto \
        --path include/rustproto.proto \
        -w
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
