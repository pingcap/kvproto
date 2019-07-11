// Generated file, please don't edit manually.

impl RequestHeader {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RequestHeader = RequestHeader::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_cluster_id(&mut self) {
        self.cluster_id = 0
    }
    #[inline]
    pub fn set_cluster_id(&mut self, v: u64) {
        self.cluster_id = v;
    }
    #[inline]
    pub fn get_cluster_id(&self) -> u64 {
        self.cluster_id
    }
}
impl ResponseHeader {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ResponseHeader = ResponseHeader::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_cluster_id(&mut self) {
        self.cluster_id = 0
    }
    #[inline]
    pub fn set_cluster_id(&mut self, v: u64) {
        self.cluster_id = v;
    }
    #[inline]
    pub fn get_cluster_id(&self) -> u64 {
        self.cluster_id
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
impl Error {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Error = Error::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_field_type(&mut self) {
        self.r#type = 0
    }
    #[inline]
    pub fn set_field_type_(&mut self, v: ErrorType) {
        self.r#type = unsafe { ::std::mem::transmute::<ErrorType, i32>(v) };
    }
    #[inline]
    pub fn get_field_type(&self) -> ErrorType {
        unsafe { ::std::mem::transmute::<i32, ErrorType>(self.r#type) }
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
}
impl TsoRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TsoRequest = TsoRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn clear_count(&mut self) {
        self.count = 0
    }
    #[inline]
    pub fn set_count(&mut self, v: u32) {
        self.count = v;
    }
    #[inline]
    pub fn get_count(&self) -> u32 {
        self.count
    }
}
impl Timestamp {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Timestamp = Timestamp::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_physical(&mut self) {
        self.physical = 0
    }
    #[inline]
    pub fn set_physical(&mut self, v: i64) {
        self.physical = v;
    }
    #[inline]
    pub fn get_physical(&self) -> i64 {
        self.physical
    }
    #[inline]
    pub fn clear_logical(&mut self) {
        self.logical = 0
    }
    #[inline]
    pub fn set_logical(&mut self, v: i64) {
        self.logical = v;
    }
    #[inline]
    pub fn get_logical(&self) -> i64 {
        self.logical
    }
}
impl TsoResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TsoResponse = TsoResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_count(&mut self) {
        self.count = 0
    }
    #[inline]
    pub fn set_count(&mut self, v: u32) {
        self.count = v;
    }
    #[inline]
    pub fn get_count(&self) -> u32 {
        self.count
    }
    #[inline]
    pub fn has_timestamp(&self) -> bool {
        self.timestamp.is_some()
    }
    #[inline]
    pub fn clear_timestamp(&mut self) {
        self.timestamp = ::std::option::Option::None
    }
    #[inline]
    pub fn set_timestamp(&mut self, v: Timestamp) {
        self.timestamp = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_timestamp(&self) -> &Timestamp {
        match self.timestamp.as_ref() {
            Some(v) => v,
            None => Timestamp::default_ref(),
        }
    }
    #[inline]
    pub fn mut_timestamp(&mut self) -> &mut Timestamp {
        if self.timestamp.is_none() {
            self.timestamp = ::std::option::Option::Some(Timestamp::default());
        }
        self.timestamp.as_mut().unwrap()
    }
    #[inline]
    pub fn take_timestamp(&mut self) -> Timestamp {
        self.timestamp.take().unwrap_or_else(Timestamp::default)
    }
}
impl BootstrapRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BootstrapRequest = BootstrapRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn has_store(&self) -> bool {
        self.store.is_some()
    }
    #[inline]
    pub fn clear_store(&mut self) {
        self.store = ::std::option::Option::None
    }
    #[inline]
    pub fn set_store(&mut self, v: super::metapb::Store) {
        self.store = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_store(&self) -> &super::metapb::Store {
        match self.store.as_ref() {
            Some(v) => v,
            None => super::metapb::Store::default_ref(),
        }
    }
    #[inline]
    pub fn mut_store(&mut self) -> &mut super::metapb::Store {
        if self.store.is_none() {
            self.store = ::std::option::Option::Some(super::metapb::Store::default());
        }
        self.store.as_mut().unwrap()
    }
    #[inline]
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store
            .take()
            .unwrap_or_else(super::metapb::Store::default)
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
}
impl BootstrapResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: BootstrapResponse = BootstrapResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl IsBootstrappedRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: IsBootstrappedRequest = IsBootstrappedRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
}
impl IsBootstrappedResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: IsBootstrappedResponse = IsBootstrappedResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_bootstrapped(&mut self) {
        self.bootstrapped = false
    }
    #[inline]
    pub fn set_bootstrapped(&mut self, v: bool) {
        self.bootstrapped = v;
    }
    #[inline]
    pub fn get_bootstrapped(&self) -> bool {
        self.bootstrapped
    }
}
impl AllocIdRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AllocIdRequest = AllocIdRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
}
impl AllocIdResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AllocIdResponse = AllocIdResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_id(&mut self) {
        self.id = 0
    }
    #[inline]
    pub fn set_id(&mut self, v: u64) {
        self.id = v;
    }
    #[inline]
    pub fn get_id(&self) -> u64 {
        self.id
    }
}
impl GetStoreRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetStoreRequest = GetStoreRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn clear_store_id(&mut self) {
        self.store_id = 0
    }
    #[inline]
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = v;
    }
    #[inline]
    pub fn get_store_id(&self) -> u64 {
        self.store_id
    }
}
impl GetStoreResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetStoreResponse = GetStoreResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn has_store(&self) -> bool {
        self.store.is_some()
    }
    #[inline]
    pub fn clear_store(&mut self) {
        self.store = ::std::option::Option::None
    }
    #[inline]
    pub fn set_store(&mut self, v: super::metapb::Store) {
        self.store = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_store(&self) -> &super::metapb::Store {
        match self.store.as_ref() {
            Some(v) => v,
            None => super::metapb::Store::default_ref(),
        }
    }
    #[inline]
    pub fn mut_store(&mut self) -> &mut super::metapb::Store {
        if self.store.is_none() {
            self.store = ::std::option::Option::Some(super::metapb::Store::default());
        }
        self.store.as_mut().unwrap()
    }
    #[inline]
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store
            .take()
            .unwrap_or_else(super::metapb::Store::default)
    }
    #[inline]
    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }
    #[inline]
    pub fn clear_stats(&mut self) {
        self.stats = ::std::option::Option::None
    }
    #[inline]
    pub fn set_stats(&mut self, v: StoreStats) {
        self.stats = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_stats(&self) -> &StoreStats {
        match self.stats.as_ref() {
            Some(v) => v,
            None => StoreStats::default_ref(),
        }
    }
    #[inline]
    pub fn mut_stats(&mut self) -> &mut StoreStats {
        if self.stats.is_none() {
            self.stats = ::std::option::Option::Some(StoreStats::default());
        }
        self.stats.as_mut().unwrap()
    }
    #[inline]
    pub fn take_stats(&mut self) -> StoreStats {
        self.stats.take().unwrap_or_else(StoreStats::default)
    }
}
impl PutStoreRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PutStoreRequest = PutStoreRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn has_store(&self) -> bool {
        self.store.is_some()
    }
    #[inline]
    pub fn clear_store(&mut self) {
        self.store = ::std::option::Option::None
    }
    #[inline]
    pub fn set_store(&mut self, v: super::metapb::Store) {
        self.store = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_store(&self) -> &super::metapb::Store {
        match self.store.as_ref() {
            Some(v) => v,
            None => super::metapb::Store::default_ref(),
        }
    }
    #[inline]
    pub fn mut_store(&mut self) -> &mut super::metapb::Store {
        if self.store.is_none() {
            self.store = ::std::option::Option::Some(super::metapb::Store::default());
        }
        self.store.as_mut().unwrap()
    }
    #[inline]
    pub fn take_store(&mut self) -> super::metapb::Store {
        self.store
            .take()
            .unwrap_or_else(super::metapb::Store::default)
    }
}
impl PutStoreResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PutStoreResponse = PutStoreResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl GetAllStoresRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetAllStoresRequest = GetAllStoresRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn clear_exclude_tombstone_stores(&mut self) {
        self.exclude_tombstone_stores = false
    }
    #[inline]
    pub fn set_exclude_tombstone_stores(&mut self, v: bool) {
        self.exclude_tombstone_stores = v;
    }
    #[inline]
    pub fn get_exclude_tombstone_stores(&self) -> bool {
        self.exclude_tombstone_stores
    }
}
impl GetAllStoresResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetAllStoresResponse = GetAllStoresResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_stores(&mut self) {
        self.stores.clear();
    }
    #[inline]
    pub fn set_stores(&mut self, v: ::std::vec::Vec<super::metapb::Store>) {
        self.stores = v;
    }
    #[inline]
    pub fn get_stores(&self) -> &::std::vec::Vec<super::metapb::Store> {
        &self.stores
    }
    #[inline]
    pub fn mut_stores(&mut self) -> &mut ::std::vec::Vec<super::metapb::Store> {
        &mut self.stores
    }
    #[inline]
    pub fn take_stores(&mut self) -> ::std::vec::Vec<super::metapb::Store> {
        ::std::mem::replace(&mut self.stores, ::std::vec::Vec::new())
    }
}
impl GetRegionRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetRegionRequest = GetRegionRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn clear_region_key(&mut self) {
        self.region_key.clear();
    }
    #[inline]
    pub fn set_region_key(&mut self, v: std::vec::Vec<u8>) {
        self.region_key = v;
    }
    #[inline]
    pub fn get_region_key(&self) -> &[u8] {
        &self.region_key
    }
    #[inline]
    pub fn mut_region_key(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.region_key
    }
    #[inline]
    pub fn take_region_key(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.region_key, ::std::vec::Vec::new())
    }
}
impl GetRegionResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetRegionResponse = GetRegionResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
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
    #[inline]
    pub fn clear_slaves(&mut self) {
        self.slaves.clear();
    }
    #[inline]
    pub fn set_slaves(&mut self, v: ::std::vec::Vec<super::metapb::Peer>) {
        self.slaves = v;
    }
    #[inline]
    pub fn get_slaves(&self) -> &::std::vec::Vec<super::metapb::Peer> {
        &self.slaves
    }
    #[inline]
    pub fn mut_slaves(&mut self) -> &mut ::std::vec::Vec<super::metapb::Peer> {
        &mut self.slaves
    }
    #[inline]
    pub fn take_slaves(&mut self) -> ::std::vec::Vec<super::metapb::Peer> {
        ::std::mem::replace(&mut self.slaves, ::std::vec::Vec::new())
    }
}
impl GetRegionByIdRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetRegionByIdRequest = GetRegionByIdRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
impl ScanRegionsRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanRegionsRequest = ScanRegionsRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
    pub fn clear_limit(&mut self) {
        self.limit = 0
    }
    #[inline]
    pub fn set_limit(&mut self, v: i32) {
        self.limit = v;
    }
    #[inline]
    pub fn get_limit(&self) -> i32 {
        self.limit
    }
}
impl ScanRegionsResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScanRegionsResponse = ScanRegionsResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_regions(&mut self) {
        self.regions.clear();
    }
    #[inline]
    pub fn set_regions(&mut self, v: ::std::vec::Vec<super::metapb::Region>) {
        self.regions = v;
    }
    #[inline]
    pub fn get_regions(&self) -> &::std::vec::Vec<super::metapb::Region> {
        &self.regions
    }
    #[inline]
    pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<super::metapb::Region> {
        &mut self.regions
    }
    #[inline]
    pub fn take_regions(&mut self) -> ::std::vec::Vec<super::metapb::Region> {
        ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_leaders(&mut self) {
        self.leaders.clear();
    }
    #[inline]
    pub fn set_leaders(&mut self, v: ::std::vec::Vec<super::metapb::Peer>) {
        self.leaders = v;
    }
    #[inline]
    pub fn get_leaders(&self) -> &::std::vec::Vec<super::metapb::Peer> {
        &self.leaders
    }
    #[inline]
    pub fn mut_leaders(&mut self) -> &mut ::std::vec::Vec<super::metapb::Peer> {
        &mut self.leaders
    }
    #[inline]
    pub fn take_leaders(&mut self) -> ::std::vec::Vec<super::metapb::Peer> {
        ::std::mem::replace(&mut self.leaders, ::std::vec::Vec::new())
    }
}
impl GetClusterConfigRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetClusterConfigRequest = GetClusterConfigRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
}
impl GetClusterConfigResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetClusterConfigResponse = GetClusterConfigResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn has_cluster(&self) -> bool {
        self.cluster.is_some()
    }
    #[inline]
    pub fn clear_cluster(&mut self) {
        self.cluster = ::std::option::Option::None
    }
    #[inline]
    pub fn set_cluster(&mut self, v: super::metapb::Cluster) {
        self.cluster = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_cluster(&self) -> &super::metapb::Cluster {
        match self.cluster.as_ref() {
            Some(v) => v,
            None => super::metapb::Cluster::default_ref(),
        }
    }
    #[inline]
    pub fn mut_cluster(&mut self) -> &mut super::metapb::Cluster {
        if self.cluster.is_none() {
            self.cluster = ::std::option::Option::Some(super::metapb::Cluster::default());
        }
        self.cluster.as_mut().unwrap()
    }
    #[inline]
    pub fn take_cluster(&mut self) -> super::metapb::Cluster {
        self.cluster
            .take()
            .unwrap_or_else(super::metapb::Cluster::default)
    }
}
impl PutClusterConfigRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PutClusterConfigRequest = PutClusterConfigRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn has_cluster(&self) -> bool {
        self.cluster.is_some()
    }
    #[inline]
    pub fn clear_cluster(&mut self) {
        self.cluster = ::std::option::Option::None
    }
    #[inline]
    pub fn set_cluster(&mut self, v: super::metapb::Cluster) {
        self.cluster = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_cluster(&self) -> &super::metapb::Cluster {
        match self.cluster.as_ref() {
            Some(v) => v,
            None => super::metapb::Cluster::default_ref(),
        }
    }
    #[inline]
    pub fn mut_cluster(&mut self) -> &mut super::metapb::Cluster {
        if self.cluster.is_none() {
            self.cluster = ::std::option::Option::Some(super::metapb::Cluster::default());
        }
        self.cluster.as_mut().unwrap()
    }
    #[inline]
    pub fn take_cluster(&mut self) -> super::metapb::Cluster {
        self.cluster
            .take()
            .unwrap_or_else(super::metapb::Cluster::default)
    }
}
impl PutClusterConfigResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PutClusterConfigResponse = PutClusterConfigResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl Member {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Member = Member::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_name(&mut self) {
        self.name.clear();
    }
    #[inline]
    pub fn set_name(&mut self, v: std::string::String) {
        self.name = v;
    }
    #[inline]
    pub fn get_name(&self) -> &str {
        &self.name
    }
    #[inline]
    pub fn mut_name(&mut self) -> &mut std::string::String {
        &mut self.name
    }
    #[inline]
    pub fn take_name(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.name, ::std::string::String::new())
    }
    #[inline]
    pub fn clear_member_id(&mut self) {
        self.member_id = 0
    }
    #[inline]
    pub fn set_member_id(&mut self, v: u64) {
        self.member_id = v;
    }
    #[inline]
    pub fn get_member_id(&self) -> u64 {
        self.member_id
    }
    #[inline]
    pub fn clear_peer_urls(&mut self) {
        self.peer_urls.clear();
    }
    #[inline]
    pub fn set_peer_urls(&mut self, v: ::std::vec::Vec<std::string::String>) {
        self.peer_urls = v;
    }
    #[inline]
    pub fn get_peer_urls(&self) -> &::std::vec::Vec<std::string::String> {
        &self.peer_urls
    }
    #[inline]
    pub fn mut_peer_urls(&mut self) -> &mut ::std::vec::Vec<std::string::String> {
        &mut self.peer_urls
    }
    #[inline]
    pub fn take_peer_urls(&mut self) -> ::std::vec::Vec<std::string::String> {
        ::std::mem::replace(&mut self.peer_urls, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_client_urls(&mut self) {
        self.client_urls.clear();
    }
    #[inline]
    pub fn set_client_urls(&mut self, v: ::std::vec::Vec<std::string::String>) {
        self.client_urls = v;
    }
    #[inline]
    pub fn get_client_urls(&self) -> &::std::vec::Vec<std::string::String> {
        &self.client_urls
    }
    #[inline]
    pub fn mut_client_urls(&mut self) -> &mut ::std::vec::Vec<std::string::String> {
        &mut self.client_urls
    }
    #[inline]
    pub fn take_client_urls(&mut self) -> ::std::vec::Vec<std::string::String> {
        ::std::mem::replace(&mut self.client_urls, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_leader_priority(&mut self) {
        self.leader_priority = 0
    }
    #[inline]
    pub fn set_leader_priority(&mut self, v: i32) {
        self.leader_priority = v;
    }
    #[inline]
    pub fn get_leader_priority(&self) -> i32 {
        self.leader_priority
    }
}
impl GetMembersRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetMembersRequest = GetMembersRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
}
impl GetMembersResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetMembersResponse = GetMembersResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_members(&mut self) {
        self.members.clear();
    }
    #[inline]
    pub fn set_members(&mut self, v: ::std::vec::Vec<Member>) {
        self.members = v;
    }
    #[inline]
    pub fn get_members(&self) -> &::std::vec::Vec<Member> {
        &self.members
    }
    #[inline]
    pub fn mut_members(&mut self) -> &mut ::std::vec::Vec<Member> {
        &mut self.members
    }
    #[inline]
    pub fn take_members(&mut self) -> ::std::vec::Vec<Member> {
        ::std::mem::replace(&mut self.members, ::std::vec::Vec::new())
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
    pub fn set_leader(&mut self, v: Member) {
        self.leader = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_leader(&self) -> &Member {
        match self.leader.as_ref() {
            Some(v) => v,
            None => Member::default_ref(),
        }
    }
    #[inline]
    pub fn mut_leader(&mut self) -> &mut Member {
        if self.leader.is_none() {
            self.leader = ::std::option::Option::Some(Member::default());
        }
        self.leader.as_mut().unwrap()
    }
    #[inline]
    pub fn take_leader(&mut self) -> Member {
        self.leader.take().unwrap_or_else(Member::default)
    }
    #[inline]
    pub fn has_etcd_leader(&self) -> bool {
        self.etcd_leader.is_some()
    }
    #[inline]
    pub fn clear_etcd_leader(&mut self) {
        self.etcd_leader = ::std::option::Option::None
    }
    #[inline]
    pub fn set_etcd_leader(&mut self, v: Member) {
        self.etcd_leader = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_etcd_leader(&self) -> &Member {
        match self.etcd_leader.as_ref() {
            Some(v) => v,
            None => Member::default_ref(),
        }
    }
    #[inline]
    pub fn mut_etcd_leader(&mut self) -> &mut Member {
        if self.etcd_leader.is_none() {
            self.etcd_leader = ::std::option::Option::Some(Member::default());
        }
        self.etcd_leader.as_mut().unwrap()
    }
    #[inline]
    pub fn take_etcd_leader(&mut self) -> Member {
        self.etcd_leader.take().unwrap_or_else(Member::default)
    }
}
impl PeerStats {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: PeerStats = PeerStats::default();
        }
        &*INSTANCE
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
    pub fn clear_down_seconds(&mut self) {
        self.down_seconds = 0
    }
    #[inline]
    pub fn set_down_seconds(&mut self, v: u64) {
        self.down_seconds = v;
    }
    #[inline]
    pub fn get_down_seconds(&self) -> u64 {
        self.down_seconds
    }
}
impl RegionHeartbeatRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionHeartbeatRequest = RegionHeartbeatRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
    #[inline]
    pub fn clear_down_peers(&mut self) {
        self.down_peers.clear();
    }
    #[inline]
    pub fn set_down_peers(&mut self, v: ::std::vec::Vec<PeerStats>) {
        self.down_peers = v;
    }
    #[inline]
    pub fn get_down_peers(&self) -> &::std::vec::Vec<PeerStats> {
        &self.down_peers
    }
    #[inline]
    pub fn mut_down_peers(&mut self) -> &mut ::std::vec::Vec<PeerStats> {
        &mut self.down_peers
    }
    #[inline]
    pub fn take_down_peers(&mut self) -> ::std::vec::Vec<PeerStats> {
        ::std::mem::replace(&mut self.down_peers, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_pending_peers(&mut self) {
        self.pending_peers.clear();
    }
    #[inline]
    pub fn set_pending_peers(&mut self, v: ::std::vec::Vec<super::metapb::Peer>) {
        self.pending_peers = v;
    }
    #[inline]
    pub fn get_pending_peers(&self) -> &::std::vec::Vec<super::metapb::Peer> {
        &self.pending_peers
    }
    #[inline]
    pub fn mut_pending_peers(&mut self) -> &mut ::std::vec::Vec<super::metapb::Peer> {
        &mut self.pending_peers
    }
    #[inline]
    pub fn take_pending_peers(&mut self) -> ::std::vec::Vec<super::metapb::Peer> {
        ::std::mem::replace(&mut self.pending_peers, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_bytes_written(&mut self) {
        self.bytes_written = 0
    }
    #[inline]
    pub fn set_bytes_written(&mut self, v: u64) {
        self.bytes_written = v;
    }
    #[inline]
    pub fn get_bytes_written(&self) -> u64 {
        self.bytes_written
    }
    #[inline]
    pub fn clear_bytes_read(&mut self) {
        self.bytes_read = 0
    }
    #[inline]
    pub fn set_bytes_read(&mut self, v: u64) {
        self.bytes_read = v;
    }
    #[inline]
    pub fn get_bytes_read(&self) -> u64 {
        self.bytes_read
    }
    #[inline]
    pub fn clear_keys_written(&mut self) {
        self.keys_written = 0
    }
    #[inline]
    pub fn set_keys_written(&mut self, v: u64) {
        self.keys_written = v;
    }
    #[inline]
    pub fn get_keys_written(&self) -> u64 {
        self.keys_written
    }
    #[inline]
    pub fn clear_keys_read(&mut self) {
        self.keys_read = 0
    }
    #[inline]
    pub fn set_keys_read(&mut self, v: u64) {
        self.keys_read = v;
    }
    #[inline]
    pub fn get_keys_read(&self) -> u64 {
        self.keys_read
    }
    #[inline]
    pub fn clear_approximate_size(&mut self) {
        self.approximate_size = 0
    }
    #[inline]
    pub fn set_approximate_size(&mut self, v: u64) {
        self.approximate_size = v;
    }
    #[inline]
    pub fn get_approximate_size(&self) -> u64 {
        self.approximate_size
    }
    #[inline]
    pub fn has_interval(&self) -> bool {
        self.interval.is_some()
    }
    #[inline]
    pub fn clear_interval(&mut self) {
        self.interval = ::std::option::Option::None
    }
    #[inline]
    pub fn set_interval(&mut self, v: TimeInterval) {
        self.interval = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_interval(&self) -> &TimeInterval {
        match self.interval.as_ref() {
            Some(v) => v,
            None => TimeInterval::default_ref(),
        }
    }
    #[inline]
    pub fn mut_interval(&mut self) -> &mut TimeInterval {
        if self.interval.is_none() {
            self.interval = ::std::option::Option::Some(TimeInterval::default());
        }
        self.interval.as_mut().unwrap()
    }
    #[inline]
    pub fn take_interval(&mut self) -> TimeInterval {
        self.interval.take().unwrap_or_else(TimeInterval::default)
    }
    #[inline]
    pub fn clear_approximate_keys(&mut self) {
        self.approximate_keys = 0
    }
    #[inline]
    pub fn set_approximate_keys(&mut self, v: u64) {
        self.approximate_keys = v;
    }
    #[inline]
    pub fn get_approximate_keys(&self) -> u64 {
        self.approximate_keys
    }
}
impl ChangePeer {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ChangePeer = ChangePeer::default();
        }
        &*INSTANCE
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
    pub fn clear_change_type(&mut self) {
        self.change_type = 0
    }
    #[inline]
    pub fn set_change_type_(&mut self, v: super::eraftpb::ConfChangeType) {
        self.change_type =
            unsafe { ::std::mem::transmute::<super::eraftpb::ConfChangeType, i32>(v) };
    }
    #[inline]
    pub fn get_change_type(&self) -> super::eraftpb::ConfChangeType {
        unsafe { ::std::mem::transmute::<i32, super::eraftpb::ConfChangeType>(self.change_type) }
    }
}
impl TransferLeader {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TransferLeader = TransferLeader::default();
        }
        &*INSTANCE
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
}
impl Merge {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Merge = Merge::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_target(&self) -> bool {
        self.target.is_some()
    }
    #[inline]
    pub fn clear_target(&mut self) {
        self.target = ::std::option::Option::None
    }
    #[inline]
    pub fn set_target(&mut self, v: super::metapb::Region) {
        self.target = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_target(&self) -> &super::metapb::Region {
        match self.target.as_ref() {
            Some(v) => v,
            None => super::metapb::Region::default_ref(),
        }
    }
    #[inline]
    pub fn mut_target(&mut self) -> &mut super::metapb::Region {
        if self.target.is_none() {
            self.target = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.target.as_mut().unwrap()
    }
    #[inline]
    pub fn take_target(&mut self) -> super::metapb::Region {
        self.target
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
}
impl SplitRegion {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SplitRegion = SplitRegion::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_policy(&mut self) {
        self.policy = 0
    }
    #[inline]
    pub fn set_policy_(&mut self, v: CheckPolicy) {
        self.policy = unsafe { ::std::mem::transmute::<CheckPolicy, i32>(v) };
    }
    #[inline]
    pub fn get_policy(&self) -> CheckPolicy {
        unsafe { ::std::mem::transmute::<i32, CheckPolicy>(self.policy) }
    }
}
impl RegionHeartbeatResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: RegionHeartbeatResponse = RegionHeartbeatResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn has_change_peer(&self) -> bool {
        self.change_peer.is_some()
    }
    #[inline]
    pub fn clear_change_peer(&mut self) {
        self.change_peer = ::std::option::Option::None
    }
    #[inline]
    pub fn set_change_peer(&mut self, v: ChangePeer) {
        self.change_peer = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_change_peer(&self) -> &ChangePeer {
        match self.change_peer.as_ref() {
            Some(v) => v,
            None => ChangePeer::default_ref(),
        }
    }
    #[inline]
    pub fn mut_change_peer(&mut self) -> &mut ChangePeer {
        if self.change_peer.is_none() {
            self.change_peer = ::std::option::Option::Some(ChangePeer::default());
        }
        self.change_peer.as_mut().unwrap()
    }
    #[inline]
    pub fn take_change_peer(&mut self) -> ChangePeer {
        self.change_peer.take().unwrap_or_else(ChangePeer::default)
    }
    #[inline]
    pub fn has_transfer_leader(&self) -> bool {
        self.transfer_leader.is_some()
    }
    #[inline]
    pub fn clear_transfer_leader(&mut self) {
        self.transfer_leader = ::std::option::Option::None
    }
    #[inline]
    pub fn set_transfer_leader(&mut self, v: TransferLeader) {
        self.transfer_leader = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_transfer_leader(&self) -> &TransferLeader {
        match self.transfer_leader.as_ref() {
            Some(v) => v,
            None => TransferLeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_transfer_leader(&mut self) -> &mut TransferLeader {
        if self.transfer_leader.is_none() {
            self.transfer_leader = ::std::option::Option::Some(TransferLeader::default());
        }
        self.transfer_leader.as_mut().unwrap()
    }
    #[inline]
    pub fn take_transfer_leader(&mut self) -> TransferLeader {
        self.transfer_leader
            .take()
            .unwrap_or_else(TransferLeader::default)
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
    pub fn has_region_epoch(&self) -> bool {
        self.region_epoch.is_some()
    }
    #[inline]
    pub fn clear_region_epoch(&mut self) {
        self.region_epoch = ::std::option::Option::None
    }
    #[inline]
    pub fn set_region_epoch(&mut self, v: super::metapb::RegionEpoch) {
        self.region_epoch = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_region_epoch(&self) -> &super::metapb::RegionEpoch {
        match self.region_epoch.as_ref() {
            Some(v) => v,
            None => super::metapb::RegionEpoch::default_ref(),
        }
    }
    #[inline]
    pub fn mut_region_epoch(&mut self) -> &mut super::metapb::RegionEpoch {
        if self.region_epoch.is_none() {
            self.region_epoch = ::std::option::Option::Some(super::metapb::RegionEpoch::default());
        }
        self.region_epoch.as_mut().unwrap()
    }
    #[inline]
    pub fn take_region_epoch(&mut self) -> super::metapb::RegionEpoch {
        self.region_epoch
            .take()
            .unwrap_or_else(super::metapb::RegionEpoch::default)
    }
    #[inline]
    pub fn has_target_peer(&self) -> bool {
        self.target_peer.is_some()
    }
    #[inline]
    pub fn clear_target_peer(&mut self) {
        self.target_peer = ::std::option::Option::None
    }
    #[inline]
    pub fn set_target_peer(&mut self, v: super::metapb::Peer) {
        self.target_peer = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_target_peer(&self) -> &super::metapb::Peer {
        match self.target_peer.as_ref() {
            Some(v) => v,
            None => super::metapb::Peer::default_ref(),
        }
    }
    #[inline]
    pub fn mut_target_peer(&mut self) -> &mut super::metapb::Peer {
        if self.target_peer.is_none() {
            self.target_peer = ::std::option::Option::Some(super::metapb::Peer::default());
        }
        self.target_peer.as_mut().unwrap()
    }
    #[inline]
    pub fn take_target_peer(&mut self) -> super::metapb::Peer {
        self.target_peer
            .take()
            .unwrap_or_else(super::metapb::Peer::default)
    }
    #[inline]
    pub fn has_merge(&self) -> bool {
        self.merge.is_some()
    }
    #[inline]
    pub fn clear_merge(&mut self) {
        self.merge = ::std::option::Option::None
    }
    #[inline]
    pub fn set_merge(&mut self, v: Merge) {
        self.merge = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_merge(&self) -> &Merge {
        match self.merge.as_ref() {
            Some(v) => v,
            None => Merge::default_ref(),
        }
    }
    #[inline]
    pub fn mut_merge(&mut self) -> &mut Merge {
        if self.merge.is_none() {
            self.merge = ::std::option::Option::Some(Merge::default());
        }
        self.merge.as_mut().unwrap()
    }
    #[inline]
    pub fn take_merge(&mut self) -> Merge {
        self.merge.take().unwrap_or_else(Merge::default)
    }
    #[inline]
    pub fn has_split_region(&self) -> bool {
        self.split_region.is_some()
    }
    #[inline]
    pub fn clear_split_region(&mut self) {
        self.split_region = ::std::option::Option::None
    }
    #[inline]
    pub fn set_split_region(&mut self, v: SplitRegion) {
        self.split_region = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_split_region(&self) -> &SplitRegion {
        match self.split_region.as_ref() {
            Some(v) => v,
            None => SplitRegion::default_ref(),
        }
    }
    #[inline]
    pub fn mut_split_region(&mut self) -> &mut SplitRegion {
        if self.split_region.is_none() {
            self.split_region = ::std::option::Option::Some(SplitRegion::default());
        }
        self.split_region.as_mut().unwrap()
    }
    #[inline]
    pub fn take_split_region(&mut self) -> SplitRegion {
        self.split_region
            .take()
            .unwrap_or_else(SplitRegion::default)
    }
}
impl AskSplitRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AskSplitRequest = AskSplitRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
}
impl AskSplitResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AskSplitResponse = AskSplitResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_new_region_id(&mut self) {
        self.new_region_id = 0
    }
    #[inline]
    pub fn set_new_region_id(&mut self, v: u64) {
        self.new_region_id = v;
    }
    #[inline]
    pub fn get_new_region_id(&self) -> u64 {
        self.new_region_id
    }
    #[inline]
    pub fn clear_new_peer_ids(&mut self) {
        self.new_peer_ids.clear();
    }
    #[inline]
    pub fn set_new_peer_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.new_peer_ids = v;
    }
    #[inline]
    pub fn get_new_peer_ids(&self) -> &::std::vec::Vec<u64> {
        &self.new_peer_ids
    }
    #[inline]
    pub fn mut_new_peer_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.new_peer_ids
    }
    #[inline]
    pub fn take_new_peer_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.new_peer_ids, ::std::vec::Vec::new())
    }
}
impl ReportSplitRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ReportSplitRequest = ReportSplitRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn has_left(&self) -> bool {
        self.left.is_some()
    }
    #[inline]
    pub fn clear_left(&mut self) {
        self.left = ::std::option::Option::None
    }
    #[inline]
    pub fn set_left(&mut self, v: super::metapb::Region) {
        self.left = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_left(&self) -> &super::metapb::Region {
        match self.left.as_ref() {
            Some(v) => v,
            None => super::metapb::Region::default_ref(),
        }
    }
    #[inline]
    pub fn mut_left(&mut self) -> &mut super::metapb::Region {
        if self.left.is_none() {
            self.left = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.left.as_mut().unwrap()
    }
    #[inline]
    pub fn take_left(&mut self) -> super::metapb::Region {
        self.left
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
    #[inline]
    pub fn has_right(&self) -> bool {
        self.right.is_some()
    }
    #[inline]
    pub fn clear_right(&mut self) {
        self.right = ::std::option::Option::None
    }
    #[inline]
    pub fn set_right(&mut self, v: super::metapb::Region) {
        self.right = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_right(&self) -> &super::metapb::Region {
        match self.right.as_ref() {
            Some(v) => v,
            None => super::metapb::Region::default_ref(),
        }
    }
    #[inline]
    pub fn mut_right(&mut self) -> &mut super::metapb::Region {
        if self.right.is_none() {
            self.right = ::std::option::Option::Some(super::metapb::Region::default());
        }
        self.right.as_mut().unwrap()
    }
    #[inline]
    pub fn take_right(&mut self) -> super::metapb::Region {
        self.right
            .take()
            .unwrap_or_else(super::metapb::Region::default)
    }
}
impl ReportSplitResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ReportSplitResponse = ReportSplitResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl AskBatchSplitRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AskBatchSplitRequest = AskBatchSplitRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
    pub fn clear_split_count(&mut self) {
        self.split_count = 0
    }
    #[inline]
    pub fn set_split_count(&mut self, v: u32) {
        self.split_count = v;
    }
    #[inline]
    pub fn get_split_count(&self) -> u32 {
        self.split_count
    }
}
impl SplitId {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SplitId = SplitId::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_new_region_id(&mut self) {
        self.new_region_id = 0
    }
    #[inline]
    pub fn set_new_region_id(&mut self, v: u64) {
        self.new_region_id = v;
    }
    #[inline]
    pub fn get_new_region_id(&self) -> u64 {
        self.new_region_id
    }
    #[inline]
    pub fn clear_new_peer_ids(&mut self) {
        self.new_peer_ids.clear();
    }
    #[inline]
    pub fn set_new_peer_ids(&mut self, v: ::std::vec::Vec<u64>) {
        self.new_peer_ids = v;
    }
    #[inline]
    pub fn get_new_peer_ids(&self) -> &::std::vec::Vec<u64> {
        &self.new_peer_ids
    }
    #[inline]
    pub fn mut_new_peer_ids(&mut self) -> &mut ::std::vec::Vec<u64> {
        &mut self.new_peer_ids
    }
    #[inline]
    pub fn take_new_peer_ids(&mut self) -> ::std::vec::Vec<u64> {
        ::std::mem::replace(&mut self.new_peer_ids, ::std::vec::Vec::new())
    }
}
impl AskBatchSplitResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: AskBatchSplitResponse = AskBatchSplitResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_ids(&mut self) {
        self.ids.clear();
    }
    #[inline]
    pub fn set_ids(&mut self, v: ::std::vec::Vec<SplitId>) {
        self.ids = v;
    }
    #[inline]
    pub fn get_ids(&self) -> &::std::vec::Vec<SplitId> {
        &self.ids
    }
    #[inline]
    pub fn mut_ids(&mut self) -> &mut ::std::vec::Vec<SplitId> {
        &mut self.ids
    }
    #[inline]
    pub fn take_ids(&mut self) -> ::std::vec::Vec<SplitId> {
        ::std::mem::replace(&mut self.ids, ::std::vec::Vec::new())
    }
}
impl ReportBatchSplitRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ReportBatchSplitRequest = ReportBatchSplitRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn clear_regions(&mut self) {
        self.regions.clear();
    }
    #[inline]
    pub fn set_regions(&mut self, v: ::std::vec::Vec<super::metapb::Region>) {
        self.regions = v;
    }
    #[inline]
    pub fn get_regions(&self) -> &::std::vec::Vec<super::metapb::Region> {
        &self.regions
    }
    #[inline]
    pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<super::metapb::Region> {
        &mut self.regions
    }
    #[inline]
    pub fn take_regions(&mut self) -> ::std::vec::Vec<super::metapb::Region> {
        ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new())
    }
}
impl ReportBatchSplitResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ReportBatchSplitResponse = ReportBatchSplitResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl TimeInterval {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: TimeInterval = TimeInterval::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_start_timestamp(&mut self) {
        self.start_timestamp = 0
    }
    #[inline]
    pub fn set_start_timestamp(&mut self, v: u64) {
        self.start_timestamp = v;
    }
    #[inline]
    pub fn get_start_timestamp(&self) -> u64 {
        self.start_timestamp
    }
    #[inline]
    pub fn clear_end_timestamp(&mut self) {
        self.end_timestamp = 0
    }
    #[inline]
    pub fn set_end_timestamp(&mut self, v: u64) {
        self.end_timestamp = v;
    }
    #[inline]
    pub fn get_end_timestamp(&self) -> u64 {
        self.end_timestamp
    }
}
impl StoreStats {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StoreStats = StoreStats::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_store_id(&mut self) {
        self.store_id = 0
    }
    #[inline]
    pub fn set_store_id(&mut self, v: u64) {
        self.store_id = v;
    }
    #[inline]
    pub fn get_store_id(&self) -> u64 {
        self.store_id
    }
    #[inline]
    pub fn clear_capacity(&mut self) {
        self.capacity = 0
    }
    #[inline]
    pub fn set_capacity(&mut self, v: u64) {
        self.capacity = v;
    }
    #[inline]
    pub fn get_capacity(&self) -> u64 {
        self.capacity
    }
    #[inline]
    pub fn clear_available(&mut self) {
        self.available = 0
    }
    #[inline]
    pub fn set_available(&mut self, v: u64) {
        self.available = v;
    }
    #[inline]
    pub fn get_available(&self) -> u64 {
        self.available
    }
    #[inline]
    pub fn clear_region_count(&mut self) {
        self.region_count = 0
    }
    #[inline]
    pub fn set_region_count(&mut self, v: u32) {
        self.region_count = v;
    }
    #[inline]
    pub fn get_region_count(&self) -> u32 {
        self.region_count
    }
    #[inline]
    pub fn clear_sending_snap_count(&mut self) {
        self.sending_snap_count = 0
    }
    #[inline]
    pub fn set_sending_snap_count(&mut self, v: u32) {
        self.sending_snap_count = v;
    }
    #[inline]
    pub fn get_sending_snap_count(&self) -> u32 {
        self.sending_snap_count
    }
    #[inline]
    pub fn clear_receiving_snap_count(&mut self) {
        self.receiving_snap_count = 0
    }
    #[inline]
    pub fn set_receiving_snap_count(&mut self, v: u32) {
        self.receiving_snap_count = v;
    }
    #[inline]
    pub fn get_receiving_snap_count(&self) -> u32 {
        self.receiving_snap_count
    }
    #[inline]
    pub fn clear_start_time(&mut self) {
        self.start_time = 0
    }
    #[inline]
    pub fn set_start_time(&mut self, v: u32) {
        self.start_time = v;
    }
    #[inline]
    pub fn get_start_time(&self) -> u32 {
        self.start_time
    }
    #[inline]
    pub fn clear_applying_snap_count(&mut self) {
        self.applying_snap_count = 0
    }
    #[inline]
    pub fn set_applying_snap_count(&mut self, v: u32) {
        self.applying_snap_count = v;
    }
    #[inline]
    pub fn get_applying_snap_count(&self) -> u32 {
        self.applying_snap_count
    }
    #[inline]
    pub fn clear_is_busy(&mut self) {
        self.is_busy = false
    }
    #[inline]
    pub fn set_is_busy(&mut self, v: bool) {
        self.is_busy = v;
    }
    #[inline]
    pub fn get_is_busy(&self) -> bool {
        self.is_busy
    }
    #[inline]
    pub fn clear_used_size(&mut self) {
        self.used_size = 0
    }
    #[inline]
    pub fn set_used_size(&mut self, v: u64) {
        self.used_size = v;
    }
    #[inline]
    pub fn get_used_size(&self) -> u64 {
        self.used_size
    }
    #[inline]
    pub fn clear_bytes_written(&mut self) {
        self.bytes_written = 0
    }
    #[inline]
    pub fn set_bytes_written(&mut self, v: u64) {
        self.bytes_written = v;
    }
    #[inline]
    pub fn get_bytes_written(&self) -> u64 {
        self.bytes_written
    }
    #[inline]
    pub fn clear_keys_written(&mut self) {
        self.keys_written = 0
    }
    #[inline]
    pub fn set_keys_written(&mut self, v: u64) {
        self.keys_written = v;
    }
    #[inline]
    pub fn get_keys_written(&self) -> u64 {
        self.keys_written
    }
    #[inline]
    pub fn clear_bytes_read(&mut self) {
        self.bytes_read = 0
    }
    #[inline]
    pub fn set_bytes_read(&mut self, v: u64) {
        self.bytes_read = v;
    }
    #[inline]
    pub fn get_bytes_read(&self) -> u64 {
        self.bytes_read
    }
    #[inline]
    pub fn clear_keys_read(&mut self) {
        self.keys_read = 0
    }
    #[inline]
    pub fn set_keys_read(&mut self, v: u64) {
        self.keys_read = v;
    }
    #[inline]
    pub fn get_keys_read(&self) -> u64 {
        self.keys_read
    }
    #[inline]
    pub fn has_interval(&self) -> bool {
        self.interval.is_some()
    }
    #[inline]
    pub fn clear_interval(&mut self) {
        self.interval = ::std::option::Option::None
    }
    #[inline]
    pub fn set_interval(&mut self, v: TimeInterval) {
        self.interval = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_interval(&self) -> &TimeInterval {
        match self.interval.as_ref() {
            Some(v) => v,
            None => TimeInterval::default_ref(),
        }
    }
    #[inline]
    pub fn mut_interval(&mut self) -> &mut TimeInterval {
        if self.interval.is_none() {
            self.interval = ::std::option::Option::Some(TimeInterval::default());
        }
        self.interval.as_mut().unwrap()
    }
    #[inline]
    pub fn take_interval(&mut self) -> TimeInterval {
        self.interval.take().unwrap_or_else(TimeInterval::default)
    }
}
impl StoreHeartbeatRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StoreHeartbeatRequest = StoreHeartbeatRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn has_stats(&self) -> bool {
        self.stats.is_some()
    }
    #[inline]
    pub fn clear_stats(&mut self) {
        self.stats = ::std::option::Option::None
    }
    #[inline]
    pub fn set_stats(&mut self, v: StoreStats) {
        self.stats = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_stats(&self) -> &StoreStats {
        match self.stats.as_ref() {
            Some(v) => v,
            None => StoreStats::default_ref(),
        }
    }
    #[inline]
    pub fn mut_stats(&mut self) -> &mut StoreStats {
        if self.stats.is_none() {
            self.stats = ::std::option::Option::Some(StoreStats::default());
        }
        self.stats.as_mut().unwrap()
    }
    #[inline]
    pub fn take_stats(&mut self) -> StoreStats {
        self.stats.take().unwrap_or_else(StoreStats::default)
    }
}
impl StoreHeartbeatResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: StoreHeartbeatResponse = StoreHeartbeatResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl ScatterRegionRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScatterRegionRequest = ScatterRegionRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
impl ScatterRegionResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: ScatterRegionResponse = ScatterRegionResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
}
impl GetGcSafePointRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetGcSafePointRequest = GetGcSafePointRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
}
impl GetGcSafePointResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetGcSafePointResponse = GetGcSafePointResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_safe_point(&mut self) {
        self.safe_point = 0
    }
    #[inline]
    pub fn set_safe_point(&mut self, v: u64) {
        self.safe_point = v;
    }
    #[inline]
    pub fn get_safe_point(&self) -> u64 {
        self.safe_point
    }
}
impl UpdateGcSafePointRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UpdateGcSafePointRequest = UpdateGcSafePointRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn clear_safe_point(&mut self) {
        self.safe_point = 0
    }
    #[inline]
    pub fn set_safe_point(&mut self, v: u64) {
        self.safe_point = v;
    }
    #[inline]
    pub fn get_safe_point(&self) -> u64 {
        self.safe_point
    }
}
impl UpdateGcSafePointResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UpdateGcSafePointResponse = UpdateGcSafePointResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_new_safe_point(&mut self) {
        self.new_safe_point = 0
    }
    #[inline]
    pub fn set_new_safe_point(&mut self, v: u64) {
        self.new_safe_point = v;
    }
    #[inline]
    pub fn get_new_safe_point(&self) -> u64 {
        self.new_safe_point
    }
}
impl SyncRegionRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SyncRegionRequest = SyncRegionRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
    }
    #[inline]
    pub fn has_member(&self) -> bool {
        self.member.is_some()
    }
    #[inline]
    pub fn clear_member(&mut self) {
        self.member = ::std::option::Option::None
    }
    #[inline]
    pub fn set_member(&mut self, v: Member) {
        self.member = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_member(&self) -> &Member {
        match self.member.as_ref() {
            Some(v) => v,
            None => Member::default_ref(),
        }
    }
    #[inline]
    pub fn mut_member(&mut self) -> &mut Member {
        if self.member.is_none() {
            self.member = ::std::option::Option::Some(Member::default());
        }
        self.member.as_mut().unwrap()
    }
    #[inline]
    pub fn take_member(&mut self) -> Member {
        self.member.take().unwrap_or_else(Member::default)
    }
    #[inline]
    pub fn clear_start_index(&mut self) {
        self.start_index = 0
    }
    #[inline]
    pub fn set_start_index(&mut self, v: u64) {
        self.start_index = v;
    }
    #[inline]
    pub fn get_start_index(&self) -> u64 {
        self.start_index
    }
}
impl SyncRegionResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SyncRegionResponse = SyncRegionResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
    }
    #[inline]
    pub fn clear_regions(&mut self) {
        self.regions.clear();
    }
    #[inline]
    pub fn set_regions(&mut self, v: ::std::vec::Vec<super::metapb::Region>) {
        self.regions = v;
    }
    #[inline]
    pub fn get_regions(&self) -> &::std::vec::Vec<super::metapb::Region> {
        &self.regions
    }
    #[inline]
    pub fn mut_regions(&mut self) -> &mut ::std::vec::Vec<super::metapb::Region> {
        &mut self.regions
    }
    #[inline]
    pub fn take_regions(&mut self) -> ::std::vec::Vec<super::metapb::Region> {
        ::std::mem::replace(&mut self.regions, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_start_index(&mut self) {
        self.start_index = 0
    }
    #[inline]
    pub fn set_start_index(&mut self, v: u64) {
        self.start_index = v;
    }
    #[inline]
    pub fn get_start_index(&self) -> u64 {
        self.start_index
    }
}
impl GetOperatorRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetOperatorRequest = GetOperatorRequest::default();
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
    pub fn set_header(&mut self, v: RequestHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &RequestHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => RequestHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut RequestHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(RequestHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> RequestHeader {
        self.header.take().unwrap_or_else(RequestHeader::default)
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
impl GetOperatorResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: GetOperatorResponse = GetOperatorResponse::default();
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
    pub fn set_header(&mut self, v: ResponseHeader) {
        self.header = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_header(&self) -> &ResponseHeader {
        match self.header.as_ref() {
            Some(v) => v,
            None => ResponseHeader::default_ref(),
        }
    }
    #[inline]
    pub fn mut_header(&mut self) -> &mut ResponseHeader {
        if self.header.is_none() {
            self.header = ::std::option::Option::Some(ResponseHeader::default());
        }
        self.header.as_mut().unwrap()
    }
    #[inline]
    pub fn take_header(&mut self) -> ResponseHeader {
        self.header.take().unwrap_or_else(ResponseHeader::default)
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
    pub fn clear_desc(&mut self) {
        self.desc.clear();
    }
    #[inline]
    pub fn set_desc(&mut self, v: std::vec::Vec<u8>) {
        self.desc = v;
    }
    #[inline]
    pub fn get_desc(&self) -> &[u8] {
        &self.desc
    }
    #[inline]
    pub fn mut_desc(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.desc
    }
    #[inline]
    pub fn take_desc(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.desc, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_status(&mut self) {
        self.status = 0
    }
    #[inline]
    pub fn set_status_(&mut self, v: OperatorStatus) {
        self.status = unsafe { ::std::mem::transmute::<OperatorStatus, i32>(v) };
    }
    #[inline]
    pub fn get_status(&self) -> OperatorStatus {
        unsafe { ::std::mem::transmute::<i32, OperatorStatus>(self.status) }
    }
    #[inline]
    pub fn clear_kind(&mut self) {
        self.kind.clear();
    }
    #[inline]
    pub fn set_kind(&mut self, v: std::vec::Vec<u8>) {
        self.kind = v;
    }
    #[inline]
    pub fn get_kind(&self) -> &[u8] {
        &self.kind
    }
    #[inline]
    pub fn mut_kind(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.kind
    }
    #[inline]
    pub fn take_kind(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.kind, ::std::vec::Vec::new())
    }
}
impl ErrorType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [ErrorType] = &[
            ErrorType::Ok,
            ErrorType::Unknown,
            ErrorType::NotBootstrapped,
            ErrorType::StoreTombstone,
            ErrorType::AlreadyBootstrapped,
            ErrorType::IncompatibleVersion,
            ErrorType::RegionNotFound,
        ];
        VALUES
    }
}
impl CheckPolicy {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [CheckPolicy] = &[CheckPolicy::Scan, CheckPolicy::Approximate];
        VALUES
    }
}
impl OperatorStatus {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [OperatorStatus] = &[
            OperatorStatus::Success,
            OperatorStatus::Timeout,
            OperatorStatus::Cancel,
            OperatorStatus::Replace,
            OperatorStatus::Running,
        ];
        VALUES
    }
}
