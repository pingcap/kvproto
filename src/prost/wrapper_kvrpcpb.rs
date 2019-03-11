impl LockInfo {
    pub fn new_() -> LockInfo {
        ::std::default::Default::default()
    }
    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }
    pub fn set_primary_lock(&mut self, v: Vec<u8>) {
        self.primary_lock = v;
    }
    pub fn get_primary_lock(&self) -> &[u8] {
        &self.primary_lock
    }
    pub fn mut_primary_lock(&mut self) -> &mut Vec<u8> {
        &mut self.primary_lock
    }
    pub fn take_primary_lock(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.primary_lock, ::std::vec::Vec::new())
    }
    pub fn clear_lock_version(&mut self) {
        self.lock_version = 0
    }
    pub fn set_lock_version(&mut self, v: u64) {
        self.lock_version = v;
    }
    pub fn get_lock_version(&self) -> u64 {
        self.lock_version
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = 0
    }
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = v;
    }
    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl
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
    fn default_instance() -> &'static LockInfo {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl AlreadyExist {
    pub fn new_() -> AlreadyExist {
        ::std::default::Default::default()
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static AlreadyExist {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl KeyError {
    pub fn new_() -> KeyError {
        ::std::default::Default::default()
    }
    pub fn has_locked(&self) -> bool {
        self.locked.is_some()
    }
    pub fn clear_locked(&mut self) {
        self.locked = ::std::option::Option::None
    }
    pub fn set_locked(&mut self, v: LockInfo) {
        self.locked = ::std::option::Option::Some(v);;    }
    pub fn get_locked(&self) -> &LockInfo {
        match self.locked.as_ref() {
            Some(v) => v,
            None => <LockInfo as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_locked(&mut self) -> &mut LockInfo {
        if self.locked.is_none() {
            self.locked = ::std::option::Option::Some(LockInfo::default());
        }
        self.locked.as_mut().unwrap()
    }
    pub fn take_locked(&mut self) -> LockInfo {
        self.locked.take().unwrap_or_else(|| LockInfo::default())
    }
    pub fn clear_retryable(&mut self) {
        self.retryable.clear();
    }
    pub fn set_retryable(&mut self, v: String) {
        self.retryable = v;
    }
    pub fn get_retryable(&self) -> &str {
        &self.retryable
    }
    pub fn mut_retryable(&mut self) -> &mut String {
        &mut self.retryable
    }
    pub fn take_retryable(&mut self) -> String {
        ::std::mem::replace(&mut self.retryable, ::std::string::String::new())
    }
    pub fn clear_abort(&mut self) {
        self.abort.clear();
    }
    pub fn set_abort(&mut self, v: String) {
        self.abort = v;
    }
    pub fn get_abort(&self) -> &str {
        &self.abort
    }
    pub fn mut_abort(&mut self) -> &mut String {
        &mut self.abort
    }
    pub fn take_abort(&mut self) -> String {
        ::std::mem::replace(&mut self.abort, ::std::string::String::new())
    }
    pub fn has_conflict(&self) -> bool {
        self.conflict.is_some()
    }
    pub fn clear_conflict(&mut self) {
        self.conflict = ::std::option::Option::None
    }
    pub fn set_conflict(&mut self, v: WriteConflict) {
        self.conflict = ::std::option::Option::Some(v);;    }
    pub fn get_conflict(&self) -> &WriteConflict {
        match self.conflict.as_ref() {
            Some(v) => v,
            None => <WriteConflict as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_conflict(&mut self) -> &mut WriteConflict {
        if self.conflict.is_none() {
            self.conflict = ::std::option::Option::Some(WriteConflict::default());
        }
        self.conflict.as_mut().unwrap()
    }
    pub fn take_conflict(&mut self) -> WriteConflict {
        self.conflict
            .take()
            .unwrap_or_else(|| WriteConflict::default())
    }
    pub fn has_already_exist(&self) -> bool {
        self.already_exist.is_some()
    }
    pub fn clear_already_exist(&mut self) {
        self.already_exist = ::std::option::Option::None
    }
    pub fn set_already_exist(&mut self, v: AlreadyExist) {
        self.already_exist = ::std::option::Option::Some(v);;    }
    pub fn get_already_exist(&self) -> &AlreadyExist {
        match self.already_exist.as_ref() {
            Some(v) => v,
            None => <AlreadyExist as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_already_exist(&mut self) -> &mut AlreadyExist {
        if self.already_exist.is_none() {
            self.already_exist = ::std::option::Option::Some(AlreadyExist::default());
        }
        self.already_exist.as_mut().unwrap()
    }
    pub fn take_already_exist(&mut self) -> AlreadyExist {
        self.already_exist
            .take()
            .unwrap_or_else(|| AlreadyExist::default())
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
    fn default_instance() -> &'static KeyError {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl WriteConflict {
    pub fn new_() -> WriteConflict {
        ::std::default::Default::default()
    }
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }
    pub fn clear_conflict_ts(&mut self) {
        self.conflict_ts = 0
    }
    pub fn set_conflict_ts(&mut self, v: u64) {
        self.conflict_ts = v;
    }
    pub fn get_conflict_ts(&self) -> u64 {
        self.conflict_ts
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }
    pub fn set_primary(&mut self, v: Vec<u8>) {
        self.primary = v;
    }
    pub fn get_primary(&self) -> &[u8] {
        &self.primary
    }
    pub fn mut_primary(&mut self) -> &mut Vec<u8> {
        &mut self.primary
    }
    pub fn take_primary(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.primary, ::std::vec::Vec::new())
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
    fn default_instance() -> &'static WriteConflict {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl Context {
    pub fn new_() -> Context {
        ::std::default::Default::default()
    }
    pub fn clear_region_id(&mut self) {
        self.region_id = 0
    }
    pub fn set_region_id(&mut self, v: u64) {
        self.region_id = v;
    }
    pub fn get_region_id(&self) -> u64 {
        self.region_id
    }
    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }
    pub fn clear_region_epoch(&mut self) {
        self.region_epoch = ::std::option::Option::None
    }
    pub fn set_region_epoch(&mut self, v: super::metapb::RegionEpoch) {
        self.region_epoch = ::std::option::Option::Some(v);;    }
    pub fn get_region_epoch(&self) -> &super::metapb::RegionEpoch {
        match self.region_epoch.as_ref() {
            Some(v) => v,
            None => <super::metapb::RegionEpoch as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_epoch(&mut self) -> &mut super::metapb::RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch = ::std::option::Option::Some(super::metapb::RegionEpoch::default());
        }
        self.region_epoch.as_mut().unwrap()
    }
    pub fn take_region_epoch(&mut self) -> super::metapb::RegionEpoch {
        self.region_epoch
            .take()
            .unwrap_or_else(|| super::metapb::RegionEpoch::default())
    }
    pub fn has_peer(&self) -> bool {
        self.peer.is_some()
    }
    pub fn clear_peer(&mut self) {
        self.peer = ::std::option::Option::None
    }
    pub fn set_peer(&mut self, v: super::metapb::Peer) {
        self.peer = ::std::option::Option::Some(v);;    }
    pub fn get_peer(&self) -> &super::metapb::Peer {
        match self.peer.as_ref() {
            Some(v) => v,
            None => <super::metapb::Peer as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_peer(&mut self) -> &mut super::metapb::Peer {
        if self.peer.is_none() {
            self.peer = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.peer.as_mut().unwrap()
    }
    pub fn take_peer(&mut self) -> super::metapb::Peer {
        self.peer
            .take()
            .unwrap_or_else(|| super::metapb::Peer::default())
    }
    pub fn clear_term(&mut self) {
        self.term = 0
    }
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }
    pub fn get_term(&self) -> u64 {
        self.term
    }
    pub fn clear_priority(&mut self) {
        self.priority = 0
    }
    pub fn set_priority_(&mut self, v: CommandPri) {
        self.priority = unsafe { ::std::mem::transmute::<CommandPri, i32>(v) };
    }
    pub fn get_priority(&self) -> CommandPri {
        unsafe { ::std::mem::transmute::<i32, CommandPri>(self.priority) }
    }
    pub fn clear_isolation_level(&mut self) {
        self.isolation_level = 0
    }
    pub fn set_isolation_level_(&mut self, v: IsolationLevel) {
        self.isolation_level = unsafe { ::std::mem::transmute::<IsolationLevel, i32>(v) };
    }
    pub fn get_isolation_level(&self) -> IsolationLevel {
        unsafe { ::std::mem::transmute::<i32, IsolationLevel>(self.isolation_level) }
    }
    pub fn clear_not_fill_cache(&mut self) {
        self.not_fill_cache = false
    }
    pub fn set_not_fill_cache(&mut self, v: bool) {
        self.not_fill_cache = v;
    }
    pub fn get_not_fill_cache(&self) -> bool {
        self.not_fill_cache
    }
    pub fn clear_sync_log(&mut self) {
        self.sync_log = false
    }
    pub fn set_sync_log(&mut self, v: bool) {
        self.sync_log = v;
    }
    pub fn get_sync_log(&self) -> bool {
        self.sync_log
    }
    pub fn clear_handle_time(&mut self) {
        self.handle_time = false
    }
    pub fn set_handle_time(&mut self, v: bool) {
        self.handle_time = v;
    }
    pub fn get_handle_time(&self) -> bool {
        self.handle_time
    }
    pub fn clear_scan_detail(&mut self) {
        self.scan_detail = false
    }
    pub fn set_scan_detail(&mut self, v: bool) {
        self.scan_detail = v;
    }
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
    fn default_instance() -> &'static Context {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl HandleTime {
    pub fn new_() -> HandleTime {
        ::std::default::Default::default()
    }
    pub fn clear_wait_ms(&mut self) {
        self.wait_ms = 0
    }
    pub fn set_wait_ms(&mut self, v: i64) {
        self.wait_ms = v;
    }
    pub fn get_wait_ms(&self) -> i64 {
        self.wait_ms
    }
    pub fn clear_process_ms(&mut self) {
        self.process_ms = 0
    }
    pub fn set_process_ms(&mut self, v: i64) {
        self.process_ms = v;
    }
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
    fn default_instance() -> &'static HandleTime {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ScanInfo {
    pub fn new_() -> ScanInfo {
        ::std::default::Default::default()
    }
    pub fn clear_total(&mut self) {
        self.total = 0
    }
    pub fn set_total(&mut self, v: i64) {
        self.total = v;
    }
    pub fn get_total(&self) -> i64 {
        self.total
    }
    pub fn clear_processed(&mut self) {
        self.processed = 0
    }
    pub fn set_processed(&mut self, v: i64) {
        self.processed = v;
    }
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
    fn default_instance() -> &'static ScanInfo {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ScanDetail {
    pub fn new_() -> ScanDetail {
        ::std::default::Default::default()
    }
    pub fn has_write(&self) -> bool {
        self.write.is_some()
    }
    pub fn clear_write(&mut self) {
        self.write = ::std::option::Option::None
    }
    pub fn set_write(&mut self, v: ScanInfo) {
        self.write = ::std::option::Option::Some(v);;    }
    pub fn get_write(&self) -> &ScanInfo {
        match self.write.as_ref() {
            Some(v) => v,
            None => <ScanInfo as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_write(&mut self) -> &mut ScanInfo {
        if self.write.is_none() {
            self.write = ::std::option::Option::Some(ScanInfo::default());
        }
        self.write.as_mut().unwrap()
    }
    pub fn take_write(&mut self) -> ScanInfo {
        self.write.take().unwrap_or_else(|| ScanInfo::default())
    }
    pub fn has_lock(&self) -> bool {
        self.lock.is_some()
    }
    pub fn clear_lock(&mut self) {
        self.lock = ::std::option::Option::None
    }
    pub fn set_lock(&mut self, v: ScanInfo) {
        self.lock = ::std::option::Option::Some(v);;    }
    pub fn get_lock(&self) -> &ScanInfo {
        match self.lock.as_ref() {
            Some(v) => v,
            None => <ScanInfo as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_lock(&mut self) -> &mut ScanInfo {
        if self.lock.is_none() {
            self.lock = ::std::option::Option::Some(ScanInfo::default());
        }
        self.lock.as_mut().unwrap()
    }
    pub fn take_lock(&mut self) -> ScanInfo {
        self.lock.take().unwrap_or_else(|| ScanInfo::default())
    }
    pub fn has_data(&self) -> bool {
        self.data.is_some()
    }
    pub fn clear_data(&mut self) {
        self.data = ::std::option::Option::None
    }
    pub fn set_data(&mut self, v: ScanInfo) {
        self.data = ::std::option::Option::Some(v);;    }
    pub fn get_data(&self) -> &ScanInfo {
        match self.data.as_ref() {
            Some(v) => v,
            None => <ScanInfo as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_data(&mut self) -> &mut ScanInfo {
        if self.data.is_none() {
            self.data = ::std::option::Option::Some(ScanInfo::default());
        }
        self.data.as_mut().unwrap()
    }
    pub fn take_data(&mut self) -> ScanInfo {
        self.data.take().unwrap_or_else(|| ScanInfo::default())
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
    fn default_instance() -> &'static ScanDetail {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ExecDetails {
    pub fn new_() -> ExecDetails {
        ::std::default::Default::default()
    }
    pub fn has_handle_time(&self) -> bool {
        self.handle_time.is_some()
    }
    pub fn clear_handle_time(&mut self) {
        self.handle_time = ::std::option::Option::None
    }
    pub fn set_handle_time(&mut self, v: HandleTime) {
        self.handle_time = ::std::option::Option::Some(v);;    }
    pub fn get_handle_time(&self) -> &HandleTime {
        match self.handle_time.as_ref() {
            Some(v) => v,
            None => <HandleTime as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_handle_time(&mut self) -> &mut HandleTime {
        if self.handle_time.is_none() {
            self.handle_time = ::std::option::Option::Some(HandleTime::default());
        }
        self.handle_time.as_mut().unwrap()
    }
    pub fn take_handle_time(&mut self) -> HandleTime {
        self.handle_time
            .take()
            .unwrap_or_else(|| HandleTime::default())
    }
    pub fn has_scan_detail(&self) -> bool {
        self.scan_detail.is_some()
    }
    pub fn clear_scan_detail(&mut self) {
        self.scan_detail = ::std::option::Option::None
    }
    pub fn set_scan_detail(&mut self, v: ScanDetail) {
        self.scan_detail = ::std::option::Option::Some(v);;    }
    pub fn get_scan_detail(&self) -> &ScanDetail {
        match self.scan_detail.as_ref() {
            Some(v) => v,
            None => <ScanDetail as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_scan_detail(&mut self) -> &mut ScanDetail {
        if self.scan_detail.is_none() {
            self.scan_detail = ::std::option::Option::Some(ScanDetail::default());
        }
        self.scan_detail.as_mut().unwrap()
    }
    pub fn take_scan_detail(&mut self) -> ScanDetail {
        self.scan_detail
            .take()
            .unwrap_or_else(|| ScanDetail::default())
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
    fn default_instance() -> &'static ExecDetails {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl GetRequest {
    pub fn new_() -> GetRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
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
    fn default_instance() -> &'static GetRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl GetResponse {
    pub fn new_() -> GetResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::default())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static GetResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ScanRequest {
    pub fn new_() -> ScanRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_limit(&mut self) {
        self.limit = 0
    }
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }
    pub fn get_limit(&self) -> u32 {
        self.limit
    }
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
    pub fn get_version(&self) -> u64 {
        self.version
    }
    pub fn clear_key_only(&mut self) {
        self.key_only = false
    }
    pub fn set_key_only(&mut self, v: bool) {
        self.key_only = v;
    }
    pub fn get_key_only(&self) -> bool {
        self.key_only
    }
    pub fn clear_reverse(&mut self) {
        self.reverse = false
    }
    pub fn set_reverse(&mut self, v: bool) {
        self.reverse = v;
    }
    pub fn get_reverse(&self) -> bool {
        self.reverse
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static ScanRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl KvPair {
    pub fn new_() -> KvPair {
        ::std::default::Default::default()
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::default())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static KvPair {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ScanResponse {
    pub fn new_() -> ScanResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
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
    fn default_instance() -> &'static ScanResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl Mutation {
    pub fn new_() -> Mutation {
        ::std::default::Default::default()
    }
    pub fn clear_op(&mut self) {
        self.op = 0
    }
    pub fn set_op_(&mut self, v: Op) {
        self.op = unsafe { ::std::mem::transmute::<Op, i32>(v) };
    }
    pub fn get_op(&self) -> Op {
        unsafe { ::std::mem::transmute::<i32, Op>(self.op) }
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
    pub fn clear_assertion(&mut self) {
        self.assertion = 0
    }
    pub fn set_assertion_(&mut self, v: Assertion) {
        self.assertion = unsafe { ::std::mem::transmute::<Assertion, i32>(v) };
    }
    pub fn get_assertion(&self) -> Assertion {
        unsafe { ::std::mem::transmute::<i32, Assertion>(self.assertion) }
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
    fn default_instance() -> &'static Mutation {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl PrewriteRequest {
    pub fn new_() -> PrewriteRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }
    pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) {
        self.mutations = v;
    }
    pub fn get_mutations(&self) -> &::std::vec::Vec<Mutation> {
        &self.mutations
    }
    pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> {
        &mut self.mutations
    }
    pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new())
    }
    pub fn clear_primary_lock(&mut self) {
        self.primary_lock.clear();
    }
    pub fn set_primary_lock(&mut self, v: Vec<u8>) {
        self.primary_lock = v;
    }
    pub fn get_primary_lock(&self) -> &[u8] {
        &self.primary_lock
    }
    pub fn mut_primary_lock(&mut self) -> &mut Vec<u8> {
        &mut self.primary_lock
    }
    pub fn take_primary_lock(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.primary_lock, ::std::vec::Vec::new())
    }
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    pub fn clear_lock_ttl(&mut self) {
        self.lock_ttl = 0
    }
    pub fn set_lock_ttl(&mut self, v: u64) {
        self.lock_ttl = v;
    }
    pub fn get_lock_ttl(&self) -> u64 {
        self.lock_ttl
    }
    pub fn clear_skip_constraint_check(&mut self) {
        self.skip_constraint_check = false
    }
    pub fn set_skip_constraint_check(&mut self, v: bool) {
        self.skip_constraint_check = v;
    }
    pub fn get_skip_constraint_check(&self) -> bool {
        self.skip_constraint_check
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
    fn default_instance() -> &'static PrewriteRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl PrewriteResponse {
    pub fn new_() -> PrewriteResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }
    pub fn set_errors(&mut self, v: ::std::vec::Vec<KeyError>) {
        self.errors = v;
    }
    pub fn get_errors(&self) -> &::std::vec::Vec<KeyError> {
        &self.errors
    }
    pub fn mut_errors(&mut self) -> &mut ::std::vec::Vec<KeyError> {
        &mut self.errors
    }
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
    fn default_instance() -> &'static PrewriteResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl CommitRequest {
    pub fn new_() -> CommitRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    pub fn set_keys(&mut self, v: ::std::vec::Vec<Vec<u8>>) {
        self.keys = v;
    }
    pub fn get_keys(&self) -> &::std::vec::Vec<Vec<u8>> {
        &self.keys
    }
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<Vec<u8>> {
        &mut self.keys
    }
    pub fn take_keys(&mut self) -> ::std::vec::Vec<Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
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
    fn default_instance() -> &'static CommitRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl CommitResponse {
    pub fn new_() -> CommitResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::default())
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
    fn default_instance() -> &'static CommitResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ImportRequest {
    pub fn new_() -> ImportRequest {
        ::std::default::Default::default()
    }
    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }
    pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) {
        self.mutations = v;
    }
    pub fn get_mutations(&self) -> &::std::vec::Vec<Mutation> {
        &self.mutations
    }
    pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> {
        &mut self.mutations
    }
    pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new())
    }
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
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
    fn default_instance() -> &'static ImportRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ImportResponse {
    pub fn new_() -> ImportResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
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
    fn default_instance() -> &'static ImportResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl BatchRollbackRequest {
    pub fn new_() -> BatchRollbackRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    pub fn set_keys(&mut self, v: ::std::vec::Vec<Vec<u8>>) {
        self.keys = v;
    }
    pub fn get_keys(&self) -> &::std::vec::Vec<Vec<u8>> {
        &self.keys
    }
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<Vec<u8>> {
        &mut self.keys
    }
    pub fn take_keys(&mut self) -> ::std::vec::Vec<Vec<u8>> {
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
    fn default_instance() -> &'static BatchRollbackRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl BatchRollbackResponse {
    pub fn new_() -> BatchRollbackResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::default())
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
    fn default_instance() -> &'static BatchRollbackResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl CleanupRequest {
    pub fn new_() -> CleanupRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
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
    fn default_instance() -> &'static CleanupRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl CleanupResponse {
    pub fn new_() -> CleanupResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::default())
    }
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
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
    fn default_instance() -> &'static CleanupResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl BatchGetRequest {
    pub fn new_() -> BatchGetRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    pub fn set_keys(&mut self, v: ::std::vec::Vec<Vec<u8>>) {
        self.keys = v;
    }
    pub fn get_keys(&self) -> &::std::vec::Vec<Vec<u8>> {
        &self.keys
    }
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<Vec<u8>> {
        &mut self.keys
    }
    pub fn take_keys(&mut self) -> ::std::vec::Vec<Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
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
    fn default_instance() -> &'static BatchGetRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl BatchGetResponse {
    pub fn new_() -> BatchGetResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
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
    fn default_instance() -> &'static BatchGetResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ScanLockRequest {
    pub fn new_() -> ScanLockRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_max_version(&mut self) {
        self.max_version = 0
    }
    pub fn set_max_version(&mut self, v: u64) {
        self.max_version = v;
    }
    pub fn get_max_version(&self) -> u64 {
        self.max_version
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_limit(&mut self) {
        self.limit = 0
    }
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }
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
    fn default_instance() -> &'static ScanLockRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ScanLockResponse {
    pub fn new_() -> ScanLockResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::default())
    }
    pub fn clear_locks(&mut self) {
        self.locks.clear();
    }
    pub fn set_locks(&mut self, v: ::std::vec::Vec<LockInfo>) {
        self.locks = v;
    }
    pub fn get_locks(&self) -> &::std::vec::Vec<LockInfo> {
        &self.locks
    }
    pub fn mut_locks(&mut self) -> &mut ::std::vec::Vec<LockInfo> {
        &mut self.locks
    }
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
    fn default_instance() -> &'static ScanLockResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl TxnInfo {
    pub fn new_() -> TxnInfo {
        ::std::default::Default::default()
    }
    pub fn clear_txn(&mut self) {
        self.txn = 0
    }
    pub fn set_txn(&mut self, v: u64) {
        self.txn = v;
    }
    pub fn get_txn(&self) -> u64 {
        self.txn
    }
    pub fn clear_status(&mut self) {
        self.status = 0
    }
    pub fn set_status(&mut self, v: u64) {
        self.status = v;
    }
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
    fn default_instance() -> &'static TxnInfo {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ResolveLockRequest {
    pub fn new_() -> ResolveLockRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_start_version(&mut self) {
        self.start_version = 0
    }
    pub fn set_start_version(&mut self, v: u64) {
        self.start_version = v;
    }
    pub fn get_start_version(&self) -> u64 {
        self.start_version
    }
    pub fn clear_commit_version(&mut self) {
        self.commit_version = 0
    }
    pub fn set_commit_version(&mut self, v: u64) {
        self.commit_version = v;
    }
    pub fn get_commit_version(&self) -> u64 {
        self.commit_version
    }
    pub fn clear_txn_infos(&mut self) {
        self.txn_infos.clear();
    }
    pub fn set_txn_infos(&mut self, v: ::std::vec::Vec<TxnInfo>) {
        self.txn_infos = v;
    }
    pub fn get_txn_infos(&self) -> &::std::vec::Vec<TxnInfo> {
        &self.txn_infos
    }
    pub fn mut_txn_infos(&mut self) -> &mut ::std::vec::Vec<TxnInfo> {
        &mut self.txn_infos
    }
    pub fn take_txn_infos(&mut self) -> ::std::vec::Vec<TxnInfo> {
        ::std::mem::replace(&mut self.txn_infos, ::std::vec::Vec::new())
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
    fn default_instance() -> &'static ResolveLockRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl ResolveLockResponse {
    pub fn new_() -> ResolveLockResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::default())
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
    fn default_instance() -> &'static ResolveLockResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl GcRequest {
    pub fn new_() -> GcRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_safe_point(&mut self) {
        self.safe_point = 0
    }
    pub fn set_safe_point(&mut self, v: u64) {
        self.safe_point = v;
    }
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
    fn default_instance() -> &'static GcRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl GcResponse {
    pub fn new_() -> GcResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: KeyError) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &KeyError {
        match self.error.as_ref() {
            Some(v) => v,
            None => <KeyError as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut KeyError {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(KeyError::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> KeyError {
        self.error.take().unwrap_or_else(|| KeyError::default())
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
    fn default_instance() -> &'static GcResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawGetRequest {
    pub fn new_() -> RawGetRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> String {
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
    fn default_instance() -> &'static RawGetRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawGetResponse {
    pub fn new_() -> RawGetResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static RawGetResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawPutRequest {
    pub fn new_() -> RawPutRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> String {
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
    fn default_instance() -> &'static RawPutRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawPutResponse {
    pub fn new_() -> RawPutResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
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
    fn default_instance() -> &'static RawPutResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawBatchPutRequest {
    pub fn new_() -> RawBatchPutRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
    pub fn take_pairs(&mut self) -> ::std::vec::Vec<KvPair> {
        ::std::mem::replace(&mut self.pairs, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> String {
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
    fn default_instance() -> &'static RawBatchPutRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawBatchPutResponse {
    pub fn new_() -> RawBatchPutResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
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
    fn default_instance() -> &'static RawBatchPutResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawBatchGetRequest {
    pub fn new_() -> RawBatchGetRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    pub fn set_keys(&mut self, v: ::std::vec::Vec<Vec<u8>>) {
        self.keys = v;
    }
    pub fn get_keys(&self) -> &::std::vec::Vec<Vec<u8>> {
        &self.keys
    }
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<Vec<u8>> {
        &mut self.keys
    }
    pub fn take_keys(&mut self) -> ::std::vec::Vec<Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> String {
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
    fn default_instance() -> &'static RawBatchGetRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawBatchGetResponse {
    pub fn new_() -> RawBatchGetResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_pairs(&mut self) {
        self.pairs.clear();
    }
    pub fn set_pairs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.pairs = v;
    }
    pub fn get_pairs(&self) -> &::std::vec::Vec<KvPair> {
        &self.pairs
    }
    pub fn mut_pairs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.pairs
    }
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
    fn default_instance() -> &'static RawBatchGetResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawDeleteRequest {
    pub fn new_() -> RawDeleteRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> String {
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
    fn default_instance() -> &'static RawDeleteRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawDeleteResponse {
    pub fn new_() -> RawDeleteResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
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
    fn default_instance() -> &'static RawDeleteResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawBatchDeleteRequest {
    pub fn new_() -> RawBatchDeleteRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_keys(&mut self) {
        self.keys.clear();
    }
    pub fn set_keys(&mut self, v: ::std::vec::Vec<Vec<u8>>) {
        self.keys = v;
    }
    pub fn get_keys(&self) -> &::std::vec::Vec<Vec<u8>> {
        &self.keys
    }
    pub fn mut_keys(&mut self) -> &mut ::std::vec::Vec<Vec<u8>> {
        &mut self.keys
    }
    pub fn take_keys(&mut self) -> ::std::vec::Vec<Vec<u8>> {
        ::std::mem::replace(&mut self.keys, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> String {
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
    fn default_instance() -> &'static RawBatchDeleteRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawBatchDeleteResponse {
    pub fn new_() -> RawBatchDeleteResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
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
    fn default_instance() -> &'static RawBatchDeleteResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl DeleteRangeRequest {
    pub fn new_() -> DeleteRangeRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
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
    fn default_instance() -> &'static DeleteRangeRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl DeleteRangeResponse {
    pub fn new_() -> DeleteRangeResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
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
    fn default_instance() -> &'static DeleteRangeResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawDeleteRangeRequest {
    pub fn new_() -> RawDeleteRangeRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> String {
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
    fn default_instance() -> &'static RawDeleteRangeRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawDeleteRangeResponse {
    pub fn new_() -> RawDeleteRangeResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
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
    fn default_instance() -> &'static RawDeleteRangeResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawScanRequest {
    pub fn new_() -> RawScanRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_limit(&mut self) {
        self.limit = 0
    }
    pub fn set_limit(&mut self, v: u32) {
        self.limit = v;
    }
    pub fn get_limit(&self) -> u32 {
        self.limit
    }
    pub fn clear_key_only(&mut self) {
        self.key_only = false
    }
    pub fn set_key_only(&mut self, v: bool) {
        self.key_only = v;
    }
    pub fn get_key_only(&self) -> bool {
        self.key_only
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
    pub fn clear_reverse(&mut self) {
        self.reverse = false
    }
    pub fn set_reverse(&mut self, v: bool) {
        self.reverse = v;
    }
    pub fn get_reverse(&self) -> bool {
        self.reverse
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static RawScanRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawScanResponse {
    pub fn new_() -> RawScanResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_kvs(&mut self) {
        self.kvs.clear();
    }
    pub fn set_kvs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.kvs = v;
    }
    pub fn get_kvs(&self) -> &::std::vec::Vec<KvPair> {
        &self.kvs
    }
    pub fn mut_kvs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.kvs
    }
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
    fn default_instance() -> &'static RawScanResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl KeyRange {
    pub fn new_() -> KeyRange {
        ::std::default::Default::default()
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static KeyRange {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawBatchScanRequest {
    pub fn new_() -> RawBatchScanRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }
    pub fn set_ranges(&mut self, v: ::std::vec::Vec<KeyRange>) {
        self.ranges = v;
    }
    pub fn get_ranges(&self) -> &::std::vec::Vec<KeyRange> {
        &self.ranges
    }
    pub fn mut_ranges(&mut self) -> &mut ::std::vec::Vec<KeyRange> {
        &mut self.ranges
    }
    pub fn take_ranges(&mut self) -> ::std::vec::Vec<KeyRange> {
        ::std::mem::replace(&mut self.ranges, ::std::vec::Vec::new())
    }
    pub fn clear_each_limit(&mut self) {
        self.each_limit = 0
    }
    pub fn set_each_limit(&mut self, v: u32) {
        self.each_limit = v;
    }
    pub fn get_each_limit(&self) -> u32 {
        self.each_limit
    }
    pub fn clear_key_only(&mut self) {
        self.key_only = false
    }
    pub fn set_key_only(&mut self, v: bool) {
        self.key_only = v;
    }
    pub fn get_key_only(&self) -> bool {
        self.key_only
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
    pub fn clear_reverse(&mut self) {
        self.reverse = false
    }
    pub fn set_reverse(&mut self, v: bool) {
        self.reverse = v;
    }
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
    fn default_instance() -> &'static RawBatchScanRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl RawBatchScanResponse {
    pub fn new_() -> RawBatchScanResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_kvs(&mut self) {
        self.kvs.clear();
    }
    pub fn set_kvs(&mut self, v: ::std::vec::Vec<KvPair>) {
        self.kvs = v;
    }
    pub fn get_kvs(&self) -> &::std::vec::Vec<KvPair> {
        &self.kvs
    }
    pub fn mut_kvs(&mut self) -> &mut ::std::vec::Vec<KvPair> {
        &mut self.kvs
    }
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
    fn default_instance() -> &'static RawBatchScanResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl MvccWrite {
    pub fn new_() -> MvccWrite {
        ::std::default::Default::default()
    }
    pub fn clear_field_type(&mut self) {
        self.r#type = 0
    }
    pub fn set_field_type_(&mut self, v: Op) {
        self.r#type = unsafe { ::std::mem::transmute::<Op, i32>(v) };
    }
    pub fn get_field_type(&self) -> Op {
        unsafe { ::std::mem::transmute::<i32, Op>(self.r#type) }
    }
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }
    pub fn clear_commit_ts(&mut self) {
        self.commit_ts = 0
    }
    pub fn set_commit_ts(&mut self, v: u64) {
        self.commit_ts = v;
    }
    pub fn get_commit_ts(&self) -> u64 {
        self.commit_ts
    }
    pub fn clear_short_value(&mut self) {
        self.short_value.clear();
    }
    pub fn set_short_value(&mut self, v: Vec<u8>) {
        self.short_value = v;
    }
    pub fn get_short_value(&self) -> &[u8] {
        &self.short_value
    }
    pub fn mut_short_value(&mut self) -> &mut Vec<u8> {
        &mut self.short_value
    }
    pub fn take_short_value(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static MvccWrite {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl MvccValue {
    pub fn new_() -> MvccValue {
        ::std::default::Default::default()
    }
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static MvccValue {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl MvccLock {
    pub fn new_() -> MvccLock {
        ::std::default::Default::default()
    }
    pub fn clear_field_type(&mut self) {
        self.r#type = 0
    }
    pub fn set_field_type_(&mut self, v: Op) {
        self.r#type = unsafe { ::std::mem::transmute::<Op, i32>(v) };
    }
    pub fn get_field_type(&self) -> Op {
        unsafe { ::std::mem::transmute::<i32, Op>(self.r#type) }
    }
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
    pub fn get_start_ts(&self) -> u64 {
        self.start_ts
    }
    pub fn clear_primary(&mut self) {
        self.primary.clear();
    }
    pub fn set_primary(&mut self, v: Vec<u8>) {
        self.primary = v;
    }
    pub fn get_primary(&self) -> &[u8] {
        &self.primary
    }
    pub fn mut_primary(&mut self) -> &mut Vec<u8> {
        &mut self.primary
    }
    pub fn take_primary(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.primary, ::std::vec::Vec::new())
    }
    pub fn clear_short_value(&mut self) {
        self.short_value.clear();
    }
    pub fn set_short_value(&mut self, v: Vec<u8>) {
        self.short_value = v;
    }
    pub fn get_short_value(&self) -> &[u8] {
        &self.short_value
    }
    pub fn mut_short_value(&mut self) -> &mut Vec<u8> {
        &mut self.short_value
    }
    pub fn take_short_value(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static MvccLock {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl MvccInfo {
    pub fn new_() -> MvccInfo {
        ::std::default::Default::default()
    }
    pub fn has_lock(&self) -> bool {
        self.lock.is_some()
    }
    pub fn clear_lock(&mut self) {
        self.lock = ::std::option::Option::None
    }
    pub fn set_lock(&mut self, v: MvccLock) {
        self.lock = ::std::option::Option::Some(v);;    }
    pub fn get_lock(&self) -> &MvccLock {
        match self.lock.as_ref() {
            Some(v) => v,
            None => <MvccLock as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_lock(&mut self) -> &mut MvccLock {
        if self.lock.is_none() {
            self.lock = ::std::option::Option::Some(MvccLock::default());
        }
        self.lock.as_mut().unwrap()
    }
    pub fn take_lock(&mut self) -> MvccLock {
        self.lock.take().unwrap_or_else(|| MvccLock::default())
    }
    pub fn clear_writes(&mut self) {
        self.writes.clear();
    }
    pub fn set_writes(&mut self, v: ::std::vec::Vec<MvccWrite>) {
        self.writes = v;
    }
    pub fn get_writes(&self) -> &::std::vec::Vec<MvccWrite> {
        &self.writes
    }
    pub fn mut_writes(&mut self) -> &mut ::std::vec::Vec<MvccWrite> {
        &mut self.writes
    }
    pub fn take_writes(&mut self) -> ::std::vec::Vec<MvccWrite> {
        ::std::mem::replace(&mut self.writes, ::std::vec::Vec::new())
    }
    pub fn clear_values(&mut self) {
        self.values.clear();
    }
    pub fn set_values(&mut self, v: ::std::vec::Vec<MvccValue>) {
        self.values = v;
    }
    pub fn get_values(&self) -> &::std::vec::Vec<MvccValue> {
        &self.values
    }
    pub fn mut_values(&mut self) -> &mut ::std::vec::Vec<MvccValue> {
        &mut self.values
    }
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
    fn default_instance() -> &'static MvccInfo {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl MvccGetByKeyRequest {
    pub fn new_() -> MvccGetByKeyRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static MvccGetByKeyRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl MvccGetByKeyResponse {
    pub fn new_() -> MvccGetByKeyResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }
    pub fn clear_info(&mut self) {
        self.info = ::std::option::Option::None
    }
    pub fn set_info(&mut self, v: MvccInfo) {
        self.info = ::std::option::Option::Some(v);;    }
    pub fn get_info(&self) -> &MvccInfo {
        match self.info.as_ref() {
            Some(v) => v,
            None => <MvccInfo as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_info(&mut self) -> &mut MvccInfo {
        if self.info.is_none() {
            self.info = ::std::option::Option::Some(MvccInfo::default());
        }
        self.info.as_mut().unwrap()
    }
    pub fn take_info(&mut self) -> MvccInfo {
        self.info.take().unwrap_or_else(|| MvccInfo::default())
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
    fn default_instance() -> &'static MvccGetByKeyResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl MvccGetByStartTsRequest {
    pub fn new_() -> MvccGetByStartTsRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_start_ts(&mut self) {
        self.start_ts = 0
    }
    pub fn set_start_ts(&mut self, v: u64) {
        self.start_ts = v;
    }
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
    fn default_instance() -> &'static MvccGetByStartTsRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl MvccGetByStartTsResponse {
    pub fn new_() -> MvccGetByStartTsResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
        ::std::mem::replace(&mut self.error, ::std::string::String::new())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn has_info(&self) -> bool {
        self.info.is_some()
    }
    pub fn clear_info(&mut self) {
        self.info = ::std::option::Option::None
    }
    pub fn set_info(&mut self, v: MvccInfo) {
        self.info = ::std::option::Option::Some(v);;    }
    pub fn get_info(&self) -> &MvccInfo {
        match self.info.as_ref() {
            Some(v) => v,
            None => <MvccInfo as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_info(&mut self) -> &mut MvccInfo {
        if self.info.is_none() {
            self.info = ::std::option::Option::Some(MvccInfo::default());
        }
        self.info.as_mut().unwrap()
    }
    pub fn take_info(&mut self) -> MvccInfo {
        self.info.take().unwrap_or_else(|| MvccInfo::default())
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
    fn default_instance() -> &'static MvccGetByStartTsResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl SplitRegionRequest {
    pub fn new_() -> SplitRegionRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_split_key(&mut self) {
        self.split_key.clear();
    }
    pub fn set_split_key(&mut self, v: Vec<u8>) {
        self.split_key = v;
    }
    pub fn get_split_key(&self) -> &[u8] {
        &self.split_key
    }
    pub fn mut_split_key(&mut self) -> &mut Vec<u8> {
        &mut self.split_key
    }
    pub fn take_split_key(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static SplitRegionRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl SplitRegionResponse {
    pub fn new_() -> SplitRegionResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn has_left(&self) -> bool {
        self.left.is_some()
    }
    pub fn clear_left(&mut self) {
        self.left = ::std::option::Option::None
    }
    pub fn set_left(&mut self, v: super::metapb::Region) {
        self.left = ::std::option::Option::Some(v);;    }
    pub fn get_left(&self) -> &super::metapb::Region {
        match self.left.as_ref() {
            Some(v) => v,
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_left(&mut self) -> &mut super::metapb::Region {
        if self.left.is_none() {
            self.left = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.left.as_mut().unwrap()
    }
    pub fn take_left(&mut self) -> super::metapb::Region {
        self.left
            .take()
            .unwrap_or_else(|| super::metapb::Region::default())
    }
    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }
    pub fn clear_right(&mut self) {
        self.right = ::std::option::Option::None
    }
    pub fn set_right(&mut self, v: super::metapb::Region) {
        self.right = ::std::option::Option::Some(v);;    }
    pub fn get_right(&self) -> &super::metapb::Region {
        match self.right.as_ref() {
            Some(v) => v,
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_right(&mut self) -> &mut super::metapb::Region {
        if self.right.is_none() {
            self.right = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.right.as_mut().unwrap()
    }
    pub fn take_right(&mut self) -> super::metapb::Region {
        self.right
            .take()
            .unwrap_or_else(|| super::metapb::Region::default())
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
    fn default_instance() -> &'static SplitRegionResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl UnsafeDestroyRangeRequest {
    pub fn new_() -> UnsafeDestroyRangeRequest {
        ::std::default::Default::default()
    }
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> Context {
        self.context.take().unwrap_or_else(|| Context::default())
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> Vec<u8> {
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
    fn default_instance() -> &'static UnsafeDestroyRangeRequest {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
impl UnsafeDestroyRangeResponse {
    pub fn new_() -> UnsafeDestroyRangeResponse {
        ::std::default::Default::default()
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn clear_error(&mut self) {
        self.error.clear();
    }
    pub fn set_error(&mut self, v: String) {
        self.error = v;
    }
    pub fn get_error(&self) -> &str {
        &self.error
    }
    pub fn mut_error(&mut self) -> &mut String {
        &mut self.error
    }
    pub fn take_error(&mut self) -> String {
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
    fn default_instance() -> &'static UnsafeDestroyRangeResponse {
        unimplemented!();
    }
    fn is_initialized(&self) -> bool {
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
}
