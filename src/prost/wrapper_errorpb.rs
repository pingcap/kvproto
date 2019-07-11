// Generated file, please don't edit manually.

impl NotLeader {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: NotLeader = NotLeader::default();
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
    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }
    #[inline]
    pub fn clear_leader(&mut self) {
        self.leader = ::std::option::Option::None
    }
    #[inline]
    pub fn set_leader(&mut self, v: super::metapb::Peer) {
        self.leader = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_leader(&self) -> &super::metapb::Peer {
        match self.leader.as_ref() {
            Some(v) => v,
            None => super::metapb::Peer::default_ref(),
        }
    }
    #[inline]
    pub fn mut_leader(&mut self) -> &mut super::metapb::Peer {
        if self.leader.is_none() {
            self.leader = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.leader.as_mut().unwrap()
    }
    #[inline]
    pub fn take_leader(&mut self) -> super::metapb::Peer {
        self.leader
            .take()
            .unwrap_or_else(super::metapb::Peer::default)
    }
}
impl StoreNotMatch {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StoreNotMatch = StoreNotMatch::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_request_store_id(&mut self) {
        self.request_store_id = 0
    }
    #[inline]
    pub fn set_request_store_id(&mut self, v: u64) {
        self.request_store_id = v;
    }
    #[inline]
    pub fn get_request_store_id(&self) -> u64 {
        self.request_store_id
    }
    #[inline]
    pub fn clear_actual_store_id(&mut self) {
        self.actual_store_id = 0
    }
    #[inline]
    pub fn set_actual_store_id(&mut self, v: u64) {
        self.actual_store_id = v;
    }
    #[inline]
    pub fn get_actual_store_id(&self) -> u64 {
        self.actual_store_id
    }
}
impl RegionNotFound {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionNotFound = RegionNotFound::default();
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
impl KeyNotInRegion {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: KeyNotInRegion = KeyNotInRegion::default();
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
impl EpochNotMatch {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: EpochNotMatch = EpochNotMatch::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_current_regions(&mut self) {
        self.current_regions.clear();
    }
    #[inline]
    pub fn set_current_regions(&mut self, v: ::std::vec::Vec<super::metapb::Region>) {
        self.current_regions = v;
    }
    #[inline]
    pub fn get_current_regions(&self) -> &::std::vec::Vec<super::metapb::Region> {
        &self.current_regions
    }
    #[inline]
    pub fn mut_current_regions(&mut self) -> &mut ::std::vec::Vec<super::metapb::Region> {
        &mut self.current_regions
    }
    #[inline]
    pub fn take_current_regions(&mut self) -> ::std::vec::Vec<super::metapb::Region> {
        ::std::mem::replace(&mut self.current_regions, ::std::vec::Vec::new())
    }
}
impl ServerIsBusy {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ServerIsBusy = ServerIsBusy::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_reason(&mut self) {
        self.reason.clear();
    }
    #[inline]
    pub fn set_reason(&mut self, v: std::string::String) {
        self.reason = v;
    }
    #[inline]
    pub fn get_reason(&self) -> &str {
        &self.reason
    }
    #[inline]
    pub fn mut_reason(&mut self) -> &mut std::string::String {
        &mut self.reason
    }
    #[inline]
    pub fn take_reason(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.reason, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_backoff_ms(&mut self) {
        self.backoff_ms = 0
    }
    #[inline]
    pub fn set_backoff_ms(&mut self, v: u64) {
        self.backoff_ms = v;
    }
    #[inline]
    pub fn get_backoff_ms(&self) -> u64 {
        self.backoff_ms
    }
}
impl StaleCommand {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StaleCommand = StaleCommand::default();
        }
        &*INSTANCE
    }
}
impl RaftEntryTooLarge {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RaftEntryTooLarge = RaftEntryTooLarge::default();
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
    pub fn clear_entry_size(&mut self) {
        self.entry_size = 0
    }
    #[inline]
    pub fn set_entry_size(&mut self, v: u64) {
        self.entry_size = v;
    }
    #[inline]
    pub fn get_entry_size(&self) -> u64 {
        self.entry_size
    }
}
impl Error {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Error = Error::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_message(&mut self) {
        self.message.clear();
    }
    #[inline]
    pub fn set_message(&mut self, v: std::string::String) {
        self.message = v;
    }
    #[inline]
    pub fn get_message(&self) -> &str {
        &self.message
    }
    #[inline]
    pub fn mut_message(&mut self) -> &mut std::string::String {
        &mut self.message
    }
    #[inline]
    pub fn take_message(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }
    #[inline]
    pub fn has_not_leader(&self) -> bool {
        self.not_leader.is_some()
    }
    #[inline]
    pub fn clear_not_leader(&mut self) {
        self.not_leader = ::std::option::Option::None
    }
    #[inline]
    pub fn set_not_leader(&mut self, v: NotLeader) {
        self.not_leader = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_not_leader(&self) -> &NotLeader {
        match self.not_leader.as_ref() {
            Some(v) => v,
            None => NotLeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_not_leader(&mut self) -> &mut NotLeader {
        if self.not_leader.is_none() {
            self.not_leader = ::std::option::Option::Some(NotLeader::default());
        }
        self.not_leader.as_mut().unwrap()
    }
    #[inline]
    pub fn take_not_leader(&mut self) -> NotLeader {
        self.not_leader.take().unwrap_or_else(NotLeader::default)
    }
    #[inline]
    pub fn has_region_not_found(&self) -> bool {
        self.region_not_found.is_some()
    }
    #[inline]
    pub fn clear_region_not_found(&mut self) {
        self.region_not_found = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_not_found(&mut self, v: RegionNotFound) {
        self.region_not_found = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_not_found(&self) -> &RegionNotFound {
        match self.region_not_found.as_ref() {
            Some(v) => v,
            None => RegionNotFound::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_not_found(&mut self) -> &mut RegionNotFound {
        if self.region_not_found.is_none() {
            self.region_not_found = ::std::option::Option::Some(RegionNotFound::default());
        }
        self.region_not_found.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_not_found(&mut self) -> RegionNotFound {
        self.region_not_found
            .take()
            .unwrap_or_else(RegionNotFound::default)
    }
    #[inline]
    pub fn has_key_not_in_region(&self) -> bool {
        self.key_not_in_region.is_some()
    }
    #[inline]
    pub fn clear_key_not_in_region(&mut self) {
        self.key_not_in_region = ::std::option::Option::None
    }
    #[inline]
    pub fn set_key_not_in_region(&mut self, v: KeyNotInRegion) {
        self.key_not_in_region = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_key_not_in_region(&self) -> &KeyNotInRegion {
        match self.key_not_in_region.as_ref() {
            Some(v) => v,
            None => KeyNotInRegion::default_ref(),
        }
    }
    #[inline]
    pub fn mut_key_not_in_region(&mut self) -> &mut KeyNotInRegion {
        if self.key_not_in_region.is_none() {
            self.key_not_in_region = ::std::option::Option::Some(KeyNotInRegion::default());
        }
        self.key_not_in_region.as_mut().unwrap()
    }
    #[inline]
    pub fn take_key_not_in_region(&mut self) -> KeyNotInRegion {
        self.key_not_in_region
            .take()
            .unwrap_or_else(KeyNotInRegion::default)
    }
    #[inline]
    pub fn has_epoch_not_match(&self) -> bool {
        self.epoch_not_match.is_some()
    }
    #[inline]
    pub fn clear_epoch_not_match(&mut self) {
        self.epoch_not_match = ::std::option::Option::None
    }
    #[inline]
    pub fn set_epoch_not_match(&mut self, v: EpochNotMatch) {
        self.epoch_not_match = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_epoch_not_match(&self) -> &EpochNotMatch {
        match self.epoch_not_match.as_ref() {
            Some(v) => v,
            None => EpochNotMatch::default_ref(),
        }
    }
    #[inline]
    pub fn mut_epoch_not_match(&mut self) -> &mut EpochNotMatch {
        if self.epoch_not_match.is_none() {
            self.epoch_not_match = ::std::option::Option::Some(EpochNotMatch::default());
        }
        self.epoch_not_match.as_mut().unwrap()
    }
    #[inline]
    pub fn take_epoch_not_match(&mut self) -> EpochNotMatch {
        self.epoch_not_match
            .take()
            .unwrap_or_else(EpochNotMatch::default)
    }
    #[inline]
    pub fn has_server_is_busy(&self) -> bool {
        self.server_is_busy.is_some()
    }
    #[inline]
    pub fn clear_server_is_busy(&mut self) {
        self.server_is_busy = ::std::option::Option::None
    }
    #[inline]
    pub fn set_server_is_busy(&mut self, v: ServerIsBusy) {
        self.server_is_busy = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_server_is_busy(&self) -> &ServerIsBusy {
        match self.server_is_busy.as_ref() {
            Some(v) => v,
            None => ServerIsBusy::default_ref(),
        }
    }
    #[inline]
    pub fn mut_server_is_busy(&mut self) -> &mut ServerIsBusy {
        if self.server_is_busy.is_none() {
            self.server_is_busy = ::std::option::Option::Some(ServerIsBusy::default());
        }
        self.server_is_busy.as_mut().unwrap()
    }
    #[inline]
    pub fn take_server_is_busy(&mut self) -> ServerIsBusy {
        self.server_is_busy
            .take()
            .unwrap_or_else(ServerIsBusy::default)
    }
    #[inline]
    pub fn has_stale_command(&self) -> bool {
        self.stale_command.is_some()
    }
    #[inline]
    pub fn clear_stale_command(&mut self) {
        self.stale_command = ::std::option::Option::None
    }
    #[inline]
    pub fn set_stale_command(&mut self, v: StaleCommand) {
        self.stale_command = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_stale_command(&self) -> &StaleCommand {
        match self.stale_command.as_ref() {
            Some(v) => v,
            None => StaleCommand::default_ref(),
        }
    }
    #[inline]
    pub fn mut_stale_command(&mut self) -> &mut StaleCommand {
        if self.stale_command.is_none() {
            self.stale_command = ::std::option::Option::Some(StaleCommand::default());
        }
        self.stale_command.as_mut().unwrap()
    }
    #[inline]
    pub fn take_stale_command(&mut self) -> StaleCommand {
        self.stale_command
            .take()
            .unwrap_or_else(StaleCommand::default)
    }
    #[inline]
    pub fn has_store_not_match(&self) -> bool {
        self.store_not_match.is_some()
    }
    #[inline]
    pub fn clear_store_not_match(&mut self) {
        self.store_not_match = ::std::option::Option::None
    }
    #[inline]
    pub fn set_store_not_match(&mut self, v: StoreNotMatch) {
        self.store_not_match = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_store_not_match(&self) -> &StoreNotMatch {
        match self.store_not_match.as_ref() {
            Some(v) => v,
            None => StoreNotMatch::default_ref(),
        }
    }
    #[inline]
    pub fn mut_store_not_match(&mut self) -> &mut StoreNotMatch {
        if self.store_not_match.is_none() {
            self.store_not_match = ::std::option::Option::Some(StoreNotMatch::default());
        }
        self.store_not_match.as_mut().unwrap()
    }
    #[inline]
    pub fn take_store_not_match(&mut self) -> StoreNotMatch {
        self.store_not_match
            .take()
            .unwrap_or_else(StoreNotMatch::default)
    }
    #[inline]
    pub fn has_raft_entry_too_large(&self) -> bool {
        self.raft_entry_too_large.is_some()
    }
    #[inline]
    pub fn clear_raft_entry_too_large(&mut self) {
        self.raft_entry_too_large = ::std::option::Option::None
    }
    #[inline]
    pub fn set_raft_entry_too_large(&mut self, v: RaftEntryTooLarge) {
        self.raft_entry_too_large = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_raft_entry_too_large(&self) -> &RaftEntryTooLarge {
        match self.raft_entry_too_large.as_ref() {
            Some(v) => v,
            None => RaftEntryTooLarge::default_ref(),
        }
    }
    #[inline]
    pub fn mut_raft_entry_too_large(&mut self) -> &mut RaftEntryTooLarge {
        if self.raft_entry_too_large.is_none() {
            self.raft_entry_too_large = ::std::option::Option::Some(RaftEntryTooLarge::default());
        }
        self.raft_entry_too_large.as_mut().unwrap()
    }
    #[inline]
    pub fn take_raft_entry_too_large(&mut self) -> RaftEntryTooLarge {
        self.raft_entry_too_large
            .take()
            .unwrap_or_else(RaftEntryTooLarge::default)
    }
}
