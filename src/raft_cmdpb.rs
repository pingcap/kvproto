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
    cf: ::protobuf::SingularField<::std::string::String>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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

    // optional string cf = 1;

    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }

    pub fn has_cf(&self) -> bool {
        self.cf.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cf(&mut self, v: ::std::string::String) {
        self.cf = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cf(&mut self) -> &mut ::std::string::String {
        if self.cf.is_none() {
            self.cf.set_default();
        };
        self.cf.as_mut().unwrap()
    }

    // Take field
    pub fn take_cf(&mut self) -> ::std::string::String {
        self.cf.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cf(&self) -> &str {
        match self.cf.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_cf_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cf
    }

    fn mut_cf_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cf
    }

    // optional bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
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
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cf)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
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
        if let Some(v) = self.cf.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cf.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cf",
                    GetRequest::get_cf_for_reflect,
                    GetRequest::mut_cf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
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
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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

    // optional bytes value = 1;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
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
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(1, &v)?;
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
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
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
pub struct PutRequest {
    // message fields
    cf: ::protobuf::SingularField<::std::string::String>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PutRequest {}

impl PutRequest {
    pub fn new() -> PutRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutRequest {
        static mut instance: ::protobuf::lazy::Lazy<PutRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutRequest,
        };
        unsafe {
            instance.get(PutRequest::new)
        }
    }

    // optional string cf = 1;

    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }

    pub fn has_cf(&self) -> bool {
        self.cf.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cf(&mut self, v: ::std::string::String) {
        self.cf = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cf(&mut self) -> &mut ::std::string::String {
        if self.cf.is_none() {
            self.cf.set_default();
        };
        self.cf.as_mut().unwrap()
    }

    // Take field
    pub fn take_cf(&mut self) -> ::std::string::String {
        self.cf.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cf(&self) -> &str {
        match self.cf.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_cf_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cf
    }

    fn mut_cf_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cf
    }

    // optional bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional bytes value = 3;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for PutRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cf)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(v) = self.cf.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cf.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
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

impl ::protobuf::MessageStatic for PutRequest {
    fn new() -> PutRequest {
        PutRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cf",
                    PutRequest::get_cf_for_reflect,
                    PutRequest::mut_cf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    PutRequest::get_key_for_reflect,
                    PutRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    PutRequest::get_value_for_reflect,
                    PutRequest::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PutRequest>(
                    "PutRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutRequest {
    fn clear(&mut self) {
        self.clear_cf();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PutRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PutRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PutResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PutResponse {}

impl PutResponse {
    pub fn new() -> PutResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutResponse {
        static mut instance: ::protobuf::lazy::Lazy<PutResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutResponse,
        };
        unsafe {
            instance.get(PutResponse::new)
        }
    }
}

impl ::protobuf::Message for PutResponse {
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

impl ::protobuf::MessageStatic for PutResponse {
    fn new() -> PutResponse {
        PutResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PutResponse>(
                    "PutResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PutResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PutResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteRequest {
    // message fields
    cf: ::protobuf::SingularField<::std::string::String>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteRequest {}

impl DeleteRequest {
    pub fn new() -> DeleteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteRequest {
        static mut instance: ::protobuf::lazy::Lazy<DeleteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteRequest,
        };
        unsafe {
            instance.get(DeleteRequest::new)
        }
    }

    // optional string cf = 1;

    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }

    pub fn has_cf(&self) -> bool {
        self.cf.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cf(&mut self, v: ::std::string::String) {
        self.cf = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cf(&mut self) -> &mut ::std::string::String {
        if self.cf.is_none() {
            self.cf.set_default();
        };
        self.cf.as_mut().unwrap()
    }

    // Take field
    pub fn take_cf(&mut self) -> ::std::string::String {
        self.cf.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cf(&self) -> &str {
        match self.cf.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    fn get_cf_for_reflect(&self) -> &::protobuf::SingularField<::std::string::String> {
        &self.cf
    }

    fn mut_cf_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::string::String> {
        &mut self.cf
    }

    // optional bytes key = 2;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }
}

impl ::protobuf::Message for DeleteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cf)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
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
        if let Some(v) = self.cf.as_ref() {
            my_size += ::protobuf::rt::string_size(1, &v);
        };
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cf.as_ref() {
            os.write_string(1, &v)?;
        };
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for DeleteRequest {
    fn new() -> DeleteRequest {
        DeleteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cf",
                    DeleteRequest::get_cf_for_reflect,
                    DeleteRequest::mut_cf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    DeleteRequest::get_key_for_reflect,
                    DeleteRequest::mut_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<DeleteRequest>(
                    "DeleteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteRequest {
    fn clear(&mut self) {
        self.clear_cf();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for DeleteResponse {}

impl DeleteResponse {
    pub fn new() -> DeleteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static DeleteResponse {
        static mut instance: ::protobuf::lazy::Lazy<DeleteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const DeleteResponse,
        };
        unsafe {
            instance.get(DeleteResponse::new)
        }
    }
}

impl ::protobuf::Message for DeleteResponse {
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

impl ::protobuf::MessageStatic for DeleteResponse {
    fn new() -> DeleteResponse {
        DeleteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<DeleteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<DeleteResponse>(
                    "DeleteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for DeleteResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for DeleteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for DeleteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapRequest {}

impl SnapRequest {
    pub fn new() -> SnapRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapRequest {
        static mut instance: ::protobuf::lazy::Lazy<SnapRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapRequest,
        };
        unsafe {
            instance.get(SnapRequest::new)
        }
    }
}

impl ::protobuf::Message for SnapRequest {
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

impl ::protobuf::MessageStatic for SnapRequest {
    fn new() -> SnapRequest {
        SnapRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<SnapRequest>(
                    "SnapRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapResponse {
    // message fields
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapResponse {}

impl SnapResponse {
    pub fn new() -> SnapResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapResponse {
        static mut instance: ::protobuf::lazy::Lazy<SnapResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapResponse,
        };
        unsafe {
            instance.get(SnapResponse::new)
        }
    }

    // optional .metapb.Region region = 1;

    pub fn clear_region(&mut self) {
        self.region.clear();
    }

    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region.set_default();
        };
        self.region.as_mut().unwrap()
    }

    // Take field
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region.take().unwrap_or_else(|| super::metapb::Region::new())
    }

    pub fn get_region(&self) -> &super::metapb::Region {
        self.region.as_ref().unwrap_or_else(|| super::metapb::Region::default_instance())
    }

    fn get_region_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Region> {
        &self.region
    }

    fn mut_region_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Region> {
        &mut self.region
    }
}

impl ::protobuf::Message for SnapResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region)?;
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
        if let Some(v) = self.region.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region.as_ref() {
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

impl ::protobuf::MessageStatic for SnapResponse {
    fn new() -> SnapResponse {
        SnapResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Region>>(
                    "region",
                    SnapResponse::get_region_for_reflect,
                    SnapResponse::mut_region_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapResponse>(
                    "SnapResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapResponse {
    fn clear(&mut self) {
        self.clear_region();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PrewriteRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrewriteRequest {}

impl PrewriteRequest {
    pub fn new() -> PrewriteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrewriteRequest {
        static mut instance: ::protobuf::lazy::Lazy<PrewriteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrewriteRequest,
        };
        unsafe {
            instance.get(PrewriteRequest::new)
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }

    // optional bytes lock = 3;

    pub fn clear_lock(&mut self) {
        self.lock.clear();
    }

    pub fn has_lock(&self) -> bool {
        self.lock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lock(&mut self, v: ::std::vec::Vec<u8>) {
        self.lock = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lock(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.lock.is_none() {
            self.lock.set_default();
        };
        self.lock.as_mut().unwrap()
    }

    // Take field
    pub fn take_lock(&mut self) -> ::std::vec::Vec<u8> {
        self.lock.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_lock(&self) -> &[u8] {
        match self.lock.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_lock_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.lock
    }

    fn mut_lock_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.lock
    }
}

impl ::protobuf::Message for PrewriteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.lock)?;
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
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.lock.as_ref() {
            my_size += ::protobuf::rt::bytes_size(3, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.lock.as_ref() {
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

impl ::protobuf::MessageStatic for PrewriteRequest {
    fn new() -> PrewriteRequest {
        PrewriteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrewriteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    PrewriteRequest::get_key_for_reflect,
                    PrewriteRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    PrewriteRequest::get_value_for_reflect,
                    PrewriteRequest::mut_value_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "lock",
                    PrewriteRequest::get_lock_for_reflect,
                    PrewriteRequest::mut_lock_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PrewriteRequest>(
                    "PrewriteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrewriteRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.clear_lock();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PrewriteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PrewriteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct PrewriteResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PrewriteResponse {}

impl PrewriteResponse {
    pub fn new() -> PrewriteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PrewriteResponse {
        static mut instance: ::protobuf::lazy::Lazy<PrewriteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PrewriteResponse,
        };
        unsafe {
            instance.get(PrewriteResponse::new)
        }
    }
}

impl ::protobuf::Message for PrewriteResponse {
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

impl ::protobuf::MessageStatic for PrewriteResponse {
    fn new() -> PrewriteResponse {
        PrewriteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PrewriteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PrewriteResponse>(
                    "PrewriteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PrewriteResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for PrewriteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for PrewriteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommitRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommitRequest {}

impl CommitRequest {
    pub fn new() -> CommitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommitRequest {
        static mut instance: ::protobuf::lazy::Lazy<CommitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommitRequest,
        };
        unsafe {
            instance.get(CommitRequest::new)
        }
    }

    // optional bytes key = 1;

    pub fn clear_key(&mut self) {
        self.key.clear();
    }

    pub fn has_key(&self) -> bool {
        self.key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key(&self) -> &[u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.key
    }

    // optional bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    pub fn has_value(&self) -> bool {
        self.value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::std::vec::Vec<u8>) {
        self.value = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value(&self) -> &[u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_value_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.value
    }
}

impl ::protobuf::Message for CommitRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value)?;
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
        if let Some(v) = self.key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.value.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.value.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for CommitRequest {
    fn new() -> CommitRequest {
        CommitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    CommitRequest::get_key_for_reflect,
                    CommitRequest::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    CommitRequest::get_value_for_reflect,
                    CommitRequest::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CommitRequest>(
                    "CommitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommitRequest {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommitRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CommitResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CommitResponse {}

impl CommitResponse {
    pub fn new() -> CommitResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CommitResponse {
        static mut instance: ::protobuf::lazy::Lazy<CommitResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CommitResponse,
        };
        unsafe {
            instance.get(CommitResponse::new)
        }
    }
}

impl ::protobuf::Message for CommitResponse {
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

impl ::protobuf::MessageStatic for CommitResponse {
    fn new() -> CommitResponse {
        CommitResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CommitResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CommitResponse>(
                    "CommitResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CommitResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CommitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CommitResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Request {
    // message fields
    cmd_type: ::std::option::Option<CmdType>,
    get: ::protobuf::SingularPtrField<GetRequest>,
    put: ::protobuf::SingularPtrField<PutRequest>,
    delete: ::protobuf::SingularPtrField<DeleteRequest>,
    snap: ::protobuf::SingularPtrField<SnapRequest>,
    prewrite: ::protobuf::SingularPtrField<PrewriteRequest>,
    commit: ::protobuf::SingularPtrField<CommitRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Request {}

impl Request {
    pub fn new() -> Request {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Request {
        static mut instance: ::protobuf::lazy::Lazy<Request> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Request,
        };
        unsafe {
            instance.get(Request::new)
        }
    }

    // optional .raft_cmdpb.CmdType cmd_type = 1;

    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = ::std::option::Option::None;
    }

    pub fn has_cmd_type(&self) -> bool {
        self.cmd_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_type(&mut self, v: CmdType) {
        self.cmd_type = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_type(&self) -> CmdType {
        self.cmd_type.unwrap_or(CmdType::Invalid)
    }

    fn get_cmd_type_for_reflect(&self) -> &::std::option::Option<CmdType> {
        &self.cmd_type
    }

    fn mut_cmd_type_for_reflect(&mut self) -> &mut ::std::option::Option<CmdType> {
        &mut self.cmd_type
    }

    // optional .raft_cmdpb.GetRequest get = 2;

    pub fn clear_get(&mut self) {
        self.get.clear();
    }

    pub fn has_get(&self) -> bool {
        self.get.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get(&mut self, v: GetRequest) {
        self.get = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get(&mut self) -> &mut GetRequest {
        if self.get.is_none() {
            self.get.set_default();
        };
        self.get.as_mut().unwrap()
    }

    // Take field
    pub fn take_get(&mut self) -> GetRequest {
        self.get.take().unwrap_or_else(|| GetRequest::new())
    }

    pub fn get_get(&self) -> &GetRequest {
        self.get.as_ref().unwrap_or_else(|| GetRequest::default_instance())
    }

    fn get_get_for_reflect(&self) -> &::protobuf::SingularPtrField<GetRequest> {
        &self.get
    }

    fn mut_get_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<GetRequest> {
        &mut self.get
    }

    // optional .raft_cmdpb.PutRequest put = 4;

    pub fn clear_put(&mut self) {
        self.put.clear();
    }

    pub fn has_put(&self) -> bool {
        self.put.is_some()
    }

    // Param is passed by value, moved
    pub fn set_put(&mut self, v: PutRequest) {
        self.put = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put(&mut self) -> &mut PutRequest {
        if self.put.is_none() {
            self.put.set_default();
        };
        self.put.as_mut().unwrap()
    }

    // Take field
    pub fn take_put(&mut self) -> PutRequest {
        self.put.take().unwrap_or_else(|| PutRequest::new())
    }

    pub fn get_put(&self) -> &PutRequest {
        self.put.as_ref().unwrap_or_else(|| PutRequest::default_instance())
    }

    fn get_put_for_reflect(&self) -> &::protobuf::SingularPtrField<PutRequest> {
        &self.put
    }

    fn mut_put_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PutRequest> {
        &mut self.put
    }

    // optional .raft_cmdpb.DeleteRequest delete = 5;

    pub fn clear_delete(&mut self) {
        self.delete.clear();
    }

    pub fn has_delete(&self) -> bool {
        self.delete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delete(&mut self, v: DeleteRequest) {
        self.delete = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete(&mut self) -> &mut DeleteRequest {
        if self.delete.is_none() {
            self.delete.set_default();
        };
        self.delete.as_mut().unwrap()
    }

    // Take field
    pub fn take_delete(&mut self) -> DeleteRequest {
        self.delete.take().unwrap_or_else(|| DeleteRequest::new())
    }

    pub fn get_delete(&self) -> &DeleteRequest {
        self.delete.as_ref().unwrap_or_else(|| DeleteRequest::default_instance())
    }

    fn get_delete_for_reflect(&self) -> &::protobuf::SingularPtrField<DeleteRequest> {
        &self.delete
    }

    fn mut_delete_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DeleteRequest> {
        &mut self.delete
    }

    // optional .raft_cmdpb.SnapRequest snap = 6;

    pub fn clear_snap(&mut self) {
        self.snap.clear();
    }

    pub fn has_snap(&self) -> bool {
        self.snap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snap(&mut self, v: SnapRequest) {
        self.snap = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snap(&mut self) -> &mut SnapRequest {
        if self.snap.is_none() {
            self.snap.set_default();
        };
        self.snap.as_mut().unwrap()
    }

    // Take field
    pub fn take_snap(&mut self) -> SnapRequest {
        self.snap.take().unwrap_or_else(|| SnapRequest::new())
    }

    pub fn get_snap(&self) -> &SnapRequest {
        self.snap.as_ref().unwrap_or_else(|| SnapRequest::default_instance())
    }

    fn get_snap_for_reflect(&self) -> &::protobuf::SingularPtrField<SnapRequest> {
        &self.snap
    }

    fn mut_snap_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SnapRequest> {
        &mut self.snap
    }

    // optional .raft_cmdpb.PrewriteRequest prewrite = 7;

    pub fn clear_prewrite(&mut self) {
        self.prewrite.clear();
    }

    pub fn has_prewrite(&self) -> bool {
        self.prewrite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prewrite(&mut self, v: PrewriteRequest) {
        self.prewrite = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prewrite(&mut self) -> &mut PrewriteRequest {
        if self.prewrite.is_none() {
            self.prewrite.set_default();
        };
        self.prewrite.as_mut().unwrap()
    }

    // Take field
    pub fn take_prewrite(&mut self) -> PrewriteRequest {
        self.prewrite.take().unwrap_or_else(|| PrewriteRequest::new())
    }

    pub fn get_prewrite(&self) -> &PrewriteRequest {
        self.prewrite.as_ref().unwrap_or_else(|| PrewriteRequest::default_instance())
    }

    fn get_prewrite_for_reflect(&self) -> &::protobuf::SingularPtrField<PrewriteRequest> {
        &self.prewrite
    }

    fn mut_prewrite_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PrewriteRequest> {
        &mut self.prewrite
    }

    // optional .raft_cmdpb.CommitRequest commit = 8;

    pub fn clear_commit(&mut self) {
        self.commit.clear();
    }

    pub fn has_commit(&self) -> bool {
        self.commit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: CommitRequest) {
        self.commit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_commit(&mut self) -> &mut CommitRequest {
        if self.commit.is_none() {
            self.commit.set_default();
        };
        self.commit.as_mut().unwrap()
    }

    // Take field
    pub fn take_commit(&mut self) -> CommitRequest {
        self.commit.take().unwrap_or_else(|| CommitRequest::new())
    }

    pub fn get_commit(&self) -> &CommitRequest {
        self.commit.as_ref().unwrap_or_else(|| CommitRequest::default_instance())
    }

    fn get_commit_for_reflect(&self) -> &::protobuf::SingularPtrField<CommitRequest> {
        &self.commit
    }

    fn mut_commit_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CommitRequest> {
        &mut self.commit
    }
}

impl ::protobuf::Message for Request {
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
                    self.cmd_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.delete)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.snap)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.prewrite)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.commit)?;
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
        if let Some(v) = self.cmd_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.get.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.put.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.delete.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.snap.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.prewrite.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.commit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmd_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.get.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.put.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.delete.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.snap.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.prewrite.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.commit.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Request {
    fn new() -> Request {
        Request::new()
    }

    fn descriptor_static(_: ::std::option::Option<Request>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CmdType>>(
                    "cmd_type",
                    Request::get_cmd_type_for_reflect,
                    Request::mut_cmd_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GetRequest>>(
                    "get",
                    Request::get_get_for_reflect,
                    Request::mut_get_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PutRequest>>(
                    "put",
                    Request::get_put_for_reflect,
                    Request::mut_put_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeleteRequest>>(
                    "delete",
                    Request::get_delete_for_reflect,
                    Request::mut_delete_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SnapRequest>>(
                    "snap",
                    Request::get_snap_for_reflect,
                    Request::mut_snap_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PrewriteRequest>>(
                    "prewrite",
                    Request::get_prewrite_for_reflect,
                    Request::mut_prewrite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CommitRequest>>(
                    "commit",
                    Request::get_commit_for_reflect,
                    Request::mut_commit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Request>(
                    "Request",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        self.clear_cmd_type();
        self.clear_get();
        self.clear_put();
        self.clear_delete();
        self.clear_snap();
        self.clear_prewrite();
        self.clear_commit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Request {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Response {
    // message fields
    cmd_type: ::std::option::Option<CmdType>,
    get: ::protobuf::SingularPtrField<GetResponse>,
    put: ::protobuf::SingularPtrField<PutResponse>,
    delete: ::protobuf::SingularPtrField<DeleteResponse>,
    snap: ::protobuf::SingularPtrField<SnapResponse>,
    prewrite: ::protobuf::SingularPtrField<PrewriteResponse>,
    commit: ::protobuf::SingularPtrField<CommitResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Response {}

impl Response {
    pub fn new() -> Response {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Response {
        static mut instance: ::protobuf::lazy::Lazy<Response> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Response,
        };
        unsafe {
            instance.get(Response::new)
        }
    }

    // optional .raft_cmdpb.CmdType cmd_type = 1;

    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = ::std::option::Option::None;
    }

    pub fn has_cmd_type(&self) -> bool {
        self.cmd_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_type(&mut self, v: CmdType) {
        self.cmd_type = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_type(&self) -> CmdType {
        self.cmd_type.unwrap_or(CmdType::Invalid)
    }

    fn get_cmd_type_for_reflect(&self) -> &::std::option::Option<CmdType> {
        &self.cmd_type
    }

    fn mut_cmd_type_for_reflect(&mut self) -> &mut ::std::option::Option<CmdType> {
        &mut self.cmd_type
    }

    // optional .raft_cmdpb.GetResponse get = 2;

    pub fn clear_get(&mut self) {
        self.get.clear();
    }

    pub fn has_get(&self) -> bool {
        self.get.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get(&mut self, v: GetResponse) {
        self.get = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get(&mut self) -> &mut GetResponse {
        if self.get.is_none() {
            self.get.set_default();
        };
        self.get.as_mut().unwrap()
    }

    // Take field
    pub fn take_get(&mut self) -> GetResponse {
        self.get.take().unwrap_or_else(|| GetResponse::new())
    }

    pub fn get_get(&self) -> &GetResponse {
        self.get.as_ref().unwrap_or_else(|| GetResponse::default_instance())
    }

    fn get_get_for_reflect(&self) -> &::protobuf::SingularPtrField<GetResponse> {
        &self.get
    }

    fn mut_get_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<GetResponse> {
        &mut self.get
    }

    // optional .raft_cmdpb.PutResponse put = 4;

    pub fn clear_put(&mut self) {
        self.put.clear();
    }

    pub fn has_put(&self) -> bool {
        self.put.is_some()
    }

    // Param is passed by value, moved
    pub fn set_put(&mut self, v: PutResponse) {
        self.put = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put(&mut self) -> &mut PutResponse {
        if self.put.is_none() {
            self.put.set_default();
        };
        self.put.as_mut().unwrap()
    }

    // Take field
    pub fn take_put(&mut self) -> PutResponse {
        self.put.take().unwrap_or_else(|| PutResponse::new())
    }

    pub fn get_put(&self) -> &PutResponse {
        self.put.as_ref().unwrap_or_else(|| PutResponse::default_instance())
    }

    fn get_put_for_reflect(&self) -> &::protobuf::SingularPtrField<PutResponse> {
        &self.put
    }

    fn mut_put_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PutResponse> {
        &mut self.put
    }

    // optional .raft_cmdpb.DeleteResponse delete = 5;

    pub fn clear_delete(&mut self) {
        self.delete.clear();
    }

    pub fn has_delete(&self) -> bool {
        self.delete.is_some()
    }

    // Param is passed by value, moved
    pub fn set_delete(&mut self, v: DeleteResponse) {
        self.delete = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_delete(&mut self) -> &mut DeleteResponse {
        if self.delete.is_none() {
            self.delete.set_default();
        };
        self.delete.as_mut().unwrap()
    }

    // Take field
    pub fn take_delete(&mut self) -> DeleteResponse {
        self.delete.take().unwrap_or_else(|| DeleteResponse::new())
    }

    pub fn get_delete(&self) -> &DeleteResponse {
        self.delete.as_ref().unwrap_or_else(|| DeleteResponse::default_instance())
    }

    fn get_delete_for_reflect(&self) -> &::protobuf::SingularPtrField<DeleteResponse> {
        &self.delete
    }

    fn mut_delete_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<DeleteResponse> {
        &mut self.delete
    }

    // optional .raft_cmdpb.SnapResponse snap = 6;

    pub fn clear_snap(&mut self) {
        self.snap.clear();
    }

    pub fn has_snap(&self) -> bool {
        self.snap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_snap(&mut self, v: SnapResponse) {
        self.snap = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_snap(&mut self) -> &mut SnapResponse {
        if self.snap.is_none() {
            self.snap.set_default();
        };
        self.snap.as_mut().unwrap()
    }

    // Take field
    pub fn take_snap(&mut self) -> SnapResponse {
        self.snap.take().unwrap_or_else(|| SnapResponse::new())
    }

    pub fn get_snap(&self) -> &SnapResponse {
        self.snap.as_ref().unwrap_or_else(|| SnapResponse::default_instance())
    }

    fn get_snap_for_reflect(&self) -> &::protobuf::SingularPtrField<SnapResponse> {
        &self.snap
    }

    fn mut_snap_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SnapResponse> {
        &mut self.snap
    }

    // optional .raft_cmdpb.PrewriteResponse prewrite = 7;

    pub fn clear_prewrite(&mut self) {
        self.prewrite.clear();
    }

    pub fn has_prewrite(&self) -> bool {
        self.prewrite.is_some()
    }

    // Param is passed by value, moved
    pub fn set_prewrite(&mut self, v: PrewriteResponse) {
        self.prewrite = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_prewrite(&mut self) -> &mut PrewriteResponse {
        if self.prewrite.is_none() {
            self.prewrite.set_default();
        };
        self.prewrite.as_mut().unwrap()
    }

    // Take field
    pub fn take_prewrite(&mut self) -> PrewriteResponse {
        self.prewrite.take().unwrap_or_else(|| PrewriteResponse::new())
    }

    pub fn get_prewrite(&self) -> &PrewriteResponse {
        self.prewrite.as_ref().unwrap_or_else(|| PrewriteResponse::default_instance())
    }

    fn get_prewrite_for_reflect(&self) -> &::protobuf::SingularPtrField<PrewriteResponse> {
        &self.prewrite
    }

    fn mut_prewrite_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<PrewriteResponse> {
        &mut self.prewrite
    }

    // optional .raft_cmdpb.CommitResponse commit = 8;

    pub fn clear_commit(&mut self) {
        self.commit.clear();
    }

    pub fn has_commit(&self) -> bool {
        self.commit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit(&mut self, v: CommitResponse) {
        self.commit = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_commit(&mut self) -> &mut CommitResponse {
        if self.commit.is_none() {
            self.commit.set_default();
        };
        self.commit.as_mut().unwrap()
    }

    // Take field
    pub fn take_commit(&mut self) -> CommitResponse {
        self.commit.take().unwrap_or_else(|| CommitResponse::new())
    }

    pub fn get_commit(&self) -> &CommitResponse {
        self.commit.as_ref().unwrap_or_else(|| CommitResponse::default_instance())
    }

    fn get_commit_for_reflect(&self) -> &::protobuf::SingularPtrField<CommitResponse> {
        &self.commit
    }

    fn mut_commit_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CommitResponse> {
        &mut self.commit
    }
}

impl ::protobuf::Message for Response {
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
                    self.cmd_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.delete)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.snap)?;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.prewrite)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.commit)?;
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
        if let Some(v) = self.cmd_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.get.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.put.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.delete.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.snap.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.prewrite.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.commit.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmd_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.get.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.put.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.delete.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.snap.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.prewrite.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.commit.as_ref() {
            os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for Response {
    fn new() -> Response {
        Response::new()
    }

    fn descriptor_static(_: ::std::option::Option<Response>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<CmdType>>(
                    "cmd_type",
                    Response::get_cmd_type_for_reflect,
                    Response::mut_cmd_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<GetResponse>>(
                    "get",
                    Response::get_get_for_reflect,
                    Response::mut_get_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PutResponse>>(
                    "put",
                    Response::get_put_for_reflect,
                    Response::mut_put_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<DeleteResponse>>(
                    "delete",
                    Response::get_delete_for_reflect,
                    Response::mut_delete_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SnapResponse>>(
                    "snap",
                    Response::get_snap_for_reflect,
                    Response::mut_snap_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<PrewriteResponse>>(
                    "prewrite",
                    Response::get_prewrite_for_reflect,
                    Response::mut_prewrite_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CommitResponse>>(
                    "commit",
                    Response::get_commit_for_reflect,
                    Response::mut_commit_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Response>(
                    "Response",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        self.clear_cmd_type();
        self.clear_get();
        self.clear_put();
        self.clear_delete();
        self.clear_snap();
        self.clear_prewrite();
        self.clear_commit();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Response {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChangePeerRequest {
    // message fields
    change_type: ::std::option::Option<super::eraftpb::ConfChangeType>,
    peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChangePeerRequest {}

impl ChangePeerRequest {
    pub fn new() -> ChangePeerRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChangePeerRequest {
        static mut instance: ::protobuf::lazy::Lazy<ChangePeerRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChangePeerRequest,
        };
        unsafe {
            instance.get(ChangePeerRequest::new)
        }
    }

    // optional .eraftpb.ConfChangeType change_type = 1;

    pub fn clear_change_type(&mut self) {
        self.change_type = ::std::option::Option::None;
    }

    pub fn has_change_type(&self) -> bool {
        self.change_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_change_type(&mut self, v: super::eraftpb::ConfChangeType) {
        self.change_type = ::std::option::Option::Some(v);
    }

    pub fn get_change_type(&self) -> super::eraftpb::ConfChangeType {
        self.change_type.unwrap_or(super::eraftpb::ConfChangeType::AddNode)
    }

    fn get_change_type_for_reflect(&self) -> &::std::option::Option<super::eraftpb::ConfChangeType> {
        &self.change_type
    }

    fn mut_change_type_for_reflect(&mut self) -> &mut ::std::option::Option<super::eraftpb::ConfChangeType> {
        &mut self.change_type
    }

    // optional .metapb.Peer peer = 2;

    pub fn clear_peer(&mut self) {
        self.peer.clear();
    }

    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer(&mut self, v: super::metapb::Peer) {
        self.peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer(&mut self) -> &mut super::metapb::Peer {
        if self.peer.is_none() {
            self.peer.set_default();
        };
        self.peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer(&mut self) -> super::metapb::Peer {
        self.peer.take().unwrap_or_else(|| super::metapb::Peer::new())
    }

    pub fn get_peer(&self) -> &super::metapb::Peer {
        self.peer.as_ref().unwrap_or_else(|| super::metapb::Peer::default_instance())
    }

    fn get_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Peer> {
        &self.peer
    }

    fn mut_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Peer> {
        &mut self.peer
    }
}

impl ::protobuf::Message for ChangePeerRequest {
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
                    self.change_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer)?;
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
        if let Some(v) = self.change_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.change_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.peer.as_ref() {
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

impl ::protobuf::MessageStatic for ChangePeerRequest {
    fn new() -> ChangePeerRequest {
        ChangePeerRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChangePeerRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<super::eraftpb::ConfChangeType>>(
                    "change_type",
                    ChangePeerRequest::get_change_type_for_reflect,
                    ChangePeerRequest::mut_change_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Peer>>(
                    "peer",
                    ChangePeerRequest::get_peer_for_reflect,
                    ChangePeerRequest::mut_peer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChangePeerRequest>(
                    "ChangePeerRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChangePeerRequest {
    fn clear(&mut self) {
        self.clear_change_type();
        self.clear_peer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChangePeerRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChangePeerRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ChangePeerResponse {
    // message fields
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChangePeerResponse {}

impl ChangePeerResponse {
    pub fn new() -> ChangePeerResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChangePeerResponse {
        static mut instance: ::protobuf::lazy::Lazy<ChangePeerResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChangePeerResponse,
        };
        unsafe {
            instance.get(ChangePeerResponse::new)
        }
    }

    // optional .metapb.Region region = 1;

    pub fn clear_region(&mut self) {
        self.region.clear();
    }

    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region.set_default();
        };
        self.region.as_mut().unwrap()
    }

    // Take field
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region.take().unwrap_or_else(|| super::metapb::Region::new())
    }

    pub fn get_region(&self) -> &super::metapb::Region {
        self.region.as_ref().unwrap_or_else(|| super::metapb::Region::default_instance())
    }

    fn get_region_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Region> {
        &self.region
    }

    fn mut_region_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Region> {
        &mut self.region
    }
}

impl ::protobuf::Message for ChangePeerResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region)?;
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
        if let Some(v) = self.region.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region.as_ref() {
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

impl ::protobuf::MessageStatic for ChangePeerResponse {
    fn new() -> ChangePeerResponse {
        ChangePeerResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChangePeerResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Region>>(
                    "region",
                    ChangePeerResponse::get_region_for_reflect,
                    ChangePeerResponse::mut_region_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChangePeerResponse>(
                    "ChangePeerResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChangePeerResponse {
    fn clear(&mut self) {
        self.clear_region();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ChangePeerResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ChangePeerResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SplitRequest {
    // message fields
    split_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    new_region_id: ::std::option::Option<u64>,
    new_peer_ids: ::std::vec::Vec<u64>,
    right_derive: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SplitRequest {}

impl SplitRequest {
    pub fn new() -> SplitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SplitRequest {
        static mut instance: ::protobuf::lazy::Lazy<SplitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SplitRequest,
        };
        unsafe {
            instance.get(SplitRequest::new)
        }
    }

    // optional bytes split_key = 1;

    pub fn clear_split_key(&mut self) {
        self.split_key.clear();
    }

    pub fn has_split_key(&self) -> bool {
        self.split_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_split_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.split_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_split_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.split_key.is_none() {
            self.split_key.set_default();
        };
        self.split_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_split_key(&mut self) -> ::std::vec::Vec<u8> {
        self.split_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_split_key(&self) -> &[u8] {
        match self.split_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_split_key_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.split_key
    }

    fn mut_split_key_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.split_key
    }

    // optional uint64 new_region_id = 2;

    pub fn clear_new_region_id(&mut self) {
        self.new_region_id = ::std::option::Option::None;
    }

    pub fn has_new_region_id(&self) -> bool {
        self.new_region_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_new_region_id(&mut self, v: u64) {
        self.new_region_id = ::std::option::Option::Some(v);
    }

    pub fn get_new_region_id(&self) -> u64 {
        self.new_region_id.unwrap_or(0)
    }

    fn get_new_region_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.new_region_id
    }

    fn mut_new_region_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.new_region_id
    }

    // repeated uint64 new_peer_ids = 3;

    pub fn clear_new_peer_ids(&mut self) {
        self.new_peer_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_new_peer_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.new_peer_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_new_peer_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.new_peer_ids
    }

    // Take field
    pub fn take_new_peer_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.new_peer_ids, ::std::vec::Vec::new())
    }

    pub fn get_new_peer_ids(&self) -> &[u64] {
        &self.new_peer_ids
    }

    fn get_new_peer_ids_for_reflect(&self) -> &::std::vec::Vec<u64> {
        &self.new_peer_ids
    }

    fn mut_new_peer_ids_for_reflect(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.new_peer_ids
    }

    // optional bool right_derive = 4;

    pub fn clear_right_derive(&mut self) {
        self.right_derive = ::std::option::Option::None;
    }

    pub fn has_right_derive(&self) -> bool {
        self.right_derive.is_some()
    }

    // Param is passed by value, moved
    pub fn set_right_derive(&mut self, v: bool) {
        self.right_derive = ::std::option::Option::Some(v);
    }

    pub fn get_right_derive(&self) -> bool {
        self.right_derive.unwrap_or(false)
    }

    fn get_right_derive_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.right_derive
    }

    fn mut_right_derive_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.right_derive
    }
}

impl ::protobuf::Message for SplitRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.split_key)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.new_region_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    ::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.new_peer_ids)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.right_derive = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.split_key.as_ref() {
            my_size += ::protobuf::rt::bytes_size(1, &v);
        };
        if let Some(v) = self.new_region_id {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.new_peer_ids {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.right_derive {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.split_key.as_ref() {
            os.write_bytes(1, &v)?;
        };
        if let Some(v) = self.new_region_id {
            os.write_uint64(2, v)?;
        };
        for v in &self.new_peer_ids {
            os.write_uint64(3, *v)?;
        };
        if let Some(v) = self.right_derive {
            os.write_bool(4, v)?;
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

impl ::protobuf::MessageStatic for SplitRequest {
    fn new() -> SplitRequest {
        SplitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<SplitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "split_key",
                    SplitRequest::get_split_key_for_reflect,
                    SplitRequest::mut_split_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "new_region_id",
                    SplitRequest::get_new_region_id_for_reflect,
                    SplitRequest::mut_new_region_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_vec_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "new_peer_ids",
                    SplitRequest::get_new_peer_ids_for_reflect,
                    SplitRequest::mut_new_peer_ids_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "right_derive",
                    SplitRequest::get_right_derive_for_reflect,
                    SplitRequest::mut_right_derive_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SplitRequest>(
                    "SplitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SplitRequest {
    fn clear(&mut self) {
        self.clear_split_key();
        self.clear_new_region_id();
        self.clear_new_peer_ids();
        self.clear_right_derive();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SplitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SplitRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SplitResponse {
    // message fields
    left: ::protobuf::SingularPtrField<super::metapb::Region>,
    right: ::protobuf::SingularPtrField<super::metapb::Region>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SplitResponse {}

impl SplitResponse {
    pub fn new() -> SplitResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SplitResponse {
        static mut instance: ::protobuf::lazy::Lazy<SplitResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SplitResponse,
        };
        unsafe {
            instance.get(SplitResponse::new)
        }
    }

    // optional .metapb.Region left = 1;

    pub fn clear_left(&mut self) {
        self.left.clear();
    }

    pub fn has_left(&self) -> bool {
        self.left.is_some()
    }

    // Param is passed by value, moved
    pub fn set_left(&mut self, v: super::metapb::Region) {
        self.left = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_left(&mut self) -> &mut super::metapb::Region {
        if self.left.is_none() {
            self.left.set_default();
        };
        self.left.as_mut().unwrap()
    }

    // Take field
    pub fn take_left(&mut self) -> super::metapb::Region {
        self.left.take().unwrap_or_else(|| super::metapb::Region::new())
    }

    pub fn get_left(&self) -> &super::metapb::Region {
        self.left.as_ref().unwrap_or_else(|| super::metapb::Region::default_instance())
    }

    fn get_left_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Region> {
        &self.left
    }

    fn mut_left_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Region> {
        &mut self.left
    }

    // optional .metapb.Region right = 2;

    pub fn clear_right(&mut self) {
        self.right.clear();
    }

    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }

    // Param is passed by value, moved
    pub fn set_right(&mut self, v: super::metapb::Region) {
        self.right = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_right(&mut self) -> &mut super::metapb::Region {
        if self.right.is_none() {
            self.right.set_default();
        };
        self.right.as_mut().unwrap()
    }

    // Take field
    pub fn take_right(&mut self) -> super::metapb::Region {
        self.right.take().unwrap_or_else(|| super::metapb::Region::new())
    }

    pub fn get_right(&self) -> &super::metapb::Region {
        self.right.as_ref().unwrap_or_else(|| super::metapb::Region::default_instance())
    }

    fn get_right_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Region> {
        &self.right
    }

    fn mut_right_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Region> {
        &mut self.right
    }
}

impl ::protobuf::Message for SplitResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.left)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.right)?;
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
        if let Some(v) = self.left.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.right.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.left.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.right.as_ref() {
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

impl ::protobuf::MessageStatic for SplitResponse {
    fn new() -> SplitResponse {
        SplitResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<SplitResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Region>>(
                    "left",
                    SplitResponse::get_left_for_reflect,
                    SplitResponse::mut_left_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Region>>(
                    "right",
                    SplitResponse::get_right_for_reflect,
                    SplitResponse::mut_right_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SplitResponse>(
                    "SplitResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SplitResponse {
    fn clear(&mut self) {
        self.clear_left();
        self.clear_right();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SplitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SplitResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CompactLogRequest {
    // message fields
    compact_index: ::std::option::Option<u64>,
    compact_term: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CompactLogRequest {}

impl CompactLogRequest {
    pub fn new() -> CompactLogRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CompactLogRequest {
        static mut instance: ::protobuf::lazy::Lazy<CompactLogRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CompactLogRequest,
        };
        unsafe {
            instance.get(CompactLogRequest::new)
        }
    }

    // optional uint64 compact_index = 1;

    pub fn clear_compact_index(&mut self) {
        self.compact_index = ::std::option::Option::None;
    }

    pub fn has_compact_index(&self) -> bool {
        self.compact_index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compact_index(&mut self, v: u64) {
        self.compact_index = ::std::option::Option::Some(v);
    }

    pub fn get_compact_index(&self) -> u64 {
        self.compact_index.unwrap_or(0)
    }

    fn get_compact_index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.compact_index
    }

    fn mut_compact_index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.compact_index
    }

    // optional uint64 compact_term = 2;

    pub fn clear_compact_term(&mut self) {
        self.compact_term = ::std::option::Option::None;
    }

    pub fn has_compact_term(&self) -> bool {
        self.compact_term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compact_term(&mut self, v: u64) {
        self.compact_term = ::std::option::Option::Some(v);
    }

    pub fn get_compact_term(&self) -> u64 {
        self.compact_term.unwrap_or(0)
    }

    fn get_compact_term_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.compact_term
    }

    fn mut_compact_term_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.compact_term
    }
}

impl ::protobuf::Message for CompactLogRequest {
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
                    self.compact_index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.compact_term = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.compact_index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.compact_term {
            my_size += ::protobuf::rt::value_size(2, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.compact_index {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.compact_term {
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

impl ::protobuf::MessageStatic for CompactLogRequest {
    fn new() -> CompactLogRequest {
        CompactLogRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CompactLogRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "compact_index",
                    CompactLogRequest::get_compact_index_for_reflect,
                    CompactLogRequest::mut_compact_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "compact_term",
                    CompactLogRequest::get_compact_term_for_reflect,
                    CompactLogRequest::mut_compact_term_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CompactLogRequest>(
                    "CompactLogRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CompactLogRequest {
    fn clear(&mut self) {
        self.clear_compact_index();
        self.clear_compact_term();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CompactLogRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CompactLogRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CompactLogResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CompactLogResponse {}

impl CompactLogResponse {
    pub fn new() -> CompactLogResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CompactLogResponse {
        static mut instance: ::protobuf::lazy::Lazy<CompactLogResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CompactLogResponse,
        };
        unsafe {
            instance.get(CompactLogResponse::new)
        }
    }
}

impl ::protobuf::Message for CompactLogResponse {
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

impl ::protobuf::MessageStatic for CompactLogResponse {
    fn new() -> CompactLogResponse {
        CompactLogResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CompactLogResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CompactLogResponse>(
                    "CompactLogResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CompactLogResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CompactLogResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CompactLogResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransferLeaderRequest {
    // message fields
    peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransferLeaderRequest {}

impl TransferLeaderRequest {
    pub fn new() -> TransferLeaderRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransferLeaderRequest {
        static mut instance: ::protobuf::lazy::Lazy<TransferLeaderRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransferLeaderRequest,
        };
        unsafe {
            instance.get(TransferLeaderRequest::new)
        }
    }

    // optional .metapb.Peer peer = 1;

    pub fn clear_peer(&mut self) {
        self.peer.clear();
    }

    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer(&mut self, v: super::metapb::Peer) {
        self.peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer(&mut self) -> &mut super::metapb::Peer {
        if self.peer.is_none() {
            self.peer.set_default();
        };
        self.peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer(&mut self) -> super::metapb::Peer {
        self.peer.take().unwrap_or_else(|| super::metapb::Peer::new())
    }

    pub fn get_peer(&self) -> &super::metapb::Peer {
        self.peer.as_ref().unwrap_or_else(|| super::metapb::Peer::default_instance())
    }

    fn get_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Peer> {
        &self.peer
    }

    fn mut_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Peer> {
        &mut self.peer
    }
}

impl ::protobuf::Message for TransferLeaderRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer)?;
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
        if let Some(v) = self.peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.peer.as_ref() {
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

impl ::protobuf::MessageStatic for TransferLeaderRequest {
    fn new() -> TransferLeaderRequest {
        TransferLeaderRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransferLeaderRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Peer>>(
                    "peer",
                    TransferLeaderRequest::get_peer_for_reflect,
                    TransferLeaderRequest::mut_peer_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransferLeaderRequest>(
                    "TransferLeaderRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransferLeaderRequest {
    fn clear(&mut self) {
        self.clear_peer();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransferLeaderRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransferLeaderRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TransferLeaderResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransferLeaderResponse {}

impl TransferLeaderResponse {
    pub fn new() -> TransferLeaderResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransferLeaderResponse {
        static mut instance: ::protobuf::lazy::Lazy<TransferLeaderResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransferLeaderResponse,
        };
        unsafe {
            instance.get(TransferLeaderResponse::new)
        }
    }
}

impl ::protobuf::Message for TransferLeaderResponse {
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

impl ::protobuf::MessageStatic for TransferLeaderResponse {
    fn new() -> TransferLeaderResponse {
        TransferLeaderResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransferLeaderResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<TransferLeaderResponse>(
                    "TransferLeaderResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransferLeaderResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TransferLeaderResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TransferLeaderResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyHashRequest {
    // message fields
    index: ::std::option::Option<u64>,
    hash: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyHashRequest {}

impl VerifyHashRequest {
    pub fn new() -> VerifyHashRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyHashRequest {
        static mut instance: ::protobuf::lazy::Lazy<VerifyHashRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyHashRequest,
        };
        unsafe {
            instance.get(VerifyHashRequest::new)
        }
    }

    // optional uint64 index = 1;

    pub fn clear_index(&mut self) {
        self.index = ::std::option::Option::None;
    }

    pub fn has_index(&self) -> bool {
        self.index.is_some()
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = ::std::option::Option::Some(v);
    }

    pub fn get_index(&self) -> u64 {
        self.index.unwrap_or(0)
    }

    fn get_index_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.index
    }

    // optional bytes hash = 2;

    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }

    pub fn has_hash(&self) -> bool {
        self.hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hash(&mut self, v: ::std::vec::Vec<u8>) {
        self.hash = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hash(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.hash.is_none() {
            self.hash.set_default();
        };
        self.hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_hash(&mut self) -> ::std::vec::Vec<u8> {
        self.hash.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_hash(&self) -> &[u8] {
        match self.hash.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_hash_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.hash
    }

    fn mut_hash_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.hash
    }
}

impl ::protobuf::Message for VerifyHashRequest {
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
                    self.index = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.hash)?;
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
        if let Some(v) = self.index {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.hash.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.index {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.hash.as_ref() {
            os.write_bytes(2, &v)?;
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

impl ::protobuf::MessageStatic for VerifyHashRequest {
    fn new() -> VerifyHashRequest {
        VerifyHashRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyHashRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    VerifyHashRequest::get_index_for_reflect,
                    VerifyHashRequest::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "hash",
                    VerifyHashRequest::get_hash_for_reflect,
                    VerifyHashRequest::mut_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<VerifyHashRequest>(
                    "VerifyHashRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyHashRequest {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyHashRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyHashRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct VerifyHashResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for VerifyHashResponse {}

impl VerifyHashResponse {
    pub fn new() -> VerifyHashResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static VerifyHashResponse {
        static mut instance: ::protobuf::lazy::Lazy<VerifyHashResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const VerifyHashResponse,
        };
        unsafe {
            instance.get(VerifyHashResponse::new)
        }
    }
}

impl ::protobuf::Message for VerifyHashResponse {
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

impl ::protobuf::MessageStatic for VerifyHashResponse {
    fn new() -> VerifyHashResponse {
        VerifyHashResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<VerifyHashResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<VerifyHashResponse>(
                    "VerifyHashResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for VerifyHashResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for VerifyHashResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for VerifyHashResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminRequest {
    // message fields
    cmd_type: ::std::option::Option<AdminCmdType>,
    change_peer: ::protobuf::SingularPtrField<ChangePeerRequest>,
    split: ::protobuf::SingularPtrField<SplitRequest>,
    compact_log: ::protobuf::SingularPtrField<CompactLogRequest>,
    transfer_leader: ::protobuf::SingularPtrField<TransferLeaderRequest>,
    verify_hash: ::protobuf::SingularPtrField<VerifyHashRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminRequest {}

impl AdminRequest {
    pub fn new() -> AdminRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminRequest {
        static mut instance: ::protobuf::lazy::Lazy<AdminRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminRequest,
        };
        unsafe {
            instance.get(AdminRequest::new)
        }
    }

    // optional .raft_cmdpb.AdminCmdType cmd_type = 1;

    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = ::std::option::Option::None;
    }

    pub fn has_cmd_type(&self) -> bool {
        self.cmd_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_type(&mut self, v: AdminCmdType) {
        self.cmd_type = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_type(&self) -> AdminCmdType {
        self.cmd_type.unwrap_or(AdminCmdType::InvalidAdmin)
    }

    fn get_cmd_type_for_reflect(&self) -> &::std::option::Option<AdminCmdType> {
        &self.cmd_type
    }

    fn mut_cmd_type_for_reflect(&mut self) -> &mut ::std::option::Option<AdminCmdType> {
        &mut self.cmd_type
    }

    // optional .raft_cmdpb.ChangePeerRequest change_peer = 2;

    pub fn clear_change_peer(&mut self) {
        self.change_peer.clear();
    }

    pub fn has_change_peer(&self) -> bool {
        self.change_peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_change_peer(&mut self, v: ChangePeerRequest) {
        self.change_peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_change_peer(&mut self) -> &mut ChangePeerRequest {
        if self.change_peer.is_none() {
            self.change_peer.set_default();
        };
        self.change_peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_change_peer(&mut self) -> ChangePeerRequest {
        self.change_peer.take().unwrap_or_else(|| ChangePeerRequest::new())
    }

    pub fn get_change_peer(&self) -> &ChangePeerRequest {
        self.change_peer.as_ref().unwrap_or_else(|| ChangePeerRequest::default_instance())
    }

    fn get_change_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<ChangePeerRequest> {
        &self.change_peer
    }

    fn mut_change_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChangePeerRequest> {
        &mut self.change_peer
    }

    // optional .raft_cmdpb.SplitRequest split = 3;

    pub fn clear_split(&mut self) {
        self.split.clear();
    }

    pub fn has_split(&self) -> bool {
        self.split.is_some()
    }

    // Param is passed by value, moved
    pub fn set_split(&mut self, v: SplitRequest) {
        self.split = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_split(&mut self) -> &mut SplitRequest {
        if self.split.is_none() {
            self.split.set_default();
        };
        self.split.as_mut().unwrap()
    }

    // Take field
    pub fn take_split(&mut self) -> SplitRequest {
        self.split.take().unwrap_or_else(|| SplitRequest::new())
    }

    pub fn get_split(&self) -> &SplitRequest {
        self.split.as_ref().unwrap_or_else(|| SplitRequest::default_instance())
    }

    fn get_split_for_reflect(&self) -> &::protobuf::SingularPtrField<SplitRequest> {
        &self.split
    }

    fn mut_split_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SplitRequest> {
        &mut self.split
    }

    // optional .raft_cmdpb.CompactLogRequest compact_log = 4;

    pub fn clear_compact_log(&mut self) {
        self.compact_log.clear();
    }

    pub fn has_compact_log(&self) -> bool {
        self.compact_log.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compact_log(&mut self, v: CompactLogRequest) {
        self.compact_log = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_compact_log(&mut self) -> &mut CompactLogRequest {
        if self.compact_log.is_none() {
            self.compact_log.set_default();
        };
        self.compact_log.as_mut().unwrap()
    }

    // Take field
    pub fn take_compact_log(&mut self) -> CompactLogRequest {
        self.compact_log.take().unwrap_or_else(|| CompactLogRequest::new())
    }

    pub fn get_compact_log(&self) -> &CompactLogRequest {
        self.compact_log.as_ref().unwrap_or_else(|| CompactLogRequest::default_instance())
    }

    fn get_compact_log_for_reflect(&self) -> &::protobuf::SingularPtrField<CompactLogRequest> {
        &self.compact_log
    }

    fn mut_compact_log_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CompactLogRequest> {
        &mut self.compact_log
    }

    // optional .raft_cmdpb.TransferLeaderRequest transfer_leader = 5;

    pub fn clear_transfer_leader(&mut self) {
        self.transfer_leader.clear();
    }

    pub fn has_transfer_leader(&self) -> bool {
        self.transfer_leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transfer_leader(&mut self, v: TransferLeaderRequest) {
        self.transfer_leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transfer_leader(&mut self) -> &mut TransferLeaderRequest {
        if self.transfer_leader.is_none() {
            self.transfer_leader.set_default();
        };
        self.transfer_leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_transfer_leader(&mut self) -> TransferLeaderRequest {
        self.transfer_leader.take().unwrap_or_else(|| TransferLeaderRequest::new())
    }

    pub fn get_transfer_leader(&self) -> &TransferLeaderRequest {
        self.transfer_leader.as_ref().unwrap_or_else(|| TransferLeaderRequest::default_instance())
    }

    fn get_transfer_leader_for_reflect(&self) -> &::protobuf::SingularPtrField<TransferLeaderRequest> {
        &self.transfer_leader
    }

    fn mut_transfer_leader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TransferLeaderRequest> {
        &mut self.transfer_leader
    }

    // optional .raft_cmdpb.VerifyHashRequest verify_hash = 6;

    pub fn clear_verify_hash(&mut self) {
        self.verify_hash.clear();
    }

    pub fn has_verify_hash(&self) -> bool {
        self.verify_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_verify_hash(&mut self, v: VerifyHashRequest) {
        self.verify_hash = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_verify_hash(&mut self) -> &mut VerifyHashRequest {
        if self.verify_hash.is_none() {
            self.verify_hash.set_default();
        };
        self.verify_hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_verify_hash(&mut self) -> VerifyHashRequest {
        self.verify_hash.take().unwrap_or_else(|| VerifyHashRequest::new())
    }

    pub fn get_verify_hash(&self) -> &VerifyHashRequest {
        self.verify_hash.as_ref().unwrap_or_else(|| VerifyHashRequest::default_instance())
    }

    fn get_verify_hash_for_reflect(&self) -> &::protobuf::SingularPtrField<VerifyHashRequest> {
        &self.verify_hash
    }

    fn mut_verify_hash_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<VerifyHashRequest> {
        &mut self.verify_hash
    }
}

impl ::protobuf::Message for AdminRequest {
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
                    self.cmd_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.change_peer)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.split)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.compact_log)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transfer_leader)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.verify_hash)?;
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
        if let Some(v) = self.cmd_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.change_peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.split.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.compact_log.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.transfer_leader.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.verify_hash.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmd_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.change_peer.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.split.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.compact_log.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.transfer_leader.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.verify_hash.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for AdminRequest {
    fn new() -> AdminRequest {
        AdminRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AdminCmdType>>(
                    "cmd_type",
                    AdminRequest::get_cmd_type_for_reflect,
                    AdminRequest::mut_cmd_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChangePeerRequest>>(
                    "change_peer",
                    AdminRequest::get_change_peer_for_reflect,
                    AdminRequest::mut_change_peer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SplitRequest>>(
                    "split",
                    AdminRequest::get_split_for_reflect,
                    AdminRequest::mut_split_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CompactLogRequest>>(
                    "compact_log",
                    AdminRequest::get_compact_log_for_reflect,
                    AdminRequest::mut_compact_log_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TransferLeaderRequest>>(
                    "transfer_leader",
                    AdminRequest::get_transfer_leader_for_reflect,
                    AdminRequest::mut_transfer_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VerifyHashRequest>>(
                    "verify_hash",
                    AdminRequest::get_verify_hash_for_reflect,
                    AdminRequest::mut_verify_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminRequest>(
                    "AdminRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminRequest {
    fn clear(&mut self) {
        self.clear_cmd_type();
        self.clear_change_peer();
        self.clear_split();
        self.clear_compact_log();
        self.clear_transfer_leader();
        self.clear_verify_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct AdminResponse {
    // message fields
    cmd_type: ::std::option::Option<AdminCmdType>,
    change_peer: ::protobuf::SingularPtrField<ChangePeerResponse>,
    split: ::protobuf::SingularPtrField<SplitResponse>,
    compact_log: ::protobuf::SingularPtrField<CompactLogResponse>,
    transfer_leader: ::protobuf::SingularPtrField<TransferLeaderResponse>,
    verify_hash: ::protobuf::SingularPtrField<VerifyHashResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AdminResponse {}

impl AdminResponse {
    pub fn new() -> AdminResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AdminResponse {
        static mut instance: ::protobuf::lazy::Lazy<AdminResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AdminResponse,
        };
        unsafe {
            instance.get(AdminResponse::new)
        }
    }

    // optional .raft_cmdpb.AdminCmdType cmd_type = 1;

    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = ::std::option::Option::None;
    }

    pub fn has_cmd_type(&self) -> bool {
        self.cmd_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_type(&mut self, v: AdminCmdType) {
        self.cmd_type = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_type(&self) -> AdminCmdType {
        self.cmd_type.unwrap_or(AdminCmdType::InvalidAdmin)
    }

    fn get_cmd_type_for_reflect(&self) -> &::std::option::Option<AdminCmdType> {
        &self.cmd_type
    }

    fn mut_cmd_type_for_reflect(&mut self) -> &mut ::std::option::Option<AdminCmdType> {
        &mut self.cmd_type
    }

    // optional .raft_cmdpb.ChangePeerResponse change_peer = 2;

    pub fn clear_change_peer(&mut self) {
        self.change_peer.clear();
    }

    pub fn has_change_peer(&self) -> bool {
        self.change_peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_change_peer(&mut self, v: ChangePeerResponse) {
        self.change_peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_change_peer(&mut self) -> &mut ChangePeerResponse {
        if self.change_peer.is_none() {
            self.change_peer.set_default();
        };
        self.change_peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_change_peer(&mut self) -> ChangePeerResponse {
        self.change_peer.take().unwrap_or_else(|| ChangePeerResponse::new())
    }

    pub fn get_change_peer(&self) -> &ChangePeerResponse {
        self.change_peer.as_ref().unwrap_or_else(|| ChangePeerResponse::default_instance())
    }

    fn get_change_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<ChangePeerResponse> {
        &self.change_peer
    }

    fn mut_change_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<ChangePeerResponse> {
        &mut self.change_peer
    }

    // optional .raft_cmdpb.SplitResponse split = 3;

    pub fn clear_split(&mut self) {
        self.split.clear();
    }

    pub fn has_split(&self) -> bool {
        self.split.is_some()
    }

    // Param is passed by value, moved
    pub fn set_split(&mut self, v: SplitResponse) {
        self.split = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_split(&mut self) -> &mut SplitResponse {
        if self.split.is_none() {
            self.split.set_default();
        };
        self.split.as_mut().unwrap()
    }

    // Take field
    pub fn take_split(&mut self) -> SplitResponse {
        self.split.take().unwrap_or_else(|| SplitResponse::new())
    }

    pub fn get_split(&self) -> &SplitResponse {
        self.split.as_ref().unwrap_or_else(|| SplitResponse::default_instance())
    }

    fn get_split_for_reflect(&self) -> &::protobuf::SingularPtrField<SplitResponse> {
        &self.split
    }

    fn mut_split_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SplitResponse> {
        &mut self.split
    }

    // optional .raft_cmdpb.CompactLogResponse compact_log = 4;

    pub fn clear_compact_log(&mut self) {
        self.compact_log.clear();
    }

    pub fn has_compact_log(&self) -> bool {
        self.compact_log.is_some()
    }

    // Param is passed by value, moved
    pub fn set_compact_log(&mut self, v: CompactLogResponse) {
        self.compact_log = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_compact_log(&mut self) -> &mut CompactLogResponse {
        if self.compact_log.is_none() {
            self.compact_log.set_default();
        };
        self.compact_log.as_mut().unwrap()
    }

    // Take field
    pub fn take_compact_log(&mut self) -> CompactLogResponse {
        self.compact_log.take().unwrap_or_else(|| CompactLogResponse::new())
    }

    pub fn get_compact_log(&self) -> &CompactLogResponse {
        self.compact_log.as_ref().unwrap_or_else(|| CompactLogResponse::default_instance())
    }

    fn get_compact_log_for_reflect(&self) -> &::protobuf::SingularPtrField<CompactLogResponse> {
        &self.compact_log
    }

    fn mut_compact_log_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<CompactLogResponse> {
        &mut self.compact_log
    }

    // optional .raft_cmdpb.TransferLeaderResponse transfer_leader = 5;

    pub fn clear_transfer_leader(&mut self) {
        self.transfer_leader.clear();
    }

    pub fn has_transfer_leader(&self) -> bool {
        self.transfer_leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transfer_leader(&mut self, v: TransferLeaderResponse) {
        self.transfer_leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transfer_leader(&mut self) -> &mut TransferLeaderResponse {
        if self.transfer_leader.is_none() {
            self.transfer_leader.set_default();
        };
        self.transfer_leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_transfer_leader(&mut self) -> TransferLeaderResponse {
        self.transfer_leader.take().unwrap_or_else(|| TransferLeaderResponse::new())
    }

    pub fn get_transfer_leader(&self) -> &TransferLeaderResponse {
        self.transfer_leader.as_ref().unwrap_or_else(|| TransferLeaderResponse::default_instance())
    }

    fn get_transfer_leader_for_reflect(&self) -> &::protobuf::SingularPtrField<TransferLeaderResponse> {
        &self.transfer_leader
    }

    fn mut_transfer_leader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<TransferLeaderResponse> {
        &mut self.transfer_leader
    }

    // optional .raft_cmdpb.VerifyHashResponse verify_hash = 6;

    pub fn clear_verify_hash(&mut self) {
        self.verify_hash.clear();
    }

    pub fn has_verify_hash(&self) -> bool {
        self.verify_hash.is_some()
    }

    // Param is passed by value, moved
    pub fn set_verify_hash(&mut self, v: VerifyHashResponse) {
        self.verify_hash = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_verify_hash(&mut self) -> &mut VerifyHashResponse {
        if self.verify_hash.is_none() {
            self.verify_hash.set_default();
        };
        self.verify_hash.as_mut().unwrap()
    }

    // Take field
    pub fn take_verify_hash(&mut self) -> VerifyHashResponse {
        self.verify_hash.take().unwrap_or_else(|| VerifyHashResponse::new())
    }

    pub fn get_verify_hash(&self) -> &VerifyHashResponse {
        self.verify_hash.as_ref().unwrap_or_else(|| VerifyHashResponse::default_instance())
    }

    fn get_verify_hash_for_reflect(&self) -> &::protobuf::SingularPtrField<VerifyHashResponse> {
        &self.verify_hash
    }

    fn mut_verify_hash_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<VerifyHashResponse> {
        &mut self.verify_hash
    }
}

impl ::protobuf::Message for AdminResponse {
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
                    self.cmd_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.change_peer)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.split)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.compact_log)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transfer_leader)?;
                },
                6 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.verify_hash)?;
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
        if let Some(v) = self.cmd_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.change_peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.split.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.compact_log.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.transfer_leader.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.verify_hash.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmd_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.change_peer.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.split.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.compact_log.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.transfer_leader.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.verify_hash.as_ref() {
            os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for AdminResponse {
    fn new() -> AdminResponse {
        AdminResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AdminResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<AdminCmdType>>(
                    "cmd_type",
                    AdminResponse::get_cmd_type_for_reflect,
                    AdminResponse::mut_cmd_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<ChangePeerResponse>>(
                    "change_peer",
                    AdminResponse::get_change_peer_for_reflect,
                    AdminResponse::mut_change_peer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SplitResponse>>(
                    "split",
                    AdminResponse::get_split_for_reflect,
                    AdminResponse::mut_split_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<CompactLogResponse>>(
                    "compact_log",
                    AdminResponse::get_compact_log_for_reflect,
                    AdminResponse::mut_compact_log_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<TransferLeaderResponse>>(
                    "transfer_leader",
                    AdminResponse::get_transfer_leader_for_reflect,
                    AdminResponse::mut_transfer_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<VerifyHashResponse>>(
                    "verify_hash",
                    AdminResponse::get_verify_hash_for_reflect,
                    AdminResponse::mut_verify_hash_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AdminResponse>(
                    "AdminResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AdminResponse {
    fn clear(&mut self) {
        self.clear_cmd_type();
        self.clear_change_peer();
        self.clear_split();
        self.clear_compact_log();
        self.clear_transfer_leader();
        self.clear_verify_hash();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for AdminResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for AdminResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegionLeaderRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionLeaderRequest {}

impl RegionLeaderRequest {
    pub fn new() -> RegionLeaderRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionLeaderRequest {
        static mut instance: ::protobuf::lazy::Lazy<RegionLeaderRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionLeaderRequest,
        };
        unsafe {
            instance.get(RegionLeaderRequest::new)
        }
    }
}

impl ::protobuf::Message for RegionLeaderRequest {
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

impl ::protobuf::MessageStatic for RegionLeaderRequest {
    fn new() -> RegionLeaderRequest {
        RegionLeaderRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionLeaderRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RegionLeaderRequest>(
                    "RegionLeaderRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionLeaderRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegionLeaderRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionLeaderRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegionLeaderResponse {
    // message fields
    leader: ::protobuf::SingularPtrField<super::metapb::Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionLeaderResponse {}

impl RegionLeaderResponse {
    pub fn new() -> RegionLeaderResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionLeaderResponse {
        static mut instance: ::protobuf::lazy::Lazy<RegionLeaderResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionLeaderResponse,
        };
        unsafe {
            instance.get(RegionLeaderResponse::new)
        }
    }

    // optional .metapb.Peer leader = 1;

    pub fn clear_leader(&mut self) {
        self.leader.clear();
    }

    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader(&mut self, v: super::metapb::Peer) {
        self.leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leader(&mut self) -> &mut super::metapb::Peer {
        if self.leader.is_none() {
            self.leader.set_default();
        };
        self.leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_leader(&mut self) -> super::metapb::Peer {
        self.leader.take().unwrap_or_else(|| super::metapb::Peer::new())
    }

    pub fn get_leader(&self) -> &super::metapb::Peer {
        self.leader.as_ref().unwrap_or_else(|| super::metapb::Peer::default_instance())
    }

    fn get_leader_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Peer> {
        &self.leader
    }

    fn mut_leader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Peer> {
        &mut self.leader
    }
}

impl ::protobuf::Message for RegionLeaderResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.leader)?;
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
        if let Some(v) = self.leader.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.leader.as_ref() {
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

impl ::protobuf::MessageStatic for RegionLeaderResponse {
    fn new() -> RegionLeaderResponse {
        RegionLeaderResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionLeaderResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Peer>>(
                    "leader",
                    RegionLeaderResponse::get_leader_for_reflect,
                    RegionLeaderResponse::mut_leader_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionLeaderResponse>(
                    "RegionLeaderResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionLeaderResponse {
    fn clear(&mut self) {
        self.clear_leader();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegionLeaderResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionLeaderResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegionDetailRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionDetailRequest {}

impl RegionDetailRequest {
    pub fn new() -> RegionDetailRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionDetailRequest {
        static mut instance: ::protobuf::lazy::Lazy<RegionDetailRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionDetailRequest,
        };
        unsafe {
            instance.get(RegionDetailRequest::new)
        }
    }
}

impl ::protobuf::Message for RegionDetailRequest {
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

impl ::protobuf::MessageStatic for RegionDetailRequest {
    fn new() -> RegionDetailRequest {
        RegionDetailRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionDetailRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<RegionDetailRequest>(
                    "RegionDetailRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionDetailRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegionDetailRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionDetailRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegionDetailResponse {
    // message fields
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    leader: ::protobuf::SingularPtrField<super::metapb::Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionDetailResponse {}

impl RegionDetailResponse {
    pub fn new() -> RegionDetailResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionDetailResponse {
        static mut instance: ::protobuf::lazy::Lazy<RegionDetailResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionDetailResponse,
        };
        unsafe {
            instance.get(RegionDetailResponse::new)
        }
    }

    // optional .metapb.Region region = 1;

    pub fn clear_region(&mut self) {
        self.region.clear();
    }

    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region.set_default();
        };
        self.region.as_mut().unwrap()
    }

    // Take field
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region.take().unwrap_or_else(|| super::metapb::Region::new())
    }

    pub fn get_region(&self) -> &super::metapb::Region {
        self.region.as_ref().unwrap_or_else(|| super::metapb::Region::default_instance())
    }

    fn get_region_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Region> {
        &self.region
    }

    fn mut_region_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Region> {
        &mut self.region
    }

    // optional .metapb.Peer leader = 2;

    pub fn clear_leader(&mut self) {
        self.leader.clear();
    }

    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader(&mut self, v: super::metapb::Peer) {
        self.leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leader(&mut self) -> &mut super::metapb::Peer {
        if self.leader.is_none() {
            self.leader.set_default();
        };
        self.leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_leader(&mut self) -> super::metapb::Peer {
        self.leader.take().unwrap_or_else(|| super::metapb::Peer::new())
    }

    pub fn get_leader(&self) -> &super::metapb::Peer {
        self.leader.as_ref().unwrap_or_else(|| super::metapb::Peer::default_instance())
    }

    fn get_leader_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Peer> {
        &self.leader
    }

    fn mut_leader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Peer> {
        &mut self.leader
    }
}

impl ::protobuf::Message for RegionDetailResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.leader)?;
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
        if let Some(v) = self.region.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.leader.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.leader.as_ref() {
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

impl ::protobuf::MessageStatic for RegionDetailResponse {
    fn new() -> RegionDetailResponse {
        RegionDetailResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionDetailResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Region>>(
                    "region",
                    RegionDetailResponse::get_region_for_reflect,
                    RegionDetailResponse::mut_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Peer>>(
                    "leader",
                    RegionDetailResponse::get_leader_for_reflect,
                    RegionDetailResponse::mut_leader_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionDetailResponse>(
                    "RegionDetailResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionDetailResponse {
    fn clear(&mut self) {
        self.clear_region();
        self.clear_leader();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegionDetailResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionDetailResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatusRequest {
    // message fields
    cmd_type: ::std::option::Option<StatusCmdType>,
    region_leader: ::protobuf::SingularPtrField<RegionLeaderRequest>,
    region_detail: ::protobuf::SingularPtrField<RegionDetailRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatusRequest {}

impl StatusRequest {
    pub fn new() -> StatusRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusRequest {
        static mut instance: ::protobuf::lazy::Lazy<StatusRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusRequest,
        };
        unsafe {
            instance.get(StatusRequest::new)
        }
    }

    // optional .raft_cmdpb.StatusCmdType cmd_type = 1;

    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = ::std::option::Option::None;
    }

    pub fn has_cmd_type(&self) -> bool {
        self.cmd_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_type(&mut self, v: StatusCmdType) {
        self.cmd_type = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_type(&self) -> StatusCmdType {
        self.cmd_type.unwrap_or(StatusCmdType::InvalidStatus)
    }

    fn get_cmd_type_for_reflect(&self) -> &::std::option::Option<StatusCmdType> {
        &self.cmd_type
    }

    fn mut_cmd_type_for_reflect(&mut self) -> &mut ::std::option::Option<StatusCmdType> {
        &mut self.cmd_type
    }

    // optional .raft_cmdpb.RegionLeaderRequest region_leader = 2;

    pub fn clear_region_leader(&mut self) {
        self.region_leader.clear();
    }

    pub fn has_region_leader(&self) -> bool {
        self.region_leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_leader(&mut self, v: RegionLeaderRequest) {
        self.region_leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_leader(&mut self) -> &mut RegionLeaderRequest {
        if self.region_leader.is_none() {
            self.region_leader.set_default();
        };
        self.region_leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_leader(&mut self) -> RegionLeaderRequest {
        self.region_leader.take().unwrap_or_else(|| RegionLeaderRequest::new())
    }

    pub fn get_region_leader(&self) -> &RegionLeaderRequest {
        self.region_leader.as_ref().unwrap_or_else(|| RegionLeaderRequest::default_instance())
    }

    fn get_region_leader_for_reflect(&self) -> &::protobuf::SingularPtrField<RegionLeaderRequest> {
        &self.region_leader
    }

    fn mut_region_leader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RegionLeaderRequest> {
        &mut self.region_leader
    }

    // optional .raft_cmdpb.RegionDetailRequest region_detail = 3;

    pub fn clear_region_detail(&mut self) {
        self.region_detail.clear();
    }

    pub fn has_region_detail(&self) -> bool {
        self.region_detail.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_detail(&mut self, v: RegionDetailRequest) {
        self.region_detail = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_detail(&mut self) -> &mut RegionDetailRequest {
        if self.region_detail.is_none() {
            self.region_detail.set_default();
        };
        self.region_detail.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_detail(&mut self) -> RegionDetailRequest {
        self.region_detail.take().unwrap_or_else(|| RegionDetailRequest::new())
    }

    pub fn get_region_detail(&self) -> &RegionDetailRequest {
        self.region_detail.as_ref().unwrap_or_else(|| RegionDetailRequest::default_instance())
    }

    fn get_region_detail_for_reflect(&self) -> &::protobuf::SingularPtrField<RegionDetailRequest> {
        &self.region_detail
    }

    fn mut_region_detail_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RegionDetailRequest> {
        &mut self.region_detail
    }
}

impl ::protobuf::Message for StatusRequest {
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
                    self.cmd_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_leader)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_detail)?;
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
        if let Some(v) = self.cmd_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.region_leader.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.region_detail.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmd_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.region_leader.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.region_detail.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for StatusRequest {
    fn new() -> StatusRequest {
        StatusRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StatusCmdType>>(
                    "cmd_type",
                    StatusRequest::get_cmd_type_for_reflect,
                    StatusRequest::mut_cmd_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RegionLeaderRequest>>(
                    "region_leader",
                    StatusRequest::get_region_leader_for_reflect,
                    StatusRequest::mut_region_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RegionDetailRequest>>(
                    "region_detail",
                    StatusRequest::get_region_detail_for_reflect,
                    StatusRequest::mut_region_detail_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatusRequest>(
                    "StatusRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusRequest {
    fn clear(&mut self) {
        self.clear_cmd_type();
        self.clear_region_leader();
        self.clear_region_detail();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatusRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatusRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StatusResponse {
    // message fields
    cmd_type: ::std::option::Option<StatusCmdType>,
    region_leader: ::protobuf::SingularPtrField<RegionLeaderResponse>,
    region_detail: ::protobuf::SingularPtrField<RegionDetailResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StatusResponse {}

impl StatusResponse {
    pub fn new() -> StatusResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StatusResponse {
        static mut instance: ::protobuf::lazy::Lazy<StatusResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StatusResponse,
        };
        unsafe {
            instance.get(StatusResponse::new)
        }
    }

    // optional .raft_cmdpb.StatusCmdType cmd_type = 1;

    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = ::std::option::Option::None;
    }

    pub fn has_cmd_type(&self) -> bool {
        self.cmd_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_type(&mut self, v: StatusCmdType) {
        self.cmd_type = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_type(&self) -> StatusCmdType {
        self.cmd_type.unwrap_or(StatusCmdType::InvalidStatus)
    }

    fn get_cmd_type_for_reflect(&self) -> &::std::option::Option<StatusCmdType> {
        &self.cmd_type
    }

    fn mut_cmd_type_for_reflect(&mut self) -> &mut ::std::option::Option<StatusCmdType> {
        &mut self.cmd_type
    }

    // optional .raft_cmdpb.RegionLeaderResponse region_leader = 2;

    pub fn clear_region_leader(&mut self) {
        self.region_leader.clear();
    }

    pub fn has_region_leader(&self) -> bool {
        self.region_leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_leader(&mut self, v: RegionLeaderResponse) {
        self.region_leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_leader(&mut self) -> &mut RegionLeaderResponse {
        if self.region_leader.is_none() {
            self.region_leader.set_default();
        };
        self.region_leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_leader(&mut self) -> RegionLeaderResponse {
        self.region_leader.take().unwrap_or_else(|| RegionLeaderResponse::new())
    }

    pub fn get_region_leader(&self) -> &RegionLeaderResponse {
        self.region_leader.as_ref().unwrap_or_else(|| RegionLeaderResponse::default_instance())
    }

    fn get_region_leader_for_reflect(&self) -> &::protobuf::SingularPtrField<RegionLeaderResponse> {
        &self.region_leader
    }

    fn mut_region_leader_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RegionLeaderResponse> {
        &mut self.region_leader
    }

    // optional .raft_cmdpb.RegionDetailResponse region_detail = 3;

    pub fn clear_region_detail(&mut self) {
        self.region_detail.clear();
    }

    pub fn has_region_detail(&self) -> bool {
        self.region_detail.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_detail(&mut self, v: RegionDetailResponse) {
        self.region_detail = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_detail(&mut self) -> &mut RegionDetailResponse {
        if self.region_detail.is_none() {
            self.region_detail.set_default();
        };
        self.region_detail.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_detail(&mut self) -> RegionDetailResponse {
        self.region_detail.take().unwrap_or_else(|| RegionDetailResponse::new())
    }

    pub fn get_region_detail(&self) -> &RegionDetailResponse {
        self.region_detail.as_ref().unwrap_or_else(|| RegionDetailResponse::default_instance())
    }

    fn get_region_detail_for_reflect(&self) -> &::protobuf::SingularPtrField<RegionDetailResponse> {
        &self.region_detail
    }

    fn mut_region_detail_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RegionDetailResponse> {
        &mut self.region_detail
    }
}

impl ::protobuf::Message for StatusResponse {
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
                    self.cmd_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_leader)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_detail)?;
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
        if let Some(v) = self.cmd_type {
            my_size += ::protobuf::rt::enum_size(1, v);
        };
        if let Some(v) = self.region_leader.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.region_detail.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cmd_type {
            os.write_enum(1, v.value())?;
        };
        if let Some(v) = self.region_leader.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.region_detail.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for StatusResponse {
    fn new() -> StatusResponse {
        StatusResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StatusResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeEnum<StatusCmdType>>(
                    "cmd_type",
                    StatusResponse::get_cmd_type_for_reflect,
                    StatusResponse::mut_cmd_type_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RegionLeaderResponse>>(
                    "region_leader",
                    StatusResponse::get_region_leader_for_reflect,
                    StatusResponse::mut_region_leader_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RegionDetailResponse>>(
                    "region_detail",
                    StatusResponse::get_region_detail_for_reflect,
                    StatusResponse::mut_region_detail_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StatusResponse>(
                    "StatusResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StatusResponse {
    fn clear(&mut self) {
        self.clear_cmd_type();
        self.clear_region_leader();
        self.clear_region_detail();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StatusResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StatusResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RaftRequestHeader {
    // message fields
    region_id: ::std::option::Option<u64>,
    peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    read_quorum: ::std::option::Option<bool>,
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    region_epoch: ::protobuf::SingularPtrField<super::metapb::RegionEpoch>,
    term: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftRequestHeader {}

impl RaftRequestHeader {
    pub fn new() -> RaftRequestHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftRequestHeader {
        static mut instance: ::protobuf::lazy::Lazy<RaftRequestHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftRequestHeader,
        };
        unsafe {
            instance.get(RaftRequestHeader::new)
        }
    }

    // optional uint64 region_id = 1;

    pub fn clear_region_id(&mut self) {
        self.region_id = ::std::option::Option::None;
    }

    pub fn has_region_id(&self) -> bool {
        self.region_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = ::std::option::Option::Some(v);
    }

    pub fn get_region_id(&self) -> u64 {
        self.region_id.unwrap_or(0)
    }

    fn get_region_id_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.region_id
    }

    fn mut_region_id_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.region_id
    }

    // optional .metapb.Peer peer = 2;

    pub fn clear_peer(&mut self) {
        self.peer.clear();
    }

    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_peer(&mut self, v: super::metapb::Peer) {
        self.peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_peer(&mut self) -> &mut super::metapb::Peer {
        if self.peer.is_none() {
            self.peer.set_default();
        };
        self.peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_peer(&mut self) -> super::metapb::Peer {
        self.peer.take().unwrap_or_else(|| super::metapb::Peer::new())
    }

    pub fn get_peer(&self) -> &super::metapb::Peer {
        self.peer.as_ref().unwrap_or_else(|| super::metapb::Peer::default_instance())
    }

    fn get_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Peer> {
        &self.peer
    }

    fn mut_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Peer> {
        &mut self.peer
    }

    // optional bool read_quorum = 3;

    pub fn clear_read_quorum(&mut self) {
        self.read_quorum = ::std::option::Option::None;
    }

    pub fn has_read_quorum(&self) -> bool {
        self.read_quorum.is_some()
    }

    // Param is passed by value, moved
    pub fn set_read_quorum(&mut self, v: bool) {
        self.read_quorum = ::std::option::Option::Some(v);
    }

    pub fn get_read_quorum(&self) -> bool {
        self.read_quorum.unwrap_or(false)
    }

    fn get_read_quorum_for_reflect(&self) -> &::std::option::Option<bool> {
        &self.read_quorum
    }

    fn mut_read_quorum_for_reflect(&mut self) -> &mut ::std::option::Option<bool> {
        &mut self.read_quorum
    }

    // optional bytes uuid = 4;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        };
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_uuid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.uuid
    }

    // optional .metapb.RegionEpoch region_epoch = 5;

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
        };
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

    // optional uint64 term = 6;

    pub fn clear_term(&mut self) {
        self.term = ::std::option::Option::None;
    }

    pub fn has_term(&self) -> bool {
        self.term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = ::std::option::Option::Some(v);
    }

    pub fn get_term(&self) -> u64 {
        self.term.unwrap_or(0)
    }

    fn get_term_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.term
    }
}

impl ::protobuf::Message for RaftRequestHeader {
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
                    self.region_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.read_quorum = ::std::option::Option::Some(tmp);
                },
                4 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.uuid)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_epoch)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.term = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.region_id {
            my_size += ::protobuf::rt::value_size(1, v, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.read_quorum {
            my_size += 2;
        };
        if let Some(v) = self.uuid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(4, &v);
        };
        if let Some(v) = self.region_epoch.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.term {
            my_size += ::protobuf::rt::value_size(6, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region_id {
            os.write_uint64(1, v)?;
        };
        if let Some(v) = self.peer.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.read_quorum {
            os.write_bool(3, v)?;
        };
        if let Some(v) = self.uuid.as_ref() {
            os.write_bytes(4, &v)?;
        };
        if let Some(v) = self.region_epoch.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.term {
            os.write_uint64(6, v)?;
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

impl ::protobuf::MessageStatic for RaftRequestHeader {
    fn new() -> RaftRequestHeader {
        RaftRequestHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftRequestHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "region_id",
                    RaftRequestHeader::get_region_id_for_reflect,
                    RaftRequestHeader::mut_region_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Peer>>(
                    "peer",
                    RaftRequestHeader::get_peer_for_reflect,
                    RaftRequestHeader::mut_peer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "read_quorum",
                    RaftRequestHeader::get_read_quorum_for_reflect,
                    RaftRequestHeader::mut_read_quorum_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    RaftRequestHeader::get_uuid_for_reflect,
                    RaftRequestHeader::mut_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::RegionEpoch>>(
                    "region_epoch",
                    RaftRequestHeader::get_region_epoch_for_reflect,
                    RaftRequestHeader::mut_region_epoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    RaftRequestHeader::get_term_for_reflect,
                    RaftRequestHeader::mut_term_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftRequestHeader>(
                    "RaftRequestHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftRequestHeader {
    fn clear(&mut self) {
        self.clear_region_id();
        self.clear_peer();
        self.clear_read_quorum();
        self.clear_uuid();
        self.clear_region_epoch();
        self.clear_term();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftRequestHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftRequestHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RaftResponseHeader {
    // message fields
    error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    current_term: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftResponseHeader {}

impl RaftResponseHeader {
    pub fn new() -> RaftResponseHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftResponseHeader {
        static mut instance: ::protobuf::lazy::Lazy<RaftResponseHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftResponseHeader,
        };
        unsafe {
            instance.get(RaftResponseHeader::new)
        }
    }

    // optional .errorpb.Error error = 1;

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
        };
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

    // optional bytes uuid = 2;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    pub fn has_uuid(&self) -> bool {
        self.uuid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        };
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    fn get_uuid_for_reflect(&self) -> &::protobuf::SingularField<::std::vec::Vec<u8>> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::protobuf::SingularField<::std::vec::Vec<u8>> {
        &mut self.uuid
    }

    // optional uint64 current_term = 3;

    pub fn clear_current_term(&mut self) {
        self.current_term = ::std::option::Option::None;
    }

    pub fn has_current_term(&self) -> bool {
        self.current_term.is_some()
    }

    // Param is passed by value, moved
    pub fn set_current_term(&mut self, v: u64) {
        self.current_term = ::std::option::Option::Some(v);
    }

    pub fn get_current_term(&self) -> u64 {
        self.current_term.unwrap_or(0)
    }

    fn get_current_term_for_reflect(&self) -> &::std::option::Option<u64> {
        &self.current_term
    }

    fn mut_current_term_for_reflect(&mut self) -> &mut ::std::option::Option<u64> {
        &mut self.current_term
    }
}

impl ::protobuf::Message for RaftResponseHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.uuid)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.current_term = ::std::option::Option::Some(tmp);
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
        if let Some(v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.uuid.as_ref() {
            my_size += ::protobuf::rt::bytes_size(2, &v);
        };
        if let Some(v) = self.current_term {
            my_size += ::protobuf::rt::value_size(3, v, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.uuid.as_ref() {
            os.write_bytes(2, &v)?;
        };
        if let Some(v) = self.current_term {
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

impl ::protobuf::MessageStatic for RaftResponseHeader {
    fn new() -> RaftResponseHeader {
        RaftResponseHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftResponseHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "error",
                    RaftResponseHeader::get_error_for_reflect,
                    RaftResponseHeader::mut_error_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    RaftResponseHeader::get_uuid_for_reflect,
                    RaftResponseHeader::mut_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_option_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "current_term",
                    RaftResponseHeader::get_current_term_for_reflect,
                    RaftResponseHeader::mut_current_term_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftResponseHeader>(
                    "RaftResponseHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftResponseHeader {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_uuid();
        self.clear_current_term();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftResponseHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftResponseHeader {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RaftCmdRequest {
    // message fields
    header: ::protobuf::SingularPtrField<RaftRequestHeader>,
    requests: ::protobuf::RepeatedField<Request>,
    admin_request: ::protobuf::SingularPtrField<AdminRequest>,
    status_request: ::protobuf::SingularPtrField<StatusRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftCmdRequest {}

impl RaftCmdRequest {
    pub fn new() -> RaftCmdRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftCmdRequest {
        static mut instance: ::protobuf::lazy::Lazy<RaftCmdRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftCmdRequest,
        };
        unsafe {
            instance.get(RaftCmdRequest::new)
        }
    }

    // optional .raft_cmdpb.RaftRequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RaftRequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut RaftRequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RaftRequestHeader {
        self.header.take().unwrap_or_else(|| RaftRequestHeader::new())
    }

    pub fn get_header(&self) -> &RaftRequestHeader {
        self.header.as_ref().unwrap_or_else(|| RaftRequestHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<RaftRequestHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RaftRequestHeader> {
        &mut self.header
    }

    // repeated .raft_cmdpb.Request requests = 2;

    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }

    // Param is passed by value, moved
    pub fn set_requests(&mut self, v: ::protobuf::RepeatedField<Request>) {
        self.requests = v;
    }

    // Mutable pointer to the field.
    pub fn mut_requests(&mut self) -> &mut ::protobuf::RepeatedField<Request> {
        &mut self.requests
    }

    // Take field
    pub fn take_requests(&mut self) -> ::protobuf::RepeatedField<Request> {
        ::std::mem::replace(&mut self.requests, ::protobuf::RepeatedField::new())
    }

    pub fn get_requests(&self) -> &[Request] {
        &self.requests
    }

    fn get_requests_for_reflect(&self) -> &::protobuf::RepeatedField<Request> {
        &self.requests
    }

    fn mut_requests_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Request> {
        &mut self.requests
    }

    // optional .raft_cmdpb.AdminRequest admin_request = 3;

    pub fn clear_admin_request(&mut self) {
        self.admin_request.clear();
    }

    pub fn has_admin_request(&self) -> bool {
        self.admin_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_admin_request(&mut self, v: AdminRequest) {
        self.admin_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_admin_request(&mut self) -> &mut AdminRequest {
        if self.admin_request.is_none() {
            self.admin_request.set_default();
        };
        self.admin_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_admin_request(&mut self) -> AdminRequest {
        self.admin_request.take().unwrap_or_else(|| AdminRequest::new())
    }

    pub fn get_admin_request(&self) -> &AdminRequest {
        self.admin_request.as_ref().unwrap_or_else(|| AdminRequest::default_instance())
    }

    fn get_admin_request_for_reflect(&self) -> &::protobuf::SingularPtrField<AdminRequest> {
        &self.admin_request
    }

    fn mut_admin_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AdminRequest> {
        &mut self.admin_request
    }

    // optional .raft_cmdpb.StatusRequest status_request = 4;

    pub fn clear_status_request(&mut self) {
        self.status_request.clear();
    }

    pub fn has_status_request(&self) -> bool {
        self.status_request.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_request(&mut self, v: StatusRequest) {
        self.status_request = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status_request(&mut self) -> &mut StatusRequest {
        if self.status_request.is_none() {
            self.status_request.set_default();
        };
        self.status_request.as_mut().unwrap()
    }

    // Take field
    pub fn take_status_request(&mut self) -> StatusRequest {
        self.status_request.take().unwrap_or_else(|| StatusRequest::new())
    }

    pub fn get_status_request(&self) -> &StatusRequest {
        self.status_request.as_ref().unwrap_or_else(|| StatusRequest::default_instance())
    }

    fn get_status_request_for_reflect(&self) -> &::protobuf::SingularPtrField<StatusRequest> {
        &self.status_request
    }

    fn mut_status_request_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StatusRequest> {
        &mut self.status_request
    }
}

impl ::protobuf::Message for RaftCmdRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.requests)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.admin_request)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status_request)?;
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
        if let Some(v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.requests {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.admin_request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.status_request.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.requests {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.admin_request.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.status_request.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RaftCmdRequest {
    fn new() -> RaftCmdRequest {
        RaftCmdRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftCmdRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RaftRequestHeader>>(
                    "header",
                    RaftCmdRequest::get_header_for_reflect,
                    RaftCmdRequest::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Request>>(
                    "requests",
                    RaftCmdRequest::get_requests_for_reflect,
                    RaftCmdRequest::mut_requests_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AdminRequest>>(
                    "admin_request",
                    RaftCmdRequest::get_admin_request_for_reflect,
                    RaftCmdRequest::mut_admin_request_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StatusRequest>>(
                    "status_request",
                    RaftCmdRequest::get_status_request_for_reflect,
                    RaftCmdRequest::mut_status_request_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftCmdRequest>(
                    "RaftCmdRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftCmdRequest {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_requests();
        self.clear_admin_request();
        self.clear_status_request();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftCmdRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftCmdRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RaftCmdResponse {
    // message fields
    header: ::protobuf::SingularPtrField<RaftResponseHeader>,
    responses: ::protobuf::RepeatedField<Response>,
    admin_response: ::protobuf::SingularPtrField<AdminResponse>,
    status_response: ::protobuf::SingularPtrField<StatusResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftCmdResponse {}

impl RaftCmdResponse {
    pub fn new() -> RaftCmdResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftCmdResponse {
        static mut instance: ::protobuf::lazy::Lazy<RaftCmdResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftCmdResponse,
        };
        unsafe {
            instance.get(RaftCmdResponse::new)
        }
    }

    // optional .raft_cmdpb.RaftResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RaftResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut RaftResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RaftResponseHeader {
        self.header.take().unwrap_or_else(|| RaftResponseHeader::new())
    }

    pub fn get_header(&self) -> &RaftResponseHeader {
        self.header.as_ref().unwrap_or_else(|| RaftResponseHeader::default_instance())
    }

    fn get_header_for_reflect(&self) -> &::protobuf::SingularPtrField<RaftResponseHeader> {
        &self.header
    }

    fn mut_header_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RaftResponseHeader> {
        &mut self.header
    }

    // repeated .raft_cmdpb.Response responses = 2;

    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }

    // Param is passed by value, moved
    pub fn set_responses(&mut self, v: ::protobuf::RepeatedField<Response>) {
        self.responses = v;
    }

    // Mutable pointer to the field.
    pub fn mut_responses(&mut self) -> &mut ::protobuf::RepeatedField<Response> {
        &mut self.responses
    }

    // Take field
    pub fn take_responses(&mut self) -> ::protobuf::RepeatedField<Response> {
        ::std::mem::replace(&mut self.responses, ::protobuf::RepeatedField::new())
    }

    pub fn get_responses(&self) -> &[Response] {
        &self.responses
    }

    fn get_responses_for_reflect(&self) -> &::protobuf::RepeatedField<Response> {
        &self.responses
    }

    fn mut_responses_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Response> {
        &mut self.responses
    }

    // optional .raft_cmdpb.AdminResponse admin_response = 3;

    pub fn clear_admin_response(&mut self) {
        self.admin_response.clear();
    }

    pub fn has_admin_response(&self) -> bool {
        self.admin_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_admin_response(&mut self, v: AdminResponse) {
        self.admin_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_admin_response(&mut self) -> &mut AdminResponse {
        if self.admin_response.is_none() {
            self.admin_response.set_default();
        };
        self.admin_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_admin_response(&mut self) -> AdminResponse {
        self.admin_response.take().unwrap_or_else(|| AdminResponse::new())
    }

    pub fn get_admin_response(&self) -> &AdminResponse {
        self.admin_response.as_ref().unwrap_or_else(|| AdminResponse::default_instance())
    }

    fn get_admin_response_for_reflect(&self) -> &::protobuf::SingularPtrField<AdminResponse> {
        &self.admin_response
    }

    fn mut_admin_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<AdminResponse> {
        &mut self.admin_response
    }

    // optional .raft_cmdpb.StatusResponse status_response = 4;

    pub fn clear_status_response(&mut self) {
        self.status_response.clear();
    }

    pub fn has_status_response(&self) -> bool {
        self.status_response.is_some()
    }

    // Param is passed by value, moved
    pub fn set_status_response(&mut self, v: StatusResponse) {
        self.status_response = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_status_response(&mut self) -> &mut StatusResponse {
        if self.status_response.is_none() {
            self.status_response.set_default();
        };
        self.status_response.as_mut().unwrap()
    }

    // Take field
    pub fn take_status_response(&mut self) -> StatusResponse {
        self.status_response.take().unwrap_or_else(|| StatusResponse::new())
    }

    pub fn get_status_response(&self) -> &StatusResponse {
        self.status_response.as_ref().unwrap_or_else(|| StatusResponse::default_instance())
    }

    fn get_status_response_for_reflect(&self) -> &::protobuf::SingularPtrField<StatusResponse> {
        &self.status_response
    }

    fn mut_status_response_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<StatusResponse> {
        &mut self.status_response
    }
}

impl ::protobuf::Message for RaftCmdResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header)?;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.responses)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.admin_response)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.status_response)?;
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
        if let Some(v) = self.header.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.responses {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.admin_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.status_response.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        for v in &self.responses {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.admin_response.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.status_response.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RaftCmdResponse {
    fn new() -> RaftCmdResponse {
        RaftCmdResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftCmdResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RaftResponseHeader>>(
                    "header",
                    RaftCmdResponse::get_header_for_reflect,
                    RaftCmdResponse::mut_header_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Response>>(
                    "responses",
                    RaftCmdResponse::get_responses_for_reflect,
                    RaftCmdResponse::mut_responses_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<AdminResponse>>(
                    "admin_response",
                    RaftCmdResponse::get_admin_response_for_reflect,
                    RaftCmdResponse::mut_admin_response_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<StatusResponse>>(
                    "status_response",
                    RaftCmdResponse::get_status_response_for_reflect,
                    RaftCmdResponse::mut_status_response_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftCmdResponse>(
                    "RaftCmdResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftCmdResponse {
    fn clear(&mut self) {
        self.clear_header();
        self.clear_responses();
        self.clear_admin_response();
        self.clear_status_response();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftCmdResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftCmdResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CmdType {
    Invalid = 0,
    Get = 1,
    Put = 3,
    Delete = 4,
    Snap = 5,
    Prewrite = 6,
    Commit = 7,
}

impl ::protobuf::ProtobufEnum for CmdType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CmdType> {
        match value {
            0 => ::std::option::Option::Some(CmdType::Invalid),
            1 => ::std::option::Option::Some(CmdType::Get),
            3 => ::std::option::Option::Some(CmdType::Put),
            4 => ::std::option::Option::Some(CmdType::Delete),
            5 => ::std::option::Option::Some(CmdType::Snap),
            6 => ::std::option::Option::Some(CmdType::Prewrite),
            7 => ::std::option::Option::Some(CmdType::Commit),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CmdType] = &[
            CmdType::Invalid,
            CmdType::Get,
            CmdType::Put,
            CmdType::Delete,
            CmdType::Snap,
            CmdType::Prewrite,
            CmdType::Commit,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CmdType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CmdType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CmdType {
}

impl ::protobuf::reflect::ProtobufValue for CmdType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum AdminCmdType {
    InvalidAdmin = 0,
    ChangePeer = 1,
    Split = 2,
    CompactLog = 3,
    TransferLeader = 4,
    ComputeHash = 5,
    VerifyHash = 6,
}

impl ::protobuf::ProtobufEnum for AdminCmdType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<AdminCmdType> {
        match value {
            0 => ::std::option::Option::Some(AdminCmdType::InvalidAdmin),
            1 => ::std::option::Option::Some(AdminCmdType::ChangePeer),
            2 => ::std::option::Option::Some(AdminCmdType::Split),
            3 => ::std::option::Option::Some(AdminCmdType::CompactLog),
            4 => ::std::option::Option::Some(AdminCmdType::TransferLeader),
            5 => ::std::option::Option::Some(AdminCmdType::ComputeHash),
            6 => ::std::option::Option::Some(AdminCmdType::VerifyHash),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [AdminCmdType] = &[
            AdminCmdType::InvalidAdmin,
            AdminCmdType::ChangePeer,
            AdminCmdType::Split,
            AdminCmdType::CompactLog,
            AdminCmdType::TransferLeader,
            AdminCmdType::ComputeHash,
            AdminCmdType::VerifyHash,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<AdminCmdType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("AdminCmdType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for AdminCmdType {
}

impl ::protobuf::reflect::ProtobufValue for AdminCmdType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum StatusCmdType {
    InvalidStatus = 0,
    RegionLeader = 1,
    RegionDetail = 2,
}

impl ::protobuf::ProtobufEnum for StatusCmdType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<StatusCmdType> {
        match value {
            0 => ::std::option::Option::Some(StatusCmdType::InvalidStatus),
            1 => ::std::option::Option::Some(StatusCmdType::RegionLeader),
            2 => ::std::option::Option::Some(StatusCmdType::RegionDetail),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [StatusCmdType] = &[
            StatusCmdType::InvalidStatus,
            StatusCmdType::RegionLeader,
            StatusCmdType::RegionDetail,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<StatusCmdType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("StatusCmdType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for StatusCmdType {
}

impl ::protobuf::reflect::ProtobufValue for StatusCmdType {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x10, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x0a, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x1a, 0x0c,
    0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0d, 0x65, 0x72,
    0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0d, 0x65, 0x72, 0x61,
    0x66, 0x74, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x2e, 0x0a, 0x0a, 0x47, 0x65,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x63, 0x66, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x63, 0x66, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x22, 0x23, 0x0a, 0x0b, 0x47, 0x65,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22,
    0x44, 0x0a, 0x0a, 0x50, 0x75, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0e, 0x0a,
    0x02, 0x63, 0x66, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x63, 0x66, 0x12, 0x10, 0x0a,
    0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12,
    0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x0d, 0x0a, 0x0b, 0x50, 0x75, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x22, 0x31, 0x0a, 0x0d, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0e, 0x0a, 0x02, 0x63, 0x66, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x02, 0x63, 0x66, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x22, 0x10, 0x0a, 0x0e, 0x44, 0x65, 0x6c, 0x65, 0x74,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x0d, 0x0a, 0x0b, 0x53, 0x6e, 0x61,
    0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x36, 0x0a, 0x0c, 0x53, 0x6e, 0x61, 0x70,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70,
    0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x22, 0x4d, 0x0a, 0x0f, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x12, 0x0a, 0x04, 0x6c,
    0x6f, 0x63, 0x6b, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x6c, 0x6f, 0x63, 0x6b, 0x22,
    0x12, 0x0a, 0x10, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x22, 0x37, 0x0a, 0x0d, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x10, 0x0a, 0x0e,
    0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0xd9,
    0x02, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2e, 0x0a, 0x08, 0x63, 0x6d,
    0x64, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x72,
    0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x54, 0x79, 0x70,
    0x65, 0x52, 0x07, 0x63, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x12, 0x28, 0x0a, 0x03, 0x67, 0x65,
    0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63,
    0x6d, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52,
    0x03, 0x67, 0x65, 0x74, 0x12, 0x28, 0x0a, 0x03, 0x70, 0x75, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x16, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x50,
    0x75, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x03, 0x70, 0x75, 0x74, 0x12, 0x31,
    0x0a, 0x06, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19,
    0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x44, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x06, 0x64, 0x65, 0x6c, 0x65, 0x74,
    0x65, 0x12, 0x2b, 0x0a, 0x04, 0x73, 0x6e, 0x61, 0x70, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x17, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x53, 0x6e, 0x61,
    0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x04, 0x73, 0x6e, 0x61, 0x70, 0x12, 0x37,
    0x0a, 0x08, 0x70, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1b, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x50, 0x72,
    0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x08, 0x70,
    0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x12, 0x31, 0x0a, 0x06, 0x63, 0x6f, 0x6d, 0x6d, 0x69,
    0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63,
    0x6d, 0x64, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x52, 0x06, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x22, 0xe0, 0x02, 0x0a, 0x08, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2e, 0x0a, 0x08, 0x63, 0x6d, 0x64, 0x5f, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x13, 0x2e, 0x72, 0x61, 0x66, 0x74,
    0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x52, 0x07,
    0x63, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x12, 0x29, 0x0a, 0x03, 0x67, 0x65, 0x74, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70,
    0x62, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x03, 0x67,
    0x65, 0x74, 0x12, 0x29, 0x0a, 0x03, 0x70, 0x75, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x17, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x50, 0x75, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x03, 0x70, 0x75, 0x74, 0x12, 0x32, 0x0a,
    0x06, 0x64, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
    0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x44, 0x65, 0x6c, 0x65, 0x74,
    0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x06, 0x64, 0x65, 0x6c, 0x65, 0x74,
    0x65, 0x12, 0x2c, 0x0a, 0x04, 0x73, 0x6e, 0x61, 0x70, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x18, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x53, 0x6e, 0x61,
    0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x04, 0x73, 0x6e, 0x61, 0x70, 0x12,
    0x38, 0x0a, 0x08, 0x70, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x1c, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x50,
    0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52,
    0x08, 0x70, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x12, 0x32, 0x0a, 0x06, 0x63, 0x6f, 0x6d,
    0x6d, 0x69, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x72, 0x61, 0x66, 0x74,
    0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x06, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x22, 0x6f, 0x0a,
    0x11, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x38, 0x0a, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x65, 0x72, 0x61, 0x66, 0x74, 0x70,
    0x62, 0x2e, 0x43, 0x6f, 0x6e, 0x66, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65,
    0x52, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x20, 0x0a, 0x04,
    0x70, 0x65, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74,
    0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x52, 0x04, 0x70, 0x65, 0x65, 0x72, 0x22, 0x3c,
    0x0a, 0x12, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x22, 0x94, 0x01, 0x0a,
    0x0c, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1b, 0x0a,
    0x09, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x08, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x22, 0x0a, 0x0d, 0x6e, 0x65,
    0x77, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x0b, 0x6e, 0x65, 0x77, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x20,
    0x0a, 0x0c, 0x6e, 0x65, 0x77, 0x5f, 0x70, 0x65, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x03,
    0x20, 0x03, 0x28, 0x04, 0x52, 0x0a, 0x6e, 0x65, 0x77, 0x50, 0x65, 0x65, 0x72, 0x49, 0x64, 0x73,
    0x12, 0x21, 0x0a, 0x0c, 0x72, 0x69, 0x67, 0x68, 0x74, 0x5f, 0x64, 0x65, 0x72, 0x69, 0x76, 0x65,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x72, 0x69, 0x67, 0x68, 0x74, 0x44, 0x65, 0x72,
    0x69, 0x76, 0x65, 0x22, 0x59, 0x0a, 0x0d, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x22, 0x0a, 0x04, 0x6c, 0x65, 0x66, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x52, 0x04, 0x6c, 0x65, 0x66, 0x74, 0x12, 0x24, 0x0a, 0x05, 0x72, 0x69, 0x67, 0x68,
    0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62,
    0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x05, 0x72, 0x69, 0x67, 0x68, 0x74, 0x22, 0x5b,
    0x0a, 0x11, 0x43, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x4c, 0x6f, 0x67, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x23, 0x0a, 0x0d, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x5f, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x63, 0x6f, 0x6d, 0x70,
    0x61, 0x63, 0x74, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6f, 0x6d, 0x70,
    0x61, 0x63, 0x74, 0x5f, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b,
    0x63, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x54, 0x65, 0x72, 0x6d, 0x22, 0x14, 0x0a, 0x12, 0x43,
    0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x4c, 0x6f, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x22, 0x39, 0x0a, 0x15, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x4c, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x20, 0x0a, 0x04, 0x70, 0x65,
    0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70,
    0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x52, 0x04, 0x70, 0x65, 0x65, 0x72, 0x22, 0x18, 0x0a, 0x16,
    0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x3d, 0x0a, 0x11, 0x56, 0x65, 0x72, 0x69, 0x66, 0x79,
    0x48, 0x61, 0x73, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x14, 0x0a, 0x05, 0x69,
    0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65,
    0x78, 0x12, 0x12, 0x0a, 0x04, 0x68, 0x61, 0x73, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x04, 0x68, 0x61, 0x73, 0x68, 0x22, 0x14, 0x0a, 0x12, 0x56, 0x65, 0x72, 0x69, 0x66, 0x79, 0x48,
    0x61, 0x73, 0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0xff, 0x02, 0x0a, 0x0c,
    0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x33, 0x0a, 0x08,
    0x63, 0x6d, 0x64, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x18,
    0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x41, 0x64, 0x6d, 0x69,
    0x6e, 0x43, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x52, 0x07, 0x63, 0x6d, 0x64, 0x54, 0x79, 0x70,
    0x65, 0x12, 0x3e, 0x0a, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x70, 0x65, 0x65, 0x72,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d,
    0x64, 0x70, 0x62, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65,
    0x72, 0x12, 0x2e, 0x0a, 0x05, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x18, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x53, 0x70,
    0x6c, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x05, 0x73, 0x70, 0x6c, 0x69,
    0x74, 0x12, 0x3e, 0x0a, 0x0b, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x5f, 0x6c, 0x6f, 0x67,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d,
    0x64, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x4c, 0x6f, 0x67, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0a, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74, 0x4c, 0x6f,
    0x67, 0x12, 0x4a, 0x0a, 0x0f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x5f, 0x6c, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x72, 0x61, 0x66,
    0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72,
    0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0e, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x3e, 0x0a,
    0x0b, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79, 0x5f, 0x68, 0x61, 0x73, 0x68, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e,
    0x56, 0x65, 0x72, 0x69, 0x66, 0x79, 0x48, 0x61, 0x73, 0x68, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x52, 0x0a, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79, 0x48, 0x61, 0x73, 0x68, 0x22, 0x85, 0x03,
    0x0a, 0x0d, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x33, 0x0a, 0x08, 0x63, 0x6d, 0x64, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0e, 0x32, 0x18, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x41,
    0x64, 0x6d, 0x69, 0x6e, 0x43, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x52, 0x07, 0x63, 0x6d, 0x64,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x3f, 0x0a, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x70,
    0x65, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x72, 0x61, 0x66, 0x74,
    0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65,
    0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x50, 0x65, 0x65, 0x72, 0x12, 0x2f, 0x0a, 0x05, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70,
    0x62, 0x2e, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52,
    0x05, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x12, 0x3f, 0x0a, 0x0b, 0x63, 0x6f, 0x6d, 0x70, 0x61, 0x63,
    0x74, 0x5f, 0x6c, 0x6f, 0x67, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x72, 0x61,
    0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6d, 0x70, 0x61, 0x63, 0x74,
    0x4c, 0x6f, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0a, 0x63, 0x6f, 0x6d,
    0x70, 0x61, 0x63, 0x74, 0x4c, 0x6f, 0x67, 0x12, 0x4b, 0x0a, 0x0f, 0x74, 0x72, 0x61, 0x6e, 0x73,
    0x66, 0x65, 0x72, 0x5f, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x22, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x54, 0x72,
    0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0e, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x4c, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x12, 0x3f, 0x0a, 0x0b, 0x76, 0x65, 0x72, 0x69, 0x66, 0x79, 0x5f, 0x68,
    0x61, 0x73, 0x68, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x72, 0x61, 0x66, 0x74,
    0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x56, 0x65, 0x72, 0x69, 0x66, 0x79, 0x48, 0x61, 0x73,
    0x68, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0a, 0x76, 0x65, 0x72, 0x69, 0x66,
    0x79, 0x48, 0x61, 0x73, 0x68, 0x22, 0x15, 0x0a, 0x13, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4c,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x3c, 0x0a, 0x14,
    0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x24, 0x0a, 0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x50, 0x65,
    0x65, 0x72, 0x52, 0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0x15, 0x0a, 0x13, 0x52, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x22, 0x64, 0x0a, 0x14, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x44, 0x65, 0x74, 0x61, 0x69,
    0x6c, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61,
    0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x12, 0x24, 0x0a, 0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x52,
    0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0xd1, 0x01, 0x0a, 0x0d, 0x53, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x34, 0x0a, 0x08, 0x63, 0x6d, 0x64,
    0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x19, 0x2e, 0x72, 0x61,
    0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x43,
    0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x52, 0x07, 0x63, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x44, 0x0a, 0x0d, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d,
    0x64, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4c,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x44, 0x0a, 0x0d, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f,
    0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x72,
    0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0c, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x22, 0xd4, 0x01, 0x0a, 0x0e,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x34,
    0x0a, 0x08, 0x63, 0x6d, 0x64, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x19, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x53, 0x74,
    0x61, 0x74, 0x75, 0x73, 0x43, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x52, 0x07, 0x63, 0x6d, 0x64,
    0x54, 0x79, 0x70, 0x65, 0x12, 0x45, 0x0a, 0x0d, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x6c,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x72, 0x61,
    0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4c,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0c, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x45, 0x0a, 0x0d, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x64, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x20, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e,
    0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x44, 0x65, 0x74, 0x61, 0x69, 0x6c, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x44, 0x65, 0x74, 0x61,
    0x69, 0x6c, 0x22, 0xd3, 0x01, 0x0a, 0x11, 0x52, 0x61, 0x66, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x1b, 0x0a, 0x09, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x72, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x20, 0x0a, 0x04, 0x70, 0x65, 0x65, 0x72, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65,
    0x72, 0x52, 0x04, 0x70, 0x65, 0x65, 0x72, 0x12, 0x1f, 0x0a, 0x0b, 0x72, 0x65, 0x61, 0x64, 0x5f,
    0x71, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0a, 0x72, 0x65,
    0x61, 0x64, 0x51, 0x75, 0x6f, 0x72, 0x75, 0x6d, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12, 0x36, 0x0a, 0x0c,
    0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x52, 0x0b, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x45,
    0x70, 0x6f, 0x63, 0x68, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x06, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x22, 0x71, 0x0a, 0x12, 0x52, 0x61, 0x66, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x24,
    0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x75, 0x72, 0x72,
    0x65, 0x6e, 0x74, 0x5f, 0x74, 0x65, 0x72, 0x6d, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b,
    0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x54, 0x65, 0x72, 0x6d, 0x22, 0xf9, 0x01, 0x0a, 0x0e,
    0x52, 0x61, 0x66, 0x74, 0x43, 0x6d, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x35,
    0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d,
    0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x61, 0x66, 0x74,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2f, 0x0a, 0x08, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63,
    0x6d, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x08, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x12, 0x3d, 0x0a, 0x0d, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x5f,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e,
    0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x41, 0x64, 0x6d, 0x69, 0x6e,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0c, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x40, 0x0a, 0x0e, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x5f,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e,
    0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x53, 0x74, 0x61, 0x74, 0x75,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0d, 0x73, 0x74, 0x61, 0x74, 0x75, 0x73,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x84, 0x02, 0x0a, 0x0f, 0x52, 0x61, 0x66, 0x74,
    0x43, 0x6d, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x36, 0x0a, 0x06, 0x68,
    0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x72, 0x61,
    0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x61, 0x66, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x12, 0x32, 0x0a, 0x09, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73,
    0x18, 0x02, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d,
    0x64, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x09, 0x72, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x12, 0x40, 0x0a, 0x0e, 0x61, 0x64, 0x6d, 0x69, 0x6e,
    0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x19, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e, 0x41, 0x64, 0x6d,
    0x69, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0d, 0x61, 0x64, 0x6d, 0x69,
    0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x43, 0x0a, 0x0f, 0x73, 0x74, 0x61,
    0x74, 0x75, 0x73, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x63, 0x6d, 0x64, 0x70, 0x62, 0x2e,
    0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0e,
    0x73, 0x74, 0x61, 0x74, 0x75, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2a, 0x58,
    0x0a, 0x07, 0x43, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x49, 0x6e, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03, 0x47, 0x65, 0x74, 0x10, 0x01, 0x12,
    0x07, 0x0a, 0x03, 0x50, 0x75, 0x74, 0x10, 0x03, 0x12, 0x0a, 0x0a, 0x06, 0x44, 0x65, 0x6c, 0x65,
    0x74, 0x65, 0x10, 0x04, 0x12, 0x08, 0x0a, 0x04, 0x53, 0x6e, 0x61, 0x70, 0x10, 0x05, 0x12, 0x0c,
    0x0a, 0x08, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x10, 0x06, 0x12, 0x0a, 0x0a, 0x06,
    0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x10, 0x07, 0x2a, 0x80, 0x01, 0x0a, 0x0c, 0x41, 0x64, 0x6d,
    0x69, 0x6e, 0x43, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x12, 0x10, 0x0a, 0x0c, 0x49, 0x6e, 0x76,
    0x61, 0x6c, 0x69, 0x64, 0x41, 0x64, 0x6d, 0x69, 0x6e, 0x10, 0x00, 0x12, 0x0e, 0x0a, 0x0a, 0x43,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x10, 0x01, 0x12, 0x09, 0x0a, 0x05, 0x53,
    0x70, 0x6c, 0x69, 0x74, 0x10, 0x02, 0x12, 0x0e, 0x0a, 0x0a, 0x43, 0x6f, 0x6d, 0x70, 0x61, 0x63,
    0x74, 0x4c, 0x6f, 0x67, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x54, 0x72, 0x61, 0x6e, 0x73, 0x66,
    0x65, 0x72, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x10, 0x04, 0x12, 0x0f, 0x0a, 0x0b, 0x43, 0x6f,
    0x6d, 0x70, 0x75, 0x74, 0x65, 0x48, 0x61, 0x73, 0x68, 0x10, 0x05, 0x12, 0x0e, 0x0a, 0x0a, 0x56,
    0x65, 0x72, 0x69, 0x66, 0x79, 0x48, 0x61, 0x73, 0x68, 0x10, 0x06, 0x2a, 0x46, 0x0a, 0x0d, 0x53,
    0x74, 0x61, 0x74, 0x75, 0x73, 0x43, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x12, 0x11, 0x0a, 0x0d,
    0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x53, 0x74, 0x61, 0x74, 0x75, 0x73, 0x10, 0x00, 0x12,
    0x10, 0x0a, 0x0c, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x10,
    0x01, 0x12, 0x10, 0x0a, 0x0c, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x44, 0x65, 0x74, 0x61, 0x69,
    0x6c, 0x10, 0x02, 0x4a, 0xb5, 0x44, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xfc, 0x01, 0x01, 0x0a,
    0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03,
    0x01, 0x08, 0x12, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x07, 0x15, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x07, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x02, 0x12,
    0x03, 0x05, 0x07, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x08, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x08, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x08, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x1a,
    0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x04, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x09, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0c, 0x00,
    0x0e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x13, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x0d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0d, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0d, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x10, 0x00, 0x14, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x10, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x11, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x11, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x11, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x11,
    0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x11, 0x1c, 0x1d,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x12, 0x04, 0x1e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x12, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x12, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x12, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x12, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x13,
    0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x13, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x13, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x13, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x13, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03,
    0x12, 0x04, 0x16, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x16,
    0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1a, 0x00, 0x1d, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04,
    0x02, 0x00, 0x12, 0x03, 0x1b, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x1b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x14,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x1a, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x1c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x1c, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x1c, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x1f, 0x00, 0x21, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x1f, 0x08, 0x16, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x06, 0x12, 0x04, 0x23, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12,
    0x03, 0x23, 0x08, 0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x27, 0x00, 0x29, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x27, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x28, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x28, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x28, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x28, 0x1b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x24,
    0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x2b, 0x00, 0x2f, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02,
    0x00, 0x12, 0x03, 0x2c, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x2c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x2c,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x13, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1c, 0x1d, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x2d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x2d, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x2d, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03, 0x2e, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x13, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x2e, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04,
    0x31, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x31, 0x08, 0x18,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x35, 0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0a, 0x01, 0x12, 0x03, 0x35, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00,
    0x12, 0x03, 0x36, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x36, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x36, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x36, 0x13, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x37, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x37, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x37, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x37,
    0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x3a, 0x00, 0x3c, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00,
    0x12, 0x04, 0x3e, 0x00, 0x46, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x3e,
    0x05, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x3f, 0x04, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3f, 0x04, 0x0b, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x3f, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x40, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x40, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x40, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x41, 0x04,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x41, 0x04, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x41, 0x12, 0x13, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x42, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x42, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03,
    0x02, 0x12, 0x03, 0x42, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03,
    0x43, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x43, 0x04,
    0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x43, 0x12, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x44, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x44, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x05, 0x02, 0x12, 0x03, 0x44, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06,
    0x12, 0x03, 0x45, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03,
    0x45, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x45, 0x12,
    0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x48, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x48, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x00, 0x12, 0x03, 0x49, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x03, 0x49,
    0x0d, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x49, 0x20, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x49, 0x2e, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x4a, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x4a, 0x20, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x4a, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03, 0x4b, 0x04, 0x30,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4b, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x06, 0x12, 0x03, 0x4b, 0x0d, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4b, 0x20, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x4b, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x03,
    0x12, 0x03, 0x4c, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x4c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x06, 0x12, 0x03, 0x4c, 0x0d,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x01, 0x12, 0x03, 0x4c, 0x20, 0x26, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x03, 0x03, 0x12, 0x03, 0x4c, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x04, 0x12, 0x03, 0x4d, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x4d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04,
    0x06, 0x12, 0x03, 0x4d, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x4d, 0x20, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x04, 0x03, 0x12, 0x03, 0x4d,
    0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x05, 0x12, 0x03, 0x4e, 0x04, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x05, 0x04, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x05, 0x06, 0x12, 0x03, 0x4e, 0x0d, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x05, 0x01, 0x12, 0x03, 0x4e, 0x20, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x4e, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x06, 0x12,
    0x03, 0x4f, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x04, 0x12, 0x03, 0x4f,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x06, 0x12, 0x03, 0x4f, 0x0d, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x01, 0x12, 0x03, 0x4f, 0x20, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0c, 0x02, 0x06, 0x03, 0x12, 0x03, 0x4f, 0x2e, 0x2f, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0d, 0x12, 0x04, 0x52, 0x00, 0x5a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12,
    0x03, 0x52, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x53, 0x04,
    0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x03, 0x53, 0x0d, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x53, 0x20, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x53, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02,
    0x01, 0x12, 0x03, 0x54, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x54, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x06, 0x12, 0x03, 0x54,
    0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x54, 0x20, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x54, 0x2e, 0x2f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0d, 0x02, 0x02, 0x12, 0x03, 0x55, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x02, 0x04, 0x12, 0x03, 0x55, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x02, 0x06, 0x12, 0x03, 0x55, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x55, 0x20, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x55, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x03, 0x12, 0x03, 0x56, 0x04, 0x30,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0d, 0x02, 0x03, 0x06, 0x12, 0x03, 0x56, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x03, 0x01, 0x12, 0x03, 0x56, 0x20, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x03, 0x03, 0x12, 0x03, 0x56, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x04,
    0x12, 0x03, 0x57, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x04, 0x12, 0x03,
    0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x06, 0x12, 0x03, 0x57, 0x0d,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x01, 0x12, 0x03, 0x57, 0x20, 0x24, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x04, 0x03, 0x12, 0x03, 0x57, 0x2e, 0x2f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0d, 0x02, 0x05, 0x12, 0x03, 0x58, 0x04, 0x30, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x05, 0x04, 0x12, 0x03, 0x58, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05,
    0x06, 0x12, 0x03, 0x58, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x58, 0x20, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x05, 0x03, 0x12, 0x03, 0x58,
    0x2e, 0x2f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x06, 0x12, 0x03, 0x59, 0x04, 0x30, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x06, 0x04, 0x12, 0x03, 0x59, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x06, 0x06, 0x12, 0x03, 0x59, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x06, 0x01, 0x12, 0x03, 0x59, 0x20, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x59, 0x2e, 0x2f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x04, 0x5c,
    0x00, 0x60, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x5c, 0x08, 0x19, 0x0a,
    0x41, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x5e, 0x04, 0x34, 0x1a, 0x34, 0x20, 0x54,
    0x68, 0x69, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20,
    0x63, 0x61, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x69, 0x6e, 0x74, 0x65, 0x72, 0x6e,
    0x61, 0x6c, 0x20, 0x52, 0x61, 0x66, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x6e, 0x6f, 0x77,
    0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5e, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5e, 0x0d, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5e, 0x24, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5e, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e,
    0x02, 0x01, 0x12, 0x03, 0x5f, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x5f, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5f, 0x19,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5f, 0x32, 0x33, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x62, 0x00, 0x64, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x0f, 0x01, 0x12, 0x03, 0x62, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12,
    0x03, 0x63, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x63,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x06, 0x12, 0x03, 0x63, 0x0d, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01, 0x12, 0x03, 0x63, 0x1b, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03, 0x63, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x10, 0x12, 0x04, 0x66, 0x00, 0x73, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12,
    0x03, 0x66, 0x08, 0x14, 0x0a, 0x76, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x03, 0x69, 0x04,
    0x32, 0x1a, 0x69, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20,
    0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x63, 0x61, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x69,
    0x6e, 0x74, 0x65, 0x72, 0x6e, 0x61, 0x6c, 0x20, 0x52, 0x61, 0x66, 0x74, 0x53, 0x74, 0x6f, 0x72,
    0x65, 0x20, 0x6e, 0x6f, 0x77, 0x2e, 0x0a, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x70, 0x6c, 0x69,
    0x74, 0x5f, 0x6b, 0x65, 0x79, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65, 0x20, 0x69, 0x6e,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x62, 0x65, 0x65, 0x6e, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x74,
    0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x69, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x69, 0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x69, 0x30, 0x31, 0x0a, 0xba, 0x01, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x03, 0x6d,
    0x04, 0x32, 0x1a, 0xac, 0x01, 0x20, 0x57, 0x65, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x74,
    0x77, 0x6f, 0x2c, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x75, 0x73, 0x65, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x20, 0x0a, 0x20, 0x70, 0x61, 0x72, 0x65,
    0x6e, 0x74, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x64, 0x2c, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x20, 0x75, 0x73, 0x65,
    0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x5f, 0x69, 0x64, 0x2e, 0x0a, 0x20, 0x57, 0x65, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x67, 0x75,
    0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x6e, 0x65, 0x77, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x20, 0x69,
    0x73, 0x20, 0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75, 0x65, 0x2e,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6d, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6d, 0x14, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6d, 0x30, 0x31, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x10, 0x02,
    0x02, 0x12, 0x03, 0x6f, 0x04, 0x32, 0x1a, 0x28, 0x20, 0x54, 0x68, 0x65, 0x20, 0x70, 0x65, 0x65,
    0x72, 0x20, 0x69, 0x64, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65,
    0x77, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x04, 0x12, 0x03, 0x6f, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x02, 0x05, 0x12, 0x03, 0x6f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x02, 0x01, 0x12, 0x03, 0x6f, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x6f, 0x30, 0x31, 0x0a, 0x62, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x03,
    0x12, 0x03, 0x72, 0x04, 0x32, 0x1a, 0x55, 0x20, 0x49, 0x66, 0x20, 0x74, 0x72, 0x75, 0x65, 0x2c,
    0x20, 0x72, 0x69, 0x67, 0x68, 0x74, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x64, 0x65,
    0x72, 0x69, 0x76, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x20,
    0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x2c, 0x20, 0x0a, 0x20, 0x6c, 0x65, 0x66,
    0x74, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x75, 0x73, 0x65, 0x20, 0x6e, 0x65, 0x77,
    0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x03, 0x04, 0x12, 0x03, 0x72, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x03, 0x05, 0x12, 0x03, 0x72, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x72, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x72, 0x30, 0x31, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x04, 0x75, 0x00, 0x78, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x03, 0x75, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x11, 0x02, 0x00, 0x12, 0x03, 0x76, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x76, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x76, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x76, 0x1b, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x03, 0x76, 0x23,
    0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x03, 0x77, 0x04, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x03, 0x77, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x06, 0x12, 0x03, 0x77, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x77, 0x1b, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x77, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x04, 0x7a, 0x00,
    0x7d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x03, 0x7a, 0x08, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x03, 0x7b, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x7b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x7b, 0x14, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x7b, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x03, 0x7c, 0x04, 0x28,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7c, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7c, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x7c, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x05,
    0x7f, 0x00, 0x81, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x03, 0x7f, 0x08,
    0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06, 0x83, 0x01, 0x00, 0x85, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0x83, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0x84, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x84, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x00, 0x06, 0x12, 0x04, 0x84, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x84, 0x01, 0x19, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x84, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x06, 0x87, 0x01,
    0x00, 0x88, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08,
    0x1e, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0x8a, 0x01, 0x00, 0x8d, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x05, 0x12, 0x04, 0x8b, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x8b, 0x01, 0x14, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x8b, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04,
    0x8c, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8c,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x05, 0x12, 0x04, 0x8c, 0x01,
    0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x13,
    0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x1e, 0x1f,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0x8f, 0x01, 0x00, 0x90, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x02, 0x05,
    0x01, 0x12, 0x06, 0x92, 0x01, 0x00, 0x9a, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x05, 0x01, 0x01,
    0x12, 0x04, 0x92, 0x01, 0x05, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x04,
    0x93, 0x01, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x04, 0x93,
    0x01, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x04, 0x93, 0x01,
    0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x04, 0x94, 0x01, 0x04, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94, 0x01, 0x04, 0x0e, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x04, 0x94, 0x01, 0x16, 0x17, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x04, 0x95, 0x01, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x04, 0x95, 0x01, 0x04, 0x09, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x02, 0x02, 0x12, 0x04, 0x95, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x03, 0x12, 0x04, 0x96, 0x01, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03,
    0x01, 0x12, 0x04, 0x96, 0x01, 0x04, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02,
    0x12, 0x04, 0x96, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x04, 0x12, 0x04,
    0x97, 0x01, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x01, 0x12, 0x04, 0x97,
    0x01, 0x04, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x04, 0x02, 0x12, 0x04, 0x97, 0x01,
    0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x05, 0x12, 0x04, 0x98, 0x01, 0x04, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x01, 0x12, 0x04, 0x98, 0x01, 0x04, 0x0f, 0x0a,
    0x0d, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x05, 0x02, 0x12, 0x04, 0x98, 0x01, 0x16, 0x17, 0x0a, 0x0c,
    0x0a, 0x04, 0x05, 0x01, 0x02, 0x06, 0x12, 0x04, 0x99, 0x01, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x06, 0x01, 0x12, 0x04, 0x99, 0x01, 0x04, 0x0e, 0x0a, 0x0d, 0x0a, 0x05, 0x05,
    0x01, 0x02, 0x06, 0x02, 0x12, 0x04, 0x99, 0x01, 0x16, 0x17, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18,
    0x12, 0x06, 0x9c, 0x01, 0x00, 0xa3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12,
    0x04, 0x9c, 0x01, 0x08, 0x14, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0x9d,
    0x01, 0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0x9d, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x06, 0x12, 0x04, 0x9d, 0x01, 0x0d,
    0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x1a, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x36, 0x37, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x38, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9e, 0x01, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x1f, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x01, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02,
    0x02, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x04,
    0x12, 0x04, 0x9f, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x06, 0x12,
    0x04, 0x9f, 0x01, 0x0d, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x01, 0x12, 0x04,
    0x9f, 0x01, 0x1a, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9f,
    0x01, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x04,
    0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x04, 0x12, 0x04, 0xa0, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x06, 0x12, 0x04, 0xa0, 0x01, 0x0d, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x1f, 0x2a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x36, 0x37, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x18, 0x02, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x04, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x04, 0x06, 0x12, 0x04, 0xa1, 0x01, 0x0d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x04, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x23, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04,
    0x03, 0x12, 0x04, 0xa1, 0x01, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x05, 0x12,
    0x04, 0xa2, 0x01, 0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x04, 0x12, 0x04,
    0xa2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x06, 0x12, 0x04, 0xa2,
    0x01, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x01, 0x12, 0x04, 0xa2, 0x01,
    0x1f, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x36,
    0x37, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xa5, 0x01, 0x00, 0xac, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xa6, 0x01, 0x0d, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xa6, 0x01, 0x1a, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xa6, 0x01, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04,
    0xa7, 0x01, 0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa7,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa7, 0x01,
    0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x20,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x36, 0x37,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0xa8, 0x01, 0x04, 0x38, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x02, 0x06, 0x12, 0x04, 0xa8, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x1b, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0xa8, 0x01, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19,
    0x02, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03,
    0x04, 0x12, 0x04, 0xa9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x06,
    0x12, 0x04, 0xa9, 0x01, 0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x01, 0x12,
    0x04, 0xa9, 0x01, 0x20, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x03, 0x12, 0x04,
    0xa9, 0x01, 0x36, 0x37, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x04, 0x12, 0x04, 0xaa, 0x01,
    0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x06, 0x12, 0x04, 0xaa, 0x01, 0x0d, 0x23,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x24, 0x33, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x36, 0x37, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x19, 0x02, 0x05, 0x12, 0x04, 0xab, 0x01, 0x04, 0x38, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x05, 0x04, 0x12, 0x04, 0xab, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x05, 0x06, 0x12, 0x04, 0xab, 0x01, 0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x05, 0x01, 0x12, 0x04, 0xab, 0x01, 0x20, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x05, 0x03, 0x12, 0x04, 0xab, 0x01, 0x36, 0x37, 0x0a, 0x31, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06,
    0xaf, 0x01, 0x00, 0xb1, 0x01, 0x01, 0x1a, 0x23, 0x20, 0x46, 0x6f, 0x72, 0x20, 0x67, 0x65, 0x74,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x1a, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06,
    0xb3, 0x01, 0x00, 0xb5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xb3,
    0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xb4, 0x01, 0x04,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb4, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb4, 0x01, 0x0d, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb4, 0x01, 0x19, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb4, 0x01, 0x22, 0x23, 0x0a, 0xe4, 0x04,
    0x0a, 0x02, 0x04, 0x1c, 0x12, 0x06, 0xc0, 0x01, 0x00, 0xc2, 0x01, 0x01, 0x1a, 0xd5, 0x04, 0x20,
    0x46, 0x6f, 0x72, 0x20, 0x67, 0x65, 0x74, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x6d, 0x6f, 0x72, 0x65,
    0x20, 0x69, 0x6e, 0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x57, 0x65, 0x20,
    0x61, 0x64, 0x64, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x20, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x20, 0x6f,
    0x70, 0x65, 0x72, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x20, 0x28, 0x43, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x50, 0x65, 0x65, 0x72, 0x2c, 0x20, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x2e, 0x2e, 0x2e, 0x29,
    0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x62, 0x20, 0x6a, 0x6f, 0x62,
    0x20, 0x6c, 0x69, 0x73, 0x74, 0x2c, 0x0a, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x70, 0x64, 0x20,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x70, 0x65, 0x65, 0x6b,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x6f, 0x6e, 0x65, 0x2c, 0x20,
    0x68, 0x61, 0x6e, 0x64, 0x6c, 0x65, 0x20, 0x69, 0x74, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68,
    0x65, 0x6e, 0x20, 0x70, 0x6f, 0x70, 0x20, 0x69, 0x74, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6a, 0x6f, 0x62, 0x20, 0x6c, 0x69, 0x62, 0x2e, 0x20, 0x0a, 0x20, 0x42, 0x75,
    0x74, 0x20, 0x73, 0x6f, 0x6d, 0x65, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x2c, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x70, 0x64, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x63,
    0x72, 0x61, 0x73, 0x68, 0x20, 0x62, 0x65, 0x66, 0x6f, 0x72, 0x65, 0x20, 0x70, 0x6f, 0x70, 0x70,
    0x69, 0x6e, 0x67, 0x2e, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x61, 0x6e, 0x6f, 0x74, 0x68, 0x65,
    0x72, 0x20, 0x70, 0x64, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x0a, 0x20, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x73, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x66, 0x69, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x6a, 0x6f, 0x62, 0x20, 0x69, 0x73, 0x20, 0x72, 0x75, 0x6e, 0x6e, 0x69, 0x6e, 0x67,
    0x20, 0x62, 0x75, 0x74, 0x20, 0x6e, 0x6f, 0x74, 0x20, 0x66, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x65,
    0x64, 0x2c, 0x20, 0x69, 0x74, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74,
    0x20, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x0a, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x61, 0x66, 0x74, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20,
    0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x68, 0x61, 0x73, 0x20, 0x68, 0x61, 0x6e, 0x64,
    0x6c, 0x65, 0x64, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6a, 0x6f, 0x62, 0x2e, 0x0a, 0x20, 0x45,
    0x2c, 0x67, 0x2c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65,
    0x65, 0x72, 0x2c, 0x20, 0x69, 0x66, 0x20, 0x77, 0x65, 0x20, 0x61, 0x64, 0x64, 0x20, 0x50, 0x65,
    0x65, 0x72, 0x31, 0x30, 0x20, 0x69, 0x6e, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x31, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x66, 0x69, 0x6e, 0x64, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x31, 0x20, 0x68, 0x61, 0x73, 0x20, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79, 0x20, 0x68,
    0x61, 0x64, 0x0a, 0x20, 0x50, 0x65, 0x65, 0x72, 0x31, 0x30, 0x2c, 0x20, 0x77, 0x65, 0x20, 0x63,
    0x61, 0x6e, 0x20, 0x74, 0x68, 0x69, 0x6e, 0x6b, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x43, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6e, 0x69,
    0x73, 0x68, 0x65, 0x64, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x70, 0x6f,
    0x70, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x6a, 0x6f, 0x62, 0x20, 0x66, 0x72, 0x6f, 0x6d, 0x20,
    0x6a, 0x6f, 0x62, 0x20, 0x6c, 0x69, 0x73, 0x74, 0x0a, 0x20, 0x64, 0x69, 0x72, 0x65, 0x63, 0x74,
    0x6c, 0x79, 0x2e, 0x0a, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x08,
    0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xc4, 0x01, 0x00, 0xc7, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x08, 0x1c, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xc5, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xc5, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x00, 0x06, 0x12, 0x04, 0xc5, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xc5, 0x01, 0x1b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xc5, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x01, 0x12, 0x04,
    0xc6, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc6,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x06, 0x12, 0x04, 0xc6, 0x01,
    0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x19,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x25, 0x26,
    0x0a, 0x0c, 0x0a, 0x02, 0x05, 0x02, 0x12, 0x06, 0xca, 0x01, 0x00, 0xce, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x05, 0x02, 0x01, 0x12, 0x04, 0xca, 0x01, 0x05, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x05,
    0x02, 0x02, 0x00, 0x12, 0x04, 0xcb, 0x01, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x02, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xcb, 0x01, 0x04, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x00,
    0x02, 0x12, 0x04, 0xcb, 0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12,
    0x04, 0xcc, 0x01, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xcc, 0x01, 0x04, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x01, 0x02, 0x12, 0x04, 0xcc,
    0x01, 0x17, 0x18, 0x0a, 0x0c, 0x0a, 0x04, 0x05, 0x02, 0x02, 0x02, 0x12, 0x04, 0xcd, 0x01, 0x04,
    0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x01, 0x12, 0x04, 0xcd, 0x01, 0x04, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x05, 0x02, 0x02, 0x02, 0x02, 0x12, 0x04, 0xcd, 0x01, 0x17, 0x18, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x1e, 0x12, 0x06, 0xd0, 0x01, 0x00, 0xd4, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x1e, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e,
    0x02, 0x00, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00,
    0x04, 0x12, 0x04, 0xd1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x06,
    0x12, 0x04, 0xd1, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd1, 0x01, 0x1b, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04,
    0xd1, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x01, 0x12, 0x04, 0xd2, 0x01,
    0x04, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd2, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x06, 0x12, 0x04, 0xd2, 0x01, 0x0d, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x21, 0x2e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x34, 0x35, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1e, 0x02, 0x02, 0x12, 0x04, 0xd3, 0x01, 0x04, 0x36, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1e, 0x02, 0x02, 0x04, 0x12, 0x04, 0xd3, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1e, 0x02, 0x02, 0x06, 0x12, 0x04, 0xd3, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xd3, 0x01, 0x21, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xd3, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06,
    0xd6, 0x01, 0x00, 0xda, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0xd6,
    0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0xd7, 0x01, 0x04,
    0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xd7, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x06, 0x12, 0x04, 0xd7, 0x01, 0x0d, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x1b, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd7, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1f, 0x02, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x04, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xd8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xd8, 0x01, 0x0d, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xd8, 0x01, 0x22, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xd8, 0x01, 0x34, 0x35, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x02, 0x12,
    0x04, 0xd9, 0x01, 0x04, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x04, 0x12, 0x04,
    0xd9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x06, 0x12, 0x04, 0xd9,
    0x01, 0x0d, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x01, 0x12, 0x04, 0xd9, 0x01,
    0x22, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02, 0x03, 0x12, 0x04, 0xd9, 0x01, 0x34,
    0x35, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x20, 0x12, 0x06, 0xdc, 0x01, 0x00, 0xe6, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x20, 0x01, 0x12, 0x04, 0xdc, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x20, 0x02, 0x00, 0x12, 0x04, 0xdd, 0x01, 0x04, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x00, 0x04, 0x12, 0x04, 0xdd, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02,
    0x00, 0x05, 0x12, 0x04, 0xdd, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00,
    0x01, 0x12, 0x04, 0xdd, 0x01, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03,
    0x12, 0x04, 0xdd, 0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x01, 0x12, 0x04,
    0xde, 0x01, 0x04, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x04, 0x12, 0x04, 0xde,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x06, 0x12, 0x04, 0xde, 0x01,
    0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x01, 0x12, 0x04, 0xde, 0x01, 0x19,
    0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x01, 0x03, 0x12, 0x04, 0xde, 0x01, 0x32, 0x33,
    0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x02, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x34, 0x1a, 0x1d,
    0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x61, 0x64, 0x20, 0x6c,
    0x69, 0x6e, 0x65, 0x61, 0x72, 0x69, 0x7a, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x02, 0x04, 0x12, 0x04, 0xe0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x02, 0x05, 0x12, 0x04, 0xe0, 0x01, 0x0d, 0x11, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x02, 0x01, 0x12, 0x04, 0xe0, 0x01, 0x12, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20,
    0x02, 0x02, 0x03, 0x12, 0x04, 0xe0, 0x01, 0x32, 0x33, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x20, 0x02,
    0x03, 0x12, 0x04, 0xe2, 0x01, 0x04, 0x34, 0x1a, 0x25, 0x20, 0x31, 0x36, 0x20, 0x62, 0x79, 0x74,
    0x65, 0x73, 0x2c, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x75, 0x69,
    0x73, 0x68, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x20, 0x20, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x20, 0x02, 0x03, 0x04, 0x12, 0x04, 0xe2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x20, 0x02, 0x03, 0x05, 0x12, 0x04, 0xe2, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x20, 0x02, 0x03, 0x01, 0x12, 0x04, 0xe2, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x20, 0x02, 0x03, 0x03, 0x12, 0x04, 0xe2, 0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20,
    0x02, 0x04, 0x12, 0x04, 0xe4, 0x01, 0x04, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04,
    0x04, 0x12, 0x04, 0xe4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x06,
    0x12, 0x04, 0xe4, 0x01, 0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x01, 0x12,
    0x04, 0xe4, 0x01, 0x20, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x04, 0x03, 0x12, 0x04,
    0xe4, 0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x05, 0x12, 0x04, 0xe5, 0x01,
    0x04, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x04, 0x12, 0x04, 0xe5, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x05, 0x12, 0x04, 0xe5, 0x01, 0x0d, 0x13,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x01, 0x12, 0x04, 0xe5, 0x01, 0x14, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x05, 0x03, 0x12, 0x04, 0xe5, 0x01, 0x32, 0x33, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0xe8, 0x01, 0x00, 0xec, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x21, 0x01, 0x12, 0x04, 0xe8, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02,
    0x00, 0x12, 0x04, 0xe9, 0x01, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xe9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xe9, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xe9, 0x01, 0x1b, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe9,
    0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x21, 0x02, 0x01, 0x12, 0x04, 0xea, 0x01, 0x04,
    0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x04, 0x12, 0x04, 0xea, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x05, 0x12, 0x04, 0xea, 0x01, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x01, 0x12, 0x04, 0xea, 0x01, 0x13, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x21, 0x02, 0x01, 0x03, 0x12, 0x04, 0xea, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x21, 0x02, 0x02, 0x12, 0x04, 0xeb, 0x01, 0x04, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x21, 0x02, 0x02, 0x04, 0x12, 0x04, 0xeb, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21,
    0x02, 0x02, 0x05, 0x12, 0x04, 0xeb, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x21, 0x02, 0x02,
    0x03, 0x12, 0x04, 0xeb, 0x01, 0x24, 0x25, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x22, 0x12, 0x06, 0xee,
    0x01, 0x00, 0xf5, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x22, 0x01, 0x12, 0x04, 0xee, 0x01,
    0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x00, 0x12, 0x04, 0xef, 0x01, 0x04, 0x2e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x04, 0x12, 0x04, 0xef, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x06, 0x12, 0x04, 0xef, 0x01, 0x0d, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x01, 0x12, 0x04, 0xef, 0x01, 0x1f, 0x25, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x22, 0x02, 0x00, 0x03, 0x12, 0x04, 0xef, 0x01, 0x2c, 0x2d, 0x0a, 0x5a, 0x0a, 0x04,
    0x04, 0x22, 0x02, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x04, 0x2e, 0x1a, 0x4c, 0x20, 0x57, 0x65, 0x20,
    0x63, 0x61, 0x6e, 0x27, 0x74, 0x20, 0x65, 0x6e, 0x63, 0x6c, 0x6f, 0x73, 0x65, 0x20, 0x6e, 0x6f,
    0x72, 0x6d, 0x61, 0x6c, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x73, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x61, 0x64, 0x6d, 0x69, 0x6e, 0x69, 0x73, 0x74, 0x72, 0x61, 0x74, 0x6f, 0x72, 0x20,
    0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x20, 0x61, 0x74, 0x20, 0x73, 0x61, 0x6d, 0x65,
    0x20, 0x74, 0x69, 0x6d, 0x65, 0x2e, 0x20, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01,
    0x04, 0x12, 0x04, 0xf2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x06,
    0x12, 0x04, 0xf2, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x01, 0x12,
    0x04, 0xf2, 0x01, 0x15, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x03, 0x12, 0x04,
    0xf2, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x02, 0x12, 0x04, 0xf3, 0x01,
    0x04, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf3, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x02, 0x06, 0x12, 0x04, 0xf3, 0x01, 0x0d, 0x19,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf3, 0x01, 0x1a, 0x27, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf3, 0x01, 0x2c, 0x2d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x22, 0x02, 0x03, 0x12, 0x04, 0xf4, 0x01, 0x04, 0x2e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x22, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x22, 0x02, 0x03, 0x06, 0x12, 0x04, 0xf4, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xf4, 0x01, 0x1b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xf4, 0x01, 0x2c, 0x2d, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x23, 0x12, 0x06,
    0xf7, 0x01, 0x00, 0xfc, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x23, 0x01, 0x12, 0x04, 0xf7,
    0x01, 0x08, 0x17, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x00, 0x12, 0x04, 0xf8, 0x01, 0x04,
    0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x06, 0x12, 0x04, 0xf8, 0x01, 0x0d, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf8, 0x01, 0x20, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x23, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf8, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x23, 0x02, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x04, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x23, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23,
    0x02, 0x01, 0x06, 0x12, 0x04, 0xf9, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02,
    0x01, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x16, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x01,
    0x03, 0x12, 0x04, 0xf9, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x02, 0x12,
    0x04, 0xfa, 0x01, 0x04, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x02, 0x04, 0x12, 0x04,
    0xfa, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x02, 0x06, 0x12, 0x04, 0xfa,
    0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x02, 0x01, 0x12, 0x04, 0xfa, 0x01,
    0x1b, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x02, 0x03, 0x12, 0x04, 0xfa, 0x01, 0x2e,
    0x2f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x23, 0x02, 0x03, 0x12, 0x04, 0xfb, 0x01, 0x04, 0x30, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x04, 0x12, 0x04, 0xfb, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x23, 0x02, 0x03, 0x06, 0x12, 0x04, 0xfb, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x23, 0x02, 0x03, 0x01, 0x12, 0x04, 0xfb, 0x01, 0x1c, 0x2b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x23, 0x02, 0x03, 0x03, 0x12, 0x04, 0xfb, 0x01, 0x2e, 0x2f,
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
