// Generated file, please don't edit manually.

impl SwitchModeRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SwitchModeRequest = SwitchModeRequest::default();
        }
        &*INSTANCE
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
            None => super::import_sstpb::SwitchModeRequest::default_ref(),
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
impl SwitchModeResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SwitchModeResponse = SwitchModeResponse::default();
        }
        &*INSTANCE
    }
}
impl OpenEngineRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: OpenEngineRequest = OpenEngineRequest::default();
        }
        &*INSTANCE
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
impl OpenEngineResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: OpenEngineResponse = OpenEngineResponse::default();
        }
        &*INSTANCE
    }
}
impl WriteHead {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteHead = WriteHead::default();
        }
        &*INSTANCE
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
impl Mutation {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Mutation = Mutation::default();
        }
        &*INSTANCE
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
impl mutation::Op {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [mutation::Op] = &[mutation::Op::Put];
        VALUES
    }
}
impl WriteBatch {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteBatch = WriteBatch::default();
        }
        &*INSTANCE
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
impl WriteEngineRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteEngineRequest = WriteEngineRequest::default();
        }
        &*INSTANCE
    }
}
impl WriteEngineResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WriteEngineResponse = WriteEngineResponse::default();
        }
        &*INSTANCE
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
            None => Error::default_ref(),
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
impl CloseEngineRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CloseEngineRequest = CloseEngineRequest::default();
        }
        &*INSTANCE
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
impl CloseEngineResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CloseEngineResponse = CloseEngineResponse::default();
        }
        &*INSTANCE
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
            None => Error::default_ref(),
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
impl ImportEngineRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ImportEngineRequest = ImportEngineRequest::default();
        }
        &*INSTANCE
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
impl ImportEngineResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ImportEngineResponse = ImportEngineResponse::default();
        }
        &*INSTANCE
    }
}
impl CleanupEngineRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CleanupEngineRequest = CleanupEngineRequest::default();
        }
        &*INSTANCE
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
impl CleanupEngineResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CleanupEngineResponse = CleanupEngineResponse::default();
        }
        &*INSTANCE
    }
}
impl CompactClusterRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CompactClusterRequest = CompactClusterRequest::default();
        }
        &*INSTANCE
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
            None => super::import_sstpb::CompactRequest::default_ref(),
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
impl CompactClusterResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CompactClusterResponse = CompactClusterResponse::default();
        }
        &*INSTANCE
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
            None => error::EngineNotFound::default_ref(),
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
impl error::EngineNotFound {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: error::EngineNotFound = error::EngineNotFound::default();
        }
        &*INSTANCE
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
