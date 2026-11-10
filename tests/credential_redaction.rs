#![cfg(all(feature = "protobuf-codec", not(feature = "prost-codec")))]

use kvproto::brpb::{
    AzureBlobStorage, AzureCustomerKey, BackupRequest, CipherInfo, Gcs, StorageBackend, S3,
};
use kvproto::encryptionpb::{
    AwsKms, AzureKms, DataKey, EncryptedContent, GcpKms, KeyDictionary, MasterKey, MasterKeyKms,
};
use kvproto::import_sstpb::DownloadRequest;
use protobuf::{Message, PbPrint};
use std::fmt::Debug;

const SECRET: &str = "synthetic-credential-never-log";

fn outputs<T: Debug + PbPrint>(message: &T) -> Vec<String> {
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

fn assert_safe<T: Message + Clone + PartialEq + Debug + PbPrint>(
    message: &T,
    fields: &[&str],
    visible: &[&str],
) {
    let original = message.clone();
    let encoded = message.write_to_bytes().unwrap();
    let secret_hex: String = SECRET
        .as_bytes()
        .iter()
        .map(|byte| format!("{:02X}", byte))
        .collect();
    for output in outputs(message) {
        assert!(!output.contains(SECRET), "secret leaked: {}", output);
        assert!(
            !output.contains(&secret_hex),
            "key bytes leaked: {}",
            output
        );
        assert_eq!(
            output.matches("[REDACTED]").count(),
            fields.len(),
            "{}",
            output
        );
        for field in fields {
            assert!(
                output.contains(&format!("{}: \"[REDACTED]\"", field)),
                "field not redacted: {} in {}",
                field,
                output
            );
        }
        for value in visible {
            assert!(output.contains(value), "missing visible value: {}", value);
        }
    }
    assert_eq!(message, &original);
    assert_eq!(message.write_to_bytes().unwrap(), encoded);
    assert_eq!(protobuf::parse_from_bytes::<T>(&encoded).unwrap(), original);
}

fn assert_empty<T: Message + Debug + PbPrint>(message: &T) {
    let mut output = String::new();
    PbPrint::fmt(message, "message", &mut output);
    assert_eq!(output, "message {}");
    assert_eq!(format!("{:?}", message), "");
    assert_eq!(format!("{:#?}", message), "");
    assert_eq!(message.write_to_bytes().unwrap(), Vec::<u8>::new());
}

fn assert_storage(backend: StorageBackend, fields: &[&str]) {
    assert_safe(&backend, fields, &["visible-bucket"]);
    let mut backup = BackupRequest::new();
    backup.set_storage_backend(backend.clone());
    assert_safe(&backup, fields, &["visible-bucket"]);
    let mut download = DownloadRequest::new();
    download.set_storage_backend(backend);
    assert_safe(&download, fields, &["visible-bucket"]);
}

#[test]
fn storage_credentials_redact_direct_and_nested_output() {
    let mut s3 = S3::new();
    s3.set_bucket("visible-bucket".to_owned());
    s3.set_endpoint("visible-endpoint".to_owned());
    s3.set_access_key(SECRET.to_owned());
    s3.set_secret_access_key(SECRET.to_owned());
    s3.set_session_token(SECRET.to_owned());
    let fields = ["access_key", "secret_access_key", "session_token"];
    assert_safe(&s3, &fields, &["visible-bucket", "visible-endpoint"]);
    let mut backend = StorageBackend::new();
    backend.set_s3(s3);
    assert_storage(backend, &fields);

    let mut gcs = Gcs::new();
    gcs.set_bucket("visible-bucket".to_owned());
    gcs.set_credentials_blob(SECRET.to_owned());
    assert_safe(&gcs, &["credentials_blob"], &["visible-bucket"]);
    let mut backend = StorageBackend::new();
    backend.set_gcs(gcs);
    assert_storage(backend, &["credentials_blob"]);

    let mut customer_key = AzureCustomerKey::new();
    customer_key.set_encryption_key(SECRET.to_owned());
    customer_key.set_encryption_key_sha256("visible-key-digest".to_owned());
    assert_safe(&customer_key, &["encryption_key"], &["visible-key-digest"]);
    let mut azure = AzureBlobStorage::new();
    azure.set_bucket("visible-bucket".to_owned());
    azure.set_shared_key(SECRET.to_owned());
    azure.set_access_sig(SECRET.to_owned());
    azure.set_encryption_key(customer_key);
    let fields = ["shared_key", "access_sig", "encryption_key"];
    assert_safe(&azure, &fields, &["visible-bucket", "visible-key-digest"]);
    let mut backend = StorageBackend::new();
    backend.set_azure_blob_storage(azure);
    assert_storage(backend, &fields);
}

fn assert_kms(kms: MasterKeyKms, fields: &[&str]) {
    assert_safe(&kms, fields, &["visible-kms-key"]);
    let mut master = MasterKey::new();
    master.set_kms(kms);
    assert_safe(&master, fields, &["visible-kms-key"]);
    let mut content = EncryptedContent::new();
    content.set_master_key(master);
    assert_safe(&content, fields, &["visible-kms-key"]);
}

#[test]
fn kms_credentials_redact_direct_and_nested_output() {
    let mut aws = AwsKms::new();
    aws.set_access_key(SECRET.to_owned());
    aws.set_secret_access_key(SECRET.to_owned());
    let fields = ["access_key", "secret_access_key"];
    assert_safe(&aws, &fields, &[]);
    let mut kms = MasterKeyKms::new();
    kms.set_key_id("visible-kms-key".to_owned());
    kms.set_aws_kms(aws);
    assert_kms(kms, &fields);

    let mut azure = AzureKms::new();
    azure.set_client_id("visible-client".to_owned());
    azure.set_client_secret(SECRET.to_owned());
    azure.set_client_certificate(SECRET.to_owned());
    azure.set_client_certificate_password(SECRET.to_owned());
    let fields = [
        "client_secret",
        "client_certificate",
        "client_certificate_password",
    ];
    assert_safe(&azure, &fields, &["visible-client"]);
    let mut kms = MasterKeyKms::new();
    kms.set_key_id("visible-kms-key".to_owned());
    kms.set_azure_kms(azure);
    assert_kms(kms, &fields);

    let mut gcp = GcpKms::new();
    gcp.set_credential(SECRET.to_owned());
    assert_safe(&gcp, &["credential"], &[]);
    let mut kms = MasterKeyKms::new();
    kms.set_key_id("visible-kms-key".to_owned());
    kms.set_gcp_kms(gcp);
    assert_kms(kms, &["credential"]);
}

#[test]
fn key_bytes_redact_without_enabling_global_redaction() {
    let mut key = DataKey::new();
    key.set_key(SECRET.as_bytes().to_vec());
    key.set_creation_time(123456789);
    assert_safe(&key, &["key"], &["123456789"]);
    let mut dictionary = KeyDictionary::new();
    dictionary.mut_keys().insert(7, key);
    assert_safe(&dictionary, &["key"], &["123456789"]);

    let mut cipher = CipherInfo::new();
    cipher.set_cipher_key(SECRET.as_bytes().to_vec());
    assert_safe(&cipher, &["cipher_key"], &[]);
    let mut backup = BackupRequest::new();
    backup.set_cipher_info(cipher.clone());
    assert_safe(&backup, &["cipher_key"], &[]);
    let mut download = DownloadRequest::new();
    download.set_cipher_info(cipher);
    assert_safe(&download, &["cipher_key"], &[]);

    let mut content = EncryptedContent::new();
    content.set_content(SECRET.as_bytes().to_vec());
    content.set_iv(b"visible-iv".to_vec());
    assert_safe(&content, &["content"], &["76697369626C652D6976"]);
}

#[test]
fn empty_credentials_preserve_original_output() {
    assert_empty(&S3::new());
    assert_empty(&Gcs::new());
    assert_empty(&AzureBlobStorage::new());
    assert_empty(&AzureCustomerKey::new());
    assert_empty(&CipherInfo::new());
    assert_empty(&AwsKms::new());
    assert_empty(&AzureKms::new());
    assert_empty(&GcpKms::new());
    assert_empty(&DataKey::new());
    assert_empty(&EncryptedContent::new());

    let mut s3 = S3::new();
    s3.set_bucket("visible-bucket".to_owned());
    assert_safe(&s3, &[], &["visible-bucket"]);
}

#[test]
fn mixed_empty_and_nonempty_credentials_preserve_empty_fields() {
    let mut azure = AzureKms::new();
    azure.set_client_secret(SECRET.to_owned());
    assert_safe(&azure, &["client_secret"], &[]);
    for output in outputs(&azure) {
        assert!(!output.contains("client_certificate"));
    }
}
