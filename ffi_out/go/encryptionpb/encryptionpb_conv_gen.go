//go:build kvffi_gen
// +build kvffi_gen

package encryptionpb

/*
#cgo CFLAGS: -I../../c
#include "kvproto_abi.h"
*/
import "C"

import (
	"unsafe"
	runtime "github.com/pingcap/kvproto/ffi_out/go/runtime"
	encryptionpbproto "github.com/pingcap/kvproto/pkg/encryptionpb"
)

func NewReprAwsKmsGenerated(arena *runtime.Arena, src *encryptionpbproto.AwsKms) *AwsKms {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*AwsKms)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_AwsKms)))
	IntoReprAwsKmsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprAwsKmsGenerated(arena *runtime.Arena, dst *AwsKms, src *encryptionpbproto.AwsKms) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetAccessKey()); length > 0 {
		dst.access_key.data = (*C.char)(data)
		dst.access_key.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetSecretAccessKey()); length > 0 {
		dst.secret_access_key.data = (*C.char)(data)
		dst.secret_access_key.len = C.size_t(length)
	}
}

func FromReprAwsKmsGenerated(src *AwsKms) *encryptionpbproto.AwsKms {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.AwsKms{}
	out.AccessKey = runtime.StringFrom(unsafe.Pointer(src.access_key.data), int(src.access_key.len))
	out.SecretAccessKey = runtime.StringFrom(unsafe.Pointer(src.secret_access_key.data), int(src.secret_access_key.len))
	return out
}

func NewReprAzureKmsGenerated(arena *runtime.Arena, src *encryptionpbproto.AzureKms) *AzureKms {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*AzureKms)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_AzureKms)))
	IntoReprAzureKmsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprAzureKmsGenerated(arena *runtime.Arena, dst *AzureKms, src *encryptionpbproto.AzureKms) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetTenantId()); length > 0 {
		dst.tenant_id.data = (*C.char)(data)
		dst.tenant_id.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetClientId()); length > 0 {
		dst.client_id.data = (*C.char)(data)
		dst.client_id.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetClientSecret()); length > 0 {
		dst.client_secret.data = (*C.char)(data)
		dst.client_secret.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetKeyVaultUrl()); length > 0 {
		dst.key_vault_url.data = (*C.char)(data)
		dst.key_vault_url.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetHsmName()); length > 0 {
		dst.hsm_name.data = (*C.char)(data)
		dst.hsm_name.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetHsmUrl()); length > 0 {
		dst.hsm_url.data = (*C.char)(data)
		dst.hsm_url.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetClientCertificate()); length > 0 {
		dst.client_certificate.data = (*C.char)(data)
		dst.client_certificate.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetClientCertificatePath()); length > 0 {
		dst.client_certificate_path.data = (*C.char)(data)
		dst.client_certificate_path.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetClientCertificatePassword()); length > 0 {
		dst.client_certificate_password.data = (*C.char)(data)
		dst.client_certificate_password.len = C.size_t(length)
	}
}

func FromReprAzureKmsGenerated(src *AzureKms) *encryptionpbproto.AzureKms {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.AzureKms{}
	out.TenantId = runtime.StringFrom(unsafe.Pointer(src.tenant_id.data), int(src.tenant_id.len))
	out.ClientId = runtime.StringFrom(unsafe.Pointer(src.client_id.data), int(src.client_id.len))
	out.ClientSecret = runtime.StringFrom(unsafe.Pointer(src.client_secret.data), int(src.client_secret.len))
	out.KeyVaultUrl = runtime.StringFrom(unsafe.Pointer(src.key_vault_url.data), int(src.key_vault_url.len))
	out.HsmName = runtime.StringFrom(unsafe.Pointer(src.hsm_name.data), int(src.hsm_name.len))
	out.HsmUrl = runtime.StringFrom(unsafe.Pointer(src.hsm_url.data), int(src.hsm_url.len))
	out.ClientCertificate = runtime.StringFrom(unsafe.Pointer(src.client_certificate.data), int(src.client_certificate.len))
	out.ClientCertificatePath = runtime.StringFrom(unsafe.Pointer(src.client_certificate_path.data), int(src.client_certificate_path.len))
	out.ClientCertificatePassword = runtime.StringFrom(unsafe.Pointer(src.client_certificate_password.data), int(src.client_certificate_password.len))
	return out
}

func NewReprDataKeyGenerated(arena *runtime.Arena, src *encryptionpbproto.DataKey) *DataKey {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DataKey)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_DataKey)))
	IntoReprDataKeyGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDataKeyGenerated(arena *runtime.Arena, dst *DataKey, src *encryptionpbproto.DataKey) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	dst.method = C.int32_t(int32(src.GetMethod()))
	dst.creation_time = C.uint64_t(src.GetCreationTime())
	dst.was_exposed = C.bool(src.GetWasExposed())
}

func FromReprDataKeyGenerated(src *DataKey) *encryptionpbproto.DataKey {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.DataKey{}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Method = encryptionpbproto.EncryptionMethod(int32(src.method))
	out.CreationTime = uint64(src.creation_time)
	out.WasExposed = bool(src.was_exposed)
	return out
}

func NewReprEncryptedContentGenerated(arena *runtime.Arena, src *encryptionpbproto.EncryptedContent) *EncryptedContent {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*EncryptedContent)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_EncryptedContent)))
	IntoReprEncryptedContentGenerated(arena, ptr, src)
	return ptr
}

func IntoReprEncryptedContentGenerated(arena *runtime.Arena, dst *EncryptedContent, src *encryptionpbproto.EncryptedContent) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetMetadata(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*EncryptedContent_MetadataEntry)(nil)))
		array := unsafe.Slice((**EncryptedContent_MetadataEntry)(ptr), len(values))
		idx := 0
		for key, value := range values {
			entry := (*EncryptedContent_MetadataEntry)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_EncryptedContent_MetadataEntry)))
			if data, length := arena.AllocString(key); length > 0 {
				entry.key.data = (*C.char)(data)
				entry.key.len = C.size_t(length)
			} else {
				entry.key.data = nil
				entry.key.len = 0
			}
			if data, length := arena.AllocBytes(value); length > 0 {
				entry.value.data = (*C.uint8_t)(data)
				entry.value.len = C.size_t(length)
			} else {
				entry.value.data = nil
				entry.value.len = 0
			}
			array[idx] = entry
			idx++
		}
		dst.metadata.data = (**EncryptedContent_MetadataEntry)(ptr)
		dst.metadata.len = C.size_t(len(values))
		dst.metadata.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocBytes(src.GetContent()); length > 0 {
		dst.content.data = (*C.uint8_t)(data)
		dst.content.len = C.size_t(length)
	}
	if value := src.GetMasterKey(); value != nil {
		dst.master_key = NewReprMasterKeyGenerated(arena, value)
	} else {
		dst.master_key = nil
	}
	if data, length := arena.AllocBytes(src.GetIv()); length > 0 {
		dst.iv.data = (*C.uint8_t)(data)
		dst.iv.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetCiphertextKey()); length > 0 {
		dst.ciphertext_key.data = (*C.uint8_t)(data)
		dst.ciphertext_key.len = C.size_t(length)
	}
}

func FromReprEncryptedContentGenerated(src *EncryptedContent) *encryptionpbproto.EncryptedContent {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.EncryptedContent{}
	if src.metadata.data != nil && src.metadata.len > 0 {
		length := int(src.metadata.len)
		ptrs := unsafe.Slice((**EncryptedContent_MetadataEntry)(unsafe.Pointer(src.metadata.data)), length)
		m := make(map[string][]byte, length)
		for _, entry := range ptrs {
			if entry == nil {
				continue
			}
			key := runtime.StringFrom(unsafe.Pointer(entry.key.data), int(entry.key.len))
			value := runtime.BytesFrom(unsafe.Pointer(entry.value.data), int(entry.value.len))
			m[key] = value
		}
		out.Metadata = m
	}
	out.Content = runtime.BytesFrom(unsafe.Pointer(src.content.data), int(src.content.len))
	if src.master_key != nil {
		out.MasterKey = FromReprMasterKeyGenerated(src.master_key)
	}
	out.Iv = runtime.BytesFrom(unsafe.Pointer(src.iv.data), int(src.iv.len))
	out.CiphertextKey = runtime.BytesFrom(unsafe.Pointer(src.ciphertext_key.data), int(src.ciphertext_key.len))
	return out
}

func NewReprEncryptionMetaGenerated(arena *runtime.Arena, src *encryptionpbproto.EncryptionMeta) *EncryptionMeta {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*EncryptionMeta)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_EncryptionMeta)))
	IntoReprEncryptionMetaGenerated(arena, ptr, src)
	return ptr
}

func IntoReprEncryptionMetaGenerated(arena *runtime.Arena, dst *EncryptionMeta, src *encryptionpbproto.EncryptionMeta) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.key_id = C.uint64_t(src.GetKeyId())
	if data, length := arena.AllocBytes(src.GetIv()); length > 0 {
		dst.iv.data = (*C.uint8_t)(data)
		dst.iv.len = C.size_t(length)
	}
}

func FromReprEncryptionMetaGenerated(src *EncryptionMeta) *encryptionpbproto.EncryptionMeta {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.EncryptionMeta{}
	out.KeyId = uint64(src.key_id)
	out.Iv = runtime.BytesFrom(unsafe.Pointer(src.iv.data), int(src.iv.len))
	return out
}

func NewReprFileDictionaryGenerated(arena *runtime.Arena, src *encryptionpbproto.FileDictionary) *FileDictionary {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FileDictionary)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_FileDictionary)))
	IntoReprFileDictionaryGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFileDictionaryGenerated(arena *runtime.Arena, dst *FileDictionary, src *encryptionpbproto.FileDictionary) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetFiles(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*FileDictionary_FilesEntry)(nil)))
		array := unsafe.Slice((**FileDictionary_FilesEntry)(ptr), len(values))
		idx := 0
		for key, value := range values {
			entry := (*FileDictionary_FilesEntry)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_FileDictionary_FilesEntry)))
			if data, length := arena.AllocString(key); length > 0 {
				entry.key.data = (*C.char)(data)
				entry.key.len = C.size_t(length)
			} else {
				entry.key.data = nil
				entry.key.len = 0
			}
			if value != nil {
				entry.value = NewReprFileInfoGenerated(arena, value)
			} else {
				entry.value = nil
			}
			array[idx] = entry
			idx++
		}
		dst.files.data = (**FileDictionary_FilesEntry)(ptr)
		dst.files.len = C.size_t(len(values))
		dst.files.cap = C.size_t(len(values))
	}
}

func FromReprFileDictionaryGenerated(src *FileDictionary) *encryptionpbproto.FileDictionary {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.FileDictionary{}
	if src.files.data != nil && src.files.len > 0 {
		length := int(src.files.len)
		ptrs := unsafe.Slice((**FileDictionary_FilesEntry)(unsafe.Pointer(src.files.data)), length)
		m := make(map[string]*encryptionpbproto.FileInfo, length)
		for _, entry := range ptrs {
			if entry == nil {
				continue
			}
			key := runtime.StringFrom(unsafe.Pointer(entry.key.data), int(entry.key.len))
			value := FromReprFileInfoGenerated(entry.value)
			m[key] = value
		}
		out.Files = m
	}
	return out
}

func NewReprFileEncryptionInfoGenerated(arena *runtime.Arena, src *encryptionpbproto.FileEncryptionInfo) *FileEncryptionInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FileEncryptionInfo)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_FileEncryptionInfo)))
	IntoReprFileEncryptionInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFileEncryptionInfoGenerated(arena *runtime.Arena, dst *FileEncryptionInfo, src *encryptionpbproto.FileEncryptionInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.encryption_method = C.int32_t(int32(src.GetEncryptionMethod()))
	if data, length := arena.AllocBytes(src.GetFileIv()); length > 0 {
		dst.file_iv.data = (*C.uint8_t)(data)
		dst.file_iv.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetChecksum()); length > 0 {
		dst.checksum.data = (*C.uint8_t)(data)
		dst.checksum.len = C.size_t(length)
	}
	dst.mode_case = 0
	switch value := src.GetMode().(type) {
	case *encryptionpbproto.FileEncryptionInfo_PlainTextDataKey:
		dst.mode_case = C.int32_t(1)
		if value.PlainTextDataKey != nil {
			dst.plain_text_data_key = NewReprPlainTextDataKeyGenerated(arena, value.PlainTextDataKey)
		} else {
			dst.plain_text_data_key = nil
		}
	case *encryptionpbproto.FileEncryptionInfo_MasterKeyBased:
		dst.mode_case = C.int32_t(2)
		if value.MasterKeyBased != nil {
			dst.master_key_based = NewReprMasterKeyBasedGenerated(arena, value.MasterKeyBased)
		} else {
			dst.master_key_based = nil
		}
	default:
		dst.mode_case = 0
	}
}

func FromReprFileEncryptionInfoGenerated(src *FileEncryptionInfo) *encryptionpbproto.FileEncryptionInfo {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.FileEncryptionInfo{}
	out.EncryptionMethod = encryptionpbproto.EncryptionMethod(int32(src.encryption_method))
	out.FileIv = runtime.BytesFrom(unsafe.Pointer(src.file_iv.data), int(src.file_iv.len))
	out.Checksum = runtime.BytesFrom(unsafe.Pointer(src.checksum.data), int(src.checksum.len))
	switch int32(src.mode_case) {
	case 1:
		out.Mode = &encryptionpbproto.FileEncryptionInfo_PlainTextDataKey{
			PlainTextDataKey: FromReprPlainTextDataKeyGenerated(src.plain_text_data_key),
		}
	case 2:
		out.Mode = &encryptionpbproto.FileEncryptionInfo_MasterKeyBased{
			MasterKeyBased: FromReprMasterKeyBasedGenerated(src.master_key_based),
		}
	}
	return out
}

func NewReprFileInfoGenerated(arena *runtime.Arena, src *encryptionpbproto.FileInfo) *FileInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FileInfo)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_FileInfo)))
	IntoReprFileInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFileInfoGenerated(arena *runtime.Arena, dst *FileInfo, src *encryptionpbproto.FileInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.key_id = C.uint64_t(src.GetKeyId())
	if data, length := arena.AllocBytes(src.GetIv()); length > 0 {
		dst.iv.data = (*C.uint8_t)(data)
		dst.iv.len = C.size_t(length)
	}
	dst.method = C.int32_t(int32(src.GetMethod()))
}

func FromReprFileInfoGenerated(src *FileInfo) *encryptionpbproto.FileInfo {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.FileInfo{}
	out.KeyId = uint64(src.key_id)
	out.Iv = runtime.BytesFrom(unsafe.Pointer(src.iv.data), int(src.iv.len))
	out.Method = encryptionpbproto.EncryptionMethod(int32(src.method))
	return out
}

func NewReprGcpKmsGenerated(arena *runtime.Arena, src *encryptionpbproto.GcpKms) *GcpKms {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GcpKms)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_GcpKms)))
	IntoReprGcpKmsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGcpKmsGenerated(arena *runtime.Arena, dst *GcpKms, src *encryptionpbproto.GcpKms) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetCredential()); length > 0 {
		dst.credential.data = (*C.char)(data)
		dst.credential.len = C.size_t(length)
	}
}

func FromReprGcpKmsGenerated(src *GcpKms) *encryptionpbproto.GcpKms {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.GcpKms{}
	out.Credential = runtime.StringFrom(unsafe.Pointer(src.credential.data), int(src.credential.len))
	return out
}

func NewReprKeyDictionaryGenerated(arena *runtime.Arena, src *encryptionpbproto.KeyDictionary) *KeyDictionary {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*KeyDictionary)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_KeyDictionary)))
	IntoReprKeyDictionaryGenerated(arena, ptr, src)
	return ptr
}

func IntoReprKeyDictionaryGenerated(arena *runtime.Arena, dst *KeyDictionary, src *encryptionpbproto.KeyDictionary) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetKeys(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KeyDictionary_KeysEntry)(nil)))
		array := unsafe.Slice((**KeyDictionary_KeysEntry)(ptr), len(values))
		idx := 0
		for key, value := range values {
			entry := (*KeyDictionary_KeysEntry)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_KeyDictionary_KeysEntry)))
			entry.key = C.uint64_t(key)
			if value != nil {
				entry.value = NewReprDataKeyGenerated(arena, value)
			} else {
				entry.value = nil
			}
			array[idx] = entry
			idx++
		}
		dst.keys.data = (**KeyDictionary_KeysEntry)(ptr)
		dst.keys.len = C.size_t(len(values))
		dst.keys.cap = C.size_t(len(values))
	}
	dst.current_key_id = C.uint64_t(src.GetCurrentKeyId())
}

func FromReprKeyDictionaryGenerated(src *KeyDictionary) *encryptionpbproto.KeyDictionary {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.KeyDictionary{}
	if src.keys.data != nil && src.keys.len > 0 {
		length := int(src.keys.len)
		ptrs := unsafe.Slice((**KeyDictionary_KeysEntry)(unsafe.Pointer(src.keys.data)), length)
		m := make(map[uint64]*encryptionpbproto.DataKey, length)
		for _, entry := range ptrs {
			if entry == nil {
				continue
			}
			key := uint64(entry.key)
			value := FromReprDataKeyGenerated(entry.value)
			m[key] = value
		}
		out.Keys = m
	}
	out.CurrentKeyId = uint64(src.current_key_id)
	return out
}

func NewReprMasterKeyGenerated(arena *runtime.Arena, src *encryptionpbproto.MasterKey) *MasterKey {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MasterKey)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_MasterKey)))
	IntoReprMasterKeyGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMasterKeyGenerated(arena *runtime.Arena, dst *MasterKey, src *encryptionpbproto.MasterKey) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.backend_case = 0
	switch value := src.GetBackend().(type) {
	case *encryptionpbproto.MasterKey_Plaintext:
		dst.backend_case = C.int32_t(1)
		if value.Plaintext != nil {
			dst.plaintext = NewReprMasterKeyPlaintextGenerated(arena, value.Plaintext)
		} else {
			dst.plaintext = nil
		}
	case *encryptionpbproto.MasterKey_File:
		dst.backend_case = C.int32_t(2)
		if value.File != nil {
			dst.file = NewReprMasterKeyFileGenerated(arena, value.File)
		} else {
			dst.file = nil
		}
	case *encryptionpbproto.MasterKey_Kms:
		dst.backend_case = C.int32_t(3)
		if value.Kms != nil {
			dst.kms = NewReprMasterKeyKmsGenerated(arena, value.Kms)
		} else {
			dst.kms = nil
		}
	default:
		dst.backend_case = 0
	}
}

func FromReprMasterKeyGenerated(src *MasterKey) *encryptionpbproto.MasterKey {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.MasterKey{}
	switch int32(src.backend_case) {
	case 1:
		out.Backend = &encryptionpbproto.MasterKey_Plaintext{
			Plaintext: FromReprMasterKeyPlaintextGenerated(src.plaintext),
		}
	case 2:
		out.Backend = &encryptionpbproto.MasterKey_File{
			File: FromReprMasterKeyFileGenerated(src.file),
		}
	case 3:
		out.Backend = &encryptionpbproto.MasterKey_Kms{
			Kms: FromReprMasterKeyKmsGenerated(src.kms),
		}
	}
	return out
}

func NewReprMasterKeyBasedGenerated(arena *runtime.Arena, src *encryptionpbproto.MasterKeyBased) *MasterKeyBased {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MasterKeyBased)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_MasterKeyBased)))
	IntoReprMasterKeyBasedGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMasterKeyBasedGenerated(arena *runtime.Arena, dst *MasterKeyBased, src *encryptionpbproto.MasterKeyBased) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetDataKeyEncryptedContent(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*EncryptedContent)(nil)))
		array := unsafe.Slice((**EncryptedContent)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprEncryptedContentGenerated(arena, value)
		}
		dst.data_key_encrypted_content.data = (**EncryptedContent)(ptr)
		dst.data_key_encrypted_content.len = C.size_t(len(values))
		dst.data_key_encrypted_content.cap = C.size_t(len(values))
	}
}

func FromReprMasterKeyBasedGenerated(src *MasterKeyBased) *encryptionpbproto.MasterKeyBased {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.MasterKeyBased{}
	if src.data_key_encrypted_content.data != nil && src.data_key_encrypted_content.len > 0 {
		length := int(src.data_key_encrypted_content.len)
		ptrs := unsafe.Slice((**EncryptedContent)(unsafe.Pointer(src.data_key_encrypted_content.data)), length)
		out.DataKeyEncryptedContent = make([]*encryptionpbproto.EncryptedContent, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.DataKeyEncryptedContent = append(out.DataKeyEncryptedContent, FromReprEncryptedContentGenerated(ptr))
		}
	}
	return out
}

func NewReprMasterKeyFileGenerated(arena *runtime.Arena, src *encryptionpbproto.MasterKeyFile) *MasterKeyFile {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MasterKeyFile)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_MasterKeyFile)))
	IntoReprMasterKeyFileGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMasterKeyFileGenerated(arena *runtime.Arena, dst *MasterKeyFile, src *encryptionpbproto.MasterKeyFile) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetPath()); length > 0 {
		dst.path.data = (*C.char)(data)
		dst.path.len = C.size_t(length)
	}
}

func FromReprMasterKeyFileGenerated(src *MasterKeyFile) *encryptionpbproto.MasterKeyFile {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.MasterKeyFile{}
	out.Path = runtime.StringFrom(unsafe.Pointer(src.path.data), int(src.path.len))
	return out
}

func NewReprMasterKeyKmsGenerated(arena *runtime.Arena, src *encryptionpbproto.MasterKeyKms) *MasterKeyKms {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MasterKeyKms)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_MasterKeyKms)))
	IntoReprMasterKeyKmsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMasterKeyKmsGenerated(arena *runtime.Arena, dst *MasterKeyKms, src *encryptionpbproto.MasterKeyKms) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetVendor()); length > 0 {
		dst.vendor.data = (*C.char)(data)
		dst.vendor.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetKeyId()); length > 0 {
		dst.key_id.data = (*C.char)(data)
		dst.key_id.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetRegion()); length > 0 {
		dst.region.data = (*C.char)(data)
		dst.region.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetEndpoint()); length > 0 {
		dst.endpoint.data = (*C.char)(data)
		dst.endpoint.len = C.size_t(length)
	}
	if value := src.GetAzureKms(); value != nil {
		dst.azure_kms = NewReprAzureKmsGenerated(arena, value)
	} else {
		dst.azure_kms = nil
	}
	if value := src.GetGcpKms(); value != nil {
		dst.gcp_kms = NewReprGcpKmsGenerated(arena, value)
	} else {
		dst.gcp_kms = nil
	}
	if value := src.GetAwsKms(); value != nil {
		dst.aws_kms = NewReprAwsKmsGenerated(arena, value)
	} else {
		dst.aws_kms = nil
	}
}

func FromReprMasterKeyKmsGenerated(src *MasterKeyKms) *encryptionpbproto.MasterKeyKms {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.MasterKeyKms{}
	out.Vendor = runtime.StringFrom(unsafe.Pointer(src.vendor.data), int(src.vendor.len))
	out.KeyId = runtime.StringFrom(unsafe.Pointer(src.key_id.data), int(src.key_id.len))
	out.Region = runtime.StringFrom(unsafe.Pointer(src.region.data), int(src.region.len))
	out.Endpoint = runtime.StringFrom(unsafe.Pointer(src.endpoint.data), int(src.endpoint.len))
	if src.azure_kms != nil {
		out.AzureKms = FromReprAzureKmsGenerated(src.azure_kms)
	}
	if src.gcp_kms != nil {
		out.GcpKms = FromReprGcpKmsGenerated(src.gcp_kms)
	}
	if src.aws_kms != nil {
		out.AwsKms = FromReprAwsKmsGenerated(src.aws_kms)
	}
	return out
}

func NewReprMasterKeyPlaintextGenerated(arena *runtime.Arena, src *encryptionpbproto.MasterKeyPlaintext) *MasterKeyPlaintext {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MasterKeyPlaintext)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_MasterKeyPlaintext)))
	IntoReprMasterKeyPlaintextGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMasterKeyPlaintextGenerated(arena *runtime.Arena, dst *MasterKeyPlaintext, src *encryptionpbproto.MasterKeyPlaintext) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprMasterKeyPlaintextGenerated(src *MasterKeyPlaintext) *encryptionpbproto.MasterKeyPlaintext {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.MasterKeyPlaintext{}
	return out
}

func NewReprPlainTextDataKeyGenerated(arena *runtime.Arena, src *encryptionpbproto.PlainTextDataKey) *PlainTextDataKey {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PlainTextDataKey)(arena.AllocZero(uintptr(C.sizeof_encryptionpb_PlainTextDataKey)))
	IntoReprPlainTextDataKeyGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPlainTextDataKeyGenerated(arena *runtime.Arena, dst *PlainTextDataKey, src *encryptionpbproto.PlainTextDataKey) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprPlainTextDataKeyGenerated(src *PlainTextDataKey) *encryptionpbproto.PlainTextDataKey {
	if src == nil {
		return nil
	}
	out := &encryptionpbproto.PlainTextDataKey{}
	return out
}
