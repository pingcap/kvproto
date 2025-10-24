# FFI ABI Generator

This utility derives C/Rust/Go friendly structs from the existing protobuf
schema without touching the protobuf generation workflow. It reads the JSON
schema snapshot stored in `scripts/proto.lock`, so no additional protoc plugins
or Python protobuf packages are required.

## Usage

```bash
# Generate headers/modules/go aliases under ./ffi_out
go run ./tools/abi_gen --out ffi_out

# (The legacy `--go-package` flag is accepted for backward compatibility but ignored.)

# Optional flags:
#   --header-include Relative include path used by the Go files when importing the C header
```

The command produces:

- `ffi_out/c/kvproto_abi.h`: C structs annotated for `repr(C)` usage.
- `ffi_out/rust/kvproto_abi.rs`: Rust `#[repr(C)]` mirrors of the C structs.
- `ffi_out/go/<pkg>/<proto>_abi.go`: Go aliases grouped by proto package (mirroring the Go protobuf layout).

Pointers and slices follow a consistent pattern:

- Message fields become pointers (`Type *`) so that optional values and opaque
  external types (for example `eraftpb.Entry`) can be passed by address.
- Repeated fields map to `kvproto_slice_*` structs (`data`, `len`, `cap`). For
  repeated messages the `data` member is a pointer to pointers (`Type **`),
  which works for both local structs and opaque extern structs.
- `string` and `bytes` use view structs (`kvproto_string_view`,
  `kvproto_bytes_view`) containing pointer + length.

Because the generator uses `proto.lock`, make sure that file is up to date
before running the tool.
