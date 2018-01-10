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
pub struct Mutation {
    // message fields
    pub op: Mutation_OP,
    pub key: ::std::vec::Vec<u8>,
    pub value: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
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
            instance.get(Mutation::new)
        }
    }

    // .importpb.Mutation.OP op = 1;

    pub fn clear_op(&mut self) {
        self.op = Mutation_OP::Put;
    }

    // Param is passed by value, moved
    pub fn set_op(&mut self, v: Mutation_OP) {
        self.op = v;
    }

    pub fn get_op(&self) -> Mutation_OP {
        self.op
    }

    fn get_op_for_reflect(&self) -> &Mutation_OP {
        &self.op
    }

    fn mut_op_for_reflect(&mut self) -> &mut Mutation_OP {
        &mut self.op
    }

    // bytes key = 2;

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

    // bytes value = 3;

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

impl ::protobuf::Message for Mutation {
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
                    let tmp = is.read_enum()?;
                    self.op = tmp;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.key)?;
                },
                3 => {
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
        if self.op != Mutation_OP::Put {
            my_size += ::protobuf::rt::enum_size(1, self.op);
        }
        if !self.key.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.key);
        }
        if !self.value.is_empty() {
            my_size += ::protobuf::rt::bytes_size(3, &self.value);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.op != Mutation_OP::Put {
            os.write_enum(1, self.op.value())?;
        }
        if !self.key.is_empty() {
            os.write_bytes(2, &self.key)?;
        }
        if !self.value.is_empty() {
            os.write_bytes(3, &self.value)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<Mutation_OP>>(
                    "op",
                    Mutation::get_op_for_reflect,
                    Mutation::mut_op_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "key",
                    Mutation::get_key_for_reflect,
                    Mutation::mut_key_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "value",
                    Mutation::get_value_for_reflect,
                    Mutation::mut_value_for_reflect,
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

impl ::std::fmt::Debug for Mutation {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Mutation {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum Mutation_OP {
    Put = 0,
}

impl ::protobuf::ProtobufEnum for Mutation_OP {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Mutation_OP> {
        match value {
            0 => ::std::option::Option::Some(Mutation_OP::Put),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [Mutation_OP] = &[
            Mutation_OP::Put,
        ];
        values
    }

    fn enum_descriptor_static(_: ::std::option::Option<Mutation_OP>) -> &'static ::protobuf::reflect::EnumDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::EnumDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                ::protobuf::reflect::EnumDescriptor::new("Mutation_OP", file_descriptor_proto())
            })
        }
    }
}

impl ::std::marker::Copy for Mutation_OP {
}

impl ::std::default::Default for Mutation_OP {
    fn default() -> Self {
        Mutation_OP::Put
    }
}

impl ::protobuf::reflect::ProtobufValue for Mutation_OP {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Enum(self.descriptor())
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteBatch {
    // message fields
    pub commit_ts: u64,
    pub mutations: ::protobuf::RepeatedField<Mutation>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteBatch {}

impl WriteBatch {
    pub fn new() -> WriteBatch {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteBatch {
        static mut instance: ::protobuf::lazy::Lazy<WriteBatch> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteBatch,
        };
        unsafe {
            instance.get(WriteBatch::new)
        }
    }

    // uint64 commit_ts = 1;

    pub fn clear_commit_ts(&mut self) {
        self.commit_ts = 0;
    }

    // Param is passed by value, moved
    pub fn set_commit_ts(&mut self, v: u64) {
        self.commit_ts = v;
    }

    pub fn get_commit_ts(&self) -> u64 {
        self.commit_ts
    }

    fn get_commit_ts_for_reflect(&self) -> &u64 {
        &self.commit_ts
    }

    fn mut_commit_ts_for_reflect(&mut self) -> &mut u64 {
        &mut self.commit_ts
    }

    // repeated .importpb.Mutation mutations = 2;

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

    fn get_mutations_for_reflect(&self) -> &::protobuf::RepeatedField<Mutation> {
        &self.mutations
    }

    fn mut_mutations_for_reflect(&mut self) -> &mut ::protobuf::RepeatedField<Mutation> {
        &mut self.mutations
    }
}

impl ::protobuf::Message for WriteBatch {
    fn is_initialized(&self) -> bool {
        for v in &self.mutations {
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
                    self.commit_ts = tmp;
                },
                2 => {
                    ::protobuf::rt::read_repeated_message_into(wire_type, is, &mut self.mutations)?;
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
        if self.commit_ts != 0 {
            my_size += ::protobuf::rt::value_size(1, self.commit_ts, ::protobuf::wire_format::WireTypeVarint);
        }
        for value in &self.mutations {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        };
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.commit_ts != 0 {
            os.write_uint64(1, self.commit_ts)?;
        }
        for v in &self.mutations {
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

impl ::protobuf::MessageStatic for WriteBatch {
    fn new() -> WriteBatch {
        WriteBatch::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteBatch>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "commit_ts",
                    WriteBatch::get_commit_ts_for_reflect,
                    WriteBatch::mut_commit_ts_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_repeated_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Mutation>>(
                    "mutations",
                    WriteBatch::get_mutations_for_reflect,
                    WriteBatch::mut_mutations_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteBatch>(
                    "WriteBatch",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteBatch {
    fn clear(&mut self) {
        self.clear_commit_ts();
        self.clear_mutations();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteBatch {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteBatch {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteOptions {
    // message fields
    pub flush: bool,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteOptions {}

impl WriteOptions {
    pub fn new() -> WriteOptions {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteOptions {
        static mut instance: ::protobuf::lazy::Lazy<WriteOptions> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteOptions,
        };
        unsafe {
            instance.get(WriteOptions::new)
        }
    }

    // bool flush = 1;

    pub fn clear_flush(&mut self) {
        self.flush = false;
    }

    // Param is passed by value, moved
    pub fn set_flush(&mut self, v: bool) {
        self.flush = v;
    }

    pub fn get_flush(&self) -> bool {
        self.flush
    }

    fn get_flush_for_reflect(&self) -> &bool {
        &self.flush
    }

    fn mut_flush_for_reflect(&mut self) -> &mut bool {
        &mut self.flush
    }
}

impl ::protobuf::Message for WriteOptions {
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
                    let tmp = is.read_bool()?;
                    self.flush = tmp;
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
        if self.flush != false {
            my_size += 2;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if self.flush != false {
            os.write_bool(1, self.flush)?;
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

impl ::protobuf::MessageStatic for WriteOptions {
    fn new() -> WriteOptions {
        WriteOptions::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteOptions>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBool>(
                    "flush",
                    WriteOptions::get_flush_for_reflect,
                    WriteOptions::mut_flush_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteOptions>(
                    "WriteOptions",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteOptions {
    fn clear(&mut self) {
        self.clear_flush();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteOptions {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteOptions {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteRequest {
    // message fields
    pub head: ::protobuf::SingularPtrField<WriteRequest_Head>,
    pub batch: ::protobuf::SingularPtrField<WriteBatch>,
    pub options: ::protobuf::SingularPtrField<WriteOptions>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteRequest {}

impl WriteRequest {
    pub fn new() -> WriteRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteRequest {
        static mut instance: ::protobuf::lazy::Lazy<WriteRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteRequest,
        };
        unsafe {
            instance.get(WriteRequest::new)
        }
    }

    // .importpb.WriteRequest.Head head = 1;

    pub fn clear_head(&mut self) {
        self.head.clear();
    }

    pub fn has_head(&self) -> bool {
        self.head.is_some()
    }

    // Param is passed by value, moved
    pub fn set_head(&mut self, v: WriteRequest_Head) {
        self.head = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_head(&mut self) -> &mut WriteRequest_Head {
        if self.head.is_none() {
            self.head.set_default();
        }
        self.head.as_mut().unwrap()
    }

    // Take field
    pub fn take_head(&mut self) -> WriteRequest_Head {
        self.head.take().unwrap_or_else(|| WriteRequest_Head::new())
    }

    pub fn get_head(&self) -> &WriteRequest_Head {
        self.head.as_ref().unwrap_or_else(|| WriteRequest_Head::default_instance())
    }

    fn get_head_for_reflect(&self) -> &::protobuf::SingularPtrField<WriteRequest_Head> {
        &self.head
    }

    fn mut_head_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<WriteRequest_Head> {
        &mut self.head
    }

    // .importpb.WriteBatch batch = 2;

    pub fn clear_batch(&mut self) {
        self.batch.clear();
    }

    pub fn has_batch(&self) -> bool {
        self.batch.is_some()
    }

    // Param is passed by value, moved
    pub fn set_batch(&mut self, v: WriteBatch) {
        self.batch = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_batch(&mut self) -> &mut WriteBatch {
        if self.batch.is_none() {
            self.batch.set_default();
        }
        self.batch.as_mut().unwrap()
    }

    // Take field
    pub fn take_batch(&mut self) -> WriteBatch {
        self.batch.take().unwrap_or_else(|| WriteBatch::new())
    }

    pub fn get_batch(&self) -> &WriteBatch {
        self.batch.as_ref().unwrap_or_else(|| WriteBatch::default_instance())
    }

    fn get_batch_for_reflect(&self) -> &::protobuf::SingularPtrField<WriteBatch> {
        &self.batch
    }

    fn mut_batch_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<WriteBatch> {
        &mut self.batch
    }

    // .importpb.WriteOptions options = 3;

    pub fn clear_options(&mut self) {
        self.options.clear();
    }

    pub fn has_options(&self) -> bool {
        self.options.is_some()
    }

    // Param is passed by value, moved
    pub fn set_options(&mut self, v: WriteOptions) {
        self.options = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_options(&mut self) -> &mut WriteOptions {
        if self.options.is_none() {
            self.options.set_default();
        }
        self.options.as_mut().unwrap()
    }

    // Take field
    pub fn take_options(&mut self) -> WriteOptions {
        self.options.take().unwrap_or_else(|| WriteOptions::new())
    }

    pub fn get_options(&self) -> &WriteOptions {
        self.options.as_ref().unwrap_or_else(|| WriteOptions::default_instance())
    }

    fn get_options_for_reflect(&self) -> &::protobuf::SingularPtrField<WriteOptions> {
        &self.options
    }

    fn mut_options_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<WriteOptions> {
        &mut self.options
    }
}

impl ::protobuf::Message for WriteRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.head {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.batch {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.options {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.head)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.batch)?;
                },
                3 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.options)?;
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
        if let Some(ref v) = self.head.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.batch.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.options.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.head.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.batch.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.options.as_ref() {
            os.write_tag(3, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for WriteRequest {
    fn new() -> WriteRequest {
        WriteRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<WriteRequest_Head>>(
                    "head",
                    WriteRequest::get_head_for_reflect,
                    WriteRequest::mut_head_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<WriteBatch>>(
                    "batch",
                    WriteRequest::get_batch_for_reflect,
                    WriteRequest::mut_batch_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<WriteOptions>>(
                    "options",
                    WriteRequest::get_options_for_reflect,
                    WriteRequest::mut_options_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteRequest>(
                    "WriteRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteRequest {
    fn clear(&mut self) {
        self.clear_head();
        self.clear_batch();
        self.clear_options();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteRequest_Head {
    // message fields
    pub uuid: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteRequest_Head {}

impl WriteRequest_Head {
    pub fn new() -> WriteRequest_Head {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteRequest_Head {
        static mut instance: ::protobuf::lazy::Lazy<WriteRequest_Head> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteRequest_Head,
        };
        unsafe {
            instance.get(WriteRequest_Head::new)
        }
    }

    // bytes uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }

    fn get_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }
}

impl ::protobuf::Message for WriteRequest_Head {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.uuid)?;
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
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.uuid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uuid.is_empty() {
            os.write_bytes(1, &self.uuid)?;
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

impl ::protobuf::MessageStatic for WriteRequest_Head {
    fn new() -> WriteRequest_Head {
        WriteRequest_Head::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteRequest_Head>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    WriteRequest_Head::get_uuid_for_reflect,
                    WriteRequest_Head::mut_uuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<WriteRequest_Head>(
                    "WriteRequest_Head",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteRequest_Head {
    fn clear(&mut self) {
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteRequest_Head {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteRequest_Head {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct WriteResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for WriteResponse {}

impl WriteResponse {
    pub fn new() -> WriteResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static WriteResponse {
        static mut instance: ::protobuf::lazy::Lazy<WriteResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const WriteResponse,
        };
        unsafe {
            instance.get(WriteResponse::new)
        }
    }
}

impl ::protobuf::Message for WriteResponse {
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

impl ::protobuf::MessageStatic for WriteResponse {
    fn new() -> WriteResponse {
        WriteResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<WriteResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<WriteResponse>(
                    "WriteResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for WriteResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for WriteResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for WriteResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CloseRequest {
    // message fields
    pub uuid: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CloseRequest {}

impl CloseRequest {
    pub fn new() -> CloseRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CloseRequest {
        static mut instance: ::protobuf::lazy::Lazy<CloseRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CloseRequest,
        };
        unsafe {
            instance.get(CloseRequest::new)
        }
    }

    // bytes uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }

    fn get_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }
}

impl ::protobuf::Message for CloseRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.uuid)?;
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
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.uuid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uuid.is_empty() {
            os.write_bytes(1, &self.uuid)?;
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

impl ::protobuf::MessageStatic for CloseRequest {
    fn new() -> CloseRequest {
        CloseRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<CloseRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    CloseRequest::get_uuid_for_reflect,
                    CloseRequest::mut_uuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<CloseRequest>(
                    "CloseRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CloseRequest {
    fn clear(&mut self) {
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CloseRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CloseRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct CloseResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for CloseResponse {}

impl CloseResponse {
    pub fn new() -> CloseResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static CloseResponse {
        static mut instance: ::protobuf::lazy::Lazy<CloseResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const CloseResponse,
        };
        unsafe {
            instance.get(CloseResponse::new)
        }
    }
}

impl ::protobuf::Message for CloseResponse {
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

impl ::protobuf::MessageStatic for CloseResponse {
    fn new() -> CloseResponse {
        CloseResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<CloseResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<CloseResponse>(
                    "CloseResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for CloseResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for CloseResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for CloseResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ImportRequest {
    // message fields
    pub uuid: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImportRequest {}

impl ImportRequest {
    pub fn new() -> ImportRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImportRequest {
        static mut instance: ::protobuf::lazy::Lazy<ImportRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImportRequest,
        };
        unsafe {
            instance.get(ImportRequest::new)
        }
    }

    // bytes uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }

    fn get_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }
}

impl ::protobuf::Message for ImportRequest {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.uuid)?;
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
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.uuid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uuid.is_empty() {
            os.write_bytes(1, &self.uuid)?;
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

impl ::protobuf::MessageStatic for ImportRequest {
    fn new() -> ImportRequest {
        ImportRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImportRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    ImportRequest::get_uuid_for_reflect,
                    ImportRequest::mut_uuid_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<ImportRequest>(
                    "ImportRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImportRequest {
    fn clear(&mut self) {
        self.clear_uuid();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImportRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImportRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct ImportResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for ImportResponse {}

impl ImportResponse {
    pub fn new() -> ImportResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static ImportResponse {
        static mut instance: ::protobuf::lazy::Lazy<ImportResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ImportResponse,
        };
        unsafe {
            instance.get(ImportResponse::new)
        }
    }
}

impl ::protobuf::Message for ImportResponse {
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

impl ::protobuf::MessageStatic for ImportResponse {
    fn new() -> ImportResponse {
        ImportResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<ImportResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<ImportResponse>(
                    "ImportResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for ImportResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for ImportResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for ImportResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct DeleteRequest {
    // message fields
    pub uuid: ::std::vec::Vec<u8>,
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

    // bytes uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }

    fn get_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.uuid)?;
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
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.uuid);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uuid.is_empty() {
            os.write_bytes(1, &self.uuid)?;
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
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    DeleteRequest::get_uuid_for_reflect,
                    DeleteRequest::mut_uuid_for_reflect,
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
        self.clear_uuid();
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
pub struct Range {
    // message fields
    pub start: ::std::vec::Vec<u8>,
    pub end: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for Range {}

impl Range {
    pub fn new() -> Range {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static Range {
        static mut instance: ::protobuf::lazy::Lazy<Range> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const Range,
        };
        unsafe {
            instance.get(Range::new)
        }
    }

    // bytes start = 1;

    pub fn clear_start(&mut self) {
        self.start.clear();
    }

    // Param is passed by value, moved
    pub fn set_start(&mut self, v: ::std::vec::Vec<u8>) {
        self.start = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_start(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.start
    }

    // Take field
    pub fn take_start(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start, ::std::vec::Vec::new())
    }

    pub fn get_start(&self) -> &[u8] {
        &self.start
    }

    fn get_start_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.start
    }

    fn mut_start_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.start
    }

    // bytes end = 2;

    pub fn clear_end(&mut self) {
        self.end.clear();
    }

    // Param is passed by value, moved
    pub fn set_end(&mut self, v: ::std::vec::Vec<u8>) {
        self.end = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_end(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.end
    }

    // Take field
    pub fn take_end(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end, ::std::vec::Vec::new())
    }

    pub fn get_end(&self) -> &[u8] {
        &self.end
    }

    fn get_end_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.end
    }

    fn mut_end_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.end
    }
}

impl ::protobuf::Message for Range {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.start)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.end)?;
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
        if !self.start.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.start);
        }
        if !self.end.is_empty() {
            my_size += ::protobuf::rt::bytes_size(2, &self.end);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.start.is_empty() {
            os.write_bytes(1, &self.start)?;
        }
        if !self.end.is_empty() {
            os.write_bytes(2, &self.end)?;
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

impl ::protobuf::MessageStatic for Range {
    fn new() -> Range {
        Range::new()
    }

    fn descriptor_static(_: ::std::option::Option<Range>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "start",
                    Range::get_start_for_reflect,
                    Range::mut_start_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "end",
                    Range::get_end_for_reflect,
                    Range::mut_end_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<Range>(
                    "Range",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for Range {
    fn clear(&mut self) {
        self.clear_start();
        self.clear_end();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Range {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Range {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct SSTMeta {
    // message fields
    pub uuid: ::std::vec::Vec<u8>,
    pub range: ::protobuf::SingularPtrField<Range>,
    pub crc32: u32,
    pub length: u64,
    pub cf_name: ::std::string::String,
    pub region_id: u64,
    pub region_epoch: ::protobuf::SingularPtrField<super::metapb::RegionEpoch>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for SSTMeta {}

impl SSTMeta {
    pub fn new() -> SSTMeta {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static SSTMeta {
        static mut instance: ::protobuf::lazy::Lazy<SSTMeta> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const SSTMeta,
        };
        unsafe {
            instance.get(SSTMeta::new)
        }
    }

    // bytes uuid = 1;

    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }

    // Param is passed by value, moved
    pub fn set_uuid(&mut self, v: ::std::vec::Vec<u8>) {
        self.uuid = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_uuid(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // Take field
    pub fn take_uuid(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }

    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }

    fn get_uuid_for_reflect(&self) -> &::std::vec::Vec<u8> {
        &self.uuid
    }

    fn mut_uuid_for_reflect(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.uuid
    }

    // .importpb.Range range = 2;

    pub fn clear_range(&mut self) {
        self.range.clear();
    }

    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }

    // Param is passed by value, moved
    pub fn set_range(&mut self, v: Range) {
        self.range = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_range(&mut self) -> &mut Range {
        if self.range.is_none() {
            self.range.set_default();
        }
        self.range.as_mut().unwrap()
    }

    // Take field
    pub fn take_range(&mut self) -> Range {
        self.range.take().unwrap_or_else(|| Range::new())
    }

    pub fn get_range(&self) -> &Range {
        self.range.as_ref().unwrap_or_else(|| Range::default_instance())
    }

    fn get_range_for_reflect(&self) -> &::protobuf::SingularPtrField<Range> {
        &self.range
    }

    fn mut_range_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<Range> {
        &mut self.range
    }

    // uint32 crc32 = 3;

    pub fn clear_crc32(&mut self) {
        self.crc32 = 0;
    }

    // Param is passed by value, moved
    pub fn set_crc32(&mut self, v: u32) {
        self.crc32 = v;
    }

    pub fn get_crc32(&self) -> u32 {
        self.crc32
    }

    fn get_crc32_for_reflect(&self) -> &u32 {
        &self.crc32
    }

    fn mut_crc32_for_reflect(&mut self) -> &mut u32 {
        &mut self.crc32
    }

    // uint64 length = 4;

    pub fn clear_length(&mut self) {
        self.length = 0;
    }

    // Param is passed by value, moved
    pub fn set_length(&mut self, v: u64) {
        self.length = v;
    }

    pub fn get_length(&self) -> u64 {
        self.length
    }

    fn get_length_for_reflect(&self) -> &u64 {
        &self.length
    }

    fn mut_length_for_reflect(&mut self) -> &mut u64 {
        &mut self.length
    }

    // string cf_name = 5;

    pub fn clear_cf_name(&mut self) {
        self.cf_name.clear();
    }

    // Param is passed by value, moved
    pub fn set_cf_name(&mut self, v: ::std::string::String) {
        self.cf_name = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_cf_name(&mut self) -> &mut ::std::string::String {
        &mut self.cf_name
    }

    // Take field
    pub fn take_cf_name(&mut self) -> ::std::string::String {
        ::std::mem::replace(&mut self.cf_name, ::std::string::String::new())
    }

    pub fn get_cf_name(&self) -> &str {
        &self.cf_name
    }

    fn get_cf_name_for_reflect(&self) -> &::std::string::String {
        &self.cf_name
    }

    fn mut_cf_name_for_reflect(&mut self) -> &mut ::std::string::String {
        &mut self.cf_name
    }

    // uint64 region_id = 6;

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

    // .metapb.RegionEpoch region_epoch = 7;

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
}

impl ::protobuf::Message for SSTMeta {
    fn is_initialized(&self) -> bool {
        for v in &self.range {
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
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.uuid)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.range)?;
                },
                3 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint32()?;
                    self.crc32 = tmp;
                },
                4 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.length = tmp;
                },
                5 => {
                    ::protobuf::rt::read_singular_proto3_string_into(wire_type, is, &mut self.cf_name)?;
                },
                6 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.region_id = tmp;
                },
                7 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.region_epoch)?;
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
        if !self.uuid.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.uuid);
        }
        if let Some(ref v) = self.range.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if self.crc32 != 0 {
            my_size += ::protobuf::rt::value_size(3, self.crc32, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.length != 0 {
            my_size += ::protobuf::rt::value_size(4, self.length, ::protobuf::wire_format::WireTypeVarint);
        }
        if !self.cf_name.is_empty() {
            my_size += ::protobuf::rt::string_size(5, &self.cf_name);
        }
        if self.region_id != 0 {
            my_size += ::protobuf::rt::value_size(6, self.region_id, ::protobuf::wire_format::WireTypeVarint);
        }
        if let Some(ref v) = self.region_epoch.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if !self.uuid.is_empty() {
            os.write_bytes(1, &self.uuid)?;
        }
        if let Some(ref v) = self.range.as_ref() {
            os.write_tag(2, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if self.crc32 != 0 {
            os.write_uint32(3, self.crc32)?;
        }
        if self.length != 0 {
            os.write_uint64(4, self.length)?;
        }
        if !self.cf_name.is_empty() {
            os.write_string(5, &self.cf_name)?;
        }
        if self.region_id != 0 {
            os.write_uint64(6, self.region_id)?;
        }
        if let Some(ref v) = self.region_epoch.as_ref() {
            os.write_tag(7, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for SSTMeta {
    fn new() -> SSTMeta {
        SSTMeta::new()
    }

    fn descriptor_static(_: ::std::option::Option<SSTMeta>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "uuid",
                    SSTMeta::get_uuid_for_reflect,
                    SSTMeta::mut_uuid_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<Range>>(
                    "range",
                    SSTMeta::get_range_for_reflect,
                    SSTMeta::mut_range_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint32>(
                    "crc32",
                    SSTMeta::get_crc32_for_reflect,
                    SSTMeta::mut_crc32_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "length",
                    SSTMeta::get_length_for_reflect,
                    SSTMeta::mut_length_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeString>(
                    "cf_name",
                    SSTMeta::get_cf_name_for_reflect,
                    SSTMeta::mut_cf_name_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                    "region_id",
                    SSTMeta::get_region_id_for_reflect,
                    SSTMeta::mut_region_id_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::metapb::RegionEpoch>>(
                    "region_epoch",
                    SSTMeta::get_region_epoch_for_reflect,
                    SSTMeta::mut_region_epoch_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<SSTMeta>(
                    "SSTMeta",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for SSTMeta {
    fn clear(&mut self) {
        self.clear_uuid();
        self.clear_range();
        self.clear_crc32();
        self.clear_length();
        self.clear_cf_name();
        self.clear_region_id();
        self.clear_region_epoch();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for SSTMeta {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for SSTMeta {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UploadRequest {
    // message fields
    pub meta: ::protobuf::SingularPtrField<SSTMeta>,
    pub data: ::std::vec::Vec<u8>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UploadRequest {}

impl UploadRequest {
    pub fn new() -> UploadRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UploadRequest {
        static mut instance: ::protobuf::lazy::Lazy<UploadRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UploadRequest,
        };
        unsafe {
            instance.get(UploadRequest::new)
        }
    }

    // .importpb.SSTMeta meta = 1;

    pub fn clear_meta(&mut self) {
        self.meta.clear();
    }

    pub fn has_meta(&self) -> bool {
        self.meta.is_some()
    }

    // Param is passed by value, moved
    pub fn set_meta(&mut self, v: SSTMeta) {
        self.meta = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_meta(&mut self) -> &mut SSTMeta {
        if self.meta.is_none() {
            self.meta.set_default();
        }
        self.meta.as_mut().unwrap()
    }

    // Take field
    pub fn take_meta(&mut self) -> SSTMeta {
        self.meta.take().unwrap_or_else(|| SSTMeta::new())
    }

    pub fn get_meta(&self) -> &SSTMeta {
        self.meta.as_ref().unwrap_or_else(|| SSTMeta::default_instance())
    }

    fn get_meta_for_reflect(&self) -> &::protobuf::SingularPtrField<SSTMeta> {
        &self.meta
    }

    fn mut_meta_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SSTMeta> {
        &mut self.meta
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

impl ::protobuf::Message for UploadRequest {
    fn is_initialized(&self) -> bool {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.meta)?;
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
        if let Some(ref v) = self.meta.as_ref() {
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
        if let Some(ref v) = self.meta.as_ref() {
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

impl ::protobuf::MessageStatic for UploadRequest {
    fn new() -> UploadRequest {
        UploadRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<UploadRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SSTMeta>>(
                    "meta",
                    UploadRequest::get_meta_for_reflect,
                    UploadRequest::mut_meta_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                    "data",
                    UploadRequest::get_data_for_reflect,
                    UploadRequest::mut_data_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<UploadRequest>(
                    "UploadRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UploadRequest {
    fn clear(&mut self) {
        self.clear_meta();
        self.clear_data();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UploadRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UploadRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct UploadResponse {
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for UploadResponse {}

impl UploadResponse {
    pub fn new() -> UploadResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static UploadResponse {
        static mut instance: ::protobuf::lazy::Lazy<UploadResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const UploadResponse,
        };
        unsafe {
            instance.get(UploadResponse::new)
        }
    }
}

impl ::protobuf::Message for UploadResponse {
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

impl ::protobuf::MessageStatic for UploadResponse {
    fn new() -> UploadResponse {
        UploadResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<UploadResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let fields = ::std::vec::Vec::new();
                ::protobuf::reflect::MessageDescriptor::new::<UploadResponse>(
                    "UploadResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for UploadResponse {
    fn clear(&mut self) {
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for UploadResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for UploadResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngestRequest {
    // message fields
    pub context: ::protobuf::SingularPtrField<super::kvrpcpb::Context>,
    pub sst: ::protobuf::SingularPtrField<SSTMeta>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngestRequest {}

impl IngestRequest {
    pub fn new() -> IngestRequest {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngestRequest {
        static mut instance: ::protobuf::lazy::Lazy<IngestRequest> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngestRequest,
        };
        unsafe {
            instance.get(IngestRequest::new)
        }
    }

    // .kvrpcpb.Context context = 1;

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
    pub fn mut_context(&mut self) -> &mut super::kvrpcpb::Context {
        if self.context.is_none() {
            self.context.set_default();
        }
        self.context.as_mut().unwrap()
    }

    // Take field
    pub fn take_context(&mut self) -> super::kvrpcpb::Context {
        self.context.take().unwrap_or_else(|| super::kvrpcpb::Context::new())
    }

    pub fn get_context(&self) -> &super::kvrpcpb::Context {
        self.context.as_ref().unwrap_or_else(|| super::kvrpcpb::Context::default_instance())
    }

    fn get_context_for_reflect(&self) -> &::protobuf::SingularPtrField<super::kvrpcpb::Context> {
        &self.context
    }

    fn mut_context_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<super::kvrpcpb::Context> {
        &mut self.context
    }

    // .importpb.SSTMeta sst = 2;

    pub fn clear_sst(&mut self) {
        self.sst.clear();
    }

    pub fn has_sst(&self) -> bool {
        self.sst.is_some()
    }

    // Param is passed by value, moved
    pub fn set_sst(&mut self, v: SSTMeta) {
        self.sst = ::protobuf::SingularPtrField::some(v);
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_sst(&mut self) -> &mut SSTMeta {
        if self.sst.is_none() {
            self.sst.set_default();
        }
        self.sst.as_mut().unwrap()
    }

    // Take field
    pub fn take_sst(&mut self) -> SSTMeta {
        self.sst.take().unwrap_or_else(|| SSTMeta::new())
    }

    pub fn get_sst(&self) -> &SSTMeta {
        self.sst.as_ref().unwrap_or_else(|| SSTMeta::default_instance())
    }

    fn get_sst_for_reflect(&self) -> &::protobuf::SingularPtrField<SSTMeta> {
        &self.sst
    }

    fn mut_sst_for_reflect(&mut self) -> &mut ::protobuf::SingularPtrField<SSTMeta> {
        &mut self.sst
    }
}

impl ::protobuf::Message for IngestRequest {
    fn is_initialized(&self) -> bool {
        for v in &self.context {
            if !v.is_initialized() {
                return false;
            }
        };
        for v in &self.sst {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.context)?;
                },
                2 => {
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.sst)?;
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
        if let Some(ref v) = self.context.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        if let Some(ref v) = self.sst.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.context.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
            os.write_raw_varint32(v.get_cached_size())?;
            v.write_to_with_cached_sizes(os)?;
        }
        if let Some(ref v) = self.sst.as_ref() {
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

impl ::protobuf::MessageStatic for IngestRequest {
    fn new() -> IngestRequest {
        IngestRequest::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngestRequest>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::kvrpcpb::Context>>(
                    "context",
                    IngestRequest::get_context_for_reflect,
                    IngestRequest::mut_context_for_reflect,
                ));
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<SSTMeta>>(
                    "sst",
                    IngestRequest::get_sst_for_reflect,
                    IngestRequest::mut_sst_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngestRequest>(
                    "IngestRequest",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngestRequest {
    fn clear(&mut self) {
        self.clear_context();
        self.clear_sst();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngestRequest {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngestRequest {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct IngestResponse {
    // message fields
    pub error: ::protobuf::SingularPtrField<super::errorpb::Error>,
    // special fields
    unknown_fields: ::protobuf::UnknownFields,
    cached_size: ::protobuf::CachedSize,
}

// see codegen.rs for the explanation why impl Sync explicitly
unsafe impl ::std::marker::Sync for IngestResponse {}

impl IngestResponse {
    pub fn new() -> IngestResponse {
        ::std::default::Default::default()
    }

    pub fn default_instance() -> &'static IngestResponse {
        static mut instance: ::protobuf::lazy::Lazy<IngestResponse> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const IngestResponse,
        };
        unsafe {
            instance.get(IngestResponse::new)
        }
    }

    // .errorpb.Error error = 1;

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
        }
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
}

impl ::protobuf::Message for IngestResponse {
    fn is_initialized(&self) -> bool {
        for v in &self.error {
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
                    ::protobuf::rt::read_singular_message_into(wire_type, is, &mut self.error)?;
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
        if let Some(ref v) = self.error.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint32_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream) -> ::protobuf::ProtobufResult<()> {
        if let Some(ref v) = self.error.as_ref() {
            os.write_tag(1, ::protobuf::wire_format::WireTypeLengthDelimited)?;
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

impl ::protobuf::MessageStatic for IngestResponse {
    fn new() -> IngestResponse {
        IngestResponse::new()
    }

    fn descriptor_static(_: ::std::option::Option<IngestResponse>) -> &'static ::protobuf::reflect::MessageDescriptor {
        static mut descriptor: ::protobuf::lazy::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::lazy::Lazy {
            lock: ::protobuf::lazy::ONCE_INIT,
            ptr: 0 as *const ::protobuf::reflect::MessageDescriptor,
        };
        unsafe {
            descriptor.get(|| {
                let mut fields = ::std::vec::Vec::new();
                fields.push(::protobuf::reflect::accessor::make_singular_ptr_field_accessor::<_, ::protobuf::types::ProtobufTypeMessage<super::errorpb::Error>>(
                    "error",
                    IngestResponse::get_error_for_reflect,
                    IngestResponse::mut_error_for_reflect,
                ));
                ::protobuf::reflect::MessageDescriptor::new::<IngestResponse>(
                    "IngestResponse",
                    fields,
                    file_descriptor_proto()
                )
            })
        }
    }
}

impl ::protobuf::Clear for IngestResponse {
    fn clear(&mut self) {
        self.clear_error();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for IngestResponse {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for IngestResponse {
    fn as_ref(&self) -> ::protobuf::reflect::ProtobufValueRef {
        ::protobuf::reflect::ProtobufValueRef::Message(self)
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0eimportpb.proto\x12\x08importpb\x1a\x0cmetapb.proto\x1a\rerrorpb.pr\
    oto\x1a\rkvrpcpb.proto\x1a\x14gogoproto/gogo.proto\"h\n\x08Mutation\x12%\
    \n\x02op\x18\x01\x20\x01(\x0e2\x15.importpb.Mutation.OPR\x02op\x12\x10\n\
    \x03key\x18\x02\x20\x01(\x0cR\x03key\x12\x14\n\x05value\x18\x03\x20\x01(\
    \x0cR\x05value\"\r\n\x02OP\x12\x07\n\x03Put\x10\0\"[\n\nWriteBatch\x12\
    \x1b\n\tcommit_ts\x18\x01\x20\x01(\x04R\x08commitTs\x120\n\tmutations\
    \x18\x02\x20\x03(\x0b2\x12.importpb.MutationR\tmutations\"$\n\x0cWriteOp\
    tions\x12\x14\n\x05flush\x18\x01\x20\x01(\x08R\x05flush\"\xb9\x01\n\x0cW\
    riteRequest\x12/\n\x04head\x18\x01\x20\x01(\x0b2\x1b.importpb.WriteReque\
    st.HeadR\x04head\x12*\n\x05batch\x18\x02\x20\x01(\x0b2\x14.importpb.Writ\
    eBatchR\x05batch\x120\n\x07options\x18\x03\x20\x01(\x0b2\x16.importpb.Wr\
    iteOptionsR\x07options\x1a\x1a\n\x04Head\x12\x12\n\x04uuid\x18\x01\x20\
    \x01(\x0cR\x04uuid\"\x0f\n\rWriteResponse\"\"\n\x0cCloseRequest\x12\x12\
    \n\x04uuid\x18\x01\x20\x01(\x0cR\x04uuid\"\x0f\n\rCloseResponse\"#\n\rIm\
    portRequest\x12\x12\n\x04uuid\x18\x01\x20\x01(\x0cR\x04uuid\"\x10\n\x0eI\
    mportResponse\"#\n\rDeleteRequest\x12\x12\n\x04uuid\x18\x01\x20\x01(\x0c\
    R\x04uuid\"\x10\n\x0eDeleteResponse\"/\n\x05Range\x12\x14\n\x05start\x18\
    \x01\x20\x01(\x0cR\x05start\x12\x10\n\x03end\x18\x02\x20\x01(\x0cR\x03en\
    d\"\xe0\x01\n\x07SSTMeta\x12\x12\n\x04uuid\x18\x01\x20\x01(\x0cR\x04uuid\
    \x12%\n\x05range\x18\x02\x20\x01(\x0b2\x0f.importpb.RangeR\x05range\x12\
    \x14\n\x05crc32\x18\x03\x20\x01(\rR\x05crc32\x12\x16\n\x06length\x18\x04\
    \x20\x01(\x04R\x06length\x12\x17\n\x07cf_name\x18\x05\x20\x01(\tR\x06cfN\
    ame\x12\x1b\n\tregion_id\x18\x06\x20\x01(\x04R\x08regionId\x126\n\x0creg\
    ion_epoch\x18\x07\x20\x01(\x0b2\x13.metapb.RegionEpochR\x0bregionEpoch\"\
    J\n\rUploadRequest\x12%\n\x04meta\x18\x01\x20\x01(\x0b2\x11.importpb.SST\
    MetaR\x04meta\x12\x12\n\x04data\x18\x02\x20\x01(\x0cR\x04data\"\x10\n\
    \x0eUploadResponse\"`\n\rIngestRequest\x12*\n\x07context\x18\x01\x20\x01\
    (\x0b2\x10.kvrpcpb.ContextR\x07context\x12#\n\x03sst\x18\x02\x20\x01(\
    \x0b2\x11.importpb.SSTMetaR\x03sst\"6\n\x0eIngestResponse\x12$\n\x05erro\
    r\x18\x01\x20\x01(\x0b2\x0e.errorpb.ErrorR\x05error2\x82\x02\n\x08Import\
    KV\x12<\n\x05Write\x12\x16.importpb.WriteRequest\x1a\x17.importpb.WriteR\
    esponse\"\0(\x01\x12:\n\x05Close\x12\x16.importpb.CloseRequest\x1a\x17.i\
    mportpb.CloseResponse\"\0\x12=\n\x06Import\x12\x17.importpb.ImportReques\
    t\x1a\x18.importpb.ImportResponse\"\0\x12=\n\x06Delete\x12\x17.importpb.\
    DeleteRequest\x1a\x18.importpb.DeleteResponse\"\02\x8b\x01\n\tImportSST\
    \x12?\n\x06Upload\x12\x17.importpb.UploadRequest\x1a\x18.importpb.Upload\
    Response\"\0(\x01\x12=\n\x06Ingest\x12\x17.importpb.IngestRequest\x1a\
    \x18.importpb.IngestResponse\"\0B&\n\x18com.pingcap.tikv.kvproto\xe0\xe2\
    \x1e\x01\xd0\xe2\x1e\x01\xc8\xe2\x1e\x01J\x90\x19\n\x06\x12\x04\0\0l\x01\
    \n\x08\n\x01\x0c\x12\x03\0\0\x12\n\x08\n\x01\x02\x12\x03\x02\x08\x10\n\t\
    \n\x02\x03\0\x12\x03\x04\x07\x15\n\t\n\x02\x03\x01\x12\x03\x05\x07\x16\n\
    \t\n\x02\x03\x02\x12\x03\x06\x07\x16\n\t\n\x02\x03\x03\x12\x03\x07\x07\
    \x1d\n\x08\n\x01\x08\x12\x03\t\0$\n\x0b\n\x04\x08\xe7\x07\0\x12\x03\t\0$\
    \n\x0c\n\x05\x08\xe7\x07\0\x02\x12\x03\t\x07\x1c\n\r\n\x06\x08\xe7\x07\0\
    \x02\0\x12\x03\t\x07\x1c\n\x0e\n\x07\x08\xe7\x07\0\x02\0\x01\x12\x03\t\
    \x08\x1b\n\x0c\n\x05\x08\xe7\x07\0\x03\x12\x03\t\x1f#\n\x08\n\x01\x08\
    \x12\x03\n\0(\n\x0b\n\x04\x08\xe7\x07\x01\x12\x03\n\0(\n\x0c\n\x05\x08\
    \xe7\x07\x01\x02\x12\x03\n\x07\x20\n\r\n\x06\x08\xe7\x07\x01\x02\0\x12\
    \x03\n\x07\x20\n\x0e\n\x07\x08\xe7\x07\x01\x02\0\x01\x12\x03\n\x08\x1f\n\
    \x0c\n\x05\x08\xe7\x07\x01\x03\x12\x03\n#'\n\x08\n\x01\x08\x12\x03\x0b\0\
    *\n\x0b\n\x04\x08\xe7\x07\x02\x12\x03\x0b\0*\n\x0c\n\x05\x08\xe7\x07\x02\
    \x02\x12\x03\x0b\x07\"\n\r\n\x06\x08\xe7\x07\x02\x02\0\x12\x03\x0b\x07\"\
    \n\x0e\n\x07\x08\xe7\x07\x02\x02\0\x01\x12\x03\x0b\x08!\n\x0c\n\x05\x08\
    \xe7\x07\x02\x03\x12\x03\x0b%)\n\x08\n\x01\x08\x12\x03\r\01\n\x0b\n\x04\
    \x08\xe7\x07\x03\x12\x03\r\01\n\x0c\n\x05\x08\xe7\x07\x03\x02\x12\x03\r\
    \x07\x13\n\r\n\x06\x08\xe7\x07\x03\x02\0\x12\x03\r\x07\x13\n\x0e\n\x07\
    \x08\xe7\x07\x03\x02\0\x01\x12\x03\r\x07\x13\n\x0c\n\x05\x08\xe7\x07\x03\
    \x07\x12\x03\r\x160\n\n\n\x02\x06\0\x12\x04\x0f\0\x14\x01\n\n\n\x03\x06\
    \0\x01\x12\x03\x0f\x08\x10\n\x0b\n\x04\x06\0\x02\0\x12\x03\x10\x04=\n\
    \x0c\n\x05\x06\0\x02\0\x01\x12\x03\x10\x08\r\n\x0c\n\x05\x06\0\x02\0\x05\
    \x12\x03\x10\x0e\x14\n\x0c\n\x05\x06\0\x02\0\x02\x12\x03\x10\x15!\n\x0c\
    \n\x05\x06\0\x02\0\x03\x12\x03\x10,9\n\x0b\n\x04\x06\0\x02\x01\x12\x03\
    \x11\x046\n\x0c\n\x05\x06\0\x02\x01\x01\x12\x03\x11\x08\r\n\x0c\n\x05\
    \x06\0\x02\x01\x02\x12\x03\x11\x0e\x1a\n\x0c\n\x05\x06\0\x02\x01\x03\x12\
    \x03\x11%2\n\x0b\n\x04\x06\0\x02\x02\x12\x03\x12\x049\n\x0c\n\x05\x06\0\
    \x02\x02\x01\x12\x03\x12\x08\x0e\n\x0c\n\x05\x06\0\x02\x02\x02\x12\x03\
    \x12\x0f\x1c\n\x0c\n\x05\x06\0\x02\x02\x03\x12\x03\x12'5\n\x0b\n\x04\x06\
    \0\x02\x03\x12\x03\x13\x049\n\x0c\n\x05\x06\0\x02\x03\x01\x12\x03\x13\
    \x08\x0e\n\x0c\n\x05\x06\0\x02\x03\x02\x12\x03\x13\x0f\x1c\n\x0c\n\x05\
    \x06\0\x02\x03\x03\x12\x03\x13'5\n\n\n\x02\x06\x01\x12\x04\x16\0\x19\x01\
    \n\n\n\x03\x06\x01\x01\x12\x03\x16\x08\x11\n\x0b\n\x04\x06\x01\x02\0\x12\
    \x03\x17\x04@\n\x0c\n\x05\x06\x01\x02\0\x01\x12\x03\x17\x08\x0e\n\x0c\n\
    \x05\x06\x01\x02\0\x05\x12\x03\x17\x0f\x15\n\x0c\n\x05\x06\x01\x02\0\x02\
    \x12\x03\x17\x16#\n\x0c\n\x05\x06\x01\x02\0\x03\x12\x03\x17.<\n\x0b\n\
    \x04\x06\x01\x02\x01\x12\x03\x18\x049\n\x0c\n\x05\x06\x01\x02\x01\x01\
    \x12\x03\x18\x08\x0e\n\x0c\n\x05\x06\x01\x02\x01\x02\x12\x03\x18\x0f\x1c\
    \n\x0c\n\x05\x06\x01\x02\x01\x03\x12\x03\x18'5\n\n\n\x02\x04\0\x12\x04\
    \x1b\0\"\x01\n\n\n\x03\x04\0\x01\x12\x03\x1b\x08\x10\n\x0c\n\x04\x04\0\
    \x04\0\x12\x04\x1c\x04\x1e\x05\n\x0c\n\x05\x04\0\x04\0\x01\x12\x03\x1c\t\
    \x0b\n\r\n\x06\x04\0\x04\0\x02\0\x12\x03\x1d\x08\x10\n\x0e\n\x07\x04\0\
    \x04\0\x02\0\x01\x12\x03\x1d\x08\x0b\n\x0e\n\x07\x04\0\x04\0\x02\0\x02\
    \x12\x03\x1d\x0e\x0f\n\x0b\n\x04\x04\0\x02\0\x12\x03\x1f\x04\x0e\n\r\n\
    \x05\x04\0\x02\0\x04\x12\x04\x1f\x04\x1e\x05\n\x0c\n\x05\x04\0\x02\0\x06\
    \x12\x03\x1f\x04\x06\n\x0c\n\x05\x04\0\x02\0\x01\x12\x03\x1f\x07\t\n\x0c\
    \n\x05\x04\0\x02\0\x03\x12\x03\x1f\x0c\r\n\x0b\n\x04\x04\0\x02\x01\x12\
    \x03\x20\x04\x12\n\r\n\x05\x04\0\x02\x01\x04\x12\x04\x20\x04\x1f\x0e\n\
    \x0c\n\x05\x04\0\x02\x01\x05\x12\x03\x20\x04\t\n\x0c\n\x05\x04\0\x02\x01\
    \x01\x12\x03\x20\n\r\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x20\x10\x11\n\
    \x0b\n\x04\x04\0\x02\x02\x12\x03!\x04\x14\n\r\n\x05\x04\0\x02\x02\x04\
    \x12\x04!\x04\x20\x12\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03!\x04\t\n\x0c\
    \n\x05\x04\0\x02\x02\x01\x12\x03!\n\x0f\n\x0c\n\x05\x04\0\x02\x02\x03\
    \x12\x03!\x12\x13\n\n\n\x02\x04\x01\x12\x04$\0'\x01\n\n\n\x03\x04\x01\
    \x01\x12\x03$\x08\x12\n\x0b\n\x04\x04\x01\x02\0\x12\x03%\x04\x19\n\r\n\
    \x05\x04\x01\x02\0\x04\x12\x04%\x04$\x14\n\x0c\n\x05\x04\x01\x02\0\x05\
    \x12\x03%\x04\n\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03%\x0b\x14\n\x0c\n\
    \x05\x04\x01\x02\0\x03\x12\x03%\x17\x18\n\x0b\n\x04\x04\x01\x02\x01\x12\
    \x03&\x04$\n\x0c\n\x05\x04\x01\x02\x01\x04\x12\x03&\x04\x0c\n\x0c\n\x05\
    \x04\x01\x02\x01\x06\x12\x03&\r\x15\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\
    \x03&\x16\x1f\n\x0c\n\x05\x04\x01\x02\x01\x03\x12\x03&\"#\n\n\n\x02\x04\
    \x02\x12\x04)\0+\x01\n\n\n\x03\x04\x02\x01\x12\x03)\x08\x14\n\x0b\n\x04\
    \x04\x02\x02\0\x12\x03*\x04\x13\n\r\n\x05\x04\x02\x02\0\x04\x12\x04*\x04\
    )\x16\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03*\x04\x08\n\x0c\n\x05\x04\x02\
    \x02\0\x01\x12\x03*\t\x0e\n\x0c\n\x05\x04\x02\x02\0\x03\x12\x03*\x11\x12\
    \n\n\n\x02\x04\x03\x12\x04-\04\x01\n\n\n\x03\x04\x03\x01\x12\x03-\x08\
    \x14\n\x0c\n\x04\x04\x03\x03\0\x12\x04.\x040\x05\n\x0c\n\x05\x04\x03\x03\
    \0\x01\x12\x03.\x0c\x10\n\r\n\x06\x04\x03\x03\0\x02\0\x12\x03/\x08\x17\n\
    \x0f\n\x07\x04\x03\x03\0\x02\0\x04\x12\x04/\x08.\x12\n\x0e\n\x07\x04\x03\
    \x03\0\x02\0\x05\x12\x03/\x08\r\n\x0e\n\x07\x04\x03\x03\0\x02\0\x01\x12\
    \x03/\x0e\x12\n\x0e\n\x07\x04\x03\x03\0\x02\0\x03\x12\x03/\x15\x16\n\x0b\
    \n\x04\x04\x03\x02\0\x12\x031\x04\x12\n\r\n\x05\x04\x03\x02\0\x04\x12\
    \x041\x040\x05\n\x0c\n\x05\x04\x03\x02\0\x06\x12\x031\x04\x08\n\x0c\n\
    \x05\x04\x03\x02\0\x01\x12\x031\t\r\n\x0c\n\x05\x04\x03\x02\0\x03\x12\
    \x031\x10\x11\n\x0b\n\x04\x04\x03\x02\x01\x12\x032\x04\x19\n\r\n\x05\x04\
    \x03\x02\x01\x04\x12\x042\x041\x12\n\x0c\n\x05\x04\x03\x02\x01\x06\x12\
    \x032\x04\x0e\n\x0c\n\x05\x04\x03\x02\x01\x01\x12\x032\x0f\x14\n\x0c\n\
    \x05\x04\x03\x02\x01\x03\x12\x032\x17\x18\n\x0b\n\x04\x04\x03\x02\x02\
    \x12\x033\x04\x1d\n\r\n\x05\x04\x03\x02\x02\x04\x12\x043\x042\x19\n\x0c\
    \n\x05\x04\x03\x02\x02\x06\x12\x033\x04\x10\n\x0c\n\x05\x04\x03\x02\x02\
    \x01\x12\x033\x11\x18\n\x0c\n\x05\x04\x03\x02\x02\x03\x12\x033\x1b\x1c\n\
    \n\n\x02\x04\x04\x12\x046\07\x01\n\n\n\x03\x04\x04\x01\x12\x036\x08\x15\
    \n\n\n\x02\x04\x05\x12\x049\0;\x01\n\n\n\x03\x04\x05\x01\x12\x039\x08\
    \x14\n\x0b\n\x04\x04\x05\x02\0\x12\x03:\x04\x13\n\r\n\x05\x04\x05\x02\0\
    \x04\x12\x04:\x049\x16\n\x0c\n\x05\x04\x05\x02\0\x05\x12\x03:\x04\t\n\
    \x0c\n\x05\x04\x05\x02\0\x01\x12\x03:\n\x0e\n\x0c\n\x05\x04\x05\x02\0\
    \x03\x12\x03:\x11\x12\n\n\n\x02\x04\x06\x12\x04=\0>\x01\n\n\n\x03\x04\
    \x06\x01\x12\x03=\x08\x15\n\n\n\x02\x04\x07\x12\x04@\0B\x01\n\n\n\x03\
    \x04\x07\x01\x12\x03@\x08\x15\n\x0b\n\x04\x04\x07\x02\0\x12\x03A\x04\x13\
    \n\r\n\x05\x04\x07\x02\0\x04\x12\x04A\x04@\x17\n\x0c\n\x05\x04\x07\x02\0\
    \x05\x12\x03A\x04\t\n\x0c\n\x05\x04\x07\x02\0\x01\x12\x03A\n\x0e\n\x0c\n\
    \x05\x04\x07\x02\0\x03\x12\x03A\x11\x12\n\n\n\x02\x04\x08\x12\x04D\0E\
    \x01\n\n\n\x03\x04\x08\x01\x12\x03D\x08\x16\n\n\n\x02\x04\t\x12\x04G\0I\
    \x01\n\n\n\x03\x04\t\x01\x12\x03G\x08\x15\n\x0b\n\x04\x04\t\x02\0\x12\
    \x03H\x04\x13\n\r\n\x05\x04\t\x02\0\x04\x12\x04H\x04G\x17\n\x0c\n\x05\
    \x04\t\x02\0\x05\x12\x03H\x04\t\n\x0c\n\x05\x04\t\x02\0\x01\x12\x03H\n\
    \x0e\n\x0c\n\x05\x04\t\x02\0\x03\x12\x03H\x11\x12\n\n\n\x02\x04\n\x12\
    \x04K\0L\x01\n\n\n\x03\x04\n\x01\x12\x03K\x08\x16\n\n\n\x02\x04\x0b\x12\
    \x04N\0Q\x01\n\n\n\x03\x04\x0b\x01\x12\x03N\x08\r\n\x0b\n\x04\x04\x0b\
    \x02\0\x12\x03O\x04\x14\n\r\n\x05\x04\x0b\x02\0\x04\x12\x04O\x04N\x0f\n\
    \x0c\n\x05\x04\x0b\x02\0\x05\x12\x03O\x04\t\n\x0c\n\x05\x04\x0b\x02\0\
    \x01\x12\x03O\n\x0f\n\x0c\n\x05\x04\x0b\x02\0\x03\x12\x03O\x12\x13\n\x0b\
    \n\x04\x04\x0b\x02\x01\x12\x03P\x04\x12\n\r\n\x05\x04\x0b\x02\x01\x04\
    \x12\x04P\x04O\x14\n\x0c\n\x05\x04\x0b\x02\x01\x05\x12\x03P\x04\t\n\x0c\
    \n\x05\x04\x0b\x02\x01\x01\x12\x03P\n\r\n\x0c\n\x05\x04\x0b\x02\x01\x03\
    \x12\x03P\x10\x11\n\n\n\x02\x04\x0c\x12\x04S\0[\x01\n\n\n\x03\x04\x0c\
    \x01\x12\x03S\x08\x0f\n\x0b\n\x04\x04\x0c\x02\0\x12\x03T\x04\x13\n\r\n\
    \x05\x04\x0c\x02\0\x04\x12\x04T\x04S\x11\n\x0c\n\x05\x04\x0c\x02\0\x05\
    \x12\x03T\x04\t\n\x0c\n\x05\x04\x0c\x02\0\x01\x12\x03T\n\x0e\n\x0c\n\x05\
    \x04\x0c\x02\0\x03\x12\x03T\x11\x12\n\x0b\n\x04\x04\x0c\x02\x01\x12\x03U\
    \x04\x14\n\r\n\x05\x04\x0c\x02\x01\x04\x12\x04U\x04T\x13\n\x0c\n\x05\x04\
    \x0c\x02\x01\x06\x12\x03U\x04\t\n\x0c\n\x05\x04\x0c\x02\x01\x01\x12\x03U\
    \n\x0f\n\x0c\n\x05\x04\x0c\x02\x01\x03\x12\x03U\x12\x13\n\x0b\n\x04\x04\
    \x0c\x02\x02\x12\x03V\x04\x15\n\r\n\x05\x04\x0c\x02\x02\x04\x12\x04V\x04\
    U\x14\n\x0c\n\x05\x04\x0c\x02\x02\x05\x12\x03V\x04\n\n\x0c\n\x05\x04\x0c\
    \x02\x02\x01\x12\x03V\x0b\x10\n\x0c\n\x05\x04\x0c\x02\x02\x03\x12\x03V\
    \x13\x14\n\x0b\n\x04\x04\x0c\x02\x03\x12\x03W\x04\x16\n\r\n\x05\x04\x0c\
    \x02\x03\x04\x12\x04W\x04V\x15\n\x0c\n\x05\x04\x0c\x02\x03\x05\x12\x03W\
    \x04\n\n\x0c\n\x05\x04\x0c\x02\x03\x01\x12\x03W\x0b\x11\n\x0c\n\x05\x04\
    \x0c\x02\x03\x03\x12\x03W\x14\x15\n\x0b\n\x04\x04\x0c\x02\x04\x12\x03X\
    \x04\x17\n\r\n\x05\x04\x0c\x02\x04\x04\x12\x04X\x04W\x16\n\x0c\n\x05\x04\
    \x0c\x02\x04\x05\x12\x03X\x04\n\n\x0c\n\x05\x04\x0c\x02\x04\x01\x12\x03X\
    \x0b\x12\n\x0c\n\x05\x04\x0c\x02\x04\x03\x12\x03X\x15\x16\n\x0b\n\x04\
    \x04\x0c\x02\x05\x12\x03Y\x04\x19\n\r\n\x05\x04\x0c\x02\x05\x04\x12\x04Y\
    \x04X\x17\n\x0c\n\x05\x04\x0c\x02\x05\x05\x12\x03Y\x04\n\n\x0c\n\x05\x04\
    \x0c\x02\x05\x01\x12\x03Y\x0b\x14\n\x0c\n\x05\x04\x0c\x02\x05\x03\x12\
    \x03Y\x17\x18\n\x0b\n\x04\x04\x0c\x02\x06\x12\x03Z\x04(\n\r\n\x05\x04\
    \x0c\x02\x06\x04\x12\x04Z\x04Y\x19\n\x0c\n\x05\x04\x0c\x02\x06\x06\x12\
    \x03Z\x04\x16\n\x0c\n\x05\x04\x0c\x02\x06\x01\x12\x03Z\x17#\n\x0c\n\x05\
    \x04\x0c\x02\x06\x03\x12\x03Z&'\n\n\n\x02\x04\r\x12\x04]\0`\x01\n\n\n\
    \x03\x04\r\x01\x12\x03]\x08\x15\n\x0b\n\x04\x04\r\x02\0\x12\x03^\x04\x15\
    \n\r\n\x05\x04\r\x02\0\x04\x12\x04^\x04]\x17\n\x0c\n\x05\x04\r\x02\0\x06\
    \x12\x03^\x04\x0b\n\x0c\n\x05\x04\r\x02\0\x01\x12\x03^\x0c\x10\n\x0c\n\
    \x05\x04\r\x02\0\x03\x12\x03^\x13\x14\n\x0b\n\x04\x04\r\x02\x01\x12\x03_\
    \x04\x13\n\r\n\x05\x04\r\x02\x01\x04\x12\x04_\x04^\x15\n\x0c\n\x05\x04\r\
    \x02\x01\x05\x12\x03_\x04\t\n\x0c\n\x05\x04\r\x02\x01\x01\x12\x03_\n\x0e\
    \n\x0c\n\x05\x04\r\x02\x01\x03\x12\x03_\x11\x12\n\n\n\x02\x04\x0e\x12\
    \x04b\0c\x01\n\n\n\x03\x04\x0e\x01\x12\x03b\x08\x16\n\n\n\x02\x04\x0f\
    \x12\x04e\0h\x01\n\n\n\x03\x04\x0f\x01\x12\x03e\x08\x15\n\x0b\n\x04\x04\
    \x0f\x02\0\x12\x03f\x04\x20\n\r\n\x05\x04\x0f\x02\0\x04\x12\x04f\x04e\
    \x17\n\x0c\n\x05\x04\x0f\x02\0\x06\x12\x03f\x04\x13\n\x0c\n\x05\x04\x0f\
    \x02\0\x01\x12\x03f\x14\x1b\n\x0c\n\x05\x04\x0f\x02\0\x03\x12\x03f\x1e\
    \x1f\n\x0b\n\x04\x04\x0f\x02\x01\x12\x03g\x04\x14\n\r\n\x05\x04\x0f\x02\
    \x01\x04\x12\x04g\x04f\x20\n\x0c\n\x05\x04\x0f\x02\x01\x06\x12\x03g\x04\
    \x0b\n\x0c\n\x05\x04\x0f\x02\x01\x01\x12\x03g\x0c\x0f\n\x0c\n\x05\x04\
    \x0f\x02\x01\x03\x12\x03g\x12\x13\n\n\n\x02\x04\x10\x12\x04j\0l\x01\n\n\
    \n\x03\x04\x10\x01\x12\x03j\x08\x16\n\x0b\n\x04\x04\x10\x02\0\x12\x03k\
    \x04\x1c\n\r\n\x05\x04\x10\x02\0\x04\x12\x04k\x04j\x18\n\x0c\n\x05\x04\
    \x10\x02\0\x06\x12\x03k\x04\x11\n\x0c\n\x05\x04\x10\x02\0\x01\x12\x03k\
    \x12\x17\n\x0c\n\x05\x04\x10\x02\0\x03\x12\x03k\x1a\x1bb\x06proto3\
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
