#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

echo ">> removing generated ABI artifacts"
rm -f "${REPO_ROOT}/ffi_out/c/kvproto_abi.h"
rm -f "${REPO_ROOT}/ffi_out/rust/kvproto_abi.rs"
find "${REPO_ROOT}/ffi_out" -name '*_abi.go' -exec rm -f {} +

echo ">> removing generated Go conversion helpers"
find "${REPO_ROOT}/ffi_out" -name '*_conv_gen.go' -exec rm -f {} +

echo ">> removing generated Rust conversion files"
find "${REPO_ROOT}/src/ffi_runtime" -name '*_conv_gen.rs' -exec rm -f {} +

echo ">> removing generated Rust module stubs"
find "${REPO_ROOT}/src/ffi_runtime" \
	-mindepth 2 -maxdepth 2 \
	-name 'mod.rs' \
	-not -path "${REPO_ROOT}/src/ffi_runtime/tests/mod.rs" \
	-exec rm -f {} +

echo ">> pruning empty directories"
find "${REPO_ROOT}/src/ffi_runtime" -type d -empty -delete

cat <<'EOF'

Cleanup complete.

Note: module files with legacy fallback code are preserved. If additional
generated artifacts remain tracked, use your VCS to restore them.

EOF
