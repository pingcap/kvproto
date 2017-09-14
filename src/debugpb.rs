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
pub struct GetRequest {
    // message fields
    pub cf: CF,
    pub key_encoded: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetRequest {}

impl GetRequest {
    pub fn new() -> GetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetRequest,
        };
        unsafe {
            instance.get(GetRequest::new)
        }
    }

    // .debugpb.CF cf = 1;

    pub fn clear_cf(&mut self) {
        self.cf = CF::INVALID;
    }

    // Param is passed by value, moved
    pub fn set_cf(&mut self, v: CF) {
        self.cf = v;
    }

    pub fn get_cf(&self) -> CF {
        self.cf
    }

    fn get_cf_for_reflect(&self) -> &CF {
        &self.cf
    }

    fn mut_cf_for_reflect(&mut self) -> &mut CF {
        &mut self.cf
    }

    // bytes key_encoded = 2;

    pub fn clear_key_encoded(&mut self) {
        self.key_encoded.clear();
    }

    // Param is passed by value, moved
    pub fn set_key_encoded(&mut self, v: ::std::vec::Vec<u8>) {
        self.key_encoded = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_encoded(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key_encoded
    }

    // Take field
    pub fn take_key_encoded(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key_encoded, ::std::vec::Vec::new())
    }

    pub fn get_key_encoded(&self) -> &[u8] {
        &self.key_encoded
    }

    fn get_key_encoded_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key_encoded
    }

    fn mut_key_encoded_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key_encoded
    }
}

impl ::protobuf::Message for GetRequest {
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
                    }
                    let tmp = is.read_enum()?;
                    self.cf = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key_encoded)?;
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
        if self.cf != CF::INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.cf);
        }
        if !self.key_encoded.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key_encoded);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cf != CF::INVALID {
            os.write_enum(1, self.cf.value())?;
        }
        if !self.key_encoded.is_empty() {
            os.write_bytes(2, &self.key_encoded)?;
        }
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

impl ::protobuf::MessageStatic for GetRequest {
    fn new() -> GetRequest {
        GetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CF>>(
                    "cf",
                    GetRequest::get_cf_for_reflect,
                    GetRequest::mut_cf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key_encoded",
                    GetRequest::get_key_encoded_for_reflect,
                    GetRequest::mut_key_encoded_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetRequest>(
                    "GetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetRequest {
    fn clear(&mut self) {
        self.clear_cf();
        self.clear_key_encoded();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct GetResponse {
    // message fields
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetResponse {}

impl GetResponse {
    pub fn new() -> GetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetResponse,
        };
        unsafe {
            instance.get(GetResponse::new)
        }
    }

    // bytes value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.value
    }
}

impl ::protobuf::Message for GetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.value)?;
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
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.value.is_empty() {
            os.write_bytes(1, &self.value)?;
        }
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

impl ::protobuf::MessageStatic for GetResponse {
    fn new() -> GetResponse {
        GetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    GetResponse::get_value_for_reflect,
                    GetResponse::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetResponse>(
                    "GetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetResponse {
    fn clear(&mut self) {
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for GetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for GetResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MvccRequest {
    // message fields
    pub by: MvccRequest_By,
    pub key: ::std::string::String,
    pub start_ts: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MvccRequest {}

impl MvccRequest {
    pub fn new() -> MvccRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MvccRequest {
        static mut instance: ::protobuf::lazy::Lazy<MvccRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MvccRequest,
        };
        unsafe {
            instance.get(MvccRequest::new)
        }
    }

    // .debugpb.MvccRequest.By by = 1;

    pub fn clear_by(&mut self) {
        self.by = MvccRequest_By::INVALID;
    }

    // Param is passed by value, moved
    pub fn set_by(&mut self, v: MvccRequest_By) {
        self.by = v;
    }

    pub fn get_by(&self) -> MvccRequest_By {
        self.by
    }

    fn get_by_for_reflect(&self) -> &MvccRequest_By {
        &self.by
    }

    fn mut_by_for_reflect(&mut self) -> &mut MvccRequest_By {
        &mut self.by
    }

    // string key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::string::String {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // uint64 start_ts = 3;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }

    fn get_start_ts_for_reflect(&self) -> &u64 {
        &self.start_ts
    }

    fn mut_start_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_ts
    }
}

impl ::protobuf::Message for MvccRequest {
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
                    }
                    let tmp = is.read_enum()?;
                    self.by = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_ts = tmp;
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
        if self.by != MvccRequest_By::INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.by);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.key);
        }
        if self.start_ts != 0 {
            my_size += ::protobuf::rt::value_size(3, self.start_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.by != MvccRequest_By::INVALID {
            os.write_enum(1, self.by.value())?;
        }
        if !self.key.is_empty() {
            os.write_string(2, &self.key)?;
        }
        if self.start_ts != 0 {
            os.write_uint64(3, self.start_ts)?;
        }
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

impl ::protobuf::MessageStatic for MvccRequest {
    fn new() -> MvccRequest {
        MvccRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<MvccRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<MvccRequest_By>>(
                    "by",
                    MvccRequest::get_by_for_reflect,
                    MvccRequest::mut_by_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    MvccRequest::get_key_for_reflect,
                    MvccRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    MvccRequest::get_start_ts_for_reflect,
                    MvccRequest::mut_start_ts_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MvccRequest>(
                    "MvccRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MvccRequest {
    fn clear(&mut self) {
        self.clear_by();
        self.clear_key();
        self.clear_start_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MvccRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MvccRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MvccRequest_By {
    INVALID = 0,
    KEY = 1,
    START_TS = 2,
}

impl ::protobuf::ProtobufEnum for MvccRequest_By {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MvccRequest_By> {
        match value {
            0 => ::std::option::Option::Some(MvccRequest_By::INVALID),
            1 => ::std::option::Option::Some(MvccRequest_By::KEY),
            2 => ::std::option::Option::Some(MvccRequest_By::START_TS),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MvccRequest_By] = &[
            MvccRequest_By::INVALID,
            MvccRequest_By::KEY,
            MvccRequest_By::START_TS,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<MvccRequest_By>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MvccRequest_By", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MvccRequest_By {
}

impl ::std::default::Default for MvccRequest_By {
    fn default() -> Self {
        MvccRequest_By::INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for MvccRequest_By {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MvccResponse {
    // message fields
    pub entries: ::protobuf::RepeatedField<MvccResponse_Entry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MvccResponse {}

impl MvccResponse {
    pub fn new() -> MvccResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MvccResponse {
        static mut instance: ::protobuf::lazy::Lazy<MvccResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MvccResponse,
        };
        unsafe {
            instance.get(MvccResponse::new)
        }
    }

    // repeated .debugpb.MvccResponse.Entry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<MvccResponse_Entry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<MvccResponse_Entry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<MvccResponse_Entry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[MvccResponse_Entry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<MvccResponse_Entry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<MvccResponse_Entry> {
        &mut self.entries
    }
}

impl ::protobuf::Message for MvccResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.entries {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.entries)?;
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
        for value in &self.entries {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.entries {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
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

impl ::protobuf::MessageStatic for MvccResponse {
    fn new() -> MvccResponse {
        MvccResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<MvccResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<MvccResponse_Entry>>(
                    "entries",
                    MvccResponse::get_entries_for_reflect,
                    MvccResponse::mut_entries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MvccResponse>(
                    "MvccResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MvccResponse {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MvccResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MvccResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct MvccResponse_Entry {
    // message fields
    pub key: ::std::string::String,
    pub info: ::protobuf::SingularPtrField<super::kvrpcpb::MvccInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for MvccResponse_Entry {}

impl MvccResponse_Entry {
    pub fn new() -> MvccResponse_Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static MvccResponse_Entry {
        static mut instance: ::protobuf::lazy::Lazy<MvccResponse_Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const MvccResponse_Entry,
        };
        unsafe {
            instance.get(MvccResponse_Entry::new)
        }
    }

    // string key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::string::String) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }

    pub fn get_key(&self) -> &str {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::string::String {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.key
    }

    // .kvrpcpb.MvccInfo info = 2;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: super::kvrpcpb::MvccInfo) {
        self.info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut super::kvrpcpb::MvccInfo {
        if self.info.is_none() {
            self.info.set_default();
        }
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> super::kvrpcpb::MvccInfo {
        self.info.take().unwrap_or_else(|| super::kvrpcpb::MvccInfo::new())
    }

    pub fn get_info(&self) -> &super::kvrpcpb::MvccInfo {
        self.info.as_ref().unwrap_or_else(|| super::kvrpcpb::MvccInfo::default_instance())
    }

    fn get_info_for_reflect(&self) -> &::protobuf::SingularPtrField<super::kvrpcpb::MvccInfo> {
        &self.info
    }

    fn mut_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::kvrpcpb::MvccInfo> {
        &mut self.info
    }
}

impl ::protobuf::Message for MvccResponse_Entry {
    fn is_initialized(&self) -> bool {
        for v in &self.info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.info)?;
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.key);
        }
        if let Some(ref v) = self.info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_string(1, &self.key)?;
        }
        if let Some(ref v) = self.info.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for MvccResponse_Entry {
    fn new() -> MvccResponse_Entry {
        MvccResponse_Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<MvccResponse_Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "key",
                    MvccResponse_Entry::get_key_for_reflect,
                    MvccResponse_Entry::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::kvrpcpb::MvccInfo>>(
                    "info",
                    MvccResponse_Entry::get_info_for_reflect,
                    MvccResponse_Entry::mut_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<MvccResponse_Entry>(
                    "MvccResponse_Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for MvccResponse_Entry {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for MvccResponse_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MvccResponse_Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RaftLogRequest {
    // message fields
    pub region_id: u64,
    pub log_index: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftLogRequest {}

impl RaftLogRequest {
    pub fn new() -> RaftLogRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftLogRequest {
        static mut instance: ::protobuf::lazy::Lazy<RaftLogRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftLogRequest,
        };
        unsafe {
            instance.get(RaftLogRequest::new)
        }
    }

    // uint64 region_id = 1;

    pub fn clear_region_id(&mut self) {
        self.region_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }

    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }

    fn get_region_id_for_reflect(&self) -> &u64 {
        &self.region_id
    }

    fn mut_region_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.region_id
    }

    // uint64 log_index = 2;

    pub fn clear_log_index(&mut self) {
        self.log_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_log_index(&mut self, v: u64) {
        self.log_index = v;
    }

    pub fn get_log_index(&self) -> u64 {
        self.log_index
    }

    fn get_log_index_for_reflect(&self) -> &u64 {
        &self.log_index
    }

    fn mut_log_index_for_reflect(&mut self) -> &mut u64 {
        &mut self.log_index
    }
}

impl ::protobuf::Message for RaftLogRequest {
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
                    }
                    let tmp = is.read_uint64()?;
                    self.region_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.log_index = tmp;
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
        if self.region_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.region_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.log_index != 0 {
            my_size += ::protobuf::rt::value_size(2, self.log_index, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.region_id != 0 {
            os.write_uint64(1, self.region_id)?;
        }
        if self.log_index != 0 {
            os.write_uint64(2, self.log_index)?;
        }
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

impl ::protobuf::MessageStatic for RaftLogRequest {
    fn new() -> RaftLogRequest {
        RaftLogRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftLogRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "region_id",
                    RaftLogRequest::get_region_id_for_reflect,
                    RaftLogRequest::mut_region_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "log_index",
                    RaftLogRequest::get_log_index_for_reflect,
                    RaftLogRequest::mut_log_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftLogRequest>(
                    "RaftLogRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftLogRequest {
    fn clear(&mut self) {
        self.clear_region_id();
        self.clear_log_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftLogRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftLogRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RaftLogResponse {
    // message fields
    pub entry: ::protobuf::SingularPtrField<super::eraftpb::Entry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftLogResponse {}

impl RaftLogResponse {
    pub fn new() -> RaftLogResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftLogResponse {
        static mut instance: ::protobuf::lazy::Lazy<RaftLogResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftLogResponse,
        };
        unsafe {
            instance.get(RaftLogResponse::new)
        }
    }

    // .eraftpb.Entry entry = 1;

    pub fn clear_entry(&mut self) {
        self.entry.clear();
    }

    pub fn has_entry(&self) -> bool {
        self.entry.is_some()
    }

    // Param is passed by value, moved
    pub fn set_entry(&mut self, v: super::eraftpb::Entry) {
        self.entry = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_entry(&mut self) -> &mut super::eraftpb::Entry {
        if self.entry.is_none() {
            self.entry.set_default();
        }
        self.entry.as_mut().unwrap()
    }

    // Take field
    pub fn take_entry(&mut self) -> super::eraftpb::Entry {
        self.entry.take().unwrap_or_else(|| super::eraftpb::Entry::new())
    }

    pub fn get_entry(&self) -> &super::eraftpb::Entry {
        self.entry.as_ref().unwrap_or_else(|| super::eraftpb::Entry::default_instance())
    }

    fn get_entry_for_reflect(&self) -> &::protobuf::SingularPtrField<super::eraftpb::Entry> {
        &self.entry
    }

    fn mut_entry_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::eraftpb::Entry> {
        &mut self.entry
    }
}

impl ::protobuf::Message for RaftLogResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.entry {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.entry)?;
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
        if let Some(ref v) = self.entry.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.entry.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for RaftLogResponse {
    fn new() -> RaftLogResponse {
        RaftLogResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftLogResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::eraftpb::Entry>>(
                    "entry",
                    RaftLogResponse::get_entry_for_reflect,
                    RaftLogResponse::mut_entry_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftLogResponse>(
                    "RaftLogResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftLogResponse {
    fn clear(&mut self) {
        self.clear_entry();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftLogResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftLogResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegionInfoRequest {
    // message fields
    pub region_id: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionInfoRequest {}

impl RegionInfoRequest {
    pub fn new() -> RegionInfoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionInfoRequest {
        static mut instance: ::protobuf::lazy::Lazy<RegionInfoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionInfoRequest,
        };
        unsafe {
            instance.get(RegionInfoRequest::new)
        }
    }

    // uint64 region_id = 1;

    pub fn clear_region_id(&mut self) {
        self.region_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }

    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }

    fn get_region_id_for_reflect(&self) -> &u64 {
        &self.region_id
    }

    fn mut_region_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.region_id
    }
}

impl ::protobuf::Message for RegionInfoRequest {
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
                    }
                    let tmp = is.read_uint64()?;
                    self.region_id = tmp;
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
        if self.region_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.region_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.region_id != 0 {
            os.write_uint64(1, self.region_id)?;
        }
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

impl ::protobuf::MessageStatic for RegionInfoRequest {
    fn new() -> RegionInfoRequest {
        RegionInfoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionInfoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "region_id",
                    RegionInfoRequest::get_region_id_for_reflect,
                    RegionInfoRequest::mut_region_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionInfoRequest>(
                    "RegionInfoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionInfoRequest {
    fn clear(&mut self) {
        self.clear_region_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegionInfoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionInfoRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegionInfoResponse {
    // message fields
    pub raft_local_state: ::protobuf::SingularPtrField<super::raft_serverpb::RaftLocalState>,
    pub raft_apply_state: ::protobuf::SingularPtrField<super::raft_serverpb::RaftApplyState>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionInfoResponse {}

impl RegionInfoResponse {
    pub fn new() -> RegionInfoResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionInfoResponse {
        static mut instance: ::protobuf::lazy::Lazy<RegionInfoResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionInfoResponse,
        };
        unsafe {
            instance.get(RegionInfoResponse::new)
        }
    }

    // .raft_serverpb.RaftLocalState raft_local_state = 1;

    pub fn clear_raft_local_state(&mut self) {
        self.raft_local_state.clear();
    }

    pub fn has_raft_local_state(&self) -> bool {
        self.raft_local_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raft_local_state(&mut self, v: super::raft_serverpb::RaftLocalState) {
        self.raft_local_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raft_local_state(&mut self) -> &mut super::raft_serverpb::RaftLocalState {
        if self.raft_local_state.is_none() {
            self.raft_local_state.set_default();
        }
        self.raft_local_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_raft_local_state(&mut self) -> super::raft_serverpb::RaftLocalState {
        self.raft_local_state.take().unwrap_or_else(|| super::raft_serverpb::RaftLocalState::new())
    }

    pub fn get_raft_local_state(&self) -> &super::raft_serverpb::RaftLocalState {
        self.raft_local_state.as_ref().unwrap_or_else(|| super::raft_serverpb::RaftLocalState::default_instance())
    }

    fn get_raft_local_state_for_reflect(&self) -> &::protobuf::SingularPtrField<super::raft_serverpb::RaftLocalState> {
        &self.raft_local_state
    }

    fn mut_raft_local_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::raft_serverpb::RaftLocalState> {
        &mut self.raft_local_state
    }

    // .raft_serverpb.RaftApplyState raft_apply_state = 2;

    pub fn clear_raft_apply_state(&mut self) {
        self.raft_apply_state.clear();
    }

    pub fn has_raft_apply_state(&self) -> bool {
        self.raft_apply_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_raft_apply_state(&mut self, v: super::raft_serverpb::RaftApplyState) {
        self.raft_apply_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_raft_apply_state(&mut self) -> &mut super::raft_serverpb::RaftApplyState {
        if self.raft_apply_state.is_none() {
            self.raft_apply_state.set_default();
        }
        self.raft_apply_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_raft_apply_state(&mut self) -> super::raft_serverpb::RaftApplyState {
        self.raft_apply_state.take().unwrap_or_else(|| super::raft_serverpb::RaftApplyState::new())
    }

    pub fn get_raft_apply_state(&self) -> &super::raft_serverpb::RaftApplyState {
        self.raft_apply_state.as_ref().unwrap_or_else(|| super::raft_serverpb::RaftApplyState::default_instance())
    }

    fn get_raft_apply_state_for_reflect(&self) -> &::protobuf::SingularPtrField<super::raft_serverpb::RaftApplyState> {
        &self.raft_apply_state
    }

    fn mut_raft_apply_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::raft_serverpb::RaftApplyState> {
        &mut self.raft_apply_state
    }
}

impl ::protobuf::Message for RegionInfoResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.raft_local_state {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.raft_apply_state {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.raft_local_state)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.raft_apply_state)?;
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
        if let Some(ref v) = self.raft_local_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.raft_apply_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.raft_local_state.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.raft_apply_state.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for RegionInfoResponse {
    fn new() -> RegionInfoResponse {
        RegionInfoResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionInfoResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::raft_serverpb::RaftLocalState>>(
                    "raft_local_state",
                    RegionInfoResponse::get_raft_local_state_for_reflect,
                    RegionInfoResponse::mut_raft_local_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::raft_serverpb::RaftApplyState>>(
                    "raft_apply_state",
                    RegionInfoResponse::get_raft_apply_state_for_reflect,
                    RegionInfoResponse::mut_raft_apply_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionInfoResponse>(
                    "RegionInfoResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionInfoResponse {
    fn clear(&mut self) {
        self.clear_raft_local_state();
        self.clear_raft_apply_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegionInfoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionInfoResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SizeRequest {
    // message fields
    pub region_id: u64,
    pub cfs: ::std::vec::Vec<CF>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SizeRequest {}

impl SizeRequest {
    pub fn new() -> SizeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SizeRequest {
        static mut instance: ::protobuf::lazy::Lazy<SizeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SizeRequest,
        };
        unsafe {
            instance.get(SizeRequest::new)
        }
    }

    // uint64 region_id = 1;

    pub fn clear_region_id(&mut self) {
        self.region_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }

    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }

    fn get_region_id_for_reflect(&self) -> &u64 {
        &self.region_id
    }

    fn mut_region_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.region_id
    }

    // repeated .debugpb.CF cfs = 2;

    pub fn clear_cfs(&mut self) {
        self.cfs.clear();
    }

    // Param is passed by value, moved
    pub fn set_cfs(&mut self, v: ::std::vec::Vec<CF>) {
        self.cfs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cfs(&mut self) -> &mut ::std::vec::Vec<CF> {
        &mut self.cfs
    }

    // Take field
    pub fn take_cfs(&mut self) -> ::std::vec::Vec<CF> {
        ::std::mem::replace(&mut self.cfs, ::std::vec::Vec::new())
    }

    pub fn get_cfs(&self) -> &[CF] {
        &self.cfs
    }

    fn get_cfs_for_reflect(&self) -> &::std::vec::Vec<CF> {
        &self.cfs
    }

    fn mut_cfs_for_reflect(&mut self) -> &mut ::std::vec::Vec<CF> {
        &mut self.cfs
    }
}

impl ::protobuf::Message for SizeRequest {
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
                    }
                    let tmp = is.read_uint64()?;
                    self.region_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.cfs)?;
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
        if self.region_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.region_id, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.cfs {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.region_id != 0 {
            os.write_uint64(1, self.region_id)?;
        }
        for v in &self.cfs {
            os.write_enum(2, v.value())?;
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

impl ::protobuf::MessageStatic for SizeRequest {
    fn new() -> SizeRequest {
        SizeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SizeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "region_id",
                    SizeRequest::get_region_id_for_reflect,
                    SizeRequest::mut_region_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CF>>(
                    "cfs",
                    SizeRequest::get_cfs_for_reflect,
                    SizeRequest::mut_cfs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SizeRequest>(
                    "SizeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SizeRequest {
    fn clear(&mut self) {
        self.clear_region_id();
        self.clear_cfs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SizeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SizeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SizeResponse {
    // message fields
    pub size: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SizeResponse {}

impl SizeResponse {
    pub fn new() -> SizeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SizeResponse {
        static mut instance: ::protobuf::lazy::Lazy<SizeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SizeResponse,
        };
        unsafe {
            instance.get(SizeResponse::new)
        }
    }

    // uint64 size = 1;

    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u64) {
        self.size = v;
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    fn get_size_for_reflect(&self) -> &u64 {
        &self.size
    }

    fn mut_size_for_reflect(&mut self) -> &mut u64 {
        &mut self.size
    }
}

impl ::protobuf::Message for SizeResponse {
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
                    }
                    let tmp = is.read_uint64()?;
                    self.size = tmp;
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
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(1, self.size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.size != 0 {
            os.write_uint64(1, self.size)?;
        }
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

impl ::protobuf::MessageStatic for SizeResponse {
    fn new() -> SizeResponse {
        SizeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SizeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "size",
                    SizeResponse::get_size_for_reflect,
                    SizeResponse::mut_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SizeResponse>(
                    "SizeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SizeResponse {
    fn clear(&mut self) {
        self.clear_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SizeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SizeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanRequest {
    // message fields
    pub from_key_encoded: ::std::vec::Vec<u8>,
    pub to_key_encoded: ::std::vec::Vec<u8>,
    pub filter: ::protobuf::SingularPtrField<ScanRequest_Filter>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanRequest {}

impl ScanRequest {
    pub fn new() -> ScanRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanRequest {
        static mut instance: ::protobuf::lazy::Lazy<ScanRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanRequest,
        };
        unsafe {
            instance.get(ScanRequest::new)
        }
    }

    // bytes from_key_encoded = 1;

    pub fn clear_from_key_encoded(&mut self) {
        self.from_key_encoded.clear();
    }

    // Param is passed by value, moved
    pub fn set_from_key_encoded(&mut self, v: ::std::vec::Vec<u8>) {
        self.from_key_encoded = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_key_encoded(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.from_key_encoded
    }

    // Take field
    pub fn take_from_key_encoded(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.from_key_encoded, ::std::vec::Vec::new())
    }

    pub fn get_from_key_encoded(&self) -> &[u8] {
        &self.from_key_encoded
    }

    fn get_from_key_encoded_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.from_key_encoded
    }

    fn mut_from_key_encoded_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.from_key_encoded
    }

    // bytes to_key_encoded = 2;

    pub fn clear_to_key_encoded(&mut self) {
        self.to_key_encoded.clear();
    }

    // Param is passed by value, moved
    pub fn set_to_key_encoded(&mut self, v: ::std::vec::Vec<u8>) {
        self.to_key_encoded = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to_key_encoded(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.to_key_encoded
    }

    // Take field
    pub fn take_to_key_encoded(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.to_key_encoded, ::std::vec::Vec::new())
    }

    pub fn get_to_key_encoded(&self) -> &[u8] {
        &self.to_key_encoded
    }

    fn get_to_key_encoded_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.to_key_encoded
    }

    fn mut_to_key_encoded_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.to_key_encoded
    }

    // .debugpb.ScanRequest.Filter filter = 3;

    pub fn clear_filter(&mut self) {
        self.filter.clear();
    }

    pub fn has_filter(&self) -> bool {
        self.filter.is_some()
    }

    // Param is passed by value, moved
    pub fn set_filter(&mut self, v: ScanRequest_Filter) {
        self.filter = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_filter(&mut self) -> &mut ScanRequest_Filter {
        if self.filter.is_none() {
            self.filter.set_default();
        }
        self.filter.as_mut().unwrap()
    }

    // Take field
    pub fn take_filter(&mut self) -> ScanRequest_Filter {
        self.filter.take().unwrap_or_else(|| ScanRequest_Filter::new())
    }

    pub fn get_filter(&self) -> &ScanRequest_Filter {
        self.filter.as_ref().unwrap_or_else(|| ScanRequest_Filter::default_instance())
    }

    fn get_filter_for_reflect(&self) -> &::protobuf::SingularPtrField<ScanRequest_Filter> {
        &self.filter
    }

    fn mut_filter_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ScanRequest_Filter> {
        &mut self.filter
    }
}

impl ::protobuf::Message for ScanRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.filter {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.from_key_encoded)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.to_key_encoded)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.filter)?;
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
        if !self.from_key_encoded.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.from_key_encoded);
        }
        if !self.to_key_encoded.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.to_key_encoded);
        }
        if let Some(ref v) = self.filter.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.from_key_encoded.is_empty() {
            os.write_bytes(1, &self.from_key_encoded)?;
        }
        if !self.to_key_encoded.is_empty() {
            os.write_bytes(2, &self.to_key_encoded)?;
        }
        if let Some(ref v) = self.filter.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for ScanRequest {
    fn new() -> ScanRequest {
        ScanRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "from_key_encoded",
                    ScanRequest::get_from_key_encoded_for_reflect,
                    ScanRequest::mut_from_key_encoded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "to_key_encoded",
                    ScanRequest::get_to_key_encoded_for_reflect,
                    ScanRequest::mut_to_key_encoded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ScanRequest_Filter>>(
                    "filter",
                    ScanRequest::get_filter_for_reflect,
                    ScanRequest::mut_filter_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanRequest>(
                    "ScanRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanRequest {
    fn clear(&mut self) {
        self.clear_from_key_encoded();
        self.clear_to_key_encoded();
        self.clear_filter();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanRequest_Filter {
    // message fields
    pub limit: u64,
    pub start_ts: u64,
    pub commit_ts: u64,
    pub key_only: bool,
    pub cfs: ::std::vec::Vec<CF>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanRequest_Filter {}

impl ScanRequest_Filter {
    pub fn new() -> ScanRequest_Filter {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanRequest_Filter {
        static mut instance: ::protobuf::lazy::Lazy<ScanRequest_Filter> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanRequest_Filter,
        };
        unsafe {
            instance.get(ScanRequest_Filter::new)
        }
    }

    // uint64 limit = 1;

    pub fn clear_limit(&mut self) {
        self.limit = 0;
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: u64) {
        self.limit = v;
    }

    pub fn get_limit(&self) -> u64 {
        self.limit
    }

    fn get_limit_for_reflect(&self) -> &u64 {
        &self.limit
    }

    fn mut_limit_for_reflect(&mut self) -> &mut u64 {
        &mut self.limit
    }

    // uint64 start_ts = 2;

    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }

    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }

    fn get_start_ts_for_reflect(&self) -> &u64 {
        &self.start_ts
    }

    fn mut_start_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.start_ts
    }

    // uint64 commit_ts = 3;

    pub fn clear_commit_ts(&mut self) {
        self.commit_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_ts(&mut self, v: u64) {
        self.commit_ts = v;
    }

    pub fn get_commit_ts(&self) -> u64 {
        self.commit_ts
    }

    fn get_commit_ts_for_reflect(&self) -> &u64 {
        &self.commit_ts
    }

    fn mut_commit_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.commit_ts
    }

    // bool key_only = 4;

    pub fn clear_key_only(&mut self) {
        self.key_only = false;
    }

    // Param is passed by value, moved
    pub fn set_key_only(&mut self, v: bool) {
        self.key_only = v;
    }

    pub fn get_key_only(&self) -> bool {
        self.key_only
    }

    fn get_key_only_for_reflect(&self) -> &bool {
        &self.key_only
    }

    fn mut_key_only_for_reflect(&mut self) -> &mut bool {
        &mut self.key_only
    }

    // repeated .debugpb.CF cfs = 5;

    pub fn clear_cfs(&mut self) {
        self.cfs.clear();
    }

    // Param is passed by value, moved
    pub fn set_cfs(&mut self, v: ::std::vec::Vec<CF>) {
        self.cfs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cfs(&mut self) -> &mut ::std::vec::Vec<CF> {
        &mut self.cfs
    }

    // Take field
    pub fn take_cfs(&mut self) -> ::std::vec::Vec<CF> {
        ::std::mem::replace(&mut self.cfs, ::std::vec::Vec::new())
    }

    pub fn get_cfs(&self) -> &[CF] {
        &self.cfs
    }

    fn get_cfs_for_reflect(&self) -> &::std::vec::Vec<CF> {
        &self.cfs
    }

    fn mut_cfs_for_reflect(&mut self) -> &mut ::std::vec::Vec<CF> {
        &mut self.cfs
    }
}

impl ::protobuf::Message for ScanRequest_Filter {
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
                    }
                    let tmp = is.read_uint64()?;
                    self.limit = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.start_ts = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.commit_ts = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_bool()?;
                    self.key_only = tmp;
                },
                5 => {
                    ::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.cfs)?;
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
        if self.limit != 0 {
            my_size += ::protobuf::rt::value_size(1, self.limit, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.start_ts != 0 {
            my_size += ::protobuf::rt::value_size(2, self.start_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.commit_ts != 0 {
            my_size += ::protobuf::rt::value_size(3, self.commit_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.key_only != false {
            my_size += 2;
        }
        for value in &self.cfs {
            my_size += ::protobuf::rt::enum_size(5, *value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.limit != 0 {
            os.write_uint64(1, self.limit)?;
        }
        if self.start_ts != 0 {
            os.write_uint64(2, self.start_ts)?;
        }
        if self.commit_ts != 0 {
            os.write_uint64(3, self.commit_ts)?;
        }
        if self.key_only != false {
            os.write_bool(4, self.key_only)?;
        }
        for v in &self.cfs {
            os.write_enum(5, v.value())?;
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

impl ::protobuf::MessageStatic for ScanRequest_Filter {
    fn new() -> ScanRequest_Filter {
        ScanRequest_Filter::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanRequest_Filter>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "limit",
                    ScanRequest_Filter::get_limit_for_reflect,
                    ScanRequest_Filter::mut_limit_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "start_ts",
                    ScanRequest_Filter::get_start_ts_for_reflect,
                    ScanRequest_Filter::mut_start_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_ts",
                    ScanRequest_Filter::get_commit_ts_for_reflect,
                    ScanRequest_Filter::mut_commit_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "key_only",
                    ScanRequest_Filter::get_key_only_for_reflect,
                    ScanRequest_Filter::mut_key_only_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CF>>(
                    "cfs",
                    ScanRequest_Filter::get_cfs_for_reflect,
                    ScanRequest_Filter::mut_cfs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanRequest_Filter>(
                    "ScanRequest_Filter",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanRequest_Filter {
    fn clear(&mut self) {
        self.clear_limit();
        self.clear_start_ts();
        self.clear_commit_ts();
        self.clear_key_only();
        self.clear_cfs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanRequest_Filter {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanRequest_Filter {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanResponse {
    // message fields
    pub key_encoded: ::std::vec::Vec<u8>,
    pub info: ::protobuf::SingularPtrField<super::kvrpcpb::MvccInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanResponse {}

impl ScanResponse {
    pub fn new() -> ScanResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanResponse {
        static mut instance: ::protobuf::lazy::Lazy<ScanResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanResponse,
        };
        unsafe {
            instance.get(ScanResponse::new)
        }
    }

    // bytes key_encoded = 1;

    pub fn clear_key_encoded(&mut self) {
        self.key_encoded.clear();
    }

    // Param is passed by value, moved
    pub fn set_key_encoded(&mut self, v: ::std::vec::Vec<u8>) {
        self.key_encoded = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_encoded(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key_encoded
    }

    // Take field
    pub fn take_key_encoded(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key_encoded, ::std::vec::Vec::new())
    }

    pub fn get_key_encoded(&self) -> &[u8] {
        &self.key_encoded
    }

    fn get_key_encoded_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key_encoded
    }

    fn mut_key_encoded_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key_encoded
    }

    // .kvrpcpb.MvccInfo info = 2;

    pub fn clear_info(&mut self) {
        self.info.clear();
    }

    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_info(&mut self, v: super::kvrpcpb::MvccInfo) {
        self.info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_info(&mut self) -> &mut super::kvrpcpb::MvccInfo {
        if self.info.is_none() {
            self.info.set_default();
        }
        self.info.as_mut().unwrap()
    }

    // Take field
    pub fn take_info(&mut self) -> super::kvrpcpb::MvccInfo {
        self.info.take().unwrap_or_else(|| super::kvrpcpb::MvccInfo::new())
    }

    pub fn get_info(&self) -> &super::kvrpcpb::MvccInfo {
        self.info.as_ref().unwrap_or_else(|| super::kvrpcpb::MvccInfo::default_instance())
    }

    fn get_info_for_reflect(&self) -> &::protobuf::SingularPtrField<super::kvrpcpb::MvccInfo> {
        &self.info
    }

    fn mut_info_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::kvrpcpb::MvccInfo> {
        &mut self.info
    }
}

impl ::protobuf::Message for ScanResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.info {
            if !v.is_initialized() {
                return false;
            }
        };
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key_encoded)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.info)?;
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
        if !self.key_encoded.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key_encoded);
        }
        if let Some(ref v) = self.info.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key_encoded.is_empty() {
            os.write_bytes(1, &self.key_encoded)?;
        }
        if let Some(ref v) = self.info.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
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

impl ::protobuf::MessageStatic for ScanResponse {
    fn new() -> ScanResponse {
        ScanResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key_encoded",
                    ScanResponse::get_key_encoded_for_reflect,
                    ScanResponse::mut_key_encoded_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::kvrpcpb::MvccInfo>>(
                    "info",
                    ScanResponse::get_info_for_reflect,
                    ScanResponse::mut_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanResponse>(
                    "ScanResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanResponse {
    fn clear(&mut self) {
        self.clear_key_encoded();
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CF {
    INVALID = 0,
    DEFAULT = 1,
    WRITE = 2,
    LOCK = 3,
    RAFT = 4,
}

impl ::protobuf::ProtobufEnum for CF {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CF> {
        match value {
            0 => ::std::option::Option::Some(CF::INVALID),
            1 => ::std::option::Option::Some(CF::DEFAULT),
            2 => ::std::option::Option::Some(CF::WRITE),
            3 => ::std::option::Option::Some(CF::LOCK),
            4 => ::std::option::Option::Some(CF::RAFT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CF] = &[
            CF::INVALID,
            CF::DEFAULT,
            CF::WRITE,
            CF::LOCK,
            CF::RAFT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<CF>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CF", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CF {
}

impl ::std::default::Default for CF {
    fn default() -> Self {
        CF::INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for CF {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rdebugpb.proto\x12\x07debugpb\x1a\reraftpb.proto\x1a\rkvrpcpb.proto\
    \x1a\x13raft_serverpb.proto\x1a\x14gogoproto/gogo.proto\"J\n\nGetRequest\
    \x12\x1b\n\x02cf\x18\x01\x20\x01(\x0e2\x0b.debugpb.CFR\x02cf\x12\x1f\n\
    \x0bkey_encoded\x18\x02\x20\x01(\x0cR\nkeyEncoded\"#\n\x0bGetResponse\
    \x12\x14\n\x05value\x18\x01\x20\x01(\x0cR\x05value\"\x8d\x01\n\x0bMvccRe\
    quest\x12'\n\x02by\x18\x01\x20\x01(\x0e2\x17.debugpb.MvccRequest.ByR\x02\
    by\x12\x10\n\x03key\x18\x02\x20\x01(\tR\x03key\x12\x19\n\x08start_ts\x18\
    \x03\x20\x01(\x04R\x07startTs\"(\n\x02By\x12\x0b\n\x07INVALID\x10\0\x12\
    \x07\n\x03KEY\x10\x01\x12\x0c\n\x08START_TS\x10\x02\"\x87\x01\n\x0cMvccR\
    esponse\x125\n\x07entries\x18\x01\x20\x03(\x0b2\x1b.debugpb.MvccResponse\
    .EntryR\x07entries\x1a@\n\x05Entry\x12\x10\n\x03key\x18\x01\x20\x01(\tR\
    \x03key\x12%\n\x04info\x18\x02\x20\x01(\x0b2\x11.kvrpcpb.MvccInfoR\x04in\
    fo\"J\n\x0eRaftLogRequest\x12\x1b\n\tregion_id\x18\x01\x20\x01(\x04R\x08\
    regionId\x12\x1b\n\tlog_index\x18\x02\x20\x01(\x04R\x08logIndex\"7\n\x0f\
    RaftLogResponse\x12$\n\x05entry\x18\x01\x20\x01(\x0b2\x0e.eraftpb.EntryR\
    \x05entry\"0\n\x11RegionInfoRequest\x12\x1b\n\tregion_id\x18\x01\x20\x01\
    (\x04R\x08regionId\"\xa6\x01\n\x12RegionInfoResponse\x12G\n\x10raft_loca\
    l_state\x18\x01\x20\x01(\x0b2\x1d.raft_serverpb.RaftLocalStateR\x0eraftL\
    ocalState\x12G\n\x10raft_apply_state\x18\x02\x20\x01(\x0b2\x1d.raft_serv\
    erpb.RaftApplyStateR\x0eraftApplyState\"I\n\x0bSizeRequest\x12\x1b\n\tre\
    gion_id\x18\x01\x20\x01(\x04R\x08regionId\x12\x1d\n\x03cfs\x18\x02\x20\
    \x03(\x0e2\x0b.debugpb.CFR\x03cfs\"\"\n\x0cSizeResponse\x12\x12\n\x04siz\
    e\x18\x01\x20\x01(\x04R\x04size\"\xa5\x02\n\x0bScanRequest\x12(\n\x10fro\
    m_key_encoded\x18\x01\x20\x01(\x0cR\x0efromKeyEncoded\x12$\n\x0eto_key_e\
    ncoded\x18\x02\x20\x01(\x0cR\x0ctoKeyEncoded\x123\n\x06filter\x18\x03\
    \x20\x01(\x0b2\x1b.debugpb.ScanRequest.FilterR\x06filter\x1a\x90\x01\n\
    \x06Filter\x12\x14\n\x05limit\x18\x01\x20\x01(\x04R\x05limit\x12\x19\n\
    \x08start_ts\x18\x02\x20\x01(\x04R\x07startTs\x12\x1b\n\tcommit_ts\x18\
    \x03\x20\x01(\x04R\x08commitTs\x12\x19\n\x08key_only\x18\x04\x20\x01(\
    \x08R\x07keyOnly\x12\x1d\n\x03cfs\x18\x05\x20\x03(\x0e2\x0b.debugpb.CFR\
    \x03cfs\"V\n\x0cScanResponse\x12\x1f\n\x0bkey_encoded\x18\x01\x20\x01(\
    \x0cR\nkeyEncoded\x12%\n\x04info\x18\x02\x20\x01(\x0b2\x11.kvrpcpb.MvccI\
    nfoR\x04info*=\n\x02CF\x12\x0b\n\x07INVALID\x10\0\x12\x0b\n\x07DEFAULT\
    \x10\x01\x12\t\n\x05WRITE\x10\x02\x12\x08\n\x04LOCK\x10\x03\x12\x08\n\
    \x04RAFT\x10\x042\xed\x02\n\x05Debug\x122\n\x03get\x12\x13.debugpb.GetRe\
    quest\x1a\x14.debugpb.GetResponse\"\0\x125\n\x04mvcc\x12\x14.debugpb.Mvc\
    cRequest\x1a\x15.debugpb.MvccResponse\"\0\x12?\n\x08raft_log\x12\x17.deb\
    ugpb.RaftLogRequest\x1a\x18.debugpb.RaftLogResponse\"\0\x12H\n\x0bregion\
    _info\x12\x1a.debugpb.RegionInfoRequest\x1a\x1b.debugpb.RegionInfoRespon\
    se\"\0\x125\n\x04size\x12\x14.debugpb.SizeRequest\x1a\x15.debugpb.SizeRe\
    sponse\"\0\x127\n\x04scan\x12\x14.debugpb.ScanRequest\x1a\x15.debugpb.Sc\
    anResponse\"\00\x01B&\n\x18com.pingcap.tikv.kvproto\xd0\xe2\x1e\x01\xc8\
    \xe2\x1e\x01\xe0\xe2\x1e\x01J\xd7!\n\x07\x12\x05\0\0\x82\x01\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x01\x08\x0f\n\t\n\x02\
    \x03\0\x12\x03\x03\x07\x16\n\t\n\x02\x03\x01\x12\x03\x04\x07\x16\n\t\n\
    \x02\x03\x02\x12\x03\x05\x07\x1c\n\t\n\x02\x03\x03\x12\x03\x06\x07\x1d\n\
    \x08\n\x01\x08\x12\x03\x08\0$\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x08\0$\n\
    \x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x08\x07\x1c\n\r\n\x06\x08\xe7\x07\0\
    \x02\0\x12\x03\x08\x07\x1c\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\
    \x08\x08\x1b\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\x08\x1f#\n\x08\n\x01\
    \x08\x12\x03\t\0(\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\t\0(\n\x0c\n\x05\
    \x08\xe7\x07\x01\x02\x12\x03\t\x07\x20\n\r\n\x06\x08\xe7\x07\x01\x02\0\
    \x12\x03\t\x07\x20\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\t\x08\
    \x1f\n\x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03\t#'\n\x08\n\x01\x08\x12\x03\
    \n\0*\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\n\0*\n\x0c\n\x05\x08\xe7\x07\
    \x02\x02\x12\x03\n\x07\"\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\n\x07\"\
    \n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\n\x08!\n\x0c\n\x05\x08\
    \xe7\x07\x02\x03\x12\x03\n%)\n\x08\n\x01\x08\x12\x03\x0c\01\n\x0b\n\x04\
    \x08\xe7\x07\x03\x12\x03\x0c\01\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\
    \x0c\x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\x0c\x07\x13\n\x0e\n\
    \x07\x08\xe7\x07\x03\x02\0\x01\x12\x03\x0c\x07\x13\n\x0c\n\x05\x08\xe7\
    \x07\x03\x07\x12\x03\x0c\x160\n\xe3\x02\n\x02\x06\0\x12\x04\x17\0)\x01\
    \x1a\xd6\x02\x20Debug\x20service\x20for\x20TiKV.\n\n\x20Errors\x20are\
    \x20defined\x20as\x20follow:\n\x20\x20\x20-\x20OK:\x20Okay,\x20we\x20are\
    \x20good!\n\x20\x20\x20-\x20UNKNOWN:\x20For\x20unknown\x20error.\n\x20\
    \x20\x20-\x20INVALID_ARGUMENT:\x20Something\x20goes\x20wrong\x20within\
    \x20requests.\n\x20\x20\x20-\x20NOT_FOUND:\x20It\x20is\x20key\x20or\x20r\
    egion\x20not\x20found,\x20it's\x20based\x20on\x20context,\x20detailed\n\
    \x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20reason\
    \x20can\x20be\x20found\x20in\x20grpc\x20message.\n\x20Note:\x20It\x20byp\
    asses\x20raft\x20layer.\n\n\n\n\x03\x06\0\x01\x12\x03\x17\x08\r\n:\n\x04\
    \x06\0\x02\0\x12\x03\x19\x040\x1a-\x20Read\x20a\x20value\x20arbitrarily\
    \x20for\x20a\x20encoded\x20key.\n\n\x0c\n\x05\x06\0\x02\0\x01\x12\x03\
    \x19\x08\x0b\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x19\x0c\x16\n\x0c\n\x05\
    \x06\0\x02\0\x03\x12\x03\x19!,\n6\n\x04\x06\0\x02\x01\x12\x03\x1c\x043\
    \x1a)\x20Read\x20MVCC\x20info\x20arbitrarily\x20for\x20raw\x20key.\n\n\
    \x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x1c\x08\x0c\n\x0c\n\x05\x06\0\x02\
    \x01\x02\x12\x03\x1c\r\x18\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x1c#/\n\
    \x1e\n\x04\x06\0\x02\x02\x12\x03\x1f\x04=\x1a\x11\x20Read\x20raft\x20inf\
    o.\n\n\x0c\n\x05\x06\0\x02\x02\x01\x12\x03\x1f\x08\x10\n\x0c\n\x05\x06\0\
    \x02\x02\x02\x12\x03\x1f\x11\x1f\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\
    \x1f*9\n\x0b\n\x04\x06\0\x02\x03\x12\x03\x20\x04F\n\x0c\n\x05\x06\0\x02\
    \x03\x01\x12\x03\x20\x08\x13\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\x20\
    \x14%\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03\x200B\nn\n\x04\x06\0\x02\x04\
    \x12\x03$\x043\x1aa\x20Calculate\x20size\x20of\x20a\x20region\x20or\x20a\
    \x20cf.\n\x20Note:\x20DO\x20NOT\x20CALL\x20IT\x20IN\x20PRODUCTION,\x20it\
    's\x20really\x20expensive.\n\n\x0c\n\x05\x06\0\x02\x04\x01\x12\x03$\x08\
    \x0c\n\x0c\n\x05\x06\0\x02\x04\x02\x12\x03$\r\x18\n\x0c\n\x05\x06\0\x02\
    \x04\x03\x12\x03$#/\na\n\x04\x06\0\x02\x05\x12\x03(\x04:\x1aT\x20Scan\
    \x20a\x20specific\x20range.\n\x20Note:\x20DO\x20NOT\x20CALL\x20IT\x20IN\
    \x20PRODUCTION,\x20it's\x20really\x20expensive.\n\n\x0c\n\x05\x06\0\x02\
    \x05\x01\x12\x03(\x08\x0c\n\x0c\n\x05\x06\0\x02\x05\x02\x12\x03(\r\x18\n\
    \x0c\n\x05\x06\0\x02\x05\x06\x12\x03(#)\n\x0c\n\x05\x06\0\x02\x05\x03\
    \x12\x03(*6\n\n\n\x02\x05\0\x12\x04+\01\x01\n\n\n\x03\x05\0\x01\x12\x03+\
    \x05\x07\n\x0b\n\x04\x05\0\x02\0\x12\x03,\x04\x10\n\x0c\n\x05\x05\0\x02\
    \0\x01\x12\x03,\x04\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\x03,\x0e\x0f\n\
    \x0b\n\x04\x05\0\x02\x01\x12\x03-\x04\x10\n\x0c\n\x05\x05\0\x02\x01\x01\
    \x12\x03-\x04\x0b\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03-\x0e\x0f\n\x0b\n\
    \x04\x05\0\x02\x02\x12\x03.\x04\x0e\n\x0c\n\x05\x05\0\x02\x02\x01\x12\
    \x03.\x04\t\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03.\x0c\r\n\x0b\n\x04\x05\
    \0\x02\x03\x12\x03/\x04\r\n\x0c\n\x05\x05\0\x02\x03\x01\x12\x03/\x04\x08\
    \n\x0c\n\x05\x05\0\x02\x03\x02\x12\x03/\x0b\x0c\n\x0b\n\x04\x05\0\x02\
    \x04\x12\x030\x04\r\n\x0c\n\x05\x05\0\x02\x04\x01\x12\x030\x04\x08\n\x0c\
    \n\x05\x05\0\x02\x04\x02\x12\x030\x0b\x0c\n\n\n\x02\x04\0\x12\x043\06\
    \x01\n\n\n\x03\x04\0\x01\x12\x033\x08\x12\n\x0b\n\x04\x04\0\x02\0\x12\
    \x034\x04\x0e\n\r\n\x05\x04\0\x02\0\x04\x12\x044\x043\x14\n\x0c\n\x05\
    \x04\0\x02\0\x06\x12\x034\x04\x06\n\x0c\n\x05\x04\0\x02\0\x01\x12\x034\
    \x07\t\n\x0c\n\x05\x04\0\x02\0\x03\x12\x034\x0c\r\n\x0b\n\x04\x04\0\x02\
    \x01\x12\x035\x04\x1a\n\r\n\x05\x04\0\x02\x01\x04\x12\x045\x044\x0e\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x035\x04\t\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x035\n\x15\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x035\x18\x19\n\n\n\
    \x02\x04\x01\x12\x048\0:\x01\n\n\n\x03\x04\x01\x01\x12\x038\x08\x13\n\
    \x0b\n\x04\x04\x01\x02\0\x12\x039\x04\x14\n\r\n\x05\x04\x01\x02\0\x04\
    \x12\x049\x048\x15\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x039\x04\t\n\x0c\n\
    \x05\x04\x01\x02\0\x01\x12\x039\n\x0f\n\x0c\n\x05\x04\x01\x02\0\x03\x12\
    \x039\x12\x13\n\n\n\x02\x04\x02\x12\x04<\0F\x01\n\n\n\x03\x04\x02\x01\
    \x12\x03<\x08\x13\n\x0c\n\x04\x04\x02\x04\0\x12\x04=\x04A\x05\n\x0c\n\
    \x05\x04\x02\x04\0\x01\x12\x03=\t\x0b\n\r\n\x06\x04\x02\x04\0\x02\0\x12\
    \x03>\x08\x14\n\x0e\n\x07\x04\x02\x04\0\x02\0\x01\x12\x03>\x08\x0f\n\x0e\
    \n\x07\x04\x02\x04\0\x02\0\x02\x12\x03>\x12\x13\n\r\n\x06\x04\x02\x04\0\
    \x02\x01\x12\x03?\x08\x10\n\x0e\n\x07\x04\x02\x04\0\x02\x01\x01\x12\x03?\
    \x08\x0b\n\x0e\n\x07\x04\x02\x04\0\x02\x01\x02\x12\x03?\x0e\x0f\n\r\n\
    \x06\x04\x02\x04\0\x02\x02\x12\x03@\x08\x15\n\x0e\n\x07\x04\x02\x04\0\
    \x02\x02\x01\x12\x03@\x08\x10\n\x0e\n\x07\x04\x02\x04\0\x02\x02\x02\x12\
    \x03@\x13\x14\n\x0b\n\x04\x04\x02\x02\0\x12\x03C\x04\x0e\n\r\n\x05\x04\
    \x02\x02\0\x04\x12\x04C\x04A\x05\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03C\
    \x04\x06\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03C\x07\t\n\x0c\n\x05\x04\
    \x02\x02\0\x03\x12\x03C\x0c\r\n\x0b\n\x04\x04\x02\x02\x01\x12\x03D\x04\
    \x13\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04D\x04C\x0e\n\x0c\n\x05\x04\x02\
    \x02\x01\x05\x12\x03D\x04\n\n\x0c\n\x05\x04\x02\x02\x01\x01\x12\x03D\x0b\
    \x0e\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\x03D\x11\x12\n\x0b\n\x04\x04\
    \x02\x02\x02\x12\x03E\x04\x18\n\r\n\x05\x04\x02\x02\x02\x04\x12\x04E\x04\
    D\x13\n\x0c\n\x05\x04\x02\x02\x02\x05\x12\x03E\x04\n\n\x0c\n\x05\x04\x02\
    \x02\x02\x01\x12\x03E\x0b\x13\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03E\
    \x16\x17\nk\n\x02\x04\x03\x12\x04L\0S\x01\x1a_\x20Missing\x20part:\n\x20\
    \x20\x201.\x20Lock\x20type\x20in\x20lock.\n\x20\x20\x202.\x20Short\x20va\
    lue\x20in\x20lock.\n\x20\x20\x203.\x20Short\x20value\x20in\x20write.\n\n\
    \n\n\x03\x04\x03\x01\x12\x03L\x08\x14\n\x0c\n\x04\x04\x03\x03\0\x12\x04M\
    \x04P\x05\n\x0c\n\x05\x04\x03\x03\0\x01\x12\x03M\x0c\x11\n\r\n\x06\x04\
    \x03\x03\0\x02\0\x12\x03N\x08\x17\n\x0f\n\x07\x04\x03\x03\0\x02\0\x04\
    \x12\x04N\x08M\x13\n\x0e\n\x07\x04\x03\x03\0\x02\0\x05\x12\x03N\x08\x0e\
    \n\x0e\n\x07\x04\x03\x03\0\x02\0\x01\x12\x03N\x0f\x12\n\x0e\n\x07\x04\
    \x03\x03\0\x02\0\x03\x12\x03N\x15\x16\n\r\n\x06\x04\x03\x03\0\x02\x01\
    \x12\x03O\x08\"\n\x0f\n\x07\x04\x03\x03\0\x02\x01\x04\x12\x04O\x08N\x17\
    \n\x0e\n\x07\x04\x03\x03\0\x02\x01\x06\x12\x03O\x08\x18\n\x0e\n\x07\x04\
    \x03\x03\0\x02\x01\x01\x12\x03O\x19\x1d\n\x0e\n\x07\x04\x03\x03\0\x02\
    \x01\x03\x12\x03O\x20!\n\x0b\n\x04\x04\x03\x02\0\x12\x03R\x04\x1f\n\x0c\
    \n\x05\x04\x03\x02\0\x04\x12\x03R\x04\x0c\n\x0c\n\x05\x04\x03\x02\0\x06\
    \x12\x03R\r\x12\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03R\x13\x1a\n\x0c\n\
    \x05\x04\x03\x02\0\x03\x12\x03R\x1d\x1e\n\n\n\x02\x04\x04\x12\x04U\0X\
    \x01\n\n\n\x03\x04\x04\x01\x12\x03U\x08\x16\n\x0b\n\x04\x04\x04\x02\0\
    \x12\x03V\x04\x19\n\r\n\x05\x04\x04\x02\0\x04\x12\x04V\x04U\x18\n\x0c\n\
    \x05\x04\x04\x02\0\x05\x12\x03V\x04\n\n\x0c\n\x05\x04\x04\x02\0\x01\x12\
    \x03V\x0b\x14\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03V\x17\x18\n\x0b\n\x04\
    \x04\x04\x02\x01\x12\x03W\x04\x19\n\r\n\x05\x04\x04\x02\x01\x04\x12\x04W\
    \x04V\x19\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03W\x04\n\n\x0c\n\x05\x04\
    \x04\x02\x01\x01\x12\x03W\x0b\x14\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\
    \x03W\x17\x18\n\n\n\x02\x04\x05\x12\x04Z\0\\\x01\n\n\n\x03\x04\x05\x01\
    \x12\x03Z\x08\x17\n\x0b\n\x04\x04\x05\x02\0\x12\x03[\x04\x1c\n\r\n\x05\
    \x04\x05\x02\0\x04\x12\x04[\x04Z\x19\n\x0c\n\x05\x04\x05\x02\0\x06\x12\
    \x03[\x04\x11\n\x0c\n\x05\x04\x05\x02\0\x01\x12\x03[\x12\x17\n\x0c\n\x05\
    \x04\x05\x02\0\x03\x12\x03[\x1a\x1b\n\n\n\x02\x04\x06\x12\x04^\0`\x01\n\
    \n\n\x03\x04\x06\x01\x12\x03^\x08\x19\n\x0b\n\x04\x04\x06\x02\0\x12\x03_\
    \x04\x19\n\r\n\x05\x04\x06\x02\0\x04\x12\x04_\x04^\x1b\n\x0c\n\x05\x04\
    \x06\x02\0\x05\x12\x03_\x04\n\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x03_\x0b\
    \x14\n\x0c\n\x05\x04\x06\x02\0\x03\x12\x03_\x17\x18\n\n\n\x02\x04\x07\
    \x12\x04b\0e\x01\n\n\n\x03\x04\x07\x01\x12\x03b\x08\x1a\n\x0b\n\x04\x04\
    \x07\x02\0\x12\x03c\x046\n\r\n\x05\x04\x07\x02\0\x04\x12\x04c\x04b\x1c\n\
    \x0c\n\x05\x04\x07\x02\0\x06\x12\x03c\x04\x20\n\x0c\n\x05\x04\x07\x02\0\
    \x01\x12\x03c!1\n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03c45\n\x0b\n\x04\x04\
    \x07\x02\x01\x12\x03d\x046\n\r\n\x05\x04\x07\x02\x01\x04\x12\x04d\x04c6\
    \n\x0c\n\x05\x04\x07\x02\x01\x06\x12\x03d\x04\x20\n\x0c\n\x05\x04\x07\
    \x02\x01\x01\x12\x03d!1\n\x0c\n\x05\x04\x07\x02\x01\x03\x12\x03d45\n\n\n\
    \x02\x04\x08\x12\x04g\0j\x01\n\n\n\x03\x04\x08\x01\x12\x03g\x08\x13\n\
    \x0b\n\x04\x04\x08\x02\0\x12\x03h\x04\x19\n\r\n\x05\x04\x08\x02\0\x04\
    \x12\x04h\x04g\x15\n\x0c\n\x05\x04\x08\x02\0\x05\x12\x03h\x04\n\n\x0c\n\
    \x05\x04\x08\x02\0\x01\x12\x03h\x0b\x14\n\x0c\n\x05\x04\x08\x02\0\x03\
    \x12\x03h\x17\x18\n\x0b\n\x04\x04\x08\x02\x01\x12\x03i\x04\x18\n\x0c\n\
    \x05\x04\x08\x02\x01\x04\x12\x03i\x04\x0c\n\x0c\n\x05\x04\x08\x02\x01\
    \x06\x12\x03i\r\x0f\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03i\x10\x13\n\
    \x0c\n\x05\x04\x08\x02\x01\x03\x12\x03i\x16\x17\n\n\n\x02\x04\t\x12\x04l\
    \0n\x01\n\n\n\x03\x04\t\x01\x12\x03l\x08\x14\n\x0b\n\x04\x04\t\x02\0\x12\
    \x03m\x04\x14\n\r\n\x05\x04\t\x02\0\x04\x12\x04m\x04l\x16\n\x0c\n\x05\
    \x04\t\x02\0\x05\x12\x03m\x04\n\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03m\x0b\
    \x0f\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03m\x12\x13\n\n\n\x02\x04\n\x12\
    \x04p\0}\x01\n\n\n\x03\x04\n\x01\x12\x03p\x08\x13\n$\n\x04\x04\n\x03\0\
    \x12\x04r\x04x\x05\x1a\x16\x20Filte\x20scan\x20response.\n\n\x0c\n\x05\
    \x04\n\x03\0\x01\x12\x03r\x0c\x12\n\r\n\x06\x04\n\x03\0\x02\0\x12\x03s\
    \x08\x19\n\x0f\n\x07\x04\n\x03\0\x02\0\x04\x12\x04s\x08r\x14\n\x0e\n\x07\
    \x04\n\x03\0\x02\0\x05\x12\x03s\x08\x0e\n\x0e\n\x07\x04\n\x03\0\x02\0\
    \x01\x12\x03s\x0f\x14\n\x0e\n\x07\x04\n\x03\0\x02\0\x03\x12\x03s\x17\x18\
    \n\r\n\x06\x04\n\x03\0\x02\x01\x12\x03t\x08\x1c\n\x0f\n\x07\x04\n\x03\0\
    \x02\x01\x04\x12\x04t\x08s\x19\n\x0e\n\x07\x04\n\x03\0\x02\x01\x05\x12\
    \x03t\x08\x0e\n\x0e\n\x07\x04\n\x03\0\x02\x01\x01\x12\x03t\x0f\x17\n\x0e\
    \n\x07\x04\n\x03\0\x02\x01\x03\x12\x03t\x1a\x1b\n\r\n\x06\x04\n\x03\0\
    \x02\x02\x12\x03u\x08\x1d\n\x0f\n\x07\x04\n\x03\0\x02\x02\x04\x12\x04u\
    \x08t\x1c\n\x0e\n\x07\x04\n\x03\0\x02\x02\x05\x12\x03u\x08\x0e\n\x0e\n\
    \x07\x04\n\x03\0\x02\x02\x01\x12\x03u\x0f\x18\n\x0e\n\x07\x04\n\x03\0\
    \x02\x02\x03\x12\x03u\x1b\x1c\n\r\n\x06\x04\n\x03\0\x02\x03\x12\x03v\x08\
    \x1a\n\x0f\n\x07\x04\n\x03\0\x02\x03\x04\x12\x04v\x08u\x1d\n\x0e\n\x07\
    \x04\n\x03\0\x02\x03\x05\x12\x03v\x08\x0c\n\x0e\n\x07\x04\n\x03\0\x02\
    \x03\x01\x12\x03v\r\x15\n\x0e\n\x07\x04\n\x03\0\x02\x03\x03\x12\x03v\x18\
    \x19\n\r\n\x06\x04\n\x03\0\x02\x04\x12\x03w\x08\x1c\n\x0e\n\x07\x04\n\
    \x03\0\x02\x04\x04\x12\x03w\x08\x10\n\x0e\n\x07\x04\n\x03\0\x02\x04\x06\
    \x12\x03w\x11\x13\n\x0e\n\x07\x04\n\x03\0\x02\x04\x01\x12\x03w\x14\x17\n\
    \x0e\n\x07\x04\n\x03\0\x02\x04\x03\x12\x03w\x1a\x1b\n\x0b\n\x04\x04\n\
    \x02\0\x12\x03z\x04\x1f\n\r\n\x05\x04\n\x02\0\x04\x12\x04z\x04x\x05\n\
    \x0c\n\x05\x04\n\x02\0\x05\x12\x03z\x04\t\n\x0c\n\x05\x04\n\x02\0\x01\
    \x12\x03z\n\x1a\n\x0c\n\x05\x04\n\x02\0\x03\x12\x03z\x1d\x1e\n\x0b\n\x04\
    \x04\n\x02\x01\x12\x03{\x04\x1d\n\r\n\x05\x04\n\x02\x01\x04\x12\x04{\x04\
    z\x1f\n\x0c\n\x05\x04\n\x02\x01\x05\x12\x03{\x04\t\n\x0c\n\x05\x04\n\x02\
    \x01\x01\x12\x03{\n\x18\n\x0c\n\x05\x04\n\x02\x01\x03\x12\x03{\x1b\x1c\n\
    \x0b\n\x04\x04\n\x02\x02\x12\x03|\x04\x16\n\r\n\x05\x04\n\x02\x02\x04\
    \x12\x04|\x04{\x1d\n\x0c\n\x05\x04\n\x02\x02\x06\x12\x03|\x04\n\n\x0c\n\
    \x05\x04\n\x02\x02\x01\x12\x03|\x0b\x11\n\x0c\n\x05\x04\n\x02\x02\x03\
    \x12\x03|\x14\x15\n\x0b\n\x02\x04\x0b\x12\x05\x7f\0\x82\x01\x01\n\n\n\
    \x03\x04\x0b\x01\x12\x03\x7f\x08\x14\n\x0c\n\x04\x04\x0b\x02\0\x12\x04\
    \x80\x01\x04\x1a\n\x0e\n\x05\x04\x0b\x02\0\x04\x12\x05\x80\x01\x04\x7f\
    \x16\n\r\n\x05\x04\x0b\x02\0\x05\x12\x04\x80\x01\x04\t\n\r\n\x05\x04\x0b\
    \x02\0\x01\x12\x04\x80\x01\n\x15\n\r\n\x05\x04\x0b\x02\0\x03\x12\x04\x80\
    \x01\x18\x19\n\x0c\n\x04\x04\x0b\x02\x01\x12\x04\x81\x01\x04\x1e\n\x0f\n\
    \x05\x04\x0b\x02\x01\x04\x12\x06\x81\x01\x04\x80\x01\x1a\n\r\n\x05\x04\
    \x0b\x02\x01\x06\x12\x04\x81\x01\x04\x14\n\r\n\x05\x04\x0b\x02\x01\x01\
    \x12\x04\x81\x01\x15\x19\n\r\n\x05\x04\x0b\x02\x01\x03\x12\x04\x81\x01\
    \x1c\x1db\x06proto3\
";

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
