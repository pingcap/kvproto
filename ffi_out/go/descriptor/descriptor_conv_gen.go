//go:build kvffi_gen
// +build kvffi_gen

package descriptor

/*
#cgo CFLAGS: -I../../c
#include "kvproto_abi.h"
*/
import "C"

import (
	"unsafe"
	descriptorproto "github.com/gogo/protobuf/protoc-gen-gogo/descriptor"
	runtime "github.com/pingcap/kvproto/ffi_out/go/runtime"
)

func NewReprDescriptorProtoGenerated(arena *runtime.Arena, src *descriptorproto.DescriptorProto) *DescriptorProto {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DescriptorProto)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_DescriptorProto)))
	IntoReprDescriptorProtoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDescriptorProtoGenerated(arena *runtime.Arena, dst *DescriptorProto, src *descriptorproto.DescriptorProto) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Name != nil {
		dst.has_name = C.bool(true)
		if data, length := arena.AllocString(*src.Name); length > 0 {
			dst.name.data = (*C.char)(data)
			dst.name.len = C.size_t(length)
		} else {
			dst.name.data = nil
			dst.name.len = 0
		}
	} else {
		dst.has_name = C.bool(false)
	}
	if values := src.GetField(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*FieldDescriptorProto)(nil)))
		array := unsafe.Slice((**FieldDescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprFieldDescriptorProtoGenerated(arena, value)
		}
		dst.field.data = (**FieldDescriptorProto)(ptr)
		dst.field.len = C.size_t(len(values))
		dst.field.cap = C.size_t(len(values))
	}
	if values := src.GetNestedType(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*DescriptorProto)(nil)))
		array := unsafe.Slice((**DescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprDescriptorProtoGenerated(arena, value)
		}
		dst.nested_type.data = (**DescriptorProto)(ptr)
		dst.nested_type.len = C.size_t(len(values))
		dst.nested_type.cap = C.size_t(len(values))
	}
	if values := src.GetEnumType(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*EnumDescriptorProto)(nil)))
		array := unsafe.Slice((**EnumDescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprEnumDescriptorProtoGenerated(arena, value)
		}
		dst.enum_type.data = (**EnumDescriptorProto)(ptr)
		dst.enum_type.len = C.size_t(len(values))
		dst.enum_type.cap = C.size_t(len(values))
	}
	if values := src.GetExtensionRange(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*DescriptorProto_ExtensionRange)(nil)))
		array := unsafe.Slice((**DescriptorProto_ExtensionRange)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprDescriptorProto_ExtensionRangeGenerated(arena, value)
		}
		dst.extension_range.data = (**DescriptorProto_ExtensionRange)(ptr)
		dst.extension_range.len = C.size_t(len(values))
		dst.extension_range.cap = C.size_t(len(values))
	}
	if values := src.GetExtension(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*FieldDescriptorProto)(nil)))
		array := unsafe.Slice((**FieldDescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprFieldDescriptorProtoGenerated(arena, value)
		}
		dst.extension.data = (**FieldDescriptorProto)(ptr)
		dst.extension.len = C.size_t(len(values))
		dst.extension.cap = C.size_t(len(values))
	}
	if value := src.GetOptions(); value != nil {
		dst.options = NewReprMessageOptionsGenerated(arena, value)
	} else {
		dst.options = nil
	}
	if values := src.GetOneofDecl(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*OneofDescriptorProto)(nil)))
		array := unsafe.Slice((**OneofDescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprOneofDescriptorProtoGenerated(arena, value)
		}
		dst.oneof_decl.data = (**OneofDescriptorProto)(ptr)
		dst.oneof_decl.len = C.size_t(len(values))
		dst.oneof_decl.cap = C.size_t(len(values))
	}
	if values := src.GetReservedRange(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*DescriptorProto_ReservedRange)(nil)))
		array := unsafe.Slice((**DescriptorProto_ReservedRange)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprDescriptorProto_ReservedRangeGenerated(arena, value)
		}
		dst.reserved_range.data = (**DescriptorProto_ReservedRange)(ptr)
		dst.reserved_range.len = C.size_t(len(values))
		dst.reserved_range.cap = C.size_t(len(values))
	}
	runtime.SetStringSlice(arena, unsafe.Pointer(&dst.reserved_name), src.GetReservedName())
}

func FromReprDescriptorProtoGenerated(src *DescriptorProto) *descriptorproto.DescriptorProto {
	if src == nil {
		return nil
	}
	out := &descriptorproto.DescriptorProto{}
	if src.has_name != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.name.data), int(src.name.len))
		out.Name = &value
	} else {
		out.Name = nil
	}
	if src.field.data != nil && src.field.len > 0 {
		length := int(src.field.len)
		ptrs := unsafe.Slice((**FieldDescriptorProto)(unsafe.Pointer(src.field.data)), length)
		out.Field = make([]*descriptorproto.FieldDescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Field = append(out.Field, FromReprFieldDescriptorProtoGenerated(ptr))
		}
	}
	if src.nested_type.data != nil && src.nested_type.len > 0 {
		length := int(src.nested_type.len)
		ptrs := unsafe.Slice((**DescriptorProto)(unsafe.Pointer(src.nested_type.data)), length)
		out.NestedType = make([]*descriptorproto.DescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.NestedType = append(out.NestedType, FromReprDescriptorProtoGenerated(ptr))
		}
	}
	if src.enum_type.data != nil && src.enum_type.len > 0 {
		length := int(src.enum_type.len)
		ptrs := unsafe.Slice((**EnumDescriptorProto)(unsafe.Pointer(src.enum_type.data)), length)
		out.EnumType = make([]*descriptorproto.EnumDescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.EnumType = append(out.EnumType, FromReprEnumDescriptorProtoGenerated(ptr))
		}
	}
	if src.extension_range.data != nil && src.extension_range.len > 0 {
		length := int(src.extension_range.len)
		ptrs := unsafe.Slice((**DescriptorProto_ExtensionRange)(unsafe.Pointer(src.extension_range.data)), length)
		out.ExtensionRange = make([]*descriptorproto.DescriptorProto_ExtensionRange, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.ExtensionRange = append(out.ExtensionRange, FromReprDescriptorProto_ExtensionRangeGenerated(ptr))
		}
	}
	if src.extension.data != nil && src.extension.len > 0 {
		length := int(src.extension.len)
		ptrs := unsafe.Slice((**FieldDescriptorProto)(unsafe.Pointer(src.extension.data)), length)
		out.Extension = make([]*descriptorproto.FieldDescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Extension = append(out.Extension, FromReprFieldDescriptorProtoGenerated(ptr))
		}
	}
	if src.options != nil {
		out.Options = FromReprMessageOptionsGenerated(src.options)
	}
	if src.oneof_decl.data != nil && src.oneof_decl.len > 0 {
		length := int(src.oneof_decl.len)
		ptrs := unsafe.Slice((**OneofDescriptorProto)(unsafe.Pointer(src.oneof_decl.data)), length)
		out.OneofDecl = make([]*descriptorproto.OneofDescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.OneofDecl = append(out.OneofDecl, FromReprOneofDescriptorProtoGenerated(ptr))
		}
	}
	if src.reserved_range.data != nil && src.reserved_range.len > 0 {
		length := int(src.reserved_range.len)
		ptrs := unsafe.Slice((**DescriptorProto_ReservedRange)(unsafe.Pointer(src.reserved_range.data)), length)
		out.ReservedRange = make([]*descriptorproto.DescriptorProto_ReservedRange, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.ReservedRange = append(out.ReservedRange, FromReprDescriptorProto_ReservedRangeGenerated(ptr))
		}
	}
	out.ReservedName = runtime.CopyStringSlice(unsafe.Pointer(&src.reserved_name))
	return out
}

func NewReprDescriptorProto_ExtensionRangeGenerated(arena *runtime.Arena, src *descriptorproto.DescriptorProto_ExtensionRange) *DescriptorProto_ExtensionRange {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DescriptorProto_ExtensionRange)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_DescriptorProto_ExtensionRange)))
	IntoReprDescriptorProto_ExtensionRangeGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDescriptorProto_ExtensionRangeGenerated(arena *runtime.Arena, dst *DescriptorProto_ExtensionRange, src *descriptorproto.DescriptorProto_ExtensionRange) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Start != nil {
		dst.has_start = C.bool(true)
		dst.start = C.int32_t(*src.Start)
	} else {
		dst.has_start = C.bool(false)
	}
	if src.End != nil {
		dst.has_end = C.bool(true)
		dst.end = C.int32_t(*src.End)
	} else {
		dst.has_end = C.bool(false)
	}
	if value := src.GetOptions(); value != nil {
		dst.options = NewReprExtensionRangeOptionsGenerated(arena, value)
	} else {
		dst.options = nil
	}
}

func FromReprDescriptorProto_ExtensionRangeGenerated(src *DescriptorProto_ExtensionRange) *descriptorproto.DescriptorProto_ExtensionRange {
	if src == nil {
		return nil
	}
	out := &descriptorproto.DescriptorProto_ExtensionRange{}
	if src.has_start != C.bool(false) {
		value := int32(src.start)
		out.Start = &value
	} else {
		out.Start = nil
	}
	if src.has_end != C.bool(false) {
		value := int32(src.end)
		out.End = &value
	} else {
		out.End = nil
	}
	if src.options != nil {
		out.Options = FromReprExtensionRangeOptionsGenerated(src.options)
	}
	return out
}

func NewReprDescriptorProto_ReservedRangeGenerated(arena *runtime.Arena, src *descriptorproto.DescriptorProto_ReservedRange) *DescriptorProto_ReservedRange {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DescriptorProto_ReservedRange)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_DescriptorProto_ReservedRange)))
	IntoReprDescriptorProto_ReservedRangeGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDescriptorProto_ReservedRangeGenerated(arena *runtime.Arena, dst *DescriptorProto_ReservedRange, src *descriptorproto.DescriptorProto_ReservedRange) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Start != nil {
		dst.has_start = C.bool(true)
		dst.start = C.int32_t(*src.Start)
	} else {
		dst.has_start = C.bool(false)
	}
	if src.End != nil {
		dst.has_end = C.bool(true)
		dst.end = C.int32_t(*src.End)
	} else {
		dst.has_end = C.bool(false)
	}
}

func FromReprDescriptorProto_ReservedRangeGenerated(src *DescriptorProto_ReservedRange) *descriptorproto.DescriptorProto_ReservedRange {
	if src == nil {
		return nil
	}
	out := &descriptorproto.DescriptorProto_ReservedRange{}
	if src.has_start != C.bool(false) {
		value := int32(src.start)
		out.Start = &value
	} else {
		out.Start = nil
	}
	if src.has_end != C.bool(false) {
		value := int32(src.end)
		out.End = &value
	} else {
		out.End = nil
	}
	return out
}

func NewReprEnumDescriptorProtoGenerated(arena *runtime.Arena, src *descriptorproto.EnumDescriptorProto) *EnumDescriptorProto {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*EnumDescriptorProto)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_EnumDescriptorProto)))
	IntoReprEnumDescriptorProtoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprEnumDescriptorProtoGenerated(arena *runtime.Arena, dst *EnumDescriptorProto, src *descriptorproto.EnumDescriptorProto) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Name != nil {
		dst.has_name = C.bool(true)
		if data, length := arena.AllocString(*src.Name); length > 0 {
			dst.name.data = (*C.char)(data)
			dst.name.len = C.size_t(length)
		} else {
			dst.name.data = nil
			dst.name.len = 0
		}
	} else {
		dst.has_name = C.bool(false)
	}
	if values := src.GetValue(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*EnumValueDescriptorProto)(nil)))
		array := unsafe.Slice((**EnumValueDescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprEnumValueDescriptorProtoGenerated(arena, value)
		}
		dst.value.data = (**EnumValueDescriptorProto)(ptr)
		dst.value.len = C.size_t(len(values))
		dst.value.cap = C.size_t(len(values))
	}
	if value := src.GetOptions(); value != nil {
		dst.options = NewReprEnumOptionsGenerated(arena, value)
	} else {
		dst.options = nil
	}
	if values := src.GetReservedRange(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*EnumDescriptorProto_EnumReservedRange)(nil)))
		array := unsafe.Slice((**EnumDescriptorProto_EnumReservedRange)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprEnumDescriptorProto_EnumReservedRangeGenerated(arena, value)
		}
		dst.reserved_range.data = (**EnumDescriptorProto_EnumReservedRange)(ptr)
		dst.reserved_range.len = C.size_t(len(values))
		dst.reserved_range.cap = C.size_t(len(values))
	}
	runtime.SetStringSlice(arena, unsafe.Pointer(&dst.reserved_name), src.GetReservedName())
}

func FromReprEnumDescriptorProtoGenerated(src *EnumDescriptorProto) *descriptorproto.EnumDescriptorProto {
	if src == nil {
		return nil
	}
	out := &descriptorproto.EnumDescriptorProto{}
	if src.has_name != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.name.data), int(src.name.len))
		out.Name = &value
	} else {
		out.Name = nil
	}
	if src.value.data != nil && src.value.len > 0 {
		length := int(src.value.len)
		ptrs := unsafe.Slice((**EnumValueDescriptorProto)(unsafe.Pointer(src.value.data)), length)
		out.Value = make([]*descriptorproto.EnumValueDescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Value = append(out.Value, FromReprEnumValueDescriptorProtoGenerated(ptr))
		}
	}
	if src.options != nil {
		out.Options = FromReprEnumOptionsGenerated(src.options)
	}
	if src.reserved_range.data != nil && src.reserved_range.len > 0 {
		length := int(src.reserved_range.len)
		ptrs := unsafe.Slice((**EnumDescriptorProto_EnumReservedRange)(unsafe.Pointer(src.reserved_range.data)), length)
		out.ReservedRange = make([]*descriptorproto.EnumDescriptorProto_EnumReservedRange, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.ReservedRange = append(out.ReservedRange, FromReprEnumDescriptorProto_EnumReservedRangeGenerated(ptr))
		}
	}
	out.ReservedName = runtime.CopyStringSlice(unsafe.Pointer(&src.reserved_name))
	return out
}

func NewReprEnumDescriptorProto_EnumReservedRangeGenerated(arena *runtime.Arena, src *descriptorproto.EnumDescriptorProto_EnumReservedRange) *EnumDescriptorProto_EnumReservedRange {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*EnumDescriptorProto_EnumReservedRange)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_EnumDescriptorProto_EnumReservedRange)))
	IntoReprEnumDescriptorProto_EnumReservedRangeGenerated(arena, ptr, src)
	return ptr
}

func IntoReprEnumDescriptorProto_EnumReservedRangeGenerated(arena *runtime.Arena, dst *EnumDescriptorProto_EnumReservedRange, src *descriptorproto.EnumDescriptorProto_EnumReservedRange) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Start != nil {
		dst.has_start = C.bool(true)
		dst.start = C.int32_t(*src.Start)
	} else {
		dst.has_start = C.bool(false)
	}
	if src.End != nil {
		dst.has_end = C.bool(true)
		dst.end = C.int32_t(*src.End)
	} else {
		dst.has_end = C.bool(false)
	}
}

func FromReprEnumDescriptorProto_EnumReservedRangeGenerated(src *EnumDescriptorProto_EnumReservedRange) *descriptorproto.EnumDescriptorProto_EnumReservedRange {
	if src == nil {
		return nil
	}
	out := &descriptorproto.EnumDescriptorProto_EnumReservedRange{}
	if src.has_start != C.bool(false) {
		value := int32(src.start)
		out.Start = &value
	} else {
		out.Start = nil
	}
	if src.has_end != C.bool(false) {
		value := int32(src.end)
		out.End = &value
	} else {
		out.End = nil
	}
	return out
}

func NewReprEnumOptionsGenerated(arena *runtime.Arena, src *descriptorproto.EnumOptions) *EnumOptions {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*EnumOptions)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_EnumOptions)))
	IntoReprEnumOptionsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprEnumOptionsGenerated(arena *runtime.Arena, dst *EnumOptions, src *descriptorproto.EnumOptions) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.AllowAlias != nil {
		dst.has_allow_alias = C.bool(true)
		dst.allow_alias = C.bool(*src.AllowAlias)
	} else {
		dst.has_allow_alias = C.bool(false)
	}
	if src.Deprecated != nil {
		dst.has_deprecated = C.bool(true)
		dst.deprecated = C.bool(*src.Deprecated)
	} else {
		dst.has_deprecated = C.bool(false)
	}
	if values := src.GetUninterpretedOption(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*UninterpretedOption)(nil)))
		array := unsafe.Slice((**UninterpretedOption)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprUninterpretedOptionGenerated(arena, value)
		}
		dst.uninterpreted_option.data = (**UninterpretedOption)(ptr)
		dst.uninterpreted_option.len = C.size_t(len(values))
		dst.uninterpreted_option.cap = C.size_t(len(values))
	}
}

func FromReprEnumOptionsGenerated(src *EnumOptions) *descriptorproto.EnumOptions {
	if src == nil {
		return nil
	}
	out := &descriptorproto.EnumOptions{}
	if src.has_allow_alias != C.bool(false) {
		value := bool(src.allow_alias)
		out.AllowAlias = &value
	} else {
		out.AllowAlias = nil
	}
	if src.has_deprecated != C.bool(false) {
		value := bool(src.deprecated)
		out.Deprecated = &value
	} else {
		out.Deprecated = nil
	}
	if src.uninterpreted_option.data != nil && src.uninterpreted_option.len > 0 {
		length := int(src.uninterpreted_option.len)
		ptrs := unsafe.Slice((**UninterpretedOption)(unsafe.Pointer(src.uninterpreted_option.data)), length)
		out.UninterpretedOption = make([]*descriptorproto.UninterpretedOption, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.UninterpretedOption = append(out.UninterpretedOption, FromReprUninterpretedOptionGenerated(ptr))
		}
	}
	return out
}

func NewReprEnumValueDescriptorProtoGenerated(arena *runtime.Arena, src *descriptorproto.EnumValueDescriptorProto) *EnumValueDescriptorProto {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*EnumValueDescriptorProto)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_EnumValueDescriptorProto)))
	IntoReprEnumValueDescriptorProtoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprEnumValueDescriptorProtoGenerated(arena *runtime.Arena, dst *EnumValueDescriptorProto, src *descriptorproto.EnumValueDescriptorProto) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Name != nil {
		dst.has_name = C.bool(true)
		if data, length := arena.AllocString(*src.Name); length > 0 {
			dst.name.data = (*C.char)(data)
			dst.name.len = C.size_t(length)
		} else {
			dst.name.data = nil
			dst.name.len = 0
		}
	} else {
		dst.has_name = C.bool(false)
	}
	if src.Number != nil {
		dst.has_number = C.bool(true)
		dst.number = C.int32_t(*src.Number)
	} else {
		dst.has_number = C.bool(false)
	}
	if value := src.GetOptions(); value != nil {
		dst.options = NewReprEnumValueOptionsGenerated(arena, value)
	} else {
		dst.options = nil
	}
}

func FromReprEnumValueDescriptorProtoGenerated(src *EnumValueDescriptorProto) *descriptorproto.EnumValueDescriptorProto {
	if src == nil {
		return nil
	}
	out := &descriptorproto.EnumValueDescriptorProto{}
	if src.has_name != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.name.data), int(src.name.len))
		out.Name = &value
	} else {
		out.Name = nil
	}
	if src.has_number != C.bool(false) {
		value := int32(src.number)
		out.Number = &value
	} else {
		out.Number = nil
	}
	if src.options != nil {
		out.Options = FromReprEnumValueOptionsGenerated(src.options)
	}
	return out
}

func NewReprEnumValueOptionsGenerated(arena *runtime.Arena, src *descriptorproto.EnumValueOptions) *EnumValueOptions {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*EnumValueOptions)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_EnumValueOptions)))
	IntoReprEnumValueOptionsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprEnumValueOptionsGenerated(arena *runtime.Arena, dst *EnumValueOptions, src *descriptorproto.EnumValueOptions) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Deprecated != nil {
		dst.has_deprecated = C.bool(true)
		dst.deprecated = C.bool(*src.Deprecated)
	} else {
		dst.has_deprecated = C.bool(false)
	}
	if values := src.GetUninterpretedOption(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*UninterpretedOption)(nil)))
		array := unsafe.Slice((**UninterpretedOption)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprUninterpretedOptionGenerated(arena, value)
		}
		dst.uninterpreted_option.data = (**UninterpretedOption)(ptr)
		dst.uninterpreted_option.len = C.size_t(len(values))
		dst.uninterpreted_option.cap = C.size_t(len(values))
	}
}

func FromReprEnumValueOptionsGenerated(src *EnumValueOptions) *descriptorproto.EnumValueOptions {
	if src == nil {
		return nil
	}
	out := &descriptorproto.EnumValueOptions{}
	if src.has_deprecated != C.bool(false) {
		value := bool(src.deprecated)
		out.Deprecated = &value
	} else {
		out.Deprecated = nil
	}
	if src.uninterpreted_option.data != nil && src.uninterpreted_option.len > 0 {
		length := int(src.uninterpreted_option.len)
		ptrs := unsafe.Slice((**UninterpretedOption)(unsafe.Pointer(src.uninterpreted_option.data)), length)
		out.UninterpretedOption = make([]*descriptorproto.UninterpretedOption, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.UninterpretedOption = append(out.UninterpretedOption, FromReprUninterpretedOptionGenerated(ptr))
		}
	}
	return out
}

func NewReprExtensionRangeOptionsGenerated(arena *runtime.Arena, src *descriptorproto.ExtensionRangeOptions) *ExtensionRangeOptions {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ExtensionRangeOptions)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_ExtensionRangeOptions)))
	IntoReprExtensionRangeOptionsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprExtensionRangeOptionsGenerated(arena *runtime.Arena, dst *ExtensionRangeOptions, src *descriptorproto.ExtensionRangeOptions) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetUninterpretedOption(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*UninterpretedOption)(nil)))
		array := unsafe.Slice((**UninterpretedOption)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprUninterpretedOptionGenerated(arena, value)
		}
		dst.uninterpreted_option.data = (**UninterpretedOption)(ptr)
		dst.uninterpreted_option.len = C.size_t(len(values))
		dst.uninterpreted_option.cap = C.size_t(len(values))
	}
}

func FromReprExtensionRangeOptionsGenerated(src *ExtensionRangeOptions) *descriptorproto.ExtensionRangeOptions {
	if src == nil {
		return nil
	}
	out := &descriptorproto.ExtensionRangeOptions{}
	if src.uninterpreted_option.data != nil && src.uninterpreted_option.len > 0 {
		length := int(src.uninterpreted_option.len)
		ptrs := unsafe.Slice((**UninterpretedOption)(unsafe.Pointer(src.uninterpreted_option.data)), length)
		out.UninterpretedOption = make([]*descriptorproto.UninterpretedOption, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.UninterpretedOption = append(out.UninterpretedOption, FromReprUninterpretedOptionGenerated(ptr))
		}
	}
	return out
}

func NewReprFieldDescriptorProtoGenerated(arena *runtime.Arena, src *descriptorproto.FieldDescriptorProto) *FieldDescriptorProto {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FieldDescriptorProto)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_FieldDescriptorProto)))
	IntoReprFieldDescriptorProtoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFieldDescriptorProtoGenerated(arena *runtime.Arena, dst *FieldDescriptorProto, src *descriptorproto.FieldDescriptorProto) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Name != nil {
		dst.has_name = C.bool(true)
		if data, length := arena.AllocString(*src.Name); length > 0 {
			dst.name.data = (*C.char)(data)
			dst.name.len = C.size_t(length)
		} else {
			dst.name.data = nil
			dst.name.len = 0
		}
	} else {
		dst.has_name = C.bool(false)
	}
	if src.Extendee != nil {
		dst.has_extendee = C.bool(true)
		if data, length := arena.AllocString(*src.Extendee); length > 0 {
			dst.extendee.data = (*C.char)(data)
			dst.extendee.len = C.size_t(length)
		} else {
			dst.extendee.data = nil
			dst.extendee.len = 0
		}
	} else {
		dst.has_extendee = C.bool(false)
	}
	if src.Number != nil {
		dst.has_number = C.bool(true)
		dst.number = C.int32_t(*src.Number)
	} else {
		dst.has_number = C.bool(false)
	}
	if src.Label != nil {
		dst.has_label = C.bool(true)
		dst.label = C.int32_t(int32(*src.Label))
	} else {
		dst.has_label = C.bool(false)
	}
	if src.Type != nil {
		dst.has_type_field = C.bool(true)
		dst.type_field = C.int32_t(int32(*src.Type))
	} else {
		dst.has_type_field = C.bool(false)
	}
	if src.TypeName != nil {
		dst.has_type_name = C.bool(true)
		if data, length := arena.AllocString(*src.TypeName); length > 0 {
			dst.type_name.data = (*C.char)(data)
			dst.type_name.len = C.size_t(length)
		} else {
			dst.type_name.data = nil
			dst.type_name.len = 0
		}
	} else {
		dst.has_type_name = C.bool(false)
	}
	if src.DefaultValue != nil {
		dst.has_default_value = C.bool(true)
		if data, length := arena.AllocString(*src.DefaultValue); length > 0 {
			dst.default_value.data = (*C.char)(data)
			dst.default_value.len = C.size_t(length)
		} else {
			dst.default_value.data = nil
			dst.default_value.len = 0
		}
	} else {
		dst.has_default_value = C.bool(false)
	}
	if value := src.GetOptions(); value != nil {
		dst.options = NewReprFieldOptionsGenerated(arena, value)
	} else {
		dst.options = nil
	}
	if src.OneofIndex != nil {
		dst.has_oneof_index = C.bool(true)
		dst.oneof_index = C.int32_t(*src.OneofIndex)
	} else {
		dst.has_oneof_index = C.bool(false)
	}
	if src.JsonName != nil {
		dst.has_json_name = C.bool(true)
		if data, length := arena.AllocString(*src.JsonName); length > 0 {
			dst.json_name.data = (*C.char)(data)
			dst.json_name.len = C.size_t(length)
		} else {
			dst.json_name.data = nil
			dst.json_name.len = 0
		}
	} else {
		dst.has_json_name = C.bool(false)
	}
}

func FromReprFieldDescriptorProtoGenerated(src *FieldDescriptorProto) *descriptorproto.FieldDescriptorProto {
	if src == nil {
		return nil
	}
	out := &descriptorproto.FieldDescriptorProto{}
	if src.has_name != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.name.data), int(src.name.len))
		out.Name = &value
	} else {
		out.Name = nil
	}
	if src.has_extendee != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.extendee.data), int(src.extendee.len))
		out.Extendee = &value
	} else {
		out.Extendee = nil
	}
	if src.has_number != C.bool(false) {
		value := int32(src.number)
		out.Number = &value
	} else {
		out.Number = nil
	}
	if src.has_label != C.bool(false) {
		value := descriptorproto.FieldDescriptorProto_Label(int32(src.label))
		out.Label = &value
	} else {
		out.Label = nil
	}
	if src.has_type_field != C.bool(false) {
		value := descriptorproto.FieldDescriptorProto_Type(int32(src.type_field))
		out.Type = &value
	} else {
		out.Type = nil
	}
	if src.has_type_name != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.type_name.data), int(src.type_name.len))
		out.TypeName = &value
	} else {
		out.TypeName = nil
	}
	if src.has_default_value != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.default_value.data), int(src.default_value.len))
		out.DefaultValue = &value
	} else {
		out.DefaultValue = nil
	}
	if src.options != nil {
		out.Options = FromReprFieldOptionsGenerated(src.options)
	}
	if src.has_oneof_index != C.bool(false) {
		value := int32(src.oneof_index)
		out.OneofIndex = &value
	} else {
		out.OneofIndex = nil
	}
	if src.has_json_name != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.json_name.data), int(src.json_name.len))
		out.JsonName = &value
	} else {
		out.JsonName = nil
	}
	return out
}

func NewReprFieldOptionsGenerated(arena *runtime.Arena, src *descriptorproto.FieldOptions) *FieldOptions {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FieldOptions)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_FieldOptions)))
	IntoReprFieldOptionsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFieldOptionsGenerated(arena *runtime.Arena, dst *FieldOptions, src *descriptorproto.FieldOptions) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Ctype != nil {
		dst.has_ctype = C.bool(true)
		dst.ctype = C.int32_t(int32(*src.Ctype))
	} else {
		dst.has_ctype = C.bool(false)
	}
	if src.Packed != nil {
		dst.has_packed = C.bool(true)
		dst.packed = C.bool(*src.Packed)
	} else {
		dst.has_packed = C.bool(false)
	}
	if src.Deprecated != nil {
		dst.has_deprecated = C.bool(true)
		dst.deprecated = C.bool(*src.Deprecated)
	} else {
		dst.has_deprecated = C.bool(false)
	}
	if src.Lazy != nil {
		dst.has_lazy = C.bool(true)
		dst.lazy = C.bool(*src.Lazy)
	} else {
		dst.has_lazy = C.bool(false)
	}
	if src.Jstype != nil {
		dst.has_jstype = C.bool(true)
		dst.jstype = C.int32_t(int32(*src.Jstype))
	} else {
		dst.has_jstype = C.bool(false)
	}
	if src.Weak != nil {
		dst.has_weak = C.bool(true)
		dst.weak = C.bool(*src.Weak)
	} else {
		dst.has_weak = C.bool(false)
	}
	if values := src.GetUninterpretedOption(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*UninterpretedOption)(nil)))
		array := unsafe.Slice((**UninterpretedOption)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprUninterpretedOptionGenerated(arena, value)
		}
		dst.uninterpreted_option.data = (**UninterpretedOption)(ptr)
		dst.uninterpreted_option.len = C.size_t(len(values))
		dst.uninterpreted_option.cap = C.size_t(len(values))
	}
}

func FromReprFieldOptionsGenerated(src *FieldOptions) *descriptorproto.FieldOptions {
	if src == nil {
		return nil
	}
	out := &descriptorproto.FieldOptions{}
	if src.has_ctype != C.bool(false) {
		value := descriptorproto.FieldOptions_CType(int32(src.ctype))
		out.Ctype = &value
	} else {
		out.Ctype = nil
	}
	if src.has_packed != C.bool(false) {
		value := bool(src.packed)
		out.Packed = &value
	} else {
		out.Packed = nil
	}
	if src.has_deprecated != C.bool(false) {
		value := bool(src.deprecated)
		out.Deprecated = &value
	} else {
		out.Deprecated = nil
	}
	if src.has_lazy != C.bool(false) {
		value := bool(src.lazy)
		out.Lazy = &value
	} else {
		out.Lazy = nil
	}
	if src.has_jstype != C.bool(false) {
		value := descriptorproto.FieldOptions_JSType(int32(src.jstype))
		out.Jstype = &value
	} else {
		out.Jstype = nil
	}
	if src.has_weak != C.bool(false) {
		value := bool(src.weak)
		out.Weak = &value
	} else {
		out.Weak = nil
	}
	if src.uninterpreted_option.data != nil && src.uninterpreted_option.len > 0 {
		length := int(src.uninterpreted_option.len)
		ptrs := unsafe.Slice((**UninterpretedOption)(unsafe.Pointer(src.uninterpreted_option.data)), length)
		out.UninterpretedOption = make([]*descriptorproto.UninterpretedOption, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.UninterpretedOption = append(out.UninterpretedOption, FromReprUninterpretedOptionGenerated(ptr))
		}
	}
	return out
}

func NewReprFileDescriptorProtoGenerated(arena *runtime.Arena, src *descriptorproto.FileDescriptorProto) *FileDescriptorProto {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FileDescriptorProto)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_FileDescriptorProto)))
	IntoReprFileDescriptorProtoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFileDescriptorProtoGenerated(arena *runtime.Arena, dst *FileDescriptorProto, src *descriptorproto.FileDescriptorProto) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Name != nil {
		dst.has_name = C.bool(true)
		if data, length := arena.AllocString(*src.Name); length > 0 {
			dst.name.data = (*C.char)(data)
			dst.name.len = C.size_t(length)
		} else {
			dst.name.data = nil
			dst.name.len = 0
		}
	} else {
		dst.has_name = C.bool(false)
	}
	if src.Package != nil {
		dst.has_package_field = C.bool(true)
		if data, length := arena.AllocString(*src.Package); length > 0 {
			dst.package_field.data = (*C.char)(data)
			dst.package_field.len = C.size_t(length)
		} else {
			dst.package_field.data = nil
			dst.package_field.len = 0
		}
	} else {
		dst.has_package_field = C.bool(false)
	}
	runtime.SetStringSlice(arena, unsafe.Pointer(&dst.dependency), src.GetDependency())
	if values := src.GetMessageType(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*DescriptorProto)(nil)))
		array := unsafe.Slice((**DescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprDescriptorProtoGenerated(arena, value)
		}
		dst.message_type.data = (**DescriptorProto)(ptr)
		dst.message_type.len = C.size_t(len(values))
		dst.message_type.cap = C.size_t(len(values))
	}
	if values := src.GetEnumType(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*EnumDescriptorProto)(nil)))
		array := unsafe.Slice((**EnumDescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprEnumDescriptorProtoGenerated(arena, value)
		}
		dst.enum_type.data = (**EnumDescriptorProto)(ptr)
		dst.enum_type.len = C.size_t(len(values))
		dst.enum_type.cap = C.size_t(len(values))
	}
	if values := src.GetService(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*ServiceDescriptorProto)(nil)))
		array := unsafe.Slice((**ServiceDescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprServiceDescriptorProtoGenerated(arena, value)
		}
		dst.service.data = (**ServiceDescriptorProto)(ptr)
		dst.service.len = C.size_t(len(values))
		dst.service.cap = C.size_t(len(values))
	}
	if values := src.GetExtension(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*FieldDescriptorProto)(nil)))
		array := unsafe.Slice((**FieldDescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprFieldDescriptorProtoGenerated(arena, value)
		}
		dst.extension.data = (**FieldDescriptorProto)(ptr)
		dst.extension.len = C.size_t(len(values))
		dst.extension.cap = C.size_t(len(values))
	}
	if value := src.GetOptions(); value != nil {
		dst.options = NewReprFileOptionsGenerated(arena, value)
	} else {
		dst.options = nil
	}
	if value := src.GetSourceCodeInfo(); value != nil {
		dst.source_code_info = NewReprSourceCodeInfoGenerated(arena, value)
	} else {
		dst.source_code_info = nil
	}
	if values := src.GetPublicDependency(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.int32_t(0)))
		array := unsafe.Slice((*C.int32_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.int32_t(value)
		}
		dst.public_dependency.data = (*C.int32_t)(ptr)
		dst.public_dependency.len = C.size_t(len(values))
		dst.public_dependency.cap = C.size_t(len(values))
	}
	if values := src.GetWeakDependency(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.int32_t(0)))
		array := unsafe.Slice((*C.int32_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.int32_t(value)
		}
		dst.weak_dependency.data = (*C.int32_t)(ptr)
		dst.weak_dependency.len = C.size_t(len(values))
		dst.weak_dependency.cap = C.size_t(len(values))
	}
	if src.Syntax != nil {
		dst.has_syntax = C.bool(true)
		if data, length := arena.AllocString(*src.Syntax); length > 0 {
			dst.syntax.data = (*C.char)(data)
			dst.syntax.len = C.size_t(length)
		} else {
			dst.syntax.data = nil
			dst.syntax.len = 0
		}
	} else {
		dst.has_syntax = C.bool(false)
	}
}

func FromReprFileDescriptorProtoGenerated(src *FileDescriptorProto) *descriptorproto.FileDescriptorProto {
	if src == nil {
		return nil
	}
	out := &descriptorproto.FileDescriptorProto{}
	if src.has_name != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.name.data), int(src.name.len))
		out.Name = &value
	} else {
		out.Name = nil
	}
	if src.has_package_field != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.package_field.data), int(src.package_field.len))
		out.Package = &value
	} else {
		out.Package = nil
	}
	out.Dependency = runtime.CopyStringSlice(unsafe.Pointer(&src.dependency))
	if src.message_type.data != nil && src.message_type.len > 0 {
		length := int(src.message_type.len)
		ptrs := unsafe.Slice((**DescriptorProto)(unsafe.Pointer(src.message_type.data)), length)
		out.MessageType = make([]*descriptorproto.DescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.MessageType = append(out.MessageType, FromReprDescriptorProtoGenerated(ptr))
		}
	}
	if src.enum_type.data != nil && src.enum_type.len > 0 {
		length := int(src.enum_type.len)
		ptrs := unsafe.Slice((**EnumDescriptorProto)(unsafe.Pointer(src.enum_type.data)), length)
		out.EnumType = make([]*descriptorproto.EnumDescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.EnumType = append(out.EnumType, FromReprEnumDescriptorProtoGenerated(ptr))
		}
	}
	if src.service.data != nil && src.service.len > 0 {
		length := int(src.service.len)
		ptrs := unsafe.Slice((**ServiceDescriptorProto)(unsafe.Pointer(src.service.data)), length)
		out.Service = make([]*descriptorproto.ServiceDescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Service = append(out.Service, FromReprServiceDescriptorProtoGenerated(ptr))
		}
	}
	if src.extension.data != nil && src.extension.len > 0 {
		length := int(src.extension.len)
		ptrs := unsafe.Slice((**FieldDescriptorProto)(unsafe.Pointer(src.extension.data)), length)
		out.Extension = make([]*descriptorproto.FieldDescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Extension = append(out.Extension, FromReprFieldDescriptorProtoGenerated(ptr))
		}
	}
	if src.options != nil {
		out.Options = FromReprFileOptionsGenerated(src.options)
	}
	if src.source_code_info != nil {
		out.SourceCodeInfo = FromReprSourceCodeInfoGenerated(src.source_code_info)
	}
	if src.public_dependency.data != nil && src.public_dependency.len > 0 {
		length := int(src.public_dependency.len)
		values := unsafe.Slice((*C.int32_t)(unsafe.Pointer(src.public_dependency.data)), length)
		out.PublicDependency = make([]int32, 0, length)
		for _, value := range values {
			out.PublicDependency = append(out.PublicDependency, int32(value))
		}
	}
	if src.weak_dependency.data != nil && src.weak_dependency.len > 0 {
		length := int(src.weak_dependency.len)
		values := unsafe.Slice((*C.int32_t)(unsafe.Pointer(src.weak_dependency.data)), length)
		out.WeakDependency = make([]int32, 0, length)
		for _, value := range values {
			out.WeakDependency = append(out.WeakDependency, int32(value))
		}
	}
	if src.has_syntax != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.syntax.data), int(src.syntax.len))
		out.Syntax = &value
	} else {
		out.Syntax = nil
	}
	return out
}

func NewReprFileDescriptorSetGenerated(arena *runtime.Arena, src *descriptorproto.FileDescriptorSet) *FileDescriptorSet {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FileDescriptorSet)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_FileDescriptorSet)))
	IntoReprFileDescriptorSetGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFileDescriptorSetGenerated(arena *runtime.Arena, dst *FileDescriptorSet, src *descriptorproto.FileDescriptorSet) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetFile(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*FileDescriptorProto)(nil)))
		array := unsafe.Slice((**FileDescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprFileDescriptorProtoGenerated(arena, value)
		}
		dst.file.data = (**FileDescriptorProto)(ptr)
		dst.file.len = C.size_t(len(values))
		dst.file.cap = C.size_t(len(values))
	}
}

func FromReprFileDescriptorSetGenerated(src *FileDescriptorSet) *descriptorproto.FileDescriptorSet {
	if src == nil {
		return nil
	}
	out := &descriptorproto.FileDescriptorSet{}
	if src.file.data != nil && src.file.len > 0 {
		length := int(src.file.len)
		ptrs := unsafe.Slice((**FileDescriptorProto)(unsafe.Pointer(src.file.data)), length)
		out.File = make([]*descriptorproto.FileDescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.File = append(out.File, FromReprFileDescriptorProtoGenerated(ptr))
		}
	}
	return out
}

func NewReprFileOptionsGenerated(arena *runtime.Arena, src *descriptorproto.FileOptions) *FileOptions {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FileOptions)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_FileOptions)))
	IntoReprFileOptionsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFileOptionsGenerated(arena *runtime.Arena, dst *FileOptions, src *descriptorproto.FileOptions) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.JavaPackage != nil {
		dst.has_java_package = C.bool(true)
		if data, length := arena.AllocString(*src.JavaPackage); length > 0 {
			dst.java_package.data = (*C.char)(data)
			dst.java_package.len = C.size_t(length)
		} else {
			dst.java_package.data = nil
			dst.java_package.len = 0
		}
	} else {
		dst.has_java_package = C.bool(false)
	}
	if src.JavaOuterClassname != nil {
		dst.has_java_outer_classname = C.bool(true)
		if data, length := arena.AllocString(*src.JavaOuterClassname); length > 0 {
			dst.java_outer_classname.data = (*C.char)(data)
			dst.java_outer_classname.len = C.size_t(length)
		} else {
			dst.java_outer_classname.data = nil
			dst.java_outer_classname.len = 0
		}
	} else {
		dst.has_java_outer_classname = C.bool(false)
	}
	if src.OptimizeFor != nil {
		dst.has_optimize_for = C.bool(true)
		dst.optimize_for = C.int32_t(int32(*src.OptimizeFor))
	} else {
		dst.has_optimize_for = C.bool(false)
	}
	if src.JavaMultipleFiles != nil {
		dst.has_java_multiple_files = C.bool(true)
		dst.java_multiple_files = C.bool(*src.JavaMultipleFiles)
	} else {
		dst.has_java_multiple_files = C.bool(false)
	}
	if src.GoPackage != nil {
		dst.has_go_package = C.bool(true)
		if data, length := arena.AllocString(*src.GoPackage); length > 0 {
			dst.go_package.data = (*C.char)(data)
			dst.go_package.len = C.size_t(length)
		} else {
			dst.go_package.data = nil
			dst.go_package.len = 0
		}
	} else {
		dst.has_go_package = C.bool(false)
	}
	if src.CcGenericServices != nil {
		dst.has_cc_generic_services = C.bool(true)
		dst.cc_generic_services = C.bool(*src.CcGenericServices)
	} else {
		dst.has_cc_generic_services = C.bool(false)
	}
	if src.JavaGenericServices != nil {
		dst.has_java_generic_services = C.bool(true)
		dst.java_generic_services = C.bool(*src.JavaGenericServices)
	} else {
		dst.has_java_generic_services = C.bool(false)
	}
	if src.PyGenericServices != nil {
		dst.has_py_generic_services = C.bool(true)
		dst.py_generic_services = C.bool(*src.PyGenericServices)
	} else {
		dst.has_py_generic_services = C.bool(false)
	}
	if src.JavaGenerateEqualsAndHash != nil {
		dst.has_java_generate_equals_and_hash = C.bool(true)
		dst.java_generate_equals_and_hash = C.bool(*src.JavaGenerateEqualsAndHash)
	} else {
		dst.has_java_generate_equals_and_hash = C.bool(false)
	}
	if src.Deprecated != nil {
		dst.has_deprecated = C.bool(true)
		dst.deprecated = C.bool(*src.Deprecated)
	} else {
		dst.has_deprecated = C.bool(false)
	}
	if src.JavaStringCheckUtf8 != nil {
		dst.has_java_string_check_utf8 = C.bool(true)
		dst.java_string_check_utf8 = C.bool(*src.JavaStringCheckUtf8)
	} else {
		dst.has_java_string_check_utf8 = C.bool(false)
	}
	if src.CcEnableArenas != nil {
		dst.has_cc_enable_arenas = C.bool(true)
		dst.cc_enable_arenas = C.bool(*src.CcEnableArenas)
	} else {
		dst.has_cc_enable_arenas = C.bool(false)
	}
	if src.ObjcClassPrefix != nil {
		dst.has_objc_class_prefix = C.bool(true)
		if data, length := arena.AllocString(*src.ObjcClassPrefix); length > 0 {
			dst.objc_class_prefix.data = (*C.char)(data)
			dst.objc_class_prefix.len = C.size_t(length)
		} else {
			dst.objc_class_prefix.data = nil
			dst.objc_class_prefix.len = 0
		}
	} else {
		dst.has_objc_class_prefix = C.bool(false)
	}
	if src.CsharpNamespace != nil {
		dst.has_csharp_namespace = C.bool(true)
		if data, length := arena.AllocString(*src.CsharpNamespace); length > 0 {
			dst.csharp_namespace.data = (*C.char)(data)
			dst.csharp_namespace.len = C.size_t(length)
		} else {
			dst.csharp_namespace.data = nil
			dst.csharp_namespace.len = 0
		}
	} else {
		dst.has_csharp_namespace = C.bool(false)
	}
	if src.SwiftPrefix != nil {
		dst.has_swift_prefix = C.bool(true)
		if data, length := arena.AllocString(*src.SwiftPrefix); length > 0 {
			dst.swift_prefix.data = (*C.char)(data)
			dst.swift_prefix.len = C.size_t(length)
		} else {
			dst.swift_prefix.data = nil
			dst.swift_prefix.len = 0
		}
	} else {
		dst.has_swift_prefix = C.bool(false)
	}
	if src.PhpClassPrefix != nil {
		dst.has_php_class_prefix = C.bool(true)
		if data, length := arena.AllocString(*src.PhpClassPrefix); length > 0 {
			dst.php_class_prefix.data = (*C.char)(data)
			dst.php_class_prefix.len = C.size_t(length)
		} else {
			dst.php_class_prefix.data = nil
			dst.php_class_prefix.len = 0
		}
	} else {
		dst.has_php_class_prefix = C.bool(false)
	}
	if src.PhpNamespace != nil {
		dst.has_php_namespace = C.bool(true)
		if data, length := arena.AllocString(*src.PhpNamespace); length > 0 {
			dst.php_namespace.data = (*C.char)(data)
			dst.php_namespace.len = C.size_t(length)
		} else {
			dst.php_namespace.data = nil
			dst.php_namespace.len = 0
		}
	} else {
		dst.has_php_namespace = C.bool(false)
	}
	if src.PhpGenericServices != nil {
		dst.has_php_generic_services = C.bool(true)
		dst.php_generic_services = C.bool(*src.PhpGenericServices)
	} else {
		dst.has_php_generic_services = C.bool(false)
	}
	if values := src.GetUninterpretedOption(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*UninterpretedOption)(nil)))
		array := unsafe.Slice((**UninterpretedOption)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprUninterpretedOptionGenerated(arena, value)
		}
		dst.uninterpreted_option.data = (**UninterpretedOption)(ptr)
		dst.uninterpreted_option.len = C.size_t(len(values))
		dst.uninterpreted_option.cap = C.size_t(len(values))
	}
}

func FromReprFileOptionsGenerated(src *FileOptions) *descriptorproto.FileOptions {
	if src == nil {
		return nil
	}
	out := &descriptorproto.FileOptions{}
	if src.has_java_package != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.java_package.data), int(src.java_package.len))
		out.JavaPackage = &value
	} else {
		out.JavaPackage = nil
	}
	if src.has_java_outer_classname != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.java_outer_classname.data), int(src.java_outer_classname.len))
		out.JavaOuterClassname = &value
	} else {
		out.JavaOuterClassname = nil
	}
	if src.has_optimize_for != C.bool(false) {
		value := descriptorproto.FileOptions_OptimizeMode(int32(src.optimize_for))
		out.OptimizeFor = &value
	} else {
		out.OptimizeFor = nil
	}
	if src.has_java_multiple_files != C.bool(false) {
		value := bool(src.java_multiple_files)
		out.JavaMultipleFiles = &value
	} else {
		out.JavaMultipleFiles = nil
	}
	if src.has_go_package != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.go_package.data), int(src.go_package.len))
		out.GoPackage = &value
	} else {
		out.GoPackage = nil
	}
	if src.has_cc_generic_services != C.bool(false) {
		value := bool(src.cc_generic_services)
		out.CcGenericServices = &value
	} else {
		out.CcGenericServices = nil
	}
	if src.has_java_generic_services != C.bool(false) {
		value := bool(src.java_generic_services)
		out.JavaGenericServices = &value
	} else {
		out.JavaGenericServices = nil
	}
	if src.has_py_generic_services != C.bool(false) {
		value := bool(src.py_generic_services)
		out.PyGenericServices = &value
	} else {
		out.PyGenericServices = nil
	}
	if src.has_java_generate_equals_and_hash != C.bool(false) {
		value := bool(src.java_generate_equals_and_hash)
		out.JavaGenerateEqualsAndHash = &value
	} else {
		out.JavaGenerateEqualsAndHash = nil
	}
	if src.has_deprecated != C.bool(false) {
		value := bool(src.deprecated)
		out.Deprecated = &value
	} else {
		out.Deprecated = nil
	}
	if src.has_java_string_check_utf8 != C.bool(false) {
		value := bool(src.java_string_check_utf8)
		out.JavaStringCheckUtf8 = &value
	} else {
		out.JavaStringCheckUtf8 = nil
	}
	if src.has_cc_enable_arenas != C.bool(false) {
		value := bool(src.cc_enable_arenas)
		out.CcEnableArenas = &value
	} else {
		out.CcEnableArenas = nil
	}
	if src.has_objc_class_prefix != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.objc_class_prefix.data), int(src.objc_class_prefix.len))
		out.ObjcClassPrefix = &value
	} else {
		out.ObjcClassPrefix = nil
	}
	if src.has_csharp_namespace != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.csharp_namespace.data), int(src.csharp_namespace.len))
		out.CsharpNamespace = &value
	} else {
		out.CsharpNamespace = nil
	}
	if src.has_swift_prefix != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.swift_prefix.data), int(src.swift_prefix.len))
		out.SwiftPrefix = &value
	} else {
		out.SwiftPrefix = nil
	}
	if src.has_php_class_prefix != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.php_class_prefix.data), int(src.php_class_prefix.len))
		out.PhpClassPrefix = &value
	} else {
		out.PhpClassPrefix = nil
	}
	if src.has_php_namespace != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.php_namespace.data), int(src.php_namespace.len))
		out.PhpNamespace = &value
	} else {
		out.PhpNamespace = nil
	}
	if src.has_php_generic_services != C.bool(false) {
		value := bool(src.php_generic_services)
		out.PhpGenericServices = &value
	} else {
		out.PhpGenericServices = nil
	}
	if src.uninterpreted_option.data != nil && src.uninterpreted_option.len > 0 {
		length := int(src.uninterpreted_option.len)
		ptrs := unsafe.Slice((**UninterpretedOption)(unsafe.Pointer(src.uninterpreted_option.data)), length)
		out.UninterpretedOption = make([]*descriptorproto.UninterpretedOption, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.UninterpretedOption = append(out.UninterpretedOption, FromReprUninterpretedOptionGenerated(ptr))
		}
	}
	return out
}

func NewReprGeneratedCodeInfoGenerated(arena *runtime.Arena, src *descriptorproto.GeneratedCodeInfo) *GeneratedCodeInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GeneratedCodeInfo)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_GeneratedCodeInfo)))
	IntoReprGeneratedCodeInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGeneratedCodeInfoGenerated(arena *runtime.Arena, dst *GeneratedCodeInfo, src *descriptorproto.GeneratedCodeInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetAnnotation(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*GeneratedCodeInfo_Annotation)(nil)))
		array := unsafe.Slice((**GeneratedCodeInfo_Annotation)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprGeneratedCodeInfo_AnnotationGenerated(arena, value)
		}
		dst.annotation.data = (**GeneratedCodeInfo_Annotation)(ptr)
		dst.annotation.len = C.size_t(len(values))
		dst.annotation.cap = C.size_t(len(values))
	}
}

func FromReprGeneratedCodeInfoGenerated(src *GeneratedCodeInfo) *descriptorproto.GeneratedCodeInfo {
	if src == nil {
		return nil
	}
	out := &descriptorproto.GeneratedCodeInfo{}
	if src.annotation.data != nil && src.annotation.len > 0 {
		length := int(src.annotation.len)
		ptrs := unsafe.Slice((**GeneratedCodeInfo_Annotation)(unsafe.Pointer(src.annotation.data)), length)
		out.Annotation = make([]*descriptorproto.GeneratedCodeInfo_Annotation, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Annotation = append(out.Annotation, FromReprGeneratedCodeInfo_AnnotationGenerated(ptr))
		}
	}
	return out
}

func NewReprGeneratedCodeInfo_AnnotationGenerated(arena *runtime.Arena, src *descriptorproto.GeneratedCodeInfo_Annotation) *GeneratedCodeInfo_Annotation {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GeneratedCodeInfo_Annotation)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_GeneratedCodeInfo_Annotation)))
	IntoReprGeneratedCodeInfo_AnnotationGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGeneratedCodeInfo_AnnotationGenerated(arena *runtime.Arena, dst *GeneratedCodeInfo_Annotation, src *descriptorproto.GeneratedCodeInfo_Annotation) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetPath(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.int32_t(0)))
		array := unsafe.Slice((*C.int32_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.int32_t(value)
		}
		dst.path.data = (*C.int32_t)(ptr)
		dst.path.len = C.size_t(len(values))
		dst.path.cap = C.size_t(len(values))
	}
	if src.SourceFile != nil {
		dst.has_source_file = C.bool(true)
		if data, length := arena.AllocString(*src.SourceFile); length > 0 {
			dst.source_file.data = (*C.char)(data)
			dst.source_file.len = C.size_t(length)
		} else {
			dst.source_file.data = nil
			dst.source_file.len = 0
		}
	} else {
		dst.has_source_file = C.bool(false)
	}
	if src.Begin != nil {
		dst.has_begin = C.bool(true)
		dst.begin = C.int32_t(*src.Begin)
	} else {
		dst.has_begin = C.bool(false)
	}
	if src.End != nil {
		dst.has_end = C.bool(true)
		dst.end = C.int32_t(*src.End)
	} else {
		dst.has_end = C.bool(false)
	}
}

func FromReprGeneratedCodeInfo_AnnotationGenerated(src *GeneratedCodeInfo_Annotation) *descriptorproto.GeneratedCodeInfo_Annotation {
	if src == nil {
		return nil
	}
	out := &descriptorproto.GeneratedCodeInfo_Annotation{}
	if src.path.data != nil && src.path.len > 0 {
		length := int(src.path.len)
		values := unsafe.Slice((*C.int32_t)(unsafe.Pointer(src.path.data)), length)
		out.Path = make([]int32, 0, length)
		for _, value := range values {
			out.Path = append(out.Path, int32(value))
		}
	}
	if src.has_source_file != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.source_file.data), int(src.source_file.len))
		out.SourceFile = &value
	} else {
		out.SourceFile = nil
	}
	if src.has_begin != C.bool(false) {
		value := int32(src.begin)
		out.Begin = &value
	} else {
		out.Begin = nil
	}
	if src.has_end != C.bool(false) {
		value := int32(src.end)
		out.End = &value
	} else {
		out.End = nil
	}
	return out
}

func NewReprMessageOptionsGenerated(arena *runtime.Arena, src *descriptorproto.MessageOptions) *MessageOptions {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MessageOptions)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_MessageOptions)))
	IntoReprMessageOptionsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMessageOptionsGenerated(arena *runtime.Arena, dst *MessageOptions, src *descriptorproto.MessageOptions) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.MessageSetWireFormat != nil {
		dst.has_message_set_wire_format = C.bool(true)
		dst.message_set_wire_format = C.bool(*src.MessageSetWireFormat)
	} else {
		dst.has_message_set_wire_format = C.bool(false)
	}
	if src.NoStandardDescriptorAccessor != nil {
		dst.has_no_standard_descriptor_accessor = C.bool(true)
		dst.no_standard_descriptor_accessor = C.bool(*src.NoStandardDescriptorAccessor)
	} else {
		dst.has_no_standard_descriptor_accessor = C.bool(false)
	}
	if src.Deprecated != nil {
		dst.has_deprecated = C.bool(true)
		dst.deprecated = C.bool(*src.Deprecated)
	} else {
		dst.has_deprecated = C.bool(false)
	}
	if src.MapEntry != nil {
		dst.has_map_entry = C.bool(true)
		dst.map_entry = C.bool(*src.MapEntry)
	} else {
		dst.has_map_entry = C.bool(false)
	}
	if values := src.GetUninterpretedOption(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*UninterpretedOption)(nil)))
		array := unsafe.Slice((**UninterpretedOption)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprUninterpretedOptionGenerated(arena, value)
		}
		dst.uninterpreted_option.data = (**UninterpretedOption)(ptr)
		dst.uninterpreted_option.len = C.size_t(len(values))
		dst.uninterpreted_option.cap = C.size_t(len(values))
	}
}

func FromReprMessageOptionsGenerated(src *MessageOptions) *descriptorproto.MessageOptions {
	if src == nil {
		return nil
	}
	out := &descriptorproto.MessageOptions{}
	if src.has_message_set_wire_format != C.bool(false) {
		value := bool(src.message_set_wire_format)
		out.MessageSetWireFormat = &value
	} else {
		out.MessageSetWireFormat = nil
	}
	if src.has_no_standard_descriptor_accessor != C.bool(false) {
		value := bool(src.no_standard_descriptor_accessor)
		out.NoStandardDescriptorAccessor = &value
	} else {
		out.NoStandardDescriptorAccessor = nil
	}
	if src.has_deprecated != C.bool(false) {
		value := bool(src.deprecated)
		out.Deprecated = &value
	} else {
		out.Deprecated = nil
	}
	if src.has_map_entry != C.bool(false) {
		value := bool(src.map_entry)
		out.MapEntry = &value
	} else {
		out.MapEntry = nil
	}
	if src.uninterpreted_option.data != nil && src.uninterpreted_option.len > 0 {
		length := int(src.uninterpreted_option.len)
		ptrs := unsafe.Slice((**UninterpretedOption)(unsafe.Pointer(src.uninterpreted_option.data)), length)
		out.UninterpretedOption = make([]*descriptorproto.UninterpretedOption, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.UninterpretedOption = append(out.UninterpretedOption, FromReprUninterpretedOptionGenerated(ptr))
		}
	}
	return out
}

func NewReprMethodDescriptorProtoGenerated(arena *runtime.Arena, src *descriptorproto.MethodDescriptorProto) *MethodDescriptorProto {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MethodDescriptorProto)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_MethodDescriptorProto)))
	IntoReprMethodDescriptorProtoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMethodDescriptorProtoGenerated(arena *runtime.Arena, dst *MethodDescriptorProto, src *descriptorproto.MethodDescriptorProto) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Name != nil {
		dst.has_name = C.bool(true)
		if data, length := arena.AllocString(*src.Name); length > 0 {
			dst.name.data = (*C.char)(data)
			dst.name.len = C.size_t(length)
		} else {
			dst.name.data = nil
			dst.name.len = 0
		}
	} else {
		dst.has_name = C.bool(false)
	}
	if src.InputType != nil {
		dst.has_input_type = C.bool(true)
		if data, length := arena.AllocString(*src.InputType); length > 0 {
			dst.input_type.data = (*C.char)(data)
			dst.input_type.len = C.size_t(length)
		} else {
			dst.input_type.data = nil
			dst.input_type.len = 0
		}
	} else {
		dst.has_input_type = C.bool(false)
	}
	if src.OutputType != nil {
		dst.has_output_type = C.bool(true)
		if data, length := arena.AllocString(*src.OutputType); length > 0 {
			dst.output_type.data = (*C.char)(data)
			dst.output_type.len = C.size_t(length)
		} else {
			dst.output_type.data = nil
			dst.output_type.len = 0
		}
	} else {
		dst.has_output_type = C.bool(false)
	}
	if value := src.GetOptions(); value != nil {
		dst.options = NewReprMethodOptionsGenerated(arena, value)
	} else {
		dst.options = nil
	}
	if src.ClientStreaming != nil {
		dst.has_client_streaming = C.bool(true)
		dst.client_streaming = C.bool(*src.ClientStreaming)
	} else {
		dst.has_client_streaming = C.bool(false)
	}
	if src.ServerStreaming != nil {
		dst.has_server_streaming = C.bool(true)
		dst.server_streaming = C.bool(*src.ServerStreaming)
	} else {
		dst.has_server_streaming = C.bool(false)
	}
}

func FromReprMethodDescriptorProtoGenerated(src *MethodDescriptorProto) *descriptorproto.MethodDescriptorProto {
	if src == nil {
		return nil
	}
	out := &descriptorproto.MethodDescriptorProto{}
	if src.has_name != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.name.data), int(src.name.len))
		out.Name = &value
	} else {
		out.Name = nil
	}
	if src.has_input_type != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.input_type.data), int(src.input_type.len))
		out.InputType = &value
	} else {
		out.InputType = nil
	}
	if src.has_output_type != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.output_type.data), int(src.output_type.len))
		out.OutputType = &value
	} else {
		out.OutputType = nil
	}
	if src.options != nil {
		out.Options = FromReprMethodOptionsGenerated(src.options)
	}
	if src.has_client_streaming != C.bool(false) {
		value := bool(src.client_streaming)
		out.ClientStreaming = &value
	} else {
		out.ClientStreaming = nil
	}
	if src.has_server_streaming != C.bool(false) {
		value := bool(src.server_streaming)
		out.ServerStreaming = &value
	} else {
		out.ServerStreaming = nil
	}
	return out
}

func NewReprMethodOptionsGenerated(arena *runtime.Arena, src *descriptorproto.MethodOptions) *MethodOptions {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MethodOptions)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_MethodOptions)))
	IntoReprMethodOptionsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMethodOptionsGenerated(arena *runtime.Arena, dst *MethodOptions, src *descriptorproto.MethodOptions) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Deprecated != nil {
		dst.has_deprecated = C.bool(true)
		dst.deprecated = C.bool(*src.Deprecated)
	} else {
		dst.has_deprecated = C.bool(false)
	}
	if src.IdempotencyLevel != nil {
		dst.has_idempotency_level = C.bool(true)
		dst.idempotency_level = C.int32_t(int32(*src.IdempotencyLevel))
	} else {
		dst.has_idempotency_level = C.bool(false)
	}
	if values := src.GetUninterpretedOption(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*UninterpretedOption)(nil)))
		array := unsafe.Slice((**UninterpretedOption)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprUninterpretedOptionGenerated(arena, value)
		}
		dst.uninterpreted_option.data = (**UninterpretedOption)(ptr)
		dst.uninterpreted_option.len = C.size_t(len(values))
		dst.uninterpreted_option.cap = C.size_t(len(values))
	}
}

func FromReprMethodOptionsGenerated(src *MethodOptions) *descriptorproto.MethodOptions {
	if src == nil {
		return nil
	}
	out := &descriptorproto.MethodOptions{}
	if src.has_deprecated != C.bool(false) {
		value := bool(src.deprecated)
		out.Deprecated = &value
	} else {
		out.Deprecated = nil
	}
	if src.has_idempotency_level != C.bool(false) {
		value := descriptorproto.MethodOptions_IdempotencyLevel(int32(src.idempotency_level))
		out.IdempotencyLevel = &value
	} else {
		out.IdempotencyLevel = nil
	}
	if src.uninterpreted_option.data != nil && src.uninterpreted_option.len > 0 {
		length := int(src.uninterpreted_option.len)
		ptrs := unsafe.Slice((**UninterpretedOption)(unsafe.Pointer(src.uninterpreted_option.data)), length)
		out.UninterpretedOption = make([]*descriptorproto.UninterpretedOption, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.UninterpretedOption = append(out.UninterpretedOption, FromReprUninterpretedOptionGenerated(ptr))
		}
	}
	return out
}

func NewReprOneofDescriptorProtoGenerated(arena *runtime.Arena, src *descriptorproto.OneofDescriptorProto) *OneofDescriptorProto {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*OneofDescriptorProto)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_OneofDescriptorProto)))
	IntoReprOneofDescriptorProtoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprOneofDescriptorProtoGenerated(arena *runtime.Arena, dst *OneofDescriptorProto, src *descriptorproto.OneofDescriptorProto) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Name != nil {
		dst.has_name = C.bool(true)
		if data, length := arena.AllocString(*src.Name); length > 0 {
			dst.name.data = (*C.char)(data)
			dst.name.len = C.size_t(length)
		} else {
			dst.name.data = nil
			dst.name.len = 0
		}
	} else {
		dst.has_name = C.bool(false)
	}
	if value := src.GetOptions(); value != nil {
		dst.options = NewReprOneofOptionsGenerated(arena, value)
	} else {
		dst.options = nil
	}
}

func FromReprOneofDescriptorProtoGenerated(src *OneofDescriptorProto) *descriptorproto.OneofDescriptorProto {
	if src == nil {
		return nil
	}
	out := &descriptorproto.OneofDescriptorProto{}
	if src.has_name != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.name.data), int(src.name.len))
		out.Name = &value
	} else {
		out.Name = nil
	}
	if src.options != nil {
		out.Options = FromReprOneofOptionsGenerated(src.options)
	}
	return out
}

func NewReprOneofOptionsGenerated(arena *runtime.Arena, src *descriptorproto.OneofOptions) *OneofOptions {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*OneofOptions)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_OneofOptions)))
	IntoReprOneofOptionsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprOneofOptionsGenerated(arena *runtime.Arena, dst *OneofOptions, src *descriptorproto.OneofOptions) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetUninterpretedOption(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*UninterpretedOption)(nil)))
		array := unsafe.Slice((**UninterpretedOption)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprUninterpretedOptionGenerated(arena, value)
		}
		dst.uninterpreted_option.data = (**UninterpretedOption)(ptr)
		dst.uninterpreted_option.len = C.size_t(len(values))
		dst.uninterpreted_option.cap = C.size_t(len(values))
	}
}

func FromReprOneofOptionsGenerated(src *OneofOptions) *descriptorproto.OneofOptions {
	if src == nil {
		return nil
	}
	out := &descriptorproto.OneofOptions{}
	if src.uninterpreted_option.data != nil && src.uninterpreted_option.len > 0 {
		length := int(src.uninterpreted_option.len)
		ptrs := unsafe.Slice((**UninterpretedOption)(unsafe.Pointer(src.uninterpreted_option.data)), length)
		out.UninterpretedOption = make([]*descriptorproto.UninterpretedOption, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.UninterpretedOption = append(out.UninterpretedOption, FromReprUninterpretedOptionGenerated(ptr))
		}
	}
	return out
}

func NewReprServiceDescriptorProtoGenerated(arena *runtime.Arena, src *descriptorproto.ServiceDescriptorProto) *ServiceDescriptorProto {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ServiceDescriptorProto)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_ServiceDescriptorProto)))
	IntoReprServiceDescriptorProtoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprServiceDescriptorProtoGenerated(arena *runtime.Arena, dst *ServiceDescriptorProto, src *descriptorproto.ServiceDescriptorProto) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Name != nil {
		dst.has_name = C.bool(true)
		if data, length := arena.AllocString(*src.Name); length > 0 {
			dst.name.data = (*C.char)(data)
			dst.name.len = C.size_t(length)
		} else {
			dst.name.data = nil
			dst.name.len = 0
		}
	} else {
		dst.has_name = C.bool(false)
	}
	if values := src.GetMethod(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*MethodDescriptorProto)(nil)))
		array := unsafe.Slice((**MethodDescriptorProto)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprMethodDescriptorProtoGenerated(arena, value)
		}
		dst.method.data = (**MethodDescriptorProto)(ptr)
		dst.method.len = C.size_t(len(values))
		dst.method.cap = C.size_t(len(values))
	}
	if value := src.GetOptions(); value != nil {
		dst.options = NewReprServiceOptionsGenerated(arena, value)
	} else {
		dst.options = nil
	}
}

func FromReprServiceDescriptorProtoGenerated(src *ServiceDescriptorProto) *descriptorproto.ServiceDescriptorProto {
	if src == nil {
		return nil
	}
	out := &descriptorproto.ServiceDescriptorProto{}
	if src.has_name != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.name.data), int(src.name.len))
		out.Name = &value
	} else {
		out.Name = nil
	}
	if src.method.data != nil && src.method.len > 0 {
		length := int(src.method.len)
		ptrs := unsafe.Slice((**MethodDescriptorProto)(unsafe.Pointer(src.method.data)), length)
		out.Method = make([]*descriptorproto.MethodDescriptorProto, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Method = append(out.Method, FromReprMethodDescriptorProtoGenerated(ptr))
		}
	}
	if src.options != nil {
		out.Options = FromReprServiceOptionsGenerated(src.options)
	}
	return out
}

func NewReprServiceOptionsGenerated(arena *runtime.Arena, src *descriptorproto.ServiceOptions) *ServiceOptions {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ServiceOptions)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_ServiceOptions)))
	IntoReprServiceOptionsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprServiceOptionsGenerated(arena *runtime.Arena, dst *ServiceOptions, src *descriptorproto.ServiceOptions) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.Deprecated != nil {
		dst.has_deprecated = C.bool(true)
		dst.deprecated = C.bool(*src.Deprecated)
	} else {
		dst.has_deprecated = C.bool(false)
	}
	if values := src.GetUninterpretedOption(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*UninterpretedOption)(nil)))
		array := unsafe.Slice((**UninterpretedOption)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprUninterpretedOptionGenerated(arena, value)
		}
		dst.uninterpreted_option.data = (**UninterpretedOption)(ptr)
		dst.uninterpreted_option.len = C.size_t(len(values))
		dst.uninterpreted_option.cap = C.size_t(len(values))
	}
}

func FromReprServiceOptionsGenerated(src *ServiceOptions) *descriptorproto.ServiceOptions {
	if src == nil {
		return nil
	}
	out := &descriptorproto.ServiceOptions{}
	if src.has_deprecated != C.bool(false) {
		value := bool(src.deprecated)
		out.Deprecated = &value
	} else {
		out.Deprecated = nil
	}
	if src.uninterpreted_option.data != nil && src.uninterpreted_option.len > 0 {
		length := int(src.uninterpreted_option.len)
		ptrs := unsafe.Slice((**UninterpretedOption)(unsafe.Pointer(src.uninterpreted_option.data)), length)
		out.UninterpretedOption = make([]*descriptorproto.UninterpretedOption, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.UninterpretedOption = append(out.UninterpretedOption, FromReprUninterpretedOptionGenerated(ptr))
		}
	}
	return out
}

func NewReprSourceCodeInfoGenerated(arena *runtime.Arena, src *descriptorproto.SourceCodeInfo) *SourceCodeInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*SourceCodeInfo)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_SourceCodeInfo)))
	IntoReprSourceCodeInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprSourceCodeInfoGenerated(arena *runtime.Arena, dst *SourceCodeInfo, src *descriptorproto.SourceCodeInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetLocation(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*SourceCodeInfo_Location)(nil)))
		array := unsafe.Slice((**SourceCodeInfo_Location)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprSourceCodeInfo_LocationGenerated(arena, value)
		}
		dst.location.data = (**SourceCodeInfo_Location)(ptr)
		dst.location.len = C.size_t(len(values))
		dst.location.cap = C.size_t(len(values))
	}
}

func FromReprSourceCodeInfoGenerated(src *SourceCodeInfo) *descriptorproto.SourceCodeInfo {
	if src == nil {
		return nil
	}
	out := &descriptorproto.SourceCodeInfo{}
	if src.location.data != nil && src.location.len > 0 {
		length := int(src.location.len)
		ptrs := unsafe.Slice((**SourceCodeInfo_Location)(unsafe.Pointer(src.location.data)), length)
		out.Location = make([]*descriptorproto.SourceCodeInfo_Location, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Location = append(out.Location, FromReprSourceCodeInfo_LocationGenerated(ptr))
		}
	}
	return out
}

func NewReprSourceCodeInfo_LocationGenerated(arena *runtime.Arena, src *descriptorproto.SourceCodeInfo_Location) *SourceCodeInfo_Location {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*SourceCodeInfo_Location)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_SourceCodeInfo_Location)))
	IntoReprSourceCodeInfo_LocationGenerated(arena, ptr, src)
	return ptr
}

func IntoReprSourceCodeInfo_LocationGenerated(arena *runtime.Arena, dst *SourceCodeInfo_Location, src *descriptorproto.SourceCodeInfo_Location) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetPath(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.int32_t(0)))
		array := unsafe.Slice((*C.int32_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.int32_t(value)
		}
		dst.path.data = (*C.int32_t)(ptr)
		dst.path.len = C.size_t(len(values))
		dst.path.cap = C.size_t(len(values))
	}
	if values := src.GetSpan(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.int32_t(0)))
		array := unsafe.Slice((*C.int32_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.int32_t(value)
		}
		dst.span.data = (*C.int32_t)(ptr)
		dst.span.len = C.size_t(len(values))
		dst.span.cap = C.size_t(len(values))
	}
	if src.LeadingComments != nil {
		dst.has_leading_comments = C.bool(true)
		if data, length := arena.AllocString(*src.LeadingComments); length > 0 {
			dst.leading_comments.data = (*C.char)(data)
			dst.leading_comments.len = C.size_t(length)
		} else {
			dst.leading_comments.data = nil
			dst.leading_comments.len = 0
		}
	} else {
		dst.has_leading_comments = C.bool(false)
	}
	if src.TrailingComments != nil {
		dst.has_trailing_comments = C.bool(true)
		if data, length := arena.AllocString(*src.TrailingComments); length > 0 {
			dst.trailing_comments.data = (*C.char)(data)
			dst.trailing_comments.len = C.size_t(length)
		} else {
			dst.trailing_comments.data = nil
			dst.trailing_comments.len = 0
		}
	} else {
		dst.has_trailing_comments = C.bool(false)
	}
	runtime.SetStringSlice(arena, unsafe.Pointer(&dst.leading_detached_comments), src.GetLeadingDetachedComments())
}

func FromReprSourceCodeInfo_LocationGenerated(src *SourceCodeInfo_Location) *descriptorproto.SourceCodeInfo_Location {
	if src == nil {
		return nil
	}
	out := &descriptorproto.SourceCodeInfo_Location{}
	if src.path.data != nil && src.path.len > 0 {
		length := int(src.path.len)
		values := unsafe.Slice((*C.int32_t)(unsafe.Pointer(src.path.data)), length)
		out.Path = make([]int32, 0, length)
		for _, value := range values {
			out.Path = append(out.Path, int32(value))
		}
	}
	if src.span.data != nil && src.span.len > 0 {
		length := int(src.span.len)
		values := unsafe.Slice((*C.int32_t)(unsafe.Pointer(src.span.data)), length)
		out.Span = make([]int32, 0, length)
		for _, value := range values {
			out.Span = append(out.Span, int32(value))
		}
	}
	if src.has_leading_comments != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.leading_comments.data), int(src.leading_comments.len))
		out.LeadingComments = &value
	} else {
		out.LeadingComments = nil
	}
	if src.has_trailing_comments != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.trailing_comments.data), int(src.trailing_comments.len))
		out.TrailingComments = &value
	} else {
		out.TrailingComments = nil
	}
	out.LeadingDetachedComments = runtime.CopyStringSlice(unsafe.Pointer(&src.leading_detached_comments))
	return out
}

func NewReprUninterpretedOptionGenerated(arena *runtime.Arena, src *descriptorproto.UninterpretedOption) *UninterpretedOption {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*UninterpretedOption)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_UninterpretedOption)))
	IntoReprUninterpretedOptionGenerated(arena, ptr, src)
	return ptr
}

func IntoReprUninterpretedOptionGenerated(arena *runtime.Arena, dst *UninterpretedOption, src *descriptorproto.UninterpretedOption) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetName(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*UninterpretedOption_NamePart)(nil)))
		array := unsafe.Slice((**UninterpretedOption_NamePart)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprUninterpretedOption_NamePartGenerated(arena, value)
		}
		dst.name.data = (**UninterpretedOption_NamePart)(ptr)
		dst.name.len = C.size_t(len(values))
		dst.name.cap = C.size_t(len(values))
	}
	if src.IdentifierValue != nil {
		dst.has_identifier_value = C.bool(true)
		if data, length := arena.AllocString(*src.IdentifierValue); length > 0 {
			dst.identifier_value.data = (*C.char)(data)
			dst.identifier_value.len = C.size_t(length)
		} else {
			dst.identifier_value.data = nil
			dst.identifier_value.len = 0
		}
	} else {
		dst.has_identifier_value = C.bool(false)
	}
	if src.PositiveIntValue != nil {
		dst.has_positive_int_value = C.bool(true)
		dst.positive_int_value = C.uint64_t(*src.PositiveIntValue)
	} else {
		dst.has_positive_int_value = C.bool(false)
	}
	if src.NegativeIntValue != nil {
		dst.has_negative_int_value = C.bool(true)
		dst.negative_int_value = C.int64_t(*src.NegativeIntValue)
	} else {
		dst.has_negative_int_value = C.bool(false)
	}
	if src.DoubleValue != nil {
		dst.has_double_value = C.bool(true)
		dst.double_value = C.double(*src.DoubleValue)
	} else {
		dst.has_double_value = C.bool(false)
	}
	if src.StringValue != nil {
		dst.has_string_value = C.bool(true)
		if data, length := arena.AllocBytes(src.StringValue); length > 0 {
			dst.string_value.data = (*C.uint8_t)(data)
			dst.string_value.len = C.size_t(length)
		} else {
			dst.string_value.data = nil
			dst.string_value.len = 0
		}
	} else {
		dst.has_string_value = C.bool(false)
	}
	if src.AggregateValue != nil {
		dst.has_aggregate_value = C.bool(true)
		if data, length := arena.AllocString(*src.AggregateValue); length > 0 {
			dst.aggregate_value.data = (*C.char)(data)
			dst.aggregate_value.len = C.size_t(length)
		} else {
			dst.aggregate_value.data = nil
			dst.aggregate_value.len = 0
		}
	} else {
		dst.has_aggregate_value = C.bool(false)
	}
}

func FromReprUninterpretedOptionGenerated(src *UninterpretedOption) *descriptorproto.UninterpretedOption {
	if src == nil {
		return nil
	}
	out := &descriptorproto.UninterpretedOption{}
	if src.name.data != nil && src.name.len > 0 {
		length := int(src.name.len)
		ptrs := unsafe.Slice((**UninterpretedOption_NamePart)(unsafe.Pointer(src.name.data)), length)
		out.Name = make([]*descriptorproto.UninterpretedOption_NamePart, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Name = append(out.Name, FromReprUninterpretedOption_NamePartGenerated(ptr))
		}
	}
	if src.has_identifier_value != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.identifier_value.data), int(src.identifier_value.len))
		out.IdentifierValue = &value
	} else {
		out.IdentifierValue = nil
	}
	if src.has_positive_int_value != C.bool(false) {
		value := uint64(src.positive_int_value)
		out.PositiveIntValue = &value
	} else {
		out.PositiveIntValue = nil
	}
	if src.has_negative_int_value != C.bool(false) {
		value := int64(src.negative_int_value)
		out.NegativeIntValue = &value
	} else {
		out.NegativeIntValue = nil
	}
	if src.has_double_value != C.bool(false) {
		value := float64(src.double_value)
		out.DoubleValue = &value
	} else {
		out.DoubleValue = nil
	}
	if src.has_string_value != C.bool(false) {
		value := runtime.BytesFrom(unsafe.Pointer(src.string_value.data), int(src.string_value.len))
		if value == nil {
			value = make([]byte, 0)
		}
		out.StringValue = value
	} else {
		out.StringValue = nil
	}
	if src.has_aggregate_value != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.aggregate_value.data), int(src.aggregate_value.len))
		out.AggregateValue = &value
	} else {
		out.AggregateValue = nil
	}
	return out
}

func NewReprUninterpretedOption_NamePartGenerated(arena *runtime.Arena, src *descriptorproto.UninterpretedOption_NamePart) *UninterpretedOption_NamePart {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*UninterpretedOption_NamePart)(arena.AllocZero(uintptr(C.sizeof_google_protobuf_UninterpretedOption_NamePart)))
	IntoReprUninterpretedOption_NamePartGenerated(arena, ptr, src)
	return ptr
}

func IntoReprUninterpretedOption_NamePartGenerated(arena *runtime.Arena, dst *UninterpretedOption_NamePart, src *descriptorproto.UninterpretedOption_NamePart) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if src.NamePart != nil {
		dst.has_name_part = C.bool(true)
		if data, length := arena.AllocString(*src.NamePart); length > 0 {
			dst.name_part.data = (*C.char)(data)
			dst.name_part.len = C.size_t(length)
		} else {
			dst.name_part.data = nil
			dst.name_part.len = 0
		}
	} else {
		dst.has_name_part = C.bool(false)
	}
	if src.IsExtension != nil {
		dst.has_is_extension = C.bool(true)
		dst.is_extension = C.bool(*src.IsExtension)
	} else {
		dst.has_is_extension = C.bool(false)
	}
}

func FromReprUninterpretedOption_NamePartGenerated(src *UninterpretedOption_NamePart) *descriptorproto.UninterpretedOption_NamePart {
	if src == nil {
		return nil
	}
	out := &descriptorproto.UninterpretedOption_NamePart{}
	if src.has_name_part != C.bool(false) {
		value := runtime.StringFrom(unsafe.Pointer(src.name_part.data), int(src.name_part.len))
		out.NamePart = &value
	} else {
		out.NamePart = nil
	}
	if src.has_is_extension != C.bool(false) {
		value := bool(src.is_extension)
		out.IsExtension = &value
	} else {
		out.IsExtension = nil
	}
	return out
}
