// Copyright 2019 PingCAP, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// See the License for the specific language governing permissions and
// limitations under the License.

use protobuf_build::Builder;
use std::fs;

fn main() {
    let proto_dir = "proto";
    let mut proto_files = Vec::new();

    // Walk the proto directory and collect all .proto files.
    for entry in fs::read_dir(proto_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext == "proto" {
                    proto_files.push(path.to_string_lossy().into_owned());
                }
            }
        }
    }

    Builder::new()
        .files(&proto_files)
        .append_to_black_list("eraftpb")
        .generate()
}
