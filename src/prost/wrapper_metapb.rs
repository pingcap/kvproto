// Generated file, please don't edit manually.

impl Cluster {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Cluster = Cluster::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_id(&mut self) {
        self.id = 0
    }
    #[inline]
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    #[inline]
    pub fn get_id(&self) -> u64 {
        self.id
    }
    #[inline]
    pub fn clear_max_peer_count(&mut self) {
        self.max_peer_count = 0
    }
    #[inline]
    pub fn set_max_peer_count(&mut self, v: u32) {
        self.max_peer_count = v;
    }
    #[inline]
    pub fn get_max_peer_count(&self) -> u32 {
        self.max_peer_count
    }
}
impl StoreLabel {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StoreLabel = StoreLabel::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    #[inline]
    pub fn set_key(&mut self, v: std::string::String) {
        self.key = v;
    }
    #[inline]
    pub fn get_key(&self) -> &str {
        &self.key
    }
    #[inline]
    pub fn mut_key(&mut self) -> &mut std::string::String {
        &mut self.key
    }
    #[inline]
    pub fn take_key(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
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
impl Store {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Store = Store::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_id(&mut self) {
        self.id = 0
    }
    #[inline]
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    #[inline]
    pub fn get_id(&self) -> u64 {
        self.id
    }
    #[inline]
    pub fn clear_address(&mut self) {
        self.address.clear();
    }
    #[inline]
    pub fn set_address(&mut self, v: std::string::String) {
        self.address = v;
    }
    #[inline]
    pub fn get_address(&self) -> &str {
        &self.address
    }
    #[inline]
    pub fn mut_address(&mut self) -> &mut std::string::String {
        &mut self.address
    }
    #[inline]
    pub fn take_address(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_state(&mut self) {
        self.state = 0
    }
    #[inline]
    pub fn set_state_(&mut self, v: StoreState) {
        self.state = unsafe { ::std::mem::transmute::<StoreState, i32>(v) };
    }
    #[inline]
    pub fn get_state(&self) -> StoreState {
        unsafe { ::std::mem::transmute::<i32, StoreState>(self.state) }
    }
    #[inline]
    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }
    #[inline]
    pub fn set_labels(&mut self, v: ::std::vec::Vec<StoreLabel>) {
        self.labels = v;
    }
    #[inline]
    pub fn get_labels(&self) -> &::std::vec::Vec<StoreLabel> {
        &self.labels
    }
    #[inline]
    pub fn mut_labels(&mut self) -> &mut ::std::vec::Vec<StoreLabel> {
        &mut self.labels
    }
    #[inline]
    pub fn take_labels(&mut self) -> ::std::vec::Vec<StoreLabel> {
        ::std::mem::replace(&mut self.labels, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_version(&mut self) {
        self.version.clear();
    }
    #[inline]
    pub fn set_version(&mut self, v: std::string::String) {
        self.version = v;
    }
    #[inline]
    pub fn get_version(&self) -> &str {
        &self.version
    }
    #[inline]
    pub fn mut_version(&mut self) -> &mut std::string::String {
        &mut self.version
    }
    #[inline]
    pub fn take_version(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }
}
impl RegionEpoch {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionEpoch = RegionEpoch::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_conf_ver(&mut self) {
        self.conf_ver = 0
    }
    #[inline]
    pub fn set_conf_ver(&mut self, v: u64) {
        self.conf_ver = v;
    }
    #[inline]
    pub fn get_conf_ver(&self) -> u64 {
        self.conf_ver
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
impl Region {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Region = Region::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_id(&mut self) {
        self.id = 0
    }
    #[inline]
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    #[inline]
    pub fn get_id(&self) -> u64 {
        self.id
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
    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }
    #[inline]
    pub fn clear_region_epoch(&mut self) {
        self.region_epoch = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_epoch(&mut self, v: RegionEpoch) {
        self.region_epoch = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_epoch(&self) -> &RegionEpoch {
        match self.region_epoch.as_ref() {
            Some(v) => v,
            None => RegionEpoch::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_epoch(&mut self) -> &mut RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch = ::std::option::Option::Some(RegionEpoch::default());
        }
        self.region_epoch.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_epoch(&mut self) -> RegionEpoch {
        self.region_epoch
            .take()
            .unwrap_or_else(RegionEpoch::default)
    }
    #[inline]
    pub fn clear_peers(&mut self) {
        self.peers.clear();
    }
    #[inline]
    pub fn set_peers(&mut self, v: ::std::vec::Vec<Peer>) {
        self.peers = v;
    }
    #[inline]
    pub fn get_peers(&self) -> &::std::vec::Vec<Peer> {
        &self.peers
    }
    #[inline]
    pub fn mut_peers(&mut self) -> &mut ::std::vec::Vec<Peer> {
        &mut self.peers
    }
    #[inline]
    pub fn take_peers(&mut self) -> ::std::vec::Vec<Peer> {
        ::std::mem::replace(&mut self.peers, ::std::vec::Vec::new())
    }
}
impl Peer {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Peer = Peer::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_id(&mut self) {
        self.id = 0
    }
    #[inline]
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    #[inline]
    pub fn get_id(&self) -> u64 {
        self.id
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
    #[inline]
    pub fn clear_is_learner(&mut self) {
        self.is_learner = false
    }
    #[inline]
    pub fn set_is_learner(&mut self, v: bool) {
        self.is_learner = v;
    }
    #[inline]
    pub fn get_is_learner(&self) -> bool {
        self.is_learner
    }
}
impl StoreState {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [StoreState] =
            &[StoreState::Up, StoreState::Offline, StoreState::Tombstone];
        VALUES
    }
}
