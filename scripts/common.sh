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
		function ltrim(s) {
			sub(/^[[:space:]]+/, "", s)
			return s
		}

		function rtrim(s) {
			sub(/[[:space:]]+$/, "", s)
			return s
		}

		function trim(s) {
			return rtrim(ltrim(s))
		}

		function is_comment_line(s) {
			return (s ~ /^[[:space:]]*\/\//) || (s ~ /^[[:space:]]*\/\*/) || (s ~ /^[[:space:]]*\*/) || (s ~ /^[[:space:]]*\*\//)
		}

		function is_option_line(s) {
			return !is_comment_line(s) && s !~ /^[[:space:]]*$/
		}

		function is_gogo_option_line(s,    t) {
			t = trim(s)
			return t ~ /^\(gogoproto\./
		}

		function set_trailing_comma(s, want_comma,    body, comment) {
			body = s
			comment = ""
			if (match(body, /\/\/.*/)) {
				comment = substr(body, RSTART)
				body = substr(body, 1, RSTART - 1)
			}
			body = rtrim(body)
			sub(/,$/, "", body)
			if (want_comma) {
				body = body ","
			}
			if (comment != "") {
				return body " " comment
			}
			return body
		}

		function reset_block() {
			in_block = 0
			block_open = ""
			block_close = ""
			block_len = 0
			delete block_lines
		}

		function flush_block(    i, kept_len, option_count, last_option_idx, out_line) {
			if (!in_block) return

			# Malformed proto: keep the original block.
			if (block_close == "") {
				print block_open
				for (i = 1; i <= block_len; i++) {
					print block_lines[i]
				}
				reset_block()
				return
			}

			delete kept_lines
			delete option_indexes
			kept_len = 0
			option_count = 0
			for (i = 1; i <= block_len; i++) {
				if (is_gogo_option_line(block_lines[i])) {
					continue
				}
				kept_lines[++kept_len] = block_lines[i]
				if (is_option_line(block_lines[i])) {
					option_indexes[++option_count] = kept_len
				}
			}

			if (option_count == 0) {
				out_line = block_open
				sub(/[[:space:]]*\[[[:space:]]*$/, ";", out_line)
				print out_line
			} else {
				last_option_idx = option_indexes[option_count]
				print block_open
				for (i = 1; i <= kept_len; i++) {
					out_line = kept_lines[i]
					if (is_option_line(out_line)) {
						out_line = set_trailing_comma(out_line, i != last_option_idx)
					}
					print out_line
				}
				print block_close
			}

			delete kept_lines
			delete option_indexes
			reset_block()
		}

		BEGIN {
			reset_block()
		}

		{
			line = $0

			# Ignore comment-only lines when looking for field option blocks.
			# Some protos include examples like:
			#   // [
			#   //   ...
			#   // ]
			# which must not be treated as an options block.
			is_comment = is_comment_line(line)

			# Drop import + file-level options for gogoproto.
			if (line ~ /^[[:space:]]*import[[:space:]]+"gogoproto\/gogo\.proto";[[:space:]]*$/) next
			if (line ~ /^[[:space:]]*option[[:space:]]*\(gogoproto\./) next

			# Inside a multi-line field options block.
			if (in_block) {
				if (line ~ /\][[:space:]]*;/) {
					block_close = line
					flush_block()
				} else {
					block_lines[++block_len] = line
				}
				next
			}

			# Single-line field options that contain gogoproto.
			# Example: repeated Foo bars = 1 [(gogoproto.nullable) = false];
			if (!is_comment && line ~ /\[[^\]]*gogoproto\.[^\]]*\]/) {
				open_idx = index(line, "[")
				rest = substr(line, open_idx + 1)
				close_rel = index(rest, "]")
				if (close_rel == 0) {
					print line
					next
				}

				prefix = substr(line, 1, open_idx - 1)
				inside = substr(rest, 1, close_rel - 1)
				suffix = substr(rest, close_rel + 1)

				delete tokens
				delete kept_tokens
				token_len = split(inside, tokens, ",")
				kept_token_len = 0
				for (i = 1; i <= token_len; i++) {
					token = trim(tokens[i])
					if (token == "") {
						continue
					}
					if (token ~ /^\(gogoproto\./) {
						continue
					}
					kept_tokens[++kept_token_len] = token
				}

				if (kept_token_len == 0) {
					prefix = rtrim(prefix)
					print prefix suffix
				} else {
					out_line = prefix "["
					for (i = 1; i <= kept_token_len; i++) {
						if (i > 1) {
							out_line = out_line ", "
						}
						out_line = out_line kept_tokens[i]
					}
					out_line = out_line "]" suffix
					print out_line
				}

				next
			}

			# Start of a multi-line field options block.
			if (!is_comment && line ~ /\[[[:space:]]*$/) {
				in_block = 1
				block_open = line
				block_close = ""
				block_len = 0
				delete block_lines
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
