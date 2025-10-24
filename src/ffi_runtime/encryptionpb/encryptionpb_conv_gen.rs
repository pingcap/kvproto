//! Auto-generated conversions (feature `kvffi_gen`).
#![cfg(feature = "kvffi_gen")]

use std::ptr;
use std::os::raw::c_char;

use protobuf::ProtobufEnum;
use crate::ffi_runtime::arena::{Arena, bytes_from, string_from};
use crate::ffi_runtime::abi::{EncryptionpbAwsKms, EncryptionpbAzureKms, EncryptionpbDataKey, EncryptionpbEncryptedContent, EncryptionpbEncryptedContentMetadataEntry, EncryptionpbEncryptionMeta, EncryptionpbFileDictionary, EncryptionpbFileDictionaryFilesEntry, EncryptionpbFileEncryptionInfo, EncryptionpbFileInfo, EncryptionpbGcpKms, EncryptionpbKeyDictionary, EncryptionpbKeyDictionaryKeysEntry, EncryptionpbMasterKey, EncryptionpbMasterKeyBased, EncryptionpbMasterKeyFile, EncryptionpbMasterKeyKms, EncryptionpbMasterKeyPlaintext, EncryptionpbPlainTextDataKey, KvprotoBytesView, KvprotoSliceEncryptionpbEncryptedContentMetadataEntryPtr, KvprotoSliceEncryptionpbEncryptedContentPtr, KvprotoSliceEncryptionpbFileDictionaryFilesEntryPtr, KvprotoSliceEncryptionpbKeyDictionaryKeysEntryPtr, KvprotoStringView};
use crate::encryptionpb as pb;

pub fn aws_kms_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::AwsKms) -> &'a mut EncryptionpbAwsKms {
    let mut repr = EncryptionpbAwsKms {
        access_key: KvprotoStringView { data: ptr::null(), len: 0 },
        secret_access_key: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_access_key().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_access_key());
        repr.access_key.data = ptr as *const c_char;
        repr.access_key.len = len;
    }
    if !src.get_secret_access_key().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_secret_access_key());
        repr.secret_access_key.data = ptr as *const c_char;
        repr.secret_access_key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn aws_kms_from_repr_generated(src: *const EncryptionpbAwsKms) -> Option<pb::AwsKms> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::AwsKms::new();
    out.set_access_key(string_from(repr.access_key.data as *const u8, repr.access_key.len));
    out.set_secret_access_key(string_from(repr.secret_access_key.data as *const u8, repr.secret_access_key.len));
    Some(out)
}

pub fn azure_kms_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::AzureKms) -> &'a mut EncryptionpbAzureKms {
    let mut repr = EncryptionpbAzureKms {
        tenant_id: KvprotoStringView { data: ptr::null(), len: 0 },
        client_id: KvprotoStringView { data: ptr::null(), len: 0 },
        client_secret: KvprotoStringView { data: ptr::null(), len: 0 },
        key_vault_url: KvprotoStringView { data: ptr::null(), len: 0 },
        hsm_name: KvprotoStringView { data: ptr::null(), len: 0 },
        hsm_url: KvprotoStringView { data: ptr::null(), len: 0 },
        client_certificate: KvprotoStringView { data: ptr::null(), len: 0 },
        client_certificate_path: KvprotoStringView { data: ptr::null(), len: 0 },
        client_certificate_password: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_tenant_id().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_tenant_id());
        repr.tenant_id.data = ptr as *const c_char;
        repr.tenant_id.len = len;
    }
    if !src.get_client_id().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_client_id());
        repr.client_id.data = ptr as *const c_char;
        repr.client_id.len = len;
    }
    if !src.get_client_secret().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_client_secret());
        repr.client_secret.data = ptr as *const c_char;
        repr.client_secret.len = len;
    }
    if !src.get_key_vault_url().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_key_vault_url());
        repr.key_vault_url.data = ptr as *const c_char;
        repr.key_vault_url.len = len;
    }
    if !src.get_hsm_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_hsm_name());
        repr.hsm_name.data = ptr as *const c_char;
        repr.hsm_name.len = len;
    }
    if !src.get_hsm_url().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_hsm_url());
        repr.hsm_url.data = ptr as *const c_char;
        repr.hsm_url.len = len;
    }
    if !src.get_client_certificate().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_client_certificate());
        repr.client_certificate.data = ptr as *const c_char;
        repr.client_certificate.len = len;
    }
    if !src.get_client_certificate_path().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_client_certificate_path());
        repr.client_certificate_path.data = ptr as *const c_char;
        repr.client_certificate_path.len = len;
    }
    if !src.get_client_certificate_password().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_client_certificate_password());
        repr.client_certificate_password.data = ptr as *const c_char;
        repr.client_certificate_password.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn azure_kms_from_repr_generated(src: *const EncryptionpbAzureKms) -> Option<pb::AzureKms> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::AzureKms::new();
    out.set_tenant_id(string_from(repr.tenant_id.data as *const u8, repr.tenant_id.len));
    out.set_client_id(string_from(repr.client_id.data as *const u8, repr.client_id.len));
    out.set_client_secret(string_from(repr.client_secret.data as *const u8, repr.client_secret.len));
    out.set_key_vault_url(string_from(repr.key_vault_url.data as *const u8, repr.key_vault_url.len));
    out.set_hsm_name(string_from(repr.hsm_name.data as *const u8, repr.hsm_name.len));
    out.set_hsm_url(string_from(repr.hsm_url.data as *const u8, repr.hsm_url.len));
    out.set_client_certificate(string_from(repr.client_certificate.data as *const u8, repr.client_certificate.len));
    out.set_client_certificate_path(string_from(repr.client_certificate_path.data as *const u8, repr.client_certificate_path.len));
    out.set_client_certificate_password(string_from(repr.client_certificate_password.data as *const u8, repr.client_certificate_password.len));
    Some(out)
}

pub fn data_key_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::DataKey) -> &'a mut EncryptionpbDataKey {
    let mut repr = EncryptionpbDataKey {
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        method: Default::default(),
        creation_time: Default::default(),
        was_exposed: Default::default(),
    };
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    repr.method = src.get_method() as i32;
    repr.creation_time = src.get_creation_time();
    repr.was_exposed = src.get_was_exposed();
    arena.alloc_struct(repr)
}

pub fn data_key_from_repr_generated(src: *const EncryptionpbDataKey) -> Option<pb::DataKey> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::DataKey::new();
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_method(pb::EncryptionMethod::from_i32(repr.method).unwrap_or_default());
    out.set_creation_time(repr.creation_time);
    out.set_was_exposed(repr.was_exposed);
    Some(out)
}

pub fn encrypted_content_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::EncryptedContent) -> &'a mut EncryptionpbEncryptedContent {
    let mut repr = EncryptionpbEncryptedContent {
        metadata: KvprotoSliceEncryptionpbEncryptedContentMetadataEntryPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        content: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        master_key: ptr::null_mut(),
        iv: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        ciphertext_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    {
        let map = src.get_metadata();
        if !map.is_empty() {
            let mut entries: Vec<*mut EncryptionpbEncryptedContentMetadataEntry> = Vec::with_capacity(map.len());
            for (key, value) in map.iter() {
                let key_view = if key.is_empty() {
                    KvprotoStringView { data: ptr::null(), len: 0 }
                } else {
                    let (ptr, len) = arena.alloc_string(key);
                    KvprotoStringView { data: ptr as *const c_char, len }
                };
                let value_view = if value.is_empty() {
                    KvprotoBytesView { data: ptr::null_mut(), len: 0 }
                } else {
                    let (ptr, len) = arena.alloc_bytes(value);
                    KvprotoBytesView { data: ptr, len }
                };
                let entry_ptr = arena.alloc_struct(EncryptionpbEncryptedContentMetadataEntry {
                    key: key_view,
                    value: value_view,
                }) as *mut EncryptionpbEncryptedContentMetadataEntry;
                entries.push(entry_ptr);
            }
            if !entries.is_empty() {
                let (ptr, len) = arena.alloc_vec(entries);
                repr.metadata.data = ptr;
                repr.metadata.len = len;
                repr.metadata.cap = len;
            }
        }
    }
    if !src.get_content().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_content());
        repr.content.data = ptr;
        repr.content.len = len;
    }
    if src.has_master_key() {
        repr.master_key = master_key_to_repr_generated(arena, src.get_master_key()) as *mut _;
    } else {
        repr.master_key = ptr::null_mut();
    }
    if !src.get_iv().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_iv());
        repr.iv.data = ptr;
        repr.iv.len = len;
    }
    if !src.get_ciphertext_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_ciphertext_key());
        repr.ciphertext_key.data = ptr;
        repr.ciphertext_key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn encrypted_content_from_repr_generated(src: *const EncryptionpbEncryptedContent) -> Option<pb::EncryptedContent> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::EncryptedContent::new();
    if !repr.metadata.data.is_null() && repr.metadata.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.metadata.data, repr.metadata.len) };
        let map = out.mut_metadata();
        map.clear();
        for &entry_ptr in slice {
            if entry_ptr.is_null() {
                continue;
            }
            let entry = unsafe { &*entry_ptr };
            let key = string_from(entry.key.data as *const u8, entry.key.len);
            let value = bytes_from(entry.value.data, entry.value.len);
            map.insert(key, value);
        }
    }
    out.set_content(bytes_from(repr.content.data, repr.content.len).into());
    if !repr.master_key.is_null() {
        if let Some(value) = master_key_from_repr_generated(repr.master_key) {
            out.set_master_key(value);
        }
    }
    out.set_iv(bytes_from(repr.iv.data, repr.iv.len).into());
    out.set_ciphertext_key(bytes_from(repr.ciphertext_key.data, repr.ciphertext_key.len).into());
    Some(out)
}

pub fn encryption_meta_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::EncryptionMeta) -> &'a mut EncryptionpbEncryptionMeta {
    let mut repr = EncryptionpbEncryptionMeta {
        key_id: Default::default(),
        iv: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    repr.key_id = src.get_key_id();
    if !src.get_iv().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_iv());
        repr.iv.data = ptr;
        repr.iv.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn encryption_meta_from_repr_generated(src: *const EncryptionpbEncryptionMeta) -> Option<pb::EncryptionMeta> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::EncryptionMeta::new();
    out.set_key_id(repr.key_id);
    out.set_iv(bytes_from(repr.iv.data, repr.iv.len).into());
    Some(out)
}

pub fn file_dictionary_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::FileDictionary) -> &'a mut EncryptionpbFileDictionary {
    let mut repr = EncryptionpbFileDictionary {
        files: KvprotoSliceEncryptionpbFileDictionaryFilesEntryPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let map = src.get_files();
        if !map.is_empty() {
            let mut entries: Vec<*mut EncryptionpbFileDictionaryFilesEntry> = Vec::with_capacity(map.len());
            for (key, value) in map.iter() {
                let key_view = if key.is_empty() {
                    KvprotoStringView { data: ptr::null(), len: 0 }
                } else {
                    let (ptr, len) = arena.alloc_string(key);
                    KvprotoStringView { data: ptr as *const c_char, len }
                };
                let value_ptr = file_info_to_repr_generated(arena, value) as *mut _;
                let entry_ptr = arena.alloc_struct(EncryptionpbFileDictionaryFilesEntry {
                    key: key_view,
                    value: value_ptr,
                }) as *mut EncryptionpbFileDictionaryFilesEntry;
                entries.push(entry_ptr);
            }
            if !entries.is_empty() {
                let (ptr, len) = arena.alloc_vec(entries);
                repr.files.data = ptr;
                repr.files.len = len;
                repr.files.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn file_dictionary_from_repr_generated(src: *const EncryptionpbFileDictionary) -> Option<pb::FileDictionary> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::FileDictionary::new();
    if !repr.files.data.is_null() && repr.files.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.files.data, repr.files.len) };
        let map = out.mut_files();
        map.clear();
        for &entry_ptr in slice {
            if entry_ptr.is_null() {
                continue;
            }
            let entry = unsafe { &*entry_ptr };
            let key = string_from(entry.key.data as *const u8, entry.key.len);
            let value = file_info_from_repr_generated(entry.value).unwrap_or_default();
            map.insert(key, value);
        }
    }
    Some(out)
}

pub fn file_encryption_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::FileEncryptionInfo) -> &'a mut EncryptionpbFileEncryptionInfo {
    let mut repr = EncryptionpbFileEncryptionInfo {
        mode_case: Default::default(),
        plain_text_data_key: ptr::null_mut(),
        master_key_based: ptr::null_mut(),
        encryption_method: Default::default(),
        file_iv: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        checksum: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    repr.encryption_method = src.get_encryption_method() as i32;
    if !src.get_file_iv().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_file_iv());
        repr.file_iv.data = ptr;
        repr.file_iv.len = len;
    }
    if !src.get_checksum().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_checksum());
        repr.checksum.data = ptr;
        repr.checksum.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn file_encryption_info_from_repr_generated(src: *const EncryptionpbFileEncryptionInfo) -> Option<pb::FileEncryptionInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::FileEncryptionInfo::new();
    out.set_encryption_method(pb::EncryptionMethod::from_i32(repr.encryption_method).unwrap_or_default());
    out.set_file_iv(bytes_from(repr.file_iv.data, repr.file_iv.len).into());
    out.set_checksum(bytes_from(repr.checksum.data, repr.checksum.len).into());
    Some(out)
}

pub fn file_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::FileInfo) -> &'a mut EncryptionpbFileInfo {
    let mut repr = EncryptionpbFileInfo {
        key_id: Default::default(),
        iv: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        method: Default::default(),
    };
    repr.key_id = src.get_key_id();
    if !src.get_iv().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_iv());
        repr.iv.data = ptr;
        repr.iv.len = len;
    }
    repr.method = src.get_method() as i32;
    arena.alloc_struct(repr)
}

pub fn file_info_from_repr_generated(src: *const EncryptionpbFileInfo) -> Option<pb::FileInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::FileInfo::new();
    out.set_key_id(repr.key_id);
    out.set_iv(bytes_from(repr.iv.data, repr.iv.len).into());
    out.set_method(pb::EncryptionMethod::from_i32(repr.method).unwrap_or_default());
    Some(out)
}

pub fn gcp_kms_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GcpKms) -> &'a mut EncryptionpbGcpKms {
    let mut repr = EncryptionpbGcpKms {
        credential: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_credential().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_credential());
        repr.credential.data = ptr as *const c_char;
        repr.credential.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn gcp_kms_from_repr_generated(src: *const EncryptionpbGcpKms) -> Option<pb::GcpKms> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GcpKms::new();
    out.set_credential(string_from(repr.credential.data as *const u8, repr.credential.len));
    Some(out)
}

pub fn key_dictionary_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::KeyDictionary) -> &'a mut EncryptionpbKeyDictionary {
    let mut repr = EncryptionpbKeyDictionary {
        keys: KvprotoSliceEncryptionpbKeyDictionaryKeysEntryPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        current_key_id: Default::default(),
    };
    {
        let map = src.get_keys();
        if !map.is_empty() {
            let mut entries: Vec<*mut EncryptionpbKeyDictionaryKeysEntry> = Vec::with_capacity(map.len());
            for (key, value) in map.iter() {
                let value_ptr = data_key_to_repr_generated(arena, value) as *mut _;
                let entry_ptr = arena.alloc_struct(EncryptionpbKeyDictionaryKeysEntry {
                    key: *key,
                    value: value_ptr,
                }) as *mut EncryptionpbKeyDictionaryKeysEntry;
                entries.push(entry_ptr);
            }
            if !entries.is_empty() {
                let (ptr, len) = arena.alloc_vec(entries);
                repr.keys.data = ptr;
                repr.keys.len = len;
                repr.keys.cap = len;
            }
        }
    }
    repr.current_key_id = src.get_current_key_id();
    arena.alloc_struct(repr)
}

pub fn key_dictionary_from_repr_generated(src: *const EncryptionpbKeyDictionary) -> Option<pb::KeyDictionary> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::KeyDictionary::new();
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let map = out.mut_keys();
        map.clear();
        for &entry_ptr in slice {
            if entry_ptr.is_null() {
                continue;
            }
            let entry = unsafe { &*entry_ptr };
            let key = entry.key;
            let value = data_key_from_repr_generated(entry.value).unwrap_or_default();
            map.insert(key, value);
        }
    }
    out.set_current_key_id(repr.current_key_id);
    Some(out)
}

pub fn master_key_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MasterKey) -> &'a mut EncryptionpbMasterKey {
    let mut repr = EncryptionpbMasterKey {
        backend_case: Default::default(),
        plaintext: ptr::null_mut(),
        file: ptr::null_mut(),
        kms: ptr::null_mut(),
    };
    arena.alloc_struct(repr)
}

pub fn master_key_from_repr_generated(src: *const EncryptionpbMasterKey) -> Option<pb::MasterKey> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MasterKey::new();
    Some(out)
}

pub fn master_key_based_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MasterKeyBased) -> &'a mut EncryptionpbMasterKeyBased {
    let mut repr = EncryptionpbMasterKeyBased {
        data_key_encrypted_content: KvprotoSliceEncryptionpbEncryptedContentPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_data_key_encrypted_content();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut EncryptionpbEncryptedContent> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(encrypted_content_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.data_key_encrypted_content.data = ptr;
                repr.data_key_encrypted_content.len = len;
                repr.data_key_encrypted_content.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn master_key_based_from_repr_generated(src: *const EncryptionpbMasterKeyBased) -> Option<pb::MasterKeyBased> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MasterKeyBased::new();
    if !repr.data_key_encrypted_content.data.is_null() && repr.data_key_encrypted_content.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.data_key_encrypted_content.data, repr.data_key_encrypted_content.len) };
        let mut values: Vec<pb::EncryptedContent> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = encrypted_content_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_data_key_encrypted_content(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn master_key_file_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MasterKeyFile) -> &'a mut EncryptionpbMasterKeyFile {
    let mut repr = EncryptionpbMasterKeyFile {
        path: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_path().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_path());
        repr.path.data = ptr as *const c_char;
        repr.path.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn master_key_file_from_repr_generated(src: *const EncryptionpbMasterKeyFile) -> Option<pb::MasterKeyFile> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MasterKeyFile::new();
    out.set_path(string_from(repr.path.data as *const u8, repr.path.len));
    Some(out)
}

pub fn master_key_kms_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MasterKeyKms) -> &'a mut EncryptionpbMasterKeyKms {
    let mut repr = EncryptionpbMasterKeyKms {
        vendor: KvprotoStringView { data: ptr::null(), len: 0 },
        key_id: KvprotoStringView { data: ptr::null(), len: 0 },
        region: KvprotoStringView { data: ptr::null(), len: 0 },
        endpoint: KvprotoStringView { data: ptr::null(), len: 0 },
        azure_kms: ptr::null_mut(),
        gcp_kms: ptr::null_mut(),
        aws_kms: ptr::null_mut(),
    };
    if !src.get_vendor().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_vendor());
        repr.vendor.data = ptr as *const c_char;
        repr.vendor.len = len;
    }
    if !src.get_key_id().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_key_id());
        repr.key_id.data = ptr as *const c_char;
        repr.key_id.len = len;
    }
    if !src.get_region().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_region());
        repr.region.data = ptr as *const c_char;
        repr.region.len = len;
    }
    if !src.get_endpoint().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_endpoint());
        repr.endpoint.data = ptr as *const c_char;
        repr.endpoint.len = len;
    }
    if src.has_azure_kms() {
        repr.azure_kms = azure_kms_to_repr_generated(arena, src.get_azure_kms()) as *mut _;
    } else {
        repr.azure_kms = ptr::null_mut();
    }
    if src.has_gcp_kms() {
        repr.gcp_kms = gcp_kms_to_repr_generated(arena, src.get_gcp_kms()) as *mut _;
    } else {
        repr.gcp_kms = ptr::null_mut();
    }
    if src.has_aws_kms() {
        repr.aws_kms = aws_kms_to_repr_generated(arena, src.get_aws_kms()) as *mut _;
    } else {
        repr.aws_kms = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn master_key_kms_from_repr_generated(src: *const EncryptionpbMasterKeyKms) -> Option<pb::MasterKeyKms> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MasterKeyKms::new();
    out.set_vendor(string_from(repr.vendor.data as *const u8, repr.vendor.len));
    out.set_key_id(string_from(repr.key_id.data as *const u8, repr.key_id.len));
    out.set_region(string_from(repr.region.data as *const u8, repr.region.len));
    out.set_endpoint(string_from(repr.endpoint.data as *const u8, repr.endpoint.len));
    if !repr.azure_kms.is_null() {
        if let Some(value) = azure_kms_from_repr_generated(repr.azure_kms) {
            out.set_azure_kms(value);
        }
    }
    if !repr.gcp_kms.is_null() {
        if let Some(value) = gcp_kms_from_repr_generated(repr.gcp_kms) {
            out.set_gcp_kms(value);
        }
    }
    if !repr.aws_kms.is_null() {
        if let Some(value) = aws_kms_from_repr_generated(repr.aws_kms) {
            out.set_aws_kms(value);
        }
    }
    Some(out)
}

pub fn master_key_plaintext_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MasterKeyPlaintext) -> &'a mut EncryptionpbMasterKeyPlaintext {
    let mut repr = EncryptionpbMasterKeyPlaintext {
    };
    arena.alloc_struct(repr)
}

pub fn master_key_plaintext_from_repr_generated(src: *const EncryptionpbMasterKeyPlaintext) -> Option<pb::MasterKeyPlaintext> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MasterKeyPlaintext::new();
    Some(out)
}

pub fn plain_text_data_key_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PlainTextDataKey) -> &'a mut EncryptionpbPlainTextDataKey {
    let mut repr = EncryptionpbPlainTextDataKey {
    };
    arena.alloc_struct(repr)
}

pub fn plain_text_data_key_from_repr_generated(src: *const EncryptionpbPlainTextDataKey) -> Option<pb::PlainTextDataKey> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PlainTextDataKey::new();
    Some(out)
}

