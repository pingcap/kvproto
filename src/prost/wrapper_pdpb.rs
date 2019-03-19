impl RequestHeader {
    pub fn new_() -> RequestHeader {
        ::std::default::Default::default()
    }
    pub fn clear_cluster_id(&mut self) {
        self.cluster_id = 0
    }
    pub fn set_cluster_id(&mut self, v: u64) {
        self.cluster_id = v;
    }
    pub fn get_cluster_id(&self) -> u64 {
        self.cluster_id
    }
}
impl ::protobuf::Clear for RequestHeader {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RequestHeader {
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
    fn default_instance() -> &'static RequestHeader {
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
impl ResponseHeader {
    pub fn new_() -> ResponseHeader {
        ::std::default::Default::default()
    }
    pub fn clear_cluster_id(&mut self) {
        self.cluster_id = 0
    }
    pub fn set_cluster_id(&mut self, v: u64) {
        self.cluster_id = v;
    }
    pub fn get_cluster_id(&self) -> u64 {
        self.cluster_id
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: Error) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &Error {
        match self.error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut Error {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(Error::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(Error::default)
    }
}
impl ::protobuf::Clear for ResponseHeader {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ResponseHeader {
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
    fn default_instance() -> &'static ResponseHeader {
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
impl Error {
    pub fn new_() -> Error {
        ::std::default::Default::default()
    }
    pub fn clear_field_type(&mut self) {
        self.r#type = 0
    }
    pub fn set_field_type_(&mut self, v: ErrorType) {
        self.r#type = unsafe { ::std::mem::transmute::<ErrorType, i32>(v) };
    }
    pub fn get_field_type(&self) -> ErrorType {
        unsafe { ::std::mem::transmute::<i32, ErrorType>(self.r#type) }
    }
    pub fn clear_message(&mut self) {
        self.message.clear();
    }
    pub fn set_message(&mut self, v: std::string::String) {
        self.message = v;
    }
    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn mut_message(&mut self) -> &mut std::string::String {
        &mut self.message
    }
    pub fn take_message(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.message, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for Error {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Error {
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
    fn default_instance() -> &'static Error {
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
impl TsoRequest {
    pub fn new_() -> TsoRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn clear_count(&mut self) {
        self.count = 0
    }
    pub fn set_count(&mut self, v: u32) {
        self.count = v;
    }
    pub fn get_count(&self) -> u32 {
        self.count
    }
}
impl ::protobuf::Clear for TsoRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TsoRequest {
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
    fn default_instance() -> &'static TsoRequest {
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
impl Timestamp {
    pub fn new_() -> Timestamp {
        ::std::default::Default::default()
    }
    pub fn clear_physical(&mut self) {
        self.physical = 0
    }
    pub fn set_physical(&mut self, v: i64) {
        self.physical = v;
    }
    pub fn get_physical(&self) -> i64 {
        self.physical
    }
    pub fn clear_logical(&mut self) {
        self.logical = 0
    }
    pub fn set_logical(&mut self, v: i64) {
        self.logical = v;
    }
    pub fn get_logical(&self) -> i64 {
        self.logical
    }
}
impl ::protobuf::Clear for Timestamp {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Timestamp {
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
    fn default_instance() -> &'static Timestamp {
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
impl TsoResponse {
    pub fn new_() -> TsoResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn clear_count(&mut self) {
        self.count = 0
    }
    pub fn set_count(&mut self, v: u32) {
        self.count = v;
    }
    pub fn get_count(&self) -> u32 {
        self.count
    }
    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }
    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None
    }
    pub fn set_timestamp(&mut self, v: Timestamp) {
        self.timestamp = ::std::option::Option::Some(v);;    }
    pub fn get_timestamp(&self) -> &Timestamp {
        match self.timestamp.as_ref() {
            Some(v) => v,
            None => <Timestamp as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_timestamp(&mut self) -> &mut Timestamp {
        if self.timestamp.is_none() {
            self.timestamp = ::std::option::Option::Some(Timestamp::default());
        }
        self.timestamp.as_mut().unwrap()
    }
    pub fn take_timestamp(&mut self) -> Timestamp {
        self.timestamp.take().unwrap_or_else(Timestamp::default)
    }
}
impl ::protobuf::Clear for TsoResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TsoResponse {
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
    fn default_instance() -> &'static TsoResponse {
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
impl BootstrapRequest {
    pub fn new_() -> BootstrapRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn has_store(&self) -> bool {
        self.store.is_some()
    }
    pub fn clear_store(&mut self) {
        self.store = ::std::option::Option::None
    }
    pub fn set_store(&mut self, v: super::metapb::Store) {
        self.store = ::std::option::Option::Some(v);;    }
    pub fn get_store(&self) -> &super::metapb::Store {
        match self.store.as_ref() {
            Some(v) => v,
            None => <super::metapb::Store as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_store(&mut self) -> &mut super::metapb::Store {
        if self.store.is_none() {
            self.store = ::std::option::Option::Some(super::metapb::Store::default());
        }
        self.store.as_mut().unwrap()
    }
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store
            .take()
            .unwrap_or_else(super::metapb::Store::default)
    }
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }
    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None
    }
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::std::option::Option::Some(v);;    }
    pub fn get_region(&self) -> &super::metapb::Region {
        match self.region.as_ref() {
            Some(v) => v,
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.region.as_mut().unwrap()
    }
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
}
impl ::protobuf::Clear for BootstrapRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BootstrapRequest {
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
    fn default_instance() -> &'static BootstrapRequest {
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
impl BootstrapResponse {
    pub fn new_() -> BootstrapResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl ::protobuf::Clear for BootstrapResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BootstrapResponse {
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
    fn default_instance() -> &'static BootstrapResponse {
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
impl IsBootstrappedRequest {
    pub fn new_() -> IsBootstrappedRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
}
impl ::protobuf::Clear for IsBootstrappedRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for IsBootstrappedRequest {
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
    fn default_instance() -> &'static IsBootstrappedRequest {
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
impl IsBootstrappedResponse {
    pub fn new_() -> IsBootstrappedResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn clear_bootstrapped(&mut self) {
        self.bootstrapped = false
    }
    pub fn set_bootstrapped(&mut self, v: bool) {
        self.bootstrapped = v;
    }
    pub fn get_bootstrapped(&self) -> bool {
        self.bootstrapped
    }
}
impl ::protobuf::Clear for IsBootstrappedResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for IsBootstrappedResponse {
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
    fn default_instance() -> &'static IsBootstrappedResponse {
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
impl AllocIdRequest {
    pub fn new_() -> AllocIdRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
}
impl ::protobuf::Clear for AllocIdRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AllocIdRequest {
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
    fn default_instance() -> &'static AllocIdRequest {
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
impl AllocIdResponse {
    pub fn new_() -> AllocIdResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn clear_id(&mut self) {
        self.id = 0
    }
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    pub fn get_id(&self) -> u64 {
        self.id
    }
}
impl ::protobuf::Clear for AllocIdResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AllocIdResponse {
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
    fn default_instance() -> &'static AllocIdResponse {
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
impl GetStoreRequest {
    pub fn new_() -> GetStoreRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn clear_store_id(&mut self) {
        self.store_id = 0
    }
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = v;
    }
    pub fn get_store_id(&self) -> u64 {
        self.store_id
    }
}
impl ::protobuf::Clear for GetStoreRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetStoreRequest {
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
    fn default_instance() -> &'static GetStoreRequest {
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
impl GetStoreResponse {
    pub fn new_() -> GetStoreResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn has_store(&self) -> bool {
        self.store.is_some()
    }
    pub fn clear_store(&mut self) {
        self.store = ::std::option::Option::None
    }
    pub fn set_store(&mut self, v: super::metapb::Store) {
        self.store = ::std::option::Option::Some(v);;    }
    pub fn get_store(&self) -> &super::metapb::Store {
        match self.store.as_ref() {
            Some(v) => v,
            None => <super::metapb::Store as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_store(&mut self) -> &mut super::metapb::Store {
        if self.store.is_none() {
            self.store = ::std::option::Option::Some(super::metapb::Store::default());
        }
        self.store.as_mut().unwrap()
    }
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store
            .take()
            .unwrap_or_else(super::metapb::Store::default)
    }
}
impl ::protobuf::Clear for GetStoreResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetStoreResponse {
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
    fn default_instance() -> &'static GetStoreResponse {
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
impl PutStoreRequest {
    pub fn new_() -> PutStoreRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn has_store(&self) -> bool {
        self.store.is_some()
    }
    pub fn clear_store(&mut self) {
        self.store = ::std::option::Option::None
    }
    pub fn set_store(&mut self, v: super::metapb::Store) {
        self.store = ::std::option::Option::Some(v);;    }
    pub fn get_store(&self) -> &super::metapb::Store {
        match self.store.as_ref() {
            Some(v) => v,
            None => <super::metapb::Store as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_store(&mut self) -> &mut super::metapb::Store {
        if self.store.is_none() {
            self.store = ::std::option::Option::Some(super::metapb::Store::default());
        }
        self.store.as_mut().unwrap()
    }
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store
            .take()
            .unwrap_or_else(super::metapb::Store::default)
    }
}
impl ::protobuf::Clear for PutStoreRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PutStoreRequest {
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
    fn default_instance() -> &'static PutStoreRequest {
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
impl PutStoreResponse {
    pub fn new_() -> PutStoreResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl ::protobuf::Clear for PutStoreResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PutStoreResponse {
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
    fn default_instance() -> &'static PutStoreResponse {
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
impl GetAllStoresRequest {
    pub fn new_() -> GetAllStoresRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn clear_exclude_tombstone_stores(&mut self) {
        self.exclude_tombstone_stores = false
    }
    pub fn set_exclude_tombstone_stores(&mut self, v: bool) {
        self.exclude_tombstone_stores = v;
    }
    pub fn get_exclude_tombstone_stores(&self) -> bool {
        self.exclude_tombstone_stores
    }
}
impl ::protobuf::Clear for GetAllStoresRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetAllStoresRequest {
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
    fn default_instance() -> &'static GetAllStoresRequest {
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
impl GetAllStoresResponse {
    pub fn new_() -> GetAllStoresResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn clear_stores(&mut self) {
        self.stores.clear();
    }
    pub fn set_stores(&mut self, v: ::std::vec::Vec<super::metapb::Store>) {
        self.stores = v;
    }
    pub fn get_stores(&self) -> &::std::vec::Vec<super::metapb::Store> {
        &self.stores
    }
    pub fn mut_stores(&mut self) -> &mut ::std::vec::Vec<super::metapb::Store> {
        &mut self.stores
    }
    pub fn take_stores(&mut self) -> ::std::vec::Vec<super::metapb::Store> {
        ::std::mem::replace(&mut self.stores, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for GetAllStoresResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetAllStoresResponse {
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
    fn default_instance() -> &'static GetAllStoresResponse {
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
impl GetRegionRequest {
    pub fn new_() -> GetRegionRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn clear_region_key(&mut self) {
        self.region_key.clear();
    }
    pub fn set_region_key(&mut self, v: std::vec::Vec<u8>) {
        self.region_key = v;
    }
    pub fn get_region_key(&self) -> &[u8] {
        &self.region_key
    }
    pub fn mut_region_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.region_key
    }
    pub fn take_region_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.region_key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for GetRegionRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetRegionRequest {
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
    fn default_instance() -> &'static GetRegionRequest {
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
impl GetRegionResponse {
    pub fn new_() -> GetRegionResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }
    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None
    }
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::std::option::Option::Some(v);;    }
    pub fn get_region(&self) -> &super::metapb::Region {
        match self.region.as_ref() {
            Some(v) => v,
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.region.as_mut().unwrap()
    }
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }
    pub fn clear_leader(&mut self) {
        self.leader = ::std::option::Option::None
    }
    pub fn set_leader(&mut self, v: super::metapb::Peer) {
        self.leader = ::std::option::Option::Some(v);;    }
    pub fn get_leader(&self) -> &super::metapb::Peer {
        match self.leader.as_ref() {
            Some(v) => v,
            None => <super::metapb::Peer as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_leader(&mut self) -> &mut super::metapb::Peer {
        if self.leader.is_none() {
            self.leader = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.leader.as_mut().unwrap()
    }
    pub fn take_leader(&mut self) -> super::metapb::Peer {
        self.leader
            .take()
            .unwrap_or_else(super::metapb::Peer::default)
    }
}
impl ::protobuf::Clear for GetRegionResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetRegionResponse {
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
    fn default_instance() -> &'static GetRegionResponse {
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
impl GetRegionByIdRequest {
    pub fn new_() -> GetRegionByIdRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
}
impl ::protobuf::Clear for GetRegionByIdRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetRegionByIdRequest {
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
    fn default_instance() -> &'static GetRegionByIdRequest {
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
impl GetClusterConfigRequest {
    pub fn new_() -> GetClusterConfigRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
}
impl ::protobuf::Clear for GetClusterConfigRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetClusterConfigRequest {
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
    fn default_instance() -> &'static GetClusterConfigRequest {
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
impl GetClusterConfigResponse {
    pub fn new_() -> GetClusterConfigResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn has_cluster(&self) -> bool {
        self.cluster.is_some()
    }
    pub fn clear_cluster(&mut self) {
        self.cluster = ::std::option::Option::None
    }
    pub fn set_cluster(&mut self, v: super::metapb::Cluster) {
        self.cluster = ::std::option::Option::Some(v);;    }
    pub fn get_cluster(&self) -> &super::metapb::Cluster {
        match self.cluster.as_ref() {
            Some(v) => v,
            None => <super::metapb::Cluster as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_cluster(&mut self) -> &mut super::metapb::Cluster {
        if self.cluster.is_none() {
            self.cluster = ::std::option::Option::Some(super::metapb::Cluster::default());
        }
        self.cluster.as_mut().unwrap()
    }
    pub fn take_cluster(&mut self) -> super::metapb::Cluster {
        self.cluster
            .take()
            .unwrap_or_else(super::metapb::Cluster::default)
    }
}
impl ::protobuf::Clear for GetClusterConfigResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetClusterConfigResponse {
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
    fn default_instance() -> &'static GetClusterConfigResponse {
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
impl PutClusterConfigRequest {
    pub fn new_() -> PutClusterConfigRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn has_cluster(&self) -> bool {
        self.cluster.is_some()
    }
    pub fn clear_cluster(&mut self) {
        self.cluster = ::std::option::Option::None
    }
    pub fn set_cluster(&mut self, v: super::metapb::Cluster) {
        self.cluster = ::std::option::Option::Some(v);;    }
    pub fn get_cluster(&self) -> &super::metapb::Cluster {
        match self.cluster.as_ref() {
            Some(v) => v,
            None => <super::metapb::Cluster as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_cluster(&mut self) -> &mut super::metapb::Cluster {
        if self.cluster.is_none() {
            self.cluster = ::std::option::Option::Some(super::metapb::Cluster::default());
        }
        self.cluster.as_mut().unwrap()
    }
    pub fn take_cluster(&mut self) -> super::metapb::Cluster {
        self.cluster
            .take()
            .unwrap_or_else(super::metapb::Cluster::default)
    }
}
impl ::protobuf::Clear for PutClusterConfigRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PutClusterConfigRequest {
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
    fn default_instance() -> &'static PutClusterConfigRequest {
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
impl PutClusterConfigResponse {
    pub fn new_() -> PutClusterConfigResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl ::protobuf::Clear for PutClusterConfigResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PutClusterConfigResponse {
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
    fn default_instance() -> &'static PutClusterConfigResponse {
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
impl Member {
    pub fn new_() -> Member {
        ::std::default::Default::default()
    }
    pub fn clear_name(&mut self) {
        self.name.clear();
    }
    pub fn set_name(&mut self, v: std::string::String) {
        self.name = v;
    }
    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn mut_name(&mut self) -> &mut std::string::String {
        &mut self.name
    }
    pub fn take_name(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }
    pub fn clear_member_id(&mut self) {
        self.member_id = 0
    }
    pub fn set_member_id(&mut self, v: u64) {
        self.member_id = v;
    }
    pub fn get_member_id(&self) -> u64 {
        self.member_id
    }
    pub fn clear_peer_urls(&mut self) {
        self.peer_urls.clear();
    }
    pub fn set_peer_urls(&mut self, v: ::std::vec::Vec<std::string::String>) {
        self.peer_urls = v;
    }
    pub fn get_peer_urls(&self) -> &::std::vec::Vec<std::string::String> {
        &self.peer_urls
    }
    pub fn mut_peer_urls(&mut self) -> &mut ::std::vec::Vec<std::string::String> {
        &mut self.peer_urls
    }
    pub fn take_peer_urls(&mut self) -> ::std::vec::Vec<std::string::String> {
        ::std::mem::replace(&mut self.peer_urls, ::std::vec::Vec::new())
    }
    pub fn clear_client_urls(&mut self) {
        self.client_urls.clear();
    }
    pub fn set_client_urls(&mut self, v: ::std::vec::Vec<std::string::String>) {
        self.client_urls = v;
    }
    pub fn get_client_urls(&self) -> &::std::vec::Vec<std::string::String> {
        &self.client_urls
    }
    pub fn mut_client_urls(&mut self) -> &mut ::std::vec::Vec<std::string::String> {
        &mut self.client_urls
    }
    pub fn take_client_urls(&mut self) -> ::std::vec::Vec<std::string::String> {
        ::std::mem::replace(&mut self.client_urls, ::std::vec::Vec::new())
    }
    pub fn clear_leader_priority(&mut self) {
        self.leader_priority = 0
    }
    pub fn set_leader_priority(&mut self, v: i32) {
        self.leader_priority = v;
    }
    pub fn get_leader_priority(&self) -> i32 {
        self.leader_priority
    }
}
impl ::protobuf::Clear for Member {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Member {
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
    fn default_instance() -> &'static Member {
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
impl GetMembersRequest {
    pub fn new_() -> GetMembersRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
}
impl ::protobuf::Clear for GetMembersRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetMembersRequest {
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
    fn default_instance() -> &'static GetMembersRequest {
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
impl GetMembersResponse {
    pub fn new_() -> GetMembersResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn clear_members(&mut self) {
        self.members.clear();
    }
    pub fn set_members(&mut self, v: ::std::vec::Vec<Member>) {
        self.members = v;
    }
    pub fn get_members(&self) -> &::std::vec::Vec<Member> {
        &self.members
    }
    pub fn mut_members(&mut self) -> &mut ::std::vec::Vec<Member> {
        &mut self.members
    }
    pub fn take_members(&mut self) -> ::std::vec::Vec<Member> {
        ::std::mem::replace(&mut self.members, ::std::vec::Vec::new())
    }
    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }
    pub fn clear_leader(&mut self) {
        self.leader = ::std::option::Option::None
    }
    pub fn set_leader(&mut self, v: Member) {
        self.leader = ::std::option::Option::Some(v);;    }
    pub fn get_leader(&self) -> &Member {
        match self.leader.as_ref() {
            Some(v) => v,
            None => <Member as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_leader(&mut self) -> &mut Member {
        if self.leader.is_none() {
            self.leader = ::std::option::Option::Some(Member::default());
        }
        self.leader.as_mut().unwrap()
    }
    pub fn take_leader(&mut self) -> Member {
        self.leader.take().unwrap_or_else(Member::default)
    }
    pub fn has_etcd_leader(&self) -> bool {
        self.etcd_leader.is_some()
    }
    pub fn clear_etcd_leader(&mut self) {
        self.etcd_leader = ::std::option::Option::None
    }
    pub fn set_etcd_leader(&mut self, v: Member) {
        self.etcd_leader = ::std::option::Option::Some(v);;    }
    pub fn get_etcd_leader(&self) -> &Member {
        match self.etcd_leader.as_ref() {
            Some(v) => v,
            None => <Member as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_etcd_leader(&mut self) -> &mut Member {
        if self.etcd_leader.is_none() {
            self.etcd_leader = ::std::option::Option::Some(Member::default());
        }
        self.etcd_leader.as_mut().unwrap()
    }
    pub fn take_etcd_leader(&mut self) -> Member {
        self.etcd_leader.take().unwrap_or_else(Member::default)
    }
}
impl ::protobuf::Clear for GetMembersResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetMembersResponse {
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
    fn default_instance() -> &'static GetMembersResponse {
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
impl PeerStats {
    pub fn new_() -> PeerStats {
        ::std::default::Default::default()
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
            .unwrap_or_else(super::metapb::Peer::default)
    }
    pub fn clear_down_seconds(&mut self) {
        self.down_seconds = 0
    }
    pub fn set_down_seconds(&mut self, v: u64) {
        self.down_seconds = v;
    }
    pub fn get_down_seconds(&self) -> u64 {
        self.down_seconds
    }
}
impl ::protobuf::Clear for PeerStats {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PeerStats {
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
    fn default_instance() -> &'static PeerStats {
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
impl RegionHeartbeatRequest {
    pub fn new_() -> RegionHeartbeatRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }
    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None
    }
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::std::option::Option::Some(v);;    }
    pub fn get_region(&self) -> &super::metapb::Region {
        match self.region.as_ref() {
            Some(v) => v,
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.region.as_mut().unwrap()
    }
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }
    pub fn clear_leader(&mut self) {
        self.leader = ::std::option::Option::None
    }
    pub fn set_leader(&mut self, v: super::metapb::Peer) {
        self.leader = ::std::option::Option::Some(v);;    }
    pub fn get_leader(&self) -> &super::metapb::Peer {
        match self.leader.as_ref() {
            Some(v) => v,
            None => <super::metapb::Peer as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_leader(&mut self) -> &mut super::metapb::Peer {
        if self.leader.is_none() {
            self.leader = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.leader.as_mut().unwrap()
    }
    pub fn take_leader(&mut self) -> super::metapb::Peer {
        self.leader
            .take()
            .unwrap_or_else(super::metapb::Peer::default)
    }
    pub fn clear_down_peers(&mut self) {
        self.down_peers.clear();
    }
    pub fn set_down_peers(&mut self, v: ::std::vec::Vec<PeerStats>) {
        self.down_peers = v;
    }
    pub fn get_down_peers(&self) -> &::std::vec::Vec<PeerStats> {
        &self.down_peers
    }
    pub fn mut_down_peers(&mut self) -> &mut ::std::vec::Vec<PeerStats> {
        &mut self.down_peers
    }
    pub fn take_down_peers(&mut self) -> ::std::vec::Vec<PeerStats> {
        ::std::mem::replace(&mut self.down_peers, ::std::vec::Vec::new())
    }
    pub fn clear_pending_peers(&mut self) {
        self.pending_peers.clear();
    }
    pub fn set_pending_peers(&mut self, v: ::std::vec::Vec<super::metapb::Peer>) {
        self.pending_peers = v;
    }
    pub fn get_pending_peers(&self) -> &::std::vec::Vec<super::metapb::Peer> {
        &self.pending_peers
    }
    pub fn mut_pending_peers(&mut self) -> &mut ::std::vec::Vec<super::metapb::Peer> {
        &mut self.pending_peers
    }
    pub fn take_pending_peers(&mut self) -> ::std::vec::Vec<super::metapb::Peer> {
        ::std::mem::replace(&mut self.pending_peers, ::std::vec::Vec::new())
    }
    pub fn clear_bytes_written(&mut self) {
        self.bytes_written = 0
    }
    pub fn set_bytes_written(&mut self, v: u64) {
        self.bytes_written = v;
    }
    pub fn get_bytes_written(&self) -> u64 {
        self.bytes_written
    }
    pub fn clear_bytes_read(&mut self) {
        self.bytes_read = 0
    }
    pub fn set_bytes_read(&mut self, v: u64) {
        self.bytes_read = v;
    }
    pub fn get_bytes_read(&self) -> u64 {
        self.bytes_read
    }
    pub fn clear_keys_written(&mut self) {
        self.keys_written = 0
    }
    pub fn set_keys_written(&mut self, v: u64) {
        self.keys_written = v;
    }
    pub fn get_keys_written(&self) -> u64 {
        self.keys_written
    }
    pub fn clear_keys_read(&mut self) {
        self.keys_read = 0
    }
    pub fn set_keys_read(&mut self, v: u64) {
        self.keys_read = v;
    }
    pub fn get_keys_read(&self) -> u64 {
        self.keys_read
    }
    pub fn clear_approximate_size(&mut self) {
        self.approximate_size = 0
    }
    pub fn set_approximate_size(&mut self, v: u64) {
        self.approximate_size = v;
    }
    pub fn get_approximate_size(&self) -> u64 {
        self.approximate_size
    }
    pub fn has_interval(&self) -> bool {
        self.interval.is_some()
    }
    pub fn clear_interval(&mut self) {
        self.interval = ::std::option::Option::None
    }
    pub fn set_interval(&mut self, v: TimeInterval) {
        self.interval = ::std::option::Option::Some(v);;    }
    pub fn get_interval(&self) -> &TimeInterval {
        match self.interval.as_ref() {
            Some(v) => v,
            None => <TimeInterval as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_interval(&mut self) -> &mut TimeInterval {
        if self.interval.is_none() {
            self.interval = ::std::option::Option::Some(TimeInterval::default());
        }
        self.interval.as_mut().unwrap()
    }
    pub fn take_interval(&mut self) -> TimeInterval {
        self.interval.take().unwrap_or_else(TimeInterval::default)
    }
    pub fn clear_approximate_keys(&mut self) {
        self.approximate_keys = 0
    }
    pub fn set_approximate_keys(&mut self, v: u64) {
        self.approximate_keys = v;
    }
    pub fn get_approximate_keys(&self) -> u64 {
        self.approximate_keys
    }
}
impl ::protobuf::Clear for RegionHeartbeatRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RegionHeartbeatRequest {
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
    fn default_instance() -> &'static RegionHeartbeatRequest {
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
impl ChangePeer {
    pub fn new_() -> ChangePeer {
        ::std::default::Default::default()
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
            .unwrap_or_else(super::metapb::Peer::default)
    }
    pub fn clear_change_type(&mut self) {
        self.change_type = 0
    }
    pub fn set_change_type_(&mut self, v: super::eraftpb::ConfChangeType) {
        self.change_type =
            unsafe { ::std::mem::transmute::<super::eraftpb::ConfChangeType, i32>(v) };
    }
    pub fn get_change_type(&self) -> super::eraftpb::ConfChangeType {
        unsafe { ::std::mem::transmute::<i32, super::eraftpb::ConfChangeType>(self.change_type) }
    }
}
impl ::protobuf::Clear for ChangePeer {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ChangePeer {
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
    fn default_instance() -> &'static ChangePeer {
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
impl TransferLeader {
    pub fn new_() -> TransferLeader {
        ::std::default::Default::default()
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
            .unwrap_or_else(super::metapb::Peer::default)
    }
}
impl ::protobuf::Clear for TransferLeader {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TransferLeader {
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
    fn default_instance() -> &'static TransferLeader {
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
impl Merge {
    pub fn new_() -> Merge {
        ::std::default::Default::default()
    }
    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }
    pub fn clear_target(&mut self) {
        self.target = ::std::option::Option::None
    }
    pub fn set_target(&mut self, v: super::metapb::Region) {
        self.target = ::std::option::Option::Some(v);;    }
    pub fn get_target(&self) -> &super::metapb::Region {
        match self.target.as_ref() {
            Some(v) => v,
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_target(&mut self) -> &mut super::metapb::Region {
        if self.target.is_none() {
            self.target = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.target.as_mut().unwrap()
    }
    pub fn take_target(&mut self) -> super::metapb::Region {
        self.target
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
}
impl ::protobuf::Clear for Merge {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Merge {
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
    fn default_instance() -> &'static Merge {
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
impl SplitRegion {
    pub fn new_() -> SplitRegion {
        ::std::default::Default::default()
    }
    pub fn clear_policy(&mut self) {
        self.policy = 0
    }
    pub fn set_policy_(&mut self, v: CheckPolicy) {
        self.policy = unsafe { ::std::mem::transmute::<CheckPolicy, i32>(v) };
    }
    pub fn get_policy(&self) -> CheckPolicy {
        unsafe { ::std::mem::transmute::<i32, CheckPolicy>(self.policy) }
    }
}
impl ::protobuf::Clear for SplitRegion {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SplitRegion {
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
    fn default_instance() -> &'static SplitRegion {
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
impl RegionHeartbeatResponse {
    pub fn new_() -> RegionHeartbeatResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn has_change_peer(&self) -> bool {
        self.change_peer.is_some()
    }
    pub fn clear_change_peer(&mut self) {
        self.change_peer = ::std::option::Option::None
    }
    pub fn set_change_peer(&mut self, v: ChangePeer) {
        self.change_peer = ::std::option::Option::Some(v);;    }
    pub fn get_change_peer(&self) -> &ChangePeer {
        match self.change_peer.as_ref() {
            Some(v) => v,
            None => <ChangePeer as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_change_peer(&mut self) -> &mut ChangePeer {
        if self.change_peer.is_none() {
            self.change_peer = ::std::option::Option::Some(ChangePeer::default());
        }
        self.change_peer.as_mut().unwrap()
    }
    pub fn take_change_peer(&mut self) -> ChangePeer {
        self.change_peer.take().unwrap_or_else(ChangePeer::default)
    }
    pub fn has_transfer_leader(&self) -> bool {
        self.transfer_leader.is_some()
    }
    pub fn clear_transfer_leader(&mut self) {
        self.transfer_leader = ::std::option::Option::None
    }
    pub fn set_transfer_leader(&mut self, v: TransferLeader) {
        self.transfer_leader = ::std::option::Option::Some(v);;    }
    pub fn get_transfer_leader(&self) -> &TransferLeader {
        match self.transfer_leader.as_ref() {
            Some(v) => v,
            None => <TransferLeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_transfer_leader(&mut self) -> &mut TransferLeader {
        if self.transfer_leader.is_none() {
            self.transfer_leader = ::std::option::Option::Some(TransferLeader::default());
        }
        self.transfer_leader.as_mut().unwrap()
    }
    pub fn take_transfer_leader(&mut self) -> TransferLeader {
        self.transfer_leader
            .take()
            .unwrap_or_else(TransferLeader::default)
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
            .unwrap_or_else(super::metapb::RegionEpoch::default)
    }
    pub fn has_target_peer(&self) -> bool {
        self.target_peer.is_some()
    }
    pub fn clear_target_peer(&mut self) {
        self.target_peer = ::std::option::Option::None
    }
    pub fn set_target_peer(&mut self, v: super::metapb::Peer) {
        self.target_peer = ::std::option::Option::Some(v);;    }
    pub fn get_target_peer(&self) -> &super::metapb::Peer {
        match self.target_peer.as_ref() {
            Some(v) => v,
            None => <super::metapb::Peer as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_target_peer(&mut self) -> &mut super::metapb::Peer {
        if self.target_peer.is_none() {
            self.target_peer = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.target_peer.as_mut().unwrap()
    }
    pub fn take_target_peer(&mut self) -> super::metapb::Peer {
        self.target_peer
            .take()
            .unwrap_or_else(super::metapb::Peer::default)
    }
    pub fn has_merge(&self) -> bool {
        self.merge.is_some()
    }
    pub fn clear_merge(&mut self) {
        self.merge = ::std::option::Option::None
    }
    pub fn set_merge(&mut self, v: Merge) {
        self.merge = ::std::option::Option::Some(v);;    }
    pub fn get_merge(&self) -> &Merge {
        match self.merge.as_ref() {
            Some(v) => v,
            None => <Merge as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_merge(&mut self) -> &mut Merge {
        if self.merge.is_none() {
            self.merge = ::std::option::Option::Some(Merge::default());
        }
        self.merge.as_mut().unwrap()
    }
    pub fn take_merge(&mut self) -> Merge {
        self.merge.take().unwrap_or_else(Merge::default)
    }
    pub fn has_split_region(&self) -> bool {
        self.split_region.is_some()
    }
    pub fn clear_split_region(&mut self) {
        self.split_region = ::std::option::Option::None
    }
    pub fn set_split_region(&mut self, v: SplitRegion) {
        self.split_region = ::std::option::Option::Some(v);;    }
    pub fn get_split_region(&self) -> &SplitRegion {
        match self.split_region.as_ref() {
            Some(v) => v,
            None => <SplitRegion as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_split_region(&mut self) -> &mut SplitRegion {
        if self.split_region.is_none() {
            self.split_region = ::std::option::Option::Some(SplitRegion::default());
        }
        self.split_region.as_mut().unwrap()
    }
    pub fn take_split_region(&mut self) -> SplitRegion {
        self.split_region
            .take()
            .unwrap_or_else(SplitRegion::default)
    }
}
impl ::protobuf::Clear for RegionHeartbeatResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RegionHeartbeatResponse {
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
    fn default_instance() -> &'static RegionHeartbeatResponse {
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
impl AskSplitRequest {
    pub fn new_() -> AskSplitRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }
    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None
    }
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::std::option::Option::Some(v);;    }
    pub fn get_region(&self) -> &super::metapb::Region {
        match self.region.as_ref() {
            Some(v) => v,
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.region.as_mut().unwrap()
    }
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
}
impl ::protobuf::Clear for AskSplitRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AskSplitRequest {
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
    fn default_instance() -> &'static AskSplitRequest {
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
impl AskSplitResponse {
    pub fn new_() -> AskSplitResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn clear_new_region_id(&mut self) {
        self.new_region_id = 0
    }
    pub fn set_new_region_id(&mut self, v: u64) {
        self.new_region_id = v;
    }
    pub fn get_new_region_id(&self) -> u64 {
        self.new_region_id
    }
    pub fn clear_new_peer_ids(&mut self) {
        self.new_peer_ids.clear();
    }
    pub fn set_new_peer_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.new_peer_ids = v;
    }
    pub fn get_new_peer_ids(&self) -> &::std::vec::Vec<u64> {
        &self.new_peer_ids
    }
    pub fn mut_new_peer_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.new_peer_ids
    }
    pub fn take_new_peer_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.new_peer_ids, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for AskSplitResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AskSplitResponse {
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
    fn default_instance() -> &'static AskSplitResponse {
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
impl ReportSplitRequest {
    pub fn new_() -> ReportSplitRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
            .unwrap_or_else(super::metapb::Region::default)
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
            .unwrap_or_else(super::metapb::Region::default)
    }
}
impl ::protobuf::Clear for ReportSplitRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ReportSplitRequest {
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
    fn default_instance() -> &'static ReportSplitRequest {
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
impl ReportSplitResponse {
    pub fn new_() -> ReportSplitResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl ::protobuf::Clear for ReportSplitResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ReportSplitResponse {
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
    fn default_instance() -> &'static ReportSplitResponse {
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
impl AskBatchSplitRequest {
    pub fn new_() -> AskBatchSplitRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }
    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None
    }
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::std::option::Option::Some(v);;    }
    pub fn get_region(&self) -> &super::metapb::Region {
        match self.region.as_ref() {
            Some(v) => v,
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.region.as_mut().unwrap()
    }
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
    pub fn clear_split_count(&mut self) {
        self.split_count = 0
    }
    pub fn set_split_count(&mut self, v: u32) {
        self.split_count = v;
    }
    pub fn get_split_count(&self) -> u32 {
        self.split_count
    }
}
impl ::protobuf::Clear for AskBatchSplitRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AskBatchSplitRequest {
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
    fn default_instance() -> &'static AskBatchSplitRequest {
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
impl SplitId {
    pub fn new_() -> SplitId {
        ::std::default::Default::default()
    }
    pub fn clear_new_region_id(&mut self) {
        self.new_region_id = 0
    }
    pub fn set_new_region_id(&mut self, v: u64) {
        self.new_region_id = v;
    }
    pub fn get_new_region_id(&self) -> u64 {
        self.new_region_id
    }
    pub fn clear_new_peer_ids(&mut self) {
        self.new_peer_ids.clear();
    }
    pub fn set_new_peer_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.new_peer_ids = v;
    }
    pub fn get_new_peer_ids(&self) -> &::std::vec::Vec<u64> {
        &self.new_peer_ids
    }
    pub fn mut_new_peer_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.new_peer_ids
    }
    pub fn take_new_peer_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.new_peer_ids, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for SplitId {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SplitId {
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
    fn default_instance() -> &'static SplitId {
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
impl AskBatchSplitResponse {
    pub fn new_() -> AskBatchSplitResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn clear_ids(&mut self) {
        self.ids.clear();
    }
    pub fn set_ids(&mut self, v: ::std::vec::Vec<SplitId>) {
        self.ids = v;
    }
    pub fn get_ids(&self) -> &::std::vec::Vec<SplitId> {
        &self.ids
    }
    pub fn mut_ids(&mut self) -> &mut ::std::vec::Vec<SplitId> {
        &mut self.ids
    }
    pub fn take_ids(&mut self) -> ::std::vec::Vec<SplitId> {
        ::std::mem::replace(&mut self.ids, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for AskBatchSplitResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AskBatchSplitResponse {
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
    fn default_instance() -> &'static AskBatchSplitResponse {
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
impl ReportBatchSplitRequest {
    pub fn new_() -> ReportBatchSplitRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn clear_regions(&mut self) {
        self.regions.clear();
    }
    pub fn set_regions(&mut self, v: ::std::vec::Vec<super::metapb::Region>) {
        self.regions = v;
    }
    pub fn get_regions(&self) -> &::std::vec::Vec<super::metapb::Region> {
        &self.regions
    }
    pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<super::metapb::Region> {
        &mut self.regions
    }
    pub fn take_regions(&mut self) -> ::std::vec::Vec<super::metapb::Region> {
        ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for ReportBatchSplitRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ReportBatchSplitRequest {
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
    fn default_instance() -> &'static ReportBatchSplitRequest {
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
impl ReportBatchSplitResponse {
    pub fn new_() -> ReportBatchSplitResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl ::protobuf::Clear for ReportBatchSplitResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ReportBatchSplitResponse {
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
    fn default_instance() -> &'static ReportBatchSplitResponse {
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
impl TimeInterval {
    pub fn new_() -> TimeInterval {
        ::std::default::Default::default()
    }
    pub fn clear_start_timestamp(&mut self) {
        self.start_timestamp = 0
    }
    pub fn set_start_timestamp(&mut self, v: u64) {
        self.start_timestamp = v;
    }
    pub fn get_start_timestamp(&self) -> u64 {
        self.start_timestamp
    }
    pub fn clear_end_timestamp(&mut self) {
        self.end_timestamp = 0
    }
    pub fn set_end_timestamp(&mut self, v: u64) {
        self.end_timestamp = v;
    }
    pub fn get_end_timestamp(&self) -> u64 {
        self.end_timestamp
    }
}
impl ::protobuf::Clear for TimeInterval {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TimeInterval {
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
    fn default_instance() -> &'static TimeInterval {
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
impl StoreStats {
    pub fn new_() -> StoreStats {
        ::std::default::Default::default()
    }
    pub fn clear_store_id(&mut self) {
        self.store_id = 0
    }
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = v;
    }
    pub fn get_store_id(&self) -> u64 {
        self.store_id
    }
    pub fn clear_capacity(&mut self) {
        self.capacity = 0
    }
    pub fn set_capacity(&mut self, v: u64) {
        self.capacity = v;
    }
    pub fn get_capacity(&self) -> u64 {
        self.capacity
    }
    pub fn clear_available(&mut self) {
        self.available = 0
    }
    pub fn set_available(&mut self, v: u64) {
        self.available = v;
    }
    pub fn get_available(&self) -> u64 {
        self.available
    }
    pub fn clear_region_count(&mut self) {
        self.region_count = 0
    }
    pub fn set_region_count(&mut self, v: u32) {
        self.region_count = v;
    }
    pub fn get_region_count(&self) -> u32 {
        self.region_count
    }
    pub fn clear_sending_snap_count(&mut self) {
        self.sending_snap_count = 0
    }
    pub fn set_sending_snap_count(&mut self, v: u32) {
        self.sending_snap_count = v;
    }
    pub fn get_sending_snap_count(&self) -> u32 {
        self.sending_snap_count
    }
    pub fn clear_receiving_snap_count(&mut self) {
        self.receiving_snap_count = 0
    }
    pub fn set_receiving_snap_count(&mut self, v: u32) {
        self.receiving_snap_count = v;
    }
    pub fn get_receiving_snap_count(&self) -> u32 {
        self.receiving_snap_count
    }
    pub fn clear_start_time(&mut self) {
        self.start_time = 0
    }
    pub fn set_start_time(&mut self, v: u32) {
        self.start_time = v;
    }
    pub fn get_start_time(&self) -> u32 {
        self.start_time
    }
    pub fn clear_applying_snap_count(&mut self) {
        self.applying_snap_count = 0
    }
    pub fn set_applying_snap_count(&mut self, v: u32) {
        self.applying_snap_count = v;
    }
    pub fn get_applying_snap_count(&self) -> u32 {
        self.applying_snap_count
    }
    pub fn clear_is_busy(&mut self) {
        self.is_busy = false
    }
    pub fn set_is_busy(&mut self, v: bool) {
        self.is_busy = v;
    }
    pub fn get_is_busy(&self) -> bool {
        self.is_busy
    }
    pub fn clear_used_size(&mut self) {
        self.used_size = 0
    }
    pub fn set_used_size(&mut self, v: u64) {
        self.used_size = v;
    }
    pub fn get_used_size(&self) -> u64 {
        self.used_size
    }
    pub fn clear_bytes_written(&mut self) {
        self.bytes_written = 0
    }
    pub fn set_bytes_written(&mut self, v: u64) {
        self.bytes_written = v;
    }
    pub fn get_bytes_written(&self) -> u64 {
        self.bytes_written
    }
    pub fn clear_keys_written(&mut self) {
        self.keys_written = 0
    }
    pub fn set_keys_written(&mut self, v: u64) {
        self.keys_written = v;
    }
    pub fn get_keys_written(&self) -> u64 {
        self.keys_written
    }
    pub fn clear_bytes_read(&mut self) {
        self.bytes_read = 0
    }
    pub fn set_bytes_read(&mut self, v: u64) {
        self.bytes_read = v;
    }
    pub fn get_bytes_read(&self) -> u64 {
        self.bytes_read
    }
    pub fn clear_keys_read(&mut self) {
        self.keys_read = 0
    }
    pub fn set_keys_read(&mut self, v: u64) {
        self.keys_read = v;
    }
    pub fn get_keys_read(&self) -> u64 {
        self.keys_read
    }
    pub fn has_interval(&self) -> bool {
        self.interval.is_some()
    }
    pub fn clear_interval(&mut self) {
        self.interval = ::std::option::Option::None
    }
    pub fn set_interval(&mut self, v: TimeInterval) {
        self.interval = ::std::option::Option::Some(v);;    }
    pub fn get_interval(&self) -> &TimeInterval {
        match self.interval.as_ref() {
            Some(v) => v,
            None => <TimeInterval as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_interval(&mut self) -> &mut TimeInterval {
        if self.interval.is_none() {
            self.interval = ::std::option::Option::Some(TimeInterval::default());
        }
        self.interval.as_mut().unwrap()
    }
    pub fn take_interval(&mut self) -> TimeInterval {
        self.interval.take().unwrap_or_else(TimeInterval::default)
    }
}
impl ::protobuf::Clear for StoreStats {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for StoreStats {
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
    fn default_instance() -> &'static StoreStats {
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
impl StoreHeartbeatRequest {
    pub fn new_() -> StoreHeartbeatRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }
    pub fn clear_stats(&mut self) {
        self.stats = ::std::option::Option::None
    }
    pub fn set_stats(&mut self, v: StoreStats) {
        self.stats = ::std::option::Option::Some(v);;    }
    pub fn get_stats(&self) -> &StoreStats {
        match self.stats.as_ref() {
            Some(v) => v,
            None => <StoreStats as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_stats(&mut self) -> &mut StoreStats {
        if self.stats.is_none() {
            self.stats = ::std::option::Option::Some(StoreStats::default());
        }
        self.stats.as_mut().unwrap()
    }
    pub fn take_stats(&mut self) -> StoreStats {
        self.stats.take().unwrap_or_else(StoreStats::default)
    }
}
impl ::protobuf::Clear for StoreHeartbeatRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for StoreHeartbeatRequest {
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
    fn default_instance() -> &'static StoreHeartbeatRequest {
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
impl StoreHeartbeatResponse {
    pub fn new_() -> StoreHeartbeatResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl ::protobuf::Clear for StoreHeartbeatResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for StoreHeartbeatResponse {
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
    fn default_instance() -> &'static StoreHeartbeatResponse {
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
impl ScatterRegionRequest {
    pub fn new_() -> ScatterRegionRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
    pub fn has_region(&self) -> bool {
        self.region.is_some()
    }
    pub fn clear_region(&mut self) {
        self.region = ::std::option::Option::None
    }
    pub fn set_region(&mut self, v: super::metapb::Region) {
        self.region = ::std::option::Option::Some(v);;    }
    pub fn get_region(&self) -> &super::metapb::Region {
        match self.region.as_ref() {
            Some(v) => v,
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region(&mut self) -> &mut super::metapb::Region {
        if self.region.is_none() {
            self.region = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.region.as_mut().unwrap()
    }
    pub fn take_region(&mut self) -> super::metapb::Region {
        self.region
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
    pub fn has_leader(&self) -> bool {
        self.leader.is_some()
    }
    pub fn clear_leader(&mut self) {
        self.leader = ::std::option::Option::None
    }
    pub fn set_leader(&mut self, v: super::metapb::Peer) {
        self.leader = ::std::option::Option::Some(v);;    }
    pub fn get_leader(&self) -> &super::metapb::Peer {
        match self.leader.as_ref() {
            Some(v) => v,
            None => <super::metapb::Peer as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_leader(&mut self) -> &mut super::metapb::Peer {
        if self.leader.is_none() {
            self.leader = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.leader.as_mut().unwrap()
    }
    pub fn take_leader(&mut self) -> super::metapb::Peer {
        self.leader
            .take()
            .unwrap_or_else(super::metapb::Peer::default)
    }
}
impl ::protobuf::Clear for ScatterRegionRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScatterRegionRequest {
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
    fn default_instance() -> &'static ScatterRegionRequest {
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
impl ScatterRegionResponse {
    pub fn new_() -> ScatterRegionResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl ::protobuf::Clear for ScatterRegionResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ScatterRegionResponse {
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
    fn default_instance() -> &'static ScatterRegionResponse {
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
impl GetGcSafePointRequest {
    pub fn new_() -> GetGcSafePointRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
}
impl ::protobuf::Clear for GetGcSafePointRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetGcSafePointRequest {
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
    fn default_instance() -> &'static GetGcSafePointRequest {
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
impl GetGcSafePointResponse {
    pub fn new_() -> GetGcSafePointResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
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
impl ::protobuf::Clear for GetGcSafePointResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for GetGcSafePointResponse {
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
    fn default_instance() -> &'static GetGcSafePointResponse {
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
impl UpdateGcSafePointRequest {
    pub fn new_() -> UpdateGcSafePointRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
impl ::protobuf::Clear for UpdateGcSafePointRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for UpdateGcSafePointRequest {
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
    fn default_instance() -> &'static UpdateGcSafePointRequest {
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
impl UpdateGcSafePointResponse {
    pub fn new_() -> UpdateGcSafePointResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn clear_new_safe_point(&mut self) {
        self.new_safe_point = 0
    }
    pub fn set_new_safe_point(&mut self, v: u64) {
        self.new_safe_point = v;
    }
    pub fn get_new_safe_point(&self) -> u64 {
        self.new_safe_point
    }
}
impl ::protobuf::Clear for UpdateGcSafePointResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for UpdateGcSafePointResponse {
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
    fn default_instance() -> &'static UpdateGcSafePointResponse {
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
impl SyncRegionRequest {
    pub fn new_() -> SyncRegionRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    pub fn has_member(&self) -> bool {
        self.member.is_some()
    }
    pub fn clear_member(&mut self) {
        self.member = ::std::option::Option::None
    }
    pub fn set_member(&mut self, v: Member) {
        self.member = ::std::option::Option::Some(v);;    }
    pub fn get_member(&self) -> &Member {
        match self.member.as_ref() {
            Some(v) => v,
            None => <Member as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_member(&mut self) -> &mut Member {
        if self.member.is_none() {
            self.member = ::std::option::Option::Some(Member::default());
        }
        self.member.as_mut().unwrap()
    }
    pub fn take_member(&mut self) -> Member {
        self.member.take().unwrap_or_else(Member::default)
    }
    pub fn clear_start_index(&mut self) {
        self.start_index = 0
    }
    pub fn set_start_index(&mut self, v: u64) {
        self.start_index = v;
    }
    pub fn get_start_index(&self) -> u64 {
        self.start_index
    }
}
impl ::protobuf::Clear for SyncRegionRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SyncRegionRequest {
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
    fn default_instance() -> &'static SyncRegionRequest {
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
impl SyncRegionResponse {
    pub fn new_() -> SyncRegionResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <ResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    pub fn clear_regions(&mut self) {
        self.regions.clear();
    }
    pub fn set_regions(&mut self, v: ::std::vec::Vec<super::metapb::Region>) {
        self.regions = v;
    }
    pub fn get_regions(&self) -> &::std::vec::Vec<super::metapb::Region> {
        &self.regions
    }
    pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<super::metapb::Region> {
        &mut self.regions
    }
    pub fn take_regions(&mut self) -> ::std::vec::Vec<super::metapb::Region> {
        ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new())
    }
    pub fn clear_start_index(&mut self) {
        self.start_index = 0
    }
    pub fn set_start_index(&mut self, v: u64) {
        self.start_index = v;
    }
    pub fn get_start_index(&self) -> u64 {
        self.start_index
    }
}
impl ::protobuf::Clear for SyncRegionResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SyncRegionResponse {
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
    fn default_instance() -> &'static SyncRegionResponse {
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
