// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_imports)]

use protobuf::Message as Message_imported_for_functions;
use protobuf::ProtobufEnum as ProtobufEnum_imported_for_functions;

#[derive(Clone,Default)]
pub struct KeyRange {
    // message fields
    start: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyRange {}

impl KeyRange {
    pub fn new() -> KeyRange {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyRange {
        static mut instance: ::protobuf::lazy::Lazy<KeyRange> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyRange,
        };
        unsafe {
            instance.get(|| {
                KeyRange {
                    start: ::protobuf::SingularField::none(),
                    end: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes start = 1;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    pub fn has_start(&self) -> bool {
        self.start.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: ::std::vec::Vec<u8>) {
        self.start = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.start.is_none() {
            self.start.set_default();
        };
        self.start.as_mut().unwrap()
    }

    // Take field
    pub fn take_start(&mut self) -> ::std::vec::Vec<u8> {
        self.start.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_start<'a>(&'a self) -> &'a [u8] {
        match self.start.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes end = 2;

    pub fn clear_end(&mut self) {
        self.end.clear();
    }

    pub fn has_end(&self) -> bool {
        self.end.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.end = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.end.is_none() {
            self.end.set_default();
        };
        self.end.as_mut().unwrap()
    }

    // Take field
    pub fn take_end(&mut self) -> ::std::vec::Vec<u8> {
        self.end.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_end<'a>(&'a self) -> &'a [u8] {
        match self.end.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for KeyRange {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.end));
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
        for value in self.start.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.end.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.end.as_ref() {
            try!(os.write_bytes(2, &v));
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
        ::std::any::TypeId::of::<KeyRange>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KeyRange {
    fn new() -> KeyRange {
        KeyRange::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyRange>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "start",
                    KeyRange::has_start,
                    KeyRange::get_start,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "end",
                    KeyRange::has_end,
                    KeyRange::get_end,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyRange>(
                    "KeyRange",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyRange {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for KeyRange {
    fn eq(&self, other: &KeyRange) -> bool {
        self.start == other.start &&
        self.end == other.end &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for KeyRange {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    context: ::protobuf::SingularPtrField<super::kvrpcpb::Context>,
    tp: ::std::option::Option<i64>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ranges: ::protobuf::RepeatedField<KeyRange>,
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
                    context: ::protobuf::SingularPtrField::none(),
                    tp: ::std::option::Option::None,
                    data: ::protobuf::SingularField::none(),
                    ranges: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.Context context = 1;

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
    pub fn mut_context<'a>(&'a mut self) -> &'a mut super::kvrpcpb::Context {
        if self.context.is_none() {
            self.context.set_default();
        };
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> super::kvrpcpb::Context {
        self.context.take().unwrap_or_else(|| super::kvrpcpb::Context::new())
    }

    pub fn get_context<'a>(&'a self) -> &'a super::kvrpcpb::Context {
        self.context.as_ref().unwrap_or_else(|| super::kvrpcpb::Context::default_instance())
    }

    // optional int64 tp = 2;

    pub fn clear_tp(&mut self) {
        self.tp = ::std::option::Option::None;
    }

    pub fn has_tp(&self) -> bool {
        self.tp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_tp(&mut self, v: i64) {
        self.tp = ::std::option::Option::Some(v);
    }

    pub fn get_tp<'a>(&self) -> i64 {
        self.tp.unwrap_or(0)
    }

    // optional bytes data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data<'a>(&'a self) -> &'a [u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated .coprocessor.KeyRange ranges = 4;

    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }

    // Param is passed by value, moved
    pub fn set_ranges(&mut self, v: ::protobuf::RepeatedField<KeyRange>) {
        self.ranges = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ranges<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<KeyRange> {
        &mut self.ranges
    }

    // Take field
    pub fn take_ranges(&mut self) -> ::protobuf::RepeatedField<KeyRange> {
        ::std::mem::replace(&mut self.ranges, ::protobuf::RepeatedField::new())
    }

    pub fn get_ranges<'a>(&'a self) -> &'a [KeyRange] {
        &self.ranges
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_int64());
                    self.tp = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.ranges));
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
        for value in self.context.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.tp.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.data.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.ranges.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.context.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.tp {
            try!(os.write_int64(2, v));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        for v in self.ranges.iter() {
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
                    "context",
                    Request::has_context,
                    Request::get_context,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_i64_accessor(
                    "tp",
                    Request::has_tp,
                    Request::get_tp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    Request::has_data,
                    Request::get_data,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "ranges",
                    Request::get_ranges,
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
        self.clear_context();
        self.clear_tp();
        self.clear_data();
        self.clear_ranges();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.context == other.context &&
        self.tp == other.tp &&
        self.data == other.data &&
        self.ranges == other.ranges &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Request {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Result {
    // message fields
    field_type: ::std::option::Option<ResultType>,
    msg: ::protobuf::SingularField<::std::string::String>,
    lock_info: ::protobuf::SingularPtrField<super::kvrpcpb::LockInfo>,
    leader_info: ::protobuf::SingularPtrField<super::errorpb::NotLeader>,
    region_not_found: ::protobuf::SingularPtrField<super::errorpb::RegionNotFound>,
    not_in_region: ::protobuf::SingularPtrField<super::errorpb::KeyNotInRegion>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Result {}

impl Result {
    pub fn new() -> Result {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Result {
        static mut instance: ::protobuf::lazy::Lazy<Result> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Result,
        };
        unsafe {
            instance.get(|| {
                Result {
                    field_type: ::std::option::Option::None,
                    msg: ::protobuf::SingularField::none(),
                    lock_info: ::protobuf::SingularPtrField::none(),
                    leader_info: ::protobuf::SingularPtrField::none(),
                    region_not_found: ::protobuf::SingularPtrField::none(),
                    not_in_region: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .coprocessor.ResultType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: ResultType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type<'a>(&self) -> ResultType {
        self.field_type.unwrap_or(ResultType::Ok)
    }

    // optional string msg = 2;

    pub fn clear_msg(&mut self) {
        self.msg.clear();
    }

    pub fn has_msg(&self) -> bool {
        self.msg.is_some()
    }

    // Param is passed by value, moved
    pub fn set_msg(&mut self, v: ::std::string::String) {
        self.msg = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_msg<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.msg.is_none() {
            self.msg.set_default();
        };
        self.msg.as_mut().unwrap()
    }

    // Take field
    pub fn take_msg(&mut self) -> ::std::string::String {
        self.msg.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_msg<'a>(&'a self) -> &'a str {
        match self.msg.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional .kvrpcpb.LockInfo lock_info = 3;

    pub fn clear_lock_info(&mut self) {
        self.lock_info.clear();
    }

    pub fn has_lock_info(&self) -> bool {
        self.lock_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lock_info(&mut self, v: super::kvrpcpb::LockInfo) {
        self.lock_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_lock_info<'a>(&'a mut self) -> &'a mut super::kvrpcpb::LockInfo {
        if self.lock_info.is_none() {
            self.lock_info.set_default();
        };
        self.lock_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_lock_info(&mut self) -> super::kvrpcpb::LockInfo {
        self.lock_info.take().unwrap_or_else(|| super::kvrpcpb::LockInfo::new())
    }

    pub fn get_lock_info<'a>(&'a self) -> &'a super::kvrpcpb::LockInfo {
        self.lock_info.as_ref().unwrap_or_else(|| super::kvrpcpb::LockInfo::default_instance())
    }

    // optional .errorpb.NotLeader leader_info = 4;

    pub fn clear_leader_info(&mut self) {
        self.leader_info.clear();
    }

    pub fn has_leader_info(&self) -> bool {
        self.leader_info.is_some()
    }

    // Param is passed by value, moved
    pub fn set_leader_info(&mut self, v: super::errorpb::NotLeader) {
        self.leader_info = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_leader_info<'a>(&'a mut self) -> &'a mut super::errorpb::NotLeader {
        if self.leader_info.is_none() {
            self.leader_info.set_default();
        };
        self.leader_info.as_mut().unwrap()
    }

    // Take field
    pub fn take_leader_info(&mut self) -> super::errorpb::NotLeader {
        self.leader_info.take().unwrap_or_else(|| super::errorpb::NotLeader::new())
    }

    pub fn get_leader_info<'a>(&'a self) -> &'a super::errorpb::NotLeader {
        self.leader_info.as_ref().unwrap_or_else(|| super::errorpb::NotLeader::default_instance())
    }

    // optional .errorpb.RegionNotFound region_not_found = 5;

    pub fn clear_region_not_found(&mut self) {
        self.region_not_found.clear();
    }

    pub fn has_region_not_found(&self) -> bool {
        self.region_not_found.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_not_found(&mut self, v: super::errorpb::RegionNotFound) {
        self.region_not_found = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_not_found<'a>(&'a mut self) -> &'a mut super::errorpb::RegionNotFound {
        if self.region_not_found.is_none() {
            self.region_not_found.set_default();
        };
        self.region_not_found.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_not_found(&mut self) -> super::errorpb::RegionNotFound {
        self.region_not_found.take().unwrap_or_else(|| super::errorpb::RegionNotFound::new())
    }

    pub fn get_region_not_found<'a>(&'a self) -> &'a super::errorpb::RegionNotFound {
        self.region_not_found.as_ref().unwrap_or_else(|| super::errorpb::RegionNotFound::default_instance())
    }

    // optional .errorpb.KeyNotInRegion not_in_region = 6;

    pub fn clear_not_in_region(&mut self) {
        self.not_in_region.clear();
    }

    pub fn has_not_in_region(&self) -> bool {
        self.not_in_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_not_in_region(&mut self, v: super::errorpb::KeyNotInRegion) {
        self.not_in_region = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_not_in_region<'a>(&'a mut self) -> &'a mut super::errorpb::KeyNotInRegion {
        if self.not_in_region.is_none() {
            self.not_in_region.set_default();
        };
        self.not_in_region.as_mut().unwrap()
    }

    // Take field
    pub fn take_not_in_region(&mut self) -> super::errorpb::KeyNotInRegion {
        self.not_in_region.take().unwrap_or_else(|| super::errorpb::KeyNotInRegion::new())
    }

    pub fn get_not_in_region<'a>(&'a self) -> &'a super::errorpb::KeyNotInRegion {
        self.not_in_region.as_ref().unwrap_or_else(|| super::errorpb::KeyNotInRegion::default_instance())
    }
}

impl ::protobuf::Message for Result {
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
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.msg));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.lock_info));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.leader_info));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_not_found));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.not_in_region));
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
        for value in self.field_type.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.msg.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.lock_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.leader_info.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.region_not_found.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.not_in_region.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.field_type {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.msg.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.lock_info.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.leader_info.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.region_not_found.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.not_in_region.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
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
        ::std::any::TypeId::of::<Result>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Result {
    fn new() -> Result {
        Result::new()
    }

    fn descriptor_static(_: ::std::option::Option<Result>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Result::has_field_type,
                    Result::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "msg",
                    Result::has_msg,
                    Result::get_msg,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "lock_info",
                    Result::has_lock_info,
                    Result::get_lock_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "leader_info",
                    Result::has_leader_info,
                    Result::get_leader_info,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region_not_found",
                    Result::has_region_not_found,
                    Result::get_region_not_found,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "not_in_region",
                    Result::has_not_in_region,
                    Result::get_not_in_region,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Result>(
                    "Result",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Result {
    fn clear(&mut self) {
        self.clear_field_type();
        self.clear_msg();
        self.clear_lock_info();
        self.clear_leader_info();
        self.clear_region_not_found();
        self.clear_not_in_region();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Result {
    fn eq(&self, other: &Result) -> bool {
        self.field_type == other.field_type &&
        self.msg == other.msg &&
        self.lock_info == other.lock_info &&
        self.leader_info == other.leader_info &&
        self.region_not_found == other.region_not_found &&
        self.not_in_region == other.not_in_region &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Result {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Response {
    // message fields
    result: ::protobuf::SingularPtrField<Result>,
    data: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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
                    result: ::protobuf::SingularPtrField::none(),
                    data: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .coprocessor.Result result = 1;

    pub fn clear_result(&mut self) {
        self.result.clear();
    }

    pub fn has_result(&self) -> bool {
        self.result.is_some()
    }

    // Param is passed by value, moved
    pub fn set_result(&mut self, v: Result) {
        self.result = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_result<'a>(&'a mut self) -> &'a mut Result {
        if self.result.is_none() {
            self.result.set_default();
        };
        self.result.as_mut().unwrap()
    }

    // Take field
    pub fn take_result(&mut self) -> Result {
        self.result.take().unwrap_or_else(|| Result::new())
    }

    pub fn get_result<'a>(&'a self) -> &'a Result {
        self.result.as_ref().unwrap_or_else(|| Result::default_instance())
    }

    // optional bytes data = 2;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::std::vec::Vec<u8>) {
        self.data = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.data.is_none() {
            self.data.set_default();
        };
        self.data.as_mut().unwrap()
    }

    // Take field
    pub fn take_data(&mut self) -> ::std::vec::Vec<u8> {
        self.data.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_data<'a>(&'a self) -> &'a [u8] {
        match self.data.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.result));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.data));
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
        for value in self.result.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.data.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.result.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.data.as_ref() {
            try!(os.write_bytes(2, &v));
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
                    "result",
                    Response::has_result,
                    Response::get_result,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "data",
                    Response::has_data,
                    Response::get_data,
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
        self.clear_result();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.result == other.result &&
        self.data == other.data &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum ResultType {
    Ok = 1,
    Locked = 2,
    NotLeader = 3,
    RegionNotFound = 4,
    KeyNotInRegion = 5,
    Other = 6,
}

impl ::protobuf::ProtobufEnum for ResultType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<ResultType> {
        match value {
            1 => ::std::option::Option::Some(ResultType::Ok),
            2 => ::std::option::Option::Some(ResultType::Locked),
            3 => ::std::option::Option::Some(ResultType::NotLeader),
            4 => ::std::option::Option::Some(ResultType::RegionNotFound),
            5 => ::std::option::Option::Some(ResultType::KeyNotInRegion),
            6 => ::std::option::Option::Some(ResultType::Other),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [ResultType] = &[
            ResultType::Ok,
            ResultType::Locked,
            ResultType::NotLeader,
            ResultType::RegionNotFound,
            ResultType::KeyNotInRegion,
            ResultType::Other,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<ResultType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("ResultType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for ResultType {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x11, 0x63, 0x6f, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x12, 0x0b, 0x63, 0x6f, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f, 0x72,
    0x1a, 0x0d, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a,
    0x0d, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x26,
    0x0a, 0x08, 0x4b, 0x65, 0x79, 0x52, 0x61, 0x6e, 0x67, 0x65, 0x12, 0x0d, 0x0a, 0x05, 0x73, 0x74,
    0x61, 0x72, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0b, 0x0a, 0x03, 0x65, 0x6e, 0x64,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x6d, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x12, 0x21, 0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6e,
    0x74, 0x65, 0x78, 0x74, 0x12, 0x0a, 0x0a, 0x02, 0x74, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x03,
    0x12, 0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x25,
    0x0a, 0x06, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x15,
    0x2e, 0x63, 0x6f, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x4b, 0x65, 0x79,
    0x52, 0x61, 0x6e, 0x67, 0x65, 0x22, 0xee, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74,
    0x12, 0x25, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x17,
    0x2e, 0x63, 0x6f, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f, 0x72, 0x2e, 0x52, 0x65, 0x73,
    0x75, 0x6c, 0x74, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0b, 0x0a, 0x03, 0x6d, 0x73, 0x67, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x09, 0x12, 0x24, 0x0a, 0x09, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x69, 0x6e, 0x66,
    0x6f, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70,
    0x62, 0x2e, 0x4c, 0x6f, 0x63, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x27, 0x0a, 0x0b, 0x6c, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x5f, 0x69, 0x6e, 0x66, 0x6f, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x12, 0x2e, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e, 0x4e, 0x6f, 0x74, 0x4c, 0x65, 0x61,
    0x64, 0x65, 0x72, 0x12, 0x31, 0x0a, 0x10, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x6f,
    0x74, 0x5f, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4e, 0x6f,
    0x74, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x2e, 0x0a, 0x0d, 0x6e, 0x6f, 0x74, 0x5f, 0x69, 0x6e,
    0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e,
    0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x4e, 0x6f, 0x74, 0x49, 0x6e,
    0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x22, 0x3d, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x23, 0x0a, 0x06, 0x72, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x63, 0x6f, 0x70, 0x72, 0x6f, 0x63, 0x65, 0x73, 0x73, 0x6f, 0x72,
    0x2e, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x12, 0x0c, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x2a, 0x62, 0x0a, 0x0a, 0x52, 0x65, 0x73, 0x75, 0x6c, 0x74, 0x54,
    0x79, 0x70, 0x65, 0x12, 0x06, 0x0a, 0x02, 0x4f, 0x6b, 0x10, 0x01, 0x12, 0x0a, 0x0a, 0x06, 0x4c,
    0x6f, 0x63, 0x6b, 0x65, 0x64, 0x10, 0x02, 0x12, 0x0d, 0x0a, 0x09, 0x4e, 0x6f, 0x74, 0x4c, 0x65,
    0x61, 0x64, 0x65, 0x72, 0x10, 0x03, 0x12, 0x12, 0x0a, 0x0e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x4e, 0x6f, 0x74, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x10, 0x04, 0x12, 0x12, 0x0a, 0x0e, 0x4b, 0x65,
    0x79, 0x4e, 0x6f, 0x74, 0x49, 0x6e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x10, 0x05, 0x12, 0x09,
    0x0a, 0x05, 0x4f, 0x74, 0x68, 0x65, 0x72, 0x10, 0x06, 0x4a, 0xec, 0x0a, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x28, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x13, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x07, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x04, 0x07, 0x16, 0x0a, 0x1a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x07, 0x00, 0x0a, 0x01,
    0x1a, 0x0e, 0x20, 0x5b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x2c, 0x20, 0x65, 0x6e, 0x64, 0x29, 0x0a,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x08, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x08, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x08, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x08, 0x13, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x08, 0x1b,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x09, 0x04, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x09, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x09, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x09, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x09, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0c, 0x00,
    0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x0d, 0x0d, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0d, 0x1d, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0d, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x2b,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0e, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x0e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e, 0x1d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02,
    0x12, 0x03, 0x0f, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x0f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0f, 0x1d, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0f, 0x29, 0x2a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x10, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x03, 0x04, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03,
    0x06, 0x12, 0x03, 0x10, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x10, 0x1d, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x10,
    0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x13, 0x00, 0x1a, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x13, 0x05, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x14, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x14, 0x04, 0x06, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x14, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x15, 0x04, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x15, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x16, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x16, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x16, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x17,
    0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x17, 0x04, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x17, 0x15, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x18, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x18, 0x04, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x04, 0x02, 0x12, 0x03, 0x18, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05, 0x12,
    0x03, 0x19, 0x04, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x19,
    0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x19, 0x15, 0x16,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1c, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x1d, 0x04, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1d, 0x0d,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1d, 0x24, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1d, 0x37, 0x38, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x04, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x1e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x1e, 0x24, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1e,
    0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x04, 0x39, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x1f, 0x0d, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1f, 0x24, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x1f, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x20, 0x04, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x20,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x06, 0x12, 0x03, 0x20, 0x0d, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x01, 0x12, 0x03, 0x20, 0x24, 0x2f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03, 0x12, 0x03, 0x20, 0x37, 0x38, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x21, 0x04, 0x39, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x04, 0x04, 0x12, 0x03, 0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x06,
    0x12, 0x03, 0x21, 0x0d, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x01, 0x12, 0x03,
    0x21, 0x24, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x04, 0x03, 0x12, 0x03, 0x21, 0x37,
    0x38, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x22, 0x04, 0x39, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x05, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x05, 0x06, 0x12, 0x03, 0x22, 0x0d, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x05, 0x01, 0x12, 0x03, 0x22, 0x24, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x05,
    0x03, 0x12, 0x03, 0x22, 0x37, 0x38, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x25, 0x00,
    0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x25, 0x08, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x26, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x26, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x26, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x26, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x26, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x27, 0x04, 0x1f,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x27, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x27, 0x1d, 0x1e,
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
