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
extern crate prost_build;

use regex::Regex;
use std::fs::{File, read_dir, remove_file};
use std::process::Command;
use std::error::Error;
use std::io::{Read, Result, Write};
use std::path::PathBuf;
use std::{env, fs};

fn generate_lib_rs(protos: Vec<String>) -> Result<()> {
    let target: PathBuf = env::current_dir().map(|mut dir| {
        dir.push("src");
        dir.push("lib.rs");
        dir
    })?;
    let mut file = fs::File::create(target)?;
    file.write("extern crate futures;\n".as_bytes())?;
    file.write("extern crate grpcio;\n".as_bytes())?;
    file.write("extern crate bytes;\n".as_bytes())?;
    file.write("extern crate prost;\n".as_bytes())?;
    file.write("#[macro_use]\n".as_bytes())?;
    file.write("extern crate prost_derive;\n".as_bytes())?;
    file.write("extern crate raft;\n".as_bytes())?;
    file.write("use raft::eraftpb;\n".as_bytes())?;
    protos
        .into_iter()
        .map(|s| {
            let proto_name = s.trim_right_matches(".proto");
            file.write("pub mod ".as_bytes())?;
            file.write(proto_name.as_bytes())?;
            file.write(" { include!(concat!(env!(\"OUT_DIR\"), \"/".as_bytes())?;
            file.write(proto_name.as_bytes())?;
            file.write(".rs\")); }\n".as_bytes())?;
            Ok(())
        })
        .for_each(|x: Result<()>| x.unwrap());

    Ok(())
}

fn generate_import_all_proto() -> Result<(PathBuf, Vec<String>)> {
    let proto_dir = env::current_dir().map(|mut dir| {
        dir.push("proto");
        dir
    })?;
    let protos: Vec<_> = proto_dir
        .read_dir()
        .unwrap()
        .filter_map(|dir| {
            let entry = dir.unwrap();
            if entry.file_type().unwrap().is_file() {
                entry.file_name().to_str().map(|s| s.to_string())
            } else {
                None
            }
        })
        .collect();
    let mut import_all_path = proto_dir.clone();
    import_all_path.push("import_all.proto");
    let mut file = fs::File::create(import_all_path.clone())?;
    file.write("syntax = \"proto3\";\n".as_bytes())?;
    file.write("package this_file_is_supposed_to_be_empty;\n".as_bytes())?;
    protos
        .iter()
        .map(|name| {
            file.write("import \"".as_bytes())?;
            file.write(name.as_bytes())?;
            file.write("\";\n".as_bytes())?;
            Ok(())
        })
        .for_each(|x: Result<()>| x.unwrap());
    file.sync_all()?;
    Ok((import_all_path, protos))
}

fn compile_proto() -> Option<()> {
    let current_dir = env::current_dir().unwrap();
    let mut proto = current_dir.clone();
    proto.push("proto");
    let mut include = current_dir.clone();
    include.push("include");

    prost_build::compile_protos(
        &["proto/import_all.proto"],
        &[proto.to_str()?, include.to_str()?, current_dir.to_str()?],
    )
    .map_err(|err| {
        println!("{}", err.description());
        Err::<(), ()>(())
    })
    .unwrap();

    Some(())
}

fn prost() {
    let (import_all_path, protos) = generate_import_all_proto().unwrap();

    compile_proto().unwrap();
    fs::remove_file(import_all_path).unwrap();

    generate_lib_rs(protos).unwrap();
}

fn main() {
    check_protoc_version();
    let _ = remove_file("src/lib.rs");

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

    generate_rust_files(file_names);

    let mut mod_names: Vec<_> = read_dir("src")
        .expect("Couldn't read src directory")
        .filter_map(|e| {
            let file_name = e.expect("Couldn't list file").file_name();
            file_name
                .to_string_lossy()
                .split(".rs")
                .next()
                .map(|n| n.to_owned())
        })
        .collect();
    mod_names.sort();
    
    replace_read_unknown_fields(&mod_names);
    generate_lib_rs(&mod_names);
}

fn check_protoc_version() {
    let output = Command::new("bash")
        .arg("common.sh")
        .arg("check_protoc_version")
        .output()
        .expect("Could not execute `check_protoc_version`");
    if !output.status.success() {
        panic!(
            "Invalid version of protoc (required 3.1.x), or protoc not installed\n\nstdout:\n\n{}",
            String::from_utf8_lossy(&output.stdout)
        );
    }
}

fn generate_rust_files(file_names: Vec<&str>) {
    protoc_rust::run(protoc_rust::Args {
        out_dir: "src",
        input: &file_names,
        includes: &["proto", "include"],
        customize: protoc_rust::Customize {
            ..Default::default()
        },
    })
    .unwrap();

    protoc_grpcio::compile_grpc_protos(file_names, &["proto", "include"], "src").unwrap();
}

// Use the old way to read protobuf enums.
// FIXME: Remove this once stepancheg/rust-protobuf#233 is resolved.
fn replace_read_unknown_fields(mod_names: &[String]) {
    let regex =
        Regex::new(r"::protobuf::rt::read_proto3_enum_with_unknown_fields_into\(([^,]+), ([^,]+), &mut ([^,]+), [^\)]+\)\?").unwrap();
    for mod_name in mod_names {
        let file_name = &format!("src/{}.rs", mod_name);

        let mut text = String::new();
        {
            let mut f = File::open(file_name).unwrap();
            f.read_to_string(&mut text)
                .expect("Couldn't read source file");
        }

        let text = regex.replace_all(
            &text,
            "if $1 == ::protobuf::wire_format::WireTypeVarint {\
                $3 = $2.read_enum()?;\
             } else {\
                return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));\
             }",
        );
        let mut out = File::create(file_name).unwrap();
        out.write_all(text.as_bytes())
            .expect("Could not write source file");
    }    
}

fn generate_lib_rs(mod_names: &[String]) {
    let mut text = r"extern crate futures;
extern crate grpcio;
extern crate protobuf;
extern crate raft;

use raft::eraftpb;

"
    .to_owned();

    for mod_name in mod_names {
        text.push_str("pub mod ");
        text.push_str(mod_name);
        text.push_str(";\n");
    }

    let mut lib = File::create("src/lib.rs").expect("Could not create lib.rs");
    lib.write_all(text.as_bytes())
        .expect("Could not write lib.rs");
>>>>>>> master
}
