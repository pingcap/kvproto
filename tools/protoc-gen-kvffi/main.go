package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"path"
	"path/filepath"
	"sort"
	"strings"

	"github.com/gogo/protobuf/gogoproto"
	gogoProto "github.com/gogo/protobuf/proto"
	gogodescriptor "github.com/gogo/protobuf/protoc-gen-gogo/descriptor"
	"google.golang.org/protobuf/proto"
	descriptorpb "google.golang.org/protobuf/types/descriptorpb"
	pluginpb "google.golang.org/protobuf/types/pluginpb"
	"unicode"
)

func main() {
	input, err := io.ReadAll(os.Stdin)
	if err != nil {
		failf("failed to read input: %v", err)
	}

	req := new(pluginpb.CodeGeneratorRequest)
	if err := proto.Unmarshal(input, req); err != nil {
		failf("failed to parse CodeGeneratorRequest: %v", err)
	}

	if len(req.GetProtoFile()) == 0 {
		writeResponse(new(pluginpb.CodeGeneratorResponse))
		return
	}

	schema, err := collectFromDescriptors(req)
	if err != nil {
		failf("failed to build schema: %v", err)
	}

	builder := &specBuilder{
		messages:         schema.messages,
		enums:            schema.enums,
		files:            schema.files,
		slices:           make(map[string]*sliceSpec),
		mapEntries:       make(map[string]*messageSpec),
		externalMessages: make(map[string]struct{}),
	}

	messageSpecs := builder.buildMessageSpecs()
	sliceSpecs := builder.sortedSlices()
	external := sortedExternal(builder.externalMessages, schema.messages)

	cContent := emitC(messageSpecs, sliceSpecs, builder.requiresStringView, builder.requiresBytesView, external)
	rustContent := emitRust(messageSpecs, sliceSpecs, builder.requiresStringView, builder.requiresBytesView, external)
	goSpecs := buildGoFileSpecs(messageSpecs, external)

	resp := new(pluginpb.CodeGeneratorResponse)
	resp.File = append(resp.File, &pluginpb.CodeGeneratorResponse_File{
		Name:    proto.String(filepath.ToSlash(filepath.Join("ffi_out", "c", "kvproto_abi.h"))),
		Content: proto.String(string(cContent)),
	})
	resp.File = append(resp.File, &pluginpb.CodeGeneratorResponse_File{
		Name:    proto.String(filepath.ToSlash(filepath.Join("ffi_out", "rust", "kvproto_abi.rs"))),
		Content: proto.String(string(rustContent)),
	})

	const headerInclude = "../c"
	for _, spec := range goSpecs {
		content := buildGoFile(spec, headerInclude)
		resp.File = append(resp.File, &pluginpb.CodeGeneratorResponse_File{
			Name:    proto.String(filepath.ToSlash(goOutputPath(spec))),
			Content: proto.String(string(content)),
		})
		if conv := buildGoConversionsFile(spec); len(conv) > 0 {
			resp.File = append(resp.File, &pluginpb.CodeGeneratorResponse_File{
				Name:    proto.String(filepath.ToSlash(goConversionsPath(spec))),
				Content: proto.String(string(conv)),
			})
		}
		if conv := buildRustConversions(spec); len(conv) > 0 {
			resp.File = append(resp.File, &pluginpb.CodeGeneratorResponse_File{
				Name:    proto.String(filepath.ToSlash(rustConversionsPath(spec))),
				Content: proto.String(string(conv)),
			})
		}
	}

	writeResponse(resp)
}

func failf(format string, args ...interface{}) {
	resp := &pluginpb.CodeGeneratorResponse{
		Error: proto.String(fmt.Sprintf(format, args...)),
	}
	writeResponse(resp)
	os.Exit(1)
}

func writeResponse(resp *pluginpb.CodeGeneratorResponse) {
	out, err := proto.Marshal(resp)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to marshal response: %v\n", err)
		os.Exit(1)
	}
	if _, err := os.Stdout.Write(out); err != nil {
		fmt.Fprintf(os.Stderr, "failed to write response: %v\n", err)
		os.Exit(1)
	}
}

type descriptorSchema struct {
	messages map[string]*protoMessage
	enums    map[string]*protoFile
	files    map[string]*protoFile
}

func collectFromDescriptors(req *pluginpb.CodeGeneratorRequest) (*descriptorSchema, error) {
	files := make(map[string]*protoFile)
	messageIndex := make(map[string]*descriptorMessage)
	enumNames := make(map[string]*protoFile)

	for _, file := range req.GetProtoFile() {
		info := buildFileInfo(file)
		files[file.GetName()] = info
		registerMessages(file, info, messageIndex)
		registerEnums(file, info, enumNames)
	}

	messages := make(map[string]*protoMessage)
	for fullName, meta := range messageIndex {
		fields, maps := extractFields(meta, messageIndex)
		messages[fullName] = &protoMessage{
			FullName:   fullName,
			Package:    meta.pkg,
			GoPackage:  meta.file.GoPackage,
			ProtoPath:  meta.file.ProtoPath,
			PathParts:  append([]string(nil), meta.pathParts...),
			Fields:     fields,
			Maps:       maps,
			File:       meta.file,
			IsMapEntry: meta.mapEntry,
		}
	}

	return &descriptorSchema{
		messages: messages,
		enums:    enumNames,
		files:    files,
	}, nil
}

type descriptorMessage struct {
	desc      *descriptorpb.DescriptorProto
	file      *protoFile
	pkg       string
	pathParts []string
	fullName  string
	parent    *descriptorMessage
	mapEntry  bool
}

func buildFileInfo(file *descriptorpb.FileDescriptorProto) *protoFile {
	pkg := file.GetPackage()
	goPkg, goImport := parseGoPackageOption(file.GetOptions().GetGoPackage(), file.GetName(), pkg)
	syntax := strings.ToLower(file.GetSyntax())
	if syntax == "" {
		syntax = "proto2"
	}
	return &protoFile{
		ProtoPath:    file.GetName(),
		Package:      pkg,
		GoPackage:    goPkg,
		GoImportPath: goImport,
		Syntax:       syntax,
	}
}

func parseGoPackageOption(opt, protoPath, pkg string) (string, string) {
	if opt == "" {
		return goPackageName(pkg, protoPath), goImportPath(protoPath)
	}
	if strings.Contains(opt, ";") {
		parts := strings.Split(opt, ";")
		if len(parts) == 2 {
			return sanitizeGoPackageName(parts[1]), parts[0]
		}
	}
	if strings.Contains(opt, "/") {
		return sanitizeGoPackageName(path.Base(opt)), opt
	}
	return sanitizeGoPackageName(opt), goImportPath(protoPath)
}

func registerMessages(file *descriptorpb.FileDescriptorProto, info *protoFile, registry map[string]*descriptorMessage) {
	var walk func(parent *descriptorMessage, msg *descriptorpb.DescriptorProto, pathParts []string)
	walk = func(parent *descriptorMessage, msg *descriptorpb.DescriptorProto, pathParts []string) {
		current := append(pathParts, msg.GetName())
		fullName := messageFullName(current, info.Package)
		entry := &descriptorMessage{
			desc:      msg,
			file:      info,
			pkg:       info.Package,
			pathParts: append([]string(nil), current...),
			fullName:  fullName,
			parent:    parent,
			mapEntry:  msg.GetOptions().GetMapEntry(),
		}
		registry[fullName] = entry
		for _, nested := range msg.GetNestedType() {
			walk(entry, nested, current)
		}
		for _, enum := range msg.GetEnumType() {
			enumFull := "." + joinWithPkg(info.Package, append(current, enum.GetName())...)
			_ = enumFull // deferred; registerEnums handles final set
		}
	}
	for _, top := range file.GetMessageType() {
		walk(nil, top, nil)
	}
}

func registerEnums(file *descriptorpb.FileDescriptorProto, info *protoFile, enums map[string]*protoFile) {
	var walk func(pathParts []string, msg *descriptorpb.DescriptorProto)
	for _, enum := range file.GetEnumType() {
		full := "." + joinWithPkg(info.Package, enum.GetName())
		enums[full] = info
	}
	walk = func(pathParts []string, msg *descriptorpb.DescriptorProto) {
		current := append(pathParts, msg.GetName())
		for _, enum := range msg.GetEnumType() {
			full := "." + joinWithPkg(info.Package, append(current, enum.GetName())...)
			enums[full] = info
		}
		for _, nested := range msg.GetNestedType() {
			walk(current, nested)
		}
	}
	for _, top := range file.GetMessageType() {
		walk(nil, top)
	}
}

func extractFields(meta *descriptorMessage, registry map[string]*descriptorMessage) ([]protoField, []protoMap) {
	fields := make([]protoField, 0, len(meta.desc.GetField()))
	maps := make([]protoMap, 0)

	for _, field := range meta.desc.GetField() {
		if isMapField(field, registry) {
			mp := buildMapFieldDescriptor(field, registry, meta)
			maps = append(maps, mp)
			continue
		}
		fields = append(fields, buildProtoField(field, meta))
	}

	return fields, maps
}

func buildProtoField(field *descriptorpb.FieldDescriptorProto, meta *descriptorMessage) protoField {
	pf := protoField{
		Number:     int(field.GetNumber()),
		Name:       field.GetName(),
		TypeName:   "",
		FullType:   "",
		Kind:       fieldKindScalar,
		Scalar:     "",
		Repeated:   false,
		Oneof:      "",
		descriptor: field.GetType(),
		Nullable:   true,
	}

	if field.OneofIndex != nil && meta.desc.GetOneofDecl() != nil {
		idx := field.GetOneofIndex()
		if int(idx) < len(meta.desc.GetOneofDecl()) {
			pf.Oneof = meta.desc.GetOneofDecl()[idx].GetName()
		}
	}

	pf.Nullable = true
	if data, err := proto.Marshal(field); err == nil {
		legacy := new(gogodescriptor.FieldDescriptorProto)
		if err := gogoProto.Unmarshal(data, legacy); err == nil {
			pf.Nullable = gogoproto.IsNullable(legacy)
			pf.CustomType = gogoproto.GetCustomType(legacy)
		}
	}

	if field.GetLabel() == descriptorpb.FieldDescriptorProto_LABEL_REPEATED {
		pf.Repeated = true
	}

	syntax := meta.file.Syntax
	if syntax == "" {
		syntax = "proto2"
	}
	if strings.ToLower(syntax) == "proto2" && !pf.Repeated && pf.Oneof == "" {
		switch field.GetType() {
		case descriptorpb.FieldDescriptorProto_TYPE_STRING,
			descriptorpb.FieldDescriptorProto_TYPE_BOOL,
			descriptorpb.FieldDescriptorProto_TYPE_DOUBLE,
			descriptorpb.FieldDescriptorProto_TYPE_FLOAT,
			descriptorpb.FieldDescriptorProto_TYPE_INT32,
			descriptorpb.FieldDescriptorProto_TYPE_SINT32,
			descriptorpb.FieldDescriptorProto_TYPE_SFIXED32,
			descriptorpb.FieldDescriptorProto_TYPE_UINT32,
			descriptorpb.FieldDescriptorProto_TYPE_FIXED32,
			descriptorpb.FieldDescriptorProto_TYPE_INT64,
			descriptorpb.FieldDescriptorProto_TYPE_SINT64,
			descriptorpb.FieldDescriptorProto_TYPE_SFIXED64,
			descriptorpb.FieldDescriptorProto_TYPE_UINT64,
			descriptorpb.FieldDescriptorProto_TYPE_FIXED64,
			descriptorpb.FieldDescriptorProto_TYPE_BYTES,
			descriptorpb.FieldDescriptorProto_TYPE_ENUM:
			pf.Proto2Optional = true
		}
	}

	switch field.GetType() {
	case descriptorpb.FieldDescriptorProto_TYPE_MESSAGE, descriptorpb.FieldDescriptorProto_TYPE_ENUM:
		pf.TypeName = field.GetTypeName()
		pf.FullType = field.GetTypeName()
		if field.GetType() == descriptorpb.FieldDescriptorProto_TYPE_MESSAGE {
			pf.Kind = fieldKindMessage
		} else {
			pf.Kind = fieldKindEnum
		}
	case descriptorpb.FieldDescriptorProto_TYPE_STRING:
		pf.TypeName = "string"
		pf.Kind = fieldKindString
	case descriptorpb.FieldDescriptorProto_TYPE_BYTES:
		pf.TypeName = "bytes"
		pf.Kind = fieldKindBytes
	default:
		pf.TypeName = scalarProtoName(field.GetType())
		pf.Scalar = pf.TypeName
		pf.Kind = fieldKindScalar
	}
	return pf
}

func buildMapFieldDescriptor(field *descriptorpb.FieldDescriptorProto, registry map[string]*descriptorMessage, meta *descriptorMessage) protoMap {
	entryDesc := registry[field.GetTypeName()]
	var (
		keyType    string
		keyKind    fieldKind
		keyScalar  string
		valueField = new(descriptorpb.FieldDescriptorProto)
	)

	for _, f := range entryDesc.desc.GetField() {
		if f.GetName() == "key" {
			keyType = mapKeyTypeName(f)
			keyKind, keyScalar = mapKeyKind(f)
		}
		if f.GetName() == "value" {
			valueField = f
		}
	}

	valueProto := buildProtoField(valueField, entryDesc)
	valueProto.Number = int(field.GetNumber())
	valueProto.Name = field.GetName()
	valueProto.Oneof = ""
	valueProto.Repeated = false

	return protoMap{
		Field:     valueProto,
		KeyType:   keyType,
		KeyKind:   keyKind,
		KeyScalar: keyScalar,
	}
}

func isMapField(field *descriptorpb.FieldDescriptorProto, registry map[string]*descriptorMessage) bool {
	if field.GetType() != descriptorpb.FieldDescriptorProto_TYPE_MESSAGE {
		return false
	}
	entry := registry[field.GetTypeName()]
	if entry == nil {
		return false
	}
	return entry.desc.GetOptions().GetMapEntry()
}

func scalarProtoName(t descriptorpb.FieldDescriptorProto_Type) string {
	switch t {
	case descriptorpb.FieldDescriptorProto_TYPE_DOUBLE:
		return "double"
	case descriptorpb.FieldDescriptorProto_TYPE_FLOAT:
		return "float"
	case descriptorpb.FieldDescriptorProto_TYPE_INT64:
		return "int64"
	case descriptorpb.FieldDescriptorProto_TYPE_UINT64:
		return "uint64"
	case descriptorpb.FieldDescriptorProto_TYPE_INT32:
		return "int32"
	case descriptorpb.FieldDescriptorProto_TYPE_FIXED64:
		return "fixed64"
	case descriptorpb.FieldDescriptorProto_TYPE_FIXED32:
		return "fixed32"
	case descriptorpb.FieldDescriptorProto_TYPE_BOOL:
		return "bool"
	case descriptorpb.FieldDescriptorProto_TYPE_UINT32:
		return "uint32"
	case descriptorpb.FieldDescriptorProto_TYPE_SFIXED32:
		return "sfixed32"
	case descriptorpb.FieldDescriptorProto_TYPE_SFIXED64:
		return "sfixed64"
	case descriptorpb.FieldDescriptorProto_TYPE_SINT32:
		return "sint32"
	case descriptorpb.FieldDescriptorProto_TYPE_SINT64:
		return "sint64"
	default:
		return ""
	}
}

func mapKeyTypeName(field *descriptorpb.FieldDescriptorProto) string {
	switch field.GetType() {
	case descriptorpb.FieldDescriptorProto_TYPE_STRING:
		return "string"
	case descriptorpb.FieldDescriptorProto_TYPE_BYTES:
		return "bytes"
	case descriptorpb.FieldDescriptorProto_TYPE_MESSAGE, descriptorpb.FieldDescriptorProto_TYPE_ENUM:
		return field.GetTypeName()
	default:
		return scalarProtoName(field.GetType())
	}
}

func mapKeyKind(field *descriptorpb.FieldDescriptorProto) (fieldKind, string) {
	switch field.GetType() {
	case descriptorpb.FieldDescriptorProto_TYPE_STRING:
		return fieldKindString, ""
	case descriptorpb.FieldDescriptorProto_TYPE_BYTES:
		return fieldKindBytes, ""
	case descriptorpb.FieldDescriptorProto_TYPE_BOOL,
		descriptorpb.FieldDescriptorProto_TYPE_INT32,
		descriptorpb.FieldDescriptorProto_TYPE_UINT32,
		descriptorpb.FieldDescriptorProto_TYPE_SINT32,
		descriptorpb.FieldDescriptorProto_TYPE_FIXED32,
		descriptorpb.FieldDescriptorProto_TYPE_SFIXED32,
		descriptorpb.FieldDescriptorProto_TYPE_INT64,
		descriptorpb.FieldDescriptorProto_TYPE_UINT64,
		descriptorpb.FieldDescriptorProto_TYPE_SINT64,
		descriptorpb.FieldDescriptorProto_TYPE_FIXED64,
		descriptorpb.FieldDescriptorProto_TYPE_SFIXED64:
		return fieldKindScalar, scalarProtoName(field.GetType())
	default:
		return fieldKindScalar, ""
	}
}

type protoFile struct {
	ProtoPath    string
	Package      string
	GoPackage    string
	GoImportPath string
	Syntax       string
}

type fieldKind int

const (
	fieldKindScalar fieldKind = iota
	fieldKindString
	fieldKindBytes
	fieldKindMessage
	fieldKindEnum
)

type protoField struct {
	Number         int
	Name           string
	TypeName       string
	FullType       string
	Kind           fieldKind
	Scalar         string
	Repeated       bool
	Oneof          string
	descriptor     descriptorpb.FieldDescriptorProto_Type
	Proto2Optional bool
	Nullable       bool
	CustomType     string
}

type protoMap struct {
	Field     protoField
	KeyType   string
	KeyKind   fieldKind
	KeyScalar string
}

type protoMessage struct {
	FullName   string
	Package    string
	GoPackage  string
	ProtoPath  string
	PathParts  []string
	Fields     []protoField
	Maps       []protoMap
	File       *protoFile
	IsMapEntry bool
}

type fieldInput struct {
	field   protoField
	mapInfo *protoMap
}

type sliceSpec struct {
	ElementCType    string
	ElementRustType string
	ElementGoType   string
	CName           string
	RustName        string
	GoName          string
}

type messageSpec struct {
	FullName  string
	Package   string
	GoPackage string
	GoImport  string
	ProtoPath string
	CName     string
	RustName  string
	GoName    string
	GoAlias   string
	Fields    []fieldSpec
	File      *protoFile
	Proto     *protoMessage
}

type goFileSpec struct {
	ProtoPath      string
	GoPackage      string
	PBImportPath   string
	Messages       []*messageSpec
	Slices         []*sliceSpec
	NeedStringView bool
	NeedBytesView  bool
	External       []externalType
	IsExternalOnly bool
}

type goFileBuilder struct {
	spec     *goFileSpec
	sliceSet map[string]*sliceSpec
}

type mapFieldInfo struct {
	KeyType       string
	KeyKind       fieldKind
	KeyScalar     string
	EntryFullName string
	Entry         *messageSpec
	Value         protoField
	ValueFullType string
	ValueMessage  *messageSpec
	ValueEnumFile *protoFile
}

type fieldSpec struct {
	Name           string
	CType          string
	RustType       string
	GoType         string
	Comment        string
	Slice          *sliceSpec
	UsesString     bool
	UsesBytes      bool
	Proto          protoField
	FullType       string
	Message        *messageSpec
	Enum           bool
	EnumFile       *protoFile
	Map            *mapFieldInfo
	Proto2Optional bool
}

type specBuilder struct {
	messages map[string]*protoMessage
	enums    map[string]*protoFile
	files    map[string]*protoFile

	slices     map[string]*sliceSpec
	mapEntries map[string]*messageSpec

	requiresStringView bool
	requiresBytesView  bool

	externalMessages map[string]struct{}
}

func (b *specBuilder) buildMessageSpecs() []*messageSpec {
	names := make([]string, 0, len(b.messages))
	for name := range b.messages {
		names = append(names, name)
	}
	sort.Strings(names)

	result := make([]*messageSpec, 0, len(names))
	for _, full := range names {
		msg := b.messages[full]
		if msg.IsMapEntry {
			// Map entry helper messages are generated via buildMapField; skip duplicates.
			continue
		}
		cName := toCTypeName(full)
		rustName := toRustTypeName(full)
		goAlias := joinMessageAlias(msg.PathParts)
		fields := b.buildFields(msg)
		result = append(result, &messageSpec{
			FullName:  full,
			Package:   msg.Package,
			GoPackage: msg.GoPackage,
			GoImport:  msg.File.GoImportPath,
			ProtoPath: msg.ProtoPath,
			CName:     cName,
			RustName:  rustName,
			GoName:    goAlias,
			GoAlias:   goAlias,
			Fields:    fields,
			File:      msg.File,
			Proto:     msg,
		})
	}

	if len(b.mapEntries) > 0 {
		entryNames := make([]string, 0, len(b.mapEntries))
		for name := range b.mapEntries {
			entryNames = append(entryNames, name)
		}
		sort.Strings(entryNames)
		for _, name := range entryNames {
			result = append(result, b.mapEntries[name])
		}
	}

	specIndex := make(map[string]*messageSpec, len(result))
	for _, spec := range result {
		specIndex[spec.FullName] = spec
	}
	for _, spec := range result {
		for i := range spec.Fields {
			fs := &spec.Fields[i]
			if fs.FullType != "" && fs.Proto.Kind == fieldKindMessage {
				if target := specIndex[fs.FullType]; target != nil {
					fs.Message = target
				}
			}
			if fs.Map != nil {
				if entry := specIndex[fs.Map.EntryFullName]; entry != nil {
					fs.Map.Entry = entry
				}
				if fs.Map.ValueFullType != "" {
					if target := specIndex[fs.Map.ValueFullType]; target != nil {
						fs.Map.ValueMessage = target
					}
				}
			}
		}
	}

	return result
}

func (b *specBuilder) sortedSlices() []*sliceSpec {
	keys := make([]string, 0, len(b.slices))
	for key := range b.slices {
		keys = append(keys, key)
	}
	sort.Strings(keys)
	result := make([]*sliceSpec, 0, len(keys))
	for _, key := range keys {
		result = append(result, b.slices[key])
	}
	return result
}

func (b *specBuilder) buildFields(msg *protoMessage) []fieldSpec {
	inputs := make([]fieldInput, 0, len(msg.Fields)+len(msg.Maps))
	for _, field := range msg.Fields {
		inputs = append(inputs, fieldInput{field: field})
	}
	for i := range msg.Maps {
		inputs = append(inputs, fieldInput{field: msg.Maps[i].Field, mapInfo: &msg.Maps[i]})
	}
	sort.Slice(inputs, func(i, j int) bool {
		return inputs[i].field.Number < inputs[j].field.Number
	})

	fields := make([]fieldSpec, 0, len(inputs))
	seenOneof := make(map[string]struct{})

	for _, input := range inputs {
		field := input.field
		fieldName := sanitizeFieldName(field.Name)
		if input.mapInfo != nil {
			mapField := b.buildMapField(msg, input.mapInfo, fieldName)
			mapField.Proto = field
			fields = append(fields, mapField)
			continue
		}

		if field.Oneof != "" {
			if _, ok := seenOneof[field.Oneof]; !ok {
				caseName := sanitizeFieldName(field.Oneof + "_case")
				fields = append(fields, fieldSpec{
					Name:     caseName,
					CType:    "int32_t",
					RustType: "i32",
					GoType:   "int32",
					Comment:  "oneof discriminator storing the active field number",
				})
				seenOneof[field.Oneof] = struct{}{}
			}
		}

		switch field.TypeName {
		case "string":
			b.requiresStringView = true
			if field.Repeated {
				slice := b.ensureSlice(makeSliceIdentifier("kvproto_string_view"), "kvproto_string_view", "KvprotoStringView", "StringView")
				fs := makeSliceField(fieldName, slice)
				fs.Proto = field
				fields = append(fields, fs)
			} else {
				fields = append(fields, fieldSpec{
					Name:           fieldName,
					CType:          "kvproto_string_view",
					RustType:       "KvprotoStringView",
					GoType:         "StringView",
					UsesString:     true,
					Proto:          field,
					Proto2Optional: field.Proto2Optional,
				})
			}
			continue
		case "bytes":
			b.requiresBytesView = true
			if field.Repeated {
				slice := b.ensureSlice(makeSliceIdentifier("kvproto_bytes_view"), "kvproto_bytes_view", "KvprotoBytesView", "BytesView")
				fs := makeSliceField(fieldName, slice)
				fs.Proto = field
				fields = append(fields, fs)
			} else {
				fields = append(fields, fieldSpec{
					Name:           fieldName,
					CType:          "kvproto_bytes_view",
					RustType:       "KvprotoBytesView",
					GoType:         "BytesView",
					UsesBytes:      true,
					Proto:          field,
					Proto2Optional: field.Proto2Optional,
				})
			}
			continue
		}

		if scalar, ok := scalarTypeMap[field.TypeName]; ok {
			if field.Repeated {
				slice := b.ensureSlice(makeSliceIdentifier(scalar.c), scalar.c, scalar.rust, scalar.goType)
				fs := makeSliceField(fieldName, slice)
				fs.Proto = field
				fields = append(fields, fs)
			} else {
				fields = append(fields, fieldSpec{
					Name:           fieldName,
					CType:          scalar.c,
					RustType:       scalar.rust,
					GoType:         scalar.goType,
					Proto:          field,
					Proto2Optional: field.Proto2Optional,
				})
			}
			continue
		}

		fullType := b.resolveFullName(field.TypeName, msg)
		field.FullType = fullType
		if enumFile, ok := b.enums[fullType]; ok {
			if field.Repeated {
				slice := b.ensureSlice(makeSliceIdentifier("int32_t"), "int32_t", "i32", "int32")
				fs := makeSliceField(fieldName, slice)
				fs.Proto = field
				fs.FullType = fullType
				fs.Enum = true
				fs.EnumFile = enumFile
				fields = append(fields, fs)
			} else {
				fields = append(fields, fieldSpec{
					Name:           fieldName,
					CType:          "int32_t",
					RustType:       "i32",
					GoType:         "int32",
					Proto:          field,
					FullType:       fullType,
					Enum:           true,
					EnumFile:       enumFile,
					Proto2Optional: field.Proto2Optional,
				})
			}
			continue
		}

		cType := toCTypeName(fullType)
		rustType := toRustTypeName(fullType)
		ptrC := cType + " *"
		ptrRust := "*mut " + rustType

		if target, ok := b.messages[fullType]; ok {
			goAlias := joinMessageAlias(target.PathParts)
			goType := "*" + goAlias
			if target.GoPackage != msg.GoPackage {
				goType = "*" + target.GoPackage + "." + goAlias
			}
			if field.Repeated {
				slice := b.ensureSlice(makeSliceIdentifier(ptrC), ptrC, ptrRust, goType)
				fs := makeSliceField(fieldName, slice)
				fs.Proto = field
				fs.FullType = fullType
				fields = append(fields, fs)
			} else {
				fields = append(fields, fieldSpec{
					Name:     fieldName,
					CType:    ptrC,
					RustType: ptrRust,
					GoType:   goType,
					Proto:    field,
					FullType: fullType,
				})
			}
			continue
		}

		b.externalMessages[fullType] = struct{}{}
		extPkg, extParts := splitFullName(fullType)
		goExtPkg := sanitizeGoPackageName(extPkg)
		goAlias := joinMessageAlias(extParts)
		if goAlias == "" {
			goAlias = toPascalCase(toCTypeName(fullType))
		}
		goType := "*" + goExtPkg + "." + goAlias
		if field.Repeated {
			slice := b.ensureSlice(makeSliceIdentifier(ptrC), ptrC, ptrRust, goType)
			fs := makeSliceField(fieldName, slice)
			fs.Proto = field
			fs.FullType = fullType
			fields = append(fields, fs)
		} else {
			fields = append(fields, fieldSpec{
				Name:     fieldName,
				CType:    ptrC,
				RustType: ptrRust,
				GoType:   goType,
				Proto:    field,
				FullType: fullType,
			})
		}
	}

	return fields
}

func (b *specBuilder) buildMapField(msg *protoMessage, m *protoMap, fieldName string) fieldSpec {
	entryName := toPascalCase(m.Field.Name) + "Entry"
	entryPath := append(append([]string{}, msg.PathParts...), entryName)
	entryFullName := messageFullName(entryPath, msg.Package)

	if _, ok := b.mapEntries[entryFullName]; !ok {
		entryMsg := &protoMessage{
			FullName:  entryFullName,
			Package:   msg.Package,
			GoPackage: msg.GoPackage,
			ProtoPath: msg.ProtoPath,
			PathParts: entryPath,
			Fields: []protoField{
				{Number: 1, Name: "key", TypeName: m.KeyType},
				{Number: 2, Name: "value", TypeName: m.Field.TypeName},
			},
			File:       msg.File,
			IsMapEntry: true,
		}
		entryFields := b.buildFields(entryMsg)
		entrySpec := &messageSpec{
			FullName:  entryFullName,
			Package:   msg.Package,
			GoPackage: msg.GoPackage,
			GoImport:  msg.File.GoImportPath,
			ProtoPath: msg.ProtoPath,
			CName:     toCTypeName(entryFullName),
			RustName:  toRustTypeName(entryFullName),
			GoName:    joinMessageAlias(entryPath),
			GoAlias:   joinMessageAlias(entryPath),
			Fields:    entryFields,
			File:      msg.File,
			Proto:     entryMsg,
		}
		b.mapEntries[entryFullName] = entrySpec
	}

	entry := b.mapEntries[entryFullName]
	pointerC := entry.CName + " *"
	pointerRust := "*mut " + entry.RustName
	pointerGo := "*" + entry.GoName
	slice := b.ensureSlice(makeSliceIdentifier(pointerC), pointerC, pointerRust, pointerGo)
	fs := makeSliceField(fieldName, slice)
	mapInfo := &mapFieldInfo{
		KeyType:       m.KeyType,
		KeyKind:       m.KeyKind,
		KeyScalar:     m.KeyScalar,
		EntryFullName: entryFullName,
		Value:         m.Field,
		ValueFullType: "",
	}
	if m.Field.Kind == fieldKindMessage || m.Field.Kind == fieldKindEnum {
		mapInfo.ValueFullType = b.resolveFullName(m.Field.TypeName, msg)
		mapInfo.Value.FullType = mapInfo.ValueFullType
		if m.Field.Kind == fieldKindEnum {
			if enumFile, ok := b.enums[mapInfo.ValueFullType]; ok {
				mapInfo.ValueEnumFile = enumFile
			}
		}
	}
	fs.Map = mapInfo
	return fs
}

type scalarTypes struct {
	c      string
	rust   string
	goType string
}

var scalarTypeMap = map[string]scalarTypes{
	"double":   {"double", "f64", "float64"},
	"float":    {"float", "f32", "float32"},
	"int64":    {"int64_t", "i64", "int64"},
	"uint64":   {"uint64_t", "u64", "uint64"},
	"int32":    {"int32_t", "i32", "int32"},
	"uint32":   {"uint32_t", "u32", "uint32"},
	"fixed64":  {"uint64_t", "u64", "uint64"},
	"fixed32":  {"uint32_t", "u32", "uint32"},
	"sfixed32": {"int32_t", "i32", "int32"},
	"sfixed64": {"int64_t", "i64", "int64"},
	"sint32":   {"int32_t", "i32", "int32"},
	"sint64":   {"int64_t", "i64", "int64"},
	"bool":     {"bool", "bool", "bool"},
}

func (b *specBuilder) ensureSlice(id string, cType, rustType, goType string) *sliceSpec {
	if spec, ok := b.slices[id]; ok {
		return spec
	}
	spec := &sliceSpec{
		ElementCType:    cType,
		ElementRustType: rustType,
		ElementGoType:   goType,
		CName:           "kvproto_slice_" + id,
		RustName:        "KvprotoSlice" + toPascalCase(id),
		GoName:          "Slice" + toPascalCase(id),
	}
	b.slices[id] = spec
	return spec
}

func makeSliceField(name string, spec *sliceSpec) fieldSpec {
	field := fieldSpec{
		Name:     name,
		CType:    spec.CName,
		RustType: spec.RustName,
		GoType:   spec.GoName,
		Slice:    spec,
	}
	switch spec.ElementGoType {
	case "StringView":
		field.UsesString = true
	case "BytesView":
		field.UsesBytes = true
	}
	return field
}

func (b *specBuilder) resolveFullName(typeName string, msg *protoMessage) string {
	if strings.HasPrefix(typeName, ".") {
		return typeName
	}
	if strings.Contains(typeName, ".") {
		candidate := "." + typeName
		if _, ok := b.messages[candidate]; ok {
			return candidate
		}
		if _, ok := b.enums[candidate]; ok {
			return candidate
		}
		if msg.Package != "" {
			within := "." + msg.Package + "." + typeName
			if _, ok := b.messages[within]; ok {
				return within
			}
			if _, ok := b.enums[within]; ok {
				return within
			}
		}
		return candidate
	}
	if msg.Package != "" {
		candidate := "." + msg.Package + "." + typeName
		if _, ok := b.messages[candidate]; ok {
			return candidate
		}
		if _, ok := b.enums[candidate]; ok {
			return candidate
		}
	}
	return "." + typeName
}

type externalType struct {
	FullName  string
	CName     string
	RustName  string
	GoPackage string
	GoAlias   string
}

func sortedExternal(ext map[string]struct{}, messages map[string]*protoMessage) []externalType {
	list := make([]string, 0, len(ext))
	for name := range ext {
		if _, ok := messages[name]; ok {
			continue
		}
		list = append(list, name)
	}
	sort.Strings(list)
	result := make([]externalType, 0, len(list))
	for _, name := range list {
		pkg, parts := splitFullName(name)
		goPkg := sanitizeGoPackageName(pkg)
		goAlias := joinMessageAlias(parts)
		if goAlias == "" {
			goAlias = toPascalCase(toCTypeName(name))
		}
		result = append(result, externalType{
			FullName:  name,
			CName:     toCTypeName(name),
			RustName:  toRustTypeName(name),
			GoPackage: goPkg,
			GoAlias:   goAlias,
		})
	}
	return result
}

func emitC(messages []*messageSpec, slices []*sliceSpec, needStringView, needBytesView bool, external []externalType) []byte {
	var buf bytes.Buffer
	buf.WriteString("/* Auto-generated by protoc-gen-kvffi */\n")
	buf.WriteString("#pragma once\n\n")
	buf.WriteString("#include <stdbool.h>\n#include <stddef.h>\n#include <stdint.h>\n\n")

	if needStringView {
		buf.WriteString("typedef struct kvproto_string_view {\n    const char *data;\n    size_t len;\n} kvproto_string_view;\n\n")
	}
	if needBytesView {
		buf.WriteString("typedef struct kvproto_bytes_view {\n    uint8_t *data;\n    size_t len;\n} kvproto_bytes_view;\n\n")
	}

	seenStructs := make(map[string]struct{})

	for _, msg := range messages {
		if _, ok := seenStructs[msg.CName]; ok {
			continue
		}
		seenStructs[msg.CName] = struct{}{}
		fmt.Fprintf(&buf, "typedef struct %s %s;\n", msg.CName, msg.CName)
	}
	for _, ext := range external {
		if _, ok := seenStructs[ext.CName]; ok {
			continue
		}
		seenStructs[ext.CName] = struct{}{}
		fmt.Fprintf(&buf, "typedef struct %s %s;\n", ext.CName, ext.CName)
	}
	if len(seenStructs) > 0 {
		buf.WriteByte('\n')
	}

	for _, slice := range slices {
		fmt.Fprintf(&buf, "typedef struct %s {\n    %s *data;\n    size_t len;\n    size_t cap;\n} %s;\n\n", slice.CName, slice.ElementCType, slice.CName)
	}

	emitted := make(map[string]struct{})
	for _, msg := range messages {
		if _, ok := emitted[msg.CName]; ok {
			continue
		}
		emitted[msg.CName] = struct{}{}
		fmt.Fprintf(&buf, "struct %s {\n", msg.CName)
		for _, field := range msg.Fields {
			if field.Comment != "" {
				fmt.Fprintf(&buf, "    /* %s */\n", field.Comment)
			}
			if field.Proto2Optional {
				fmt.Fprintf(&buf, "    bool %s;\n", presenceFieldName(field.Name))
			}
			fmt.Fprintf(&buf, "    %s %s;\n", field.CType, field.Name)
		}
		buf.WriteString("};\n\n")
	}

	return append(buf.Bytes(), '\n')
}

func emitRust(messages []*messageSpec, slices []*sliceSpec, needStringView, needBytesView bool, external []externalType) []byte {
	var buf bytes.Buffer
	buf.WriteString("// Auto-generated by protoc-gen-kvffi\n")
	buf.WriteString("#![allow(non_camel_case_types)]\n#![allow(non_snake_case)]\n#![allow(dead_code)]\n\n")
	buf.WriteString("use std::os::raw::c_char;\n\n")

	if needStringView {
		buf.WriteString("#[repr(C)]\npub struct KvprotoStringView {\n    pub data: *const c_char,\n    pub len: usize,\n}\n\n")
	}
	if needBytesView {
		buf.WriteString("#[repr(C)]\npub struct KvprotoBytesView {\n    pub data: *mut u8,\n    pub len: usize,\n}\n\n")
	}
	for _, slice := range slices {
		fmt.Fprintf(&buf, "#[repr(C)]\npub struct %s {\n    pub data: *mut %s,\n    pub len: usize,\n    pub cap: usize,\n}\n\n", slice.RustName, slice.ElementRustType)
	}
	for _, msg := range messages {
		fmt.Fprintf(&buf, "#[repr(C)]\npub struct %s {\n", msg.RustName)
		for _, field := range msg.Fields {
			if field.Comment != "" {
				fmt.Fprintf(&buf, "    // %s\n", field.Comment)
			}
			if field.Proto2Optional {
				fmt.Fprintf(&buf, "    pub %s: bool,\n", rustFieldName(presenceFieldName(field.Name)))
			}
			fmt.Fprintf(&buf, "    pub %s: %s,\n", rustFieldName(field.Name), field.RustType)
		}
		buf.WriteString("}\n\n")
		if alias := rustLegacyAlias(msg.CName, msg.RustName); alias != "" {
			fmt.Fprintf(&buf, "pub type %s = %s;\n\n", alias, msg.RustName)
		}
	}
	for _, ext := range external {
		fmt.Fprintf(&buf, "#[repr(C)]\npub struct %s {\n    _unused: [u8; 0],\n}\n\n", ext.RustName)
		if alias := rustLegacyAlias(ext.CName, ext.RustName); alias != "" {
			fmt.Fprintf(&buf, "pub type %s = %s;\n\n", alias, ext.RustName)
		}
	}
	return append(buf.Bytes(), '\n')
}

func buildGoFileSpecs(messages []*messageSpec, external []externalType) []*goFileSpec {
	builders := make(map[string]*goFileBuilder)
	packageIndex := make(map[string]*goFileSpec)

	for _, msg := range messages {
		builder := builders[msg.ProtoPath]
		if builder == nil {
			builder = &goFileBuilder{
				spec: &goFileSpec{
					ProtoPath:    msg.ProtoPath,
					GoPackage:    msg.GoPackage,
					PBImportPath: msg.GoImport,
				},
				sliceSet: make(map[string]*sliceSpec),
			}
			builders[msg.ProtoPath] = builder
		}
		builder.spec.Messages = append(builder.spec.Messages, msg)
		for _, field := range msg.Fields {
			if field.Slice != nil {
				builder.sliceSet[field.Slice.CName] = field.Slice
			}
			if field.UsesString {
				builder.spec.NeedStringView = true
			}
			if field.UsesBytes {
				builder.spec.NeedBytesView = true
			}
		}
	}

	keys := make([]string, 0, len(builders))
	for key := range builders {
		keys = append(keys, key)
	}
	sort.Strings(keys)

	var specs []*goFileSpec
	for _, key := range keys {
		builder := builders[key]
		sliceList := make([]*sliceSpec, 0, len(builder.sliceSet))
		for _, slice := range builder.sliceSet {
			sliceList = append(sliceList, slice)
		}
		sort.Slice(sliceList, func(i, j int) bool {
			return sliceList[i].CName < sliceList[j].CName
		})
		builder.spec.Slices = sliceList
		sort.Slice(builder.spec.Messages, func(i, j int) bool {
			return builder.spec.Messages[i].GoAlias < builder.spec.Messages[j].GoAlias
		})
		specs = append(specs, builder.spec)
		packageIndex[builder.spec.GoPackage] = builder.spec
	}

	externalGroups := make(map[string][]externalType)
	for _, ext := range external {
		externalGroups[ext.GoPackage] = append(externalGroups[ext.GoPackage], ext)
	}

	for pkg, extList := range externalGroups {
		sort.Slice(extList, func(i, j int) bool {
			if extList[i].GoAlias == extList[j].GoAlias {
				return extList[i].CName < extList[j].CName
			}
			return extList[i].GoAlias < extList[j].GoAlias
		})
		if spec, ok := packageIndex[pkg]; ok {
			spec.External = append(spec.External, extList...)
			continue
		}
		specs = append(specs, &goFileSpec{
			GoPackage:      pkg,
			External:       extList,
			IsExternalOnly: true,
		})
	}

	sort.Slice(specs, func(i, j int) bool {
		if specs[i].GoPackage == specs[j].GoPackage {
			return specs[i].ProtoPath < specs[j].ProtoPath
		}
		return specs[i].GoPackage < specs[j].GoPackage
	})

	return specs
}

func goOutputPath(spec *goFileSpec) string {
	dir := filepath.Join("ffi_out", "go", spec.GoPackage)
	filename := "abi.go"
	if spec.ProtoPath != "" {
		base := filepath.Base(spec.ProtoPath)
		ext := filepath.Ext(base)
		if ext != "" {
			base = strings.TrimSuffix(base, ext)
		}
		if base == "" {
			base = "abi"
		}
		filename = base + "_abi.go"
	} else if spec.IsExternalOnly {
		filename = "abi_external.go"
	}
	return filepath.Join(dir, filename)
}

func goConversionsPath(spec *goFileSpec) string {
	dir := filepath.Join("ffi_out", "go", spec.GoPackage)
	if spec.ProtoPath != "" {
		base := filepath.Base(spec.ProtoPath)
		ext := filepath.Ext(base)
		if ext != "" {
			base = strings.TrimSuffix(base, ext)
		}
		if base == "" {
			base = "abi"
		}
		return filepath.Join(dir, base+"_conv_gen.go")
	}
	return filepath.Join(dir, "conversions_gen.go")
}

func buildGoFile(spec *goFileSpec, headerInclude string) []byte {
	var buf bytes.Buffer
	buf.WriteString("// Code generated by protoc-gen-kvffi. DO NOT EDIT.\n\n")
	buf.WriteString("package ")
	buf.WriteString(spec.GoPackage)
	buf.WriteString("\n\n")
	buf.WriteString("/*\n")
	fmt.Fprintf(&buf, "#cgo CFLAGS: -I%s\n", headerInclude)
	buf.WriteString("#include \"kvproto_abi.h\"\n")
	buf.WriteString("*/\n")
	buf.WriteString("import \"C\"\n\n")

	wroteHeader := false
	if spec.NeedStringView {
		buf.WriteString("type StringView = C.kvproto_string_view\n")
		wroteHeader = true
	}
	if spec.NeedBytesView {
		buf.WriteString("type BytesView = C.kvproto_bytes_view\n")
		wroteHeader = true
	}
	if wroteHeader {
		buf.WriteByte('\n')
	}

	if len(spec.Slices) > 0 {
		for _, slice := range spec.Slices {
			fmt.Fprintf(&buf, "type %s = C.%s\n", slice.GoName, slice.CName)
		}
		buf.WriteByte('\n')
	}

	if len(spec.Messages) > 0 {
		for _, msg := range spec.Messages {
			fmt.Fprintf(&buf, "type %s = C.%s\n", msg.GoAlias, msg.CName)
			if alias := goUnderscoreAlias(msg.CName, msg.GoAlias); alias != "" {
				fmt.Fprintf(&buf, "type %s = C.%s\n", alias, msg.CName)
			}
		}
	}

	if len(spec.External) > 0 {
		if len(spec.Messages) > 0 {
			buf.WriteByte('\n')
		}
		for _, ext := range spec.External {
			fmt.Fprintf(&buf, "type %s = C.%s\n", ext.GoAlias, ext.CName)
			if alias := goUnderscoreAlias(ext.CName, ext.GoAlias); alias != "" {
				fmt.Fprintf(&buf, "type %s = C.%s\n", alias, ext.CName)
			}
		}
	}

	return append(buf.Bytes(), '\n')
}

func messageFullName(parts []string, pkg string) string {
	all := make([]string, 0, len(parts)+1)
	if pkg != "" {
		all = append(all, pkg)
	}
	all = append(all, parts...)
	return "." + strings.Join(all, ".")
}

func joinMessageAlias(parts []string) string {
	if len(parts) == 0 {
		return ""
	}
	alias := make([]string, len(parts))
	for i, part := range parts {
		alias[i] = toPascalCase(part)
	}
	return strings.Join(alias, "_")
}

func joinWithPkg(pkg string, parts ...string) string {
	all := make([]string, 0, len(parts)+1)
	if pkg != "" {
		all = append(all, pkg)
	}
	all = append(all, parts...)
	return strings.Join(all, ".")
}

func splitFullName(full string) (string, []string) {
	full = strings.TrimPrefix(full, ".")
	if full == "" {
		return "", nil
	}
	parts := splitQualified(full)
	if len(parts) == 0 {
		return "", nil
	}
	return parts[0], parts[1:]
}

func splitQualified(name string) []string {
	items := strings.Split(name, ".")
	out := make([]string, 0, len(items))
	for _, item := range items {
		if item != "" {
			out = append(out, item)
		}
	}
	return out
}

func sanitizeFieldName(name string) string {
	replacer := strings.NewReplacer(
		" ", "_",
		"-", "_",
	)
	name = replacer.Replace(name)
	var buf strings.Builder
	for i, r := range name {
		if isAlphaNum(r) || r == '_' {
			buf.WriteRune(r)
			continue
		}
		if i == 0 {
			buf.WriteRune('_')
		} else {
			buf.WriteRune('_')
		}
	}
	clean := buf.String()
	if clean == "" {
		clean = "_"
	}
	if reservedCKeywords[clean] || reservedGoKeywords[clean] {
		clean = clean + "_field"
	}
	return clean
}

func presenceFieldName(name string) string {
	return "has_" + name
}

func rustFieldName(name string) string {
	if _, ok := rustKeywords[name]; ok {
		return "r#" + name
	}
	return name
}

func isAlphaNum(r rune) bool {
	return r >= 'a' && r <= 'z' || r >= 'A' && r <= 'Z' || r >= '0' && r <= '9'
}

var reservedCKeywords = map[string]bool{
	"auto":     true,
	"break":    true,
	"case":     true,
	"char":     true,
	"const":    true,
	"continue": true,
	"default":  true,
	"do":       true,
	"double":   true,
	"else":     true,
	"enum":     true,
	"extern":   true,
	"float":    true,
	"for":      true,
	"goto":     true,
	"if":       true,
	"inline":   true,
	"int":      true,
	"long":     true,
	"register": true,
	"restrict": true,
	"return":   true,
	"short":    true,
	"signed":   true,
	"sizeof":   true,
	"static":   true,
	"struct":   true,
	"switch":   true,
	"typedef":  true,
	"union":    true,
	"unsigned": true,
	"void":     true,
	"volatile": true,
	"while":    true,
}

var reservedGoKeywords = map[string]bool{
	"break":       true,
	"case":        true,
	"chan":        true,
	"const":       true,
	"continue":    true,
	"default":     true,
	"defer":       true,
	"else":        true,
	"fallthrough": true,
	"for":         true,
	"func":        true,
	"go":          true,
	"goto":        true,
	"if":          true,
	"import":      true,
	"interface":   true,
	"map":         true,
	"package":     true,
	"range":       true,
	"return":      true,
	"select":      true,
	"struct":      true,
	"switch":      true,
	"type":        true,
	"var":         true,
}

var rustKeywords = map[string]struct{}{
	"as": {}, "break": {}, "const": {}, "continue": {}, "crate": {}, "else": {}, "enum": {}, "extern": {},
	"false": {}, "fn": {}, "for": {}, "if": {}, "impl": {}, "in": {}, "let": {}, "loop": {}, "match": {},
	"mod": {}, "move": {}, "mut": {}, "pub": {}, "ref": {}, "return": {}, "self": {}, "Self": {}, "static": {},
	"struct": {}, "super": {}, "trait": {}, "true": {}, "type": {}, "unsafe": {}, "use": {}, "where": {},
	"while": {}, "async": {}, "await": {}, "try": {},
}

func makeSliceIdentifier(typeRepr string) string {
	trimmed := strings.TrimSpace(typeRepr)
	trimmed = strings.ReplaceAll(trimmed, "*", "ptr")
	trimmed = strings.ReplaceAll(trimmed, " ", "_")
	var buf strings.Builder
	for _, r := range trimmed {
		if isAlphaNum(r) || r == '_' {
			buf.WriteRune(r)
		} else {
			buf.WriteRune('_')
		}
	}
	result := buf.String()
	result = strings.Trim(result, "_")
	if result == "" {
		return "unnamed"
	}
	for strings.Contains(result, "__") {
		result = strings.ReplaceAll(result, "__", "_")
	}
	return result
}

func toPascalCase(id string) string {
	var buf strings.Builder
	upperNext := true
	for _, r := range id {
		switch {
		case r == '_' || r == '-' || r == ' ':
			upperNext = true
		default:
			if upperNext {
				buf.WriteRune(unicode.ToUpper(r))
			} else {
				buf.WriteRune(r)
			}
			if unicode.IsDigit(r) {
				upperNext = true
			} else {
				upperNext = false
			}
		}
	}
	return buf.String()
}

func goGetterName(field protoField) string {
	name := goExportedName(field.Name)
	if name == "Size_" {
		return "Get" + name
	}
	if name == "Size" {
		return "Get" + name + "_"
	}
	return "Get" + name
}

func goExportedName(name string) string {
	exported := toPascalCase(name)
	if exported == "Size" {
		return "Size_"
	}
	return exported
}

func goScalarGoType(name string) string {
	switch name {
	case "int32", "sint32", "sfixed32":
		return "int32"
	case "uint32", "fixed32":
		return "uint32"
	case "int64", "sint64", "sfixed64":
		return "int64"
	case "uint64", "fixed64":
		return "uint64"
	case "bool":
		return "bool"
	case "float":
		return "float32"
	case "double":
		return "float64"
	default:
		return "int32"
	}
}

func goUnderscoreAlias(cName, current string) string {
	idx := strings.Index(cName, "_")
	if idx < 0 || idx+1 >= len(cName) {
		return ""
	}
	alias := cName[idx+1:]
	if alias == current {
		return ""
	}
	return alias
}

func rustLegacyAlias(cName, rustName string) string {
	idx := strings.Index(cName, "_")
	if idx < 0 || idx+1 >= len(cName) {
		return ""
	}
	prefix := cName[:idx]
	suffix := cName[idx+1:]
	if strings.ContainsRune(prefix, '_') || suffix == "" {
		return ""
	}
	prefixAlias := strings.ToUpper(prefix[:1]) + strings.ToLower(prefix[1:])
	cleaned := strings.ReplaceAll(suffix, "_", "")
	if cleaned == "" {
		return ""
	}
	lowerSuffix := strings.ToLower(cleaned)
	alias := prefixAlias + strings.ToUpper(lowerSuffix[:1]) + lowerSuffix[1:]
	if alias == rustName {
		return ""
	}
	return alias
}

func buildRustConversionsFile(messages []*messageSpec) []byte {
	var memberSpec, getMembersSpec *messageSpec
	for _, msg := range messages {
		if msg.Package != "pdpb" {
			continue
		}
		switch msg.GoAlias {
		case "Member":
			memberSpec = msg
		case "GetMembersResponse":
			getMembersSpec = msg
		}
	}
	if memberSpec == nil && getMembersSpec == nil {
		return nil
	}

	var buf bytes.Buffer
	buf.WriteString("//! Auto-generated conversions (feature `kvffi_gen`).\n")
	buf.WriteString("#![cfg(feature = \"kvffi_gen\")]\n\n")
	buf.WriteString("use std::collections::HashMap;\n")
	buf.WriteString("use std::os::raw::c_char;\n")
	buf.WriteString("use std::ptr;\n\n")
	buf.WriteString("use crate::ffi_runtime::abi::{\n")
	buf.WriteString("    KvprotoSliceKvprotoStringView, KvprotoSlicePtrMember, KvprotoSlicePdpbGetMembersResponseTsoAllocatorLeadersEntryPtr,\n")
	buf.WriteString("    KvprotoStringView, PdpbGetMembersResponse, PdpbGetMembersResponseTsoAllocatorLeadersEntry, PdpbMember,\n")
	buf.WriteString("};\n")
	buf.WriteString("use super::arena::{self, Arena};\n")
	buf.WriteString("use crate::pdpb;\n\n")
	buf.WriteString("use super::{response_header_from_repr, response_header_to_repr, string_slice_to_view};\n\n")

	if memberSpec != nil {
		buf.WriteString(buildRustMemberConversions(memberSpec))
		buf.WriteByte('\n')
	}
	if getMembersSpec != nil {
		buf.WriteString(buildRustGetMembersConversions(getMembersSpec))
		buf.WriteByte('\n')
	}

	return buf.Bytes()
}

func buildRustMemberConversions(msg *messageSpec) string {
	var buf bytes.Buffer
	buf.WriteString("pub fn member_to_repr_generated(arena: &mut Arena, src: &pdpb::Member) -> *mut PdpbMember {\n")
	buf.WriteString("    let name_view = if src.get_name().is_empty() {\n")
	buf.WriteString("        KvprotoStringView { data: ptr::null(), len: 0 }\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        let (ptr, len) = arena.alloc_string(src.get_name());\n")
	buf.WriteString("        KvprotoStringView { data: ptr as *const c_char, len }\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let peer_urls_view = if src.get_peer_urls().is_empty() {\n")
	buf.WriteString("        KvprotoSliceKvprotoStringView { data: ptr::null_mut(), len: 0, cap: 0 }\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        string_slice_to_view(arena, src.get_peer_urls())\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let client_urls_view = if src.get_client_urls().is_empty() {\n")
	buf.WriteString("        KvprotoSliceKvprotoStringView { data: ptr::null_mut(), len: 0, cap: 0 }\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        string_slice_to_view(arena, src.get_client_urls())\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let deploy_path_view = if src.get_deploy_path().is_empty() {\n")
	buf.WriteString("        KvprotoStringView { data: ptr::null(), len: 0 }\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        let (ptr, len) = arena.alloc_string(src.get_deploy_path());\n")
	buf.WriteString("        KvprotoStringView { data: ptr as *const c_char, len }\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let binary_version_view = if src.get_binary_version().is_empty() {\n")
	buf.WriteString("        KvprotoStringView { data: ptr::null(), len: 0 }\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        let (ptr, len) = arena.alloc_string(src.get_binary_version());\n")
	buf.WriteString("        KvprotoStringView { data: ptr as *const c_char, len }\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let git_hash_view = if src.get_git_hash().is_empty() {\n")
	buf.WriteString("        KvprotoStringView { data: ptr::null(), len: 0 }\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        let (ptr, len) = arena.alloc_string(src.get_git_hash());\n")
	buf.WriteString("        KvprotoStringView { data: ptr as *const c_char, len }\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let dc_location_view = if src.get_dc_location().is_empty() {\n")
	buf.WriteString("        KvprotoStringView { data: ptr::null(), len: 0 }\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        let (ptr, len) = arena.alloc_string(src.get_dc_location());\n")
	buf.WriteString("        KvprotoStringView { data: ptr as *const c_char, len }\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let repr = arena.alloc_struct(PdpbMember {\n")
	buf.WriteString("        name: name_view,\n")
	buf.WriteString("        member_id: src.get_member_id(),\n")
	buf.WriteString("        peer_urls: peer_urls_view,\n")
	buf.WriteString("        client_urls: client_urls_view,\n")
	buf.WriteString("        leader_priority: src.get_leader_priority(),\n")
	buf.WriteString("        deploy_path: deploy_path_view,\n")
	buf.WriteString("        binary_version: binary_version_view,\n")
	buf.WriteString("        git_hash: git_hash_view,\n")
	buf.WriteString("        dc_location: dc_location_view,\n")
	buf.WriteString("    });\n")
	buf.WriteString("    repr as *mut _\n")
	buf.WriteString("}\n\n")

	buf.WriteString("pub fn member_from_repr_generated(ptr: *const PdpbMember) -> Option<pdpb::Member> {\n")
	buf.WriteString("    if ptr.is_null() {\n")
	buf.WriteString("        return None;\n")
	buf.WriteString("    }\n")
	buf.WriteString("    let repr = unsafe { &*ptr };\n")
	buf.WriteString("    let mut out = pdpb::Member::new();\n")
	buf.WriteString("    out.set_name(arena::string_from(repr.name.data as *const u8, repr.name.len));\n")
	buf.WriteString("    out.set_member_id(repr.member_id);\n")
	buf.WriteString("    out.set_leader_priority(repr.leader_priority);\n")
	buf.WriteString("    out.set_peer_urls(::protobuf::RepeatedField::from_vec(arena::string_slice_from_view(&repr.peer_urls)));\n")
	buf.WriteString("    out.set_client_urls(::protobuf::RepeatedField::from_vec(arena::string_slice_from_view(&repr.client_urls)));\n")
	buf.WriteString("    out.set_deploy_path(arena::string_from(repr.deploy_path.data as *const u8, repr.deploy_path.len));\n")
	buf.WriteString("    out.set_binary_version(arena::string_from(repr.binary_version.data as *const u8, repr.binary_version.len));\n")
	buf.WriteString("    out.set_git_hash(arena::string_from(repr.git_hash.data as *const u8, repr.git_hash.len));\n")
	buf.WriteString("    out.set_dc_location(arena::string_from(repr.dc_location.data as *const u8, repr.dc_location.len));\n")
	buf.WriteString("    Some(out)\n")
	buf.WriteString("}\n")
	return buf.String()
}

func buildRustGetMembersConversions(msg *messageSpec) string {
	var buf bytes.Buffer
	buf.WriteString("pub fn get_members_response_to_repr_generated(arena: &mut Arena, src: &pdpb::GetMembersResponse) -> *mut PdpbGetMembersResponse {\n")
	buf.WriteString("    let header_ptr = if src.has_header() {\n")
	buf.WriteString("        response_header_to_repr(arena, src.get_header())\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        ptr::null_mut()\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let members_slice = if src.get_members().is_empty() {\n")
	buf.WriteString("        KvprotoSlicePtrMember { data: ptr::null_mut(), len: 0, cap: 0 }\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        let mut member_ptrs = Vec::with_capacity(src.get_members().len());\n")
	buf.WriteString("        for member in src.get_members() {\n")
	buf.WriteString("            member_ptrs.push(member_to_repr_generated(arena, member));\n")
	buf.WriteString("        }\n")
	buf.WriteString("        let (ptr, len) = arena.alloc_ptr_array(member_ptrs);\n")
	buf.WriteString("        KvprotoSlicePtrMember { data: ptr, len, cap: len }\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let leader_ptr = if src.has_leader() {\n")
	buf.WriteString("        member_to_repr_generated(arena, src.get_leader())\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        ptr::null_mut()\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let etcd_leader_ptr = if src.has_etcd_leader() {\n")
	buf.WriteString("        member_to_repr_generated(arena, src.get_etcd_leader())\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        ptr::null_mut()\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let tso_allocator_leaders_slice = if src.get_tso_allocator_leaders().is_empty() {\n")
	buf.WriteString("        KvprotoSlicePdpbGetMembersResponseTsoAllocatorLeadersEntryPtr { data: ptr::null_mut(), len: 0, cap: 0 }\n")
	buf.WriteString("    } else {\n")
	buf.WriteString("        let mut entries = Vec::with_capacity(src.get_tso_allocator_leaders().len());\n")
	buf.WriteString("        for (key, value) in src.get_tso_allocator_leaders() {\n")
	buf.WriteString("            let key_view = if key.is_empty() {\n")
	buf.WriteString("                KvprotoStringView { data: ptr::null(), len: 0 }\n")
	buf.WriteString("            } else {\n")
	buf.WriteString("                let (ptr, len) = arena.alloc_string(key);\n")
	buf.WriteString("                KvprotoStringView { data: ptr as *const c_char, len }\n")
	buf.WriteString("            };\n")
	buf.WriteString("            let value_ptr = member_to_repr_generated(arena, value);\n")
	buf.WriteString("            let entry_ptr = arena.alloc_struct(PdpbGetMembersResponseTsoAllocatorLeadersEntry { key: key_view, value: value_ptr }) as *mut _;\n")
	buf.WriteString("            entries.push(entry_ptr);\n")
	buf.WriteString("        }\n")
	buf.WriteString("        let (ptr, len) = arena.alloc_ptr_array(entries);\n")
	buf.WriteString("        KvprotoSlicePdpbGetMembersResponseTsoAllocatorLeadersEntryPtr { data: ptr, len, cap: len }\n")
	buf.WriteString("    };\n")
	buf.WriteString("    let repr = arena.alloc_struct(PdpbGetMembersResponse {\n")
	buf.WriteString("        header: header_ptr,\n")
	buf.WriteString("        members: members_slice,\n")
	buf.WriteString("        leader: leader_ptr,\n")
	buf.WriteString("        etcd_leader: etcd_leader_ptr,\n")
	buf.WriteString("        tso_allocator_leaders: tso_allocator_leaders_slice,\n")
	buf.WriteString("    });\n")
	buf.WriteString("    repr as *mut _\n")
	buf.WriteString("}\n\n")

	buf.WriteString("pub fn get_members_response_from_repr_generated(repr: *const PdpbGetMembersResponse) -> Option<pdpb::GetMembersResponse> {\n")
	buf.WriteString("    if repr.is_null() {\n")
	buf.WriteString("        return None;\n")
	buf.WriteString("    }\n")
	buf.WriteString("    let repr = unsafe { &*repr };\n")
	buf.WriteString("    let mut out = pdpb::GetMembersResponse::new();\n")
	buf.WriteString("    if !repr.header.is_null() {\n")
	buf.WriteString("        if let Some(header) = response_header_from_repr(repr.header) {\n")
	buf.WriteString("            out.set_header(header);\n")
	buf.WriteString("        }\n")
	buf.WriteString("    }\n")
	buf.WriteString("    if !repr.members.data.is_null() && repr.members.len > 0 {\n")
	buf.WriteString("        let slice = unsafe { std::slice::from_raw_parts(repr.members.data, repr.members.len) };\n")
	buf.WriteString("        for &ptr in slice {\n")
	buf.WriteString("            if let Some(member) = member_from_repr_generated(ptr) {\n")
	buf.WriteString("                out.mut_members().push(member);\n")
	buf.WriteString("            }\n")
	buf.WriteString("        }\n")
	buf.WriteString("    }\n")
	buf.WriteString("    if let Some(leader) = member_from_repr_generated(repr.leader) {\n")
	buf.WriteString("        out.set_leader(leader);\n")
	buf.WriteString("    }\n")
	buf.WriteString("    if let Some(leader) = member_from_repr_generated(repr.etcd_leader) {\n")
	buf.WriteString("        out.set_etcd_leader(leader);\n")
	buf.WriteString("    }\n")
	buf.WriteString("    if !repr.tso_allocator_leaders.data.is_null() && repr.tso_allocator_leaders.len > 0 {\n")
	buf.WriteString("        let slice = unsafe {\n")
	buf.WriteString("            std::slice::from_raw_parts(repr.tso_allocator_leaders.data, repr.tso_allocator_leaders.len)\n")
	buf.WriteString("        };\n")
	buf.WriteString("        let mut map = HashMap::with_capacity(slice.len());\n")
	buf.WriteString("        for &entry_ptr in slice {\n")
	buf.WriteString("            if entry_ptr.is_null() {\n")
	buf.WriteString("                continue;\n")
	buf.WriteString("            }\n")
	buf.WriteString("            let entry = unsafe { &*entry_ptr };\n")
	buf.WriteString("            let key = arena::string_from(entry.key.data as *const u8, entry.key.len);\n")
	buf.WriteString("            let value = member_from_repr_generated(entry.value).unwrap_or_default();\n")
	buf.WriteString("            map.insert(key, value);\n")
	buf.WriteString("        }\n")
	buf.WriteString("        out.tso_allocator_leaders = map;\n")
	buf.WriteString("    }\n")
	buf.WriteString("    Some(out)\n")
	buf.WriteString("}\n")
	return buf.String()
}

func sanitizeGoPackageName(name string) string {
	name = strings.TrimSpace(name)
	if name == "" {
		return "kvprotoabi"
	}
	name = strings.ReplaceAll(name, ".", "_")
	name = strings.ReplaceAll(name, "-", "_")
	name = strings.ReplaceAll(name, " ", "_")
	name = strings.ToLower(name)
	if name == "" {
		return "kvprotoabi"
	}
	return name
}

func goPackageName(pkg, protoPath string) string {
	if pkg != "" {
		return sanitizeGoPackageName(pkg)
	}
	base := filepath.Base(protoPath)
	ext := filepath.Ext(base)
	if ext != "" {
		base = strings.TrimSuffix(base, ext)
	}
	if base == "" {
		return "kvprotoabi"
	}
	return sanitizeGoPackageName(base)
}

func goImportPath(protoPath string) string {
	switch protoPath {
	case "google/protobuf/descriptor.proto", "include/google/protobuf/descriptor.proto":
		return "github.com/gogo/protobuf/protoc-gen-gogo/descriptor"
	}
	base := filepath.Base(protoPath)
	ext := filepath.Ext(base)
	if ext != "" {
		base = strings.TrimSuffix(base, ext)
	}
	if base == "" {
		base = "kvprotoabi"
	}
	return "github.com/pingcap/kvproto/pkg/" + base
}

func toCTypeName(full string) string {
	full = strings.TrimPrefix(full, ".")
	if full == "" {
		return "kvproto_unknown"
	}
	parts := splitQualified(full)
	return strings.Join(parts, "_")
}

func toRustTypeName(full string) string {
	full = strings.TrimPrefix(full, ".")
	if full == "" {
		return "KvprotoUnknown"
	}
	parts := splitQualified(full)
	for i, part := range parts {
		parts[i] = toPascalCase(part)
	}
	return strings.Join(parts, "")
}
