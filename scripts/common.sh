#!/usr/bin/env bash

function push() {
    pushd $1 >/dev/null 2>&1
}

function pop() {
    popd >/dev/null 2>&1
}

function sed_inplace()
{
	# bsd sed does not support --version.
	if `sed --version > /dev/null 2>&1`; then
		sed -i "$@"
	else
		sed -i '' "$@"
	fi
}

function clean_gogo_proto()
{
	local file=$1
	# C++ codegen doesn't need gogoproto options, and we intentionally do not
	# compile/include gogoproto/gogo.proto in the C++ build.
	#
	# buf format may break field options into multi-line blocks like:
	#   bytes data = 1 [
	#     (gogoproto.customtype) = "...",
	#     (gogoproto.nullable) = false
	#   ];
	# so the old one-line sed approach is not sufficient.
	awk '
		function flush_block(    i, first) {
			if (!in_block) return
			if (block_has_gogo) {
				first = block_lines[1]
				sub(/[[:space:]]*\[[[:space:]]*$/, ";", first)
				print first
			} else {
				for (i = 1; i <= block_len; i++) {
					print block_lines[i]
				}
			}
			in_block = 0
			block_has_gogo = 0
			block_len = 0
			delete block_lines
		}

		BEGIN {
			in_block = 0
			block_has_gogo = 0
			block_len = 0
		}

		{
			line = $0

			# Ignore comment-only lines when looking for field option blocks.
			# Some protos include examples like:
			#   // [
			#   //   ...
			#   // ]
			# which must not be treated as an options block.
			is_comment = (line ~ /^[[:space:]]*\/\//) || (line ~ /^[[:space:]]*\/\*/) || (line ~ /^[[:space:]]*\*/)

			# Drop import + file-level options for gogoproto.
			if (line ~ /^[[:space:]]*import[[:space:]]+"gogoproto\/gogo\.proto";[[:space:]]*$/) next
			if (line ~ /^[[:space:]]*option[[:space:]]*\(gogoproto\./) next

			# Inside a multi-line field options block.
			if (in_block) {
				block_lines[++block_len] = line
				if (line ~ /gogoproto\./) block_has_gogo = 1
				if (line ~ /\][[:space:]]*;/) {
					flush_block()
				}
				next
			}

			# Single-line field options that contain gogoproto.
			# Example: repeated Foo bars = 1 [(gogoproto.nullable) = false];
			if (!is_comment && line ~ /\[[^\]]*gogoproto\.[^\]]*\]/) {
				gsub(/[[:space:]]*\[[^\]]*gogoproto\.[^\]]*\]/, "", line)
				print line
				next
			}

			# Start of a multi-line field options block.
			if (!is_comment && line ~ /\[[[:space:]]*$/) {
				in_block = 1
				block_len = 1
				block_lines[1] = line
				block_has_gogo = 0
				next
			}

			print line
		}

		END {
			flush_block()
		}
	' "${file}" > "${file}.tmp" && mv "${file}.tmp" "${file}"
}
export -f clean_gogo_proto
