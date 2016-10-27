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
pub struct NotLeader {
    // message fields
    region_id: ::std::option::Option<u64>,
    leader: ::protobuf::SingularPtrField<super::metapb::Peer>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for NotLeader {}

impl NotLeader {
    pub fn new() -> NotLeader {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static NotLeader {
        static mut instance: ::protobuf::lazy::Lazy<NotLeader> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const NotLeader,
        };
        unsafe {
            instance.get(|| {
                NotLeader {
                    region_id: ::std::option::Option::None,
                    leader: ::protobuf::SingularPtrField::none(),
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

impl ::protobuf::Message for NotLeader {
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
        for value in &self.region_id {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
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
        if let Some(v) = self.region_id {
            try!(os.write_uint64(1, v));
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
        ::std::any::TypeId::of::<NotLeader>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for NotLeader {
    fn new() -> NotLeader {
        NotLeader::new()
    }

    fn descriptor_static(_: ::std::option::Option<NotLeader>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "region_id",
                    NotLeader::has_region_id,
                    NotLeader::get_region_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "leader",
                    NotLeader::has_leader,
                    NotLeader::get_leader,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<NotLeader>(
                    "NotLeader",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for NotLeader {
    fn clear(&mut self) {
        self.clear_region_id();
        self.clear_leader();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for NotLeader {
    fn eq(&self, other: &NotLeader) -> bool {
        self.region_id == other.region_id &&
        self.leader == other.leader &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for NotLeader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RegionNotFound {
    // message fields
    region_id: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionNotFound {}

impl RegionNotFound {
    pub fn new() -> RegionNotFound {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionNotFound {
        static mut instance: ::protobuf::lazy::Lazy<RegionNotFound> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionNotFound,
        };
        unsafe {
            instance.get(|| {
                RegionNotFound {
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

impl ::protobuf::Message for RegionNotFound {
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
        ::std::any::TypeId::of::<RegionNotFound>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RegionNotFound {
    fn new() -> RegionNotFound {
        RegionNotFound::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionNotFound>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "region_id",
                    RegionNotFound::has_region_id,
                    RegionNotFound::get_region_id,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionNotFound>(
                    "RegionNotFound",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionNotFound {
    fn clear(&mut self) {
        self.clear_region_id();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RegionNotFound {
    fn eq(&self, other: &RegionNotFound) -> bool {
        self.region_id == other.region_id &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RegionNotFound {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct KeyNotInRegion {
    // message fields
    key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    region_id: ::std::option::Option<u64>,
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyNotInRegion {}

impl KeyNotInRegion {
    pub fn new() -> KeyNotInRegion {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyNotInRegion {
        static mut instance: ::protobuf::lazy::Lazy<KeyNotInRegion> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyNotInRegion,
        };
        unsafe {
            instance.get(|| {
                KeyNotInRegion {
                    key: ::protobuf::SingularField::none(),
                    region_id: ::std::option::Option::None,
                    start_key: ::protobuf::SingularField::none(),
                    end_key: ::protobuf::SingularField::none(),
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

    // optional uint64 region_id = 2;

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

    // optional bytes start_key = 3;

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
    pub fn mut_start_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.start_key.is_none() {
            self.start_key.set_default();
        };
        self.start_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::std::vec::Vec<u8> {
        self.start_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_start_key(&self) -> &[u8] {
        match self.start_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional bytes end_key = 4;

    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }

    pub fn has_end_key(&self) -> bool {
        self.end_key.is_some()
    }

    // Param is passed by value, moved
    pub fn set_end_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.end_key = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        if self.end_key.is_none() {
            self.end_key.set_default();
        };
        self.end_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_key(&mut self) -> ::std::vec::Vec<u8> {
        self.end_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_end_key(&self) -> &[u8] {
        match self.end_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }
}

impl ::protobuf::Message for KeyNotInRegion {
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
                    self.region_id = ::std::option::Option::Some(tmp);
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start_key));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.end_key));
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
        for value in &self.key {
            my_size += ::protobuf::rt::bytes_size(1, &value);
        };
        for value in &self.region_id {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.start_key {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in &self.end_key {
            my_size += ::protobuf::rt::bytes_size(4, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.key.as_ref() {
            try!(os.write_bytes(1, &v));
        };
        if let Some(v) = self.region_id {
            try!(os.write_uint64(2, v));
        };
        if let Some(v) = self.start_key.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.end_key.as_ref() {
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
        ::std::any::TypeId::of::<KeyNotInRegion>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for KeyNotInRegion {
    fn new() -> KeyNotInRegion {
        KeyNotInRegion::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyNotInRegion>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "key",
                    KeyNotInRegion::has_key,
                    KeyNotInRegion::get_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "region_id",
                    KeyNotInRegion::has_region_id,
                    KeyNotInRegion::get_region_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "start_key",
                    KeyNotInRegion::has_start_key,
                    KeyNotInRegion::get_start_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "end_key",
                    KeyNotInRegion::has_end_key,
                    KeyNotInRegion::get_end_key,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyNotInRegion>(
                    "KeyNotInRegion",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyNotInRegion {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_region_id();
        self.clear_start_key();
        self.clear_end_key();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for KeyNotInRegion {
    fn eq(&self, other: &KeyNotInRegion) -> bool {
        self.key == other.key &&
        self.region_id == other.region_id &&
        self.start_key == other.start_key &&
        self.end_key == other.end_key &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for KeyNotInRegion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct StaleEpoch {
    // message fields
    new_regions: ::protobuf::RepeatedField<super::metapb::Region>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StaleEpoch {}

impl StaleEpoch {
    pub fn new() -> StaleEpoch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StaleEpoch {
        static mut instance: ::protobuf::lazy::Lazy<StaleEpoch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StaleEpoch,
        };
        unsafe {
            instance.get(|| {
                StaleEpoch {
                    new_regions: ::protobuf::RepeatedField::new(),
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // repeated .metapb.Region new_regions = 1;

    pub fn clear_new_regions(&mut self) {
        self.new_regions.clear();
    }

    // Param is passed by value, moved
    pub fn set_new_regions(&mut self, v: ::protobuf::RepeatedField<super::metapb::Region>) {
        self.new_regions = v;
    }

    // Mutable pointer to the field.
    pub fn mut_new_regions(&mut self) -> &mut ::protobuf::RepeatedField<super::metapb::Region> {
        &mut self.new_regions
    }

    // Take field
    pub fn take_new_regions(&mut self) -> ::protobuf::RepeatedField<super::metapb::Region> {
        ::std::mem::replace(&mut self.new_regions, ::protobuf::RepeatedField::new())
    }

    pub fn get_new_regions(&self) -> &[super::metapb::Region] {
        &self.new_regions
    }
}

impl ::protobuf::Message for StaleEpoch {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !try!(is.eof()) {
            let (field_number, wire_type) = try!(is.read_tag_unpack());
            match field_number {
                1 => {
                    try!(::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.new_regions));
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
        for value in &self.new_regions {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.new_regions {
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
        ::std::any::TypeId::of::<StaleEpoch>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for StaleEpoch {
    fn new() -> StaleEpoch {
        StaleEpoch::new()
    }

    fn descriptor_static(_: ::std::option::Option<StaleEpoch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_message_accessor(
                    "new_regions",
                    StaleEpoch::get_new_regions,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StaleEpoch>(
                    "StaleEpoch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StaleEpoch {
    fn clear(&mut self) {
        self.clear_new_regions();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for StaleEpoch {
    fn eq(&self, other: &StaleEpoch) -> bool {
        self.new_regions == other.new_regions &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for StaleEpoch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct ServerIsBusy {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ServerIsBusy {}

impl ServerIsBusy {
    pub fn new() -> ServerIsBusy {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ServerIsBusy {
        static mut instance: ::protobuf::lazy::Lazy<ServerIsBusy> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ServerIsBusy,
        };
        unsafe {
            instance.get(|| {
                ServerIsBusy {
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }
}

impl ::protobuf::Message for ServerIsBusy {
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
        ::std::any::TypeId::of::<ServerIsBusy>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for ServerIsBusy {
    fn new() -> ServerIsBusy {
        ServerIsBusy::new()
    }

    fn descriptor_static(_: ::std::option::Option<ServerIsBusy>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ServerIsBusy>(
                    "ServerIsBusy",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ServerIsBusy {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for ServerIsBusy {
    fn eq(&self, other: &ServerIsBusy) -> bool {
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for ServerIsBusy {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Error {
    // message fields
    message: ::protobuf::SingularField<::std::string::String>,
    not_leader: ::protobuf::SingularPtrField<NotLeader>,
    region_not_found: ::protobuf::SingularPtrField<RegionNotFound>,
    key_not_in_region: ::protobuf::SingularPtrField<KeyNotInRegion>,
    stale_epoch: ::protobuf::SingularPtrField<StaleEpoch>,
    server_is_busy: ::protobuf::SingularPtrField<ServerIsBusy>,
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
                    not_leader: ::protobuf::SingularPtrField::none(),
                    region_not_found: ::protobuf::SingularPtrField::none(),
                    key_not_in_region: ::protobuf::SingularPtrField::none(),
                    stale_epoch: ::protobuf::SingularPtrField::none(),
                    server_is_busy: ::protobuf::SingularPtrField::none(),
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

    // optional .errorpb.NotLeader not_leader = 2;

    pub fn clear_not_leader(&mut self) {
        self.not_leader.clear();
    }

    pub fn has_not_leader(&self) -> bool {
        self.not_leader.is_some()
    }

    // Param is passed by value, moved
    pub fn set_not_leader(&mut self, v: NotLeader) {
        self.not_leader = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_not_leader(&mut self) -> &mut NotLeader {
        if self.not_leader.is_none() {
            self.not_leader.set_default();
        };
        self.not_leader.as_mut().unwrap()
    }

    // Take field
    pub fn take_not_leader(&mut self) -> NotLeader {
        self.not_leader.take().unwrap_or_else(|| NotLeader::new())
    }

    pub fn get_not_leader(&self) -> &NotLeader {
        self.not_leader.as_ref().unwrap_or_else(|| NotLeader::default_instance())
    }

    // optional .errorpb.RegionNotFound region_not_found = 3;

    pub fn clear_region_not_found(&mut self) {
        self.region_not_found.clear();
    }

    pub fn has_region_not_found(&self) -> bool {
        self.region_not_found.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_not_found(&mut self, v: RegionNotFound) {
        self.region_not_found = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_not_found(&mut self) -> &mut RegionNotFound {
        if self.region_not_found.is_none() {
            self.region_not_found.set_default();
        };
        self.region_not_found.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_not_found(&mut self) -> RegionNotFound {
        self.region_not_found.take().unwrap_or_else(|| RegionNotFound::new())
    }

    pub fn get_region_not_found(&self) -> &RegionNotFound {
        self.region_not_found.as_ref().unwrap_or_else(|| RegionNotFound::default_instance())
    }

    // optional .errorpb.KeyNotInRegion key_not_in_region = 4;

    pub fn clear_key_not_in_region(&mut self) {
        self.key_not_in_region.clear();
    }

    pub fn has_key_not_in_region(&self) -> bool {
        self.key_not_in_region.is_some()
    }

    // Param is passed by value, moved
    pub fn set_key_not_in_region(&mut self, v: KeyNotInRegion) {
        self.key_not_in_region = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key_not_in_region(&mut self) -> &mut KeyNotInRegion {
        if self.key_not_in_region.is_none() {
            self.key_not_in_region.set_default();
        };
        self.key_not_in_region.as_mut().unwrap()
    }

    // Take field
    pub fn take_key_not_in_region(&mut self) -> KeyNotInRegion {
        self.key_not_in_region.take().unwrap_or_else(|| KeyNotInRegion::new())
    }

    pub fn get_key_not_in_region(&self) -> &KeyNotInRegion {
        self.key_not_in_region.as_ref().unwrap_or_else(|| KeyNotInRegion::default_instance())
    }

    // optional .errorpb.StaleEpoch stale_epoch = 5;

    pub fn clear_stale_epoch(&mut self) {
        self.stale_epoch.clear();
    }

    pub fn has_stale_epoch(&self) -> bool {
        self.stale_epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_stale_epoch(&mut self, v: StaleEpoch) {
        self.stale_epoch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_stale_epoch(&mut self) -> &mut StaleEpoch {
        if self.stale_epoch.is_none() {
            self.stale_epoch.set_default();
        };
        self.stale_epoch.as_mut().unwrap()
    }

    // Take field
    pub fn take_stale_epoch(&mut self) -> StaleEpoch {
        self.stale_epoch.take().unwrap_or_else(|| StaleEpoch::new())
    }

    pub fn get_stale_epoch(&self) -> &StaleEpoch {
        self.stale_epoch.as_ref().unwrap_or_else(|| StaleEpoch::default_instance())
    }

    // optional .errorpb.ServerIsBusy server_is_busy = 6;

    pub fn clear_server_is_busy(&mut self) {
        self.server_is_busy.clear();
    }

    pub fn has_server_is_busy(&self) -> bool {
        self.server_is_busy.is_some()
    }

    // Param is passed by value, moved
    pub fn set_server_is_busy(&mut self, v: ServerIsBusy) {
        self.server_is_busy = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_server_is_busy(&mut self) -> &mut ServerIsBusy {
        if self.server_is_busy.is_none() {
            self.server_is_busy.set_default();
        };
        self.server_is_busy.as_mut().unwrap()
    }

    // Take field
    pub fn take_server_is_busy(&mut self) -> ServerIsBusy {
        self.server_is_busy.take().unwrap_or_else(|| ServerIsBusy::new())
    }

    pub fn get_server_is_busy(&self) -> &ServerIsBusy {
        self.server_is_busy.as_ref().unwrap_or_else(|| ServerIsBusy::default_instance())
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
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.not_leader));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_not_found));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.key_not_in_region));
                },
                5 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.stale_epoch));
                },
                6 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.server_is_busy));
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
        for value in &self.not_leader {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.region_not_found {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.key_not_in_region {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.stale_epoch {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in &self.server_is_busy {
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
        if let Some(v) = self.not_leader.as_ref() {
            try!(os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.region_not_found.as_ref() {
            try!(os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.key_not_in_region.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.stale_epoch.as_ref() {
            try!(os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        if let Some(v) = self.server_is_busy.as_ref() {
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
                    "not_leader",
                    Error::has_not_leader,
                    Error::get_not_leader,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region_not_found",
                    Error::has_region_not_found,
                    Error::get_region_not_found,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "key_not_in_region",
                    Error::has_key_not_in_region,
                    Error::get_key_not_in_region,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "stale_epoch",
                    Error::has_stale_epoch,
                    Error::get_stale_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "server_is_busy",
                    Error::has_server_is_busy,
                    Error::get_server_is_busy,
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
        self.clear_not_leader();
        self.clear_region_not_found();
        self.clear_key_not_in_region();
        self.clear_stale_epoch();
        self.clear_server_is_busy();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Error {
    fn eq(&self, other: &Error) -> bool {
        self.message == other.message &&
        self.not_leader == other.not_leader &&
        self.region_not_found == other.region_not_found &&
        self.key_not_in_region == other.key_not_in_region &&
        self.stale_epoch == other.stale_epoch &&
        self.server_is_busy == other.server_is_busy &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Error {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0d, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12,
    0x07, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x1a, 0x0c, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x14, 0x67, 0x6f, 0x67, 0x6f, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x2f, 0x67, 0x6f, 0x67, 0x6f, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x3c, 0x0a, 0x09,
    0x4e, 0x6f, 0x74, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x11, 0x0a, 0x09, 0x72, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x12, 0x1c, 0x0a, 0x06,
    0x6c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d,
    0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x22, 0x23, 0x0a, 0x0e, 0x52, 0x65,
    0x67, 0x69, 0x6f, 0x6e, 0x4e, 0x6f, 0x74, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x11, 0x0a, 0x09,
    0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x22,
    0x54, 0x0a, 0x0e, 0x4b, 0x65, 0x79, 0x4e, 0x6f, 0x74, 0x49, 0x6e, 0x52, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x12, 0x0b, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x12, 0x11,
    0x0a, 0x09, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x04, 0x12, 0x11, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x0c, 0x12, 0x0f, 0x0a, 0x07, 0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18,
    0x04, 0x20, 0x01, 0x28, 0x0c, 0x22, 0x31, 0x0a, 0x0a, 0x53, 0x74, 0x61, 0x6c, 0x65, 0x45, 0x70,
    0x6f, 0x63, 0x68, 0x12, 0x23, 0x0a, 0x0b, 0x6e, 0x65, 0x77, 0x5f, 0x72, 0x65, 0x67, 0x69, 0x6f,
    0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70,
    0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x22, 0x0e, 0x0a, 0x0c, 0x53, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x49, 0x73, 0x42, 0x75, 0x73, 0x79, 0x22, 0x80, 0x02, 0x0a, 0x05, 0x45, 0x72, 0x72,
    0x6f, 0x72, 0x12, 0x0f, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20,
    0x01, 0x28, 0x09, 0x12, 0x26, 0x0a, 0x0a, 0x6e, 0x6f, 0x74, 0x5f, 0x6c, 0x65, 0x61, 0x64, 0x65,
    0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x12, 0x2e, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70,
    0x62, 0x2e, 0x4e, 0x6f, 0x74, 0x4c, 0x65, 0x61, 0x64, 0x65, 0x72, 0x12, 0x31, 0x0a, 0x10, 0x72,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x6e, 0x6f, 0x74, 0x5f, 0x66, 0x6f, 0x75, 0x6e, 0x64, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e,
    0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4e, 0x6f, 0x74, 0x46, 0x6f, 0x75, 0x6e, 0x64, 0x12, 0x32,
    0x0a, 0x11, 0x6b, 0x65, 0x79, 0x5f, 0x6e, 0x6f, 0x74, 0x5f, 0x69, 0x6e, 0x5f, 0x72, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x65, 0x72, 0x72, 0x6f,
    0x72, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x4e, 0x6f, 0x74, 0x49, 0x6e, 0x52, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x12, 0x28, 0x0a, 0x0b, 0x73, 0x74, 0x61, 0x6c, 0x65, 0x5f, 0x65, 0x70, 0x6f, 0x63,
    0x68, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70,
    0x62, 0x2e, 0x53, 0x74, 0x61, 0x6c, 0x65, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x2d, 0x0a, 0x0e,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x5f, 0x69, 0x73, 0x5f, 0x62, 0x75, 0x73, 0x79, 0x18, 0x06,
    0x20, 0x01, 0x28, 0x0b, 0x32, 0x15, 0x2e, 0x65, 0x72, 0x72, 0x6f, 0x72, 0x70, 0x62, 0x2e, 0x53,
    0x65, 0x72, 0x76, 0x65, 0x72, 0x49, 0x73, 0x42, 0x75, 0x73, 0x79, 0x42, 0x0c, 0xd0, 0xe2, 0x1e,
    0x01, 0xc8, 0xe2, 0x1e, 0x01, 0xe0, 0xe2, 0x1e, 0x01, 0x4a, 0xf4, 0x0a, 0x0a, 0x06, 0x12, 0x04,
    0x00, 0x00, 0x28, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0f, 0x0a, 0x09,
    0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x03, 0x07, 0x15, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12,
    0x03, 0x04, 0x07, 0x1d, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x28, 0x0a, 0x0b,
    0x0a, 0x04, 0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x06, 0x00, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x08,
    0xe7, 0x07, 0x00, 0x02, 0x12, 0x03, 0x06, 0x07, 0x20, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x00, 0x12, 0x03, 0x06, 0x07, 0x20, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00,
    0x03, 0x12, 0x03, 0x06, 0x23, 0x27, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x07, 0x00, 0x24,
    0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x01, 0x12, 0x03, 0x07, 0x00, 0x24, 0x0a, 0x0c, 0x0a,
    0x05, 0x08, 0xe7, 0x07, 0x01, 0x02, 0x12, 0x03, 0x07, 0x07, 0x1c, 0x0a, 0x0d, 0x0a, 0x06, 0x08,
    0xe7, 0x07, 0x01, 0x02, 0x00, 0x12, 0x03, 0x07, 0x07, 0x1c, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7,
    0x07, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x08, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7,
    0x07, 0x01, 0x03, 0x12, 0x03, 0x07, 0x1f, 0x23, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x08,
    0x00, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x08, 0xe7, 0x07, 0x02, 0x12, 0x03, 0x08, 0x00, 0x2a, 0x0a,
    0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x12, 0x03, 0x08, 0x07, 0x22, 0x0a, 0x0d, 0x0a,
    0x06, 0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x12, 0x03, 0x08, 0x07, 0x22, 0x0a, 0x0e, 0x0a, 0x07,
    0x08, 0xe7, 0x07, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x21, 0x0a, 0x0c, 0x0a, 0x05,
    0x08, 0xe7, 0x07, 0x02, 0x03, 0x12, 0x03, 0x08, 0x25, 0x29, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x0a, 0x00, 0x0d, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x0a,
    0x08, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x04, 0x27, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x0b, 0x25, 0x26, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x0c, 0x04, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x0c,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x0d, 0x18,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x19, 0x1f, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0c, 0x25, 0x26, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x01, 0x12, 0x04, 0x0f, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12,
    0x03, 0x0f, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x10, 0x04,
    0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x10, 0x04, 0x0c, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x10, 0x0d, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x10, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x10, 0x20, 0x21, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12,
    0x04, 0x13, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x13, 0x08,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00, 0x12, 0x03, 0x14, 0x04, 0x23, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x13, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x14, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x15, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x15, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x15, 0x0d, 0x13, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x15, 0x14, 0x1d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x03, 0x12, 0x03, 0x15, 0x21, 0x22, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x02, 0x02, 0x02, 0x12, 0x03, 0x16, 0x04, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12,
    0x03, 0x16, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x16,
    0x13, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x16, 0x21, 0x22,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x04, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x03, 0x04, 0x12, 0x03, 0x17, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x03, 0x05, 0x12, 0x03, 0x17, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x17, 0x13, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x17, 0x21, 0x22, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x1a, 0x00, 0x1c,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x12, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1b, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x1b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x06, 0x12, 0x03, 0x1b, 0x0d, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x1b, 0x1b, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b,
    0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04, 0x1e, 0x00, 0x1f, 0x01, 0x0a, 0x0a,
    0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x1e, 0x08, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x05,
    0x12, 0x04, 0x21, 0x00, 0x28, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03, 0x21,
    0x08, 0x0d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x00, 0x12, 0x03, 0x22, 0x04, 0x34, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x00, 0x04, 0x12, 0x03, 0x22, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x00, 0x05, 0x12, 0x03, 0x22, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x05, 0x02, 0x00, 0x01, 0x12, 0x03, 0x22, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x00, 0x03, 0x12, 0x03, 0x22, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x01, 0x12,
    0x03, 0x23, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x04, 0x12, 0x03, 0x23,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x06, 0x12, 0x03, 0x23, 0x0d, 0x16,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x01, 0x12, 0x03, 0x23, 0x17, 0x21, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x01, 0x03, 0x12, 0x03, 0x23, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x05, 0x02, 0x02, 0x12, 0x03, 0x24, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02,
    0x02, 0x04, 0x12, 0x03, 0x24, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x06,
    0x12, 0x03, 0x24, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x01, 0x12, 0x03,
    0x24, 0x1c, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x02, 0x03, 0x12, 0x03, 0x24, 0x32,
    0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x03, 0x12, 0x03, 0x25, 0x04, 0x34, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x05, 0x02, 0x03, 0x04, 0x12, 0x03, 0x25, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x05, 0x02, 0x03, 0x06, 0x12, 0x03, 0x25, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x25, 0x1c, 0x2d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x25, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x05, 0x02, 0x04, 0x12, 0x03,
    0x26, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x04, 0x12, 0x03, 0x26, 0x04,
    0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x06, 0x12, 0x03, 0x26, 0x0d, 0x17, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x04, 0x01, 0x12, 0x03, 0x26, 0x18, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x05, 0x02, 0x04, 0x03, 0x12, 0x03, 0x26, 0x32, 0x33, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x05, 0x02, 0x05, 0x12, 0x03, 0x27, 0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05,
    0x04, 0x12, 0x03, 0x27, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x06, 0x12,
    0x03, 0x27, 0x0d, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x01, 0x12, 0x03, 0x27,
    0x1a, 0x28, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x05, 0x02, 0x05, 0x03, 0x12, 0x03, 0x27, 0x32, 0x33,
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
