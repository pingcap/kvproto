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

use kvproto_build::*;
use std::env;
use std::fs::read_dir;
use std::fs::File;
use std::io::Write;

fn main() {
    // This build script creates files in the `src` directory. Since that is
    // outside Cargo's OUT_DIR it will cause an error when this crate is used
    // as a dependency. Therefore, the user must opt-in to regenerating the
    // Rust files.
    if env::var_os("CARGO_FEATURE_REGENERATE").is_none() {
        println!("cargo:rerun-if-changed=build.rs");
        return;
    }

    check_protoc_version();

    let file_names: Vec<_> = read_dir("proto")
        .expect("Couldn't read proto directory")
        .map(|e| {
            format!(
                "proto/{}",
                e.expect("Couldn't list file").file_name().to_string_lossy()
            )
        })
        .collect();
    let file_names: Vec<_> = file_names.iter().map(|s| &**s).collect();

    for f in &file_names {
        println!("cargo:rerun-if-changed={}", f);
    }

    match BufferLib::from_env_vars() {
        BufferLib::Protobuf => {
            generate_protobuf_files(&file_names, "src/protobuf");

            let mod_names = module_names_for_dir("src/protobuf");

            let out_file_names: Vec<_> = mod_names
                .iter()
                .map(|m| format!("src/protobuf/{}.rs", m))
                .collect();
            let out_file_names: Vec<_> = out_file_names.iter().map(|f| &**f).collect();
            replace_read_unknown_fields(&out_file_names);
            generate_protobuf_rs(&mod_names);
        }
        BufferLib::Prost => {
            unimplemented!("Prost support is not yet implemented");
        }
    }
}

#[derive(Eq, PartialEq)]
enum BufferLib {
    Prost,
    Protobuf,
}

impl BufferLib {
    fn from_env_vars() -> BufferLib {
        match (
            env::var_os("CARGO_FEATURE_LIB_PROST"),
            env::var_os("CARGO_FEATURE_LIB_RUST_PROTOBUF"),
        ) {
            (Some(_), Some(_)) | (None, None) => {
                panic!("You must use exactly one of `lib-rust-protobuf` and `prost-buf` features")
            }
            (Some(_), _) => BufferLib::Prost,
            (_, Some(_)) => BufferLib::Protobuf,
        }
    }
}

fn generate_protobuf_rs(mod_names: &[String]) {
    let mut text = "use raft::eraftpb;\n\n".to_owned();

    for mod_name in mod_names {
        text.push_str("pub mod ");
        text.push_str(mod_name);
        text.push_str(";\n");
    }

    let mut lib = File::create("src/protobuf.rs").expect("Could not create protobuf.rs");
    lib.write_all(text.as_bytes())
        .expect("Could not write protobuf.rs");
}
