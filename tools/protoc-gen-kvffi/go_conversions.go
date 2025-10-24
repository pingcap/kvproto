package main

import (
	"bytes"
	"fmt"
	"path"
	"sort"
	"strings"
)

func buildGoConversionsFile(spec *goFileSpec) []byte {
	if spec.IsExternalOnly || len(spec.Messages) == 0 {
		return nil
	}
	builder := newGoConversionsBuilder(spec)
	return builder.build()
}

type goConversionsBuilder struct {
	spec *goFileSpec

	body bytes.Buffer

	pbAliases map[string]string
	pbUsed    map[string]bool
	pbOrder   []string

	ffiAliases map[string]string
	ffiUsed    map[string]bool
	ffiOrder   []string

	customAliases map[string]string
	customUsed    map[string]bool
	customOrder   []string

	aliasInUse    map[string]struct{}
	aliasCounters map[string]int

	pbMainPath  string
	pbMainAlias string

	useUnsafe bool
}

func newGoConversionsBuilder(spec *goFileSpec) *goConversionsBuilder {
	b := &goConversionsBuilder{
		spec:          spec,
		pbAliases:     make(map[string]string),
		pbUsed:        make(map[string]bool),
		pbOrder:       make([]string, 0),
		ffiAliases:    make(map[string]string),
		ffiUsed:       make(map[string]bool),
		ffiOrder:      make([]string, 0),
		customAliases: make(map[string]string),
		customUsed:    make(map[string]bool),
		customOrder:   make([]string, 0),
		aliasInUse:    make(map[string]struct{}),
		aliasCounters: make(map[string]int),
	}

	b.reserveAlias("runtime")

	b.pbMainPath = spec.PBImportPath
	b.pbMainAlias = b.registerPbAlias(spec.PBImportPath)
	b.pbUsed[spec.PBImportPath] = true

	return b
}

func (b *goConversionsBuilder) build() []byte {
	for _, msg := range b.spec.Messages {
		b.emitMessageConversions(msg)
	}
	header := b.buildHeader()
	var out bytes.Buffer
	out.Write(header)
	out.Write(b.body.Bytes())
	return out.Bytes()
}

type goImport struct {
	alias string
	path  string
}

type oneofGroup struct {
	Name     string
	CaseName string
	Fields   []protoField
}

func (b *goConversionsBuilder) emitMessageConversions(msg *messageSpec) {
	if msg.Proto != nil && msg.Proto.IsMapEntry {
		return
	}
	pbAlias := b.usePbAlias(msg.GoImport)

	if b.body.Len() > 0 {
		b.body.WriteByte('\n')
	}
	b.emitNewRepr(msg, pbAlias)
	b.body.WriteByte('\n')
	b.emitIntoRepr(msg, pbAlias)
	b.body.WriteByte('\n')
	b.emitFromRepr(msg, pbAlias)
}

func (b *goConversionsBuilder) emitNewRepr(msg *messageSpec, pbAlias string) {
	goName := msg.GoAlias
	fmt.Fprintf(&b.body, "func NewRepr%sGenerated(arena *runtime.Arena, src *%s.%s) *%s {\n", goName, pbAlias, goName, goName)
	b.body.WriteString("\tif arena == nil || src == nil {\n\t\treturn nil\n\t}\n")
	fmt.Fprintf(&b.body, "\tptr := (*%s)(arena.AllocZero(uintptr(C.sizeof_%s)))\n", goName, msg.CName)
	fmt.Fprintf(&b.body, "\tIntoRepr%sGenerated(arena, ptr, src)\n", goName)
	b.body.WriteString("\treturn ptr\n}\n")
}

func (b *goConversionsBuilder) emitIntoRepr(msg *messageSpec, pbAlias string) {
	goName := msg.GoAlias
	fmt.Fprintf(&b.body, "func IntoRepr%sGenerated(arena *runtime.Arena, dst *%s, src *%s.%s) {\n", goName, goName, pbAlias, goName)
	b.body.WriteString("\tif arena == nil || dst == nil || src == nil {\n\t\treturn\n\t}\n")
	for i := range msg.Fields {
		fs := &msg.Fields[i]
		pf := fs.Proto
		if pf.Number == 0 {
			continue
		}
		if pf.Oneof != "" {
			continue
		}
		b.emitIntoField(msg, fs, pf)
	}
	fieldSpecs := b.fieldSpecIndex(msg)
	for _, group := range buildOneofGroups(msg) {
		b.emitIntoOneof(msg, group, fieldSpecs, pbAlias)
	}
	b.body.WriteString("}\n")
}

func (b *goConversionsBuilder) emitFromRepr(msg *messageSpec, pbAlias string) {
	goName := msg.GoAlias
	fmt.Fprintf(&b.body, "func FromRepr%sGenerated(src *%s) *%s.%s {\n", goName, goName, pbAlias, goName)
	b.body.WriteString("\tif src == nil {\n\t\treturn nil\n\t}\n")
	fmt.Fprintf(&b.body, "\tout := &%s.%s{}\n", pbAlias, goName)
	for i := range msg.Fields {
		fs := &msg.Fields[i]
		pf := fs.Proto
		if pf.Number == 0 {
			continue
		}
		if pf.Oneof != "" {
			continue
		}
		b.emitFromField(msg, fs, pf)
	}
	fieldSpecs := b.fieldSpecIndex(msg)
	for _, group := range buildOneofGroups(msg) {
		b.emitFromOneof(msg, group, fieldSpecs, pbAlias)
	}
	b.body.WriteString("\treturn out\n}\n")
}

func (b *goConversionsBuilder) buildHeader() []byte {
	var buf bytes.Buffer
	buf.WriteString("//go:build kvffi_gen\n")
	buf.WriteString("// +build kvffi_gen\n\n")
	fmt.Fprintf(&buf, "package %s\n\n", b.spec.GoPackage)
	buf.WriteString("/*\n")
	buf.WriteString("#cgo CFLAGS: -I../../c\n")
	buf.WriteString("#include \"kvproto_abi.h\"\n")
	buf.WriteString("*/\n")
	buf.WriteString("import \"C\"\n\n")

	imports := b.collectImports()
	if len(imports) > 0 {
		buf.WriteString("import (\n")
		for _, imp := range imports {
			if imp.alias == "" {
				fmt.Fprintf(&buf, "\t\"%s\"\n", imp.path)
			} else {
				fmt.Fprintf(&buf, "\t%s \"%s\"\n", imp.alias, imp.path)
			}
		}
		buf.WriteString(")\n\n")
	} else {
		buf.WriteByte('\n')
	}
	return buf.Bytes()
}

func (b *goConversionsBuilder) collectImports() []goImport {
	var std []goImport
	var others []goImport

	if b.useUnsafe {
		std = append(std, goImport{path: "unsafe"})
	}

	others = append(others, goImport{alias: "runtime", path: "github.com/pingcap/kvproto/ffi_out/go/runtime"})

	for _, p := range b.pbOrder {
		if !b.pbUsed[p] {
			continue
		}
		alias := b.pbAliases[p]
		others = append(others, goImport{alias: alias, path: p})
	}

	for _, p := range b.ffiOrder {
		if !b.ffiUsed[p] {
			continue
		}
		alias := b.ffiAliases[p]
		others = append(others, goImport{alias: alias, path: p})
	}

	for _, p := range b.customOrder {
		if !b.customUsed[p] {
			continue
		}
		alias := b.customAliases[p]
		others = append(others, goImport{alias: alias, path: p})
	}

	sort.Slice(std, func(i, j int) bool { return std[i].path < std[j].path })
	sort.Slice(others, func(i, j int) bool { return others[i].path < others[j].path })

	return append(std, others...)
}

func (b *goConversionsBuilder) fieldSpecIndex(msg *messageSpec) map[int]*fieldSpec {
	result := make(map[int]*fieldSpec, len(msg.Fields))
	for i := range msg.Fields {
		num := msg.Fields[i].Proto.Number
		if num != 0 {
			result[num] = &msg.Fields[i]
		}
	}
	return result
}

func buildOneofGroups(msg *messageSpec) []oneofGroup {
	groupMap := make(map[string]*oneofGroup)
	order := make([]string, 0)
	for _, pf := range msg.Proto.Fields {
		if pf.Oneof == "" {
			continue
		}
		group := groupMap[pf.Oneof]
		if group == nil {
			group = &oneofGroup{
				Name:     pf.Oneof,
				CaseName: sanitizeFieldName(pf.Oneof + "_case"),
				Fields:   make([]protoField, 0),
			}
			groupMap[pf.Oneof] = group
			order = append(order, pf.Oneof)
		}
		group.Fields = append(group.Fields, pf)
	}
	result := make([]oneofGroup, 0, len(order))
	for _, name := range order {
		result = append(result, *groupMap[name])
	}
	return result
}

func (b *goConversionsBuilder) reserveAlias(alias string) {
	if alias == "" {
		return
	}
	b.aliasInUse[alias] = struct{}{}
}

func (b *goConversionsBuilder) uniqueAlias(base string) string {
	base = strings.TrimSpace(base)
	if base == "" {
		base = "alias"
	}
	if _, exists := b.aliasInUse[base]; !exists {
		b.aliasInUse[base] = struct{}{}
		return base
	}
	idx := b.aliasCounters[base]
	for {
		idx++
		candidate := fmt.Sprintf("%s%d", base, idx)
		if _, ok := b.aliasInUse[candidate]; !ok {
			b.aliasCounters[base] = idx
			b.aliasInUse[candidate] = struct{}{}
			return candidate
		}
	}
}

func (b *goConversionsBuilder) registerPbAlias(importPath string) string {
	if alias, ok := b.pbAliases[importPath]; ok {
		return alias
	}
	base := sanitizeGoPackageName(path.Base(importPath))
	if base == "" {
		base = "pb"
	}
	alias := b.uniqueAlias(base + "proto")
	b.pbAliases[importPath] = alias
	b.pbOrder = append(b.pbOrder, importPath)
	return alias
}

func (b *goConversionsBuilder) usePbAlias(importPath string) string {
	alias := b.registerPbAlias(importPath)
	b.pbUsed[importPath] = true
	return alias
}

func (b *goConversionsBuilder) registerFfiAlias(goPkg string) (string, string) {
	importPath := fmt.Sprintf("github.com/pingcap/kvproto/ffi_out/go/%s", goPkg)
	if alias, ok := b.ffiAliases[importPath]; ok {
		return alias, importPath
	}
	base := sanitizeGoPackageName(goPkg)
	if base == "" {
		base = "ffi"
	}
	alias := b.uniqueAlias(base + "ffi")
	b.ffiAliases[importPath] = alias
	b.ffiOrder = append(b.ffiOrder, importPath)
	return alias, importPath
}

func (b *goConversionsBuilder) useFfiAlias(goPkg string) string {
	alias, importPath := b.registerFfiAlias(goPkg)
	b.ffiUsed[importPath] = true
	return alias
}

func (b *goConversionsBuilder) useCustomAlias(importPath string) string {
	if alias, ok := b.customAliases[importPath]; ok {
		b.customUsed[importPath] = true
		return alias
	}
	base := sanitizeGoPackageName(path.Base(importPath))
	if base == "" {
		base = "custom"
	}
	alias := b.uniqueAlias(base)
	b.customAliases[importPath] = alias
	b.customOrder = append(b.customOrder, importPath)
	b.customUsed[importPath] = true
	return alias
}

func (b *goConversionsBuilder) markUnsafe() {
	b.useUnsafe = true
}

func (b *goConversionsBuilder) emitIntoField(msg *messageSpec, fs *fieldSpec, pf protoField) {
	switch {
	case fs.Map != nil:
		b.emitIntoMapField(msg, fs, pf)
	case fs.Slice != nil && fs.UsesString:
		b.emitIntoRepeatedString(fs, pf)
	case fs.Slice != nil && fs.UsesBytes:
		b.emitIntoRepeatedBytes(fs, pf)
	case fs.Slice != nil && pf.Kind == fieldKindMessage:
		b.emitIntoRepeatedMessage(msg, fs, pf)
	case fs.Slice != nil:
		b.emitIntoRepeatedScalar(msg, fs, pf)
	case fs.UsesString:
		b.emitIntoStringField(fs, pf)
	case fs.UsesBytes:
		b.emitIntoBytesField(fs, pf)
	case pf.Kind == fieldKindMessage:
		b.emitIntoMessageField(msg, fs, pf)
	case pf.Kind == fieldKindEnum:
		b.emitIntoEnumField(fs, pf)
	case pf.Kind == fieldKindScalar:
		b.emitIntoScalarField(fs, pf)
	}
}

func (b *goConversionsBuilder) emitFromField(msg *messageSpec, fs *fieldSpec, pf protoField) {
	switch {
	case fs.Map != nil:
		b.emitFromMapField(msg, fs, pf)
	case fs.Slice != nil && fs.UsesString:
		b.emitFromRepeatedString(fs, pf)
	case fs.Slice != nil && fs.UsesBytes:
		b.emitFromRepeatedBytes(fs, pf)
	case fs.Slice != nil && pf.Kind == fieldKindMessage:
		b.emitFromRepeatedMessage(fs, pf)
	case fs.Slice != nil:
		b.emitFromRepeatedScalar(fs, pf)
	case fs.UsesString:
		b.emitFromStringField(fs, pf)
	case fs.UsesBytes:
		b.emitFromBytesField(fs, pf)
	case pf.Kind == fieldKindMessage:
		b.emitFromMessageField(msg, fs, pf)
	case pf.Kind == fieldKindEnum:
		b.emitFromEnumField(fs, pf)
	case pf.Kind == fieldKindScalar:
		b.emitFromScalarField(fs, pf)
	}
}

func (b *goConversionsBuilder) emitIntoOneof(msg *messageSpec, group oneofGroup, fieldSpecs map[int]*fieldSpec, pbAlias string) {
	fmt.Fprintf(&b.body, "	dst.%s = 0\n", group.CaseName)
	method := fmt.Sprintf("src.Get%s()", goExportedName(group.Name))
	fmt.Fprintf(&b.body, "	switch value := %s.(type) {\n", method)
	for _, pf := range group.Fields {
		fs := fieldSpecs[pf.Number]
		if fs == nil {
			continue
		}
		wrapper := fmt.Sprintf("%s_%s", msg.GoAlias, goExportedName(pf.Name))
		if fs.Message != nil && fs.Message.GoAlias == wrapper {
			wrapper += "_"
		}
		fmt.Fprintf(&b.body, "	case *%s.%s:\n", pbAlias, wrapper)
		fmt.Fprintf(&b.body, "		dst.%s = C.int32_t(%d)\n", group.CaseName, pf.Number)
		b.emitIntoOneofField(fs, pf, fmt.Sprintf("value.%s", goExportedName(pf.Name)), "		")
	}
	b.body.WriteString("	default:\n")
	fmt.Fprintf(&b.body, "		dst.%s = 0\n", group.CaseName)
	b.body.WriteString("	}\n")
}

func (b *goConversionsBuilder) emitFromOneof(msg *messageSpec, group oneofGroup, fieldSpecs map[int]*fieldSpec, pbAlias string) {
	fmt.Fprintf(&b.body, "	switch int32(src.%s) {\n", group.CaseName)
	for _, pf := range group.Fields {
		fs := fieldSpecs[pf.Number]
		if fs == nil {
			continue
		}
		wrapper := fmt.Sprintf("%s_%s", msg.GoAlias, goExportedName(pf.Name))
		if fs.Message != nil && fs.Message.GoAlias == wrapper {
			wrapper += "_"
		}
		fmt.Fprintf(&b.body, "	case %d:\n", pf.Number)
		fmt.Fprintf(&b.body, "		out.%s = &%s.%s{\n", goExportedName(group.Name), pbAlias, wrapper)
		b.emitFromOneofField(fs, pf, "			")
		b.body.WriteString("		}\n")
	}
	b.body.WriteString("	}\n")
}

func (b *goConversionsBuilder) emitIntoOneofField(fs *fieldSpec, pf protoField, valueExpr, indent string) {
	switch {
	case fs.UsesString:
		fmt.Fprintf(&b.body, "%sif data, length := arena.AllocString(%s); length > 0 {\n", indent, valueExpr)
		fmt.Fprintf(&b.body, "%s	dst.%s.data = (*C.char)(data)\n", indent, fs.Name)
		fmt.Fprintf(&b.body, "%s	dst.%s.len = C.size_t(length)\n", indent, fs.Name)
		fmt.Fprintf(&b.body, "%s} else {\n", indent)
		fmt.Fprintf(&b.body, "%s	dst.%s.data = nil\n", indent, fs.Name)
		fmt.Fprintf(&b.body, "%s	dst.%s.len = 0\n", indent, fs.Name)
		fmt.Fprintf(&b.body, "%s}\n", indent)
	case fs.UsesBytes:
		fmt.Fprintf(&b.body, "%sif data, length := arena.AllocBytes(%s); length > 0 {\n", indent, valueExpr)
		fmt.Fprintf(&b.body, "%s	dst.%s.data = (*C.uint8_t)(data)\n", indent, fs.Name)
		fmt.Fprintf(&b.body, "%s	dst.%s.len = C.size_t(length)\n", indent, fs.Name)
		fmt.Fprintf(&b.body, "%s} else {\n", indent)
		fmt.Fprintf(&b.body, "%s	dst.%s.data = nil\n", indent, fs.Name)
		fmt.Fprintf(&b.body, "%s	dst.%s.len = 0\n", indent, fs.Name)
		fmt.Fprintf(&b.body, "%s}\n", indent)
	case pf.Kind == fieldKindMessage:
		fmt.Fprintf(&b.body, "%sif %s != nil {\n", indent, valueExpr)
		fmt.Fprintf(&b.body, "%s	dst.%s = %s\n", indent, fs.Name, b.messageIntoCall(fs.Message, valueExpr))
		fmt.Fprintf(&b.body, "%s} else {\n", indent)
		fmt.Fprintf(&b.body, "%s	dst.%s = nil\n", indent, fs.Name)
		fmt.Fprintf(&b.body, "%s}\n", indent)
	case pf.Kind == fieldKindEnum:
		fmt.Fprintf(&b.body, "%sdst.%s = C.int32_t(int32(%s))\n", indent, fs.Name, valueExpr)
	case pf.Kind == fieldKindScalar:
		fmt.Fprintf(&b.body, "%sdst.%s = %s\n", indent, fs.Name, b.cScalarCast(fs.CType, valueExpr))
	default:
		fmt.Fprintf(&b.body, "%sdst.%s = %s\n", indent, fs.Name, b.cScalarCast(fs.CType, valueExpr))
	}
}

func (b *goConversionsBuilder) emitFromOneofField(fs *fieldSpec, pf protoField, indent string) {
	fieldName := goExportedName(pf.Name)
	valueExpr := b.oneofValueFrom(fs, pf, fmt.Sprintf("src.%s", fs.Name))
	fmt.Fprintf(&b.body, "%s%s: %s,\n", indent, fieldName, valueExpr)
}

func (b *goConversionsBuilder) oneofValueFrom(fs *fieldSpec, pf protoField, expr string) string {
	switch {
	case fs.UsesString:
		b.markUnsafe()
		return fmt.Sprintf("runtime.StringFrom(unsafe.Pointer(%s.data), int(%s.len))", expr, expr)
	case fs.UsesBytes:
		b.markUnsafe()
		return fmt.Sprintf("runtime.BytesFrom(unsafe.Pointer(%s.data), int(%s.len))", expr, expr)
	case pf.Kind == fieldKindMessage:
		return b.messageFromCall(fs.Message, expr)
	case pf.Kind == fieldKindEnum:
		enumType := b.enumQualifiedType(fs.EnumFile, pf.FullType)
		return fmt.Sprintf("%s(int32(%s))", enumType, expr)
	case pf.Kind == fieldKindScalar:
		goType := goScalarGoType(pf.TypeName)
		return b.goScalarFromC(goType, expr)
	default:
		return expr
	}
}

func (b *goConversionsBuilder) emitIntoStringField(fs *fieldSpec, pf protoField) {
	name := fs.Name
	if fs.Proto2Optional {
		presence := presenceFieldName(name)
		fieldName := goExportedName(pf.Name)
		fmt.Fprintf(&b.body, "	if src.%s != nil {\n", fieldName)
		fmt.Fprintf(&b.body, "		dst.%s = C.bool(true)\n", presence)
		fmt.Fprintf(&b.body, "		if data, length := arena.AllocString(*src.%s); length > 0 {\n", fieldName)
		fmt.Fprintf(&b.body, "			dst.%s.data = (*C.char)(data)\n", name)
		fmt.Fprintf(&b.body, "			dst.%s.len = C.size_t(length)\n", name)
		b.body.WriteString("		} else {\n")
		fmt.Fprintf(&b.body, "			dst.%s.data = nil\n", name)
		fmt.Fprintf(&b.body, "			dst.%s.len = 0\n", name)
		b.body.WriteString("		}\n")
		b.body.WriteString("	} else {\n")
		fmt.Fprintf(&b.body, "		dst.%s = C.bool(false)\n", presence)
		b.body.WriteString("	}\n")
	} else {
		getter := fmt.Sprintf("src.%s()", goGetterName(pf))
		fmt.Fprintf(&b.body, "	if data, length := arena.AllocString(%s); length > 0 {\n", getter)
		fmt.Fprintf(&b.body, "		dst.%s.data = (*C.char)(data)\n", name)
		fmt.Fprintf(&b.body, "		dst.%s.len = C.size_t(length)\n", name)
		b.body.WriteString("	}\n")
	}
}

func (b *goConversionsBuilder) emitFromStringField(fs *fieldSpec, pf protoField) {
	name := fs.Name
	exported := goExportedName(pf.Name)
	b.markUnsafe()
	if fs.Proto2Optional {
		presence := presenceFieldName(name)
		fmt.Fprintf(&b.body, "	if src.%s != C.bool(false) {\n", presence)
		fmt.Fprintf(&b.body, "		value := runtime.StringFrom(unsafe.Pointer(src.%s.data), int(src.%s.len))\n", name, name)
		fmt.Fprintf(&b.body, "		out.%s = &value\n", exported)
		b.body.WriteString("	} else {\n")
		fmt.Fprintf(&b.body, "		out.%s = nil\n", exported)
		b.body.WriteString("	}\n")
	} else {
		fmt.Fprintf(&b.body, "	out.%s = runtime.StringFrom(unsafe.Pointer(src.%s.data), int(src.%s.len))\n", exported, name, name)
	}
}

func (b *goConversionsBuilder) emitIntoBytesField(fs *fieldSpec, pf protoField) {
	name := fs.Name
	if pf.CustomType != "" && !pf.Proto2Optional && !pf.Nullable {
		fieldExpr := fmt.Sprintf("src.%s", goExportedName(pf.Name))
		fmt.Fprintf(&b.body, "	if data, length := arena.AllocBytes([]byte(%s)); length > 0 {\n", fieldExpr)
		fmt.Fprintf(&b.body, "		dst.%s.data = (*C.uint8_t)(data)\n", name)
		fmt.Fprintf(&b.body, "		dst.%s.len = C.size_t(length)\n", name)
		b.body.WriteString("	}\n")
		return
	}
	if fs.Proto2Optional {
		presence := presenceFieldName(name)
		fieldName := goExportedName(pf.Name)
		fmt.Fprintf(&b.body, "	if src.%s != nil {\n", fieldName)
		fmt.Fprintf(&b.body, "		dst.%s = C.bool(true)\n", presence)
		fmt.Fprintf(&b.body, "		if data, length := arena.AllocBytes(src.%s); length > 0 {\n", fieldName)
		fmt.Fprintf(&b.body, "			dst.%s.data = (*C.uint8_t)(data)\n", name)
		fmt.Fprintf(&b.body, "			dst.%s.len = C.size_t(length)\n", name)
		b.body.WriteString("		} else {\n")
		fmt.Fprintf(&b.body, "			dst.%s.data = nil\n", name)
		fmt.Fprintf(&b.body, "			dst.%s.len = 0\n", name)
		b.body.WriteString("		}\n")
		b.body.WriteString("	} else {\n")
		fmt.Fprintf(&b.body, "		dst.%s = C.bool(false)\n", presence)
		b.body.WriteString("	}\n")
	} else {
		getter := fmt.Sprintf("src.%s()", goGetterName(pf))
		fmt.Fprintf(&b.body, "	if data, length := arena.AllocBytes(%s); length > 0 {\n", getter)
		fmt.Fprintf(&b.body, "		dst.%s.data = (*C.uint8_t)(data)\n", name)
		fmt.Fprintf(&b.body, "		dst.%s.len = C.size_t(length)\n", name)
		b.body.WriteString("	}\n")
	}
}

func (b *goConversionsBuilder) emitFromBytesField(fs *fieldSpec, pf protoField) {
	name := fs.Name
	exported := goExportedName(pf.Name)
	b.markUnsafe()
	if pf.CustomType != "" && !pf.Proto2Optional {
		pkgPath, typeName := splitCustomType(pf.CustomType)
		alias := b.useCustomAlias(pkgPath)
		if pf.Nullable {
			fmt.Fprintf(&b.body, "	if src.%s != nil {\n", name)
			fmt.Fprintf(&b.body, "		value := runtime.BytesFrom(unsafe.Pointer(src.%s.data), int(src.%s.len))\n", name, name)
			fmt.Fprintf(&b.body, "		out.%s = %s.%s(value)\n", exported, alias, typeName)
			b.body.WriteString("	}\n")
		} else {
			fmt.Fprintf(&b.body, "	out.%s = %s.%s(runtime.BytesFrom(unsafe.Pointer(src.%s.data), int(src.%s.len)))\n", exported, alias, typeName, name, name)
		}
		return
	}
	if fs.Proto2Optional {
		presence := presenceFieldName(name)
		fmt.Fprintf(&b.body, "	if src.%s != C.bool(false) {\n", presence)
		fmt.Fprintf(&b.body, "		value := runtime.BytesFrom(unsafe.Pointer(src.%s.data), int(src.%s.len))\n", name, name)
		b.body.WriteString("		if value == nil {\n")
		b.body.WriteString("			value = make([]byte, 0)\n")
		b.body.WriteString("		}\n")
		fmt.Fprintf(&b.body, "		out.%s = value\n", exported)
		b.body.WriteString("	} else {\n")
		fmt.Fprintf(&b.body, "		out.%s = nil\n", exported)
		b.body.WriteString("	}\n")
	} else {
		fmt.Fprintf(&b.body, "	out.%s = runtime.BytesFrom(unsafe.Pointer(src.%s.data), int(src.%s.len))\n", exported, name, name)
	}
}

func (b *goConversionsBuilder) emitIntoScalarField(fs *fieldSpec, pf protoField) {
	name := fs.Name
	if fs.Proto2Optional {
		presence := presenceFieldName(name)
		fieldName := goExportedName(pf.Name)
		fmt.Fprintf(&b.body, "	if src.%s != nil {\n", fieldName)
		fmt.Fprintf(&b.body, "		dst.%s = C.bool(true)\n", presence)
		fmt.Fprintf(&b.body, "		dst.%s = %s\n", name, b.cScalarCast(fs.CType, fmt.Sprintf("*src.%s", fieldName)))
		b.body.WriteString("	} else {\n")
		fmt.Fprintf(&b.body, "		dst.%s = C.bool(false)\n", presence)
		b.body.WriteString("	}\n")
	} else {
		getter := fmt.Sprintf("src.%s()", goGetterName(pf))
		fmt.Fprintf(&b.body, "	dst.%s = %s\n", name, b.cScalarCast(fs.CType, getter))
	}
}

func (b *goConversionsBuilder) emitFromScalarField(fs *fieldSpec, pf protoField) {
	name := fs.Name
	exported := goExportedName(pf.Name)
	goType := goScalarGoType(pf.TypeName)
	valueExpr := b.goScalarFromC(goType, fmt.Sprintf("src.%s", name))
	if fs.Proto2Optional {
		presence := presenceFieldName(name)
		fmt.Fprintf(&b.body, "	if src.%s != C.bool(false) {\n", presence)
		fmt.Fprintf(&b.body, "		value := %s\n", valueExpr)
		fmt.Fprintf(&b.body, "		out.%s = &value\n", exported)
		b.body.WriteString("	} else {\n")
		fmt.Fprintf(&b.body, "		out.%s = nil\n", exported)
		b.body.WriteString("	}\n")
	} else {
		fmt.Fprintf(&b.body, "	out.%s = %s\n", exported, valueExpr)
	}
}

func (b *goConversionsBuilder) emitIntoEnumField(fs *fieldSpec, pf protoField) {
	name := fs.Name
	if fs.Proto2Optional {
		presence := presenceFieldName(name)
		fieldName := goExportedName(pf.Name)
		fmt.Fprintf(&b.body, "	if src.%s != nil {\n", fieldName)
		fmt.Fprintf(&b.body, "		dst.%s = C.bool(true)\n", presence)
		fmt.Fprintf(&b.body, "		dst.%s = C.int32_t(int32(*src.%s))\n", name, fieldName)
		b.body.WriteString("	} else {\n")
		fmt.Fprintf(&b.body, "		dst.%s = C.bool(false)\n", presence)
		b.body.WriteString("	}\n")
	} else {
		getter := fmt.Sprintf("src.%s()", goGetterName(pf))
		fmt.Fprintf(&b.body, "	dst.%s = C.int32_t(int32(%s))\n", name, getter)
	}
}

func (b *goConversionsBuilder) emitFromEnumField(fs *fieldSpec, pf protoField) {
	name := fs.Name
	exported := goExportedName(pf.Name)
	enumType := b.enumQualifiedType(fs.EnumFile, pf.FullType)
	if fs.Proto2Optional {
		presence := presenceFieldName(name)
		fmt.Fprintf(&b.body, "	if src.%s != C.bool(false) {\n", presence)
		fmt.Fprintf(&b.body, "		value := %s(int32(src.%s))\n", enumType, name)
		fmt.Fprintf(&b.body, "		out.%s = &value\n", exported)
		b.body.WriteString("	} else {\n")
		fmt.Fprintf(&b.body, "		out.%s = nil\n", exported)
		b.body.WriteString("	}\n")
	} else {
		fmt.Fprintf(&b.body, "	out.%s = %s(int32(src.%s))\n", exported, enumType, name)
	}
}

func (b *goConversionsBuilder) emitIntoMessageField(msg *messageSpec, fs *fieldSpec, pf protoField) {
	name := fs.Name
	getter := fmt.Sprintf("src.%s()", goGetterName(pf))
	if fs.Proto.Nullable {
		fmt.Fprintf(&b.body, "	if value := %s; value != nil {\n", getter)
		fmt.Fprintf(&b.body, "		dst.%s = %s\n", name, b.messageIntoCall(fs.Message, "value"))
		b.body.WriteString("	} else {\n")
		fmt.Fprintf(&b.body, "		dst.%s = nil\n", name)
		b.body.WriteString("	}\n")
	} else {
		fmt.Fprintf(&b.body, "	{\n")
		fmt.Fprintf(&b.body, "	\tvalue := %s\n", getter)
		fmt.Fprintf(&b.body, "	\tdst.%s = %s\n", name, b.messageIntoCall(fs.Message, "&value"))
		b.body.WriteString("	}\n")
	}
}

func (b *goConversionsBuilder) emitFromMessageField(msg *messageSpec, fs *fieldSpec, pf protoField) {
	name := fs.Name
	exported := goExportedName(pf.Name)
	if fs.Proto.Nullable {
		fmt.Fprintf(&b.body, "	if src.%s != nil {\n", name)
		fmt.Fprintf(&b.body, "		out.%s = %s\n", exported, b.messageFromCall(fs.Message, fmt.Sprintf("src.%s", name)))
		b.body.WriteString("	}\n")
	} else {
		fmt.Fprintf(&b.body, "	if src.%s != nil {\n", name)
		result := b.messageFromCall(fs.Message, fmt.Sprintf("src.%s", name))
		fmt.Fprintf(&b.body, "		if result := %s; result != nil {\n", result)
		fmt.Fprintf(&b.body, "			out.%s = *result\n", exported)
		b.body.WriteString("		}\n")
		b.body.WriteString("	}\n")
	}
}

func (b *goConversionsBuilder) emitIntoRepeatedString(fs *fieldSpec, pf protoField) {
	getter := fmt.Sprintf("src.%s()", goGetterName(pf))
	b.markUnsafe()
	fmt.Fprintf(&b.body, "	runtime.SetStringSlice(arena, unsafe.Pointer(&dst.%s), %s)\n", fs.Name, getter)
}

func (b *goConversionsBuilder) emitFromRepeatedString(fs *fieldSpec, pf protoField) {
	b.markUnsafe()
	fmt.Fprintf(&b.body, "	out.%s = runtime.CopyStringSlice(unsafe.Pointer(&src.%s))\n", goExportedName(pf.Name), fs.Name)
}

func (b *goConversionsBuilder) emitIntoRepeatedBytes(fs *fieldSpec, pf protoField) {
	getter := fmt.Sprintf("src.%s()", goGetterName(pf))
	b.markUnsafe()
	fmt.Fprintf(&b.body, "	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.%s), %s)\n", fs.Name, getter)
}

func (b *goConversionsBuilder) emitFromRepeatedBytes(fs *fieldSpec, pf protoField) {
	b.markUnsafe()
	fmt.Fprintf(&b.body, "	out.%s = runtime.CopyBytesSlice(unsafe.Pointer(&src.%s))\n", goExportedName(pf.Name), fs.Name)
}

func (b *goConversionsBuilder) emitIntoRepeatedMessage(msg *messageSpec, fs *fieldSpec, pf protoField) {
	getter := fmt.Sprintf("src.%s()", goGetterName(pf))
	reprType := b.reprTypeSymbol(fs.Message)
	fmt.Fprintf(&b.body, "	if values := %s; len(values) > 0 {\n", getter)
	b.markUnsafe()
	fmt.Fprintf(&b.body, "		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*%s)(nil)))\n", reprType)
	fmt.Fprintf(&b.body, "		array := unsafe.Slice((**%s)(ptr), len(values))\n", reprType)
	if fs.Proto.Nullable {
		fmt.Fprintf(&b.body, "		for i, value := range values {\n")
		fmt.Fprintf(&b.body, "			array[i] = %s\n", b.messageIntoCall(fs.Message, "value"))
		fmt.Fprintf(&b.body, "		}\n")
	} else {
		fmt.Fprintf(&b.body, "		for i := range values {\n")
		fmt.Fprintf(&b.body, "			value := values[i]\n")
		fmt.Fprintf(&b.body, "			array[i] = %s\n", b.messageIntoCall(fs.Message, "&value"))
		fmt.Fprintf(&b.body, "		}\n")
	}
	fmt.Fprintf(&b.body, "		dst.%s.data = (**%s)(ptr)\n", fs.Name, reprType)
	fmt.Fprintf(&b.body, "		dst.%s.len = C.size_t(len(values))\n", fs.Name)
	fmt.Fprintf(&b.body, "		dst.%s.cap = C.size_t(len(values))\n", fs.Name)
	fmt.Fprintf(&b.body, "	}\n")
}

func (b *goConversionsBuilder) emitFromRepeatedMessage(fs *fieldSpec, pf protoField) {
	exported := goExportedName(pf.Name)
	reprType := b.reprTypeSymbol(fs.Message)
	b.markUnsafe()
	fmt.Fprintf(&b.body, "	if src.%s.data != nil && src.%s.len > 0 {\n", fs.Name, fs.Name)
	fmt.Fprintf(&b.body, "		length := int(src.%s.len)\n", fs.Name)
	fmt.Fprintf(&b.body, "		ptrs := unsafe.Slice((**%s)(unsafe.Pointer(src.%s.data)), length)\n", reprType, fs.Name)
	pbType := b.pbMessageType(fs.Message)
	if fs.Proto.Nullable {
		fmt.Fprintf(&b.body, "		out.%s = make([]*%s, 0, length)\n", exported, pbType)
		fmt.Fprintf(&b.body, "		for _, ptr := range ptrs {\n")
		fmt.Fprintf(&b.body, "			if ptr == nil {\n")
		b.body.WriteString("				continue\n")
		fmt.Fprintf(&b.body, "			}\n")
		fmt.Fprintf(&b.body, "			out.%s = append(out.%s, %s)\n", exported, exported, b.messageFromCall(fs.Message, "ptr"))
		fmt.Fprintf(&b.body, "		}\n")
	} else {
		fmt.Fprintf(&b.body, "		out.%s = make([]%s, 0, length)\n", exported, pbType)
		fmt.Fprintf(&b.body, "		for _, ptr := range ptrs {\n")
		fmt.Fprintf(&b.body, "			if ptr == nil {\n")
		b.body.WriteString("				continue\n")
		fmt.Fprintf(&b.body, "			}\n")
		result := b.messageFromCall(fs.Message, "ptr")
		fmt.Fprintf(&b.body, "			if result := %s; result != nil {\n", result)
		fmt.Fprintf(&b.body, "				out.%s = append(out.%s, *result)\n", exported, exported)
		b.body.WriteString("			}\n")
		fmt.Fprintf(&b.body, "		}\n")
	}
	fmt.Fprintf(&b.body, "	}\n")
}

func (b *goConversionsBuilder) emitIntoRepeatedScalar(msg *messageSpec, fs *fieldSpec, pf protoField) {
	getter := fmt.Sprintf("src.%s()", goGetterName(pf))
	elemC := fs.Slice.ElementCType
	fmt.Fprintf(&b.body, "	if values := %s; len(values) > 0 {\n", getter)
	b.markUnsafe()
	fmt.Fprintf(&b.body, "		ptr := arena.AllocZero(uintptr(len(values)) * %s)\n", b.scalarElementSize(elemC))
	fmt.Fprintf(&b.body, "		array := unsafe.Slice((*C.%s)(ptr), len(values))\n", elemC)
	fmt.Fprintf(&b.body, "		for i, value := range values {\n")
	if elemC == "bool" {
		b.body.WriteString("			if value {\n")
		b.body.WriteString("				array[i] = C.bool(true)\n")
		b.body.WriteString("			} else {\n")
		b.body.WriteString("				array[i] = C.bool(false)\n")
		b.body.WriteString("			}\n")
	} else if pf.Kind == fieldKindEnum {
		b.body.WriteString("			array[i] = C.int32_t(int32(value))\n")
	} else {
		fmt.Fprintf(&b.body, "			array[i] = C.%s(value)\n", elemC)
	}
	fmt.Fprintf(&b.body, "		}\n")
	fmt.Fprintf(&b.body, "		dst.%s.data = (*C.%s)(ptr)\n", fs.Name, elemC)
	fmt.Fprintf(&b.body, "		dst.%s.len = C.size_t(len(values))\n", fs.Name)
	fmt.Fprintf(&b.body, "		dst.%s.cap = C.size_t(len(values))\n", fs.Name)
	fmt.Fprintf(&b.body, "	}\n")
}

func (b *goConversionsBuilder) emitFromRepeatedScalar(fs *fieldSpec, pf protoField) {
	elemC := fs.Slice.ElementCType
	exported := goExportedName(pf.Name)
	b.markUnsafe()
	fmt.Fprintf(&b.body, "	if src.%s.data != nil && src.%s.len > 0 {\n", fs.Name, fs.Name)
	fmt.Fprintf(&b.body, "		length := int(src.%s.len)\n", fs.Name)
	fmt.Fprintf(&b.body, "		values := unsafe.Slice((*C.%s)(unsafe.Pointer(src.%s.data)), length)\n", elemC, fs.Name)
	if pf.Kind == fieldKindEnum {
		enumType := b.enumQualifiedType(fs.EnumFile, pf.FullType)
		fmt.Fprintf(&b.body, "		out.%s = make([]%s, 0, length)\n", exported, enumType)
		b.body.WriteString("		for _, value := range values {\n")
		fmt.Fprintf(&b.body, "			out.%s = append(out.%s, %s(int32(value)))\n", exported, exported, enumType)
		b.body.WriteString("		}\n")
	} else {
		goType := goScalarGoType(pf.TypeName)
		fmt.Fprintf(&b.body, "		out.%s = make([]%s, 0, length)\n", exported, goType)
		b.body.WriteString("		for _, value := range values {\n")
		fmt.Fprintf(&b.body, "			out.%s = append(out.%s, %s)\n", exported, exported, b.goScalarFromC(goType, "value"))
		b.body.WriteString("		}\n")
	}
	b.body.WriteString("	}\n")
}

func (b *goConversionsBuilder) scalarElementSize(elemC string) string {
	if elemC == "bool" {
		return "unsafe.Sizeof(C.bool(false))"
	}
	return fmt.Sprintf("unsafe.Sizeof(C.%s(0))", elemC)
}

func (b *goConversionsBuilder) emitIntoMapField(msg *messageSpec, fs *fieldSpec, pf protoField) {
	getter := fmt.Sprintf("src.%s()", goGetterName(pf))
	entry := fs.Map.Entry
	reprType := b.reprTypeSymbol(entry)
	fmt.Fprintf(&b.body, "	if values := %s; len(values) > 0 {\n", getter)
	b.markUnsafe()
	fmt.Fprintf(&b.body, "		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*%s)(nil)))\n", reprType)
	fmt.Fprintf(&b.body, "		array := unsafe.Slice((**%s)(ptr), len(values))\n", reprType)
	b.body.WriteString("		idx := 0\n")
	b.body.WriteString("		for key, value := range values {\n")
	fmt.Fprintf(&b.body, "			entry := (*%s)(arena.AllocZero(uintptr(C.sizeof_%s)))\n", reprType, entry.CName)
	keySpec := mapEntryField(entry, "key")
	valueSpec := mapEntryField(entry, "value")
	b.writeMapKeyInto("entry", keySpec, fs.Map, "key", "			")
	b.writeMapValueInto("entry", valueSpec, fs.Map, "value", "			")
	b.body.WriteString("			array[idx] = entry\n")
	b.body.WriteString("			idx++\n")
	b.body.WriteString("		}\n")
	fmt.Fprintf(&b.body, "		dst.%s.data = (**%s)(ptr)\n", fs.Name, reprType)
	fmt.Fprintf(&b.body, "		dst.%s.len = C.size_t(len(values))\n", fs.Name)
	fmt.Fprintf(&b.body, "		dst.%s.cap = C.size_t(len(values))\n", fs.Name)
	fmt.Fprintf(&b.body, "	}\n")
}

func (b *goConversionsBuilder) emitFromMapField(msg *messageSpec, fs *fieldSpec, pf protoField) {
	entry := fs.Map.Entry
	reprType := b.reprTypeSymbol(entry)
	keySpec := mapEntryField(entry, "key")
	valueSpec := mapEntryField(entry, "value")
	b.markUnsafe()
	fmt.Fprintf(&b.body, "	if src.%s.data != nil && src.%s.len > 0 {\n", fs.Name, fs.Name)
	fmt.Fprintf(&b.body, "		length := int(src.%s.len)\n", fs.Name)
	fmt.Fprintf(&b.body, "		ptrs := unsafe.Slice((**%s)(unsafe.Pointer(src.%s.data)), length)\n", reprType, fs.Name)
	keyType := b.mapKeyGoType(fs.Map)
	valueType := b.mapValueGoType(fs.Map)
	fmt.Fprintf(&b.body, "		m := make(map[%s]%s, length)\n", keyType, valueType)
	b.body.WriteString("		for _, entry := range ptrs {\n")
	b.body.WriteString("			if entry == nil {\n")
	b.body.WriteString("				continue\n")
	b.body.WriteString("			}\n")
	keyExpr := b.mapKeyExpression(keySpec, fs.Map, "entry")
	valueExpr := b.mapValueExpression(valueSpec, fs.Map, "entry")
	fmt.Fprintf(&b.body, "			key := %s\n", keyExpr)
	fmt.Fprintf(&b.body, "			value := %s\n", valueExpr)
	b.body.WriteString("			m[key] = value\n")
	b.body.WriteString("		}\n")
	fmt.Fprintf(&b.body, "		out.%s = m\n", goExportedName(pf.Name))
	b.body.WriteString("	}\n")
}

func (b *goConversionsBuilder) writeMapKeyInto(entryVar string, keySpec *fieldSpec, info *mapFieldInfo, keyVar, indent string) {
	switch info.KeyKind {
	case fieldKindString:
		fmt.Fprintf(&b.body, "%sif data, length := arena.AllocString(%s); length > 0 {\n", indent, keyVar)
		fmt.Fprintf(&b.body, "%s	%s.%s.data = (*C.char)(data)\n", indent, entryVar, keySpec.Name)
		fmt.Fprintf(&b.body, "%s	%s.%s.len = C.size_t(length)\n", indent, entryVar, keySpec.Name)
		fmt.Fprintf(&b.body, "%s} else {\n", indent)
		fmt.Fprintf(&b.body, "%s	%s.%s.data = nil\n", indent, entryVar, keySpec.Name)
		fmt.Fprintf(&b.body, "%s	%s.%s.len = 0\n", indent, entryVar, keySpec.Name)
		fmt.Fprintf(&b.body, "%s}\n", indent)
	default:
		fmt.Fprintf(&b.body, "%s%s.%s = %s\n", indent, entryVar, keySpec.Name, b.cScalarCast(keySpec.CType, keyVar))
	}
}

func (b *goConversionsBuilder) writeMapValueInto(entryVar string, valueSpec *fieldSpec, info *mapFieldInfo, valueVar, indent string) {
	switch info.Value.Kind {
	case fieldKindMessage:
		fmt.Fprintf(&b.body, "%sif %s != nil {\n", indent, valueVar)
		fmt.Fprintf(&b.body, "%s	%s.%s = %s\n", indent, entryVar, valueSpec.Name, b.messageIntoCall(info.ValueMessage, valueVar))
		fmt.Fprintf(&b.body, "%s} else {\n", indent)
		fmt.Fprintf(&b.body, "%s	%s.%s = nil\n", indent, entryVar, valueSpec.Name)
		fmt.Fprintf(&b.body, "%s}\n", indent)
	case fieldKindString:
		fmt.Fprintf(&b.body, "%sif data, length := arena.AllocString(%s); length > 0 {\n", indent, valueVar)
		fmt.Fprintf(&b.body, "%s	%s.%s.data = (*C.char)(data)\n", indent, entryVar, valueSpec.Name)
		fmt.Fprintf(&b.body, "%s	%s.%s.len = C.size_t(length)\n", indent, entryVar, valueSpec.Name)
		fmt.Fprintf(&b.body, "%s} else {\n", indent)
		fmt.Fprintf(&b.body, "%s	%s.%s.data = nil\n", indent, entryVar, valueSpec.Name)
		fmt.Fprintf(&b.body, "%s	%s.%s.len = 0\n", indent, entryVar, valueSpec.Name)
		fmt.Fprintf(&b.body, "%s}\n", indent)
	case fieldKindBytes:
		fmt.Fprintf(&b.body, "%sif data, length := arena.AllocBytes(%s); length > 0 {\n", indent, valueVar)
		fmt.Fprintf(&b.body, "%s	%s.%s.data = (*C.uint8_t)(data)\n", indent, entryVar, valueSpec.Name)
		fmt.Fprintf(&b.body, "%s	%s.%s.len = C.size_t(length)\n", indent, entryVar, valueSpec.Name)
		fmt.Fprintf(&b.body, "%s} else {\n", indent)
		fmt.Fprintf(&b.body, "%s	%s.%s.data = nil\n", indent, entryVar, valueSpec.Name)
		fmt.Fprintf(&b.body, "%s	%s.%s.len = 0\n", indent, entryVar, valueSpec.Name)
		fmt.Fprintf(&b.body, "%s}\n", indent)
	case fieldKindEnum:
		fmt.Fprintf(&b.body, "%s%s.%s = C.int32_t(int32(%s))\n", indent, entryVar, valueSpec.Name, valueVar)
	default:
		fmt.Fprintf(&b.body, "%s%s.%s = %s\n", indent, entryVar, valueSpec.Name, b.cScalarCast(valueSpec.CType, valueVar))
	}
}
func mapEntryField(entry *messageSpec, name string) *fieldSpec {
	if entry == nil {
		return nil
	}
	for i := range entry.Fields {
		if entry.Fields[i].Proto.Name == name {
			return &entry.Fields[i]
		}
	}
	return nil
}

func (b *goConversionsBuilder) mapKeyGoType(info *mapFieldInfo) string {
	switch info.KeyKind {
	case fieldKindString:
		return "string"
	case fieldKindScalar:
		return goScalarGoType(info.KeyScalar)
	default:
		return "string"
	}
}

func (b *goConversionsBuilder) mapValueGoType(info *mapFieldInfo) string {
	switch info.Value.Kind {
	case fieldKindMessage:
		return "*" + b.pbMessageType(info.ValueMessage)
	case fieldKindString:
		return "string"
	case fieldKindBytes:
		return "[]byte"
	case fieldKindEnum:
		return b.enumQualifiedType(info.ValueEnumFile, info.ValueFullType)
	case fieldKindScalar:
		return goScalarGoType(info.Value.Scalar)
	default:
		return "interface{}"
	}
}

func (b *goConversionsBuilder) mapKeyExpression(keySpec *fieldSpec, info *mapFieldInfo, entryVar string) string {
	switch info.KeyKind {
	case fieldKindString:
		b.markUnsafe()
		return fmt.Sprintf("runtime.StringFrom(unsafe.Pointer(%s.%s.data), int(%s.%s.len))", entryVar, keySpec.Name, entryVar, keySpec.Name)
	case fieldKindScalar:
		return b.goScalarFromC(goScalarGoType(info.KeyScalar), fmt.Sprintf("%s.%s", entryVar, keySpec.Name))
	default:
		b.markUnsafe()
		return fmt.Sprintf("runtime.StringFrom(unsafe.Pointer(%s.%s.data), int(%s.%s.len))", entryVar, keySpec.Name, entryVar, keySpec.Name)
	}
}

func (b *goConversionsBuilder) mapValueExpression(valueSpec *fieldSpec, info *mapFieldInfo, entryVar string) string {
	switch info.Value.Kind {
	case fieldKindMessage:
		return b.messageFromCall(info.ValueMessage, fmt.Sprintf("%s.%s", entryVar, valueSpec.Name))
	case fieldKindString:
		b.markUnsafe()
		return fmt.Sprintf("runtime.StringFrom(unsafe.Pointer(%s.%s.data), int(%s.%s.len))", entryVar, valueSpec.Name, entryVar, valueSpec.Name)
	case fieldKindBytes:
		b.markUnsafe()
		return fmt.Sprintf("runtime.BytesFrom(unsafe.Pointer(%s.%s.data), int(%s.%s.len))", entryVar, valueSpec.Name, entryVar, valueSpec.Name)
	case fieldKindEnum:
		enumType := b.enumQualifiedType(info.ValueEnumFile, info.ValueFullType)
		return fmt.Sprintf("%s(int32(%s.%s))", enumType, entryVar, valueSpec.Name)
	case fieldKindScalar:
		return b.goScalarFromC(goScalarGoType(info.Value.Scalar), fmt.Sprintf("%s.%s", entryVar, valueSpec.Name))
	default:
		b.markUnsafe()
		return fmt.Sprintf("runtime.BytesFrom(unsafe.Pointer(%s.%s.data), int(%s.%s.len))", entryVar, valueSpec.Name, entryVar, valueSpec.Name)
	}
}
func (b *goConversionsBuilder) messageIntoCall(target *messageSpec, expr string) string {
	fn := fmt.Sprintf("NewRepr%sGenerated", target.GoAlias)
	if target.GoPackage == b.spec.GoPackage {
		return fmt.Sprintf("%s(arena, %s)", fn, expr)
	}
	alias := b.useFfiAlias(target.GoPackage)
	call := fmt.Sprintf("%s.%s(arena, %s)", alias, fn, expr)
	b.markUnsafe()
	return fmt.Sprintf("(*C.%s)(unsafe.Pointer(%s))", target.CName, call)
}

func (b *goConversionsBuilder) messageFromCall(target *messageSpec, expr string) string {
	fn := fmt.Sprintf("FromRepr%sGenerated", target.GoAlias)
	if target.GoPackage == b.spec.GoPackage {
		return fmt.Sprintf("%s(%s)", fn, expr)
	}
	alias := b.useFfiAlias(target.GoPackage)
	b.markUnsafe()
	return fmt.Sprintf("%s.%s((*%s.%s)(unsafe.Pointer(%s)))", alias, fn, alias, target.GoAlias, expr)
}

func (b *goConversionsBuilder) reprTypeSymbol(target *messageSpec) string {
	if target.GoPackage == b.spec.GoPackage {
		return target.GoAlias
	}
	return fmt.Sprintf("C.%s", target.CName)
}

func (b *goConversionsBuilder) pbMessageType(target *messageSpec) string {
	alias := b.usePbAlias(target.GoImport)
	return fmt.Sprintf("%s.%s", alias, target.GoAlias)
}

func (b *goConversionsBuilder) enumQualifiedType(enumFile *protoFile, fullName string) string {
	alias := b.pbMainAlias
	if enumFile != nil {
		alias = b.usePbAlias(enumFile.GoImportPath)
	} else if b.pbMainPath != "" {
		alias = b.usePbAlias(b.pbMainPath)
	}
	var parts []string
	if enumFile != nil && enumFile.GoImportPath == "github.com/gogo/protobuf/protoc-gen-gogo/descriptor" {
		trimmed := strings.TrimPrefix(fullName, ".google.protobuf.")
		parts = splitQualified(strings.TrimPrefix(trimmed, "."))
	} else {
		_, parts = splitFullName(fullName)
	}
	typeName := joinMessageAlias(parts)
	if enumFile != nil && enumFile.GoImportPath == "github.com/gogo/protobuf/protoc-gen-gogo/descriptor" {
		if len(parts) > 0 {
			typeName = strings.Join(parts, "_")
		}
	}
	return fmt.Sprintf("%s.%s", alias, typeName)
}
func (b *goConversionsBuilder) cScalarCast(cType, value string) string {
	if cType == "bool" {
		return fmt.Sprintf("C.bool(%s)", value)
	}
	return fmt.Sprintf("C.%s(%s)", cType, value)
}

func (b *goConversionsBuilder) goScalarFromC(goType, expr string) string {
	switch goType {
	case "bool":
		return fmt.Sprintf("bool(%s)", expr)
	case "float32":
		return fmt.Sprintf("float32(%s)", expr)
	case "float64":
		return fmt.Sprintf("float64(%s)", expr)
	case "int32", "int64", "uint32", "uint64":
		return fmt.Sprintf("%s(%s)", goType, expr)
	default:
		return fmt.Sprintf("int32(%s)", expr)
	}
}

func splitCustomType(full string) (string, string) {
	full = strings.TrimSpace(full)
	if full == "" {
		return "", ""
	}
	idx := strings.LastIndex(full, ".")
	if idx <= 0 || idx+1 >= len(full) {
		return "", full
	}
	return full[:idx], full[idx+1:]
}
