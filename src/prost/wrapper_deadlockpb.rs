// Generated file, please don't edit manually.

impl WaitForEntriesRequest {
    pub fn new_() -> WaitForEntriesRequest {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for WaitForEntriesRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for WaitForEntriesRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static WaitForEntriesRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WaitForEntriesRequest = WaitForEntriesRequest::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl WaitForEntriesResponse {
    pub fn new_() -> WaitForEntriesResponse {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }
    #[inline]
    pub fn set_entries(&mut self, v: ::std::vec::Vec<WaitForEntry>) {
        self.entries = v;
    }
    #[inline]
    pub fn get_entries(&self) -> &::std::vec::Vec<WaitForEntry> {
        &self.entries
    }
    #[inline]
    pub fn mut_entries(&mut self) -> &mut ::std::vec::Vec<WaitForEntry> {
        &mut self.entries
    }
    #[inline]
    pub fn take_entries(&mut self) -> ::std::vec::Vec<WaitForEntry> {
        ::std::mem::replace(&mut self.entries, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for WaitForEntriesResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for WaitForEntriesResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static WaitForEntriesResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WaitForEntriesResponse = WaitForEntriesResponse::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl WaitForEntry {
    pub fn new_() -> WaitForEntry {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_txn(&mut self) {
        self.txn = 0
    }
    #[inline]
    pub fn set_txn(&mut self, v: u64) {
        self.txn = v;
    }
    #[inline]
    pub fn get_txn(&self) -> u64 {
        self.txn
    }
    #[inline]
    pub fn clear_wait_for_txn(&mut self) {
        self.wait_for_txn = 0
    }
    #[inline]
    pub fn set_wait_for_txn(&mut self, v: u64) {
        self.wait_for_txn = v;
    }
    #[inline]
    pub fn get_wait_for_txn(&self) -> u64 {
        self.wait_for_txn
    }
    #[inline]
    pub fn clear_key_hash(&mut self) {
        self.key_hash = 0
    }
    #[inline]
    pub fn set_key_hash(&mut self, v: u64) {
        self.key_hash = v;
    }
    #[inline]
    pub fn get_key_hash(&self) -> u64 {
        self.key_hash
    }
}
impl ::protobuf::Clear for WaitForEntry {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for WaitForEntry {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static WaitForEntry {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WaitForEntry = WaitForEntry::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl DeadlockRequest {
    pub fn new_() -> DeadlockRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_tp(&mut self) {
        self.tp = 0
    }
    #[inline]
    pub fn set_tp_(&mut self, v: DeadlockRequestType) {
        self.tp = unsafe { ::std::mem::transmute::<DeadlockRequestType, i32>(v) };
    }
    #[inline]
    pub fn get_tp(&self) -> DeadlockRequestType {
        unsafe { ::std::mem::transmute::<i32, DeadlockRequestType>(self.tp) }
    }
    #[inline]
    pub fn has_entry(&self) -> bool {
        self.entry.is_some()
    }
    #[inline]
    pub fn clear_entry(&mut self) {
        self.entry = ::std::option::Option::None
    }
    #[inline]
    pub fn set_entry(&mut self, v: WaitForEntry) {
        self.entry = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_entry(&self) -> &WaitForEntry {
        match self.entry.as_ref() {
            Some(v) => v,
            None => <WaitForEntry as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_entry(&mut self) -> &mut WaitForEntry {
        if self.entry.is_none() {
            self.entry = ::std::option::Option::Some(WaitForEntry::default());
        }
        self.entry.as_mut().unwrap()
    }
    #[inline]
    pub fn take_entry(&mut self) -> WaitForEntry {
        self.entry.take().unwrap_or_else(WaitForEntry::default)
    }
}
impl ::protobuf::Clear for DeadlockRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for DeadlockRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static DeadlockRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DeadlockRequest = DeadlockRequest::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl DeadlockResponse {
    pub fn new_() -> DeadlockResponse {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_entry(&self) -> bool {
        self.entry.is_some()
    }
    #[inline]
    pub fn clear_entry(&mut self) {
        self.entry = ::std::option::Option::None
    }
    #[inline]
    pub fn set_entry(&mut self, v: WaitForEntry) {
        self.entry = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_entry(&self) -> &WaitForEntry {
        match self.entry.as_ref() {
            Some(v) => v,
            None => <WaitForEntry as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_entry(&mut self) -> &mut WaitForEntry {
        if self.entry.is_none() {
            self.entry = ::std::option::Option::Some(WaitForEntry::default());
        }
        self.entry.as_mut().unwrap()
    }
    #[inline]
    pub fn take_entry(&mut self) -> WaitForEntry {
        self.entry.take().unwrap_or_else(WaitForEntry::default)
    }
    #[inline]
    pub fn clear_deadlock_key_hash(&mut self) {
        self.deadlock_key_hash = 0
    }
    #[inline]
    pub fn set_deadlock_key_hash(&mut self, v: u64) {
        self.deadlock_key_hash = v;
    }
    #[inline]
    pub fn get_deadlock_key_hash(&self) -> u64 {
        self.deadlock_key_hash
    }
}
impl ::protobuf::Clear for DeadlockResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for DeadlockResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &::std::any::Any {
        self as &::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::new_()
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn default_instance() -> &'static DeadlockResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DeadlockResponse = DeadlockResponse::new_();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn merge_from(
        &mut self,
        _is: &mut ::protobuf::CodedInputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
    }
    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        unimplemented!();
    }
    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        unimplemented!();
    }
    fn write_to_bytes(&self) -> ::protobuf::ProtobufResult<Vec<u8>> {
        let mut buf = Vec::new();
        if let Err(_) = ::prost::Message::encode(self, &mut buf) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if let Err(_) = ::prost::Message::merge(self, bytes) {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl DeadlockRequestType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [DeadlockRequestType] = &[
            DeadlockRequestType::Detect,
            DeadlockRequestType::CleanUpWaitFor,
            DeadlockRequestType::CleanUp,
        ];
        VALUES
    }
}
