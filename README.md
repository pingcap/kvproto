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

# Docker image

The easiest way to compile the protobufs is to use docker.

```
./scripts/docker-build.sh  # build the docker image once
./scripts/docker-run.sh make go
```

# Build locally

There are many dependencies to be installed, please refer to the relevant scripts in [workflow](.github/workflows) to set them up.
