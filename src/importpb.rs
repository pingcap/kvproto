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
pub struct SSTMeta {
    // message fields
    pub len: u64,
    pub crc32: u32,
    pub handle: ::protobuf::SingularPtrField<SSTHandle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSTMeta {}

impl SSTMeta {
    pub fn new() -> SSTMeta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSTMeta {
        static mut instance: ::protobuf::lazy::Lazy<SSTMeta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSTMeta,
        };
        unsafe {
            instance.get(SSTMeta::new)
        }
    }

    // uint64 len = 1;

    pub fn clear_len(&mut self) {
        self.len = 0;
    }

    // Param is passed by value, moved
    pub fn set_len(&mut self, v: u64) {
        self.len = v;
    }

    pub fn get_len(&self) -> u64 {
        self.len
    }

    fn get_len_for_reflect(&self) -> &u64 {
        &self.len
    }

    fn mut_len_for_reflect(&mut self) -> &mut u64 {
        &mut self.len
    }

    // uint32 crc32 = 2;

    pub fn clear_crc32(&mut self) {
        self.crc32 = 0;
    }

    // Param is passed by value, moved
    pub fn set_crc32(&mut self, v: u32) {
        self.crc32 = v;
    }

    pub fn get_crc32(&self) -> u32 {
        self.crc32
    }

    fn get_crc32_for_reflect(&self) -> &u32 {
        &self.crc32
    }

    fn mut_crc32_for_reflect(&mut self) -> &mut u32 {
        &mut self.crc32
    }

    // .importpb.SSTHandle handle = 3;

    pub fn clear_handle(&mut self) {
        self.handle.clear();
    }

    pub fn has_handle(&self) -> bool {
        self.handle.is_some()
    }

    // Param is passed by value, moved
    pub fn set_handle(&mut self, v: SSTHandle) {
        self.handle = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_handle(&mut self) -> &mut SSTHandle {
        if self.handle.is_none() {
            self.handle.set_default();
        }
        self.handle.as_mut().unwrap()
    }

    // Take field
    pub fn take_handle(&mut self) -> SSTHandle {
        self.handle.take().unwrap_or_else(|| SSTHandle::new())
    }

    pub fn get_handle(&self) -> &SSTHandle {
        self.handle.as_ref().unwrap_or_else(|| SSTHandle::default_instance())
    }

    fn get_handle_for_reflect(&self) -> &::protobuf::SingularPtrField<SSTHandle> {
        &self.handle
    }

    fn mut_handle_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SSTHandle> {
        &mut self.handle
    }
}

impl ::protobuf::Message for SSTMeta {
    fn is_initialized(&self) -> bool {
        for v in &self.handle {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.len = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.crc32 = tmp;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.handle)?;
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
        if self.len != 0 {
            my_size += ::protobuf::rt::value_size(1, self.len, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.crc32 != 0 {
            my_size += ::protobuf::rt::value_size(2, self.crc32, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.handle.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.len != 0 {
            os.write_uint64(1, self.len)?;
        }
        if self.crc32 != 0 {
            os.write_uint32(2, self.crc32)?;
        }
        if let Some(ref v) = self.handle.as_ref() {
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

impl ::protobuf::MessageStatic for SSTMeta {
    fn new() -> SSTMeta {
        SSTMeta::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSTMeta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "len",
                    SSTMeta::get_len_for_reflect,
                    SSTMeta::mut_len_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "crc32",
                    SSTMeta::get_crc32_for_reflect,
                    SSTMeta::mut_crc32_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SSTHandle>>(
                    "handle",
                    SSTMeta::get_handle_for_reflect,
                    SSTMeta::mut_handle_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSTMeta>(
                    "SSTMeta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSTMeta {
    fn clear(&mut self) {
        self.clear_len();
        self.clear_crc32();
        self.clear_handle();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SSTMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SSTMeta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SSTHandle {
    // message fields
    pub uuid: ::std::vec::Vec<u8>,
    pub cf_name: ::std::string::String,
    pub region_id: u64,
    pub region_epoch: ::protobuf::SingularPtrField<super::metapb::RegionEpoch>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSTHandle {}

impl SSTHandle {
    pub fn new() -> SSTHandle {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSTHandle {
        static mut instance: ::protobuf::lazy::Lazy<SSTHandle> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSTHandle,
        };
        unsafe {
            instance.get(SSTHandle::new)
        }
    }

    // bytes uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }

    fn get_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // string cf_name = 2;

    pub fn clear_cf_name(&mut self) {
        self.cf_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_cf_name(&mut self, v: ::std::string::String) {
        self.cf_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cf_name(&mut self) -> &mut ::std::string::String {
        &mut self.cf_name
    }

    // Take field
    pub fn take_cf_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cf_name, ::std::string::String::new())
    }

    pub fn get_cf_name(&self) -> &str {
        &self.cf_name
    }

    fn get_cf_name_for_reflect(&self) -> &::std::string::String {
        &self.cf_name
    }

    fn mut_cf_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.cf_name
    }

    // uint64 region_id = 3;

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

    // .metapb.RegionEpoch region_epoch = 4;

    pub fn clear_region_epoch(&mut self) {
        self.region_epoch.clear();
    }

    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_epoch(&mut self, v: super::metapb::RegionEpoch) {
        self.region_epoch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_epoch(&mut self) -> &mut super::metapb::RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch.set_default();
        }
        self.region_epoch.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_epoch(&mut self) -> super::metapb::RegionEpoch {
        self.region_epoch.take().unwrap_or_else(|| super::metapb::RegionEpoch::new())
    }

    pub fn get_region_epoch(&self) -> &super::metapb::RegionEpoch {
        self.region_epoch.as_ref().unwrap_or_else(|| super::metapb::RegionEpoch::default_instance())
    }

    fn get_region_epoch_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::RegionEpoch> {
        &self.region_epoch
    }

    fn mut_region_epoch_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::RegionEpoch> {
        &mut self.region_epoch
    }
}

impl ::protobuf::Message for SSTHandle {
    fn is_initialized(&self) -> bool {
        for v in &self.region_epoch {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.uuid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cf_name)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.region_id = tmp;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_epoch)?;
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
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.uuid);
        }
        if !self.cf_name.is_empty() {
            my_size += ::protobuf::rt::string_size(2, &self.cf_name);
        }
        if self.region_id != 0 {
            my_size += ::protobuf::rt::value_size(3, self.region_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.region_epoch.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uuid.is_empty() {
            os.write_bytes(1, &self.uuid)?;
        }
        if !self.cf_name.is_empty() {
            os.write_string(2, &self.cf_name)?;
        }
        if self.region_id != 0 {
            os.write_uint64(3, self.region_id)?;
        }
        if let Some(ref v) = self.region_epoch.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SSTHandle {
    fn new() -> SSTHandle {
        SSTHandle::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSTHandle>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    SSTHandle::get_uuid_for_reflect,
                    SSTHandle::mut_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cf_name",
                    SSTHandle::get_cf_name_for_reflect,
                    SSTHandle::mut_cf_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "region_id",
                    SSTHandle::get_region_id_for_reflect,
                    SSTHandle::mut_region_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::RegionEpoch>>(
                    "region_epoch",
                    SSTHandle::get_region_epoch_for_reflect,
                    SSTHandle::mut_region_epoch_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSTHandle>(
                    "SSTHandle",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSTHandle {
    fn clear(&mut self) {
        self.clear_uuid();
        self.clear_cf_name();
        self.clear_region_id();
        self.clear_region_epoch();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SSTHandle {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SSTHandle {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngestSSTRequest {
    // message fields
    pub context: ::protobuf::SingularPtrField<super::kvrpcpb::Context>,
    pub handles: ::protobuf::RepeatedField<SSTHandle>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngestSSTRequest {}

impl IngestSSTRequest {
    pub fn new() -> IngestSSTRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngestSSTRequest {
        static mut instance: ::protobuf::lazy::Lazy<IngestSSTRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngestSSTRequest,
        };
        unsafe {
            instance.get(IngestSSTRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: super::kvrpcpb::Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut super::kvrpcpb::Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> super::kvrpcpb::Context {
        self.context.take().unwrap_or_else(|| super::kvrpcpb::Context::new())
    }

    pub fn get_context(&self) -> &super::kvrpcpb::Context {
        self.context.as_ref().unwrap_or_else(|| super::kvrpcpb::Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<super::kvrpcpb::Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::kvrpcpb::Context> {
        &mut self.context
    }

    // repeated .importpb.SSTHandle handles = 2;

    pub fn clear_handles(&mut self) {
        self.handles.clear();
    }

    // Param is passed by value, moved
    pub fn set_handles(&mut self, v: ::protobuf::RepeatedField<SSTHandle>) {
        self.handles = v;
    }

    // Mutable pointer to the field.
    pub fn mut_handles(&mut self) -> &mut ::protobuf::RepeatedField<SSTHandle> {
        &mut self.handles
    }

    // Take field
    pub fn take_handles(&mut self) -> ::protobuf::RepeatedField<SSTHandle> {
        ::std::mem::replace(&mut self.handles, ::protobuf::RepeatedField::new())
    }

    pub fn get_handles(&self) -> &[SSTHandle] {
        &self.handles
    }

    fn get_handles_for_reflect(&self) -> &::protobuf::RepeatedField<SSTHandle> {
        &self.handles
    }

    fn mut_handles_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SSTHandle> {
        &mut self.handles
    }
}

impl ::protobuf::Message for IngestSSTRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.handles {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.handles)?;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        for value in &self.handles {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        for v in &self.handles {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for IngestSSTRequest {
    fn new() -> IngestSSTRequest {
        IngestSSTRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngestSSTRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::kvrpcpb::Context>>(
                    "context",
                    IngestSSTRequest::get_context_for_reflect,
                    IngestSSTRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SSTHandle>>(
                    "handles",
                    IngestSSTRequest::get_handles_for_reflect,
                    IngestSSTRequest::mut_handles_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngestSSTRequest>(
                    "IngestSSTRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngestSSTRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_handles();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngestSSTRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngestSSTRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngestSSTResponse {
    // message fields
    pub error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngestSSTResponse {}

impl IngestSSTResponse {
    pub fn new() -> IngestSSTResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngestSSTResponse {
        static mut instance: ::protobuf::lazy::Lazy<IngestSSTResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngestSSTResponse,
        };
        unsafe {
            instance.get(IngestSSTResponse::new)
        }
    }

    // .errorpb.Error error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: super::errorpb::Error) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut super::errorpb::Error {
        if self.error.is_none() {
            self.error.set_default();
        }
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> super::errorpb::Error {
        self.error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_error(&self) -> &super::errorpb::Error {
        self.error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    fn get_error_for_reflect(&self) -> &::protobuf::SingularPtrField<super::errorpb::Error> {
        &self.error
    }

    fn mut_error_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::errorpb::Error> {
        &mut self.error
    }
}

impl ::protobuf::Message for IngestSSTResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
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
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.error.as_ref() {
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

impl ::protobuf::MessageStatic for IngestSSTResponse {
    fn new() -> IngestSSTResponse {
        IngestSSTResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngestSSTResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "error",
                    IngestSSTResponse::get_error_for_reflect,
                    IngestSSTResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngestSSTResponse>(
                    "IngestSSTResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngestSSTResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngestSSTResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngestSSTResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UploadSSTRequest {
    // message fields
    pub meta: ::protobuf::SingularPtrField<SSTMeta>,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UploadSSTRequest {}

impl UploadSSTRequest {
    pub fn new() -> UploadSSTRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UploadSSTRequest {
        static mut instance: ::protobuf::lazy::Lazy<UploadSSTRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UploadSSTRequest,
        };
        unsafe {
            instance.get(UploadSSTRequest::new)
        }
    }

    // .importpb.SSTMeta meta = 1;

    pub fn clear_meta(&mut self) {
        self.meta.clear();
    }

    pub fn has_meta(&self) -> bool {
        self.meta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meta(&mut self, v: SSTMeta) {
        self.meta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meta(&mut self) -> &mut SSTMeta {
        if self.meta.is_none() {
            self.meta.set_default();
        }
        self.meta.as_mut().unwrap()
    }

    // Take field
    pub fn take_meta(&mut self) -> SSTMeta {
        self.meta.take().unwrap_or_else(|| SSTMeta::new())
    }

    pub fn get_meta(&self) -> &SSTMeta {
        self.meta.as_ref().unwrap_or_else(|| SSTMeta::default_instance())
    }

    fn get_meta_for_reflect(&self) -> &::protobuf::SingularPtrField<SSTMeta> {
        &self.meta
    }

    fn mut_meta_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SSTMeta> {
        &mut self.meta
    }

    // bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.data
    }
}

impl ::protobuf::Message for UploadSSTRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.meta {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meta)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.data)?;
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
        if let Some(ref v) = self.meta.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.meta.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
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

impl ::protobuf::MessageStatic for UploadSSTRequest {
    fn new() -> UploadSSTRequest {
        UploadSSTRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UploadSSTRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SSTMeta>>(
                    "meta",
                    UploadSSTRequest::get_meta_for_reflect,
                    UploadSSTRequest::mut_meta_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    UploadSSTRequest::get_data_for_reflect,
                    UploadSSTRequest::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UploadSSTRequest>(
                    "UploadSSTRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UploadSSTRequest {
    fn clear(&mut self) {
        self.clear_meta();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UploadSSTRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UploadSSTRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UploadSSTResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UploadSSTResponse {}

impl UploadSSTResponse {
    pub fn new() -> UploadSSTResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UploadSSTResponse {
        static mut instance: ::protobuf::lazy::Lazy<UploadSSTResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UploadSSTResponse,
        };
        unsafe {
            instance.get(UploadSSTResponse::new)
        }
    }
}

impl ::protobuf::Message for UploadSSTResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
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
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
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

impl ::protobuf::MessageStatic for UploadSSTResponse {
    fn new() -> UploadSSTResponse {
        UploadSSTResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UploadSSTResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<UploadSSTResponse>(
                    "UploadSSTResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UploadSSTResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UploadSSTResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UploadSSTResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eimportpb.proto\x12\x08importpb\x1a\x0cmetapb.proto\x1a\rkvrpcpb.pr\
    oto\x1a\rerrorpb.proto\x1a\x14gogoproto/gogo.proto\"^\n\x07SSTMeta\x12\
    \x10\n\x03len\x18\x01\x20\x01(\x04R\x03len\x12\x14\n\x05crc32\x18\x02\
    \x20\x01(\rR\x05crc32\x12+\n\x06handle\x18\x03\x20\x01(\x0b2\x13.importp\
    b.SSTHandleR\x06handle\"\x8d\x01\n\tSSTHandle\x12\x12\n\x04uuid\x18\x01\
    \x20\x01(\x0cR\x04uuid\x12\x17\n\x07cf_name\x18\x02\x20\x01(\tR\x06cfNam\
    e\x12\x1b\n\tregion_id\x18\x03\x20\x01(\x04R\x08regionId\x126\n\x0cregio\
    n_epoch\x18\x04\x20\x01(\x0b2\x13.metapb.RegionEpochR\x0bregionEpoch\"m\
    \n\x10IngestSSTRequest\x12*\n\x07context\x18\x01\x20\x01(\x0b2\x10.kvrpc\
    pb.ContextR\x07context\x12-\n\x07handles\x18\x02\x20\x03(\x0b2\x13.impor\
    tpb.SSTHandleR\x07handles\"9\n\x11IngestSSTResponse\x12$\n\x05error\x18\
    \x01\x20\x01(\x0b2\x0e.errorpb.ErrorR\x05error\"M\n\x10UploadSSTRequest\
    \x12%\n\x04meta\x18\x01\x20\x01(\x0b2\x11.importpb.SSTMetaR\x04meta\x12\
    \x12\n\x04data\x18\x02\x20\x01(\x0cR\x04data\"\x13\n\x11UploadSSTRespons\
    e2\x9a\x01\n\x06Import\x12F\n\tIngestSST\x12\x1a.importpb.IngestSSTReque\
    st\x1a\x1b.importpb.IngestSSTResponse\"\0\x12H\n\tUploadSST\x12\x1a.impo\
    rtpb.UploadSSTRequest\x1a\x1b.importpb.UploadSSTResponse\"\0(\x01B&\n\
    \x18com.pingcap.tikv.kvproto\xe0\xe2\x1e\x01\xc8\xe2\x1e\x01\xd0\xe2\x1e\
    \x01J\xb7\x0f\n\x06\x12\x04\0\09\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\
    \x08\n\x01\x02\x12\x03\x02\x08\x10\n\t\n\x02\x03\0\x12\x03\x04\x07\x15\n\
    \t\n\x02\x03\x01\x12\x03\x05\x07\x16\n\t\n\x02\x03\x02\x12\x03\x06\x07\
    \x16\n\t\n\x02\x03\x03\x12\x03\x07\x07\x1d\n\x08\n\x01\x08\x12\x03\t\0$\
    \n\x0b\n\x04\x08\xe7\x07\0\x12\x03\t\0$\n\x0c\n\x05\x08\xe7\x07\0\x02\
    \x12\x03\t\x07\x1c\n\r\n\x06\x08\xe7\x07\0\x02\0\x12\x03\t\x07\x1c\n\x0e\
    \n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\t\x08\x1b\n\x0c\n\x05\x08\xe7\x07\
    \0\x03\x12\x03\t\x1f#\n\x08\n\x01\x08\x12\x03\n\0(\n\x0b\n\x04\x08\xe7\
    \x07\x01\x12\x03\n\0(\n\x0c\n\x05\x08\xe7\x07\x01\x02\x12\x03\n\x07\x20\
    \n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\x03\n\x07\x20\n\x0e\n\x07\x08\xe7\
    \x07\x01\x02\0\x01\x12\x03\n\x08\x1f\n\x0c\n\x05\x08\xe7\x07\x01\x03\x12\
    \x03\n#'\n\x08\n\x01\x08\x12\x03\x0b\0*\n\x0b\n\x04\x08\xe7\x07\x02\x12\
    \x03\x0b\0*\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\x03\x0b\x07\"\n\r\n\x06\
    \x08\xe7\x07\x02\x02\0\x12\x03\x0b\x07\"\n\x0e\n\x07\x08\xe7\x07\x02\x02\
    \0\x01\x12\x03\x0b\x08!\n\x0c\n\x05\x08\xe7\x07\x02\x03\x12\x03\x0b%)\n\
    \x08\n\x01\x08\x12\x03\r\01\n\x0b\n\x04\x08\xe7\x07\x03\x12\x03\r\01\n\
    \x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\r\x07\x13\n\r\n\x06\x08\xe7\x07\
    \x03\x02\0\x12\x03\r\x07\x13\n\x0e\n\x07\x08\xe7\x07\x03\x02\0\x01\x12\
    \x03\r\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\x07\x12\x03\r\x160\n\n\n\x02\
    \x06\0\x12\x04\x0f\0\x12\x01\n\n\n\x03\x06\0\x01\x12\x03\x0f\x08\x0e\n\
    \x0b\n\x04\x06\0\x02\0\x12\x03\x10\x04B\n\x0c\n\x05\x06\0\x02\0\x01\x12\
    \x03\x10\x08\x11\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x10\x12\"\n\x0c\n\
    \x05\x06\0\x02\0\x03\x12\x03\x10->\n\x0b\n\x04\x06\0\x02\x01\x12\x03\x11\
    \x04I\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x11\x08\x11\n\x0c\n\x05\x06\
    \0\x02\x01\x05\x12\x03\x11\x12\x18\n\x0c\n\x05\x06\0\x02\x01\x02\x12\x03\
    \x11\x19)\n\x0c\n\x05\x06\0\x02\x01\x03\x12\x03\x114E\n\n\n\x02\x04\0\
    \x12\x04\x14\0\x1b\x01\n\n\n\x03\x04\0\x01\x12\x03\x14\x08\x0f\n$\n\x04\
    \x04\0\x02\0\x12\x03\x16\x04\x13\x1a\x17\x20The\x20size\x20of\x20the\x20\
    file.\n\n\r\n\x05\x04\0\x02\0\x04\x12\x04\x16\x04\x14\x11\n\x0c\n\x05\
    \x04\0\x02\0\x05\x12\x03\x16\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\
    \x16\x0b\x0e\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x16\x11\x12\n.\n\x04\
    \x04\0\x02\x01\x12\x03\x18\x04\x15\x1a!\x20The\x20CRC32\x20checksum\x20o\
    f\x20the\x20file.\n\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x18\x04\x16\x13\
    \n\x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x18\x04\n\n\x0c\n\x05\x04\0\x02\
    \x01\x01\x12\x03\x18\x0b\x10\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x18\
    \x13\x14\n4\n\x04\x04\0\x02\x02\x12\x03\x1a\x04\x19\x1a'\x20The\x20handl\
    e\x20which\x20identifies\x20the\x20file.\n\n\r\n\x05\x04\0\x02\x02\x04\
    \x12\x04\x1a\x04\x18\x15\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x1a\x04\r\
    \n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x1a\x0e\x14\n\x0c\n\x05\x04\0\x02\
    \x02\x03\x12\x03\x1a\x17\x18\n\n\n\x02\x04\x01\x12\x04\x1d\0&\x01\n\n\n\
    \x03\x04\x01\x01\x12\x03\x1d\x08\x11\nM\n\x04\x04\x01\x02\0\x12\x03\x1f\
    \x04\x13\x1a@\x20The\x20UUID\x20of\x20the\x20file,\x20to\x20distinguish\
    \x20files\x20with\x20the\x20same\x20data.\n\n\r\n\x05\x04\x01\x02\0\x04\
    \x12\x04\x1f\x04\x1d\x13\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x1f\x04\t\
    \n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1f\n\x0e\n\x0c\n\x05\x04\x01\x02\
    \0\x03\x12\x03\x1f\x11\x12\n9\n\x04\x04\x01\x02\x01\x12\x03!\x04\x17\x1a\
    ,\x20The\x20CF\x20that\x20this\x20file\x20will\x20be\x20ingested\x20to.\
    \n\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04!\x04\x1f\x13\n\x0c\n\x05\x04\
    \x01\x02\x01\x05\x12\x03!\x04\n\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03!\
    \x0b\x12\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03!\x15\x16\n=\n\x04\x04\
    \x01\x02\x02\x12\x03#\x04\x19\x1a0\x20The\x20region\x20that\x20this\x20f\
    ile\x20will\x20be\x20ingested\x20to.\n\n\r\n\x05\x04\x01\x02\x02\x04\x12\
    \x04#\x04!\x17\n\x0c\n\x05\x04\x01\x02\x02\x05\x12\x03#\x04\n\n\x0c\n\
    \x05\x04\x01\x02\x02\x01\x12\x03#\x0b\x14\n\x0c\n\x05\x04\x01\x02\x02\
    \x03\x12\x03#\x17\x18\nB\n\x04\x04\x01\x02\x03\x12\x03%\x04(\x1a5\x20The\
    \x20epoch\x20of\x20the\x20region\x20when\x20this\x20file\x20is\x20upload\
    ed.\n\n\r\n\x05\x04\x01\x02\x03\x04\x12\x04%\x04#\x19\n\x0c\n\x05\x04\
    \x01\x02\x03\x06\x12\x03%\x04\x16\n\x0c\n\x05\x04\x01\x02\x03\x01\x12\
    \x03%\x17#\n\x0c\n\x05\x04\x01\x02\x03\x03\x12\x03%&'\n\n\n\x02\x04\x02\
    \x12\x04(\0-\x01\n\n\n\x03\x04\x02\x01\x12\x03(\x08\x18\n\x0b\n\x04\x04\
    \x02\x02\0\x12\x03)\x04\x20\n\r\n\x05\x04\x02\x02\0\x04\x12\x04)\x04(\
    \x1a\n\x0c\n\x05\x04\x02\x02\0\x06\x12\x03)\x04\x13\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x03)\x14\x1b\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03)\x1e\
    \x1f\n\x88\x01\n\x04\x04\x02\x02\x01\x12\x03,\x04#\x1a{\x20The\x20handle\
    s\x20of\x20the\x20files\x20that\x20will\x20be\x20ingested.\n\x20Only\x20\
    files\x20of\x20the\x20same\x20column\x20family\x20can\x20be\x20ingested\
    \x20atomically\x20for\x20now.\n\n\x0c\n\x05\x04\x02\x02\x01\x04\x12\x03,\
    \x04\x0c\n\x0c\n\x05\x04\x02\x02\x01\x06\x12\x03,\r\x16\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03,\x17\x1e\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03,!\"\n\n\n\x02\x04\x03\x12\x04/\01\x01\n\n\n\x03\x04\x03\x01\x12\x03\
    /\x08\x19\n\x0b\n\x04\x04\x03\x02\0\x12\x030\x04\x1c\n\r\n\x05\x04\x03\
    \x02\0\x04\x12\x040\x04/\x1b\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x030\x04\
    \x11\n\x0c\n\x05\x04\x03\x02\0\x01\x12\x030\x12\x17\n\x0c\n\x05\x04\x03\
    \x02\0\x03\x12\x030\x1a\x1b\n\n\n\x02\x04\x04\x12\x043\06\x01\n\n\n\x03\
    \x04\x04\x01\x12\x033\x08\x18\n\x0b\n\x04\x04\x04\x02\0\x12\x034\x04\x15\
    \n\r\n\x05\x04\x04\x02\0\x04\x12\x044\x043\x1a\n\x0c\n\x05\x04\x04\x02\0\
    \x06\x12\x034\x04\x0b\n\x0c\n\x05\x04\x04\x02\0\x01\x12\x034\x0c\x10\n\
    \x0c\n\x05\x04\x04\x02\0\x03\x12\x034\x13\x14\n\x0b\n\x04\x04\x04\x02\
    \x01\x12\x035\x04\x15\n\r\n\x05\x04\x04\x02\x01\x04\x12\x045\x044\x15\n\
    \x0c\n\x05\x04\x04\x02\x01\x05\x12\x035\x04\t\n\x0c\n\x05\x04\x04\x02\
    \x01\x01\x12\x035\x0c\x10\n\x0c\n\x05\x04\x04\x02\x01\x03\x12\x035\x13\
    \x14\n\n\n\x02\x04\x05\x12\x048\09\x01\n\n\n\x03\x04\x05\x01\x12\x038\
    \x08\x19b\x06proto3\
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
