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
    pub fn clear_mode(&mut self) {
        self.mode = 0
    }
    #[inline]
    pub fn set_mode_(&mut self, v: SwitchMode) {
        self.mode = unsafe { ::std::mem::transmute::<SwitchMode, i32>(v) };
    }
    #[inline]
    pub fn get_mode(&self) -> SwitchMode {
        unsafe { ::std::mem::transmute::<i32, SwitchMode>(self.mode) }
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
impl Range {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: Range = Range::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_start(&mut self) {
        self.start.clear();
    }
    #[inline]
    pub fn set_start(&mut self, v: std::vec::Vec<u8>) {
        self.start = v;
    }
    #[inline]
    pub fn get_start(&self) -> &[u8] {
        &self.start
    }
    #[inline]
    pub fn mut_start(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.start
    }
    #[inline]
    pub fn take_start(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.start, ::std::vec::Vec::new())
    }
    #[inline]
    pub fn clear_end(&mut self) {
        self.end.clear();
    }
    #[inline]
    pub fn set_end(&mut self, v: std::vec::Vec<u8>) {
        self.end = v;
    }
    #[inline]
    pub fn get_end(&self) -> &[u8] {
        &self.end
    }
    #[inline]
    pub fn mut_end(&mut self) -> &mut std::vec::Vec<u8> {
        &mut self.end
    }
    #[inline]
    pub fn take_end(&mut self) -> std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.end, ::std::vec::Vec::new())
    }
}
impl SstMeta {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: SstMeta = SstMeta::default();
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
    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }
    #[inline]
    pub fn clear_range(&mut self) {
        self.range = ::std::option::Option::None
    }
    #[inline]
    pub fn set_range(&mut self, v: Range) {
        self.range = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_range(&self) -> &Range {
        match self.range.as_ref() {
            Some(v) => v,
            None => Range::default_ref(),
        }
    }
    #[inline]
    pub fn mut_range(&mut self) -> &mut Range {
        if self.range.is_none() {
            self.range = ::std::option::Option::Some(Range::default());
        }
        self.range.as_mut().unwrap()
    }
    #[inline]
    pub fn take_range(&mut self) -> Range {
        self.range.take().unwrap_or_else(Range::default)
    }
    #[inline]
    pub fn clear_crc32(&mut self) {
        self.crc32 = 0
    }
    #[inline]
    pub fn set_crc32(&mut self, v: u32) {
        self.crc32 = v;
    }
    #[inline]
    pub fn get_crc32(&self) -> u32 {
        self.crc32
    }
    #[inline]
    pub fn clear_length(&mut self) {
        self.length = 0
    }
    #[inline]
    pub fn set_length(&mut self, v: u64) {
        self.length = v;
    }
    #[inline]
    pub fn get_length(&self) -> u64 {
        self.length
    }
    #[inline]
    pub fn clear_cf_name(&mut self) {
        self.cf_name.clear();
    }
    #[inline]
    pub fn set_cf_name(&mut self, v: std::string::String) {
        self.cf_name = v;
    }
    #[inline]
    pub fn get_cf_name(&self) -> &str {
        &self.cf_name
    }
    #[inline]
    pub fn mut_cf_name(&mut self) -> &mut std::string::String {
        &mut self.cf_name
    }
    #[inline]
    pub fn take_cf_name(&mut self) -> std::string::String {
        ::std::mem::replace(&mut self.cf_name, ::std::string::String::new())
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
}
impl UploadRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UploadRequest = UploadRequest::default();
        }
        &*INSTANCE
    }
}
impl UploadResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: UploadResponse = UploadResponse::default();
        }
        &*INSTANCE
    }
}
impl IngestRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: IngestRequest = IngestRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    #[inline]
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    #[inline]
    pub fn set_context(&mut self, v: super::kvrpcpb::Context) {
        self.context = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_context(&self) -> &super::kvrpcpb::Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => super::kvrpcpb::Context::default_ref(),
        }
    }
    #[inline]
    pub fn mut_context(&mut self) -> &mut super::kvrpcpb::Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(super::kvrpcpb::Context::default());
        }
        self.context.as_mut().unwrap()
    }
    #[inline]
    pub fn take_context(&mut self) -> super::kvrpcpb::Context {
        self.context
            .take()
            .unwrap_or_else(super::kvrpcpb::Context::default)
    }
    #[inline]
    pub fn has_sst(&self) -> bool {
        self.sst.is_some()
    }
    #[inline]
    pub fn clear_sst(&mut self) {
        self.sst = ::std::option::Option::None
    }
    #[inline]
    pub fn set_sst(&mut self, v: SstMeta) {
        self.sst = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_sst(&self) -> &SstMeta {
        match self.sst.as_ref() {
            Some(v) => v,
            None => SstMeta::default_ref(),
        }
    }
    #[inline]
    pub fn mut_sst(&mut self) -> &mut SstMeta {
        if self.sst.is_none() {
            self.sst = ::std::option::Option::Some(SstMeta::default());
        }
        self.sst.as_mut().unwrap()
    }
    #[inline]
    pub fn take_sst(&mut self) -> SstMeta {
        self.sst.take().unwrap_or_else(SstMeta::default)
    }
}
impl IngestResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: IngestResponse = IngestResponse::default();
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
    pub fn set_error(&mut self, v: super::errorpb::Error) {
        self.error = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_error(&self) -> &super::errorpb::Error {
        match self.error.as_ref() {
            Some(v) => v,
            None => super::errorpb::Error::default_ref(),
        }
    }
    #[inline]
    pub fn mut_error(&mut self) -> &mut super::errorpb::Error {
        if self.error.is_none() {
            self.error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.error.as_mut().unwrap()
    }
    #[inline]
    pub fn take_error(&mut self) -> super::errorpb::Error {
        self.error
            .take()
            .unwrap_or_else(super::errorpb::Error::default)
    }
}
impl CompactRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CompactRequest = CompactRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }
    #[inline]
    pub fn clear_range(&mut self) {
        self.range = ::std::option::Option::None
    }
    #[inline]
    pub fn set_range(&mut self, v: Range) {
        self.range = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_range(&self) -> &Range {
        match self.range.as_ref() {
            Some(v) => v,
            None => Range::default_ref(),
        }
    }
    #[inline]
    pub fn mut_range(&mut self) -> &mut Range {
        if self.range.is_none() {
            self.range = ::std::option::Option::Some(Range::default());
        }
        self.range.as_mut().unwrap()
    }
    #[inline]
    pub fn take_range(&mut self) -> Range {
        self.range.take().unwrap_or_else(Range::default)
    }
    #[inline]
    pub fn clear_output_level(&mut self) {
        self.output_level = 0
    }
    #[inline]
    pub fn set_output_level(&mut self, v: i32) {
        self.output_level = v;
    }
    #[inline]
    pub fn get_output_level(&self) -> i32 {
        self.output_level
    }
}
impl CompactResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: CompactResponse = CompactResponse::default();
        }
        &*INSTANCE
    }
}
impl SwitchMode {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [SwitchMode] = &[SwitchMode::Normal, SwitchMode::Import];
        VALUES
    }
}
