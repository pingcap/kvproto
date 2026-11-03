package logredact_test

import (
	"bytes"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"strings"
	"testing"

	"github.com/golang/protobuf/proto"
	brpb "github.com/pingcap/kvproto/pkg/brpb"
	"github.com/pingcap/kvproto/pkg/encryptionpb"
	"github.com/pingcap/kvproto/pkg/import_sstpb"
	"github.com/pingcap/kvproto/pkg/raft_serverpb"
)

func TestGeneratedStringRedactsSecrets(t *testing.T) {
	const secret = "TEST_SECRET_2468"
	kms := &encryptionpb.MasterKey{
		Backend: &encryptionpb.MasterKey_Kms{Kms: &encryptionpb.MasterKeyKms{
			Vendor:   "visible",
			AzureKms: &encryptionpb.AzureKms{ClientSecret: secret, ClientCertificate: secret, ClientCertificatePassword: secret},
			GcpKms:   &encryptionpb.GcpKms{Credential: secret},
			AwsKms:   &encryptionpb.AwsKms{AccessKey: secret, SecretAccessKey: secret},
		}},
	}
	storage := &brpb.StorageBackend{
		Backend: &brpb.StorageBackend_S3{S3: &brpb.S3{
			Bucket: "visible", AccessKey: secret, SecretAccessKey: secret, SessionToken: secret,
		}},
	}
	fileEncryption := &encryptionpb.FileEncryptionInfo{
		Mode: &encryptionpb.FileEncryptionInfo_MasterKeyBased{MasterKeyBased: &encryptionpb.MasterKeyBased{
			DataKeyEncryptedContent: []*encryptionpb.EncryptedContent{{Content: []byte(secret), MasterKey: kms}},
		}},
	}
	tests := []struct {
		name    string
		message proto.Message
		visible bool
	}{
		{"s3", &brpb.S3{Bucket: "visible", AccessKey: secret, SecretAccessKey: secret, SessionToken: secret}, true},
		{"gcs", &brpb.GCS{Bucket: "visible", CredentialsBlob: secret}, true},
		{"azure customer key", &brpb.AzureCustomerKey{EncryptionKey: secret}, false},
		{"azure blob", &brpb.AzureBlobStorage{Bucket: "visible", SharedKey: secret, AccessSig: secret, EncryptionKey: &brpb.AzureCustomerKey{EncryptionKey: secret}}, true},
		{"cipher", &brpb.CipherInfo{CipherKey: []byte(secret)}, false},
		{"data key", &encryptionpb.DataKey{Key: []byte(secret)}, false},
		{"encrypted content", &encryptionpb.EncryptedContent{Content: []byte(secret)}, false},
		{"azure kms", &encryptionpb.AzureKms{ClientId: "visible", ClientSecret: secret, ClientCertificate: secret, ClientCertificatePassword: secret}, true},
		{"gcp kms", &encryptionpb.GcpKms{Credential: secret}, false},
		{"aws kms", &encryptionpb.AwsKms{AccessKey: secret, SecretAccessKey: secret}, false},
		{"key dictionary map", &encryptionpb.KeyDictionary{Keys: map[uint64]*encryptionpb.DataKey{3: {Key: []byte(secret)}}}, false},
		{"kms oneof", kms, true},
		{"backup request", &brpb.BackupRequest{StorageBackend: storage, CipherInfo: &brpb.CipherInfo{CipherKey: []byte(secret)}}, true},
		{"download request", &import_sstpb.DownloadRequest{Name: "visible", StorageBackend: storage, CipherInfo: &brpb.CipherInfo{CipherKey: []byte(secret)}}, true},
		{"nested repeated oneofs", &import_sstpb.ApplyRequest{Metas: []*import_sstpb.KVMeta{{Name: "visible", FileEncryptionInfo: fileEncryption}}}, true},
		{"snapshot oneof", &raft_serverpb.TabletSnapshotRequest{Payload: &raft_serverpb.TabletSnapshotRequest_Chunk{Chunk: &raft_serverpb.TabletSnapshotFileChunk{FileName: "visible", Key: &encryptionpb.DataKey{Key: []byte(secret)}}}}, true},
	}
	for _, test := range tests {
		t.Run(test.name, func(t *testing.T) {
			beforeWire, err := proto.Marshal(test.message)
			if err != nil {
				t.Fatal(err)
			}
			if !bytes.Contains(beforeWire, []byte(secret)) {
				t.Fatal("protobuf encoding did not retain the original secret")
			}
			beforeJSON, err := json.Marshal(test.message)
			if err != nil {
				t.Fatal(err)
			}
			if !bytes.Contains(beforeJSON, []byte(secret)) && !bytes.Contains(beforeJSON, []byte(base64.StdEncoding.EncodeToString([]byte(secret)))) {
				t.Fatal("JSON encoding did not retain the original secret")
			}
			for _, output := range []string{
				test.message.String(),
				fmt.Sprintf("%v", test.message),
				fmt.Sprintf("%+v", test.message),
				fmt.Sprintf("%s", test.message),
			} {
				if strings.Contains(output, secret) || !strings.Contains(output, "[REDACTED]") {
					t.Fatalf("secret was not redacted: %q", output)
				}
				if test.visible && !strings.Contains(output, "visible") {
					t.Fatalf("non-secret field was lost: %q", output)
				}
			}
			afterWire, err := proto.Marshal(test.message)
			if err != nil {
				t.Fatal(err)
			}
			afterJSON, err := json.Marshal(test.message)
			if err != nil {
				t.Fatal(err)
			}
			if !bytes.Equal(beforeWire, afterWire) || !bytes.Equal(beforeJSON, afterJSON) {
				t.Fatal("rendering modified the original message")
			}
		})
	}
}

func TestGeneratedStringPreservesMessagesWithoutSecrets(t *testing.T) {
	values := []proto.Message{
		&brpb.S3{Bucket: "visible"},
		&brpb.BackupRequest{ClusterId: 42},
		&encryptionpb.KeyDictionary{CurrentKeyId: 7},
		&import_sstpb.DownloadRequest{Name: "visible"},
	}
	for _, value := range values {
		if got, want := value.String(), proto.CompactTextString(value); got != want {
			t.Fatalf("%T: got %q, want %q", value, got, want)
		}
	}
	var nilS3 *brpb.S3
	if got, want := nilS3.String(), proto.CompactTextString(nilS3); got != want {
		t.Fatalf("nil message: got %q, want %q", got, want)
	}
}
