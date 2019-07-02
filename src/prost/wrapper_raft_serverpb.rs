// Generated file, please don't edit manually.

impl RaftMessage {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RaftMessage = RaftMessage::default();
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
    pub fn has_from_peer(&self) -> bool {
        self.from_peer.is_some()
    }
    #[inline]
    pub fn clear_from_peer(&mut self) {
        self.from_peer = ::std::option::Option::None
    }
    #[inline]
    pub fn set_from_peer(&mut self, v: super::metapb::Peer) {
        self.from_peer = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_from_peer(&self) -> &super::metapb::Peer {
        match self.from_peer.as_ref() {
            Some(v) => v,
            None => super::metapb::Peer::default_ref(),
        }
    }
    #[inline]
    pub fn mut_from_peer(&mut self) -> &mut super::metapb::Peer {
        if self.from_peer.is_none() {
            self.from_peer = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.from_peer.as_mut().unwrap()
    }
    #[inline]
    pub fn take_from_peer(&mut self) -> super::metapb::Peer {
        self.from_peer
            .take()
            .unwrap_or_else(super::metapb::Peer::default)
    }
    #[inline]
    pub fn has_to_peer(&self) -> bool {
        self.to_peer.is_some()
    }
    #[inline]
    pub fn clear_to_peer(&mut self) {
        self.to_peer = ::std::option::Option::None
    }
    #[inline]
    pub fn set_to_peer(&mut self, v: super::metapb::Peer) {
        self.to_peer = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_to_peer(&self) -> &super::metapb::Peer {
        match self.to_peer.as_ref() {
            Some(v) => v,
            None => super::metapb::Peer::default_ref(),
        }
    }
    #[inline]
    pub fn mut_to_peer(&mut self) -> &mut super::metapb::Peer {
        if self.to_peer.is_none() {
            self.to_peer = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.to_peer.as_mut().unwrap()
    }
    #[inline]
    pub fn take_to_peer(&mut self) -> super::metapb::Peer {
        self.to_peer
            .take()
            .unwrap_or_else(super::metapb::Peer::default)
    }
    #[inline]
    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }
    #[inline]
    pub fn clear_message(&mut self) {
        self.message = ::std::option::Option::None
    }
    #[inline]
    pub fn set_message(&mut self, v: super::eraftpb::Message) {
        self.message = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_message(&self) -> &super::eraftpb::Message {
        match self.message.as_ref() {
            Some(v) => v,
            None => super::eraftpb::Message::default_ref(),
        }
    }
    #[inline]
    pub fn mut_message(&mut self) -> &mut super::eraftpb::Message {
        if self.message.is_none() {
            self.message = ::std::option::Option::Some(super::eraftpb::Message::default());
        }
        self.message.as_mut().unwrap()
    }
    #[inline]
    pub fn take_message(&mut self) -> super::eraftpb::Message {
        self.message
            .take()
            .unwrap_or_else(super::eraftpb::Message::default)
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
    pub fn clear_is_tombstone(&mut self) {
        self.is_tombstone = false
    }
    #[inline]
    pub fn set_is_tombstone(&mut self, v: bool) {
        self.is_tombstone = v;
    }
    #[inline]
    pub fn get_is_tombstone(&self) -> bool {
        self.is_tombstone
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
    pub fn has_merge_target(&self) -> bool {
        self.merge_target.is_some()
    }
    #[inline]
    pub fn clear_merge_target(&mut self) {
        self.merge_target = ::std::option::Option::None
    }
    #[inline]
    pub fn set_merge_target(&mut self, v: super::metapb::Region) {
        self.merge_target = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_merge_target(&self) -> &super::metapb::Region {
        match self.merge_target.as_ref() {
            Some(v) => v,
            None => super::metapb::Region::default_ref(),
        }
    }
    #[inline]
    pub fn mut_merge_target(&mut self) -> &mut super::metapb::Region {
        if self.merge_target.is_none() {
            self.merge_target = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.merge_target.as_mut().unwrap()
    }
    #[inline]
    pub fn take_merge_target(&mut self) -> super::metapb::Region {
        self.merge_target
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
}
impl RaftTruncatedState {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RaftTruncatedState = RaftTruncatedState::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_index(&mut self) {
        self.index = 0
    }
    #[inline]
    pub fn set_index(&mut self, v: u64) {
        self.index = v;
    }
    #[inline]
    pub fn get_index(&self) -> u64 {
        self.index
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
}
impl SnapshotCfFile {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotCfFile = SnapshotCfFile::default();
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
    #[inline]
    pub fn clear_checksum(&mut self) {
        self.checksum = 0
    }
    #[inline]
    pub fn set_checksum(&mut self, v: u32) {
        self.checksum = v;
    }
    #[inline]
    pub fn get_checksum(&self) -> u32 {
        self.checksum
    }
}
impl SnapshotMeta {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotMeta = SnapshotMeta::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_cf_files(&mut self) {
        self.cf_files.clear();
    }
    #[inline]
    pub fn set_cf_files(&mut self, v: ::std::vec::Vec<SnapshotCfFile>) {
        self.cf_files = v;
    }
    #[inline]
    pub fn get_cf_files(&self) -> &::std::vec::Vec<SnapshotCfFile> {
        &self.cf_files
    }
    #[inline]
    pub fn mut_cf_files(&mut self) -> &mut ::std::vec::Vec<SnapshotCfFile> {
        &mut self.cf_files
    }
    #[inline]
    pub fn take_cf_files(&mut self) -> ::std::vec::Vec<SnapshotCfFile> {
        ::std::mem::replace(&mut self.cf_files, ::std::vec::Vec::new())
    }
}
impl SnapshotChunk {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotChunk = SnapshotChunk::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_message(&self) -> bool {
        self.message.is_some()
    }
    #[inline]
    pub fn clear_message(&mut self) {
        self.message = ::std::option::Option::None
    }
    #[inline]
    pub fn set_message(&mut self, v: RaftMessage) {
        self.message = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_message(&self) -> &RaftMessage {
        match self.message.as_ref() {
            Some(v) => v,
            None => RaftMessage::default_ref(),
        }
    }
    #[inline]
    pub fn mut_message(&mut self) -> &mut RaftMessage {
        if self.message.is_none() {
            self.message = ::std::option::Option::Some(RaftMessage::default());
        }
        self.message.as_mut().unwrap()
    }
    #[inline]
    pub fn take_message(&mut self) -> RaftMessage {
        self.message.take().unwrap_or_else(RaftMessage::default)
    }
    #[inline]
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
    #[inline]
    pub fn set_data(&mut self, v: std::vec::Vec<u8>) {
        self.data = v;
    }
    #[inline]
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    #[inline]
    pub fn mut_data(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.data
    }
    #[inline]
    pub fn take_data(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}
impl Done {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Done = Done::default();
        }
        &*INSTANCE
    }
}
impl KeyValue {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: KeyValue = KeyValue::default();
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
impl RaftSnapshotData {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RaftSnapshotData = RaftSnapshotData::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }
    #[inline]
    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region(&self) -> &super::metapb::Region {
        match self.region.as_ref() {
            Some(v) => v,
            None => super::metapb::Region::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.region.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
    #[inline]
    pub fn clear_file_size(&mut self) {
        self.file_size = 0
    }
    #[inline]
    pub fn set_file_size(&mut self, v: u64) {
        self.file_size = v;
    }
    #[inline]
    pub fn get_file_size(&self) -> u64 {
        self.file_size
    }
    #[inline]
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
    #[inline]
    pub fn set_data(&mut self, v: ::std::vec::Vec<KeyValue>) {
        self.data = v;
    }
    #[inline]
    pub fn get_data(&self) -> &::std::vec::Vec<KeyValue> {
        &self.data
    }
    #[inline]
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<KeyValue> {
        &mut self.data
    }
    #[inline]
    pub fn take_data(&mut self) -> ::std::vec::Vec<KeyValue> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
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
    pub fn has_meta(&self) -> bool {
        self.meta.is_some()
    }
    #[inline]
    pub fn clear_meta(&mut self) {
        self.meta = ::std::option::Option::None
    }
    #[inline]
    pub fn set_meta(&mut self, v: SnapshotMeta) {
        self.meta = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_meta(&self) -> &SnapshotMeta {
        match self.meta.as_ref() {
            Some(v) => v,
            None => SnapshotMeta::default_ref(),
        }
    }
    #[inline]
    pub fn mut_meta(&mut self) -> &mut SnapshotMeta {
        if self.meta.is_none() {
            self.meta = ::std::option::Option::Some(SnapshotMeta::default());
        }
        self.meta.as_mut().unwrap()
    }
    #[inline]
    pub fn take_meta(&mut self) -> SnapshotMeta {
        self.meta.take().unwrap_or_else(SnapshotMeta::default)
    }
}
impl StoreIdent {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StoreIdent = StoreIdent::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_cluster_id(&mut self) {
        self.cluster_id = 0
    }
    #[inline]
    pub fn set_cluster_id(&mut self, v: u64) {
        self.cluster_id = v;
    }
    #[inline]
    pub fn get_cluster_id(&self) -> u64 {
        self.cluster_id
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
impl RaftLocalState {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RaftLocalState = RaftLocalState::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_hard_state(&self) -> bool {
        self.hard_state.is_some()
    }
    #[inline]
    pub fn clear_hard_state(&mut self) {
        self.hard_state = ::std::option::Option::None
    }
    #[inline]
    pub fn set_hard_state(&mut self, v: super::eraftpb::HardState) {
        self.hard_state = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_hard_state(&self) -> &super::eraftpb::HardState {
        match self.hard_state.as_ref() {
            Some(v) => v,
            None => super::eraftpb::HardState::default_ref(),
        }
    }
    #[inline]
    pub fn mut_hard_state(&mut self) -> &mut super::eraftpb::HardState {
        if self.hard_state.is_none() {
            self.hard_state = ::std::option::Option::Some(super::eraftpb::HardState::default());
        }
        self.hard_state.as_mut().unwrap()
    }
    #[inline]
    pub fn take_hard_state(&mut self) -> super::eraftpb::HardState {
        self.hard_state
            .take()
            .unwrap_or_else(super::eraftpb::HardState::default)
    }
    #[inline]
    pub fn clear_last_index(&mut self) {
        self.last_index = 0
    }
    #[inline]
    pub fn set_last_index(&mut self, v: u64) {
        self.last_index = v;
    }
    #[inline]
    pub fn get_last_index(&self) -> u64 {
        self.last_index
    }
}
impl RaftApplyState {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RaftApplyState = RaftApplyState::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_applied_index(&mut self) {
        self.applied_index = 0
    }
    #[inline]
    pub fn set_applied_index(&mut self, v: u64) {
        self.applied_index = v;
    }
    #[inline]
    pub fn get_applied_index(&self) -> u64 {
        self.applied_index
    }
    #[inline]
    pub fn has_truncated_state(&self) -> bool {
        self.truncated_state.is_some()
    }
    #[inline]
    pub fn clear_truncated_state(&mut self) {
        self.truncated_state = ::std::option::Option::None
    }
    #[inline]
    pub fn set_truncated_state(&mut self, v: RaftTruncatedState) {
        self.truncated_state = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_truncated_state(&self) -> &RaftTruncatedState {
        match self.truncated_state.as_ref() {
            Some(v) => v,
            None => RaftTruncatedState::default_ref(),
        }
    }
    #[inline]
    pub fn mut_truncated_state(&mut self) -> &mut RaftTruncatedState {
        if self.truncated_state.is_none() {
            self.truncated_state = ::std::option::Option::Some(RaftTruncatedState::default());
        }
        self.truncated_state.as_mut().unwrap()
    }
    #[inline]
    pub fn take_truncated_state(&mut self) -> RaftTruncatedState {
        self.truncated_state
            .take()
            .unwrap_or_else(RaftTruncatedState::default)
    }
}
impl MergeState {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: MergeState = MergeState::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_min_index(&mut self) {
        self.min_index = 0
    }
    #[inline]
    pub fn set_min_index(&mut self, v: u64) {
        self.min_index = v;
    }
    #[inline]
    pub fn get_min_index(&self) -> u64 {
        self.min_index
    }
    #[inline]
    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }
    #[inline]
    pub fn clear_target(&mut self) {
        self.target = ::std::option::Option::None
    }
    #[inline]
    pub fn set_target(&mut self, v: super::metapb::Region) {
        self.target = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_target(&self) -> &super::metapb::Region {
        match self.target.as_ref() {
            Some(v) => v,
            None => super::metapb::Region::default_ref(),
        }
    }
    #[inline]
    pub fn mut_target(&mut self) -> &mut super::metapb::Region {
        if self.target.is_none() {
            self.target = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.target.as_mut().unwrap()
    }
    #[inline]
    pub fn take_target(&mut self) -> super::metapb::Region {
        self.target
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
    #[inline]
    pub fn clear_commit(&mut self) {
        self.commit = 0
    }
    #[inline]
    pub fn set_commit(&mut self, v: u64) {
        self.commit = v;
    }
    #[inline]
    pub fn get_commit(&self) -> u64 {
        self.commit
    }
}
impl RegionLocalState {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionLocalState = RegionLocalState::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_state(&mut self) {
        self.state = 0
    }
    #[inline]
    pub fn set_state_(&mut self, v: PeerState) {
        self.state = unsafe { ::std::mem::transmute::<PeerState, i32>(v) };
    }
    #[inline]
    pub fn get_state(&self) -> PeerState {
        unsafe { ::std::mem::transmute::<i32, PeerState>(self.state) }
    }
    #[inline]
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }
    #[inline]
    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region(&self) -> &super::metapb::Region {
        match self.region.as_ref() {
            Some(v) => v,
            None => super::metapb::Region::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.region.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
    #[inline]
    pub fn has_merge_state(&self) -> bool {
        self.merge_state.is_some()
    }
    #[inline]
    pub fn clear_merge_state(&mut self) {
        self.merge_state = ::std::option::Option::None
    }
    #[inline]
    pub fn set_merge_state(&mut self, v: MergeState) {
        self.merge_state = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_merge_state(&self) -> &MergeState {
        match self.merge_state.as_ref() {
            Some(v) => v,
            None => MergeState::default_ref(),
        }
    }
    #[inline]
    pub fn mut_merge_state(&mut self) -> &mut MergeState {
        if self.merge_state.is_none() {
            self.merge_state = ::std::option::Option::Some(MergeState::default());
        }
        self.merge_state.as_mut().unwrap()
    }
    #[inline]
    pub fn take_merge_state(&mut self) -> MergeState {
        self.merge_state.take().unwrap_or_else(MergeState::default)
    }
}
impl PeerState {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [PeerState] = &[
            PeerState::Normal,
            PeerState::Applying,
            PeerState::Tombstone,
            PeerState::Merging,
        ];
        VALUES
    }
}
