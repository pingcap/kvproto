#![cfg(all(feature = "protobuf-codec", not(feature = "prost-codec")))]

use kvproto::brpb::{BackupRequest, Bucket, CloudDynamic, StorageBackend};
use kvproto::import_sstpb::DownloadRequest;
use protobuf::{Message, PbPrint};
use std::fmt::Debug;

const SECRETS: [&str; 3] = [
    "cloud-access-test-secret",
    "cloud-token-test-secret",
    "cloud-extension-test-secret",
];

fn cloud_dynamic() -> CloudDynamic {
    let mut bucket = Bucket::new();
    bucket.set_bucket("visible-bucket".to_owned());
    bucket.set_endpoint("visible-endpoint".to_owned());
    bucket.set_region("visible-region".to_owned());
    bucket.set_prefix("visible-prefix".to_owned());
    bucket.set_storage_class("visible-storage-class".to_owned());

    let mut cloud = CloudDynamic::new();
    cloud.set_bucket(bucket);
    cloud.set_provider_name("visible-provider".to_owned());
    for (key, value) in [
        ("access_key", SECRETS[0]),
        ("session_token", SECRETS[1]),
        ("extension_option", SECRETS[2]),
        ("empty_option", ""),
    ] {
        cloud.mut_attrs().insert(key.to_owned(), value.to_owned());
    }
    cloud
}

fn formats<T: Debug + PbPrint>(message: &T) -> Vec<String> {
    let mut named = String::new();
    PbPrint::fmt(message, "message", &mut named);
    let mut unnamed = String::new();
    PbPrint::fmt(message, "", &mut unnamed);
    vec![
        format!("{:?}", message),
        format!("{:#?}", message),
        named,
        unnamed,
    ]
}

fn assert_redacted<T: Message + Clone + PartialEq + Debug + PbPrint>(message: &T) {
    let original = message.clone();
    let encoded = message.write_to_bytes().unwrap();
    for output in formats(message) {
        for secret in SECRETS {
            assert!(!output.contains(secret), "credential leaked: {}", output);
        }
        for visible in [
            "visible-bucket",
            "visible-endpoint",
            "visible-region",
            "visible-prefix",
            "visible-storage-class",
            "visible-provider",
            "access_key",
            "session_token",
            "extension_option",
            "empty_option",
        ] {
            assert!(output.contains(visible), "missing field: {}", visible);
        }
        assert_eq!(output.matches("[REDACTED]").count(), SECRETS.len());
        let empty_value = output
            .split_once("\"empty_option\":")
            .unwrap()
            .1
            .trim_start();
        assert!(
            empty_value.starts_with(']')
                || [
                    "\"access_key\":",
                    "\"session_token\":",
                    "\"extension_option\":"
                ]
                .iter()
                .any(|key| empty_value.starts_with(*key)),
            "empty attribute value changed: {}",
            output
        );
    }
    assert_eq!(message, &original);
    assert_eq!(message.write_to_bytes().unwrap(), encoded);
    assert_eq!(protobuf::parse_from_bytes::<T>(&encoded).unwrap(), original);
}

#[test]
fn direct_cloud_dynamic_redacts_without_mutating_data() {
    let cloud = cloud_dynamic();
    assert_redacted(&cloud);
    assert_eq!(cloud.get_attrs().get("access_key").unwrap(), SECRETS[0]);
    assert_eq!(cloud.get_attrs().get("session_token").unwrap(), SECRETS[1]);
    assert_eq!(
        cloud.get_attrs().get("extension_option").unwrap(),
        SECRETS[2]
    );
    assert_eq!(cloud.get_attrs().get("empty_option").unwrap(), "");
}

#[test]
fn nested_cloud_dynamic_redacts_without_mutating_data() {
    let mut backend = StorageBackend::new();
    backend.set_cloud_dynamic(cloud_dynamic());
    assert_redacted(&backend);

    let mut backup = BackupRequest::new();
    backup.set_storage_backend(backend.clone());
    assert_redacted(&backup);

    let mut download = DownloadRequest::new();
    download.set_storage_backend(backend);
    assert_redacted(&download);
}

#[test]
fn empty_attrs_preserve_original_printing() {
    for attrs in [
        std::collections::HashMap::new(),
        [("empty_option".to_owned(), String::new())]
            .iter()
            .cloned()
            .collect(),
        [
            ("first_empty".to_owned(), String::new()),
            ("second_empty".to_owned(), String::new()),
        ]
        .iter()
        .cloned()
        .collect(),
    ] {
        let mut cloud = cloud_dynamic();
        cloud.set_attrs(attrs);
        let mut expected = String::new();
        PbPrint::fmt(&cloud.bucket, "bucket", &mut expected);
        PbPrint::fmt(&cloud.provider_name, "provider_name", &mut expected);
        PbPrint::fmt(&cloud.attrs, "attrs", &mut expected);
        assert_eq!(format!("{:?}", cloud), expected);
        assert_eq!(format!("{:#?}", cloud), expected);

        let mut named = String::new();
        PbPrint::fmt(&cloud, "message", &mut named);
        let mut expected_named = String::new();
        protobuf::push_message_start("message", &mut expected_named);
        if !expected.is_empty() {
            expected_named.push(' ');
        }
        expected_named.push_str(&expected);
        if !expected.is_empty() {
            expected_named.push(' ');
        }
        expected_named.push('}');
        assert_eq!(named, expected_named);
    }

    let empty = CloudDynamic::new();
    assert_eq!(format!("{:?}", empty), "");
    let mut output = String::new();
    PbPrint::fmt(&empty, "message", &mut output);
    assert_eq!(output, "message {}");
}
