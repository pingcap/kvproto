// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Leader {
    // message fields
    addr: ::protobuf::SingularField<::std::string::String>,
    id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Leader {}

impl Leader {
    pub fn new() -> Leader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Leader {
        static mut instance: ::protobuf::lazy::Lazy<Leader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Leader,
        };
        unsafe {
            instance.get(|| {
                Leader {
                    addr: ::protobuf::SingularField::none(),
                    id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string addr = 1;

    pub fn clear_addr(&mut self) {
        self.addr.clear();
    }

    pub fn has_addr(&self) -> bool {
        self.addr.is_some()
    }

    // Param is passed by value, moved
    pub fn set_addr(&mut self, v: ::std::string::String) {
        self.addr = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_addr(&mut self) -> &mut ::std::string::String {
        if self.addr.is_none() {
            self.addr.set_default();
        };
        self.addr.as_mut().unwrap()
    }

    // Take field
    pub fn take_addr(&mut self) -> ::std::string::String {
        self.addr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_addr(&self) -> &str {
        match self.addr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint64 id = 3;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }
}

impl ::protobuf::Message for Leader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.addr));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.addr {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.addr.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.id {
            try!(os.write_uint64(3, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Leader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Leader {
    fn new() -> Leader {
        Leader::new()
    }

    fn descriptor_static(_: ::std::option::Option<Leader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "addr",
                    Leader::has_addr,
                    Leader::get_addr,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "id",
                    Leader::has_id,
                    Leader::get_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Leader>(
                    "Leader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Leader {
    fn clear(&mut self) {
        self.clear_addr();
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Leader {
    fn eq(&self, other: &Leader) -> bool {
        self.addr == other.addr &&
        self.id == other.id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Leader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsoRequest {
    // message fields
    count: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsoRequest {}

impl TsoRequest {
    pub fn new() -> TsoRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsoRequest {
        static mut instance: ::protobuf::lazy::Lazy<TsoRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsoRequest,
        };
        unsafe {
            instance.get(|| {
                TsoRequest {
                    count: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 count = 1;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> u32 {
        self.count.unwrap_or(0)
    }
}

impl ::protobuf::Message for TsoRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.count = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.count {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.count {
            try!(os.write_uint32(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsoRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsoRequest {
    fn new() -> TsoRequest {
        TsoRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsoRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "count",
                    TsoRequest::has_count,
                    TsoRequest::get_count,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsoRequest>(
                    "TsoRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsoRequest {
    fn clear(&mut self) {
        self.clear_count();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsoRequest {
    fn eq(&self, other: &TsoRequest) -> bool {
        self.count == other.count &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsoRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Timestamp {
    // message fields
    physical: ::std::option::Option<i64>,
    logical: ::std::option::Option<i64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Timestamp {}

impl Timestamp {
    pub fn new() -> Timestamp {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Timestamp {
        static mut instance: ::protobuf::lazy::Lazy<Timestamp> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Timestamp,
        };
        unsafe {
            instance.get(|| {
                Timestamp {
                    physical: ::std::option::Option::None,
                    logical: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional int64 physical = 1;

    pub fn clear_physical(&mut self) {
        self.physical = ::std::option::Option::None;
    }

    pub fn has_physical(&self) -> bool {
        self.physical.is_some()
    }

    // Param is passed by value, moved
    pub fn set_physical(&mut self, v: i64) {
        self.physical = ::std::option::Option::Some(v);
    }

    pub fn get_physical(&self) -> i64 {
        self.physical.unwrap_or(0)
    }

    // optional int64 logical = 2;

    pub fn clear_logical(&mut self) {
        self.logical = ::std::option::Option::None;
    }

    pub fn has_logical(&self) -> bool {
        self.logical.is_some()
    }

    // Param is passed by value, moved
    pub fn set_logical(&mut self, v: i64) {
        self.logical = ::std::option::Option::Some(v);
    }

    pub fn get_logical(&self) -> i64 {
        self.logical.unwrap_or(0)
    }
}

impl ::protobuf::Message for Timestamp {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.physical = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.logical = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.physical {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.logical {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.physical {
            try!(os.write_int64(1, v));
        };
        if let Some(v) = self.logical {
            try!(os.write_int64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Timestamp>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Timestamp {
    fn new() -> Timestamp {
        Timestamp::new()
    }

    fn descriptor_static(_: ::std::option::Option<Timestamp>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "physical",
                    Timestamp::has_physical,
                    Timestamp::get_physical,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "logical",
                    Timestamp::has_logical,
                    Timestamp::get_logical,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Timestamp>(
                    "Timestamp",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Timestamp {
    fn clear(&mut self) {
        self.clear_physical();
        self.clear_logical();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Timestamp {
    fn eq(&self, other: &Timestamp) -> bool {
        self.physical == other.physical &&
        self.logical == other.logical &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Timestamp {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TsoResponse {
    // message fields
    count: ::std::option::Option<u32>,
    timestamp: ::protobuf::SingularPtrField<Timestamp>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TsoResponse {}

impl TsoResponse {
    pub fn new() -> TsoResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TsoResponse {
        static mut instance: ::protobuf::lazy::Lazy<TsoResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TsoResponse,
        };
        unsafe {
            instance.get(|| {
                TsoResponse {
                    count: ::std::option::Option::None,
                    timestamp: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 count = 1;

    pub fn clear_count(&mut self) {
        self.count = ::std::option::Option::None;
    }

    pub fn has_count(&self) -> bool {
        self.count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_count(&mut self, v: u32) {
        self.count = ::std::option::Option::Some(v);
    }

    pub fn get_count(&self) -> u32 {
        self.count.unwrap_or(0)
    }

    // optional .pdpb.Timestamp timestamp = 2;

    pub fn clear_timestamp(&mut self) {
        self.timestamp.clear();
    }

    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_timestamp(&mut self, v: Timestamp) {
        self.timestamp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_timestamp(&mut self) -> &mut Timestamp {
        if self.timestamp.is_none() {
            self.timestamp.set_default();
        };
        self.timestamp.as_mut().unwrap()
    }

    // Take field
    pub fn take_timestamp(&mut self) -> Timestamp {
        self.timestamp.take().unwrap_or_else(|| Timestamp::new())
    }

    pub fn get_timestamp(&self) -> &Timestamp {
        self.timestamp.as_ref().unwrap_or_else(|| Timestamp::default_instance())
    }
}

impl ::protobuf::Message for TsoResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.count = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.timestamp));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.count {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.timestamp {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.count {
            try!(os.write_uint32(1, v));
        };
        if let Some(v) = self.timestamp.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TsoResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TsoResponse {
    fn new() -> TsoResponse {
        TsoResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<TsoResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "count",
                    TsoResponse::has_count,
                    TsoResponse::get_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "timestamp",
                    TsoResponse::has_timestamp,
                    TsoResponse::get_timestamp,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TsoResponse>(
                    "TsoResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TsoResponse {
    fn clear(&mut self) {
        self.clear_count();
        self.clear_timestamp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsoResponse {
    fn eq(&self, other: &TsoResponse) -> bool {
        self.count == other.count &&
        self.timestamp == other.timestamp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TsoResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BootstrapRequest {
    // message fields
    store: ::protobuf::SingularPtrField<super::metapb::Store>,
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BootstrapRequest {}

impl BootstrapRequest {
    pub fn new() -> BootstrapRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BootstrapRequest {
        static mut instance: ::protobuf::lazy::Lazy<BootstrapRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BootstrapRequest,
        };
        unsafe {
            instance.get(|| {
                BootstrapRequest {
                    store: ::protobuf::SingularPtrField::none(),
                    region: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .metapb.Store store = 1;

    pub fn clear_store(&mut self) {
        self.store.clear();
    }

    pub fn has_store(&self) -> bool {
        self.store.is_some()
    }

    // Param is passed by value, moved
    pub fn set_store(&mut self, v: super::metapb::Store) {
        self.store = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_store(&mut self) -> &mut super::metapb::Store {
        if self.store.is_none() {
            self.store.set_default();
        };
        self.store.as_mut().unwrap()
    }

    // Take field
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store.take().unwrap_or_else(|| super::metapb::Store::new())
    }

    pub fn get_store(&self) -> &super::metapb::Store {
        self.store.as_ref().unwrap_or_else(|| super::metapb::Store::default_instance())
    }

    // optional .metapb.Region region = 2;

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
}

impl ::protobuf::Message for BootstrapRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.store));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.store {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.region {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.store.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.region.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BootstrapRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BootstrapRequest {
    fn new() -> BootstrapRequest {
        BootstrapRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<BootstrapRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "store",
                    BootstrapRequest::has_store,
                    BootstrapRequest::get_store,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region",
                    BootstrapRequest::has_region,
                    BootstrapRequest::get_region,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<BootstrapRequest>(
                    "BootstrapRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BootstrapRequest {
    fn clear(&mut self) {
        self.clear_store();
        self.clear_region();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BootstrapRequest {
    fn eq(&self, other: &BootstrapRequest) -> bool {
        self.store == other.store &&
        self.region == other.region &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BootstrapRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BootstrapResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BootstrapResponse {}

impl BootstrapResponse {
    pub fn new() -> BootstrapResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BootstrapResponse {
        static mut instance: ::protobuf::lazy::Lazy<BootstrapResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BootstrapResponse,
        };
        unsafe {
            instance.get(|| {
                BootstrapResponse {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for BootstrapResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BootstrapResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BootstrapResponse {
    fn new() -> BootstrapResponse {
        BootstrapResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<BootstrapResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<BootstrapResponse>(
                    "BootstrapResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BootstrapResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BootstrapResponse {
    fn eq(&self, other: &BootstrapResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BootstrapResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IsBootstrappedRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IsBootstrappedRequest {}

impl IsBootstrappedRequest {
    pub fn new() -> IsBootstrappedRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IsBootstrappedRequest {
        static mut instance: ::protobuf::lazy::Lazy<IsBootstrappedRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IsBootstrappedRequest,
        };
        unsafe {
            instance.get(|| {
                IsBootstrappedRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for IsBootstrappedRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<IsBootstrappedRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IsBootstrappedRequest {
    fn new() -> IsBootstrappedRequest {
        IsBootstrappedRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<IsBootstrappedRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<IsBootstrappedRequest>(
                    "IsBootstrappedRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IsBootstrappedRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IsBootstrappedRequest {
    fn eq(&self, other: &IsBootstrappedRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IsBootstrappedRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct IsBootstrappedResponse {
    // message fields
    bootstrapped: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IsBootstrappedResponse {}

impl IsBootstrappedResponse {
    pub fn new() -> IsBootstrappedResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IsBootstrappedResponse {
        static mut instance: ::protobuf::lazy::Lazy<IsBootstrappedResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IsBootstrappedResponse,
        };
        unsafe {
            instance.get(|| {
                IsBootstrappedResponse {
                    bootstrapped: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bool bootstrapped = 1;

    pub fn clear_bootstrapped(&mut self) {
        self.bootstrapped = ::std::option::Option::None;
    }

    pub fn has_bootstrapped(&self) -> bool {
        self.bootstrapped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bootstrapped(&mut self, v: bool) {
        self.bootstrapped = ::std::option::Option::Some(v);
    }

    pub fn get_bootstrapped(&self) -> bool {
        self.bootstrapped.unwrap_or(false)
    }
}

impl ::protobuf::Message for IsBootstrappedResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.bootstrapped = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.bootstrapped.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.bootstrapped {
            try!(os.write_bool(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<IsBootstrappedResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for IsBootstrappedResponse {
    fn new() -> IsBootstrappedResponse {
        IsBootstrappedResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<IsBootstrappedResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "bootstrapped",
                    IsBootstrappedResponse::has_bootstrapped,
                    IsBootstrappedResponse::get_bootstrapped,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IsBootstrappedResponse>(
                    "IsBootstrappedResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IsBootstrappedResponse {
    fn clear(&mut self) {
        self.clear_bootstrapped();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for IsBootstrappedResponse {
    fn eq(&self, other: &IsBootstrappedResponse) -> bool {
        self.bootstrapped == other.bootstrapped &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for IsBootstrappedResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AllocIdRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AllocIdRequest {}

impl AllocIdRequest {
    pub fn new() -> AllocIdRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AllocIdRequest {
        static mut instance: ::protobuf::lazy::Lazy<AllocIdRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AllocIdRequest,
        };
        unsafe {
            instance.get(|| {
                AllocIdRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for AllocIdRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AllocIdRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AllocIdRequest {
    fn new() -> AllocIdRequest {
        AllocIdRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AllocIdRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<AllocIdRequest>(
                    "AllocIdRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AllocIdRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AllocIdRequest {
    fn eq(&self, other: &AllocIdRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AllocIdRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AllocIdResponse {
    // message fields
    id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AllocIdResponse {}

impl AllocIdResponse {
    pub fn new() -> AllocIdResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AllocIdResponse {
        static mut instance: ::protobuf::lazy::Lazy<AllocIdResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AllocIdResponse,
        };
        unsafe {
            instance.get(|| {
                AllocIdResponse {
                    id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 id = 1;

    pub fn clear_id(&mut self) {
        self.id = ::std::option::Option::None;
    }

    pub fn has_id(&self) -> bool {
        self.id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_id(&mut self, v: u64) {
        self.id = ::std::option::Option::Some(v);
    }

    pub fn get_id(&self) -> u64 {
        self.id.unwrap_or(0)
    }
}

impl ::protobuf::Message for AllocIdResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AllocIdResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AllocIdResponse {
    fn new() -> AllocIdResponse {
        AllocIdResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AllocIdResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "id",
                    AllocIdResponse::has_id,
                    AllocIdResponse::get_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AllocIdResponse>(
                    "AllocIdResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AllocIdResponse {
    fn clear(&mut self) {
        self.clear_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AllocIdResponse {
    fn eq(&self, other: &AllocIdResponse) -> bool {
        self.id == other.id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AllocIdResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetStoreRequest {
    // message fields
    store_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetStoreRequest {}

impl GetStoreRequest {
    pub fn new() -> GetStoreRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetStoreRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetStoreRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetStoreRequest,
        };
        unsafe {
            instance.get(|| {
                GetStoreRequest {
                    store_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 store_id = 1;

    pub fn clear_store_id(&mut self) {
        self.store_id = ::std::option::Option::None;
    }

    pub fn has_store_id(&self) -> bool {
        self.store_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = ::std::option::Option::Some(v);
    }

    pub fn get_store_id(&self) -> u64 {
        self.store_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for GetStoreRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.store_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.store_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.store_id {
            try!(os.write_uint64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetStoreRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetStoreRequest {
    fn new() -> GetStoreRequest {
        GetStoreRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetStoreRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "store_id",
                    GetStoreRequest::has_store_id,
                    GetStoreRequest::get_store_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetStoreRequest>(
                    "GetStoreRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetStoreRequest {
    fn clear(&mut self) {
        self.clear_store_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetStoreRequest {
    fn eq(&self, other: &GetStoreRequest) -> bool {
        self.store_id == other.store_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetStoreRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetStoreResponse {
    // message fields
    store: ::protobuf::SingularPtrField<super::metapb::Store>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetStoreResponse {}

impl GetStoreResponse {
    pub fn new() -> GetStoreResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetStoreResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetStoreResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetStoreResponse,
        };
        unsafe {
            instance.get(|| {
                GetStoreResponse {
                    store: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .metapb.Store store = 1;

    pub fn clear_store(&mut self) {
        self.store.clear();
    }

    pub fn has_store(&self) -> bool {
        self.store.is_some()
    }

    // Param is passed by value, moved
    pub fn set_store(&mut self, v: super::metapb::Store) {
        self.store = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_store(&mut self) -> &mut super::metapb::Store {
        if self.store.is_none() {
            self.store.set_default();
        };
        self.store.as_mut().unwrap()
    }

    // Take field
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store.take().unwrap_or_else(|| super::metapb::Store::new())
    }

    pub fn get_store(&self) -> &super::metapb::Store {
        self.store.as_ref().unwrap_or_else(|| super::metapb::Store::default_instance())
    }
}

impl ::protobuf::Message for GetStoreResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.store));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.store {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.store.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetStoreResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetStoreResponse {
    fn new() -> GetStoreResponse {
        GetStoreResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetStoreResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "store",
                    GetStoreResponse::has_store,
                    GetStoreResponse::get_store,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetStoreResponse>(
                    "GetStoreResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetStoreResponse {
    fn clear(&mut self) {
        self.clear_store();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetStoreResponse {
    fn eq(&self, other: &GetStoreResponse) -> bool {
        self.store == other.store &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetStoreResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetRegionRequest {
    // message fields
    region_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetRegionRequest {}

impl GetRegionRequest {
    pub fn new() -> GetRegionRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetRegionRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetRegionRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetRegionRequest,
        };
        unsafe {
            instance.get(|| {
                GetRegionRequest {
                    region_key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes region_key = 1;

    pub fn clear_region_key(&mut self) {
        self.region_key.clear();
    }

    pub fn has_region_key(&self) -> bool {
        self.region_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.region_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.region_key.is_none() {
            self.region_key.set_default();
        };
        self.region_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_key(&mut self) -> ::std::vec::Vec<u8> {
        self.region_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_region_key(&self) -> &[u8] {
        match self.region_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for GetRegionRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.region_key));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.region_key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region_key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetRegionRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetRegionRequest {
    fn new() -> GetRegionRequest {
        GetRegionRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetRegionRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "region_key",
                    GetRegionRequest::has_region_key,
                    GetRegionRequest::get_region_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetRegionRequest>(
                    "GetRegionRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetRegionRequest {
    fn clear(&mut self) {
        self.clear_region_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetRegionRequest {
    fn eq(&self, other: &GetRegionRequest) -> bool {
        self.region_key == other.region_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetRegionRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetRegionResponse {
    // message fields
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    leader: ::protobuf::SingularPtrField<super::metapb::Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetRegionResponse {}

impl GetRegionResponse {
    pub fn new() -> GetRegionResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetRegionResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetRegionResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetRegionResponse,
        };
        unsafe {
            instance.get(|| {
                GetRegionResponse {
                    region: ::protobuf::SingularPtrField::none(),
                    leader: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for GetRegionResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.leader));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.region {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.leader {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.leader.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetRegionResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetRegionResponse {
    fn new() -> GetRegionResponse {
        GetRegionResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetRegionResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region",
                    GetRegionResponse::has_region,
                    GetRegionResponse::get_region,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "leader",
                    GetRegionResponse::has_leader,
                    GetRegionResponse::get_leader,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetRegionResponse>(
                    "GetRegionResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetRegionResponse {
    fn clear(&mut self) {
        self.clear_region();
        self.clear_leader();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetRegionResponse {
    fn eq(&self, other: &GetRegionResponse) -> bool {
        self.region == other.region &&
        self.leader == other.leader &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetRegionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetRegionByIDRequest {
    // message fields
    region_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetRegionByIDRequest {}

impl GetRegionByIDRequest {
    pub fn new() -> GetRegionByIDRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetRegionByIDRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetRegionByIDRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetRegionByIDRequest,
        };
        unsafe {
            instance.get(|| {
                GetRegionByIDRequest {
                    region_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for GetRegionByIDRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.region_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.region_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region_id {
            try!(os.write_uint64(1, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetRegionByIDRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetRegionByIDRequest {
    fn new() -> GetRegionByIDRequest {
        GetRegionByIDRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetRegionByIDRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "region_id",
                    GetRegionByIDRequest::has_region_id,
                    GetRegionByIDRequest::get_region_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetRegionByIDRequest>(
                    "GetRegionByIDRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetRegionByIDRequest {
    fn clear(&mut self) {
        self.clear_region_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetRegionByIDRequest {
    fn eq(&self, other: &GetRegionByIDRequest) -> bool {
        self.region_id == other.region_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetRegionByIDRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetClusterConfigRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetClusterConfigRequest {}

impl GetClusterConfigRequest {
    pub fn new() -> GetClusterConfigRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetClusterConfigRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetClusterConfigRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetClusterConfigRequest,
        };
        unsafe {
            instance.get(|| {
                GetClusterConfigRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetClusterConfigRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetClusterConfigRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetClusterConfigRequest {
    fn new() -> GetClusterConfigRequest {
        GetClusterConfigRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetClusterConfigRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetClusterConfigRequest>(
                    "GetClusterConfigRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetClusterConfigRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetClusterConfigRequest {
    fn eq(&self, other: &GetClusterConfigRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetClusterConfigRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetClusterConfigResponse {
    // message fields
    cluster: ::protobuf::SingularPtrField<super::metapb::Cluster>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetClusterConfigResponse {}

impl GetClusterConfigResponse {
    pub fn new() -> GetClusterConfigResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetClusterConfigResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetClusterConfigResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetClusterConfigResponse,
        };
        unsafe {
            instance.get(|| {
                GetClusterConfigResponse {
                    cluster: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .metapb.Cluster cluster = 1;

    pub fn clear_cluster(&mut self) {
        self.cluster.clear();
    }

    pub fn has_cluster(&self) -> bool {
        self.cluster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster(&mut self, v: super::metapb::Cluster) {
        self.cluster = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster(&mut self) -> &mut super::metapb::Cluster {
        if self.cluster.is_none() {
            self.cluster.set_default();
        };
        self.cluster.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster(&mut self) -> super::metapb::Cluster {
        self.cluster.take().unwrap_or_else(|| super::metapb::Cluster::new())
    }

    pub fn get_cluster(&self) -> &super::metapb::Cluster {
        self.cluster.as_ref().unwrap_or_else(|| super::metapb::Cluster::default_instance())
    }
}

impl ::protobuf::Message for GetClusterConfigResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cluster));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.cluster {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cluster.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetClusterConfigResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetClusterConfigResponse {
    fn new() -> GetClusterConfigResponse {
        GetClusterConfigResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetClusterConfigResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cluster",
                    GetClusterConfigResponse::has_cluster,
                    GetClusterConfigResponse::get_cluster,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetClusterConfigResponse>(
                    "GetClusterConfigResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetClusterConfigResponse {
    fn clear(&mut self) {
        self.clear_cluster();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetClusterConfigResponse {
    fn eq(&self, other: &GetClusterConfigResponse) -> bool {
        self.cluster == other.cluster &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetClusterConfigResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PutStoreRequest {
    // message fields
    store: ::protobuf::SingularPtrField<super::metapb::Store>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PutStoreRequest {}

impl PutStoreRequest {
    pub fn new() -> PutStoreRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutStoreRequest {
        static mut instance: ::protobuf::lazy::Lazy<PutStoreRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutStoreRequest,
        };
        unsafe {
            instance.get(|| {
                PutStoreRequest {
                    store: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .metapb.Store store = 1;

    pub fn clear_store(&mut self) {
        self.store.clear();
    }

    pub fn has_store(&self) -> bool {
        self.store.is_some()
    }

    // Param is passed by value, moved
    pub fn set_store(&mut self, v: super::metapb::Store) {
        self.store = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_store(&mut self) -> &mut super::metapb::Store {
        if self.store.is_none() {
            self.store.set_default();
        };
        self.store.as_mut().unwrap()
    }

    // Take field
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store.take().unwrap_or_else(|| super::metapb::Store::new())
    }

    pub fn get_store(&self) -> &super::metapb::Store {
        self.store.as_ref().unwrap_or_else(|| super::metapb::Store::default_instance())
    }
}

impl ::protobuf::Message for PutStoreRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.store));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.store {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.store.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PutStoreRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PutStoreRequest {
    fn new() -> PutStoreRequest {
        PutStoreRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutStoreRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "store",
                    PutStoreRequest::has_store,
                    PutStoreRequest::get_store,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PutStoreRequest>(
                    "PutStoreRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutStoreRequest {
    fn clear(&mut self) {
        self.clear_store();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PutStoreRequest {
    fn eq(&self, other: &PutStoreRequest) -> bool {
        self.store == other.store &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PutStoreRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PutStoreResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PutStoreResponse {}

impl PutStoreResponse {
    pub fn new() -> PutStoreResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutStoreResponse {
        static mut instance: ::protobuf::lazy::Lazy<PutStoreResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutStoreResponse,
        };
        unsafe {
            instance.get(|| {
                PutStoreResponse {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for PutStoreResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PutStoreResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PutStoreResponse {
    fn new() -> PutStoreResponse {
        PutStoreResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutStoreResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PutStoreResponse>(
                    "PutStoreResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutStoreResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PutStoreResponse {
    fn eq(&self, other: &PutStoreResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PutStoreResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PDMember {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    client_urls: ::protobuf::RepeatedField<::std::string::String>,
    peer_urls: ::protobuf::RepeatedField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PDMember {}

impl PDMember {
    pub fn new() -> PDMember {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PDMember {
        static mut instance: ::protobuf::lazy::Lazy<PDMember> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PDMember,
        };
        unsafe {
            instance.get(|| {
                PDMember {
                    name: ::protobuf::SingularField::none(),
                    client_urls: ::protobuf::RepeatedField::new(),
                    peer_urls: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string name = 1;

    pub fn clear_name(&mut self) {
        self.name.clear();
    }

    pub fn has_name(&self) -> bool {
        self.name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_name(&mut self, v: ::std::string::String) {
        self.name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_name(&mut self) -> &mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name(&self) -> &str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // repeated string client_urls = 2;

    pub fn clear_client_urls(&mut self) {
        self.client_urls.clear();
    }

    // Param is passed by value, moved
    pub fn set_client_urls(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.client_urls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_client_urls(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.client_urls
    }

    // Take field
    pub fn take_client_urls(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.client_urls, ::protobuf::RepeatedField::new())
    }

    pub fn get_client_urls(&self) -> &[::std::string::String] {
        &self.client_urls
    }

    // repeated string peer_urls = 3;

    pub fn clear_peer_urls(&mut self) {
        self.peer_urls.clear();
    }

    // Param is passed by value, moved
    pub fn set_peer_urls(&mut self, v: ::protobuf::RepeatedField<::std::string::String>) {
        self.peer_urls = v;
    }

    // Mutable pointer to the field.
    pub fn mut_peer_urls(&mut self) -> &mut ::protobuf::RepeatedField<::std::string::String> {
        &mut self.peer_urls
    }

    // Take field
    pub fn take_peer_urls(&mut self) -> ::protobuf::RepeatedField<::std::string::String> {
        ::std::mem::replace(&mut self.peer_urls, ::protobuf::RepeatedField::new())
    }

    pub fn get_peer_urls(&self) -> &[::std::string::String] {
        &self.peer_urls
    }
}

impl ::protobuf::Message for PDMember {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.name));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.client_urls));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_string_into(wire_type, is, &mut self.peer_urls));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.name {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.client_urls {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in &self.peer_urls {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        for v in &self.client_urls {
            try!(os.write_string(2, &v));
        };
        for v in &self.peer_urls {
            try!(os.write_string(3, &v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PDMember>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PDMember {
    fn new() -> PDMember {
        PDMember::new()
    }

    fn descriptor_static(_: ::std::option::Option<PDMember>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    PDMember::has_name,
                    PDMember::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "client_urls",
                    PDMember::get_client_urls,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_string_accessor(
                    "peer_urls",
                    PDMember::get_peer_urls,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PDMember>(
                    "PDMember",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PDMember {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_client_urls();
        self.clear_peer_urls();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PDMember {
    fn eq(&self, other: &PDMember) -> bool {
        self.name == other.name &&
        self.client_urls == other.client_urls &&
        self.peer_urls == other.peer_urls &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PDMember {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetPDMembersRequest {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetPDMembersRequest {}

impl GetPDMembersRequest {
    pub fn new() -> GetPDMembersRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetPDMembersRequest {
        static mut instance: ::protobuf::lazy::Lazy<GetPDMembersRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetPDMembersRequest,
        };
        unsafe {
            instance.get(|| {
                GetPDMembersRequest {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for GetPDMembersRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetPDMembersRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetPDMembersRequest {
    fn new() -> GetPDMembersRequest {
        GetPDMembersRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetPDMembersRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<GetPDMembersRequest>(
                    "GetPDMembersRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetPDMembersRequest {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetPDMembersRequest {
    fn eq(&self, other: &GetPDMembersRequest) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetPDMembersRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetPDMembersResponse {
    // message fields
    members: ::protobuf::RepeatedField<PDMember>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for GetPDMembersResponse {}

impl GetPDMembersResponse {
    pub fn new() -> GetPDMembersResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static GetPDMembersResponse {
        static mut instance: ::protobuf::lazy::Lazy<GetPDMembersResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const GetPDMembersResponse,
        };
        unsafe {
            instance.get(|| {
                GetPDMembersResponse {
                    members: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .pdpb.PDMember members = 1;

    pub fn clear_members(&mut self) {
        self.members.clear();
    }

    // Param is passed by value, moved
    pub fn set_members(&mut self, v: ::protobuf::RepeatedField<PDMember>) {
        self.members = v;
    }

    // Mutable pointer to the field.
    pub fn mut_members(&mut self) -> &mut ::protobuf::RepeatedField<PDMember> {
        &mut self.members
    }

    // Take field
    pub fn take_members(&mut self) -> ::protobuf::RepeatedField<PDMember> {
        ::std::mem::replace(&mut self.members, ::protobuf::RepeatedField::new())
    }

    pub fn get_members(&self) -> &[PDMember] {
        &self.members
    }
}

impl ::protobuf::Message for GetPDMembersResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.members));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.members {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.members {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<GetPDMembersResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for GetPDMembersResponse {
    fn new() -> GetPDMembersResponse {
        GetPDMembersResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<GetPDMembersResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "members",
                    GetPDMembersResponse::get_members,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<GetPDMembersResponse>(
                    "GetPDMembersResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for GetPDMembersResponse {
    fn clear(&mut self) {
        self.clear_members();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetPDMembersResponse {
    fn eq(&self, other: &GetPDMembersResponse) -> bool {
        self.members == other.members &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetPDMembersResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PeerStats {
    // message fields
    peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    down_seconds: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PeerStats {}

impl PeerStats {
    pub fn new() -> PeerStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PeerStats {
        static mut instance: ::protobuf::lazy::Lazy<PeerStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PeerStats,
        };
        unsafe {
            instance.get(|| {
                PeerStats {
                    peer: ::protobuf::SingularPtrField::none(),
                    down_seconds: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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

    // optional uint64 down_seconds = 2;

    pub fn clear_down_seconds(&mut self) {
        self.down_seconds = ::std::option::Option::None;
    }

    pub fn has_down_seconds(&self) -> bool {
        self.down_seconds.is_some()
    }

    // Param is passed by value, moved
    pub fn set_down_seconds(&mut self, v: u64) {
        self.down_seconds = ::std::option::Option::Some(v);
    }

    pub fn get_down_seconds(&self) -> u64 {
        self.down_seconds.unwrap_or(0)
    }
}

impl ::protobuf::Message for PeerStats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.down_seconds = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.peer {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.down_seconds {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.peer.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.down_seconds {
            try!(os.write_uint64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PeerStats>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PeerStats {
    fn new() -> PeerStats {
        PeerStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<PeerStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "peer",
                    PeerStats::has_peer,
                    PeerStats::get_peer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "down_seconds",
                    PeerStats::has_down_seconds,
                    PeerStats::get_down_seconds,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PeerStats>(
                    "PeerStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PeerStats {
    fn clear(&mut self) {
        self.clear_peer();
        self.clear_down_seconds();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PeerStats {
    fn eq(&self, other: &PeerStats) -> bool {
        self.peer == other.peer &&
        self.down_seconds == other.down_seconds &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PeerStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RegionHeartbeatRequest {
    // message fields
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    leader: ::protobuf::SingularPtrField<super::metapb::Peer>,
    down_peers: ::protobuf::RepeatedField<PeerStats>,
    pending_peers: ::protobuf::RepeatedField<super::metapb::Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionHeartbeatRequest {}

impl RegionHeartbeatRequest {
    pub fn new() -> RegionHeartbeatRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionHeartbeatRequest {
        static mut instance: ::protobuf::lazy::Lazy<RegionHeartbeatRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionHeartbeatRequest,
        };
        unsafe {
            instance.get(|| {
                RegionHeartbeatRequest {
                    region: ::protobuf::SingularPtrField::none(),
                    leader: ::protobuf::SingularPtrField::none(),
                    down_peers: ::protobuf::RepeatedField::new(),
                    pending_peers: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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

    // repeated .pdpb.PeerStats down_peers = 3;

    pub fn clear_down_peers(&mut self) {
        self.down_peers.clear();
    }

    // Param is passed by value, moved
    pub fn set_down_peers(&mut self, v: ::protobuf::RepeatedField<PeerStats>) {
        self.down_peers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_down_peers(&mut self) -> &mut ::protobuf::RepeatedField<PeerStats> {
        &mut self.down_peers
    }

    // Take field
    pub fn take_down_peers(&mut self) -> ::protobuf::RepeatedField<PeerStats> {
        ::std::mem::replace(&mut self.down_peers, ::protobuf::RepeatedField::new())
    }

    pub fn get_down_peers(&self) -> &[PeerStats] {
        &self.down_peers
    }

    // repeated .metapb.Peer pending_peers = 4;

    pub fn clear_pending_peers(&mut self) {
        self.pending_peers.clear();
    }

    // Param is passed by value, moved
    pub fn set_pending_peers(&mut self, v: ::protobuf::RepeatedField<super::metapb::Peer>) {
        self.pending_peers = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pending_peers(&mut self) -> &mut ::protobuf::RepeatedField<super::metapb::Peer> {
        &mut self.pending_peers
    }

    // Take field
    pub fn take_pending_peers(&mut self) -> ::protobuf::RepeatedField<super::metapb::Peer> {
        ::std::mem::replace(&mut self.pending_peers, ::protobuf::RepeatedField::new())
    }

    pub fn get_pending_peers(&self) -> &[super::metapb::Peer] {
        &self.pending_peers
    }
}

impl ::protobuf::Message for RegionHeartbeatRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.leader));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.down_peers));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pending_peers));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.region {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.leader {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.down_peers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.pending_peers {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.leader.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.down_peers {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in &self.pending_peers {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RegionHeartbeatRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RegionHeartbeatRequest {
    fn new() -> RegionHeartbeatRequest {
        RegionHeartbeatRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionHeartbeatRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region",
                    RegionHeartbeatRequest::has_region,
                    RegionHeartbeatRequest::get_region,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "leader",
                    RegionHeartbeatRequest::has_leader,
                    RegionHeartbeatRequest::get_leader,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "down_peers",
                    RegionHeartbeatRequest::get_down_peers,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "pending_peers",
                    RegionHeartbeatRequest::get_pending_peers,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionHeartbeatRequest>(
                    "RegionHeartbeatRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionHeartbeatRequest {
    fn clear(&mut self) {
        self.clear_region();
        self.clear_leader();
        self.clear_down_peers();
        self.clear_pending_peers();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RegionHeartbeatRequest {
    fn eq(&self, other: &RegionHeartbeatRequest) -> bool {
        self.region == other.region &&
        self.leader == other.leader &&
        self.down_peers == other.down_peers &&
        self.pending_peers == other.pending_peers &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RegionHeartbeatRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ChangePeer {
    // message fields
    change_type: ::std::option::Option<super::eraftpb::ConfChangeType>,
    peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ChangePeer {}

impl ChangePeer {
    pub fn new() -> ChangePeer {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ChangePeer {
        static mut instance: ::protobuf::lazy::Lazy<ChangePeer> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ChangePeer,
        };
        unsafe {
            instance.get(|| {
                ChangePeer {
                    change_type: ::std::option::Option::None,
                    peer: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for ChangePeer {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.change_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.change_type {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in &self.peer {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.change_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.peer.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ChangePeer>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ChangePeer {
    fn new() -> ChangePeer {
        ChangePeer::new()
    }

    fn descriptor_static(_: ::std::option::Option<ChangePeer>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "change_type",
                    ChangePeer::has_change_type,
                    ChangePeer::get_change_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "peer",
                    ChangePeer::has_peer,
                    ChangePeer::get_peer,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ChangePeer>(
                    "ChangePeer",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ChangePeer {
    fn clear(&mut self) {
        self.clear_change_type();
        self.clear_peer();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ChangePeer {
    fn eq(&self, other: &ChangePeer) -> bool {
        self.change_type == other.change_type &&
        self.peer == other.peer &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ChangePeer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct TransferLeader {
    // message fields
    peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for TransferLeader {}

impl TransferLeader {
    pub fn new() -> TransferLeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static TransferLeader {
        static mut instance: ::protobuf::lazy::Lazy<TransferLeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const TransferLeader,
        };
        unsafe {
            instance.get(|| {
                TransferLeader {
                    peer: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for TransferLeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.peer));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.peer {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.peer.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<TransferLeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for TransferLeader {
    fn new() -> TransferLeader {
        TransferLeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<TransferLeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "peer",
                    TransferLeader::has_peer,
                    TransferLeader::get_peer,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<TransferLeader>(
                    "TransferLeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for TransferLeader {
    fn clear(&mut self) {
        self.clear_peer();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TransferLeader {
    fn eq(&self, other: &TransferLeader) -> bool {
        self.peer == other.peer &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for TransferLeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RegionHeartbeatResponse {
    // message fields
    change_peer: ::protobuf::SingularPtrField<ChangePeer>,
    transfer_leader: ::protobuf::SingularPtrField<TransferLeader>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionHeartbeatResponse {}

impl RegionHeartbeatResponse {
    pub fn new() -> RegionHeartbeatResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionHeartbeatResponse {
        static mut instance: ::protobuf::lazy::Lazy<RegionHeartbeatResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionHeartbeatResponse,
        };
        unsafe {
            instance.get(|| {
                RegionHeartbeatResponse {
                    change_peer: ::protobuf::SingularPtrField::none(),
                    transfer_leader: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .pdpb.ChangePeer change_peer = 1;

    pub fn clear_change_peer(&mut self) {
        self.change_peer.clear();
    }

    pub fn has_change_peer(&self) -> bool {
        self.change_peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_change_peer(&mut self, v: ChangePeer) {
        self.change_peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_change_peer(&mut self) -> &mut ChangePeer {
        if self.change_peer.is_none() {
            self.change_peer.set_default();
        };
        self.change_peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_change_peer(&mut self) -> ChangePeer {
        self.change_peer.take().unwrap_or_else(|| ChangePeer::new())
    }

    pub fn get_change_peer(&self) -> &ChangePeer {
        self.change_peer.as_ref().unwrap_or_else(|| ChangePeer::default_instance())
    }

    // optional .pdpb.TransferLeader transfer_leader = 2;

    pub fn clear_transfer_leader(&mut self) {
        self.transfer_leader.clear();
    }

    pub fn has_transfer_leader(&self) -> bool {
        self.transfer_leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_transfer_leader(&mut self, v: TransferLeader) {
        self.transfer_leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_transfer_leader(&mut self) -> &mut TransferLeader {
        if self.transfer_leader.is_none() {
            self.transfer_leader.set_default();
        };
        self.transfer_leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_transfer_leader(&mut self) -> TransferLeader {
        self.transfer_leader.take().unwrap_or_else(|| TransferLeader::new())
    }

    pub fn get_transfer_leader(&self) -> &TransferLeader {
        self.transfer_leader.as_ref().unwrap_or_else(|| TransferLeader::default_instance())
    }
}

impl ::protobuf::Message for RegionHeartbeatResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.change_peer));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.transfer_leader));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.change_peer {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.transfer_leader {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.change_peer.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.transfer_leader.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RegionHeartbeatResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RegionHeartbeatResponse {
    fn new() -> RegionHeartbeatResponse {
        RegionHeartbeatResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionHeartbeatResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "change_peer",
                    RegionHeartbeatResponse::has_change_peer,
                    RegionHeartbeatResponse::get_change_peer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "transfer_leader",
                    RegionHeartbeatResponse::has_transfer_leader,
                    RegionHeartbeatResponse::get_transfer_leader,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionHeartbeatResponse>(
                    "RegionHeartbeatResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionHeartbeatResponse {
    fn clear(&mut self) {
        self.clear_change_peer();
        self.clear_transfer_leader();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RegionHeartbeatResponse {
    fn eq(&self, other: &RegionHeartbeatResponse) -> bool {
        self.change_peer == other.change_peer &&
        self.transfer_leader == other.transfer_leader &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RegionHeartbeatResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PutClusterConfigRequest {
    // message fields
    cluster: ::protobuf::SingularPtrField<super::metapb::Cluster>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PutClusterConfigRequest {}

impl PutClusterConfigRequest {
    pub fn new() -> PutClusterConfigRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutClusterConfigRequest {
        static mut instance: ::protobuf::lazy::Lazy<PutClusterConfigRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutClusterConfigRequest,
        };
        unsafe {
            instance.get(|| {
                PutClusterConfigRequest {
                    cluster: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .metapb.Cluster cluster = 1;

    pub fn clear_cluster(&mut self) {
        self.cluster.clear();
    }

    pub fn has_cluster(&self) -> bool {
        self.cluster.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster(&mut self, v: super::metapb::Cluster) {
        self.cluster = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster(&mut self) -> &mut super::metapb::Cluster {
        if self.cluster.is_none() {
            self.cluster.set_default();
        };
        self.cluster.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster(&mut self) -> super::metapb::Cluster {
        self.cluster.take().unwrap_or_else(|| super::metapb::Cluster::new())
    }

    pub fn get_cluster(&self) -> &super::metapb::Cluster {
        self.cluster.as_ref().unwrap_or_else(|| super::metapb::Cluster::default_instance())
    }
}

impl ::protobuf::Message for PutClusterConfigRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cluster));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.cluster {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cluster.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PutClusterConfigRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PutClusterConfigRequest {
    fn new() -> PutClusterConfigRequest {
        PutClusterConfigRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutClusterConfigRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cluster",
                    PutClusterConfigRequest::has_cluster,
                    PutClusterConfigRequest::get_cluster,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<PutClusterConfigRequest>(
                    "PutClusterConfigRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutClusterConfigRequest {
    fn clear(&mut self) {
        self.clear_cluster();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PutClusterConfigRequest {
    fn eq(&self, other: &PutClusterConfigRequest) -> bool {
        self.cluster == other.cluster &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PutClusterConfigRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct PutClusterConfigResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for PutClusterConfigResponse {}

impl PutClusterConfigResponse {
    pub fn new() -> PutClusterConfigResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static PutClusterConfigResponse {
        static mut instance: ::protobuf::lazy::Lazy<PutClusterConfigResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const PutClusterConfigResponse,
        };
        unsafe {
            instance.get(|| {
                PutClusterConfigResponse {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for PutClusterConfigResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<PutClusterConfigResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for PutClusterConfigResponse {
    fn new() -> PutClusterConfigResponse {
        PutClusterConfigResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<PutClusterConfigResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<PutClusterConfigResponse>(
                    "PutClusterConfigResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for PutClusterConfigResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for PutClusterConfigResponse {
    fn eq(&self, other: &PutClusterConfigResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for PutClusterConfigResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AskSplitRequest {
    // message fields
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AskSplitRequest {}

impl AskSplitRequest {
    pub fn new() -> AskSplitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AskSplitRequest {
        static mut instance: ::protobuf::lazy::Lazy<AskSplitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AskSplitRequest,
        };
        unsafe {
            instance.get(|| {
                AskSplitRequest {
                    region: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for AskSplitRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.region {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AskSplitRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AskSplitRequest {
    fn new() -> AskSplitRequest {
        AskSplitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AskSplitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region",
                    AskSplitRequest::has_region,
                    AskSplitRequest::get_region,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AskSplitRequest>(
                    "AskSplitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AskSplitRequest {
    fn clear(&mut self) {
        self.clear_region();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AskSplitRequest {
    fn eq(&self, other: &AskSplitRequest) -> bool {
        self.region == other.region &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AskSplitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AskSplitResponse {
    // message fields
    new_region_id: ::std::option::Option<u64>,
    new_peer_ids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AskSplitResponse {}

impl AskSplitResponse {
    pub fn new() -> AskSplitResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AskSplitResponse {
        static mut instance: ::protobuf::lazy::Lazy<AskSplitResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AskSplitResponse,
        };
        unsafe {
            instance.get(|| {
                AskSplitResponse {
                    new_region_id: ::std::option::Option::None,
                    new_peer_ids: ::std::vec::Vec::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 new_region_id = 1;

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

    // repeated uint64 new_peer_ids = 2;

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
}

impl ::protobuf::Message for AskSplitResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.new_region_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.new_peer_ids));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.new_region_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.new_peer_ids {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.new_region_id {
            try!(os.write_uint64(1, v));
        };
        for v in &self.new_peer_ids {
            try!(os.write_uint64(2, *v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<AskSplitResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AskSplitResponse {
    fn new() -> AskSplitResponse {
        AskSplitResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AskSplitResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "new_region_id",
                    AskSplitResponse::has_new_region_id,
                    AskSplitResponse::get_new_region_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "new_peer_ids",
                    AskSplitResponse::get_new_peer_ids,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AskSplitResponse>(
                    "AskSplitResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AskSplitResponse {
    fn clear(&mut self) {
        self.clear_new_region_id();
        self.clear_new_peer_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AskSplitResponse {
    fn eq(&self, other: &AskSplitResponse) -> bool {
        self.new_region_id == other.new_region_id &&
        self.new_peer_ids == other.new_peer_ids &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AskSplitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StoreStats {
    // message fields
    store_id: ::std::option::Option<u64>,
    capacity: ::std::option::Option<u64>,
    available: ::std::option::Option<u64>,
    region_count: ::std::option::Option<u32>,
    sending_snap_count: ::std::option::Option<u32>,
    receiving_snap_count: ::std::option::Option<u32>,
    start_time: ::std::option::Option<u32>,
    applying_snap_count: ::std::option::Option<u32>,
    is_busy: ::std::option::Option<bool>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StoreStats {}

impl StoreStats {
    pub fn new() -> StoreStats {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StoreStats {
        static mut instance: ::protobuf::lazy::Lazy<StoreStats> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StoreStats,
        };
        unsafe {
            instance.get(|| {
                StoreStats {
                    store_id: ::std::option::Option::None,
                    capacity: ::std::option::Option::None,
                    available: ::std::option::Option::None,
                    region_count: ::std::option::Option::None,
                    sending_snap_count: ::std::option::Option::None,
                    receiving_snap_count: ::std::option::Option::None,
                    start_time: ::std::option::Option::None,
                    applying_snap_count: ::std::option::Option::None,
                    is_busy: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 store_id = 1;

    pub fn clear_store_id(&mut self) {
        self.store_id = ::std::option::Option::None;
    }

    pub fn has_store_id(&self) -> bool {
        self.store_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = ::std::option::Option::Some(v);
    }

    pub fn get_store_id(&self) -> u64 {
        self.store_id.unwrap_or(0)
    }

    // optional uint64 capacity = 2;

    pub fn clear_capacity(&mut self) {
        self.capacity = ::std::option::Option::None;
    }

    pub fn has_capacity(&self) -> bool {
        self.capacity.is_some()
    }

    // Param is passed by value, moved
    pub fn set_capacity(&mut self, v: u64) {
        self.capacity = ::std::option::Option::Some(v);
    }

    pub fn get_capacity(&self) -> u64 {
        self.capacity.unwrap_or(0)
    }

    // optional uint64 available = 3;

    pub fn clear_available(&mut self) {
        self.available = ::std::option::Option::None;
    }

    pub fn has_available(&self) -> bool {
        self.available.is_some()
    }

    // Param is passed by value, moved
    pub fn set_available(&mut self, v: u64) {
        self.available = ::std::option::Option::Some(v);
    }

    pub fn get_available(&self) -> u64 {
        self.available.unwrap_or(0)
    }

    // optional uint32 region_count = 4;

    pub fn clear_region_count(&mut self) {
        self.region_count = ::std::option::Option::None;
    }

    pub fn has_region_count(&self) -> bool {
        self.region_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_count(&mut self, v: u32) {
        self.region_count = ::std::option::Option::Some(v);
    }

    pub fn get_region_count(&self) -> u32 {
        self.region_count.unwrap_or(0)
    }

    // optional uint32 sending_snap_count = 5;

    pub fn clear_sending_snap_count(&mut self) {
        self.sending_snap_count = ::std::option::Option::None;
    }

    pub fn has_sending_snap_count(&self) -> bool {
        self.sending_snap_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sending_snap_count(&mut self, v: u32) {
        self.sending_snap_count = ::std::option::Option::Some(v);
    }

    pub fn get_sending_snap_count(&self) -> u32 {
        self.sending_snap_count.unwrap_or(0)
    }

    // optional uint32 receiving_snap_count = 6;

    pub fn clear_receiving_snap_count(&mut self) {
        self.receiving_snap_count = ::std::option::Option::None;
    }

    pub fn has_receiving_snap_count(&self) -> bool {
        self.receiving_snap_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_receiving_snap_count(&mut self, v: u32) {
        self.receiving_snap_count = ::std::option::Option::Some(v);
    }

    pub fn get_receiving_snap_count(&self) -> u32 {
        self.receiving_snap_count.unwrap_or(0)
    }

    // optional uint32 start_time = 7;

    pub fn clear_start_time(&mut self) {
        self.start_time = ::std::option::Option::None;
    }

    pub fn has_start_time(&self) -> bool {
        self.start_time.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_time(&mut self, v: u32) {
        self.start_time = ::std::option::Option::Some(v);
    }

    pub fn get_start_time(&self) -> u32 {
        self.start_time.unwrap_or(0)
    }

    // optional uint32 applying_snap_count = 8;

    pub fn clear_applying_snap_count(&mut self) {
        self.applying_snap_count = ::std::option::Option::None;
    }

    pub fn has_applying_snap_count(&self) -> bool {
        self.applying_snap_count.is_some()
    }

    // Param is passed by value, moved
    pub fn set_applying_snap_count(&mut self, v: u32) {
        self.applying_snap_count = ::std::option::Option::Some(v);
    }

    pub fn get_applying_snap_count(&self) -> u32 {
        self.applying_snap_count.unwrap_or(0)
    }

    // optional bool is_busy = 9;

    pub fn clear_is_busy(&mut self) {
        self.is_busy = ::std::option::Option::None;
    }

    pub fn has_is_busy(&self) -> bool {
        self.is_busy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_busy(&mut self, v: bool) {
        self.is_busy = ::std::option::Option::Some(v);
    }

    pub fn get_is_busy(&self) -> bool {
        self.is_busy.unwrap_or(false)
    }
}

impl ::protobuf::Message for StoreStats {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.store_id = ::std::option::Option::Some(tmp);
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.capacity = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.available = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.region_count = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.sending_snap_count = ::std::option::Option::Some(tmp);
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.receiving_snap_count = ::std::option::Option::Some(tmp);
                },
                7 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.start_time = ::std::option::Option::Some(tmp);
                },
                8 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.applying_snap_count = ::std::option::Option::Some(tmp);
                },
                9 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_bool());
                    self.is_busy = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.store_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.capacity {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.available {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.region_count {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.sending_snap_count {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.receiving_snap_count {
            my_size += ::protobuf::rt::value_size(6, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.start_time {
            my_size += ::protobuf::rt::value_size(7, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.applying_snap_count {
            my_size += ::protobuf::rt::value_size(8, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.is_busy.is_some() {
            my_size += 2;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.store_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.capacity {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.available {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.region_count {
            try!(os.write_uint32(4, v));
        };
        if let Some(v) = self.sending_snap_count {
            try!(os.write_uint32(5, v));
        };
        if let Some(v) = self.receiving_snap_count {
            try!(os.write_uint32(6, v));
        };
        if let Some(v) = self.start_time {
            try!(os.write_uint32(7, v));
        };
        if let Some(v) = self.applying_snap_count {
            try!(os.write_uint32(8, v));
        };
        if let Some(v) = self.is_busy {
            try!(os.write_bool(9, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StoreStats>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StoreStats {
    fn new() -> StoreStats {
        StoreStats::new()
    }

    fn descriptor_static(_: ::std::option::Option<StoreStats>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "store_id",
                    StoreStats::has_store_id,
                    StoreStats::get_store_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "capacity",
                    StoreStats::has_capacity,
                    StoreStats::get_capacity,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "available",
                    StoreStats::has_available,
                    StoreStats::get_available,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "region_count",
                    StoreStats::has_region_count,
                    StoreStats::get_region_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "sending_snap_count",
                    StoreStats::has_sending_snap_count,
                    StoreStats::get_sending_snap_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "receiving_snap_count",
                    StoreStats::has_receiving_snap_count,
                    StoreStats::get_receiving_snap_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "start_time",
                    StoreStats::has_start_time,
                    StoreStats::get_start_time,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "applying_snap_count",
                    StoreStats::has_applying_snap_count,
                    StoreStats::get_applying_snap_count,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bool_accessor(
                    "is_busy",
                    StoreStats::has_is_busy,
                    StoreStats::get_is_busy,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StoreStats>(
                    "StoreStats",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StoreStats {
    fn clear(&mut self) {
        self.clear_store_id();
        self.clear_capacity();
        self.clear_available();
        self.clear_region_count();
        self.clear_sending_snap_count();
        self.clear_receiving_snap_count();
        self.clear_start_time();
        self.clear_applying_snap_count();
        self.clear_is_busy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StoreStats {
    fn eq(&self, other: &StoreStats) -> bool {
        self.store_id == other.store_id &&
        self.capacity == other.capacity &&
        self.available == other.available &&
        self.region_count == other.region_count &&
        self.sending_snap_count == other.sending_snap_count &&
        self.receiving_snap_count == other.receiving_snap_count &&
        self.start_time == other.start_time &&
        self.applying_snap_count == other.applying_snap_count &&
        self.is_busy == other.is_busy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StoreStats {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StoreHeartbeatRequest {
    // message fields
    stats: ::protobuf::SingularPtrField<StoreStats>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StoreHeartbeatRequest {}

impl StoreHeartbeatRequest {
    pub fn new() -> StoreHeartbeatRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StoreHeartbeatRequest {
        static mut instance: ::protobuf::lazy::Lazy<StoreHeartbeatRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StoreHeartbeatRequest,
        };
        unsafe {
            instance.get(|| {
                StoreHeartbeatRequest {
                    stats: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .pdpb.StoreStats stats = 1;

    pub fn clear_stats(&mut self) {
        self.stats.clear();
    }

    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stats(&mut self, v: StoreStats) {
        self.stats = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stats(&mut self) -> &mut StoreStats {
        if self.stats.is_none() {
            self.stats.set_default();
        };
        self.stats.as_mut().unwrap()
    }

    // Take field
    pub fn take_stats(&mut self) -> StoreStats {
        self.stats.take().unwrap_or_else(|| StoreStats::new())
    }

    pub fn get_stats(&self) -> &StoreStats {
        self.stats.as_ref().unwrap_or_else(|| StoreStats::default_instance())
    }
}

impl ::protobuf::Message for StoreHeartbeatRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stats));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.stats {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.stats.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StoreHeartbeatRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StoreHeartbeatRequest {
    fn new() -> StoreHeartbeatRequest {
        StoreHeartbeatRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<StoreHeartbeatRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stats",
                    StoreHeartbeatRequest::has_stats,
                    StoreHeartbeatRequest::get_stats,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StoreHeartbeatRequest>(
                    "StoreHeartbeatRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StoreHeartbeatRequest {
    fn clear(&mut self) {
        self.clear_stats();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StoreHeartbeatRequest {
    fn eq(&self, other: &StoreHeartbeatRequest) -> bool {
        self.stats == other.stats &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StoreHeartbeatRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StoreHeartbeatResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StoreHeartbeatResponse {}

impl StoreHeartbeatResponse {
    pub fn new() -> StoreHeartbeatResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StoreHeartbeatResponse {
        static mut instance: ::protobuf::lazy::Lazy<StoreHeartbeatResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StoreHeartbeatResponse,
        };
        unsafe {
            instance.get(|| {
                StoreHeartbeatResponse {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for StoreHeartbeatResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StoreHeartbeatResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StoreHeartbeatResponse {
    fn new() -> StoreHeartbeatResponse {
        StoreHeartbeatResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<StoreHeartbeatResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StoreHeartbeatResponse>(
                    "StoreHeartbeatResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StoreHeartbeatResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StoreHeartbeatResponse {
    fn eq(&self, other: &StoreHeartbeatResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StoreHeartbeatResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReportSplitRequest {
    // message fields
    left: ::protobuf::SingularPtrField<super::metapb::Region>,
    right: ::protobuf::SingularPtrField<super::metapb::Region>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReportSplitRequest {}

impl ReportSplitRequest {
    pub fn new() -> ReportSplitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReportSplitRequest {
        static mut instance: ::protobuf::lazy::Lazy<ReportSplitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReportSplitRequest,
        };
        unsafe {
            instance.get(|| {
                ReportSplitRequest {
                    left: ::protobuf::SingularPtrField::none(),
                    right: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
}

impl ::protobuf::Message for ReportSplitRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.left));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.right));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.left {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.right {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.left.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.right.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReportSplitRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReportSplitRequest {
    fn new() -> ReportSplitRequest {
        ReportSplitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReportSplitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "left",
                    ReportSplitRequest::has_left,
                    ReportSplitRequest::get_left,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "right",
                    ReportSplitRequest::has_right,
                    ReportSplitRequest::get_right,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ReportSplitRequest>(
                    "ReportSplitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReportSplitRequest {
    fn clear(&mut self) {
        self.clear_left();
        self.clear_right();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReportSplitRequest {
    fn eq(&self, other: &ReportSplitRequest) -> bool {
        self.left == other.left &&
        self.right == other.right &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReportSplitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ReportSplitResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ReportSplitResponse {}

impl ReportSplitResponse {
    pub fn new() -> ReportSplitResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ReportSplitResponse {
        static mut instance: ::protobuf::lazy::Lazy<ReportSplitResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ReportSplitResponse,
        };
        unsafe {
            instance.get(|| {
                ReportSplitResponse {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ReportSplitResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ReportSplitResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ReportSplitResponse {
    fn new() -> ReportSplitResponse {
        ReportSplitResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ReportSplitResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ReportSplitResponse>(
                    "ReportSplitResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ReportSplitResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ReportSplitResponse {
    fn eq(&self, other: &ReportSplitResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ReportSplitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RequestHeader {
    // message fields
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    cluster_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RequestHeader {}

impl RequestHeader {
    pub fn new() -> RequestHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RequestHeader {
        static mut instance: ::protobuf::lazy::Lazy<RequestHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RequestHeader,
        };
        unsafe {
            instance.get(|| {
                RequestHeader {
                    uuid: ::protobuf::SingularField::none(),
                    cluster_id: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes uuid = 1;

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

    // optional uint64 cluster_id = 2;

    pub fn clear_cluster_id(&mut self) {
        self.cluster_id = ::std::option::Option::None;
    }

    pub fn has_cluster_id(&self) -> bool {
        self.cluster_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_id(&mut self, v: u64) {
        self.cluster_id = ::std::option::Option::Some(v);
    }

    pub fn get_cluster_id(&self) -> u64 {
        self.cluster_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for RequestHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.uuid));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.cluster_id = ::std::option::Option::Some(tmp);
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.uuid {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.cluster_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uuid.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.cluster_id {
            try!(os.write_uint64(2, v));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<RequestHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RequestHeader {
    fn new() -> RequestHeader {
        RequestHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<RequestHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "uuid",
                    RequestHeader::has_uuid,
                    RequestHeader::get_uuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "cluster_id",
                    RequestHeader::has_cluster_id,
                    RequestHeader::get_cluster_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RequestHeader>(
                    "RequestHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RequestHeader {
    fn clear(&mut self) {
        self.clear_uuid();
        self.clear_cluster_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestHeader {
    fn eq(&self, other: &RequestHeader) -> bool {
        self.uuid == other.uuid &&
        self.cluster_id == other.cluster_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RequestHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ResponseHeader {
    // message fields
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    cluster_id: ::std::option::Option<u64>,
    error: ::protobuf::SingularPtrField<Error>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ResponseHeader {}

impl ResponseHeader {
    pub fn new() -> ResponseHeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ResponseHeader {
        static mut instance: ::protobuf::lazy::Lazy<ResponseHeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ResponseHeader,
        };
        unsafe {
            instance.get(|| {
                ResponseHeader {
                    uuid: ::protobuf::SingularField::none(),
                    cluster_id: ::std::option::Option::None,
                    error: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes uuid = 1;

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

    // optional uint64 cluster_id = 2;

    pub fn clear_cluster_id(&mut self) {
        self.cluster_id = ::std::option::Option::None;
    }

    pub fn has_cluster_id(&self) -> bool {
        self.cluster_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_id(&mut self, v: u64) {
        self.cluster_id = ::std::option::Option::Some(v);
    }

    pub fn get_cluster_id(&self) -> u64 {
        self.cluster_id.unwrap_or(0)
    }

    // optional .pdpb.Error error = 3;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: Error) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut Error {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(|| Error::new())
    }

    pub fn get_error(&self) -> &Error {
        self.error.as_ref().unwrap_or_else(|| Error::default_instance())
    }
}

impl ::protobuf::Message for ResponseHeader {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.uuid));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.cluster_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.uuid {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.cluster_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.error {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uuid.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.cluster_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.error.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<ResponseHeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ResponseHeader {
    fn new() -> ResponseHeader {
        ResponseHeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<ResponseHeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "uuid",
                    ResponseHeader::has_uuid,
                    ResponseHeader::get_uuid,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "cluster_id",
                    ResponseHeader::has_cluster_id,
                    ResponseHeader::get_cluster_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    ResponseHeader::has_error,
                    ResponseHeader::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ResponseHeader>(
                    "ResponseHeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ResponseHeader {
    fn clear(&mut self) {
        self.clear_uuid();
        self.clear_cluster_id();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ResponseHeader {
    fn eq(&self, other: &ResponseHeader) -> bool {
        self.uuid == other.uuid &&
        self.cluster_id == other.cluster_id &&
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ResponseHeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    header: ::protobuf::SingularPtrField<RequestHeader>,
    cmd_type: ::std::option::Option<CommandType>,
    tso: ::protobuf::SingularPtrField<TsoRequest>,
    bootstrap: ::protobuf::SingularPtrField<BootstrapRequest>,
    is_bootstrapped: ::protobuf::SingularPtrField<IsBootstrappedRequest>,
    alloc_id: ::protobuf::SingularPtrField<AllocIdRequest>,
    get_store: ::protobuf::SingularPtrField<GetStoreRequest>,
    put_store: ::protobuf::SingularPtrField<PutStoreRequest>,
    ask_split: ::protobuf::SingularPtrField<AskSplitRequest>,
    get_region: ::protobuf::SingularPtrField<GetRegionRequest>,
    region_heartbeat: ::protobuf::SingularPtrField<RegionHeartbeatRequest>,
    get_cluster_config: ::protobuf::SingularPtrField<GetClusterConfigRequest>,
    put_cluster_config: ::protobuf::SingularPtrField<PutClusterConfigRequest>,
    store_heartbeat: ::protobuf::SingularPtrField<StoreHeartbeatRequest>,
    report_split: ::protobuf::SingularPtrField<ReportSplitRequest>,
    get_region_by_id: ::protobuf::SingularPtrField<GetRegionByIDRequest>,
    get_pd_members: ::protobuf::SingularPtrField<GetPDMembersRequest>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Request {
                    header: ::protobuf::SingularPtrField::none(),
                    cmd_type: ::std::option::Option::None,
                    tso: ::protobuf::SingularPtrField::none(),
                    bootstrap: ::protobuf::SingularPtrField::none(),
                    is_bootstrapped: ::protobuf::SingularPtrField::none(),
                    alloc_id: ::protobuf::SingularPtrField::none(),
                    get_store: ::protobuf::SingularPtrField::none(),
                    put_store: ::protobuf::SingularPtrField::none(),
                    ask_split: ::protobuf::SingularPtrField::none(),
                    get_region: ::protobuf::SingularPtrField::none(),
                    region_heartbeat: ::protobuf::SingularPtrField::none(),
                    get_cluster_config: ::protobuf::SingularPtrField::none(),
                    put_cluster_config: ::protobuf::SingularPtrField::none(),
                    store_heartbeat: ::protobuf::SingularPtrField::none(),
                    report_split: ::protobuf::SingularPtrField::none(),
                    get_region_by_id: ::protobuf::SingularPtrField::none(),
                    get_pd_members: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .pdpb.RequestHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header(&self) -> &RequestHeader {
        self.header.as_ref().unwrap_or_else(|| RequestHeader::default_instance())
    }

    // optional .pdpb.CommandType cmd_type = 2;

    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = ::std::option::Option::None;
    }

    pub fn has_cmd_type(&self) -> bool {
        self.cmd_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_type(&mut self, v: CommandType) {
        self.cmd_type = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_type(&self) -> CommandType {
        self.cmd_type.unwrap_or(CommandType::Invalid)
    }

    // optional .pdpb.TsoRequest tso = 3;

    pub fn clear_tso(&mut self) {
        self.tso.clear();
    }

    pub fn has_tso(&self) -> bool {
        self.tso.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tso(&mut self, v: TsoRequest) {
        self.tso = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tso(&mut self) -> &mut TsoRequest {
        if self.tso.is_none() {
            self.tso.set_default();
        };
        self.tso.as_mut().unwrap()
    }

    // Take field
    pub fn take_tso(&mut self) -> TsoRequest {
        self.tso.take().unwrap_or_else(|| TsoRequest::new())
    }

    pub fn get_tso(&self) -> &TsoRequest {
        self.tso.as_ref().unwrap_or_else(|| TsoRequest::default_instance())
    }

    // optional .pdpb.BootstrapRequest bootstrap = 4;

    pub fn clear_bootstrap(&mut self) {
        self.bootstrap.clear();
    }

    pub fn has_bootstrap(&self) -> bool {
        self.bootstrap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bootstrap(&mut self, v: BootstrapRequest) {
        self.bootstrap = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bootstrap(&mut self) -> &mut BootstrapRequest {
        if self.bootstrap.is_none() {
            self.bootstrap.set_default();
        };
        self.bootstrap.as_mut().unwrap()
    }

    // Take field
    pub fn take_bootstrap(&mut self) -> BootstrapRequest {
        self.bootstrap.take().unwrap_or_else(|| BootstrapRequest::new())
    }

    pub fn get_bootstrap(&self) -> &BootstrapRequest {
        self.bootstrap.as_ref().unwrap_or_else(|| BootstrapRequest::default_instance())
    }

    // optional .pdpb.IsBootstrappedRequest is_bootstrapped = 5;

    pub fn clear_is_bootstrapped(&mut self) {
        self.is_bootstrapped.clear();
    }

    pub fn has_is_bootstrapped(&self) -> bool {
        self.is_bootstrapped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_bootstrapped(&mut self, v: IsBootstrappedRequest) {
        self.is_bootstrapped = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_is_bootstrapped(&mut self) -> &mut IsBootstrappedRequest {
        if self.is_bootstrapped.is_none() {
            self.is_bootstrapped.set_default();
        };
        self.is_bootstrapped.as_mut().unwrap()
    }

    // Take field
    pub fn take_is_bootstrapped(&mut self) -> IsBootstrappedRequest {
        self.is_bootstrapped.take().unwrap_or_else(|| IsBootstrappedRequest::new())
    }

    pub fn get_is_bootstrapped(&self) -> &IsBootstrappedRequest {
        self.is_bootstrapped.as_ref().unwrap_or_else(|| IsBootstrappedRequest::default_instance())
    }

    // optional .pdpb.AllocIdRequest alloc_id = 6;

    pub fn clear_alloc_id(&mut self) {
        self.alloc_id.clear();
    }

    pub fn has_alloc_id(&self) -> bool {
        self.alloc_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alloc_id(&mut self, v: AllocIdRequest) {
        self.alloc_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alloc_id(&mut self) -> &mut AllocIdRequest {
        if self.alloc_id.is_none() {
            self.alloc_id.set_default();
        };
        self.alloc_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_alloc_id(&mut self) -> AllocIdRequest {
        self.alloc_id.take().unwrap_or_else(|| AllocIdRequest::new())
    }

    pub fn get_alloc_id(&self) -> &AllocIdRequest {
        self.alloc_id.as_ref().unwrap_or_else(|| AllocIdRequest::default_instance())
    }

    // optional .pdpb.GetStoreRequest get_store = 7;

    pub fn clear_get_store(&mut self) {
        self.get_store.clear();
    }

    pub fn has_get_store(&self) -> bool {
        self.get_store.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_store(&mut self, v: GetStoreRequest) {
        self.get_store = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_store(&mut self) -> &mut GetStoreRequest {
        if self.get_store.is_none() {
            self.get_store.set_default();
        };
        self.get_store.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_store(&mut self) -> GetStoreRequest {
        self.get_store.take().unwrap_or_else(|| GetStoreRequest::new())
    }

    pub fn get_get_store(&self) -> &GetStoreRequest {
        self.get_store.as_ref().unwrap_or_else(|| GetStoreRequest::default_instance())
    }

    // optional .pdpb.PutStoreRequest put_store = 8;

    pub fn clear_put_store(&mut self) {
        self.put_store.clear();
    }

    pub fn has_put_store(&self) -> bool {
        self.put_store.is_some()
    }

    // Param is passed by value, moved
    pub fn set_put_store(&mut self, v: PutStoreRequest) {
        self.put_store = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put_store(&mut self) -> &mut PutStoreRequest {
        if self.put_store.is_none() {
            self.put_store.set_default();
        };
        self.put_store.as_mut().unwrap()
    }

    // Take field
    pub fn take_put_store(&mut self) -> PutStoreRequest {
        self.put_store.take().unwrap_or_else(|| PutStoreRequest::new())
    }

    pub fn get_put_store(&self) -> &PutStoreRequest {
        self.put_store.as_ref().unwrap_or_else(|| PutStoreRequest::default_instance())
    }

    // optional .pdpb.AskSplitRequest ask_split = 9;

    pub fn clear_ask_split(&mut self) {
        self.ask_split.clear();
    }

    pub fn has_ask_split(&self) -> bool {
        self.ask_split.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ask_split(&mut self, v: AskSplitRequest) {
        self.ask_split = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ask_split(&mut self) -> &mut AskSplitRequest {
        if self.ask_split.is_none() {
            self.ask_split.set_default();
        };
        self.ask_split.as_mut().unwrap()
    }

    // Take field
    pub fn take_ask_split(&mut self) -> AskSplitRequest {
        self.ask_split.take().unwrap_or_else(|| AskSplitRequest::new())
    }

    pub fn get_ask_split(&self) -> &AskSplitRequest {
        self.ask_split.as_ref().unwrap_or_else(|| AskSplitRequest::default_instance())
    }

    // optional .pdpb.GetRegionRequest get_region = 10;

    pub fn clear_get_region(&mut self) {
        self.get_region.clear();
    }

    pub fn has_get_region(&self) -> bool {
        self.get_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_region(&mut self, v: GetRegionRequest) {
        self.get_region = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_region(&mut self) -> &mut GetRegionRequest {
        if self.get_region.is_none() {
            self.get_region.set_default();
        };
        self.get_region.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_region(&mut self) -> GetRegionRequest {
        self.get_region.take().unwrap_or_else(|| GetRegionRequest::new())
    }

    pub fn get_get_region(&self) -> &GetRegionRequest {
        self.get_region.as_ref().unwrap_or_else(|| GetRegionRequest::default_instance())
    }

    // optional .pdpb.RegionHeartbeatRequest region_heartbeat = 11;

    pub fn clear_region_heartbeat(&mut self) {
        self.region_heartbeat.clear();
    }

    pub fn has_region_heartbeat(&self) -> bool {
        self.region_heartbeat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_heartbeat(&mut self, v: RegionHeartbeatRequest) {
        self.region_heartbeat = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_heartbeat(&mut self) -> &mut RegionHeartbeatRequest {
        if self.region_heartbeat.is_none() {
            self.region_heartbeat.set_default();
        };
        self.region_heartbeat.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_heartbeat(&mut self) -> RegionHeartbeatRequest {
        self.region_heartbeat.take().unwrap_or_else(|| RegionHeartbeatRequest::new())
    }

    pub fn get_region_heartbeat(&self) -> &RegionHeartbeatRequest {
        self.region_heartbeat.as_ref().unwrap_or_else(|| RegionHeartbeatRequest::default_instance())
    }

    // optional .pdpb.GetClusterConfigRequest get_cluster_config = 12;

    pub fn clear_get_cluster_config(&mut self) {
        self.get_cluster_config.clear();
    }

    pub fn has_get_cluster_config(&self) -> bool {
        self.get_cluster_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_cluster_config(&mut self, v: GetClusterConfigRequest) {
        self.get_cluster_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_cluster_config(&mut self) -> &mut GetClusterConfigRequest {
        if self.get_cluster_config.is_none() {
            self.get_cluster_config.set_default();
        };
        self.get_cluster_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_cluster_config(&mut self) -> GetClusterConfigRequest {
        self.get_cluster_config.take().unwrap_or_else(|| GetClusterConfigRequest::new())
    }

    pub fn get_get_cluster_config(&self) -> &GetClusterConfigRequest {
        self.get_cluster_config.as_ref().unwrap_or_else(|| GetClusterConfigRequest::default_instance())
    }

    // optional .pdpb.PutClusterConfigRequest put_cluster_config = 13;

    pub fn clear_put_cluster_config(&mut self) {
        self.put_cluster_config.clear();
    }

    pub fn has_put_cluster_config(&self) -> bool {
        self.put_cluster_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_put_cluster_config(&mut self, v: PutClusterConfigRequest) {
        self.put_cluster_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put_cluster_config(&mut self) -> &mut PutClusterConfigRequest {
        if self.put_cluster_config.is_none() {
            self.put_cluster_config.set_default();
        };
        self.put_cluster_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_put_cluster_config(&mut self) -> PutClusterConfigRequest {
        self.put_cluster_config.take().unwrap_or_else(|| PutClusterConfigRequest::new())
    }

    pub fn get_put_cluster_config(&self) -> &PutClusterConfigRequest {
        self.put_cluster_config.as_ref().unwrap_or_else(|| PutClusterConfigRequest::default_instance())
    }

    // optional .pdpb.StoreHeartbeatRequest store_heartbeat = 14;

    pub fn clear_store_heartbeat(&mut self) {
        self.store_heartbeat.clear();
    }

    pub fn has_store_heartbeat(&self) -> bool {
        self.store_heartbeat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_store_heartbeat(&mut self, v: StoreHeartbeatRequest) {
        self.store_heartbeat = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_store_heartbeat(&mut self) -> &mut StoreHeartbeatRequest {
        if self.store_heartbeat.is_none() {
            self.store_heartbeat.set_default();
        };
        self.store_heartbeat.as_mut().unwrap()
    }

    // Take field
    pub fn take_store_heartbeat(&mut self) -> StoreHeartbeatRequest {
        self.store_heartbeat.take().unwrap_or_else(|| StoreHeartbeatRequest::new())
    }

    pub fn get_store_heartbeat(&self) -> &StoreHeartbeatRequest {
        self.store_heartbeat.as_ref().unwrap_or_else(|| StoreHeartbeatRequest::default_instance())
    }

    // optional .pdpb.ReportSplitRequest report_split = 15;

    pub fn clear_report_split(&mut self) {
        self.report_split.clear();
    }

    pub fn has_report_split(&self) -> bool {
        self.report_split.is_some()
    }

    // Param is passed by value, moved
    pub fn set_report_split(&mut self, v: ReportSplitRequest) {
        self.report_split = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_report_split(&mut self) -> &mut ReportSplitRequest {
        if self.report_split.is_none() {
            self.report_split.set_default();
        };
        self.report_split.as_mut().unwrap()
    }

    // Take field
    pub fn take_report_split(&mut self) -> ReportSplitRequest {
        self.report_split.take().unwrap_or_else(|| ReportSplitRequest::new())
    }

    pub fn get_report_split(&self) -> &ReportSplitRequest {
        self.report_split.as_ref().unwrap_or_else(|| ReportSplitRequest::default_instance())
    }

    // optional .pdpb.GetRegionByIDRequest get_region_by_id = 16;

    pub fn clear_get_region_by_id(&mut self) {
        self.get_region_by_id.clear();
    }

    pub fn has_get_region_by_id(&self) -> bool {
        self.get_region_by_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_region_by_id(&mut self, v: GetRegionByIDRequest) {
        self.get_region_by_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_region_by_id(&mut self) -> &mut GetRegionByIDRequest {
        if self.get_region_by_id.is_none() {
            self.get_region_by_id.set_default();
        };
        self.get_region_by_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_region_by_id(&mut self) -> GetRegionByIDRequest {
        self.get_region_by_id.take().unwrap_or_else(|| GetRegionByIDRequest::new())
    }

    pub fn get_get_region_by_id(&self) -> &GetRegionByIDRequest {
        self.get_region_by_id.as_ref().unwrap_or_else(|| GetRegionByIDRequest::default_instance())
    }

    // optional .pdpb.GetPDMembersRequest get_pd_members = 17;

    pub fn clear_get_pd_members(&mut self) {
        self.get_pd_members.clear();
    }

    pub fn has_get_pd_members(&self) -> bool {
        self.get_pd_members.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_pd_members(&mut self, v: GetPDMembersRequest) {
        self.get_pd_members = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_pd_members(&mut self) -> &mut GetPDMembersRequest {
        if self.get_pd_members.is_none() {
            self.get_pd_members.set_default();
        };
        self.get_pd_members.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_pd_members(&mut self) -> GetPDMembersRequest {
        self.get_pd_members.take().unwrap_or_else(|| GetPDMembersRequest::new())
    }

    pub fn get_get_pd_members(&self) -> &GetPDMembersRequest {
        self.get_pd_members.as_ref().unwrap_or_else(|| GetPDMembersRequest::default_instance())
    }
}

impl ::protobuf::Message for Request {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.cmd_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tso));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.bootstrap));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.is_bootstrapped));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.alloc_id));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_store));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put_store));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ask_split));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_region));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_heartbeat));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_cluster_config));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put_cluster_config));
                },
                14 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.store_heartbeat));
                },
                15 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.report_split));
                },
                16 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_region_by_id));
                },
                17 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_pd_members));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.header {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.cmd_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.tso {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.bootstrap {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.is_bootstrapped {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.alloc_id {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_store {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.put_store {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.ask_split {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_region {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.region_heartbeat {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_cluster_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.put_cluster_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.store_heartbeat {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.report_split {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_region_by_id {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_pd_members {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.tso.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.bootstrap.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.is_bootstrapped.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.alloc_id.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_store.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.put_store.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ask_split.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_region.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.region_heartbeat.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_cluster_config.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.put_cluster_config.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.store_heartbeat.as_ref() {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.report_split.as_ref() {
            try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_region_by_id.as_ref() {
            try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_pd_members.as_ref() {
            try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Request>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    Request::has_header,
                    Request::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "cmd_type",
                    Request::has_cmd_type,
                    Request::get_cmd_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "tso",
                    Request::has_tso,
                    Request::get_tso,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "bootstrap",
                    Request::has_bootstrap,
                    Request::get_bootstrap,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "is_bootstrapped",
                    Request::has_is_bootstrapped,
                    Request::get_is_bootstrapped,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "alloc_id",
                    Request::has_alloc_id,
                    Request::get_alloc_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_store",
                    Request::has_get_store,
                    Request::get_get_store,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put_store",
                    Request::has_put_store,
                    Request::get_put_store,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ask_split",
                    Request::has_ask_split,
                    Request::get_ask_split,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_region",
                    Request::has_get_region,
                    Request::get_get_region,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region_heartbeat",
                    Request::has_region_heartbeat,
                    Request::get_region_heartbeat,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_cluster_config",
                    Request::has_get_cluster_config,
                    Request::get_get_cluster_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put_cluster_config",
                    Request::has_put_cluster_config,
                    Request::get_put_cluster_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "store_heartbeat",
                    Request::has_store_heartbeat,
                    Request::get_store_heartbeat,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "report_split",
                    Request::has_report_split,
                    Request::get_report_split,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_region_by_id",
                    Request::has_get_region_by_id,
                    Request::get_get_region_by_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_pd_members",
                    Request::has_get_pd_members,
                    Request::get_get_pd_members,
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
        self.clear_header();
        self.clear_cmd_type();
        self.clear_tso();
        self.clear_bootstrap();
        self.clear_is_bootstrapped();
        self.clear_alloc_id();
        self.clear_get_store();
        self.clear_put_store();
        self.clear_ask_split();
        self.clear_get_region();
        self.clear_region_heartbeat();
        self.clear_get_cluster_config();
        self.clear_put_cluster_config();
        self.clear_store_heartbeat();
        self.clear_report_split();
        self.clear_get_region_by_id();
        self.clear_get_pd_members();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.header == other.header &&
        self.cmd_type == other.cmd_type &&
        self.tso == other.tso &&
        self.bootstrap == other.bootstrap &&
        self.is_bootstrapped == other.is_bootstrapped &&
        self.alloc_id == other.alloc_id &&
        self.get_store == other.get_store &&
        self.put_store == other.put_store &&
        self.ask_split == other.ask_split &&
        self.get_region == other.get_region &&
        self.region_heartbeat == other.region_heartbeat &&
        self.get_cluster_config == other.get_cluster_config &&
        self.put_cluster_config == other.put_cluster_config &&
        self.store_heartbeat == other.store_heartbeat &&
        self.report_split == other.report_split &&
        self.get_region_by_id == other.get_region_by_id &&
        self.get_pd_members == other.get_pd_members &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Response {
    // message fields
    header: ::protobuf::SingularPtrField<ResponseHeader>,
    cmd_type: ::std::option::Option<CommandType>,
    tso: ::protobuf::SingularPtrField<TsoResponse>,
    bootstrap: ::protobuf::SingularPtrField<BootstrapResponse>,
    is_bootstrapped: ::protobuf::SingularPtrField<IsBootstrappedResponse>,
    alloc_id: ::protobuf::SingularPtrField<AllocIdResponse>,
    get_store: ::protobuf::SingularPtrField<GetStoreResponse>,
    put_store: ::protobuf::SingularPtrField<PutStoreResponse>,
    ask_split: ::protobuf::SingularPtrField<AskSplitResponse>,
    get_region: ::protobuf::SingularPtrField<GetRegionResponse>,
    region_heartbeat: ::protobuf::SingularPtrField<RegionHeartbeatResponse>,
    get_cluster_config: ::protobuf::SingularPtrField<GetClusterConfigResponse>,
    put_cluster_config: ::protobuf::SingularPtrField<PutClusterConfigResponse>,
    store_heartbeat: ::protobuf::SingularPtrField<StoreHeartbeatResponse>,
    report_split: ::protobuf::SingularPtrField<ReportSplitResponse>,
    get_region_by_id: ::protobuf::SingularPtrField<GetRegionResponse>,
    get_pd_members: ::protobuf::SingularPtrField<GetPDMembersResponse>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
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
            instance.get(|| {
                Response {
                    header: ::protobuf::SingularPtrField::none(),
                    cmd_type: ::std::option::Option::None,
                    tso: ::protobuf::SingularPtrField::none(),
                    bootstrap: ::protobuf::SingularPtrField::none(),
                    is_bootstrapped: ::protobuf::SingularPtrField::none(),
                    alloc_id: ::protobuf::SingularPtrField::none(),
                    get_store: ::protobuf::SingularPtrField::none(),
                    put_store: ::protobuf::SingularPtrField::none(),
                    ask_split: ::protobuf::SingularPtrField::none(),
                    get_region: ::protobuf::SingularPtrField::none(),
                    region_heartbeat: ::protobuf::SingularPtrField::none(),
                    get_cluster_config: ::protobuf::SingularPtrField::none(),
                    put_cluster_config: ::protobuf::SingularPtrField::none(),
                    store_heartbeat: ::protobuf::SingularPtrField::none(),
                    report_split: ::protobuf::SingularPtrField::none(),
                    get_region_by_id: ::protobuf::SingularPtrField::none(),
                    get_pd_members: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .pdpb.ResponseHeader header = 1;

    pub fn clear_header(&mut self) {
        self.header.clear();
    }

    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }

    // Param is passed by value, moved
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header(&self) -> &ResponseHeader {
        self.header.as_ref().unwrap_or_else(|| ResponseHeader::default_instance())
    }

    // optional .pdpb.CommandType cmd_type = 2;

    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = ::std::option::Option::None;
    }

    pub fn has_cmd_type(&self) -> bool {
        self.cmd_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_type(&mut self, v: CommandType) {
        self.cmd_type = ::std::option::Option::Some(v);
    }

    pub fn get_cmd_type(&self) -> CommandType {
        self.cmd_type.unwrap_or(CommandType::Invalid)
    }

    // optional .pdpb.TsoResponse tso = 3;

    pub fn clear_tso(&mut self) {
        self.tso.clear();
    }

    pub fn has_tso(&self) -> bool {
        self.tso.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tso(&mut self, v: TsoResponse) {
        self.tso = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tso(&mut self) -> &mut TsoResponse {
        if self.tso.is_none() {
            self.tso.set_default();
        };
        self.tso.as_mut().unwrap()
    }

    // Take field
    pub fn take_tso(&mut self) -> TsoResponse {
        self.tso.take().unwrap_or_else(|| TsoResponse::new())
    }

    pub fn get_tso(&self) -> &TsoResponse {
        self.tso.as_ref().unwrap_or_else(|| TsoResponse::default_instance())
    }

    // optional .pdpb.BootstrapResponse bootstrap = 4;

    pub fn clear_bootstrap(&mut self) {
        self.bootstrap.clear();
    }

    pub fn has_bootstrap(&self) -> bool {
        self.bootstrap.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bootstrap(&mut self, v: BootstrapResponse) {
        self.bootstrap = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bootstrap(&mut self) -> &mut BootstrapResponse {
        if self.bootstrap.is_none() {
            self.bootstrap.set_default();
        };
        self.bootstrap.as_mut().unwrap()
    }

    // Take field
    pub fn take_bootstrap(&mut self) -> BootstrapResponse {
        self.bootstrap.take().unwrap_or_else(|| BootstrapResponse::new())
    }

    pub fn get_bootstrap(&self) -> &BootstrapResponse {
        self.bootstrap.as_ref().unwrap_or_else(|| BootstrapResponse::default_instance())
    }

    // optional .pdpb.IsBootstrappedResponse is_bootstrapped = 5;

    pub fn clear_is_bootstrapped(&mut self) {
        self.is_bootstrapped.clear();
    }

    pub fn has_is_bootstrapped(&self) -> bool {
        self.is_bootstrapped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_bootstrapped(&mut self, v: IsBootstrappedResponse) {
        self.is_bootstrapped = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_is_bootstrapped(&mut self) -> &mut IsBootstrappedResponse {
        if self.is_bootstrapped.is_none() {
            self.is_bootstrapped.set_default();
        };
        self.is_bootstrapped.as_mut().unwrap()
    }

    // Take field
    pub fn take_is_bootstrapped(&mut self) -> IsBootstrappedResponse {
        self.is_bootstrapped.take().unwrap_or_else(|| IsBootstrappedResponse::new())
    }

    pub fn get_is_bootstrapped(&self) -> &IsBootstrappedResponse {
        self.is_bootstrapped.as_ref().unwrap_or_else(|| IsBootstrappedResponse::default_instance())
    }

    // optional .pdpb.AllocIdResponse alloc_id = 6;

    pub fn clear_alloc_id(&mut self) {
        self.alloc_id.clear();
    }

    pub fn has_alloc_id(&self) -> bool {
        self.alloc_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_alloc_id(&mut self, v: AllocIdResponse) {
        self.alloc_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_alloc_id(&mut self) -> &mut AllocIdResponse {
        if self.alloc_id.is_none() {
            self.alloc_id.set_default();
        };
        self.alloc_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_alloc_id(&mut self) -> AllocIdResponse {
        self.alloc_id.take().unwrap_or_else(|| AllocIdResponse::new())
    }

    pub fn get_alloc_id(&self) -> &AllocIdResponse {
        self.alloc_id.as_ref().unwrap_or_else(|| AllocIdResponse::default_instance())
    }

    // optional .pdpb.GetStoreResponse get_store = 7;

    pub fn clear_get_store(&mut self) {
        self.get_store.clear();
    }

    pub fn has_get_store(&self) -> bool {
        self.get_store.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_store(&mut self, v: GetStoreResponse) {
        self.get_store = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_store(&mut self) -> &mut GetStoreResponse {
        if self.get_store.is_none() {
            self.get_store.set_default();
        };
        self.get_store.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_store(&mut self) -> GetStoreResponse {
        self.get_store.take().unwrap_or_else(|| GetStoreResponse::new())
    }

    pub fn get_get_store(&self) -> &GetStoreResponse {
        self.get_store.as_ref().unwrap_or_else(|| GetStoreResponse::default_instance())
    }

    // optional .pdpb.PutStoreResponse put_store = 8;

    pub fn clear_put_store(&mut self) {
        self.put_store.clear();
    }

    pub fn has_put_store(&self) -> bool {
        self.put_store.is_some()
    }

    // Param is passed by value, moved
    pub fn set_put_store(&mut self, v: PutStoreResponse) {
        self.put_store = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put_store(&mut self) -> &mut PutStoreResponse {
        if self.put_store.is_none() {
            self.put_store.set_default();
        };
        self.put_store.as_mut().unwrap()
    }

    // Take field
    pub fn take_put_store(&mut self) -> PutStoreResponse {
        self.put_store.take().unwrap_or_else(|| PutStoreResponse::new())
    }

    pub fn get_put_store(&self) -> &PutStoreResponse {
        self.put_store.as_ref().unwrap_or_else(|| PutStoreResponse::default_instance())
    }

    // optional .pdpb.AskSplitResponse ask_split = 9;

    pub fn clear_ask_split(&mut self) {
        self.ask_split.clear();
    }

    pub fn has_ask_split(&self) -> bool {
        self.ask_split.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ask_split(&mut self, v: AskSplitResponse) {
        self.ask_split = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ask_split(&mut self) -> &mut AskSplitResponse {
        if self.ask_split.is_none() {
            self.ask_split.set_default();
        };
        self.ask_split.as_mut().unwrap()
    }

    // Take field
    pub fn take_ask_split(&mut self) -> AskSplitResponse {
        self.ask_split.take().unwrap_or_else(|| AskSplitResponse::new())
    }

    pub fn get_ask_split(&self) -> &AskSplitResponse {
        self.ask_split.as_ref().unwrap_or_else(|| AskSplitResponse::default_instance())
    }

    // optional .pdpb.GetRegionResponse get_region = 10;

    pub fn clear_get_region(&mut self) {
        self.get_region.clear();
    }

    pub fn has_get_region(&self) -> bool {
        self.get_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_region(&mut self, v: GetRegionResponse) {
        self.get_region = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_region(&mut self) -> &mut GetRegionResponse {
        if self.get_region.is_none() {
            self.get_region.set_default();
        };
        self.get_region.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_region(&mut self) -> GetRegionResponse {
        self.get_region.take().unwrap_or_else(|| GetRegionResponse::new())
    }

    pub fn get_get_region(&self) -> &GetRegionResponse {
        self.get_region.as_ref().unwrap_or_else(|| GetRegionResponse::default_instance())
    }

    // optional .pdpb.RegionHeartbeatResponse region_heartbeat = 11;

    pub fn clear_region_heartbeat(&mut self) {
        self.region_heartbeat.clear();
    }

    pub fn has_region_heartbeat(&self) -> bool {
        self.region_heartbeat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_heartbeat(&mut self, v: RegionHeartbeatResponse) {
        self.region_heartbeat = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_heartbeat(&mut self) -> &mut RegionHeartbeatResponse {
        if self.region_heartbeat.is_none() {
            self.region_heartbeat.set_default();
        };
        self.region_heartbeat.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_heartbeat(&mut self) -> RegionHeartbeatResponse {
        self.region_heartbeat.take().unwrap_or_else(|| RegionHeartbeatResponse::new())
    }

    pub fn get_region_heartbeat(&self) -> &RegionHeartbeatResponse {
        self.region_heartbeat.as_ref().unwrap_or_else(|| RegionHeartbeatResponse::default_instance())
    }

    // optional .pdpb.GetClusterConfigResponse get_cluster_config = 12;

    pub fn clear_get_cluster_config(&mut self) {
        self.get_cluster_config.clear();
    }

    pub fn has_get_cluster_config(&self) -> bool {
        self.get_cluster_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_cluster_config(&mut self, v: GetClusterConfigResponse) {
        self.get_cluster_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_cluster_config(&mut self) -> &mut GetClusterConfigResponse {
        if self.get_cluster_config.is_none() {
            self.get_cluster_config.set_default();
        };
        self.get_cluster_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_cluster_config(&mut self) -> GetClusterConfigResponse {
        self.get_cluster_config.take().unwrap_or_else(|| GetClusterConfigResponse::new())
    }

    pub fn get_get_cluster_config(&self) -> &GetClusterConfigResponse {
        self.get_cluster_config.as_ref().unwrap_or_else(|| GetClusterConfigResponse::default_instance())
    }

    // optional .pdpb.PutClusterConfigResponse put_cluster_config = 13;

    pub fn clear_put_cluster_config(&mut self) {
        self.put_cluster_config.clear();
    }

    pub fn has_put_cluster_config(&self) -> bool {
        self.put_cluster_config.is_some()
    }

    // Param is passed by value, moved
    pub fn set_put_cluster_config(&mut self, v: PutClusterConfigResponse) {
        self.put_cluster_config = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_put_cluster_config(&mut self) -> &mut PutClusterConfigResponse {
        if self.put_cluster_config.is_none() {
            self.put_cluster_config.set_default();
        };
        self.put_cluster_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_put_cluster_config(&mut self) -> PutClusterConfigResponse {
        self.put_cluster_config.take().unwrap_or_else(|| PutClusterConfigResponse::new())
    }

    pub fn get_put_cluster_config(&self) -> &PutClusterConfigResponse {
        self.put_cluster_config.as_ref().unwrap_or_else(|| PutClusterConfigResponse::default_instance())
    }

    // optional .pdpb.StoreHeartbeatResponse store_heartbeat = 14;

    pub fn clear_store_heartbeat(&mut self) {
        self.store_heartbeat.clear();
    }

    pub fn has_store_heartbeat(&self) -> bool {
        self.store_heartbeat.is_some()
    }

    // Param is passed by value, moved
    pub fn set_store_heartbeat(&mut self, v: StoreHeartbeatResponse) {
        self.store_heartbeat = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_store_heartbeat(&mut self) -> &mut StoreHeartbeatResponse {
        if self.store_heartbeat.is_none() {
            self.store_heartbeat.set_default();
        };
        self.store_heartbeat.as_mut().unwrap()
    }

    // Take field
    pub fn take_store_heartbeat(&mut self) -> StoreHeartbeatResponse {
        self.store_heartbeat.take().unwrap_or_else(|| StoreHeartbeatResponse::new())
    }

    pub fn get_store_heartbeat(&self) -> &StoreHeartbeatResponse {
        self.store_heartbeat.as_ref().unwrap_or_else(|| StoreHeartbeatResponse::default_instance())
    }

    // optional .pdpb.ReportSplitResponse report_split = 15;

    pub fn clear_report_split(&mut self) {
        self.report_split.clear();
    }

    pub fn has_report_split(&self) -> bool {
        self.report_split.is_some()
    }

    // Param is passed by value, moved
    pub fn set_report_split(&mut self, v: ReportSplitResponse) {
        self.report_split = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_report_split(&mut self) -> &mut ReportSplitResponse {
        if self.report_split.is_none() {
            self.report_split.set_default();
        };
        self.report_split.as_mut().unwrap()
    }

    // Take field
    pub fn take_report_split(&mut self) -> ReportSplitResponse {
        self.report_split.take().unwrap_or_else(|| ReportSplitResponse::new())
    }

    pub fn get_report_split(&self) -> &ReportSplitResponse {
        self.report_split.as_ref().unwrap_or_else(|| ReportSplitResponse::default_instance())
    }

    // optional .pdpb.GetRegionResponse get_region_by_id = 16;

    pub fn clear_get_region_by_id(&mut self) {
        self.get_region_by_id.clear();
    }

    pub fn has_get_region_by_id(&self) -> bool {
        self.get_region_by_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_region_by_id(&mut self, v: GetRegionResponse) {
        self.get_region_by_id = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_region_by_id(&mut self) -> &mut GetRegionResponse {
        if self.get_region_by_id.is_none() {
            self.get_region_by_id.set_default();
        };
        self.get_region_by_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_region_by_id(&mut self) -> GetRegionResponse {
        self.get_region_by_id.take().unwrap_or_else(|| GetRegionResponse::new())
    }

    pub fn get_get_region_by_id(&self) -> &GetRegionResponse {
        self.get_region_by_id.as_ref().unwrap_or_else(|| GetRegionResponse::default_instance())
    }

    // optional .pdpb.GetPDMembersResponse get_pd_members = 17;

    pub fn clear_get_pd_members(&mut self) {
        self.get_pd_members.clear();
    }

    pub fn has_get_pd_members(&self) -> bool {
        self.get_pd_members.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_pd_members(&mut self, v: GetPDMembersResponse) {
        self.get_pd_members = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_get_pd_members(&mut self) -> &mut GetPDMembersResponse {
        if self.get_pd_members.is_none() {
            self.get_pd_members.set_default();
        };
        self.get_pd_members.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_pd_members(&mut self) -> GetPDMembersResponse {
        self.get_pd_members.take().unwrap_or_else(|| GetPDMembersResponse::new())
    }

    pub fn get_get_pd_members(&self) -> &GetPDMembersResponse {
        self.get_pd_members.as_ref().unwrap_or_else(|| GetPDMembersResponse::default_instance())
    }
}

impl ::protobuf::Message for Response {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.header));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.cmd_type = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.tso));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.bootstrap));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.is_bootstrapped));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.alloc_id));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_store));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put_store));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ask_split));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_region));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_heartbeat));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_cluster_config));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put_cluster_config));
                },
                14 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.store_heartbeat));
                },
                15 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.report_split));
                },
                16 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_region_by_id));
                },
                17 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_pd_members));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.header {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.cmd_type {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in &self.tso {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.bootstrap {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.is_bootstrapped {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.alloc_id {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_store {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.put_store {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.ask_split {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_region {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.region_heartbeat {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_cluster_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.put_cluster_config {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.store_heartbeat {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.report_split {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_region_by_id {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.get_pd_members {
            let len = value.compute_size();
            my_size += 2 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.header.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_type {
            try!(os.write_enum(2, v.value()));
        };
        if let Some(v) = self.tso.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.bootstrap.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.is_bootstrapped.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.alloc_id.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_store.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.put_store.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ask_split.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_region.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.region_heartbeat.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_cluster_config.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.put_cluster_config.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.store_heartbeat.as_ref() {
            try!(os.write_tag(14, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.report_split.as_ref() {
            try!(os.write_tag(15, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_region_by_id.as_ref() {
            try!(os.write_tag(16, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_pd_members.as_ref() {
            try!(os.write_tag(17, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Response>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
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
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "header",
                    Response::has_header,
                    Response::get_header,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "cmd_type",
                    Response::has_cmd_type,
                    Response::get_cmd_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "tso",
                    Response::has_tso,
                    Response::get_tso,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "bootstrap",
                    Response::has_bootstrap,
                    Response::get_bootstrap,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "is_bootstrapped",
                    Response::has_is_bootstrapped,
                    Response::get_is_bootstrapped,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "alloc_id",
                    Response::has_alloc_id,
                    Response::get_alloc_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_store",
                    Response::has_get_store,
                    Response::get_get_store,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put_store",
                    Response::has_put_store,
                    Response::get_put_store,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ask_split",
                    Response::has_ask_split,
                    Response::get_ask_split,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_region",
                    Response::has_get_region,
                    Response::get_get_region,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region_heartbeat",
                    Response::has_region_heartbeat,
                    Response::get_region_heartbeat,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_cluster_config",
                    Response::has_get_cluster_config,
                    Response::get_get_cluster_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put_cluster_config",
                    Response::has_put_cluster_config,
                    Response::get_put_cluster_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "store_heartbeat",
                    Response::has_store_heartbeat,
                    Response::get_store_heartbeat,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "report_split",
                    Response::has_report_split,
                    Response::get_report_split,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_region_by_id",
                    Response::has_get_region_by_id,
                    Response::get_get_region_by_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_pd_members",
                    Response::has_get_pd_members,
                    Response::get_get_pd_members,
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
        self.clear_header();
        self.clear_cmd_type();
        self.clear_tso();
        self.clear_bootstrap();
        self.clear_is_bootstrapped();
        self.clear_alloc_id();
        self.clear_get_store();
        self.clear_put_store();
        self.clear_ask_split();
        self.clear_get_region();
        self.clear_region_heartbeat();
        self.clear_get_cluster_config();
        self.clear_put_cluster_config();
        self.clear_store_heartbeat();
        self.clear_report_split();
        self.clear_get_region_by_id();
        self.clear_get_pd_members();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.header == other.header &&
        self.cmd_type == other.cmd_type &&
        self.tso == other.tso &&
        self.bootstrap == other.bootstrap &&
        self.is_bootstrapped == other.is_bootstrapped &&
        self.alloc_id == other.alloc_id &&
        self.get_store == other.get_store &&
        self.put_store == other.put_store &&
        self.ask_split == other.ask_split &&
        self.get_region == other.get_region &&
        self.region_heartbeat == other.region_heartbeat &&
        self.get_cluster_config == other.get_cluster_config &&
        self.put_cluster_config == other.put_cluster_config &&
        self.store_heartbeat == other.store_heartbeat &&
        self.report_split == other.report_split &&
        self.get_region_by_id == other.get_region_by_id &&
        self.get_pd_members == other.get_pd_members &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct BootstrappedError {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for BootstrappedError {}

impl BootstrappedError {
    pub fn new() -> BootstrappedError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static BootstrappedError {
        static mut instance: ::protobuf::lazy::Lazy<BootstrappedError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const BootstrappedError,
        };
        unsafe {
            instance.get(|| {
                BootstrappedError {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for BootstrappedError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<BootstrappedError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for BootstrappedError {
    fn new() -> BootstrappedError {
        BootstrappedError::new()
    }

    fn descriptor_static(_: ::std::option::Option<BootstrappedError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<BootstrappedError>(
                    "BootstrappedError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for BootstrappedError {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for BootstrappedError {
    fn eq(&self, other: &BootstrappedError) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for BootstrappedError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StoreIsTombstoneError {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StoreIsTombstoneError {}

impl StoreIsTombstoneError {
    pub fn new() -> StoreIsTombstoneError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StoreIsTombstoneError {
        static mut instance: ::protobuf::lazy::Lazy<StoreIsTombstoneError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StoreIsTombstoneError,
        };
        unsafe {
            instance.get(|| {
                StoreIsTombstoneError {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for StoreIsTombstoneError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
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
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<StoreIsTombstoneError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StoreIsTombstoneError {
    fn new() -> StoreIsTombstoneError {
        StoreIsTombstoneError::new()
    }

    fn descriptor_static(_: ::std::option::Option<StoreIsTombstoneError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<StoreIsTombstoneError>(
                    "StoreIsTombstoneError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StoreIsTombstoneError {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StoreIsTombstoneError {
    fn eq(&self, other: &StoreIsTombstoneError) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StoreIsTombstoneError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Error {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    bootstrapped: ::protobuf::SingularPtrField<BootstrappedError>,
    is_tombstone: ::protobuf::SingularPtrField<StoreIsTombstoneError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Error {}

impl Error {
    pub fn new() -> Error {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Error {
        static mut instance: ::protobuf::lazy::Lazy<Error> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Error,
        };
        unsafe {
            instance.get(|| {
                Error {
                    message: ::protobuf::SingularField::none(),
                    bootstrapped: ::protobuf::SingularPtrField::none(),
                    is_tombstone: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: ::std::string::String) {
        self.message = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message(&self) -> &str {
        match self.message.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .pdpb.BootstrappedError bootstrapped = 2;

    pub fn clear_bootstrapped(&mut self) {
        self.bootstrapped.clear();
    }

    pub fn has_bootstrapped(&self) -> bool {
        self.bootstrapped.is_some()
    }

    // Param is passed by value, moved
    pub fn set_bootstrapped(&mut self, v: BootstrappedError) {
        self.bootstrapped = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_bootstrapped(&mut self) -> &mut BootstrappedError {
        if self.bootstrapped.is_none() {
            self.bootstrapped.set_default();
        };
        self.bootstrapped.as_mut().unwrap()
    }

    // Take field
    pub fn take_bootstrapped(&mut self) -> BootstrappedError {
        self.bootstrapped.take().unwrap_or_else(|| BootstrappedError::new())
    }

    pub fn get_bootstrapped(&self) -> &BootstrappedError {
        self.bootstrapped.as_ref().unwrap_or_else(|| BootstrappedError::default_instance())
    }

    // optional .pdpb.StoreIsTombstoneError is_tombstone = 3;

    pub fn clear_is_tombstone(&mut self) {
        self.is_tombstone.clear();
    }

    pub fn has_is_tombstone(&self) -> bool {
        self.is_tombstone.is_some()
    }

    // Param is passed by value, moved
    pub fn set_is_tombstone(&mut self, v: StoreIsTombstoneError) {
        self.is_tombstone = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_is_tombstone(&mut self) -> &mut StoreIsTombstoneError {
        if self.is_tombstone.is_none() {
            self.is_tombstone.set_default();
        };
        self.is_tombstone.as_mut().unwrap()
    }

    // Take field
    pub fn take_is_tombstone(&mut self) -> StoreIsTombstoneError {
        self.is_tombstone.take().unwrap_or_else(|| StoreIsTombstoneError::new())
    }

    pub fn get_is_tombstone(&self) -> &StoreIsTombstoneError {
        self.is_tombstone.as_ref().unwrap_or_else(|| StoreIsTombstoneError::default_instance())
    }
}

impl ::protobuf::Message for Error {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.message));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.bootstrapped));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.is_tombstone));
                },
                _ => {
                    try!(::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields()));
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        for value in &self.message {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in &self.bootstrapped {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.is_tombstone {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.bootstrapped.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.is_tombstone.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        try!(os.write_unknown_fields(self.get_unknown_fields()));
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

    fn type_id(&self) -> ::std::any::TypeId {
        ::std::any::TypeId::of::<Error>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Error {
    fn new() -> Error {
        Error::new()
    }

    fn descriptor_static(_: ::std::option::Option<Error>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "message",
                    Error::has_message,
                    Error::get_message,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "bootstrapped",
                    Error::has_bootstrapped,
                    Error::get_bootstrapped,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "is_tombstone",
                    Error::has_is_tombstone,
                    Error::get_is_tombstone,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Error>(
                    "Error",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Error {
    fn clear(&mut self) {
        self.clear_message();
        self.clear_bootstrapped();
        self.clear_is_tombstone();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.message == other.message &&
        self.bootstrapped == other.bootstrapped &&
        self.is_tombstone == other.is_tombstone &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum CommandType {
    Invalid = 0,
    Tso = 1,
    Bootstrap = 2,
    IsBootstrapped = 3,
    AllocId = 4,
    GetStore = 5,
    PutStore = 6,
    AskSplit = 7,
    GetRegion = 8,
    RegionHeartbeat = 9,
    GetClusterConfig = 10,
    PutClusterConfig = 11,
    StoreHeartbeat = 12,
    ReportSplit = 13,
    GetRegionByID = 14,
    GetPDMembers = 15,
}

impl ::protobuf::ProtobufEnum for CommandType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<CommandType> {
        match value {
            0 => ::std::option::Option::Some(CommandType::Invalid),
            1 => ::std::option::Option::Some(CommandType::Tso),
            2 => ::std::option::Option::Some(CommandType::Bootstrap),
            3 => ::std::option::Option::Some(CommandType::IsBootstrapped),
            4 => ::std::option::Option::Some(CommandType::AllocId),
            5 => ::std::option::Option::Some(CommandType::GetStore),
            6 => ::std::option::Option::Some(CommandType::PutStore),
            7 => ::std::option::Option::Some(CommandType::AskSplit),
            8 => ::std::option::Option::Some(CommandType::GetRegion),
            9 => ::std::option::Option::Some(CommandType::RegionHeartbeat),
            10 => ::std::option::Option::Some(CommandType::GetClusterConfig),
            11 => ::std::option::Option::Some(CommandType::PutClusterConfig),
            12 => ::std::option::Option::Some(CommandType::StoreHeartbeat),
            13 => ::std::option::Option::Some(CommandType::ReportSplit),
            14 => ::std::option::Option::Some(CommandType::GetRegionByID),
            15 => ::std::option::Option::Some(CommandType::GetPDMembers),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [CommandType] = &[
            CommandType::Invalid,
            CommandType::Tso,
            CommandType::Bootstrap,
            CommandType::IsBootstrapped,
            CommandType::AllocId,
            CommandType::GetStore,
            CommandType::PutStore,
            CommandType::AskSplit,
            CommandType::GetRegion,
            CommandType::RegionHeartbeat,
            CommandType::GetClusterConfig,
            CommandType::PutClusterConfig,
            CommandType::StoreHeartbeat,
            CommandType::ReportSplit,
            CommandType::GetRegionByID,
            CommandType::GetPDMembers,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<CommandType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("CommandType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for CommandType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0a, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x04, 0x70, 0x64,
    0x70, 0x62, 0x1a, 0x0c, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f,
    0x1a, 0x0d, 0x65, 0x72, 0x61, 0x66, 0x74, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x38, 0x0a, 0x06, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12,
    0x18, 0x0a, 0x04, 0x61, 0x64, 0x64, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x42, 0x04, 0xc8,
    0xde, 0x1f, 0x00, 0x52, 0x04, 0x61, 0x64, 0x64, 0x72, 0x12, 0x14, 0x0a, 0x02, 0x69, 0x64, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x04, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x02, 0x69, 0x64, 0x22,
    0x28, 0x0a, 0x0a, 0x54, 0x73, 0x6f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1a, 0x0a,
    0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x42, 0x04, 0xc8, 0xde,
    0x1f, 0x00, 0x52, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x22, 0x4d, 0x0a, 0x09, 0x54, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x12, 0x20, 0x0a, 0x08, 0x70, 0x68, 0x79, 0x73, 0x69, 0x63,
    0x61, 0x6c, 0x18, 0x01, 0x20, 0x01, 0x28, 0x03, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x08,
    0x70, 0x68, 0x79, 0x73, 0x69, 0x63, 0x61, 0x6c, 0x12, 0x1e, 0x0a, 0x07, 0x6c, 0x6f, 0x67, 0x69,
    0x63, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52,
    0x07, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x22, 0x5e, 0x0a, 0x0b, 0x54, 0x73, 0x6f, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1a, 0x0a, 0x05, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x05, 0x63, 0x6f,
    0x75, 0x6e, 0x74, 0x12, 0x33, 0x0a, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x54, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x09, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x22, 0x5f, 0x0a, 0x10, 0x42, 0x6f, 0x6f, 0x74,
    0x73, 0x74, 0x72, 0x61, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x23, 0x0a, 0x05,
    0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d, 0x65,
    0x74, 0x61, 0x70, 0x62, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x05, 0x73, 0x74, 0x6f, 0x72,
    0x65, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x22, 0x13, 0x0a, 0x11, 0x42, 0x6f, 0x6f,
    0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x17,
    0x0a, 0x15, 0x49, 0x73, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x3c, 0x0a, 0x16, 0x49, 0x73, 0x42, 0x6f, 0x6f,
    0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x12, 0x22, 0x0a, 0x0c, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72,
    0x61, 0x70, 0x70, 0x65, 0x64, 0x22, 0x10, 0x0a, 0x0e, 0x41, 0x6c, 0x6c, 0x6f, 0x63, 0x49, 0x64,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x27, 0x0a, 0x0f, 0x41, 0x6c, 0x6c, 0x6f, 0x63,
    0x49, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x14, 0x0a, 0x02, 0x69, 0x64,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x02, 0x69, 0x64,
    0x22, 0x32, 0x0a, 0x0f, 0x47, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x1f, 0x0a, 0x08, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x07, 0x73, 0x74, 0x6f,
    0x72, 0x65, 0x49, 0x64, 0x22, 0x37, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x23, 0x0a, 0x05, 0x73, 0x74, 0x6f, 0x72,
    0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62,
    0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x05, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x22, 0x31, 0x0a,
    0x10, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4b, 0x65, 0x79,
    0x22, 0x61, 0x0a, 0x11, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x24, 0x0a,
    0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e,
    0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x52, 0x06, 0x6c, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x22, 0x39, 0x0a, 0x14, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x42, 0x79, 0x49, 0x44, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x21, 0x0a, 0x09, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x42, 0x04,
    0xc8, 0xde, 0x1f, 0x00, 0x52, 0x08, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x22, 0x19,
    0x0a, 0x17, 0x47, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x45, 0x0a, 0x18, 0x47, 0x65, 0x74,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x29, 0x0a, 0x07, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x52, 0x07, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x22, 0x36, 0x0a, 0x0f, 0x50, 0x75, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x23, 0x0a, 0x05, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x53, 0x74, 0x6f, 0x72,
    0x65, 0x52, 0x05, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x22, 0x12, 0x0a, 0x10, 0x50, 0x75, 0x74, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x5c, 0x0a, 0x08,
    0x50, 0x44, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1f, 0x0a, 0x0b,
    0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x5f, 0x75, 0x72, 0x6c, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28,
    0x09, 0x52, 0x0a, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x55, 0x72, 0x6c, 0x73, 0x12, 0x1b, 0x0a,
    0x09, 0x70, 0x65, 0x65, 0x72, 0x5f, 0x75, 0x72, 0x6c, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x09,
    0x52, 0x08, 0x70, 0x65, 0x65, 0x72, 0x55, 0x72, 0x6c, 0x73, 0x22, 0x15, 0x0a, 0x13, 0x47, 0x65,
    0x74, 0x50, 0x44, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x22, 0x40, 0x0a, 0x14, 0x47, 0x65, 0x74, 0x50, 0x44, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x28, 0x0a, 0x07, 0x6d, 0x65, 0x6d,
    0x62, 0x65, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x70, 0x64, 0x70,
    0x62, 0x2e, 0x50, 0x44, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x52, 0x07, 0x6d, 0x65, 0x6d, 0x62,
    0x65, 0x72, 0x73, 0x22, 0x50, 0x0a, 0x09, 0x50, 0x65, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x73,
    0x12, 0x20, 0x0a, 0x04, 0x70, 0x65, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c,
    0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x52, 0x04, 0x70, 0x65,
    0x65, 0x72, 0x12, 0x21, 0x0a, 0x0c, 0x64, 0x6f, 0x77, 0x6e, 0x5f, 0x73, 0x65, 0x63, 0x6f, 0x6e,
    0x64, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x64, 0x6f, 0x77, 0x6e, 0x53, 0x65,
    0x63, 0x6f, 0x6e, 0x64, 0x73, 0x22, 0xc9, 0x01, 0x0a, 0x16, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x24, 0x0a, 0x06, 0x6c, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70,
    0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x52, 0x06, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2e,
    0x0a, 0x0a, 0x64, 0x6f, 0x77, 0x6e, 0x5f, 0x70, 0x65, 0x65, 0x72, 0x73, 0x18, 0x03, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x53, 0x74,
    0x61, 0x74, 0x73, 0x52, 0x09, 0x64, 0x6f, 0x77, 0x6e, 0x50, 0x65, 0x65, 0x72, 0x73, 0x12, 0x31,
    0x0a, 0x0d, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x5f, 0x70, 0x65, 0x65, 0x72, 0x73, 0x18,
    0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x50,
    0x65, 0x65, 0x72, 0x52, 0x0c, 0x70, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x50, 0x65, 0x65, 0x72,
    0x73, 0x22, 0x68, 0x0a, 0x0a, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x12,
    0x38, 0x0a, 0x0b, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x17, 0x2e, 0x65, 0x72, 0x61, 0x66, 0x74, 0x70, 0x62, 0x2e, 0x43,
    0x6f, 0x6e, 0x66, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x0a, 0x63,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x20, 0x0a, 0x04, 0x70, 0x65, 0x65,
    0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62,
    0x2e, 0x50, 0x65, 0x65, 0x72, 0x52, 0x04, 0x70, 0x65, 0x65, 0x72, 0x22, 0x32, 0x0a, 0x0e, 0x54,
    0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x20, 0x0a,
    0x04, 0x70, 0x65, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65,
    0x74, 0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x52, 0x04, 0x70, 0x65, 0x65, 0x72, 0x22,
    0x8b, 0x01, 0x0a, 0x17, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62,
    0x65, 0x61, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x31, 0x0a, 0x0b, 0x63,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x5f, 0x70, 0x65, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x10, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65,
    0x65, 0x72, 0x52, 0x0a, 0x63, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x12, 0x3d,
    0x0a, 0x0f, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x5f, 0x6c, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x54,
    0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x0e, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x22, 0x44, 0x0a,
    0x17, 0x50, 0x75, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x29, 0x0a, 0x07, 0x63, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x74, 0x61,
    0x70, 0x62, 0x2e, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x52, 0x07, 0x63, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x22, 0x1a, 0x0a, 0x18, 0x50, 0x75, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
    0x39, 0x0a, 0x0f, 0x41, 0x73, 0x6b, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x22, 0x5e, 0x0a, 0x10, 0x41, 0x73,
    0x6b, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x28,
    0x0a, 0x0d, 0x6e, 0x65, 0x77, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x04, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x0b, 0x6e, 0x65, 0x77,
    0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x20, 0x0a, 0x0c, 0x6e, 0x65, 0x77, 0x5f,
    0x70, 0x65, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x04, 0x52, 0x0a,
    0x6e, 0x65, 0x77, 0x50, 0x65, 0x65, 0x72, 0x49, 0x64, 0x73, 0x22, 0x82, 0x03, 0x0a, 0x0a, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x53, 0x74, 0x61, 0x74, 0x73, 0x12, 0x1f, 0x0a, 0x08, 0x73, 0x74, 0x6f,
    0x72, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x42, 0x04, 0xc8, 0xde, 0x1f,
    0x00, 0x52, 0x07, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x64, 0x12, 0x20, 0x0a, 0x08, 0x63, 0x61,
    0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x42, 0x04, 0xc8, 0xde,
    0x1f, 0x00, 0x52, 0x08, 0x63, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74, 0x79, 0x12, 0x22, 0x0a, 0x09,
    0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x42,
    0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x09, 0x61, 0x76, 0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65,
    0x12, 0x27, 0x0a, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74,
    0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x0b, 0x72, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x32, 0x0a, 0x12, 0x73, 0x65, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x6e, 0x61, 0x70, 0x5f, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0d, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x10, 0x73, 0x65, 0x6e,
    0x64, 0x69, 0x6e, 0x67, 0x53, 0x6e, 0x61, 0x70, 0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x36, 0x0a,
    0x14, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x6e, 0x61, 0x70, 0x5f,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0d, 0x42, 0x04, 0xc8, 0xde, 0x1f,
    0x00, 0x52, 0x12, 0x72, 0x65, 0x63, 0x65, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x53, 0x6e, 0x61, 0x70,
    0x43, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x23, 0x0a, 0x0a, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74,
    0x69, 0x6d, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0d, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52,
    0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x54, 0x69, 0x6d, 0x65, 0x12, 0x34, 0x0a, 0x13, 0x61, 0x70,
    0x70, 0x6c, 0x79, 0x69, 0x6e, 0x67, 0x5f, 0x73, 0x6e, 0x61, 0x70, 0x5f, 0x63, 0x6f, 0x75, 0x6e,
    0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0d, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x11, 0x61,
    0x70, 0x70, 0x6c, 0x79, 0x69, 0x6e, 0x67, 0x53, 0x6e, 0x61, 0x70, 0x43, 0x6f, 0x75, 0x6e, 0x74,
    0x12, 0x1d, 0x0a, 0x07, 0x69, 0x73, 0x5f, 0x62, 0x75, 0x73, 0x79, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x08, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x06, 0x69, 0x73, 0x42, 0x75, 0x73, 0x79, 0x22,
    0x3f, 0x0a, 0x15, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x26, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74,
    0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x53, 0x74, 0x61, 0x74, 0x73, 0x52, 0x05, 0x73, 0x74, 0x61, 0x74, 0x73,
    0x22, 0x18, 0x0a, 0x16, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65,
    0x61, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x5e, 0x0a, 0x12, 0x52, 0x65,
    0x70, 0x6f, 0x72, 0x74, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x22, 0x0a, 0x04, 0x6c, 0x65, 0x66, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e,
    0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x04,
    0x6c, 0x65, 0x66, 0x74, 0x12, 0x24, 0x0a, 0x05, 0x72, 0x69, 0x67, 0x68, 0x74, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x52, 0x05, 0x72, 0x69, 0x67, 0x68, 0x74, 0x22, 0x15, 0x0a, 0x13, 0x52, 0x65,
    0x70, 0x6f, 0x72, 0x74, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x22, 0x48, 0x0a, 0x0d, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12, 0x23, 0x0a, 0x0a, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00,
    0x52, 0x09, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x49, 0x64, 0x22, 0x6c, 0x0a, 0x0e, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x12, 0x0a,
    0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x75, 0x75, 0x69,
    0x64, 0x12, 0x23, 0x0a, 0x0a, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x04, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x09, 0x63, 0x6c, 0x75,
    0x73, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12, 0x21, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0xfa, 0x07, 0x0a, 0x07, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2b, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x12, 0x32, 0x0a, 0x08, 0x63, 0x6d, 0x64, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x54, 0x79, 0x70, 0x65, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x07, 0x63,
    0x6d, 0x64, 0x54, 0x79, 0x70, 0x65, 0x12, 0x22, 0x0a, 0x03, 0x74, 0x73, 0x6f, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x54, 0x73, 0x6f, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x03, 0x74, 0x73, 0x6f, 0x12, 0x34, 0x0a, 0x09, 0x62, 0x6f,
    0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e,
    0x70, 0x64, 0x70, 0x62, 0x2e, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x09, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70,
    0x12, 0x44, 0x0a, 0x0f, 0x69, 0x73, 0x5f, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70,
    0x70, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x70, 0x64, 0x70, 0x62,
    0x2e, 0x49, 0x73, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0e, 0x69, 0x73, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74,
    0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x12, 0x2f, 0x0a, 0x08, 0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x5f,
    0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e,
    0x41, 0x6c, 0x6c, 0x6f, 0x63, 0x49, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x07,
    0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x49, 0x64, 0x12, 0x32, 0x0a, 0x09, 0x67, 0x65, 0x74, 0x5f, 0x73,
    0x74, 0x6f, 0x72, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x70, 0x64, 0x70,
    0x62, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x52, 0x08, 0x67, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x12, 0x32, 0x0a, 0x09, 0x70,
    0x75, 0x74, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15,
    0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x50, 0x75, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x08, 0x70, 0x75, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x12,
    0x32, 0x0a, 0x09, 0x61, 0x73, 0x6b, 0x5f, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x15, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x41, 0x73, 0x6b, 0x53, 0x70, 0x6c,
    0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x08, 0x61, 0x73, 0x6b, 0x53, 0x70,
    0x6c, 0x69, 0x74, 0x12, 0x35, 0x0a, 0x0a, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x47,
    0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52,
    0x09, 0x67, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x47, 0x0a, 0x10, 0x72, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x18, 0x0b,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x52, 0x0f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62,
    0x65, 0x61, 0x74, 0x12, 0x4b, 0x0a, 0x12, 0x67, 0x65, 0x74, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74,
    0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1d, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x10,
    0x67, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x12, 0x4b, 0x0a, 0x12, 0x70, 0x75, 0x74, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f,
    0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x70,
    0x64, 0x70, 0x62, 0x2e, 0x50, 0x75, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f,
    0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x10, 0x70, 0x75, 0x74,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x44, 0x0a,
    0x0f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74,
    0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x53, 0x74,
    0x6f, 0x72, 0x65, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x52, 0x0e, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62,
    0x65, 0x61, 0x74, 0x12, 0x3b, 0x0a, 0x0c, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x70,
    0x6c, 0x69, 0x74, 0x18, 0x0f, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x70, 0x64, 0x70, 0x62,
    0x2e, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x52, 0x0b, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x53, 0x70, 0x6c, 0x69, 0x74,
    0x12, 0x43, 0x0a, 0x10, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x62,
    0x79, 0x5f, 0x69, 0x64, 0x18, 0x10, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x70, 0x64, 0x70,
    0x62, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x42, 0x79, 0x49, 0x44, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0d, 0x67, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x42, 0x79, 0x49, 0x64, 0x12, 0x3f, 0x0a, 0x0e, 0x67, 0x65, 0x74, 0x5f, 0x70, 0x64, 0x5f,
    0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x18, 0x11, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e,
    0x70, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65, 0x74, 0x50, 0x44, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72,
    0x73, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0c, 0x67, 0x65, 0x74, 0x50, 0x64, 0x4d,
    0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x22, 0x87, 0x08, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x2c, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x12, 0x32, 0x0a, 0x08, 0x63, 0x6d, 0x64, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x61,
    0x6e, 0x64, 0x54, 0x79, 0x70, 0x65, 0x42, 0x04, 0xc8, 0xde, 0x1f, 0x00, 0x52, 0x07, 0x63, 0x6d,
    0x64, 0x54, 0x79, 0x70, 0x65, 0x12, 0x23, 0x0a, 0x03, 0x74, 0x73, 0x6f, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x54, 0x73, 0x6f, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x03, 0x74, 0x73, 0x6f, 0x12, 0x35, 0x0a, 0x09, 0x62, 0x6f,
    0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e,
    0x70, 0x64, 0x70, 0x62, 0x2e, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x09, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61,
    0x70, 0x12, 0x45, 0x0a, 0x0f, 0x69, 0x73, 0x5f, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61,
    0x70, 0x70, 0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x70, 0x64, 0x70,
    0x62, 0x2e, 0x49, 0x73, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0e, 0x69, 0x73, 0x42, 0x6f, 0x6f, 0x74,
    0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x12, 0x30, 0x0a, 0x08, 0x61, 0x6c, 0x6c, 0x6f,
    0x63, 0x5f, 0x69, 0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x70, 0x64, 0x70,
    0x62, 0x2e, 0x41, 0x6c, 0x6c, 0x6f, 0x63, 0x49, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x52, 0x07, 0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x49, 0x64, 0x12, 0x33, 0x0a, 0x09, 0x67, 0x65,
    0x74, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e,
    0x70, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x08, 0x67, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x12,
    0x33, 0x0a, 0x09, 0x70, 0x75, 0x74, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x16, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x50, 0x75, 0x74, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x08, 0x70, 0x75, 0x74, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x12, 0x33, 0x0a, 0x09, 0x61, 0x73, 0x6b, 0x5f, 0x73, 0x70, 0x6c, 0x69,
    0x74, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x41,
    0x73, 0x6b, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52,
    0x08, 0x61, 0x73, 0x6b, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x12, 0x36, 0x0a, 0x0a, 0x67, 0x65, 0x74,
    0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e,
    0x70, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x09, 0x67, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x12, 0x48, 0x0a, 0x10, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x68, 0x65, 0x61, 0x72,
    0x74, 0x62, 0x65, 0x61, 0x74, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x70, 0x64,
    0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65,
    0x61, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0f, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x12, 0x4c, 0x0a, 0x12, 0x67,
    0x65, 0x74, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69,
    0x67, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x47,
    0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x10, 0x67, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x4c, 0x0a, 0x12, 0x70, 0x75, 0x74,
    0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18,
    0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x50, 0x75, 0x74,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x10, 0x70, 0x75, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x45, 0x0a, 0x0f, 0x73, 0x74, 0x6f, 0x72, 0x65,
    0x5f, 0x68, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x18, 0x0e, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1c, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x48, 0x65, 0x61,
    0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0e,
    0x73, 0x74, 0x6f, 0x72, 0x65, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x12, 0x3c,
    0x0a, 0x0c, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x5f, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x18, 0x0f,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x70, 0x6f,
    0x72, 0x74, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52,
    0x0b, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x12, 0x40, 0x0a, 0x10,
    0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x62, 0x79, 0x5f, 0x69, 0x64,
    0x18, 0x10, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65,
    0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52,
    0x0d, 0x67, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x42, 0x79, 0x49, 0x64, 0x12, 0x40,
    0x0a, 0x0e, 0x67, 0x65, 0x74, 0x5f, 0x70, 0x64, 0x5f, 0x6d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x18, 0x11, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65,
    0x74, 0x50, 0x44, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x52, 0x0c, 0x67, 0x65, 0x74, 0x50, 0x64, 0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73,
    0x22, 0x13, 0x0a, 0x11, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x17, 0x0a, 0x15, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x73,
    0x54, 0x6f, 0x6d, 0x62, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x22, 0x9e,
    0x01, 0x0a, 0x05, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x12, 0x3b, 0x0a, 0x0c, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70,
    0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e,
    0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x52, 0x0c, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x12,
    0x3e, 0x0a, 0x0c, 0x69, 0x73, 0x5f, 0x74, 0x6f, 0x6d, 0x62, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x49, 0x73, 0x54, 0x6f, 0x6d, 0x62, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x52, 0x0b, 0x69, 0x73, 0x54, 0x6f, 0x6d, 0x62, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x2a,
    0x97, 0x02, 0x0a, 0x0b, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x0b, 0x0a, 0x07, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x10, 0x00, 0x12, 0x07, 0x0a, 0x03,
    0x54, 0x73, 0x6f, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72,
    0x61, 0x70, 0x10, 0x02, 0x12, 0x12, 0x0a, 0x0e, 0x49, 0x73, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74,
    0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x41, 0x6c, 0x6c, 0x6f,
    0x63, 0x49, 0x64, 0x10, 0x04, 0x12, 0x0c, 0x0a, 0x08, 0x47, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72,
    0x65, 0x10, 0x05, 0x12, 0x0c, 0x0a, 0x08, 0x50, 0x75, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x10,
    0x06, 0x12, 0x0c, 0x0a, 0x08, 0x41, 0x73, 0x6b, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x10, 0x07, 0x12,
    0x0d, 0x0a, 0x09, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x10, 0x08, 0x12, 0x13,
    0x0a, 0x0f, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61,
    0x74, 0x10, 0x09, 0x12, 0x14, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x10, 0x0a, 0x12, 0x14, 0x0a, 0x10, 0x50, 0x75, 0x74,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x10, 0x0b, 0x12,
    0x12, 0x0a, 0x0e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x48, 0x65, 0x61, 0x72, 0x74, 0x62, 0x65, 0x61,
    0x74, 0x10, 0x0c, 0x12, 0x0f, 0x0a, 0x0b, 0x52, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x53, 0x70, 0x6c,
    0x69, 0x74, 0x10, 0x0d, 0x12, 0x11, 0x0a, 0x0d, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x42, 0x79, 0x49, 0x44, 0x10, 0x0e, 0x12, 0x10, 0x0a, 0x0c, 0x47, 0x65, 0x74, 0x50, 0x44,
    0x4d, 0x65, 0x6d, 0x62, 0x65, 0x72, 0x73, 0x10, 0x0f, 0x42, 0x0c, 0xc8, 0xe2, 0x1e, 0x01, 0xd0,
    0xe2, 0x1e, 0x01, 0xe0, 0xe2, 0x1e, 0x01, 0x4a, 0xdc, 0x66, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00,
    0xa8, 0x02, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a,
    0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0c, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03,
    0x07, 0x15, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04, 0x07, 0x16, 0x0a, 0x09, 0x0a,
    0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x07, 0x1d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08,
    0x00, 0x28, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x08, 0x00, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x08, 0x07, 0x20, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x07, 0x20, 0x0a, 0x0e, 0x0a, 0x07,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x08, 0x23, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12,
    0x03, 0x09, 0x00, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x09, 0x00,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x09, 0x07, 0x1c, 0x0a,
    0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x09, 0x07, 0x1c, 0x0a, 0x0e,
    0x0a, 0x07, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x08, 0xe7, 0x07, 0x01, 0x03, 0x12, 0x03, 0x09, 0x1f, 0x23, 0x0a, 0x08, 0x0a, 0x01,
    0x08, 0x12, 0x03, 0x0a, 0x00, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12, 0x03,
    0x0a, 0x00, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x0a, 0x07,
    0x22, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x0a, 0x07, 0x22,
    0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x21,
    0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x03, 0x12, 0x03, 0x0a, 0x25, 0x29, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x0c, 0x00, 0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00,
    0x01, 0x12, 0x03, 0x0c, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x0d, 0x04, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0d, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0d, 0x14, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x08, 0x12, 0x03, 0x0d, 0x1e, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x0d, 0x1f, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x0d, 0x1f, 0x33, 0x0a, 0x11, 0x0a, 0x0a,
    0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x1f, 0x33, 0x0a,
    0x12, 0x0a, 0x0b, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x0d, 0x20, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03,
    0x12, 0x03, 0x0d, 0x36, 0x3b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0e,
    0x04, 0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0e, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x1e, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x00, 0x02, 0x01,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x0e, 0x1f, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x0e, 0x1f, 0x33, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0e, 0x1f, 0x33, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0e,
    0x20, 0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x00, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x0e, 0x36, 0x3b, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x14, 0x00, 0x25, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x14, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x15, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02,
    0x12, 0x03, 0x15, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x16,
    0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x04, 0x07,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x16, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x17, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x02, 0x12, 0x03, 0x17, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12,
    0x03, 0x18, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x18,
    0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x18, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x19, 0x04, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x19, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x19, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x05, 0x12, 0x03, 0x1a, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12,
    0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x1a,
    0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x1b, 0x04, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x1b, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x07, 0x12, 0x03, 0x1c, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07,
    0x01, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12,
    0x03, 0x1c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x1d, 0x04,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x1d, 0x04, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x1d, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03, 0x1e, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x09, 0x01, 0x12, 0x03, 0x1e, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09,
    0x02, 0x12, 0x03, 0x1e, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03,
    0x1f, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x1f, 0x04,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0a, 0x02, 0x12, 0x03, 0x1f, 0x1a, 0x1c, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b, 0x12, 0x03, 0x20, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03, 0x20, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0b, 0x02, 0x12, 0x03, 0x20, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c,
    0x12, 0x03, 0x21, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03,
    0x21, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x21, 0x1a,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0d, 0x12, 0x03, 0x22, 0x04, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0d, 0x01, 0x12, 0x03, 0x22, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0d, 0x02, 0x12, 0x03, 0x22, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x0e, 0x12, 0x03, 0x23, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x01,
    0x12, 0x03, 0x23, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0e, 0x02, 0x12, 0x03,
    0x23, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0f, 0x12, 0x03, 0x24, 0x04, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x01, 0x12, 0x03, 0x24, 0x04, 0x10, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0f, 0x02, 0x12, 0x03, 0x24, 0x1a, 0x1c, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x27, 0x00, 0x29, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12,
    0x03, 0x27, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x28, 0x04,
    0x3d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x28, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x08, 0x12, 0x03, 0x28, 0x1e, 0x3c, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x01, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x03, 0x28, 0x1f, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x28, 0x1f, 0x33, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x01,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x28, 0x1f, 0x33, 0x0a, 0x12, 0x0a,
    0x0b, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x20,
    0x32, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x01, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03,
    0x28, 0x36, 0x3b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x2b, 0x00, 0x2e, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x04, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x2c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x2c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2c,
    0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x1e, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x08, 0x12, 0x03, 0x2c, 0x20, 0x3e, 0x0a, 0x0f,
    0x0a, 0x08, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x2c, 0x21, 0x3d, 0x0a,
    0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x2c, 0x21,
    0x35, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x2c, 0x21, 0x35, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x2c, 0x22, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x38, 0x3d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x2d, 0x04, 0x3f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x2d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x2d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2d, 0x13,
    0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2d, 0x1e, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x08, 0x12, 0x03, 0x2d, 0x20, 0x3e, 0x0a, 0x0f, 0x0a,
    0x08, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x2d, 0x21, 0x3d, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x2d, 0x21, 0x35,
    0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03,
    0x2d, 0x21, 0x35, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x02, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x2d, 0x22, 0x34, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x02, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x38, 0x3d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x30, 0x00, 0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x30, 0x08,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x31, 0x04, 0x44, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x31, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x31, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x08, 0x12,
    0x03, 0x31, 0x25, 0x43, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x03, 0x31, 0x26, 0x42, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x31, 0x26, 0x3a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x03, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x31, 0x26, 0x3a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x03,
    0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x31, 0x27, 0x39, 0x0a, 0x10,
    0x0a, 0x09, 0x04, 0x03, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x31, 0x3d, 0x42,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x32, 0x04, 0x44, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x32, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x06, 0x12, 0x03, 0x32, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x32, 0x17, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x32, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x08, 0x12, 0x03,
    0x32, 0x25, 0x43, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x03, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x03, 0x32, 0x26, 0x42, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x03, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x03, 0x32, 0x26, 0x3a, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x03, 0x02, 0x01, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x32, 0x26, 0x3a, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x03, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x32, 0x27, 0x39, 0x0a, 0x10, 0x0a,
    0x09, 0x04, 0x03, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x32, 0x3d, 0x42, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x35, 0x00, 0x38, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x04, 0x01, 0x12, 0x03, 0x35, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12,
    0x03, 0x36, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x36,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x36, 0x0d, 0x19,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x36, 0x1a, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x37, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06,
    0x12, 0x03, 0x37, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x37, 0x1b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x37, 0x24,
    0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x3a, 0x00, 0x3c, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x3a, 0x08, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12,
    0x04, 0x3e, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x3e, 0x08,
    0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x42, 0x00, 0x44, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x42, 0x08, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02,
    0x00, 0x12, 0x03, 0x43, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x43, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x43,
    0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x43, 0x12, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x43, 0x21, 0x22, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x46, 0x00, 0x48, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08,
    0x01, 0x12, 0x03, 0x46, 0x08, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x4a, 0x00,
    0x4c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03, 0x4a, 0x08, 0x17, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x4b, 0x04, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x4b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x4b, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x4b, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x08, 0x12, 0x03, 0x4b, 0x27,
    0x45, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x09, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x4b,
    0x28, 0x44, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x09, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x03, 0x4b, 0x28, 0x3c, 0x0a, 0x11, 0x0a, 0x0a, 0x04, 0x09, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x4b, 0x28, 0x3c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x09, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4b, 0x29, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04,
    0x09, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x03, 0x4b, 0x3f, 0x44, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x0a, 0x12, 0x04, 0x4e, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01,
    0x12, 0x03, 0x4e, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x4f,
    0x04, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4f, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x4f, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4f, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x00, 0x08, 0x12, 0x03, 0x4f, 0x27, 0x45, 0x0a, 0x0f, 0x0a, 0x08, 0x04, 0x0a, 0x02, 0x00,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x4f, 0x28, 0x44, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0a, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x4f, 0x28, 0x3c, 0x0a, 0x11, 0x0a, 0x0a, 0x04,
    0x0a, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x4f, 0x28, 0x3c, 0x0a, 0x12,
    0x0a, 0x0b, 0x04, 0x0a, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4f,
    0x29, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0a, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x03, 0x4f, 0x3f, 0x44, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x52, 0x00, 0x54, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x52, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x53, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x53, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x53, 0x1a, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x53, 0x26,
    0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x56, 0x00, 0x58, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x56, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02,
    0x00, 0x12, 0x03, 0x57, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x57,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57, 0x13, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x57, 0x25, 0x26, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x5a, 0x00, 0x5d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d,
    0x01, 0x12, 0x03, 0x5a, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03,
    0x5b, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5b, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5b, 0x0d, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5b, 0x1b, 0x21, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5b, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0d, 0x02, 0x01, 0x12, 0x03, 0x5c, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x5c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x06, 0x12,
    0x03, 0x5c, 0x0d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x5c,
    0x19, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5c, 0x26, 0x27,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x04, 0x5f, 0x00, 0x61, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0e, 0x01, 0x12, 0x03, 0x5f, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00,
    0x12, 0x03, 0x60, 0x04, 0x46, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x60, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x60, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03, 0x60, 0x14, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x60, 0x25, 0x26, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x00, 0x08, 0x12, 0x03, 0x60, 0x27, 0x45, 0x0a, 0x0f, 0x0a, 0x08, 0x04,
    0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x60, 0x28, 0x44, 0x0a, 0x10, 0x0a, 0x09,
    0x04, 0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x60, 0x28, 0x3c, 0x0a, 0x11,
    0x0a, 0x0a, 0x04, 0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x03, 0x60, 0x28,
    0x3c, 0x0a, 0x12, 0x0a, 0x0b, 0x04, 0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x60, 0x29, 0x3b, 0x0a, 0x10, 0x0a, 0x09, 0x04, 0x0e, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x03, 0x60, 0x3f, 0x44, 0x0a, 0x4c, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x65,
    0x00, 0x67, 0x01, 0x32, 0x40, 0x20, 0x55, 0x73, 0x65, 0x20, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x61, 0x73, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x47,
    0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x42, 0x79, 0x49, 0x44, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x65, 0x08,
    0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x04, 0x69, 0x00, 0x6b, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x10, 0x01, 0x12, 0x03, 0x69, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02,
    0x00, 0x12, 0x03, 0x6a, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12,
    0x03, 0x6a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6a,
    0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6a, 0x1c, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6a, 0x26, 0x27, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x11, 0x12, 0x04, 0x6e, 0x00, 0x70, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x11,
    0x01, 0x12, 0x03, 0x6e, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x03,
    0x6f, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6f, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x03, 0x6f, 0x0d, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x03, 0x6f, 0x1a, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x03, 0x6f, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x12, 0x12, 0x04, 0x72, 0x00, 0x73, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x03,
    0x72, 0x08, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x04, 0x75, 0x00, 0x79, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x03, 0x75, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x13, 0x02, 0x00, 0x12, 0x03, 0x76, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x76, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x76, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x03, 0x76,
    0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x03, 0x76, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x03, 0x77, 0x04, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x03, 0x77, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x01, 0x05, 0x12, 0x03, 0x77, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x77, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x77, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x02, 0x12, 0x03, 0x78,
    0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x04, 0x12, 0x03, 0x78, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x05, 0x12, 0x03, 0x78, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x13, 0x02, 0x02, 0x01, 0x12, 0x03, 0x78, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x02, 0x03, 0x12, 0x03, 0x78, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x14,
    0x12, 0x04, 0x7b, 0x00, 0x7c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x03, 0x7b,
    0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x05, 0x7e, 0x00, 0x80, 0x01, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x03, 0x7e, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x15, 0x02, 0x00, 0x12, 0x03, 0x7f, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x7f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x7f, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7f,
    0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7f, 0x20, 0x21,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0x82, 0x01, 0x00, 0x85, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x82, 0x01, 0x08, 0x11, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x16, 0x02, 0x00, 0x12, 0x04, 0x83, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x83, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x83, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x83, 0x01, 0x19, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x83, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04, 0x84,
    0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0x84, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x05, 0x12, 0x04, 0x84, 0x01, 0x0d,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0x84, 0x01, 0x14, 0x20,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0x84, 0x01, 0x23, 0x24, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0x87, 0x01, 0x00, 0x90, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0x87, 0x01, 0x08, 0x1e, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x17,
    0x02, 0x00, 0x12, 0x04, 0x88, 0x01, 0x04, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x88, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x06,
    0x12, 0x04, 0x88, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x88, 0x01, 0x1b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x88, 0x01, 0x2a, 0x2b, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x01, 0x12, 0x04, 0x8a, 0x01,
    0x04, 0x2c, 0x1a, 0x25, 0x20, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x50, 0x65, 0x65, 0x72,
    0x20, 0x73, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x68, 0x65, 0x61,
    0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x2e, 0x20, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x01, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01,
    0x06, 0x12, 0x04, 0x8a, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x01,
    0x12, 0x04, 0x8a, 0x01, 0x19, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x01, 0x03, 0x12,
    0x04, 0x8a, 0x01, 0x2a, 0x2b, 0x0a, 0x3b, 0x0a, 0x04, 0x04, 0x17, 0x02, 0x02, 0x12, 0x04, 0x8c,
    0x01, 0x04, 0x2c, 0x1a, 0x2d, 0x20, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6e,
    0x73, 0x69, 0x64, 0x65, 0x72, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74, 0x68, 0x65, 0x73,
    0x65, 0x20, 0x70, 0x65, 0x65, 0x72, 0x73, 0x20, 0x61, 0x72, 0x65, 0x20, 0x64, 0x6f, 0x77, 0x6e,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x06, 0x12, 0x04, 0x8c, 0x01, 0x0d, 0x16,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x17, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x2a, 0x2b, 0x0a, 0x61,
    0x0a, 0x04, 0x04, 0x17, 0x02, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x2c, 0x1a, 0x53, 0x20, 0x50,
    0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x70, 0x65, 0x65, 0x72, 0x73, 0x20, 0x61, 0x72, 0x65,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x65, 0x65, 0x72, 0x73, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x63, 0x61, 0x6e, 0x27, 0x74,
    0x20, 0x63, 0x6f, 0x6e, 0x73, 0x69, 0x64, 0x65, 0x72, 0x20, 0x61, 0x73, 0x0a, 0x20, 0x77, 0x6f,
    0x72, 0x6b, 0x69, 0x6e, 0x67, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77, 0x65, 0x72, 0x73, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x06, 0x12, 0x04, 0x8f, 0x01, 0x0d, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x19, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x17, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x18, 0x12, 0x06, 0x92, 0x01, 0x00, 0x95, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x18, 0x01, 0x12, 0x04, 0x92, 0x01, 0x08, 0x12, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00,
    0x12, 0x04, 0x93, 0x01, 0x04, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x93, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x06, 0x12, 0x04,
    0x93, 0x01, 0x0d, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0x93,
    0x01, 0x24, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0x93, 0x01,
    0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x04, 0x94, 0x01, 0x04, 0x34,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x04, 0x94, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x06, 0x12, 0x04, 0x94, 0x01, 0x0d, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12, 0x04, 0x94, 0x01, 0x19, 0x1d, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04, 0x94, 0x01, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x19, 0x12, 0x06, 0x97, 0x01, 0x00, 0x99, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19,
    0x01, 0x12, 0x04, 0x97, 0x01, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12,
    0x04, 0x98, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x98, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x06, 0x12, 0x04, 0x98,
    0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0x98, 0x01,
    0x19, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0x98, 0x01, 0x20,
    0x21, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0x9b, 0x01, 0x00, 0xad, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x08, 0x1f, 0x0a, 0xd5, 0x06, 0x0a,
    0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0xaa, 0x01, 0x04, 0x32, 0x1a, 0xc6, 0x06, 0x20, 0x4e,
    0x6f, 0x74, 0x69, 0x63, 0x65, 0x2c, 0x20, 0x50, 0x64, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x61,
    0x6c, 0x6c, 0x6f, 0x77, 0x73, 0x20, 0x68, 0x61, 0x6e, 0x64, 0x6c, 0x69, 0x6e, 0x67, 0x20, 0x72,
    0x65, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x20, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x20, 0x3e, 0x3d,
    0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x70, 0x64, 0x27, 0x73, 0x2e, 0x20, 0x0a,
    0x20, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x70, 0x65, 0x65, 0x72, 0x20, 0x72, 0x65, 0x70,
    0x6f, 0x72, 0x74, 0x73, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x74,
    0x75, 0x73, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x48, 0x65,
    0x61, 0x72, 0x74, 0x62, 0x65, 0x61, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x0a, 0x20,
    0x74, 0x6f, 0x20, 0x70, 0x64, 0x20, 0x72, 0x65, 0x67, 0x75, 0x6c, 0x61, 0x72, 0x6c, 0x79, 0x2c,
    0x20, 0x70, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x64, 0x65, 0x74, 0x65, 0x72, 0x6d, 0x69,
    0x6e, 0x65, 0x20, 0x77, 0x68, 0x65, 0x74, 0x68, 0x65, 0x72, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20,
    0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x0a, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20,
    0x64, 0x6f, 0x20, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x20, 0x6f, 0x72,
    0x20, 0x6e, 0x6f, 0x74, 0x2e, 0x0a, 0x20, 0x45, 0x2c, 0x67, 0x2c, 0x20, 0x6d, 0x61, 0x78, 0x20,
    0x70, 0x65, 0x65, 0x72, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x33,
    0x2c, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x41, 0x2c, 0x20, 0x66, 0x69, 0x72, 0x73,
    0x74, 0x20, 0x6f, 0x6e, 0x6c, 0x79, 0x20, 0x70, 0x65, 0x65, 0x72, 0x20, 0x31, 0x20, 0x69, 0x6e,
    0x20, 0x41, 0x2e, 0x0a, 0x20, 0x31, 0x2e, 0x20, 0x50, 0x64, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x2d, 0x3e, 0x20, 0x50, 0x65, 0x65, 0x72, 0x73,
    0x20, 0x28, 0x31, 0x29, 0x2c, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x56, 0x65, 0x72, 0x20, 0x28, 0x31,
    0x29, 0x2e, 0x0a, 0x20, 0x32, 0x2e, 0x20, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x70, 0x65,
    0x65, 0x72, 0x20, 0x31, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x20, 0x72, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x64, 0x2c,
    0x20, 0x70, 0x64, 0x20, 0x66, 0x69, 0x6e, 0x64, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x0a, 0x20,
    0x70, 0x65, 0x65, 0x72, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x3c,
    0x20, 0x33, 0x2c, 0x20, 0x73, 0x6f, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x63, 0x68, 0x61,
    0x6e, 0x67, 0x65, 0x73, 0x20, 0x69, 0x74, 0x73, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74,
    0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x0a, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20,
    0x2d, 0x3e, 0x20, 0x50, 0x65, 0x65, 0x72, 0x73, 0x20, 0x28, 0x31, 0x2c, 0x20, 0x32, 0x29, 0x2c,
    0x20, 0x43, 0x6f, 0x6e, 0x66, 0x56, 0x65, 0x72, 0x20, 0x28, 0x31, 0x29, 0x2c, 0x20, 0x61, 0x6e,
    0x64, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x73, 0x20, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x50, 0x65, 0x65, 0x72, 0x20, 0x41, 0x64, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x32, 0x2e, 0x0a, 0x20,
    0x33, 0x2e, 0x20, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x20, 0x43,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x2c, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20,
    0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x73, 0x20, 0x50, 0x65, 0x65, 0x72, 0x73, 0x20, 0x28, 0x31,
    0x2c, 0x20, 0x32, 0x29, 0x2c, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x56, 0x65, 0x72, 0x20, 0x28, 0x32,
    0x29, 0x2c, 0x0a, 0x20, 0x70, 0x64, 0x20, 0x75, 0x70, 0x64, 0x61, 0x74, 0x65, 0x73, 0x20, 0x69,
    0x74, 0x73, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x20, 0x2d, 0x3e, 0x20, 0x50, 0x65, 0x65, 0x72,
    0x73, 0x20, 0x28, 0x31, 0x2c, 0x20, 0x32, 0x29, 0x2c, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x56, 0x65,
    0x72, 0x20, 0x28, 0x32, 0x29, 0x2e, 0x0a, 0x20, 0x34, 0x2e, 0x20, 0x4c, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x72, 0x65, 0x70, 0x6f, 0x72, 0x74, 0x20, 0x6f, 0x6c, 0x64,
    0x20, 0x50, 0x65, 0x65, 0x72, 0x73, 0x20, 0x28, 0x31, 0x29, 0x2c, 0x20, 0x43, 0x6f, 0x6e, 0x66,
    0x56, 0x65, 0x72, 0x20, 0x28, 0x31, 0x29, 0x20, 0x74, 0x6f, 0x20, 0x70, 0x64, 0x20, 0x62, 0x65,
    0x66, 0x6f, 0x72, 0x65, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x0a,
    0x20, 0x66, 0x69, 0x6e, 0x69, 0x73, 0x68, 0x65, 0x64, 0x2c, 0x20, 0x70, 0x64, 0x20, 0x73, 0x74,
    0x69, 0x6c, 0x6c, 0x73, 0x20, 0x72, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x73, 0x20, 0x43,
    0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x20, 0x41, 0x64, 0x64, 0x69, 0x6e, 0x67,
    0x20, 0x32, 0x2c, 0x20, 0x6f, 0x66, 0x20, 0x63, 0x6f, 0x75, 0x72, 0x73, 0x65, 0x2c, 0x20, 0x77,
    0x65, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20, 0x0a, 0x20, 0x67, 0x75, 0x61, 0x72, 0x61, 0x6e, 0x74,
    0x65, 0x65, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x20, 0x43, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x20, 0x63, 0x61, 0x6e, 0x27, 0x74, 0x20, 0x62,
    0x65, 0x20, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x54, 0x69, 0x4b,
    0x56, 0x2e, 0x20, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0xaa,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x06, 0x12, 0x04, 0xaa, 0x01,
    0x0d, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x18,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x30, 0x31,
    0x0a, 0x57, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12, 0x04, 0xac, 0x01, 0x04, 0x32, 0x1a, 0x49,
    0x20, 0x50, 0x64, 0x20, 0x63, 0x61, 0x6e, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x20, 0x74,
    0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x5f, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x74,
    0x6f, 0x20, 0x6c, 0x65, 0x74, 0x20, 0x54, 0x69, 0x4b, 0x56, 0x20, 0x64, 0x6f, 0x65, 0x73, 0x20,
    0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x66, 0x65, 0x72, 0x20,
    0x69, 0x74, 0x73, 0x65, 0x6c, 0x66, 0x2e, 0x20, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x01, 0x04, 0x12, 0x04, 0xac, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01,
    0x06, 0x12, 0x04, 0xac, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01,
    0x12, 0x04, 0xac, 0x01, 0x1c, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12,
    0x04, 0xac, 0x01, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0xaf, 0x01, 0x00,
    0xb1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x08, 0x1f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x00, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x28, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb0, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x1c, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb0, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1c,
    0x12, 0x06, 0xb3, 0x01, 0x00, 0xb4, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c, 0x01, 0x12,
    0x04, 0xb3, 0x01, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xb6, 0x01, 0x00,
    0xb8, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xb6, 0x01, 0x08, 0x17,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x2b, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xb7, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x00, 0x06, 0x12, 0x04, 0xb7, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xb7, 0x01, 0x1b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xb7, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1e,
    0x12, 0x06, 0xba, 0x01, 0x00, 0xc1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1e, 0x01, 0x12,
    0x04, 0xba, 0x01, 0x08, 0x18, 0x0a, 0xbb, 0x01, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x00, 0x12, 0x04,
    0xbe, 0x01, 0x04, 0x51, 0x1a, 0xac, 0x01, 0x20, 0x57, 0x65, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x6e, 0x74, 0x6f,
    0x20, 0x74, 0x77, 0x6f, 0x2c, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x75, 0x73, 0x65, 0x73,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6f, 0x72, 0x69, 0x67, 0x69, 0x6e, 0x20, 0x0a, 0x20, 0x70, 0x61,
    0x72, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x64, 0x2c, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x20, 0x75,
    0x73, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x5f, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x2e, 0x0a, 0x20, 0x57, 0x65, 0x20, 0x6d, 0x75, 0x73, 0x74, 0x20,
    0x67, 0x75, 0x61, 0x72, 0x61, 0x6e, 0x74, 0x65, 0x65, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64,
    0x20, 0x69, 0x73, 0x20, 0x67, 0x6c, 0x6f, 0x62, 0x61, 0x6c, 0x20, 0x75, 0x6e, 0x69, 0x71, 0x75,
    0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x04, 0x12, 0x04, 0xbe, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x05, 0x12, 0x04, 0xbe, 0x01, 0x0d,
    0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x14, 0x21,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x30, 0x31, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x00, 0x08, 0x12, 0x04, 0xbe, 0x01, 0x32, 0x50, 0x0a, 0x10,
    0x0a, 0x08, 0x04, 0x1e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xbe, 0x01, 0x33, 0x4f,
    0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xbe,
    0x01, 0x33, 0x47, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x1e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x04, 0xbe, 0x01, 0x33, 0x47, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x1e, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xbe, 0x01, 0x34, 0x46, 0x0a, 0x11, 0x0a, 0x09,
    0x04, 0x1e, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xbe, 0x01, 0x4a, 0x4f, 0x0a,
    0x36, 0x0a, 0x04, 0x04, 0x1e, 0x02, 0x01, 0x12, 0x04, 0xc0, 0x01, 0x04, 0x32, 0x1a, 0x28, 0x20,
    0x54, 0x68, 0x65, 0x20, 0x70, 0x65, 0x65, 0x72, 0x20, 0x69, 0x64, 0x73, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xc0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xc0, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xc0, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1e, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc0,
    0x01, 0x30, 0x31, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1f, 0x12, 0x06, 0xc3, 0x01, 0x00, 0xd5, 0x01,
    0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1f, 0x01, 0x12, 0x04, 0xc3, 0x01, 0x08, 0x12, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1f, 0x02, 0x00, 0x12, 0x04, 0xc4, 0x01, 0x04, 0x4c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x00, 0x04, 0x12, 0x04, 0xc4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x00, 0x05, 0x12, 0x04, 0xc4, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xc4, 0x01, 0x14, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x00, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x00,
    0x08, 0x12, 0x04, 0xc4, 0x01, 0x2d, 0x4b, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1f, 0x02, 0x00, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x04, 0xc4, 0x01, 0x2e, 0x4a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02,
    0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xc4, 0x01, 0x2e, 0x42, 0x0a, 0x12, 0x0a, 0x0a,
    0x04, 0x1f, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xc4, 0x01, 0x2e, 0x42,
    0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x1f, 0x02, 0x00, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xc4, 0x01, 0x2f, 0x41, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02, 0x00, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x04, 0xc4, 0x01, 0x45, 0x4a, 0x0a, 0x27, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x01,
    0x12, 0x04, 0xc6, 0x01, 0x04, 0x4c, 0x1a, 0x19, 0x20, 0x43, 0x61, 0x70, 0x61, 0x63, 0x69, 0x74,
    0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x04, 0x12, 0x04, 0xc6, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x05, 0x12, 0x04, 0xc6, 0x01, 0x0d, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x14, 0x1c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x01, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x01, 0x08, 0x12, 0x04, 0xc6, 0x01, 0x2d, 0x4b, 0x0a, 0x10, 0x0a, 0x08,
    0x04, 0x1f, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xc6, 0x01, 0x2e, 0x4a, 0x0a, 0x11,
    0x0a, 0x09, 0x04, 0x1f, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xc6, 0x01, 0x2e,
    0x42, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x1f, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xc6, 0x01, 0x2e, 0x42, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x1f, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xc6, 0x01, 0x2f, 0x41, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xc6, 0x01, 0x45, 0x4a, 0x0a, 0x2d, 0x0a,
    0x04, 0x04, 0x1f, 0x02, 0x02, 0x12, 0x04, 0xc8, 0x01, 0x04, 0x4c, 0x1a, 0x1f, 0x20, 0x41, 0x76,
    0x61, 0x69, 0x6c, 0x61, 0x62, 0x6c, 0x65, 0x20, 0x73, 0x69, 0x7a, 0x65, 0x20, 0x66, 0x6f, 0x72,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x02, 0x04, 0x12, 0x04, 0xc8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x02, 0x05, 0x12, 0x04, 0xc8, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x02, 0x01, 0x12, 0x04, 0xc8, 0x01, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x02, 0x03, 0x12, 0x04, 0xc8, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x02,
    0x08, 0x12, 0x04, 0xc8, 0x01, 0x2d, 0x4b, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1f, 0x02, 0x02, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x04, 0xc8, 0x01, 0x2e, 0x4a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02,
    0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xc8, 0x01, 0x2e, 0x42, 0x0a, 0x12, 0x0a, 0x0a,
    0x04, 0x1f, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xc8, 0x01, 0x2e, 0x42,
    0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x1f, 0x02, 0x02, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xc8, 0x01, 0x2f, 0x41, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02, 0x02, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x04, 0xc8, 0x01, 0x45, 0x4a, 0x0a, 0x32, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x03,
    0x12, 0x04, 0xca, 0x01, 0x04, 0x4c, 0x1a, 0x24, 0x20, 0x54, 0x6f, 0x74, 0x61, 0x6c, 0x20, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x68, 0x69, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2e, 0x20, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x03, 0x04, 0x12, 0x04, 0xca, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x03, 0x05, 0x12, 0x04, 0xca, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x03, 0x01, 0x12, 0x04, 0xca, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x03, 0x03, 0x12, 0x04, 0xca, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x03,
    0x08, 0x12, 0x04, 0xca, 0x01, 0x2d, 0x4b, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1f, 0x02, 0x03, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x04, 0xca, 0x01, 0x2e, 0x4a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02,
    0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xca, 0x01, 0x2e, 0x42, 0x0a, 0x12, 0x0a, 0x0a,
    0x04, 0x1f, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xca, 0x01, 0x2e, 0x42,
    0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x1f, 0x02, 0x03, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xca, 0x01, 0x2f, 0x41, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02, 0x03, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x04, 0xca, 0x01, 0x45, 0x4a, 0x0a, 0x2f, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x04,
    0x12, 0x04, 0xcc, 0x01, 0x04, 0x4c, 0x1a, 0x21, 0x20, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74,
    0x20, 0x73, 0x65, 0x6e, 0x64, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f,
    0x74, 0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x04, 0x04, 0x12, 0x04, 0xcc, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04,
    0x05, 0x12, 0x04, 0xcc, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x01,
    0x12, 0x04, 0xcc, 0x01, 0x14, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x03, 0x12,
    0x04, 0xcc, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x04, 0x08, 0x12, 0x04,
    0xcc, 0x01, 0x2d, 0x4b, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1f, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00,
    0x12, 0x04, 0xcc, 0x01, 0x2e, 0x4a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02, 0x04, 0x08, 0xe7,
    0x07, 0x00, 0x02, 0x12, 0x04, 0xcc, 0x01, 0x2e, 0x42, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x1f, 0x02,
    0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xcc, 0x01, 0x2e, 0x42, 0x0a, 0x13, 0x0a,
    0x0b, 0x04, 0x1f, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xcc, 0x01,
    0x2f, 0x41, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12,
    0x04, 0xcc, 0x01, 0x45, 0x4a, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x05, 0x12, 0x04, 0xce,
    0x01, 0x04, 0x4c, 0x1a, 0x23, 0x20, 0x43, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65,
    0x63, 0x65, 0x69, 0x76, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74,
    0x20, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x05,
    0x04, 0x12, 0x04, 0xce, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x05, 0x05,
    0x12, 0x04, 0xce, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x05, 0x01, 0x12,
    0x04, 0xce, 0x01, 0x14, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x05, 0x03, 0x12, 0x04,
    0xce, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x05, 0x08, 0x12, 0x04, 0xce,
    0x01, 0x2d, 0x4b, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1f, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x12,
    0x04, 0xce, 0x01, 0x2e, 0x4a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x04, 0xce, 0x01, 0x2e, 0x42, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x1f, 0x02, 0x05,
    0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xce, 0x01, 0x2e, 0x42, 0x0a, 0x13, 0x0a, 0x0b,
    0x04, 0x1f, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xce, 0x01, 0x2f,
    0x41, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04,
    0xce, 0x01, 0x45, 0x4a, 0x0a, 0x46, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x06, 0x12, 0x04, 0xd0, 0x01,
    0x04, 0x4c, 0x1a, 0x38, 0x20, 0x57, 0x68, 0x65, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20, 0x73, 0x74,
    0x6f, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x73, 0x74, 0x61, 0x72, 0x74, 0x65, 0x64, 0x20, 0x28,
    0x75, 0x6e, 0x69, 0x78, 0x20, 0x74, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x20, 0x69,
    0x6e, 0x20, 0x73, 0x65, 0x63, 0x6f, 0x6e, 0x64, 0x73, 0x29, 0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x06, 0x04, 0x12, 0x04, 0xd0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x06, 0x05, 0x12, 0x04, 0xd0, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f,
    0x02, 0x06, 0x01, 0x12, 0x04, 0xd0, 0x01, 0x14, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02,
    0x06, 0x03, 0x12, 0x04, 0xd0, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x06,
    0x08, 0x12, 0x04, 0xd0, 0x01, 0x2d, 0x4b, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1f, 0x02, 0x06, 0x08,
    0xe7, 0x07, 0x00, 0x12, 0x04, 0xd0, 0x01, 0x2e, 0x4a, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02,
    0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xd0, 0x01, 0x2e, 0x42, 0x0a, 0x12, 0x0a, 0x0a,
    0x04, 0x1f, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xd0, 0x01, 0x2e, 0x42,
    0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x1f, 0x02, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x04, 0xd0, 0x01, 0x2f, 0x41, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x03, 0x12, 0x04, 0xd0, 0x01, 0x45, 0x4a, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x1f, 0x02, 0x07,
    0x12, 0x04, 0xd2, 0x01, 0x04, 0x4c, 0x1a, 0x27, 0x20, 0x48, 0x6f, 0x77, 0x20, 0x6d, 0x61, 0x6e,
    0x79, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20, 0x61, 0x70, 0x70, 0x6c,
    0x79, 0x69, 0x6e, 0x67, 0x20, 0x73, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x2e, 0x0a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x07, 0x04, 0x12, 0x04, 0xd2, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x07, 0x05, 0x12, 0x04, 0xd2, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x07, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x14, 0x27, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1f, 0x02, 0x07, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1f, 0x02, 0x07, 0x08, 0x12, 0x04, 0xd2, 0x01, 0x2d, 0x4b, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x1f,
    0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xd2, 0x01, 0x2e, 0x4a, 0x0a, 0x11, 0x0a, 0x09,
    0x04, 0x1f, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xd2, 0x01, 0x2e, 0x42, 0x0a,
    0x12, 0x0a, 0x0a, 0x04, 0x1f, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xd2,
    0x01, 0x2e, 0x42, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x1f, 0x02, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x01, 0x12, 0x04, 0xd2, 0x01, 0x2f, 0x41, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f, 0x02, 0x07,
    0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xd2, 0x01, 0x45, 0x4a, 0x0a, 0x24, 0x0a, 0x04, 0x04,
    0x1f, 0x02, 0x08, 0x12, 0x04, 0xd4, 0x01, 0x04, 0x4c, 0x1a, 0x16, 0x20, 0x49, 0x66, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x73, 0x20, 0x62, 0x75, 0x73, 0x79,
    0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x08, 0x04, 0x12, 0x04, 0xd4, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x08, 0x05, 0x12, 0x04, 0xd4, 0x01, 0x0d, 0x11, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1f, 0x02, 0x08, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x12, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1f, 0x02, 0x08, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x2b, 0x2c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1f, 0x02, 0x08, 0x08, 0x12, 0x04, 0xd4, 0x01, 0x2d, 0x4b, 0x0a, 0x10, 0x0a, 0x08,
    0x04, 0x1f, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xd4, 0x01, 0x2e, 0x4a, 0x0a, 0x11,
    0x0a, 0x09, 0x04, 0x1f, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xd4, 0x01, 0x2e,
    0x42, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x1f, 0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xd4, 0x01, 0x2e, 0x42, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x1f, 0x02, 0x08, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd4, 0x01, 0x2f, 0x41, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x1f,
    0x02, 0x08, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xd4, 0x01, 0x45, 0x4a, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x20, 0x12, 0x06, 0xd7, 0x01, 0x00, 0xd9, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x20, 0x01, 0x12, 0x04, 0xd7, 0x01, 0x08, 0x1d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x20, 0x02, 0x00,
    0x12, 0x04, 0xd8, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xd8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xd8, 0x01, 0x0d, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x01, 0x12, 0x04, 0xd8,
    0x01, 0x18, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x20, 0x02, 0x00, 0x03, 0x12, 0x04, 0xd8, 0x01,
    0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x21, 0x12, 0x06, 0xdb, 0x01, 0x00, 0xdd, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x21, 0x01, 0x12, 0x04, 0xdb, 0x01, 0x08, 0x1e, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x22, 0x12, 0x06, 0xdf, 0x01, 0x00, 0xe2, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x22, 0x01, 0x12, 0x04, 0xdf, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x00,
    0x12, 0x04, 0xe0, 0x01, 0x04, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xe0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xe0, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x01, 0x12, 0x04, 0xe0,
    0x01, 0x1b, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x00, 0x03, 0x12, 0x04, 0xe0, 0x01,
    0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x22, 0x02, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x04, 0x27,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x04, 0x12, 0x04, 0xe1, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x06, 0x12, 0x04, 0xe1, 0x01, 0x0d, 0x1a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x22, 0x02, 0x01, 0x01, 0x12, 0x04, 0xe1, 0x01, 0x1b, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x22, 0x02, 0x01, 0x03, 0x12, 0x04, 0xe1, 0x01, 0x25, 0x26, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x23, 0x12, 0x06, 0xe4, 0x01, 0x00, 0xe6, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x23,
    0x01, 0x12, 0x04, 0xe4, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x24, 0x12, 0x06, 0xe8,
    0x01, 0x00, 0xec, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x24, 0x01, 0x12, 0x04, 0xe8, 0x01,
    0x08, 0x15, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x00, 0x12, 0x04, 0xea, 0x01, 0x04, 0x2b,
    0x1a, 0x25, 0x20, 0x31, 0x36, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x2c, 0x20, 0x74, 0x6f, 0x20,
    0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x75, 0x69, 0x73, 0x68, 0x20, 0x72, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x2e, 0x20, 0x20, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xea, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x05, 0x12,
    0x04, 0xea, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xea, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x00, 0x03, 0x12, 0x04, 0xea,
    0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x24, 0x02, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x04,
    0x4a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x04, 0x12, 0x04, 0xeb, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x05, 0x12, 0x04, 0xeb, 0x01, 0x0d, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x14, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x24, 0x02, 0x01, 0x03, 0x12, 0x04, 0xeb, 0x01, 0x29, 0x2a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x24, 0x02, 0x01, 0x08, 0x12, 0x04, 0xeb, 0x01, 0x2b, 0x49, 0x0a, 0x10, 0x0a, 0x08,
    0x04, 0x24, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xeb, 0x01, 0x2c, 0x48, 0x0a, 0x11,
    0x0a, 0x09, 0x04, 0x24, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xeb, 0x01, 0x2c,
    0x40, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x24, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12,
    0x04, 0xeb, 0x01, 0x2c, 0x40, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x24, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xeb, 0x01, 0x2d, 0x3f, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x24,
    0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xeb, 0x01, 0x43, 0x48, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x25, 0x12, 0x06, 0xee, 0x01, 0x00, 0xf3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x25, 0x01, 0x12, 0x04, 0xee, 0x01, 0x08, 0x16, 0x0a, 0x33, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x00,
    0x12, 0x04, 0xf0, 0x01, 0x04, 0x2b, 0x1a, 0x25, 0x20, 0x31, 0x36, 0x20, 0x62, 0x79, 0x74, 0x65,
    0x73, 0x2c, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x75, 0x69, 0x73,
    0x68, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x20, 0x20, 0x0a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x25, 0x02, 0x00, 0x04, 0x12, 0x04, 0xf0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x25, 0x02, 0x00, 0x05, 0x12, 0x04, 0xf0, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x25, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf0, 0x01, 0x13, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25,
    0x02, 0x00, 0x03, 0x12, 0x04, 0xf0, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02,
    0x01, 0x12, 0x04, 0xf1, 0x01, 0x04, 0x4a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x04,
    0x12, 0x04, 0xf1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x05, 0x12,
    0x04, 0xf1, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x01, 0x12, 0x04,
    0xf1, 0x01, 0x14, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf1,
    0x01, 0x29, 0x2a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x01, 0x08, 0x12, 0x04, 0xf1, 0x01,
    0x2b, 0x49, 0x0a, 0x10, 0x0a, 0x08, 0x04, 0x25, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04,
    0xf1, 0x01, 0x2c, 0x48, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x25, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x12, 0x04, 0xf1, 0x01, 0x2c, 0x40, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x25, 0x02, 0x01, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04, 0xf1, 0x01, 0x2c, 0x40, 0x0a, 0x13, 0x0a, 0x0b, 0x04,
    0x25, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf1, 0x01, 0x2d, 0x3f,
    0x0a, 0x11, 0x0a, 0x09, 0x04, 0x25, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xf1,
    0x01, 0x43, 0x48, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x25, 0x02, 0x02, 0x12, 0x04, 0xf2, 0x01, 0x04,
    0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x04, 0x12, 0x04, 0xf2, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x06, 0x12, 0x04, 0xf2, 0x01, 0x0d, 0x12, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x01, 0x12, 0x04, 0xf2, 0x01, 0x13, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x25, 0x02, 0x02, 0x03, 0x12, 0x04, 0xf2, 0x01, 0x29, 0x2a, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x26, 0x12, 0x06, 0xf5, 0x01, 0x00, 0x87, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x26, 0x01, 0x12, 0x04, 0xf5, 0x01, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x00,
    0x12, 0x04, 0xf6, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xf6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x06, 0x12, 0x04,
    0xf6, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x01, 0x12, 0x04, 0xf6,
    0x01, 0x1b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x00, 0x03, 0x12, 0x04, 0xf6, 0x01,
    0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x04, 0x5f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x04, 0x12, 0x04, 0xf7, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x06, 0x12, 0x04, 0xf7, 0x01, 0x0d, 0x18, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x01, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x19, 0x21, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x01, 0x03, 0x12, 0x04, 0xf7, 0x01, 0x3e, 0x3f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x01, 0x08, 0x12, 0x04, 0xf7, 0x01, 0x40, 0x5e, 0x0a, 0x10, 0x0a, 0x08, 0x04,
    0x26, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0xf7, 0x01, 0x41, 0x5d, 0x0a, 0x11, 0x0a,
    0x09, 0x04, 0x26, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12, 0x04, 0xf7, 0x01, 0x41, 0x55,
    0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x26, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x12, 0x04,
    0xf7, 0x01, 0x41, 0x55, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x26, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x04, 0xf7, 0x01, 0x42, 0x54, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x26, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0xf7, 0x01, 0x58, 0x5d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x26, 0x02, 0x02, 0x12, 0x04, 0xf8, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xf8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x02, 0x06, 0x12, 0x04, 0xf8, 0x01, 0x0d, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xf8, 0x01, 0x18, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xf8, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x03, 0x12, 0x04,
    0xf9, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x04, 0x12, 0x04, 0xf9,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x06, 0x12, 0x04, 0xf9, 0x01,
    0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x01, 0x12, 0x04, 0xf9, 0x01, 0x1e,
    0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x03, 0x03, 0x12, 0x04, 0xf9, 0x01, 0x3e, 0x3f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x04, 0x12, 0x04, 0xfa, 0x01, 0x04, 0x40, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x04, 0x04, 0x12, 0x04, 0xfa, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x04, 0x06, 0x12, 0x04, 0xfa, 0x01, 0x0d, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x04, 0x01, 0x12, 0x04, 0xfa, 0x01, 0x23, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x04, 0x03, 0x12, 0x04, 0xfa, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26,
    0x02, 0x05, 0x12, 0x04, 0xfb, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x05,
    0x04, 0x12, 0x04, 0xfb, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x05, 0x06,
    0x12, 0x04, 0xfb, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x05, 0x01, 0x12,
    0x04, 0xfb, 0x01, 0x1c, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x05, 0x03, 0x12, 0x04,
    0xfb, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x06, 0x12, 0x04, 0xfc, 0x01,
    0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x06, 0x04, 0x12, 0x04, 0xfc, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x06, 0x06, 0x12, 0x04, 0xfc, 0x01, 0x0d, 0x1c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x06, 0x01, 0x12, 0x04, 0xfc, 0x01, 0x1d, 0x26, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x06, 0x03, 0x12, 0x04, 0xfc, 0x01, 0x3e, 0x3f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x26, 0x02, 0x07, 0x12, 0x04, 0xfd, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x07, 0x04, 0x12, 0x04, 0xfd, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x07, 0x06, 0x12, 0x04, 0xfd, 0x01, 0x0d, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x07, 0x01, 0x12, 0x04, 0xfd, 0x01, 0x1d, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x07, 0x03, 0x12, 0x04, 0xfd, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x08,
    0x12, 0x04, 0xfe, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x08, 0x04, 0x12,
    0x04, 0xfe, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x08, 0x06, 0x12, 0x04,
    0xfe, 0x01, 0x0d, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x08, 0x01, 0x12, 0x04, 0xfe,
    0x01, 0x1d, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x08, 0x03, 0x12, 0x04, 0xfe, 0x01,
    0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x09, 0x12, 0x04, 0xff, 0x01, 0x04, 0x41,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x09, 0x04, 0x12, 0x04, 0xff, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x09, 0x06, 0x12, 0x04, 0xff, 0x01, 0x0d, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x09, 0x01, 0x12, 0x04, 0xff, 0x01, 0x1e, 0x28, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x09, 0x03, 0x12, 0x04, 0xff, 0x01, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x26, 0x02, 0x0a, 0x12, 0x04, 0x80, 0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x0a, 0x04, 0x12, 0x04, 0x80, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x0a, 0x06, 0x12, 0x04, 0x80, 0x02, 0x0d, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0a,
    0x01, 0x12, 0x04, 0x80, 0x02, 0x24, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0a, 0x03,
    0x12, 0x04, 0x80, 0x02, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x0b, 0x12, 0x04,
    0x81, 0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0b, 0x04, 0x12, 0x04, 0x81,
    0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0b, 0x06, 0x12, 0x04, 0x81, 0x02,
    0x0d, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0b, 0x01, 0x12, 0x04, 0x81, 0x02, 0x25,
    0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0b, 0x03, 0x12, 0x04, 0x81, 0x02, 0x3e, 0x40,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x0c, 0x12, 0x04, 0x82, 0x02, 0x04, 0x41, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x26, 0x02, 0x0c, 0x04, 0x12, 0x04, 0x82, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x26, 0x02, 0x0c, 0x06, 0x12, 0x04, 0x82, 0x02, 0x0d, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x82, 0x02, 0x25, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x0c, 0x03, 0x12, 0x04, 0x82, 0x02, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26,
    0x02, 0x0d, 0x12, 0x04, 0x83, 0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0d,
    0x04, 0x12, 0x04, 0x83, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0d, 0x06,
    0x12, 0x04, 0x83, 0x02, 0x0d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0d, 0x01, 0x12,
    0x04, 0x83, 0x02, 0x23, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0d, 0x03, 0x12, 0x04,
    0x83, 0x02, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x0e, 0x12, 0x04, 0x84, 0x02,
    0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0e, 0x04, 0x12, 0x04, 0x84, 0x02, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0e, 0x06, 0x12, 0x04, 0x84, 0x02, 0x0d, 0x1f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0e, 0x01, 0x12, 0x04, 0x84, 0x02, 0x20, 0x2c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x0e, 0x03, 0x12, 0x04, 0x84, 0x02, 0x3e, 0x40, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x26, 0x02, 0x0f, 0x12, 0x04, 0x85, 0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x26, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x85, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x26, 0x02, 0x0f, 0x06, 0x12, 0x04, 0x85, 0x02, 0x0d, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26,
    0x02, 0x0f, 0x01, 0x12, 0x04, 0x85, 0x02, 0x22, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02,
    0x0f, 0x03, 0x12, 0x04, 0x85, 0x02, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x26, 0x02, 0x10,
    0x12, 0x04, 0x86, 0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x10, 0x04, 0x12,
    0x04, 0x86, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x10, 0x06, 0x12, 0x04,
    0x86, 0x02, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x10, 0x01, 0x12, 0x04, 0x86,
    0x02, 0x21, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x26, 0x02, 0x10, 0x03, 0x12, 0x04, 0x86, 0x02,
    0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x27, 0x12, 0x06, 0x89, 0x02, 0x00, 0x9b, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x27, 0x01, 0x12, 0x04, 0x89, 0x02, 0x08, 0x10, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x27, 0x02, 0x00, 0x12, 0x04, 0x8a, 0x02, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8a, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x00, 0x06, 0x12, 0x04, 0x8a, 0x02, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x8a, 0x02, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x8a, 0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x01, 0x12,
    0x04, 0x8b, 0x02, 0x04, 0x5f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x8b, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x06, 0x12, 0x04, 0x8b,
    0x02, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x02,
    0x19, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x3e,
    0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x01, 0x08, 0x12, 0x04, 0x8b, 0x02, 0x40, 0x5e,
    0x0a, 0x10, 0x0a, 0x08, 0x04, 0x27, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x04, 0x8b, 0x02,
    0x41, 0x5d, 0x0a, 0x11, 0x0a, 0x09, 0x04, 0x27, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x12,
    0x04, 0x8b, 0x02, 0x41, 0x55, 0x0a, 0x12, 0x0a, 0x0a, 0x04, 0x27, 0x02, 0x01, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x04, 0x8b, 0x02, 0x41, 0x55, 0x0a, 0x13, 0x0a, 0x0b, 0x04, 0x27, 0x02,
    0x01, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x02, 0x42, 0x54, 0x0a, 0x11,
    0x0a, 0x09, 0x04, 0x27, 0x02, 0x01, 0x08, 0xe7, 0x07, 0x00, 0x03, 0x12, 0x04, 0x8b, 0x02, 0x58,
    0x5d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x02, 0x12, 0x04, 0x8c, 0x02, 0x04, 0x40, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8c, 0x02, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x02, 0x06, 0x12, 0x04, 0x8c, 0x02, 0x0d, 0x18, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x02, 0x19, 0x1c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8c, 0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x27, 0x02, 0x03, 0x12, 0x04, 0x8d, 0x02, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x8d, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x03,
    0x06, 0x12, 0x04, 0x8d, 0x02, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x03, 0x01,
    0x12, 0x04, 0x8d, 0x02, 0x1f, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x03, 0x03, 0x12,
    0x04, 0x8d, 0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x04, 0x12, 0x04, 0x8e,
    0x02, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x04, 0x04, 0x12, 0x04, 0x8e, 0x02,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x04, 0x06, 0x12, 0x04, 0x8e, 0x02, 0x0d,
    0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8e, 0x02, 0x24, 0x33,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x04, 0x03, 0x12, 0x04, 0x8e, 0x02, 0x3e, 0x3f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x05, 0x12, 0x04, 0x8f, 0x02, 0x04, 0x40, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x05, 0x04, 0x12, 0x04, 0x8f, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x05, 0x06, 0x12, 0x04, 0x8f, 0x02, 0x0d, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x05, 0x01, 0x12, 0x04, 0x8f, 0x02, 0x1d, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x05, 0x03, 0x12, 0x04, 0x8f, 0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02,
    0x06, 0x12, 0x04, 0x90, 0x02, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x90, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x06, 0x06, 0x12,
    0x04, 0x90, 0x02, 0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x06, 0x01, 0x12, 0x04,
    0x90, 0x02, 0x1e, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x06, 0x03, 0x12, 0x04, 0x90,
    0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x07, 0x12, 0x04, 0x91, 0x02, 0x04,
    0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x07, 0x04, 0x12, 0x04, 0x91, 0x02, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x07, 0x06, 0x12, 0x04, 0x91, 0x02, 0x0d, 0x1d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x07, 0x01, 0x12, 0x04, 0x91, 0x02, 0x1e, 0x27, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x07, 0x03, 0x12, 0x04, 0x91, 0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x27, 0x02, 0x08, 0x12, 0x04, 0x92, 0x02, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x08, 0x04, 0x12, 0x04, 0x92, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x08, 0x06, 0x12, 0x04, 0x92, 0x02, 0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x08, 0x01, 0x12, 0x04, 0x92, 0x02, 0x1e, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x08,
    0x03, 0x12, 0x04, 0x92, 0x02, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x09, 0x12,
    0x04, 0x93, 0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x09, 0x04, 0x12, 0x04,
    0x93, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x09, 0x06, 0x12, 0x04, 0x93,
    0x02, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x09, 0x01, 0x12, 0x04, 0x93, 0x02,
    0x1f, 0x29, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x09, 0x03, 0x12, 0x04, 0x93, 0x02, 0x3e,
    0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x0a, 0x12, 0x04, 0x94, 0x02, 0x04, 0x41, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x94, 0x02, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x94, 0x02, 0x0d, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x94, 0x02, 0x25, 0x35, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x94, 0x02, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x27, 0x02, 0x0b, 0x12, 0x04, 0x95, 0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x0b, 0x04, 0x12, 0x04, 0x95, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0b,
    0x06, 0x12, 0x04, 0x95, 0x02, 0x0d, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0b, 0x01,
    0x12, 0x04, 0x95, 0x02, 0x26, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0b, 0x03, 0x12,
    0x04, 0x95, 0x02, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x0c, 0x12, 0x04, 0x96,
    0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0c, 0x04, 0x12, 0x04, 0x96, 0x02,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0c, 0x06, 0x12, 0x04, 0x96, 0x02, 0x0d,
    0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x96, 0x02, 0x26, 0x38,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0c, 0x03, 0x12, 0x04, 0x96, 0x02, 0x3e, 0x40, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x0d, 0x12, 0x04, 0x97, 0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x27, 0x02, 0x0d, 0x04, 0x12, 0x04, 0x97, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x27, 0x02, 0x0d, 0x06, 0x12, 0x04, 0x97, 0x02, 0x0d, 0x23, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x0d, 0x01, 0x12, 0x04, 0x97, 0x02, 0x24, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x0d, 0x03, 0x12, 0x04, 0x97, 0x02, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02,
    0x0e, 0x12, 0x04, 0x98, 0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0e, 0x04,
    0x12, 0x04, 0x98, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0e, 0x06, 0x12,
    0x04, 0x98, 0x02, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0e, 0x01, 0x12, 0x04,
    0x98, 0x02, 0x21, 0x2d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0e, 0x03, 0x12, 0x04, 0x98,
    0x02, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x27, 0x02, 0x0f, 0x12, 0x04, 0x99, 0x02, 0x04,
    0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0f, 0x04, 0x12, 0x04, 0x99, 0x02, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0f, 0x06, 0x12, 0x04, 0x99, 0x02, 0x0d, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x0f, 0x01, 0x12, 0x04, 0x99, 0x02, 0x1f, 0x2f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x27, 0x02, 0x0f, 0x03, 0x12, 0x04, 0x99, 0x02, 0x3e, 0x40, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x27, 0x02, 0x10, 0x12, 0x04, 0x9a, 0x02, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x27, 0x02, 0x10, 0x04, 0x12, 0x04, 0x9a, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27,
    0x02, 0x10, 0x06, 0x12, 0x04, 0x9a, 0x02, 0x0d, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02,
    0x10, 0x01, 0x12, 0x04, 0x9a, 0x02, 0x22, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x27, 0x02, 0x10,
    0x03, 0x12, 0x04, 0x9a, 0x02, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x28, 0x12, 0x06, 0x9d,
    0x02, 0x00, 0x9f, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x28, 0x01, 0x12, 0x04, 0x9d, 0x02,
    0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x29, 0x12, 0x06, 0xa1, 0x02, 0x00, 0xa2, 0x02, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x29, 0x01, 0x12, 0x04, 0xa1, 0x02, 0x08, 0x1d, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x2a, 0x12, 0x06, 0xa4, 0x02, 0x00, 0xa8, 0x02, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x2a, 0x01, 0x12, 0x04, 0xa4, 0x02, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a, 0x02, 0x00,
    0x12, 0x04, 0xa5, 0x02, 0x04, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x04, 0x12,
    0x04, 0xa5, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x05, 0x12, 0x04,
    0xa5, 0x02, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x01, 0x12, 0x04, 0xa5,
    0x02, 0x14, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa5, 0x02,
    0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x2a, 0x02, 0x01, 0x12, 0x04, 0xa6, 0x02, 0x04, 0x34,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa6, 0x02, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa6, 0x02, 0x0d, 0x1e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x2a, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa6, 0x02, 0x23, 0x2f, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x2a, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa6, 0x02, 0x32, 0x33, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x2a, 0x02, 0x02, 0x12, 0x04, 0xa7, 0x02, 0x04, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a,
    0x02, 0x02, 0x04, 0x12, 0x04, 0xa7, 0x02, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02,
    0x02, 0x06, 0x12, 0x04, 0xa7, 0x02, 0x0d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x02,
    0x01, 0x12, 0x04, 0xa7, 0x02, 0x23, 0x2f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x2a, 0x02, 0x02, 0x03,
    0x12, 0x04, 0xa7, 0x02, 0x32, 0x33,
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
