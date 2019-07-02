// Generated file, please don't edit manually.

impl GetRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetRequest = GetRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_db(&mut self) {
        self.db = 0
    }
    #[inline]
    pub fn set_db_(&mut self, v: Db) {
        self.db = unsafe { ::std::mem::transmute::<Db, i32>(v) };
    }
    #[inline]
    pub fn get_db(&self) -> Db {
        unsafe { ::std::mem::transmute::<i32, Db>(self.db) }
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
impl GetResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetResponse = GetResponse::default();
        }
        &*INSTANCE
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
impl RaftLogRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RaftLogRequest = RaftLogRequest::default();
        }
        &*INSTANCE
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
    pub fn clear_log_index(&mut self) {
        self.log_index = 0
    }
    #[inline]
    pub fn set_log_index(&mut self, v: u64) {
        self.log_index = v;
    }
    #[inline]
    pub fn get_log_index(&self) -> u64 {
        self.log_index
    }
}
impl RaftLogResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RaftLogResponse = RaftLogResponse::default();
        }
        &*INSTANCE
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
    pub fn set_entry(&mut self, v: super::eraftpb::Entry) {
        self.entry = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_entry(&self) -> &super::eraftpb::Entry {
        match self.entry.as_ref() {
            Some(v) => v,
            None => super::eraftpb::Entry::default_ref(),
        }
    }
    #[inline]
    pub fn mut_entry(&mut self) -> &mut super::eraftpb::Entry {
        if self.entry.is_none() {
            self.entry = ::std::option::Option::Some(super::eraftpb::Entry::default());
        }
        self.entry.as_mut().unwrap()
    }
    #[inline]
    pub fn take_entry(&mut self) -> super::eraftpb::Entry {
        self.entry
            .take()
            .unwrap_or_else(super::eraftpb::Entry::default)
    }
}
impl RegionInfoRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionInfoRequest = RegionInfoRequest::default();
        }
        &*INSTANCE
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
}
impl RegionInfoResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionInfoResponse = RegionInfoResponse::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_raft_local_state(&self) -> bool {
        self.raft_local_state.is_some()
    }
    #[inline]
    pub fn clear_raft_local_state(&mut self) {
        self.raft_local_state = ::std::option::Option::None
    }
    #[inline]
    pub fn set_raft_local_state(&mut self, v: super::raft_serverpb::RaftLocalState) {
        self.raft_local_state = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_raft_local_state(&self) -> &super::raft_serverpb::RaftLocalState {
        match self.raft_local_state.as_ref() {
            Some(v) => v,
            None => super::raft_serverpb::RaftLocalState::default_ref(),
        }
    }
    #[inline]
    pub fn mut_raft_local_state(&mut self) -> &mut super::raft_serverpb::RaftLocalState {
        if self.raft_local_state.is_none() {
            self.raft_local_state =
                ::std::option::Option::Some(super::raft_serverpb::RaftLocalState::default());
        }
        self.raft_local_state.as_mut().unwrap()
    }
    #[inline]
    pub fn take_raft_local_state(&mut self) -> super::raft_serverpb::RaftLocalState {
        self.raft_local_state
            .take()
            .unwrap_or_else(super::raft_serverpb::RaftLocalState::default)
    }
    #[inline]
    pub fn has_raft_apply_state(&self) -> bool {
        self.raft_apply_state.is_some()
    }
    #[inline]
    pub fn clear_raft_apply_state(&mut self) {
        self.raft_apply_state = ::std::option::Option::None
    }
    #[inline]
    pub fn set_raft_apply_state(&mut self, v: super::raft_serverpb::RaftApplyState) {
        self.raft_apply_state = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_raft_apply_state(&self) -> &super::raft_serverpb::RaftApplyState {
        match self.raft_apply_state.as_ref() {
            Some(v) => v,
            None => super::raft_serverpb::RaftApplyState::default_ref(),
        }
    }
    #[inline]
    pub fn mut_raft_apply_state(&mut self) -> &mut super::raft_serverpb::RaftApplyState {
        if self.raft_apply_state.is_none() {
            self.raft_apply_state =
                ::std::option::Option::Some(super::raft_serverpb::RaftApplyState::default());
        }
        self.raft_apply_state.as_mut().unwrap()
    }
    #[inline]
    pub fn take_raft_apply_state(&mut self) -> super::raft_serverpb::RaftApplyState {
        self.raft_apply_state
            .take()
            .unwrap_or_else(super::raft_serverpb::RaftApplyState::default)
    }
    #[inline]
    pub fn has_region_local_state(&self) -> bool {
        self.region_local_state.is_some()
    }
    #[inline]
    pub fn clear_region_local_state(&mut self) {
        self.region_local_state = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_local_state(&mut self, v: super::raft_serverpb::RegionLocalState) {
        self.region_local_state = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_local_state(&self) -> &super::raft_serverpb::RegionLocalState {
        match self.region_local_state.as_ref() {
            Some(v) => v,
            None => super::raft_serverpb::RegionLocalState::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_local_state(&mut self) -> &mut super::raft_serverpb::RegionLocalState {
        if self.region_local_state.is_none() {
            self.region_local_state =
                ::std::option::Option::Some(super::raft_serverpb::RegionLocalState::default());
        }
        self.region_local_state.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_local_state(&mut self) -> super::raft_serverpb::RegionLocalState {
        self.region_local_state
            .take()
            .unwrap_or_else(super::raft_serverpb::RegionLocalState::default)
    }
}
impl RegionSizeRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionSizeRequest = RegionSizeRequest::default();
        }
        &*INSTANCE
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
    pub fn clear_cfs(&mut self) {
        self.cfs.clear();
    }
    #[inline]
    pub fn set_cfs(&mut self, v: ::std::vec::Vec<std::string::String>) {
        self.cfs = v;
    }
    #[inline]
    pub fn get_cfs(&self) -> &::std::vec::Vec<std::string::String> {
        &self.cfs
    }
    #[inline]
    pub fn mut_cfs(&mut self) -> &mut ::std::vec::Vec<std::string::String> {
        &mut self.cfs
    }
    #[inline]
    pub fn take_cfs(&mut self) -> ::std::vec::Vec<std::string::String> {
        ::std::mem::replace(&mut self.cfs, ::std::vec::Vec::new())
    }
}
impl RegionSizeResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionSizeResponse = RegionSizeResponse::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }
    #[inline]
    pub fn set_entries(&mut self, v: ::std::vec::Vec<region_size_response::Entry>) {
        self.entries = v;
    }
    #[inline]
    pub fn get_entries(&self) -> &::std::vec::Vec<region_size_response::Entry> {
        &self.entries
    }
    #[inline]
    pub fn mut_entries(&mut self) -> &mut ::std::vec::Vec<region_size_response::Entry> {
        &mut self.entries
    }
    #[inline]
    pub fn take_entries(&mut self) -> ::std::vec::Vec<region_size_response::Entry> {
        ::std::mem::replace(&mut self.entries, ::std::vec::Vec::new())
    }
}
impl region_size_response::Entry {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: region_size_response::Entry = region_size_response::Entry::default();
        }
        &*INSTANCE
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
    pub fn clear_size(&mut self) {
        self.size = 0
    }
    #[inline]
    pub fn set_size(&mut self, v: u64) {
        self.size = v;
    }
    #[inline]
    pub fn get_size(&self) -> u64 {
        self.size
    }
}
impl ScanMvccRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanMvccRequest = ScanMvccRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_from_key(&mut self) {
        self.from_key.clear();
    }
    #[inline]
    pub fn set_from_key(&mut self, v: std::vec::Vec<u8>) {
        self.from_key = v;
    }
    #[inline]
    pub fn get_from_key(&self) -> &[u8] {
        &self.from_key
    }
    #[inline]
    pub fn mut_from_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.from_key
    }
    #[inline]
    pub fn take_from_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.from_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_to_key(&mut self) {
        self.to_key.clear();
    }
    #[inline]
    pub fn set_to_key(&mut self, v: std::vec::Vec<u8>) {
        self.to_key = v;
    }
    #[inline]
    pub fn get_to_key(&self) -> &[u8] {
        &self.to_key
    }
    #[inline]
    pub fn mut_to_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.to_key
    }
    #[inline]
    pub fn take_to_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.to_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_limit(&mut self) {
        self.limit = 0
    }
    #[inline]
    pub fn set_limit(&mut self, v: u64) {
        self.limit = v;
    }
    #[inline]
    pub fn get_limit(&self) -> u64 {
        self.limit
    }
}
impl ScanMvccResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanMvccResponse = ScanMvccResponse::default();
        }
        &*INSTANCE
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
    pub fn set_info(&mut self, v: super::kvrpcpb::MvccInfo) {
        self.info = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_info(&self) -> &super::kvrpcpb::MvccInfo {
        match self.info.as_ref() {
            Some(v) => v,
            None => super::kvrpcpb::MvccInfo::default_ref(),
        }
    }
    #[inline]
    pub fn mut_info(&mut self) -> &mut super::kvrpcpb::MvccInfo {
        if self.info.is_none() {
            self.info = ::std::option::Option::Some(super::kvrpcpb::MvccInfo::default());
        }
        self.info.as_mut().unwrap()
    }
    #[inline]
    pub fn take_info(&mut self) -> super::kvrpcpb::MvccInfo {
        self.info
            .take()
            .unwrap_or_else(super::kvrpcpb::MvccInfo::default)
    }
}
impl CompactRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CompactRequest = CompactRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_db(&mut self) {
        self.db = 0
    }
    #[inline]
    pub fn set_db_(&mut self, v: Db) {
        self.db = unsafe { ::std::mem::transmute::<Db, i32>(v) };
    }
    #[inline]
    pub fn get_db(&self) -> Db {
        unsafe { ::std::mem::transmute::<i32, Db>(self.db) }
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
    pub fn clear_from_key(&mut self) {
        self.from_key.clear();
    }
    #[inline]
    pub fn set_from_key(&mut self, v: std::vec::Vec<u8>) {
        self.from_key = v;
    }
    #[inline]
    pub fn get_from_key(&self) -> &[u8] {
        &self.from_key
    }
    #[inline]
    pub fn mut_from_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.from_key
    }
    #[inline]
    pub fn take_from_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.from_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_to_key(&mut self) {
        self.to_key.clear();
    }
    #[inline]
    pub fn set_to_key(&mut self, v: std::vec::Vec<u8>) {
        self.to_key = v;
    }
    #[inline]
    pub fn get_to_key(&self) -> &[u8] {
        &self.to_key
    }
    #[inline]
    pub fn mut_to_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.to_key
    }
    #[inline]
    pub fn take_to_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.to_key, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_threads(&mut self) {
        self.threads = 0
    }
    #[inline]
    pub fn set_threads(&mut self, v: u32) {
        self.threads = v;
    }
    #[inline]
    pub fn get_threads(&self) -> u32 {
        self.threads
    }
    #[inline]
    pub fn clear_bottommost_level_compaction(&mut self) {
        self.bottommost_level_compaction = 0
    }
    #[inline]
    pub fn set_bottommost_level_compaction_(&mut self, v: BottommostLevelCompaction) {
        self.bottommost_level_compaction =
            unsafe { ::std::mem::transmute::<BottommostLevelCompaction, i32>(v) };
    }
    #[inline]
    pub fn get_bottommost_level_compaction(&self) -> BottommostLevelCompaction {
        unsafe {
            ::std::mem::transmute::<i32, BottommostLevelCompaction>(
                self.bottommost_level_compaction,
            )
        }
    }
}
impl CompactResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CompactResponse = CompactResponse::default();
        }
        &*INSTANCE
    }
}
impl InjectFailPointRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: InjectFailPointRequest = InjectFailPointRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_name(&mut self) {
        self.name.clear();
    }
    #[inline]
    pub fn set_name(&mut self, v: std::string::String) {
        self.name = v;
    }
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    #[inline]
    pub fn mut_name(&mut self) -> &mut std::string::String {
        &mut self.name
    }
    #[inline]
    pub fn take_name(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }
    #[inline]
    pub fn set_actions(&mut self, v: std::string::String) {
        self.actions = v;
    }
    #[inline]
    pub fn get_actions(&self) -> &str {
        &self.actions
    }
    #[inline]
    pub fn mut_actions(&mut self) -> &mut std::string::String {
        &mut self.actions
    }
    #[inline]
    pub fn take_actions(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.actions, ::std::string::String::new())
    }
}
impl InjectFailPointResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: InjectFailPointResponse = InjectFailPointResponse::default();
        }
        &*INSTANCE
    }
}
impl RecoverFailPointRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RecoverFailPointRequest = RecoverFailPointRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_name(&mut self) {
        self.name.clear();
    }
    #[inline]
    pub fn set_name(&mut self, v: std::string::String) {
        self.name = v;
    }
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    #[inline]
    pub fn mut_name(&mut self) -> &mut std::string::String {
        &mut self.name
    }
    #[inline]
    pub fn take_name(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }
}
impl RecoverFailPointResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RecoverFailPointResponse = RecoverFailPointResponse::default();
        }
        &*INSTANCE
    }
}
impl ListFailPointsRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ListFailPointsRequest = ListFailPointsRequest::default();
        }
        &*INSTANCE
    }
}
impl ListFailPointsResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ListFailPointsResponse = ListFailPointsResponse::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }
    #[inline]
    pub fn set_entries(&mut self, v: ::std::vec::Vec<list_fail_points_response::Entry>) {
        self.entries = v;
    }
    #[inline]
    pub fn get_entries(&self) -> &::std::vec::Vec<list_fail_points_response::Entry> {
        &self.entries
    }
    #[inline]
    pub fn mut_entries(&mut self) -> &mut ::std::vec::Vec<list_fail_points_response::Entry> {
        &mut self.entries
    }
    #[inline]
    pub fn take_entries(&mut self) -> ::std::vec::Vec<list_fail_points_response::Entry> {
        ::std::mem::replace(&mut self.entries, ::std::vec::Vec::new())
    }
}
impl list_fail_points_response::Entry {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: list_fail_points_response::Entry = list_fail_points_response::Entry::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_name(&mut self) {
        self.name.clear();
    }
    #[inline]
    pub fn set_name(&mut self, v: std::string::String) {
        self.name = v;
    }
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    #[inline]
    pub fn mut_name(&mut self) -> &mut std::string::String {
        &mut self.name
    }
    #[inline]
    pub fn take_name(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_actions(&mut self) {
        self.actions.clear();
    }
    #[inline]
    pub fn set_actions(&mut self, v: std::string::String) {
        self.actions = v;
    }
    #[inline]
    pub fn get_actions(&self) -> &str {
        &self.actions
    }
    #[inline]
    pub fn mut_actions(&mut self) -> &mut std::string::String {
        &mut self.actions
    }
    #[inline]
    pub fn take_actions(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.actions, ::std::string::String::new())
    }
}
impl GetMetricsRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetMetricsRequest = GetMetricsRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_all(&mut self) {
        self.all = false
    }
    #[inline]
    pub fn set_all(&mut self, v: bool) {
        self.all = v;
    }
    #[inline]
    pub fn get_all(&self) -> bool {
        self.all
    }
}
impl GetMetricsResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetMetricsResponse = GetMetricsResponse::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_prometheus(&mut self) {
        self.prometheus.clear();
    }
    #[inline]
    pub fn set_prometheus(&mut self, v: std::string::String) {
        self.prometheus = v;
    }
    #[inline]
    pub fn get_prometheus(&self) -> &str {
        &self.prometheus
    }
    #[inline]
    pub fn mut_prometheus(&mut self) -> &mut std::string::String {
        &mut self.prometheus
    }
    #[inline]
    pub fn take_prometheus(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.prometheus, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_rocksdb_kv(&mut self) {
        self.rocksdb_kv.clear();
    }
    #[inline]
    pub fn set_rocksdb_kv(&mut self, v: std::string::String) {
        self.rocksdb_kv = v;
    }
    #[inline]
    pub fn get_rocksdb_kv(&self) -> &str {
        &self.rocksdb_kv
    }
    #[inline]
    pub fn mut_rocksdb_kv(&mut self) -> &mut std::string::String {
        &mut self.rocksdb_kv
    }
    #[inline]
    pub fn take_rocksdb_kv(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.rocksdb_kv, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_rocksdb_raft(&mut self) {
        self.rocksdb_raft.clear();
    }
    #[inline]
    pub fn set_rocksdb_raft(&mut self, v: std::string::String) {
        self.rocksdb_raft = v;
    }
    #[inline]
    pub fn get_rocksdb_raft(&self) -> &str {
        &self.rocksdb_raft
    }
    #[inline]
    pub fn mut_rocksdb_raft(&mut self) -> &mut std::string::String {
        &mut self.rocksdb_raft
    }
    #[inline]
    pub fn take_rocksdb_raft(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.rocksdb_raft, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_jemalloc(&mut self) {
        self.jemalloc.clear();
    }
    #[inline]
    pub fn set_jemalloc(&mut self, v: std::string::String) {
        self.jemalloc = v;
    }
    #[inline]
    pub fn get_jemalloc(&self) -> &str {
        &self.jemalloc
    }
    #[inline]
    pub fn mut_jemalloc(&mut self) -> &mut std::string::String {
        &mut self.jemalloc
    }
    #[inline]
    pub fn take_jemalloc(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.jemalloc, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_store_id(&mut self) {
        self.store_id = 0
    }
    #[inline]
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = v;
    }
    #[inline]
    pub fn get_store_id(&self) -> u64 {
        self.store_id
    }
}
impl RegionConsistencyCheckRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionConsistencyCheckRequest = RegionConsistencyCheckRequest::default();
        }
        &*INSTANCE
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
}
impl RegionConsistencyCheckResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionConsistencyCheckResponse = RegionConsistencyCheckResponse::default();
        }
        &*INSTANCE
    }
}
impl ModifyTikvConfigRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ModifyTikvConfigRequest = ModifyTikvConfigRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_module(&mut self) {
        self.module = 0
    }
    #[inline]
    pub fn set_module_(&mut self, v: Module) {
        self.module = unsafe { ::std::mem::transmute::<Module, i32>(v) };
    }
    #[inline]
    pub fn get_module(&self) -> Module {
        unsafe { ::std::mem::transmute::<i32, Module>(self.module) }
    }
    #[inline]
    pub fn clear_config_name(&mut self) {
        self.config_name.clear();
    }
    #[inline]
    pub fn set_config_name(&mut self, v: std::string::String) {
        self.config_name = v;
    }
    #[inline]
    pub fn get_config_name(&self) -> &str {
        &self.config_name
    }
    #[inline]
    pub fn mut_config_name(&mut self) -> &mut std::string::String {
        &mut self.config_name
    }
    #[inline]
    pub fn take_config_name(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.config_name, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_config_value(&mut self) {
        self.config_value.clear();
    }
    #[inline]
    pub fn set_config_value(&mut self, v: std::string::String) {
        self.config_value = v;
    }
    #[inline]
    pub fn get_config_value(&self) -> &str {
        &self.config_value
    }
    #[inline]
    pub fn mut_config_value(&mut self) -> &mut std::string::String {
        &mut self.config_value
    }
    #[inline]
    pub fn take_config_value(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.config_value, ::std::string::String::new())
    }
}
impl ModifyTikvConfigResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ModifyTikvConfigResponse = ModifyTikvConfigResponse::default();
        }
        &*INSTANCE
    }
}
impl Property {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Property = Property::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_name(&mut self) {
        self.name.clear();
    }
    #[inline]
    pub fn set_name(&mut self, v: std::string::String) {
        self.name = v;
    }
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    #[inline]
    pub fn mut_name(&mut self) -> &mut std::string::String {
        &mut self.name
    }
    #[inline]
    pub fn take_name(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    #[inline]
    pub fn set_value(&mut self, v: std::string::String) {
        self.value = v;
    }
    #[inline]
    pub fn get_value(&self) -> &str {
        &self.value
    }
    #[inline]
    pub fn mut_value(&mut self) -> &mut std::string::String {
        &mut self.value
    }
    #[inline]
    pub fn take_value(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }
}
impl GetRegionPropertiesRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetRegionPropertiesRequest = GetRegionPropertiesRequest::default();
        }
        &*INSTANCE
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
}
impl GetRegionPropertiesResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetRegionPropertiesResponse = GetRegionPropertiesResponse::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_props(&mut self) {
        self.props.clear();
    }
    #[inline]
    pub fn set_props(&mut self, v: ::std::vec::Vec<Property>) {
        self.props = v;
    }
    #[inline]
    pub fn get_props(&self) -> &::std::vec::Vec<Property> {
        &self.props
    }
    #[inline]
    pub fn mut_props(&mut self) -> &mut ::std::vec::Vec<Property> {
        &mut self.props
    }
    #[inline]
    pub fn take_props(&mut self) -> ::std::vec::Vec<Property> {
        ::std::mem::replace(&mut self.props, ::std::vec::Vec::new())
    }
}
impl Db {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [Db] = &[Db::Invalid, Db::Kv, Db::Raft];
        VALUES
    }
}
impl Module {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [Module] = &[
            Module::Unused,
            Module::Kvdb,
            Module::Raftdb,
            Module::Readpool,
            Module::Server,
            Module::Storage,
            Module::Pd,
            Module::Metric,
            Module::Coprocessor,
            Module::Security,
            Module::Import,
        ];
        VALUES
    }
}
impl BottommostLevelCompaction {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [BottommostLevelCompaction] = &[
            BottommostLevelCompaction::Skip,
            BottommostLevelCompaction::Force,
            BottommostLevelCompaction::IfHaveCompactionFilter,
        ];
        VALUES
    }
}
