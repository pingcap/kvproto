# kvproto
Protocol buffer files for TiKV

# Dependencies

* Rust
* Go
* Protoc 3.8.0
* This repo correctly placed in your `$GOPATH`.

# Usage

+ Write your own protocol file in proto folder.
+ If you need to update raft-rs, please download the proto file
    respectively and overwrite the one in include folder.
+ Run `make` to generate go and rust code.
    We generate all go codes in pkg folder and rust in src folder.
+ Update the dependent projects.

# Multiple `protoc` Versions

If you need to override your version of `protoc` because you have a later version you can install the correct version like so:

```bash
PROTOC_VERSION=3.8.0
curl -L https://github.com/google/protobuf/releases/download/v$PROTOC_VERSION/protoc-$PROTOC_VERSION-linux-x86_64.zip -o protoc.zip &&\
unzip protoc.zip -d protoc &&\
rm protoc.zip
```

Then you can run `PATH="$(pwd)/protoc/bin:$PATH" make`

