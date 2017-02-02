// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(PartialEq,Clone,Default)]
pub struct MetaItem {
    // message fields
    start_ts: ::std::option::Option<u64>,
    commit_ts: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MetaItem {}

impl MetaItem {
    pub fn new() -> MetaItem {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MetaItem {
        static mut instance: ::protobuf::lazy::Lazy<MetaItem> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MetaItem,
        };
        unsafe {
            instance.get(MetaItem::new)
        }
    }

    // optional uint64 start_ts = 1;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = ::std::option::Option::None;
    }

    pub fn has_start_ts(&self) -> bool {
        self.start_ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = ::std::option::Option::Some(v);
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts.unwrap_or(0)
    }

    fn get_start_ts_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_ts
    }

    fn mut_start_ts_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_ts
    }

    // optional uint64 commit_ts = 2;

    pub fn clear_commit_ts(&mut self) {
        self.commit_ts = ::std::option::Option::None;
    }

    pub fn has_commit_ts(&self) -> bool {
        self.commit_ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_ts(&mut self, v: u64) {
        self.commit_ts = ::std::option::Option::Some(v);
    }

    pub fn get_commit_ts(&self) -> u64 {
        self.commit_ts.unwrap_or(0)
    }

    fn get_commit_ts_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.commit_ts
    }

    fn mut_commit_ts_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.commit_ts
    }
}

impl ::protobuf::Message for MetaItem {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.start_ts = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.commit_ts = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.start_ts {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.commit_ts {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_ts {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.commit_ts {
            os.write_uint64(2, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MetaItem {
    fn new() -> MetaItem {
        MetaItem::new()
    }

    fn descriptor_static(_: ::std::option::Option<MetaItem>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    MetaItem::get_start_ts_for_reflect,
                    MetaItem::mut_start_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_ts",
                    MetaItem::get_commit_ts_for_reflect,
                    MetaItem::mut_commit_ts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MetaItem>(
                    "MetaItem",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MetaItem {
    fn clear(&mut self) {
        self.clear_start_ts();
        self.clear_commit_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetaItem {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetaItem {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MetaLock {
    // message fields
    field_type: ::std::option::Option<MetaLockType>,
    start_ts: ::std::option::Option<u64>,
    primary_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MetaLock {}

impl MetaLock {
    pub fn new() -> MetaLock {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MetaLock {
        static mut instance: ::protobuf::lazy::Lazy<MetaLock> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MetaLock,
        };
        unsafe {
            instance.get(MetaLock::new)
        }
    }

    // optional .mvccpb.MetaLockType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MetaLockType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> MetaLockType {
        self.field_type.unwrap_or(MetaLockType::ReadOnly)
    }

    fn get_field_type_for_reflect(&self) -> &::std::option::Option<MetaLockType> {
        &self.field_type
    }

    fn mut_field_type_for_reflect(&mut self) -> &mut ::std::option::Option<MetaLockType> {
        &mut self.field_type
    }

    // optional uint64 start_ts = 2;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = ::std::option::Option::None;
    }

    pub fn has_start_ts(&self) -> bool {
        self.start_ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = ::std::option::Option::Some(v);
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts.unwrap_or(0)
    }

    fn get_start_ts_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.start_ts
    }

    fn mut_start_ts_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.start_ts
    }

    // optional bytes primary_key = 3;

    pub fn clear_primary_key(&mut self) {
        self.primary_key.clear();
    }

    pub fn has_primary_key(&self) -> bool {
        self.primary_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.primary_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.primary_key.is_none() {
            self.primary_key.set_default();
        };
        self.primary_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary_key(&mut self) -> ::std::vec::Vec<u8> {
        self.primary_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_primary_key(&self) -> &[u8] {
        match self.primary_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_primary_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.primary_key
    }

    fn mut_primary_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.primary_key
    }
}

impl ::protobuf::Message for MetaLock {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_enum()?;
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.start_ts = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.primary_key)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.field_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.start_ts {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.primary_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.start_ts {
            os.write_uint64(2, v)?;
        };
        if let Some(v) = self.primary_key.as_ref() {
            os.write_bytes(3, &v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for MetaLock {
    fn new() -> MetaLock {
        MetaLock::new()
    }

    fn descriptor_static(_: ::std::option::Option<MetaLock>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MetaLockType>>(
                    "type",
                    MetaLock::get_field_type_for_reflect,
                    MetaLock::mut_field_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    MetaLock::get_start_ts_for_reflect,
                    MetaLock::mut_start_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "primary_key",
                    MetaLock::get_primary_key_for_reflect,
                    MetaLock::mut_primary_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MetaLock>(
                    "MetaLock",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MetaLock {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_start_ts();
        self.clear_primary_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MetaLock {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MetaLock {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Meta {
    // message fields
    lock: ::protobuf::SingularPtrField<MetaLock>,
    items: ::protobuf::RepeatedField<MetaItem>,
    next: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Meta {}

impl Meta {
    pub fn new() -> Meta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Meta {
        static mut instance: ::protobuf::lazy::Lazy<Meta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Meta,
        };
        unsafe {
            instance.get(Meta::new)
        }
    }

    // optional .mvccpb.MetaLock lock = 1;

    pub fn clear_lock(&mut self) {
        self.lock.clear();
    }

    pub fn has_lock(&self) -> bool {
        self.lock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lock(&mut self, v: MetaLock) {
        self.lock = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lock(&mut self) -> &mut MetaLock {
        if self.lock.is_none() {
            self.lock.set_default();
        };
        self.lock.as_mut().unwrap()
    }

    // Take field
    pub fn take_lock(&mut self) -> MetaLock {
        self.lock.take().unwrap_or_else(|| MetaLock::new())
    }

    pub fn get_lock(&self) -> &MetaLock {
        self.lock.as_ref().unwrap_or_else(|| MetaLock::default_instance())
    }

    fn get_lock_for_reflect(&self) -> &::protobuf::SingularPtrField<MetaLock> {
        &self.lock
    }

    fn mut_lock_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<MetaLock> {
        &mut self.lock
    }

    // repeated .mvccpb.MetaItem items = 2;

    pub fn clear_items(&mut self) {
        self.items.clear();
    }

    // Param is passed by value, moved
    pub fn set_items(&mut self, v: ::protobuf::RepeatedField<MetaItem>) {
        self.items = v;
    }

    // Mutable pointer to the field.
    pub fn mut_items(&mut self) -> &mut ::protobuf::RepeatedField<MetaItem> {
        &mut self.items
    }

    // Take field
    pub fn take_items(&mut self) -> ::protobuf::RepeatedField<MetaItem> {
        ::std::mem::replace(&mut self.items, ::protobuf::RepeatedField::new())
    }

    pub fn get_items(&self) -> &[MetaItem] {
        &self.items
    }

    fn get_items_for_reflect(&self) -> &::protobuf::RepeatedField<MetaItem> {
        &self.items
    }

    fn mut_items_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MetaItem> {
        &mut self.items
    }

    // optional uint64 next = 3;

    pub fn clear_next(&mut self) {
        self.next = ::std::option::Option::None;
    }

    pub fn has_next(&self) -> bool {
        self.next.is_some()
    }

    // Param is passed by value, moved
    pub fn set_next(&mut self, v: u64) {
        self.next = ::std::option::Option::Some(v);
    }

    pub fn get_next(&self) -> u64 {
        self.next.unwrap_or(0)
    }

    fn get_next_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.next
    }

    fn mut_next_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.next
    }
}

impl ::protobuf::Message for Meta {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lock)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.items)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.next = ::std::option::Option::Some(tmp);
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if let Some(v) = self.lock.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.items {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.next {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.lock.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.items {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.next {
            os.write_uint64(3, v)?;
        };
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn as_any_mut(&mut self) -> &mut ::std::any::Any {
        self as &mut ::std::any::Any
    }
    fn into_any(self: Box<Self>) -> ::std::boxed::Box<::std::any::Any> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Meta {
    fn new() -> Meta {
        Meta::new()
    }

    fn descriptor_static(_: ::std::option::Option<Meta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MetaLock>>(
                    "lock",
                    Meta::get_lock_for_reflect,
                    Meta::mut_lock_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MetaItem>>(
                    "items",
                    Meta::get_items_for_reflect,
                    Meta::mut_items_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "next",
                    Meta::get_next_for_reflect,
                    Meta::mut_next_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Meta>(
                    "Meta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Meta {
    fn clear(&mut self) {
        self.clear_lock();
        self.clear_items();
        self.clear_next();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Meta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Meta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MetaLockType {
    ReadOnly = 1,
    ReadWrite = 2,
}

impl ::protobuf::ProtobufEnum for MetaLockType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MetaLockType> {
        match value {
            1 => ::std::option::Option::Some(MetaLockType::ReadOnly),
            2 => ::std::option::Option::Some(MetaLockType::ReadWrite),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MetaLockType] = &[
            MetaLockType::ReadOnly,
            MetaLockType::ReadWrite,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MetaLockType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MetaLockType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MetaLockType {
}

impl ::protobuf::reflect::ProtobufValue for MetaLockType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x6d, 0x76, 0x63, 0x63, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06,
    0x6d, 0x76, 0x63, 0x63, 0x70, 0x62, 0x22, 0x42, 0x0a, 0x08, 0x4d, 0x65, 0x74, 0x61, 0x49, 0x74,
    0x65, 0x6d, 0x12, 0x19, 0x0a, 0x08, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x73, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x73, 0x12, 0x1b, 0x0a,
    0x09, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x08, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x54, 0x73, 0x22, 0x70, 0x0a, 0x08, 0x4d, 0x65,
    0x74, 0x61, 0x4c, 0x6f, 0x63, 0x6b, 0x12, 0x28, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x6d, 0x76, 0x63, 0x63, 0x70, 0x62, 0x2e, 0x4d, 0x65,
    0x74, 0x61, 0x4c, 0x6f, 0x63, 0x6b, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x12, 0x19, 0x0a, 0x08, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x07, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x73, 0x12, 0x1f, 0x0a, 0x0b, 0x70,
    0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x0a, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x4b, 0x65, 0x79, 0x22, 0x68, 0x0a, 0x04,
    0x4d, 0x65, 0x74, 0x61, 0x12, 0x24, 0x0a, 0x04, 0x6c, 0x6f, 0x63, 0x6b, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6d, 0x76, 0x63, 0x63, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x74, 0x61,
    0x4c, 0x6f, 0x63, 0x6b, 0x52, 0x04, 0x6c, 0x6f, 0x63, 0x6b, 0x12, 0x26, 0x0a, 0x05, 0x69, 0x74,
    0x65, 0x6d, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6d, 0x76, 0x63, 0x63,
    0x70, 0x62, 0x2e, 0x4d, 0x65, 0x74, 0x61, 0x49, 0x74, 0x65, 0x6d, 0x52, 0x05, 0x69, 0x74, 0x65,
    0x6d, 0x73, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x65, 0x78, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x04, 0x6e, 0x65, 0x78, 0x74, 0x2a, 0x2b, 0x0a, 0x0c, 0x4d, 0x65, 0x74, 0x61, 0x4c, 0x6f,
    0x63, 0x6b, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0c, 0x0a, 0x08, 0x52, 0x65, 0x61, 0x64, 0x4f, 0x6e,
    0x6c, 0x79, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x52, 0x65, 0x61, 0x64, 0x57, 0x72, 0x69, 0x74,
    0x65, 0x10, 0x02, 0x4a, 0x80, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x1b, 0x01, 0x0a, 0x08,
    0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01,
    0x08, 0x0e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x03, 0x00, 0x06, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x04, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x04, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x04, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x14,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x04, 0x21, 0x22, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x05, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x05, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x05, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x05, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x05, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x08, 0x00, 0x0b, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x08, 0x05, 0x11, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x09, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a,
    0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x04, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x0a, 0x10, 0x11, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0d, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01,
    0x01, 0x12, 0x03, 0x0d, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03,
    0x0e, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0e, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x0e, 0x0d, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e, 0x1a, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0e, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x04, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0f,
    0x1a, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0f, 0x28, 0x29,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x10, 0x04, 0x2a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x10, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x10, 0x1a, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x10, 0x28, 0x29, 0x0a, 0x93, 0x02, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x17, 0x00,
    0x1b, 0x01, 0x1a, 0x86, 0x02, 0x20, 0x4d, 0x65, 0x74, 0x61, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20,
    0x62, 0x65, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x74, 0x6f,
    0x20, 0x61, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x20, 0x69, 0x66, 0x20, 0x69, 0x74, 0x20, 0x67, 0x65,
    0x74, 0x73, 0x20, 0x74, 0x6f, 0x6f, 0x20, 0x62, 0x69, 0x67, 0x2e, 0x0a, 0x20, 0x41, 0x6e, 0x20,
    0x61, 0x75, 0x74, 0x6f, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x61, 0x73, 0x65, 0x64, 0x20, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x62, 0x65, 0x20, 0x61, 0x73, 0x73,
    0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x61, 0x63, 0x68, 0x20, 0x4d, 0x65,
    0x74, 0x61, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x2e, 0x20, 0x54, 0x68, 0x65, 0x20, 0x66, 0x69, 0x65,
    0x6c, 0x64, 0x20, 0x60, 0x6e, 0x65, 0x78, 0x74, 0x60, 0x0a, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65,
    0x73, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x4d, 0x65, 0x74, 0x61, 0x27, 0x73, 0x20, 0x69, 0x6e,
    0x64, 0x65, 0x78, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x6e, 0x65, 0x78, 0x74, 0x20, 0x69, 0x73, 0x20,
    0x30, 0x20, 0x69, 0x66, 0x20, 0x6e, 0x6f, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x4d, 0x65, 0x74,
    0x61, 0x20, 0x65, 0x78, 0x69, 0x73, 0x74, 0x73, 0x2e, 0x0a, 0x20, 0x4d, 0x65, 0x74, 0x61, 0x30,
    0x20, 0x61, 0x6c, 0x77, 0x61, 0x79, 0x73, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x61, 0x69, 0x6e, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x4c, 0x6f, 0x63, 0x6b, 0x28, 0x69, 0x66, 0x20, 0x61, 0x6e, 0x79,
    0x29, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x61, 0x74, 0x65, 0x73, 0x74,
    0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x02, 0x01, 0x12, 0x03, 0x17, 0x08, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12,
    0x03, 0x18, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x18,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x18, 0x0d, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x18, 0x16, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x18, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x19, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x19, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x19, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x19, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x19, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1a, 0x04, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x1a, 0x1e, 0x1f,
];

static mut file_descriptor_proto_lazy: ::protobuf::lazy::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::lazy::Lazy {
    lock: ::protobuf::lazy::ONCE_INIT,
    ptr: 0 as *const ::protobuf::descriptor::FileDescriptorProto,
};

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    unsafe {
        file_descriptor_proto_lazy.get(|| {
            parse_descriptor_proto()
        })
    }
}
