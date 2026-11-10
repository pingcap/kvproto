// Command redact_proto_strings protects generated formatting methods using the
// current protobuf message graph.
package main

import (
	"bytes"
	"fmt"
	"go/format"
	"os"
	"path/filepath"
	"sort"
	"strings"

	"github.com/gogo/protobuf/proto"
	"github.com/gogo/protobuf/protoc-gen-gogo/descriptor"
)

var secrets = map[string][]string{
	".backup.S3":                     {"access_key", "secret_access_key", "session_token"},
	".backup.GCS":                    {"credentials_blob"},
	".backup.AzureCustomerKey":       {"encryption_key"},
	".backup.AzureBlobStorage":       {"shared_key", "access_sig"},
	".backup.CipherInfo":             {"cipher_key"},
	".encryptionpb.DataKey":          {"key"},
	".encryptionpb.EncryptedContent": {"content"},
	".encryptionpb.AzureKms":         {"client_secret", "client_certificate", "client_certificate_password"},
	".encryptionpb.GcpKms":           {"credential"},
	".encryptionpb.AwsKms":           {"access_key", "secret_access_key"},
}

var secretMaps = map[string][]string{
	".backup.CloudDynamic": {"attrs"},
}

type message struct {
	file     string
	goName   string
	mapEntry bool
	fields   map[string]*descriptor.FieldDescriptorProto
	edges    []string
}

func main() {
	if len(os.Args) != 2 {
		fail("usage: redact_proto_strings descriptor-set")
	}
	data, err := os.ReadFile(os.Args[1])
	check(err)
	var set descriptor.FileDescriptorSet
	check(proto.Unmarshal(data, &set))
	messages := make(map[string]message)
	var add func(string, string, string, *descriptor.DescriptorProto)
	add = func(prefix, goPrefix, file string, desc *descriptor.DescriptorProto) {
		name := prefix + "." + desc.GetName()
		goName := goPrefix + desc.GetName()
		item := message{file: file, goName: goName, mapEntry: desc.GetOptions().GetMapEntry(), fields: make(map[string]*descriptor.FieldDescriptorProto)}
		for _, field := range desc.GetField() {
			item.fields[field.GetName()] = field
			if field.GetTypeName() != "" {
				item.edges = append(item.edges, field.GetTypeName())
			}
		}
		messages[name] = item
		for _, nested := range desc.GetNestedType() {
			add(name, goName+"_", file, nested)
		}
	}
	for _, file := range set.GetFile() {
		for _, desc := range file.GetMessageType() {
			add("."+file.GetPackage(), "", file.GetName(), desc)
		}
	}

	reachable := make(map[string]bool)
	for name, fields := range secrets {
		item, found := messages[name]
		if !found {
			fail("missing sensitive message %s", name)
		}
		for _, field := range fields {
			f := item.fields[field]
			if f == nil {
				fail("missing sensitive field %s.%s", name, field)
			}
			if f.GetLabel() == descriptor.FieldDescriptorProto_LABEL_REPEATED ||
				(f.GetType() != descriptor.FieldDescriptorProto_TYPE_STRING && f.GetType() != descriptor.FieldDescriptorProto_TYPE_BYTES) {
				fail("unsupported sensitive field type %s.%s", name, field)
			}
		}
		reachable[name] = true
	}
	for name, fields := range secretMaps {
		item, found := messages[name]
		if !found {
			fail("missing sensitive message %s", name)
		}
		for _, field := range fields {
			f := item.fields[field]
			if f == nil {
				fail("missing sensitive map %s.%s", name, field)
			}
			entry, found := messages[f.GetTypeName()]
			if f.GetLabel() != descriptor.FieldDescriptorProto_LABEL_REPEATED ||
				f.GetType() != descriptor.FieldDescriptorProto_TYPE_MESSAGE ||
				!found || !entry.mapEntry ||
				entry.fields["key"].GetType() != descriptor.FieldDescriptorProto_TYPE_STRING ||
				entry.fields["value"].GetType() != descriptor.FieldDescriptorProto_TYPE_STRING {
				fail("unsupported sensitive map type %s.%s", name, field)
			}
		}
		reachable[name] = true
	}
	for changed := true; changed; {
		changed = false
		for name, item := range messages {
			for _, target := range item.edges {
				if reachable[target] && !reachable[name] {
					reachable[name], changed = true, true
				}
			}
		}
	}

	root, err := filepath.Abs("..")
	check(err)
	names := make([]string, 0, len(reachable))
	for name := range reachable {
		if !messages[name].mapEntry {
			names = append(names, name)
		}
	}
	sort.Strings(names)
	byFile := make(map[string][]message)
	for _, name := range names {
		item := messages[name]
		byFile[item.file] = append(byFile[item.file], item)
	}
	const importLine = "\tlogredact \"github.com/pingcap/kvproto/pkg/logredact\"\n"
	const protoImport = "\tproto \"github.com/golang/protobuf/proto\"\n"
	for file, items := range byFile {
		base := strings.TrimSuffix(filepath.Base(file), ".proto")
		path := filepath.Join(root, "pkg", base, base+".pb.go")
		source, err := os.ReadFile(path)
		check(err)
		text := string(source)
		if strings.Count(text, protoImport) != 1 || strings.Contains(text, importLine) {
			fail("unexpected imports in %s", path)
		}
		text = strings.Replace(text, protoImport, protoImport+importLine, 1)
		for _, item := range items {
			old := fmt.Sprintf("func (m *%s) String() string { return proto.CompactTextString(m) }", item.goName)
			replacement := fmt.Sprintf("func (m *%s) String() string { return logredact.CompactTextString(m) }\n\n"+
				"func (message %s) GoString() string { return logredact.CompactTextString(&message) }\n\n"+
				"func (message %s) Format(state fmt.State, verb rune) {\n\tlogredact.Format(state, verb, &message)\n}",
				item.goName, item.goName, item.goName)
			if strings.Count(text, old) != 1 {
				fail("unexpected String method for %s in %s", item.goName, path)
			}
			text = strings.Replace(text, old, replacement, 1)
		}
		formatted, err := format.Source([]byte(text))
		check(err)
		check(os.WriteFile(path, formatted, 0644))
	}

	goType := func(item message) string {
		base := strings.TrimSuffix(filepath.Base(item.file), ".proto")
		return "github.com/pingcap/kvproto/pkg/" + base + "." + item.goName
	}
	var out bytes.Buffer
	fmt.Fprint(&out, "// Code generated by redact_proto_strings. DO NOT EDIT.\n\npackage logredact\n\n")
	fmt.Fprintln(&out, "var protectedMessageTypes = map[string]bool{")
	for _, name := range names {
		fmt.Fprintf(&out, "%q: true,\n", goType(messages[name]))
	}
	fmt.Fprint(&out, "}\n\nvar secretFields = map[string]map[string]bool{\n")
	for _, name := range names {
		if len(secrets[name]) == 0 {
			continue
		}
		fmt.Fprintf(&out, "%q: {\n", goType(messages[name]))
		for _, field := range secrets[name] {
			fmt.Fprintf(&out, "%q: true,\n", field)
		}
		fmt.Fprintln(&out, "},")
	}
	fmt.Fprint(&out, "}\n\nvar secretMapFields = map[string]map[string]bool{\n")
	for _, name := range names {
		if len(secretMaps[name]) == 0 {
			continue
		}
		fmt.Fprintf(&out, "%q: {\n", goType(messages[name]))
		for _, field := range secretMaps[name] {
			fmt.Fprintf(&out, "%q: true,\n", field)
		}
		fmt.Fprintln(&out, "},")
	}
	fmt.Fprintln(&out, "}")
	formatted, err := format.Source(out.Bytes())
	check(err)
	check(os.WriteFile(filepath.Join(root, "pkg", "logredact", "types.pb.go"), formatted, 0644))
}

func check(err error) {
	if err != nil {
		fail("%v", err)
	}
}

func fail(format string, args ...any) {
	fmt.Fprintf(os.Stderr, format+"\n", args...)
	os.Exit(1)
}
