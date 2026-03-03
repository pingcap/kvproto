package main

import (
	"bytes"
	"fmt"
	"path/filepath"
	"sort"
	"strings"
	"unicode"
)

func rustConversionsPath(spec *goFileSpec) string {
	pkgDir := filepath.Join("src", "ffi_runtime", spec.GoPackage)
	base := "conv_gen"
	if spec.ProtoPath != "" {
		name := filepath.Base(spec.ProtoPath)
		if ext := filepath.Ext(name); ext != "" {
			name = strings.TrimSuffix(name, ext)
		}
		if name != "" {
			base = name + "_conv_gen"
		}
	} else if spec.IsExternalOnly {
		base = "external_conv_gen"
	}
	return filepath.Join(pkgDir, base+".rs")
}

func buildRustConversions(spec *goFileSpec) []byte {
	if spec.IsExternalOnly || len(spec.Messages) == 0 {
		return nil
	}
	builder := newRustConversionsBuilder(spec)
	return builder.build()
}

type rustConversionsBuilder struct {
	spec *goFileSpec

	body bytes.Buffer

	abiTypes        map[string]struct{}
	needsStringView bool
	needsBytesView  bool
	needsCString    bool
	needsTryInto    bool
	needsArenaFuncs map[string]struct{}

	depPkgs map[string]struct{}
}

var rustImportOverrides = map[string]string{
	"eraftpb": "raft_proto::eraftpb",
}

var fieldNameOverrides = map[string]map[string]map[string]string{
	"eraftpb": {
		"Message": {
			"deprecated_priority": "priority",
		},
	},
}

var fieldToReprCastOverrides = map[string]map[string]map[string]string{
	"eraftpb": {
		"Message": {
			"priority":            ".try_into().unwrap_or_default()",
			"deprecated_priority": ".try_into().unwrap_or_default()",
		},
	},
}

var fieldFromReprCastOverrides = map[string]map[string]map[string]string{
	"eraftpb": {
		"Message": {
			"priority":            ".try_into().unwrap_or_default()",
			"deprecated_priority": ".try_into().unwrap_or_default()",
		},
	},
}

func newRustConversionsBuilder(spec *goFileSpec) *rustConversionsBuilder {
	return &rustConversionsBuilder{
		spec:            spec,
		abiTypes:        make(map[string]struct{}),
		needsArenaFuncs: make(map[string]struct{}),
		depPkgs:         make(map[string]struct{}),
	}
}

func (b *rustConversionsBuilder) build() []byte {
	for _, msg := range b.spec.Messages {
		b.recordMessageTypes(msg)
	}

	for _, msg := range b.spec.Messages {
		b.emitMessage(msg)
	}

	if b.body.Len() == 0 {
		return nil
	}

	header := b.buildHeader()

	var out bytes.Buffer
	out.Write(header)
	out.Write(b.body.Bytes())
	if !strings.HasSuffix(out.String(), "\n") {
		out.WriteByte('\n')
	}
	return out.Bytes()
}

func (b *rustConversionsBuilder) recordMessageTypes(msg *messageSpec) {
	if msg.Proto != nil && msg.Proto.IsMapEntry {
		return
	}
	b.abiTypes[msg.RustName] = struct{}{}
	for i := range msg.Fields {
		fs := &msg.Fields[i]
		if fs.Slice != nil {
			b.abiTypes[fs.Slice.RustName] = struct{}{}
		}
		if fs.Map != nil {
			b.recordMapFieldTypes(msg, fs)
			continue
		}
		switch {
		case fs.UsesString:
			b.needsStringView = true
			b.needsCString = true
		case fs.UsesBytes:
			b.needsBytesView = true
		}
		if fs.Proto.Kind == fieldKindMessage && fs.Message != nil {
			b.abiTypes[fs.Message.RustName] = struct{}{}
			if fs.Message.Package != msg.Package {
				b.depPkgs[fs.Message.Package] = struct{}{}
			}
		}
		if fs.Proto.Kind == fieldKindEnum && fs.EnumFile != nil && fs.EnumFile.Package != msg.Package {
			b.depPkgs[fs.EnumFile.Package] = struct{}{}
		}
	}
}

func (b *rustConversionsBuilder) recordMapFieldTypes(parent *messageSpec, fs *fieldSpec) {
	if fs.Map == nil {
		return
	}
	mapInfo := fs.Map
	entry := mapInfo.Entry
	if entry == nil || len(entry.Fields) < 2 {
		return
	}
	b.abiTypes[entry.RustName] = struct{}{}
	keyField := &entry.Fields[0]
	valueField := &entry.Fields[1]
	b.recordMapEntryField(parent, keyField)
	b.recordMapEntryField(parent, valueField)
	if mapInfo.Value.Kind == fieldKindMessage && mapInfo.ValueMessage != nil {
		b.abiTypes[mapInfo.ValueMessage.RustName] = struct{}{}
		if mapInfo.ValueMessage.Package != parent.Package {
			b.depPkgs[mapInfo.ValueMessage.Package] = struct{}{}
		}
	}
	if mapInfo.Value.Kind == fieldKindEnum && mapInfo.ValueEnumFile != nil && mapInfo.ValueEnumFile.Package != parent.Package {
		b.depPkgs[mapInfo.ValueEnumFile.Package] = struct{}{}
	}
}

func (b *rustConversionsBuilder) recordMapEntryField(parent *messageSpec, field *fieldSpec) {
	if field.Slice != nil {
		b.abiTypes[field.Slice.RustName] = struct{}{}
	}
	switch {
	case field.UsesString:
		b.needsStringView = true
		b.needsCString = true
	case field.UsesBytes:
		b.needsBytesView = true
	}
	if field.Proto.Kind == fieldKindMessage && field.Message != nil {
		b.abiTypes[field.Message.RustName] = struct{}{}
		if field.Message.Package != parent.Package {
			b.depPkgs[field.Message.Package] = struct{}{}
		}
	}
	if field.Proto.Kind == fieldKindEnum && field.EnumFile != nil && field.EnumFile.Package != parent.Package {
		b.depPkgs[field.EnumFile.Package] = struct{}{}
	}
}

func (b *rustConversionsBuilder) buildHeader() []byte {
	var buf bytes.Buffer
	buf.WriteString("//! Auto-generated conversions (feature `kvffi_gen`).\n")
	buf.WriteString("#![cfg(feature = \"kvffi_gen\")]\n")
	buf.WriteString("#![allow(unused_imports, unused_variables, unused_mut, non_snake_case)]\n\n")

	if b.needsTryInto {
		buf.WriteString("use std::convert::TryInto;\n")
	}
	buf.WriteString("use std::ptr;\n")
	if b.needsCString {
		buf.WriteString("use std::os::raw::c_char;\n")
	}
	buf.WriteByte('\n')

	buf.WriteString("use protobuf::Message;\n")
	buf.WriteString("use protobuf::ProtobufEnum;\n")

	arenaImports := []string{"Arena"}
	funcNames := make([]string, 0, len(b.needsArenaFuncs))
	for fn := range b.needsArenaFuncs {
		funcNames = append(funcNames, fn)
	}
	sort.Strings(funcNames)
	arenaImports = append(arenaImports, funcNames...)
	buf.WriteString("use crate::ffi_runtime::arena::{")
	for i, name := range arenaImports {
		if i > 0 {
			buf.WriteString(", ")
		}
		buf.WriteString(name)
	}
	buf.WriteString("};\n")

	abiNames := make([]string, 0, len(b.abiTypes))
	for name := range b.abiTypes {
		abiNames = append(abiNames, name)
	}
	sort.Strings(abiNames)
	if b.needsStringView {
		abiNames = append(abiNames, "KvprotoStringView")
	}
	if b.needsBytesView {
		abiNames = append(abiNames, "KvprotoBytesView")
	}
	sort.Strings(abiNames)
	if len(abiNames) > 0 {
		buf.WriteString("use crate::ffi_runtime::abi::{")
		for i, name := range abiNames {
			if i > 0 {
				buf.WriteString(", ")
			}
			buf.WriteString(name)
		}
		buf.WriteString("};\n")
	}

	if override, ok := rustImportOverrides[b.spec.GoPackage]; ok {
		buf.WriteString(fmt.Sprintf("use %s as pb;\n", override))
	} else {
		buf.WriteString(fmt.Sprintf("use crate::%s as pb;\n", b.spec.GoPackage))
	}
	depNames := make([]string, 0, len(b.depPkgs))
	for pkg := range b.depPkgs {
		depNames = append(depNames, sanitizeGoPackageName(pkg))
	}
	sort.Strings(depNames)
	for _, name := range depNames {
		if override, ok := rustImportOverrides[name]; ok {
			buf.WriteString(fmt.Sprintf("use %s;\n", override))
		} else {
			buf.WriteString(fmt.Sprintf("use crate::%s;\n", name))
		}
	}
	buf.WriteByte('\n')
	return buf.Bytes()
}

func (b *rustConversionsBuilder) emitMessage(msg *messageSpec) {
	if msg.Proto != nil && msg.Proto.IsMapEntry {
		return
	}
	snakeName := toSnakeCase(msg.GoAlias)
	reprType := msg.RustName
	pbType := b.messagePbPath(msg)

	fmt.Fprintf(&b.body, "pub fn %s_to_repr_generated<'a>(arena: &'a mut Arena, src: &%s) -> &'a mut %s {\n", snakeName, pbType, reprType)
	fmt.Fprintf(&b.body, "    let mut repr = %s {\n", reprType)
	for i := range msg.Fields {
		fs := &msg.Fields[i]
		if fs.Proto2Optional {
			presence := rustFieldName(presenceFieldName(fs.Name))
			fmt.Fprintf(&b.body, "        %s: false,\n", presence)
		}
		fieldName := rustFieldName(fs.Name)
		fmt.Fprintf(&b.body, "        %s: %s,\n", fieldName, b.defaultValue(fs))
	}
	b.body.WriteString("    };\n")

	for i := range msg.Fields {
		fs := &msg.Fields[i]
		pf := fs.Proto
		if pf.Number == 0 || pf.Oneof != "" {
			continue
		}
		b.emitToField(msg, fs, pf)
	}

	b.body.WriteString("    arena.alloc_struct(repr)\n}")
	b.body.WriteString("\n\n")

	fmt.Fprintf(&b.body, "pub fn %s_from_repr_generated(src: *const %s) -> Option<%s> {\n", snakeName, reprType, pbType)
	b.body.WriteString("    if src.is_null() {\n        return None;\n    }\n")
	b.body.WriteString("    let repr = unsafe { &*src };\n")
	fmt.Fprintf(&b.body, "    let mut out = %s::new();\n", pbType)

	for i := range msg.Fields {
		fs := &msg.Fields[i]
		pf := fs.Proto
		if pf.Number == 0 || pf.Oneof != "" {
			continue
		}
		b.emitFromField(msg, fs, pf)
	}

	b.body.WriteString("    Some(out)\n}\n")
	b.body.WriteString("\n")
}

func (b *rustConversionsBuilder) defaultValue(fs *fieldSpec) string {
	if fs.Slice != nil {
		return fmt.Sprintf("%s { data: ptr::null_mut(), len: 0, cap: 0 }", fs.Slice.RustName)
	}
	if fs.UsesString {
		return "KvprotoStringView { data: ptr::null(), len: 0 }"
	}
	if fs.UsesBytes {
		return "KvprotoBytesView { data: ptr::null_mut(), len: 0 }"
	}
	switch fs.Proto.Kind {
	case fieldKindMessage:
		return "ptr::null_mut()"
	case fieldKindScalar, fieldKindEnum:
		return "Default::default()"
	default:
		return "Default::default()"
	}
}

func (b *rustConversionsBuilder) emitToField(msg *messageSpec, fs *fieldSpec, pf protoField) {
	setter := fmt.Sprintf("repr.%s", rustFieldName(fs.Name))
	getter := fmt.Sprintf("src.%s()", b.rustGetterName(msg, fs))
	isMapField := fs.Map != nil || (fs.Slice != nil && fs.Message != nil && fs.Message.Proto != nil && fs.Message.Proto.IsMapEntry)

	switch {
	case isMapField:
		b.emitToMapField(msg, fs, setter, getter)
	case fs.UsesString:
		b.emitToStringField(fs, pf, setter, getter)
	case fs.UsesBytes:
		b.emitToBytesField(fs, pf, setter, getter)
	case fs.Slice != nil && pf.Kind == fieldKindMessage:
		b.emitToRepeatedMessage(msg, fs, setter, getter)
	case fs.Slice != nil && pf.Kind == fieldKindEnum:
		b.emitToRepeatedEnum(fs, pf, setter, getter)
	case fs.Slice != nil:
		b.emitToRepeatedScalar(fs, setter, getter)
	case pf.Kind == fieldKindMessage:
		b.emitToMessageField(msg, fs, setter)
	case pf.Kind == fieldKindEnum:
		fmt.Fprintf(&b.body, "    %s = %s as i32;\n", setter, getter)
	default:
		cast := b.fieldToReprCast(msg, fs)
		fmt.Fprintf(&b.body, "    %s = %s%s;\n", setter, getter, cast)
	}
}

func (b *rustConversionsBuilder) emitFromField(msg *messageSpec, fs *fieldSpec, pf protoField) {
	getter := fmt.Sprintf("repr.%s", rustFieldName(fs.Name))
	setter := fmt.Sprintf("out.set_%s", b.fieldAccessorName(msg, fs))
	isMapField := fs.Map != nil || (fs.Slice != nil && fs.Message != nil && fs.Message.Proto != nil && fs.Message.Proto.IsMapEntry)

	switch {
	case isMapField:
		b.emitFromMapField(msg, fs, getter)
	case fs.UsesString:
		b.emitFromStringField(msg, fs, getter, setter)
	case fs.UsesBytes:
		b.emitFromBytesField(msg, fs, getter, setter)
	case fs.Slice != nil && pf.Kind == fieldKindMessage:
		b.emitFromRepeatedMessage(msg, fs, getter, setter)
	case fs.Slice != nil && pf.Kind == fieldKindEnum:
		b.emitFromRepeatedEnum(fs, pf, getter, setter)
	case fs.Slice != nil:
		b.emitFromRepeatedScalar(fs, getter, setter)
	case pf.Kind == fieldKindMessage:
		b.emitFromMessageField(msg, fs, getter)
	case pf.Kind == fieldKindEnum:
		fmt.Fprintf(&b.body, "    %s(%s::from_i32(%s).unwrap_or_default());\n", setter, b.enumRustType(fs, pf), getter)
	default:
		cast := b.fieldFromReprCast(msg, fs)
		fmt.Fprintf(&b.body, "    %s(%s%s);\n", setter, getter, cast)
	}
}

func (b *rustConversionsBuilder) emitToStringField(fs *fieldSpec, _pf protoField, setter, getter string) {
	if fs.Slice != nil {
		b.body.WriteString("    {")
		b.body.WriteString("\n        let values = ")
		b.body.WriteString(getter)
		b.body.WriteString(";\n        if !values.is_empty() {\n            let mut views = Vec::with_capacity(values.len());\n            for value in values {\n                if value.is_empty() { continue; }\n                let (ptr, len) = arena.alloc_string(value);\n                views.push(KvprotoStringView { data: ptr as *const c_char, len });\n            }\n            if !views.is_empty() {\n                let (ptr, len) = arena.alloc_vec(views);\n                ")
		b.body.WriteString(setter)
		b.body.WriteString(".data = ptr;\n                ")
		b.body.WriteString(setter)
		b.body.WriteString(".len = len;\n                ")
		b.body.WriteString(setter)
		b.body.WriteString(".cap = len;\n            }\n        }\n    }\n")
		return
	}
	b.body.WriteString("    if !")
	b.body.WriteString(getter)
	b.body.WriteString(".is_empty() {\n        let (ptr, len) = arena.alloc_string(")
	b.body.WriteString(getter)
	b.body.WriteString(");\n        ")
	b.body.WriteString(setter)
	b.body.WriteString(".data = ptr as *const c_char;\n        ")
	b.body.WriteString(setter)
	b.body.WriteString(".len = len;\n    }\n")
}

func (b *rustConversionsBuilder) emitToBytesField(fs *fieldSpec, _pf protoField, setter, getter string) {
	if fs.Slice != nil {
		b.body.WriteString("    {\n        let values = ")
		b.body.WriteString(getter)
		b.body.WriteString(";\n        if !values.is_empty() {\n            let mut views = Vec::with_capacity(values.len());\n            for value in values {\n                if value.is_empty() { continue; }\n                let (ptr, len) = arena.alloc_bytes(value);\n                views.push(KvprotoBytesView { data: ptr, len });\n            }\n            if !views.is_empty() {\n                let (ptr, len) = arena.alloc_vec(views);\n                ")
		b.body.WriteString(setter)
		b.body.WriteString(".data = ptr;\n                ")
		b.body.WriteString(setter)
		b.body.WriteString(".len = len;\n                ")
		b.body.WriteString(setter)
		b.body.WriteString(".cap = len;\n            }\n        }\n    }\n")
		return
	}
	b.body.WriteString("    if !")
	b.body.WriteString(getter)
	b.body.WriteString(".is_empty() {\n        let (ptr, len) = arena.alloc_bytes(")
	b.body.WriteString(getter)
	b.body.WriteString(");\n        ")
	b.body.WriteString(setter)
	b.body.WriteString(".data = ptr;\n        ")
	b.body.WriteString(setter)
	b.body.WriteString(".len = len;\n    }\n")
}

func (b *rustConversionsBuilder) emitToRepeatedScalar(fs *fieldSpec, setter, getter string) {
	elem := fs.Slice.ElementRustType
	b.body.WriteString("    {\n")
	b.body.WriteString("        let values = ")
	b.body.WriteString(getter)
	b.body.WriteString(";\n")
	b.body.WriteString("        if !values.is_empty() {\n")
	fmt.Fprintf(&b.body, "            let mut vec: Vec<%s> = Vec::with_capacity(values.len());\n", elem)
	b.body.WriteString("            for value in values.iter() {\n                vec.push(*value);\n            }\n")
	b.body.WriteString("            let (ptr, len) = arena.alloc_vec(vec);\n")
	fmt.Fprintf(&b.body, "            %s.data = ptr;\n", setter)
	fmt.Fprintf(&b.body, "            %s.len = len;\n", setter)
	fmt.Fprintf(&b.body, "            %s.cap = len;\n", setter)
	b.body.WriteString("        }\n")
	b.body.WriteString("    }\n")
}

func (b *rustConversionsBuilder) emitFromRepeatedScalar(fs *fieldSpec, getter, setter string) {
	elem := fs.Slice.ElementRustType
	b.body.WriteString("    if !")
	b.body.WriteString(getter)
	b.body.WriteString(".data.is_null() && ")
	b.body.WriteString(getter)
	b.body.WriteString(".len > 0 {\n")
	fmt.Fprintf(&b.body, "        let slice = unsafe { std::slice::from_raw_parts(%s.data, %s.len) };\n", getter, getter)
	fmt.Fprintf(&b.body, "        let mut values: Vec<%s> = Vec::with_capacity(slice.len());\n", elem)
	b.body.WriteString("        values.extend_from_slice(slice);\n")
	fmt.Fprintf(&b.body, "        %s(values);\n", setter)
	b.body.WriteString("    }\n")
}

func (b *rustConversionsBuilder) emitToRepeatedEnum(fs *fieldSpec, _pf protoField, setter, getter string) {
	elem := fs.Slice.ElementRustType
	b.body.WriteString("    {\n")
	b.body.WriteString("        let values = ")
	b.body.WriteString(getter)
	b.body.WriteString(";\n")
	b.body.WriteString("        if !values.is_empty() {\n")
	fmt.Fprintf(&b.body, "            let mut vec: Vec<%s> = Vec::with_capacity(values.len());\n", elem)
	b.body.WriteString("            for value in values.iter() {\n                vec.push(value.value());\n            }\n")
	b.body.WriteString("            let (ptr, len) = arena.alloc_vec(vec);\n")
	fmt.Fprintf(&b.body, "            %s.data = ptr;\n", setter)
	fmt.Fprintf(&b.body, "            %s.len = len;\n", setter)
	fmt.Fprintf(&b.body, "            %s.cap = len;\n", setter)
	b.body.WriteString("        }\n")
	b.body.WriteString("    }\n")
}

func (b *rustConversionsBuilder) emitFromRepeatedEnum(fs *fieldSpec, pf protoField, getter, setter string) {
	enumType := b.enumRustType(fs, pf)
	b.body.WriteString("    if !")
	b.body.WriteString(getter)
	b.body.WriteString(".data.is_null() && ")
	b.body.WriteString(getter)
	b.body.WriteString(".len > 0 {\n")
	fmt.Fprintf(&b.body, "        let slice = unsafe { std::slice::from_raw_parts(%s.data, %s.len) };\n", getter, getter)
	fmt.Fprintf(&b.body, "        let mut values: Vec<%s> = Vec::with_capacity(slice.len());\n", enumType)
	b.body.WriteString("        for &value in slice {\n")
	fmt.Fprintf(&b.body, "            values.push(%s::from_i32(value).unwrap_or_default());\n", enumType)
	b.body.WriteString("        }\n")
	fmt.Fprintf(&b.body, "        %s(values);\n", setter)
	b.body.WriteString("    }\n")
}

func (b *rustConversionsBuilder) emitToRepeatedMessage(msg *messageSpec, fs *fieldSpec, setter, getter string) {
	elem := fs.Slice.ElementRustType
	b.body.WriteString("    {\n")
	b.body.WriteString("        let values = ")
	b.body.WriteString(getter)
	b.body.WriteString(";\n")
	b.body.WriteString("        if !values.is_empty() {\n")
	fmt.Fprintf(&b.body, "            let mut ptrs: Vec<%s> = Vec::with_capacity(values.len());\n", elem)
	call := b.messageToReprCallWithExpr(msg, fs, "value")
	b.body.WriteString("            for value in values.iter() {\n                ptrs.push(" + call + ");\n            }\n")
	b.body.WriteString("            if !ptrs.is_empty() {\n")
	b.body.WriteString("                let (ptr, len) = arena.alloc_vec(ptrs);\n")
	fmt.Fprintf(&b.body, "                %s.data = ptr;\n", setter)
	fmt.Fprintf(&b.body, "                %s.len = len;\n", setter)
	fmt.Fprintf(&b.body, "                %s.cap = len;\n", setter)
	b.body.WriteString("            }\n")
	b.body.WriteString("        }\n")
	b.body.WriteString("    }\n")
}

func (b *rustConversionsBuilder) emitFromRepeatedMessage(msg *messageSpec, fs *fieldSpec, getter, setter string) {
	b.body.WriteString("    if !")
	b.body.WriteString(getter)
	b.body.WriteString(".data.is_null() && ")
	b.body.WriteString(getter)
	b.body.WriteString(".len > 0 {\n")
	fmt.Fprintf(&b.body, "        let slice = unsafe { std::slice::from_raw_parts(%s.data, %s.len) };\n", getter, getter)
	pbType := b.messagePbPath(fs.Message)
	fmt.Fprintf(&b.body, "        let mut values: Vec<%s> = Vec::with_capacity(slice.len());\n", pbType)
	b.body.WriteString("        for &ptr in slice {\n            if ptr.is_null() {\n                continue;\n            }\n")
	call := b.messageFromReprCallWithExpr(msg, fs, "ptr")
	b.body.WriteString("            if let Some(value) = " + call + " {\n                values.push(value);\n            }\n        }\n        if !values.is_empty() {\n")
	fmt.Fprintf(&b.body, "            %s(::protobuf::RepeatedField::from_vec(values));\n", setter)
	b.body.WriteString("        }\n    }\n")
}

func (b *rustConversionsBuilder) emitToMessageField(msg *messageSpec, fs *fieldSpec, setter string) {
	call := b.messageToReprCall(msg, fs)
	accessor := b.fieldAccessorName(msg, fs)
	fmt.Fprintf(&b.body, "    if src.has_%s() {\n        %s = %s;\n    } else {\n        %s = ptr::null_mut();\n    }\n", accessor, setter, call, setter)
}

func (b *rustConversionsBuilder) emitFromMessageField(msg *messageSpec, fs *fieldSpec, getter string) {
	call := b.messageFromReprCall(msg, fs, getter)
	setter := fmt.Sprintf("out.set_%s", b.fieldAccessorName(msg, fs))
	fmt.Fprintf(&b.body, "    if !%s.is_null() {\n        if let Some(value) = %s {\n            %s(value);\n        }\n    }\n", getter, call, setter)
}

func (b *rustConversionsBuilder) emitFromStringField(_msg *messageSpec, fs *fieldSpec, getter, setter string) {
	b.needsArenaFuncs["string_from"] = struct{}{}
	if fs.Slice != nil {
		b.body.WriteString("    if !")
		b.body.WriteString(getter)
		b.body.WriteString(".data.is_null() && ")
		b.body.WriteString(getter)
		b.body.WriteString(".len > 0 {\n")
		b.body.WriteString("        let slice = unsafe { std::slice::from_raw_parts(")
		b.body.WriteString(getter)
		b.body.WriteString(".data, ")
		b.body.WriteString(getter)
		b.body.WriteString(".len) };\n")
		b.body.WriteString("        let mut values = Vec::with_capacity(slice.len());\n")
		b.body.WriteString("        for view in slice {\n            values.push(string_from(view.data as *const u8, view.len));\n        }\n")
		fmt.Fprintf(&b.body, "        %s(::protobuf::RepeatedField::from_vec(values));\n", setter)
		b.body.WriteString("    }\n")
		return
	}
	fmt.Fprintf(&b.body, "    %s(string_from(%s.data as *const u8, %s.len));\n", setter, getter, getter)
}

func (b *rustConversionsBuilder) emitFromBytesField(_msg *messageSpec, fs *fieldSpec, getter, setter string) {
	b.needsArenaFuncs["bytes_from"] = struct{}{}
	if fs.Slice != nil {
		b.body.WriteString("    if !")
		b.body.WriteString(getter)
		b.body.WriteString(".data.is_null() && ")
		b.body.WriteString(getter)
		b.body.WriteString(".len > 0 {\n")
		b.body.WriteString("        let slice = unsafe { std::slice::from_raw_parts(")
		b.body.WriteString(getter)
		b.body.WriteString(".data, ")
		b.body.WriteString(getter)
		b.body.WriteString(".len) };\n")
		b.body.WriteString("        let mut values = Vec::with_capacity(slice.len());\n")
		b.body.WriteString("        for view in slice {\n            values.push(bytes_from(view.data, view.len));\n        }\n")
		fmt.Fprintf(&b.body, "        %s(::protobuf::RepeatedField::from_vec(values));\n", setter)
		b.body.WriteString("    }\n")
		return
	}
	expr := fmt.Sprintf("bytes_from(%s.data, %s.len).into()", getter, getter)
	fmt.Fprintf(&b.body, "    %s(%s);\n", setter, expr)
}

func (b *rustConversionsBuilder) messageToReprCall(msg *messageSpec, fs *fieldSpec) string {
	expr := fmt.Sprintf("src.%s()", b.rustGetterName(msg, fs))
	return b.messageToReprCallWithExpr(msg, fs, expr)
}

func (b *rustConversionsBuilder) messageFromReprCall(msg *messageSpec, fs *fieldSpec, reprExpr string) string {
	return b.messageFromReprCallWithExpr(msg, fs, reprExpr)
}

func (b *rustConversionsBuilder) messageToReprCallWithExpr(msg *messageSpec, fs *fieldSpec, expr string) string {
	target := fs.Message
	fnName := toSnakeCase(target.GoAlias) + "_to_repr_generated"
	if target.Package == msg.Package {
		return fmt.Sprintf("%s(arena, %s) as *mut _", fnName, expr)
	}
	module := sanitizeGoPackageName(target.Package)
	return fmt.Sprintf("crate::ffi_runtime::%s::%s(arena, %s) as *mut _", module, fnName, expr)
}

func (b *rustConversionsBuilder) messageFromReprCallWithExpr(msg *messageSpec, fs *fieldSpec, expr string) string {
	target := fs.Message
	fnName := toSnakeCase(target.GoAlias) + "_from_repr_generated"
	if target.Package == msg.Package {
		return fmt.Sprintf("%s(%s)", fnName, expr)
	}
	module := sanitizeGoPackageName(target.Package)
	return fmt.Sprintf("crate::ffi_runtime::%s::%s(%s)", module, fnName, expr)
}

func (b *rustConversionsBuilder) fieldAccessorName(msg *messageSpec, fs *fieldSpec) string {
	name := fs.Proto.Name
	if pkgOverrides, ok := fieldNameOverrides[msg.Package]; ok {
		if msgOverrides, ok := pkgOverrides[msg.GoAlias]; ok {
			if override, ok := msgOverrides[name]; ok {
				name = override
			}
		}
	}
	return camelToSnake(name)
}

func (b *rustConversionsBuilder) rustGetterName(msg *messageSpec, fs *fieldSpec) string {
	return fmt.Sprintf("get_%s", b.fieldAccessorName(msg, fs))
}

func (b *rustConversionsBuilder) fieldToReprCast(msg *messageSpec, fs *fieldSpec) string {
	if pkgOverrides, ok := fieldToReprCastOverrides[msg.Package]; ok {
		if msgOverrides, ok := pkgOverrides[msg.GoAlias]; ok {
			if cast, ok := msgOverrides[fs.Proto.Name]; ok {
				if strings.Contains(cast, "try_into") {
					b.needsTryInto = true
				}
				return cast
			}
		}
	}
	return ""
}

func (b *rustConversionsBuilder) fieldFromReprCast(msg *messageSpec, fs *fieldSpec) string {
	if pkgOverrides, ok := fieldFromReprCastOverrides[msg.Package]; ok {
		if msgOverrides, ok := pkgOverrides[msg.GoAlias]; ok {
			if cast, ok := msgOverrides[fs.Proto.Name]; ok {
				if strings.Contains(cast, "try_into") {
					b.needsTryInto = true
				}
				return cast
			}
		}
	}
	return ""
}

func (b *rustConversionsBuilder) enumRustType(fs *fieldSpec, pf protoField) string {
	pkg, parts := splitProtoType(pf.FullType)
	if pkg == "" {
		return "pb::" + rustProtoIdent(goExportedName(pf.Name))
	}
	module := sanitizeGoPackageName(pkg)
	if module == b.spec.GoPackage {
		module = "pb"
	}
	if len(parts) == 0 {
		return module + "::" + rustProtoIdent(goExportedName(pf.Name))
	}
	var typeName strings.Builder
	for _, part := range parts {
		typeName.WriteString(rustProtoIdent(part))
	}
	return module + "::" + typeName.String()
}

func toSnakeCase(name string) string {
	if name == "" {
		return name
	}
	var buf strings.Builder
	for i, r := range name {
		if r >= 'A' && r <= 'Z' {
			if i > 0 {
				buf.WriteByte('_')
			}
			buf.WriteRune(r + ('a' - 'A'))
		} else {
			buf.WriteRune(r)
		}
	}
	return buf.String()
}

func camelToSnake(name string) string {
	if name == "" {
		return name
	}
	runes := []rune(name)
	var out []rune
	for i, r := range runes {
		switch {
		case r == '_' || r == '-' || r == ' ':
			if len(out) == 0 || out[len(out)-1] == '_' {
				continue
			}
			out = append(out, '_')
		case unicode.IsUpper(r):
			if i > 0 {
				prev := runes[i-1]
				var next rune
				if i+1 < len(runes) {
					next = runes[i+1]
				}
				if unicode.IsLower(prev) || unicode.IsDigit(prev) || (next != 0 && unicode.IsLower(next)) {
					if len(out) > 0 && out[len(out)-1] != '_' {
						out = append(out, '_')
					}
				}
			}
			out = append(out, unicode.ToLower(r))
		default:
			out = append(out, r)
		}
	}
	// collapse duplicate underscores
	res := make([]rune, 0, len(out))
	for _, r := range out {
		if r == '_' && len(res) > 0 && res[len(res)-1] == '_' {
			continue
		}
		res = append(res, r)
	}
	if len(res) > 0 && res[len(res)-1] == '_' {
		res = res[:len(res)-1]
	}
	return string(res)
}

func rustProtoIdent(name string) string {
	if name == "" {
		return name
	}
	snake := camelToSnake(name)
	if snake == "" {
		snake = strings.ToLower(name)
	}
	return toPascalCase(snake)
}

func splitProtoType(full string) (string, []string) {
	full = strings.TrimPrefix(full, ".")
	if full == "" {
		return "", nil
	}
	parts := strings.Split(full, ".")
	if len(parts) == 0 {
		return "", nil
	}
	return parts[0], parts[1:]
}

func (b *rustConversionsBuilder) messagePbPath(msg *messageSpec) string {
	if msg.Proto == nil {
		return "pb::" + rustProtoIdent(msg.GoAlias)
	}
	return b.pbPath(msg.Package, msg.Proto.PathParts)
}

func (b *rustConversionsBuilder) pbPath(pkg string, parts []string) string {
	module := sanitizeGoPackageName(pkg)
	if module == b.spec.GoPackage {
		module = "pb"
	}
	if len(parts) == 0 {
		return module
	}
	var typeName strings.Builder
	for _, part := range parts {
		typeName.WriteString(rustProtoIdent(part))
	}
	return module + "::" + typeName.String()
}

func (b *rustConversionsBuilder) emitToMapField(msg *messageSpec, fs *fieldSpec, setter, getter string) {
	mapInfo := fs.Map
	if mapInfo == nil {
		fmt.Fprintf(&b.body, "    // TODO: map field %s not yet supported\n", fs.Name)
		return
	}
	entrySpec := mapInfo.Entry
	if entrySpec == nil {
		entrySpec = fs.Message
	}
	if entrySpec == nil || entrySpec.Proto == nil || !entrySpec.Proto.IsMapEntry || len(entrySpec.Fields) < 2 {
		fmt.Fprintf(&b.body, "    // TODO: map field %s not yet supported\n", fs.Name)
		return
	}

	keyField := &entrySpec.Fields[0]
	valueField := &entrySpec.Fields[1]
	entryType := entrySpec.RustName

	keyName := rustFieldName(keyField.Name)
	valueName := rustFieldName(valueField.Name)

	valueSpec := *valueField
	valueSpec.Proto = mapInfo.Value
	valueSpec.FullType = mapInfo.Value.FullType
	if mapInfo.ValueMessage != nil {
		valueSpec.Message = mapInfo.ValueMessage
	}
	if mapInfo.ValueEnumFile != nil {
		valueSpec.EnumFile = mapInfo.ValueEnumFile
	}

	b.body.WriteString("    {\n")
	fmt.Fprintf(&b.body, "        let map = %s;\n", getter)
	b.body.WriteString("        if !map.is_empty() {\n")
	fmt.Fprintf(&b.body, "            let mut entries: Vec<*mut %s> = Vec::with_capacity(map.len());\n", entryType)
	b.body.WriteString("            for (key, value) in map.iter() {\n")

	var keyExpr string
	switch mapInfo.KeyKind {
	case fieldKindString:
		b.body.WriteString("                let key_view = if key.is_empty() {\n")
		b.body.WriteString("                    KvprotoStringView { data: ptr::null(), len: 0 }\n")
		b.body.WriteString("                } else {\n")
		b.body.WriteString("                    let (ptr, len) = arena.alloc_string(key);\n")
		b.body.WriteString("                    KvprotoStringView { data: ptr as *const c_char, len }\n")
		b.body.WriteString("                };\n")
		keyExpr = "key_view"
	case fieldKindScalar:
		keyExpr = "*key"
		if cast := b.fieldToReprCast(entrySpec, keyField); cast != "" {
			keyExpr = fmt.Sprintf("(*key%s)", cast)
		}
	default:
		fmt.Fprintf(&b.body, "                // TODO: unsupported map key kind for %s\n", fs.Name)
		b.body.WriteString("                continue;\n")
		keyExpr = "*key"
	}

	var valueExpr string
	switch mapInfo.Value.Kind {
	case fieldKindMessage:
		if valueSpec.Message == nil {
			fmt.Fprintf(&b.body, "                // TODO: unsupported map value kind for %s\n", fs.Name)
			b.body.WriteString("                continue;\n")
			valueExpr = "*value"
			break
		}
		valueCall := b.messageToReprCallWithExpr(entrySpec, &valueSpec, "value")
		fmt.Fprintf(&b.body, "                let value_ptr = %s;\n", valueCall)
		valueExpr = "value_ptr"
	case fieldKindString:
		b.body.WriteString("                let value_view = if value.is_empty() {\n")
		b.body.WriteString("                    KvprotoStringView { data: ptr::null(), len: 0 }\n")
		b.body.WriteString("                } else {\n")
		b.body.WriteString("                    let (ptr, len) = arena.alloc_string(value);\n")
		b.body.WriteString("                    KvprotoStringView { data: ptr as *const c_char, len }\n")
		b.body.WriteString("                };\n")
		valueExpr = "value_view"
	case fieldKindBytes:
		b.body.WriteString("                let value_view = if value.is_empty() {\n")
		b.body.WriteString("                    KvprotoBytesView { data: ptr::null_mut(), len: 0 }\n")
		b.body.WriteString("                } else {\n")
		b.body.WriteString("                    let (ptr, len) = arena.alloc_bytes(value);\n")
		b.body.WriteString("                    KvprotoBytesView { data: ptr, len }\n")
		b.body.WriteString("                };\n")
		valueExpr = "value_view"
	case fieldKindEnum:
		valueExpr = "value.value()"
	default:
		valueExpr = "*value"
		if cast := b.fieldToReprCast(entrySpec, &valueSpec); cast != "" {
			valueExpr = fmt.Sprintf("(*value%s)", cast)
		}
	}

	fmt.Fprintf(&b.body, "                let entry_ptr = arena.alloc_struct(%s {\n", entryType)
	fmt.Fprintf(&b.body, "                    %s: %s,\n", keyName, keyExpr)
	fmt.Fprintf(&b.body, "                    %s: %s,\n", valueName, valueExpr)
	fmt.Fprintf(&b.body, "                }) as *mut %s;\n", entryType)
	b.body.WriteString("                entries.push(entry_ptr);\n")
	b.body.WriteString("            }\n")
	b.body.WriteString("            if !entries.is_empty() {\n")
	b.body.WriteString("                let (ptr, len) = arena.alloc_vec(entries);\n")
	fmt.Fprintf(&b.body, "                %s.data = ptr;\n", setter)
	fmt.Fprintf(&b.body, "                %s.len = len;\n", setter)
	fmt.Fprintf(&b.body, "                %s.cap = len;\n", setter)
	b.body.WriteString("            }\n")
	b.body.WriteString("        }\n")
	b.body.WriteString("    }\n")
}

func (b *rustConversionsBuilder) emitFromMapField(msg *messageSpec, fs *fieldSpec, getter string) {
	mapInfo := fs.Map
	if mapInfo == nil {
		fmt.Fprintf(&b.body, "    // TODO: map field %s not yet supported\n", fs.Name)
		return
	}
	entrySpec := mapInfo.Entry
	if entrySpec == nil {
		entrySpec = fs.Message
	}
	if entrySpec == nil || entrySpec.Proto == nil || !entrySpec.Proto.IsMapEntry || len(entrySpec.Fields) < 2 {
		fmt.Fprintf(&b.body, "    // TODO: map field %s not yet supported\n", fs.Name)
		return
	}

	keyField := &entrySpec.Fields[0]
	valueField := &entrySpec.Fields[1]

	valueSpec := *valueField
	valueSpec.Proto = mapInfo.Value
	valueSpec.FullType = mapInfo.Value.FullType
	if mapInfo.ValueMessage != nil {
		valueSpec.Message = mapInfo.ValueMessage
	}
	if mapInfo.ValueEnumFile != nil {
		valueSpec.EnumFile = mapInfo.ValueEnumFile
	}

	accessor := b.fieldAccessorName(msg, fs)

	b.body.WriteString("    if !")
	b.body.WriteString(getter)
	b.body.WriteString(".data.is_null() && ")
	b.body.WriteString(getter)
	b.body.WriteString(".len > 0 {\n")
	fmt.Fprintf(&b.body, "        let slice = unsafe { std::slice::from_raw_parts(%s.data, %s.len) };\n", getter, getter)
	fmt.Fprintf(&b.body, "        let map = out.mut_%s();\n", accessor)
	b.body.WriteString("        map.clear();\n")
	b.body.WriteString("        for &entry_ptr in slice {\n")
	b.body.WriteString("            if entry_ptr.is_null() {\n")
	b.body.WriteString("                continue;\n")
	b.body.WriteString("            }\n")
	b.body.WriteString("            let entry = unsafe { &*entry_ptr };\n")
	keyName := rustFieldName(keyField.Name)
	valueName := rustFieldName(valueField.Name)

	switch mapInfo.KeyKind {
	case fieldKindString:
		b.needsArenaFuncs["string_from"] = struct{}{}
		fmt.Fprintf(&b.body, "            let key = string_from(entry.%s.data as *const u8, entry.%s.len);\n", keyName, keyName)
	case fieldKindScalar:
		keyExpr := fmt.Sprintf("entry.%s", keyName)
		if cast := b.fieldFromReprCast(entrySpec, keyField); cast != "" {
			keyExpr = fmt.Sprintf("(%s%s)", keyExpr, cast)
		}
		fmt.Fprintf(&b.body, "            let key = %s;\n", keyExpr)
	default:
		fmt.Fprintf(&b.body, "            // TODO: unsupported map key kind for %s\n", fs.Name)
		b.body.WriteString("            continue;\n")
	}

	switch mapInfo.Value.Kind {
	case fieldKindMessage:
		if valueSpec.Message == nil {
			fmt.Fprintf(&b.body, "            // TODO: unsupported map value kind for %s\n", fs.Name)
			b.body.WriteString("            continue;\n")
			break
		}
		valueCall := b.messageFromReprCallWithExpr(entrySpec, &valueSpec, fmt.Sprintf("entry.%s", valueName))
		fmt.Fprintf(&b.body, "            let value = %s.unwrap_or_default();\n", valueCall)
	case fieldKindString:
		b.needsArenaFuncs["string_from"] = struct{}{}
		fmt.Fprintf(&b.body, "            let value = string_from(entry.%s.data as *const u8, entry.%s.len);\n", valueName, valueName)
	case fieldKindBytes:
		b.needsArenaFuncs["bytes_from"] = struct{}{}
		fmt.Fprintf(&b.body, "            let value = bytes_from(entry.%s.data, entry.%s.len);\n", valueName, valueName)
	case fieldKindEnum:
		enumType := b.enumRustType(&valueSpec, mapInfo.Value)
		fmt.Fprintf(&b.body, "            let value = %s::from_i32(entry.%s).unwrap_or_default();\n", enumType, valueName)
	default:
		valueExpr := fmt.Sprintf("entry.%s", valueName)
		if cast := b.fieldFromReprCast(entrySpec, &valueSpec); cast != "" {
			valueExpr = fmt.Sprintf("(%s%s)", valueExpr, cast)
		}
		fmt.Fprintf(&b.body, "            let value = %s;\n", valueExpr)
	}

	b.body.WriteString("            map.insert(key, value);\n")
	b.body.WriteString("        }\n")
	b.body.WriteString("    }\n")
}
