// Generated file, please don't edit manually.

impl BatchCommandsRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
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
impl ::protobuf::Clear for BatchCommandsRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchCommandsRequest {
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
    fn default_instance() -> &'static BatchCommandsRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchCommandsRequest = BatchCommandsRequest::default();
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
impl batch_commands_request::Request {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
}
impl ::protobuf::Clear for batch_commands_request::Request {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for batch_commands_request::Request {
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
    fn default_instance() -> &'static batch_commands_request::Request {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: batch_commands_request::Request = batch_commands_request::Request::default();
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
impl BatchCommandsResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
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
impl ::protobuf::Clear for BatchCommandsResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchCommandsResponse {
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
    fn default_instance() -> &'static BatchCommandsResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchCommandsResponse = BatchCommandsResponse::default();
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
impl batch_commands_response::Response {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
}
impl ::protobuf::Clear for batch_commands_response::Response {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for batch_commands_response::Response {
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
    fn default_instance() -> &'static batch_commands_response::Response {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: batch_commands_response::Response = batch_commands_response::Response::default();
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
impl BatchRaftMessage {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
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
impl ::protobuf::Clear for BatchRaftMessage {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchRaftMessage {
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
    fn default_instance() -> &'static BatchRaftMessage {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchRaftMessage = BatchRaftMessage::default();
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
impl BatchCommandsEmptyRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_test_id(&mut self) {
        self.test_id = 0
    }
    #[inline]
    pub fn set_test_id(&mut self, v: u64) {
        self.test_id = v;
    }
    #[inline]
    pub fn get_test_id(&self) -> u64 {
        self.test_id
    }
    #[inline]
    pub fn clear_delay_time(&mut self) {
        self.delay_time = 0
    }
    #[inline]
    pub fn set_delay_time(&mut self, v: u64) {
        self.delay_time = v;
    }
    #[inline]
    pub fn get_delay_time(&self) -> u64 {
        self.delay_time
    }
}
impl ::protobuf::Clear for BatchCommandsEmptyRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchCommandsEmptyRequest {
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
    fn default_instance() -> &'static BatchCommandsEmptyRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchCommandsEmptyRequest = BatchCommandsEmptyRequest::default();
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
impl BatchCommandsEmptyResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::protobuf::Message::default_instance()
    }
    #[inline]
    pub fn clear_test_id(&mut self) {
        self.test_id = 0
    }
    #[inline]
    pub fn set_test_id(&mut self, v: u64) {
        self.test_id = v;
    }
    #[inline]
    pub fn get_test_id(&self) -> u64 {
        self.test_id
    }
}
impl ::protobuf::Clear for BatchCommandsEmptyResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchCommandsEmptyResponse {
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
    fn default_instance() -> &'static BatchCommandsEmptyResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BatchCommandsEmptyResponse = BatchCommandsEmptyResponse::default();
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
