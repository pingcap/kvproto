#!/usr/bin/env bash

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
BIN_DIR="${REPO_ROOT}/_tools/bin"

mkdir -p "${BIN_DIR}"

echo ">> building protoc-gen-kvffi"
(
	cd "${REPO_ROOT}"
	go build -o "${BIN_DIR}/protoc-gen-kvffi" ./tools/protoc-gen-kvffi
)

declare -a proto_files=()

if [[ $# -gt 0 ]]; then
	echo ">> using whitelist of proto files"
	for arg in "$@"; do
		if [[ -f "${REPO_ROOT}/${arg}" ]]; then
			proto_files+=("${arg}")
		elif [[ -f "${REPO_ROOT}/proto/${arg}" ]]; then
			proto_files+=("proto/${arg}")
		else
			echo "requested proto '${arg}' not found" >&2
			exit 1
		fi
	done
else
	echo ">> collecting proto files"
	while IFS= read -r -d '' file; do
		proto_files+=("${file}")
	done < <(
		cd "${REPO_ROOT}"
		find proto -name '*.proto' -print0 | sort -z
	)

	if [[ ${#proto_files[@]} -eq 0 ]]; then
		echo "no proto files found under ${REPO_ROOT}/proto" >&2
		exit 1
	fi
fi

whitelist_mode=false
if [[ $# -gt 0 ]]; then
	whitelist_mode=true
fi

declare -a requested_modules=()
for file in "${proto_files[@]}"; do
	base="$(basename "${file}")"
	module="${base%.proto}"
	requested_modules+=("${module}")
done
if [[ ${#requested_modules[@]} -gt 0 ]]; then
	mapfile -t requested_modules < <(printf '%s\n' "${requested_modules[@]}" | sort -u)
fi
declare -A requested_set=()
for module in "${requested_modules[@]}"; do
	requested_set["$module"]=1
done

echo ">> running protoc"
(
	cd "${REPO_ROOT}"
	PATH="${BIN_DIR}:${PATH}" \
		protoc -Iproto -Iinclude --kvffi_out=. \
		include/eraftpb.proto \
		"${proto_files[@]}"
)

echo ">> ensuring Rust module stubs"
declare -a rust_modules=()
while IFS= read -r -d '' conv_file; do
	dirname="$(dirname "${conv_file}")"
	pkg_name="$(basename "${dirname}")"
	if [[ ${whitelist_mode} == true && -z ${requested_set[${pkg_name}]+x} ]]; then
		continue
	fi
	rust_modules+=("${pkg_name}")
	mod_file="${dirname}/mod.rs"
	top_level="${REPO_ROOT}/src/ffi_runtime/${pkg_name}.rs"
	if [[ -f "${mod_file}" || -f "${top_level}" ]]; then
		continue
	fi
	base="$(basename "${conv_file}")"
	stem="${base%.rs}"
	cat <<EOF > "${mod_file}"
#[cfg(feature = "kvffi_gen")]
#[path = "${base}"]
mod ${stem};
#[cfg(feature = "kvffi_gen")]
pub use ${stem}::*;
EOF
done < <(find "${REPO_ROOT}/src/ffi_runtime" -name '*_conv_gen.rs' -print0)

if [[ ${#rust_modules[@]} -gt 0 || ${#requested_modules[@]} -gt 0 ]]; then
	if [[ ${#rust_modules[@]} -gt 0 ]]; then
		mapfile -t rust_modules < <(printf '%s\n' "${rust_modules[@]}" | sort -u)
	fi
	mod_rs="${REPO_ROOT}/src/ffi_runtime/mod.rs"
	cat <<'EOF' > "${mod_rs}"
#[path = "../../ffi_out/rust/kvproto_abi.rs"]
pub mod abi;

pub mod arena;
EOF
	declare -a modules_for_mod=()
	if [[ ${whitelist_mode} == true && ${#requested_modules[@]} -gt 0 ]]; then
		modules_for_mod=("${requested_modules[@]}")
	else
		modules_for_mod=("${rust_modules[@]}")
	fi
	for module in "${modules_for_mod[@]}"; do
		if [[ -f "${REPO_ROOT}/src/ffi_runtime/${module}.rs" || -d "${REPO_ROOT}/src/ffi_runtime/${module}" ]]; then
			printf 'pub mod %s;\n' "${module}" >> "${mod_rs}"
		fi
	done
	cat <<'EOF' >> "${mod_rs}"

#[cfg(test)]
mod tests;
EOF
fi

cat <<'EOF'

Generation complete.

It is recommended to validate the build afterwards:

    go build -tags kvffi_gen ./...
    go test  -tags kvffi_gen ./...
    cargo test --features kvffi_gen

EOF
