extern crate prost_build;

use std::error::Error;
use std::io::{Result, Write};
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

fn main() {
    let (import_all_path, protos) = generate_import_all_proto().unwrap();

    compile_proto().unwrap();
    fs::remove_file(import_all_path).unwrap();

    generate_lib_rs(protos).unwrap();
}
