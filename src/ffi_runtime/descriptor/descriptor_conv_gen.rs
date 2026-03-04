//! Auto-generated conversions (feature `kvffi_gen`).
#![cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
#![allow(unused_imports, unused_variables, unused_mut, non_snake_case)]

use std::ptr;
use std::os::raw::c_char;

use protobuf::Message;
use protobuf::ProtobufEnum;
use crate::ffi_runtime::arena::{Arena, bytes_from, string_from};
use crate::ffi_runtime::abi::{GoogleProtobufDescriptorProto, GoogleProtobufDescriptorProtoExtensionRange, GoogleProtobufDescriptorProtoReservedRange, GoogleProtobufEnumDescriptorProto, GoogleProtobufEnumDescriptorProtoEnumReservedRange, GoogleProtobufEnumOptions, GoogleProtobufEnumValueDescriptorProto, GoogleProtobufEnumValueOptions, GoogleProtobufExtensionRangeOptions, GoogleProtobufFieldDescriptorProto, GoogleProtobufFieldOptions, GoogleProtobufFileDescriptorProto, GoogleProtobufFileDescriptorSet, GoogleProtobufFileOptions, GoogleProtobufGeneratedCodeInfo, GoogleProtobufGeneratedCodeInfoAnnotation, GoogleProtobufMessageOptions, GoogleProtobufMethodDescriptorProto, GoogleProtobufMethodOptions, GoogleProtobufOneofDescriptorProto, GoogleProtobufOneofOptions, GoogleProtobufServiceDescriptorProto, GoogleProtobufServiceOptions, GoogleProtobufSourceCodeInfo, GoogleProtobufSourceCodeInfoLocation, GoogleProtobufUninterpretedOption, GoogleProtobufUninterpretedOptionNamePart, KvprotoBytesView, KvprotoSliceGoogleProtobufDescriptorProtoExtensionRangePtr, KvprotoSliceGoogleProtobufDescriptorProtoPtr, KvprotoSliceGoogleProtobufDescriptorProtoReservedRangePtr, KvprotoSliceGoogleProtobufEnumDescriptorProtoEnumReservedRangePtr, KvprotoSliceGoogleProtobufEnumDescriptorProtoPtr, KvprotoSliceGoogleProtobufEnumValueDescriptorProtoPtr, KvprotoSliceGoogleProtobufFieldDescriptorProtoPtr, KvprotoSliceGoogleProtobufFileDescriptorProtoPtr, KvprotoSliceGoogleProtobufGeneratedCodeInfoAnnotationPtr, KvprotoSliceGoogleProtobufMethodDescriptorProtoPtr, KvprotoSliceGoogleProtobufOneofDescriptorProtoPtr, KvprotoSliceGoogleProtobufServiceDescriptorProtoPtr, KvprotoSliceGoogleProtobufSourceCodeInfoLocationPtr, KvprotoSliceGoogleProtobufUninterpretedOptionNamePartPtr, KvprotoSliceGoogleProtobufUninterpretedOptionPtr, KvprotoSliceInt32T, KvprotoSliceKvprotoStringView, KvprotoStringView};
use crate::descriptor as pb;

pub fn descriptor_proto_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::DescriptorProto) -> &'a mut GoogleProtobufDescriptorProto {
    let mut repr = GoogleProtobufDescriptorProto {
        has_name: false,
        name: KvprotoStringView { data: ptr::null(), len: 0 },
        field: KvprotoSliceGoogleProtobufFieldDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        nested_type: KvprotoSliceGoogleProtobufDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        enum_type: KvprotoSliceGoogleProtobufEnumDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        extension_range: KvprotoSliceGoogleProtobufDescriptorProtoExtensionRangePtr { data: ptr::null_mut(), len: 0, cap: 0 },
        extension: KvprotoSliceGoogleProtobufFieldDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        options: ptr::null_mut(),
        oneof_decl: KvprotoSliceGoogleProtobufOneofDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        reserved_range: KvprotoSliceGoogleProtobufDescriptorProtoReservedRangePtr { data: ptr::null_mut(), len: 0, cap: 0 },
        reserved_name: KvprotoSliceKvprotoStringView { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if !src.get_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name());
        repr.name.data = ptr as *const c_char;
        repr.name.len = len;
    }
    {
        let values = src.get_field();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufFieldDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(field_descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.field.data = ptr;
                repr.field.len = len;
                repr.field.cap = len;
            }
        }
    }
    {
        let values = src.get_nested_type();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.nested_type.data = ptr;
                repr.nested_type.len = len;
                repr.nested_type.cap = len;
            }
        }
    }
    {
        let values = src.get_enum_type();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufEnumDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(enum_descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.enum_type.data = ptr;
                repr.enum_type.len = len;
                repr.enum_type.cap = len;
            }
        }
    }
    {
        let values = src.get_extension_range();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufDescriptorProtoExtensionRange> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(descriptor_proto__extension_range_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.extension_range.data = ptr;
                repr.extension_range.len = len;
                repr.extension_range.cap = len;
            }
        }
    }
    {
        let values = src.get_extension();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufFieldDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(field_descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.extension.data = ptr;
                repr.extension.len = len;
                repr.extension.cap = len;
            }
        }
    }
    if src.has_options() {
        repr.options = message_options_to_repr_generated(arena, src.get_options()) as *mut _;
    } else {
        repr.options = ptr::null_mut();
    }
    {
        let values = src.get_oneof_decl();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufOneofDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(oneof_descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.oneof_decl.data = ptr;
                repr.oneof_decl.len = len;
                repr.oneof_decl.cap = len;
            }
        }
    }
    {
        let values = src.get_reserved_range();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufDescriptorProtoReservedRange> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(descriptor_proto__reserved_range_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.reserved_range.data = ptr;
                repr.reserved_range.len = len;
                repr.reserved_range.cap = len;
            }
        }
    }
    {
        let values = src.get_reserved_name();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_string(value);
                views.push(KvprotoStringView { data: ptr as *const c_char, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.reserved_name.data = ptr;
                repr.reserved_name.len = len;
                repr.reserved_name.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn descriptor_proto_from_repr_generated(src: *const GoogleProtobufDescriptorProto) -> Option<google_protobuf::DescriptorProto> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::DescriptorProto::new();
    out.set_name(string_from(repr.name.data as *const u8, repr.name.len));
    if !repr.field.data.is_null() && repr.field.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.field.data, repr.field.len) };
        let mut values: Vec<google_protobuf::FieldDescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = field_descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_field(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.nested_type.data.is_null() && repr.nested_type.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.nested_type.data, repr.nested_type.len) };
        let mut values: Vec<google_protobuf::DescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_nested_type(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.enum_type.data.is_null() && repr.enum_type.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.enum_type.data, repr.enum_type.len) };
        let mut values: Vec<google_protobuf::EnumDescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = enum_descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_enum_type(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.extension_range.data.is_null() && repr.extension_range.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.extension_range.data, repr.extension_range.len) };
        let mut values: Vec<google_protobuf::DescriptorProtoExtensionRange> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = descriptor_proto__extension_range_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_extension_range(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.extension.data.is_null() && repr.extension.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.extension.data, repr.extension.len) };
        let mut values: Vec<google_protobuf::FieldDescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = field_descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_extension(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.options.is_null() {
        if let Some(value) = message_options_from_repr_generated(repr.options) {
            out.set_options(value);
        }
    }
    if !repr.oneof_decl.data.is_null() && repr.oneof_decl.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.oneof_decl.data, repr.oneof_decl.len) };
        let mut values: Vec<google_protobuf::OneofDescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = oneof_descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_oneof_decl(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.reserved_range.data.is_null() && repr.reserved_range.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.reserved_range.data, repr.reserved_range.len) };
        let mut values: Vec<google_protobuf::DescriptorProtoReservedRange> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = descriptor_proto__reserved_range_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_reserved_range(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.reserved_name.data.is_null() && repr.reserved_name.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.reserved_name.data, repr.reserved_name.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(string_from(view.data as *const u8, view.len));
        }
        out.set_reserved_name(::protobuf::RepeatedField::from_vec(values));
    }
    Some(out)
}

pub fn descriptor_proto__extension_range_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::DescriptorProtoExtensionRange) -> &'a mut GoogleProtobufDescriptorProtoExtensionRange {
    let mut repr = GoogleProtobufDescriptorProtoExtensionRange {
        has_start: false,
        start: Default::default(),
        has_end: false,
        end: Default::default(),
        options: ptr::null_mut(),
    };
    repr.start = src.get_start();
    repr.end = src.get_end();
    if src.has_options() {
        repr.options = extension_range_options_to_repr_generated(arena, src.get_options()) as *mut _;
    } else {
        repr.options = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn descriptor_proto__extension_range_from_repr_generated(src: *const GoogleProtobufDescriptorProtoExtensionRange) -> Option<google_protobuf::DescriptorProtoExtensionRange> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::DescriptorProtoExtensionRange::new();
    out.set_start(repr.start);
    out.set_end(repr.end);
    if !repr.options.is_null() {
        if let Some(value) = extension_range_options_from_repr_generated(repr.options) {
            out.set_options(value);
        }
    }
    Some(out)
}

pub fn descriptor_proto__reserved_range_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::DescriptorProtoReservedRange) -> &'a mut GoogleProtobufDescriptorProtoReservedRange {
    let mut repr = GoogleProtobufDescriptorProtoReservedRange {
        has_start: false,
        start: Default::default(),
        has_end: false,
        end: Default::default(),
    };
    repr.start = src.get_start();
    repr.end = src.get_end();
    arena.alloc_struct(repr)
}

pub fn descriptor_proto__reserved_range_from_repr_generated(src: *const GoogleProtobufDescriptorProtoReservedRange) -> Option<google_protobuf::DescriptorProtoReservedRange> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::DescriptorProtoReservedRange::new();
    out.set_start(repr.start);
    out.set_end(repr.end);
    Some(out)
}

pub fn enum_descriptor_proto_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::EnumDescriptorProto) -> &'a mut GoogleProtobufEnumDescriptorProto {
    let mut repr = GoogleProtobufEnumDescriptorProto {
        has_name: false,
        name: KvprotoStringView { data: ptr::null(), len: 0 },
        value: KvprotoSliceGoogleProtobufEnumValueDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        options: ptr::null_mut(),
        reserved_range: KvprotoSliceGoogleProtobufEnumDescriptorProtoEnumReservedRangePtr { data: ptr::null_mut(), len: 0, cap: 0 },
        reserved_name: KvprotoSliceKvprotoStringView { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if !src.get_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name());
        repr.name.data = ptr as *const c_char;
        repr.name.len = len;
    }
    {
        let values = src.get_value();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufEnumValueDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(enum_value_descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.value.data = ptr;
                repr.value.len = len;
                repr.value.cap = len;
            }
        }
    }
    if src.has_options() {
        repr.options = enum_options_to_repr_generated(arena, src.get_options()) as *mut _;
    } else {
        repr.options = ptr::null_mut();
    }
    {
        let values = src.get_reserved_range();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufEnumDescriptorProtoEnumReservedRange> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(enum_descriptor_proto__enum_reserved_range_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.reserved_range.data = ptr;
                repr.reserved_range.len = len;
                repr.reserved_range.cap = len;
            }
        }
    }
    {
        let values = src.get_reserved_name();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_string(value);
                views.push(KvprotoStringView { data: ptr as *const c_char, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.reserved_name.data = ptr;
                repr.reserved_name.len = len;
                repr.reserved_name.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn enum_descriptor_proto_from_repr_generated(src: *const GoogleProtobufEnumDescriptorProto) -> Option<google_protobuf::EnumDescriptorProto> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::EnumDescriptorProto::new();
    out.set_name(string_from(repr.name.data as *const u8, repr.name.len));
    if !repr.value.data.is_null() && repr.value.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.value.data, repr.value.len) };
        let mut values: Vec<google_protobuf::EnumValueDescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = enum_value_descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_value(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.options.is_null() {
        if let Some(value) = enum_options_from_repr_generated(repr.options) {
            out.set_options(value);
        }
    }
    if !repr.reserved_range.data.is_null() && repr.reserved_range.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.reserved_range.data, repr.reserved_range.len) };
        let mut values: Vec<google_protobuf::EnumDescriptorProtoEnumReservedRange> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = enum_descriptor_proto__enum_reserved_range_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_reserved_range(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.reserved_name.data.is_null() && repr.reserved_name.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.reserved_name.data, repr.reserved_name.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(string_from(view.data as *const u8, view.len));
        }
        out.set_reserved_name(::protobuf::RepeatedField::from_vec(values));
    }
    Some(out)
}

pub fn enum_descriptor_proto__enum_reserved_range_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::EnumDescriptorProtoEnumReservedRange) -> &'a mut GoogleProtobufEnumDescriptorProtoEnumReservedRange {
    let mut repr = GoogleProtobufEnumDescriptorProtoEnumReservedRange {
        has_start: false,
        start: Default::default(),
        has_end: false,
        end: Default::default(),
    };
    repr.start = src.get_start();
    repr.end = src.get_end();
    arena.alloc_struct(repr)
}

pub fn enum_descriptor_proto__enum_reserved_range_from_repr_generated(src: *const GoogleProtobufEnumDescriptorProtoEnumReservedRange) -> Option<google_protobuf::EnumDescriptorProtoEnumReservedRange> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::EnumDescriptorProtoEnumReservedRange::new();
    out.set_start(repr.start);
    out.set_end(repr.end);
    Some(out)
}

pub fn enum_options_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::EnumOptions) -> &'a mut GoogleProtobufEnumOptions {
    let mut repr = GoogleProtobufEnumOptions {
        has_allow_alias: false,
        allow_alias: Default::default(),
        has_deprecated: false,
        deprecated: Default::default(),
        uninterpreted_option: KvprotoSliceGoogleProtobufUninterpretedOptionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    repr.allow_alias = src.get_allow_alias();
    repr.deprecated = src.get_deprecated();
    {
        let values = src.get_uninterpreted_option();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufUninterpretedOption> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(uninterpreted_option_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.uninterpreted_option.data = ptr;
                repr.uninterpreted_option.len = len;
                repr.uninterpreted_option.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn enum_options_from_repr_generated(src: *const GoogleProtobufEnumOptions) -> Option<google_protobuf::EnumOptions> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::EnumOptions::new();
    out.set_allow_alias(repr.allow_alias);
    out.set_deprecated(repr.deprecated);
    if !repr.uninterpreted_option.data.is_null() && repr.uninterpreted_option.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.uninterpreted_option.data, repr.uninterpreted_option.len) };
        let mut values: Vec<google_protobuf::UninterpretedOption> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = uninterpreted_option_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_uninterpreted_option(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn enum_value_descriptor_proto_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::EnumValueDescriptorProto) -> &'a mut GoogleProtobufEnumValueDescriptorProto {
    let mut repr = GoogleProtobufEnumValueDescriptorProto {
        has_name: false,
        name: KvprotoStringView { data: ptr::null(), len: 0 },
        has_number: false,
        number: Default::default(),
        options: ptr::null_mut(),
    };
    if !src.get_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name());
        repr.name.data = ptr as *const c_char;
        repr.name.len = len;
    }
    repr.number = src.get_number();
    if src.has_options() {
        repr.options = enum_value_options_to_repr_generated(arena, src.get_options()) as *mut _;
    } else {
        repr.options = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn enum_value_descriptor_proto_from_repr_generated(src: *const GoogleProtobufEnumValueDescriptorProto) -> Option<google_protobuf::EnumValueDescriptorProto> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::EnumValueDescriptorProto::new();
    out.set_name(string_from(repr.name.data as *const u8, repr.name.len));
    out.set_number(repr.number);
    if !repr.options.is_null() {
        if let Some(value) = enum_value_options_from_repr_generated(repr.options) {
            out.set_options(value);
        }
    }
    Some(out)
}

pub fn enum_value_options_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::EnumValueOptions) -> &'a mut GoogleProtobufEnumValueOptions {
    let mut repr = GoogleProtobufEnumValueOptions {
        has_deprecated: false,
        deprecated: Default::default(),
        uninterpreted_option: KvprotoSliceGoogleProtobufUninterpretedOptionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    repr.deprecated = src.get_deprecated();
    {
        let values = src.get_uninterpreted_option();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufUninterpretedOption> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(uninterpreted_option_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.uninterpreted_option.data = ptr;
                repr.uninterpreted_option.len = len;
                repr.uninterpreted_option.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn enum_value_options_from_repr_generated(src: *const GoogleProtobufEnumValueOptions) -> Option<google_protobuf::EnumValueOptions> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::EnumValueOptions::new();
    out.set_deprecated(repr.deprecated);
    if !repr.uninterpreted_option.data.is_null() && repr.uninterpreted_option.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.uninterpreted_option.data, repr.uninterpreted_option.len) };
        let mut values: Vec<google_protobuf::UninterpretedOption> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = uninterpreted_option_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_uninterpreted_option(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn extension_range_options_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::ExtensionRangeOptions) -> &'a mut GoogleProtobufExtensionRangeOptions {
    let mut repr = GoogleProtobufExtensionRangeOptions {
        uninterpreted_option: KvprotoSliceGoogleProtobufUninterpretedOptionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_uninterpreted_option();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufUninterpretedOption> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(uninterpreted_option_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.uninterpreted_option.data = ptr;
                repr.uninterpreted_option.len = len;
                repr.uninterpreted_option.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn extension_range_options_from_repr_generated(src: *const GoogleProtobufExtensionRangeOptions) -> Option<google_protobuf::ExtensionRangeOptions> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::ExtensionRangeOptions::new();
    if !repr.uninterpreted_option.data.is_null() && repr.uninterpreted_option.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.uninterpreted_option.data, repr.uninterpreted_option.len) };
        let mut values: Vec<google_protobuf::UninterpretedOption> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = uninterpreted_option_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_uninterpreted_option(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn field_descriptor_proto_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::FieldDescriptorProto) -> &'a mut GoogleProtobufFieldDescriptorProto {
    let mut repr = GoogleProtobufFieldDescriptorProto {
        has_name: false,
        name: KvprotoStringView { data: ptr::null(), len: 0 },
        has_extendee: false,
        extendee: KvprotoStringView { data: ptr::null(), len: 0 },
        has_number: false,
        number: Default::default(),
        has_label: false,
        label: Default::default(),
        has_type_field: false,
        type_field: Default::default(),
        has_type_name: false,
        type_name: KvprotoStringView { data: ptr::null(), len: 0 },
        has_default_value: false,
        default_value: KvprotoStringView { data: ptr::null(), len: 0 },
        options: ptr::null_mut(),
        has_oneof_index: false,
        oneof_index: Default::default(),
        has_json_name: false,
        json_name: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name());
        repr.name.data = ptr as *const c_char;
        repr.name.len = len;
    }
    if !src.get_extendee().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_extendee());
        repr.extendee.data = ptr as *const c_char;
        repr.extendee.len = len;
    }
    repr.number = src.get_number();
    repr.label = src.get_label() as i32;
    repr.type_field = src.get_type() as i32;
    if !src.get_type_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_type_name());
        repr.type_name.data = ptr as *const c_char;
        repr.type_name.len = len;
    }
    if !src.get_default_value().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_default_value());
        repr.default_value.data = ptr as *const c_char;
        repr.default_value.len = len;
    }
    if src.has_options() {
        repr.options = field_options_to_repr_generated(arena, src.get_options()) as *mut _;
    } else {
        repr.options = ptr::null_mut();
    }
    repr.oneof_index = src.get_oneof_index();
    if !src.get_json_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_json_name());
        repr.json_name.data = ptr as *const c_char;
        repr.json_name.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn field_descriptor_proto_from_repr_generated(src: *const GoogleProtobufFieldDescriptorProto) -> Option<google_protobuf::FieldDescriptorProto> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::FieldDescriptorProto::new();
    out.set_name(string_from(repr.name.data as *const u8, repr.name.len));
    out.set_extendee(string_from(repr.extendee.data as *const u8, repr.extendee.len));
    out.set_number(repr.number);
    out.set_label(google::ProtobufFieldDescriptorProtoLabel::from_i32(repr.label).unwrap_or_default());
    out.set_type(google::ProtobufFieldDescriptorProtoType::from_i32(repr.type_field).unwrap_or_default());
    out.set_type_name(string_from(repr.type_name.data as *const u8, repr.type_name.len));
    out.set_default_value(string_from(repr.default_value.data as *const u8, repr.default_value.len));
    if !repr.options.is_null() {
        if let Some(value) = field_options_from_repr_generated(repr.options) {
            out.set_options(value);
        }
    }
    out.set_oneof_index(repr.oneof_index);
    out.set_json_name(string_from(repr.json_name.data as *const u8, repr.json_name.len));
    Some(out)
}

pub fn field_options_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::FieldOptions) -> &'a mut GoogleProtobufFieldOptions {
    let mut repr = GoogleProtobufFieldOptions {
        has_ctype: false,
        ctype: Default::default(),
        has_packed: false,
        packed: Default::default(),
        has_deprecated: false,
        deprecated: Default::default(),
        has_lazy: false,
        lazy: Default::default(),
        has_jstype: false,
        jstype: Default::default(),
        has_weak: false,
        weak: Default::default(),
        uninterpreted_option: KvprotoSliceGoogleProtobufUninterpretedOptionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    repr.ctype = src.get_ctype() as i32;
    repr.packed = src.get_packed();
    repr.deprecated = src.get_deprecated();
    repr.lazy = src.get_lazy();
    repr.jstype = src.get_jstype() as i32;
    repr.weak = src.get_weak();
    {
        let values = src.get_uninterpreted_option();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufUninterpretedOption> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(uninterpreted_option_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.uninterpreted_option.data = ptr;
                repr.uninterpreted_option.len = len;
                repr.uninterpreted_option.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn field_options_from_repr_generated(src: *const GoogleProtobufFieldOptions) -> Option<google_protobuf::FieldOptions> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::FieldOptions::new();
    out.set_ctype(google::ProtobufFieldOptionsCType::from_i32(repr.ctype).unwrap_or_default());
    out.set_packed(repr.packed);
    out.set_deprecated(repr.deprecated);
    out.set_lazy(repr.lazy);
    out.set_jstype(google::ProtobufFieldOptionsJsType::from_i32(repr.jstype).unwrap_or_default());
    out.set_weak(repr.weak);
    if !repr.uninterpreted_option.data.is_null() && repr.uninterpreted_option.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.uninterpreted_option.data, repr.uninterpreted_option.len) };
        let mut values: Vec<google_protobuf::UninterpretedOption> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = uninterpreted_option_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_uninterpreted_option(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn file_descriptor_proto_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::FileDescriptorProto) -> &'a mut GoogleProtobufFileDescriptorProto {
    let mut repr = GoogleProtobufFileDescriptorProto {
        has_name: false,
        name: KvprotoStringView { data: ptr::null(), len: 0 },
        has_package_field: false,
        package_field: KvprotoStringView { data: ptr::null(), len: 0 },
        dependency: KvprotoSliceKvprotoStringView { data: ptr::null_mut(), len: 0, cap: 0 },
        message_type: KvprotoSliceGoogleProtobufDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        enum_type: KvprotoSliceGoogleProtobufEnumDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        service: KvprotoSliceGoogleProtobufServiceDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        extension: KvprotoSliceGoogleProtobufFieldDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        options: ptr::null_mut(),
        source_code_info: ptr::null_mut(),
        public_dependency: KvprotoSliceInt32T { data: ptr::null_mut(), len: 0, cap: 0 },
        weak_dependency: KvprotoSliceInt32T { data: ptr::null_mut(), len: 0, cap: 0 },
        has_syntax: false,
        syntax: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name());
        repr.name.data = ptr as *const c_char;
        repr.name.len = len;
    }
    if !src.get_package().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_package());
        repr.package_field.data = ptr as *const c_char;
        repr.package_field.len = len;
    }
    {
        let values = src.get_dependency();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_string(value);
                views.push(KvprotoStringView { data: ptr as *const c_char, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.dependency.data = ptr;
                repr.dependency.len = len;
                repr.dependency.cap = len;
            }
        }
    }
    {
        let values = src.get_message_type();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.message_type.data = ptr;
                repr.message_type.len = len;
                repr.message_type.cap = len;
            }
        }
    }
    {
        let values = src.get_enum_type();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufEnumDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(enum_descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.enum_type.data = ptr;
                repr.enum_type.len = len;
                repr.enum_type.cap = len;
            }
        }
    }
    {
        let values = src.get_service();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufServiceDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(service_descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.service.data = ptr;
                repr.service.len = len;
                repr.service.cap = len;
            }
        }
    }
    {
        let values = src.get_extension();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufFieldDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(field_descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.extension.data = ptr;
                repr.extension.len = len;
                repr.extension.cap = len;
            }
        }
    }
    if src.has_options() {
        repr.options = file_options_to_repr_generated(arena, src.get_options()) as *mut _;
    } else {
        repr.options = ptr::null_mut();
    }
    if src.has_source_code_info() {
        repr.source_code_info = source_code_info_to_repr_generated(arena, src.get_source_code_info()) as *mut _;
    } else {
        repr.source_code_info = ptr::null_mut();
    }
    {
        let values = src.get_public_dependency();
        if !values.is_empty() {
            let mut vec: Vec<i32> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.public_dependency.data = ptr;
            repr.public_dependency.len = len;
            repr.public_dependency.cap = len;
        }
    }
    {
        let values = src.get_weak_dependency();
        if !values.is_empty() {
            let mut vec: Vec<i32> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.weak_dependency.data = ptr;
            repr.weak_dependency.len = len;
            repr.weak_dependency.cap = len;
        }
    }
    if !src.get_syntax().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_syntax());
        repr.syntax.data = ptr as *const c_char;
        repr.syntax.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn file_descriptor_proto_from_repr_generated(src: *const GoogleProtobufFileDescriptorProto) -> Option<google_protobuf::FileDescriptorProto> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::FileDescriptorProto::new();
    out.set_name(string_from(repr.name.data as *const u8, repr.name.len));
    out.set_package(string_from(repr.package_field.data as *const u8, repr.package_field.len));
    if !repr.dependency.data.is_null() && repr.dependency.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.dependency.data, repr.dependency.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(string_from(view.data as *const u8, view.len));
        }
        out.set_dependency(::protobuf::RepeatedField::from_vec(values));
    }
    if !repr.message_type.data.is_null() && repr.message_type.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.message_type.data, repr.message_type.len) };
        let mut values: Vec<google_protobuf::DescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_message_type(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.enum_type.data.is_null() && repr.enum_type.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.enum_type.data, repr.enum_type.len) };
        let mut values: Vec<google_protobuf::EnumDescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = enum_descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_enum_type(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.service.data.is_null() && repr.service.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.service.data, repr.service.len) };
        let mut values: Vec<google_protobuf::ServiceDescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = service_descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_service(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.extension.data.is_null() && repr.extension.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.extension.data, repr.extension.len) };
        let mut values: Vec<google_protobuf::FieldDescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = field_descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_extension(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.options.is_null() {
        if let Some(value) = file_options_from_repr_generated(repr.options) {
            out.set_options(value);
        }
    }
    if !repr.source_code_info.is_null() {
        if let Some(value) = source_code_info_from_repr_generated(repr.source_code_info) {
            out.set_source_code_info(value);
        }
    }
    if !repr.public_dependency.data.is_null() && repr.public_dependency.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.public_dependency.data, repr.public_dependency.len) };
        let mut values: Vec<i32> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_public_dependency(values);
    }
    if !repr.weak_dependency.data.is_null() && repr.weak_dependency.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.weak_dependency.data, repr.weak_dependency.len) };
        let mut values: Vec<i32> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_weak_dependency(values);
    }
    out.set_syntax(string_from(repr.syntax.data as *const u8, repr.syntax.len));
    Some(out)
}

pub fn file_descriptor_set_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::FileDescriptorSet) -> &'a mut GoogleProtobufFileDescriptorSet {
    let mut repr = GoogleProtobufFileDescriptorSet {
        file: KvprotoSliceGoogleProtobufFileDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_file();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufFileDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(file_descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.file.data = ptr;
                repr.file.len = len;
                repr.file.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn file_descriptor_set_from_repr_generated(src: *const GoogleProtobufFileDescriptorSet) -> Option<google_protobuf::FileDescriptorSet> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::FileDescriptorSet::new();
    if !repr.file.data.is_null() && repr.file.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.file.data, repr.file.len) };
        let mut values: Vec<google_protobuf::FileDescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = file_descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_file(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn file_options_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::FileOptions) -> &'a mut GoogleProtobufFileOptions {
    let mut repr = GoogleProtobufFileOptions {
        has_java_package: false,
        java_package: KvprotoStringView { data: ptr::null(), len: 0 },
        has_java_outer_classname: false,
        java_outer_classname: KvprotoStringView { data: ptr::null(), len: 0 },
        has_optimize_for: false,
        optimize_for: Default::default(),
        has_java_multiple_files: false,
        java_multiple_files: Default::default(),
        has_go_package: false,
        go_package: KvprotoStringView { data: ptr::null(), len: 0 },
        has_cc_generic_services: false,
        cc_generic_services: Default::default(),
        has_java_generic_services: false,
        java_generic_services: Default::default(),
        has_py_generic_services: false,
        py_generic_services: Default::default(),
        has_java_generate_equals_and_hash: false,
        java_generate_equals_and_hash: Default::default(),
        has_deprecated: false,
        deprecated: Default::default(),
        has_java_string_check_utf8: false,
        java_string_check_utf8: Default::default(),
        has_cc_enable_arenas: false,
        cc_enable_arenas: Default::default(),
        has_objc_class_prefix: false,
        objc_class_prefix: KvprotoStringView { data: ptr::null(), len: 0 },
        has_csharp_namespace: false,
        csharp_namespace: KvprotoStringView { data: ptr::null(), len: 0 },
        has_swift_prefix: false,
        swift_prefix: KvprotoStringView { data: ptr::null(), len: 0 },
        has_php_class_prefix: false,
        php_class_prefix: KvprotoStringView { data: ptr::null(), len: 0 },
        has_php_namespace: false,
        php_namespace: KvprotoStringView { data: ptr::null(), len: 0 },
        has_php_generic_services: false,
        php_generic_services: Default::default(),
        uninterpreted_option: KvprotoSliceGoogleProtobufUninterpretedOptionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if !src.get_java_package().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_java_package());
        repr.java_package.data = ptr as *const c_char;
        repr.java_package.len = len;
    }
    if !src.get_java_outer_classname().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_java_outer_classname());
        repr.java_outer_classname.data = ptr as *const c_char;
        repr.java_outer_classname.len = len;
    }
    repr.optimize_for = src.get_optimize_for() as i32;
    repr.java_multiple_files = src.get_java_multiple_files();
    if !src.get_go_package().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_go_package());
        repr.go_package.data = ptr as *const c_char;
        repr.go_package.len = len;
    }
    repr.cc_generic_services = src.get_cc_generic_services();
    repr.java_generic_services = src.get_java_generic_services();
    repr.py_generic_services = src.get_py_generic_services();
    repr.java_generate_equals_and_hash = src.get_java_generate_equals_and_hash();
    repr.deprecated = src.get_deprecated();
    repr.java_string_check_utf8 = src.get_java_string_check_utf8();
    repr.cc_enable_arenas = src.get_cc_enable_arenas();
    if !src.get_objc_class_prefix().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_objc_class_prefix());
        repr.objc_class_prefix.data = ptr as *const c_char;
        repr.objc_class_prefix.len = len;
    }
    if !src.get_csharp_namespace().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_csharp_namespace());
        repr.csharp_namespace.data = ptr as *const c_char;
        repr.csharp_namespace.len = len;
    }
    if !src.get_swift_prefix().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_swift_prefix());
        repr.swift_prefix.data = ptr as *const c_char;
        repr.swift_prefix.len = len;
    }
    if !src.get_php_class_prefix().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_php_class_prefix());
        repr.php_class_prefix.data = ptr as *const c_char;
        repr.php_class_prefix.len = len;
    }
    if !src.get_php_namespace().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_php_namespace());
        repr.php_namespace.data = ptr as *const c_char;
        repr.php_namespace.len = len;
    }
    repr.php_generic_services = src.get_php_generic_services();
    {
        let values = src.get_uninterpreted_option();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufUninterpretedOption> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(uninterpreted_option_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.uninterpreted_option.data = ptr;
                repr.uninterpreted_option.len = len;
                repr.uninterpreted_option.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn file_options_from_repr_generated(src: *const GoogleProtobufFileOptions) -> Option<google_protobuf::FileOptions> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::FileOptions::new();
    out.set_java_package(string_from(repr.java_package.data as *const u8, repr.java_package.len));
    out.set_java_outer_classname(string_from(repr.java_outer_classname.data as *const u8, repr.java_outer_classname.len));
    out.set_optimize_for(google::ProtobufFileOptionsOptimizeMode::from_i32(repr.optimize_for).unwrap_or_default());
    out.set_java_multiple_files(repr.java_multiple_files);
    out.set_go_package(string_from(repr.go_package.data as *const u8, repr.go_package.len));
    out.set_cc_generic_services(repr.cc_generic_services);
    out.set_java_generic_services(repr.java_generic_services);
    out.set_py_generic_services(repr.py_generic_services);
    out.set_java_generate_equals_and_hash(repr.java_generate_equals_and_hash);
    out.set_deprecated(repr.deprecated);
    out.set_java_string_check_utf8(repr.java_string_check_utf8);
    out.set_cc_enable_arenas(repr.cc_enable_arenas);
    out.set_objc_class_prefix(string_from(repr.objc_class_prefix.data as *const u8, repr.objc_class_prefix.len));
    out.set_csharp_namespace(string_from(repr.csharp_namespace.data as *const u8, repr.csharp_namespace.len));
    out.set_swift_prefix(string_from(repr.swift_prefix.data as *const u8, repr.swift_prefix.len));
    out.set_php_class_prefix(string_from(repr.php_class_prefix.data as *const u8, repr.php_class_prefix.len));
    out.set_php_namespace(string_from(repr.php_namespace.data as *const u8, repr.php_namespace.len));
    out.set_php_generic_services(repr.php_generic_services);
    if !repr.uninterpreted_option.data.is_null() && repr.uninterpreted_option.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.uninterpreted_option.data, repr.uninterpreted_option.len) };
        let mut values: Vec<google_protobuf::UninterpretedOption> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = uninterpreted_option_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_uninterpreted_option(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn generated_code_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::GeneratedCodeInfo) -> &'a mut GoogleProtobufGeneratedCodeInfo {
    let mut repr = GoogleProtobufGeneratedCodeInfo {
        annotation: KvprotoSliceGoogleProtobufGeneratedCodeInfoAnnotationPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_annotation();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufGeneratedCodeInfoAnnotation> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(generated_code_info__annotation_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.annotation.data = ptr;
                repr.annotation.len = len;
                repr.annotation.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn generated_code_info_from_repr_generated(src: *const GoogleProtobufGeneratedCodeInfo) -> Option<google_protobuf::GeneratedCodeInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::GeneratedCodeInfo::new();
    if !repr.annotation.data.is_null() && repr.annotation.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.annotation.data, repr.annotation.len) };
        let mut values: Vec<google_protobuf::GeneratedCodeInfoAnnotation> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = generated_code_info__annotation_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_annotation(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn generated_code_info__annotation_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::GeneratedCodeInfoAnnotation) -> &'a mut GoogleProtobufGeneratedCodeInfoAnnotation {
    let mut repr = GoogleProtobufGeneratedCodeInfoAnnotation {
        path: KvprotoSliceInt32T { data: ptr::null_mut(), len: 0, cap: 0 },
        has_source_file: false,
        source_file: KvprotoStringView { data: ptr::null(), len: 0 },
        has_begin: false,
        begin: Default::default(),
        has_end: false,
        end: Default::default(),
    };
    {
        let values = src.get_path();
        if !values.is_empty() {
            let mut vec: Vec<i32> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.path.data = ptr;
            repr.path.len = len;
            repr.path.cap = len;
        }
    }
    if !src.get_source_file().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_source_file());
        repr.source_file.data = ptr as *const c_char;
        repr.source_file.len = len;
    }
    repr.begin = src.get_begin();
    repr.end = src.get_end();
    arena.alloc_struct(repr)
}

pub fn generated_code_info__annotation_from_repr_generated(src: *const GoogleProtobufGeneratedCodeInfoAnnotation) -> Option<google_protobuf::GeneratedCodeInfoAnnotation> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::GeneratedCodeInfoAnnotation::new();
    if !repr.path.data.is_null() && repr.path.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.path.data, repr.path.len) };
        let mut values: Vec<i32> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_path(values);
    }
    out.set_source_file(string_from(repr.source_file.data as *const u8, repr.source_file.len));
    out.set_begin(repr.begin);
    out.set_end(repr.end);
    Some(out)
}

pub fn message_options_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::MessageOptions) -> &'a mut GoogleProtobufMessageOptions {
    let mut repr = GoogleProtobufMessageOptions {
        has_message_set_wire_format: false,
        message_set_wire_format: Default::default(),
        has_no_standard_descriptor_accessor: false,
        no_standard_descriptor_accessor: Default::default(),
        has_deprecated: false,
        deprecated: Default::default(),
        has_map_entry: false,
        map_entry: Default::default(),
        uninterpreted_option: KvprotoSliceGoogleProtobufUninterpretedOptionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    repr.message_set_wire_format = src.get_message_set_wire_format();
    repr.no_standard_descriptor_accessor = src.get_no_standard_descriptor_accessor();
    repr.deprecated = src.get_deprecated();
    repr.map_entry = src.get_map_entry();
    {
        let values = src.get_uninterpreted_option();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufUninterpretedOption> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(uninterpreted_option_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.uninterpreted_option.data = ptr;
                repr.uninterpreted_option.len = len;
                repr.uninterpreted_option.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn message_options_from_repr_generated(src: *const GoogleProtobufMessageOptions) -> Option<google_protobuf::MessageOptions> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::MessageOptions::new();
    out.set_message_set_wire_format(repr.message_set_wire_format);
    out.set_no_standard_descriptor_accessor(repr.no_standard_descriptor_accessor);
    out.set_deprecated(repr.deprecated);
    out.set_map_entry(repr.map_entry);
    if !repr.uninterpreted_option.data.is_null() && repr.uninterpreted_option.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.uninterpreted_option.data, repr.uninterpreted_option.len) };
        let mut values: Vec<google_protobuf::UninterpretedOption> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = uninterpreted_option_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_uninterpreted_option(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn method_descriptor_proto_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::MethodDescriptorProto) -> &'a mut GoogleProtobufMethodDescriptorProto {
    let mut repr = GoogleProtobufMethodDescriptorProto {
        has_name: false,
        name: KvprotoStringView { data: ptr::null(), len: 0 },
        has_input_type: false,
        input_type: KvprotoStringView { data: ptr::null(), len: 0 },
        has_output_type: false,
        output_type: KvprotoStringView { data: ptr::null(), len: 0 },
        options: ptr::null_mut(),
        has_client_streaming: false,
        client_streaming: Default::default(),
        has_server_streaming: false,
        server_streaming: Default::default(),
    };
    if !src.get_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name());
        repr.name.data = ptr as *const c_char;
        repr.name.len = len;
    }
    if !src.get_input_type().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_input_type());
        repr.input_type.data = ptr as *const c_char;
        repr.input_type.len = len;
    }
    if !src.get_output_type().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_output_type());
        repr.output_type.data = ptr as *const c_char;
        repr.output_type.len = len;
    }
    if src.has_options() {
        repr.options = method_options_to_repr_generated(arena, src.get_options()) as *mut _;
    } else {
        repr.options = ptr::null_mut();
    }
    repr.client_streaming = src.get_client_streaming();
    repr.server_streaming = src.get_server_streaming();
    arena.alloc_struct(repr)
}

pub fn method_descriptor_proto_from_repr_generated(src: *const GoogleProtobufMethodDescriptorProto) -> Option<google_protobuf::MethodDescriptorProto> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::MethodDescriptorProto::new();
    out.set_name(string_from(repr.name.data as *const u8, repr.name.len));
    out.set_input_type(string_from(repr.input_type.data as *const u8, repr.input_type.len));
    out.set_output_type(string_from(repr.output_type.data as *const u8, repr.output_type.len));
    if !repr.options.is_null() {
        if let Some(value) = method_options_from_repr_generated(repr.options) {
            out.set_options(value);
        }
    }
    out.set_client_streaming(repr.client_streaming);
    out.set_server_streaming(repr.server_streaming);
    Some(out)
}

pub fn method_options_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::MethodOptions) -> &'a mut GoogleProtobufMethodOptions {
    let mut repr = GoogleProtobufMethodOptions {
        has_deprecated: false,
        deprecated: Default::default(),
        has_idempotency_level: false,
        idempotency_level: Default::default(),
        uninterpreted_option: KvprotoSliceGoogleProtobufUninterpretedOptionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    repr.deprecated = src.get_deprecated();
    repr.idempotency_level = src.get_idempotency_level() as i32;
    {
        let values = src.get_uninterpreted_option();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufUninterpretedOption> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(uninterpreted_option_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.uninterpreted_option.data = ptr;
                repr.uninterpreted_option.len = len;
                repr.uninterpreted_option.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn method_options_from_repr_generated(src: *const GoogleProtobufMethodOptions) -> Option<google_protobuf::MethodOptions> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::MethodOptions::new();
    out.set_deprecated(repr.deprecated);
    out.set_idempotency_level(google::ProtobufMethodOptionsIdempotencyLevel::from_i32(repr.idempotency_level).unwrap_or_default());
    if !repr.uninterpreted_option.data.is_null() && repr.uninterpreted_option.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.uninterpreted_option.data, repr.uninterpreted_option.len) };
        let mut values: Vec<google_protobuf::UninterpretedOption> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = uninterpreted_option_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_uninterpreted_option(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn oneof_descriptor_proto_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::OneofDescriptorProto) -> &'a mut GoogleProtobufOneofDescriptorProto {
    let mut repr = GoogleProtobufOneofDescriptorProto {
        has_name: false,
        name: KvprotoStringView { data: ptr::null(), len: 0 },
        options: ptr::null_mut(),
    };
    if !src.get_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name());
        repr.name.data = ptr as *const c_char;
        repr.name.len = len;
    }
    if src.has_options() {
        repr.options = oneof_options_to_repr_generated(arena, src.get_options()) as *mut _;
    } else {
        repr.options = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn oneof_descriptor_proto_from_repr_generated(src: *const GoogleProtobufOneofDescriptorProto) -> Option<google_protobuf::OneofDescriptorProto> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::OneofDescriptorProto::new();
    out.set_name(string_from(repr.name.data as *const u8, repr.name.len));
    if !repr.options.is_null() {
        if let Some(value) = oneof_options_from_repr_generated(repr.options) {
            out.set_options(value);
        }
    }
    Some(out)
}

pub fn oneof_options_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::OneofOptions) -> &'a mut GoogleProtobufOneofOptions {
    let mut repr = GoogleProtobufOneofOptions {
        uninterpreted_option: KvprotoSliceGoogleProtobufUninterpretedOptionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_uninterpreted_option();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufUninterpretedOption> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(uninterpreted_option_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.uninterpreted_option.data = ptr;
                repr.uninterpreted_option.len = len;
                repr.uninterpreted_option.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn oneof_options_from_repr_generated(src: *const GoogleProtobufOneofOptions) -> Option<google_protobuf::OneofOptions> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::OneofOptions::new();
    if !repr.uninterpreted_option.data.is_null() && repr.uninterpreted_option.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.uninterpreted_option.data, repr.uninterpreted_option.len) };
        let mut values: Vec<google_protobuf::UninterpretedOption> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = uninterpreted_option_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_uninterpreted_option(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn service_descriptor_proto_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::ServiceDescriptorProto) -> &'a mut GoogleProtobufServiceDescriptorProto {
    let mut repr = GoogleProtobufServiceDescriptorProto {
        has_name: false,
        name: KvprotoStringView { data: ptr::null(), len: 0 },
        method: KvprotoSliceGoogleProtobufMethodDescriptorProtoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        options: ptr::null_mut(),
    };
    if !src.get_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name());
        repr.name.data = ptr as *const c_char;
        repr.name.len = len;
    }
    {
        let values = src.get_method();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufMethodDescriptorProto> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(method_descriptor_proto_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.method.data = ptr;
                repr.method.len = len;
                repr.method.cap = len;
            }
        }
    }
    if src.has_options() {
        repr.options = service_options_to_repr_generated(arena, src.get_options()) as *mut _;
    } else {
        repr.options = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn service_descriptor_proto_from_repr_generated(src: *const GoogleProtobufServiceDescriptorProto) -> Option<google_protobuf::ServiceDescriptorProto> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::ServiceDescriptorProto::new();
    out.set_name(string_from(repr.name.data as *const u8, repr.name.len));
    if !repr.method.data.is_null() && repr.method.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.method.data, repr.method.len) };
        let mut values: Vec<google_protobuf::MethodDescriptorProto> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = method_descriptor_proto_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_method(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.options.is_null() {
        if let Some(value) = service_options_from_repr_generated(repr.options) {
            out.set_options(value);
        }
    }
    Some(out)
}

pub fn service_options_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::ServiceOptions) -> &'a mut GoogleProtobufServiceOptions {
    let mut repr = GoogleProtobufServiceOptions {
        has_deprecated: false,
        deprecated: Default::default(),
        uninterpreted_option: KvprotoSliceGoogleProtobufUninterpretedOptionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    repr.deprecated = src.get_deprecated();
    {
        let values = src.get_uninterpreted_option();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufUninterpretedOption> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(uninterpreted_option_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.uninterpreted_option.data = ptr;
                repr.uninterpreted_option.len = len;
                repr.uninterpreted_option.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn service_options_from_repr_generated(src: *const GoogleProtobufServiceOptions) -> Option<google_protobuf::ServiceOptions> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::ServiceOptions::new();
    out.set_deprecated(repr.deprecated);
    if !repr.uninterpreted_option.data.is_null() && repr.uninterpreted_option.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.uninterpreted_option.data, repr.uninterpreted_option.len) };
        let mut values: Vec<google_protobuf::UninterpretedOption> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = uninterpreted_option_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_uninterpreted_option(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn source_code_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::SourceCodeInfo) -> &'a mut GoogleProtobufSourceCodeInfo {
    let mut repr = GoogleProtobufSourceCodeInfo {
        location: KvprotoSliceGoogleProtobufSourceCodeInfoLocationPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_location();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufSourceCodeInfoLocation> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(source_code_info__location_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.location.data = ptr;
                repr.location.len = len;
                repr.location.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn source_code_info_from_repr_generated(src: *const GoogleProtobufSourceCodeInfo) -> Option<google_protobuf::SourceCodeInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::SourceCodeInfo::new();
    if !repr.location.data.is_null() && repr.location.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.location.data, repr.location.len) };
        let mut values: Vec<google_protobuf::SourceCodeInfoLocation> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = source_code_info__location_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_location(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn source_code_info__location_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::SourceCodeInfoLocation) -> &'a mut GoogleProtobufSourceCodeInfoLocation {
    let mut repr = GoogleProtobufSourceCodeInfoLocation {
        path: KvprotoSliceInt32T { data: ptr::null_mut(), len: 0, cap: 0 },
        span: KvprotoSliceInt32T { data: ptr::null_mut(), len: 0, cap: 0 },
        has_leading_comments: false,
        leading_comments: KvprotoStringView { data: ptr::null(), len: 0 },
        has_trailing_comments: false,
        trailing_comments: KvprotoStringView { data: ptr::null(), len: 0 },
        leading_detached_comments: KvprotoSliceKvprotoStringView { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_path();
        if !values.is_empty() {
            let mut vec: Vec<i32> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.path.data = ptr;
            repr.path.len = len;
            repr.path.cap = len;
        }
    }
    {
        let values = src.get_span();
        if !values.is_empty() {
            let mut vec: Vec<i32> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.span.data = ptr;
            repr.span.len = len;
            repr.span.cap = len;
        }
    }
    if !src.get_leading_comments().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_leading_comments());
        repr.leading_comments.data = ptr as *const c_char;
        repr.leading_comments.len = len;
    }
    if !src.get_trailing_comments().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_trailing_comments());
        repr.trailing_comments.data = ptr as *const c_char;
        repr.trailing_comments.len = len;
    }
    {
        let values = src.get_leading_detached_comments();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_string(value);
                views.push(KvprotoStringView { data: ptr as *const c_char, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.leading_detached_comments.data = ptr;
                repr.leading_detached_comments.len = len;
                repr.leading_detached_comments.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn source_code_info__location_from_repr_generated(src: *const GoogleProtobufSourceCodeInfoLocation) -> Option<google_protobuf::SourceCodeInfoLocation> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::SourceCodeInfoLocation::new();
    if !repr.path.data.is_null() && repr.path.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.path.data, repr.path.len) };
        let mut values: Vec<i32> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_path(values);
    }
    if !repr.span.data.is_null() && repr.span.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.span.data, repr.span.len) };
        let mut values: Vec<i32> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_span(values);
    }
    out.set_leading_comments(string_from(repr.leading_comments.data as *const u8, repr.leading_comments.len));
    out.set_trailing_comments(string_from(repr.trailing_comments.data as *const u8, repr.trailing_comments.len));
    if !repr.leading_detached_comments.data.is_null() && repr.leading_detached_comments.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.leading_detached_comments.data, repr.leading_detached_comments.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(string_from(view.data as *const u8, view.len));
        }
        out.set_leading_detached_comments(::protobuf::RepeatedField::from_vec(values));
    }
    Some(out)
}

pub fn uninterpreted_option_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::UninterpretedOption) -> &'a mut GoogleProtobufUninterpretedOption {
    let mut repr = GoogleProtobufUninterpretedOption {
        name: KvprotoSliceGoogleProtobufUninterpretedOptionNamePartPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        has_identifier_value: false,
        identifier_value: KvprotoStringView { data: ptr::null(), len: 0 },
        has_positive_int_value: false,
        positive_int_value: Default::default(),
        has_negative_int_value: false,
        negative_int_value: Default::default(),
        has_double_value: false,
        double_value: Default::default(),
        has_string_value: false,
        string_value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        has_aggregate_value: false,
        aggregate_value: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    {
        let values = src.get_name();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut GoogleProtobufUninterpretedOptionNamePart> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(uninterpreted_option__name_part_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.name.data = ptr;
                repr.name.len = len;
                repr.name.cap = len;
            }
        }
    }
    if !src.get_identifier_value().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_identifier_value());
        repr.identifier_value.data = ptr as *const c_char;
        repr.identifier_value.len = len;
    }
    repr.positive_int_value = src.get_positive_int_value();
    repr.negative_int_value = src.get_negative_int_value();
    repr.double_value = src.get_double_value();
    if !src.get_string_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_string_value());
        repr.string_value.data = ptr;
        repr.string_value.len = len;
    }
    if !src.get_aggregate_value().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_aggregate_value());
        repr.aggregate_value.data = ptr as *const c_char;
        repr.aggregate_value.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn uninterpreted_option_from_repr_generated(src: *const GoogleProtobufUninterpretedOption) -> Option<google_protobuf::UninterpretedOption> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::UninterpretedOption::new();
    if !repr.name.data.is_null() && repr.name.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.name.data, repr.name.len) };
        let mut values: Vec<google_protobuf::UninterpretedOptionNamePart> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = uninterpreted_option__name_part_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_name(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_identifier_value(string_from(repr.identifier_value.data as *const u8, repr.identifier_value.len));
    out.set_positive_int_value(repr.positive_int_value);
    out.set_negative_int_value(repr.negative_int_value);
    out.set_double_value(repr.double_value);
    out.set_string_value(bytes_from(repr.string_value.data, repr.string_value.len).into());
    out.set_aggregate_value(string_from(repr.aggregate_value.data as *const u8, repr.aggregate_value.len));
    Some(out)
}

pub fn uninterpreted_option__name_part_to_repr_generated<'a>(arena: &'a mut Arena, src: &google_protobuf::UninterpretedOptionNamePart) -> &'a mut GoogleProtobufUninterpretedOptionNamePart {
    let mut repr = GoogleProtobufUninterpretedOptionNamePart {
        has_name_part: false,
        name_part: KvprotoStringView { data: ptr::null(), len: 0 },
        has_is_extension: false,
        is_extension: Default::default(),
    };
    if !src.get_name_part().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name_part());
        repr.name_part.data = ptr as *const c_char;
        repr.name_part.len = len;
    }
    repr.is_extension = src.get_is_extension();
    arena.alloc_struct(repr)
}

pub fn uninterpreted_option__name_part_from_repr_generated(src: *const GoogleProtobufUninterpretedOptionNamePart) -> Option<google_protobuf::UninterpretedOptionNamePart> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = google_protobuf::UninterpretedOptionNamePart::new();
    out.set_name_part(string_from(repr.name_part.data as *const u8, repr.name_part.len));
    out.set_is_extension(repr.is_extension);
    Some(out)
}

