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
pub struct LockInfo {
    // message fields
    primary: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ts: ::std::option::Option<u64>,
    row: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for LockInfo {}

impl LockInfo {
    pub fn new() -> LockInfo {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static LockInfo {
        static mut instance: ::protobuf::lazy::Lazy<LockInfo> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const LockInfo,
        };
        unsafe {
            instance.get(|| {
                LockInfo {
                    primary: ::protobuf::SingularField::none(),
                    ts: ::std::option::Option::None,
                    row: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes primary = 1;

    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: ::std::vec::Vec<u8>) {
        self.primary = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.primary.is_none() {
            self.primary.set_default();
        };
        self.primary.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary(&mut self) -> ::std::vec::Vec<u8> {
        self.primary.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_primary(&self) -> &[u8] {
        match self.primary.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 ts = 2;

    pub fn clear_ts(&mut self) {
        self.ts = ::std::option::Option::None;
    }

    pub fn has_ts(&self) -> bool {
        self.ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: u64) {
        self.ts = ::std::option::Option::Some(v);
    }

    pub fn get_ts(&self) -> u64 {
        self.ts.unwrap_or(0)
    }

    // optional bytes row = 3;

    pub fn clear_row(&mut self) {
        self.row.clear();
    }

    pub fn has_row(&self) -> bool {
        self.row.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row(&mut self, v: ::std::vec::Vec<u8>) {
        self.row = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.row.is_none() {
            self.row.set_default();
        };
        self.row.as_mut().unwrap()
    }

    // Take field
    pub fn take_row(&mut self) -> ::std::vec::Vec<u8> {
        self.row.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_row(&self) -> &[u8] {
        match self.row.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for LockInfo {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.primary));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.ts = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.row));
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
        for value in self.primary.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.ts.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.row.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.primary.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.ts {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.row.as_ref() {
            try!(os.write_bytes(3, &v));
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
        ::std::any::TypeId::of::<LockInfo>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for LockInfo {
    fn new() -> LockInfo {
        LockInfo::new()
    }

    fn descriptor_static(_: ::std::option::Option<LockInfo>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "primary",
                    LockInfo::has_primary,
                    LockInfo::get_primary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "ts",
                    LockInfo::has_ts,
                    LockInfo::get_ts,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "row",
                    LockInfo::has_row,
                    LockInfo::get_row,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<LockInfo>(
                    "LockInfo",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for LockInfo {
    fn clear(&mut self) {
        self.clear_primary();
        self.clear_ts();
        self.clear_row();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LockInfo {
    fn eq(&self, other: &LockInfo) -> bool {
        self.primary == other.primary &&
        self.ts == other.ts &&
        self.row == other.row &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for LockInfo {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct KeyError {
    // message fields
    locked: ::protobuf::SingularPtrField<LockInfo>,
    retryable: ::protobuf::SingularField<::std::string::String>,
    abort: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyError {}

impl KeyError {
    pub fn new() -> KeyError {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyError {
        static mut instance: ::protobuf::lazy::Lazy<KeyError> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyError,
        };
        unsafe {
            instance.get(|| {
                KeyError {
                    locked: ::protobuf::SingularPtrField::none(),
                    retryable: ::protobuf::SingularField::none(),
                    abort: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.LockInfo locked = 1;

    pub fn clear_locked(&mut self) {
        self.locked.clear();
    }

    pub fn has_locked(&self) -> bool {
        self.locked.is_some()
    }

    // Param is passed by value, moved
    pub fn set_locked(&mut self, v: LockInfo) {
        self.locked = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_locked(&mut self) -> &mut LockInfo {
        if self.locked.is_none() {
            self.locked.set_default();
        };
        self.locked.as_mut().unwrap()
    }

    // Take field
    pub fn take_locked(&mut self) -> LockInfo {
        self.locked.take().unwrap_or_else(|| LockInfo::new())
    }

    pub fn get_locked(&self) -> &LockInfo {
        self.locked.as_ref().unwrap_or_else(|| LockInfo::default_instance())
    }

    // optional string retryable = 2;

    pub fn clear_retryable(&mut self) {
        self.retryable.clear();
    }

    pub fn has_retryable(&self) -> bool {
        self.retryable.is_some()
    }

    // Param is passed by value, moved
    pub fn set_retryable(&mut self, v: ::std::string::String) {
        self.retryable = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_retryable(&mut self) -> &mut ::std::string::String {
        if self.retryable.is_none() {
            self.retryable.set_default();
        };
        self.retryable.as_mut().unwrap()
    }

    // Take field
    pub fn take_retryable(&mut self) -> ::std::string::String {
        self.retryable.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_retryable(&self) -> &str {
        match self.retryable.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional string abort = 3;

    pub fn clear_abort(&mut self) {
        self.abort.clear();
    }

    pub fn has_abort(&self) -> bool {
        self.abort.is_some()
    }

    // Param is passed by value, moved
    pub fn set_abort(&mut self, v: ::std::string::String) {
        self.abort = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_abort(&mut self) -> &mut ::std::string::String {
        if self.abort.is_none() {
            self.abort.set_default();
        };
        self.abort.as_mut().unwrap()
    }

    // Take field
    pub fn take_abort(&mut self) -> ::std::string::String {
        self.abort.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_abort(&self) -> &str {
        match self.abort.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for KeyError {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.locked));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.retryable));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.abort));
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
        for value in self.locked.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.retryable.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        for value in self.abort.iter() {
            my_size += ::protobuf::rt::string_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.locked.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.retryable.as_ref() {
            try!(os.write_string(2, &v));
        };
        if let Some(v) = self.abort.as_ref() {
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
        ::std::any::TypeId::of::<KeyError>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KeyError {
    fn new() -> KeyError {
        KeyError::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyError>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "locked",
                    KeyError::has_locked,
                    KeyError::get_locked,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "retryable",
                    KeyError::has_retryable,
                    KeyError::get_retryable,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "abort",
                    KeyError::has_abort,
                    KeyError::get_abort,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyError>(
                    "KeyError",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyError {
    fn clear(&mut self) {
        self.clear_locked();
        self.clear_retryable();
        self.clear_abort();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for KeyError {
    fn eq(&self, other: &KeyError) -> bool {
        self.locked == other.locked &&
        self.retryable == other.retryable &&
        self.abort == other.abort &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for KeyError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Context {
    // message fields
    region_id: ::std::option::Option<u64>,
    region_epoch: ::protobuf::SingularPtrField<super::metapb::RegionEpoch>,
    peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Context {}

impl Context {
    pub fn new() -> Context {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Context {
        static mut instance: ::protobuf::lazy::Lazy<Context> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Context,
        };
        unsafe {
            instance.get(|| {
                Context {
                    region_id: ::std::option::Option::None,
                    region_epoch: ::protobuf::SingularPtrField::none(),
                    peer: ::protobuf::SingularPtrField::none(),
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

    // optional .metapb.RegionEpoch region_epoch = 2;

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

    // optional .metapb.Peer peer = 3;

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

impl ::protobuf::Message for Context {
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
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_epoch));
                },
                3 => {
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
        for value in self.region_id.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.region_epoch.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.peer.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.region_id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.region_epoch.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.peer.as_ref() {
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
        ::std::any::TypeId::of::<Context>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Context {
    fn new() -> Context {
        Context::new()
    }

    fn descriptor_static(_: ::std::option::Option<Context>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "region_id",
                    Context::has_region_id,
                    Context::get_region_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region_epoch",
                    Context::has_region_epoch,
                    Context::get_region_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "peer",
                    Context::has_peer,
                    Context::get_peer,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Context>(
                    "Context",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Context {
    fn clear(&mut self) {
        self.clear_region_id();
        self.clear_region_epoch();
        self.clear_peer();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Context {
    fn eq(&self, other: &Context) -> bool {
        self.region_id == other.region_id &&
        self.region_epoch == other.region_epoch &&
        self.peer == other.peer &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Row {
    // message fields
    row_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    columns: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Row {}

impl Row {
    pub fn new() -> Row {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Row {
        static mut instance: ::protobuf::lazy::Lazy<Row> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Row,
        };
        unsafe {
            instance.get(|| {
                Row {
                    row_key: ::protobuf::SingularField::none(),
                    columns: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes row_key = 1;

    pub fn clear_row_key(&mut self) {
        self.row_key.clear();
    }

    pub fn has_row_key(&self) -> bool {
        self.row_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.row_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.row_key.is_none() {
            self.row_key.set_default();
        };
        self.row_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_row_key(&mut self) -> ::std::vec::Vec<u8> {
        self.row_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_row_key(&self) -> &[u8] {
        match self.row_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated bytes columns = 2;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[::std::vec::Vec<u8>] {
        &self.columns
    }
}

impl ::protobuf::Message for Row {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.row_key));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.columns));
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
        for value in self.row_key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.columns.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.row_key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        for v in self.columns.iter() {
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
        ::std::any::TypeId::of::<Row>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Row {
    fn new() -> Row {
        Row::new()
    }

    fn descriptor_static(_: ::std::option::Option<Row>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "row_key",
                    Row::has_row_key,
                    Row::get_row_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "columns",
                    Row::get_columns,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Row>(
                    "Row",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Row {
    fn clear(&mut self) {
        self.clear_row_key();
        self.clear_columns();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Row {
    fn eq(&self, other: &Row) -> bool {
        self.row_key == other.row_key &&
        self.columns == other.columns &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Row {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RowValue {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    row_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    columns: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    values: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RowValue {}

impl RowValue {
    pub fn new() -> RowValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RowValue {
        static mut instance: ::protobuf::lazy::Lazy<RowValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RowValue,
        };
        unsafe {
            instance.get(|| {
                RowValue {
                    error: ::protobuf::SingularPtrField::none(),
                    row_key: ::protobuf::SingularField::none(),
                    columns: ::protobuf::RepeatedField::new(),
                    values: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    // optional bytes row_key = 2;

    pub fn clear_row_key(&mut self) {
        self.row_key.clear();
    }

    pub fn has_row_key(&self) -> bool {
        self.row_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.row_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.row_key.is_none() {
            self.row_key.set_default();
        };
        self.row_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_row_key(&mut self) -> ::std::vec::Vec<u8> {
        self.row_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_row_key(&self) -> &[u8] {
        match self.row_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated bytes columns = 3;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[::std::vec::Vec<u8>] {
        &self.columns
    }

    // repeated bytes values = 4;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }

    pub fn get_values(&self) -> &[::std::vec::Vec<u8>] {
        &self.values
    }
}

impl ::protobuf::Message for RowValue {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.row_key));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.columns));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.values));
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
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.row_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.columns.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.values.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.row_key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        for v in self.columns.iter() {
            try!(os.write_bytes(3, &v));
        };
        for v in self.values.iter() {
            try!(os.write_bytes(4, &v));
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
        ::std::any::TypeId::of::<RowValue>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RowValue {
    fn new() -> RowValue {
        RowValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<RowValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    RowValue::has_error,
                    RowValue::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "row_key",
                    RowValue::has_row_key,
                    RowValue::get_row_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "columns",
                    RowValue::get_columns,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "values",
                    RowValue::get_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RowValue>(
                    "RowValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RowValue {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_row_key();
        self.clear_columns();
        self.clear_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RowValue {
    fn eq(&self, other: &RowValue) -> bool {
        self.error == other.error &&
        self.row_key == other.row_key &&
        self.columns == other.columns &&
        self.values == other.values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RowValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Mutation {
    // message fields
    row_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ops: ::std::vec::Vec<Op>,
    columns: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    values: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Mutation {}

impl Mutation {
    pub fn new() -> Mutation {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Mutation {
        static mut instance: ::protobuf::lazy::Lazy<Mutation> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Mutation,
        };
        unsafe {
            instance.get(|| {
                Mutation {
                    row_key: ::protobuf::SingularField::none(),
                    ops: ::std::vec::Vec::new(),
                    columns: ::protobuf::RepeatedField::new(),
                    values: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes row_key = 1;

    pub fn clear_row_key(&mut self) {
        self.row_key.clear();
    }

    pub fn has_row_key(&self) -> bool {
        self.row_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.row_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.row_key.is_none() {
            self.row_key.set_default();
        };
        self.row_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_row_key(&mut self) -> ::std::vec::Vec<u8> {
        self.row_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_row_key(&self) -> &[u8] {
        match self.row_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // repeated .kvrpcpb.Op ops = 2;

    pub fn clear_ops(&mut self) {
        self.ops.clear();
    }

    // Param is passed by value, moved
    pub fn set_ops(&mut self, v: ::std::vec::Vec<Op>) {
        self.ops = v;
    }

    // Mutable pointer to the field.
    pub fn mut_ops(&mut self) -> &mut ::std::vec::Vec<Op> {
        &mut self.ops
    }

    // Take field
    pub fn take_ops(&mut self) -> ::std::vec::Vec<Op> {
        ::std::mem::replace(&mut self.ops, ::std::vec::Vec::new())
    }

    pub fn get_ops(&self) -> &[Op] {
        &self.ops
    }

    // repeated bytes columns = 3;

    pub fn clear_columns(&mut self) {
        self.columns.clear();
    }

    // Param is passed by value, moved
    pub fn set_columns(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.columns = v;
    }

    // Mutable pointer to the field.
    pub fn mut_columns(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.columns
    }

    // Take field
    pub fn take_columns(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.columns, ::protobuf::RepeatedField::new())
    }

    pub fn get_columns(&self) -> &[::std::vec::Vec<u8>] {
        &self.columns
    }

    // repeated bytes values = 4;

    pub fn clear_values(&mut self) {
        self.values.clear();
    }

    // Param is passed by value, moved
    pub fn set_values(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_values(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.values
    }

    // Take field
    pub fn take_values(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.values, ::protobuf::RepeatedField::new())
    }

    pub fn get_values(&self) -> &[::std::vec::Vec<u8>] {
        &self.values
    }
}

impl ::protobuf::Message for Mutation {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.row_key));
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_enum_into(wire_type, is, &mut self.ops));
                },
                3 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.columns));
                },
                4 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.values));
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
        for value in self.row_key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.ops.iter() {
            my_size += ::protobuf::rt::enum_size(2, *value);
        };
        for value in self.columns.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.values.iter() {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.row_key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        for v in self.ops.iter() {
            try!(os.write_enum(2, v.value()));
        };
        for v in self.columns.iter() {
            try!(os.write_bytes(3, &v));
        };
        for v in self.values.iter() {
            try!(os.write_bytes(4, &v));
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
        ::std::any::TypeId::of::<Mutation>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Mutation {
    fn new() -> Mutation {
        Mutation::new()
    }

    fn descriptor_static(_: ::std::option::Option<Mutation>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "row_key",
                    Mutation::has_row_key,
                    Mutation::get_row_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_enum_accessor(
                    "ops",
                    Mutation::get_ops,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "columns",
                    Mutation::get_columns,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "values",
                    Mutation::get_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Mutation>(
                    "Mutation",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Mutation {
    fn clear(&mut self) {
        self.clear_row_key();
        self.clear_ops();
        self.clear_columns();
        self.clear_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Mutation {
    fn eq(&self, other: &Mutation) -> bool {
        self.row_key == other.row_key &&
        self.ops == other.ops &&
        self.columns == other.columns &&
        self.values == other.values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Mutation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdGetRequest {
    // message fields
    row: ::protobuf::SingularPtrField<Row>,
    ts: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdGetRequest {}

impl CmdGetRequest {
    pub fn new() -> CmdGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdGetRequest,
        };
        unsafe {
            instance.get(|| {
                CmdGetRequest {
                    row: ::protobuf::SingularPtrField::none(),
                    ts: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.Row row = 1;

    pub fn clear_row(&mut self) {
        self.row.clear();
    }

    pub fn has_row(&self) -> bool {
        self.row.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row(&mut self, v: Row) {
        self.row = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row(&mut self) -> &mut Row {
        if self.row.is_none() {
            self.row.set_default();
        };
        self.row.as_mut().unwrap()
    }

    // Take field
    pub fn take_row(&mut self) -> Row {
        self.row.take().unwrap_or_else(|| Row::new())
    }

    pub fn get_row(&self) -> &Row {
        self.row.as_ref().unwrap_or_else(|| Row::default_instance())
    }

    // optional uint64 ts = 2;

    pub fn clear_ts(&mut self) {
        self.ts = ::std::option::Option::None;
    }

    pub fn has_ts(&self) -> bool {
        self.ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: u64) {
        self.ts = ::std::option::Option::Some(v);
    }

    pub fn get_ts(&self) -> u64 {
        self.ts.unwrap_or(0)
    }
}

impl ::protobuf::Message for CmdGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.row));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.ts = ::std::option::Option::Some(tmp);
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
        for value in self.row.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ts.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.row.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ts {
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
        ::std::any::TypeId::of::<CmdGetRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdGetRequest {
    fn new() -> CmdGetRequest {
        CmdGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "row",
                    CmdGetRequest::has_row,
                    CmdGetRequest::get_row,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "ts",
                    CmdGetRequest::has_ts,
                    CmdGetRequest::get_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdGetRequest>(
                    "CmdGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdGetRequest {
    fn clear(&mut self) {
        self.clear_row();
        self.clear_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdGetRequest {
    fn eq(&self, other: &CmdGetRequest) -> bool {
        self.row == other.row &&
        self.ts == other.ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdGetResponse {
    // message fields
    row: ::protobuf::SingularPtrField<RowValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdGetResponse {}

impl CmdGetResponse {
    pub fn new() -> CmdGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdGetResponse,
        };
        unsafe {
            instance.get(|| {
                CmdGetResponse {
                    row: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.RowValue row = 1;

    pub fn clear_row(&mut self) {
        self.row.clear();
    }

    pub fn has_row(&self) -> bool {
        self.row.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row(&mut self, v: RowValue) {
        self.row = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row(&mut self) -> &mut RowValue {
        if self.row.is_none() {
            self.row.set_default();
        };
        self.row.as_mut().unwrap()
    }

    // Take field
    pub fn take_row(&mut self) -> RowValue {
        self.row.take().unwrap_or_else(|| RowValue::new())
    }

    pub fn get_row(&self) -> &RowValue {
        self.row.as_ref().unwrap_or_else(|| RowValue::default_instance())
    }
}

impl ::protobuf::Message for CmdGetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.row));
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
        for value in self.row.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.row.as_ref() {
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
        ::std::any::TypeId::of::<CmdGetResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdGetResponse {
    fn new() -> CmdGetResponse {
        CmdGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "row",
                    CmdGetResponse::has_row,
                    CmdGetResponse::get_row,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdGetResponse>(
                    "CmdGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdGetResponse {
    fn clear(&mut self) {
        self.clear_row();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdGetResponse {
    fn eq(&self, other: &CmdGetResponse) -> bool {
        self.row == other.row &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdScanRequest {
    // message fields
    start_row: ::protobuf::SingularPtrField<Row>,
    limit: ::std::option::Option<u32>,
    ts: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdScanRequest {}

impl CmdScanRequest {
    pub fn new() -> CmdScanRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdScanRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdScanRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdScanRequest,
        };
        unsafe {
            instance.get(|| {
                CmdScanRequest {
                    start_row: ::protobuf::SingularPtrField::none(),
                    limit: ::std::option::Option::None,
                    ts: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.Row start_row = 1;

    pub fn clear_start_row(&mut self) {
        self.start_row.clear();
    }

    pub fn has_start_row(&self) -> bool {
        self.start_row.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_row(&mut self, v: Row) {
        self.start_row = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_row(&mut self) -> &mut Row {
        if self.start_row.is_none() {
            self.start_row.set_default();
        };
        self.start_row.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_row(&mut self) -> Row {
        self.start_row.take().unwrap_or_else(|| Row::new())
    }

    pub fn get_start_row(&self) -> &Row {
        self.start_row.as_ref().unwrap_or_else(|| Row::default_instance())
    }

    // optional uint32 limit = 2;

    pub fn clear_limit(&mut self) {
        self.limit = ::std::option::Option::None;
    }

    pub fn has_limit(&self) -> bool {
        self.limit.is_some()
    }

    // Param is passed by value, moved
    pub fn set_limit(&mut self, v: u32) {
        self.limit = ::std::option::Option::Some(v);
    }

    pub fn get_limit(&self) -> u32 {
        self.limit.unwrap_or(0)
    }

    // optional uint64 ts = 3;

    pub fn clear_ts(&mut self) {
        self.ts = ::std::option::Option::None;
    }

    pub fn has_ts(&self) -> bool {
        self.ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: u64) {
        self.ts = ::std::option::Option::Some(v);
    }

    pub fn get_ts(&self) -> u64 {
        self.ts.unwrap_or(0)
    }
}

impl ::protobuf::Message for CmdScanRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.start_row));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.limit = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.ts = ::std::option::Option::Some(tmp);
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
        for value in self.start_row.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.limit.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.ts.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_row.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.limit {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.ts {
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
        ::std::any::TypeId::of::<CmdScanRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdScanRequest {
    fn new() -> CmdScanRequest {
        CmdScanRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdScanRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "start_row",
                    CmdScanRequest::has_start_row,
                    CmdScanRequest::get_start_row,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "limit",
                    CmdScanRequest::has_limit,
                    CmdScanRequest::get_limit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "ts",
                    CmdScanRequest::has_ts,
                    CmdScanRequest::get_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdScanRequest>(
                    "CmdScanRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdScanRequest {
    fn clear(&mut self) {
        self.clear_start_row();
        self.clear_limit();
        self.clear_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdScanRequest {
    fn eq(&self, other: &CmdScanRequest) -> bool {
        self.start_row == other.start_row &&
        self.limit == other.limit &&
        self.ts == other.ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdScanRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdScanResponse {
    // message fields
    rows: ::protobuf::RepeatedField<RowValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdScanResponse {}

impl CmdScanResponse {
    pub fn new() -> CmdScanResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdScanResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdScanResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdScanResponse,
        };
        unsafe {
            instance.get(|| {
                CmdScanResponse {
                    rows: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kvrpcpb.RowValue rows = 1;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<RowValue>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<RowValue> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<RowValue> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[RowValue] {
        &self.rows
    }
}

impl ::protobuf::Message for CmdScanResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows));
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
        for value in self.rows.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.rows.iter() {
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
        ::std::any::TypeId::of::<CmdScanResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdScanResponse {
    fn new() -> CmdScanResponse {
        CmdScanResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdScanResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "rows",
                    CmdScanResponse::get_rows,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdScanResponse>(
                    "CmdScanResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdScanResponse {
    fn clear(&mut self) {
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdScanResponse {
    fn eq(&self, other: &CmdScanResponse) -> bool {
        self.rows == other.rows &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdScanResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdPrewriteRequest {
    // message fields
    mutations: ::protobuf::RepeatedField<Mutation>,
    primary: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ts: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdPrewriteRequest {}

impl CmdPrewriteRequest {
    pub fn new() -> CmdPrewriteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdPrewriteRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdPrewriteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdPrewriteRequest,
        };
        unsafe {
            instance.get(|| {
                CmdPrewriteRequest {
                    mutations: ::protobuf::RepeatedField::new(),
                    primary: ::protobuf::SingularField::none(),
                    ts: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kvrpcpb.Mutation mutations = 1;

    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }

    // Param is passed by value, moved
    pub fn set_mutations(&mut self, v: ::protobuf::RepeatedField<Mutation>) {
        self.mutations = v;
    }

    // Mutable pointer to the field.
    pub fn mut_mutations(&mut self) -> &mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }

    // Take field
    pub fn take_mutations(&mut self) -> ::protobuf::RepeatedField<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::protobuf::RepeatedField::new())
    }

    pub fn get_mutations(&self) -> &[Mutation] {
        &self.mutations
    }

    // optional bytes primary = 2;

    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }

    pub fn has_primary(&self) -> bool {
        self.primary.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary(&mut self, v: ::std::vec::Vec<u8>) {
        self.primary = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.primary.is_none() {
            self.primary.set_default();
        };
        self.primary.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary(&mut self) -> ::std::vec::Vec<u8> {
        self.primary.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_primary(&self) -> &[u8] {
        match self.primary.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 ts = 3;

    pub fn clear_ts(&mut self) {
        self.ts = ::std::option::Option::None;
    }

    pub fn has_ts(&self) -> bool {
        self.ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: u64) {
        self.ts = ::std::option::Option::Some(v);
    }

    pub fn get_ts(&self) -> u64 {
        self.ts.unwrap_or(0)
    }
}

impl ::protobuf::Message for CmdPrewriteRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.mutations));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.primary));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.ts = ::std::option::Option::Some(tmp);
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
        for value in self.mutations.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.primary.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.ts.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.mutations.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.primary.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.ts {
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
        ::std::any::TypeId::of::<CmdPrewriteRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdPrewriteRequest {
    fn new() -> CmdPrewriteRequest {
        CmdPrewriteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdPrewriteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "mutations",
                    CmdPrewriteRequest::get_mutations,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "primary",
                    CmdPrewriteRequest::has_primary,
                    CmdPrewriteRequest::get_primary,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "ts",
                    CmdPrewriteRequest::has_ts,
                    CmdPrewriteRequest::get_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdPrewriteRequest>(
                    "CmdPrewriteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdPrewriteRequest {
    fn clear(&mut self) {
        self.clear_mutations();
        self.clear_primary();
        self.clear_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdPrewriteRequest {
    fn eq(&self, other: &CmdPrewriteRequest) -> bool {
        self.mutations == other.mutations &&
        self.primary == other.primary &&
        self.ts == other.ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdPrewriteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdPrewriteResponse {
    // message fields
    errors: ::protobuf::RepeatedField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdPrewriteResponse {}

impl CmdPrewriteResponse {
    pub fn new() -> CmdPrewriteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdPrewriteResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdPrewriteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdPrewriteResponse,
        };
        unsafe {
            instance.get(|| {
                CmdPrewriteResponse {
                    errors: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kvrpcpb.KeyError errors = 1;

    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    // Param is passed by value, moved
    pub fn set_errors(&mut self, v: ::protobuf::RepeatedField<KeyError>) {
        self.errors = v;
    }

    // Mutable pointer to the field.
    pub fn mut_errors(&mut self) -> &mut ::protobuf::RepeatedField<KeyError> {
        &mut self.errors
    }

    // Take field
    pub fn take_errors(&mut self) -> ::protobuf::RepeatedField<KeyError> {
        ::std::mem::replace(&mut self.errors, ::protobuf::RepeatedField::new())
    }

    pub fn get_errors(&self) -> &[KeyError] {
        &self.errors
    }
}

impl ::protobuf::Message for CmdPrewriteResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.errors));
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
        for value in self.errors.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.errors.iter() {
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
        ::std::any::TypeId::of::<CmdPrewriteResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdPrewriteResponse {
    fn new() -> CmdPrewriteResponse {
        CmdPrewriteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdPrewriteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "errors",
                    CmdPrewriteResponse::get_errors,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdPrewriteResponse>(
                    "CmdPrewriteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdPrewriteResponse {
    fn clear(&mut self) {
        self.clear_errors();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdPrewriteResponse {
    fn eq(&self, other: &CmdPrewriteResponse) -> bool {
        self.errors == other.errors &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdPrewriteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdCommitRequest {
    // message fields
    start_ts: ::std::option::Option<u64>,
    rows: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    commit_ts: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdCommitRequest {}

impl CmdCommitRequest {
    pub fn new() -> CmdCommitRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdCommitRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdCommitRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdCommitRequest,
        };
        unsafe {
            instance.get(|| {
                CmdCommitRequest {
                    start_ts: ::std::option::Option::None,
                    rows: ::protobuf::RepeatedField::new(),
                    commit_ts: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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

    // repeated bytes rows = 2;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[::std::vec::Vec<u8>] {
        &self.rows
    }

    // optional uint64 commit_ts = 3;

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
}

impl ::protobuf::Message for CmdCommitRequest {
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
                    self.start_ts = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.rows));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_ts = ::std::option::Option::Some(tmp);
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
        for value in self.start_ts.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.rows.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.commit_ts.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_ts {
            try!(os.write_uint64(1, v));
        };
        for v in self.rows.iter() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.commit_ts {
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
        ::std::any::TypeId::of::<CmdCommitRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdCommitRequest {
    fn new() -> CmdCommitRequest {
        CmdCommitRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdCommitRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "start_ts",
                    CmdCommitRequest::has_start_ts,
                    CmdCommitRequest::get_start_ts,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "rows",
                    CmdCommitRequest::get_rows,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_ts",
                    CmdCommitRequest::has_commit_ts,
                    CmdCommitRequest::get_commit_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdCommitRequest>(
                    "CmdCommitRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdCommitRequest {
    fn clear(&mut self) {
        self.clear_start_ts();
        self.clear_rows();
        self.clear_commit_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCommitRequest {
    fn eq(&self, other: &CmdCommitRequest) -> bool {
        self.start_ts == other.start_ts &&
        self.rows == other.rows &&
        self.commit_ts == other.commit_ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdCommitRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdCommitResponse {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdCommitResponse {}

impl CmdCommitResponse {
    pub fn new() -> CmdCommitResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdCommitResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdCommitResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdCommitResponse,
        };
        unsafe {
            instance.get(|| {
                CmdCommitResponse {
                    error: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }
}

impl ::protobuf::Message for CmdCommitResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
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
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
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
        ::std::any::TypeId::of::<CmdCommitResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdCommitResponse {
    fn new() -> CmdCommitResponse {
        CmdCommitResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdCommitResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    CmdCommitResponse::has_error,
                    CmdCommitResponse::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdCommitResponse>(
                    "CmdCommitResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdCommitResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCommitResponse {
    fn eq(&self, other: &CmdCommitResponse) -> bool {
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdCommitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdBatchRollbackRequest {
    // message fields
    ts: ::std::option::Option<u64>,
    rows: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdBatchRollbackRequest {}

impl CmdBatchRollbackRequest {
    pub fn new() -> CmdBatchRollbackRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdBatchRollbackRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdBatchRollbackRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdBatchRollbackRequest,
        };
        unsafe {
            instance.get(|| {
                CmdBatchRollbackRequest {
                    ts: ::std::option::Option::None,
                    rows: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 ts = 1;

    pub fn clear_ts(&mut self) {
        self.ts = ::std::option::Option::None;
    }

    pub fn has_ts(&self) -> bool {
        self.ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: u64) {
        self.ts = ::std::option::Option::Some(v);
    }

    pub fn get_ts(&self) -> u64 {
        self.ts.unwrap_or(0)
    }

    // repeated bytes rows = 2;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[::std::vec::Vec<u8>] {
        &self.rows
    }
}

impl ::protobuf::Message for CmdBatchRollbackRequest {
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
                    self.ts = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.rows));
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
        for value in self.ts.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.rows.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.ts {
            try!(os.write_uint64(1, v));
        };
        for v in self.rows.iter() {
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
        ::std::any::TypeId::of::<CmdBatchRollbackRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdBatchRollbackRequest {
    fn new() -> CmdBatchRollbackRequest {
        CmdBatchRollbackRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdBatchRollbackRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "ts",
                    CmdBatchRollbackRequest::has_ts,
                    CmdBatchRollbackRequest::get_ts,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "rows",
                    CmdBatchRollbackRequest::get_rows,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdBatchRollbackRequest>(
                    "CmdBatchRollbackRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdBatchRollbackRequest {
    fn clear(&mut self) {
        self.clear_ts();
        self.clear_rows();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdBatchRollbackRequest {
    fn eq(&self, other: &CmdBatchRollbackRequest) -> bool {
        self.ts == other.ts &&
        self.rows == other.rows &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdBatchRollbackRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdBatchRollbackResponse {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdBatchRollbackResponse {}

impl CmdBatchRollbackResponse {
    pub fn new() -> CmdBatchRollbackResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdBatchRollbackResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdBatchRollbackResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdBatchRollbackResponse,
        };
        unsafe {
            instance.get(|| {
                CmdBatchRollbackResponse {
                    error: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }
}

impl ::protobuf::Message for CmdBatchRollbackResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
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
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
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
        ::std::any::TypeId::of::<CmdBatchRollbackResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdBatchRollbackResponse {
    fn new() -> CmdBatchRollbackResponse {
        CmdBatchRollbackResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdBatchRollbackResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    CmdBatchRollbackResponse::has_error,
                    CmdBatchRollbackResponse::get_error,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdBatchRollbackResponse>(
                    "CmdBatchRollbackResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdBatchRollbackResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdBatchRollbackResponse {
    fn eq(&self, other: &CmdBatchRollbackResponse) -> bool {
        self.error == other.error &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdBatchRollbackResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdCleanupRequest {
    // message fields
    row: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    ts: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdCleanupRequest {}

impl CmdCleanupRequest {
    pub fn new() -> CmdCleanupRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdCleanupRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdCleanupRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdCleanupRequest,
        };
        unsafe {
            instance.get(|| {
                CmdCleanupRequest {
                    row: ::protobuf::SingularField::none(),
                    ts: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes row = 1;

    pub fn clear_row(&mut self) {
        self.row.clear();
    }

    pub fn has_row(&self) -> bool {
        self.row.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row(&mut self, v: ::std::vec::Vec<u8>) {
        self.row = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.row.is_none() {
            self.row.set_default();
        };
        self.row.as_mut().unwrap()
    }

    // Take field
    pub fn take_row(&mut self) -> ::std::vec::Vec<u8> {
        self.row.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_row(&self) -> &[u8] {
        match self.row.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 ts = 2;

    pub fn clear_ts(&mut self) {
        self.ts = ::std::option::Option::None;
    }

    pub fn has_ts(&self) -> bool {
        self.ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: u64) {
        self.ts = ::std::option::Option::Some(v);
    }

    pub fn get_ts(&self) -> u64 {
        self.ts.unwrap_or(0)
    }
}

impl ::protobuf::Message for CmdCleanupRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.row));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.ts = ::std::option::Option::Some(tmp);
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
        for value in self.row.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.ts.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.row.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.ts {
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
        ::std::any::TypeId::of::<CmdCleanupRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdCleanupRequest {
    fn new() -> CmdCleanupRequest {
        CmdCleanupRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdCleanupRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "row",
                    CmdCleanupRequest::has_row,
                    CmdCleanupRequest::get_row,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "ts",
                    CmdCleanupRequest::has_ts,
                    CmdCleanupRequest::get_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdCleanupRequest>(
                    "CmdCleanupRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdCleanupRequest {
    fn clear(&mut self) {
        self.clear_row();
        self.clear_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCleanupRequest {
    fn eq(&self, other: &CmdCleanupRequest) -> bool {
        self.row == other.row &&
        self.ts == other.ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdCleanupRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdCleanupResponse {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    commit_ts: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdCleanupResponse {}

impl CmdCleanupResponse {
    pub fn new() -> CmdCleanupResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdCleanupResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdCleanupResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdCleanupResponse,
        };
        unsafe {
            instance.get(|| {
                CmdCleanupResponse {
                    error: ::protobuf::SingularPtrField::none(),
                    commit_ts: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.KeyError error = 1;

    pub fn clear_error(&mut self) {
        self.error.clear();
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error(&self) -> &KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
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
}

impl ::protobuf::Message for CmdCleanupResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_ts = ::std::option::Option::Some(tmp);
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
        for value in self.error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.commit_ts.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.error.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.commit_ts {
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
        ::std::any::TypeId::of::<CmdCleanupResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdCleanupResponse {
    fn new() -> CmdCleanupResponse {
        CmdCleanupResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdCleanupResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    CmdCleanupResponse::has_error,
                    CmdCleanupResponse::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_ts",
                    CmdCleanupResponse::has_commit_ts,
                    CmdCleanupResponse::get_commit_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdCleanupResponse>(
                    "CmdCleanupResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdCleanupResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_commit_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCleanupResponse {
    fn eq(&self, other: &CmdCleanupResponse) -> bool {
        self.error == other.error &&
        self.commit_ts == other.commit_ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdCleanupResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdRollbackThenGetRequest {
    // message fields
    row: ::protobuf::SingularPtrField<Row>,
    ts: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdRollbackThenGetRequest {}

impl CmdRollbackThenGetRequest {
    pub fn new() -> CmdRollbackThenGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdRollbackThenGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdRollbackThenGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdRollbackThenGetRequest,
        };
        unsafe {
            instance.get(|| {
                CmdRollbackThenGetRequest {
                    row: ::protobuf::SingularPtrField::none(),
                    ts: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.Row row = 1;

    pub fn clear_row(&mut self) {
        self.row.clear();
    }

    pub fn has_row(&self) -> bool {
        self.row.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row(&mut self, v: Row) {
        self.row = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row(&mut self) -> &mut Row {
        if self.row.is_none() {
            self.row.set_default();
        };
        self.row.as_mut().unwrap()
    }

    // Take field
    pub fn take_row(&mut self) -> Row {
        self.row.take().unwrap_or_else(|| Row::new())
    }

    pub fn get_row(&self) -> &Row {
        self.row.as_ref().unwrap_or_else(|| Row::default_instance())
    }

    // optional uint64 ts = 2;

    pub fn clear_ts(&mut self) {
        self.ts = ::std::option::Option::None;
    }

    pub fn has_ts(&self) -> bool {
        self.ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: u64) {
        self.ts = ::std::option::Option::Some(v);
    }

    pub fn get_ts(&self) -> u64 {
        self.ts.unwrap_or(0)
    }
}

impl ::protobuf::Message for CmdRollbackThenGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.row));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.ts = ::std::option::Option::Some(tmp);
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
        for value in self.row.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ts.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.row.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ts {
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
        ::std::any::TypeId::of::<CmdRollbackThenGetRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdRollbackThenGetRequest {
    fn new() -> CmdRollbackThenGetRequest {
        CmdRollbackThenGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdRollbackThenGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "row",
                    CmdRollbackThenGetRequest::has_row,
                    CmdRollbackThenGetRequest::get_row,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "ts",
                    CmdRollbackThenGetRequest::has_ts,
                    CmdRollbackThenGetRequest::get_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdRollbackThenGetRequest>(
                    "CmdRollbackThenGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdRollbackThenGetRequest {
    fn clear(&mut self) {
        self.clear_row();
        self.clear_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdRollbackThenGetRequest {
    fn eq(&self, other: &CmdRollbackThenGetRequest) -> bool {
        self.row == other.row &&
        self.ts == other.ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdRollbackThenGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdRollbackThenGetResponse {
    // message fields
    row_value: ::protobuf::SingularPtrField<RowValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdRollbackThenGetResponse {}

impl CmdRollbackThenGetResponse {
    pub fn new() -> CmdRollbackThenGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdRollbackThenGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdRollbackThenGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdRollbackThenGetResponse,
        };
        unsafe {
            instance.get(|| {
                CmdRollbackThenGetResponse {
                    row_value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.RowValue row_value = 1;

    pub fn clear_row_value(&mut self) {
        self.row_value.clear();
    }

    pub fn has_row_value(&self) -> bool {
        self.row_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row_value(&mut self, v: RowValue) {
        self.row_value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row_value(&mut self) -> &mut RowValue {
        if self.row_value.is_none() {
            self.row_value.set_default();
        };
        self.row_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_row_value(&mut self) -> RowValue {
        self.row_value.take().unwrap_or_else(|| RowValue::new())
    }

    pub fn get_row_value(&self) -> &RowValue {
        self.row_value.as_ref().unwrap_or_else(|| RowValue::default_instance())
    }
}

impl ::protobuf::Message for CmdRollbackThenGetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.row_value));
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
        for value in self.row_value.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.row_value.as_ref() {
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
        ::std::any::TypeId::of::<CmdRollbackThenGetResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdRollbackThenGetResponse {
    fn new() -> CmdRollbackThenGetResponse {
        CmdRollbackThenGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdRollbackThenGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "row_value",
                    CmdRollbackThenGetResponse::has_row_value,
                    CmdRollbackThenGetResponse::get_row_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdRollbackThenGetResponse>(
                    "CmdRollbackThenGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdRollbackThenGetResponse {
    fn clear(&mut self) {
        self.clear_row_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdRollbackThenGetResponse {
    fn eq(&self, other: &CmdRollbackThenGetResponse) -> bool {
        self.row_value == other.row_value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdRollbackThenGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdCommitThenGetRequest {
    // message fields
    row: ::protobuf::SingularPtrField<Row>,
    start_ts: ::std::option::Option<u64>,
    commit_ts: ::std::option::Option<u64>,
    get_ts: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdCommitThenGetRequest {}

impl CmdCommitThenGetRequest {
    pub fn new() -> CmdCommitThenGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdCommitThenGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdCommitThenGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdCommitThenGetRequest,
        };
        unsafe {
            instance.get(|| {
                CmdCommitThenGetRequest {
                    row: ::protobuf::SingularPtrField::none(),
                    start_ts: ::std::option::Option::None,
                    commit_ts: ::std::option::Option::None,
                    get_ts: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.Row row = 1;

    pub fn clear_row(&mut self) {
        self.row.clear();
    }

    pub fn has_row(&self) -> bool {
        self.row.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row(&mut self, v: Row) {
        self.row = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row(&mut self) -> &mut Row {
        if self.row.is_none() {
            self.row.set_default();
        };
        self.row.as_mut().unwrap()
    }

    // Take field
    pub fn take_row(&mut self) -> Row {
        self.row.take().unwrap_or_else(|| Row::new())
    }

    pub fn get_row(&self) -> &Row {
        self.row.as_ref().unwrap_or_else(|| Row::default_instance())
    }

    // optional uint64 start_ts = 3;

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

    // optional uint64 commit_ts = 4;

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

    // optional uint64 get_ts = 5;

    pub fn clear_get_ts(&mut self) {
        self.get_ts = ::std::option::Option::None;
    }

    pub fn has_get_ts(&self) -> bool {
        self.get_ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_ts(&mut self, v: u64) {
        self.get_ts = ::std::option::Option::Some(v);
    }

    pub fn get_get_ts(&self) -> u64 {
        self.get_ts.unwrap_or(0)
    }
}

impl ::protobuf::Message for CmdCommitThenGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.row));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.start_ts = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_ts = ::std::option::Option::Some(tmp);
                },
                5 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.get_ts = ::std::option::Option::Some(tmp);
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
        for value in self.row.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.start_ts.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.commit_ts.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.get_ts.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.row.as_ref() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.start_ts {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.commit_ts {
            try!(os.write_uint64(4, v));
        };
        if let Some(v) = self.get_ts {
            try!(os.write_uint64(5, v));
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
        ::std::any::TypeId::of::<CmdCommitThenGetRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdCommitThenGetRequest {
    fn new() -> CmdCommitThenGetRequest {
        CmdCommitThenGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdCommitThenGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "row",
                    CmdCommitThenGetRequest::has_row,
                    CmdCommitThenGetRequest::get_row,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "start_ts",
                    CmdCommitThenGetRequest::has_start_ts,
                    CmdCommitThenGetRequest::get_start_ts,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_ts",
                    CmdCommitThenGetRequest::has_commit_ts,
                    CmdCommitThenGetRequest::get_commit_ts,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "get_ts",
                    CmdCommitThenGetRequest::has_get_ts,
                    CmdCommitThenGetRequest::get_get_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdCommitThenGetRequest>(
                    "CmdCommitThenGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdCommitThenGetRequest {
    fn clear(&mut self) {
        self.clear_row();
        self.clear_start_ts();
        self.clear_commit_ts();
        self.clear_get_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCommitThenGetRequest {
    fn eq(&self, other: &CmdCommitThenGetRequest) -> bool {
        self.row == other.row &&
        self.start_ts == other.start_ts &&
        self.commit_ts == other.commit_ts &&
        self.get_ts == other.get_ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdCommitThenGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdCommitThenGetResponse {
    // message fields
    row_value: ::protobuf::SingularPtrField<RowValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdCommitThenGetResponse {}

impl CmdCommitThenGetResponse {
    pub fn new() -> CmdCommitThenGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdCommitThenGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdCommitThenGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdCommitThenGetResponse,
        };
        unsafe {
            instance.get(|| {
                CmdCommitThenGetResponse {
                    row_value: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.RowValue row_value = 1;

    pub fn clear_row_value(&mut self) {
        self.row_value.clear();
    }

    pub fn has_row_value(&self) -> bool {
        self.row_value.is_some()
    }

    // Param is passed by value, moved
    pub fn set_row_value(&mut self, v: RowValue) {
        self.row_value = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_row_value(&mut self) -> &mut RowValue {
        if self.row_value.is_none() {
            self.row_value.set_default();
        };
        self.row_value.as_mut().unwrap()
    }

    // Take field
    pub fn take_row_value(&mut self) -> RowValue {
        self.row_value.take().unwrap_or_else(|| RowValue::new())
    }

    pub fn get_row_value(&self) -> &RowValue {
        self.row_value.as_ref().unwrap_or_else(|| RowValue::default_instance())
    }
}

impl ::protobuf::Message for CmdCommitThenGetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.row_value));
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
        for value in self.row_value.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.row_value.as_ref() {
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
        ::std::any::TypeId::of::<CmdCommitThenGetResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdCommitThenGetResponse {
    fn new() -> CmdCommitThenGetResponse {
        CmdCommitThenGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdCommitThenGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "row_value",
                    CmdCommitThenGetResponse::has_row_value,
                    CmdCommitThenGetResponse::get_row_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdCommitThenGetResponse>(
                    "CmdCommitThenGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdCommitThenGetResponse {
    fn clear(&mut self) {
        self.clear_row_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCommitThenGetResponse {
    fn eq(&self, other: &CmdCommitThenGetResponse) -> bool {
        self.row_value == other.row_value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdCommitThenGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdBatchGetRequest {
    // message fields
    rows: ::protobuf::RepeatedField<Row>,
    ts: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdBatchGetRequest {}

impl CmdBatchGetRequest {
    pub fn new() -> CmdBatchGetRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdBatchGetRequest {
        static mut instance: ::protobuf::lazy::Lazy<CmdBatchGetRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdBatchGetRequest,
        };
        unsafe {
            instance.get(|| {
                CmdBatchGetRequest {
                    rows: ::protobuf::RepeatedField::new(),
                    ts: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kvrpcpb.Row rows = 1;

    pub fn clear_rows(&mut self) {
        self.rows.clear();
    }

    // Param is passed by value, moved
    pub fn set_rows(&mut self, v: ::protobuf::RepeatedField<Row>) {
        self.rows = v;
    }

    // Mutable pointer to the field.
    pub fn mut_rows(&mut self) -> &mut ::protobuf::RepeatedField<Row> {
        &mut self.rows
    }

    // Take field
    pub fn take_rows(&mut self) -> ::protobuf::RepeatedField<Row> {
        ::std::mem::replace(&mut self.rows, ::protobuf::RepeatedField::new())
    }

    pub fn get_rows(&self) -> &[Row] {
        &self.rows
    }

    // optional uint64 ts = 2;

    pub fn clear_ts(&mut self) {
        self.ts = ::std::option::Option::None;
    }

    pub fn has_ts(&self) -> bool {
        self.ts.is_some()
    }

    // Param is passed by value, moved
    pub fn set_ts(&mut self, v: u64) {
        self.ts = ::std::option::Option::Some(v);
    }

    pub fn get_ts(&self) -> u64 {
        self.ts.unwrap_or(0)
    }
}

impl ::protobuf::Message for CmdBatchGetRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.rows));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.ts = ::std::option::Option::Some(tmp);
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
        for value in self.rows.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.ts.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.rows.iter() {
            try!(os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.ts {
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
        ::std::any::TypeId::of::<CmdBatchGetRequest>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdBatchGetRequest {
    fn new() -> CmdBatchGetRequest {
        CmdBatchGetRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdBatchGetRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "rows",
                    CmdBatchGetRequest::get_rows,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "ts",
                    CmdBatchGetRequest::has_ts,
                    CmdBatchGetRequest::get_ts,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdBatchGetRequest>(
                    "CmdBatchGetRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdBatchGetRequest {
    fn clear(&mut self) {
        self.clear_rows();
        self.clear_ts();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdBatchGetRequest {
    fn eq(&self, other: &CmdBatchGetRequest) -> bool {
        self.rows == other.rows &&
        self.ts == other.ts &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdBatchGetRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdBatchGetResponse {
    // message fields
    row_values: ::protobuf::RepeatedField<RowValue>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CmdBatchGetResponse {}

impl CmdBatchGetResponse {
    pub fn new() -> CmdBatchGetResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CmdBatchGetResponse {
        static mut instance: ::protobuf::lazy::Lazy<CmdBatchGetResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CmdBatchGetResponse,
        };
        unsafe {
            instance.get(|| {
                CmdBatchGetResponse {
                    row_values: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kvrpcpb.RowValue row_values = 1;

    pub fn clear_row_values(&mut self) {
        self.row_values.clear();
    }

    // Param is passed by value, moved
    pub fn set_row_values(&mut self, v: ::protobuf::RepeatedField<RowValue>) {
        self.row_values = v;
    }

    // Mutable pointer to the field.
    pub fn mut_row_values(&mut self) -> &mut ::protobuf::RepeatedField<RowValue> {
        &mut self.row_values
    }

    // Take field
    pub fn take_row_values(&mut self) -> ::protobuf::RepeatedField<RowValue> {
        ::std::mem::replace(&mut self.row_values, ::protobuf::RepeatedField::new())
    }

    pub fn get_row_values(&self) -> &[RowValue] {
        &self.row_values
    }
}

impl ::protobuf::Message for CmdBatchGetResponse {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.row_values));
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
        for value in self.row_values.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.row_values.iter() {
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
        ::std::any::TypeId::of::<CmdBatchGetResponse>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for CmdBatchGetResponse {
    fn new() -> CmdBatchGetResponse {
        CmdBatchGetResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CmdBatchGetResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "row_values",
                    CmdBatchGetResponse::get_row_values,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CmdBatchGetResponse>(
                    "CmdBatchGetResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CmdBatchGetResponse {
    fn clear(&mut self) {
        self.clear_row_values();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdBatchGetResponse {
    fn eq(&self, other: &CmdBatchGetResponse) -> bool {
        self.row_values == other.row_values &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdBatchGetResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Request {
    // message fields
    field_type: ::std::option::Option<MessageType>,
    context: ::protobuf::SingularPtrField<Context>,
    cmd_get_req: ::protobuf::SingularPtrField<CmdGetRequest>,
    cmd_scan_req: ::protobuf::SingularPtrField<CmdScanRequest>,
    cmd_prewrite_req: ::protobuf::SingularPtrField<CmdPrewriteRequest>,
    cmd_commit_req: ::protobuf::SingularPtrField<CmdCommitRequest>,
    cmd_cleanup_req: ::protobuf::SingularPtrField<CmdCleanupRequest>,
    cmd_rb_get_req: ::protobuf::SingularPtrField<CmdRollbackThenGetRequest>,
    cmd_commit_get_req: ::protobuf::SingularPtrField<CmdCommitThenGetRequest>,
    cmd_batch_get_req: ::protobuf::SingularPtrField<CmdBatchGetRequest>,
    cmd_batch_rollback_req: ::protobuf::SingularPtrField<CmdBatchRollbackRequest>,
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
                    field_type: ::std::option::Option::None,
                    context: ::protobuf::SingularPtrField::none(),
                    cmd_get_req: ::protobuf::SingularPtrField::none(),
                    cmd_scan_req: ::protobuf::SingularPtrField::none(),
                    cmd_prewrite_req: ::protobuf::SingularPtrField::none(),
                    cmd_commit_req: ::protobuf::SingularPtrField::none(),
                    cmd_cleanup_req: ::protobuf::SingularPtrField::none(),
                    cmd_rb_get_req: ::protobuf::SingularPtrField::none(),
                    cmd_commit_get_req: ::protobuf::SingularPtrField::none(),
                    cmd_batch_get_req: ::protobuf::SingularPtrField::none(),
                    cmd_batch_rollback_req: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.MessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MessageType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> MessageType {
        self.field_type.unwrap_or(MessageType::CmdGet)
    }

    // optional .kvrpcpb.Context context = 2;

    pub fn clear_context(&mut self) {
        self.context.clear();
    }

    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }

    // Param is passed by value, moved
    pub fn set_context(&mut self, v: Context) {
        self.context = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context.set_default();
        };
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context(&self) -> &Context {
        self.context.as_ref().unwrap_or_else(|| Context::default_instance())
    }

    // optional .kvrpcpb.CmdGetRequest cmd_get_req = 3;

    pub fn clear_cmd_get_req(&mut self) {
        self.cmd_get_req.clear();
    }

    pub fn has_cmd_get_req(&self) -> bool {
        self.cmd_get_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_get_req(&mut self, v: CmdGetRequest) {
        self.cmd_get_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_get_req(&mut self) -> &mut CmdGetRequest {
        if self.cmd_get_req.is_none() {
            self.cmd_get_req.set_default();
        };
        self.cmd_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_get_req(&mut self) -> CmdGetRequest {
        self.cmd_get_req.take().unwrap_or_else(|| CmdGetRequest::new())
    }

    pub fn get_cmd_get_req(&self) -> &CmdGetRequest {
        self.cmd_get_req.as_ref().unwrap_or_else(|| CmdGetRequest::default_instance())
    }

    // optional .kvrpcpb.CmdScanRequest cmd_scan_req = 4;

    pub fn clear_cmd_scan_req(&mut self) {
        self.cmd_scan_req.clear();
    }

    pub fn has_cmd_scan_req(&self) -> bool {
        self.cmd_scan_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_scan_req(&mut self, v: CmdScanRequest) {
        self.cmd_scan_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_scan_req(&mut self) -> &mut CmdScanRequest {
        if self.cmd_scan_req.is_none() {
            self.cmd_scan_req.set_default();
        };
        self.cmd_scan_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_scan_req(&mut self) -> CmdScanRequest {
        self.cmd_scan_req.take().unwrap_or_else(|| CmdScanRequest::new())
    }

    pub fn get_cmd_scan_req(&self) -> &CmdScanRequest {
        self.cmd_scan_req.as_ref().unwrap_or_else(|| CmdScanRequest::default_instance())
    }

    // optional .kvrpcpb.CmdPrewriteRequest cmd_prewrite_req = 5;

    pub fn clear_cmd_prewrite_req(&mut self) {
        self.cmd_prewrite_req.clear();
    }

    pub fn has_cmd_prewrite_req(&self) -> bool {
        self.cmd_prewrite_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_prewrite_req(&mut self, v: CmdPrewriteRequest) {
        self.cmd_prewrite_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_prewrite_req(&mut self) -> &mut CmdPrewriteRequest {
        if self.cmd_prewrite_req.is_none() {
            self.cmd_prewrite_req.set_default();
        };
        self.cmd_prewrite_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_prewrite_req(&mut self) -> CmdPrewriteRequest {
        self.cmd_prewrite_req.take().unwrap_or_else(|| CmdPrewriteRequest::new())
    }

    pub fn get_cmd_prewrite_req(&self) -> &CmdPrewriteRequest {
        self.cmd_prewrite_req.as_ref().unwrap_or_else(|| CmdPrewriteRequest::default_instance())
    }

    // optional .kvrpcpb.CmdCommitRequest cmd_commit_req = 6;

    pub fn clear_cmd_commit_req(&mut self) {
        self.cmd_commit_req.clear();
    }

    pub fn has_cmd_commit_req(&self) -> bool {
        self.cmd_commit_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_commit_req(&mut self, v: CmdCommitRequest) {
        self.cmd_commit_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_commit_req(&mut self) -> &mut CmdCommitRequest {
        if self.cmd_commit_req.is_none() {
            self.cmd_commit_req.set_default();
        };
        self.cmd_commit_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_commit_req(&mut self) -> CmdCommitRequest {
        self.cmd_commit_req.take().unwrap_or_else(|| CmdCommitRequest::new())
    }

    pub fn get_cmd_commit_req(&self) -> &CmdCommitRequest {
        self.cmd_commit_req.as_ref().unwrap_or_else(|| CmdCommitRequest::default_instance())
    }

    // optional .kvrpcpb.CmdCleanupRequest cmd_cleanup_req = 7;

    pub fn clear_cmd_cleanup_req(&mut self) {
        self.cmd_cleanup_req.clear();
    }

    pub fn has_cmd_cleanup_req(&self) -> bool {
        self.cmd_cleanup_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_cleanup_req(&mut self, v: CmdCleanupRequest) {
        self.cmd_cleanup_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_cleanup_req(&mut self) -> &mut CmdCleanupRequest {
        if self.cmd_cleanup_req.is_none() {
            self.cmd_cleanup_req.set_default();
        };
        self.cmd_cleanup_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_cleanup_req(&mut self) -> CmdCleanupRequest {
        self.cmd_cleanup_req.take().unwrap_or_else(|| CmdCleanupRequest::new())
    }

    pub fn get_cmd_cleanup_req(&self) -> &CmdCleanupRequest {
        self.cmd_cleanup_req.as_ref().unwrap_or_else(|| CmdCleanupRequest::default_instance())
    }

    // optional .kvrpcpb.CmdRollbackThenGetRequest cmd_rb_get_req = 8;

    pub fn clear_cmd_rb_get_req(&mut self) {
        self.cmd_rb_get_req.clear();
    }

    pub fn has_cmd_rb_get_req(&self) -> bool {
        self.cmd_rb_get_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_rb_get_req(&mut self, v: CmdRollbackThenGetRequest) {
        self.cmd_rb_get_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_rb_get_req(&mut self) -> &mut CmdRollbackThenGetRequest {
        if self.cmd_rb_get_req.is_none() {
            self.cmd_rb_get_req.set_default();
        };
        self.cmd_rb_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_rb_get_req(&mut self) -> CmdRollbackThenGetRequest {
        self.cmd_rb_get_req.take().unwrap_or_else(|| CmdRollbackThenGetRequest::new())
    }

    pub fn get_cmd_rb_get_req(&self) -> &CmdRollbackThenGetRequest {
        self.cmd_rb_get_req.as_ref().unwrap_or_else(|| CmdRollbackThenGetRequest::default_instance())
    }

    // optional .kvrpcpb.CmdCommitThenGetRequest cmd_commit_get_req = 9;

    pub fn clear_cmd_commit_get_req(&mut self) {
        self.cmd_commit_get_req.clear();
    }

    pub fn has_cmd_commit_get_req(&self) -> bool {
        self.cmd_commit_get_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_commit_get_req(&mut self, v: CmdCommitThenGetRequest) {
        self.cmd_commit_get_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_commit_get_req(&mut self) -> &mut CmdCommitThenGetRequest {
        if self.cmd_commit_get_req.is_none() {
            self.cmd_commit_get_req.set_default();
        };
        self.cmd_commit_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_commit_get_req(&mut self) -> CmdCommitThenGetRequest {
        self.cmd_commit_get_req.take().unwrap_or_else(|| CmdCommitThenGetRequest::new())
    }

    pub fn get_cmd_commit_get_req(&self) -> &CmdCommitThenGetRequest {
        self.cmd_commit_get_req.as_ref().unwrap_or_else(|| CmdCommitThenGetRequest::default_instance())
    }

    // optional .kvrpcpb.CmdBatchGetRequest cmd_batch_get_req = 10;

    pub fn clear_cmd_batch_get_req(&mut self) {
        self.cmd_batch_get_req.clear();
    }

    pub fn has_cmd_batch_get_req(&self) -> bool {
        self.cmd_batch_get_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_batch_get_req(&mut self, v: CmdBatchGetRequest) {
        self.cmd_batch_get_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_batch_get_req(&mut self) -> &mut CmdBatchGetRequest {
        if self.cmd_batch_get_req.is_none() {
            self.cmd_batch_get_req.set_default();
        };
        self.cmd_batch_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_batch_get_req(&mut self) -> CmdBatchGetRequest {
        self.cmd_batch_get_req.take().unwrap_or_else(|| CmdBatchGetRequest::new())
    }

    pub fn get_cmd_batch_get_req(&self) -> &CmdBatchGetRequest {
        self.cmd_batch_get_req.as_ref().unwrap_or_else(|| CmdBatchGetRequest::default_instance())
    }

    // optional .kvrpcpb.CmdBatchRollbackRequest cmd_batch_rollback_req = 11;

    pub fn clear_cmd_batch_rollback_req(&mut self) {
        self.cmd_batch_rollback_req.clear();
    }

    pub fn has_cmd_batch_rollback_req(&self) -> bool {
        self.cmd_batch_rollback_req.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_batch_rollback_req(&mut self, v: CmdBatchRollbackRequest) {
        self.cmd_batch_rollback_req = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_batch_rollback_req(&mut self) -> &mut CmdBatchRollbackRequest {
        if self.cmd_batch_rollback_req.is_none() {
            self.cmd_batch_rollback_req.set_default();
        };
        self.cmd_batch_rollback_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_batch_rollback_req(&mut self) -> CmdBatchRollbackRequest {
        self.cmd_batch_rollback_req.take().unwrap_or_else(|| CmdBatchRollbackRequest::new())
    }

    pub fn get_cmd_batch_rollback_req(&self) -> &CmdBatchRollbackRequest {
        self.cmd_batch_rollback_req.as_ref().unwrap_or_else(|| CmdBatchRollbackRequest::default_instance())
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_get_req));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_scan_req));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_prewrite_req));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_commit_req));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_cleanup_req));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_rb_get_req));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_commit_get_req));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_batch_get_req));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_batch_rollback_req));
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
        for value in self.context.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_get_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_scan_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_prewrite_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_commit_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_cleanup_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_rb_get_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_commit_get_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_batch_get_req.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_batch_rollback_req.iter() {
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
        if let Some(v) = self.context.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_get_req.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_scan_req.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_prewrite_req.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_commit_req.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_cleanup_req.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_rb_get_req.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_commit_get_req.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_batch_get_req.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_batch_rollback_req.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Request::has_field_type,
                    Request::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "context",
                    Request::has_context,
                    Request::get_context,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_get_req",
                    Request::has_cmd_get_req,
                    Request::get_cmd_get_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_scan_req",
                    Request::has_cmd_scan_req,
                    Request::get_cmd_scan_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_prewrite_req",
                    Request::has_cmd_prewrite_req,
                    Request::get_cmd_prewrite_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_commit_req",
                    Request::has_cmd_commit_req,
                    Request::get_cmd_commit_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_cleanup_req",
                    Request::has_cmd_cleanup_req,
                    Request::get_cmd_cleanup_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_rb_get_req",
                    Request::has_cmd_rb_get_req,
                    Request::get_cmd_rb_get_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_commit_get_req",
                    Request::has_cmd_commit_get_req,
                    Request::get_cmd_commit_get_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_batch_get_req",
                    Request::has_cmd_batch_get_req,
                    Request::get_cmd_batch_get_req,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_batch_rollback_req",
                    Request::has_cmd_batch_rollback_req,
                    Request::get_cmd_batch_rollback_req,
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
        self.clear_field_type();
        self.clear_context();
        self.clear_cmd_get_req();
        self.clear_cmd_scan_req();
        self.clear_cmd_prewrite_req();
        self.clear_cmd_commit_req();
        self.clear_cmd_cleanup_req();
        self.clear_cmd_rb_get_req();
        self.clear_cmd_commit_get_req();
        self.clear_cmd_batch_get_req();
        self.clear_cmd_batch_rollback_req();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Request {
    fn eq(&self, other: &Request) -> bool {
        self.field_type == other.field_type &&
        self.context == other.context &&
        self.cmd_get_req == other.cmd_get_req &&
        self.cmd_scan_req == other.cmd_scan_req &&
        self.cmd_prewrite_req == other.cmd_prewrite_req &&
        self.cmd_commit_req == other.cmd_commit_req &&
        self.cmd_cleanup_req == other.cmd_cleanup_req &&
        self.cmd_rb_get_req == other.cmd_rb_get_req &&
        self.cmd_commit_get_req == other.cmd_commit_get_req &&
        self.cmd_batch_get_req == other.cmd_batch_get_req &&
        self.cmd_batch_rollback_req == other.cmd_batch_rollback_req &&
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
    field_type: ::std::option::Option<MessageType>,
    region_error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    cmd_get_resp: ::protobuf::SingularPtrField<CmdGetResponse>,
    cmd_scan_resp: ::protobuf::SingularPtrField<CmdScanResponse>,
    cmd_prewrite_resp: ::protobuf::SingularPtrField<CmdPrewriteResponse>,
    cmd_commit_resp: ::protobuf::SingularPtrField<CmdCommitResponse>,
    cmd_cleanup_resp: ::protobuf::SingularPtrField<CmdCleanupResponse>,
    cmd_rb_get_resp: ::protobuf::SingularPtrField<CmdRollbackThenGetResponse>,
    cmd_commit_get_resp: ::protobuf::SingularPtrField<CmdCommitThenGetResponse>,
    cmd_batch_get_resp: ::protobuf::SingularPtrField<CmdBatchGetResponse>,
    cmd_batch_rollback_resp: ::protobuf::SingularPtrField<CmdBatchRollbackResponse>,
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
                    field_type: ::std::option::Option::None,
                    region_error: ::protobuf::SingularPtrField::none(),
                    cmd_get_resp: ::protobuf::SingularPtrField::none(),
                    cmd_scan_resp: ::protobuf::SingularPtrField::none(),
                    cmd_prewrite_resp: ::protobuf::SingularPtrField::none(),
                    cmd_commit_resp: ::protobuf::SingularPtrField::none(),
                    cmd_cleanup_resp: ::protobuf::SingularPtrField::none(),
                    cmd_rb_get_resp: ::protobuf::SingularPtrField::none(),
                    cmd_commit_get_resp: ::protobuf::SingularPtrField::none(),
                    cmd_batch_get_resp: ::protobuf::SingularPtrField::none(),
                    cmd_batch_rollback_resp: ::protobuf::SingularPtrField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.MessageType type = 1;

    pub fn clear_field_type(&mut self) {
        self.field_type = ::std::option::Option::None;
    }

    pub fn has_field_type(&self) -> bool {
        self.field_type.is_some()
    }

    // Param is passed by value, moved
    pub fn set_field_type(&mut self, v: MessageType) {
        self.field_type = ::std::option::Option::Some(v);
    }

    pub fn get_field_type(&self) -> MessageType {
        self.field_type.unwrap_or(MessageType::CmdGet)
    }

    // optional .errorpb.Error region_error = 2;

    pub fn clear_region_error(&mut self) {
        self.region_error.clear();
    }

    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        };
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error(&self) -> &super::errorpb::Error {
        self.region_error.as_ref().unwrap_or_else(|| super::errorpb::Error::default_instance())
    }

    // optional .kvrpcpb.CmdGetResponse cmd_get_resp = 3;

    pub fn clear_cmd_get_resp(&mut self) {
        self.cmd_get_resp.clear();
    }

    pub fn has_cmd_get_resp(&self) -> bool {
        self.cmd_get_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_get_resp(&mut self, v: CmdGetResponse) {
        self.cmd_get_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_get_resp(&mut self) -> &mut CmdGetResponse {
        if self.cmd_get_resp.is_none() {
            self.cmd_get_resp.set_default();
        };
        self.cmd_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_get_resp(&mut self) -> CmdGetResponse {
        self.cmd_get_resp.take().unwrap_or_else(|| CmdGetResponse::new())
    }

    pub fn get_cmd_get_resp(&self) -> &CmdGetResponse {
        self.cmd_get_resp.as_ref().unwrap_or_else(|| CmdGetResponse::default_instance())
    }

    // optional .kvrpcpb.CmdScanResponse cmd_scan_resp = 4;

    pub fn clear_cmd_scan_resp(&mut self) {
        self.cmd_scan_resp.clear();
    }

    pub fn has_cmd_scan_resp(&self) -> bool {
        self.cmd_scan_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_scan_resp(&mut self, v: CmdScanResponse) {
        self.cmd_scan_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_scan_resp(&mut self) -> &mut CmdScanResponse {
        if self.cmd_scan_resp.is_none() {
            self.cmd_scan_resp.set_default();
        };
        self.cmd_scan_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_scan_resp(&mut self) -> CmdScanResponse {
        self.cmd_scan_resp.take().unwrap_or_else(|| CmdScanResponse::new())
    }

    pub fn get_cmd_scan_resp(&self) -> &CmdScanResponse {
        self.cmd_scan_resp.as_ref().unwrap_or_else(|| CmdScanResponse::default_instance())
    }

    // optional .kvrpcpb.CmdPrewriteResponse cmd_prewrite_resp = 5;

    pub fn clear_cmd_prewrite_resp(&mut self) {
        self.cmd_prewrite_resp.clear();
    }

    pub fn has_cmd_prewrite_resp(&self) -> bool {
        self.cmd_prewrite_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_prewrite_resp(&mut self, v: CmdPrewriteResponse) {
        self.cmd_prewrite_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_prewrite_resp(&mut self) -> &mut CmdPrewriteResponse {
        if self.cmd_prewrite_resp.is_none() {
            self.cmd_prewrite_resp.set_default();
        };
        self.cmd_prewrite_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_prewrite_resp(&mut self) -> CmdPrewriteResponse {
        self.cmd_prewrite_resp.take().unwrap_or_else(|| CmdPrewriteResponse::new())
    }

    pub fn get_cmd_prewrite_resp(&self) -> &CmdPrewriteResponse {
        self.cmd_prewrite_resp.as_ref().unwrap_or_else(|| CmdPrewriteResponse::default_instance())
    }

    // optional .kvrpcpb.CmdCommitResponse cmd_commit_resp = 6;

    pub fn clear_cmd_commit_resp(&mut self) {
        self.cmd_commit_resp.clear();
    }

    pub fn has_cmd_commit_resp(&self) -> bool {
        self.cmd_commit_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_commit_resp(&mut self, v: CmdCommitResponse) {
        self.cmd_commit_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_commit_resp(&mut self) -> &mut CmdCommitResponse {
        if self.cmd_commit_resp.is_none() {
            self.cmd_commit_resp.set_default();
        };
        self.cmd_commit_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_commit_resp(&mut self) -> CmdCommitResponse {
        self.cmd_commit_resp.take().unwrap_or_else(|| CmdCommitResponse::new())
    }

    pub fn get_cmd_commit_resp(&self) -> &CmdCommitResponse {
        self.cmd_commit_resp.as_ref().unwrap_or_else(|| CmdCommitResponse::default_instance())
    }

    // optional .kvrpcpb.CmdCleanupResponse cmd_cleanup_resp = 7;

    pub fn clear_cmd_cleanup_resp(&mut self) {
        self.cmd_cleanup_resp.clear();
    }

    pub fn has_cmd_cleanup_resp(&self) -> bool {
        self.cmd_cleanup_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_cleanup_resp(&mut self, v: CmdCleanupResponse) {
        self.cmd_cleanup_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_cleanup_resp(&mut self) -> &mut CmdCleanupResponse {
        if self.cmd_cleanup_resp.is_none() {
            self.cmd_cleanup_resp.set_default();
        };
        self.cmd_cleanup_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_cleanup_resp(&mut self) -> CmdCleanupResponse {
        self.cmd_cleanup_resp.take().unwrap_or_else(|| CmdCleanupResponse::new())
    }

    pub fn get_cmd_cleanup_resp(&self) -> &CmdCleanupResponse {
        self.cmd_cleanup_resp.as_ref().unwrap_or_else(|| CmdCleanupResponse::default_instance())
    }

    // optional .kvrpcpb.CmdRollbackThenGetResponse cmd_rb_get_resp = 8;

    pub fn clear_cmd_rb_get_resp(&mut self) {
        self.cmd_rb_get_resp.clear();
    }

    pub fn has_cmd_rb_get_resp(&self) -> bool {
        self.cmd_rb_get_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_rb_get_resp(&mut self, v: CmdRollbackThenGetResponse) {
        self.cmd_rb_get_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_rb_get_resp(&mut self) -> &mut CmdRollbackThenGetResponse {
        if self.cmd_rb_get_resp.is_none() {
            self.cmd_rb_get_resp.set_default();
        };
        self.cmd_rb_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_rb_get_resp(&mut self) -> CmdRollbackThenGetResponse {
        self.cmd_rb_get_resp.take().unwrap_or_else(|| CmdRollbackThenGetResponse::new())
    }

    pub fn get_cmd_rb_get_resp(&self) -> &CmdRollbackThenGetResponse {
        self.cmd_rb_get_resp.as_ref().unwrap_or_else(|| CmdRollbackThenGetResponse::default_instance())
    }

    // optional .kvrpcpb.CmdCommitThenGetResponse cmd_commit_get_resp = 9;

    pub fn clear_cmd_commit_get_resp(&mut self) {
        self.cmd_commit_get_resp.clear();
    }

    pub fn has_cmd_commit_get_resp(&self) -> bool {
        self.cmd_commit_get_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_commit_get_resp(&mut self, v: CmdCommitThenGetResponse) {
        self.cmd_commit_get_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_commit_get_resp(&mut self) -> &mut CmdCommitThenGetResponse {
        if self.cmd_commit_get_resp.is_none() {
            self.cmd_commit_get_resp.set_default();
        };
        self.cmd_commit_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_commit_get_resp(&mut self) -> CmdCommitThenGetResponse {
        self.cmd_commit_get_resp.take().unwrap_or_else(|| CmdCommitThenGetResponse::new())
    }

    pub fn get_cmd_commit_get_resp(&self) -> &CmdCommitThenGetResponse {
        self.cmd_commit_get_resp.as_ref().unwrap_or_else(|| CmdCommitThenGetResponse::default_instance())
    }

    // optional .kvrpcpb.CmdBatchGetResponse cmd_batch_get_resp = 10;

    pub fn clear_cmd_batch_get_resp(&mut self) {
        self.cmd_batch_get_resp.clear();
    }

    pub fn has_cmd_batch_get_resp(&self) -> bool {
        self.cmd_batch_get_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_batch_get_resp(&mut self, v: CmdBatchGetResponse) {
        self.cmd_batch_get_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_batch_get_resp(&mut self) -> &mut CmdBatchGetResponse {
        if self.cmd_batch_get_resp.is_none() {
            self.cmd_batch_get_resp.set_default();
        };
        self.cmd_batch_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_batch_get_resp(&mut self) -> CmdBatchGetResponse {
        self.cmd_batch_get_resp.take().unwrap_or_else(|| CmdBatchGetResponse::new())
    }

    pub fn get_cmd_batch_get_resp(&self) -> &CmdBatchGetResponse {
        self.cmd_batch_get_resp.as_ref().unwrap_or_else(|| CmdBatchGetResponse::default_instance())
    }

    // optional .kvrpcpb.CmdBatchRollbackResponse cmd_batch_rollback_resp = 11;

    pub fn clear_cmd_batch_rollback_resp(&mut self) {
        self.cmd_batch_rollback_resp.clear();
    }

    pub fn has_cmd_batch_rollback_resp(&self) -> bool {
        self.cmd_batch_rollback_resp.is_some()
    }

    // Param is passed by value, moved
    pub fn set_cmd_batch_rollback_resp(&mut self, v: CmdBatchRollbackResponse) {
        self.cmd_batch_rollback_resp = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cmd_batch_rollback_resp(&mut self) -> &mut CmdBatchRollbackResponse {
        if self.cmd_batch_rollback_resp.is_none() {
            self.cmd_batch_rollback_resp.set_default();
        };
        self.cmd_batch_rollback_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_batch_rollback_resp(&mut self) -> CmdBatchRollbackResponse {
        self.cmd_batch_rollback_resp.take().unwrap_or_else(|| CmdBatchRollbackResponse::new())
    }

    pub fn get_cmd_batch_rollback_resp(&self) -> &CmdBatchRollbackResponse {
        self.cmd_batch_rollback_resp.as_ref().unwrap_or_else(|| CmdBatchRollbackResponse::default_instance())
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.field_type = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_error));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_get_resp));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_scan_resp));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_prewrite_resp));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_commit_resp));
                },
                7 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_cleanup_resp));
                },
                8 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_rb_get_resp));
                },
                9 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_commit_get_resp));
                },
                10 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_batch_get_resp));
                },
                11 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.cmd_batch_rollback_resp));
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
        for value in self.region_error.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_get_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_scan_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_prewrite_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_commit_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_cleanup_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_rb_get_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_commit_get_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_batch_get_resp.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.cmd_batch_rollback_resp.iter() {
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
        if let Some(v) = self.region_error.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_get_resp.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_scan_resp.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_prewrite_resp.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_commit_resp.as_ref() {
            try!(os.write_tag(6, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_cleanup_resp.as_ref() {
            try!(os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_rb_get_resp.as_ref() {
            try!(os.write_tag(8, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_commit_get_resp.as_ref() {
            try!(os.write_tag(9, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_batch_get_resp.as_ref() {
            try!(os.write_tag(10, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.cmd_batch_rollback_resp.as_ref() {
            try!(os.write_tag(11, ::protobuf::wire_format::WireTypeLengthDelimited));
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
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "type",
                    Response::has_field_type,
                    Response::get_field_type,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region_error",
                    Response::has_region_error,
                    Response::get_region_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_get_resp",
                    Response::has_cmd_get_resp,
                    Response::get_cmd_get_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_scan_resp",
                    Response::has_cmd_scan_resp,
                    Response::get_cmd_scan_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_prewrite_resp",
                    Response::has_cmd_prewrite_resp,
                    Response::get_cmd_prewrite_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_commit_resp",
                    Response::has_cmd_commit_resp,
                    Response::get_cmd_commit_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_cleanup_resp",
                    Response::has_cmd_cleanup_resp,
                    Response::get_cmd_cleanup_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_rb_get_resp",
                    Response::has_cmd_rb_get_resp,
                    Response::get_cmd_rb_get_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_commit_get_resp",
                    Response::has_cmd_commit_get_resp,
                    Response::get_cmd_commit_get_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_batch_get_resp",
                    Response::has_cmd_batch_get_resp,
                    Response::get_cmd_batch_get_resp,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "cmd_batch_rollback_resp",
                    Response::has_cmd_batch_rollback_resp,
                    Response::get_cmd_batch_rollback_resp,
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
        self.clear_field_type();
        self.clear_region_error();
        self.clear_cmd_get_resp();
        self.clear_cmd_scan_resp();
        self.clear_cmd_prewrite_resp();
        self.clear_cmd_commit_resp();
        self.clear_cmd_cleanup_resp();
        self.clear_cmd_rb_get_resp();
        self.clear_cmd_commit_get_resp();
        self.clear_cmd_batch_get_resp();
        self.clear_cmd_batch_rollback_resp();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Response {
    fn eq(&self, other: &Response) -> bool {
        self.field_type == other.field_type &&
        self.region_error == other.region_error &&
        self.cmd_get_resp == other.cmd_get_resp &&
        self.cmd_scan_resp == other.cmd_scan_resp &&
        self.cmd_prewrite_resp == other.cmd_prewrite_resp &&
        self.cmd_commit_resp == other.cmd_commit_resp &&
        self.cmd_cleanup_resp == other.cmd_cleanup_resp &&
        self.cmd_rb_get_resp == other.cmd_rb_get_resp &&
        self.cmd_commit_get_resp == other.cmd_commit_get_resp &&
        self.cmd_batch_get_resp == other.cmd_batch_get_resp &&
        self.cmd_batch_rollback_resp == other.cmd_batch_rollback_resp &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Response {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum MessageType {
    CmdGet = 1,
    CmdScan = 2,
    CmdPrewrite = 3,
    CmdCommit = 4,
    CmdCleanup = 5,
    CmdRollbackThenGet = 6,
    CmdCommitThenGet = 7,
    CmdBatchGet = 8,
    CmdBatchRollback = 9,
}

impl ::protobuf::ProtobufEnum for MessageType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<MessageType> {
        match value {
            1 => ::std::option::Option::Some(MessageType::CmdGet),
            2 => ::std::option::Option::Some(MessageType::CmdScan),
            3 => ::std::option::Option::Some(MessageType::CmdPrewrite),
            4 => ::std::option::Option::Some(MessageType::CmdCommit),
            5 => ::std::option::Option::Some(MessageType::CmdCleanup),
            6 => ::std::option::Option::Some(MessageType::CmdRollbackThenGet),
            7 => ::std::option::Option::Some(MessageType::CmdCommitThenGet),
            8 => ::std::option::Option::Some(MessageType::CmdBatchGet),
            9 => ::std::option::Option::Some(MessageType::CmdBatchRollback),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [MessageType] = &[
            MessageType::CmdGet,
            MessageType::CmdScan,
            MessageType::CmdPrewrite,
            MessageType::CmdCommit,
            MessageType::CmdCleanup,
            MessageType::CmdRollbackThenGet,
            MessageType::CmdCommitThenGet,
            MessageType::CmdBatchGet,
            MessageType::CmdBatchRollback,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<MessageType>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("MessageType", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for MessageType {
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Op {
    Put = 1,
    Del = 2,
    Lock = 3,
}

impl ::protobuf::ProtobufEnum for Op {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Op> {
        match value {
            1 => ::std::option::Option::Some(Op::Put),
            2 => ::std::option::Option::Some(Op::Del),
            3 => ::std::option::Option::Some(Op::Lock),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Op] = &[
            Op::Put,
            Op::Del,
            Op::Lock,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<Op>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Op", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Op {
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x07, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x1a, 0x0c, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x0d, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x34, 0x0a, 0x08, 0x4c, 0x6f, 0x63, 0x6b, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x0f, 0x0a, 0x07, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x12, 0x0a, 0x0a, 0x02, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x12, 0x0b,
    0x0a, 0x03, 0x72, 0x6f, 0x77, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x4f, 0x0a, 0x08, 0x4b,
    0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x21, 0x0a, 0x06, 0x6c, 0x6f, 0x63, 0x6b, 0x65,
    0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70,
    0x62, 0x2e, 0x4c, 0x6f, 0x63, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x65,
    0x74, 0x72, 0x79, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x12, 0x0d, 0x0a,
    0x05, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x22, 0x63, 0x0a, 0x07,
    0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x12, 0x29, 0x0a, 0x0c, 0x72, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x13, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x45, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x1a, 0x0a, 0x04, 0x70, 0x65, 0x65, 0x72, 0x18, 0x03, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65,
    0x72, 0x22, 0x27, 0x0a, 0x03, 0x52, 0x6f, 0x77, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x6f, 0x77, 0x5f,
    0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x6f, 0x6c,
    0x75, 0x6d, 0x6e, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0c, 0x22, 0x5e, 0x0a, 0x08, 0x52, 0x6f,
    0x77, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x20, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x6f, 0x77, 0x5f,
    0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x6f, 0x6c,
    0x75, 0x6d, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03, 0x28, 0x0c, 0x22, 0x56, 0x0a, 0x08, 0x4d, 0x75,
    0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0f, 0x0a, 0x07, 0x72, 0x6f, 0x77, 0x5f, 0x6b, 0x65,
    0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x18, 0x0a, 0x03, 0x6f, 0x70, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0e, 0x32, 0x0b, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4f,
    0x70, 0x12, 0x0f, 0x0a, 0x07, 0x63, 0x6f, 0x6c, 0x75, 0x6d, 0x6e, 0x73, 0x18, 0x03, 0x20, 0x03,
    0x28, 0x0c, 0x12, 0x0e, 0x0a, 0x06, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x73, 0x18, 0x04, 0x20, 0x03,
    0x28, 0x0c, 0x22, 0x36, 0x0a, 0x0d, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x12, 0x19, 0x0a, 0x03, 0x72, 0x6f, 0x77, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x0c, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x52, 0x6f, 0x77, 0x12, 0x0a,
    0x0a, 0x02, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x22, 0x30, 0x0a, 0x0e, 0x43, 0x6d,
    0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1e, 0x0a, 0x03,
    0x72, 0x6f, 0x77, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70,
    0x63, 0x70, 0x62, 0x2e, 0x52, 0x6f, 0x77, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x4c, 0x0a, 0x0e,
    0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1f,
    0x0a, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x72, 0x6f, 0x77, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0c, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x52, 0x6f, 0x77, 0x12,
    0x0d, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x12, 0x0a,
    0x0a, 0x02, 0x74, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x22, 0x32, 0x0a, 0x0f, 0x43, 0x6d,
    0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x1f, 0x0a,
    0x04, 0x72, 0x6f, 0x77, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76,
    0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x52, 0x6f, 0x77, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x57,
    0x0a, 0x12, 0x43, 0x6d, 0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x24, 0x0a, 0x09, 0x6d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e,
    0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70,
    0x62, 0x2e, 0x4d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x0f, 0x0a, 0x07, 0x70, 0x72,
    0x69, 0x6d, 0x61, 0x72, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x0a, 0x0a, 0x02, 0x74,
    0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x22, 0x38, 0x0a, 0x13, 0x43, 0x6d, 0x64, 0x50, 0x72,
    0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x21,
    0x0a, 0x06, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x22, 0x45, 0x0a, 0x10, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x08, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x74,
    0x73, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x12, 0x0c, 0x0a, 0x04, 0x72, 0x6f, 0x77, 0x73, 0x18,
    0x02, 0x20, 0x03, 0x28, 0x0c, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f,
    0x74, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x22, 0x35, 0x0a, 0x11, 0x43, 0x6d, 0x64, 0x43,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x20, 0x0a,
    0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b,
    0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x22,
    0x33, 0x0a, 0x17, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x6f, 0x6c, 0x6c, 0x62,
    0x61, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0a, 0x0a, 0x02, 0x74, 0x73,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x12, 0x0c, 0x0a, 0x04, 0x72, 0x6f, 0x77, 0x73, 0x18, 0x02,
    0x20, 0x03, 0x28, 0x0c, 0x22, 0x3c, 0x0a, 0x18, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68,
    0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65,
    0x12, 0x20, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x22, 0x2c, 0x0a, 0x11, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70,
    0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x0b, 0x0a, 0x03, 0x72, 0x6f, 0x77, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x0a, 0x0a, 0x02, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x22, 0x49, 0x0a, 0x12, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x20, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x11, 0x0a, 0x09, 0x63, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x5f, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x22, 0x42, 0x0a, 0x19, 0x43,
    0x6d, 0x64, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x54, 0x68, 0x65, 0x6e, 0x47, 0x65,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x19, 0x0a, 0x03, 0x72, 0x6f, 0x77, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x52, 0x6f, 0x77, 0x12, 0x0a, 0x0a, 0x02, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x22,
    0x42, 0x0a, 0x1a, 0x43, 0x6d, 0x64, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x54, 0x68,
    0x65, 0x6e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x24, 0x0a,
    0x09, 0x72, 0x6f, 0x77, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x52, 0x6f, 0x77, 0x56, 0x61,
    0x6c, 0x75, 0x65, 0x22, 0x69, 0x0a, 0x17, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74,
    0x54, 0x68, 0x65, 0x6e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x19,
    0x0a, 0x03, 0x72, 0x6f, 0x77, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6b, 0x76,
    0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x52, 0x6f, 0x77, 0x12, 0x10, 0x0a, 0x08, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x5f, 0x74, 0x73, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x12, 0x11, 0x0a, 0x09, 0x63,
    0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x74, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x12, 0x0e,
    0x0a, 0x06, 0x67, 0x65, 0x74, 0x5f, 0x74, 0x73, 0x18, 0x05, 0x20, 0x01, 0x28, 0x04, 0x22, 0x40,
    0x0a, 0x18, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x54, 0x68, 0x65, 0x6e, 0x47,
    0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x24, 0x0a, 0x09, 0x72, 0x6f,
    0x77, 0x5f, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e,
    0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x52, 0x6f, 0x77, 0x56, 0x61, 0x6c, 0x75, 0x65,
    0x22, 0x3c, 0x0a, 0x12, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x1a, 0x0a, 0x04, 0x72, 0x6f, 0x77, 0x73, 0x18, 0x01,
    0x20, 0x03, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x52,
    0x6f, 0x77, 0x12, 0x0a, 0x0a, 0x02, 0x74, 0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x22, 0x3c,
    0x0a, 0x13, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x25, 0x0a, 0x0a, 0x72, 0x6f, 0x77, 0x5f, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70,
    0x63, 0x70, 0x62, 0x2e, 0x52, 0x6f, 0x77, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x22, 0xbf, 0x04, 0x0a,
    0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x22, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65,
    0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62,
    0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x21, 0x0a, 0x07,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e,
    0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x12,
    0x2b, 0x0a, 0x0b, 0x63, 0x6d, 0x64, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43,
    0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2d, 0x0a, 0x0c,
    0x63, 0x6d, 0x64, 0x5f, 0x73, 0x63, 0x61, 0x6e, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x04, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x17, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64,
    0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x35, 0x0a, 0x10, 0x63,
    0x6d, 0x64, 0x5f, 0x70, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x71, 0x18,
    0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x43, 0x6d, 0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x31, 0x0a, 0x0e, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74,
    0x5f, 0x72, 0x65, 0x71, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x6b, 0x76, 0x72,
    0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x33, 0x0a, 0x0f, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6c, 0x65,
    0x61, 0x6e, 0x75, 0x70, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61,
    0x6e, 0x75, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3a, 0x0a, 0x0e, 0x63, 0x6d,
    0x64, 0x5f, 0x72, 0x62, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x08, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x22, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64,
    0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x54, 0x68, 0x65, 0x6e, 0x47, 0x65, 0x74, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x3c, 0x0a, 0x12, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6f,
    0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x09, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x20, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64,
    0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x54, 0x68, 0x65, 0x6e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x36, 0x0a, 0x11, 0x63, 0x6d, 0x64, 0x5f, 0x62, 0x61, 0x74, 0x63,
    0x68, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1b, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74,
    0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x40, 0x0a, 0x16,
    0x63, 0x6d, 0x64, 0x5f, 0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x72, 0x6f, 0x6c, 0x6c, 0x62, 0x61,
    0x63, 0x6b, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x0b, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x6b,
    0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52,
    0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x22, 0xd5,
    0x04, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x22, 0x0a, 0x04, 0x74,
    0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14, 0x2e, 0x6b, 0x76, 0x72, 0x70,
    0x63, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12,
    0x24, 0x0a, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x2d, 0x0a, 0x0c, 0x63, 0x6d, 0x64, 0x5f, 0x67, 0x65, 0x74,
    0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x6b, 0x76,
    0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70,
    0x6f, 0x6e, 0x73, 0x65, 0x12, 0x2f, 0x0a, 0x0d, 0x63, 0x6d, 0x64, 0x5f, 0x73, 0x63, 0x61, 0x6e,
    0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x18, 0x2e, 0x6b, 0x76,
    0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x37, 0x0a, 0x11, 0x63, 0x6d, 0x64, 0x5f, 0x70, 0x72, 0x65,
    0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1c, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x50, 0x72,
    0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x33,
    0x0a, 0x0f, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x72, 0x65, 0x73,
    0x70, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70,
    0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x35, 0x0a, 0x10, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6c, 0x65, 0x61, 0x6e,
    0x75, 0x70, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e,
    0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e,
    0x75, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3c, 0x0a, 0x0f, 0x63, 0x6d,
    0x64, 0x5f, 0x72, 0x62, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x08, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d,
    0x64, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x54, 0x68, 0x65, 0x6e, 0x47, 0x65, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x3e, 0x0a, 0x13, 0x63, 0x6d, 0x64, 0x5f,
    0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18,
    0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x54, 0x68, 0x65, 0x6e, 0x47, 0x65, 0x74,
    0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x38, 0x0a, 0x12, 0x63, 0x6d, 0x64, 0x5f,
    0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x0a,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43,
    0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x42, 0x0a, 0x17, 0x63, 0x6d, 0x64, 0x5f, 0x62, 0x61, 0x74, 0x63, 0x68, 0x5f,
    0x72, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x0b, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d,
    0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x52, 0x65,
    0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x2a, 0xab, 0x01, 0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74,
    0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x10, 0x02, 0x12,
    0x0f, 0x0a, 0x0b, 0x43, 0x6d, 0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x10, 0x03,
    0x12, 0x0d, 0x0a, 0x09, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x10, 0x04, 0x12,
    0x0e, 0x0a, 0x0a, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x10, 0x05, 0x12,
    0x16, 0x0a, 0x12, 0x43, 0x6d, 0x64, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x54, 0x68,
    0x65, 0x6e, 0x47, 0x65, 0x74, 0x10, 0x06, 0x12, 0x14, 0x0a, 0x10, 0x43, 0x6d, 0x64, 0x43, 0x6f,
    0x6d, 0x6d, 0x69, 0x74, 0x54, 0x68, 0x65, 0x6e, 0x47, 0x65, 0x74, 0x10, 0x07, 0x12, 0x0f, 0x0a,
    0x0b, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x10, 0x08, 0x12, 0x14,
    0x0a, 0x10, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61,
    0x63, 0x6b, 0x10, 0x09, 0x2a, 0x20, 0x0a, 0x02, 0x4f, 0x70, 0x12, 0x07, 0x0a, 0x03, 0x50, 0x75,
    0x74, 0x10, 0x01, 0x12, 0x07, 0x0a, 0x03, 0x44, 0x65, 0x6c, 0x10, 0x02, 0x12, 0x08, 0x0a, 0x04,
    0x4c, 0x6f, 0x63, 0x6b, 0x10, 0x03, 0x4a, 0x8e, 0x36, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xb1,
    0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0f, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x15, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x03,
    0x07, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x05, 0x00, 0x13, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x05, 0x05, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00,
    0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x06, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03,
    0x06, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x07, 0x04, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07, 0x04, 0x0b, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x07, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x08, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02,
    0x12, 0x03, 0x08, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x03, 0x12, 0x03, 0x09,
    0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x09, 0x04, 0x0d,
    0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0a, 0x04, 0x0e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x04, 0x02, 0x12, 0x03, 0x0a, 0x1a, 0x1b, 0x0a, 0x8a, 0x02, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x05,
    0x12, 0x03, 0x0f, 0x04, 0x1c, 0x1a, 0xfc, 0x01, 0x20, 0x42, 0x65, 0x6c, 0x6f, 0x77, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x73, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x75, 0x73, 0x65, 0x20, 0x66, 0x6f,
    0x72, 0x20, 0x47, 0x65, 0x74, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x2e, 0x20, 0x49, 0x66,
    0x20, 0x47, 0x65, 0x74, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x2c, 0x20, 0x69, 0x74, 0x20,
    0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x2e, 0x0a, 0x20,
    0x53, 0x6f, 0x20, 0x69, 0x74, 0x20, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20, 0x74, 0x6f, 0x20, 0x63,
    0x6c, 0x65, 0x61, 0x6e, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x20, 0x6c, 0x6f, 0x63,
    0x6b, 0x28, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x29, 0x2c, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x20, 0x77,
    0x69, 0x6c, 0x6c, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x0a, 0x20, 0x65, 0x69, 0x74, 0x68,
    0x65, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x20,
    0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x2e, 0x20, 0x46, 0x69, 0x6e,
    0x61, 0x6c, 0x6c, 0x79, 0x2c, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x69, 0x6c,
    0x6c, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x2f, 0x72, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63,
    0x6b, 0x0a, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x20, 0x6c, 0x6f, 0x63, 0x6b, 0x20,
    0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x47, 0x65, 0x74, 0x20, 0x61, 0x67, 0x61,
    0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f,
    0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12, 0x03, 0x0f, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x10, 0x04, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x10, 0x04, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x10, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x07, 0x12, 0x03, 0x11, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x01, 0x12,
    0x03, 0x11, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07, 0x02, 0x12, 0x03, 0x11,
    0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x08, 0x12, 0x03, 0x12, 0x04, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x08, 0x01, 0x12, 0x03, 0x12, 0x04, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x08, 0x02, 0x12, 0x03, 0x12, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x00, 0x12, 0x04, 0x15, 0x00, 0x19, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03,
    0x15, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x16, 0x04, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x17, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x17, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x17, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x14, 0x16, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x18, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x18, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x18, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x18, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x18,
    0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x1b, 0x00, 0x1f, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x1b, 0x08, 0x10, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x00, 0x12, 0x03, 0x1c, 0x04, 0x24, 0x22, 0x37, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x6f, 0x66, 0x66, 0x20,
    0x6f, 0x72, 0x20, 0x63, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c,
    0x6f, 0x63, 0x6b, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x74, 0x72, 0x79, 0x2e, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12, 0x03, 0x1c, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x1c, 0x22, 0x23, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x1d, 0x04, 0x24, 0x22, 0x31, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x6d,
    0x61, 0x79, 0x20, 0x72, 0x65, 0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74,
    0x78, 0x6e, 0x2e, 0x20, 0x65, 0x2e, 0x67, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x63, 0x6f,
    0x6e, 0x66, 0x6c, 0x69, 0x63, 0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x1d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1d,
    0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1d, 0x22, 0x23,
    0x0a, 0x2b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1e, 0x04, 0x24, 0x22, 0x1e, 0x20,
    0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x61, 0x62,
    0x6f, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x78, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x1e, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x1e, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x21, 0x00, 0x25,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x21, 0x08, 0x0f, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x22, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x22, 0x20, 0x29, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x22,
    0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x23, 0x04, 0x31, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x23, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x06, 0x12, 0x03, 0x23, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x20, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x23, 0x2f, 0x30, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x24, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x03, 0x24,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x06, 0x12, 0x03, 0x24, 0x0d, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x24, 0x20, 0x24, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x24, 0x2f, 0x30, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x03, 0x12, 0x04, 0x27, 0x00, 0x2a, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12,
    0x03, 0x27, 0x08, 0x0b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x28, 0x04,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x28, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03, 0x28, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x28, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x28, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02,
    0x01, 0x12, 0x03, 0x29, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x29, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x05, 0x12, 0x03, 0x29,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01, 0x12, 0x03, 0x29, 0x13, 0x1a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03, 0x29, 0x1d, 0x1e, 0x0a, 0x0a,
    0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2c, 0x00, 0x31, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04,
    0x01, 0x12, 0x03, 0x2c, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03,
    0x2d, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x03, 0x2d, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x2d, 0x0d, 0x15, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03, 0x2d, 0x16, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2d, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x2e, 0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x2e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x2e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2e,
    0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x2e, 0x20, 0x21,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x02, 0x12, 0x03, 0x2f, 0x04, 0x22, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x02, 0x04, 0x12, 0x03, 0x2f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x04, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x02, 0x01, 0x12, 0x03, 0x2f, 0x16, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x02, 0x03,
    0x12, 0x03, 0x2f, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x03, 0x12, 0x03, 0x30,
    0x04, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x04, 0x12, 0x03, 0x30, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x05, 0x12, 0x03, 0x30, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x03, 0x01, 0x12, 0x03, 0x30, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x03, 0x03, 0x12, 0x03, 0x30, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01,
    0x12, 0x04, 0x33, 0x00, 0x37, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x33,
    0x05, 0x07, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x34, 0x04, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x34, 0x04, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x34, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x35, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x35, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12,
    0x03, 0x35, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x36, 0x04,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x36, 0x04, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x36, 0x0b, 0x0c, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x05, 0x12, 0x04, 0x39, 0x00, 0x3e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01,
    0x12, 0x03, 0x39, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x3a,
    0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x3a, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x3a, 0x0d, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3a, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3a, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05,
    0x02, 0x01, 0x12, 0x03, 0x3b, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x3b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x3b, 0x0d, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3b, 0x13,
    0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3b, 0x1d, 0x1e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x3c, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03, 0x3c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x3c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x3c, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x3c, 0x1d, 0x1e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x3d, 0x04,
    0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x3d, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x05, 0x12, 0x03, 0x3d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x03, 0x01, 0x12, 0x03, 0x3d, 0x13, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x03, 0x03, 0x12, 0x03, 0x3d, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12,
    0x04, 0x40, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x40, 0x08,
    0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x41, 0x04, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x41, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x41, 0x0d, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x41, 0x15, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x41, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03,
    0x42, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x03, 0x42, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x42, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x42, 0x15, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x42, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x07, 0x12, 0x04, 0x45, 0x00, 0x47, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03,
    0x45, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x46, 0x04, 0x1e,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x03, 0x46, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x46, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03, 0x46, 0x16, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x46, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04,
    0x49, 0x00, 0x4d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x49, 0x08, 0x16,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x4a, 0x04, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x08, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4a, 0x0d, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x4a, 0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x4a, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x4b,
    0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x4b, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x4b, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4b, 0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4b, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x02, 0x12, 0x03, 0x4c, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04,
    0x12, 0x03, 0x4c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03,
    0x4c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4c, 0x15,
    0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x4c, 0x21, 0x22, 0x0a,
    0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x4f, 0x00, 0x51, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x09, 0x01, 0x12, 0x03, 0x4f, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12,
    0x03, 0x50, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x50,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x50, 0x0d, 0x15,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x50, 0x16, 0x1a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x50, 0x1d, 0x1e, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x0a, 0x12, 0x04, 0x53, 0x00, 0x57, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12,
    0x03, 0x53, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x54, 0x04,
    0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x54, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x06, 0x12, 0x03, 0x54, 0x0d, 0x15, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12, 0x03, 0x54, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x54, 0x22, 0x23, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02,
    0x01, 0x12, 0x03, 0x55, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12,
    0x03, 0x55, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x05, 0x12, 0x03, 0x55,
    0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x55, 0x16, 0x1d,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x03, 0x12, 0x03, 0x55, 0x22, 0x23, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x02, 0x12, 0x03, 0x56, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x02, 0x04, 0x12, 0x03, 0x56, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x02, 0x05, 0x12, 0x03, 0x56, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x01,
    0x12, 0x03, 0x56, 0x16, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x02, 0x03, 0x12, 0x03,
    0x56, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x59, 0x00, 0x5b, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x59, 0x08, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x00, 0x12, 0x03, 0x5a, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x5a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x5a, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5a,
    0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5a, 0x1f, 0x20,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c, 0x12, 0x04, 0x5d, 0x00, 0x61, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x0c, 0x01, 0x12, 0x03, 0x5d, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00,
    0x12, 0x03, 0x5e, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x5e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x05, 0x12, 0x03, 0x5e, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5e, 0x15, 0x1d, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x03, 0x12, 0x03, 0x5e, 0x21, 0x22, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x0c, 0x02, 0x01, 0x12, 0x03, 0x5f, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x5f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x5f, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x5f, 0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x01, 0x03, 0x12, 0x03, 0x5f,
    0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x02, 0x12, 0x03, 0x60, 0x04, 0x23, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x02, 0x04, 0x12, 0x03, 0x60, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x02, 0x05, 0x12, 0x03, 0x60, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x02, 0x01, 0x12, 0x03, 0x60, 0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x02, 0x03, 0x12, 0x03, 0x60, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x63,
    0x00, 0x65, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x63, 0x08, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x64, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x64, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x64, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x64, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x64, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12, 0x04, 0x67, 0x00, 0x6a, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x67, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x68, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x68, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x68, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x68, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x03, 0x12, 0x03, 0x68, 0x1b,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x03, 0x69, 0x04, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x03, 0x69, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x03, 0x69, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x69, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x69, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x6c, 0x00,
    0x6e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x6c, 0x08, 0x20, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x6d, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x6d, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x6d, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x6d, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x10, 0x12, 0x04, 0x70, 0x00, 0x73, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x03, 0x70, 0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x10, 0x02, 0x00, 0x12, 0x03, 0x71, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x71, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x71, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x01, 0x12, 0x03, 0x71,
    0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x03, 0x12, 0x03, 0x71, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x03, 0x72, 0x04, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x03, 0x72, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x10, 0x02, 0x01, 0x05, 0x12, 0x03, 0x72, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x72, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x72, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x11, 0x12, 0x04, 0x75, 0x00, 0x78,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x03, 0x75, 0x08, 0x1a, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x03, 0x76, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x76, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x76, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x76, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x03, 0x12, 0x03, 0x76,
    0x22, 0x23, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01, 0x12, 0x03, 0x77, 0x04, 0x24, 0x22,
    0x2b, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x6f, 0x77, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6c, 0x72, 0x65, 0x61, 0x64, 0x79,
    0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x03, 0x77, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x77, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x77, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x77, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x04, 0x7a, 0x00, 0x7d, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x03, 0x7a, 0x08, 0x21, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x12, 0x02, 0x00, 0x12, 0x03, 0x7b, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x7b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x7b, 0x0d, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x7b, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12, 0x03, 0x7b, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x03, 0x7c, 0x04, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x7c, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x7c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x02, 0x04, 0x13, 0x12, 0x05, 0x7f, 0x00,
    0x81, 0x01, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x03, 0x7f, 0x08, 0x22, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0x80, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0x80, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x00, 0x06, 0x12, 0x04, 0x80, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0x80, 0x01, 0x16, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x00, 0x03, 0x12, 0x04, 0x80, 0x01, 0x22, 0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12,
    0x06, 0x83, 0x01, 0x00, 0x88, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04,
    0x83, 0x01, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0x84, 0x01,
    0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0x84, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x04, 0x84, 0x01, 0x0d, 0x10,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0x84, 0x01, 0x14, 0x17, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0x84, 0x01, 0x20, 0x21, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x14, 0x02, 0x01, 0x12, 0x04, 0x85, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x14, 0x02, 0x01, 0x04, 0x12, 0x04, 0x85, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x14, 0x02, 0x01, 0x05, 0x12, 0x04, 0x85, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x85, 0x01, 0x14, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x85, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x02,
    0x12, 0x04, 0x86, 0x01, 0x04, 0x22, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x04, 0x12,
    0x04, 0x86, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x05, 0x12, 0x04,
    0x86, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x01, 0x12, 0x04, 0x86,
    0x01, 0x14, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x02, 0x03, 0x12, 0x04, 0x86, 0x01,
    0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x03, 0x12, 0x04, 0x87, 0x01, 0x04, 0x22,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x04, 0x12, 0x04, 0x87, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x05, 0x12, 0x04, 0x87, 0x01, 0x0d, 0x13, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x03, 0x01, 0x12, 0x04, 0x87, 0x01, 0x14, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x14, 0x02, 0x03, 0x03, 0x12, 0x04, 0x87, 0x01, 0x20, 0x21, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x15, 0x12, 0x06, 0x8a, 0x01, 0x00, 0x8c, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x15,
    0x01, 0x12, 0x04, 0x8a, 0x01, 0x08, 0x20, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00, 0x12,
    0x04, 0x8b, 0x01, 0x04, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x8b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04, 0x8b,
    0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8b, 0x01,
    0x16, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x22,
    0x23, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x16, 0x12, 0x06, 0x8e, 0x01, 0x00, 0x91, 0x01, 0x01, 0x0a,
    0x0b, 0x0a, 0x03, 0x04, 0x16, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x16, 0x02, 0x00, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x00, 0x04, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x00, 0x06, 0x12, 0x04, 0x8f, 0x01, 0x0d, 0x10, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00,
    0x01, 0x12, 0x04, 0x8f, 0x01, 0x14, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03,
    0x12, 0x04, 0x8f, 0x01, 0x1b, 0x1c, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04,
    0x90, 0x01, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0x90,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x05, 0x12, 0x04, 0x90, 0x01,
    0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0x90, 0x01, 0x14,
    0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0x90, 0x01, 0x1b, 0x1c,
    0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x17, 0x12, 0x06, 0x93, 0x01, 0x00, 0x95, 0x01, 0x01, 0x0a, 0x0b,
    0x0a, 0x03, 0x04, 0x17, 0x01, 0x12, 0x04, 0x93, 0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x17, 0x02, 0x00, 0x12, 0x04, 0x94, 0x01, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x94, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00,
    0x06, 0x12, 0x04, 0x94, 0x01, 0x0d, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x01,
    0x12, 0x04, 0x94, 0x01, 0x16, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x17, 0x02, 0x00, 0x03, 0x12,
    0x04, 0x94, 0x01, 0x23, 0x24, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x18, 0x12, 0x06, 0x97, 0x01, 0x00,
    0xa3, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x18, 0x01, 0x12, 0x04, 0x97, 0x01, 0x08, 0x0f,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x00, 0x12, 0x04, 0x98, 0x01, 0x04, 0x42, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x00, 0x04, 0x12, 0x04, 0x98, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x00, 0x06, 0x12, 0x04, 0x98, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x00, 0x01, 0x12, 0x04, 0x98, 0x01, 0x27, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x00, 0x03, 0x12, 0x04, 0x98, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18,
    0x02, 0x01, 0x12, 0x04, 0x99, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x99, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x06,
    0x12, 0x04, 0x99, 0x01, 0x0d, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x01, 0x12,
    0x04, 0x99, 0x01, 0x27, 0x2e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x01, 0x03, 0x12, 0x04,
    0x99, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x02, 0x12, 0x04, 0x9a, 0x01,
    0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x04, 0x12, 0x04, 0x9a, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x06, 0x12, 0x04, 0x9a, 0x01, 0x0d, 0x1a,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x27, 0x32, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x02, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x40, 0x41, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x18, 0x02, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x03, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x03, 0x06, 0x12, 0x04, 0x9b, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x03, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x27, 0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x03, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x04,
    0x12, 0x04, 0x9c, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x04, 0x12,
    0x04, 0x9c, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x06, 0x12, 0x04,
    0x9c, 0x01, 0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x01, 0x12, 0x04, 0x9c,
    0x01, 0x27, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x04, 0x03, 0x12, 0x04, 0x9c, 0x01,
    0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x05, 0x12, 0x04, 0x9d, 0x01, 0x04, 0x42,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x06, 0x12, 0x04, 0x9d, 0x01, 0x0d, 0x1d, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x05, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x27, 0x35, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x05, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x18, 0x02, 0x06, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18,
    0x02, 0x06, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02,
    0x06, 0x06, 0x12, 0x04, 0x9e, 0x01, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x06,
    0x01, 0x12, 0x04, 0x9e, 0x01, 0x27, 0x36, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x06, 0x03,
    0x12, 0x04, 0x9e, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x07, 0x12, 0x04,
    0x9f, 0x01, 0x04, 0x42, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x07, 0x04, 0x12, 0x04, 0x9f,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x07, 0x06, 0x12, 0x04, 0x9f, 0x01,
    0x0d, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x07, 0x01, 0x12, 0x04, 0x9f, 0x01, 0x27,
    0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x07, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x40, 0x41,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x08, 0x12, 0x04, 0xa0, 0x01, 0x04, 0x42, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x18, 0x02, 0x08, 0x04, 0x12, 0x04, 0xa0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x18, 0x02, 0x08, 0x06, 0x12, 0x04, 0xa0, 0x01, 0x0d, 0x24, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x18, 0x02, 0x08, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x27, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x18, 0x02, 0x08, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x40, 0x41, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18,
    0x02, 0x09, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x09,
    0x04, 0x12, 0x04, 0xa1, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x09, 0x06,
    0x12, 0x04, 0xa1, 0x01, 0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x09, 0x01, 0x12,
    0x04, 0xa1, 0x01, 0x27, 0x38, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x09, 0x03, 0x12, 0x04,
    0xa1, 0x01, 0x40, 0x42, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x18, 0x02, 0x0a, 0x12, 0x04, 0xa2, 0x01,
    0x04, 0x43, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xa2, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x0a, 0x06, 0x12, 0x04, 0xa2, 0x01, 0x0d, 0x24,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x0a, 0x01, 0x12, 0x04, 0xa2, 0x01, 0x27, 0x3d, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x18, 0x02, 0x0a, 0x03, 0x12, 0x04, 0xa2, 0x01, 0x40, 0x42, 0x0a, 0x0c,
    0x0a, 0x02, 0x04, 0x19, 0x12, 0x06, 0xa5, 0x01, 0x00, 0xb1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03,
    0x04, 0x19, 0x01, 0x12, 0x04, 0xa5, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02,
    0x00, 0x12, 0x04, 0xa6, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x04,
    0x12, 0x04, 0xa6, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x06, 0x12,
    0x04, 0xa6, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x01, 0x12, 0x04,
    0xa6, 0x01, 0x28, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x00, 0x03, 0x12, 0x04, 0xa6,
    0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x04,
    0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x04, 0x12, 0x04, 0xa7, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x06, 0x12, 0x04, 0xa7, 0x01, 0x0d, 0x1a, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x01, 0x12, 0x04, 0xa7, 0x01, 0x28, 0x34, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x01, 0x03, 0x12, 0x04, 0xa7, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x19, 0x02, 0x02, 0x12, 0x04, 0xa8, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x02, 0x04, 0x12, 0x04, 0xa8, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x02, 0x06, 0x12, 0x04, 0xa8, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x02, 0x01, 0x12, 0x04, 0xa8, 0x01, 0x28, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x02,
    0x03, 0x12, 0x04, 0xa8, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x03, 0x12,
    0x04, 0xa9, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x04, 0x12, 0x04,
    0xa9, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x06, 0x12, 0x04, 0xa9,
    0x01, 0x0d, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x01, 0x12, 0x04, 0xa9, 0x01,
    0x28, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x03, 0x03, 0x12, 0x04, 0xa9, 0x01, 0x42,
    0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x04, 0x44, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x04, 0x12, 0x04, 0xaa, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x04, 0x06, 0x12, 0x04, 0xaa, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x04, 0x01, 0x12, 0x04, 0xaa, 0x01, 0x28, 0x39, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x04, 0x03, 0x12, 0x04, 0xaa, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x19, 0x02, 0x05, 0x12, 0x04, 0xab, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x05, 0x04, 0x12, 0x04, 0xab, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x05,
    0x06, 0x12, 0x04, 0xab, 0x01, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x05, 0x01,
    0x12, 0x04, 0xab, 0x01, 0x28, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x05, 0x03, 0x12,
    0x04, 0xab, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x06, 0x12, 0x04, 0xac,
    0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x06, 0x04, 0x12, 0x04, 0xac, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x06, 0x06, 0x12, 0x04, 0xac, 0x01, 0x0d,
    0x1f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x06, 0x01, 0x12, 0x04, 0xac, 0x01, 0x28, 0x38,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x06, 0x03, 0x12, 0x04, 0xac, 0x01, 0x42, 0x43, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x07, 0x12, 0x04, 0xad, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x19, 0x02, 0x07, 0x04, 0x12, 0x04, 0xad, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x19, 0x02, 0x07, 0x06, 0x12, 0x04, 0xad, 0x01, 0x0d, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x07, 0x01, 0x12, 0x04, 0xad, 0x01, 0x28, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x07, 0x03, 0x12, 0x04, 0xad, 0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02,
    0x08, 0x12, 0x04, 0xae, 0x01, 0x04, 0x44, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x08, 0x04,
    0x12, 0x04, 0xae, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x08, 0x06, 0x12,
    0x04, 0xae, 0x01, 0x0d, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x08, 0x01, 0x12, 0x04,
    0xae, 0x01, 0x28, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x08, 0x03, 0x12, 0x04, 0xae,
    0x01, 0x42, 0x43, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x19, 0x02, 0x09, 0x12, 0x04, 0xaf, 0x01, 0x04,
    0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x09, 0x04, 0x12, 0x04, 0xaf, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x09, 0x06, 0x12, 0x04, 0xaf, 0x01, 0x0d, 0x20, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x09, 0x01, 0x12, 0x04, 0xaf, 0x01, 0x28, 0x3a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x19, 0x02, 0x09, 0x03, 0x12, 0x04, 0xaf, 0x01, 0x42, 0x44, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x19, 0x02, 0x0a, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x45, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x19, 0x02, 0x0a, 0x04, 0x12, 0x04, 0xb0, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19,
    0x02, 0x0a, 0x06, 0x12, 0x04, 0xb0, 0x01, 0x0d, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02,
    0x0a, 0x01, 0x12, 0x04, 0xb0, 0x01, 0x28, 0x3f, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x19, 0x02, 0x0a,
    0x03, 0x12, 0x04, 0xb0, 0x01, 0x42, 0x44,
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
