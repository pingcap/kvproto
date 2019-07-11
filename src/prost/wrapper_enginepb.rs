// Generated file, please don't edit manually.

impl CommandRequestHeader {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandRequestHeader = CommandRequestHeader::default();
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
    pub fn clear_destroy(&mut self) {
        self.destroy = false
    }
    #[inline]
    pub fn set_destroy(&mut self, v: bool) {
        self.destroy = v;
    }
    #[inline]
    pub fn get_destroy(&self) -> bool {
        self.destroy
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context.clear();
    }
    #[inline]
    pub fn set_context(&mut self, v: std::vec::Vec<u8>) {
        self.context = v;
    }
    #[inline]
    pub fn get_context(&self) -> &[u8] {
        &self.context
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.context
    }
    #[inline]
    pub fn take_context(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.context, ::std::vec::Vec::new())
    }
}
impl CommandRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandRequest = CommandRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    #[inline]
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    #[inline]
    pub fn set_header(&mut self, v: CommandRequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &CommandRequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => CommandRequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut CommandRequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(CommandRequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> CommandRequestHeader {
        self.header
            .take()
            .unwrap_or_else(CommandRequestHeader::default)
    }
    #[inline]
    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }
    #[inline]
    pub fn set_requests(&mut self, v: ::std::vec::Vec<super::raft_cmdpb::Request>) {
        self.requests = v;
    }
    #[inline]
    pub fn get_requests(&self) -> &::std::vec::Vec<super::raft_cmdpb::Request> {
        &self.requests
    }
    #[inline]
    pub fn mut_requests(&mut self) -> &mut ::std::vec::Vec<super::raft_cmdpb::Request> {
        &mut self.requests
    }
    #[inline]
    pub fn take_requests(&mut self) -> ::std::vec::Vec<super::raft_cmdpb::Request> {
        ::std::mem::replace(&mut self.requests, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn has_admin_request(&self) -> bool {
        self.admin_request.is_some()
    }
    #[inline]
    pub fn clear_admin_request(&mut self) {
        self.admin_request = ::std::option::Option::None
    }
    #[inline]
    pub fn set_admin_request(&mut self, v: super::raft_cmdpb::AdminRequest) {
        self.admin_request = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_admin_request(&self) -> &super::raft_cmdpb::AdminRequest {
        match self.admin_request.as_ref() {
            Some(v) => v,
            None => super::raft_cmdpb::AdminRequest::default_ref(),
        }
    }
    #[inline]
    pub fn mut_admin_request(&mut self) -> &mut super::raft_cmdpb::AdminRequest {
        if self.admin_request.is_none() {
            self.admin_request =
                ::std::option::Option::Some(super::raft_cmdpb::AdminRequest::default());
        }
        self.admin_request.as_mut().unwrap()
    }
    #[inline]
    pub fn take_admin_request(&mut self) -> super::raft_cmdpb::AdminRequest {
        self.admin_request
            .take()
            .unwrap_or_else(super::raft_cmdpb::AdminRequest::default)
    }
    #[inline]
    pub fn has_admin_response(&self) -> bool {
        self.admin_response.is_some()
    }
    #[inline]
    pub fn clear_admin_response(&mut self) {
        self.admin_response = ::std::option::Option::None
    }
    #[inline]
    pub fn set_admin_response(&mut self, v: super::raft_cmdpb::AdminResponse) {
        self.admin_response = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_admin_response(&self) -> &super::raft_cmdpb::AdminResponse {
        match self.admin_response.as_ref() {
            Some(v) => v,
            None => super::raft_cmdpb::AdminResponse::default_ref(),
        }
    }
    #[inline]
    pub fn mut_admin_response(&mut self) -> &mut super::raft_cmdpb::AdminResponse {
        if self.admin_response.is_none() {
            self.admin_response =
                ::std::option::Option::Some(super::raft_cmdpb::AdminResponse::default());
        }
        self.admin_response.as_mut().unwrap()
    }
    #[inline]
    pub fn take_admin_response(&mut self) -> super::raft_cmdpb::AdminResponse {
        self.admin_response
            .take()
            .unwrap_or_else(super::raft_cmdpb::AdminResponse::default)
    }
}
impl CommandRequestBatch {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandRequestBatch = CommandRequestBatch::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }
    #[inline]
    pub fn set_requests(&mut self, v: ::std::vec::Vec<CommandRequest>) {
        self.requests = v;
    }
    #[inline]
    pub fn get_requests(&self) -> &::std::vec::Vec<CommandRequest> {
        &self.requests
    }
    #[inline]
    pub fn mut_requests(&mut self) -> &mut ::std::vec::Vec<CommandRequest> {
        &mut self.requests
    }
    #[inline]
    pub fn take_requests(&mut self) -> ::std::vec::Vec<CommandRequest> {
        ::std::mem::replace(&mut self.requests, ::std::vec::Vec::new())
    }
}
impl CommandResponseHeader {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandResponseHeader = CommandResponseHeader::default();
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
    pub fn clear_destroyed(&mut self) {
        self.destroyed = false
    }
    #[inline]
    pub fn set_destroyed(&mut self, v: bool) {
        self.destroyed = v;
    }
    #[inline]
    pub fn get_destroyed(&self) -> bool {
        self.destroyed
    }
}
impl CommandResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandResponse = CommandResponse::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    #[inline]
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    #[inline]
    pub fn set_header(&mut self, v: CommandResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &CommandResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => CommandResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut CommandResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(CommandResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> CommandResponseHeader {
        self.header
            .take()
            .unwrap_or_else(CommandResponseHeader::default)
    }
    #[inline]
    pub fn has_apply_state(&self) -> bool {
        self.apply_state.is_some()
    }
    #[inline]
    pub fn clear_apply_state(&mut self) {
        self.apply_state = ::std::option::Option::None
    }
    #[inline]
    pub fn set_apply_state(&mut self, v: super::raft_serverpb::RaftApplyState) {
        self.apply_state = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_apply_state(&self) -> &super::raft_serverpb::RaftApplyState {
        match self.apply_state.as_ref() {
            Some(v) => v,
            None => super::raft_serverpb::RaftApplyState::default_ref(),
        }
    }
    #[inline]
    pub fn mut_apply_state(&mut self) -> &mut super::raft_serverpb::RaftApplyState {
        if self.apply_state.is_none() {
            self.apply_state =
                ::std::option::Option::Some(super::raft_serverpb::RaftApplyState::default());
        }
        self.apply_state.as_mut().unwrap()
    }
    #[inline]
    pub fn take_apply_state(&mut self) -> super::raft_serverpb::RaftApplyState {
        self.apply_state
            .take()
            .unwrap_or_else(super::raft_serverpb::RaftApplyState::default)
    }
    #[inline]
    pub fn clear_applied_term(&mut self) {
        self.applied_term = 0
    }
    #[inline]
    pub fn set_applied_term(&mut self, v: u64) {
        self.applied_term = v;
    }
    #[inline]
    pub fn get_applied_term(&self) -> u64 {
        self.applied_term
    }
}
impl CommandResponseBatch {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandResponseBatch = CommandResponseBatch::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }
    #[inline]
    pub fn set_responses(&mut self, v: ::std::vec::Vec<CommandResponse>) {
        self.responses = v;
    }
    #[inline]
    pub fn get_responses(&self) -> &::std::vec::Vec<CommandResponse> {
        &self.responses
    }
    #[inline]
    pub fn mut_responses(&mut self) -> &mut ::std::vec::Vec<CommandResponse> {
        &mut self.responses
    }
    #[inline]
    pub fn take_responses(&mut self) -> ::std::vec::Vec<CommandResponse> {
        ::std::mem::replace(&mut self.responses, ::std::vec::Vec::new())
    }
}
impl SnapshotState {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotState = SnapshotState::default();
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
    pub fn has_apply_state(&self) -> bool {
        self.apply_state.is_some()
    }
    #[inline]
    pub fn clear_apply_state(&mut self) {
        self.apply_state = ::std::option::Option::None
    }
    #[inline]
    pub fn set_apply_state(&mut self, v: super::raft_serverpb::RaftApplyState) {
        self.apply_state = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_apply_state(&self) -> &super::raft_serverpb::RaftApplyState {
        match self.apply_state.as_ref() {
            Some(v) => v,
            None => super::raft_serverpb::RaftApplyState::default_ref(),
        }
    }
    #[inline]
    pub fn mut_apply_state(&mut self) -> &mut super::raft_serverpb::RaftApplyState {
        if self.apply_state.is_none() {
            self.apply_state =
                ::std::option::Option::Some(super::raft_serverpb::RaftApplyState::default());
        }
        self.apply_state.as_mut().unwrap()
    }
    #[inline]
    pub fn take_apply_state(&mut self) -> super::raft_serverpb::RaftApplyState {
        self.apply_state
            .take()
            .unwrap_or_else(super::raft_serverpb::RaftApplyState::default)
    }
}
impl SnapshotData {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotData = SnapshotData::default();
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
    #[inline]
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
    #[inline]
    pub fn set_data(&mut self, v: ::std::vec::Vec<super::raft_serverpb::KeyValue>) {
        self.data = v;
    }
    #[inline]
    pub fn get_data(&self) -> &::std::vec::Vec<super::raft_serverpb::KeyValue> {
        &self.data
    }
    #[inline]
    pub fn mut_data(&mut self) -> &mut ::std::vec::Vec<super::raft_serverpb::KeyValue> {
        &mut self.data
    }
    #[inline]
    pub fn take_data(&mut self) -> ::std::vec::Vec<super::raft_serverpb::KeyValue> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
}
impl SnapshotRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotRequest = SnapshotRequest::default();
        }
        &*INSTANCE
    }
}
impl SnapshotDone {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotDone = SnapshotDone::default();
        }
        &*INSTANCE
    }
}
