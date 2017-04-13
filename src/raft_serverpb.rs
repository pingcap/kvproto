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
pub struct RaftMessage {
    // message fields
    pub region_id: u64,
    from_peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    to_peer: ::protobuf::SingularPtrField<super::metapb::Peer>,
    message: ::protobuf::SingularPtrField<super::eraftpb::Message>,
    region_epoch: ::protobuf::SingularPtrField<super::metapb::RegionEpoch>,
    pub is_tombstone: bool,
    pub start_key: ::std::vec::Vec<u8>,
    pub end_key: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftMessage {}

impl RaftMessage {
    pub fn new() -> RaftMessage {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftMessage {
        static mut instance: ::protobuf::lazy::Lazy<RaftMessage> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftMessage,
        };
        unsafe {
            instance.get(RaftMessage::new)
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

    // .metapb.Peer from_peer = 2;

    pub fn clear_from_peer(&mut self) {
        self.from_peer.clear();
    }

    pub fn has_from_peer(&self) -> bool {
        self.from_peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_from_peer(&mut self, v: super::metapb::Peer) {
        self.from_peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_from_peer(&mut self) -> &mut super::metapb::Peer {
        if self.from_peer.is_none() {
            self.from_peer.set_default();
        };
        self.from_peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_from_peer(&mut self) -> super::metapb::Peer {
        self.from_peer.take().unwrap_or_else(|| super::metapb::Peer::new())
    }

    pub fn get_from_peer(&self) -> &super::metapb::Peer {
        self.from_peer.as_ref().unwrap_or_else(|| super::metapb::Peer::default_instance())
    }

    fn get_from_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Peer> {
        &self.from_peer
    }

    fn mut_from_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Peer> {
        &mut self.from_peer
    }

    // .metapb.Peer to_peer = 3;

    pub fn clear_to_peer(&mut self) {
        self.to_peer.clear();
    }

    pub fn has_to_peer(&self) -> bool {
        self.to_peer.is_some()
    }

    // Param is passed by value, moved
    pub fn set_to_peer(&mut self, v: super::metapb::Peer) {
        self.to_peer = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_to_peer(&mut self) -> &mut super::metapb::Peer {
        if self.to_peer.is_none() {
            self.to_peer.set_default();
        };
        self.to_peer.as_mut().unwrap()
    }

    // Take field
    pub fn take_to_peer(&mut self) -> super::metapb::Peer {
        self.to_peer.take().unwrap_or_else(|| super::metapb::Peer::new())
    }

    pub fn get_to_peer(&self) -> &super::metapb::Peer {
        self.to_peer.as_ref().unwrap_or_else(|| super::metapb::Peer::default_instance())
    }

    fn get_to_peer_for_reflect(&self) -> &::protobuf::SingularPtrField<super::metapb::Peer> {
        &self.to_peer
    }

    fn mut_to_peer_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::metapb::Peer> {
        &mut self.to_peer
    }

    // .eraftpb.Message message = 4;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: super::eraftpb::Message) {
        self.message = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut super::eraftpb::Message {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> super::eraftpb::Message {
        self.message.take().unwrap_or_else(|| super::eraftpb::Message::new())
    }

    pub fn get_message(&self) -> &super::eraftpb::Message {
        self.message.as_ref().unwrap_or_else(|| super::eraftpb::Message::default_instance())
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularPtrField<super::eraftpb::Message> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::eraftpb::Message> {
        &mut self.message
    }

    // .metapb.RegionEpoch region_epoch = 5;

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

    // bool is_tombstone = 6;

    pub fn clear_is_tombstone(&mut self) {
        self.is_tombstone = false;
    }

    // Param is passed by value, moved
    pub fn set_is_tombstone(&mut self, v: bool) {
        self.is_tombstone = v;
    }

    pub fn get_is_tombstone(&self) -> bool {
        self.is_tombstone
    }

    fn get_is_tombstone_for_reflect(&self) -> &bool {
        &self.is_tombstone
    }

    fn mut_is_tombstone_for_reflect(&mut self) -> &mut bool {
        &mut self.is_tombstone
    }

    // bytes start_key = 7;

    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_start_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.start_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.start_key
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }

    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }

    fn get_start_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.start_key
    }

    fn mut_start_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.start_key
    }

    // bytes end_key = 8;

    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_end_key(&mut self, v: ::std::vec::Vec<u8>) {
        self.end_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_key(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.end_key
    }

    // Take field
    pub fn take_end_key(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }

    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }

    fn get_end_key_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.end_key
    }

    fn mut_end_key_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.end_key
    }
}

impl ::protobuf::Message for RaftMessage {
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
                    self.region_id = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.from_peer)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.to_peer)?;
                },
                4 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.message)?;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_epoch)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_bool()?;
                    self.is_tombstone = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.start_key)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.end_key)?;
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
        };
        if let Some(v) = self.from_peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.to_peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.message.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if let Some(v) = self.region_epoch.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.is_tombstone != false {
            my_size += 2;
        };
        if !self.start_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.start_key);
        };
        if !self.end_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(8, &self.end_key);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.region_id != 0 {
            os.write_uint64(1, self.region_id)?;
        };
        if let Some(v) = self.from_peer.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.to_peer.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.message.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if let Some(v) = self.region_epoch.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.is_tombstone != false {
            os.write_bool(6, self.is_tombstone)?;
        };
        if !self.start_key.is_empty() {
            os.write_bytes(7, &self.start_key)?;
        };
        if !self.end_key.is_empty() {
            os.write_bytes(8, &self.end_key)?;
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

impl ::protobuf::MessageStatic for RaftMessage {
    fn new() -> RaftMessage {
        RaftMessage::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftMessage>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "region_id",
                    RaftMessage::get_region_id_for_reflect,
                    RaftMessage::mut_region_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Peer>>(
                    "from_peer",
                    RaftMessage::get_from_peer_for_reflect,
                    RaftMessage::mut_from_peer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Peer>>(
                    "to_peer",
                    RaftMessage::get_to_peer_for_reflect,
                    RaftMessage::mut_to_peer_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::eraftpb::Message>>(
                    "message",
                    RaftMessage::get_message_for_reflect,
                    RaftMessage::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::RegionEpoch>>(
                    "region_epoch",
                    RaftMessage::get_region_epoch_for_reflect,
                    RaftMessage::mut_region_epoch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "is_tombstone",
                    RaftMessage::get_is_tombstone_for_reflect,
                    RaftMessage::mut_is_tombstone_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "start_key",
                    RaftMessage::get_start_key_for_reflect,
                    RaftMessage::mut_start_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "end_key",
                    RaftMessage::get_end_key_for_reflect,
                    RaftMessage::mut_end_key_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftMessage>(
                    "RaftMessage",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftMessage {
    fn clear(&mut self) {
        self.clear_region_id();
        self.clear_from_peer();
        self.clear_to_peer();
        self.clear_message();
        self.clear_region_epoch();
        self.clear_is_tombstone();
        self.clear_start_key();
        self.clear_end_key();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftMessage {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftMessage {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RaftTruncatedState {
    // message fields
    pub index: u64,
    pub term: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftTruncatedState {}

impl RaftTruncatedState {
    pub fn new() -> RaftTruncatedState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftTruncatedState {
        static mut instance: ::protobuf::lazy::Lazy<RaftTruncatedState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftTruncatedState,
        };
        unsafe {
            instance.get(RaftTruncatedState::new)
        }
    }

    // uint64 index = 1;

    pub fn clear_index(&mut self) {
        self.index = 0;
    }

    // Param is passed by value, moved
    pub fn set_index(&mut self, v: u64) {
        self.index = v;
    }

    pub fn get_index(&self) -> u64 {
        self.index
    }

    fn get_index_for_reflect(&self) -> &u64 {
        &self.index
    }

    fn mut_index_for_reflect(&mut self) -> &mut u64 {
        &mut self.index
    }

    // uint64 term = 2;

    pub fn clear_term(&mut self) {
        self.term = 0;
    }

    // Param is passed by value, moved
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }

    pub fn get_term(&self) -> u64 {
        self.term
    }

    fn get_term_for_reflect(&self) -> &u64 {
        &self.term
    }

    fn mut_term_for_reflect(&mut self) -> &mut u64 {
        &mut self.term
    }
}

impl ::protobuf::Message for RaftTruncatedState {
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
                    self.index = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.term = tmp;
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
        if self.index != 0 {
            my_size += ::protobuf::rt::value_size(1, self.index, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(2, self.term, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.index != 0 {
            os.write_uint64(1, self.index)?;
        };
        if self.term != 0 {
            os.write_uint64(2, self.term)?;
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

impl ::protobuf::MessageStatic for RaftTruncatedState {
    fn new() -> RaftTruncatedState {
        RaftTruncatedState::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftTruncatedState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "index",
                    RaftTruncatedState::get_index_for_reflect,
                    RaftTruncatedState::mut_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "term",
                    RaftTruncatedState::get_term_for_reflect,
                    RaftTruncatedState::mut_term_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftTruncatedState>(
                    "RaftTruncatedState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftTruncatedState {
    fn clear(&mut self) {
        self.clear_index();
        self.clear_term();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftTruncatedState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftTruncatedState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotCFFile {
    // message fields
    pub cf: ::std::string::String,
    pub size: u64,
    pub checksum: u32,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotCFFile {}

impl SnapshotCFFile {
    pub fn new() -> SnapshotCFFile {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotCFFile {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotCFFile> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotCFFile,
        };
        unsafe {
            instance.get(SnapshotCFFile::new)
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

    // uint32 checksum = 3;

    pub fn clear_checksum(&mut self) {
        self.checksum = 0;
    }

    // Param is passed by value, moved
    pub fn set_checksum(&mut self, v: u32) {
        self.checksum = v;
    }

    pub fn get_checksum(&self) -> u32 {
        self.checksum
    }

    fn get_checksum_for_reflect(&self) -> &u32 {
        &self.checksum
    }

    fn mut_checksum_for_reflect(&mut self) -> &mut u32 {
        &mut self.checksum
    }
}

impl ::protobuf::Message for SnapshotCFFile {
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
                    };
                    let tmp = is.read_uint64()?;
                    self.size = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint32()?;
                    self.checksum = tmp;
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
        };
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.size, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.checksum != 0 {
            my_size += ::protobuf::rt::value_size(3, self.checksum, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.cf.is_empty() {
            os.write_string(1, &self.cf)?;
        };
        if self.size != 0 {
            os.write_uint64(2, self.size)?;
        };
        if self.checksum != 0 {
            os.write_uint32(3, self.checksum)?;
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

impl ::protobuf::MessageStatic for SnapshotCFFile {
    fn new() -> SnapshotCFFile {
        SnapshotCFFile::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotCFFile>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cf",
                    SnapshotCFFile::get_cf_for_reflect,
                    SnapshotCFFile::mut_cf_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "size",
                    SnapshotCFFile::get_size_for_reflect,
                    SnapshotCFFile::mut_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "checksum",
                    SnapshotCFFile::get_checksum_for_reflect,
                    SnapshotCFFile::mut_checksum_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotCFFile>(
                    "SnapshotCFFile",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotCFFile {
    fn clear(&mut self) {
        self.clear_cf();
        self.clear_size();
        self.clear_checksum();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotCFFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotCFFile {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotMeta {
    // message fields
    cf_files: ::protobuf::RepeatedField<SnapshotCFFile>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotMeta {}

impl SnapshotMeta {
    pub fn new() -> SnapshotMeta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotMeta {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotMeta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotMeta,
        };
        unsafe {
            instance.get(SnapshotMeta::new)
        }
    }

    // repeated .raft_serverpb.SnapshotCFFile cf_files = 1;

    pub fn clear_cf_files(&mut self) {
        self.cf_files.clear();
    }

    // Param is passed by value, moved
    pub fn set_cf_files(&mut self, v: ::protobuf::RepeatedField<SnapshotCFFile>) {
        self.cf_files = v;
    }

    // Mutable pointer to the field.
    pub fn mut_cf_files(&mut self) -> &mut ::protobuf::RepeatedField<SnapshotCFFile> {
        &mut self.cf_files
    }

    // Take field
    pub fn take_cf_files(&mut self) -> ::protobuf::RepeatedField<SnapshotCFFile> {
        ::std::mem::replace(&mut self.cf_files, ::protobuf::RepeatedField::new())
    }

    pub fn get_cf_files(&self) -> &[SnapshotCFFile] {
        &self.cf_files
    }

    fn get_cf_files_for_reflect(&self) -> &::protobuf::RepeatedField<SnapshotCFFile> {
        &self.cf_files
    }

    fn mut_cf_files_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<SnapshotCFFile> {
        &mut self.cf_files
    }
}

impl ::protobuf::Message for SnapshotMeta {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.cf_files)?;
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
        for value in &self.cf_files {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        for v in &self.cf_files {
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

impl ::protobuf::MessageStatic for SnapshotMeta {
    fn new() -> SnapshotMeta {
        SnapshotMeta::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotMeta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SnapshotCFFile>>(
                    "cf_files",
                    SnapshotMeta::get_cf_files_for_reflect,
                    SnapshotMeta::mut_cf_files_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotMeta>(
                    "SnapshotMeta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotMeta {
    fn clear(&mut self) {
        self.clear_cf_files();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotMeta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SnapshotChunk {
    // message fields
    message: ::protobuf::SingularPtrField<RaftMessage>,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SnapshotChunk {}

impl SnapshotChunk {
    pub fn new() -> SnapshotChunk {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SnapshotChunk {
        static mut instance: ::protobuf::lazy::Lazy<SnapshotChunk> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SnapshotChunk,
        };
        unsafe {
            instance.get(SnapshotChunk::new)
        }
    }

    // .raft_serverpb.RaftMessage message = 1;

    pub fn clear_message(&mut self) {
        self.message.clear();
    }

    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }

    // Param is passed by value, moved
    pub fn set_message(&mut self, v: RaftMessage) {
        self.message = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_message(&mut self) -> &mut RaftMessage {
        if self.message.is_none() {
            self.message.set_default();
        };
        self.message.as_mut().unwrap()
    }

    // Take field
    pub fn take_message(&mut self) -> RaftMessage {
        self.message.take().unwrap_or_else(|| RaftMessage::new())
    }

    pub fn get_message(&self) -> &RaftMessage {
        self.message.as_ref().unwrap_or_else(|| RaftMessage::default_instance())
    }

    fn get_message_for_reflect(&self) -> &::protobuf::SingularPtrField<RaftMessage> {
        &self.message
    }

    fn mut_message_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RaftMessage> {
        &mut self.message
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

impl ::protobuf::Message for SnapshotChunk {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.message)?;
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
        if let Some(v) = self.message.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if !self.data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.data);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.message.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if !self.data.is_empty() {
            os.write_bytes(2, &self.data)?;
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

impl ::protobuf::MessageStatic for SnapshotChunk {
    fn new() -> SnapshotChunk {
        SnapshotChunk::new()
    }

    fn descriptor_static(_: ::std::option::Option<SnapshotChunk>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RaftMessage>>(
                    "message",
                    SnapshotChunk::get_message_for_reflect,
                    SnapshotChunk::mut_message_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    SnapshotChunk::get_data_for_reflect,
                    SnapshotChunk::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SnapshotChunk>(
                    "SnapshotChunk",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SnapshotChunk {
    fn clear(&mut self) {
        self.clear_message();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SnapshotChunk {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SnapshotChunk {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Done {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Done {}

impl Done {
    pub fn new() -> Done {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Done {
        static mut instance: ::protobuf::lazy::Lazy<Done> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Done,
        };
        unsafe {
            instance.get(Done::new)
        }
    }
}

impl ::protobuf::Message for Done {
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

impl ::protobuf::MessageStatic for Done {
    fn new() -> Done {
        Done::new()
    }

    fn descriptor_static(_: ::std::option::Option<Done>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<Done>(
                    "Done",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Done {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Done {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Done {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct KeyValue {
    // message fields
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for KeyValue {}

impl KeyValue {
    pub fn new() -> KeyValue {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static KeyValue {
        static mut instance: ::protobuf::lazy::Lazy<KeyValue> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const KeyValue,
        };
        unsafe {
            instance.get(KeyValue::new)
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

    // bytes value = 2;

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

impl ::protobuf::Message for KeyValue {
    fn is_initialized(&self) -> bool {
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
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.key);
        };
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        };
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
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

impl ::protobuf::MessageStatic for KeyValue {
    fn new() -> KeyValue {
        KeyValue::new()
    }

    fn descriptor_static(_: ::std::option::Option<KeyValue>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    KeyValue::get_key_for_reflect,
                    KeyValue::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    KeyValue::get_value_for_reflect,
                    KeyValue::mut_value_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<KeyValue>(
                    "KeyValue",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for KeyValue {
    fn clear(&mut self) {
        self.clear_key();
        self.clear_value();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for KeyValue {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for KeyValue {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RaftSnapshotData {
    // message fields
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    pub file_size: u64,
    data: ::protobuf::RepeatedField<KeyValue>,
    pub version: u64,
    meta: ::protobuf::SingularPtrField<SnapshotMeta>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftSnapshotData {}

impl RaftSnapshotData {
    pub fn new() -> RaftSnapshotData {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftSnapshotData {
        static mut instance: ::protobuf::lazy::Lazy<RaftSnapshotData> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftSnapshotData,
        };
        unsafe {
            instance.get(RaftSnapshotData::new)
        }
    }

    // .metapb.Region region = 1;

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

    // uint64 file_size = 2;

    pub fn clear_file_size(&mut self) {
        self.file_size = 0;
    }

    // Param is passed by value, moved
    pub fn set_file_size(&mut self, v: u64) {
        self.file_size = v;
    }

    pub fn get_file_size(&self) -> u64 {
        self.file_size
    }

    fn get_file_size_for_reflect(&self) -> &u64 {
        &self.file_size
    }

    fn mut_file_size_for_reflect(&mut self) -> &mut u64 {
        &mut self.file_size
    }

    // repeated .raft_serverpb.KeyValue data = 3;

    pub fn clear_data(&mut self) {
        self.data.clear();
    }

    // Param is passed by value, moved
    pub fn set_data(&mut self, v: ::protobuf::RepeatedField<KeyValue>) {
        self.data = v;
    }

    // Mutable pointer to the field.
    pub fn mut_data(&mut self) -> &mut ::protobuf::RepeatedField<KeyValue> {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::protobuf::RepeatedField<KeyValue> {
        ::std::mem::replace(&mut self.data, ::protobuf::RepeatedField::new())
    }

    pub fn get_data(&self) -> &[KeyValue] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::protobuf::RepeatedField<KeyValue> {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<KeyValue> {
        &mut self.data
    }

    // uint64 version = 4;

    pub fn clear_version(&mut self) {
        self.version = 0;
    }

    // Param is passed by value, moved
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }

    pub fn get_version(&self) -> u64 {
        self.version
    }

    fn get_version_for_reflect(&self) -> &u64 {
        &self.version
    }

    fn mut_version_for_reflect(&mut self) -> &mut u64 {
        &mut self.version
    }

    // .raft_serverpb.SnapshotMeta meta = 5;

    pub fn clear_meta(&mut self) {
        self.meta.clear();
    }

    pub fn has_meta(&self) -> bool {
        self.meta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meta(&mut self, v: SnapshotMeta) {
        self.meta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meta(&mut self) -> &mut SnapshotMeta {
        if self.meta.is_none() {
            self.meta.set_default();
        };
        self.meta.as_mut().unwrap()
    }

    // Take field
    pub fn take_meta(&mut self) -> SnapshotMeta {
        self.meta.take().unwrap_or_else(|| SnapshotMeta::new())
    }

    pub fn get_meta(&self) -> &SnapshotMeta {
        self.meta.as_ref().unwrap_or_else(|| SnapshotMeta::default_instance())
    }

    fn get_meta_for_reflect(&self) -> &::protobuf::SingularPtrField<SnapshotMeta> {
        &self.meta
    }

    fn mut_meta_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SnapshotMeta> {
        &mut self.meta
    }
}

impl ::protobuf::Message for RaftSnapshotData {
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.file_size = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.version = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meta)?;
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
        if self.file_size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.file_size, ::protobuf::wire_format::WireTypeVarint);
        };
        for value in &self.data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(4, self.version, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.meta.as_ref() {
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
        if self.file_size != 0 {
            os.write_uint64(2, self.file_size)?;
        };
        for v in &self.data {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.version != 0 {
            os.write_uint64(4, self.version)?;
        };
        if let Some(v) = self.meta.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for RaftSnapshotData {
    fn new() -> RaftSnapshotData {
        RaftSnapshotData::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftSnapshotData>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Region>>(
                    "region",
                    RaftSnapshotData::get_region_for_reflect,
                    RaftSnapshotData::mut_region_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "file_size",
                    RaftSnapshotData::get_file_size_for_reflect,
                    RaftSnapshotData::mut_file_size_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<KeyValue>>(
                    "data",
                    RaftSnapshotData::get_data_for_reflect,
                    RaftSnapshotData::mut_data_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "version",
                    RaftSnapshotData::get_version_for_reflect,
                    RaftSnapshotData::mut_version_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SnapshotMeta>>(
                    "meta",
                    RaftSnapshotData::get_meta_for_reflect,
                    RaftSnapshotData::mut_meta_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftSnapshotData>(
                    "RaftSnapshotData",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftSnapshotData {
    fn clear(&mut self) {
        self.clear_region();
        self.clear_file_size();
        self.clear_data();
        self.clear_version();
        self.clear_meta();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftSnapshotData {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftSnapshotData {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct StoreIdent {
    // message fields
    pub cluster_id: u64,
    pub store_id: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for StoreIdent {}

impl StoreIdent {
    pub fn new() -> StoreIdent {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static StoreIdent {
        static mut instance: ::protobuf::lazy::Lazy<StoreIdent> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const StoreIdent,
        };
        unsafe {
            instance.get(StoreIdent::new)
        }
    }

    // uint64 cluster_id = 1;

    pub fn clear_cluster_id(&mut self) {
        self.cluster_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_cluster_id(&mut self, v: u64) {
        self.cluster_id = v;
    }

    pub fn get_cluster_id(&self) -> u64 {
        self.cluster_id
    }

    fn get_cluster_id_for_reflect(&self) -> &u64 {
        &self.cluster_id
    }

    fn mut_cluster_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.cluster_id
    }

    // uint64 store_id = 2;

    pub fn clear_store_id(&mut self) {
        self.store_id = 0;
    }

    // Param is passed by value, moved
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = v;
    }

    pub fn get_store_id(&self) -> u64 {
        self.store_id
    }

    fn get_store_id_for_reflect(&self) -> &u64 {
        &self.store_id
    }

    fn mut_store_id_for_reflect(&mut self) -> &mut u64 {
        &mut self.store_id
    }
}

impl ::protobuf::Message for StoreIdent {
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
                    self.cluster_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.store_id = tmp;
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
        if self.cluster_id != 0 {
            my_size += ::protobuf::rt::value_size(1, self.cluster_id, ::protobuf::wire_format::WireTypeVarint);
        };
        if self.store_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.store_id, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cluster_id != 0 {
            os.write_uint64(1, self.cluster_id)?;
        };
        if self.store_id != 0 {
            os.write_uint64(2, self.store_id)?;
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

impl ::protobuf::MessageStatic for StoreIdent {
    fn new() -> StoreIdent {
        StoreIdent::new()
    }

    fn descriptor_static(_: ::std::option::Option<StoreIdent>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "cluster_id",
                    StoreIdent::get_cluster_id_for_reflect,
                    StoreIdent::mut_cluster_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "store_id",
                    StoreIdent::get_store_id_for_reflect,
                    StoreIdent::mut_store_id_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<StoreIdent>(
                    "StoreIdent",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for StoreIdent {
    fn clear(&mut self) {
        self.clear_cluster_id();
        self.clear_store_id();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for StoreIdent {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for StoreIdent {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RaftLocalState {
    // message fields
    hard_state: ::protobuf::SingularPtrField<super::eraftpb::HardState>,
    pub last_index: u64,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftLocalState {}

impl RaftLocalState {
    pub fn new() -> RaftLocalState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftLocalState {
        static mut instance: ::protobuf::lazy::Lazy<RaftLocalState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftLocalState,
        };
        unsafe {
            instance.get(RaftLocalState::new)
        }
    }

    // .eraftpb.HardState hard_state = 1;

    pub fn clear_hard_state(&mut self) {
        self.hard_state.clear();
    }

    pub fn has_hard_state(&self) -> bool {
        self.hard_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_hard_state(&mut self, v: super::eraftpb::HardState) {
        self.hard_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_hard_state(&mut self) -> &mut super::eraftpb::HardState {
        if self.hard_state.is_none() {
            self.hard_state.set_default();
        };
        self.hard_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_hard_state(&mut self) -> super::eraftpb::HardState {
        self.hard_state.take().unwrap_or_else(|| super::eraftpb::HardState::new())
    }

    pub fn get_hard_state(&self) -> &super::eraftpb::HardState {
        self.hard_state.as_ref().unwrap_or_else(|| super::eraftpb::HardState::default_instance())
    }

    fn get_hard_state_for_reflect(&self) -> &::protobuf::SingularPtrField<super::eraftpb::HardState> {
        &self.hard_state
    }

    fn mut_hard_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::eraftpb::HardState> {
        &mut self.hard_state
    }

    // uint64 last_index = 2;

    pub fn clear_last_index(&mut self) {
        self.last_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_last_index(&mut self, v: u64) {
        self.last_index = v;
    }

    pub fn get_last_index(&self) -> u64 {
        self.last_index
    }

    fn get_last_index_for_reflect(&self) -> &u64 {
        &self.last_index
    }

    fn mut_last_index_for_reflect(&mut self) -> &mut u64 {
        &mut self.last_index
    }
}

impl ::protobuf::Message for RaftLocalState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hard_state)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    };
                    let tmp = is.read_uint64()?;
                    self.last_index = tmp;
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
        if let Some(v) = self.hard_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.last_index != 0 {
            my_size += ::protobuf::rt::value_size(2, self.last_index, ::protobuf::wire_format::WireTypeVarint);
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(v) = self.hard_state.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.last_index != 0 {
            os.write_uint64(2, self.last_index)?;
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

impl ::protobuf::MessageStatic for RaftLocalState {
    fn new() -> RaftLocalState {
        RaftLocalState::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftLocalState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::eraftpb::HardState>>(
                    "hard_state",
                    RaftLocalState::get_hard_state_for_reflect,
                    RaftLocalState::mut_hard_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "last_index",
                    RaftLocalState::get_last_index_for_reflect,
                    RaftLocalState::mut_last_index_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftLocalState>(
                    "RaftLocalState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftLocalState {
    fn clear(&mut self) {
        self.clear_hard_state();
        self.clear_last_index();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftLocalState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftLocalState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RaftApplyState {
    // message fields
    pub applied_index: u64,
    truncated_state: ::protobuf::SingularPtrField<RaftTruncatedState>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RaftApplyState {}

impl RaftApplyState {
    pub fn new() -> RaftApplyState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RaftApplyState {
        static mut instance: ::protobuf::lazy::Lazy<RaftApplyState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RaftApplyState,
        };
        unsafe {
            instance.get(RaftApplyState::new)
        }
    }

    // uint64 applied_index = 1;

    pub fn clear_applied_index(&mut self) {
        self.applied_index = 0;
    }

    // Param is passed by value, moved
    pub fn set_applied_index(&mut self, v: u64) {
        self.applied_index = v;
    }

    pub fn get_applied_index(&self) -> u64 {
        self.applied_index
    }

    fn get_applied_index_for_reflect(&self) -> &u64 {
        &self.applied_index
    }

    fn mut_applied_index_for_reflect(&mut self) -> &mut u64 {
        &mut self.applied_index
    }

    // .raft_serverpb.RaftTruncatedState truncated_state = 2;

    pub fn clear_truncated_state(&mut self) {
        self.truncated_state.clear();
    }

    pub fn has_truncated_state(&self) -> bool {
        self.truncated_state.is_some()
    }

    // Param is passed by value, moved
    pub fn set_truncated_state(&mut self, v: RaftTruncatedState) {
        self.truncated_state = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_truncated_state(&mut self) -> &mut RaftTruncatedState {
        if self.truncated_state.is_none() {
            self.truncated_state.set_default();
        };
        self.truncated_state.as_mut().unwrap()
    }

    // Take field
    pub fn take_truncated_state(&mut self) -> RaftTruncatedState {
        self.truncated_state.take().unwrap_or_else(|| RaftTruncatedState::new())
    }

    pub fn get_truncated_state(&self) -> &RaftTruncatedState {
        self.truncated_state.as_ref().unwrap_or_else(|| RaftTruncatedState::default_instance())
    }

    fn get_truncated_state_for_reflect(&self) -> &::protobuf::SingularPtrField<RaftTruncatedState> {
        &self.truncated_state
    }

    fn mut_truncated_state_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<RaftTruncatedState> {
        &mut self.truncated_state
    }
}

impl ::protobuf::Message for RaftApplyState {
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
                    self.applied_index = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.truncated_state)?;
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
        if self.applied_index != 0 {
            my_size += ::protobuf::rt::value_size(1, self.applied_index, ::protobuf::wire_format::WireTypeVarint);
        };
        if let Some(v) = self.truncated_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.applied_index != 0 {
            os.write_uint64(1, self.applied_index)?;
        };
        if let Some(v) = self.truncated_state.as_ref() {
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

impl ::protobuf::MessageStatic for RaftApplyState {
    fn new() -> RaftApplyState {
        RaftApplyState::new()
    }

    fn descriptor_static(_: ::std::option::Option<RaftApplyState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "applied_index",
                    RaftApplyState::get_applied_index_for_reflect,
                    RaftApplyState::mut_applied_index_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<RaftTruncatedState>>(
                    "truncated_state",
                    RaftApplyState::get_truncated_state_for_reflect,
                    RaftApplyState::mut_truncated_state_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RaftApplyState>(
                    "RaftApplyState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RaftApplyState {
    fn clear(&mut self) {
        self.clear_applied_index();
        self.clear_truncated_state();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RaftApplyState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RaftApplyState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct RegionLocalState {
    // message fields
    pub state: PeerState,
    region: ::protobuf::SingularPtrField<super::metapb::Region>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for RegionLocalState {}

impl RegionLocalState {
    pub fn new() -> RegionLocalState {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static RegionLocalState {
        static mut instance: ::protobuf::lazy::Lazy<RegionLocalState> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const RegionLocalState,
        };
        unsafe {
            instance.get(RegionLocalState::new)
        }
    }

    // .raft_serverpb.PeerState state = 1;

    pub fn clear_state(&mut self) {
        self.state = PeerState::Normal;
    }

    // Param is passed by value, moved
    pub fn set_state(&mut self, v: PeerState) {
        self.state = v;
    }

    pub fn get_state(&self) -> PeerState {
        self.state
    }

    fn get_state_for_reflect(&self) -> &PeerState {
        &self.state
    }

    fn mut_state_for_reflect(&mut self) -> &mut PeerState {
        &mut self.state
    }

    // .metapb.Region region = 2;

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

impl ::protobuf::Message for RegionLocalState {
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
                    self.state = tmp;
                },
                2 => {
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
        if self.state != PeerState::Normal {
            my_size += ::protobuf::rt::enum_size(1, self.state);
        };
        if let Some(v) = self.region.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.state != PeerState::Normal {
            os.write_enum(1, self.state.value())?;
        };
        if let Some(v) = self.region.as_ref() {
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

impl ::protobuf::MessageStatic for RegionLocalState {
    fn new() -> RegionLocalState {
        RegionLocalState::new()
    }

    fn descriptor_static(_: ::std::option::Option<RegionLocalState>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<PeerState>>(
                    "state",
                    RegionLocalState::get_state_for_reflect,
                    RegionLocalState::mut_state_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::Region>>(
                    "region",
                    RegionLocalState::get_region_for_reflect,
                    RegionLocalState::mut_region_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<RegionLocalState>(
                    "RegionLocalState",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for RegionLocalState {
    fn clear(&mut self) {
        self.clear_state();
        self.clear_region();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for RegionLocalState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for RegionLocalState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum PeerState {
    Normal = 0,
    Applying = 1,
    Tombstone = 2,
}

impl ::protobuf::ProtobufEnum for PeerState {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<PeerState> {
        match value {
            0 => ::std::option::Option::Some(PeerState::Normal),
            1 => ::std::option::Option::Some(PeerState::Applying),
            2 => ::std::option::Option::Some(PeerState::Tombstone),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [PeerState] = &[
            PeerState::Normal,
            PeerState::Applying,
            PeerState::Tombstone,
        ];
        values
    }

    fn enum_descriptor_static(_: Option<PeerState>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("PeerState", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for PeerState {
}

impl ::std::default::Default for PeerState {
    fn default() -> Self {
        PeerState::Normal
    }
}

impl ::protobuf::reflect::ProtobufValue for PeerState {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

static file_descriptor_proto_data: &'static [u8] = &[
    0x0a, 0x13, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x70, 0x62, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x73, 0x65, 0x72, 0x76,
    0x65, 0x72, 0x70, 0x62, 0x1a, 0x0d, 0x65, 0x72, 0x61, 0x66, 0x74, 0x70, 0x62, 0x2e, 0x70, 0x72,
    0x6f, 0x74, 0x6f, 0x1a, 0x0c, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x70, 0x72, 0x6f, 0x74,
    0x6f, 0x22, 0xb9, 0x02, 0x0a, 0x0b, 0x52, 0x61, 0x66, 0x74, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x12, 0x1b, 0x0a, 0x09, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x08, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x29,
    0x0a, 0x09, 0x66, 0x72, 0x6f, 0x6d, 0x5f, 0x70, 0x65, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x52,
    0x08, 0x66, 0x72, 0x6f, 0x6d, 0x50, 0x65, 0x65, 0x72, 0x12, 0x25, 0x0a, 0x07, 0x74, 0x6f, 0x5f,
    0x70, 0x65, 0x65, 0x72, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0c, 0x2e, 0x6d, 0x65, 0x74,
    0x61, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x52, 0x06, 0x74, 0x6f, 0x50, 0x65, 0x65, 0x72,
    0x12, 0x2a, 0x0a, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x04, 0x20, 0x01, 0x28,
    0x0b, 0x32, 0x10, 0x2e, 0x65, 0x72, 0x61, 0x66, 0x74, 0x70, 0x62, 0x2e, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x36, 0x0a, 0x0c,
    0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x5f, 0x65, 0x70, 0x6f, 0x63, 0x68, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x0b, 0x32, 0x13, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52, 0x65, 0x67, 0x69,
    0x6f, 0x6e, 0x45, 0x70, 0x6f, 0x63, 0x68, 0x52, 0x0b, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x45,
    0x70, 0x6f, 0x63, 0x68, 0x12, 0x21, 0x0a, 0x0c, 0x69, 0x73, 0x5f, 0x74, 0x6f, 0x6d, 0x62, 0x73,
    0x74, 0x6f, 0x6e, 0x65, 0x18, 0x06, 0x20, 0x01, 0x28, 0x08, 0x52, 0x0b, 0x69, 0x73, 0x54, 0x6f,
    0x6d, 0x62, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x73, 0x74, 0x61, 0x72, 0x74,
    0x5f, 0x6b, 0x65, 0x79, 0x18, 0x07, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x08, 0x73, 0x74, 0x61, 0x72,
    0x74, 0x4b, 0x65, 0x79, 0x12, 0x17, 0x0a, 0x07, 0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x18,
    0x08, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x65, 0x6e, 0x64, 0x4b, 0x65, 0x79, 0x22, 0x3e, 0x0a,
    0x12, 0x52, 0x61, 0x66, 0x74, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x64, 0x53, 0x74,
    0x61, 0x74, 0x65, 0x12, 0x14, 0x0a, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x04, 0x52, 0x05, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x12, 0x0a, 0x04, 0x74, 0x65, 0x72,
    0x6d, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x74, 0x65, 0x72, 0x6d, 0x22, 0x50, 0x0a,
    0x0e, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x43, 0x46, 0x46, 0x69, 0x6c, 0x65, 0x12,
    0x0e, 0x0a, 0x02, 0x63, 0x66, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x02, 0x63, 0x66, 0x12,
    0x12, 0x0a, 0x04, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04, 0x52, 0x04, 0x73,
    0x69, 0x7a, 0x65, 0x12, 0x1a, 0x0a, 0x08, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x18,
    0x03, 0x20, 0x01, 0x28, 0x0d, 0x52, 0x08, 0x63, 0x68, 0x65, 0x63, 0x6b, 0x73, 0x75, 0x6d, 0x22,
    0x48, 0x0a, 0x0c, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x4d, 0x65, 0x74, 0x61, 0x12,
    0x38, 0x0a, 0x08, 0x63, 0x66, 0x5f, 0x66, 0x69, 0x6c, 0x65, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28,
    0x0b, 0x32, 0x1d, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x70,
    0x62, 0x2e, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x43, 0x46, 0x46, 0x69, 0x6c, 0x65,
    0x52, 0x07, 0x63, 0x66, 0x46, 0x69, 0x6c, 0x65, 0x73, 0x22, 0x59, 0x0a, 0x0d, 0x53, 0x6e, 0x61,
    0x70, 0x73, 0x68, 0x6f, 0x74, 0x43, 0x68, 0x75, 0x6e, 0x6b, 0x12, 0x34, 0x0a, 0x07, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e, 0x72, 0x61,
    0x66, 0x74, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x70, 0x62, 0x2e, 0x52, 0x61, 0x66, 0x74,
    0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x52, 0x07, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x12, 0x12, 0x0a, 0x04, 0x64, 0x61, 0x74, 0x61, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04,
    0x64, 0x61, 0x74, 0x61, 0x22, 0x06, 0x0a, 0x04, 0x44, 0x6f, 0x6e, 0x65, 0x22, 0x32, 0x0a, 0x08,
    0x4b, 0x65, 0x79, 0x56, 0x61, 0x6c, 0x75, 0x65, 0x12, 0x10, 0x0a, 0x03, 0x6b, 0x65, 0x79, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x03, 0x6b, 0x65, 0x79, 0x12, 0x14, 0x0a, 0x05, 0x76, 0x61,
    0x6c, 0x75, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x05, 0x76, 0x61, 0x6c, 0x75, 0x65,
    0x22, 0xcf, 0x01, 0x0a, 0x10, 0x52, 0x61, 0x66, 0x74, 0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f,
    0x74, 0x44, 0x61, 0x74, 0x61, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e, 0x52,
    0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a,
    0x09, 0x66, 0x69, 0x6c, 0x65, 0x5f, 0x73, 0x69, 0x7a, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x04,
    0x52, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x53, 0x69, 0x7a, 0x65, 0x12, 0x2b, 0x0a, 0x04, 0x64, 0x61,
    0x74, 0x61, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x17, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f,
    0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x70, 0x62, 0x2e, 0x4b, 0x65, 0x79, 0x56, 0x61, 0x6c, 0x75,
    0x65, 0x52, 0x04, 0x64, 0x61, 0x74, 0x61, 0x12, 0x18, 0x0a, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69,
    0x6f, 0x6e, 0x18, 0x04, 0x20, 0x01, 0x28, 0x04, 0x52, 0x07, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f,
    0x6e, 0x12, 0x2f, 0x0a, 0x04, 0x6d, 0x65, 0x74, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x1b, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x70, 0x62, 0x2e,
    0x53, 0x6e, 0x61, 0x70, 0x73, 0x68, 0x6f, 0x74, 0x4d, 0x65, 0x74, 0x61, 0x52, 0x04, 0x6d, 0x65,
    0x74, 0x61, 0x22, 0x46, 0x0a, 0x0a, 0x53, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x64, 0x65, 0x6e, 0x74,
    0x12, 0x1d, 0x0a, 0x0a, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x5f, 0x69, 0x64, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x04, 0x52, 0x09, 0x63, 0x6c, 0x75, 0x73, 0x74, 0x65, 0x72, 0x49, 0x64, 0x12,
    0x19, 0x0a, 0x08, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28,
    0x04, 0x52, 0x07, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x49, 0x64, 0x22, 0x62, 0x0a, 0x0e, 0x52, 0x61,
    0x66, 0x74, 0x4c, 0x6f, 0x63, 0x61, 0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x31, 0x0a, 0x0a,
    0x68, 0x61, 0x72, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b,
    0x32, 0x12, 0x2e, 0x65, 0x72, 0x61, 0x66, 0x74, 0x70, 0x62, 0x2e, 0x48, 0x61, 0x72, 0x64, 0x53,
    0x74, 0x61, 0x74, 0x65, 0x52, 0x09, 0x68, 0x61, 0x72, 0x64, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12,
    0x1d, 0x0a, 0x0a, 0x6c, 0x61, 0x73, 0x74, 0x5f, 0x69, 0x6e, 0x64, 0x65, 0x78, 0x18, 0x02, 0x20,
    0x01, 0x28, 0x04, 0x52, 0x09, 0x6c, 0x61, 0x73, 0x74, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x22, 0x81,
    0x01, 0x0a, 0x0e, 0x52, 0x61, 0x66, 0x74, 0x41, 0x70, 0x70, 0x6c, 0x79, 0x53, 0x74, 0x61, 0x74,
    0x65, 0x12, 0x23, 0x0a, 0x0d, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65, 0x64, 0x5f, 0x69, 0x6e, 0x64,
    0x65, 0x78, 0x18, 0x01, 0x20, 0x01, 0x28, 0x04, 0x52, 0x0c, 0x61, 0x70, 0x70, 0x6c, 0x69, 0x65,
    0x64, 0x49, 0x6e, 0x64, 0x65, 0x78, 0x12, 0x4a, 0x0a, 0x0f, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61,
    0x74, 0x65, 0x64, 0x5f, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32,
    0x21, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x73, 0x65, 0x72, 0x76, 0x65, 0x72, 0x70, 0x62, 0x2e,
    0x52, 0x61, 0x66, 0x74, 0x54, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x64, 0x53, 0x74, 0x61,
    0x74, 0x65, 0x52, 0x0e, 0x74, 0x72, 0x75, 0x6e, 0x63, 0x61, 0x74, 0x65, 0x64, 0x53, 0x74, 0x61,
    0x74, 0x65, 0x22, 0x6a, 0x0a, 0x10, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x4c, 0x6f, 0x63, 0x61,
    0x6c, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x2e, 0x0a, 0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x18, 0x2e, 0x72, 0x61, 0x66, 0x74, 0x5f, 0x73, 0x65, 0x72,
    0x76, 0x65, 0x72, 0x70, 0x62, 0x2e, 0x50, 0x65, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x52,
    0x05, 0x73, 0x74, 0x61, 0x74, 0x65, 0x12, 0x26, 0x0a, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e,
    0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x0e, 0x2e, 0x6d, 0x65, 0x74, 0x61, 0x70, 0x62, 0x2e,
    0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x72, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x2a, 0x34,
    0x0a, 0x09, 0x50, 0x65, 0x65, 0x72, 0x53, 0x74, 0x61, 0x74, 0x65, 0x12, 0x0a, 0x0a, 0x06, 0x4e,
    0x6f, 0x72, 0x6d, 0x61, 0x6c, 0x10, 0x00, 0x12, 0x0c, 0x0a, 0x08, 0x41, 0x70, 0x70, 0x6c, 0x79,
    0x69, 0x6e, 0x67, 0x10, 0x01, 0x12, 0x0d, 0x0a, 0x09, 0x54, 0x6f, 0x6d, 0x62, 0x73, 0x74, 0x6f,
    0x6e, 0x65, 0x10, 0x02, 0x42, 0x17, 0x0a, 0x15, 0x63, 0x6f, 0x6d, 0x2e, 0x70, 0x69, 0x6e, 0x67,
    0x63, 0x61, 0x70, 0x2e, 0x74, 0x69, 0x6b, 0x76, 0x2e, 0x67, 0x72, 0x70, 0x63, 0x4a, 0x9f, 0x16,
    0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x50, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00,
    0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x01, 0x08, 0x15, 0x0a, 0x09, 0x0a, 0x02,
    0x03, 0x00, 0x12, 0x03, 0x03, 0x07, 0x16, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x04,
    0x07, 0x15, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x06, 0x00, 0x2e, 0x0a, 0x0b, 0x0a, 0x04,
    0x08, 0xe7, 0x07, 0x00, 0x12, 0x03, 0x06, 0x00, 0x2e, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07,
    0x00, 0x02, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0d, 0x0a, 0x06, 0x08, 0xe7, 0x07, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0e, 0x0a, 0x07, 0x08, 0xe7, 0x07, 0x00, 0x02, 0x00,
    0x01, 0x12, 0x03, 0x06, 0x07, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x08, 0xe7, 0x07, 0x00, 0x07, 0x12,
    0x03, 0x06, 0x16, 0x2d, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x08, 0x00, 0x13, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x08, 0x08, 0x13, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x09, 0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x09, 0x04, 0x08, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x09, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x09, 0x0b, 0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x09,
    0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0a, 0x04, 0x1e, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x04, 0x12, 0x04, 0x0a, 0x04, 0x09, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0a, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x10, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x0a, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02,
    0x12, 0x03, 0x0b, 0x04, 0x1c, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x04, 0x12, 0x04,
    0x0b, 0x04, 0x0a, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0b,
    0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0b, 0x10, 0x17,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0b, 0x1a, 0x1b, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0c, 0x04, 0x20, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x04, 0x12, 0x04, 0x0c, 0x04, 0x0b, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x03, 0x06, 0x12, 0x03, 0x0c, 0x04, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03,
    0x01, 0x12, 0x03, 0x0c, 0x14, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12,
    0x03, 0x0c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x0d, 0x04,
    0x28, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x04, 0x0d, 0x04, 0x0c, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12, 0x03, 0x0d, 0x04, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0d, 0x17, 0x23, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x0d, 0x26, 0x27, 0x0a, 0x52, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x05, 0x12, 0x03, 0x0f, 0x04, 0x1a, 0x1a, 0x45, 0x20, 0x74, 0x72, 0x75, 0x65, 0x20, 0x6d,
    0x65, 0x61, 0x6e, 0x73, 0x20, 0x74, 0x6f, 0x5f, 0x70, 0x65, 0x65, 0x72, 0x20, 0x69, 0x73, 0x20,
    0x61, 0x20, 0x74, 0x6f, 0x6d, 0x62, 0x73, 0x74, 0x6f, 0x6e, 0x65, 0x20, 0x70, 0x65, 0x65, 0x72,
    0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x74, 0x20, 0x73, 0x68, 0x6f, 0x75, 0x6c, 0x64, 0x20, 0x72,
    0x65, 0x6d, 0x6f, 0x76, 0x65, 0x20, 0x69, 0x74, 0x73, 0x65, 0x6c, 0x66, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x05, 0x04, 0x12, 0x04, 0x0f, 0x04, 0x0d, 0x28, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x05, 0x05, 0x12, 0x03, 0x0f, 0x04, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x05, 0x01, 0x12, 0x03, 0x0f, 0x09, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x05, 0x03, 0x12, 0x03, 0x0f, 0x18, 0x19, 0x0a, 0x35, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x06, 0x12,
    0x03, 0x11, 0x04, 0x18, 0x1a, 0x28, 0x20, 0x52, 0x65, 0x67, 0x69, 0x6f, 0x6e, 0x20, 0x6b, 0x65,
    0x79, 0x20, 0x72, 0x61, 0x6e, 0x67, 0x65, 0x20, 0x5b, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6b,
    0x65, 0x79, 0x2c, 0x20, 0x65, 0x6e, 0x64, 0x5f, 0x6b, 0x65, 0x79, 0x29, 0x2e, 0x0a, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x06, 0x04, 0x12, 0x04, 0x11, 0x04, 0x0f, 0x1a, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x06, 0x05, 0x12, 0x03, 0x11, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x06, 0x01, 0x12, 0x03, 0x11, 0x0a, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x06, 0x03, 0x12, 0x03, 0x11, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x07, 0x12,
    0x03, 0x12, 0x04, 0x16, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x04, 0x12, 0x04, 0x12,
    0x04, 0x11, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x05, 0x12, 0x03, 0x12, 0x04,
    0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x01, 0x12, 0x03, 0x12, 0x0a, 0x11, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x07, 0x03, 0x12, 0x03, 0x12, 0x14, 0x15, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x15, 0x00, 0x18, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x15, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x16,
    0x04, 0x15, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x04, 0x12, 0x04, 0x16, 0x04, 0x15,
    0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x16, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x16, 0x0b, 0x10, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x16, 0x13, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x01, 0x12, 0x03, 0x17, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x04, 0x12, 0x04, 0x17, 0x04, 0x16, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x05,
    0x12, 0x03, 0x17, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x17, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x12,
    0x13, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x02, 0x12, 0x04, 0x1a, 0x00, 0x1e, 0x01, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x02, 0x01, 0x12, 0x03, 0x1a, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02,
    0x00, 0x12, 0x03, 0x1b, 0x04, 0x12, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x04, 0x12,
    0x04, 0x1b, 0x04, 0x1a, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x05, 0x12, 0x03,
    0x1b, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1b, 0x0b,
    0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x00, 0x03, 0x12, 0x03, 0x1b, 0x10, 0x11, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x04, 0x14, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x02, 0x02, 0x01, 0x04, 0x12, 0x04, 0x1c, 0x04, 0x1b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x02, 0x02, 0x01, 0x05, 0x12, 0x03, 0x1c, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02,
    0x01, 0x01, 0x12, 0x03, 0x1c, 0x0b, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x01, 0x03,
    0x12, 0x03, 0x1c, 0x12, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1d,
    0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x04, 0x12, 0x04, 0x1d, 0x04, 0x1c,
    0x14, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x05, 0x12, 0x03, 0x1d, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x02, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1d, 0x0b, 0x13, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x02, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x16, 0x17, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x03, 0x12, 0x04, 0x20, 0x00, 0x22, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x03, 0x01, 0x12, 0x03,
    0x20, 0x08, 0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x03, 0x02, 0x00, 0x12, 0x03, 0x21, 0x04, 0x29,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x04, 0x12, 0x03, 0x21, 0x04, 0x0c, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x03, 0x02, 0x00, 0x06, 0x12, 0x03, 0x21, 0x0d, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x03, 0x02, 0x00, 0x01, 0x12, 0x03, 0x21, 0x1c, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x03,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x21, 0x27, 0x28, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x04, 0x12, 0x04,
    0x24, 0x00, 0x27, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x04, 0x01, 0x12, 0x03, 0x24, 0x08, 0x15,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x00, 0x12, 0x03, 0x25, 0x04, 0x1c, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x04, 0x02, 0x00, 0x04, 0x12, 0x04, 0x25, 0x04, 0x24, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x04, 0x02, 0x00, 0x06, 0x12, 0x03, 0x25, 0x04, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04,
    0x02, 0x00, 0x01, 0x12, 0x03, 0x25, 0x10, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x00,
    0x03, 0x12, 0x03, 0x25, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x04, 0x02, 0x01, 0x12, 0x03,
    0x26, 0x04, 0x13, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x04, 0x12, 0x04, 0x26, 0x04,
    0x25, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x05, 0x12, 0x03, 0x26, 0x04, 0x09,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x01, 0x12, 0x03, 0x26, 0x0a, 0x0e, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x04, 0x02, 0x01, 0x03, 0x12, 0x03, 0x26, 0x11, 0x12, 0x0a, 0x09, 0x0a, 0x02,
    0x04, 0x05, 0x12, 0x03, 0x29, 0x00, 0x0f, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x05, 0x01, 0x12, 0x03,
    0x29, 0x08, 0x0c, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x06, 0x12, 0x04, 0x2b, 0x00, 0x2e, 0x01, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x06, 0x01, 0x12, 0x03, 0x2b, 0x08, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x06, 0x02, 0x00, 0x12, 0x03, 0x2c, 0x04, 0x17, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00,
    0x04, 0x12, 0x04, 0x2c, 0x04, 0x2b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x05,
    0x12, 0x03, 0x2c, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x01, 0x12, 0x03,
    0x2c, 0x0a, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02, 0x00, 0x03, 0x12, 0x03, 0x2c, 0x15,
    0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x06, 0x02, 0x01, 0x12, 0x03, 0x2d, 0x04, 0x17, 0x0a, 0x0d,
    0x0a, 0x05, 0x04, 0x06, 0x02, 0x01, 0x04, 0x12, 0x04, 0x2d, 0x04, 0x2c, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x06, 0x02, 0x01, 0x05, 0x12, 0x03, 0x2d, 0x04, 0x09, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x06, 0x02, 0x01, 0x01, 0x12, 0x03, 0x2d, 0x0a, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x06, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x2d, 0x15, 0x16, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x07, 0x12, 0x04, 0x30,
    0x00, 0x36, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x07, 0x01, 0x12, 0x03, 0x30, 0x08, 0x18, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x00, 0x12, 0x03, 0x31, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x00, 0x04, 0x12, 0x04, 0x31, 0x04, 0x30, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x07, 0x02, 0x00, 0x06, 0x12, 0x03, 0x31, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02,
    0x00, 0x01, 0x12, 0x03, 0x31, 0x12, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x00, 0x03,
    0x12, 0x03, 0x31, 0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x01, 0x12, 0x03, 0x32,
    0x04, 0x19, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x04, 0x12, 0x04, 0x32, 0x04, 0x31,
    0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x05, 0x12, 0x03, 0x32, 0x04, 0x0a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x01, 0x01, 0x12, 0x03, 0x32, 0x0b, 0x14, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x01, 0x03, 0x12, 0x03, 0x32, 0x17, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x07, 0x02, 0x02, 0x12, 0x03, 0x33, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02,
    0x04, 0x12, 0x03, 0x33, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x06, 0x12,
    0x03, 0x33, 0x0d, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x01, 0x12, 0x03, 0x33,
    0x16, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x02, 0x03, 0x12, 0x03, 0x33, 0x1d, 0x1e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x03, 0x12, 0x03, 0x34, 0x04, 0x17, 0x0a, 0x0d, 0x0a,
    0x05, 0x04, 0x07, 0x02, 0x03, 0x04, 0x12, 0x04, 0x34, 0x04, 0x33, 0x1f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x07, 0x02, 0x03, 0x05, 0x12, 0x03, 0x34, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07,
    0x02, 0x03, 0x01, 0x12, 0x03, 0x34, 0x0b, 0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x03,
    0x03, 0x12, 0x03, 0x34, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x07, 0x02, 0x04, 0x12, 0x03,
    0x35, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x04, 0x12, 0x04, 0x35, 0x04,
    0x34, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x06, 0x12, 0x03, 0x35, 0x04, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x01, 0x12, 0x03, 0x35, 0x11, 0x15, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x07, 0x02, 0x04, 0x03, 0x12, 0x03, 0x35, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02,
    0x04, 0x08, 0x12, 0x04, 0x38, 0x00, 0x3b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x08, 0x01, 0x12,
    0x03, 0x38, 0x08, 0x12, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08, 0x02, 0x00, 0x12, 0x03, 0x39, 0x04,
    0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x04, 0x12, 0x04, 0x39, 0x04, 0x38, 0x14,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x05, 0x12, 0x03, 0x39, 0x04, 0x0a, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x08, 0x02, 0x00, 0x01, 0x12, 0x03, 0x39, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x08, 0x02, 0x00, 0x03, 0x12, 0x03, 0x39, 0x18, 0x19, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x08,
    0x02, 0x01, 0x12, 0x03, 0x3a, 0x04, 0x18, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x04,
    0x12, 0x04, 0x3a, 0x04, 0x39, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x05, 0x12,
    0x03, 0x3a, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x01, 0x12, 0x03, 0x3a,
    0x0b, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x08, 0x02, 0x01, 0x03, 0x12, 0x03, 0x3a, 0x16, 0x17,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x09, 0x12, 0x04, 0x3d, 0x00, 0x40, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x09, 0x01, 0x12, 0x03, 0x3d, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x09, 0x02, 0x00,
    0x12, 0x03, 0x3e, 0x04, 0x25, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x04, 0x12, 0x04,
    0x3e, 0x04, 0x3d, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x06, 0x12, 0x03, 0x3e,
    0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x01, 0x12, 0x03, 0x3e, 0x16, 0x20,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x00, 0x03, 0x12, 0x03, 0x3e, 0x23, 0x24, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x09, 0x02, 0x01, 0x12, 0x03, 0x3f, 0x04, 0x1a, 0x0a, 0x0d, 0x0a, 0x05, 0x04,
    0x09, 0x02, 0x01, 0x04, 0x12, 0x04, 0x3f, 0x04, 0x3e, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x3f, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x3f, 0x0b, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x09, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x3f, 0x18, 0x19, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x0a, 0x12, 0x04, 0x42, 0x00, 0x45, 0x01,
    0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0a, 0x01, 0x12, 0x03, 0x42, 0x08, 0x16, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x0a, 0x02, 0x00, 0x12, 0x03, 0x43, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02,
    0x00, 0x04, 0x12, 0x04, 0x43, 0x04, 0x42, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x43, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x43, 0x0b, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x00, 0x03, 0x12, 0x03, 0x43,
    0x1b, 0x1c, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0a, 0x02, 0x01, 0x12, 0x03, 0x44, 0x04, 0x2b, 0x0a,
    0x0d, 0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x04, 0x12, 0x04, 0x44, 0x04, 0x43, 0x1d, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x0a, 0x02, 0x01, 0x06, 0x12, 0x03, 0x44, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x0a, 0x02, 0x01, 0x01, 0x12, 0x03, 0x44, 0x17, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0a,
    0x02, 0x01, 0x03, 0x12, 0x03, 0x44, 0x29, 0x2a, 0x0a, 0x0a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04,
    0x47, 0x00, 0x4b, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x47, 0x05, 0x0e,
    0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x00, 0x12, 0x03, 0x48, 0x04, 0x0f, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x48, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x05,
    0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x48, 0x0d, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x01, 0x12, 0x03, 0x49, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x49, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x49,
    0x0f, 0x10, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x02, 0x12, 0x03, 0x4a, 0x04, 0x12, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x4a, 0x04, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12, 0x03, 0x4a, 0x10, 0x11, 0x0a, 0x0a, 0x0a, 0x02, 0x04,
    0x0b, 0x12, 0x04, 0x4d, 0x00, 0x50, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x0b, 0x01, 0x12, 0x03,
    0x4d, 0x08, 0x18, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02, 0x00, 0x12, 0x03, 0x4e, 0x04, 0x18,
    0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x04, 0x12, 0x04, 0x4e, 0x04, 0x4d, 0x1a, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x00, 0x06, 0x12, 0x03, 0x4e, 0x04, 0x0d, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x0b, 0x02, 0x00, 0x01, 0x12, 0x03, 0x4e, 0x0e, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x0b, 0x02, 0x00, 0x03, 0x12, 0x03, 0x4e, 0x16, 0x17, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x0b, 0x02,
    0x01, 0x12, 0x03, 0x4f, 0x04, 0x1d, 0x0a, 0x0d, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x04, 0x12,
    0x04, 0x4f, 0x04, 0x4e, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x06, 0x12, 0x03,
    0x4f, 0x04, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x01, 0x12, 0x03, 0x4f, 0x12,
    0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x0b, 0x02, 0x01, 0x03, 0x12, 0x03, 0x4f, 0x1b, 0x1c, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
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
