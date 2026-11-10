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
        .generate();

    #[cfg(all(feature = "protobuf-codec", not(feature = "prost-codec")))]
    redact_sensitive_fields();
}

#[cfg(all(feature = "protobuf-codec", not(feature = "prost-codec")))]
fn redact_sensitive_fields() {
    let output = std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap()).join("protos");
    for (module, messages) in [
        (
            "brpb",
            &[
                (
                    "S3",
                    &["access_key", "secret_access_key", "session_token"][..],
                ),
                ("Gcs", &["credentials_blob"][..]),
                ("AzureBlobStorage", &["shared_key", "access_sig"][..]),
                ("AzureCustomerKey", &["encryption_key"][..]),
                ("CipherInfo", &["cipher_key"][..]),
                ("CloudDynamic", &["attrs"][..]),
            ][..],
        ),
        (
            "encryptionpb",
            &[
                ("AwsKms", &["access_key", "secret_access_key"][..]),
                (
                    "AzureKms",
                    &[
                        "client_secret",
                        "client_certificate",
                        "client_certificate_password",
                    ][..],
                ),
                ("GcpKms", &["credential"][..]),
                ("DataKey", &["key"][..]),
                ("EncryptedContent", &["content"][..]),
            ][..],
        ),
    ] {
        let path = output.join(format!("{}.rs", module));
        let mut source = fs::read_to_string(&path).unwrap();
        for (message, fields) in messages {
            for (trait_name, buffer) in [
                ("::protobuf::PbPrint", "buf"),
                ("::std::fmt::Debug", "&mut s"),
            ] {
                let implementation = format!("impl {} for {} {{", trait_name, message);
                assert_eq!(
                    source.matches(&implementation).count(),
                    1,
                    "expected one generated formatting implementation for {}",
                    message
                );
                let start = source.find(&implementation).unwrap();
                let end = start + source[start..].find("\n}\n").unwrap();
                let mut block = source[start..end].to_owned();
                for field in *fields {
                    let original = format!(
                        "::protobuf::PbPrint::fmt(&self.{0}, \"{0}\", {1});",
                        field, buffer
                    );
                    assert_eq!(
                        block.matches(&original).count(),
                        1,
                        "unexpected generated formatting for {}.{}",
                        message,
                        field
                    );
                    let helper = if *message == "CloudDynamic" {
                        "cloud_dynamic_attrs"
                    } else {
                        "secret"
                    };
                    let replacement = format!(
                        "crate::logredact::{0}(&self.{1}, \"{1}\", {2});",
                        helper, field, buffer
                    );
                    block = block.replacen(&original, &replacement, 1);
                }
                source.replace_range(start..end, &block);
            }
        }
        fs::write(path, source).unwrap();
    }
}
