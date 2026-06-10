# kvproto

Protocol buffer files for TiKV

# Usage

+ Write your own protocol file in proto folder.
+ If you need to update raft-rs, please download the proto file
    respectively and overwrite the one in include folder.
+ Run `make` to generate go and rust code.
    We generate all go codes in pkg folder and rust in src folder.
+ Update the dependent projects.

# Dependencies

* Rust
* Go
* Protoc 3.8.0

# Proto formatting

To avoid IDE-induced diffs, we use `buf format` to keep all `.proto` files consistently formatted.

* Check formatting (also runs as part of `make check`): `make proto-fmt-check`
* Format in-place: `make proto-fmt`

The formatter is pinned and will be downloaded automatically into `./bin` (gitignored) when needed.
If `make check` fails on formatting, run `make proto-fmt` and then rerun `make check`.

# Docker image

The easiest way to compile the protobufs is to use docker.

```
./scripts/docker-build.sh  # build the docker image once
./scripts/docker-run.sh make go
```

# Build locally

There are many dependencies to be installed, please refer to the relevant scripts in [workflow](.github/workflows) to set them up.

# BackupMeta Compatibility

If you change backup metadata wire schema in `proto/brpb.proto` (for
`backup.BackupMeta` or messages reachable from it), increment
`BackupSchemaVersion` in
`pkg/brpb/backup_schema_version.go`.
