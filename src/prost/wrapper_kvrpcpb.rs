// Generated file, please don't edit manually.

impl LockInfo {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }
    #[inline]
    pub fn set_primary_lock(&mut self, v: std::vec::Vec<u8>) {
        self.primary_lock = v;
    }
    #[inline]
    pub fn get_primary_lock(&self) -> &[u8] {
        &self.primary_lock
    }
    #[inline]
    pub fn mut_primary_lock(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.primary_lock
    }
    #[inline]
    pub fn take_primary_lock(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.primary_lock, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_lock_version(&mut self) {
        self.lock_version = 0
    }
    #[inline]
    pub fn set_lock_version(&mut self, v: u64) {
        self.lock_version = v;
    }
    #[inline]
    pub fn get_lock_version(&self) -> u64 {
        self.lock_version
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = 0
    }
    #[inline]
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = v;
    }
    #[inline]
    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl
    }
    #[inline]
    pub fn clear_txn_size(&mut self) {
        self.txn_size = 0
    }
    #[inline]
    pub fn set_txn_size(&mut self, v: u64) {
        self.txn_size = v;
    }
    #[inline]
    pub fn get_txn_size(&self) -> u64 {
        self.txn_size
    }
}
impl ::protobuf::Clear for LockInfo {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for LockInfo {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static LockInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: LockInfo = LockInfo::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl AlreadyExist {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for AlreadyExist {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AlreadyExist {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static AlreadyExist {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AlreadyExist = AlreadyExist::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl KeyError {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_locked(&self) -> bool {
        self.locked.is_some()
    }
    #[inline]
    pub fn clear_locked(&mut self) {
        self.locked = ::std::option::Option::None
    }
    #[inline]
    pub fn set_locked(&mut self, v: LockInfo) {
        self.locked = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_locked(&self) -> &LockInfo {
        match self.locked.as_ref() {
            Some(v) => v,
            None => LockInfo::default_ref(),
        }
    }
    #[inline]
    pub fn mut_locked(&mut self) -> &mut LockInfo {
        if self.locked.is_none() {
            self.locked = ::std::option::Option::Some(LockInfo::default());
        }
        self.locked.as_mut().unwrap()
    }
    #[inline]
    pub fn take_locked(&mut self) -> LockInfo {
        self.locked.take().unwrap_or_else(LockInfo::default)
    }
    #[inline]
    pub fn clear_retryable(&mut self) {
        self.retryable.clear();
    }
    #[inline]
    pub fn set_retryable(&mut self, v: std::string::String) {
        self.retryable = v;
    }
    #[inline]
    pub fn get_retryable(&self) -> &str {
        &self.retryable
    }
    #[inline]
    pub fn mut_retryable(&mut self) -> &mut std::string::String {
        &mut self.retryable
    }
    #[inline]
    pub fn take_retryable(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.retryable, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_abort(&mut self) {
        self.abort.clear();
    }
    #[inline]
    pub fn set_abort(&mut self, v: std::string::String) {
        self.abort = v;
    }
    #[inline]
    pub fn get_abort(&self) -> &str {
        &self.abort
    }
    #[inline]
    pub fn mut_abort(&mut self) -> &mut std::string::String {
        &mut self.abort
    }
    #[inline]
    pub fn take_abort(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.abort, ::std::string::String::new())
    }
    #[inline]
    pub fn has_conflict(&self) -> bool {
        self.conflict.is_some()
    }
    #[inline]
    pub fn clear_conflict(&mut self) {
        self.conflict = ::std::option::Option::None
    }
    #[inline]
    pub fn set_conflict(&mut self, v: WriteConflict) {
        self.conflict = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_conflict(&self) -> &WriteConflict {
        match self.conflict.as_ref() {
            Some(v) => v,
            None => WriteConflict::default_ref(),
        }
    }
    #[inline]
    pub fn mut_conflict(&mut self) -> &mut WriteConflict {
        if self.conflict.is_none() {
            self.conflict = ::std::option::Option::Some(WriteConflict::default());
        }
        self.conflict.as_mut().unwrap()
    }
    #[inline]
    pub fn take_conflict(&mut self) -> WriteConflict {
        self.conflict.take().unwrap_or_else(WriteConflict::default)
    }
    #[inline]
    pub fn has_already_exist(&self) -> bool {
        self.already_exist.is_some()
    }
    #[inline]
    pub fn clear_already_exist(&mut self) {
        self.already_exist = ::std::option::Option::None
    }
    #[inline]
    pub fn set_already_exist(&mut self, v: AlreadyExist) {
        self.already_exist = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_already_exist(&self) -> &AlreadyExist {
        match self.already_exist.as_ref() {
            Some(v) => v,
            None => AlreadyExist::default_ref(),
        }
    }
    #[inline]
    pub fn mut_already_exist(&mut self) -> &mut AlreadyExist {
        if self.already_exist.is_none() {
            self.already_exist = ::std::option::Option::Some(AlreadyExist::default());
        }
        self.already_exist.as_mut().unwrap()
    }
    #[inline]
    pub fn take_already_exist(&mut self) -> AlreadyExist {
        self.already_exist
            .take()
            .unwrap_or_else(AlreadyExist::default)
    }
    #[inline]
    pub fn has_deadlock(&self) -> bool {
        self.deadlock.is_some()
    }
    #[inline]
    pub fn clear_deadlock(&mut self) {
        self.deadlock = ::std::option::Option::None
    }
    #[inline]
    pub fn set_deadlock(&mut self, v: Deadlock) {
        self.deadlock = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_deadlock(&self) -> &Deadlock {
        match self.deadlock.as_ref() {
            Some(v) => v,
            None => Deadlock::default_ref(),
        }
    }
    #[inline]
    pub fn mut_deadlock(&mut self) -> &mut Deadlock {
        if self.deadlock.is_none() {
            self.deadlock = ::std::option::Option::Some(Deadlock::default());
        }
        self.deadlock.as_mut().unwrap()
    }
    #[inline]
    pub fn take_deadlock(&mut self) -> Deadlock {
        self.deadlock.take().unwrap_or_else(Deadlock::default)
    }
}
impl ::protobuf::Clear for KeyError {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for KeyError {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static KeyError {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: KeyError = KeyError::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl WriteConflict {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    #[inline]
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
    #[inline]
    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }
    #[inline]
    pub fn clear_conflict_ts(&mut self) {
        self.conflict_ts = 0
    }
    #[inline]
    pub fn set_conflict_ts(&mut self, v: u64) {
        self.conflict_ts = v;
    }
    #[inline]
    pub fn get_conflict_ts(&self) -> u64 {
        self.conflict_ts
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }
    #[inline]
    pub fn set_primary(&mut self, v: std::vec::Vec<u8>) {
        self.primary = v;
    }
    #[inline]
    pub fn get_primary(&self) -> &[u8] {
        &self.primary
    }
    #[inline]
    pub fn mut_primary(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.primary
    }
    #[inline]
    pub fn take_primary(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.primary, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_conflict_commit_ts(&mut self) {
        self.conflict_commit_ts = 0
    }
    #[inline]
    pub fn set_conflict_commit_ts(&mut self, v: u64) {
        self.conflict_commit_ts = v;
    }
    #[inline]
    pub fn get_conflict_commit_ts(&self) -> u64 {
        self.conflict_commit_ts
    }
}
impl ::protobuf::Clear for WriteConflict {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for WriteConflict {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static WriteConflict {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteConflict = WriteConflict::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Deadlock {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_lock_ts(&mut self) {
        self.lock_ts = 0
    }
    #[inline]
    pub fn set_lock_ts(&mut self, v: u64) {
        self.lock_ts = v;
    }
    #[inline]
    pub fn get_lock_ts(&self) -> u64 {
        self.lock_ts
    }
    #[inline]
    pub fn clear_lock_key(&mut self) {
        self.lock_key.clear();
    }
    #[inline]
    pub fn set_lock_key(&mut self, v: std::vec::Vec<u8>) {
        self.lock_key = v;
    }
    #[inline]
    pub fn get_lock_key(&self) -> &[u8] {
        &self.lock_key
    }
    #[inline]
    pub fn mut_lock_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.lock_key
    }
    #[inline]
    pub fn take_lock_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.lock_key, ::std::vec::Vec::new())
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
impl ::protobuf::Clear for Deadlock {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Deadlock {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Deadlock {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Deadlock = Deadlock::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Context {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_region_id(&mut self) {
        self.region_id = 0
    }
    #[inline]
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }
    #[inline]
    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }
    #[inline]
    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }
    #[inline]
    pub fn clear_region_epoch(&mut self) {
        self.region_epoch = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_epoch(&mut self, v: super::metapb::RegionEpoch) {
        self.region_epoch = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_epoch(&self) -> &super::metapb::RegionEpoch {
        match self.region_epoch.as_ref() {
            Some(v) => v,
            None => super::metapb::RegionEpoch::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_epoch(&mut self) -> &mut super::metapb::RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch = ::std::option::Option::Some(super::metapb::RegionEpoch::default());
        }
        self.region_epoch.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_epoch(&mut self) -> super::metapb::RegionEpoch {
        self.region_epoch
            .take()
            .unwrap_or_else(super::metapb::RegionEpoch::default)
    }
    #[inline]
    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }
    #[inline]
    pub fn clear_peer(&mut self) {
        self.peer = ::std::option::Option::None
    }
    #[inline]
    pub fn set_peer(&mut self, v: super::metapb::Peer) {
        self.peer = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_peer(&self) -> &super::metapb::Peer {
        match self.peer.as_ref() {
            Some(v) => v,
            None => super::metapb::Peer::default_ref(),
        }
    }
    #[inline]
    pub fn mut_peer(&mut self) -> &mut super::metapb::Peer {
        if self.peer.is_none() {
            self.peer = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.peer.as_mut().unwrap()
    }
    #[inline]
    pub fn take_peer(&mut self) -> super::metapb::Peer {
        self.peer
            .take()
            .unwrap_or_else(super::metapb::Peer::default)
    }
    #[inline]
    pub fn clear_term(&mut self) {
        self.term = 0
    }
    #[inline]
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }
    #[inline]
    pub fn get_term(&self) -> u64 {
        self.term
    }
    #[inline]
    pub fn clear_priority(&mut self) {
        self.priority = 0
    }
    #[inline]
    pub fn set_priority_(&mut self, v: CommandPri) {
        self.priority = unsafe { ::std::mem::transmute::<CommandPri, i32>(v) };
    }
    #[inline]
    pub fn get_priority(&self) -> CommandPri {
        match CommandPri::from_i32(self.priority) {
            Some(e) => e,
            None => panic!("Unknown enum variant: {}", self.priority),
        }
    }
    #[inline]
    pub fn clear_isolation_level(&mut self) {
        self.isolation_level = 0
    }
    #[inline]
    pub fn set_isolation_level_(&mut self, v: IsolationLevel) {
        self.isolation_level = unsafe { ::std::mem::transmute::<IsolationLevel, i32>(v) };
    }
    #[inline]
    pub fn get_isolation_level(&self) -> IsolationLevel {
        match IsolationLevel::from_i32(self.isolation_level) {
            Some(e) => e,
            None => panic!("Unknown enum variant: {}", self.isolation_level),
        }
    }
    #[inline]
    pub fn clear_not_fill_cache(&mut self) {
        self.not_fill_cache = false
    }
    #[inline]
    pub fn set_not_fill_cache(&mut self, v: bool) {
        self.not_fill_cache = v;
    }
    #[inline]
    pub fn get_not_fill_cache(&self) -> bool {
        self.not_fill_cache
    }
    #[inline]
    pub fn clear_sync_log(&mut self) {
        self.sync_log = false
    }
    #[inline]
    pub fn set_sync_log(&mut self, v: bool) {
        self.sync_log = v;
    }
    #[inline]
    pub fn get_sync_log(&self) -> bool {
        self.sync_log
    }
    #[inline]
    pub fn clear_handle_time(&mut self) {
        self.handle_time = false
    }
    #[inline]
    pub fn set_handle_time(&mut self, v: bool) {
        self.handle_time = v;
    }
    #[inline]
    pub fn get_handle_time(&self) -> bool {
        self.handle_time
    }
    #[inline]
    pub fn clear_scan_detail(&mut self) {
        self.scan_detail = false
    }
    #[inline]
    pub fn set_scan_detail(&mut self, v: bool) {
        self.scan_detail = v;
    }
    #[inline]
    pub fn get_scan_detail(&self) -> bool {
        self.scan_detail
    }
}
impl ::protobuf::Clear for Context {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Context {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Context {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Context = Context::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl HandleTime {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_wait_ms(&mut self) {
        self.wait_ms = 0
    }
    #[inline]
    pub fn set_wait_ms(&mut self, v: i64) {
        self.wait_ms = v;
    }
    #[inline]
    pub fn get_wait_ms(&self) -> i64 {
        self.wait_ms
    }
    #[inline]
    pub fn clear_process_ms(&mut self) {
        self.process_ms = 0
    }
    #[inline]
    pub fn set_process_ms(&mut self, v: i64) {
        self.process_ms = v;
    }
    #[inline]
    pub fn get_process_ms(&self) -> i64 {
        self.process_ms
    }
}
impl ::protobuf::Clear for HandleTime {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for HandleTime {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static HandleTime {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: HandleTime = HandleTime::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ScanInfo {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_total(&mut self) {
        self.total = 0
    }
    #[inline]
    pub fn set_total(&mut self, v: i64) {
        self.total = v;
    }
    #[inline]
    pub fn get_total(&self) -> i64 {
        self.total
    }
    #[inline]
    pub fn clear_processed(&mut self) {
        self.processed = 0
    }
    #[inline]
    pub fn set_processed(&mut self, v: i64) {
        self.processed = v;
    }
    #[inline]
    pub fn get_processed(&self) -> i64 {
        self.processed
    }
}
impl ::protobuf::Clear for ScanInfo {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScanInfo {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ScanInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanInfo = ScanInfo::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ScanDetail {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_write(&self) -> bool {
        self.write.is_some()
    }
    #[inline]
    pub fn clear_write(&mut self) {
        self.write = ::std::option::Option::None
    }
    #[inline]
    pub fn set_write(&mut self, v: ScanInfo) {
        self.write = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_write(&self) -> &ScanInfo {
        match self.write.as_ref() {
            Some(v) => v,
            None => ScanInfo::default_ref(),
        }
    }
    #[inline]
    pub fn mut_write(&mut self) -> &mut ScanInfo {
        if self.write.is_none() {
            self.write = ::std::option::Option::Some(ScanInfo::default());
        }
        self.write.as_mut().unwrap()
    }
    #[inline]
    pub fn take_write(&mut self) -> ScanInfo {
        self.write.take().unwrap_or_else(ScanInfo::default)
    }
    #[inline]
    pub fn has_lock(&self) -> bool {
        self.lock.is_some()
    }
    #[inline]
    pub fn clear_lock(&mut self) {
        self.lock = ::std::option::Option::None
    }
    #[inline]
    pub fn set_lock(&mut self, v: ScanInfo) {
        self.lock = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_lock(&self) -> &ScanInfo {
        match self.lock.as_ref() {
            Some(v) => v,
            None => ScanInfo::default_ref(),
        }
    }
    #[inline]
    pub fn mut_lock(&mut self) -> &mut ScanInfo {
        if self.lock.is_none() {
            self.lock = ::std::option::Option::Some(ScanInfo::default());
        }
        self.lock.as_mut().unwrap()
    }
    #[inline]
    pub fn take_lock(&mut self) -> ScanInfo {
        self.lock.take().unwrap_or_else(ScanInfo::default)
    }
    #[inline]
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
    #[inline]
    pub fn clear_data(&mut self) {
        self.data = ::std::option::Option::None
    }
    #[inline]
    pub fn set_data(&mut self, v: ScanInfo) {
        self.data = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_data(&self) -> &ScanInfo {
        match self.data.as_ref() {
            Some(v) => v,
            None => ScanInfo::default_ref(),
        }
    }
    #[inline]
    pub fn mut_data(&mut self) -> &mut ScanInfo {
        if self.data.is_none() {
            self.data = ::std::option::Option::Some(ScanInfo::default());
        }
        self.data.as_mut().unwrap()
    }
    #[inline]
    pub fn take_data(&mut self) -> ScanInfo {
        self.data.take().unwrap_or_else(ScanInfo::default)
    }
}
impl ::protobuf::Clear for ScanDetail {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScanDetail {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ScanDetail {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanDetail = ScanDetail::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ExecDetails {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_handle_time(&self) -> bool {
        self.handle_time.is_some()
    }
    #[inline]
    pub fn clear_handle_time(&mut self) {
        self.handle_time = ::std::option::Option::None
    }
    #[inline]
    pub fn set_handle_time(&mut self, v: HandleTime) {
        self.handle_time = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_handle_time(&self) -> &HandleTime {
        match self.handle_time.as_ref() {
            Some(v) => v,
            None => HandleTime::default_ref(),
        }
    }
    #[inline]
    pub fn mut_handle_time(&mut self) -> &mut HandleTime {
        if self.handle_time.is_none() {
            self.handle_time = ::std::option::Option::Some(HandleTime::default());
        }
        self.handle_time.as_mut().unwrap()
    }
    #[inline]
    pub fn take_handle_time(&mut self) -> HandleTime {
        self.handle_time.take().unwrap_or_else(HandleTime::default)
    }
    #[inline]
    pub fn has_scan_detail(&self) -> bool {
        self.scan_detail.is_some()
    }
    #[inline]
    pub fn clear_scan_detail(&mut self) {
        self.scan_detail = ::std::option::Option::None
    }
    #[inline]
    pub fn set_scan_detail(&mut self, v: ScanDetail) {
        self.scan_detail = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_scan_detail(&self) -> &ScanDetail {
        match self.scan_detail.as_ref() {
            Some(v) => v,
            None => ScanDetail::default_ref(),
        }
    }
    #[inline]
    pub fn mut_scan_detail(&mut self) -> &mut ScanDetail {
        if self.scan_detail.is_none() {
            self.scan_detail = ::std::option::Option::Some(ScanDetail::default());
        }
        self.scan_detail.as_mut().unwrap()
    }
    #[inline]
    pub fn take_scan_detail(&mut self) -> ScanDetail {
        self.scan_detail.take().unwrap_or_else(ScanDetail::default)
    }
}
impl ::protobuf::Clear for ExecDetails {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ExecDetails {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ExecDetails {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ExecDetails = ExecDetails::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl GetRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    #[inline]
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
    #[inline]
    pub fn get_version(&self) -> u64 {
        self.version
    }
}
impl ::protobuf::Clear for GetRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static GetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetRequest = GetRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl GetResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => KeyError::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    #[inline]
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    #[inline]
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    #[inline]
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    #[inline]
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for GetResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static GetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetResponse = GetResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ScanRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    #[inline]
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    #[inline]
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    #[inline]
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    #[inline]
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_limit(&mut self) {
        self.limit = 0
    }
    #[inline]
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }
    #[inline]
    pub fn get_limit(&self) -> u32 {
        self.limit
    }
    #[inline]
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    #[inline]
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
    #[inline]
    pub fn get_version(&self) -> u64 {
        self.version
    }
    #[inline]
    pub fn clear_key_only(&mut self) {
        self.key_only = false
    }
    #[inline]
    pub fn set_key_only(&mut self, v: bool) {
        self.key_only = v;
    }
    #[inline]
    pub fn get_key_only(&self) -> bool {
        self.key_only
    }
    #[inline]
    pub fn clear_reverse(&mut self) {
        self.reverse = false
    }
    #[inline]
    pub fn set_reverse(&mut self, v: bool) {
        self.reverse = v;
    }
    #[inline]
    pub fn get_reverse(&self) -> bool {
        self.reverse
    }
    #[inline]
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    #[inline]
    pub fn set_end_key(&mut self, v: std::vec::Vec<u8>) {
        self.end_key = v;
    }
    #[inline]
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    #[inline]
    pub fn mut_end_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end_key
    }
    #[inline]
    pub fn take_end_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for ScanRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScanRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ScanRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanRequest = ScanRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl KvPair {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => KeyError::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    #[inline]
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    #[inline]
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    #[inline]
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    #[inline]
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for KvPair {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for KvPair {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static KvPair {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: KvPair = KvPair::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ScanResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    #[inline]
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    #[inline]
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    #[inline]
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
    #[inline]
    pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for ScanResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScanResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ScanResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanResponse = ScanResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl Mutation {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_op(&mut self) {
        self.op = 0
    }
    #[inline]
    pub fn set_op_(&mut self, v: Op) {
        self.op = unsafe { ::std::mem::transmute::<Op, i32>(v) };
    }
    #[inline]
    pub fn get_op(&self) -> Op {
        match Op::from_i32(self.op) {
            Some(e) => e,
            None => panic!("Unknown enum variant: {}", self.op),
        }
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    #[inline]
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    #[inline]
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    #[inline]
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    #[inline]
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_assertion(&mut self) {
        self.assertion = 0
    }
    #[inline]
    pub fn set_assertion_(&mut self, v: Assertion) {
        self.assertion = unsafe { ::std::mem::transmute::<Assertion, i32>(v) };
    }
    #[inline]
    pub fn get_assertion(&self) -> Assertion {
        match Assertion::from_i32(self.assertion) {
            Some(e) => e,
            None => panic!("Unknown enum variant: {}", self.assertion),
        }
    }
}
impl ::protobuf::Clear for Mutation {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Mutation {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static Mutation {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Mutation = Mutation::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl PrewriteRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }
    #[inline]
    pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) {
        self.mutations = v;
    }
    #[inline]
    pub fn get_mutations(&self) -> &::std::vec::Vec<Mutation> {
        &self.mutations
    }
    #[inline]
    pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> {
        &mut self.mutations
    }
    #[inline]
    pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }
    #[inline]
    pub fn set_primary_lock(&mut self, v: std::vec::Vec<u8>) {
        self.primary_lock = v;
    }
    #[inline]
    pub fn get_primary_lock(&self) -> &[u8] {
        &self.primary_lock
    }
    #[inline]
    pub fn mut_primary_lock(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.primary_lock
    }
    #[inline]
    pub fn take_primary_lock(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.primary_lock, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    #[inline]
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    #[inline]
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    #[inline]
    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = 0
    }
    #[inline]
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = v;
    }
    #[inline]
    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl
    }
    #[inline]
    pub fn clear_skip_constraint_check(&mut self) {
        self.skip_constraint_check = false
    }
    #[inline]
    pub fn set_skip_constraint_check(&mut self, v: bool) {
        self.skip_constraint_check = v;
    }
    #[inline]
    pub fn get_skip_constraint_check(&self) -> bool {
        self.skip_constraint_check
    }
    #[inline]
    pub fn clear_is_pessimistic_lock(&mut self) {
        self.is_pessimistic_lock.clear();
    }
    #[inline]
    pub fn set_is_pessimistic_lock(&mut self, v: ::std::vec::Vec<bool>) {
        self.is_pessimistic_lock = v;
    }
    #[inline]
    pub fn get_is_pessimistic_lock(&self) -> &::std::vec::Vec<bool> {
        &self.is_pessimistic_lock
    }
    #[inline]
    pub fn mut_is_pessimistic_lock(&mut self) -> &mut ::std::vec::Vec<bool> {
        &mut self.is_pessimistic_lock
    }
    #[inline]
    pub fn take_is_pessimistic_lock(&mut self) -> ::std::vec::Vec<bool> {
        ::std::mem::replace(&mut self.is_pessimistic_lock, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_txn_size(&mut self) {
        self.txn_size = 0
    }
    #[inline]
    pub fn set_txn_size(&mut self, v: u64) {
        self.txn_size = v;
    }
    #[inline]
    pub fn get_txn_size(&self) -> u64 {
        self.txn_size
    }
    #[inline]
    pub fn clear_for_update_ts(&mut self) {
        self.for_update_ts = 0
    }
    #[inline]
    pub fn set_for_update_ts(&mut self, v: u64) {
        self.for_update_ts = v;
    }
    #[inline]
    pub fn get_for_update_ts(&self) -> u64 {
        self.for_update_ts
    }
}
impl ::protobuf::Clear for PrewriteRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PrewriteRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static PrewriteRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PrewriteRequest = PrewriteRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl PrewriteResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
    #[inline]
    pub fn set_errors(&mut self, v: ::std::vec::Vec<KeyError>) {
        self.errors = v;
    }
    #[inline]
    pub fn get_errors(&self) -> &::std::vec::Vec<KeyError> {
        &self.errors
    }
    #[inline]
    pub fn mut_errors(&mut self) -> &mut ::std::vec::Vec<KeyError> {
        &mut self.errors
    }
    #[inline]
    pub fn take_errors(&mut self) -> ::std::vec::Vec<KeyError> {
        ::std::mem::replace(&mut self.errors, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for PrewriteResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PrewriteResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static PrewriteResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PrewriteResponse = PrewriteResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl PessimisticLockRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }
    #[inline]
    pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) {
        self.mutations = v;
    }
    #[inline]
    pub fn get_mutations(&self) -> &::std::vec::Vec<Mutation> {
        &self.mutations
    }
    #[inline]
    pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> {
        &mut self.mutations
    }
    #[inline]
    pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }
    #[inline]
    pub fn set_primary_lock(&mut self, v: std::vec::Vec<u8>) {
        self.primary_lock = v;
    }
    #[inline]
    pub fn get_primary_lock(&self) -> &[u8] {
        &self.primary_lock
    }
    #[inline]
    pub fn mut_primary_lock(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.primary_lock
    }
    #[inline]
    pub fn take_primary_lock(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.primary_lock, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    #[inline]
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    #[inline]
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    #[inline]
    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = 0
    }
    #[inline]
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = v;
    }
    #[inline]
    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl
    }
    #[inline]
    pub fn clear_for_update_ts(&mut self) {
        self.for_update_ts = 0
    }
    #[inline]
    pub fn set_for_update_ts(&mut self, v: u64) {
        self.for_update_ts = v;
    }
    #[inline]
    pub fn get_for_update_ts(&self) -> u64 {
        self.for_update_ts
    }
    #[inline]
    pub fn clear_is_first_lock(&mut self) {
        self.is_first_lock = false
    }
    #[inline]
    pub fn set_is_first_lock(&mut self, v: bool) {
        self.is_first_lock = v;
    }
    #[inline]
    pub fn get_is_first_lock(&self) -> bool {
        self.is_first_lock
    }
}
impl ::protobuf::Clear for PessimisticLockRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PessimisticLockRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static PessimisticLockRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PessimisticLockRequest = PessimisticLockRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl PessimisticLockResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
    #[inline]
    pub fn set_errors(&mut self, v: ::std::vec::Vec<KeyError>) {
        self.errors = v;
    }
    #[inline]
    pub fn get_errors(&self) -> &::std::vec::Vec<KeyError> {
        &self.errors
    }
    #[inline]
    pub fn mut_errors(&mut self) -> &mut ::std::vec::Vec<KeyError> {
        &mut self.errors
    }
    #[inline]
    pub fn take_errors(&mut self) -> ::std::vec::Vec<KeyError> {
        ::std::mem::replace(&mut self.errors, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for PessimisticLockResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PessimisticLockResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static PessimisticLockResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PessimisticLockResponse = PessimisticLockResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl PessimisticRollbackRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    #[inline]
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    #[inline]
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    #[inline]
    pub fn clear_for_update_ts(&mut self) {
        self.for_update_ts = 0
    }
    #[inline]
    pub fn set_for_update_ts(&mut self, v: u64) {
        self.for_update_ts = v;
    }
    #[inline]
    pub fn get_for_update_ts(&self) -> u64 {
        self.for_update_ts
    }
    #[inline]
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    #[inline]
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    #[inline]
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    #[inline]
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    #[inline]
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for PessimisticRollbackRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PessimisticRollbackRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static PessimisticRollbackRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PessimisticRollbackRequest = PessimisticRollbackRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl PessimisticRollbackResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
    #[inline]
    pub fn set_errors(&mut self, v: ::std::vec::Vec<KeyError>) {
        self.errors = v;
    }
    #[inline]
    pub fn get_errors(&self) -> &::std::vec::Vec<KeyError> {
        &self.errors
    }
    #[inline]
    pub fn mut_errors(&mut self) -> &mut ::std::vec::Vec<KeyError> {
        &mut self.errors
    }
    #[inline]
    pub fn take_errors(&mut self) -> ::std::vec::Vec<KeyError> {
        ::std::mem::replace(&mut self.errors, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for PessimisticRollbackResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PessimisticRollbackResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static PessimisticRollbackResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PessimisticRollbackResponse = PessimisticRollbackResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl CommitRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    #[inline]
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    #[inline]
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    #[inline]
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    #[inline]
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    #[inline]
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    #[inline]
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    #[inline]
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    #[inline]
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
    #[inline]
    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }
}
impl ::protobuf::Clear for CommitRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommitRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static CommitRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommitRequest = CommitRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl CommitResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => KeyError::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
}
impl ::protobuf::Clear for CommitResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommitResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static CommitResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommitResponse = CommitResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ImportRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }
    #[inline]
    pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) {
        self.mutations = v;
    }
    #[inline]
    pub fn get_mutations(&self) -> &::std::vec::Vec<Mutation> {
        &self.mutations
    }
    #[inline]
    pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> {
        &mut self.mutations
    }
    #[inline]
    pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    #[inline]
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
    #[inline]
    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }
}
impl ::protobuf::Clear for ImportRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ImportRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ImportRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ImportRequest = ImportRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ImportResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for ImportResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ImportResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ImportResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ImportResponse = ImportResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl BatchRollbackRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    #[inline]
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    #[inline]
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    #[inline]
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    #[inline]
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    #[inline]
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    #[inline]
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    #[inline]
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for BatchRollbackRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchRollbackRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static BatchRollbackRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchRollbackRequest = BatchRollbackRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl BatchRollbackResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => KeyError::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
}
impl ::protobuf::Clear for BatchRollbackResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchRollbackResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static BatchRollbackResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchRollbackResponse = BatchRollbackResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl CleanupRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    #[inline]
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    #[inline]
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
}
impl ::protobuf::Clear for CleanupRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CleanupRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static CleanupRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CleanupRequest = CleanupRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl CleanupResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => KeyError::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
    #[inline]
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    #[inline]
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
    #[inline]
    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }
}
impl ::protobuf::Clear for CleanupResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CleanupResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static CleanupResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CleanupResponse = CleanupResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl BatchGetRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    #[inline]
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    #[inline]
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    #[inline]
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    #[inline]
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    #[inline]
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
    #[inline]
    pub fn get_version(&self) -> u64 {
        self.version
    }
}
impl ::protobuf::Clear for BatchGetRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchGetRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static BatchGetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchGetRequest = BatchGetRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl BatchGetResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    #[inline]
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    #[inline]
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    #[inline]
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
    #[inline]
    pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for BatchGetResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchGetResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static BatchGetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchGetResponse = BatchGetResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ScanLockRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_max_version(&mut self) {
        self.max_version = 0
    }
    #[inline]
    pub fn set_max_version(&mut self, v: u64) {
        self.max_version = v;
    }
    #[inline]
    pub fn get_max_version(&self) -> u64 {
        self.max_version
    }
    #[inline]
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    #[inline]
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    #[inline]
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    #[inline]
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    #[inline]
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_limit(&mut self) {
        self.limit = 0
    }
    #[inline]
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }
    #[inline]
    pub fn get_limit(&self) -> u32 {
        self.limit
    }
}
impl ::protobuf::Clear for ScanLockRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScanLockRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ScanLockRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanLockRequest = ScanLockRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ScanLockResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => KeyError::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
    #[inline]
    pub fn clear_locks(&mut self) {
        self.locks.clear();
    }
    #[inline]
    pub fn set_locks(&mut self, v: ::std::vec::Vec<LockInfo>) {
        self.locks = v;
    }
    #[inline]
    pub fn get_locks(&self) -> &::std::vec::Vec<LockInfo> {
        &self.locks
    }
    #[inline]
    pub fn mut_locks(&mut self) -> &mut ::std::vec::Vec<LockInfo> {
        &mut self.locks
    }
    #[inline]
    pub fn take_locks(&mut self) -> ::std::vec::Vec<LockInfo> {
        ::std::mem::replace(&mut self.locks, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for ScanLockResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScanLockResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ScanLockResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanLockResponse = ScanLockResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl TxnInfo {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
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
    pub fn clear_status(&mut self) {
        self.status = 0
    }
    #[inline]
    pub fn set_status(&mut self, v: u64) {
        self.status = v;
    }
    #[inline]
    pub fn get_status(&self) -> u64 {
        self.status
    }
}
impl ::protobuf::Clear for TxnInfo {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TxnInfo {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static TxnInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TxnInfo = TxnInfo::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ResolveLockRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    #[inline]
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    #[inline]
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    #[inline]
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    #[inline]
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
    #[inline]
    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }
    #[inline]
    pub fn clear_txn_infos(&mut self) {
        self.txn_infos.clear();
    }
    #[inline]
    pub fn set_txn_infos(&mut self, v: ::std::vec::Vec<TxnInfo>) {
        self.txn_infos = v;
    }
    #[inline]
    pub fn get_txn_infos(&self) -> &::std::vec::Vec<TxnInfo> {
        &self.txn_infos
    }
    #[inline]
    pub fn mut_txn_infos(&mut self) -> &mut ::std::vec::Vec<TxnInfo> {
        &mut self.txn_infos
    }
    #[inline]
    pub fn take_txn_infos(&mut self) -> ::std::vec::Vec<TxnInfo> {
        ::std::mem::replace(&mut self.txn_infos, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    #[inline]
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    #[inline]
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    #[inline]
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    #[inline]
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for ResolveLockRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ResolveLockRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ResolveLockRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ResolveLockRequest = ResolveLockRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ResolveLockResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => KeyError::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
}
impl ::protobuf::Clear for ResolveLockResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ResolveLockResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ResolveLockResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ResolveLockResponse = ResolveLockResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl GcRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_safe_point(&mut self) {
        self.safe_point = 0
    }
    #[inline]
    pub fn set_safe_point(&mut self, v: u64) {
        self.safe_point = v;
    }
    #[inline]
    pub fn get_safe_point(&self) -> u64 {
        self.safe_point
    }
}
impl ::protobuf::Clear for GcRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GcRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static GcRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GcRequest = GcRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl GcResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => KeyError::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(KeyError::default)
    }
}
impl ::protobuf::Clear for GcResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GcResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static GcResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GcResponse = GcResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawGetRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    #[inline]
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    #[inline]
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    #[inline]
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    #[inline]
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawGetRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawGetRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawGetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawGetRequest = RawGetRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawGetResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    #[inline]
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    #[inline]
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    #[inline]
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    #[inline]
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for RawGetResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawGetResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawGetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawGetResponse = RawGetResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawPutRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    #[inline]
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    #[inline]
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    #[inline]
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    #[inline]
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    #[inline]
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    #[inline]
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    #[inline]
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    #[inline]
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawPutRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawPutRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawPutRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawPutRequest = RawPutRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawPutResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawPutResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawPutResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawPutResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawPutResponse = RawPutResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawBatchPutRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    #[inline]
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    #[inline]
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    #[inline]
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
    #[inline]
    pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    #[inline]
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    #[inline]
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    #[inline]
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    #[inline]
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawBatchPutRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchPutRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawBatchPutRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchPutRequest = RawBatchPutRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawBatchPutResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawBatchPutResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchPutResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawBatchPutResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchPutResponse = RawBatchPutResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawBatchGetRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    #[inline]
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    #[inline]
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    #[inline]
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    #[inline]
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    #[inline]
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    #[inline]
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    #[inline]
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    #[inline]
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawBatchGetRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchGetRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawBatchGetRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchGetRequest = RawBatchGetRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawBatchGetResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    #[inline]
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    #[inline]
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    #[inline]
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
    #[inline]
    pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for RawBatchGetResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchGetResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawBatchGetResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchGetResponse = RawBatchGetResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawDeleteRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    #[inline]
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    #[inline]
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    #[inline]
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    #[inline]
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawDeleteRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawDeleteRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawDeleteRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawDeleteRequest = RawDeleteRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawDeleteResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawDeleteResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawDeleteResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawDeleteResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawDeleteResponse = RawDeleteResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawBatchDeleteRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    #[inline]
    pub fn set_keys(&mut self, v: ::std::vec::Vec<std::vec::Vec<u8>>) {
        self.keys = v;
    }
    #[inline]
    pub fn get_keys(&self) -> &::std::vec::Vec<std::vec::Vec<u8>> {
        &self.keys
    }
    #[inline]
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<std::vec::Vec<u8>> {
        &mut self.keys
    }
    #[inline]
    pub fn take_keys(&mut self) -> ::std::vec::Vec<std::vec::Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    #[inline]
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    #[inline]
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    #[inline]
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    #[inline]
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawBatchDeleteRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchDeleteRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawBatchDeleteRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchDeleteRequest = RawBatchDeleteRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawBatchDeleteResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawBatchDeleteResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchDeleteResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawBatchDeleteResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchDeleteResponse = RawBatchDeleteResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl DeleteRangeRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    #[inline]
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    #[inline]
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    #[inline]
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    #[inline]
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    #[inline]
    pub fn set_end_key(&mut self, v: std::vec::Vec<u8>) {
        self.end_key = v;
    }
    #[inline]
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    #[inline]
    pub fn mut_end_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end_key
    }
    #[inline]
    pub fn take_end_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_notify_only(&mut self) {
        self.notify_only = false
    }
    #[inline]
    pub fn set_notify_only(&mut self, v: bool) {
        self.notify_only = v;
    }
    #[inline]
    pub fn get_notify_only(&self) -> bool {
        self.notify_only
    }
}
impl ::protobuf::Clear for DeleteRangeRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for DeleteRangeRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static DeleteRangeRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DeleteRangeRequest = DeleteRangeRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl DeleteRangeResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for DeleteRangeResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for DeleteRangeResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static DeleteRangeResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DeleteRangeResponse = DeleteRangeResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawDeleteRangeRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    #[inline]
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    #[inline]
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    #[inline]
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    #[inline]
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    #[inline]
    pub fn set_end_key(&mut self, v: std::vec::Vec<u8>) {
        self.end_key = v;
    }
    #[inline]
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    #[inline]
    pub fn mut_end_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end_key
    }
    #[inline]
    pub fn take_end_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    #[inline]
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    #[inline]
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    #[inline]
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    #[inline]
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawDeleteRangeRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawDeleteRangeRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawDeleteRangeRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawDeleteRangeRequest = RawDeleteRangeRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawDeleteRangeResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for RawDeleteRangeResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawDeleteRangeResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawDeleteRangeResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawDeleteRangeResponse = RawDeleteRangeResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawScanRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    #[inline]
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    #[inline]
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    #[inline]
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    #[inline]
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_limit(&mut self) {
        self.limit = 0
    }
    #[inline]
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }
    #[inline]
    pub fn get_limit(&self) -> u32 {
        self.limit
    }
    #[inline]
    pub fn clear_key_only(&mut self) {
        self.key_only = false
    }
    #[inline]
    pub fn set_key_only(&mut self, v: bool) {
        self.key_only = v;
    }
    #[inline]
    pub fn get_key_only(&self) -> bool {
        self.key_only
    }
    #[inline]
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    #[inline]
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    #[inline]
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    #[inline]
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    #[inline]
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_reverse(&mut self) {
        self.reverse = false
    }
    #[inline]
    pub fn set_reverse(&mut self, v: bool) {
        self.reverse = v;
    }
    #[inline]
    pub fn get_reverse(&self) -> bool {
        self.reverse
    }
    #[inline]
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    #[inline]
    pub fn set_end_key(&mut self, v: std::vec::Vec<u8>) {
        self.end_key = v;
    }
    #[inline]
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    #[inline]
    pub fn mut_end_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end_key
    }
    #[inline]
    pub fn take_end_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for RawScanRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawScanRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawScanRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawScanRequest = RawScanRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawScanResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_kvs(&mut self) {
        self.kvs.clear();
    }
    #[inline]
    pub fn set_kvs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.kvs = v;
    }
    #[inline]
    pub fn get_kvs(&self) -> &::std::vec::Vec<KvPair> {
        &self.kvs
    }
    #[inline]
    pub fn mut_kvs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.kvs
    }
    #[inline]
    pub fn take_kvs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.kvs, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for RawScanResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawScanResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawScanResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawScanResponse = RawScanResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl KeyRange {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    #[inline]
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    #[inline]
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    #[inline]
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    #[inline]
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    #[inline]
    pub fn set_end_key(&mut self, v: std::vec::Vec<u8>) {
        self.end_key = v;
    }
    #[inline]
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    #[inline]
    pub fn mut_end_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end_key
    }
    #[inline]
    pub fn take_end_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for KeyRange {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for KeyRange {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static KeyRange {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: KeyRange = KeyRange::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawBatchScanRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }
    #[inline]
    pub fn set_ranges(&mut self, v: ::std::vec::Vec<KeyRange>) {
        self.ranges = v;
    }
    #[inline]
    pub fn get_ranges(&self) -> &::std::vec::Vec<KeyRange> {
        &self.ranges
    }
    #[inline]
    pub fn mut_ranges(&mut self) -> &mut ::std::vec::Vec<KeyRange> {
        &mut self.ranges
    }
    #[inline]
    pub fn take_ranges(&mut self) -> ::std::vec::Vec<KeyRange> {
        ::std::mem::replace(&mut self.ranges, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_each_limit(&mut self) {
        self.each_limit = 0
    }
    #[inline]
    pub fn set_each_limit(&mut self, v: u32) {
        self.each_limit = v;
    }
    #[inline]
    pub fn get_each_limit(&self) -> u32 {
        self.each_limit
    }
    #[inline]
    pub fn clear_key_only(&mut self) {
        self.key_only = false
    }
    #[inline]
    pub fn set_key_only(&mut self, v: bool) {
        self.key_only = v;
    }
    #[inline]
    pub fn get_key_only(&self) -> bool {
        self.key_only
    }
    #[inline]
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    #[inline]
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    #[inline]
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    #[inline]
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    #[inline]
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_reverse(&mut self) {
        self.reverse = false
    }
    #[inline]
    pub fn set_reverse(&mut self, v: bool) {
        self.reverse = v;
    }
    #[inline]
    pub fn get_reverse(&self) -> bool {
        self.reverse
    }
}
impl ::protobuf::Clear for RawBatchScanRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchScanRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawBatchScanRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchScanRequest = RawBatchScanRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl RawBatchScanResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_kvs(&mut self) {
        self.kvs.clear();
    }
    #[inline]
    pub fn set_kvs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.kvs = v;
    }
    #[inline]
    pub fn get_kvs(&self) -> &::std::vec::Vec<KvPair> {
        &self.kvs
    }
    #[inline]
    pub fn mut_kvs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.kvs
    }
    #[inline]
    pub fn take_kvs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.kvs, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for RawBatchScanResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RawBatchScanResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static RawBatchScanResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RawBatchScanResponse = RawBatchScanResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl MvccWrite {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_field_type(&mut self) {
        self.r#type = 0
    }
    #[inline]
    pub fn set_field_type_(&mut self, v: Op) {
        self.r#type = unsafe { ::std::mem::transmute::<Op, i32>(v) };
    }
    #[inline]
    pub fn get_field_type(&self) -> Op {
        match Op::from_i32(self.r#type) {
            Some(e) => e,
            None => panic!("Unknown enum variant: {}", self.r#type),
        }
    }
    #[inline]
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    #[inline]
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
    #[inline]
    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }
    #[inline]
    pub fn clear_commit_ts(&mut self) {
        self.commit_ts = 0
    }
    #[inline]
    pub fn set_commit_ts(&mut self, v: u64) {
        self.commit_ts = v;
    }
    #[inline]
    pub fn get_commit_ts(&self) -> u64 {
        self.commit_ts
    }
    #[inline]
    pub fn clear_short_value(&mut self) {
        self.short_value.clear();
    }
    #[inline]
    pub fn set_short_value(&mut self, v: std::vec::Vec<u8>) {
        self.short_value = v;
    }
    #[inline]
    pub fn get_short_value(&self) -> &[u8] {
        &self.short_value
    }
    #[inline]
    pub fn mut_short_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.short_value
    }
    #[inline]
    pub fn take_short_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.short_value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for MvccWrite {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for MvccWrite {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static MvccWrite {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccWrite = MvccWrite::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl MvccValue {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    #[inline]
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
    #[inline]
    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    #[inline]
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    #[inline]
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    #[inline]
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    #[inline]
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for MvccValue {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for MvccValue {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static MvccValue {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccValue = MvccValue::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl MvccLock {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_field_type(&mut self) {
        self.r#type = 0
    }
    #[inline]
    pub fn set_field_type_(&mut self, v: Op) {
        self.r#type = unsafe { ::std::mem::transmute::<Op, i32>(v) };
    }
    #[inline]
    pub fn get_field_type(&self) -> Op {
        match Op::from_i32(self.r#type) {
            Some(e) => e,
            None => panic!("Unknown enum variant: {}", self.r#type),
        }
    }
    #[inline]
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    #[inline]
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
    #[inline]
    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }
    #[inline]
    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }
    #[inline]
    pub fn set_primary(&mut self, v: std::vec::Vec<u8>) {
        self.primary = v;
    }
    #[inline]
    pub fn get_primary(&self) -> &[u8] {
        &self.primary
    }
    #[inline]
    pub fn mut_primary(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.primary
    }
    #[inline]
    pub fn take_primary(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.primary, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_short_value(&mut self) {
        self.short_value.clear();
    }
    #[inline]
    pub fn set_short_value(&mut self, v: std::vec::Vec<u8>) {
        self.short_value = v;
    }
    #[inline]
    pub fn get_short_value(&self) -> &[u8] {
        &self.short_value
    }
    #[inline]
    pub fn mut_short_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.short_value
    }
    #[inline]
    pub fn take_short_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.short_value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for MvccLock {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for MvccLock {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static MvccLock {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccLock = MvccLock::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl MvccInfo {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_lock(&self) -> bool {
        self.lock.is_some()
    }
    #[inline]
    pub fn clear_lock(&mut self) {
        self.lock = ::std::option::Option::None
    }
    #[inline]
    pub fn set_lock(&mut self, v: MvccLock) {
        self.lock = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_lock(&self) -> &MvccLock {
        match self.lock.as_ref() {
            Some(v) => v,
            None => MvccLock::default_ref(),
        }
    }
    #[inline]
    pub fn mut_lock(&mut self) -> &mut MvccLock {
        if self.lock.is_none() {
            self.lock = ::std::option::Option::Some(MvccLock::default());
        }
        self.lock.as_mut().unwrap()
    }
    #[inline]
    pub fn take_lock(&mut self) -> MvccLock {
        self.lock.take().unwrap_or_else(MvccLock::default)
    }
    #[inline]
    pub fn clear_writes(&mut self) {
        self.writes.clear();
    }
    #[inline]
    pub fn set_writes(&mut self, v: ::std::vec::Vec<MvccWrite>) {
        self.writes = v;
    }
    #[inline]
    pub fn get_writes(&self) -> &::std::vec::Vec<MvccWrite> {
        &self.writes
    }
    #[inline]
    pub fn mut_writes(&mut self) -> &mut ::std::vec::Vec<MvccWrite> {
        &mut self.writes
    }
    #[inline]
    pub fn take_writes(&mut self) -> ::std::vec::Vec<MvccWrite> {
        ::std::mem::replace(&mut self.writes, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_values(&mut self) {
        self.values.clear();
    }
    #[inline]
    pub fn set_values(&mut self, v: ::std::vec::Vec<MvccValue>) {
        self.values = v;
    }
    #[inline]
    pub fn get_values(&self) -> &::std::vec::Vec<MvccValue> {
        &self.values
    }
    #[inline]
    pub fn mut_values(&mut self) -> &mut ::std::vec::Vec<MvccValue> {
        &mut self.values
    }
    #[inline]
    pub fn take_values(&mut self) -> ::std::vec::Vec<MvccValue> {
        ::std::mem::replace(&mut self.values, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for MvccInfo {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for MvccInfo {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static MvccInfo {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccInfo = MvccInfo::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl MvccGetByKeyRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for MvccGetByKeyRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for MvccGetByKeyRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static MvccGetByKeyRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccGetByKeyRequest = MvccGetByKeyRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl MvccGetByKeyResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
    #[inline]
    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }
    #[inline]
    pub fn clear_info(&mut self) {
        self.info = ::std::option::Option::None
    }
    #[inline]
    pub fn set_info(&mut self, v: MvccInfo) {
        self.info = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_info(&self) -> &MvccInfo {
        match self.info.as_ref() {
            Some(v) => v,
            None => MvccInfo::default_ref(),
        }
    }
    #[inline]
    pub fn mut_info(&mut self) -> &mut MvccInfo {
        if self.info.is_none() {
            self.info = ::std::option::Option::Some(MvccInfo::default());
        }
        self.info.as_mut().unwrap()
    }
    #[inline]
    pub fn take_info(&mut self) -> MvccInfo {
        self.info.take().unwrap_or_else(MvccInfo::default)
    }
}
impl ::protobuf::Clear for MvccGetByKeyResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for MvccGetByKeyResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static MvccGetByKeyResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccGetByKeyResponse = MvccGetByKeyResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl MvccGetByStartTsRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    #[inline]
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
    #[inline]
    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }
}
impl ::protobuf::Clear for MvccGetByStartTsRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for MvccGetByStartTsRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static MvccGetByStartTsRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccGetByStartTsRequest = MvccGetByStartTsRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl MvccGetByStartTsResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }
    #[inline]
    pub fn clear_info(&mut self) {
        self.info = ::std::option::Option::None
    }
    #[inline]
    pub fn set_info(&mut self, v: MvccInfo) {
        self.info = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_info(&self) -> &MvccInfo {
        match self.info.as_ref() {
            Some(v) => v,
            None => MvccInfo::default_ref(),
        }
    }
    #[inline]
    pub fn mut_info(&mut self) -> &mut MvccInfo {
        if self.info.is_none() {
            self.info = ::std::option::Option::Some(MvccInfo::default());
        }
        self.info.as_mut().unwrap()
    }
    #[inline]
    pub fn take_info(&mut self) -> MvccInfo {
        self.info.take().unwrap_or_else(MvccInfo::default)
    }
}
impl ::protobuf::Clear for MvccGetByStartTsResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for MvccGetByStartTsResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static MvccGetByStartTsResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MvccGetByStartTsResponse = MvccGetByStartTsResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl SplitRegionRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_split_key(&mut self) {
        self.split_key.clear();
    }
    #[inline]
    pub fn set_split_key(&mut self, v: std::vec::Vec<u8>) {
        self.split_key = v;
    }
    #[inline]
    pub fn get_split_key(&self) -> &[u8] {
        &self.split_key
    }
    #[inline]
    pub fn mut_split_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.split_key
    }
    #[inline]
    pub fn take_split_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.split_key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for SplitRegionRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SplitRegionRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static SplitRegionRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SplitRegionRequest = SplitRegionRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl SplitRegionResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn has_left(&self) -> bool {
        self.left.is_some()
    }
    #[inline]
    pub fn clear_left(&mut self) {
        self.left = ::std::option::Option::None
    }
    #[inline]
    pub fn set_left(&mut self, v: super::metapb::Region) {
        self.left = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_left(&self) -> &super::metapb::Region {
        match self.left.as_ref() {
            Some(v) => v,
            None => super::metapb::Region::default_ref(),
        }
    }
    #[inline]
    pub fn mut_left(&mut self) -> &mut super::metapb::Region {
        if self.left.is_none() {
            self.left = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.left.as_mut().unwrap()
    }
    #[inline]
    pub fn take_left(&mut self) -> super::metapb::Region {
        self.left
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
    #[inline]
    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }
    #[inline]
    pub fn clear_right(&mut self) {
        self.right = ::std::option::Option::None
    }
    #[inline]
    pub fn set_right(&mut self, v: super::metapb::Region) {
        self.right = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_right(&self) -> &super::metapb::Region {
        match self.right.as_ref() {
            Some(v) => v,
            None => super::metapb::Region::default_ref(),
        }
    }
    #[inline]
    pub fn mut_right(&mut self) -> &mut super::metapb::Region {
        if self.right.is_none() {
            self.right = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.right.as_mut().unwrap()
    }
    #[inline]
    pub fn take_right(&mut self) -> super::metapb::Region {
        self.right
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
}
impl ::protobuf::Clear for SplitRegionResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SplitRegionResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static SplitRegionResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SplitRegionResponse = SplitRegionResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl UnsafeDestroyRangeRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
    #[inline]
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    #[inline]
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    #[inline]
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    #[inline]
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    #[inline]
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    #[inline]
    pub fn set_end_key(&mut self, v: std::vec::Vec<u8>) {
        self.end_key = v;
    }
    #[inline]
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    #[inline]
    pub fn mut_end_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end_key
    }
    #[inline]
    pub fn take_end_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for UnsafeDestroyRangeRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for UnsafeDestroyRangeRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static UnsafeDestroyRangeRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UnsafeDestroyRangeRequest = UnsafeDestroyRangeRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl UnsafeDestroyRangeResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    #[inline]
    pub fn set_error(&mut self, v: std::string::String) {
        self.error = v;
    }
    #[inline]
    pub fn get_error(&self) -> &str {
        &self.error
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut std::string::String {
        &mut self.error
    }
    #[inline]
    pub fn take_error(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for UnsafeDestroyRangeResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for UnsafeDestroyRangeResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static UnsafeDestroyRangeResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UnsafeDestroyRangeResponse = UnsafeDestroyRangeResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ReadIndexRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(Context::default)
    }
}
impl ::protobuf::Clear for ReadIndexRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ReadIndexRequest {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ReadIndexRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ReadIndexRequest = ReadIndexRequest::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl ReadIndexResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    #[inline]
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    #[inline]
    pub fn clear_read_index(&mut self) {
        self.read_index = 0
    }
    #[inline]
    pub fn set_read_index(&mut self, v: u64) {
        self.read_index = v;
    }
    #[inline]
    pub fn get_read_index(&self) -> u64 {
        self.read_index
    }
}
impl ::protobuf::Clear for ReadIndexResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ReadIndexResponse {
    fn compute_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn get_cached_size(&self) -> u32 {
        ::prost::Message::encoded_len(self) as u32
    }
    fn as_any(&self) -> &dyn::std::any::Any {
        self as &dyn::std::any::Any
    }
    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }
    fn new() -> Self {
        Self::default()
    }
    fn default_instance() -> &'static ReadIndexResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ReadIndexResponse = ReadIndexResponse::default();
        }
        &*INSTANCE
    }
    fn is_initialized(&self) -> bool {
        true
    }
    fn write_to_with_cached_sizes(
        &self,
        _os: &mut ::protobuf::CodedOutputStream,
    ) -> ::protobuf::ProtobufResult<()> {
        unimplemented!();
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
        if ::prost::Message::encode(self, &mut buf).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(buf)
    }
    fn merge_from_bytes(&mut self, bytes: &[u8]) -> ::protobuf::ProtobufResult<()> {
        if ::prost::Message::merge(self, bytes).is_err() {
            return Err(::protobuf::ProtobufError::WireError(
                ::protobuf::error::WireError::Other,
            ));
        }
        Ok(())
    }
}
impl CommandPri {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [CommandPri] =
            &[CommandPri::Normal, CommandPri::Low, CommandPri::High];
        VALUES
    }
}
impl IsolationLevel {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [IsolationLevel] = &[IsolationLevel::Si, IsolationLevel::Rc];
        VALUES
    }
}
impl Op {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [Op] = &[
            Op::Put,
            Op::Del,
            Op::Lock,
            Op::Rollback,
            Op::Insert,
            Op::PessimisticLock,
        ];
        VALUES
    }
}
impl Assertion {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [Assertion] =
            &[Assertion::None, Assertion::Exist, Assertion::NotExist];
        VALUES
    }
}
