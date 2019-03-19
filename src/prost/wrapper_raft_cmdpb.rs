impl GetRequest {
    pub fn new_() -> GetRequest {
        ::std::default::Default::default()
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
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
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
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
impl PutRequest {
    pub fn new_() -> PutRequest {
        ::std::default::Default::default()
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for PutRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PutRequest {
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
    fn default_instance() -> &'static PutRequest {
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
impl PutResponse {
    pub fn new_() -> PutResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for PutResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PutResponse {
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
    fn default_instance() -> &'static PutResponse {
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
impl DeleteRequest {
    pub fn new_() -> DeleteRequest {
        ::std::default::Default::default()
    }
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for DeleteRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for DeleteRequest {
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
    fn default_instance() -> &'static DeleteRequest {
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
impl DeleteResponse {
    pub fn new_() -> DeleteResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for DeleteResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for DeleteResponse {
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
    fn default_instance() -> &'static DeleteResponse {
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
    pub fn clear_cf(&mut self) {
        self.cf.clear();
    }
    pub fn set_cf(&mut self, v: std::string::String) {
        self.cf = v;
    }
    pub fn get_cf(&self) -> &str {
        &self.cf
    }
    pub fn mut_cf(&mut self) -> &mut std::string::String {
        &mut self.cf
    }
    pub fn take_cf(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf, ::std::string::String::new())
    }
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: std::vec::Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: std::vec::Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> std::vec::Vec<u8> {
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
impl SnapRequest {
    pub fn new_() -> SnapRequest {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for SnapRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SnapRequest {
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
    fn default_instance() -> &'static SnapRequest {
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
impl SnapResponse {
    pub fn new_() -> SnapResponse {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for SnapResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SnapResponse {
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
    fn default_instance() -> &'static SnapResponse {
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
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: std::vec::Vec<u8>) {
        self.key = v;
    }
    pub fn get_key(&self) -> &[u8] {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.key
    }
    pub fn take_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.key, ::std::vec::Vec::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: std::vec::Vec<u8>) {
        self.value = v;
    }
    pub fn get_value(&self) -> &[u8] {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.value
    }
    pub fn take_value(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.value, ::std::vec::Vec::new())
    }
    pub fn clear_lock(&mut self) {
        self.lock.clear();
    }
    pub fn set_lock(&mut self, v: std::vec::Vec<u8>) {
        self.lock = v;
    }
    pub fn get_lock(&self) -> &[u8] {
        &self.lock
    }
    pub fn mut_lock(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.lock
    }
    pub fn take_lock(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.lock, ::std::vec::Vec::new())
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
impl IngestSstRequest {
    pub fn new_() -> IngestSstRequest {
        ::std::default::Default::default()
    }
    pub fn has_sst(&self) -> bool {
        self.sst.is_some()
    }
    pub fn clear_sst(&mut self) {
        self.sst = ::std::option::Option::None
    }
    pub fn set_sst(&mut self, v: super::import_sstpb::SstMeta) {
        self.sst = ::std::option::Option::Some(v);;    }
    pub fn get_sst(&self) -> &super::import_sstpb::SstMeta {
        match self.sst.as_ref() {
            Some(v) => v,
            None => <super::import_sstpb::SstMeta as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_sst(&mut self) -> &mut super::import_sstpb::SstMeta {
        if self.sst.is_none() {
            self.sst = ::std::option::Option::Some(super::import_sstpb::SstMeta::default());
        }
        self.sst.as_mut().unwrap()
    }
    pub fn take_sst(&mut self) -> super::import_sstpb::SstMeta {
        self.sst
            .take()
            .unwrap_or_else(super::import_sstpb::SstMeta::default)
    }
}
impl ::protobuf::Clear for IngestSstRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for IngestSstRequest {
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
    fn default_instance() -> &'static IngestSstRequest {
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
impl IngestSstResponse {
    pub fn new_() -> IngestSstResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for IngestSstResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for IngestSstResponse {
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
    fn default_instance() -> &'static IngestSstResponse {
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
impl Request {
    pub fn new_() -> Request {
        ::std::default::Default::default()
    }
    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = 0
    }
    pub fn set_cmd_type_(&mut self, v: CmdType) {
        self.cmd_type = unsafe { ::std::mem::transmute::<CmdType, i32>(v) };
    }
    pub fn get_cmd_type(&self) -> CmdType {
        unsafe { ::std::mem::transmute::<i32, CmdType>(self.cmd_type) }
    }
    pub fn has_get(&self) -> bool {
        self.get.is_some()
    }
    pub fn clear_get(&mut self) {
        self.get = ::std::option::Option::None
    }
    pub fn set_get(&mut self, v: GetRequest) {
        self.get = ::std::option::Option::Some(v);;    }
    pub fn get_get(&self) -> &GetRequest {
        match self.get.as_ref() {
            Some(v) => v,
            None => <GetRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_get(&mut self) -> &mut GetRequest {
        if self.get.is_none() {
            self.get = ::std::option::Option::Some(GetRequest::default());
        }
        self.get.as_mut().unwrap()
    }
    pub fn take_get(&mut self) -> GetRequest {
        self.get.take().unwrap_or_else(GetRequest::default)
    }
    pub fn has_put(&self) -> bool {
        self.put.is_some()
    }
    pub fn clear_put(&mut self) {
        self.put = ::std::option::Option::None
    }
    pub fn set_put(&mut self, v: PutRequest) {
        self.put = ::std::option::Option::Some(v);;    }
    pub fn get_put(&self) -> &PutRequest {
        match self.put.as_ref() {
            Some(v) => v,
            None => <PutRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_put(&mut self) -> &mut PutRequest {
        if self.put.is_none() {
            self.put = ::std::option::Option::Some(PutRequest::default());
        }
        self.put.as_mut().unwrap()
    }
    pub fn take_put(&mut self) -> PutRequest {
        self.put.take().unwrap_or_else(PutRequest::default)
    }
    pub fn has_delete(&self) -> bool {
        self.delete.is_some()
    }
    pub fn clear_delete(&mut self) {
        self.delete = ::std::option::Option::None
    }
    pub fn set_delete(&mut self, v: DeleteRequest) {
        self.delete = ::std::option::Option::Some(v);;    }
    pub fn get_delete(&self) -> &DeleteRequest {
        match self.delete.as_ref() {
            Some(v) => v,
            None => <DeleteRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_delete(&mut self) -> &mut DeleteRequest {
        if self.delete.is_none() {
            self.delete = ::std::option::Option::Some(DeleteRequest::default());
        }
        self.delete.as_mut().unwrap()
    }
    pub fn take_delete(&mut self) -> DeleteRequest {
        self.delete.take().unwrap_or_else(DeleteRequest::default)
    }
    pub fn has_snap(&self) -> bool {
        self.snap.is_some()
    }
    pub fn clear_snap(&mut self) {
        self.snap = ::std::option::Option::None
    }
    pub fn set_snap(&mut self, v: SnapRequest) {
        self.snap = ::std::option::Option::Some(v);;    }
    pub fn get_snap(&self) -> &SnapRequest {
        match self.snap.as_ref() {
            Some(v) => v,
            None => <SnapRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_snap(&mut self) -> &mut SnapRequest {
        if self.snap.is_none() {
            self.snap = ::std::option::Option::Some(SnapRequest::default());
        }
        self.snap.as_mut().unwrap()
    }
    pub fn take_snap(&mut self) -> SnapRequest {
        self.snap.take().unwrap_or_else(SnapRequest::default)
    }
    pub fn has_prewrite(&self) -> bool {
        self.prewrite.is_some()
    }
    pub fn clear_prewrite(&mut self) {
        self.prewrite = ::std::option::Option::None
    }
    pub fn set_prewrite(&mut self, v: PrewriteRequest) {
        self.prewrite = ::std::option::Option::Some(v);;    }
    pub fn get_prewrite(&self) -> &PrewriteRequest {
        match self.prewrite.as_ref() {
            Some(v) => v,
            None => <PrewriteRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_prewrite(&mut self) -> &mut PrewriteRequest {
        if self.prewrite.is_none() {
            self.prewrite = ::std::option::Option::Some(PrewriteRequest::default());
        }
        self.prewrite.as_mut().unwrap()
    }
    pub fn take_prewrite(&mut self) -> PrewriteRequest {
        self.prewrite
            .take()
            .unwrap_or_else(PrewriteRequest::default)
    }
    pub fn has_delete_range(&self) -> bool {
        self.delete_range.is_some()
    }
    pub fn clear_delete_range(&mut self) {
        self.delete_range = ::std::option::Option::None
    }
    pub fn set_delete_range(&mut self, v: DeleteRangeRequest) {
        self.delete_range = ::std::option::Option::Some(v);;    }
    pub fn get_delete_range(&self) -> &DeleteRangeRequest {
        match self.delete_range.as_ref() {
            Some(v) => v,
            None => <DeleteRangeRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_delete_range(&mut self) -> &mut DeleteRangeRequest {
        if self.delete_range.is_none() {
            self.delete_range = ::std::option::Option::Some(DeleteRangeRequest::default());
        }
        self.delete_range.as_mut().unwrap()
    }
    pub fn take_delete_range(&mut self) -> DeleteRangeRequest {
        self.delete_range
            .take()
            .unwrap_or_else(DeleteRangeRequest::default)
    }
    pub fn has_ingest_sst(&self) -> bool {
        self.ingest_sst.is_some()
    }
    pub fn clear_ingest_sst(&mut self) {
        self.ingest_sst = ::std::option::Option::None
    }
    pub fn set_ingest_sst(&mut self, v: IngestSstRequest) {
        self.ingest_sst = ::std::option::Option::Some(v);;    }
    pub fn get_ingest_sst(&self) -> &IngestSstRequest {
        match self.ingest_sst.as_ref() {
            Some(v) => v,
            None => <IngestSstRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_ingest_sst(&mut self) -> &mut IngestSstRequest {
        if self.ingest_sst.is_none() {
            self.ingest_sst = ::std::option::Option::Some(IngestSstRequest::default());
        }
        self.ingest_sst.as_mut().unwrap()
    }
    pub fn take_ingest_sst(&mut self) -> IngestSstRequest {
        self.ingest_sst
            .take()
            .unwrap_or_else(IngestSstRequest::default)
    }
}
impl ::protobuf::Clear for Request {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Request {
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
    fn default_instance() -> &'static Request {
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
impl Response {
    pub fn new_() -> Response {
        ::std::default::Default::default()
    }
    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = 0
    }
    pub fn set_cmd_type_(&mut self, v: CmdType) {
        self.cmd_type = unsafe { ::std::mem::transmute::<CmdType, i32>(v) };
    }
    pub fn get_cmd_type(&self) -> CmdType {
        unsafe { ::std::mem::transmute::<i32, CmdType>(self.cmd_type) }
    }
    pub fn has_get(&self) -> bool {
        self.get.is_some()
    }
    pub fn clear_get(&mut self) {
        self.get = ::std::option::Option::None
    }
    pub fn set_get(&mut self, v: GetResponse) {
        self.get = ::std::option::Option::Some(v);;    }
    pub fn get_get(&self) -> &GetResponse {
        match self.get.as_ref() {
            Some(v) => v,
            None => <GetResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_get(&mut self) -> &mut GetResponse {
        if self.get.is_none() {
            self.get = ::std::option::Option::Some(GetResponse::default());
        }
        self.get.as_mut().unwrap()
    }
    pub fn take_get(&mut self) -> GetResponse {
        self.get.take().unwrap_or_else(GetResponse::default)
    }
    pub fn has_put(&self) -> bool {
        self.put.is_some()
    }
    pub fn clear_put(&mut self) {
        self.put = ::std::option::Option::None
    }
    pub fn set_put(&mut self, v: PutResponse) {
        self.put = ::std::option::Option::Some(v);;    }
    pub fn get_put(&self) -> &PutResponse {
        match self.put.as_ref() {
            Some(v) => v,
            None => <PutResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_put(&mut self) -> &mut PutResponse {
        if self.put.is_none() {
            self.put = ::std::option::Option::Some(PutResponse::default());
        }
        self.put.as_mut().unwrap()
    }
    pub fn take_put(&mut self) -> PutResponse {
        self.put.take().unwrap_or_else(PutResponse::default)
    }
    pub fn has_delete(&self) -> bool {
        self.delete.is_some()
    }
    pub fn clear_delete(&mut self) {
        self.delete = ::std::option::Option::None
    }
    pub fn set_delete(&mut self, v: DeleteResponse) {
        self.delete = ::std::option::Option::Some(v);;    }
    pub fn get_delete(&self) -> &DeleteResponse {
        match self.delete.as_ref() {
            Some(v) => v,
            None => <DeleteResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_delete(&mut self) -> &mut DeleteResponse {
        if self.delete.is_none() {
            self.delete = ::std::option::Option::Some(DeleteResponse::default());
        }
        self.delete.as_mut().unwrap()
    }
    pub fn take_delete(&mut self) -> DeleteResponse {
        self.delete.take().unwrap_or_else(DeleteResponse::default)
    }
    pub fn has_snap(&self) -> bool {
        self.snap.is_some()
    }
    pub fn clear_snap(&mut self) {
        self.snap = ::std::option::Option::None
    }
    pub fn set_snap(&mut self, v: SnapResponse) {
        self.snap = ::std::option::Option::Some(v);;    }
    pub fn get_snap(&self) -> &SnapResponse {
        match self.snap.as_ref() {
            Some(v) => v,
            None => <SnapResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_snap(&mut self) -> &mut SnapResponse {
        if self.snap.is_none() {
            self.snap = ::std::option::Option::Some(SnapResponse::default());
        }
        self.snap.as_mut().unwrap()
    }
    pub fn take_snap(&mut self) -> SnapResponse {
        self.snap.take().unwrap_or_else(SnapResponse::default)
    }
    pub fn has_prewrite(&self) -> bool {
        self.prewrite.is_some()
    }
    pub fn clear_prewrite(&mut self) {
        self.prewrite = ::std::option::Option::None
    }
    pub fn set_prewrite(&mut self, v: PrewriteResponse) {
        self.prewrite = ::std::option::Option::Some(v);;    }
    pub fn get_prewrite(&self) -> &PrewriteResponse {
        match self.prewrite.as_ref() {
            Some(v) => v,
            None => <PrewriteResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_prewrite(&mut self) -> &mut PrewriteResponse {
        if self.prewrite.is_none() {
            self.prewrite = ::std::option::Option::Some(PrewriteResponse::default());
        }
        self.prewrite.as_mut().unwrap()
    }
    pub fn take_prewrite(&mut self) -> PrewriteResponse {
        self.prewrite
            .take()
            .unwrap_or_else(PrewriteResponse::default)
    }
    pub fn has_delte_range(&self) -> bool {
        self.delte_range.is_some()
    }
    pub fn clear_delte_range(&mut self) {
        self.delte_range = ::std::option::Option::None
    }
    pub fn set_delte_range(&mut self, v: DeleteRangeResponse) {
        self.delte_range = ::std::option::Option::Some(v);;    }
    pub fn get_delte_range(&self) -> &DeleteRangeResponse {
        match self.delte_range.as_ref() {
            Some(v) => v,
            None => <DeleteRangeResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_delte_range(&mut self) -> &mut DeleteRangeResponse {
        if self.delte_range.is_none() {
            self.delte_range = ::std::option::Option::Some(DeleteRangeResponse::default());
        }
        self.delte_range.as_mut().unwrap()
    }
    pub fn take_delte_range(&mut self) -> DeleteRangeResponse {
        self.delte_range
            .take()
            .unwrap_or_else(DeleteRangeResponse::default)
    }
    pub fn has_ingest_sst(&self) -> bool {
        self.ingest_sst.is_some()
    }
    pub fn clear_ingest_sst(&mut self) {
        self.ingest_sst = ::std::option::Option::None
    }
    pub fn set_ingest_sst(&mut self, v: IngestSstResponse) {
        self.ingest_sst = ::std::option::Option::Some(v);;    }
    pub fn get_ingest_sst(&self) -> &IngestSstResponse {
        match self.ingest_sst.as_ref() {
            Some(v) => v,
            None => <IngestSstResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_ingest_sst(&mut self) -> &mut IngestSstResponse {
        if self.ingest_sst.is_none() {
            self.ingest_sst = ::std::option::Option::Some(IngestSstResponse::default());
        }
        self.ingest_sst.as_mut().unwrap()
    }
    pub fn take_ingest_sst(&mut self) -> IngestSstResponse {
        self.ingest_sst
            .take()
            .unwrap_or_else(IngestSstResponse::default)
    }
}
impl ::protobuf::Clear for Response {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Response {
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
    fn default_instance() -> &'static Response {
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
impl ChangePeerRequest {
    pub fn new_() -> ChangePeerRequest {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for ChangePeerRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ChangePeerRequest {
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
    fn default_instance() -> &'static ChangePeerRequest {
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
impl ChangePeerResponse {
    pub fn new_() -> ChangePeerResponse {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for ChangePeerResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ChangePeerResponse {
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
    fn default_instance() -> &'static ChangePeerResponse {
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
impl SplitRequest {
    pub fn new_() -> SplitRequest {
        ::std::default::Default::default()
    }
    pub fn clear_split_key(&mut self) {
        self.split_key.clear();
    }
    pub fn set_split_key(&mut self, v: std::vec::Vec<u8>) {
        self.split_key = v;
    }
    pub fn get_split_key(&self) -> &[u8] {
        &self.split_key
    }
    pub fn mut_split_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.split_key
    }
    pub fn take_split_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.split_key, ::std::vec::Vec::new())
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
    pub fn clear_right_derive(&mut self) {
        self.right_derive = false
    }
    pub fn set_right_derive(&mut self, v: bool) {
        self.right_derive = v;
    }
    pub fn get_right_derive(&self) -> bool {
        self.right_derive
    }
}
impl ::protobuf::Clear for SplitRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SplitRequest {
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
    fn default_instance() -> &'static SplitRequest {
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
impl SplitResponse {
    pub fn new_() -> SplitResponse {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for SplitResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SplitResponse {
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
    fn default_instance() -> &'static SplitResponse {
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
impl BatchSplitRequest {
    pub fn new_() -> BatchSplitRequest {
        ::std::default::Default::default()
    }
    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }
    pub fn set_requests(&mut self, v: ::std::vec::Vec<SplitRequest>) {
        self.requests = v;
    }
    pub fn get_requests(&self) -> &::std::vec::Vec<SplitRequest> {
        &self.requests
    }
    pub fn mut_requests(&mut self) -> &mut ::std::vec::Vec<SplitRequest> {
        &mut self.requests
    }
    pub fn take_requests(&mut self) -> ::std::vec::Vec<SplitRequest> {
        ::std::mem::replace(&mut self.requests, ::std::vec::Vec::new())
    }
    pub fn clear_right_derive(&mut self) {
        self.right_derive = false
    }
    pub fn set_right_derive(&mut self, v: bool) {
        self.right_derive = v;
    }
    pub fn get_right_derive(&self) -> bool {
        self.right_derive
    }
}
impl ::protobuf::Clear for BatchSplitRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchSplitRequest {
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
    fn default_instance() -> &'static BatchSplitRequest {
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
impl BatchSplitResponse {
    pub fn new_() -> BatchSplitResponse {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for BatchSplitResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for BatchSplitResponse {
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
    fn default_instance() -> &'static BatchSplitResponse {
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
impl CompactLogRequest {
    pub fn new_() -> CompactLogRequest {
        ::std::default::Default::default()
    }
    pub fn clear_compact_index(&mut self) {
        self.compact_index = 0
    }
    pub fn set_compact_index(&mut self, v: u64) {
        self.compact_index = v;
    }
    pub fn get_compact_index(&self) -> u64 {
        self.compact_index
    }
    pub fn clear_compact_term(&mut self) {
        self.compact_term = 0
    }
    pub fn set_compact_term(&mut self, v: u64) {
        self.compact_term = v;
    }
    pub fn get_compact_term(&self) -> u64 {
        self.compact_term
    }
}
impl ::protobuf::Clear for CompactLogRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CompactLogRequest {
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
    fn default_instance() -> &'static CompactLogRequest {
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
impl CompactLogResponse {
    pub fn new_() -> CompactLogResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for CompactLogResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CompactLogResponse {
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
    fn default_instance() -> &'static CompactLogResponse {
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
impl TransferLeaderRequest {
    pub fn new_() -> TransferLeaderRequest {
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
impl ::protobuf::Clear for TransferLeaderRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TransferLeaderRequest {
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
    fn default_instance() -> &'static TransferLeaderRequest {
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
impl TransferLeaderResponse {
    pub fn new_() -> TransferLeaderResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for TransferLeaderResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for TransferLeaderResponse {
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
    fn default_instance() -> &'static TransferLeaderResponse {
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
impl VerifyHashRequest {
    pub fn new_() -> VerifyHashRequest {
        ::std::default::Default::default()
    }
    pub fn clear_index(&mut self) {
        self.index = 0
    }
    pub fn set_index(&mut self, v: u64) {
        self.index = v;
    }
    pub fn get_index(&self) -> u64 {
        self.index
    }
    pub fn clear_hash(&mut self) {
        self.hash.clear();
    }
    pub fn set_hash(&mut self, v: std::vec::Vec<u8>) {
        self.hash = v;
    }
    pub fn get_hash(&self) -> &[u8] {
        &self.hash
    }
    pub fn mut_hash(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.hash
    }
    pub fn take_hash(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.hash, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for VerifyHashRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for VerifyHashRequest {
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
    fn default_instance() -> &'static VerifyHashRequest {
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
impl VerifyHashResponse {
    pub fn new_() -> VerifyHashResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for VerifyHashResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for VerifyHashResponse {
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
    fn default_instance() -> &'static VerifyHashResponse {
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
impl PrepareMergeRequest {
    pub fn new_() -> PrepareMergeRequest {
        ::std::default::Default::default()
    }
    pub fn clear_min_index(&mut self) {
        self.min_index = 0
    }
    pub fn set_min_index(&mut self, v: u64) {
        self.min_index = v;
    }
    pub fn get_min_index(&self) -> u64 {
        self.min_index
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
impl ::protobuf::Clear for PrepareMergeRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PrepareMergeRequest {
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
    fn default_instance() -> &'static PrepareMergeRequest {
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
impl PrepareMergeResponse {
    pub fn new_() -> PrepareMergeResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for PrepareMergeResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for PrepareMergeResponse {
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
    fn default_instance() -> &'static PrepareMergeResponse {
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
impl CommitMergeRequest {
    pub fn new_() -> CommitMergeRequest {
        ::std::default::Default::default()
    }
    pub fn has_source(&self) -> bool {
        self.source.is_some()
    }
    pub fn clear_source(&mut self) {
        self.source = ::std::option::Option::None
    }
    pub fn set_source(&mut self, v: super::metapb::Region) {
        self.source = ::std::option::Option::Some(v);;    }
    pub fn get_source(&self) -> &super::metapb::Region {
        match self.source.as_ref() {
            Some(v) => v,
            None => <super::metapb::Region as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_source(&mut self) -> &mut super::metapb::Region {
        if self.source.is_none() {
            self.source = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.source.as_mut().unwrap()
    }
    pub fn take_source(&mut self) -> super::metapb::Region {
        self.source
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
    pub fn clear_commit(&mut self) {
        self.commit = 0
    }
    pub fn set_commit(&mut self, v: u64) {
        self.commit = v;
    }
    pub fn get_commit(&self) -> u64 {
        self.commit
    }
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }
    pub fn set_entries(&mut self, v: ::std::vec::Vec<super::eraftpb::Entry>) {
        self.entries = v;
    }
    pub fn get_entries(&self) -> &::std::vec::Vec<super::eraftpb::Entry> {
        &self.entries
    }
    pub fn mut_entries(&mut self) -> &mut ::std::vec::Vec<super::eraftpb::Entry> {
        &mut self.entries
    }
    pub fn take_entries(&mut self) -> ::std::vec::Vec<super::eraftpb::Entry> {
        ::std::mem::replace(&mut self.entries, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for CommitMergeRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommitMergeRequest {
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
    fn default_instance() -> &'static CommitMergeRequest {
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
impl CommitMergeResponse {
    pub fn new_() -> CommitMergeResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for CommitMergeResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CommitMergeResponse {
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
    fn default_instance() -> &'static CommitMergeResponse {
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
impl RollbackMergeRequest {
    pub fn new_() -> RollbackMergeRequest {
        ::std::default::Default::default()
    }
    pub fn clear_commit(&mut self) {
        self.commit = 0
    }
    pub fn set_commit(&mut self, v: u64) {
        self.commit = v;
    }
    pub fn get_commit(&self) -> u64 {
        self.commit
    }
}
impl ::protobuf::Clear for RollbackMergeRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RollbackMergeRequest {
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
    fn default_instance() -> &'static RollbackMergeRequest {
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
impl RollbackMergeResponse {
    pub fn new_() -> RollbackMergeResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for RollbackMergeResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RollbackMergeResponse {
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
    fn default_instance() -> &'static RollbackMergeResponse {
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
impl AdminRequest {
    pub fn new_() -> AdminRequest {
        ::std::default::Default::default()
    }
    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = 0
    }
    pub fn set_cmd_type_(&mut self, v: AdminCmdType) {
        self.cmd_type = unsafe { ::std::mem::transmute::<AdminCmdType, i32>(v) };
    }
    pub fn get_cmd_type(&self) -> AdminCmdType {
        unsafe { ::std::mem::transmute::<i32, AdminCmdType>(self.cmd_type) }
    }
    pub fn has_change_peer(&self) -> bool {
        self.change_peer.is_some()
    }
    pub fn clear_change_peer(&mut self) {
        self.change_peer = ::std::option::Option::None
    }
    pub fn set_change_peer(&mut self, v: ChangePeerRequest) {
        self.change_peer = ::std::option::Option::Some(v);;    }
    pub fn get_change_peer(&self) -> &ChangePeerRequest {
        match self.change_peer.as_ref() {
            Some(v) => v,
            None => <ChangePeerRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_change_peer(&mut self) -> &mut ChangePeerRequest {
        if self.change_peer.is_none() {
            self.change_peer = ::std::option::Option::Some(ChangePeerRequest::default());
        }
        self.change_peer.as_mut().unwrap()
    }
    pub fn take_change_peer(&mut self) -> ChangePeerRequest {
        self.change_peer
            .take()
            .unwrap_or_else(ChangePeerRequest::default)
    }
    pub fn has_split(&self) -> bool {
        self.split.is_some()
    }
    pub fn clear_split(&mut self) {
        self.split = ::std::option::Option::None
    }
    pub fn set_split(&mut self, v: SplitRequest) {
        self.split = ::std::option::Option::Some(v);;    }
    pub fn get_split(&self) -> &SplitRequest {
        match self.split.as_ref() {
            Some(v) => v,
            None => <SplitRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_split(&mut self) -> &mut SplitRequest {
        if self.split.is_none() {
            self.split = ::std::option::Option::Some(SplitRequest::default());
        }
        self.split.as_mut().unwrap()
    }
    pub fn take_split(&mut self) -> SplitRequest {
        self.split.take().unwrap_or_else(SplitRequest::default)
    }
    pub fn has_compact_log(&self) -> bool {
        self.compact_log.is_some()
    }
    pub fn clear_compact_log(&mut self) {
        self.compact_log = ::std::option::Option::None
    }
    pub fn set_compact_log(&mut self, v: CompactLogRequest) {
        self.compact_log = ::std::option::Option::Some(v);;    }
    pub fn get_compact_log(&self) -> &CompactLogRequest {
        match self.compact_log.as_ref() {
            Some(v) => v,
            None => <CompactLogRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_compact_log(&mut self) -> &mut CompactLogRequest {
        if self.compact_log.is_none() {
            self.compact_log = ::std::option::Option::Some(CompactLogRequest::default());
        }
        self.compact_log.as_mut().unwrap()
    }
    pub fn take_compact_log(&mut self) -> CompactLogRequest {
        self.compact_log
            .take()
            .unwrap_or_else(CompactLogRequest::default)
    }
    pub fn has_transfer_leader(&self) -> bool {
        self.transfer_leader.is_some()
    }
    pub fn clear_transfer_leader(&mut self) {
        self.transfer_leader = ::std::option::Option::None
    }
    pub fn set_transfer_leader(&mut self, v: TransferLeaderRequest) {
        self.transfer_leader = ::std::option::Option::Some(v);;    }
    pub fn get_transfer_leader(&self) -> &TransferLeaderRequest {
        match self.transfer_leader.as_ref() {
            Some(v) => v,
            None => <TransferLeaderRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_transfer_leader(&mut self) -> &mut TransferLeaderRequest {
        if self.transfer_leader.is_none() {
            self.transfer_leader = ::std::option::Option::Some(TransferLeaderRequest::default());
        }
        self.transfer_leader.as_mut().unwrap()
    }
    pub fn take_transfer_leader(&mut self) -> TransferLeaderRequest {
        self.transfer_leader
            .take()
            .unwrap_or_else(TransferLeaderRequest::default)
    }
    pub fn has_verify_hash(&self) -> bool {
        self.verify_hash.is_some()
    }
    pub fn clear_verify_hash(&mut self) {
        self.verify_hash = ::std::option::Option::None
    }
    pub fn set_verify_hash(&mut self, v: VerifyHashRequest) {
        self.verify_hash = ::std::option::Option::Some(v);;    }
    pub fn get_verify_hash(&self) -> &VerifyHashRequest {
        match self.verify_hash.as_ref() {
            Some(v) => v,
            None => <VerifyHashRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_verify_hash(&mut self) -> &mut VerifyHashRequest {
        if self.verify_hash.is_none() {
            self.verify_hash = ::std::option::Option::Some(VerifyHashRequest::default());
        }
        self.verify_hash.as_mut().unwrap()
    }
    pub fn take_verify_hash(&mut self) -> VerifyHashRequest {
        self.verify_hash
            .take()
            .unwrap_or_else(VerifyHashRequest::default)
    }
    pub fn has_prepare_merge(&self) -> bool {
        self.prepare_merge.is_some()
    }
    pub fn clear_prepare_merge(&mut self) {
        self.prepare_merge = ::std::option::Option::None
    }
    pub fn set_prepare_merge(&mut self, v: PrepareMergeRequest) {
        self.prepare_merge = ::std::option::Option::Some(v);;    }
    pub fn get_prepare_merge(&self) -> &PrepareMergeRequest {
        match self.prepare_merge.as_ref() {
            Some(v) => v,
            None => <PrepareMergeRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_prepare_merge(&mut self) -> &mut PrepareMergeRequest {
        if self.prepare_merge.is_none() {
            self.prepare_merge = ::std::option::Option::Some(PrepareMergeRequest::default());
        }
        self.prepare_merge.as_mut().unwrap()
    }
    pub fn take_prepare_merge(&mut self) -> PrepareMergeRequest {
        self.prepare_merge
            .take()
            .unwrap_or_else(PrepareMergeRequest::default)
    }
    pub fn has_commit_merge(&self) -> bool {
        self.commit_merge.is_some()
    }
    pub fn clear_commit_merge(&mut self) {
        self.commit_merge = ::std::option::Option::None
    }
    pub fn set_commit_merge(&mut self, v: CommitMergeRequest) {
        self.commit_merge = ::std::option::Option::Some(v);;    }
    pub fn get_commit_merge(&self) -> &CommitMergeRequest {
        match self.commit_merge.as_ref() {
            Some(v) => v,
            None => <CommitMergeRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_commit_merge(&mut self) -> &mut CommitMergeRequest {
        if self.commit_merge.is_none() {
            self.commit_merge = ::std::option::Option::Some(CommitMergeRequest::default());
        }
        self.commit_merge.as_mut().unwrap()
    }
    pub fn take_commit_merge(&mut self) -> CommitMergeRequest {
        self.commit_merge
            .take()
            .unwrap_or_else(CommitMergeRequest::default)
    }
    pub fn has_rollback_merge(&self) -> bool {
        self.rollback_merge.is_some()
    }
    pub fn clear_rollback_merge(&mut self) {
        self.rollback_merge = ::std::option::Option::None
    }
    pub fn set_rollback_merge(&mut self, v: RollbackMergeRequest) {
        self.rollback_merge = ::std::option::Option::Some(v);;    }
    pub fn get_rollback_merge(&self) -> &RollbackMergeRequest {
        match self.rollback_merge.as_ref() {
            Some(v) => v,
            None => <RollbackMergeRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_rollback_merge(&mut self) -> &mut RollbackMergeRequest {
        if self.rollback_merge.is_none() {
            self.rollback_merge = ::std::option::Option::Some(RollbackMergeRequest::default());
        }
        self.rollback_merge.as_mut().unwrap()
    }
    pub fn take_rollback_merge(&mut self) -> RollbackMergeRequest {
        self.rollback_merge
            .take()
            .unwrap_or_else(RollbackMergeRequest::default)
    }
    pub fn has_splits(&self) -> bool {
        self.splits.is_some()
    }
    pub fn clear_splits(&mut self) {
        self.splits = ::std::option::Option::None
    }
    pub fn set_splits(&mut self, v: BatchSplitRequest) {
        self.splits = ::std::option::Option::Some(v);;    }
    pub fn get_splits(&self) -> &BatchSplitRequest {
        match self.splits.as_ref() {
            Some(v) => v,
            None => <BatchSplitRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_splits(&mut self) -> &mut BatchSplitRequest {
        if self.splits.is_none() {
            self.splits = ::std::option::Option::Some(BatchSplitRequest::default());
        }
        self.splits.as_mut().unwrap()
    }
    pub fn take_splits(&mut self) -> BatchSplitRequest {
        self.splits
            .take()
            .unwrap_or_else(BatchSplitRequest::default)
    }
}
impl ::protobuf::Clear for AdminRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AdminRequest {
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
    fn default_instance() -> &'static AdminRequest {
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
impl AdminResponse {
    pub fn new_() -> AdminResponse {
        ::std::default::Default::default()
    }
    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = 0
    }
    pub fn set_cmd_type_(&mut self, v: AdminCmdType) {
        self.cmd_type = unsafe { ::std::mem::transmute::<AdminCmdType, i32>(v) };
    }
    pub fn get_cmd_type(&self) -> AdminCmdType {
        unsafe { ::std::mem::transmute::<i32, AdminCmdType>(self.cmd_type) }
    }
    pub fn has_change_peer(&self) -> bool {
        self.change_peer.is_some()
    }
    pub fn clear_change_peer(&mut self) {
        self.change_peer = ::std::option::Option::None
    }
    pub fn set_change_peer(&mut self, v: ChangePeerResponse) {
        self.change_peer = ::std::option::Option::Some(v);;    }
    pub fn get_change_peer(&self) -> &ChangePeerResponse {
        match self.change_peer.as_ref() {
            Some(v) => v,
            None => <ChangePeerResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_change_peer(&mut self) -> &mut ChangePeerResponse {
        if self.change_peer.is_none() {
            self.change_peer = ::std::option::Option::Some(ChangePeerResponse::default());
        }
        self.change_peer.as_mut().unwrap()
    }
    pub fn take_change_peer(&mut self) -> ChangePeerResponse {
        self.change_peer
            .take()
            .unwrap_or_else(ChangePeerResponse::default)
    }
    pub fn has_split(&self) -> bool {
        self.split.is_some()
    }
    pub fn clear_split(&mut self) {
        self.split = ::std::option::Option::None
    }
    pub fn set_split(&mut self, v: SplitResponse) {
        self.split = ::std::option::Option::Some(v);;    }
    pub fn get_split(&self) -> &SplitResponse {
        match self.split.as_ref() {
            Some(v) => v,
            None => <SplitResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_split(&mut self) -> &mut SplitResponse {
        if self.split.is_none() {
            self.split = ::std::option::Option::Some(SplitResponse::default());
        }
        self.split.as_mut().unwrap()
    }
    pub fn take_split(&mut self) -> SplitResponse {
        self.split.take().unwrap_or_else(SplitResponse::default)
    }
    pub fn has_compact_log(&self) -> bool {
        self.compact_log.is_some()
    }
    pub fn clear_compact_log(&mut self) {
        self.compact_log = ::std::option::Option::None
    }
    pub fn set_compact_log(&mut self, v: CompactLogResponse) {
        self.compact_log = ::std::option::Option::Some(v);;    }
    pub fn get_compact_log(&self) -> &CompactLogResponse {
        match self.compact_log.as_ref() {
            Some(v) => v,
            None => <CompactLogResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_compact_log(&mut self) -> &mut CompactLogResponse {
        if self.compact_log.is_none() {
            self.compact_log = ::std::option::Option::Some(CompactLogResponse::default());
        }
        self.compact_log.as_mut().unwrap()
    }
    pub fn take_compact_log(&mut self) -> CompactLogResponse {
        self.compact_log
            .take()
            .unwrap_or_else(CompactLogResponse::default)
    }
    pub fn has_transfer_leader(&self) -> bool {
        self.transfer_leader.is_some()
    }
    pub fn clear_transfer_leader(&mut self) {
        self.transfer_leader = ::std::option::Option::None
    }
    pub fn set_transfer_leader(&mut self, v: TransferLeaderResponse) {
        self.transfer_leader = ::std::option::Option::Some(v);;    }
    pub fn get_transfer_leader(&self) -> &TransferLeaderResponse {
        match self.transfer_leader.as_ref() {
            Some(v) => v,
            None => <TransferLeaderResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_transfer_leader(&mut self) -> &mut TransferLeaderResponse {
        if self.transfer_leader.is_none() {
            self.transfer_leader = ::std::option::Option::Some(TransferLeaderResponse::default());
        }
        self.transfer_leader.as_mut().unwrap()
    }
    pub fn take_transfer_leader(&mut self) -> TransferLeaderResponse {
        self.transfer_leader
            .take()
            .unwrap_or_else(TransferLeaderResponse::default)
    }
    pub fn has_verify_hash(&self) -> bool {
        self.verify_hash.is_some()
    }
    pub fn clear_verify_hash(&mut self) {
        self.verify_hash = ::std::option::Option::None
    }
    pub fn set_verify_hash(&mut self, v: VerifyHashResponse) {
        self.verify_hash = ::std::option::Option::Some(v);;    }
    pub fn get_verify_hash(&self) -> &VerifyHashResponse {
        match self.verify_hash.as_ref() {
            Some(v) => v,
            None => <VerifyHashResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_verify_hash(&mut self) -> &mut VerifyHashResponse {
        if self.verify_hash.is_none() {
            self.verify_hash = ::std::option::Option::Some(VerifyHashResponse::default());
        }
        self.verify_hash.as_mut().unwrap()
    }
    pub fn take_verify_hash(&mut self) -> VerifyHashResponse {
        self.verify_hash
            .take()
            .unwrap_or_else(VerifyHashResponse::default)
    }
    pub fn has_prepare_merge(&self) -> bool {
        self.prepare_merge.is_some()
    }
    pub fn clear_prepare_merge(&mut self) {
        self.prepare_merge = ::std::option::Option::None
    }
    pub fn set_prepare_merge(&mut self, v: PrepareMergeResponse) {
        self.prepare_merge = ::std::option::Option::Some(v);;    }
    pub fn get_prepare_merge(&self) -> &PrepareMergeResponse {
        match self.prepare_merge.as_ref() {
            Some(v) => v,
            None => <PrepareMergeResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_prepare_merge(&mut self) -> &mut PrepareMergeResponse {
        if self.prepare_merge.is_none() {
            self.prepare_merge = ::std::option::Option::Some(PrepareMergeResponse::default());
        }
        self.prepare_merge.as_mut().unwrap()
    }
    pub fn take_prepare_merge(&mut self) -> PrepareMergeResponse {
        self.prepare_merge
            .take()
            .unwrap_or_else(PrepareMergeResponse::default)
    }
    pub fn has_commit_merge(&self) -> bool {
        self.commit_merge.is_some()
    }
    pub fn clear_commit_merge(&mut self) {
        self.commit_merge = ::std::option::Option::None
    }
    pub fn set_commit_merge(&mut self, v: CommitMergeResponse) {
        self.commit_merge = ::std::option::Option::Some(v);;    }
    pub fn get_commit_merge(&self) -> &CommitMergeResponse {
        match self.commit_merge.as_ref() {
            Some(v) => v,
            None => <CommitMergeResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_commit_merge(&mut self) -> &mut CommitMergeResponse {
        if self.commit_merge.is_none() {
            self.commit_merge = ::std::option::Option::Some(CommitMergeResponse::default());
        }
        self.commit_merge.as_mut().unwrap()
    }
    pub fn take_commit_merge(&mut self) -> CommitMergeResponse {
        self.commit_merge
            .take()
            .unwrap_or_else(CommitMergeResponse::default)
    }
    pub fn has_rollback_merge(&self) -> bool {
        self.rollback_merge.is_some()
    }
    pub fn clear_rollback_merge(&mut self) {
        self.rollback_merge = ::std::option::Option::None
    }
    pub fn set_rollback_merge(&mut self, v: RollbackMergeResponse) {
        self.rollback_merge = ::std::option::Option::Some(v);;    }
    pub fn get_rollback_merge(&self) -> &RollbackMergeResponse {
        match self.rollback_merge.as_ref() {
            Some(v) => v,
            None => <RollbackMergeResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_rollback_merge(&mut self) -> &mut RollbackMergeResponse {
        if self.rollback_merge.is_none() {
            self.rollback_merge = ::std::option::Option::Some(RollbackMergeResponse::default());
        }
        self.rollback_merge.as_mut().unwrap()
    }
    pub fn take_rollback_merge(&mut self) -> RollbackMergeResponse {
        self.rollback_merge
            .take()
            .unwrap_or_else(RollbackMergeResponse::default)
    }
    pub fn has_splits(&self) -> bool {
        self.splits.is_some()
    }
    pub fn clear_splits(&mut self) {
        self.splits = ::std::option::Option::None
    }
    pub fn set_splits(&mut self, v: BatchSplitResponse) {
        self.splits = ::std::option::Option::Some(v);;    }
    pub fn get_splits(&self) -> &BatchSplitResponse {
        match self.splits.as_ref() {
            Some(v) => v,
            None => <BatchSplitResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_splits(&mut self) -> &mut BatchSplitResponse {
        if self.splits.is_none() {
            self.splits = ::std::option::Option::Some(BatchSplitResponse::default());
        }
        self.splits.as_mut().unwrap()
    }
    pub fn take_splits(&mut self) -> BatchSplitResponse {
        self.splits
            .take()
            .unwrap_or_else(BatchSplitResponse::default)
    }
}
impl ::protobuf::Clear for AdminResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for AdminResponse {
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
    fn default_instance() -> &'static AdminResponse {
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
impl RegionLeaderRequest {
    pub fn new_() -> RegionLeaderRequest {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for RegionLeaderRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RegionLeaderRequest {
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
    fn default_instance() -> &'static RegionLeaderRequest {
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
impl RegionLeaderResponse {
    pub fn new_() -> RegionLeaderResponse {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for RegionLeaderResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RegionLeaderResponse {
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
    fn default_instance() -> &'static RegionLeaderResponse {
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
impl RegionDetailRequest {
    pub fn new_() -> RegionDetailRequest {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for RegionDetailRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RegionDetailRequest {
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
    fn default_instance() -> &'static RegionDetailRequest {
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
impl RegionDetailResponse {
    pub fn new_() -> RegionDetailResponse {
        ::std::default::Default::default()
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
impl ::protobuf::Clear for RegionDetailResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RegionDetailResponse {
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
    fn default_instance() -> &'static RegionDetailResponse {
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
impl StatusRequest {
    pub fn new_() -> StatusRequest {
        ::std::default::Default::default()
    }
    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = 0
    }
    pub fn set_cmd_type_(&mut self, v: StatusCmdType) {
        self.cmd_type = unsafe { ::std::mem::transmute::<StatusCmdType, i32>(v) };
    }
    pub fn get_cmd_type(&self) -> StatusCmdType {
        unsafe { ::std::mem::transmute::<i32, StatusCmdType>(self.cmd_type) }
    }
    pub fn has_region_leader(&self) -> bool {
        self.region_leader.is_some()
    }
    pub fn clear_region_leader(&mut self) {
        self.region_leader = ::std::option::Option::None
    }
    pub fn set_region_leader(&mut self, v: RegionLeaderRequest) {
        self.region_leader = ::std::option::Option::Some(v);;    }
    pub fn get_region_leader(&self) -> &RegionLeaderRequest {
        match self.region_leader.as_ref() {
            Some(v) => v,
            None => <RegionLeaderRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_leader(&mut self) -> &mut RegionLeaderRequest {
        if self.region_leader.is_none() {
            self.region_leader = ::std::option::Option::Some(RegionLeaderRequest::default());
        }
        self.region_leader.as_mut().unwrap()
    }
    pub fn take_region_leader(&mut self) -> RegionLeaderRequest {
        self.region_leader
            .take()
            .unwrap_or_else(RegionLeaderRequest::default)
    }
    pub fn has_region_detail(&self) -> bool {
        self.region_detail.is_some()
    }
    pub fn clear_region_detail(&mut self) {
        self.region_detail = ::std::option::Option::None
    }
    pub fn set_region_detail(&mut self, v: RegionDetailRequest) {
        self.region_detail = ::std::option::Option::Some(v);;    }
    pub fn get_region_detail(&self) -> &RegionDetailRequest {
        match self.region_detail.as_ref() {
            Some(v) => v,
            None => <RegionDetailRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_detail(&mut self) -> &mut RegionDetailRequest {
        if self.region_detail.is_none() {
            self.region_detail = ::std::option::Option::Some(RegionDetailRequest::default());
        }
        self.region_detail.as_mut().unwrap()
    }
    pub fn take_region_detail(&mut self) -> RegionDetailRequest {
        self.region_detail
            .take()
            .unwrap_or_else(RegionDetailRequest::default)
    }
}
impl ::protobuf::Clear for StatusRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for StatusRequest {
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
    fn default_instance() -> &'static StatusRequest {
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
impl StatusResponse {
    pub fn new_() -> StatusResponse {
        ::std::default::Default::default()
    }
    pub fn clear_cmd_type(&mut self) {
        self.cmd_type = 0
    }
    pub fn set_cmd_type_(&mut self, v: StatusCmdType) {
        self.cmd_type = unsafe { ::std::mem::transmute::<StatusCmdType, i32>(v) };
    }
    pub fn get_cmd_type(&self) -> StatusCmdType {
        unsafe { ::std::mem::transmute::<i32, StatusCmdType>(self.cmd_type) }
    }
    pub fn has_region_leader(&self) -> bool {
        self.region_leader.is_some()
    }
    pub fn clear_region_leader(&mut self) {
        self.region_leader = ::std::option::Option::None
    }
    pub fn set_region_leader(&mut self, v: RegionLeaderResponse) {
        self.region_leader = ::std::option::Option::Some(v);;    }
    pub fn get_region_leader(&self) -> &RegionLeaderResponse {
        match self.region_leader.as_ref() {
            Some(v) => v,
            None => <RegionLeaderResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_leader(&mut self) -> &mut RegionLeaderResponse {
        if self.region_leader.is_none() {
            self.region_leader = ::std::option::Option::Some(RegionLeaderResponse::default());
        }
        self.region_leader.as_mut().unwrap()
    }
    pub fn take_region_leader(&mut self) -> RegionLeaderResponse {
        self.region_leader
            .take()
            .unwrap_or_else(RegionLeaderResponse::default)
    }
    pub fn has_region_detail(&self) -> bool {
        self.region_detail.is_some()
    }
    pub fn clear_region_detail(&mut self) {
        self.region_detail = ::std::option::Option::None
    }
    pub fn set_region_detail(&mut self, v: RegionDetailResponse) {
        self.region_detail = ::std::option::Option::Some(v);;    }
    pub fn get_region_detail(&self) -> &RegionDetailResponse {
        match self.region_detail.as_ref() {
            Some(v) => v,
            None => <RegionDetailResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_detail(&mut self) -> &mut RegionDetailResponse {
        if self.region_detail.is_none() {
            self.region_detail = ::std::option::Option::Some(RegionDetailResponse::default());
        }
        self.region_detail.as_mut().unwrap()
    }
    pub fn take_region_detail(&mut self) -> RegionDetailResponse {
        self.region_detail
            .take()
            .unwrap_or_else(RegionDetailResponse::default)
    }
}
impl ::protobuf::Clear for StatusResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for StatusResponse {
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
    fn default_instance() -> &'static StatusResponse {
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
impl RaftRequestHeader {
    pub fn new_() -> RaftRequestHeader {
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
    pub fn clear_read_quorum(&mut self) {
        self.read_quorum = false
    }
    pub fn set_read_quorum(&mut self, v: bool) {
        self.read_quorum = v;
    }
    pub fn get_read_quorum(&self) -> bool {
        self.read_quorum
    }
    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }
    pub fn set_uuid(&mut self, v: std::vec::Vec<u8>) {
        self.uuid = v;
    }
    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }
    pub fn mut_uuid(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.uuid
    }
    pub fn take_uuid(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
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
    pub fn clear_term(&mut self) {
        self.term = 0
    }
    pub fn set_term(&mut self, v: u64) {
        self.term = v;
    }
    pub fn get_term(&self) -> u64 {
        self.term
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
}
impl ::protobuf::Clear for RaftRequestHeader {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RaftRequestHeader {
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
    fn default_instance() -> &'static RaftRequestHeader {
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
impl RaftResponseHeader {
    pub fn new_() -> RaftResponseHeader {
        ::std::default::Default::default()
    }
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    pub fn set_error(&mut self, v: super::errorpb::Error) {
        self.error = ::std::option::Option::Some(v);;    }
    pub fn get_error(&self) -> &super::errorpb::Error {
        match self.error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_error(&mut self) -> &mut super::errorpb::Error {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.error.as_mut().unwrap()
    }
    pub fn take_error(&mut self) -> super::errorpb::Error {
        self.error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }
    pub fn set_uuid(&mut self, v: std::vec::Vec<u8>) {
        self.uuid = v;
    }
    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }
    pub fn mut_uuid(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.uuid
    }
    pub fn take_uuid(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }
    pub fn clear_current_term(&mut self) {
        self.current_term = 0
    }
    pub fn set_current_term(&mut self, v: u64) {
        self.current_term = v;
    }
    pub fn get_current_term(&self) -> u64 {
        self.current_term
    }
}
impl ::protobuf::Clear for RaftResponseHeader {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RaftResponseHeader {
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
    fn default_instance() -> &'static RaftResponseHeader {
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
impl RaftCmdRequest {
    pub fn new_() -> RaftCmdRequest {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RaftRequestHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RaftRequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RaftRequestHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RaftRequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RaftRequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RaftRequestHeader {
        self.header
            .take()
            .unwrap_or_else(RaftRequestHeader::default)
    }
    pub fn clear_requests(&mut self) {
        self.requests.clear();
    }
    pub fn set_requests(&mut self, v: ::std::vec::Vec<Request>) {
        self.requests = v;
    }
    pub fn get_requests(&self) -> &::std::vec::Vec<Request> {
        &self.requests
    }
    pub fn mut_requests(&mut self) -> &mut ::std::vec::Vec<Request> {
        &mut self.requests
    }
    pub fn take_requests(&mut self) -> ::std::vec::Vec<Request> {
        ::std::mem::replace(&mut self.requests, ::std::vec::Vec::new())
    }
    pub fn has_admin_request(&self) -> bool {
        self.admin_request.is_some()
    }
    pub fn clear_admin_request(&mut self) {
        self.admin_request = ::std::option::Option::None
    }
    pub fn set_admin_request(&mut self, v: AdminRequest) {
        self.admin_request = ::std::option::Option::Some(v);;    }
    pub fn get_admin_request(&self) -> &AdminRequest {
        match self.admin_request.as_ref() {
            Some(v) => v,
            None => <AdminRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_admin_request(&mut self) -> &mut AdminRequest {
        if self.admin_request.is_none() {
            self.admin_request = ::std::option::Option::Some(AdminRequest::default());
        }
        self.admin_request.as_mut().unwrap()
    }
    pub fn take_admin_request(&mut self) -> AdminRequest {
        self.admin_request
            .take()
            .unwrap_or_else(AdminRequest::default)
    }
    pub fn has_status_request(&self) -> bool {
        self.status_request.is_some()
    }
    pub fn clear_status_request(&mut self) {
        self.status_request = ::std::option::Option::None
    }
    pub fn set_status_request(&mut self, v: StatusRequest) {
        self.status_request = ::std::option::Option::Some(v);;    }
    pub fn get_status_request(&self) -> &StatusRequest {
        match self.status_request.as_ref() {
            Some(v) => v,
            None => <StatusRequest as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_status_request(&mut self) -> &mut StatusRequest {
        if self.status_request.is_none() {
            self.status_request = ::std::option::Option::Some(StatusRequest::default());
        }
        self.status_request.as_mut().unwrap()
    }
    pub fn take_status_request(&mut self) -> StatusRequest {
        self.status_request
            .take()
            .unwrap_or_else(StatusRequest::default)
    }
}
impl ::protobuf::Clear for RaftCmdRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RaftCmdRequest {
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
    fn default_instance() -> &'static RaftCmdRequest {
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
impl RaftCmdResponse {
    pub fn new_() -> RaftCmdResponse {
        ::std::default::Default::default()
    }
    pub fn has_header(&self) -> bool {
        self.header.is_some()
    }
    pub fn clear_header(&mut self) {
        self.header = ::std::option::Option::None
    }
    pub fn set_header(&mut self, v: RaftResponseHeader) {
        self.header = ::std::option::Option::Some(v);;    }
    pub fn get_header(&self) -> &RaftResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => <RaftResponseHeader as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_header(&mut self) -> &mut RaftResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RaftResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    pub fn take_header(&mut self) -> RaftResponseHeader {
        self.header
            .take()
            .unwrap_or_else(RaftResponseHeader::default)
    }
    pub fn clear_responses(&mut self) {
        self.responses.clear();
    }
    pub fn set_responses(&mut self, v: ::std::vec::Vec<Response>) {
        self.responses = v;
    }
    pub fn get_responses(&self) -> &::std::vec::Vec<Response> {
        &self.responses
    }
    pub fn mut_responses(&mut self) -> &mut ::std::vec::Vec<Response> {
        &mut self.responses
    }
    pub fn take_responses(&mut self) -> ::std::vec::Vec<Response> {
        ::std::mem::replace(&mut self.responses, ::std::vec::Vec::new())
    }
    pub fn has_admin_response(&self) -> bool {
        self.admin_response.is_some()
    }
    pub fn clear_admin_response(&mut self) {
        self.admin_response = ::std::option::Option::None
    }
    pub fn set_admin_response(&mut self, v: AdminResponse) {
        self.admin_response = ::std::option::Option::Some(v);;    }
    pub fn get_admin_response(&self) -> &AdminResponse {
        match self.admin_response.as_ref() {
            Some(v) => v,
            None => <AdminResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_admin_response(&mut self) -> &mut AdminResponse {
        if self.admin_response.is_none() {
            self.admin_response = ::std::option::Option::Some(AdminResponse::default());
        }
        self.admin_response.as_mut().unwrap()
    }
    pub fn take_admin_response(&mut self) -> AdminResponse {
        self.admin_response
            .take()
            .unwrap_or_else(AdminResponse::default)
    }
    pub fn has_status_response(&self) -> bool {
        self.status_response.is_some()
    }
    pub fn clear_status_response(&mut self) {
        self.status_response = ::std::option::Option::None
    }
    pub fn set_status_response(&mut self, v: StatusResponse) {
        self.status_response = ::std::option::Option::Some(v);;    }
    pub fn get_status_response(&self) -> &StatusResponse {
        match self.status_response.as_ref() {
            Some(v) => v,
            None => <StatusResponse as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_status_response(&mut self) -> &mut StatusResponse {
        if self.status_response.is_none() {
            self.status_response = ::std::option::Option::Some(StatusResponse::default());
        }
        self.status_response.as_mut().unwrap()
    }
    pub fn take_status_response(&mut self) -> StatusResponse {
        self.status_response
            .take()
            .unwrap_or_else(StatusResponse::default)
    }
}
impl ::protobuf::Clear for RaftCmdResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RaftCmdResponse {
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
    fn default_instance() -> &'static RaftCmdResponse {
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
