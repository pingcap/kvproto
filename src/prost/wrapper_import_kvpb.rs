// Generated file, please don't edit manually.

impl SwitchModeRequest {
    pub fn new_() -> SwitchModeRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_pd_addr(&mut self) {
        self.pd_addr.clear();
    }
    #[inline]
    pub fn set_pd_addr(&mut self, v: std::string::String) {
        self.pd_addr = v;
    }
    #[inline]
    pub fn get_pd_addr(&self) -> &str {
        &self.pd_addr
    }
    #[inline]
    pub fn mut_pd_addr(&mut self) -> &mut std::string::String {
        &mut self.pd_addr
    }
    #[inline]
    pub fn take_pd_addr(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.pd_addr, ::std::string::String::new())
    }
    #[inline]
    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }
    #[inline]
    pub fn clear_request(&mut self) {
        self.request = ::std::option::Option::None
    }
    #[inline]
    pub fn set_request(&mut self, v: super::import_sstpb::SwitchModeRequest) {
        self.request = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_request(&self) -> &super::import_sstpb::SwitchModeRequest {
        match self.request.as_ref() {
            Some(v) => v,
            None => {
                <super::import_sstpb::SwitchModeRequest as ::protobuf::Message>::default_instance()
            }
        }
    }
    #[inline]
    pub fn mut_request(&mut self) -> &mut super::import_sstpb::SwitchModeRequest {
        if self.request.is_none() {
            self.request =
                ::std::option::Option::Some(super::import_sstpb::SwitchModeRequest::default());
        }
        self.request.as_mut().unwrap()
    }
    #[inline]
    pub fn take_request(&mut self) -> super::import_sstpb::SwitchModeRequest {
        self.request
            .take()
            .unwrap_or_else(super::import_sstpb::SwitchModeRequest::default)
    }
}
impl ::protobuf::Clear for SwitchModeRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SwitchModeRequest {
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
    fn default_instance() -> &'static SwitchModeRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SwitchModeRequest = SwitchModeRequest::new_();
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
impl SwitchModeResponse {
    pub fn new_() -> SwitchModeResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for SwitchModeResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for SwitchModeResponse {
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
    fn default_instance() -> &'static SwitchModeResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SwitchModeResponse = SwitchModeResponse::new_();
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
impl OpenEngineRequest {
    pub fn new_() -> OpenEngineRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }
    #[inline]
    pub fn set_uuid(&mut self, v: std::vec::Vec<u8>) {
        self.uuid = v;
    }
    #[inline]
    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }
    #[inline]
    pub fn mut_uuid(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.uuid
    }
    #[inline]
    pub fn take_uuid(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for OpenEngineRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for OpenEngineRequest {
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
    fn default_instance() -> &'static OpenEngineRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: OpenEngineRequest = OpenEngineRequest::new_();
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
impl OpenEngineResponse {
    pub fn new_() -> OpenEngineResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for OpenEngineResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for OpenEngineResponse {
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
    fn default_instance() -> &'static OpenEngineResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: OpenEngineResponse = OpenEngineResponse::new_();
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
impl WriteHead {
    pub fn new_() -> WriteHead {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }
    #[inline]
    pub fn set_uuid(&mut self, v: std::vec::Vec<u8>) {
        self.uuid = v;
    }
    #[inline]
    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }
    #[inline]
    pub fn mut_uuid(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.uuid
    }
    #[inline]
    pub fn take_uuid(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for WriteHead {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for WriteHead {
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
    fn default_instance() -> &'static WriteHead {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteHead = WriteHead::new_();
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
impl Mutation {
    pub fn new_() -> Mutation {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_op(&mut self) {
        self.op = 0
    }
    #[inline]
    pub fn set_op_(&mut self, v: mutation::Op) {
        self.op = unsafe { ::std::mem::transmute::<mutation::Op, i32>(v) };
    }
    #[inline]
    pub fn get_op(&self) -> mutation::Op {
        unsafe { ::std::mem::transmute::<i32, mutation::Op>(self.op) }
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
impl ::protobuf::Clear for Mutation {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for Mutation {
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
    fn default_instance() -> &'static Mutation {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Mutation = Mutation::new_();
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
impl mutation::Op {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [mutation::Op] = &[mutation::Op::Put];
        VALUES
    }
}
impl WriteBatch {
    pub fn new_() -> WriteBatch {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_commit_ts(&mut self) {
        self.commit_ts = 0
    }
    #[inline]
    pub fn set_commit_ts(&mut self, v: u64) {
        self.commit_ts = v;
    }
    #[inline]
    pub fn get_commit_ts(&self) -> u64 {
        self.commit_ts
    }
    #[inline]
    pub fn clear_mutations(&mut self) {
        self.mutations.clear();
    }
    #[inline]
    pub fn set_mutations(&mut self, v: ::std::vec::Vec<Mutation>) {
        self.mutations = v;
    }
    #[inline]
    pub fn get_mutations(&self) -> &::std::vec::Vec<Mutation> {
        &self.mutations
    }
    #[inline]
    pub fn mut_mutations(&mut self) -> &mut ::std::vec::Vec<Mutation> {
        &mut self.mutations
    }
    #[inline]
    pub fn take_mutations(&mut self) -> ::std::vec::Vec<Mutation> {
        ::std::mem::replace(&mut self.mutations, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for WriteBatch {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for WriteBatch {
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
    fn default_instance() -> &'static WriteBatch {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteBatch = WriteBatch::new_();
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
impl WriteEngineRequest {
    pub fn new_() -> WriteEngineRequest {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for WriteEngineRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for WriteEngineRequest {
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
    fn default_instance() -> &'static WriteEngineRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteEngineRequest = WriteEngineRequest::new_();
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
impl WriteEngineResponse {
    pub fn new_() -> WriteEngineResponse {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: Error) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &Error {
        match self.error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut Error {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(Error::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(Error::default)
    }
}
impl ::protobuf::Clear for WriteEngineResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for WriteEngineResponse {
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
    fn default_instance() -> &'static WriteEngineResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteEngineResponse = WriteEngineResponse::new_();
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
impl CloseEngineRequest {
    pub fn new_() -> CloseEngineRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }
    #[inline]
    pub fn set_uuid(&mut self, v: std::vec::Vec<u8>) {
        self.uuid = v;
    }
    #[inline]
    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }
    #[inline]
    pub fn mut_uuid(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.uuid
    }
    #[inline]
    pub fn take_uuid(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for CloseEngineRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CloseEngineRequest {
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
    fn default_instance() -> &'static CloseEngineRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CloseEngineRequest = CloseEngineRequest::new_();
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
impl CloseEngineResponse {
    pub fn new_() -> CloseEngineResponse {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
    #[inline]
    pub fn clear_error(&mut self) {
        self.error = ::std::option::Option::None
    }
    #[inline]
    pub fn set_error(&mut self, v: Error) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &Error {
        match self.error.as_ref() {
            Some(v) => v,
            None => <Error as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut Error {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(Error::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> Error {
        self.error.take().unwrap_or_else(Error::default)
    }
}
impl ::protobuf::Clear for CloseEngineResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CloseEngineResponse {
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
    fn default_instance() -> &'static CloseEngineResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CloseEngineResponse = CloseEngineResponse::new_();
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
impl ImportEngineRequest {
    pub fn new_() -> ImportEngineRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }
    #[inline]
    pub fn set_uuid(&mut self, v: std::vec::Vec<u8>) {
        self.uuid = v;
    }
    #[inline]
    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }
    #[inline]
    pub fn mut_uuid(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.uuid
    }
    #[inline]
    pub fn take_uuid(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_pd_addr(&mut self) {
        self.pd_addr.clear();
    }
    #[inline]
    pub fn set_pd_addr(&mut self, v: std::string::String) {
        self.pd_addr = v;
    }
    #[inline]
    pub fn get_pd_addr(&self) -> &str {
        &self.pd_addr
    }
    #[inline]
    pub fn mut_pd_addr(&mut self) -> &mut std::string::String {
        &mut self.pd_addr
    }
    #[inline]
    pub fn take_pd_addr(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.pd_addr, ::std::string::String::new())
    }
}
impl ::protobuf::Clear for ImportEngineRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ImportEngineRequest {
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
    fn default_instance() -> &'static ImportEngineRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ImportEngineRequest = ImportEngineRequest::new_();
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
impl ImportEngineResponse {
    pub fn new_() -> ImportEngineResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for ImportEngineResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for ImportEngineResponse {
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
    fn default_instance() -> &'static ImportEngineResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ImportEngineResponse = ImportEngineResponse::new_();
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
impl CleanupEngineRequest {
    pub fn new_() -> CleanupEngineRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }
    #[inline]
    pub fn set_uuid(&mut self, v: std::vec::Vec<u8>) {
        self.uuid = v;
    }
    #[inline]
    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }
    #[inline]
    pub fn mut_uuid(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.uuid
    }
    #[inline]
    pub fn take_uuid(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for CleanupEngineRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CleanupEngineRequest {
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
    fn default_instance() -> &'static CleanupEngineRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CleanupEngineRequest = CleanupEngineRequest::new_();
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
impl CleanupEngineResponse {
    pub fn new_() -> CleanupEngineResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for CleanupEngineResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CleanupEngineResponse {
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
    fn default_instance() -> &'static CleanupEngineResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CleanupEngineResponse = CleanupEngineResponse::new_();
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
impl CompactClusterRequest {
    pub fn new_() -> CompactClusterRequest {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_pd_addr(&mut self) {
        self.pd_addr.clear();
    }
    #[inline]
    pub fn set_pd_addr(&mut self, v: std::string::String) {
        self.pd_addr = v;
    }
    #[inline]
    pub fn get_pd_addr(&self) -> &str {
        &self.pd_addr
    }
    #[inline]
    pub fn mut_pd_addr(&mut self) -> &mut std::string::String {
        &mut self.pd_addr
    }
    #[inline]
    pub fn take_pd_addr(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.pd_addr, ::std::string::String::new())
    }
    #[inline]
    pub fn has_request(&self) -> bool {
        self.request.is_some()
    }
    #[inline]
    pub fn clear_request(&mut self) {
        self.request = ::std::option::Option::None
    }
    #[inline]
    pub fn set_request(&mut self, v: super::import_sstpb::CompactRequest) {
        self.request = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_request(&self) -> &super::import_sstpb::CompactRequest {
        match self.request.as_ref() {
            Some(v) => v,
            None => {
                <super::import_sstpb::CompactRequest as ::protobuf::Message>::default_instance()
            }
        }
    }
    #[inline]
    pub fn mut_request(&mut self) -> &mut super::import_sstpb::CompactRequest {
        if self.request.is_none() {
            self.request =
                ::std::option::Option::Some(super::import_sstpb::CompactRequest::default());
        }
        self.request.as_mut().unwrap()
    }
    #[inline]
    pub fn take_request(&mut self) -> super::import_sstpb::CompactRequest {
        self.request
            .take()
            .unwrap_or_else(super::import_sstpb::CompactRequest::default)
    }
}
impl ::protobuf::Clear for CompactClusterRequest {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CompactClusterRequest {
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
    fn default_instance() -> &'static CompactClusterRequest {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CompactClusterRequest = CompactClusterRequest::new_();
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
impl CompactClusterResponse {
    pub fn new_() -> CompactClusterResponse {
        ::std::default::Default::default()
    }
}
impl ::protobuf::Clear for CompactClusterResponse {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for CompactClusterResponse {
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
    fn default_instance() -> &'static CompactClusterResponse {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CompactClusterResponse = CompactClusterResponse::new_();
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
impl Error {
    pub fn new_() -> Error {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn has_engine_not_found(&self) -> bool {
        self.engine_not_found.is_some()
    }
    #[inline]
    pub fn clear_engine_not_found(&mut self) {
        self.engine_not_found = ::std::option::Option::None
    }
    #[inline]
    pub fn set_engine_not_found(&mut self, v: error::EngineNotFound) {
        self.engine_not_found = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_engine_not_found(&self) -> &error::EngineNotFound {
        match self.engine_not_found.as_ref() {
            Some(v) => v,
            None => <error::EngineNotFound as ::protobuf::Message>::default_instance(),
        }
    }
    #[inline]
    pub fn mut_engine_not_found(&mut self) -> &mut error::EngineNotFound {
        if self.engine_not_found.is_none() {
            self.engine_not_found = ::std::option::Option::Some(error::EngineNotFound::default());
        }
        self.engine_not_found.as_mut().unwrap()
    }
    #[inline]
    pub fn take_engine_not_found(&mut self) -> error::EngineNotFound {
        self.engine_not_found
            .take()
            .unwrap_or_else(error::EngineNotFound::default)
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
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Error = Error::new_();
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
impl error::EngineNotFound {
    pub fn new_() -> error::EngineNotFound {
        ::std::default::Default::default()
    }
    #[inline]
    pub fn clear_uuid(&mut self) {
        self.uuid.clear();
    }
    #[inline]
    pub fn set_uuid(&mut self, v: std::vec::Vec<u8>) {
        self.uuid = v;
    }
    #[inline]
    pub fn get_uuid(&self) -> &[u8] {
        &self.uuid
    }
    #[inline]
    pub fn mut_uuid(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.uuid
    }
    #[inline]
    pub fn take_uuid(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.uuid, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for error::EngineNotFound {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for error::EngineNotFound {
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
    fn default_instance() -> &'static error::EngineNotFound {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: error::EngineNotFound = error::EngineNotFound::new_();
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
