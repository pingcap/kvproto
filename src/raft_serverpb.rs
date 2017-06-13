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
    pub start_key: ::bytes::Bytes,
    pub end_key: ::bytes::Bytes,
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
        }
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
        }
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
        }
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
    pub fn set_start_key(&mut self, v: ::bytes::Bytes) {
        self.start_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.start_key
    }

    // Take field
    pub fn take_start_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.start_key, ::bytes::Bytes::new())
    }

    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }

    fn get_start_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.start_key
    }

    fn mut_start_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.start_key
    }

    // bytes end_key = 8;

    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }

    // Param is passed by value, moved
    pub fn set_end_key(&mut self, v: ::bytes::Bytes) {
        self.end_key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.end_key
    }

    // Take field
    pub fn take_end_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.end_key, ::bytes::Bytes::new())
    }

    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }

    fn get_end_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.end_key
    }

    fn mut_end_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.end_key
    }
}

impl ::protobuf::Message for RaftMessage {
    fn is_initialized(&self) -> bool {
        for v in &self.from_peer {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.to_peer {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.message {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
                    }
                    let tmp = is.read_bool()?;
                    self.is_tombstone = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.start_key)?;
                },
                8 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.end_key)?;
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
        }
        if let Some(ref v) = self.from_peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.to_peer.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.message.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.region_epoch.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.is_tombstone != false {
            my_size += 2;
        }
        if !self.start_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(7, &self.start_key);
        }
        if !self.end_key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(8, &self.end_key);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.region_id != 0 {
            os.write_uint64(1, self.region_id)?;
        }
        if let Some(ref v) = self.from_peer.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.to_peer.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.message.as_ref() {
            os.write_tag(4, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.region_epoch.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.is_tombstone != false {
            os.write_bool(6, self.is_tombstone)?;
        }
        if !self.start_key.is_empty() {
            os.write_bytes(7, &self.start_key)?;
        }
        if !self.end_key.is_empty() {
            os.write_bytes(8, &self.end_key)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "start_key",
                    RaftMessage::get_start_key_for_reflect,
                    RaftMessage::mut_start_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
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
                    }
                    let tmp = is.read_uint64()?;
                    self.index = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if self.term != 0 {
            my_size += ::protobuf::rt::value_size(2, self.term, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.index != 0 {
            os.write_uint64(1, self.index)?;
        }
        if self.term != 0 {
            os.write_uint64(2, self.term)?;
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
    pub cf: ::protobuf::chars::Chars,
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
        ::protobuf::Clear::clear(&mut self.cf);
    }

    // Param is passed by value, moved
    pub fn set_cf(&mut self, v: ::protobuf::chars::Chars) {
        self.cf = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cf(&mut self) -> &mut ::protobuf::chars::Chars {
        &mut self.cf
    }

    // Take field
    pub fn take_cf(&mut self) -> ::protobuf::chars::Chars {
        ::std::mem::replace(&mut self.cf, ::protobuf::chars::Chars::new())
    }

    pub fn get_cf(&self) -> &str {
        &self.cf
    }

    fn get_cf_for_reflect(&self) -> &::protobuf::chars::Chars {
        &self.cf
    }

    fn mut_cf_for_reflect(&mut self) -> &mut ::protobuf::chars::Chars {
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
                    ::protobuf::rt::read_singular_proto3_carllerche_string_into(wire_type, is, &mut self.cf)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.size = tmp;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.checksum != 0 {
            my_size += ::protobuf::rt::value_size(3, self.checksum, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.cf.is_empty() {
            os.write_string(1, &self.cf)?;
        }
        if self.size != 0 {
            os.write_uint64(2, self.size)?;
        }
        if self.checksum != 0 {
            os.write_uint32(3, self.checksum)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheChars>(
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
        for v in &self.cf_files {
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
    pub data: ::bytes::Bytes,
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
        }
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
    pub fn set_data(&mut self, v: ::bytes::Bytes) {
        self.data = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_data(&mut self) -> &mut ::bytes::Bytes {
        &mut self.data
    }

    // Take field
    pub fn take_data(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.data, ::bytes::Bytes::new())
    }

    pub fn get_data(&self) -> &[u8] {
        &self.data
    }

    fn get_data_for_reflect(&self) -> &::bytes::Bytes {
        &self.data
    }

    fn mut_data_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.data
    }
}

impl ::protobuf::Message for SnapshotChunk {
    fn is_initialized(&self) -> bool {
        for v in &self.message {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.message)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.data)?;
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
        if let Some(ref v) = self.message.as_ref() {
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
        if let Some(ref v) = self.message.as_ref() {
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
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
    pub key: ::bytes::Bytes,
    pub value: ::bytes::Bytes,
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
    pub fn set_key(&mut self, v: ::bytes::Bytes) {
        self.key = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_key(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // Take field
    pub fn take_key(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.key, ::bytes::Bytes::new())
    }

    pub fn get_key(&self) -> &[u8] {
        &self.key
    }

    fn get_key_for_reflect(&self) -> &::bytes::Bytes {
        &self.key
    }

    fn mut_key_for_reflect(&mut self) -> &mut ::bytes::Bytes {
        &mut self.key
    }

    // bytes value = 2;

    pub fn clear_value(&mut self) {
        self.value.clear();
    }

    // Param is passed by value, moved
    pub fn set_value(&mut self, v: ::bytes::Bytes) {
        self.value = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_value(&mut self) -> &mut ::bytes::Bytes {
        &mut self.value
    }

    // Take field
    pub fn take_value(&mut self) -> ::bytes::Bytes {
        ::std::mem::replace(&mut self.value, ::bytes::Bytes::new())
    }

    pub fn get_value(&self) -> &[u8] {
        &self.value
    }

    fn get_value_for_reflect(&self) -> &::bytes::Bytes {
        &self.value
    }

    fn mut_value_for_reflect(&mut self) -> &mut ::bytes::Bytes {
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
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.key)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_carllerche_bytes_into(wire_type, is, &mut self.value)?;
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
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.key.is_empty() {
            os.write_bytes(1, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(2, &self.value)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
                    "key",
                    KeyValue::get_key_for_reflect,
                    KeyValue::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeCarllercheBytes>(
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
        }
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
        }
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
        for v in &self.region {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.data {
            if !v.is_initialized() {
                return false;
            }
        };
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.file_size = tmp;
                },
                3 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.data)?;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        if let Some(ref v) = self.region.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.file_size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.file_size, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.data {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        if self.version != 0 {
            my_size += ::protobuf::rt::value_size(4, self.version, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.meta.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.region.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.file_size != 0 {
            os.write_uint64(2, self.file_size)?;
        }
        for v in &self.data {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        };
        if self.version != 0 {
            os.write_uint64(4, self.version)?;
        }
        if let Some(ref v) = self.meta.as_ref() {
            os.write_tag(5, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
                    }
                    let tmp = is.read_uint64()?;
                    self.cluster_id = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        }
        if self.store_id != 0 {
            my_size += ::protobuf::rt::value_size(2, self.store_id, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.cluster_id != 0 {
            os.write_uint64(1, self.cluster_id)?;
        }
        if self.store_id != 0 {
            os.write_uint64(2, self.store_id)?;
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
        }
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
        for v in &self.hard_state {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.hard_state)?;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
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
        if let Some(ref v) = self.hard_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.last_index != 0 {
            my_size += ::protobuf::rt::value_size(2, self.last_index, ::protobuf::wire_format::WireTypeVarint);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.hard_state.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.last_index != 0 {
            os.write_uint64(2, self.last_index)?;
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
        }
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
        for v in &self.truncated_state {
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
        }
        if let Some(ref v) = self.truncated_state.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.applied_index != 0 {
            os.write_uint64(1, self.applied_index)?;
        }
        if let Some(ref v) = self.truncated_state.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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
        }
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
        for v in &self.region {
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
        }
        if let Some(ref v) = self.region.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.state != PeerState::Normal {
            os.write_enum(1, self.state.value())?;
        }
        if let Some(ref v) = self.region.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

    fn enum_descriptor_static(_: ::std::option::Option<PeerState>) -> &'static ::protobuf::reflect::EnumDescriptor {
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

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x13raft_serverpb.proto\x12\rraft_serverpb\x1a\reraftpb.proto\x1a\x0cm\
    etapb.proto\x1a\x0frustproto.proto\"\xb9\x02\n\x0bRaftMessage\x12\x1b\n\
    \tregion_id\x18\x01\x20\x01(\x04R\x08regionId\x12)\n\tfrom_peer\x18\x02\
    \x20\x01(\x0b2\x0c.metapb.PeerR\x08fromPeer\x12%\n\x07to_peer\x18\x03\
    \x20\x01(\x0b2\x0c.metapb.PeerR\x06toPeer\x12*\n\x07message\x18\x04\x20\
    \x01(\x0b2\x10.eraftpb.MessageR\x07message\x126\n\x0cregion_epoch\x18\
    \x05\x20\x01(\x0b2\x13.metapb.RegionEpochR\x0bregionEpoch\x12!\n\x0cis_t\
    ombstone\x18\x06\x20\x01(\x08R\x0bisTombstone\x12\x1b\n\tstart_key\x18\
    \x07\x20\x01(\x0cR\x08startKey\x12\x17\n\x07end_key\x18\x08\x20\x01(\x0c\
    R\x06endKey\">\n\x12RaftTruncatedState\x12\x14\n\x05index\x18\x01\x20\
    \x01(\x04R\x05index\x12\x12\n\x04term\x18\x02\x20\x01(\x04R\x04term\"P\n\
    \x0eSnapshotCFFile\x12\x0e\n\x02cf\x18\x01\x20\x01(\tR\x02cf\x12\x12\n\
    \x04size\x18\x02\x20\x01(\x04R\x04size\x12\x1a\n\x08checksum\x18\x03\x20\
    \x01(\rR\x08checksum\"H\n\x0cSnapshotMeta\x128\n\x08cf_files\x18\x01\x20\
    \x03(\x0b2\x1d.raft_serverpb.SnapshotCFFileR\x07cfFiles\"Y\n\rSnapshotCh\
    unk\x124\n\x07message\x18\x01\x20\x01(\x0b2\x1a.raft_serverpb.RaftMessag\
    eR\x07message\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04data\"\x06\n\
    \x04Done\"2\n\x08KeyValue\x12\x10\n\x03key\x18\x01\x20\x01(\x0cR\x03key\
    \x12\x14\n\x05value\x18\x02\x20\x01(\x0cR\x05value\"\xcf\x01\n\x10RaftSn\
    apshotData\x12&\n\x06region\x18\x01\x20\x01(\x0b2\x0e.metapb.RegionR\x06\
    region\x12\x1b\n\tfile_size\x18\x02\x20\x01(\x04R\x08fileSize\x12+\n\x04\
    data\x18\x03\x20\x03(\x0b2\x17.raft_serverpb.KeyValueR\x04data\x12\x18\n\
    \x07version\x18\x04\x20\x01(\x04R\x07version\x12/\n\x04meta\x18\x05\x20\
    \x01(\x0b2\x1b.raft_serverpb.SnapshotMetaR\x04meta\"F\n\nStoreIdent\x12\
    \x1d\n\ncluster_id\x18\x01\x20\x01(\x04R\tclusterId\x12\x19\n\x08store_i\
    d\x18\x02\x20\x01(\x04R\x07storeId\"b\n\x0eRaftLocalState\x121\n\nhard_s\
    tate\x18\x01\x20\x01(\x0b2\x12.eraftpb.HardStateR\thardState\x12\x1d\n\n\
    last_index\x18\x02\x20\x01(\x04R\tlastIndex\"\x81\x01\n\x0eRaftApplyStat\
    e\x12#\n\rapplied_index\x18\x01\x20\x01(\x04R\x0cappliedIndex\x12J\n\x0f\
    truncated_state\x18\x02\x20\x01(\x0b2!.raft_serverpb.RaftTruncatedStateR\
    \x0etruncatedState\"j\n\x10RegionLocalState\x12.\n\x05state\x18\x01\x20\
    \x01(\x0e2\x18.raft_serverpb.PeerStateR\x05state\x12&\n\x06region\x18\
    \x02\x20\x01(\x0b2\x0e.metapb.RegionR\x06region*4\n\tPeerState\x12\n\n\
    \x06Normal\x10\0\x12\x0c\n\x08Applying\x10\x01\x12\r\n\tTombstone\x10\
    \x02B\"\n\x18com.pingcap.tikv.kvproto\x98\xa7\x08\x01\xa0\xa7\x08\x01J\
    \xce\x17\n\x06\x12\x04\0\0T\x01\n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\
    \x01\x02\x12\x03\x01\x08\x15\n\t\n\x02\x03\0\x12\x03\x03\x07\x16\n\t\n\
    \x02\x03\x01\x12\x03\x04\x07\x15\n\t\n\x02\x03\x02\x12\x03\x05\x07\x18\n\
    \x08\n\x01\x08\x12\x03\x07\01\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\x07\01\n\
    \x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\x07\x07\x13\n\r\n\x06\x08\xe7\x07\0\
    \x02\0\x12\x03\x07\x07\x13\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\
    \x07\x07\x13\n\x0c\n\x05\x08\xe7\x07\0\x07\x12\x03\x07\x160\n\x08\n\x01\
    \x08\x12\x03\t\09\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\t\09\n\x0c\n\x05\
    \x08\xe7\x07\x01\x02\x12\x03\t\x071\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x03\t\x071\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\t\x080\n\x0c\n\
    \x05\x08\xe7\x07\x01\x03\x12\x03\t48\n\x08\n\x01\x08\x12\x03\n\0:\n\x0b\
    \n\x04\x08\xe7\x07\x02\x12\x03\n\0:\n\x0c\n\x05\x08\xe7\x07\x02\x02\x12\
    \x03\n\x072\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\n\x072\n\x0e\n\x07\
    \x08\xe7\x07\x02\x02\0\x01\x12\x03\n\x081\n\x0c\n\x05\x08\xe7\x07\x02\
    \x03\x12\x03\n59\n\n\n\x02\x04\0\x12\x04\x0c\0\x17\x01\n\n\n\x03\x04\0\
    \x01\x12\x03\x0c\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\r\x04\x19\n\r\n\
    \x05\x04\0\x02\0\x04\x12\x04\r\x04\x0c\x15\n\x0c\n\x05\x04\0\x02\0\x05\
    \x12\x03\r\x04\n\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\r\x0b\x14\n\x0c\n\
    \x05\x04\0\x02\0\x03\x12\x03\r\x17\x18\n\x0b\n\x04\x04\0\x02\x01\x12\x03\
    \x0e\x04\x1e\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x0e\x04\r\x19\n\x0c\n\
    \x05\x04\0\x02\x01\x06\x12\x03\x0e\x04\x0f\n\x0c\n\x05\x04\0\x02\x01\x01\
    \x12\x03\x0e\x10\x19\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x0e\x1c\x1d\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03\x0f\x04\x1c\n\r\n\x05\x04\0\x02\x02\x04\
    \x12\x04\x0f\x04\x0e\x1e\n\x0c\n\x05\x04\0\x02\x02\x06\x12\x03\x0f\x04\
    \x0f\n\x0c\n\x05\x04\0\x02\x02\x01\x12\x03\x0f\x10\x17\n\x0c\n\x05\x04\0\
    \x02\x02\x03\x12\x03\x0f\x1a\x1b\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x10\
    \x04\x20\n\r\n\x05\x04\0\x02\x03\x04\x12\x04\x10\x04\x0f\x1c\n\x0c\n\x05\
    \x04\0\x02\x03\x06\x12\x03\x10\x04\x13\n\x0c\n\x05\x04\0\x02\x03\x01\x12\
    \x03\x10\x14\x1b\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x10\x1e\x1f\n\x0b\
    \n\x04\x04\0\x02\x04\x12\x03\x11\x04(\n\r\n\x05\x04\0\x02\x04\x04\x12\
    \x04\x11\x04\x10\x20\n\x0c\n\x05\x04\0\x02\x04\x06\x12\x03\x11\x04\x16\n\
    \x0c\n\x05\x04\0\x02\x04\x01\x12\x03\x11\x17#\n\x0c\n\x05\x04\0\x02\x04\
    \x03\x12\x03\x11&'\nR\n\x04\x04\0\x02\x05\x12\x03\x13\x04\x1a\x1aE\x20tr\
    ue\x20means\x20to_peer\x20is\x20a\x20tombstone\x20peer\x20and\x20it\x20s\
    hould\x20remove\x20itself.\n\n\r\n\x05\x04\0\x02\x05\x04\x12\x04\x13\x04\
    \x11(\n\x0c\n\x05\x04\0\x02\x05\x05\x12\x03\x13\x04\x08\n\x0c\n\x05\x04\
    \0\x02\x05\x01\x12\x03\x13\t\x15\n\x0c\n\x05\x04\0\x02\x05\x03\x12\x03\
    \x13\x18\x19\n5\n\x04\x04\0\x02\x06\x12\x03\x15\x04\x18\x1a(\x20Region\
    \x20key\x20range\x20[start_key,\x20end_key).\n\n\r\n\x05\x04\0\x02\x06\
    \x04\x12\x04\x15\x04\x13\x1a\n\x0c\n\x05\x04\0\x02\x06\x05\x12\x03\x15\
    \x04\t\n\x0c\n\x05\x04\0\x02\x06\x01\x12\x03\x15\n\x13\n\x0c\n\x05\x04\0\
    \x02\x06\x03\x12\x03\x15\x16\x17\n\x0b\n\x04\x04\0\x02\x07\x12\x03\x16\
    \x04\x16\n\r\n\x05\x04\0\x02\x07\x04\x12\x04\x16\x04\x15\x18\n\x0c\n\x05\
    \x04\0\x02\x07\x05\x12\x03\x16\x04\t\n\x0c\n\x05\x04\0\x02\x07\x01\x12\
    \x03\x16\n\x11\n\x0c\n\x05\x04\0\x02\x07\x03\x12\x03\x16\x14\x15\n\n\n\
    \x02\x04\x01\x12\x04\x19\0\x1c\x01\n\n\n\x03\x04\x01\x01\x12\x03\x19\x08\
    \x1a\n\x0b\n\x04\x04\x01\x02\0\x12\x03\x1a\x04\x15\n\r\n\x05\x04\x01\x02\
    \0\x04\x12\x04\x1a\x04\x19\x1c\n\x0c\n\x05\x04\x01\x02\0\x05\x12\x03\x1a\
    \x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\x1a\x0b\x10\n\x0c\n\x05\x04\
    \x01\x02\0\x03\x12\x03\x1a\x13\x14\n\x0b\n\x04\x04\x01\x02\x01\x12\x03\
    \x1b\x04\x14\n\r\n\x05\x04\x01\x02\x01\x04\x12\x04\x1b\x04\x1a\x15\n\x0c\
    \n\x05\x04\x01\x02\x01\x05\x12\x03\x1b\x04\n\n\x0c\n\x05\x04\x01\x02\x01\
    \x01\x12\x03\x1b\x0b\x0f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03\x1b\x12\
    \x13\n\n\n\x02\x04\x02\x12\x04\x1e\0\"\x01\n\n\n\x03\x04\x02\x01\x12\x03\
    \x1e\x08\x16\n\x0b\n\x04\x04\x02\x02\0\x12\x03\x1f\x04\x12\n\r\n\x05\x04\
    \x02\x02\0\x04\x12\x04\x1f\x04\x1e\x18\n\x0c\n\x05\x04\x02\x02\0\x05\x12\
    \x03\x1f\x04\n\n\x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x1f\x0b\r\n\x0c\n\
    \x05\x04\x02\x02\0\x03\x12\x03\x1f\x10\x11\n\x0b\n\x04\x04\x02\x02\x01\
    \x12\x03\x20\x04\x14\n\r\n\x05\x04\x02\x02\x01\x04\x12\x04\x20\x04\x1f\
    \x12\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x20\x04\n\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03\x20\x0b\x0f\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03\x20\x12\x13\n\x0b\n\x04\x04\x02\x02\x02\x12\x03!\x04\x18\n\r\n\x05\
    \x04\x02\x02\x02\x04\x12\x04!\x04\x20\x14\n\x0c\n\x05\x04\x02\x02\x02\
    \x05\x12\x03!\x04\n\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03!\x0b\x13\n\
    \x0c\n\x05\x04\x02\x02\x02\x03\x12\x03!\x16\x17\n\n\n\x02\x04\x03\x12\
    \x04$\0&\x01\n\n\n\x03\x04\x03\x01\x12\x03$\x08\x14\n\x0b\n\x04\x04\x03\
    \x02\0\x12\x03%\x04)\n\x0c\n\x05\x04\x03\x02\0\x04\x12\x03%\x04\x0c\n\
    \x0c\n\x05\x04\x03\x02\0\x06\x12\x03%\r\x1b\n\x0c\n\x05\x04\x03\x02\0\
    \x01\x12\x03%\x1c$\n\x0c\n\x05\x04\x03\x02\0\x03\x12\x03%'(\n\n\n\x02\
    \x04\x04\x12\x04(\0+\x01\n\n\n\x03\x04\x04\x01\x12\x03(\x08\x15\n\x0b\n\
    \x04\x04\x04\x02\0\x12\x03)\x04\x1c\n\r\n\x05\x04\x04\x02\0\x04\x12\x04)\
    \x04(\x17\n\x0c\n\x05\x04\x04\x02\0\x06\x12\x03)\x04\x0f\n\x0c\n\x05\x04\
    \x04\x02\0\x01\x12\x03)\x10\x17\n\x0c\n\x05\x04\x04\x02\0\x03\x12\x03)\
    \x1a\x1b\n\x0b\n\x04\x04\x04\x02\x01\x12\x03*\x04\x13\n\r\n\x05\x04\x04\
    \x02\x01\x04\x12\x04*\x04)\x1c\n\x0c\n\x05\x04\x04\x02\x01\x05\x12\x03*\
    \x04\t\n\x0c\n\x05\x04\x04\x02\x01\x01\x12\x03*\n\x0e\n\x0c\n\x05\x04\
    \x04\x02\x01\x03\x12\x03*\x11\x12\n\t\n\x02\x04\x05\x12\x03-\0\x0f\n\n\n\
    \x03\x04\x05\x01\x12\x03-\x08\x0c\n\n\n\x02\x04\x06\x12\x04/\02\x01\n\n\
    \n\x03\x04\x06\x01\x12\x03/\x08\x10\n\x0b\n\x04\x04\x06\x02\0\x12\x030\
    \x04\x17\n\r\n\x05\x04\x06\x02\0\x04\x12\x040\x04/\x12\n\x0c\n\x05\x04\
    \x06\x02\0\x05\x12\x030\x04\t\n\x0c\n\x05\x04\x06\x02\0\x01\x12\x030\n\r\
    \n\x0c\n\x05\x04\x06\x02\0\x03\x12\x030\x15\x16\n\x0b\n\x04\x04\x06\x02\
    \x01\x12\x031\x04\x17\n\r\n\x05\x04\x06\x02\x01\x04\x12\x041\x040\x17\n\
    \x0c\n\x05\x04\x06\x02\x01\x05\x12\x031\x04\t\n\x0c\n\x05\x04\x06\x02\
    \x01\x01\x12\x031\n\x0f\n\x0c\n\x05\x04\x06\x02\x01\x03\x12\x031\x15\x16\
    \n\n\n\x02\x04\x07\x12\x044\0:\x01\n\n\n\x03\x04\x07\x01\x12\x034\x08\
    \x18\n\x0b\n\x04\x04\x07\x02\0\x12\x035\x04\x1d\n\r\n\x05\x04\x07\x02\0\
    \x04\x12\x045\x044\x1a\n\x0c\n\x05\x04\x07\x02\0\x06\x12\x035\x04\x11\n\
    \x0c\n\x05\x04\x07\x02\0\x01\x12\x035\x12\x18\n\x0c\n\x05\x04\x07\x02\0\
    \x03\x12\x035\x1b\x1c\n\x0b\n\x04\x04\x07\x02\x01\x12\x036\x04\x19\n\r\n\
    \x05\x04\x07\x02\x01\x04\x12\x046\x045\x1d\n\x0c\n\x05\x04\x07\x02\x01\
    \x05\x12\x036\x04\n\n\x0c\n\x05\x04\x07\x02\x01\x01\x12\x036\x0b\x14\n\
    \x0c\n\x05\x04\x07\x02\x01\x03\x12\x036\x17\x18\n\x0b\n\x04\x04\x07\x02\
    \x02\x12\x037\x04\x1f\n\x0c\n\x05\x04\x07\x02\x02\x04\x12\x037\x04\x0c\n\
    \x0c\n\x05\x04\x07\x02\x02\x06\x12\x037\r\x15\n\x0c\n\x05\x04\x07\x02\
    \x02\x01\x12\x037\x16\x1a\n\x0c\n\x05\x04\x07\x02\x02\x03\x12\x037\x1d\
    \x1e\n\x0b\n\x04\x04\x07\x02\x03\x12\x038\x04\x17\n\r\n\x05\x04\x07\x02\
    \x03\x04\x12\x048\x047\x1f\n\x0c\n\x05\x04\x07\x02\x03\x05\x12\x038\x04\
    \n\n\x0c\n\x05\x04\x07\x02\x03\x01\x12\x038\x0b\x12\n\x0c\n\x05\x04\x07\
    \x02\x03\x03\x12\x038\x15\x16\n\x0b\n\x04\x04\x07\x02\x04\x12\x039\x04\
    \x1a\n\r\n\x05\x04\x07\x02\x04\x04\x12\x049\x048\x17\n\x0c\n\x05\x04\x07\
    \x02\x04\x06\x12\x039\x04\x10\n\x0c\n\x05\x04\x07\x02\x04\x01\x12\x039\
    \x11\x15\n\x0c\n\x05\x04\x07\x02\x04\x03\x12\x039\x18\x19\n\n\n\x02\x04\
    \x08\x12\x04<\0?\x01\n\n\n\x03\x04\x08\x01\x12\x03<\x08\x12\n\x0b\n\x04\
    \x04\x08\x02\0\x12\x03=\x04\x1a\n\r\n\x05\x04\x08\x02\0\x04\x12\x04=\x04\
    <\x14\n\x0c\n\x05\x04\x08\x02\0\x05\x12\x03=\x04\n\n\x0c\n\x05\x04\x08\
    \x02\0\x01\x12\x03=\x0b\x15\n\x0c\n\x05\x04\x08\x02\0\x03\x12\x03=\x18\
    \x19\n\x0b\n\x04\x04\x08\x02\x01\x12\x03>\x04\x18\n\r\n\x05\x04\x08\x02\
    \x01\x04\x12\x04>\x04=\x1a\n\x0c\n\x05\x04\x08\x02\x01\x05\x12\x03>\x04\
    \n\n\x0c\n\x05\x04\x08\x02\x01\x01\x12\x03>\x0b\x13\n\x0c\n\x05\x04\x08\
    \x02\x01\x03\x12\x03>\x16\x17\n\n\n\x02\x04\t\x12\x04A\0D\x01\n\n\n\x03\
    \x04\t\x01\x12\x03A\x08\x16\n\x0b\n\x04\x04\t\x02\0\x12\x03B\x04%\n\r\n\
    \x05\x04\t\x02\0\x04\x12\x04B\x04A\x18\n\x0c\n\x05\x04\t\x02\0\x06\x12\
    \x03B\x04\x15\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03B\x16\x20\n\x0c\n\x05\
    \x04\t\x02\0\x03\x12\x03B#$\n\x0b\n\x04\x04\t\x02\x01\x12\x03C\x04\x1a\n\
    \r\n\x05\x04\t\x02\x01\x04\x12\x04C\x04B%\n\x0c\n\x05\x04\t\x02\x01\x05\
    \x12\x03C\x04\n\n\x0c\n\x05\x04\t\x02\x01\x01\x12\x03C\x0b\x15\n\x0c\n\
    \x05\x04\t\x02\x01\x03\x12\x03C\x18\x19\n\n\n\x02\x04\n\x12\x04F\0I\x01\
    \n\n\n\x03\x04\n\x01\x12\x03F\x08\x16\n\x0b\n\x04\x04\n\x02\0\x12\x03G\
    \x04\x1d\n\r\n\x05\x04\n\x02\0\x04\x12\x04G\x04F\x18\n\x0c\n\x05\x04\n\
    \x02\0\x05\x12\x03G\x04\n\n\x0c\n\x05\x04\n\x02\0\x01\x12\x03G\x0b\x18\n\
    \x0c\n\x05\x04\n\x02\0\x03\x12\x03G\x1b\x1c\n\x0b\n\x04\x04\n\x02\x01\
    \x12\x03H\x04+\n\r\n\x05\x04\n\x02\x01\x04\x12\x04H\x04G\x1d\n\x0c\n\x05\
    \x04\n\x02\x01\x06\x12\x03H\x04\x16\n\x0c\n\x05\x04\n\x02\x01\x01\x12\
    \x03H\x17&\n\x0c\n\x05\x04\n\x02\x01\x03\x12\x03H)*\n\n\n\x02\x05\0\x12\
    \x04K\0O\x01\n\n\n\x03\x05\0\x01\x12\x03K\x05\x0e\n\x0b\n\x04\x05\0\x02\
    \0\x12\x03L\x04\x0f\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03L\x04\n\n\x0c\n\
    \x05\x05\0\x02\0\x02\x12\x03L\r\x0e\n\x0b\n\x04\x05\0\x02\x01\x12\x03M\
    \x04\x11\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03M\x04\x0c\n\x0c\n\x05\x05\
    \0\x02\x01\x02\x12\x03M\x0f\x10\n\x0b\n\x04\x05\0\x02\x02\x12\x03N\x04\
    \x12\n\x0c\n\x05\x05\0\x02\x02\x01\x12\x03N\x04\r\n\x0c\n\x05\x05\0\x02\
    \x02\x02\x12\x03N\x10\x11\n\n\n\x02\x04\x0b\x12\x04Q\0T\x01\n\n\n\x03\
    \x04\x0b\x01\x12\x03Q\x08\x18\n\x0b\n\x04\x04\x0b\x02\0\x12\x03R\x04\x18\
    \n\r\n\x05\x04\x0b\x02\0\x04\x12\x04R\x04Q\x1a\n\x0c\n\x05\x04\x0b\x02\0\
    \x06\x12\x03R\x04\r\n\x0c\n\x05\x04\x0b\x02\0\x01\x12\x03R\x0e\x13\n\x0c\
    \n\x05\x04\x0b\x02\0\x03\x12\x03R\x16\x17\n\x0b\n\x04\x04\x0b\x02\x01\
    \x12\x03S\x04\x1d\n\r\n\x05\x04\x0b\x02\x01\x04\x12\x04S\x04R\x18\n\x0c\
    \n\x05\x04\x0b\x02\x01\x06\x12\x03S\x04\x11\n\x0c\n\x05\x04\x0b\x02\x01\
    \x01\x12\x03S\x12\x18\n\x0c\n\x05\x04\x0b\x02\x01\x03\x12\x03S\x1b\x1cb\
    \x06proto3\
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
