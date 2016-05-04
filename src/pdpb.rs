// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct Leader {
    // message fields
    addr: ::protobuf::SingularField<::std::string::String>,
    pid: ::std::option::Option<i64>,
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
                    pid: ::std::option::Option::None,
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
    pub fn mut_addr<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.addr.is_none() {
            self.addr.set_default();
        };
        self.addr.as_mut().unwrap()
    }

    // Take field
    pub fn take_addr(&mut self) -> ::std::string::String {
        self.addr.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_addr<'a>(&'a self) -> &'a str {
        match self.addr.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional int64 pid = 2;

    pub fn clear_pid(&mut self) {
        self.pid = ::std::option::Option::None;
    }

    pub fn has_pid(&self) -> bool {
        self.pid.is_some()
    }

    // Param is passed by value, moved
    pub fn set_pid(&mut self, v: i64) {
        self.pid = ::std::option::Option::Some(v);
    }

    pub fn get_pid<'a>(&self) -> i64 {
        self.pid.unwrap_or(0)
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
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.pid = ::std::option::Option::Some(tmp);
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
        for value in self.addr.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.pid.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.addr.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.pid {
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
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "pid",
                    Leader::has_pid,
                    Leader::get_pid,
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
        self.clear_pid();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Leader {
    fn eq(&self, other: &Leader) -> bool {
        self.addr == other.addr &&
        self.pid == other.pid &&
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
    number: ::std::option::Option<u32>,
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
                    number: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint32 number = 1;

    pub fn clear_number(&mut self) {
        self.number = ::std::option::Option::None;
    }

    pub fn has_number(&self) -> bool {
        self.number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_number(&mut self, v: u32) {
        self.number = ::std::option::Option::Some(v);
    }

    pub fn get_number<'a>(&self) -> u32 {
        self.number.unwrap_or(0)
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
                    self.number = ::std::option::Option::Some(tmp);
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
        for value in self.number.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.number {
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
                    "number",
                    TsoRequest::has_number,
                    TsoRequest::get_number,
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
        self.clear_number();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsoRequest {
    fn eq(&self, other: &TsoRequest) -> bool {
        self.number == other.number &&
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

    pub fn get_physical<'a>(&self) -> i64 {
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

    pub fn get_logical<'a>(&self) -> i64 {
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
        for value in self.physical.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.logical.iter() {
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
    timestamps: ::protobuf::RepeatedField<Timestamp>,
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
                    timestamps: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .pdpb.Timestamp timestamps = 1;

    pub fn clear_timestamps(&mut self) {
        self.timestamps.clear();
    }

    // Param is passed by value, moved
    pub fn set_timestamps(&mut self, v: ::protobuf::RepeatedField<Timestamp>) {
        self.timestamps = v;
    }

    // Mutable pointer to the field.
    pub fn mut_timestamps<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Timestamp> {
        &mut self.timestamps
    }

    // Take field
    pub fn take_timestamps(&mut self) -> ::protobuf::RepeatedField<Timestamp> {
        ::std::mem::replace(&mut self.timestamps, ::protobuf::RepeatedField::new())
    }

    pub fn get_timestamps<'a>(&'a self) -> &'a [Timestamp] {
        &self.timestamps
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
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.timestamps));
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
        for value in self.timestamps.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.timestamps.iter() {
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
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "timestamps",
                    TsoResponse::get_timestamps,
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
        self.clear_timestamps();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for TsoResponse {
    fn eq(&self, other: &TsoResponse) -> bool {
        self.timestamps == other.timestamps &&
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
    pub fn mut_store<'a>(&'a mut self) -> &'a mut super::metapb::Store {
        if self.store.is_none() {
            self.store.set_default();
        };
        self.store.as_mut().unwrap()
    }

    // Take field
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store.take().unwrap_or_else(|| super::metapb::Store::new())
    }

    pub fn get_store<'a>(&'a self) -> &'a super::metapb::Store {
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
    pub fn mut_region<'a>(&'a mut self) -> &'a mut super::metapb::Region {
        if self.region.is_none() {
            self.region.set_default();
        };
        self.region.as_mut().unwrap()
    }

    // Take field
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region.take().unwrap_or_else(|| super::metapb::Region::new())
    }

    pub fn get_region<'a>(&'a self) -> &'a super::metapb::Region {
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
        for value in self.store.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.region.iter() {
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

    pub fn get_bootstrapped<'a>(&self) -> bool {
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

    pub fn get_id<'a>(&self) -> u64 {
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
        for value in self.id.iter() {
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

    pub fn get_store_id<'a>(&self) -> u64 {
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
        for value in self.store_id.iter() {
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
    pub fn mut_store<'a>(&'a mut self) -> &'a mut super::metapb::Store {
        if self.store.is_none() {
            self.store.set_default();
        };
        self.store.as_mut().unwrap()
    }

    // Take field
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store.take().unwrap_or_else(|| super::metapb::Store::new())
    }

    pub fn get_store<'a>(&'a self) -> &'a super::metapb::Store {
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
        for value in self.store.iter() {
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
    pub fn mut_store<'a>(&'a mut self) -> &'a mut super::metapb::Store {
        if self.store.is_none() {
            self.store.set_default();
        };
        self.store.as_mut().unwrap()
    }

    // Take field
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store.take().unwrap_or_else(|| super::metapb::Store::new())
    }

    pub fn get_store<'a>(&'a self) -> &'a super::metapb::Store {
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
        for value in self.store.iter() {
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
    pub fn mut_region_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.region_key.is_none() {
            self.region_key.set_default();
        };
        self.region_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_key(&mut self) -> ::std::vec::Vec<u8> {
        self.region_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_region_key<'a>(&'a self) -> &'a [u8] {
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
        for value in self.region_key.iter() {
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
    pub fn mut_region<'a>(&'a mut self) -> &'a mut super::metapb::Region {
        if self.region.is_none() {
            self.region.set_default();
        };
        self.region.as_mut().unwrap()
    }

    // Take field
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region.take().unwrap_or_else(|| super::metapb::Region::new())
    }

    pub fn get_region<'a>(&'a self) -> &'a super::metapb::Region {
        self.region.as_ref().unwrap_or_else(|| super::metapb::Region::default_instance())
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
        for value in self.region.iter() {
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetRegionResponse {
    fn eq(&self, other: &GetRegionResponse) -> bool {
        self.region == other.region &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for GetRegionResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct GetClusterConfigRequest {
    // message fields
    cluster_name: ::protobuf::SingularField<::std::string::String>,
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
                    cluster_name: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional string cluster_name = 1;

    pub fn clear_cluster_name(&mut self) {
        self.cluster_name.clear();
    }

    pub fn has_cluster_name(&self) -> bool {
        self.cluster_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_name(&mut self, v: ::std::string::String) {
        self.cluster_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.cluster_name.is_none() {
            self.cluster_name.set_default();
        };
        self.cluster_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster_name(&mut self) -> ::std::string::String {
        self.cluster_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cluster_name<'a>(&'a self) -> &'a str {
        match self.cluster_name.as_ref() {
            Some(v) => &v,
            None => "",
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
                1 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cluster_name));
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
        for value in self.cluster_name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.cluster_name.as_ref() {
            try!(os.write_string(1, &v));
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
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cluster_name",
                    GetClusterConfigRequest::has_cluster_name,
                    GetClusterConfigRequest::get_cluster_name,
                ));
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
        self.clear_cluster_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for GetClusterConfigRequest {
    fn eq(&self, other: &GetClusterConfigRequest) -> bool {
        self.cluster_name == other.cluster_name &&
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
    pub fn mut_cluster<'a>(&'a mut self) -> &'a mut super::metapb::Cluster {
        if self.cluster.is_none() {
            self.cluster.set_default();
        };
        self.cluster.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster(&mut self) -> super::metapb::Cluster {
        self.cluster.take().unwrap_or_else(|| super::metapb::Cluster::new())
    }

    pub fn get_cluster<'a>(&'a self) -> &'a super::metapb::Cluster {
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
        for value in self.cluster.iter() {
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
    pub fn mut_cluster<'a>(&'a mut self) -> &'a mut super::metapb::Cluster {
        if self.cluster.is_none() {
            self.cluster.set_default();
        };
        self.cluster.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster(&mut self) -> super::metapb::Cluster {
        self.cluster.take().unwrap_or_else(|| super::metapb::Cluster::new())
    }

    pub fn get_cluster<'a>(&'a self) -> &'a super::metapb::Cluster {
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
        for value in self.cluster.iter() {
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
pub struct AskChangePeerRequest {
    // message fields
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    leader_store_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AskChangePeerRequest {}

impl AskChangePeerRequest {
    pub fn new() -> AskChangePeerRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AskChangePeerRequest {
        static mut instance: ::protobuf::lazy::Lazy<AskChangePeerRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AskChangePeerRequest,
        };
        unsafe {
            instance.get(|| {
                AskChangePeerRequest {
                    region: ::protobuf::SingularPtrField::none(),
                    leader_store_id: ::std::option::Option::None,
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
    pub fn mut_region<'a>(&'a mut self) -> &'a mut super::metapb::Region {
        if self.region.is_none() {
            self.region.set_default();
        };
        self.region.as_mut().unwrap()
    }

    // Take field
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region.take().unwrap_or_else(|| super::metapb::Region::new())
    }

    pub fn get_region<'a>(&'a self) -> &'a super::metapb::Region {
        self.region.as_ref().unwrap_or_else(|| super::metapb::Region::default_instance())
    }

    // optional uint64 leader_store_id = 2;

    pub fn clear_leader_store_id(&mut self) {
        self.leader_store_id = ::std::option::Option::None;
    }

    pub fn has_leader_store_id(&self) -> bool {
        self.leader_store_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader_store_id(&mut self, v: u64) {
        self.leader_store_id = ::std::option::Option::Some(v);
    }

    pub fn get_leader_store_id<'a>(&self) -> u64 {
        self.leader_store_id.unwrap_or(0)
    }
}

impl ::protobuf::Message for AskChangePeerRequest {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.leader_store_id = ::std::option::Option::Some(tmp);
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
        for value in self.region.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.leader_store_id.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.leader_store_id {
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
        ::std::any::TypeId::of::<AskChangePeerRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AskChangePeerRequest {
    fn new() -> AskChangePeerRequest {
        AskChangePeerRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<AskChangePeerRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region",
                    AskChangePeerRequest::has_region,
                    AskChangePeerRequest::get_region,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "leader_store_id",
                    AskChangePeerRequest::has_leader_store_id,
                    AskChangePeerRequest::get_leader_store_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<AskChangePeerRequest>(
                    "AskChangePeerRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AskChangePeerRequest {
    fn clear(&mut self) {
        self.clear_region();
        self.clear_leader_store_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AskChangePeerRequest {
    fn eq(&self, other: &AskChangePeerRequest) -> bool {
        self.region == other.region &&
        self.leader_store_id == other.leader_store_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AskChangePeerRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AskChangePeerResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for AskChangePeerResponse {}

impl AskChangePeerResponse {
    pub fn new() -> AskChangePeerResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static AskChangePeerResponse {
        static mut instance: ::protobuf::lazy::Lazy<AskChangePeerResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const AskChangePeerResponse,
        };
        unsafe {
            instance.get(|| {
                AskChangePeerResponse {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for AskChangePeerResponse {
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
        ::std::any::TypeId::of::<AskChangePeerResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for AskChangePeerResponse {
    fn new() -> AskChangePeerResponse {
        AskChangePeerResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<AskChangePeerResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<AskChangePeerResponse>(
                    "AskChangePeerResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for AskChangePeerResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AskChangePeerResponse {
    fn eq(&self, other: &AskChangePeerResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AskChangePeerResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct AskSplitRequest {
    // message fields
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    split_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    leader_store_id: ::std::option::Option<u64>,
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
                    split_key: ::protobuf::SingularField::none(),
                    leader_store_id: ::std::option::Option::None,
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
    pub fn mut_region<'a>(&'a mut self) -> &'a mut super::metapb::Region {
        if self.region.is_none() {
            self.region.set_default();
        };
        self.region.as_mut().unwrap()
    }

    // Take field
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region.take().unwrap_or_else(|| super::metapb::Region::new())
    }

    pub fn get_region<'a>(&'a self) -> &'a super::metapb::Region {
        self.region.as_ref().unwrap_or_else(|| super::metapb::Region::default_instance())
    }

    // optional bytes split_key = 2;

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
    pub fn mut_split_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.split_key.is_none() {
            self.split_key.set_default();
        };
        self.split_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_split_key(&mut self) -> ::std::vec::Vec<u8> {
        self.split_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_split_key<'a>(&'a self) -> &'a [u8] {
        match self.split_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 leader_store_id = 3;

    pub fn clear_leader_store_id(&mut self) {
        self.leader_store_id = ::std::option::Option::None;
    }

    pub fn has_leader_store_id(&self) -> bool {
        self.leader_store_id.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader_store_id(&mut self, v: u64) {
        self.leader_store_id = ::std::option::Option::Some(v);
    }

    pub fn get_leader_store_id<'a>(&self) -> u64 {
        self.leader_store_id.unwrap_or(0)
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
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.split_key));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.leader_store_id = ::std::option::Option::Some(tmp);
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
        for value in self.region.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.split_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.leader_store_id.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.split_key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.leader_store_id {
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "split_key",
                    AskSplitRequest::has_split_key,
                    AskSplitRequest::get_split_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "leader_store_id",
                    AskSplitRequest::has_leader_store_id,
                    AskSplitRequest::get_leader_store_id,
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
        self.clear_split_key();
        self.clear_leader_store_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AskSplitRequest {
    fn eq(&self, other: &AskSplitRequest) -> bool {
        self.region == other.region &&
        self.split_key == other.split_key &&
        self.leader_store_id == other.leader_store_id &&
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
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
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
                let fields = ::std::vec::Vec::new();
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for AskSplitResponse {
    fn eq(&self, other: &AskSplitResponse) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for AskSplitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RequestHeader {
    // message fields
    uuid: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    cluster_name: ::protobuf::SingularField<::std::string::String>,
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
                    cluster_name: ::protobuf::SingularField::none(),
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
    pub fn mut_uuid<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        };
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid<'a>(&'a self) -> &'a [u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string cluster_name = 2;

    pub fn clear_cluster_name(&mut self) {
        self.cluster_name.clear();
    }

    pub fn has_cluster_name(&self) -> bool {
        self.cluster_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_name(&mut self, v: ::std::string::String) {
        self.cluster_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.cluster_name.is_none() {
            self.cluster_name.set_default();
        };
        self.cluster_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster_name(&mut self) -> ::std::string::String {
        self.cluster_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cluster_name<'a>(&'a self) -> &'a str {
        match self.cluster_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cluster_name));
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
        for value in self.uuid.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.cluster_name.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.uuid.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.cluster_name.as_ref() {
            try!(os.write_string(2, &v));
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cluster_name",
                    RequestHeader::has_cluster_name,
                    RequestHeader::get_cluster_name,
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
        self.clear_cluster_name();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RequestHeader {
    fn eq(&self, other: &RequestHeader) -> bool {
        self.uuid == other.uuid &&
        self.cluster_name == other.cluster_name &&
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
    cluster_name: ::protobuf::SingularField<::std::string::String>,
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
                    cluster_name: ::protobuf::SingularField::none(),
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
    pub fn mut_uuid<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.uuid.is_none() {
            self.uuid.set_default();
        };
        self.uuid.as_mut().unwrap()
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        self.uuid.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_uuid<'a>(&'a self) -> &'a [u8] {
        match self.uuid.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional string cluster_name = 2;

    pub fn clear_cluster_name(&mut self) {
        self.cluster_name.clear();
    }

    pub fn has_cluster_name(&self) -> bool {
        self.cluster_name.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cluster_name(&mut self, v: ::std::string::String) {
        self.cluster_name = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cluster_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.cluster_name.is_none() {
            self.cluster_name.set_default();
        };
        self.cluster_name.as_mut().unwrap()
    }

    // Take field
    pub fn take_cluster_name(&mut self) -> ::std::string::String {
        self.cluster_name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_cluster_name<'a>(&'a self) -> &'a str {
        match self.cluster_name.as_ref() {
            Some(v) => &v,
            None => "",
        }
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
    pub fn mut_error<'a>(&'a mut self) -> &'a mut Error {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(|| Error::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a Error {
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
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.cluster_name));
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
        for value in self.uuid.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.cluster_name.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.error.iter() {
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
        if let Some(v) = self.cluster_name.as_ref() {
            try!(os.write_string(2, &v));
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
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "cluster_name",
                    ResponseHeader::has_cluster_name,
                    ResponseHeader::get_cluster_name,
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
        self.clear_cluster_name();
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ResponseHeader {
    fn eq(&self, other: &ResponseHeader) -> bool {
        self.uuid == other.uuid &&
        self.cluster_name == other.cluster_name &&
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
    ask_change_peer: ::protobuf::SingularPtrField<AskChangePeerRequest>,
    ask_split: ::protobuf::SingularPtrField<AskSplitRequest>,
    get_store: ::protobuf::SingularPtrField<GetStoreRequest>,
    get_region: ::protobuf::SingularPtrField<GetRegionRequest>,
    get_cluster_config: ::protobuf::SingularPtrField<GetClusterConfigRequest>,
    put_store: ::protobuf::SingularPtrField<PutStoreRequest>,
    put_cluster_config: ::protobuf::SingularPtrField<PutClusterConfigRequest>,
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
                    ask_change_peer: ::protobuf::SingularPtrField::none(),
                    ask_split: ::protobuf::SingularPtrField::none(),
                    get_store: ::protobuf::SingularPtrField::none(),
                    get_region: ::protobuf::SingularPtrField::none(),
                    get_cluster_config: ::protobuf::SingularPtrField::none(),
                    put_store: ::protobuf::SingularPtrField::none(),
                    put_cluster_config: ::protobuf::SingularPtrField::none(),
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
    pub fn mut_header<'a>(&'a mut self) -> &'a mut RequestHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(|| RequestHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a RequestHeader {
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

    pub fn get_cmd_type<'a>(&self) -> CommandType {
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
    pub fn mut_tso<'a>(&'a mut self) -> &'a mut TsoRequest {
        if self.tso.is_none() {
            self.tso.set_default();
        };
        self.tso.as_mut().unwrap()
    }

    // Take field
    pub fn take_tso(&mut self) -> TsoRequest {
        self.tso.take().unwrap_or_else(|| TsoRequest::new())
    }

    pub fn get_tso<'a>(&'a self) -> &'a TsoRequest {
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
    pub fn mut_bootstrap<'a>(&'a mut self) -> &'a mut BootstrapRequest {
        if self.bootstrap.is_none() {
            self.bootstrap.set_default();
        };
        self.bootstrap.as_mut().unwrap()
    }

    // Take field
    pub fn take_bootstrap(&mut self) -> BootstrapRequest {
        self.bootstrap.take().unwrap_or_else(|| BootstrapRequest::new())
    }

    pub fn get_bootstrap<'a>(&'a self) -> &'a BootstrapRequest {
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
    pub fn mut_is_bootstrapped<'a>(&'a mut self) -> &'a mut IsBootstrappedRequest {
        if self.is_bootstrapped.is_none() {
            self.is_bootstrapped.set_default();
        };
        self.is_bootstrapped.as_mut().unwrap()
    }

    // Take field
    pub fn take_is_bootstrapped(&mut self) -> IsBootstrappedRequest {
        self.is_bootstrapped.take().unwrap_or_else(|| IsBootstrappedRequest::new())
    }

    pub fn get_is_bootstrapped<'a>(&'a self) -> &'a IsBootstrappedRequest {
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
    pub fn mut_alloc_id<'a>(&'a mut self) -> &'a mut AllocIdRequest {
        if self.alloc_id.is_none() {
            self.alloc_id.set_default();
        };
        self.alloc_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_alloc_id(&mut self) -> AllocIdRequest {
        self.alloc_id.take().unwrap_or_else(|| AllocIdRequest::new())
    }

    pub fn get_alloc_id<'a>(&'a self) -> &'a AllocIdRequest {
        self.alloc_id.as_ref().unwrap_or_else(|| AllocIdRequest::default_instance())
    }

    // optional .pdpb.AskChangePeerRequest ask_change_peer = 7;

    pub fn clear_ask_change_peer(&mut self) {
        self.ask_change_peer.clear();
    }

    pub fn has_ask_change_peer(&self) -> bool {
        self.ask_change_peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ask_change_peer(&mut self, v: AskChangePeerRequest) {
        self.ask_change_peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ask_change_peer<'a>(&'a mut self) -> &'a mut AskChangePeerRequest {
        if self.ask_change_peer.is_none() {
            self.ask_change_peer.set_default();
        };
        self.ask_change_peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_ask_change_peer(&mut self) -> AskChangePeerRequest {
        self.ask_change_peer.take().unwrap_or_else(|| AskChangePeerRequest::new())
    }

    pub fn get_ask_change_peer<'a>(&'a self) -> &'a AskChangePeerRequest {
        self.ask_change_peer.as_ref().unwrap_or_else(|| AskChangePeerRequest::default_instance())
    }

    // optional .pdpb.AskSplitRequest ask_split = 8;

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
    pub fn mut_ask_split<'a>(&'a mut self) -> &'a mut AskSplitRequest {
        if self.ask_split.is_none() {
            self.ask_split.set_default();
        };
        self.ask_split.as_mut().unwrap()
    }

    // Take field
    pub fn take_ask_split(&mut self) -> AskSplitRequest {
        self.ask_split.take().unwrap_or_else(|| AskSplitRequest::new())
    }

    pub fn get_ask_split<'a>(&'a self) -> &'a AskSplitRequest {
        self.ask_split.as_ref().unwrap_or_else(|| AskSplitRequest::default_instance())
    }

    // optional .pdpb.GetStoreRequest get_store = 9;

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
    pub fn mut_get_store<'a>(&'a mut self) -> &'a mut GetStoreRequest {
        if self.get_store.is_none() {
            self.get_store.set_default();
        };
        self.get_store.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_store(&mut self) -> GetStoreRequest {
        self.get_store.take().unwrap_or_else(|| GetStoreRequest::new())
    }

    pub fn get_get_store<'a>(&'a self) -> &'a GetStoreRequest {
        self.get_store.as_ref().unwrap_or_else(|| GetStoreRequest::default_instance())
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
    pub fn mut_get_region<'a>(&'a mut self) -> &'a mut GetRegionRequest {
        if self.get_region.is_none() {
            self.get_region.set_default();
        };
        self.get_region.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_region(&mut self) -> GetRegionRequest {
        self.get_region.take().unwrap_or_else(|| GetRegionRequest::new())
    }

    pub fn get_get_region<'a>(&'a self) -> &'a GetRegionRequest {
        self.get_region.as_ref().unwrap_or_else(|| GetRegionRequest::default_instance())
    }

    // optional .pdpb.GetClusterConfigRequest get_cluster_config = 11;

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
    pub fn mut_get_cluster_config<'a>(&'a mut self) -> &'a mut GetClusterConfigRequest {
        if self.get_cluster_config.is_none() {
            self.get_cluster_config.set_default();
        };
        self.get_cluster_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_cluster_config(&mut self) -> GetClusterConfigRequest {
        self.get_cluster_config.take().unwrap_or_else(|| GetClusterConfigRequest::new())
    }

    pub fn get_get_cluster_config<'a>(&'a self) -> &'a GetClusterConfigRequest {
        self.get_cluster_config.as_ref().unwrap_or_else(|| GetClusterConfigRequest::default_instance())
    }

    // optional .pdpb.PutStoreRequest put_store = 12;

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
    pub fn mut_put_store<'a>(&'a mut self) -> &'a mut PutStoreRequest {
        if self.put_store.is_none() {
            self.put_store.set_default();
        };
        self.put_store.as_mut().unwrap()
    }

    // Take field
    pub fn take_put_store(&mut self) -> PutStoreRequest {
        self.put_store.take().unwrap_or_else(|| PutStoreRequest::new())
    }

    pub fn get_put_store<'a>(&'a self) -> &'a PutStoreRequest {
        self.put_store.as_ref().unwrap_or_else(|| PutStoreRequest::default_instance())
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
    pub fn mut_put_cluster_config<'a>(&'a mut self) -> &'a mut PutClusterConfigRequest {
        if self.put_cluster_config.is_none() {
            self.put_cluster_config.set_default();
        };
        self.put_cluster_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_put_cluster_config(&mut self) -> PutClusterConfigRequest {
        self.put_cluster_config.take().unwrap_or_else(|| PutClusterConfigRequest::new())
    }

    pub fn get_put_cluster_config<'a>(&'a self) -> &'a PutClusterConfigRequest {
        self.put_cluster_config.as_ref().unwrap_or_else(|| PutClusterConfigRequest::default_instance())
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ask_change_peer));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ask_split));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_store));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_region));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_cluster_config));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put_store));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put_cluster_config));
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_type.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.tso.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.bootstrap.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.is_bootstrapped.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.alloc_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ask_change_peer.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ask_split.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.get_store.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.get_region.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.get_cluster_config.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.put_store.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.put_cluster_config.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(v) = self.ask_change_peer.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ask_split.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_store.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_region.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_cluster_config.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.put_store.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.put_cluster_config.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                    "ask_change_peer",
                    Request::has_ask_change_peer,
                    Request::get_ask_change_peer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ask_split",
                    Request::has_ask_split,
                    Request::get_ask_split,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_store",
                    Request::has_get_store,
                    Request::get_get_store,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_region",
                    Request::has_get_region,
                    Request::get_get_region,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_cluster_config",
                    Request::has_get_cluster_config,
                    Request::get_get_cluster_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put_store",
                    Request::has_put_store,
                    Request::get_put_store,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put_cluster_config",
                    Request::has_put_cluster_config,
                    Request::get_put_cluster_config,
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
        self.clear_ask_change_peer();
        self.clear_ask_split();
        self.clear_get_store();
        self.clear_get_region();
        self.clear_get_cluster_config();
        self.clear_put_store();
        self.clear_put_cluster_config();
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
        self.ask_change_peer == other.ask_change_peer &&
        self.ask_split == other.ask_split &&
        self.get_store == other.get_store &&
        self.get_region == other.get_region &&
        self.get_cluster_config == other.get_cluster_config &&
        self.put_store == other.put_store &&
        self.put_cluster_config == other.put_cluster_config &&
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
    ask_change_peer: ::protobuf::SingularPtrField<AskChangePeerResponse>,
    ask_split: ::protobuf::SingularPtrField<AskSplitResponse>,
    get_store: ::protobuf::SingularPtrField<GetStoreResponse>,
    get_region: ::protobuf::SingularPtrField<GetRegionResponse>,
    get_cluster_config: ::protobuf::SingularPtrField<GetClusterConfigResponse>,
    put_store: ::protobuf::SingularPtrField<PutStoreResponse>,
    put_cluster_config: ::protobuf::SingularPtrField<PutClusterConfigResponse>,
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
                    ask_change_peer: ::protobuf::SingularPtrField::none(),
                    ask_split: ::protobuf::SingularPtrField::none(),
                    get_store: ::protobuf::SingularPtrField::none(),
                    get_region: ::protobuf::SingularPtrField::none(),
                    get_cluster_config: ::protobuf::SingularPtrField::none(),
                    put_store: ::protobuf::SingularPtrField::none(),
                    put_cluster_config: ::protobuf::SingularPtrField::none(),
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
    pub fn mut_header<'a>(&'a mut self) -> &'a mut ResponseHeader {
        if self.header.is_none() {
            self.header.set_default();
        };
        self.header.as_mut().unwrap()
    }

    // Take field
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(|| ResponseHeader::new())
    }

    pub fn get_header<'a>(&'a self) -> &'a ResponseHeader {
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

    pub fn get_cmd_type<'a>(&self) -> CommandType {
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
    pub fn mut_tso<'a>(&'a mut self) -> &'a mut TsoResponse {
        if self.tso.is_none() {
            self.tso.set_default();
        };
        self.tso.as_mut().unwrap()
    }

    // Take field
    pub fn take_tso(&mut self) -> TsoResponse {
        self.tso.take().unwrap_or_else(|| TsoResponse::new())
    }

    pub fn get_tso<'a>(&'a self) -> &'a TsoResponse {
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
    pub fn mut_bootstrap<'a>(&'a mut self) -> &'a mut BootstrapResponse {
        if self.bootstrap.is_none() {
            self.bootstrap.set_default();
        };
        self.bootstrap.as_mut().unwrap()
    }

    // Take field
    pub fn take_bootstrap(&mut self) -> BootstrapResponse {
        self.bootstrap.take().unwrap_or_else(|| BootstrapResponse::new())
    }

    pub fn get_bootstrap<'a>(&'a self) -> &'a BootstrapResponse {
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
    pub fn mut_is_bootstrapped<'a>(&'a mut self) -> &'a mut IsBootstrappedResponse {
        if self.is_bootstrapped.is_none() {
            self.is_bootstrapped.set_default();
        };
        self.is_bootstrapped.as_mut().unwrap()
    }

    // Take field
    pub fn take_is_bootstrapped(&mut self) -> IsBootstrappedResponse {
        self.is_bootstrapped.take().unwrap_or_else(|| IsBootstrappedResponse::new())
    }

    pub fn get_is_bootstrapped<'a>(&'a self) -> &'a IsBootstrappedResponse {
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
    pub fn mut_alloc_id<'a>(&'a mut self) -> &'a mut AllocIdResponse {
        if self.alloc_id.is_none() {
            self.alloc_id.set_default();
        };
        self.alloc_id.as_mut().unwrap()
    }

    // Take field
    pub fn take_alloc_id(&mut self) -> AllocIdResponse {
        self.alloc_id.take().unwrap_or_else(|| AllocIdResponse::new())
    }

    pub fn get_alloc_id<'a>(&'a self) -> &'a AllocIdResponse {
        self.alloc_id.as_ref().unwrap_or_else(|| AllocIdResponse::default_instance())
    }

    // optional .pdpb.AskChangePeerResponse ask_change_peer = 7;

    pub fn clear_ask_change_peer(&mut self) {
        self.ask_change_peer.clear();
    }

    pub fn has_ask_change_peer(&self) -> bool {
        self.ask_change_peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ask_change_peer(&mut self, v: AskChangePeerResponse) {
        self.ask_change_peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_ask_change_peer<'a>(&'a mut self) -> &'a mut AskChangePeerResponse {
        if self.ask_change_peer.is_none() {
            self.ask_change_peer.set_default();
        };
        self.ask_change_peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_ask_change_peer(&mut self) -> AskChangePeerResponse {
        self.ask_change_peer.take().unwrap_or_else(|| AskChangePeerResponse::new())
    }

    pub fn get_ask_change_peer<'a>(&'a self) -> &'a AskChangePeerResponse {
        self.ask_change_peer.as_ref().unwrap_or_else(|| AskChangePeerResponse::default_instance())
    }

    // optional .pdpb.AskSplitResponse ask_split = 8;

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
    pub fn mut_ask_split<'a>(&'a mut self) -> &'a mut AskSplitResponse {
        if self.ask_split.is_none() {
            self.ask_split.set_default();
        };
        self.ask_split.as_mut().unwrap()
    }

    // Take field
    pub fn take_ask_split(&mut self) -> AskSplitResponse {
        self.ask_split.take().unwrap_or_else(|| AskSplitResponse::new())
    }

    pub fn get_ask_split<'a>(&'a self) -> &'a AskSplitResponse {
        self.ask_split.as_ref().unwrap_or_else(|| AskSplitResponse::default_instance())
    }

    // optional .pdpb.GetStoreResponse get_store = 9;

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
    pub fn mut_get_store<'a>(&'a mut self) -> &'a mut GetStoreResponse {
        if self.get_store.is_none() {
            self.get_store.set_default();
        };
        self.get_store.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_store(&mut self) -> GetStoreResponse {
        self.get_store.take().unwrap_or_else(|| GetStoreResponse::new())
    }

    pub fn get_get_store<'a>(&'a self) -> &'a GetStoreResponse {
        self.get_store.as_ref().unwrap_or_else(|| GetStoreResponse::default_instance())
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
    pub fn mut_get_region<'a>(&'a mut self) -> &'a mut GetRegionResponse {
        if self.get_region.is_none() {
            self.get_region.set_default();
        };
        self.get_region.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_region(&mut self) -> GetRegionResponse {
        self.get_region.take().unwrap_or_else(|| GetRegionResponse::new())
    }

    pub fn get_get_region<'a>(&'a self) -> &'a GetRegionResponse {
        self.get_region.as_ref().unwrap_or_else(|| GetRegionResponse::default_instance())
    }

    // optional .pdpb.GetClusterConfigResponse get_cluster_config = 11;

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
    pub fn mut_get_cluster_config<'a>(&'a mut self) -> &'a mut GetClusterConfigResponse {
        if self.get_cluster_config.is_none() {
            self.get_cluster_config.set_default();
        };
        self.get_cluster_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_get_cluster_config(&mut self) -> GetClusterConfigResponse {
        self.get_cluster_config.take().unwrap_or_else(|| GetClusterConfigResponse::new())
    }

    pub fn get_get_cluster_config<'a>(&'a self) -> &'a GetClusterConfigResponse {
        self.get_cluster_config.as_ref().unwrap_or_else(|| GetClusterConfigResponse::default_instance())
    }

    // optional .pdpb.PutStoreResponse put_store = 12;

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
    pub fn mut_put_store<'a>(&'a mut self) -> &'a mut PutStoreResponse {
        if self.put_store.is_none() {
            self.put_store.set_default();
        };
        self.put_store.as_mut().unwrap()
    }

    // Take field
    pub fn take_put_store(&mut self) -> PutStoreResponse {
        self.put_store.take().unwrap_or_else(|| PutStoreResponse::new())
    }

    pub fn get_put_store<'a>(&'a self) -> &'a PutStoreResponse {
        self.put_store.as_ref().unwrap_or_else(|| PutStoreResponse::default_instance())
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
    pub fn mut_put_cluster_config<'a>(&'a mut self) -> &'a mut PutClusterConfigResponse {
        if self.put_cluster_config.is_none() {
            self.put_cluster_config.set_default();
        };
        self.put_cluster_config.as_mut().unwrap()
    }

    // Take field
    pub fn take_put_cluster_config(&mut self) -> PutClusterConfigResponse {
        self.put_cluster_config.take().unwrap_or_else(|| PutClusterConfigResponse::new())
    }

    pub fn get_put_cluster_config<'a>(&'a self) -> &'a PutClusterConfigResponse {
        self.put_cluster_config.as_ref().unwrap_or_else(|| PutClusterConfigResponse::default_instance())
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ask_change_peer));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.ask_split));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_store));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_region));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.get_cluster_config));
                },
                12 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put_store));
                },
                13 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.put_cluster_config));
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
        for value in self.header.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_type.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.tso.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.bootstrap.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.is_bootstrapped.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.alloc_id.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ask_change_peer.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ask_split.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.get_store.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.get_region.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.get_cluster_config.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.put_store.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.put_cluster_config.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
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
        if let Some(v) = self.ask_change_peer.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ask_split.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_store.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_region.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.get_cluster_config.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.put_store.as_ref() {
            try!(os.write_tag(12, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.put_cluster_config.as_ref() {
            try!(os.write_tag(13, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                    "ask_change_peer",
                    Response::has_ask_change_peer,
                    Response::get_ask_change_peer,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "ask_split",
                    Response::has_ask_split,
                    Response::get_ask_split,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_store",
                    Response::has_get_store,
                    Response::get_get_store,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_region",
                    Response::has_get_region,
                    Response::get_get_region,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "get_cluster_config",
                    Response::has_get_cluster_config,
                    Response::get_get_cluster_config,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put_store",
                    Response::has_put_store,
                    Response::get_put_store,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "put_cluster_config",
                    Response::has_put_cluster_config,
                    Response::get_put_cluster_config,
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
        self.clear_ask_change_peer();
        self.clear_ask_split();
        self.clear_get_store();
        self.clear_get_region();
        self.clear_get_cluster_config();
        self.clear_put_store();
        self.clear_put_cluster_config();
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
        self.ask_change_peer == other.ask_change_peer &&
        self.ask_split == other.ask_split &&
        self.get_store == other.get_store &&
        self.get_region == other.get_region &&
        self.get_cluster_config == other.get_cluster_config &&
        self.put_store == other.put_store &&
        self.put_cluster_config == other.put_cluster_config &&
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
pub struct Error {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    bootstrapped: ::protobuf::SingularPtrField<BootstrappedError>,
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
    pub fn mut_message<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> ::std::string::String {
        self.message.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_message<'a>(&'a self) -> &'a str {
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
    pub fn mut_bootstrapped<'a>(&'a mut self) -> &'a mut BootstrappedError {
        if self.bootstrapped.is_none() {
            self.bootstrapped.set_default();
        };
        self.bootstrapped.as_mut().unwrap()
    }

    // Take field
    pub fn take_bootstrapped(&mut self) -> BootstrappedError {
        self.bootstrapped.take().unwrap_or_else(|| BootstrappedError::new())
    }

    pub fn get_bootstrapped<'a>(&'a self) -> &'a BootstrappedError {
        self.bootstrapped.as_ref().unwrap_or_else(|| BootstrappedError::default_instance())
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
        for value in self.message.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.bootstrapped.iter() {
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.message == other.message &&
        self.bootstrapped == other.bootstrapped &&
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
    DeleteMeta = 7,
    AskChangePeer = 8,
    AskSplit = 9,
    GetRegion = 10,
    GetClusterConfig = 11,
    PutClusterConfig = 12,
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
            7 => ::std::option::Option::Some(CommandType::DeleteMeta),
            8 => ::std::option::Option::Some(CommandType::AskChangePeer),
            9 => ::std::option::Option::Some(CommandType::AskSplit),
            10 => ::std::option::Option::Some(CommandType::GetRegion),
            11 => ::std::option::Option::Some(CommandType::GetClusterConfig),
            12 => ::std::option::Option::Some(CommandType::PutClusterConfig),
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
            CommandType::DeleteMeta,
            CommandType::AskChangePeer,
            CommandType::AskSplit,
            CommandType::GetRegion,
            CommandType::GetClusterConfig,
            CommandType::PutClusterConfig,
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
    0x22, 0x2e, 0x0a, 0x06, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x61, 0x64,
    0x64, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x04, 0x61, 0x64, 0x64, 0x72, 0x12, 0x10,
    0x0a, 0x03, 0x70, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03, 0x52, 0x03, 0x70, 0x69, 0x64,
    0x22, 0x24, 0x0a, 0x0a, 0x54, 0x73, 0x6f, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x16,
    0x0a, 0x06, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x06,
    0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x22, 0x41, 0x0a, 0x09, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74,
    0x61, 0x6d, 0x70, 0x12, 0x1a, 0x0a, 0x08, 0x70, 0x68, 0x79, 0x73, 0x69, 0x63, 0x61, 0x6c, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x03, 0x52, 0x08, 0x70, 0x68, 0x79, 0x73, 0x69, 0x63, 0x61, 0x6c, 0x12,
    0x18, 0x0a, 0x07, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03,
    0x52, 0x07, 0x6c, 0x6f, 0x67, 0x69, 0x63, 0x61, 0x6c, 0x22, 0x3e, 0x0a, 0x0b, 0x54, 0x73, 0x6f,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x0a, 0x74, 0x69, 0x6d, 0x65,
    0x73, 0x74, 0x61, 0x6d, 0x70, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x70,
    0x64, 0x70, 0x62, 0x2e, 0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x52, 0x0a, 0x74,
    0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x73, 0x22, 0x5f, 0x0a, 0x10, 0x42, 0x6f, 0x6f,
    0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x23, 0x0a,
    0x05, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d,
    0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x05, 0x73, 0x74, 0x6f,
    0x72, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x22, 0x13, 0x0a, 0x11, 0x42, 0x6f,
    0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22,
    0x17, 0x0a, 0x15, 0x49, 0x73, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65,
    0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x3c, 0x0a, 0x16, 0x49, 0x73, 0x42, 0x6f,
    0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x22, 0x0a, 0x0c, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70,
    0x65, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0c, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74,
    0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x22, 0x10, 0x0a, 0x0e, 0x41, 0x6c, 0x6c, 0x6f, 0x63, 0x49,
    0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0x21, 0x0a, 0x0f, 0x41, 0x6c, 0x6c, 0x6f,
    0x63, 0x49, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x22, 0x2c, 0x0a, 0x0f, 0x47,
    0x65, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x19,
    0x0a, 0x08, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x07, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x64, 0x22, 0x37, 0x0a, 0x10, 0x47, 0x65, 0x74,
    0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x23, 0x0a,
    0x05, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d,
    0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x05, 0x73, 0x74, 0x6f,
    0x72, 0x65, 0x22, 0x36, 0x0a, 0x0f, 0x50, 0x75, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x23, 0x0a, 0x05, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x0d, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x53, 0x74,
    0x6f, 0x72, 0x65, 0x52, 0x05, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x22, 0x12, 0x0a, 0x10, 0x50, 0x75,
    0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x31,
    0x0a, 0x10, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x1d, 0x0a, 0x0a, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x6b, 0x65, 0x79,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x09, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4b, 0x65,
    0x79, 0x22, 0x3b, 0x0a, 0x11, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e,
    0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x22, 0x3c,
    0x0a, 0x17, 0x47, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6c, 0x75,
    0x73, 0x74, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x0b, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x22, 0x45, 0x0a, 0x18,
    0x47, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x29, 0x0a, 0x07, 0x63, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6d, 0x65, 0x74, 0x61,
    0x70, 0x62, 0x2e, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x52, 0x07, 0x63, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x22, 0x44, 0x0a, 0x17, 0x50, 0x75, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x29,
    0x0a, 0x07, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x0f, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x52, 0x07, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x22, 0x1a, 0x0a, 0x18, 0x50, 0x75, 0x74,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x66, 0x0a, 0x14, 0x41, 0x73, 0x6b, 0x43, 0x68, 0x61, 0x6e,
    0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x26, 0x0a,
    0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e,
    0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x26, 0x0a, 0x0f, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f,
    0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d,
    0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x64, 0x22, 0x17, 0x0a,
    0x15, 0x41, 0x73, 0x6b, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x7e, 0x0a, 0x0f, 0x41, 0x73, 0x6b, 0x53, 0x70, 0x6c,
    0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61,
    0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x26,
    0x0a, 0x0f, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69,
    0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x49, 0x64, 0x22, 0x12, 0x0a, 0x10, 0x41, 0x73, 0x6b, 0x53, 0x70, 0x6c,
    0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x22, 0x46, 0x0a, 0x0d, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x75,
    0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12,
    0x21, 0x0a, 0x0c, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x4e, 0x61,
    0x6d, 0x65, 0x22, 0x6a, 0x0a, 0x0e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x12, 0x12, 0x0a, 0x04, 0x75, 0x75, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x04, 0x75, 0x75, 0x69, 0x64, 0x12, 0x21, 0x0a, 0x0c, 0x63, 0x6c, 0x75, 0x73,
    0x74, 0x65, 0x72, 0x5f, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0b,
    0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x4e, 0x61, 0x6d, 0x65, 0x12, 0x21, 0x0a, 0x05, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0b, 0x2e, 0x70, 0x64, 0x70,
    0x62, 0x2e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x22, 0xe6,
    0x05, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2b, 0x0a, 0x06, 0x68, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x70, 0x64, 0x70,
    0x62, 0x2e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52,
    0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x2c, 0x0a, 0x08, 0x63, 0x6d, 0x64, 0x5f, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x70, 0x64, 0x70, 0x62,
    0x2e, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x54, 0x79, 0x70, 0x65, 0x52, 0x07, 0x63, 0x6d,
    0x64, 0x54, 0x79, 0x70, 0x65, 0x12, 0x22, 0x0a, 0x03, 0x74, 0x73, 0x6f, 0x18, 0x03, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x54, 0x73, 0x6f, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x52, 0x03, 0x74, 0x73, 0x6f, 0x12, 0x34, 0x0a, 0x09, 0x62, 0x6f, 0x6f,
    0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x70,
    0x64, 0x70, 0x62, 0x2e, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x52, 0x09, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x12,
    0x44, 0x0a, 0x0f, 0x69, 0x73, 0x5f, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70,
    0x65, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e,
    0x49, 0x73, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0e, 0x69, 0x73, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72,
    0x61, 0x70, 0x70, 0x65, 0x64, 0x12, 0x2f, 0x0a, 0x08, 0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x5f, 0x69,
    0x64, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x41,
    0x6c, 0x6c, 0x6f, 0x63, 0x49, 0x64, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x07, 0x61,
    0x6c, 0x6c, 0x6f, 0x63, 0x49, 0x64, 0x12, 0x42, 0x0a, 0x0f, 0x61, 0x73, 0x6b, 0x5f, 0x63, 0x68,
    0x61, 0x6e, 0x67, 0x65, 0x5f, 0x70, 0x65, 0x65, 0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1a, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x41, 0x73, 0x6b, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65,
    0x50, 0x65, 0x65, 0x72, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0d, 0x61, 0x73, 0x6b,
    0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x12, 0x32, 0x0a, 0x09, 0x61, 0x73,
    0x6b, 0x5f, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e,
    0x70, 0x64, 0x70, 0x62, 0x2e, 0x41, 0x73, 0x6b, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x52, 0x08, 0x61, 0x73, 0x6b, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x12, 0x32,
    0x0a, 0x09, 0x67, 0x65, 0x74, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x09, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x15, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x74, 0x6f, 0x72,
    0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x08, 0x67, 0x65, 0x74, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x12, 0x35, 0x0a, 0x0a, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65,
    0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x09,
    0x67, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x4b, 0x0a, 0x12, 0x67, 0x65, 0x74,
    0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18,
    0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65, 0x74,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x52, 0x10, 0x67, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72,
    0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x32, 0x0a, 0x09, 0x70, 0x75, 0x74, 0x5f, 0x73, 0x74,
    0x6f, 0x72, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x70, 0x64, 0x70, 0x62,
    0x2e, 0x50, 0x75, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x52, 0x08, 0x70, 0x75, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x12, 0x4b, 0x0a, 0x12, 0x70, 0x75,
    0x74, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x50, 0x75,
    0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x10, 0x70, 0x75, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x22, 0xf3, 0x05, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2c, 0x0a, 0x06, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x14, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x48, 0x65, 0x61, 0x64, 0x65, 0x72, 0x52, 0x06, 0x68, 0x65, 0x61, 0x64,
    0x65, 0x72, 0x12, 0x2c, 0x0a, 0x08, 0x63, 0x6d, 0x64, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x0e, 0x32, 0x11, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6d, 0x6d,
    0x61, 0x6e, 0x64, 0x54, 0x79, 0x70, 0x65, 0x52, 0x07, 0x63, 0x6d, 0x64, 0x54, 0x79, 0x70, 0x65,
    0x12, 0x23, 0x0a, 0x03, 0x74, 0x73, 0x6f, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x70, 0x64, 0x70, 0x62, 0x2e, 0x54, 0x73, 0x6f, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x52, 0x03, 0x74, 0x73, 0x6f, 0x12, 0x35, 0x0a, 0x09, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72,
    0x61, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e,
    0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x52, 0x09, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x12, 0x45, 0x0a, 0x0f,
    0x69, 0x73, 0x5f, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x49, 0x73, 0x42,
    0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x52, 0x0e, 0x69, 0x73, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70,
    0x70, 0x65, 0x64, 0x12, 0x30, 0x0a, 0x08, 0x61, 0x6c, 0x6c, 0x6f, 0x63, 0x5f, 0x69, 0x64, 0x18,
    0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x41, 0x6c, 0x6c,
    0x6f, 0x63, 0x49, 0x64, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x07, 0x61, 0x6c,
    0x6c, 0x6f, 0x63, 0x49, 0x64, 0x12, 0x43, 0x0a, 0x0f, 0x61, 0x73, 0x6b, 0x5f, 0x63, 0x68, 0x61,
    0x6e, 0x67, 0x65, 0x5f, 0x70, 0x65, 0x65, 0x72, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b,
    0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x41, 0x73, 0x6b, 0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50,
    0x65, 0x65, 0x72, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0d, 0x61, 0x73, 0x6b,
    0x43, 0x68, 0x61, 0x6e, 0x67, 0x65, 0x50, 0x65, 0x65, 0x72, 0x12, 0x33, 0x0a, 0x09, 0x61, 0x73,
    0x6b, 0x5f, 0x73, 0x70, 0x6c, 0x69, 0x74, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e,
    0x70, 0x64, 0x70, 0x62, 0x2e, 0x41, 0x73, 0x6b, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x08, 0x61, 0x73, 0x6b, 0x53, 0x70, 0x6c, 0x69, 0x74, 0x12,
    0x33, 0x0a, 0x09, 0x67, 0x65, 0x74, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x16, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e, 0x47, 0x65, 0x74, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x08, 0x67, 0x65, 0x74, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x12, 0x36, 0x0a, 0x0a, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e,
    0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x52, 0x09, 0x67, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x4c, 0x0a, 0x12,
    0x67, 0x65, 0x74, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x63, 0x6f, 0x6e, 0x66,
    0x69, 0x67, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x70, 0x64, 0x70, 0x62, 0x2e,
    0x47, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x10, 0x67, 0x65, 0x74, 0x43, 0x6c, 0x75,
    0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x12, 0x33, 0x0a, 0x09, 0x70, 0x75,
    0x74, 0x5f, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x18, 0x0c, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e,
    0x70, 0x64, 0x70, 0x62, 0x2e, 0x50, 0x75, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x08, 0x70, 0x75, 0x74, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x12,
    0x4c, 0x0a, 0x12, 0x70, 0x75, 0x74, 0x5f, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x63,
    0x6f, 0x6e, 0x66, 0x69, 0x67, 0x18, 0x0d, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1e, 0x2e, 0x70, 0x64,
    0x70, 0x62, 0x2e, 0x50, 0x75, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e,
    0x66, 0x69, 0x67, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x10, 0x70, 0x75, 0x74,
    0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x22, 0x13, 0x0a,
    0x11, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x22, 0x5e, 0x0a, 0x05, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x3b, 0x0a, 0x0c, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72,
    0x61, 0x70, 0x70, 0x65, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x70, 0x64,
    0x70, 0x62, 0x2e, 0x42, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x45,
    0x72, 0x72, 0x6f, 0x72, 0x52, 0x0c, 0x62, 0x6f, 0x6f, 0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70,
    0x65, 0x64, 0x2a, 0xdb, 0x01, 0x0a, 0x0b, 0x43, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x54, 0x79,
    0x70, 0x65, 0x12, 0x0b, 0x0a, 0x07, 0x49, 0x6e, 0x76, 0x61, 0x6c, 0x69, 0x64, 0x10, 0x00, 0x12,
    0x07, 0x0a, 0x03, 0x54, 0x73, 0x6f, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x42, 0x6f, 0x6f, 0x74,
    0x73, 0x74, 0x72, 0x61, 0x70, 0x10, 0x02, 0x12, 0x12, 0x0a, 0x0e, 0x49, 0x73, 0x42, 0x6f, 0x6f,
    0x74, 0x73, 0x74, 0x72, 0x61, 0x70, 0x70, 0x65, 0x64, 0x10, 0x03, 0x12, 0x0b, 0x0a, 0x07, 0x41,
    0x6c, 0x6c, 0x6f, 0x63, 0x49, 0x64, 0x10, 0x04, 0x12, 0x0c, 0x0a, 0x08, 0x47, 0x65, 0x74, 0x53,
    0x74, 0x6f, 0x72, 0x65, 0x10, 0x05, 0x12, 0x0c, 0x0a, 0x08, 0x50, 0x75, 0x74, 0x53, 0x74, 0x6f,
    0x72, 0x65, 0x10, 0x06, 0x12, 0x0e, 0x0a, 0x0a, 0x44, 0x65, 0x6c, 0x65, 0x74, 0x65, 0x4d, 0x65,
    0x74, 0x61, 0x10, 0x07, 0x12, 0x11, 0x0a, 0x0d, 0x41, 0x73, 0x6b, 0x43, 0x68, 0x61, 0x6e, 0x67,
    0x65, 0x50, 0x65, 0x65, 0x72, 0x10, 0x08, 0x12, 0x0c, 0x0a, 0x08, 0x41, 0x73, 0x6b, 0x53, 0x70,
    0x6c, 0x69, 0x74, 0x10, 0x09, 0x12, 0x0d, 0x0a, 0x09, 0x47, 0x65, 0x74, 0x52, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x10, 0x0a, 0x12, 0x14, 0x0a, 0x10, 0x47, 0x65, 0x74, 0x43, 0x6c, 0x75, 0x73, 0x74,
    0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x10, 0x0b, 0x12, 0x14, 0x0a, 0x10, 0x50, 0x75,
    0x74, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x43, 0x6f, 0x6e, 0x66, 0x69, 0x67, 0x10, 0x0c,
    0x4a, 0x83, 0x2c, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xb0, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0c,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x07, 0x15, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x05, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x05, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x06, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x06, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x06, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x07, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x07, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x07, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x13, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a,
    0x02, 0x05, 0x00, 0x12, 0x04, 0x0a, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01,
    0x12, 0x03, 0x0a, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b,
    0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x04, 0x0b,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x0b, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x01, 0x02, 0x12, 0x03, 0x0c, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x0d, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d,
    0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x04, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x0e, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x04, 0x12, 0x03, 0x0f, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x0f, 0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x0f,
    0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x10, 0x04, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x10, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x06, 0x12, 0x03, 0x11, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06,
    0x01, 0x12, 0x03, 0x11, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12,
    0x03, 0x11, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x12, 0x04,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x12, 0x04, 0x0e, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x12, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x13, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x08, 0x01, 0x12, 0x03, 0x13, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08,
    0x02, 0x12, 0x03, 0x13, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x09, 0x12, 0x03,
    0x14, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x01, 0x12, 0x03, 0x14, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x09, 0x02, 0x12, 0x03, 0x14, 0x1a, 0x1b, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0a, 0x12, 0x03, 0x15, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0a, 0x01, 0x12, 0x03, 0x15, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x0a, 0x02, 0x12, 0x03, 0x15, 0x1a, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0b,
    0x12, 0x03, 0x16, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x01, 0x12, 0x03,
    0x16, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x0b, 0x02, 0x12, 0x03, 0x16, 0x1a,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x0c, 0x12, 0x03, 0x17, 0x04, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x0c, 0x01, 0x12, 0x03, 0x17, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x00, 0x02, 0x0c, 0x02, 0x12, 0x03, 0x17, 0x1a, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x1a, 0x00, 0x1c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1a,
    0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x04, 0x1f, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x1b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x1b, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1e,
    0x00, 0x21, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x11, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x1f, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x1f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1f, 0x13, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1f, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x20, 0x04,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x20, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x20, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x20, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x20, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12,
    0x04, 0x23, 0x00, 0x25, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x23, 0x08,
    0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x24, 0x04, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x24, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x24, 0x0d, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x24, 0x17, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x24, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x27, 0x00,
    0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x27, 0x08, 0x18, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x28, 0x04, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x28, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x28, 0x1a, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x28, 0x24, 0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x29, 0x04, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x29, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x06, 0x12, 0x03, 0x29, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x1b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04,
    0x2c, 0x00, 0x2e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2c, 0x08, 0x19,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x30, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x06, 0x01, 0x12, 0x03, 0x30, 0x08, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04,
    0x34, 0x00, 0x36, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x34, 0x08, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x35, 0x04, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x35, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x05, 0x12, 0x03, 0x35, 0x0d, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x35, 0x12, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x35, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x38, 0x00, 0x3a,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x38, 0x08, 0x16, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x09, 0x12, 0x04, 0x3c, 0x00, 0x3e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01,
    0x12, 0x03, 0x3c, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x3d,
    0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3d, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3d, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3d, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a,
    0x12, 0x04, 0x40, 0x00, 0x42, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x40,
    0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x41, 0x04, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x05, 0x12, 0x03, 0x41, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x14, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x41, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x44,
    0x00, 0x46, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x44, 0x08, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x45, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x03, 0x45, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x45, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x45, 0x1a, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x45, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x48, 0x00, 0x4a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x48, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x49, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x49, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x49, 0x1a, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x49, 0x22,
    0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x4c, 0x00, 0x4d, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x4c, 0x08, 0x18, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12,
    0x04, 0x4f, 0x00, 0x51, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x4f, 0x08,
    0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x50, 0x04, 0x25, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x50, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x05, 0x12, 0x03, 0x50, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x13, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x50, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x53, 0x00,
    0x55, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x53, 0x08, 0x19, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x54, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x54, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x54, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x54, 0x1b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x54, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x04, 0x57, 0x00, 0x59, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x03, 0x57, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x10, 0x02, 0x00, 0x12, 0x03, 0x58, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x58, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x58, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x03, 0x58,
    0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x03, 0x58, 0x23, 0x24,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x04, 0x5b, 0x00, 0x5d, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x11, 0x01, 0x12, 0x03, 0x5b, 0x08, 0x20, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00,
    0x12, 0x03, 0x5c, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x5c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5c, 0x0d,
    0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5c, 0x1c, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5c, 0x26, 0x27, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x12, 0x12, 0x04, 0x5f, 0x00, 0x61, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x12, 0x01,
    0x12, 0x03, 0x5f, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x03, 0x60,
    0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x03, 0x60, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06, 0x12, 0x03, 0x60, 0x0d, 0x1b, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x03, 0x60, 0x1c, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x03, 0x60, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x13,
    0x12, 0x04, 0x63, 0x00, 0x64, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x03, 0x63,
    0x08, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x04, 0x66, 0x00, 0x6e, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x03, 0x66, 0x08, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x14,
    0x02, 0x00, 0x12, 0x03, 0x67, 0x04, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x67, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x03,
    0x67, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x03, 0x67, 0x1b,
    0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x03, 0x67, 0x27, 0x28, 0x0a,
    0xdb, 0x01, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x03, 0x6d, 0x04, 0x29, 0x1a, 0xcd, 0x01,
    0x20, 0x54, 0x68, 0x65, 0x20, 0x63, 0x75, 0x72, 0x72, 0x65, 0x6e, 0x74, 0x20, 0x6c, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x20, 0x69, 0x64, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x50, 0x64, 0x20,
    0x63, 0x61, 0x6e, 0x20, 0x66, 0x69, 0x72, 0x73, 0x74, 0x20, 0x74, 0x72, 0x79, 0x20, 0x74, 0x6f,
    0x20, 0x73, 0x65, 0x6e, 0x64, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x6f,
    0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x2c, 0x0a, 0x20, 0x69, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x70, 0x65, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20, 0x6e, 0x6f, 0x74,
    0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x20, 0x6e, 0x6f, 0x77, 0x2c, 0x20, 0x70, 0x64, 0x20,
    0x77, 0x69, 0x6c, 0x6c, 0x20, 0x74, 0x72, 0x79, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x66, 0x69, 0x6e,
    0x64, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x65, 0x77, 0x20, 0x6c, 0x65, 0x61, 0x64, 0x65, 0x72,
    0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x73, 0x65, 0x6e, 0x64, 0x0a, 0x20, 0x63, 0x6f,
    0x6d, 0x6d, 0x61, 0x6e, 0x64, 0x20, 0x61, 0x67, 0x61, 0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x6d, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x6d, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x15, 0x12, 0x04, 0x70, 0x00, 0x71,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x15, 0x01, 0x12, 0x03, 0x70, 0x08, 0x1d, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x16, 0x12, 0x04, 0x73, 0x00, 0x77, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x16, 0x01,
    0x12, 0x03, 0x73, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12, 0x03, 0x74,
    0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x03, 0x74, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x06, 0x12, 0x03, 0x74, 0x0d, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x03, 0x74, 0x1b, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x03, 0x74, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x16,
    0x02, 0x01, 0x12, 0x03, 0x75, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x75, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x75, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x03, 0x75, 0x13,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x03, 0x75, 0x29, 0x2a, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x02, 0x12, 0x03, 0x76, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x02, 0x04, 0x12, 0x03, 0x76, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x76, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x76, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x76, 0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x04, 0x79, 0x00, 0x7a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x03, 0x79, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x02,
    0x04, 0x18, 0x12, 0x05, 0x7c, 0x00, 0x80, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x18, 0x01,
    0x12, 0x03, 0x7c, 0x08, 0x15, 0x0a, 0x30, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x03, 0x7e,
    0x04, 0x2c, 0x1a, 0x23, 0x20, 0x31, 0x36, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x2c, 0x20, 0x74,
    0x6f, 0x20, 0x64, 0x69, 0x73, 0x74, 0x69, 0x6e, 0x67, 0x75, 0x69, 0x73, 0x68, 0x20, 0x72, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x7e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x7e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x03, 0x7e, 0x13,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7e, 0x2a, 0x2b, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x01, 0x12, 0x03, 0x7f, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x7f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x7f, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x7f, 0x2a, 0x2b, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0x82, 0x01, 0x00, 0x87,
    0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x19, 0x01, 0x12, 0x04, 0x82, 0x01, 0x08, 0x16, 0x0a,
    0x31, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x00, 0x12, 0x04, 0x84, 0x01, 0x04, 0x28, 0x1a, 0x23, 0x20,
    0x31, 0x36, 0x20, 0x62, 0x79, 0x74, 0x65, 0x73, 0x2c, 0x20, 0x74, 0x6f, 0x20, 0x64, 0x69, 0x73,
    0x74, 0x69, 0x6e, 0x67, 0x75, 0x69, 0x73, 0x68, 0x20, 0x72, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x2e, 0x0a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04, 0x12, 0x04, 0x84, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x05, 0x12, 0x04, 0x84, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04, 0x84, 0x01, 0x13, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0x84, 0x01, 0x26, 0x27, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0x85, 0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0x85, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x01, 0x05, 0x12, 0x04, 0x85, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x85, 0x01, 0x14, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x85, 0x01, 0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x02,
    0x12, 0x04, 0x86, 0x01, 0x04, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x86, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x06, 0x12, 0x04,
    0x86, 0x01, 0x0d, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x01, 0x12, 0x04, 0x86,
    0x01, 0x13, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02, 0x03, 0x12, 0x04, 0x86, 0x01,
    0x26, 0x27, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1a, 0x12, 0x06, 0x89, 0x01, 0x00, 0x97, 0x01, 0x01,
    0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1a, 0x01, 0x12, 0x04, 0x89, 0x01, 0x08, 0x0f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1a, 0x02, 0x00, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x00, 0x04, 0x12, 0x04, 0x8a, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x00, 0x06, 0x12, 0x04, 0x8a, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x00, 0x01, 0x12, 0x04, 0x8a, 0x01, 0x1b, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x00,
    0x03, 0x12, 0x04, 0x8a, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x01, 0x12,
    0x04, 0x8b, 0x01, 0x04, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x04, 0x12, 0x04,
    0x8b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x06, 0x12, 0x04, 0x8b,
    0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x01,
    0x19, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x3a,
    0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x3c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x02, 0x06, 0x12, 0x04, 0x8c, 0x01, 0x0d, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x02, 0x01, 0x12, 0x04, 0x8c, 0x01, 0x18, 0x1b, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x02, 0x03, 0x12, 0x04, 0x8c, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1a, 0x02, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x03, 0x04, 0x12, 0x04, 0x8d, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03,
    0x06, 0x12, 0x04, 0x8d, 0x01, 0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x01,
    0x12, 0x04, 0x8d, 0x01, 0x1e, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x03, 0x03, 0x12,
    0x04, 0x8d, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x04, 0x12, 0x04, 0x8e,
    0x01, 0x04, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x8e, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x8e, 0x01, 0x0d,
    0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x23, 0x32,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x3a, 0x3b, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x3c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x05, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x05, 0x06, 0x12, 0x04, 0x8f, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x8f, 0x01, 0x1c, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x05, 0x03, 0x12, 0x04, 0x8f, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02,
    0x06, 0x12, 0x04, 0x90, 0x01, 0x04, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x06, 0x04,
    0x12, 0x04, 0x90, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x06, 0x06, 0x12,
    0x04, 0x90, 0x01, 0x0d, 0x21, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x06, 0x01, 0x12, 0x04,
    0x90, 0x01, 0x22, 0x31, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x06, 0x03, 0x12, 0x04, 0x90,
    0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x07, 0x12, 0x04, 0x91, 0x01, 0x04,
    0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x07, 0x04, 0x12, 0x04, 0x91, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x07, 0x06, 0x12, 0x04, 0x91, 0x01, 0x0d, 0x1c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x07, 0x01, 0x12, 0x04, 0x91, 0x01, 0x1d, 0x26, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x07, 0x03, 0x12, 0x04, 0x91, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x1a, 0x02, 0x08, 0x12, 0x04, 0x92, 0x01, 0x04, 0x3c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1a, 0x02, 0x08, 0x04, 0x12, 0x04, 0x92, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a,
    0x02, 0x08, 0x06, 0x12, 0x04, 0x92, 0x01, 0x0d, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x08, 0x01, 0x12, 0x04, 0x92, 0x01, 0x1d, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x08,
    0x03, 0x12, 0x04, 0x92, 0x01, 0x3a, 0x3b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x09, 0x12,
    0x04, 0x93, 0x01, 0x04, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x09, 0x04, 0x12, 0x04,
    0x93, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x09, 0x06, 0x12, 0x04, 0x93,
    0x01, 0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x09, 0x01, 0x12, 0x04, 0x93, 0x01,
    0x1e, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x09, 0x03, 0x12, 0x04, 0x93, 0x01, 0x3a,
    0x3c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x0a, 0x12, 0x04, 0x94, 0x01, 0x04, 0x3d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0a, 0x04, 0x12, 0x04, 0x94, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0a, 0x06, 0x12, 0x04, 0x94, 0x01, 0x0d, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1a, 0x02, 0x0a, 0x01, 0x12, 0x04, 0x94, 0x01, 0x25, 0x37, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1a, 0x02, 0x0a, 0x03, 0x12, 0x04, 0x94, 0x01, 0x3a, 0x3c, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x1a, 0x02, 0x0b, 0x12, 0x04, 0x95, 0x01, 0x04, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02,
    0x0b, 0x04, 0x12, 0x04, 0x95, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0b,
    0x06, 0x12, 0x04, 0x95, 0x01, 0x0d, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0b, 0x01,
    0x12, 0x04, 0x95, 0x01, 0x1d, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0b, 0x03, 0x12,
    0x04, 0x95, 0x01, 0x3a, 0x3c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1a, 0x02, 0x0c, 0x12, 0x04, 0x96,
    0x01, 0x04, 0x3d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0c, 0x04, 0x12, 0x04, 0x96, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0c, 0x06, 0x12, 0x04, 0x96, 0x01, 0x0d,
    0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0c, 0x01, 0x12, 0x04, 0x96, 0x01, 0x25, 0x37,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1a, 0x02, 0x0c, 0x03, 0x12, 0x04, 0x96, 0x01, 0x3a, 0x3c, 0x0a,
    0x0c, 0x0a, 0x02, 0x04, 0x1b, 0x12, 0x06, 0x99, 0x01, 0x00, 0xa7, 0x01, 0x01, 0x0a, 0x0b, 0x0a,
    0x03, 0x04, 0x1b, 0x01, 0x12, 0x04, 0x99, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b,
    0x02, 0x00, 0x12, 0x04, 0x9a, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x9a, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x06,
    0x12, 0x04, 0x9a, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x01, 0x12,
    0x04, 0x9a, 0x01, 0x1c, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x00, 0x03, 0x12, 0x04,
    0x9a, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x01, 0x12, 0x04, 0x9b, 0x01,
    0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x06, 0x12, 0x04, 0x9b, 0x01, 0x0d, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x19, 0x21, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x01, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x3e, 0x3f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1b, 0x02, 0x02, 0x12, 0x04, 0x9c, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9c, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x02, 0x06, 0x12, 0x04, 0x9c, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x02, 0x01, 0x12, 0x04, 0x9c, 0x01, 0x19, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x02, 0x03, 0x12, 0x04, 0x9c, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x03,
    0x12, 0x04, 0x9d, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x03, 0x04, 0x12,
    0x04, 0x9d, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x03, 0x06, 0x12, 0x04,
    0x9d, 0x01, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9d,
    0x01, 0x1f, 0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x03, 0x03, 0x12, 0x04, 0x9d, 0x01,
    0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x40,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x04, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x04, 0x06, 0x12, 0x04, 0x9e, 0x01, 0x0d, 0x23, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x04, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x24, 0x33, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x04, 0x03, 0x12, 0x04, 0x9e, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1b, 0x02, 0x05, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x05, 0x04, 0x12, 0x04, 0x9f, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x05, 0x06, 0x12, 0x04, 0x9f, 0x01, 0x0d, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x05,
    0x01, 0x12, 0x04, 0x9f, 0x01, 0x1d, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x05, 0x03,
    0x12, 0x04, 0x9f, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x06, 0x12, 0x04,
    0xa0, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x06, 0x04, 0x12, 0x04, 0xa0,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x06, 0x06, 0x12, 0x04, 0xa0, 0x01,
    0x0d, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x06, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x23,
    0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x06, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x3e, 0x3f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x07, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x40, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x07, 0x04, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x07, 0x06, 0x12, 0x04, 0xa1, 0x01, 0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x07, 0x01, 0x12, 0x04, 0xa1, 0x01, 0x1e, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x07, 0x03, 0x12, 0x04, 0xa1, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b,
    0x02, 0x08, 0x12, 0x04, 0xa2, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x08,
    0x04, 0x12, 0x04, 0xa2, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x08, 0x06,
    0x12, 0x04, 0xa2, 0x01, 0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x08, 0x01, 0x12,
    0x04, 0xa2, 0x01, 0x1e, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x08, 0x03, 0x12, 0x04,
    0xa2, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x09, 0x12, 0x04, 0xa3, 0x01,
    0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x09, 0x04, 0x12, 0x04, 0xa3, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x09, 0x06, 0x12, 0x04, 0xa3, 0x01, 0x0d, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x09, 0x01, 0x12, 0x04, 0xa3, 0x01, 0x1f, 0x29, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x09, 0x03, 0x12, 0x04, 0xa3, 0x01, 0x3e, 0x40, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x1b, 0x02, 0x0a, 0x12, 0x04, 0xa4, 0x01, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x1b, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xa4, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x1b, 0x02, 0x0a, 0x06, 0x12, 0x04, 0xa4, 0x01, 0x0d, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b,
    0x02, 0x0a, 0x01, 0x12, 0x04, 0xa4, 0x01, 0x26, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02,
    0x0a, 0x03, 0x12, 0x04, 0xa4, 0x01, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x0b,
    0x12, 0x04, 0xa5, 0x01, 0x04, 0x41, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x0b, 0x04, 0x12,
    0x04, 0xa5, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x0b, 0x06, 0x12, 0x04,
    0xa5, 0x01, 0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x0b, 0x01, 0x12, 0x04, 0xa5,
    0x01, 0x1e, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x0b, 0x03, 0x12, 0x04, 0xa5, 0x01,
    0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1b, 0x02, 0x0c, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x41,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x0c, 0x04, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1b, 0x02, 0x0c, 0x06, 0x12, 0x04, 0xa6, 0x01, 0x0d, 0x25, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1b, 0x02, 0x0c, 0x01, 0x12, 0x04, 0xa6, 0x01, 0x26, 0x38, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1b, 0x02, 0x0c, 0x03, 0x12, 0x04, 0xa6, 0x01, 0x3e, 0x40, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x1c, 0x12, 0x06, 0xa9, 0x01, 0x00, 0xab, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1c,
    0x01, 0x12, 0x04, 0xa9, 0x01, 0x08, 0x19, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x1d, 0x12, 0x06, 0xad,
    0x01, 0x00, 0xb0, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x1d, 0x01, 0x12, 0x04, 0xad, 0x01,
    0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x1d, 0x02, 0x00, 0x12, 0x04, 0xae, 0x01, 0x04, 0x30,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x04, 0x12, 0x04, 0xae, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x05, 0x12, 0x04, 0xae, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x1d, 0x02, 0x00, 0x01, 0x12, 0x04, 0xae, 0x01, 0x14, 0x1b, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x1d, 0x02, 0x00, 0x03, 0x12, 0x04, 0xae, 0x01, 0x2e, 0x2f, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x1d, 0x02, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x30, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d,
    0x02, 0x01, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02,
    0x01, 0x06, 0x12, 0x04, 0xaf, 0x01, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01,
    0x01, 0x12, 0x04, 0xaf, 0x01, 0x1f, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x1d, 0x02, 0x01, 0x03,
    0x12, 0x04, 0xaf, 0x01, 0x2e, 0x2f,
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
