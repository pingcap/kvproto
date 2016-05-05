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
    primary_lock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lock_version: ::std::option::Option<u64>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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
                    primary_lock: ::protobuf::SingularField::none(),
                    lock_version: ::std::option::Option::None,
                    key: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes primary_lock = 1;

    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }

    pub fn has_primary_lock(&self) -> bool {
        self.primary_lock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary_lock(&mut self, v: ::std::vec::Vec<u8>) {
        self.primary_lock = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary_lock<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.primary_lock.is_none() {
            self.primary_lock.set_default();
        };
        self.primary_lock.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary_lock(&mut self) -> ::std::vec::Vec<u8> {
        self.primary_lock.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_primary_lock<'a>(&'a self) -> &'a [u8] {
        match self.primary_lock.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 lock_version = 2;

    pub fn clear_lock_version(&mut self) {
        self.lock_version = ::std::option::Option::None;
    }

    pub fn has_lock_version(&self) -> bool {
        self.lock_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lock_version(&mut self, v: u64) {
        self.lock_version = ::std::option::Option::Some(v);
    }

    pub fn get_lock_version<'a>(&self) -> u64 {
        self.lock_version.unwrap_or(0)
    }

    // optional bytes key = 3;

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
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.primary_lock));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.lock_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
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
        for value in self.primary_lock.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.lock_version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.primary_lock.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.lock_version {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.key.as_ref() {
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
                    "primary_lock",
                    LockInfo::has_primary_lock,
                    LockInfo::get_primary_lock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "lock_version",
                    LockInfo::has_lock_version,
                    LockInfo::get_lock_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    LockInfo::has_key,
                    LockInfo::get_key,
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
        self.clear_primary_lock();
        self.clear_lock_version();
        self.clear_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for LockInfo {
    fn eq(&self, other: &LockInfo) -> bool {
        self.primary_lock == other.primary_lock &&
        self.lock_version == other.lock_version &&
        self.key == other.key &&
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
    pub fn mut_locked<'a>(&'a mut self) -> &'a mut LockInfo {
        if self.locked.is_none() {
            self.locked.set_default();
        };
        self.locked.as_mut().unwrap()
    }

    // Take field
    pub fn take_locked(&mut self) -> LockInfo {
        self.locked.take().unwrap_or_else(|| LockInfo::new())
    }

    pub fn get_locked<'a>(&'a self) -> &'a LockInfo {
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
    pub fn mut_retryable<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.retryable.is_none() {
            self.retryable.set_default();
        };
        self.retryable.as_mut().unwrap()
    }

    // Take field
    pub fn take_retryable(&mut self) -> ::std::string::String {
        self.retryable.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_retryable<'a>(&'a self) -> &'a str {
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
    pub fn mut_abort<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.abort.is_none() {
            self.abort.set_default();
        };
        self.abort.as_mut().unwrap()
    }

    // Take field
    pub fn take_abort(&mut self) -> ::std::string::String {
        self.abort.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_abort<'a>(&'a self) -> &'a str {
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

    pub fn get_region_id<'a>(&self) -> u64 {
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
    pub fn mut_region_epoch<'a>(&'a mut self) -> &'a mut super::metapb::RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch.set_default();
        };
        self.region_epoch.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_epoch(&mut self) -> super::metapb::RegionEpoch {
        self.region_epoch.take().unwrap_or_else(|| super::metapb::RegionEpoch::new())
    }

    pub fn get_region_epoch<'a>(&'a self) -> &'a super::metapb::RegionEpoch {
        self.region_epoch.as_ref().unwrap_or_else(|| super::metapb::RegionEpoch::default_instance())
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
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Context {
    fn eq(&self, other: &Context) -> bool {
        self.region_id == other.region_id &&
        self.region_epoch == other.region_epoch &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Context {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdGetRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    version: ::std::option::Option<u64>,
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
                    key: ::protobuf::SingularField::none(),
                    version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version<'a>(&self) -> u64 {
        self.version.unwrap_or(0)
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.version = ::std::option::Option::Some(tmp);
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.version {
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    CmdGetRequest::has_key,
                    CmdGetRequest::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "version",
                    CmdGetRequest::has_version,
                    CmdGetRequest::get_version,
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
        self.clear_key();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdGetRequest {
    fn eq(&self, other: &CmdGetRequest) -> bool {
        self.key == other.key &&
        self.version == other.version &&
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
    error: ::protobuf::SingularPtrField<KeyError>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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
                    error: ::protobuf::SingularPtrField::none(),
                    value: ::protobuf::SingularField::none(),
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
    pub fn mut_error<'a>(&'a mut self) -> &'a mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
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
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
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
        if let Some(v) = self.value.as_ref() {
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
                    "error",
                    CmdGetResponse::has_error,
                    CmdGetResponse::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    CmdGetResponse::has_value,
                    CmdGetResponse::get_value,
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
        self.clear_error();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdGetResponse {
    fn eq(&self, other: &CmdGetResponse) -> bool {
        self.error == other.error &&
        self.value == other.value &&
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
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    limit: ::std::option::Option<u32>,
    version: ::std::option::Option<u64>,
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
                    start_key: ::protobuf::SingularField::none(),
                    limit: ::std::option::Option::None,
                    version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional bytes start_key = 1;

    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }

    pub fn has_start_key(&self) -> bool {
        self.start_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.start_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.start_key.is_none() {
            self.start_key.set_default();
        };
        self.start_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::std::vec::Vec<u8> {
        self.start_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_start_key<'a>(&'a self) -> &'a [u8] {
        match self.start_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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

    pub fn get_limit<'a>(&self) -> u32 {
        self.limit.unwrap_or(0)
    }

    // optional uint64 version = 3;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version<'a>(&self) -> u64 {
        self.version.unwrap_or(0)
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start_key));
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
                    self.version = ::std::option::Option::Some(tmp);
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
        for value in self.start_key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.limit.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.version.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.limit {
            try!(os.write_uint32(2, v));
        };
        if let Some(v) = self.version {
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "start_key",
                    CmdScanRequest::has_start_key,
                    CmdScanRequest::get_start_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "limit",
                    CmdScanRequest::has_limit,
                    CmdScanRequest::get_limit,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "version",
                    CmdScanRequest::has_version,
                    CmdScanRequest::get_version,
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
        self.clear_start_key();
        self.clear_limit();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdScanRequest {
    fn eq(&self, other: &CmdScanRequest) -> bool {
        self.start_key == other.start_key &&
        self.limit == other.limit &&
        self.version == other.version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdScanRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct KvPair {
    // message fields
    error: ::protobuf::SingularPtrField<KeyError>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KvPair {}

impl KvPair {
    pub fn new() -> KvPair {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KvPair {
        static mut instance: ::protobuf::lazy::Lazy<KvPair> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KvPair,
        };
        unsafe {
            instance.get(|| {
                KvPair {
                    error: ::protobuf::SingularPtrField::none(),
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
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
    pub fn mut_error<'a>(&'a mut self) -> &'a mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
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
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for KvPair {
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
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
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.value.as_ref() {
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
        ::std::any::TypeId::of::<KvPair>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KvPair {
    fn new() -> KvPair {
        KvPair::new()
    }

    fn descriptor_static(_: ::std::option::Option<KvPair>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "error",
                    KvPair::has_error,
                    KvPair::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    KvPair::has_key,
                    KvPair::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    KvPair::has_value,
                    KvPair::get_value,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KvPair>(
                    "KvPair",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KvPair {
    fn clear(&mut self) {
        self.clear_error();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for KvPair {
    fn eq(&self, other: &KvPair) -> bool {
        self.error == other.error &&
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for KvPair {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdScanResponse {
    // message fields
    pairs: ::protobuf::RepeatedField<KvPair>,
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
                    pairs: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kvrpcpb.KvPair pairs = 1;

    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }

    // Param is passed by value, moved
    pub fn set_pairs(&mut self, v: ::protobuf::RepeatedField<KvPair>) {
        self.pairs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pairs<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<KvPair> {
        &mut self.pairs
    }

    // Take field
    pub fn take_pairs(&mut self) -> ::protobuf::RepeatedField<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::protobuf::RepeatedField::new())
    }

    pub fn get_pairs<'a>(&'a self) -> &'a [KvPair] {
        &self.pairs
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
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pairs));
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
        for value in self.pairs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.pairs.iter() {
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
                    "pairs",
                    CmdScanResponse::get_pairs,
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
        self.clear_pairs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdScanResponse {
    fn eq(&self, other: &CmdScanResponse) -> bool {
        self.pairs == other.pairs &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdScanResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Mutation {
    // message fields
    op: ::std::option::Option<Op>,
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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
                    op: ::std::option::Option::None,
                    key: ::protobuf::SingularField::none(),
                    value: ::protobuf::SingularField::none(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional .kvrpcpb.Op op = 1;

    pub fn clear_op(&mut self) {
        self.op = ::std::option::Option::None;
    }

    pub fn has_op(&self) -> bool {
        self.op.is_some()
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: Op) {
        self.op = ::std::option::Option::Some(v);
    }

    pub fn get_op<'a>(&self) -> Op {
        self.op.unwrap_or(Op::Put)
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
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_enum());
                    self.op = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in self.op.iter() {
            my_size += ::protobuf::rt::enum_size(1, *value);
        };
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.op {
            try!(os.write_enum(1, v.value()));
        };
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.value.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_singular_enum_accessor(
                    "op",
                    Mutation::has_op,
                    Mutation::get_op,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    Mutation::has_key,
                    Mutation::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    Mutation::has_value,
                    Mutation::get_value,
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
        self.clear_op();
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Mutation {
    fn eq(&self, other: &Mutation) -> bool {
        self.op == other.op &&
        self.key == other.key &&
        self.value == other.value &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Mutation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdPrewriteRequest {
    // message fields
    mutations: ::protobuf::RepeatedField<Mutation>,
    primary_lock: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_version: ::std::option::Option<u64>,
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
                    primary_lock: ::protobuf::SingularField::none(),
                    start_version: ::std::option::Option::None,
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
    pub fn mut_mutations<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }

    // Take field
    pub fn take_mutations(&mut self) -> ::protobuf::RepeatedField<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::protobuf::RepeatedField::new())
    }

    pub fn get_mutations<'a>(&'a self) -> &'a [Mutation] {
        &self.mutations
    }

    // optional bytes primary_lock = 2;

    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }

    pub fn has_primary_lock(&self) -> bool {
        self.primary_lock.is_some()
    }

    // Param is passed by value, moved
    pub fn set_primary_lock(&mut self, v: ::std::vec::Vec<u8>) {
        self.primary_lock = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_primary_lock<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.primary_lock.is_none() {
            self.primary_lock.set_default();
        };
        self.primary_lock.as_mut().unwrap()
    }

    // Take field
    pub fn take_primary_lock(&mut self) -> ::std::vec::Vec<u8> {
        self.primary_lock.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_primary_lock<'a>(&'a self) -> &'a [u8] {
        match self.primary_lock.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 start_version = 3;

    pub fn clear_start_version(&mut self) {
        self.start_version = ::std::option::Option::None;
    }

    pub fn has_start_version(&self) -> bool {
        self.start_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = ::std::option::Option::Some(v);
    }

    pub fn get_start_version<'a>(&self) -> u64 {
        self.start_version.unwrap_or(0)
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.primary_lock));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.start_version = ::std::option::Option::Some(tmp);
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
        for value in self.primary_lock.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.start_version.iter() {
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
        if let Some(v) = self.primary_lock.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.start_version {
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
                    "primary_lock",
                    CmdPrewriteRequest::has_primary_lock,
                    CmdPrewriteRequest::get_primary_lock,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "start_version",
                    CmdPrewriteRequest::has_start_version,
                    CmdPrewriteRequest::get_start_version,
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
        self.clear_primary_lock();
        self.clear_start_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdPrewriteRequest {
    fn eq(&self, other: &CmdPrewriteRequest) -> bool {
        self.mutations == other.mutations &&
        self.primary_lock == other.primary_lock &&
        self.start_version == other.start_version &&
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
    pub fn mut_errors<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<KeyError> {
        &mut self.errors
    }

    // Take field
    pub fn take_errors(&mut self) -> ::protobuf::RepeatedField<KeyError> {
        ::std::mem::replace(&mut self.errors, ::protobuf::RepeatedField::new())
    }

    pub fn get_errors<'a>(&'a self) -> &'a [KeyError] {
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
    start_version: ::std::option::Option<u64>,
    keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    commit_version: ::std::option::Option<u64>,
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
                    start_version: ::std::option::Option::None,
                    keys: ::protobuf::RepeatedField::new(),
                    commit_version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 start_version = 1;

    pub fn clear_start_version(&mut self) {
        self.start_version = ::std::option::Option::None;
    }

    pub fn has_start_version(&self) -> bool {
        self.start_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = ::std::option::Option::Some(v);
    }

    pub fn get_start_version<'a>(&self) -> u64 {
        self.start_version.unwrap_or(0)
    }

    // repeated bytes keys = 2;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys<'a>(&'a self) -> &'a [::std::vec::Vec<u8>] {
        &self.keys
    }

    // optional uint64 commit_version = 3;

    pub fn clear_commit_version(&mut self) {
        self.commit_version = ::std::option::Option::None;
    }

    pub fn has_commit_version(&self) -> bool {
        self.commit_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = ::std::option::Option::Some(v);
    }

    pub fn get_commit_version<'a>(&self) -> u64 {
        self.commit_version.unwrap_or(0)
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
                    self.start_version = ::std::option::Option::Some(tmp);
                },
                2 => {
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys));
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_version = ::std::option::Option::Some(tmp);
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
        for value in self.start_version.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.keys.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.commit_version.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.start_version {
            try!(os.write_uint64(1, v));
        };
        for v in self.keys.iter() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.commit_version {
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
                    "start_version",
                    CmdCommitRequest::has_start_version,
                    CmdCommitRequest::get_start_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "keys",
                    CmdCommitRequest::get_keys,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_version",
                    CmdCommitRequest::has_commit_version,
                    CmdCommitRequest::get_commit_version,
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
        self.clear_start_version();
        self.clear_keys();
        self.clear_commit_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCommitRequest {
    fn eq(&self, other: &CmdCommitRequest) -> bool {
        self.start_version == other.start_version &&
        self.keys == other.keys &&
        self.commit_version == other.commit_version &&
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
    errors: ::protobuf::RepeatedField<KeyError>,
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
    pub fn mut_errors<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<KeyError> {
        &mut self.errors
    }

    // Take field
    pub fn take_errors(&mut self) -> ::protobuf::RepeatedField<KeyError> {
        ::std::mem::replace(&mut self.errors, ::protobuf::RepeatedField::new())
    }

    pub fn get_errors<'a>(&'a self) -> &'a [KeyError] {
        &self.errors
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
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "errors",
                    CmdCommitResponse::get_errors,
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
        self.clear_errors();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCommitResponse {
    fn eq(&self, other: &CmdCommitResponse) -> bool {
        self.errors == other.errors &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for CmdCommitResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct CmdCleanupRequest {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    start_version: ::std::option::Option<u64>,
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
                    key: ::protobuf::SingularField::none(),
                    start_version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 start_version = 2;

    pub fn clear_start_version(&mut self) {
        self.start_version = ::std::option::Option::None;
    }

    pub fn has_start_version(&self) -> bool {
        self.start_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = ::std::option::Option::Some(v);
    }

    pub fn get_start_version<'a>(&self) -> u64 {
        self.start_version.unwrap_or(0)
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.start_version = ::std::option::Option::Some(tmp);
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.start_version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.start_version {
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
                    "key",
                    CmdCleanupRequest::has_key,
                    CmdCleanupRequest::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "start_version",
                    CmdCleanupRequest::has_start_version,
                    CmdCleanupRequest::get_start_version,
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
        self.clear_key();
        self.clear_start_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCleanupRequest {
    fn eq(&self, other: &CmdCleanupRequest) -> bool {
        self.key == other.key &&
        self.start_version == other.start_version &&
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
    commit_version: ::std::option::Option<u64>,
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
                    commit_version: ::std::option::Option::None,
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
    pub fn mut_error<'a>(&'a mut self) -> &'a mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
    }

    // optional uint64 commit_version = 2;

    pub fn clear_commit_version(&mut self) {
        self.commit_version = ::std::option::Option::None;
    }

    pub fn has_commit_version(&self) -> bool {
        self.commit_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = ::std::option::Option::Some(v);
    }

    pub fn get_commit_version<'a>(&self) -> u64 {
        self.commit_version.unwrap_or(0)
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
                    self.commit_version = ::std::option::Option::Some(tmp);
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
        for value in self.commit_version.iter() {
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
        if let Some(v) = self.commit_version {
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
                    "commit_version",
                    CmdCleanupResponse::has_commit_version,
                    CmdCleanupResponse::get_commit_version,
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
        self.clear_commit_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCleanupResponse {
    fn eq(&self, other: &CmdCleanupResponse) -> bool {
        self.error == other.error &&
        self.commit_version == other.commit_version &&
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
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lock_version: ::std::option::Option<u64>,
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
                    key: ::protobuf::SingularField::none(),
                    lock_version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 lock_version = 2;

    pub fn clear_lock_version(&mut self) {
        self.lock_version = ::std::option::Option::None;
    }

    pub fn has_lock_version(&self) -> bool {
        self.lock_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lock_version(&mut self, v: u64) {
        self.lock_version = ::std::option::Option::Some(v);
    }

    pub fn get_lock_version<'a>(&self) -> u64 {
        self.lock_version.unwrap_or(0)
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.lock_version = ::std::option::Option::Some(tmp);
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.lock_version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.lock_version {
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    CmdRollbackThenGetRequest::has_key,
                    CmdRollbackThenGetRequest::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "lock_version",
                    CmdRollbackThenGetRequest::has_lock_version,
                    CmdRollbackThenGetRequest::get_lock_version,
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
        self.clear_key();
        self.clear_lock_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdRollbackThenGetRequest {
    fn eq(&self, other: &CmdRollbackThenGetRequest) -> bool {
        self.key == other.key &&
        self.lock_version == other.lock_version &&
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
    error: ::protobuf::SingularPtrField<KeyError>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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
                    error: ::protobuf::SingularPtrField::none(),
                    value: ::protobuf::SingularField::none(),
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
    pub fn mut_error<'a>(&'a mut self) -> &'a mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
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
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
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
        if let Some(v) = self.value.as_ref() {
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
                    "error",
                    CmdRollbackThenGetResponse::has_error,
                    CmdRollbackThenGetResponse::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    CmdRollbackThenGetResponse::has_value,
                    CmdRollbackThenGetResponse::get_value,
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
        self.clear_error();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdRollbackThenGetResponse {
    fn eq(&self, other: &CmdRollbackThenGetResponse) -> bool {
        self.error == other.error &&
        self.value == other.value &&
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
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    lock_version: ::std::option::Option<u64>,
    commit_version: ::std::option::Option<u64>,
    get_version: ::std::option::Option<u64>,
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
                    key: ::protobuf::SingularField::none(),
                    lock_version: ::std::option::Option::None,
                    commit_version: ::std::option::Option::None,
                    get_version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
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
    pub fn mut_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.key.is_none() {
            self.key.set_default();
        };
        self.key.as_mut().unwrap()
    }

    // Take field
    pub fn take_key(&mut self) -> ::std::vec::Vec<u8> {
        self.key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_key<'a>(&'a self) -> &'a [u8] {
        match self.key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional uint64 lock_version = 2;

    pub fn clear_lock_version(&mut self) {
        self.lock_version = ::std::option::Option::None;
    }

    pub fn has_lock_version(&self) -> bool {
        self.lock_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_lock_version(&mut self, v: u64) {
        self.lock_version = ::std::option::Option::Some(v);
    }

    pub fn get_lock_version<'a>(&self) -> u64 {
        self.lock_version.unwrap_or(0)
    }

    // optional uint64 commit_version = 3;

    pub fn clear_commit_version(&mut self) {
        self.commit_version = ::std::option::Option::None;
    }

    pub fn has_commit_version(&self) -> bool {
        self.commit_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = ::std::option::Option::Some(v);
    }

    pub fn get_commit_version<'a>(&self) -> u64 {
        self.commit_version.unwrap_or(0)
    }

    // optional uint64 get_version = 4;

    pub fn clear_get_version(&mut self) {
        self.get_version = ::std::option::Option::None;
    }

    pub fn has_get_version(&self) -> bool {
        self.get_version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_get_version(&mut self, v: u64) {
        self.get_version = ::std::option::Option::Some(v);
    }

    pub fn get_get_version<'a>(&self) -> u64 {
        self.get_version.unwrap_or(0)
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
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.key));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.lock_version = ::std::option::Option::Some(tmp);
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.commit_version = ::std::option::Option::Some(tmp);
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.get_version = ::std::option::Option::Some(tmp);
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
        for value in self.key.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.lock_version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.commit_version.iter() {
            my_size += ::protobuf::rt::value_size(3, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.get_version.iter() {
            my_size += ::protobuf::rt::value_size(4, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.lock_version {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.commit_version {
            try!(os.write_uint64(3, v));
        };
        if let Some(v) = self.get_version {
            try!(os.write_uint64(4, v));
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
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    CmdCommitThenGetRequest::has_key,
                    CmdCommitThenGetRequest::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "lock_version",
                    CmdCommitThenGetRequest::has_lock_version,
                    CmdCommitThenGetRequest::get_lock_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "commit_version",
                    CmdCommitThenGetRequest::has_commit_version,
                    CmdCommitThenGetRequest::get_commit_version,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "get_version",
                    CmdCommitThenGetRequest::has_get_version,
                    CmdCommitThenGetRequest::get_get_version,
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
        self.clear_key();
        self.clear_lock_version();
        self.clear_commit_version();
        self.clear_get_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCommitThenGetRequest {
    fn eq(&self, other: &CmdCommitThenGetRequest) -> bool {
        self.key == other.key &&
        self.lock_version == other.lock_version &&
        self.commit_version == other.commit_version &&
        self.get_version == other.get_version &&
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
    error: ::protobuf::SingularPtrField<KeyError>,
    value: ::protobuf::SingularField<::std::vec::Vec<u8>>,
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
                    error: ::protobuf::SingularPtrField::none(),
                    value: ::protobuf::SingularField::none(),
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
    pub fn mut_error<'a>(&'a mut self) -> &'a mut KeyError {
        if self.error.is_none() {
            self.error.set_default();
        };
        self.error.as_mut().unwrap()
    }

    // Take field
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::new())
    }

    pub fn get_error<'a>(&'a self) -> &'a KeyError {
        self.error.as_ref().unwrap_or_else(|| KeyError::default_instance())
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
    pub fn mut_value<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.value.is_none() {
            self.value.set_default();
        };
        self.value.as_mut().unwrap()
    }

    // Take field
    pub fn take_value(&mut self) -> ::std::vec::Vec<u8> {
        self.value.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_value<'a>(&'a self) -> &'a [u8] {
        match self.value.as_ref() {
            Some(v) => &v,
            None => &[],
        }
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error));
                },
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.value));
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
        for value in self.value.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
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
        if let Some(v) = self.value.as_ref() {
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
                    "error",
                    CmdCommitThenGetResponse::has_error,
                    CmdCommitThenGetResponse::get_error,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "value",
                    CmdCommitThenGetResponse::has_value,
                    CmdCommitThenGetResponse::get_value,
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
        self.clear_error();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdCommitThenGetResponse {
    fn eq(&self, other: &CmdCommitThenGetResponse) -> bool {
        self.error == other.error &&
        self.value == other.value &&
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
    keys: ::protobuf::RepeatedField<::std::vec::Vec<u8>>,
    version: ::std::option::Option<u64>,
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
                    keys: ::protobuf::RepeatedField::new(),
                    version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated bytes keys = 1;

    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }

    // Param is passed by value, moved
    pub fn set_keys(&mut self, v: ::protobuf::RepeatedField<::std::vec::Vec<u8>>) {
        self.keys = v;
    }

    // Mutable pointer to the field.
    pub fn mut_keys<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        &mut self.keys
    }

    // Take field
    pub fn take_keys(&mut self) -> ::protobuf::RepeatedField<::std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::protobuf::RepeatedField::new())
    }

    pub fn get_keys<'a>(&'a self) -> &'a [::std::vec::Vec<u8>] {
        &self.keys
    }

    // optional uint64 version = 2;

    pub fn clear_version(&mut self) {
        self.version = ::std::option::Option::None;
    }

    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = ::std::option::Option::Some(v);
    }

    pub fn get_version<'a>(&self) -> u64 {
        self.version.unwrap_or(0)
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
                    try!(::protobuf::rt::read_repeated_bytes_into(wire_type, is, &mut self.keys));
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint64());
                    self.version = ::std::option::Option::Some(tmp);
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
        for value in self.keys.iter() {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in self.version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.keys.iter() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.version {
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
                fields.push(::protobuf::reflect::accessor::make_repeated_bytes_accessor(
                    "keys",
                    CmdBatchGetRequest::get_keys,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "version",
                    CmdBatchGetRequest::has_version,
                    CmdBatchGetRequest::get_version,
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
        self.clear_keys();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdBatchGetRequest {
    fn eq(&self, other: &CmdBatchGetRequest) -> bool {
        self.keys == other.keys &&
        self.version == other.version &&
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
    pairs: ::protobuf::RepeatedField<KvPair>,
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
                    pairs: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .kvrpcpb.KvPair pairs = 1;

    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }

    // Param is passed by value, moved
    pub fn set_pairs(&mut self, v: ::protobuf::RepeatedField<KvPair>) {
        self.pairs = v;
    }

    // Mutable pointer to the field.
    pub fn mut_pairs<'a>(&'a mut self) -> &'a mut ::protobuf::RepeatedField<KvPair> {
        &mut self.pairs
    }

    // Take field
    pub fn take_pairs(&mut self) -> ::protobuf::RepeatedField<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::protobuf::RepeatedField::new())
    }

    pub fn get_pairs<'a>(&'a self) -> &'a [KvPair] {
        &self.pairs
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
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.pairs));
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
        for value in self.pairs.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in self.pairs.iter() {
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
                    "pairs",
                    CmdBatchGetResponse::get_pairs,
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
        self.clear_pairs();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for CmdBatchGetResponse {
    fn eq(&self, other: &CmdBatchGetResponse) -> bool {
        self.pairs == other.pairs &&
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

    pub fn get_field_type<'a>(&self) -> MessageType {
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
    pub fn mut_context<'a>(&'a mut self) -> &'a mut Context {
        if self.context.is_none() {
            self.context.set_default();
        };
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::new())
    }

    pub fn get_context<'a>(&'a self) -> &'a Context {
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
    pub fn mut_cmd_get_req<'a>(&'a mut self) -> &'a mut CmdGetRequest {
        if self.cmd_get_req.is_none() {
            self.cmd_get_req.set_default();
        };
        self.cmd_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_get_req(&mut self) -> CmdGetRequest {
        self.cmd_get_req.take().unwrap_or_else(|| CmdGetRequest::new())
    }

    pub fn get_cmd_get_req<'a>(&'a self) -> &'a CmdGetRequest {
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
    pub fn mut_cmd_scan_req<'a>(&'a mut self) -> &'a mut CmdScanRequest {
        if self.cmd_scan_req.is_none() {
            self.cmd_scan_req.set_default();
        };
        self.cmd_scan_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_scan_req(&mut self) -> CmdScanRequest {
        self.cmd_scan_req.take().unwrap_or_else(|| CmdScanRequest::new())
    }

    pub fn get_cmd_scan_req<'a>(&'a self) -> &'a CmdScanRequest {
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
    pub fn mut_cmd_prewrite_req<'a>(&'a mut self) -> &'a mut CmdPrewriteRequest {
        if self.cmd_prewrite_req.is_none() {
            self.cmd_prewrite_req.set_default();
        };
        self.cmd_prewrite_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_prewrite_req(&mut self) -> CmdPrewriteRequest {
        self.cmd_prewrite_req.take().unwrap_or_else(|| CmdPrewriteRequest::new())
    }

    pub fn get_cmd_prewrite_req<'a>(&'a self) -> &'a CmdPrewriteRequest {
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
    pub fn mut_cmd_commit_req<'a>(&'a mut self) -> &'a mut CmdCommitRequest {
        if self.cmd_commit_req.is_none() {
            self.cmd_commit_req.set_default();
        };
        self.cmd_commit_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_commit_req(&mut self) -> CmdCommitRequest {
        self.cmd_commit_req.take().unwrap_or_else(|| CmdCommitRequest::new())
    }

    pub fn get_cmd_commit_req<'a>(&'a self) -> &'a CmdCommitRequest {
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
    pub fn mut_cmd_cleanup_req<'a>(&'a mut self) -> &'a mut CmdCleanupRequest {
        if self.cmd_cleanup_req.is_none() {
            self.cmd_cleanup_req.set_default();
        };
        self.cmd_cleanup_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_cleanup_req(&mut self) -> CmdCleanupRequest {
        self.cmd_cleanup_req.take().unwrap_or_else(|| CmdCleanupRequest::new())
    }

    pub fn get_cmd_cleanup_req<'a>(&'a self) -> &'a CmdCleanupRequest {
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
    pub fn mut_cmd_rb_get_req<'a>(&'a mut self) -> &'a mut CmdRollbackThenGetRequest {
        if self.cmd_rb_get_req.is_none() {
            self.cmd_rb_get_req.set_default();
        };
        self.cmd_rb_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_rb_get_req(&mut self) -> CmdRollbackThenGetRequest {
        self.cmd_rb_get_req.take().unwrap_or_else(|| CmdRollbackThenGetRequest::new())
    }

    pub fn get_cmd_rb_get_req<'a>(&'a self) -> &'a CmdRollbackThenGetRequest {
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
    pub fn mut_cmd_commit_get_req<'a>(&'a mut self) -> &'a mut CmdCommitThenGetRequest {
        if self.cmd_commit_get_req.is_none() {
            self.cmd_commit_get_req.set_default();
        };
        self.cmd_commit_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_commit_get_req(&mut self) -> CmdCommitThenGetRequest {
        self.cmd_commit_get_req.take().unwrap_or_else(|| CmdCommitThenGetRequest::new())
    }

    pub fn get_cmd_commit_get_req<'a>(&'a self) -> &'a CmdCommitThenGetRequest {
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
    pub fn mut_cmd_batch_get_req<'a>(&'a mut self) -> &'a mut CmdBatchGetRequest {
        if self.cmd_batch_get_req.is_none() {
            self.cmd_batch_get_req.set_default();
        };
        self.cmd_batch_get_req.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_batch_get_req(&mut self) -> CmdBatchGetRequest {
        self.cmd_batch_get_req.take().unwrap_or_else(|| CmdBatchGetRequest::new())
    }

    pub fn get_cmd_batch_get_req<'a>(&'a self) -> &'a CmdBatchGetRequest {
        self.cmd_batch_get_req.as_ref().unwrap_or_else(|| CmdBatchGetRequest::default_instance())
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

    pub fn get_field_type<'a>(&self) -> MessageType {
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
    pub fn mut_region_error<'a>(&'a mut self) -> &'a mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error.set_default();
        };
        self.region_error.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error.take().unwrap_or_else(|| super::errorpb::Error::new())
    }

    pub fn get_region_error<'a>(&'a self) -> &'a super::errorpb::Error {
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
    pub fn mut_cmd_get_resp<'a>(&'a mut self) -> &'a mut CmdGetResponse {
        if self.cmd_get_resp.is_none() {
            self.cmd_get_resp.set_default();
        };
        self.cmd_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_get_resp(&mut self) -> CmdGetResponse {
        self.cmd_get_resp.take().unwrap_or_else(|| CmdGetResponse::new())
    }

    pub fn get_cmd_get_resp<'a>(&'a self) -> &'a CmdGetResponse {
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
    pub fn mut_cmd_scan_resp<'a>(&'a mut self) -> &'a mut CmdScanResponse {
        if self.cmd_scan_resp.is_none() {
            self.cmd_scan_resp.set_default();
        };
        self.cmd_scan_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_scan_resp(&mut self) -> CmdScanResponse {
        self.cmd_scan_resp.take().unwrap_or_else(|| CmdScanResponse::new())
    }

    pub fn get_cmd_scan_resp<'a>(&'a self) -> &'a CmdScanResponse {
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
    pub fn mut_cmd_prewrite_resp<'a>(&'a mut self) -> &'a mut CmdPrewriteResponse {
        if self.cmd_prewrite_resp.is_none() {
            self.cmd_prewrite_resp.set_default();
        };
        self.cmd_prewrite_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_prewrite_resp(&mut self) -> CmdPrewriteResponse {
        self.cmd_prewrite_resp.take().unwrap_or_else(|| CmdPrewriteResponse::new())
    }

    pub fn get_cmd_prewrite_resp<'a>(&'a self) -> &'a CmdPrewriteResponse {
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
    pub fn mut_cmd_commit_resp<'a>(&'a mut self) -> &'a mut CmdCommitResponse {
        if self.cmd_commit_resp.is_none() {
            self.cmd_commit_resp.set_default();
        };
        self.cmd_commit_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_commit_resp(&mut self) -> CmdCommitResponse {
        self.cmd_commit_resp.take().unwrap_or_else(|| CmdCommitResponse::new())
    }

    pub fn get_cmd_commit_resp<'a>(&'a self) -> &'a CmdCommitResponse {
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
    pub fn mut_cmd_cleanup_resp<'a>(&'a mut self) -> &'a mut CmdCleanupResponse {
        if self.cmd_cleanup_resp.is_none() {
            self.cmd_cleanup_resp.set_default();
        };
        self.cmd_cleanup_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_cleanup_resp(&mut self) -> CmdCleanupResponse {
        self.cmd_cleanup_resp.take().unwrap_or_else(|| CmdCleanupResponse::new())
    }

    pub fn get_cmd_cleanup_resp<'a>(&'a self) -> &'a CmdCleanupResponse {
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
    pub fn mut_cmd_rb_get_resp<'a>(&'a mut self) -> &'a mut CmdRollbackThenGetResponse {
        if self.cmd_rb_get_resp.is_none() {
            self.cmd_rb_get_resp.set_default();
        };
        self.cmd_rb_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_rb_get_resp(&mut self) -> CmdRollbackThenGetResponse {
        self.cmd_rb_get_resp.take().unwrap_or_else(|| CmdRollbackThenGetResponse::new())
    }

    pub fn get_cmd_rb_get_resp<'a>(&'a self) -> &'a CmdRollbackThenGetResponse {
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
    pub fn mut_cmd_commit_get_resp<'a>(&'a mut self) -> &'a mut CmdCommitThenGetResponse {
        if self.cmd_commit_get_resp.is_none() {
            self.cmd_commit_get_resp.set_default();
        };
        self.cmd_commit_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_commit_get_resp(&mut self) -> CmdCommitThenGetResponse {
        self.cmd_commit_get_resp.take().unwrap_or_else(|| CmdCommitThenGetResponse::new())
    }

    pub fn get_cmd_commit_get_resp<'a>(&'a self) -> &'a CmdCommitThenGetResponse {
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
    pub fn mut_cmd_batch_get_resp<'a>(&'a mut self) -> &'a mut CmdBatchGetResponse {
        if self.cmd_batch_get_resp.is_none() {
            self.cmd_batch_get_resp.set_default();
        };
        self.cmd_batch_get_resp.as_mut().unwrap()
    }

    // Take field
    pub fn take_cmd_batch_get_resp(&mut self) -> CmdBatchGetResponse {
        self.cmd_batch_get_resp.take().unwrap_or_else(|| CmdBatchGetResponse::new())
    }

    pub fn get_cmd_batch_get_resp<'a>(&'a self) -> &'a CmdBatchGetResponse {
        self.cmd_batch_get_resp.as_ref().unwrap_or_else(|| CmdBatchGetResponse::default_instance())
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
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x62, 0x0a, 0x08, 0x4c, 0x6f, 0x63, 0x6b, 0x49, 0x6e, 0x66,
    0x6f, 0x12, 0x21, 0x0a, 0x0c, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x5f, 0x6c, 0x6f, 0x63,
    0x6b, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79,
    0x4c, 0x6f, 0x63, 0x6b, 0x12, 0x21, 0x0a, 0x0c, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x6c, 0x6f, 0x63, 0x6b,
    0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x22, 0x69, 0x0a, 0x08, 0x4b, 0x65, 0x79,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x29, 0x0a, 0x06, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x4c, 0x6f, 0x63, 0x6b, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x06, 0x6c, 0x6f, 0x63, 0x6b, 0x65, 0x64,
    0x12, 0x1c, 0x0a, 0x09, 0x72, 0x65, 0x74, 0x72, 0x79, 0x61, 0x62, 0x6c, 0x65, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x09, 0x72, 0x65, 0x74, 0x72, 0x79, 0x61, 0x62, 0x6c, 0x65, 0x12, 0x14,
    0x0a, 0x05, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x61,
    0x62, 0x6f, 0x72, 0x74, 0x22, 0x5e, 0x0a, 0x07, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x12,
    0x1b, 0x0a, 0x09, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x08, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x36, 0x0a, 0x0c,
    0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x52, 0x0b, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x45,
    0x70, 0x6f, 0x63, 0x68, 0x22, 0x3b, 0x0a, 0x0d, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x22, 0x4f, 0x0a, 0x0e, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x22, 0x5d, 0x0a, 0x0e, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71,
    0x75, 0x65, 0x73, 0x74, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65,
    0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x73, 0x74, 0x61, 0x72, 0x74, 0x4b, 0x65,
    0x79, 0x12, 0x14, 0x0a, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x05, 0x6c, 0x69, 0x6d, 0x69, 0x74, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x22, 0x59, 0x0a, 0x06, 0x4b, 0x76, 0x50, 0x61, 0x69, 0x72, 0x12, 0x27, 0x0a, 0x05, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72,
    0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65,
    0x72, 0x72, 0x6f, 0x72, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x38, 0x0a, 0x0f,
    0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x25, 0x0a, 0x05, 0x70, 0x61, 0x69, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x76, 0x50, 0x61, 0x69, 0x72, 0x52,
    0x05, 0x70, 0x61, 0x69, 0x72, 0x73, 0x22, 0x4f, 0x0a, 0x08, 0x4d, 0x75, 0x74, 0x61, 0x74, 0x69,
    0x6f, 0x6e, 0x12, 0x1b, 0x0a, 0x02, 0x6f, 0x70, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x0b,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4f, 0x70, 0x52, 0x02, 0x6f, 0x70, 0x12,
    0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65,
    0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x8d, 0x01, 0x0a, 0x12, 0x43, 0x6d, 0x64, 0x50,
    0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x2f,
    0x0a, 0x09, 0x6d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4d, 0x75, 0x74, 0x61,
    0x74, 0x69, 0x6f, 0x6e, 0x52, 0x09, 0x6d, 0x75, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12,
    0x21, 0x0a, 0x0c, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x5f, 0x6c, 0x6f, 0x63, 0x6b, 0x18,
    0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x0b, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x4c, 0x6f,
    0x63, 0x6b, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73,
    0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x40, 0x0a, 0x13, 0x43, 0x6d, 0x64, 0x50, 0x72,
    0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x29,
    0x0a, 0x06, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x52, 0x06, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x22, 0x72, 0x0a, 0x10, 0x43, 0x6d, 0x64,
    0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x23, 0x0a,
    0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73, 0x74, 0x61, 0x72, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x12, 0x12, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0c,
    0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74,
    0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d,
    0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x3e, 0x0a,
    0x11, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x29, 0x0a, 0x06, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x06, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x73, 0x22, 0x4a, 0x0a,
    0x11, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x52, 0x65, 0x71, 0x75, 0x65,
    0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52,
    0x03, 0x6b, 0x65, 0x79, 0x12, 0x23, 0x0a, 0x0d, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x73, 0x74, 0x61,
    0x72, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x64, 0x0a, 0x12, 0x43, 0x6d, 0x64,
    0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x0d, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22,
    0x50, 0x0a, 0x19, 0x43, 0x6d, 0x64, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x54, 0x68,
    0x65, 0x6e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03,
    0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x21,
    0x0a, 0x0c, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x0b, 0x6c, 0x6f, 0x63, 0x6b, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x22, 0x5b, 0x0a, 0x1a, 0x43, 0x6d, 0x64, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b,
    0x54, 0x68, 0x65, 0x6e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12,
    0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x11,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x45, 0x72, 0x72, 0x6f,
    0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61, 0x6c, 0x75,
    0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65, 0x22, 0x96,
    0x01, 0x0a, 0x17, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x54, 0x68, 0x65, 0x6e,
    0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65,
    0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x21, 0x0a, 0x0c,
    0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x0b, 0x6c, 0x6f, 0x63, 0x6b, 0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12,
    0x25, 0x0a, 0x0e, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0d, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x56,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x0b, 0x67, 0x65, 0x74, 0x5f, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0a, 0x67, 0x65, 0x74,
    0x56, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x59, 0x0a, 0x18, 0x43, 0x6d, 0x64, 0x43, 0x6f,
    0x6d, 0x6d, 0x69, 0x74, 0x54, 0x68, 0x65, 0x6e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f,
    0x6e, 0x73, 0x65, 0x12, 0x27, 0x0a, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x11, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79,
    0x45, 0x72, 0x72, 0x6f, 0x72, 0x52, 0x05, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x12, 0x14, 0x0a, 0x05,
    0x76, 0x61, 0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c,
    0x75, 0x65, 0x22, 0x42, 0x0a, 0x12, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65,
    0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6b, 0x65, 0x79, 0x73,
    0x18, 0x01, 0x20, 0x03, 0x28, 0x0c, 0x52, 0x04, 0x6b, 0x65, 0x79, 0x73, 0x12, 0x18, 0x0a, 0x07,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76,
    0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x22, 0x3c, 0x0a, 0x13, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74,
    0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x12, 0x25, 0x0a,
    0x05, 0x70, 0x61, 0x69, 0x72, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0f, 0x2e, 0x6b,
    0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4b, 0x76, 0x50, 0x61, 0x69, 0x72, 0x52, 0x05, 0x70,
    0x61, 0x69, 0x72, 0x73, 0x22, 0xfe, 0x04, 0x0a, 0x07, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x12, 0x28, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x14,
    0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x2a, 0x0a, 0x07, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x78, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x10, 0x2e, 0x6b, 0x76,
    0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x52, 0x07, 0x63,
    0x6f, 0x6e, 0x74, 0x65, 0x78, 0x74, 0x12, 0x36, 0x0a, 0x0b, 0x63, 0x6d, 0x64, 0x5f, 0x67, 0x65,
    0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x16, 0x2e, 0x6b, 0x76,
    0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x52, 0x09, 0x63, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x39,
    0x0a, 0x0c, 0x63, 0x6d, 0x64, 0x5f, 0x73, 0x63, 0x61, 0x6e, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43,
    0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0a, 0x63,
    0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x71, 0x12, 0x45, 0x0a, 0x10, 0x63, 0x6d, 0x64,
    0x5f, 0x70, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d,
    0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74,
    0x52, 0x0e, 0x63, 0x6d, 0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x71,
    0x12, 0x3f, 0x0a, 0x0e, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x72,
    0x65, 0x71, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x19, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63,
    0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65, 0x71, 0x75,
    0x65, 0x73, 0x74, 0x52, 0x0c, 0x63, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52, 0x65,
    0x71, 0x12, 0x42, 0x0a, 0x0f, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70,
    0x5f, 0x72, 0x65, 0x71, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x76, 0x72,
    0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x52,
    0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0d, 0x63, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e,
    0x75, 0x70, 0x52, 0x65, 0x71, 0x12, 0x47, 0x0a, 0x0e, 0x63, 0x6d, 0x64, 0x5f, 0x72, 0x62, 0x5f,
    0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x71, 0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x22, 0x2e,
    0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x52, 0x6f, 0x6c, 0x6c, 0x62,
    0x61, 0x63, 0x6b, 0x54, 0x68, 0x65, 0x6e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73,
    0x74, 0x52, 0x0b, 0x63, 0x6d, 0x64, 0x52, 0x62, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x4d,
    0x0a, 0x12, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x67, 0x65, 0x74,
    0x5f, 0x72, 0x65, 0x71, 0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x20, 0x2e, 0x6b, 0x76, 0x72,
    0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x54, 0x68,
    0x65, 0x6e, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0f, 0x63, 0x6d,
    0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x47, 0x65, 0x74, 0x52, 0x65, 0x71, 0x12, 0x46, 0x0a,
    0x11, 0x63, 0x6d, 0x64, 0x5f, 0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72,
    0x65, 0x71, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1b, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63,
    0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65,
    0x71, 0x75, 0x65, 0x73, 0x74, 0x52, 0x0e, 0x63, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47,
    0x65, 0x74, 0x52, 0x65, 0x71, 0x22, 0x9e, 0x05, 0x0a, 0x08, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e,
    0x73, 0x65, 0x12, 0x28, 0x0a, 0x04, 0x74, 0x79, 0x70, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0e,
    0x32, 0x14, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x52, 0x04, 0x74, 0x79, 0x70, 0x65, 0x12, 0x31, 0x0a, 0x0c,
    0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x52, 0x0b, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x45, 0x72, 0x72, 0x6f, 0x72, 0x12,
    0x39, 0x0a, 0x0c, 0x63, 0x6d, 0x64, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e,
    0x43, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0a,
    0x63, 0x6d, 0x64, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x3c, 0x0a, 0x0d, 0x63, 0x6d,
    0x64, 0x5f, 0x73, 0x63, 0x61, 0x6e, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x18, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x53,
    0x63, 0x61, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0b, 0x63, 0x6d, 0x64,
    0x53, 0x63, 0x61, 0x6e, 0x52, 0x65, 0x73, 0x70, 0x12, 0x48, 0x0a, 0x11, 0x63, 0x6d, 0x64, 0x5f,
    0x70, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d,
    0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73,
    0x65, 0x52, 0x0f, 0x63, 0x6d, 0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x52, 0x65,
    0x73, 0x70, 0x12, 0x42, 0x0a, 0x0f, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74,
    0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x06, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x6b, 0x76,
    0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x52,
    0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0d, 0x63, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d,
    0x69, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x45, 0x0a, 0x10, 0x63, 0x6d, 0x64, 0x5f, 0x63, 0x6c,
    0x65, 0x61, 0x6e, 0x75, 0x70, 0x5f, 0x72, 0x65, 0x73, 0x70, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x1b, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62, 0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6c,
    0x65, 0x61, 0x6e, 0x75, 0x70, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0e, 0x63,
    0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x52, 0x65, 0x73, 0x70, 0x12, 0x4a, 0x0a,
    0x0f, 0x63, 0x6d, 0x64, 0x5f, 0x72, 0x62, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70,
    0x18, 0x08, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x23, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62,
    0x2e, 0x43, 0x6d, 0x64, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x54, 0x68, 0x65, 0x6e,
    0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0c, 0x63, 0x6d, 0x64,
    0x52, 0x62, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x50, 0x0a, 0x13, 0x63, 0x6d, 0x64,
    0x5f, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73, 0x70,
    0x18, 0x09, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x21, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70, 0x62,
    0x2e, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x54, 0x68, 0x65, 0x6e, 0x47, 0x65,
    0x74, 0x52, 0x65, 0x73, 0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x10, 0x63, 0x6d, 0x64, 0x43, 0x6f,
    0x6d, 0x6d, 0x69, 0x74, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x12, 0x49, 0x0a, 0x12, 0x63,
    0x6d, 0x64, 0x5f, 0x62, 0x61, 0x74, 0x63, 0x68, 0x5f, 0x67, 0x65, 0x74, 0x5f, 0x72, 0x65, 0x73,
    0x70, 0x18, 0x0a, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1c, 0x2e, 0x6b, 0x76, 0x72, 0x70, 0x63, 0x70,
    0x62, 0x2e, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x52, 0x65, 0x73,
    0x70, 0x6f, 0x6e, 0x73, 0x65, 0x52, 0x0f, 0x63, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47,
    0x65, 0x74, 0x52, 0x65, 0x73, 0x70, 0x2a, 0x95, 0x01, 0x0a, 0x0b, 0x4d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x54, 0x79, 0x70, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x43, 0x6d, 0x64, 0x47, 0x65, 0x74,
    0x10, 0x01, 0x12, 0x0b, 0x0a, 0x07, 0x43, 0x6d, 0x64, 0x53, 0x63, 0x61, 0x6e, 0x10, 0x02, 0x12,
    0x0f, 0x0a, 0x0b, 0x43, 0x6d, 0x64, 0x50, 0x72, 0x65, 0x77, 0x72, 0x69, 0x74, 0x65, 0x10, 0x03,
    0x12, 0x0d, 0x0a, 0x09, 0x43, 0x6d, 0x64, 0x43, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x10, 0x04, 0x12,
    0x0e, 0x0a, 0x0a, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70, 0x10, 0x05, 0x12,
    0x16, 0x0a, 0x12, 0x43, 0x6d, 0x64, 0x52, 0x6f, 0x6c, 0x6c, 0x62, 0x61, 0x63, 0x6b, 0x54, 0x68,
    0x65, 0x6e, 0x47, 0x65, 0x74, 0x10, 0x06, 0x12, 0x14, 0x0a, 0x10, 0x43, 0x6d, 0x64, 0x43, 0x6f,
    0x6d, 0x6d, 0x69, 0x74, 0x54, 0x68, 0x65, 0x6e, 0x47, 0x65, 0x74, 0x10, 0x07, 0x12, 0x0f, 0x0a,
    0x0b, 0x43, 0x6d, 0x64, 0x42, 0x61, 0x74, 0x63, 0x68, 0x47, 0x65, 0x74, 0x10, 0x08, 0x2a, 0x20,
    0x0a, 0x02, 0x4f, 0x70, 0x12, 0x07, 0x0a, 0x03, 0x50, 0x75, 0x74, 0x10, 0x01, 0x12, 0x07, 0x0a,
    0x03, 0x44, 0x65, 0x6c, 0x10, 0x02, 0x12, 0x08, 0x0a, 0x04, 0x4c, 0x6f, 0x63, 0x6b, 0x10, 0x03,
    0x4a, 0xa8, 0x31, 0x0a, 0x07, 0x12, 0x05, 0x00, 0x00, 0xa1, 0x01, 0x01, 0x0a, 0x08, 0x0a, 0x01,
    0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0f,
    0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x02, 0x07, 0x15, 0x0a, 0x09, 0x0a, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x03, 0x07, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x05, 0x00,
    0x12, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x05, 0x05, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02,
    0x00, 0x02, 0x12, 0x03, 0x06, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x07, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x07,
    0x04, 0x0b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x07, 0x1a, 0x1b,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x08, 0x04, 0x1c, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x08, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x08, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x03, 0x12, 0x03, 0x09, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x09, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x03, 0x02, 0x12, 0x03, 0x09,
    0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0a, 0x04, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0a, 0x04, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x04, 0x02, 0x12, 0x03, 0x0a, 0x1a, 0x1b, 0x0a, 0x8a, 0x02, 0x0a, 0x04,
    0x05, 0x00, 0x02, 0x05, 0x12, 0x03, 0x0f, 0x04, 0x1c, 0x1a, 0xfc, 0x01, 0x20, 0x42, 0x65, 0x6c,
    0x6f, 0x77, 0x20, 0x74, 0x79, 0x70, 0x65, 0x73, 0x20, 0x62, 0x6f, 0x74, 0x68, 0x20, 0x75, 0x73,
    0x65, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x47, 0x65, 0x74, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64,
    0x2e, 0x20, 0x49, 0x66, 0x20, 0x47, 0x65, 0x74, 0x20, 0x66, 0x61, 0x69, 0x6c, 0x65, 0x64, 0x2c,
    0x20, 0x69, 0x74, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x62, 0x65, 0x20, 0x6c, 0x6f, 0x63, 0x6b, 0x65,
    0x64, 0x2e, 0x0a, 0x20, 0x53, 0x6f, 0x20, 0x69, 0x74, 0x20, 0x74, 0x72, 0x69, 0x65, 0x73, 0x20,
    0x74, 0x6f, 0x20, 0x63, 0x6c, 0x65, 0x61, 0x6e, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79,
    0x20, 0x6c, 0x6f, 0x63, 0x6b, 0x28, 0x43, 0x6d, 0x64, 0x43, 0x6c, 0x65, 0x61, 0x6e, 0x75, 0x70,
    0x29, 0x2c, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x72, 0x65, 0x74, 0x75, 0x72, 0x6e, 0x0a, 0x20,
    0x65, 0x69, 0x74, 0x68, 0x65, 0x72, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64,
    0x20, 0x6f, 0x72, 0x20, 0x72, 0x6f, 0x6c, 0x6c, 0x65, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x2e,
    0x20, 0x46, 0x69, 0x6e, 0x61, 0x6c, 0x6c, 0x79, 0x2c, 0x20, 0x63, 0x6c, 0x69, 0x65, 0x6e, 0x74,
    0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x2f, 0x72, 0x6f, 0x6c,
    0x6c, 0x62, 0x61, 0x63, 0x6b, 0x0a, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x20, 0x6c,
    0x6f, 0x63, 0x6b, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x74, 0x68, 0x65, 0x6e, 0x20, 0x47, 0x65, 0x74,
    0x20, 0x61, 0x67, 0x61, 0x69, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05,
    0x01, 0x12, 0x03, 0x0f, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x05, 0x02, 0x12,
    0x03, 0x0f, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x06, 0x12, 0x03, 0x10, 0x04,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x10, 0x04, 0x14, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x06, 0x02, 0x12, 0x03, 0x10, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a,
    0x04, 0x05, 0x00, 0x02, 0x07, 0x12, 0x03, 0x11, 0x04, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00,
    0x02, 0x07, 0x01, 0x12, 0x03, 0x11, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x07,
    0x02, 0x12, 0x03, 0x11, 0x1a, 0x1b, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x14, 0x00,
    0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x10, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x15, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x15, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x15, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x15, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x15, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x16, 0x04, 0x25,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x16, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x16, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x17, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x17, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x17, 0x0d,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x14, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x23, 0x24, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x1a, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x1a, 0x08, 0x10, 0x0a, 0x44, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1b,
    0x04, 0x24, 0x22, 0x37, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75,
    0x6c, 0x64, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x6f, 0x66, 0x66, 0x20, 0x6f, 0x72, 0x20, 0x63, 0x6c,
    0x65, 0x61, 0x6e, 0x75, 0x70, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6c, 0x6f, 0x63, 0x6b, 0x20, 0x74,
    0x68, 0x65, 0x6e, 0x20, 0x72, 0x65, 0x74, 0x72, 0x79, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x1b, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x1b, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x1b, 0x22, 0x23, 0x0a, 0x3e, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x24,
    0x22, 0x31, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x61, 0x79, 0x20, 0x72, 0x65,
    0x73, 0x74, 0x61, 0x72, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x74, 0x78, 0x6e, 0x2e, 0x20, 0x65,
    0x2e, 0x67, 0x20, 0x77, 0x72, 0x69, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x66, 0x6c, 0x69, 0x63,
    0x74, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1c, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1c, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1c, 0x16, 0x1f, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x1c, 0x22, 0x23, 0x0a, 0x2b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x1d, 0x04, 0x24, 0x22, 0x1e, 0x20, 0x43, 0x6c, 0x69, 0x65, 0x6e,
    0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x61, 0x62, 0x6f, 0x72, 0x74, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x74, 0x78, 0x6e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x1d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1d,
    0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x22, 0x23,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x20, 0x00, 0x23, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x20, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x21, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x21, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x21, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x20, 0x29, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x2f, 0x30, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x22, 0x04, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x01, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01,
    0x06, 0x12, 0x03, 0x22, 0x0d, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x22, 0x20, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x22,
    0x2f, 0x30, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x25, 0x00, 0x28, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x25, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03,
    0x02, 0x00, 0x12, 0x03, 0x26, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04,
    0x12, 0x03, 0x26, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x26, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x26, 0x15,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x26, 0x1f, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x27, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x27, 0x15, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x27, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x2a, 0x00, 0x2d, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x2a, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x2b, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x2b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x2b, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x2b, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2b, 0x1e,
    0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03, 0x2c, 0x04, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x03, 0x2c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x2c, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x2c, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05, 0x12, 0x04, 0x2f, 0x00,
    0x33, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x2f, 0x08, 0x16, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x30, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x30, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x05, 0x12, 0x03, 0x30, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x30, 0x15, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x30, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12, 0x03, 0x31, 0x04, 0x23,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x31, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x05, 0x12, 0x03, 0x31, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x31, 0x15, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x31, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x02,
    0x12, 0x03, 0x32, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x04, 0x12, 0x03,
    0x32, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x05, 0x12, 0x03, 0x32, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03, 0x32, 0x15, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x32, 0x21, 0x22, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x06, 0x12, 0x04, 0x35, 0x00, 0x39, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01,
    0x12, 0x03, 0x35, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x00, 0x12, 0x03, 0x36,
    0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x04, 0x12, 0x03, 0x36, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x06, 0x12, 0x03, 0x36, 0x0d, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03, 0x36, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x36, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06,
    0x02, 0x01, 0x12, 0x03, 0x37, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04,
    0x12, 0x03, 0x37, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x37, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x37, 0x16,
    0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x03, 0x12, 0x03, 0x37, 0x1e, 0x1f, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x02, 0x12, 0x03, 0x38, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x06, 0x02, 0x02, 0x04, 0x12, 0x03, 0x38, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06,
    0x02, 0x02, 0x05, 0x12, 0x03, 0x38, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x38, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x02, 0x03, 0x12,
    0x03, 0x38, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x3b, 0x00, 0x3d, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x3b, 0x08, 0x17, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x3c, 0x04, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x3c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x3c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x3c, 0x14, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3c, 0x1c,
    0x1d, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x3f, 0x00, 0x43, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x3f, 0x05, 0x07, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02,
    0x00, 0x12, 0x03, 0x40, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x40, 0x04, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x00, 0x02, 0x12, 0x03, 0x40,
    0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01, 0x12, 0x03, 0x41, 0x04, 0x0d, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x41, 0x04, 0x07, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x41, 0x0b, 0x0c, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x01, 0x02, 0x02, 0x12, 0x03, 0x42, 0x04, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x42, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x42, 0x0b, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x08, 0x12, 0x04, 0x45, 0x00, 0x49, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12, 0x03, 0x45, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x46, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02,
    0x00, 0x04, 0x12, 0x03, 0x46, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x06,
    0x12, 0x03, 0x46, 0x0d, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x46, 0x13, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x46, 0x1b,
    0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x01, 0x12, 0x03, 0x47, 0x04, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04, 0x12, 0x03, 0x47, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x01, 0x05, 0x12, 0x03, 0x47, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08,
    0x02, 0x01, 0x01, 0x12, 0x03, 0x47, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01,
    0x03, 0x12, 0x03, 0x47, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x02, 0x12, 0x03,
    0x48, 0x04, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x04, 0x12, 0x03, 0x48, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x05, 0x12, 0x03, 0x48, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x02, 0x01, 0x12, 0x03, 0x48, 0x13, 0x18, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x08, 0x02, 0x02, 0x03, 0x12, 0x03, 0x48, 0x1b, 0x1c, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x09, 0x12, 0x04, 0x4b, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x09, 0x01, 0x12, 0x03,
    0x4b, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00, 0x12, 0x03, 0x4c, 0x04, 0x26,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x03, 0x4c, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4c, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4c, 0x16, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x4c, 0x24, 0x25, 0x0a, 0x1f, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x01,
    0x12, 0x03, 0x4e, 0x04, 0x26, 0x1a, 0x12, 0x20, 0x70, 0x72, 0x69, 0x6d, 0x61, 0x72, 0x79, 0x5f,
    0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x6b, 0x65, 0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02,
    0x01, 0x04, 0x12, 0x03, 0x4e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x4e, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x4e, 0x13, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4e, 0x24,
    0x25, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x02, 0x12, 0x03, 0x4f, 0x04, 0x26, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x09, 0x02, 0x02, 0x04, 0x12, 0x03, 0x4f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x09, 0x02, 0x02, 0x05, 0x12, 0x03, 0x4f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x4f, 0x14, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x4f, 0x24, 0x25, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x52, 0x00,
    0x54, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x52, 0x08, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x53, 0x04, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0a, 0x02, 0x00, 0x04, 0x12, 0x03, 0x53, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x53, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x53, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x53, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0b, 0x12, 0x04, 0x56, 0x00, 0x5a, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03, 0x56, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x0b, 0x02, 0x00, 0x12, 0x03, 0x57, 0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x57, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x57, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x57,
    0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x57, 0x26, 0x27,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x01, 0x12, 0x03, 0x58, 0x04, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12, 0x03, 0x58, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x01, 0x05, 0x12, 0x03, 0x58, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x58, 0x15, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x58, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x02, 0x12, 0x03, 0x59,
    0x04, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x04, 0x12, 0x03, 0x59, 0x04, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x05, 0x12, 0x03, 0x59, 0x0d, 0x13, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0b, 0x02, 0x02, 0x01, 0x12, 0x03, 0x59, 0x15, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0b, 0x02, 0x02, 0x03, 0x12, 0x03, 0x59, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0c,
    0x12, 0x04, 0x5c, 0x00, 0x5e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0c, 0x01, 0x12, 0x03, 0x5c,
    0x08, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0c, 0x02, 0x00, 0x12, 0x03, 0x5d, 0x04, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02, 0x00, 0x04, 0x12, 0x03, 0x5d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0c, 0x02, 0x00, 0x06, 0x12, 0x03, 0x5d, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0c, 0x02, 0x00, 0x01, 0x12, 0x03, 0x5d, 0x16, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0c, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x5d, 0x1f, 0x20, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0d, 0x12, 0x04, 0x60,
    0x00, 0x63, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0d, 0x01, 0x12, 0x03, 0x60, 0x08, 0x19, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x00, 0x12, 0x03, 0x61, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0d, 0x02, 0x00, 0x04, 0x12, 0x03, 0x61, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x61, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x61, 0x15, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x61, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0d, 0x02, 0x01, 0x12, 0x03, 0x62, 0x04,
    0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x04, 0x12, 0x03, 0x62, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0d, 0x02, 0x01, 0x05, 0x12, 0x03, 0x62, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0d, 0x02, 0x01, 0x01, 0x12, 0x03, 0x62, 0x15, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0d, 0x02, 0x01, 0x03, 0x12, 0x03, 0x62, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0e, 0x12,
    0x04, 0x65, 0x00, 0x68, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0e, 0x01, 0x12, 0x03, 0x65, 0x08,
    0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x00, 0x12, 0x03, 0x66, 0x04, 0x29, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00, 0x04, 0x12, 0x03, 0x66, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0e, 0x02, 0x00, 0x06, 0x12, 0x03, 0x66, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x66, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x66, 0x27, 0x28, 0x0a, 0x37, 0x0a, 0x04, 0x04, 0x0e, 0x02, 0x01, 0x12, 0x03,
    0x67, 0x04, 0x29, 0x22, 0x2a, 0x20, 0x73, 0x65, 0x74, 0x20, 0x74, 0x68, 0x69, 0x73, 0x20, 0x69,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x69, 0x73, 0x20, 0x61, 0x6c, 0x72,
    0x65, 0x61, 0x64, 0x79, 0x20, 0x63, 0x6f, 0x6d, 0x6d, 0x69, 0x74, 0x74, 0x65, 0x64, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02, 0x01, 0x04, 0x12, 0x03, 0x67, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0e, 0x02, 0x01, 0x05, 0x12, 0x03, 0x67, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0e, 0x02, 0x01, 0x01, 0x12, 0x03, 0x67, 0x16, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0e, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x67, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0f, 0x12, 0x04, 0x6a,
    0x00, 0x6d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0f, 0x01, 0x12, 0x03, 0x6a, 0x08, 0x21, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x00, 0x12, 0x03, 0x6b, 0x04, 0x25, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0f, 0x02, 0x00, 0x04, 0x12, 0x03, 0x6b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x6b, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x6b, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x6b, 0x23, 0x24, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0f, 0x02, 0x01, 0x12, 0x03, 0x6c, 0x04,
    0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x04, 0x12, 0x03, 0x6c, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0f, 0x02, 0x01, 0x05, 0x12, 0x03, 0x6c, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0f, 0x02, 0x01, 0x01, 0x12, 0x03, 0x6c, 0x14, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0f, 0x02, 0x01, 0x03, 0x12, 0x03, 0x6c, 0x23, 0x24, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x10, 0x12,
    0x04, 0x6f, 0x00, 0x72, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x10, 0x01, 0x12, 0x03, 0x6f, 0x08,
    0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x00, 0x12, 0x03, 0x70, 0x04, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x10, 0x02, 0x00, 0x04, 0x12, 0x03, 0x70, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x10, 0x02, 0x00, 0x06, 0x12, 0x03, 0x70, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x70, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x70, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x10, 0x02, 0x01, 0x12, 0x03,
    0x71, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x04, 0x12, 0x03, 0x71, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x05, 0x12, 0x03, 0x71, 0x0d, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x10, 0x02, 0x01, 0x01, 0x12, 0x03, 0x71, 0x16, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x10, 0x02, 0x01, 0x03, 0x12, 0x03, 0x71, 0x1e, 0x1f, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x11, 0x12, 0x04, 0x74, 0x00, 0x79, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x11, 0x01, 0x12, 0x03,
    0x74, 0x08, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x00, 0x12, 0x03, 0x75, 0x04, 0x27,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x04, 0x12, 0x03, 0x75, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x11, 0x02, 0x00, 0x05, 0x12, 0x03, 0x75, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x11, 0x02, 0x00, 0x01, 0x12, 0x03, 0x75, 0x14, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x75, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x01,
    0x12, 0x03, 0x76, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x04, 0x12, 0x03,
    0x76, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x05, 0x12, 0x03, 0x76, 0x0d,
    0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x01, 0x12, 0x03, 0x76, 0x14, 0x20, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x01, 0x03, 0x12, 0x03, 0x76, 0x25, 0x26, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x11, 0x02, 0x02, 0x12, 0x03, 0x77, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x77, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02,
    0x05, 0x12, 0x03, 0x77, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x77, 0x14, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x02, 0x03, 0x12, 0x03, 0x77,
    0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x11, 0x02, 0x03, 0x12, 0x03, 0x78, 0x04, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02, 0x03, 0x04, 0x12, 0x03, 0x78, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x11, 0x02, 0x03, 0x05, 0x12, 0x03, 0x78, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x11, 0x02, 0x03, 0x01, 0x12, 0x03, 0x78, 0x14, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x11, 0x02,
    0x03, 0x03, 0x12, 0x03, 0x78, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x12, 0x12, 0x04, 0x7b,
    0x00, 0x7e, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x12, 0x01, 0x12, 0x03, 0x7b, 0x08, 0x20, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x00, 0x12, 0x03, 0x7c, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x12, 0x02, 0x00, 0x04, 0x12, 0x03, 0x7c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12,
    0x02, 0x00, 0x06, 0x12, 0x03, 0x7c, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x7c, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x7c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x12, 0x02, 0x01, 0x12, 0x03, 0x7d, 0x04,
    0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x04, 0x12, 0x03, 0x7d, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x12, 0x02, 0x01, 0x05, 0x12, 0x03, 0x7d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x12, 0x02, 0x01, 0x01, 0x12, 0x03, 0x7d, 0x16, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x12, 0x02, 0x01, 0x03, 0x12, 0x03, 0x7d, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x13, 0x12,
    0x06, 0x80, 0x01, 0x00, 0x83, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x13, 0x01, 0x12, 0x04,
    0x80, 0x01, 0x08, 0x1a, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x13, 0x02, 0x00, 0x12, 0x04, 0x81, 0x01,
    0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x04, 0x12, 0x04, 0x81, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x05, 0x12, 0x04, 0x81, 0x01, 0x0d, 0x12,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x01, 0x12, 0x04, 0x81, 0x01, 0x14, 0x18, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02, 0x00, 0x03, 0x12, 0x04, 0x81, 0x01, 0x1e, 0x1f, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x13, 0x02, 0x01, 0x12, 0x04, 0x82, 0x01, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x13, 0x02, 0x01, 0x04, 0x12, 0x04, 0x82, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x13, 0x02, 0x01, 0x05, 0x12, 0x04, 0x82, 0x01, 0x0d, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13,
    0x02, 0x01, 0x01, 0x12, 0x04, 0x82, 0x01, 0x14, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x13, 0x02,
    0x01, 0x03, 0x12, 0x04, 0x82, 0x01, 0x1e, 0x1f, 0x0a, 0x0c, 0x0a, 0x02, 0x04, 0x14, 0x12, 0x06,
    0x85, 0x01, 0x00, 0x87, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x14, 0x01, 0x12, 0x04, 0x85,
    0x01, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x14, 0x02, 0x00, 0x12, 0x04, 0x86, 0x01, 0x04,
    0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x04, 0x12, 0x04, 0x86, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x06, 0x12, 0x04, 0x86, 0x01, 0x0d, 0x13, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x01, 0x12, 0x04, 0x86, 0x01, 0x14, 0x19, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x14, 0x02, 0x00, 0x03, 0x12, 0x04, 0x86, 0x01, 0x1c, 0x1d, 0x0a, 0x0c, 0x0a,
    0x02, 0x04, 0x15, 0x12, 0x06, 0x89, 0x01, 0x00, 0x94, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04,
    0x15, 0x01, 0x12, 0x04, 0x89, 0x01, 0x08, 0x0f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x00,
    0x12, 0x04, 0x8a, 0x01, 0x04, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x8a, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x06, 0x12, 0x04,
    0x8a, 0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x01, 0x12, 0x04, 0x8a,
    0x01, 0x27, 0x2b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x00, 0x03, 0x12, 0x04, 0x8a, 0x01,
    0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x3e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x04, 0x12, 0x04, 0x8b, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x06, 0x12, 0x04, 0x8b, 0x01, 0x0d, 0x14, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x01, 0x01, 0x12, 0x04, 0x8b, 0x01, 0x27, 0x2e, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x01, 0x03, 0x12, 0x04, 0x8b, 0x01, 0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x04,
    0x04, 0x15, 0x02, 0x02, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x02, 0x04, 0x12, 0x04, 0x8c, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x02, 0x06, 0x12, 0x04, 0x8c, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02,
    0x01, 0x12, 0x04, 0x8c, 0x01, 0x27, 0x32, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x02, 0x03,
    0x12, 0x04, 0x8c, 0x01, 0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x03, 0x12, 0x04,
    0x8d, 0x01, 0x04, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x04, 0x12, 0x04, 0x8d,
    0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x06, 0x12, 0x04, 0x8d, 0x01,
    0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x01, 0x12, 0x04, 0x8d, 0x01, 0x27,
    0x33, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x03, 0x03, 0x12, 0x04, 0x8d, 0x01, 0x3c, 0x3d,
    0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x3e, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x04, 0x04, 0x12, 0x04, 0x8e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x04, 0x06, 0x12, 0x04, 0x8e, 0x01, 0x0d, 0x1f, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x04, 0x01, 0x12, 0x04, 0x8e, 0x01, 0x27, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x04, 0x03, 0x12, 0x04, 0x8e, 0x01, 0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15,
    0x02, 0x05, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05,
    0x04, 0x12, 0x04, 0x8f, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x06,
    0x12, 0x04, 0x8f, 0x01, 0x0d, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x01, 0x12,
    0x04, 0x8f, 0x01, 0x27, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x05, 0x03, 0x12, 0x04,
    0x8f, 0x01, 0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x06, 0x12, 0x04, 0x90, 0x01,
    0x04, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x04, 0x12, 0x04, 0x90, 0x01, 0x04,
    0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x06, 0x12, 0x04, 0x90, 0x01, 0x0d, 0x1e,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x01, 0x12, 0x04, 0x90, 0x01, 0x27, 0x36, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x06, 0x03, 0x12, 0x04, 0x90, 0x01, 0x3c, 0x3d, 0x0a, 0x0c,
    0x0a, 0x04, 0x04, 0x15, 0x02, 0x07, 0x12, 0x04, 0x91, 0x01, 0x04, 0x3e, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x15, 0x02, 0x07, 0x04, 0x12, 0x04, 0x91, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x15, 0x02, 0x07, 0x06, 0x12, 0x04, 0x91, 0x01, 0x0d, 0x26, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15,
    0x02, 0x07, 0x01, 0x12, 0x04, 0x91, 0x01, 0x27, 0x35, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02,
    0x07, 0x03, 0x12, 0x04, 0x91, 0x01, 0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x08,
    0x12, 0x04, 0x92, 0x01, 0x04, 0x3e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x04, 0x12,
    0x04, 0x92, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x06, 0x12, 0x04,
    0x92, 0x01, 0x0d, 0x24, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x01, 0x12, 0x04, 0x92,
    0x01, 0x27, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x08, 0x03, 0x12, 0x04, 0x92, 0x01,
    0x3c, 0x3d, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x15, 0x02, 0x09, 0x12, 0x04, 0x93, 0x01, 0x04, 0x3f,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x04, 0x12, 0x04, 0x93, 0x01, 0x04, 0x0c, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x06, 0x12, 0x04, 0x93, 0x01, 0x0d, 0x1f, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x15, 0x02, 0x09, 0x01, 0x12, 0x04, 0x93, 0x01, 0x27, 0x38, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x15, 0x02, 0x09, 0x03, 0x12, 0x04, 0x93, 0x01, 0x3c, 0x3e, 0x0a, 0x0c, 0x0a, 0x02,
    0x04, 0x16, 0x12, 0x06, 0x96, 0x01, 0x00, 0xa1, 0x01, 0x01, 0x0a, 0x0b, 0x0a, 0x03, 0x04, 0x16,
    0x01, 0x12, 0x04, 0x96, 0x01, 0x08, 0x10, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x00, 0x12,
    0x04, 0x97, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x97, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x06, 0x12, 0x04, 0x97,
    0x01, 0x0d, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x01, 0x12, 0x04, 0x97, 0x01,
    0x28, 0x2c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x00, 0x03, 0x12, 0x04, 0x97, 0x01, 0x3e,
    0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x01, 0x12, 0x04, 0x98, 0x01, 0x04, 0x40, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x04, 0x12, 0x04, 0x98, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x01, 0x06, 0x12, 0x04, 0x98, 0x01, 0x0d, 0x1a, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x01, 0x01, 0x12, 0x04, 0x98, 0x01, 0x28, 0x34, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x01, 0x03, 0x12, 0x04, 0x98, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04,
    0x16, 0x02, 0x02, 0x12, 0x04, 0x99, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x02, 0x04, 0x12, 0x04, 0x99, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02,
    0x06, 0x12, 0x04, 0x99, 0x01, 0x0d, 0x1b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x01,
    0x12, 0x04, 0x99, 0x01, 0x28, 0x34, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x02, 0x03, 0x12,
    0x04, 0x99, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x03, 0x12, 0x04, 0x9a,
    0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x04, 0x12, 0x04, 0x9a, 0x01,
    0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x06, 0x12, 0x04, 0x9a, 0x01, 0x0d,
    0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x01, 0x12, 0x04, 0x9a, 0x01, 0x28, 0x35,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x03, 0x03, 0x12, 0x04, 0x9a, 0x01, 0x3e, 0x3f, 0x0a,
    0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x04, 0x04, 0x12, 0x04, 0x9b, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x04, 0x06, 0x12, 0x04, 0x9b, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x04, 0x01, 0x12, 0x04, 0x9b, 0x01, 0x28, 0x39, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x04, 0x03, 0x12, 0x04, 0x9b, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02,
    0x05, 0x12, 0x04, 0x9c, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x04,
    0x12, 0x04, 0x9c, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x06, 0x12,
    0x04, 0x9c, 0x01, 0x0d, 0x1e, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x01, 0x12, 0x04,
    0x9c, 0x01, 0x28, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x05, 0x03, 0x12, 0x04, 0x9c,
    0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x06, 0x12, 0x04, 0x9d, 0x01, 0x04,
    0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x06, 0x04, 0x12, 0x04, 0x9d, 0x01, 0x04, 0x0c,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x06, 0x06, 0x12, 0x04, 0x9d, 0x01, 0x0d, 0x1f, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x06, 0x01, 0x12, 0x04, 0x9d, 0x01, 0x28, 0x38, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x06, 0x03, 0x12, 0x04, 0x9d, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x16, 0x02, 0x07, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x16, 0x02, 0x07, 0x04, 0x12, 0x04, 0x9e, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16,
    0x02, 0x07, 0x06, 0x12, 0x04, 0x9e, 0x01, 0x0d, 0x27, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02,
    0x07, 0x01, 0x12, 0x04, 0x9e, 0x01, 0x28, 0x37, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x07,
    0x03, 0x12, 0x04, 0x9e, 0x01, 0x3e, 0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x08, 0x12,
    0x04, 0x9f, 0x01, 0x04, 0x40, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x04, 0x12, 0x04,
    0x9f, 0x01, 0x04, 0x0c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x06, 0x12, 0x04, 0x9f,
    0x01, 0x0d, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x01, 0x12, 0x04, 0x9f, 0x01,
    0x28, 0x3b, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x08, 0x03, 0x12, 0x04, 0x9f, 0x01, 0x3e,
    0x3f, 0x0a, 0x0c, 0x0a, 0x04, 0x04, 0x16, 0x02, 0x09, 0x12, 0x04, 0xa0, 0x01, 0x04, 0x41, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x16, 0x02, 0x09, 0x04, 0x12, 0x04, 0xa0, 0x01, 0x04, 0x0c, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x16, 0x02, 0x09, 0x06, 0x12, 0x04, 0xa0, 0x01, 0x0d, 0x20, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x16, 0x02, 0x09, 0x01, 0x12, 0x04, 0xa0, 0x01, 0x28, 0x3a, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x16, 0x02, 0x09, 0x03, 0x12, 0x04, 0xa0, 0x01, 0x3e, 0x40,
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
