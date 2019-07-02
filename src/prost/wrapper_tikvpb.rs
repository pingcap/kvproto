// Generated file, please don't edit manually.

impl BatchCommandsRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchCommandsRequest = BatchCommandsRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }
    #[inline]
    pub fn set_requests(&mut self, v: ::std::vec::Vec<batch_commands_request::Request>) {
        self.requests = v;
    }
    #[inline]
    pub fn get_requests(&self) -> &::std::vec::Vec<batch_commands_request::Request> {
        &self.requests
    }
    #[inline]
    pub fn mut_requests(&mut self) -> &mut ::std::vec::Vec<batch_commands_request::Request> {
        &mut self.requests
    }
    #[inline]
    pub fn take_requests(&mut self) -> ::std::vec::Vec<batch_commands_request::Request> {
        ::std::mem::replace(&mut self.requests, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_request_ids(&mut self) {
        self.request_ids.clear();
    }
    #[inline]
    pub fn set_request_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.request_ids = v;
    }
    #[inline]
    pub fn get_request_ids(&self) -> &::std::vec::Vec<u64> {
        &self.request_ids
    }
    #[inline]
    pub fn mut_request_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.request_ids
    }
    #[inline]
    pub fn take_request_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.request_ids, ::std::vec::Vec::new())
    }
}
impl batch_commands_request::Request {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: batch_commands_request::Request = batch_commands_request::Request::default();
        }
        &*INSTANCE
    }
}
impl BatchCommandsResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchCommandsResponse = BatchCommandsResponse::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }
    #[inline]
    pub fn set_responses(&mut self, v: ::std::vec::Vec<batch_commands_response::Response>) {
        self.responses = v;
    }
    #[inline]
    pub fn get_responses(&self) -> &::std::vec::Vec<batch_commands_response::Response> {
        &self.responses
    }
    #[inline]
    pub fn mut_responses(&mut self) -> &mut ::std::vec::Vec<batch_commands_response::Response> {
        &mut self.responses
    }
    #[inline]
    pub fn take_responses(&mut self) -> ::std::vec::Vec<batch_commands_response::Response> {
        ::std::mem::replace(&mut self.responses, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_request_ids(&mut self) {
        self.request_ids.clear();
    }
    #[inline]
    pub fn set_request_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.request_ids = v;
    }
    #[inline]
    pub fn get_request_ids(&self) -> &::std::vec::Vec<u64> {
        &self.request_ids
    }
    #[inline]
    pub fn mut_request_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.request_ids
    }
    #[inline]
    pub fn take_request_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.request_ids, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_transport_layer_load(&mut self) {
        self.transport_layer_load = 0
    }
    #[inline]
    pub fn set_transport_layer_load(&mut self, v: u64) {
        self.transport_layer_load = v;
    }
    #[inline]
    pub fn get_transport_layer_load(&self) -> u64 {
        self.transport_layer_load
    }
}
impl batch_commands_response::Response {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: batch_commands_response::Response = batch_commands_response::Response::default();
        }
        &*INSTANCE
    }
}
impl BatchRaftMessage {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchRaftMessage = BatchRaftMessage::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_msgs(&mut self) {
        self.msgs.clear();
    }
    #[inline]
    pub fn set_msgs(&mut self, v: ::std::vec::Vec<super::raft_serverpb::RaftMessage>) {
        self.msgs = v;
    }
    #[inline]
    pub fn get_msgs(&self) -> &::std::vec::Vec<super::raft_serverpb::RaftMessage> {
        &self.msgs
    }
    #[inline]
    pub fn mut_msgs(&mut self) -> &mut ::std::vec::Vec<super::raft_serverpb::RaftMessage> {
        &mut self.msgs
    }
    #[inline]
    pub fn take_msgs(&mut self) -> ::std::vec::Vec<super::raft_serverpb::RaftMessage> {
        ::std::mem::replace(&mut self.msgs, ::std::vec::Vec::new())
    }
}
