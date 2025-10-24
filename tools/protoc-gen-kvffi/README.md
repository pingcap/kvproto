# protoc-gen-kvffi

`protoc-gen-kvffi` is the generator that derives the C/Rust ABI header plus
the Go conversion helpers that bridge kvproto’s gogoproto output with the new
FFI runtime.

## Regenerating everything

After modifying any proto, run the helper script from the repository root:

```bash
./scripts/regenerate_kvffi.sh
```

The script builds the plugin into `_tools/bin/` and executes `protoc` across
`proto/*.proto` (plus the vendored `include/eraftpb.proto`). Afterwards it is a
good idea to make sure the tree still builds and the generated tests pass:

```bash
go build -tags kvffi_gen ./...
go test  -tags kvffi_gen ./...
```

### Regenerating a subset

Pass one or more proto paths (relative to the repo root or the `proto/`
directory) to regenerate only those files plus shared dependencies like the
ABI header:

```bash
./scripts/regenerate_kvffi.sh proto/kvrpcpb.proto storagepb.proto
```

## Manual invocation

If you prefer to run the commands yourself:

```bash
go build -o _tools/bin/protoc-gen-kvffi ./tools/protoc-gen-kvffi
PATH=_tools/bin:$PATH protoc -Iproto -Iinclude --kvffi_out=. include/eraftpb.proto proto/*.proto
```

The generator rewrites files under `ffi_out/`, so run it from the repository
root to keep paths consistent.
