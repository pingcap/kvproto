// Generated file, please don't edit manually.

impl CommandRequestHeader {
    pub fn new_() -> CommandRequestHeader {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for CommandRequestHeader {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommandRequestHeader {
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
    fn default_instance() -> &'static CommandRequestHeader {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandRequestHeader = CommandRequestHeader::new_();
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
impl CommandRequest {
    pub fn new_() -> CommandRequest {
        ::std::default::Default::default()
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
            None => <CommandRequestHeader as ::protobuf::Message>::default_instance(),
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
            None => <super::raft_cmdpb::AdminRequest as ::protobuf::Message>::default_instance(),
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
            None => <super::raft_cmdpb::AdminResponse as ::protobuf::Message>::default_instance(),
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
impl ::protobuf::Clear for CommandRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommandRequest {
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
    fn default_instance() -> &'static CommandRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandRequest = CommandRequest::new_();
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
impl CommandRequestBatch {
    pub fn new_() -> CommandRequestBatch {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for CommandRequestBatch {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommandRequestBatch {
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
    fn default_instance() -> &'static CommandRequestBatch {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandRequestBatch = CommandRequestBatch::new_();
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
impl CommandResponseHeader {
    pub fn new_() -> CommandResponseHeader {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for CommandResponseHeader {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommandResponseHeader {
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
    fn default_instance() -> &'static CommandResponseHeader {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandResponseHeader = CommandResponseHeader::new_();
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
impl CommandResponse {
    pub fn new_() -> CommandResponse {
        ::std::default::Default::default()
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
            None => <CommandResponseHeader as ::protobuf::Message>::default_instance(),
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
            None => {
                <super::raft_serverpb::RaftApplyState as ::protobuf::Message>::default_instance()
            }
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
impl ::protobuf::Clear for CommandResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommandResponse {
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
    fn default_instance() -> &'static CommandResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandResponse = CommandResponse::new_();
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
impl CommandResponseBatch {
    pub fn new_() -> CommandResponseBatch {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for CommandResponseBatch {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommandResponseBatch {
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
    fn default_instance() -> &'static CommandResponseBatch {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CommandResponseBatch = CommandResponseBatch::new_();
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
impl SnapshotState {
    pub fn new_() -> SnapshotState {
        ::std::default::Default::default()
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
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
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
            None => <super::metapb::Peer as ::protobuf::Message>::default_instance(),
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
            None => {
                <super::raft_serverpb::RaftApplyState as ::protobuf::Message>::default_instance()
            }
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
impl ::protobuf::Clear for SnapshotState {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SnapshotState {
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
    fn default_instance() -> &'static SnapshotState {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotState = SnapshotState::new_();
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
impl SnapshotData {
    pub fn new_() -> SnapshotData {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for SnapshotData {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SnapshotData {
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
    fn default_instance() -> &'static SnapshotData {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotData = SnapshotData::new_();
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
impl SnapshotRequest {
    pub fn new_() -> SnapshotRequest {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for SnapshotRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SnapshotRequest {
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
    fn default_instance() -> &'static SnapshotRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotRequest = SnapshotRequest::new_();
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
impl SnapshotDone {
    pub fn new_() -> SnapshotDone {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for SnapshotDone {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SnapshotDone {
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
    fn default_instance() -> &'static SnapshotDone {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SnapshotDone = SnapshotDone::new_();
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
