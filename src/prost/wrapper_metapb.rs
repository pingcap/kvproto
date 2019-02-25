impl Cluster {
    pub fn new_() -> Cluster {
        ::std::default::Default::default()
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
    pub fn clear_max_peer_count(&mut self) {
        self.max_peer_count = 0
    }
    pub fn set_max_peer_count(&mut self, v: u32) {
        self.max_peer_count = v;
    }
    pub fn get_max_peer_count(&self) -> u32 {
        self.max_peer_count
    }
}
impl ::protobuf::Clear for Cluster {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Cluster {
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
    fn default_instance() -> &'static Cluster {
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
impl StoreLabel {
    pub fn new_() -> StoreLabel {
        ::std::default::Default::default()
    }
    pub fn clear_key(&mut self) {
        self.key.clear();
    }
    pub fn set_key(&mut self, v: String) {
        self.key = v;
    }
    pub fn get_key(&self) -> &str {
        &self.key
    }
    pub fn mut_key(&mut self) -> &mut String {
        &mut self.key
    }
    pub fn take_key(&mut self) -> String {
        ::std::mem::replace(&mut self.key, ::std::string::String::new())
    }
    pub fn clear_value(&mut self) {
        self.value.clear();
    }
    pub fn set_value(&mut self, v: String) {
        self.value = v;
    }
    pub fn get_value(&self) -> &str {
        &self.value
    }
    pub fn mut_value(&mut self) -> &mut String {
        &mut self.value
    }
    pub fn take_value(&mut self) -> String {
        ::std::mem::replace(&mut self.value, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for StoreLabel {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for StoreLabel {
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
    fn default_instance() -> &'static StoreLabel {
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
impl Store {
    pub fn new_() -> Store {
        ::std::default::Default::default()
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
    pub fn clear_address(&mut self) {
        self.address.clear();
    }
    pub fn set_address(&mut self, v: String) {
        self.address = v;
    }
    pub fn get_address(&self) -> &str {
        &self.address
    }
    pub fn mut_address(&mut self) -> &mut String {
        &mut self.address
    }
    pub fn take_address(&mut self) -> String {
        ::std::mem::replace(&mut self.address, ::std::string::String::new())
    }
    pub fn clear_state(&mut self) {
        self.state = 0
    }
    pub fn set_state(&mut self, v: StoreState) {
        self.state = unsafe { ::std::mem::transmute::<StoreState, i32>(v) };
    }
    pub fn get_state(&self) -> StoreState {
        unsafe { ::std::mem::transmute::<i32, StoreState>(self.state) }
    }
    pub fn clear_labels(&mut self) {
        self.labels.clear();
    }
    pub fn set_labels(&mut self, v: ::std::vec::Vec<StoreLabel>) {
        self.labels = v;
    }
    pub fn get_labels(&self) -> &::std::vec::Vec<StoreLabel> {
        &self.labels
    }
    pub fn mut_labels(&mut self) -> &mut ::std::vec::Vec<StoreLabel> {
        &mut self.labels
    }
    pub fn take_labels(&mut self) -> ::std::vec::Vec<StoreLabel> {
        ::std::mem::replace(&mut self.labels, ::std::vec::Vec::new())
    }
    pub fn clear_version(&mut self) {
        self.version.clear();
    }
    pub fn set_version(&mut self, v: String) {
        self.version = v;
    }
    pub fn get_version(&self) -> &str {
        &self.version
    }
    pub fn mut_version(&mut self) -> &mut String {
        &mut self.version
    }
    pub fn take_version(&mut self) -> String {
        ::std::mem::replace(&mut self.version, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for Store {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Store {
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
    fn default_instance() -> &'static Store {
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
impl RegionEpoch {
    pub fn new_() -> RegionEpoch {
        ::std::default::Default::default()
    }
    pub fn clear_conf_ver(&mut self) {
        self.conf_ver = 0
    }
    pub fn set_conf_ver(&mut self, v: u64) {
        self.conf_ver = v;
    }
    pub fn get_conf_ver(&self) -> u64 {
        self.conf_ver
    }
    pub fn clear_version(&mut self) {
        self.version = 0
    }
    pub fn set_version(&mut self, v: u64) {
        self.version = v;
    }
    pub fn get_version(&self) -> u64 {
        self.version
    }
}
impl ::protobuf::Clear for RegionEpoch {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for RegionEpoch {
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
    fn default_instance() -> &'static RegionEpoch {
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
impl Region {
    pub fn new_() -> Region {
        ::std::default::Default::default()
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
    pub fn clear_start_key(&mut self) {
        self.start_key.clear();
    }
    pub fn set_start_key(&mut self, v: Vec<u8>) {
        self.start_key = v;
    }
    pub fn get_start_key(&self) -> &[u8] {
        &self.start_key
    }
    pub fn mut_start_key(&mut self) -> &mut Vec<u8> {
        &mut self.start_key
    }
    pub fn take_start_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.start_key, ::std::vec::Vec::new())
    }
    pub fn clear_end_key(&mut self) {
        self.end_key.clear();
    }
    pub fn set_end_key(&mut self, v: Vec<u8>) {
        self.end_key = v;
    }
    pub fn get_end_key(&self) -> &[u8] {
        &self.end_key
    }
    pub fn mut_end_key(&mut self) -> &mut Vec<u8> {
        &mut self.end_key
    }
    pub fn take_end_key(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.end_key, ::std::vec::Vec::new())
    }
    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }
    pub fn clear_region_epoch(&mut self) {
        self.region_epoch = ::std::option::Option::None
    }
    pub fn set_region_epoch(&mut self, v: RegionEpoch) {
        self.region_epoch = ::std::option::Option::Some(v);;    }
    pub fn get_region_epoch(&self) -> &RegionEpoch {
        self.region_epoch
            .as_ref()
            .unwrap_or_else(|| <RegionEpoch as ::protobuf::Message>::default_instance())
    }
    pub fn mut_region_epoch(&mut self) -> &mut RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch = ::std::option::Option::Some(RegionEpoch::default());
        }
        self.region_epoch.as_mut().unwrap()
    }
    pub fn take_region_epoch(&mut self) -> RegionEpoch {
        self.region_epoch
            .take()
            .unwrap_or_else(|| RegionEpoch::default())
    }
    pub fn clear_peers(&mut self) {
        self.peers.clear();
    }
    pub fn set_peers(&mut self, v: ::std::vec::Vec<Peer>) {
        self.peers = v;
    }
    pub fn get_peers(&self) -> &::std::vec::Vec<Peer> {
        &self.peers
    }
    pub fn mut_peers(&mut self) -> &mut ::std::vec::Vec<Peer> {
        &mut self.peers
    }
    pub fn take_peers(&mut self) -> ::std::vec::Vec<Peer> {
        ::std::mem::replace(&mut self.peers, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for Region {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Region {
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
    fn default_instance() -> &'static Region {
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
impl Peer {
    pub fn new_() -> Peer {
        ::std::default::Default::default()
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
    pub fn clear_store_id(&mut self) {
        self.store_id = 0
    }
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = v;
    }
    pub fn get_store_id(&self) -> u64 {
        self.store_id
    }
    pub fn clear_is_learner(&mut self) {
        self.is_learner = false
    }
    pub fn set_is_learner(&mut self, v: bool) {
        self.is_learner = v;
    }
    pub fn get_is_learner(&self) -> bool {
        self.is_learner
    }
}
impl ::protobuf::Clear for Peer {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Peer {
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
    fn default_instance() -> &'static Peer {
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
