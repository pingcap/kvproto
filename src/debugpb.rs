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
    pub db: DB,
    pub cf: ::std::string::String,
    pub key: ::std::vec::Vec<u8>,
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

    // .debugpb.DB db = 1;

    pub fn clear_db(&mut self) {
        self.db = DB::INVALID;
    }

    // Param is passed by value, moved
    pub fn set_db(&mut self, v: DB) {
        self.db = v;
    }

    pub fn get_db(&self) -> DB {
        self.db
    }

    fn get_db_for_reflect(&self) -> &DB {
        &self.db
    }

    fn mut_db_for_reflect(&mut self) -> &mut DB {
        &mut self.db
    }

    // string cf = 2;

    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }

    // Param is passed by value, moved
    pub fn set_cf(&mut self, v: ::std::string::String) {
        self.cf = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cf(&mut self) -> &mut ::std::string::String {
        &mut self.cf
    }

    // Take field
    pub fn take_cf(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }

    pub fn get_cf(&self) -> &str {
        &self.cf
    }

    fn get_cf_for_reflect(&self) -> &::std::string::String {
        &self.cf
    }

    fn mut_cf_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.cf
    }

    // bytes key = 3;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
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
                    self.db = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cf)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
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
        if self.db != DB::INVALID {
            my_size += ::protobuf::rt::enum_size(1, self.db);
        }
        if !self.cf.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.cf);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.db != DB::INVALID {
            os.write_enum(1, self.db.value())?;
        }
        if !self.cf.is_empty() {
            os.write_string(2, &self.cf)?;
        }
        if !self.key.is_empty() {
            os.write_bytes(3, &self.key)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<DB>>(
                    "db",
                    GetRequest::get_db_for_reflect,
                    GetRequest::mut_db_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cf",
                    GetRequest::get_cf_for_reflect,
                    GetRequest::mut_cf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    GetRequest::get_key_for_reflect,
                    GetRequest::mut_key_for_reflect,
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
        self.clear_db();
        self.clear_cf();
        self.clear_key();
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
    pub region_local_state: ::protobuf::SingularPtrField<super::raft_serverpb::RegionLocalState>,
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

    // .raft_serverpb.RegionLocalState region_local_state = 3;

    pub fn clear_region_local_state(&mut self) {
        self.region_local_state.clear();
    }

    pub fn has_region_local_state(&self) -> bool {
        self.region_local_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_local_state(&mut self, v: super::raft_serverpb::RegionLocalState) {
        self.region_local_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_local_state(&mut self) -> &mut super::raft_serverpb::RegionLocalState {
        if self.region_local_state.is_none() {
            self.region_local_state.set_default();
        }
        self.region_local_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_local_state(&mut self) -> super::raft_serverpb::RegionLocalState {
        self.region_local_state.take().unwrap_or_else(|| super::raft_serverpb::RegionLocalState::new())
    }

    pub fn get_region_local_state(&self) -> &super::raft_serverpb::RegionLocalState {
        self.region_local_state.as_ref().unwrap_or_else(|| super::raft_serverpb::RegionLocalState::default_instance())
    }

    fn get_region_local_state_for_reflect(&self) -> &::protobuf::SingularPtrField<super::raft_serverpb::RegionLocalState> {
        &self.region_local_state
    }

    fn mut_region_local_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::raft_serverpb::RegionLocalState> {
        &mut self.region_local_state
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
        for v in &self.region_local_state {
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
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_local_state)?;
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
        if let Some(ref v) = self.region_local_state.as_ref() {
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
        if let Some(ref v) = self.region_local_state.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::raft_serverpb::RegionLocalState>>(
                    "region_local_state",
                    RegionInfoResponse::get_region_local_state_for_reflect,
                    RegionInfoResponse::mut_region_local_state_for_reflect,
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
        self.clear_region_local_state();
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
pub struct RegionSizeRequest {
    // message fields
    pub region_id: u64,
    pub cfs: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionSizeRequest {}

impl RegionSizeRequest {
    pub fn new() -> RegionSizeRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionSizeRequest {
        static mut instance: ::protobuf::lazy::Lazy<RegionSizeRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionSizeRequest,
        };
        unsafe {
            instance.get(RegionSizeRequest::new)
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

    // repeated string cfs = 2;

    pub fn clear_cfs(&mut self) {
        self.cfs.clear();
    }

    // Param is passed by value, moved
    pub fn set_cfs(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.cfs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cfs(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.cfs
    }

    // Take field
    pub fn take_cfs(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.cfs, ::protobuf::RepeatedField::new())
    }

    pub fn get_cfs(&self) -> &[::std::string::String] {
        &self.cfs
    }

    fn get_cfs_for_reflect(&self) -> &::protobuf::RepeatedField<::std::string::String> {
        &self.cfs
    }

    fn mut_cfs_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.cfs
    }
}

impl ::protobuf::Message for RegionSizeRequest {
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
                    ::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.cfs)?;
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
            my_size += ::protobuf::rt::string_size(2, &value);
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
            os.write_string(2, &v)?;
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

impl ::protobuf::MessageStatic for RegionSizeRequest {
    fn new() -> RegionSizeRequest {
        RegionSizeRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionSizeRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "region_id",
                    RegionSizeRequest::get_region_id_for_reflect,
                    RegionSizeRequest::mut_region_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cfs",
                    RegionSizeRequest::get_cfs_for_reflect,
                    RegionSizeRequest::mut_cfs_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionSizeRequest>(
                    "RegionSizeRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionSizeRequest {
    fn clear(&mut self) {
        self.clear_region_id();
        self.clear_cfs();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegionSizeRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionSizeRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegionSizeResponse {
    // message fields
    pub entries: ::protobuf::RepeatedField<RegionSizeResponse_Entry>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionSizeResponse {}

impl RegionSizeResponse {
    pub fn new() -> RegionSizeResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionSizeResponse {
        static mut instance: ::protobuf::lazy::Lazy<RegionSizeResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionSizeResponse,
        };
        unsafe {
            instance.get(RegionSizeResponse::new)
        }
    }

    // repeated .debugpb.RegionSizeResponse.Entry entries = 1;

    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }

    // Param is passed by value, moved
    pub fn set_entries(&mut self, v: ::protobuf::RepeatedField<RegionSizeResponse_Entry>) {
        self.entries = v;
    }

    // Mutable pointer to the field.
    pub fn mut_entries(&mut self) -> &mut ::protobuf::RepeatedField<RegionSizeResponse_Entry> {
        &mut self.entries
    }

    // Take field
    pub fn take_entries(&mut self) -> ::protobuf::RepeatedField<RegionSizeResponse_Entry> {
        ::std::mem::replace(&mut self.entries, ::protobuf::RepeatedField::new())
    }

    pub fn get_entries(&self) -> &[RegionSizeResponse_Entry] {
        &self.entries
    }

    fn get_entries_for_reflect(&self) -> &::protobuf::RepeatedField<RegionSizeResponse_Entry> {
        &self.entries
    }

    fn mut_entries_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<RegionSizeResponse_Entry> {
        &mut self.entries
    }
}

impl ::protobuf::Message for RegionSizeResponse {
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

impl ::protobuf::MessageStatic for RegionSizeResponse {
    fn new() -> RegionSizeResponse {
        RegionSizeResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionSizeResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RegionSizeResponse_Entry>>(
                    "entries",
                    RegionSizeResponse::get_entries_for_reflect,
                    RegionSizeResponse::mut_entries_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionSizeResponse>(
                    "RegionSizeResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionSizeResponse {
    fn clear(&mut self) {
        self.clear_entries();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegionSizeResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionSizeResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegionSizeResponse_Entry {
    // message fields
    pub cf: ::std::string::String,
    pub size: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionSizeResponse_Entry {}

impl RegionSizeResponse_Entry {
    pub fn new() -> RegionSizeResponse_Entry {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionSizeResponse_Entry {
        static mut instance: ::protobuf::lazy::Lazy<RegionSizeResponse_Entry> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionSizeResponse_Entry,
        };
        unsafe {
            instance.get(RegionSizeResponse_Entry::new)
        }
    }

    // string cf = 1;

    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }

    // Param is passed by value, moved
    pub fn set_cf(&mut self, v: ::std::string::String) {
        self.cf = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cf(&mut self) -> &mut ::std::string::String {
        &mut self.cf
    }

    // Take field
    pub fn take_cf(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }

    pub fn get_cf(&self) -> &str {
        &self.cf
    }

    fn get_cf_for_reflect(&self) -> &::std::string::String {
        &self.cf
    }

    fn mut_cf_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.cf
    }

    // uint64 size = 2;

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

impl ::protobuf::Message for RegionSizeResponse_Entry {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cf)?;
                },
                2 => {
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
        if !self.cf.is_empty() {
            my_size += ::protobuf::rt::string_size(1, &self.cf);
        }
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.size, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.cf.is_empty() {
            os.write_string(1, &self.cf)?;
        }
        if self.size != 0 {
            os.write_uint64(2, self.size)?;
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

impl ::protobuf::MessageStatic for RegionSizeResponse_Entry {
    fn new() -> RegionSizeResponse_Entry {
        RegionSizeResponse_Entry::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionSizeResponse_Entry>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cf",
                    RegionSizeResponse_Entry::get_cf_for_reflect,
                    RegionSizeResponse_Entry::mut_cf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "size",
                    RegionSizeResponse_Entry::get_size_for_reflect,
                    RegionSizeResponse_Entry::mut_size_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionSizeResponse_Entry>(
                    "RegionSizeResponse_Entry",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionSizeResponse_Entry {
    fn clear(&mut self) {
        self.clear_cf();
        self.clear_size();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegionSizeResponse_Entry {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionSizeResponse_Entry {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanMvccRequest {
    // message fields
    pub from_key: ::std::vec::Vec<u8>,
    pub to_key: ::std::vec::Vec<u8>,
    pub limit: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanMvccRequest {}

impl ScanMvccRequest {
    pub fn new() -> ScanMvccRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanMvccRequest {
        static mut instance: ::protobuf::lazy::Lazy<ScanMvccRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanMvccRequest,
        };
        unsafe {
            instance.get(ScanMvccRequest::new)
        }
    }

    // bytes from_key = 1;

    pub fn clear_from_key(&mut self) {
        self.from_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_from_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.from_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.from_key
    }

    // Take field
    pub fn take_from_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.from_key, ::std::vec::Vec::new())
    }

    pub fn get_from_key(&self) -> &[u8] {
        &self.from_key
    }

    fn get_from_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.from_key
    }

    fn mut_from_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.from_key
    }

    // bytes to_key = 2;

    pub fn clear_to_key(&mut self) {
        self.to_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_to_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.to_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.to_key
    }

    // Take field
    pub fn take_to_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.to_key, ::std::vec::Vec::new())
    }

    pub fn get_to_key(&self) -> &[u8] {
        &self.to_key
    }

    fn get_to_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.to_key
    }

    fn mut_to_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.to_key
    }

    // uint64 limit = 3;

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
}

impl ::protobuf::Message for ScanMvccRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.from_key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.to_key)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.limit = tmp;
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
        if !self.from_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.from_key);
        }
        if !self.to_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.to_key);
        }
        if self.limit != 0 {
            my_size += ::protobuf::rt::value_size(3, self.limit, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.from_key.is_empty() {
            os.write_bytes(1, &self.from_key)?;
        }
        if !self.to_key.is_empty() {
            os.write_bytes(2, &self.to_key)?;
        }
        if self.limit != 0 {
            os.write_uint64(3, self.limit)?;
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

impl ::protobuf::MessageStatic for ScanMvccRequest {
    fn new() -> ScanMvccRequest {
        ScanMvccRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanMvccRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "from_key",
                    ScanMvccRequest::get_from_key_for_reflect,
                    ScanMvccRequest::mut_from_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "to_key",
                    ScanMvccRequest::get_to_key_for_reflect,
                    ScanMvccRequest::mut_to_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "limit",
                    ScanMvccRequest::get_limit_for_reflect,
                    ScanMvccRequest::mut_limit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanMvccRequest>(
                    "ScanMvccRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanMvccRequest {
    fn clear(&mut self) {
        self.clear_from_key();
        self.clear_to_key();
        self.clear_limit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanMvccRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanMvccRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ScanMvccResponse {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub info: ::protobuf::SingularPtrField<super::kvrpcpb::MvccInfo>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ScanMvccResponse {}

impl ScanMvccResponse {
    pub fn new() -> ScanMvccResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ScanMvccResponse {
        static mut instance: ::protobuf::lazy::Lazy<ScanMvccResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ScanMvccResponse,
        };
        unsafe {
            instance.get(ScanMvccResponse::new)
        }
    }

    // bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
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

impl ::protobuf::Message for ScanMvccResponse {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
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
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
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
            os.write_bytes(1, &self.key)?;
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

impl ::protobuf::MessageStatic for ScanMvccResponse {
    fn new() -> ScanMvccResponse {
        ScanMvccResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ScanMvccResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    ScanMvccResponse::get_key_for_reflect,
                    ScanMvccResponse::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::kvrpcpb::MvccInfo>>(
                    "info",
                    ScanMvccResponse::get_info_for_reflect,
                    ScanMvccResponse::mut_info_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ScanMvccResponse>(
                    "ScanMvccResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ScanMvccResponse {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_info();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ScanMvccResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ScanMvccResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum DB {
    INVALID = 0,
    KV = 1,
    RAFT = 2,
}

impl ::protobuf::ProtobufEnum for DB {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<DB> {
        match value {
            0 => ::std::option::Option::Some(DB::INVALID),
            1 => ::std::option::Option::Some(DB::KV),
            2 => ::std::option::Option::Some(DB::RAFT),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [DB] = &[
            DB::INVALID,
            DB::KV,
            DB::RAFT,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<DB>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("DB", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for DB {
}

impl ::std::default::Default for DB {
    fn default() -> Self {
        DB::INVALID
    }
}

impl ::protobuf::reflect::ProtobufValue for DB {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\rdebugpb.proto\x12\x07debugpb\x1a\reraftpb.proto\x1a\rkvrpcpb.proto\
    \x1a\x13raft_serverpb.proto\x1a\x14gogoproto/gogo.proto\"K\n\nGetRequest\
    \x12\x1b\n\x02db\x18\x01\x20\x01(\x0e2\x0b.debugpb.DBR\x02db\x12\x0e\n\
    \x02cf\x18\x02\x20\x01(\tR\x02cf\x12\x10\n\x03key\x18\x03\x20\x01(\x0cR\
    \x03key\"#\n\x0bGetResponse\x12\x14\n\x05value\x18\x01\x20\x01(\x0cR\x05\
    value\"J\n\x0eRaftLogRequest\x12\x1b\n\tregion_id\x18\x01\x20\x01(\x04R\
    \x08regionId\x12\x1b\n\tlog_index\x18\x02\x20\x01(\x04R\x08logIndex\"7\n\
    \x0fRaftLogResponse\x12$\n\x05entry\x18\x01\x20\x01(\x0b2\x0e.eraftpb.En\
    tryR\x05entry\"0\n\x11RegionInfoRequest\x12\x1b\n\tregion_id\x18\x01\x20\
    \x01(\x04R\x08regionId\"\xf5\x01\n\x12RegionInfoResponse\x12G\n\x10raft_\
    local_state\x18\x01\x20\x01(\x0b2\x1d.raft_serverpb.RaftLocalStateR\x0er\
    aftLocalState\x12G\n\x10raft_apply_state\x18\x02\x20\x01(\x0b2\x1d.raft_\
    serverpb.RaftApplyStateR\x0eraftApplyState\x12M\n\x12region_local_state\
    \x18\x03\x20\x01(\x0b2\x1f.raft_serverpb.RegionLocalStateR\x10regionLoca\
    lState\"B\n\x11RegionSizeRequest\x12\x1b\n\tregion_id\x18\x01\x20\x01(\
    \x04R\x08regionId\x12\x10\n\x03cfs\x18\x02\x20\x03(\tR\x03cfs\"~\n\x12Re\
    gionSizeResponse\x12;\n\x07entries\x18\x01\x20\x03(\x0b2!.debugpb.Region\
    SizeResponse.EntryR\x07entries\x1a+\n\x05Entry\x12\x0e\n\x02cf\x18\x01\
    \x20\x01(\tR\x02cf\x12\x12\n\x04size\x18\x02\x20\x01(\x04R\x04size\"Y\n\
    \x0fScanMvccRequest\x12\x19\n\x08from_key\x18\x01\x20\x01(\x0cR\x07fromK\
    ey\x12\x15\n\x06to_key\x18\x02\x20\x01(\x0cR\x05toKey\x12\x14\n\x05limit\
    \x18\x03\x20\x01(\x04R\x05limit\"K\n\x10ScanMvccResponse\x12\x10\n\x03ke\
    y\x18\x01\x20\x01(\x0cR\x03key\x12%\n\x04info\x18\x02\x20\x01(\x0b2\x11.\
    kvrpcpb.MvccInfoR\x04info*#\n\x02DB\x12\x0b\n\x07INVALID\x10\0\x12\x06\n\
    \x02KV\x10\x01\x12\x08\n\x04RAFT\x10\x022\xd6\x02\n\x05Debug\x122\n\x03g\
    et\x12\x13.debugpb.GetRequest\x1a\x14.debugpb.GetResponse\"\0\x12?\n\x08\
    raft_log\x12\x17.debugpb.RaftLogRequest\x1a\x18.debugpb.RaftLogResponse\
    \"\0\x12H\n\x0bregion_info\x12\x1a.debugpb.RegionInfoRequest\x1a\x1b.deb\
    ugpb.RegionInfoResponse\"\0\x12H\n\x0bregion_size\x12\x1a.debugpb.Region\
    SizeRequest\x1a\x1b.debugpb.RegionSizeResponse\"\0\x12D\n\tscan_mvcc\x12\
    \x18.debugpb.ScanMvccRequest\x1a\x19.debugpb.ScanMvccResponse\"\00\x01B&\
    \n\x18com.pingcap.tikv.kvproto\xc8\xe2\x1e\x01\xe0\xe2\x1e\x01\xd0\xe2\
    \x1e\x01J\xe4\x19\n\x06\x12\x04\0\0d\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\
    \n\x08\n\x01\x02\x12\x03\x01\x08\x0f\n\t\n\x02\x03\0\x12\x03\x03\x07\x16\
    \n\t\n\x02\x03\x01\x12\x03\x04\x07\x16\n\t\n\x02\x03\x02\x12\x03\x05\x07\
    \x1c\n\t\n\x02\x03\x03\x12\x03\x06\x07\x1d\n\x08\n\x01\x08\x12\x03\x08\0\
    $\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x08\0$\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\x08\x07\x1c\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\x08\x07\x1c\n\
    \x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\x08\x08\x1b\n\x0c\n\x05\x08\
    \xe7\x07\0\x03\x12\x03\x08\x1f#\n\x08\n\x01\x08\x12\x03\t\0(\n\x0b\n\x04\
    \x08\xe7\x07\x01\x12\x03\t\0(\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\t\
    \x07\x20\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\t\x07\x20\n\x0e\n\x07\
    \x08\xe7\x07\x01\x02\0\x01\x12\x03\t\x08\x1f\n\x0c\n\x05\x08\xe7\x07\x01\
    \x03\x12\x03\t#'\n\x08\n\x01\x08\x12\x03\n\0*\n\x0b\n\x04\x08\xe7\x07\
    \x02\x12\x03\n\0*\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\n\x07\"\n\r\n\
    \x06\x08\xe7\x07\x02\x02\0\x12\x03\n\x07\"\n\x0e\n\x07\x08\xe7\x07\x02\
    \x02\0\x01\x12\x03\n\x08!\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\n%)\n\
    \x08\n\x01\x08\x12\x03\x0c\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\x0c\01\
    \n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\x0c\x07\x13\n\r\n\x06\x08\xe7\
    \x07\x03\x02\0\x12\x03\x0c\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\
    \x12\x03\x0c\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\x0c\x160\n\
    \xe3\x02\n\x02\x06\0\x12\x04\x17\0(\x01\x1a\xd6\x02\x20Debug\x20service\
    \x20for\x20TiKV.\n\n\x20Errors\x20are\x20defined\x20as\x20follow:\n\x20\
    \x20\x20-\x20OK:\x20Okay,\x20we\x20are\x20good!\n\x20\x20\x20-\x20UNKNOW\
    N:\x20For\x20unknown\x20error.\n\x20\x20\x20-\x20INVALID_ARGUMENT:\x20So\
    mething\x20goes\x20wrong\x20within\x20requests.\n\x20\x20\x20-\x20NOT_FO\
    UND:\x20It\x20is\x20key\x20or\x20region\x20not\x20found,\x20it's\x20base\
    d\x20on\x20context,\x20detailed\n\x20\x20\x20\x20\x20\x20\x20\x20\x20\
    \x20\x20\x20\x20\x20\x20\x20reason\x20can\x20be\x20found\x20in\x20grpc\
    \x20message.\n\x20Note:\x20It\x20bypasses\x20raft\x20layer.\n\n\n\n\x03\
    \x06\0\x01\x12\x03\x17\x08\r\nd\n\x04\x06\0\x02\0\x12\x03\x1a\x040\x1aW\
    \x20Read\x20a\x20value\x20arbitrarily\x20for\x20a\x20key.\n\x20Note:\x20\
    Server\x20uses\x20key\x20directly\x20w/o\x20any\x20encoding.\n\n\x0c\n\
    \x05\x06\0\x02\0\x01\x12\x03\x1a\x08\x0b\n\x0c\n\x05\x06\0\x02\0\x02\x12\
    \x03\x1a\x0c\x16\n\x0c\n\x05\x06\0\x02\0\x03\x12\x03\x1a!,\n\x1e\n\x04\
    \x06\0\x02\x01\x12\x03\x1d\x04=\x1a\x11\x20Read\x20raft\x20info.\n\n\x0c\
    \n\x05\x06\0\x02\x01\x01\x12\x03\x1d\x08\x10\n\x0c\n\x05\x06\0\x02\x01\
    \x02\x12\x03\x1d\x11\x1f\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x1d*9\n\
    \x0b\n\x04\x06\0\x02\x02\x12\x03\x1e\x04F\n\x0c\n\x05\x06\0\x02\x02\x01\
    \x12\x03\x1e\x08\x13\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03\x1e\x14%\n\
    \x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x1e0B\nf\n\x04\x06\0\x02\x03\x12\
    \x03\"\x04F\x1aY\x20Calculate\x20size\x20of\x20a\x20region.\n\x20Note:\
    \x20DO\x20NOT\x20CALL\x20IT\x20IN\x20PRODUCTION,\x20it's\x20really\x20ex\
    pensive.\n\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03\"\x08\x13\n\x0c\n\x05\
    \x06\0\x02\x03\x02\x12\x03\"\x14%\n\x0c\n\x05\x06\0\x02\x03\x03\x12\x03\
    \"0B\n\x95\x01\n\x04\x06\0\x02\x04\x12\x03'\x04G\x1a\x87\x01\x20Scan\x20\
    a\x20specific\x20range.\n\x20Note:\x20DO\x20NOT\x20CALL\x20IT\x20IN\x20P\
    RODUCTION,\x20it's\x20really\x20expensive.\n\x20\x20\x20\x20\x20\x20\x20\
    Server\x20uses\x20keys\x20directly\x20w/o\x20any\x20encoding.\n\n\x0c\n\
    \x05\x06\0\x02\x04\x01\x12\x03'\x08\x11\n\x0c\n\x05\x06\0\x02\x04\x02\
    \x12\x03'\x12!\n\x0c\n\x05\x06\0\x02\x04\x06\x12\x03',2\n\x0c\n\x05\x06\
    \0\x02\x04\x03\x12\x03'3C\n\n\n\x02\x05\0\x12\x04*\0.\x01\n\n\n\x03\x05\
    \0\x01\x12\x03*\x05\x07\n\x0b\n\x04\x05\0\x02\0\x12\x03+\x04\x10\n\x0c\n\
    \x05\x05\0\x02\0\x01\x12\x03+\x04\x0b\n\x0c\n\x05\x05\0\x02\0\x02\x12\
    \x03+\x0e\x0f\n\x0b\n\x04\x05\0\x02\x01\x12\x03,\x04\x0b\n\x0c\n\x05\x05\
    \0\x02\x01\x01\x12\x03,\x04\x06\n\x0c\n\x05\x05\0\x02\x01\x02\x12\x03,\t\
    \n\n\x0b\n\x04\x05\0\x02\x02\x12\x03-\x04\r\n\x0c\n\x05\x05\0\x02\x02\
    \x01\x12\x03-\x04\x08\n\x0c\n\x05\x05\0\x02\x02\x02\x12\x03-\x0b\x0c\n\n\
    \n\x02\x04\0\x12\x040\04\x01\n\n\n\x03\x04\0\x01\x12\x030\x08\x12\n\x0b\
    \n\x04\x04\0\x02\0\x12\x031\x04\x0e\n\r\n\x05\x04\0\x02\0\x04\x12\x041\
    \x040\x14\n\x0c\n\x05\x04\0\x02\0\x06\x12\x031\x04\x06\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x031\x07\t\n\x0c\n\x05\x04\0\x02\0\x03\x12\x031\x0c\r\n\
    \x0b\n\x04\x04\0\x02\x01\x12\x032\x04\x12\n\r\n\x05\x04\0\x02\x01\x04\
    \x12\x042\x041\x0e\n\x0c\n\x05\x04\0\x02\x01\x05\x12\x032\x04\n\n\x0c\n\
    \x05\x04\0\x02\x01\x01\x12\x032\x0b\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\
    \x032\x10\x11\n\x0b\n\x04\x04\0\x02\x02\x12\x033\x04\x12\n\r\n\x05\x04\0\
    \x02\x02\x04\x12\x043\x042\x12\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x033\
    \x04\t\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x033\n\r\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x033\x10\x11\n\n\n\x02\x04\x01\x12\x046\08\x01\n\n\n\x03\
    \x04\x01\x01\x12\x036\x08\x13\n\x0b\n\x04\x04\x01\x02\0\x12\x037\x04\x14\
    \n\r\n\x05\x04\x01\x02\0\x04\x12\x047\x046\x15\n\x0c\n\x05\x04\x01\x02\0\
    \x05\x12\x037\x04\t\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x037\n\x0f\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x037\x12\x13\n\n\n\x02\x04\x02\x12\x04:\0=\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03:\x08\x16\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03;\x04\x19\n\r\n\x05\x04\x02\x02\0\x04\x12\x04;\x04:\x18\n\x0c\n\
    \x05\x04\x02\x02\0\x05\x12\x03;\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\
    \x03;\x0b\x14\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03;\x17\x18\n\x0b\n\x04\
    \x04\x02\x02\x01\x12\x03<\x04\x19\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04<\
    \x04;\x19\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03<\x04\n\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03<\x0b\x14\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03<\x17\x18\n\n\n\x02\x04\x03\x12\x04?\0A\x01\n\n\n\x03\x04\x03\x01\
    \x12\x03?\x08\x17\n\x0b\n\x04\x04\x03\x02\0\x12\x03@\x04\x1c\n\r\n\x05\
    \x04\x03\x02\0\x04\x12\x04@\x04?\x19\n\x0c\n\x05\x04\x03\x02\0\x06\x12\
    \x03@\x04\x11\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x03@\x12\x17\n\x0c\n\x05\
    \x04\x03\x02\0\x03\x12\x03@\x1a\x1b\n\n\n\x02\x04\x04\x12\x04C\0E\x01\n\
    \n\n\x03\x04\x04\x01\x12\x03C\x08\x19\n\x0b\n\x04\x04\x04\x02\0\x12\x03D\
    \x04\x19\n\r\n\x05\x04\x04\x02\0\x04\x12\x04D\x04C\x1b\n\x0c\n\x05\x04\
    \x04\x02\0\x05\x12\x03D\x04\n\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x03D\x0b\
    \x14\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03D\x17\x18\n\n\n\x02\x04\x05\
    \x12\x04G\0K\x01\n\n\n\x03\x04\x05\x01\x12\x03G\x08\x1a\n\x0b\n\x04\x04\
    \x05\x02\0\x12\x03H\x046\n\r\n\x05\x04\x05\x02\0\x04\x12\x04H\x04G\x1c\n\
    \x0c\n\x05\x04\x05\x02\0\x06\x12\x03H\x04\x20\n\x0c\n\x05\x04\x05\x02\0\
    \x01\x12\x03H!1\n\x0c\n\x05\x04\x05\x02\0\x03\x12\x03H45\n\x0b\n\x04\x04\
    \x05\x02\x01\x12\x03I\x046\n\r\n\x05\x04\x05\x02\x01\x04\x12\x04I\x04H6\
    \n\x0c\n\x05\x04\x05\x02\x01\x06\x12\x03I\x04\x20\n\x0c\n\x05\x04\x05\
    \x02\x01\x01\x12\x03I!1\n\x0c\n\x05\x04\x05\x02\x01\x03\x12\x03I45\n\x0b\
    \n\x04\x04\x05\x02\x02\x12\x03J\x04:\n\r\n\x05\x04\x05\x02\x02\x04\x12\
    \x04J\x04I6\n\x0c\n\x05\x04\x05\x02\x02\x06\x12\x03J\x04\"\n\x0c\n\x05\
    \x04\x05\x02\x02\x01\x12\x03J#5\n\x0c\n\x05\x04\x05\x02\x02\x03\x12\x03J\
    89\n\n\n\x02\x04\x06\x12\x04M\0P\x01\n\n\n\x03\x04\x06\x01\x12\x03M\x08\
    \x19\n\x0b\n\x04\x04\x06\x02\0\x12\x03N\x04\x19\n\r\n\x05\x04\x06\x02\0\
    \x04\x12\x04N\x04M\x1b\n\x0c\n\x05\x04\x06\x02\0\x05\x12\x03N\x04\n\n\
    \x0c\n\x05\x04\x06\x02\0\x01\x12\x03N\x0b\x14\n\x0c\n\x05\x04\x06\x02\0\
    \x03\x12\x03N\x17\x18\n\x0b\n\x04\x04\x06\x02\x01\x12\x03O\x04\x1c\n\x0c\
    \n\x05\x04\x06\x02\x01\x04\x12\x03O\x04\x0c\n\x0c\n\x05\x04\x06\x02\x01\
    \x05\x12\x03O\r\x13\n\x0c\n\x05\x04\x06\x02\x01\x01\x12\x03O\x14\x17\n\
    \x0c\n\x05\x04\x06\x02\x01\x03\x12\x03O\x1a\x1b\n\n\n\x02\x04\x07\x12\
    \x04R\0Y\x01\n\n\n\x03\x04\x07\x01\x12\x03R\x08\x1a\n\x0c\n\x04\x04\x07\
    \x03\0\x12\x04S\x04V\x05\n\x0c\n\x05\x04\x07\x03\0\x01\x12\x03S\x0c\x11\
    \n\r\n\x06\x04\x07\x03\0\x02\0\x12\x03T\x08\x16\n\x0f\n\x07\x04\x07\x03\
    \0\x02\0\x04\x12\x04T\x08S\x13\n\x0e\n\x07\x04\x07\x03\0\x02\0\x05\x12\
    \x03T\x08\x0e\n\x0e\n\x07\x04\x07\x03\0\x02\0\x01\x12\x03T\x0f\x11\n\x0e\
    \n\x07\x04\x07\x03\0\x02\0\x03\x12\x03T\x14\x15\n\r\n\x06\x04\x07\x03\0\
    \x02\x01\x12\x03U\x08\x18\n\x0f\n\x07\x04\x07\x03\0\x02\x01\x04\x12\x04U\
    \x08T\x16\n\x0e\n\x07\x04\x07\x03\0\x02\x01\x05\x12\x03U\x08\x0e\n\x0e\n\
    \x07\x04\x07\x03\0\x02\x01\x01\x12\x03U\x0f\x13\n\x0e\n\x07\x04\x07\x03\
    \0\x02\x01\x03\x12\x03U\x16\x17\n\x0b\n\x04\x04\x07\x02\0\x12\x03X\x04\
    \x1f\n\x0c\n\x05\x04\x07\x02\0\x04\x12\x03X\x04\x0c\n\x0c\n\x05\x04\x07\
    \x02\0\x06\x12\x03X\r\x12\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03X\x13\x1a\
    \n\x0c\n\x05\x04\x07\x02\0\x03\x12\x03X\x1d\x1e\n\n\n\x02\x04\x08\x12\
    \x04[\0_\x01\n\n\n\x03\x04\x08\x01\x12\x03[\x08\x17\n\x0b\n\x04\x04\x08\
    \x02\0\x12\x03\\\x04\x17\n\r\n\x05\x04\x08\x02\0\x04\x12\x04\\\x04[\x19\
    \n\x0c\n\x05\x04\x08\x02\0\x05\x12\x03\\\x04\t\n\x0c\n\x05\x04\x08\x02\0\
    \x01\x12\x03\\\n\x12\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03\\\x15\x16\n\
    \x0b\n\x04\x04\x08\x02\x01\x12\x03]\x04\x15\n\r\n\x05\x04\x08\x02\x01\
    \x04\x12\x04]\x04\\\x17\n\x0c\n\x05\x04\x08\x02\x01\x05\x12\x03]\x04\t\n\
    \x0c\n\x05\x04\x08\x02\x01\x01\x12\x03]\n\x10\n\x0c\n\x05\x04\x08\x02\
    \x01\x03\x12\x03]\x13\x14\n\x0b\n\x04\x04\x08\x02\x02\x12\x03^\x04\x15\n\
    \r\n\x05\x04\x08\x02\x02\x04\x12\x04^\x04]\x15\n\x0c\n\x05\x04\x08\x02\
    \x02\x05\x12\x03^\x04\n\n\x0c\n\x05\x04\x08\x02\x02\x01\x12\x03^\x0b\x10\
    \n\x0c\n\x05\x04\x08\x02\x02\x03\x12\x03^\x13\x14\n\n\n\x02\x04\t\x12\
    \x04a\0d\x01\n\n\n\x03\x04\t\x01\x12\x03a\x08\x18\n\x0b\n\x04\x04\t\x02\
    \0\x12\x03b\x04\x12\n\r\n\x05\x04\t\x02\0\x04\x12\x04b\x04a\x1a\n\x0c\n\
    \x05\x04\t\x02\0\x05\x12\x03b\x04\t\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03b\
    \n\r\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03b\x10\x11\n\x0b\n\x04\x04\t\x02\
    \x01\x12\x03c\x04\x1e\n\r\n\x05\x04\t\x02\x01\x04\x12\x04c\x04b\x12\n\
    \x0c\n\x05\x04\t\x02\x01\x06\x12\x03c\x04\x14\n\x0c\n\x05\x04\t\x02\x01\
    \x01\x12\x03c\x15\x19\n\x0c\n\x05\x04\t\x02\x01\x03\x12\x03c\x1c\x1db\
    \x06proto3\
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
