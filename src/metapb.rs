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
pub struct Cluster {
    // message fields
    name: ::protobuf::SingularField<::std::string::String>,
    max_peer_number: ::std::option::Option<u32>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Cluster {}

impl Cluster {
    pub fn new() -> Cluster {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Cluster {
        static mut instance: ::protobuf::lazy::Lazy<Cluster> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Cluster,
        };
        unsafe {
            instance.get(|| {
                Cluster {
                    name: ::protobuf::SingularField::none(),
                    max_peer_number: ::std::option::Option::None,
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
    pub fn mut_name<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.name.is_none() {
            self.name.set_default();
        };
        self.name.as_mut().unwrap()
    }

    // Take field
    pub fn take_name(&mut self) -> ::std::string::String {
        self.name.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        match self.name.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }

    // optional uint32 max_peer_number = 2;

    pub fn clear_max_peer_number(&mut self) {
        self.max_peer_number = ::std::option::Option::None;
    }

    pub fn has_max_peer_number(&self) -> bool {
        self.max_peer_number.is_some()
    }

    // Param is passed by value, moved
    pub fn set_max_peer_number(&mut self, v: u32) {
        self.max_peer_number = ::std::option::Option::Some(v);
    }

    pub fn get_max_peer_number<'a>(&self) -> u32 {
        self.max_peer_number.unwrap_or(0)
    }
}

impl ::protobuf::Message for Cluster {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = try!(is.read_uint32());
                    self.max_peer_number = ::std::option::Option::Some(tmp);
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
        for value in self.name.iter() {
            my_size += ::protobuf::rt::string_size(1, &value);
        };
        for value in self.max_peer_number.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.name.as_ref() {
            try!(os.write_string(1, &v));
        };
        if let Some(v) = self.max_peer_number {
            try!(os.write_uint32(2, v));
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
        ::std::any::TypeId::of::<Cluster>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Cluster {
    fn new() -> Cluster {
        Cluster::new()
    }

    fn descriptor_static(_: ::std::option::Option<Cluster>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "name",
                    Cluster::has_name,
                    Cluster::get_name,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u32_accessor(
                    "max_peer_number",
                    Cluster::has_max_peer_number,
                    Cluster::get_max_peer_number,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Cluster>(
                    "Cluster",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Cluster {
    fn clear(&mut self) {
        self.clear_name();
        self.clear_max_peer_number();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Cluster {
    fn eq(&self, other: &Cluster) -> bool {
        self.name == other.name &&
        self.max_peer_number == other.max_peer_number &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Cluster {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Store {
    // message fields
    id: ::std::option::Option<u64>,
    address: ::protobuf::SingularField<::std::string::String>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Store {}

impl Store {
    pub fn new() -> Store {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Store {
        static mut instance: ::protobuf::lazy::Lazy<Store> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Store,
        };
        unsafe {
            instance.get(|| {
                Store {
                    id: ::std::option::Option::None,
                    address: ::protobuf::SingularField::none(),
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

    // optional string address = 2;

    pub fn clear_address(&mut self) {
        self.address.clear();
    }

    pub fn has_address(&self) -> bool {
        self.address.is_some()
    }

    // Param is passed by value, moved
    pub fn set_address(&mut self, v: ::std::string::String) {
        self.address = ::protobuf::SingularField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_address<'a>(&'a mut self) -> &'a mut ::std::string::String {
        if self.address.is_none() {
            self.address.set_default();
        };
        self.address.as_mut().unwrap()
    }

    // Take field
    pub fn take_address(&mut self) -> ::std::string::String {
        self.address.take().unwrap_or_else(|| ::std::string::String::new())
    }

    pub fn get_address<'a>(&'a self) -> &'a str {
        match self.address.as_ref() {
            Some(v) => &v,
            None => "",
        }
    }
}

impl ::protobuf::Message for Store {
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
                2 => {
                    try!(::protobuf::rt::read_singular_string_into(wire_type, is, &mut self.address));
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
        for value in self.address.iter() {
            my_size += ::protobuf::rt::string_size(2, &value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.address.as_ref() {
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
        ::std::any::TypeId::of::<Store>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Store {
    fn new() -> Store {
        Store::new()
    }

    fn descriptor_static(_: ::std::option::Option<Store>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "id",
                    Store::has_id,
                    Store::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_string_accessor(
                    "address",
                    Store::has_address,
                    Store::get_address,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Store>(
                    "Store",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Store {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_address();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Store {
    fn eq(&self, other: &Store) -> bool {
        self.id == other.id &&
        self.address == other.address &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Store {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct RegionEpoch {
    // message fields
    conf_ver: ::std::option::Option<u64>,
    version: ::std::option::Option<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionEpoch {}

impl RegionEpoch {
    pub fn new() -> RegionEpoch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionEpoch {
        static mut instance: ::protobuf::lazy::Lazy<RegionEpoch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionEpoch,
        };
        unsafe {
            instance.get(|| {
                RegionEpoch {
                    conf_ver: ::std::option::Option::None,
                    version: ::std::option::Option::None,
                    unknown_fields: ::protobuf::UnknownFields::new(),
                    cached_size: ::std::cell::Cell::new(0),
                }
            })
        }
    }

    // optional uint64 conf_ver = 1;

    pub fn clear_conf_ver(&mut self) {
        self.conf_ver = ::std::option::Option::None;
    }

    pub fn has_conf_ver(&self) -> bool {
        self.conf_ver.is_some()
    }

    // Param is passed by value, moved
    pub fn set_conf_ver(&mut self, v: u64) {
        self.conf_ver = ::std::option::Option::Some(v);
    }

    pub fn get_conf_ver<'a>(&self) -> u64 {
        self.conf_ver.unwrap_or(0)
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

impl ::protobuf::Message for RegionEpoch {
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
                    self.conf_ver = ::std::option::Option::Some(tmp);
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
        for value in self.conf_ver.iter() {
            my_size += ::protobuf::rt::value_size(1, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in self.version.iter() {
            my_size += ::protobuf::rt::value_size(2, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.conf_ver {
            try!(os.write_uint64(1, v));
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
        ::std::any::TypeId::of::<RegionEpoch>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for RegionEpoch {
    fn new() -> RegionEpoch {
        RegionEpoch::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionEpoch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "conf_ver",
                    RegionEpoch::has_conf_ver,
                    RegionEpoch::get_conf_ver,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "version",
                    RegionEpoch::has_version,
                    RegionEpoch::get_version,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionEpoch>(
                    "RegionEpoch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionEpoch {
    fn clear(&mut self) {
        self.clear_conf_ver();
        self.clear_version();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for RegionEpoch {
    fn eq(&self, other: &RegionEpoch) -> bool {
        self.conf_ver == other.conf_ver &&
        self.version == other.version &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for RegionEpoch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

#[derive(Clone,Default)]
pub struct Region {
    // message fields
    id: ::std::option::Option<u64>,
    start_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    end_key: ::protobuf::SingularField<::std::vec::Vec<u8>>,
    region_epoch: ::protobuf::SingularPtrField<RegionEpoch>,
    store_ids: ::std::vec::Vec<u64>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::std::cell::Cell<u32>,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Region {}

impl Region {
    pub fn new() -> Region {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Region {
        static mut instance: ::protobuf::lazy::Lazy<Region> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Region,
        };
        unsafe {
            instance.get(|| {
                Region {
                    id: ::std::option::Option::None,
                    start_key: ::protobuf::SingularField::none(),
                    end_key: ::protobuf::SingularField::none(),
                    region_epoch: ::protobuf::SingularPtrField::none(),
                    store_ids: ::std::vec::Vec::new(),
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

    // optional bytes start_key = 2;

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

    // optional bytes end_key = 3;

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
    pub fn mut_end_key<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u8> {
        if self.end_key.is_none() {
            self.end_key.set_default();
        };
        self.end_key.as_mut().unwrap()
    }

    // Take field
    pub fn take_end_key(&mut self) -> ::std::vec::Vec<u8> {
        self.end_key.take().unwrap_or_else(|| ::std::vec::Vec::new())
    }

    pub fn get_end_key<'a>(&'a self) -> &'a [u8] {
        match self.end_key.as_ref() {
            Some(v) => &v,
            None => &[],
        }
    }

    // optional .metapb.RegionEpoch region_epoch = 4;

    pub fn clear_region_epoch(&mut self) {
        self.region_epoch.clear();
    }

    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_region_epoch(&mut self, v: RegionEpoch) {
        self.region_epoch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_region_epoch<'a>(&'a mut self) -> &'a mut RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch.set_default();
        };
        self.region_epoch.as_mut().unwrap()
    }

    // Take field
    pub fn take_region_epoch(&mut self) -> RegionEpoch {
        self.region_epoch.take().unwrap_or_else(|| RegionEpoch::new())
    }

    pub fn get_region_epoch<'a>(&'a self) -> &'a RegionEpoch {
        self.region_epoch.as_ref().unwrap_or_else(|| RegionEpoch::default_instance())
    }

    // repeated uint64 store_ids = 5;

    pub fn clear_store_ids(&mut self) {
        self.store_ids.clear();
    }

    // Param is passed by value, moved
    pub fn set_store_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.store_ids = v;
    }

    // Mutable pointer to the field.
    pub fn mut_store_ids<'a>(&'a mut self) -> &'a mut ::std::vec::Vec<u64> {
        &mut self.store_ids
    }

    // Take field
    pub fn take_store_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.store_ids, ::std::vec::Vec::new())
    }

    pub fn get_store_ids<'a>(&'a self) -> &'a [u64] {
        &self.store_ids
    }
}

impl ::protobuf::Message for Region {
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
                2 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.start_key));
                },
                3 => {
                    try!(::protobuf::rt::read_singular_bytes_into(wire_type, is, &mut self.end_key));
                },
                4 => {
                    try!(::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_epoch));
                },
                5 => {
                    try!(::protobuf::rt::read_repeated_uint64_into(wire_type, is, &mut self.store_ids));
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
        for value in self.start_key.iter() {
            my_size += ::protobuf::rt::bytes_size(2, &value);
        };
        for value in self.end_key.iter() {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        for value in self.region_epoch.iter() {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        for value in self.store_ids.iter() {
            my_size += ::protobuf::rt::value_size(5, *value, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.id {
            try!(os.write_uint64(1, v));
        };
        if let Some(v) = self.start_key.as_ref() {
            try!(os.write_bytes(2, &v));
        };
        if let Some(v) = self.end_key.as_ref() {
            try!(os.write_bytes(3, &v));
        };
        if let Some(v) = self.region_epoch.as_ref() {
            try!(os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited));
            try!(os.write_raw_varint32(v.get_cached_size()));
            try!(v.write_to_with_cached_sizes(os));
        };
        for v in self.store_ids.iter() {
            try!(os.write_uint64(5, *v));
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
        ::std::any::TypeId::of::<Region>()
    }

    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        ::protobuf::MessageStatic::descriptor_static(None::<Self>)
    }
}

impl ::protobuf::MessageStatic for Region {
    fn new() -> Region {
        Region::new()
    }

    fn descriptor_static(_: ::std::option::Option<Region>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_u64_accessor(
                    "id",
                    Region::has_id,
                    Region::get_id,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "start_key",
                    Region::has_start_key,
                    Region::get_start_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_bytes_accessor(
                    "end_key",
                    Region::has_end_key,
                    Region::get_end_key,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_message_accessor(
                    "region_epoch",
                    Region::has_region_epoch,
                    Region::get_region_epoch,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_u64_accessor(
                    "store_ids",
                    Region::get_store_ids,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Region>(
                    "Region",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Region {
    fn clear(&mut self) {
        self.clear_id();
        self.clear_start_key();
        self.clear_end_key();
        self.clear_region_epoch();
        self.clear_store_ids();
        self.unknown_fields.clear();
    }
}

impl ::std::cmp::PartialEq for Region {
    fn eq(&self, other: &Region) -> bool {
        self.id == other.id &&
        self.start_key == other.start_key &&
        self.end_key == other.end_key &&
        self.region_epoch == other.region_epoch &&
        self.store_ids == other.store_ids &&
        self.unknown_fields == other.unknown_fields
    }
}

impl ::std::fmt::Debug for Region {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x0c, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x06,
    0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x22, 0x45, 0x0a, 0x07, 0x43, 0x6c, 0x75, 0x73, 0x74, 0x65,
    0x72, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x26, 0x0a, 0x0f, 0x6d, 0x61, 0x78, 0x5f, 0x70, 0x65, 0x65,
    0x72, 0x5f, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x0d,
    0x6d, 0x61, 0x78, 0x50, 0x65, 0x65, 0x72, 0x4e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x22, 0x31, 0x0a,
    0x05, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x12, 0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73,
    0x73, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x64, 0x64, 0x72, 0x65, 0x73, 0x73,
    0x22, 0x42, 0x0a, 0x0b, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x12,
    0x19, 0x0a, 0x08, 0x63, 0x6f, 0x6e, 0x66, 0x5f, 0x76, 0x65, 0x72, 0x18, 0x01, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x07, 0x63, 0x6f, 0x6e, 0x66, 0x56, 0x65, 0x72, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65,
    0x72, 0x73, 0x69, 0x6f, 0x6e, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x22, 0xa3, 0x01, 0x0a, 0x06, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12,
    0x0e, 0x0a, 0x02, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x02, 0x69, 0x64, 0x12,
    0x1b, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x02, 0x20, 0x01,
    0x28, 0x0c, 0x52, 0x08, 0x73, 0x74, 0x61, 0x72, 0x74, 0x4b, 0x65, 0x79, 0x12, 0x17, 0x0a, 0x07,
    0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x65,
    0x6e, 0x64, 0x4b, 0x65, 0x79, 0x12, 0x36, 0x0a, 0x0c, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f,
    0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d, 0x65,
    0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x45, 0x70, 0x6f, 0x63, 0x68,
    0x52, 0x0b, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x12, 0x1b, 0x0a,
    0x09, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69, 0x64, 0x73, 0x18, 0x05, 0x20, 0x03, 0x28, 0x04,
    0x52, 0x08, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x64, 0x73, 0x4a, 0xa5, 0x09, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x20, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x0e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x03, 0x00, 0x09, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x03, 0x08,
    0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x04, 0x04, 0x28, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03, 0x04, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x04, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x04, 0x14, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x04, 0x26, 0x27, 0x0a, 0x84, 0x01, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12,
    0x03, 0x07, 0x04, 0x28, 0x1a, 0x5e, 0x20, 0x6d, 0x61, 0x78, 0x20, 0x70, 0x65, 0x65, 0x72, 0x20,
    0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x61, 0x20, 0x72, 0x65, 0x67,
    0x69, 0x6f, 0x6e, 0x2e, 0x0a, 0x20, 0x70, 0x64, 0x20, 0x77, 0x69, 0x6c, 0x6c, 0x20, 0x64, 0x6f,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x61, 0x75, 0x74, 0x6f, 0x2d, 0x62, 0x61, 0x6c, 0x61, 0x6e, 0x63,
    0x65, 0x20, 0x69, 0x66, 0x20, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x70, 0x65, 0x65, 0x72,
    0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6d, 0x69, 0x73, 0x6d, 0x61, 0x74, 0x63, 0x68,
    0x65, 0x73, 0x2e, 0x0a, 0x22, 0x17, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x61, 0x74, 0x74, 0x72,
    0x69, 0x62, 0x75, 0x74, 0x65, 0x73, 0x2e, 0x2e, 0x2e, 0x2e, 0x2e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x03, 0x07, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x07, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x07, 0x14, 0x23, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x07, 0x26, 0x27, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0c, 0x00, 0x10,
    0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x08, 0x0d, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0d, 0x04, 0x20, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x00, 0x04, 0x12, 0x03, 0x0d, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x0d, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x0d, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0d,
    0x1e, 0x1f, 0x0a, 0x24, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0e, 0x04, 0x20, 0x22,
    0x17, 0x20, 0x6d, 0x6f, 0x72, 0x65, 0x20, 0x61, 0x74, 0x74, 0x72, 0x69, 0x62, 0x75, 0x74, 0x65,
    0x73, 0x2e, 0x2e, 0x2e, 0x2e, 0x2e, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x03, 0x0e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x0e, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0e,
    0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x0e, 0x1e, 0x1f,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x12, 0x00, 0x17, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x02, 0x01, 0x12, 0x03, 0x12, 0x08, 0x13, 0x0a, 0x4a, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x00,
    0x12, 0x03, 0x14, 0x04, 0x24, 0x1a, 0x3d, 0x20, 0x43, 0x6f, 0x6e, 0x66, 0x20, 0x63, 0x68, 0x61,
    0x6e, 0x67, 0x65, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x61, 0x75, 0x74,
    0x6f, 0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e,
    0x20, 0x61, 0x64, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x20, 0x70,
    0x65, 0x65, 0x72, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12, 0x03, 0x14,
    0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x0d, 0x13,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x14, 0x1c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x22, 0x23, 0x0a, 0x41, 0x0a, 0x04,
    0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x16, 0x04, 0x24, 0x1a, 0x34, 0x20, 0x52, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x2c, 0x20, 0x61, 0x75, 0x74, 0x6f,
    0x20, 0x69, 0x6e, 0x63, 0x72, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x77, 0x68, 0x65, 0x6e, 0x20,
    0x73, 0x70, 0x6c, 0x69, 0x74, 0x20, 0x6f, 0x72, 0x20, 0x6d, 0x65, 0x72, 0x67, 0x65, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x03, 0x16, 0x04, 0x0c, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x16, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x16, 0x22, 0x23, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x03, 0x12, 0x04, 0x19,
    0x00, 0x20, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03, 0x19, 0x08, 0x0e, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x1a, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x05, 0x12, 0x03, 0x1a, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x1a, 0x14, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x03, 0x12,
    0x03, 0x1a, 0x2a, 0x2b, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x04,
    0x2c, 0x1a, 0x28, 0x20, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x6b, 0x65, 0x79, 0x20, 0x72,
    0x61, 0x6e, 0x67, 0x65, 0x20, 0x5b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b, 0x65, 0x79, 0x2c,
    0x20, 0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x29, 0x2e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x03, 0x02, 0x01, 0x04, 0x12, 0x03, 0x1c, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02,
    0x01, 0x05, 0x12, 0x03, 0x1c, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x1c, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x1c, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x02, 0x12, 0x03, 0x1d, 0x04, 0x2c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x04, 0x12, 0x03, 0x1d, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x0d, 0x12, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x03,
    0x12, 0x03, 0x1e, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x1e, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x06, 0x12, 0x03, 0x1e, 0x0d,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1e, 0x19, 0x25, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1e, 0x2a, 0x2b, 0x0a, 0x0b, 0x0a,
    0x04, 0x04, 0x03, 0x02, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x2c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x04, 0x04, 0x12, 0x03, 0x1f, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04,
    0x05, 0x12, 0x03, 0x1f, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x01, 0x12,
    0x03, 0x1f, 0x14, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x04, 0x03, 0x12, 0x03, 0x1f,
    0x2a, 0x2b,
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
