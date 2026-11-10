package logredact_test

import (
	"bytes"
	"encoding/base64"
	"encoding/json"
	"fmt"
	"reflect"
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
	dynamic := &brpb.CloudDynamic{
		ProviderName: "visible", Attrs: map[string]string{"access_key": secret},
	}
	dynamicStorage := &brpb.StorageBackend{
		Backend: &brpb.StorageBackend_CloudDynamic{CloudDynamic: dynamic},
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
		{"cloud dynamic", dynamic, true},
		{"cloud dynamic backend", dynamicStorage, true},
		{"backup request with cloud dynamic", &brpb.BackupRequest{StorageBackend: dynamicStorage}, true},
		{"download request with cloud dynamic", &import_sstpb.DownloadRequest{StorageBackend: dynamicStorage}, true},
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
				fmt.Sprintf("%#v", test.message),
				fmt.Sprintf("%#v", reflect.ValueOf(test.message).Elem().Interface()),
				fmt.Sprintf("%v", reflect.ValueOf(test.message).Elem().Interface()),
				fmt.Sprintf("%+v", reflect.ValueOf(test.message).Elem().Interface()),
				fmt.Sprintf("%s", reflect.ValueOf(test.message).Elem().Interface()),
				fmt.Sprintf("%v", []interface{}{reflect.ValueOf(test.message).Elem().Interface()}),
				fmt.Sprintf("%+v", map[string]interface{}{"message": reflect.ValueOf(test.message).Elem().Interface()}),
				fmt.Sprintf("%#v", []proto.Message{test.message}),
				fmt.Sprintf("%#v", map[string]proto.Message{"message": test.message}),
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

func TestCloudDynamicMapValuesAreRedactedWithoutMutation(t *testing.T) {
	dynamic := &brpb.CloudDynamic{
		ProviderName: "visible-provider",
		Attrs: map[string]string{
			"access_key": "TEST_ACCESS_KEY",
			"token":      "TEST_SESSION_TOKEN",
			"endpoint":   "TEST_OTHER_VALUE",
			"empty":      "",
		},
	}
	before := proto.Clone(dynamic)
	output := dynamic.String()
	for _, original := range []string{"TEST_ACCESS_KEY", "TEST_SESSION_TOKEN", "TEST_OTHER_VALUE"} {
		if strings.Contains(output, original) {
			t.Fatalf("map value was not redacted: %q", output)
		}
	}
	for _, visible := range []string{"visible-provider", "access_key", "token", "endpoint", "empty"} {
		if !strings.Contains(output, visible) {
			t.Fatalf("provider or map key was lost: %q", output)
		}
	}
	if strings.Count(output, "[REDACTED]") != 3 {
		t.Fatalf("non-empty map values were not all redacted: %q", output)
	}
	if !proto.Equal(before, dynamic) {
		t.Fatal("rendering modified the original map")
	}
}

func TestGeneratedStringPreservesMessagesWithoutSecrets(t *testing.T) {
	values := []proto.Message{
		&brpb.S3{Bucket: "visible"},
		&brpb.CloudDynamic{ProviderName: "visible"},
		&brpb.CloudDynamic{Attrs: map[string]string{"empty": ""}},
		&brpb.BackupRequest{ClusterId: 42},
		&encryptionpb.KeyDictionary{CurrentKeyId: 7},
		&import_sstpb.DownloadRequest{Name: "visible"},
	}
	for _, value := range values {
		for _, output := range []string{
			value.String(),
			fmt.Sprintf("%#v", value),
			fmt.Sprintf("%#v", reflect.ValueOf(value).Elem().Interface()),
		} {
			if want := proto.CompactTextString(value); output != want {
				t.Fatalf("%T: got %q, want %q", value, output, want)
			}
		}
	}
	var nilS3 *brpb.S3
	if got, want := nilS3.String(), proto.CompactTextString(nilS3); got != want {
		t.Fatalf("nil message: got %q, want %q", got, want)
	}
}

func TestGeneratedGoStringFormatsNilMessages(t *testing.T) {
	values := []proto.Message{
		(*brpb.S3)(nil),
		(*brpb.StorageBackend)(nil),
		(*brpb.CloudDynamic)(nil),
		(*brpb.BackupRequest)(nil),
		(*encryptionpb.DataKey)(nil),
		(*encryptionpb.KeyDictionary)(nil),
		(*import_sstpb.DownloadRequest)(nil),
	}
	for _, value := range values {
		for _, format := range []string{"%v", "%+v", "%#v", "%s"} {
			if got := fmt.Sprintf(format, value); got != "<nil>" {
				t.Fatalf("%T %s: got %q, want <nil>", value, format, got)
			}
		}
	}
}

func TestGeneratedFormatPreservesFlagsWidthAndPrecision(t *testing.T) {
	message := brpb.S3{Bucket: "visible"}
	expected := message.String()
	for _, format := range []string{"%v", "%+v", "%#v", "%s", "%q", "%#q", "%x", "%X", "%30s", "%-30s", "%030s", "%.7s", "%30.7s"} {
		stringFormat := format
		if strings.HasSuffix(format, "v") {
			stringFormat = strings.ReplaceAll(format, "#", "")
		}
		for _, value := range []interface{}{message, &message} {
			if got, want := fmt.Sprintf(format, value), fmt.Sprintf(stringFormat, expected); got != want {
				t.Fatalf("%s: got %q, want %q", format, got, want)
			}
		}
	}
}
