package main

import (
	"bytes"
	"encoding/json"
	"flag"
	"fmt"
	"os"
	"path/filepath"
	"sort"
	"strings"
)

type protoLock struct {
	Definitions []definition `json:"definitions"`
}

type definition struct {
	ProtoPath string         `json:"protopath"`
	Def       definitionBody `json:"def"`
}

type definitionBody struct {
	Package  packageDef   `json:"package"`
	Enums    []enumDef    `json:"enums"`
	Messages []messageDef `json:"messages"`
}

type packageDef struct {
	Name string `json:"name"`
}

type protoFile struct {
	ProtoPath    string
	Package      string
	GoPackage    string
	GoImportPath string
}

type enumDef struct {
	Name string `json:"name"`
}

type messageDef struct {
	Name     string       `json:"name"`
	Fields   []fieldDef   `json:"fields"`
	Messages []messageDef `json:"messages"`
	Maps     []mapDef     `json:"maps"`
}

type mapDef struct {
	KeyType string      `json:"key_type"`
	Field   mapFieldDef `json:"field"`
}

type mapFieldDef struct {
	ID   int    `json:"id"`
	Name string `json:"name"`
	Type string `json:"type"`
}

type fieldDef struct {
	ID          int    `json:"id"`
	Name        string `json:"name"`
	Type        string `json:"type"`
	IsRepeated  bool   `json:"is_repeated"`
	OneofParent string `json:"oneof_parent"`
}

type protoField struct {
	Number   int
	Name     string
	TypeName string
	Repeated bool
	Oneof    string
}

type protoMessage struct {
	FullName  string
	Package   string
	GoPackage string
	ProtoPath string
	PathParts []string
	Fields    []protoField
	Maps      []protoMap
	File      *protoFile
}

type protoMap struct {
	Field   protoField
	KeyType string
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

type fieldSpec struct {
	Name       string
	CType      string
	RustType   string
	GoType     string
	Comment    string
	Slice      *sliceSpec
	UsesString bool
	UsesBytes  bool
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

type specBuilder struct {
	messages map[string]*protoMessage
	enums    map[string]struct{}
	files    map[string]*protoFile

	slices     map[string]*sliceSpec
	mapEntries map[string]*messageSpec

	requiresStringView bool
	requiresBytesView  bool

	externalMessages map[string]struct{}
}

func main() {
	var (
		protoLockPath = flag.String("proto-lock", "scripts/proto.lock", "Path to proto.lock schema snapshot.")
		outDir        = flag.String("out", "", "Output directory for generated files.")
		goPackage     = flag.String("go-package", "kvprotoffi", "Go package name for generated bindings.")
		headerInclude = flag.String("header-include", "../c", "Relative include path used by Go cgo header.")
	)
	flag.Parse()

	if *outDir == "" {
		fmt.Fprintf(os.Stderr, "--out is required\n")
		os.Exit(1)
	}

	lock, err := os.ReadFile(*protoLockPath)
	if err != nil {
		fmt.Fprintf(os.Stderr, "failed to read proto lock %s: %v\n", *protoLockPath, err)
		os.Exit(1)
	}

	var lockData protoLock
	if err := json.Unmarshal(lock, &lockData); err != nil {
		fmt.Fprintf(os.Stderr, "failed to parse proto lock: %v\n", err)
		os.Exit(1)
	}

	messages, enums, files := collectSchema(lockData)

	if *goPackage != "" {
		// Retained for backward compatibility; individual Go packages are derived per proto file.
		_ = goPackage
	}

	builder := &specBuilder{
		messages:         messages,
		enums:            enums,
		files:            files,
		slices:           make(map[string]*sliceSpec),
		mapEntries:       make(map[string]*messageSpec),
		externalMessages: make(map[string]struct{}),
	}

	messageSpecs := builder.buildMessageSpecs()
	sliceSpecs := builder.sortedSlices()

	messageNames := make(map[string]struct{}, len(messageSpecs))
	for _, spec := range messageSpecs {
		messageNames[spec.FullName] = struct{}{}
	}

	externalList := sortedExternal(builder.externalMessages, messageNames)

	if err := writeAll(*outDir, *headerInclude, messageSpecs, sliceSpecs, builder.requiresStringView, builder.requiresBytesView, externalList); err != nil {
		fmt.Fprintf(os.Stderr, "generation failed: %v\n", err)
		os.Exit(1)
	}
}

func collectSchema(lock protoLock) (map[string]*protoMessage, map[string]struct{}, map[string]*protoFile) {
	messages := make(map[string]*protoMessage)
	enumRecords := make([]enumRecord, 0)
	files := make(map[string]*protoFile)

	for _, def := range lock.Definitions {
		pkg := def.Def.Package.Name
		goPkg := goPackageName(pkg, def.ProtoPath)
		file := &protoFile{
			ProtoPath:    def.ProtoPath,
			Package:      pkg,
			GoPackage:    goPkg,
			GoImportPath: goImportPath(def.ProtoPath),
		}
		files[def.ProtoPath] = file
		for _, enum := range def.Def.Enums {
			enumRecords = append(enumRecords, enumRecord{Package: pkg, RawName: enum.Name})
		}

		var walk func(stack []string, msgs []messageDef)
		walk = func(stack []string, msgs []messageDef) {
			for _, msg := range msgs {
				currentStack := append(stack, msg.Name)
				fullName := messageFullName(currentStack, pkg)
				fields := make([]protoField, 0, len(msg.Fields))
				for _, f := range msg.Fields {
					fields = append(fields, protoField{
						Number:   f.ID,
						Name:     f.Name,
						TypeName: f.Type,
						Repeated: f.IsRepeated,
						Oneof:    f.OneofParent,
					})
				}
				maps := make([]protoMap, 0, len(msg.Maps))
				for _, mp := range msg.Maps {
					maps = append(maps, protoMap{
						Field: protoField{
							Number:   mp.Field.ID,
							Name:     mp.Field.Name,
							TypeName: mp.Field.Type,
						},
						KeyType: mp.KeyType,
					})
				}

				messages[fullName] = &protoMessage{
					FullName:  fullName,
					Package:   pkg,
					GoPackage: goPkg,
					ProtoPath: def.ProtoPath,
					PathParts: append([]string(nil), currentStack...),
					Fields:    fields,
					Maps:      maps,
					File:      file,
				}

				if len(msg.Messages) > 0 {
					walk(currentStack, msg.Messages)
				}
			}
		}

		walk(nil, def.Def.Messages)
	}

	enums := make(map[string]struct{})
	for _, rec := range enumRecords {
		full := resolveEnumFullName(rec.RawName, rec.Package, messages)
		enums[full] = struct{}{}
	}

	return messages, enums, files
}

type enumRecord struct {
	Package string
	RawName string
}

func messageFullName(parts []string, pkg string) string {
	all := make([]string, 0, len(parts)+1)
	if pkg != "" {
		all = append(all, pkg)
	}
	all = append(all, parts...)
	return "." + strings.Join(all, ".")
}

func resolveEnumFullName(raw, pkg string, messages map[string]*protoMessage) string {
	partList := splitQualified(raw)
	if len(partList) == 0 {
		return ""
	}

	if len(partList) == 1 {
		if pkg != "" {
			return "." + pkg + "." + partList[0]
		}
		return "." + partList[0]
	}

	scopeParts := partList[:len(partList)-1]
	enumName := partList[len(partList)-1]

	var best *protoMessage
	for _, msg := range messages {
		if msg.Package != pkg {
			continue
		}
		if len(msg.PathParts) < len(scopeParts) {
			continue
		}
		if equalTail(msg.PathParts, scopeParts) {
			if best == nil || len(msg.PathParts) > len(best.PathParts) {
				best = msg
			}
		}
	}

	var finalParts []string
	if pkg != "" {
		finalParts = append(finalParts, pkg)
	}
	if best != nil {
		finalParts = append(finalParts, best.PathParts...)
	} else {
		finalParts = append(finalParts, scopeParts...)
	}
	finalParts = append(finalParts, enumName)
	return "." + strings.Join(finalParts, ".")
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

func equalTail(haystack, suffix []string) bool {
	if len(haystack) < len(suffix) {
		return false
	}
	start := len(haystack) - len(suffix)
	for i := range suffix {
		if haystack[start+i] != suffix[i] {
			return false
		}
	}
	return true
}

func joinMessageAlias(parts []string) string {
	if len(parts) == 0 {
		return ""
	}
	return strings.Join(parts, "_")
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
			fields = append(fields, b.buildMapField(msg, input.mapInfo, fieldName))
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
				fields = append(fields, makeSliceField(fieldName, slice))
			} else {
				fields = append(fields, fieldSpec{
					Name:       fieldName,
					CType:      "kvproto_string_view",
					RustType:   "KvprotoStringView",
					GoType:     "StringView",
					UsesString: true,
				})
			}
			continue
		case "bytes":
			b.requiresBytesView = true
			if field.Repeated {
				slice := b.ensureSlice(makeSliceIdentifier("kvproto_bytes_view"), "kvproto_bytes_view", "KvprotoBytesView", "BytesView")
				fields = append(fields, makeSliceField(fieldName, slice))
			} else {
				fields = append(fields, fieldSpec{
					Name:      fieldName,
					CType:     "kvproto_bytes_view",
					RustType:  "KvprotoBytesView",
					GoType:    "BytesView",
					UsesBytes: true,
				})
			}
			continue
		}

		if scalar, ok := scalarTypeMap[field.TypeName]; ok {
			if field.Repeated {
				slice := b.ensureSlice(makeSliceIdentifier(scalar.c), scalar.c, scalar.rust, scalar.goType)
				fields = append(fields, makeSliceField(fieldName, slice))
			} else {
				fields = append(fields, fieldSpec{
					Name:     fieldName,
					CType:    scalar.c,
					RustType: scalar.rust,
					GoType:   scalar.goType,
				})
			}
			continue
		}

		fullType := b.resolveFullName(field.TypeName, msg)
		if _, ok := b.enums[fullType]; ok {
			if field.Repeated {
				slice := b.ensureSlice(makeSliceIdentifier("int32_t"), "int32_t", "i32", "int32")
				fields = append(fields, makeSliceField(fieldName, slice))
			} else {
				fields = append(fields, fieldSpec{
					Name:     fieldName,
					CType:    "int32_t",
					RustType: "i32",
					GoType:   "int32",
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
				slice := b.ensureSlice(makeSliceIdentifier(goType), ptrC, ptrRust, goType)
				fields = append(fields, makeSliceField(fieldName, slice))
			} else {
				fields = append(fields, fieldSpec{
					Name:     fieldName,
					CType:    ptrC,
					RustType: ptrRust,
					GoType:   goType,
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
			slice := b.ensureSlice(makeSliceIdentifier(goType), ptrC, ptrRust, goType)
			fields = append(fields, makeSliceField(fieldName, slice))
		} else {
			fields = append(fields, fieldSpec{
				Name:     fieldName,
				CType:    ptrC,
				RustType: ptrRust,
				GoType:   goType,
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
			File: msg.File,
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
	return makeSliceField(fieldName, slice)
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
		return "." + typeName
	}

	scope := append([]string{}, msg.PathParts...)
	for len(scope) > 0 {
		candidate := "." + joinWithPkg(msg.Package, append(scope, typeName)...)
		if _, ok := b.messages[candidate]; ok || containsEnum(b.enums, candidate) {
			return candidate
		}
		scope = scope[:len(scope)-1]
	}

	if msg.Package != "" {
		candidate := "." + msg.Package + "." + typeName
		if _, ok := b.messages[candidate]; ok || containsEnum(b.enums, candidate) {
			return candidate
		}
	}
	return "." + typeName
}

func containsEnum(set map[string]struct{}, key string) bool {
	_, ok := set[key]
	return ok
}

var rustKeywords = map[string]struct{}{
	"as": {}, "break": {}, "const": {}, "continue": {}, "crate": {}, "else": {}, "enum": {}, "extern": {},
	"false": {}, "fn": {}, "for": {}, "if": {}, "impl": {}, "in": {}, "let": {}, "loop": {}, "match": {},
	"mod": {}, "move": {}, "mut": {}, "pub": {}, "ref": {}, "return": {}, "self": {}, "Self": {}, "static": {},
	"struct": {}, "super": {}, "trait": {}, "true": {}, "type": {}, "unsafe": {}, "use": {}, "where": {},
	"while": {}, "async": {}, "await": {}, "try": {},
}

func rustFieldName(name string) string {
	if _, ok := rustKeywords[name]; ok {
		return "r#" + name
	}
	return name
}

func joinWithPkg(pkg string, parts ...string) string {
	all := make([]string, 0, len(parts)+1)
	if pkg != "" {
		all = append(all, pkg)
	}
	all = append(all, parts...)
	return strings.Join(all, ".")
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
	if reservedCKeywords[clean] {
		clean = clean + "_field"
	}
	return clean
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
	parts := strings.Split(id, "_")
	var buf strings.Builder
	for _, part := range parts {
		if part == "" {
			continue
		}
		buf.WriteString(strings.ToUpper(part[:1]))
		if len(part) > 1 {
			buf.WriteString(part[1:])
		}
	}
	if buf.Len() == 0 {
		return "Id"
	}
	return buf.String()
}

func toCTypeName(full string) string {
	full = strings.TrimPrefix(full, ".")
	full = strings.ReplaceAll(full, ".", "_")
	return sanitizeIdentifier(full)
}

func toRustTypeName(full string) string {
	name := toCTypeName(full)
	parts := strings.Split(name, "_")
	var buf strings.Builder
	for _, part := range parts {
		if part == "" {
			continue
		}
		buf.WriteString(strings.ToUpper(part[:1]))
		if len(part) > 1 {
			buf.WriteString(strings.ToLower(part[1:]))
		}
	}
	return buf.String()
}

func toGoTypeName(full string) string {
	return toRustTypeName(full)
}

func sanitizeIdentifier(in string) string {
	var buf strings.Builder
	for _, r := range in {
		if isAlphaNum(r) {
			buf.WriteRune(r)
		} else {
			buf.WriteRune('_')
		}
	}
	return buf.String()
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

type externalType struct {
	FullName  string
	CName     string
	RustName  string
	GoPackage string
	GoAlias   string
}

func sortedExternal(ext map[string]struct{}, messages map[string]struct{}) []externalType {
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

func writeAll(outDir, headerInclude string, messages []*messageSpec, slices []*sliceSpec, needStringView, needBytesView bool, external []externalType) error {
	cDir := filepath.Join(outDir, "c")
	rustDir := filepath.Join(outDir, "rust")
	goDir := filepath.Join(outDir, "go")

	for _, dir := range []string{cDir, rustDir, goDir} {
		if err := os.MkdirAll(dir, 0o755); err != nil {
			return err
		}
	}

	if err := writeFile(filepath.Join(cDir, "kvproto_abi.h"), emitC(messages, slices, needStringView, needBytesView, external)); err != nil {
		return err
	}
	if err := writeFile(filepath.Join(rustDir, "kvproto_abi.rs"), emitRust(messages, slices, needStringView, needBytesView, external)); err != nil {
		return err
	}
	goSpecs := buildGoFileSpecs(messages, external)
	if err := emitGoPackages(goDir, headerInclude, goSpecs); err != nil {
		return err
	}
	return nil
}

type goFileBuilder struct {
	spec     *goFileSpec
	sliceSet map[string]*sliceSpec
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

	specs := make([]*goFileSpec, 0, len(keys))
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

func emitGoPackages(goDir, headerInclude string, specs []*goFileSpec) error {
	for _, spec := range specs {
		dir := filepath.Join(goDir, spec.GoPackage)
		if err := os.MkdirAll(dir, 0o755); err != nil {
			return err
		}
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
		content := buildGoFile(spec, headerInclude)
		if err := writeFile(filepath.Join(dir, filename), content); err != nil {
			return err
		}
	}
	return nil
}

func buildGoFile(spec *goFileSpec, headerInclude string) []byte {
	var buf bytes.Buffer
	buf.WriteString("// Code generated by tools/abi_gen/generate.go. DO NOT EDIT.\n\n")
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
		}
	}

	if len(spec.External) > 0 {
		if len(spec.Messages) > 0 {
			buf.WriteByte('\n')
		}
		for _, ext := range spec.External {
			fmt.Fprintf(&buf, "type %s = C.%s\n", ext.GoAlias, ext.CName)
		}
	}

	return append(buf.Bytes(), '\n')
}

func writeFile(path string, content []byte) error {
	return os.WriteFile(path, content, 0o644)
}

func emitC(messages []*messageSpec, slices []*sliceSpec, needStringView, needBytesView bool, external []externalType) []byte {
	var buf bytes.Buffer
	buf.WriteString("/* Auto-generated by tools/abi_gen/generate.go */\n")
	buf.WriteString("#pragma once\n\n")
	buf.WriteString("#include <stdbool.h>\n#include <stddef.h>\n#include <stdint.h>\n\n")

	if needStringView {
		buf.WriteString("typedef struct kvproto_string_view {\n    const char *data;\n    size_t len;\n} kvproto_string_view;\n\n")
	}
	if needBytesView {
		buf.WriteString("typedef struct kvproto_bytes_view {\n    uint8_t *data;\n    size_t len;\n} kvproto_bytes_view;\n\n")
	}

	for _, msg := range messages {
		fmt.Fprintf(&buf, "typedef struct %s %s;\n", msg.CName, msg.CName)
	}
	for _, ext := range external {
		fmt.Fprintf(&buf, "typedef struct %s %s;\n", ext.CName, ext.CName)
	}
	if len(messages) > 0 || len(external) > 0 {
		buf.WriteByte('\n')
	}

	for _, slice := range slices {
		fmt.Fprintf(&buf, "typedef struct %s {\n    %s *data;\n    size_t len;\n    size_t cap;\n} %s;\n\n", slice.CName, slice.ElementCType, slice.CName)
	}

	for _, msg := range messages {
		fmt.Fprintf(&buf, "struct %s {\n", msg.CName)
		for _, field := range msg.Fields {
			if field.Comment != "" {
				fmt.Fprintf(&buf, "    /* %s */\n", field.Comment)
			}
			fmt.Fprintf(&buf, "    %s %s;\n", field.CType, field.Name)
		}
		buf.WriteString("};\n\n")
	}

	return append(buf.Bytes(), '\n')
}

func emitRust(messages []*messageSpec, slices []*sliceSpec, needStringView, needBytesView bool, external []externalType) []byte {
	var buf bytes.Buffer
	buf.WriteString("// Auto-generated by tools/abi_gen/generate.go\n")
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
			fmt.Fprintf(&buf, "    pub %s: %s,\n", rustFieldName(field.Name), field.RustType)
		}
		buf.WriteString("}\n\n")
	}
	for _, ext := range external {
		fmt.Fprintf(&buf, "#[repr(C)]\npub struct %s {\n    _unused: [u8; 0],\n}\n\n", ext.RustName)
	}
	return append(buf.Bytes(), '\n')
}
