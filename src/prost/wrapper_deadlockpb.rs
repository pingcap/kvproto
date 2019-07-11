// Generated file, please don't edit manually.

impl WaitForEntriesRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WaitForEntriesRequest = WaitForEntriesRequest::default();
        }
        &*INSTANCE
    }
}
impl WaitForEntriesResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WaitForEntriesResponse = WaitForEntriesResponse::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_entries(&mut self) {
        self.entries.clear();
    }
    #[inline]
    pub fn set_entries(&mut self, v: ::std::vec::Vec<WaitForEntry>) {
        self.entries = v;
    }
    #[inline]
    pub fn get_entries(&self) -> &::std::vec::Vec<WaitForEntry> {
        &self.entries
    }
    #[inline]
    pub fn mut_entries(&mut self) -> &mut ::std::vec::Vec<WaitForEntry> {
        &mut self.entries
    }
    #[inline]
    pub fn take_entries(&mut self) -> ::std::vec::Vec<WaitForEntry> {
        ::std::mem::replace(&mut self.entries, ::std::vec::Vec::new())
    }
}
impl WaitForEntry {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: WaitForEntry = WaitForEntry::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_txn(&mut self) {
        self.txn = 0
    }
    #[inline]
    pub fn set_txn(&mut self, v: u64) {
        self.txn = v;
    }
    #[inline]
    pub fn get_txn(&self) -> u64 {
        self.txn
    }
    #[inline]
    pub fn clear_wait_for_txn(&mut self) {
        self.wait_for_txn = 0
    }
    #[inline]
    pub fn set_wait_for_txn(&mut self, v: u64) {
        self.wait_for_txn = v;
    }
    #[inline]
    pub fn get_wait_for_txn(&self) -> u64 {
        self.wait_for_txn
    }
    #[inline]
    pub fn clear_key_hash(&mut self) {
        self.key_hash = 0
    }
    #[inline]
    pub fn set_key_hash(&mut self, v: u64) {
        self.key_hash = v;
    }
    #[inline]
    pub fn get_key_hash(&self) -> u64 {
        self.key_hash
    }
}
impl DeadlockRequest {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DeadlockRequest = DeadlockRequest::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn clear_tp(&mut self) {
        self.tp = 0
    }
    #[inline]
    pub fn set_tp_(&mut self, v: DeadlockRequestType) {
        self.tp = unsafe { ::std::mem::transmute::<DeadlockRequestType, i32>(v) };
    }
    #[inline]
    pub fn get_tp(&self) -> DeadlockRequestType {
        unsafe { ::std::mem::transmute::<i32, DeadlockRequestType>(self.tp) }
    }
    #[inline]
    pub fn has_entry(&self) -> bool {
        self.entry.is_some()
    }
    #[inline]
    pub fn clear_entry(&mut self) {
        self.entry = ::std::option::Option::None
    }
    #[inline]
    pub fn set_entry(&mut self, v: WaitForEntry) {
        self.entry = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_entry(&self) -> &WaitForEntry {
        match self.entry.as_ref() {
            Some(v) => v,
            None => WaitForEntry::default_ref(),
        }
    }
    #[inline]
    pub fn mut_entry(&mut self) -> &mut WaitForEntry {
        if self.entry.is_none() {
            self.entry = ::std::option::Option::Some(WaitForEntry::default());
        }
        self.entry.as_mut().unwrap()
    }
    #[inline]
    pub fn take_entry(&mut self) -> WaitForEntry {
        self.entry.take().unwrap_or_else(WaitForEntry::default)
    }
}
impl DeadlockResponse {
    #[inline]
    pub fn default_ref() -> &'static Self {
        ::lazy_static::lazy_static! {
            static ref INSTANCE: DeadlockResponse = DeadlockResponse::default();
        }
        &*INSTANCE
    }
    #[inline]
    pub fn has_entry(&self) -> bool {
        self.entry.is_some()
    }
    #[inline]
    pub fn clear_entry(&mut self) {
        self.entry = ::std::option::Option::None
    }
    #[inline]
    pub fn set_entry(&mut self, v: WaitForEntry) {
        self.entry = ::std::option::Option::Some(v);
    }
    #[inline]
    pub fn get_entry(&self) -> &WaitForEntry {
        match self.entry.as_ref() {
            Some(v) => v,
            None => WaitForEntry::default_ref(),
        }
    }
    #[inline]
    pub fn mut_entry(&mut self) -> &mut WaitForEntry {
        if self.entry.is_none() {
            self.entry = ::std::option::Option::Some(WaitForEntry::default());
        }
        self.entry.as_mut().unwrap()
    }
    #[inline]
    pub fn take_entry(&mut self) -> WaitForEntry {
        self.entry.take().unwrap_or_else(WaitForEntry::default)
    }
    #[inline]
    pub fn clear_deadlock_key_hash(&mut self) {
        self.deadlock_key_hash = 0
    }
    #[inline]
    pub fn set_deadlock_key_hash(&mut self, v: u64) {
        self.deadlock_key_hash = v;
    }
    #[inline]
    pub fn get_deadlock_key_hash(&self) -> u64 {
        self.deadlock_key_hash
    }
}
impl DeadlockRequestType {
    pub fn values() -> &'static [Self] {
        static VALUES: &'static [DeadlockRequestType] = &[
            DeadlockRequestType::Detect,
            DeadlockRequestType::CleanUpWaitFor,
            DeadlockRequestType::CleanUp,
        ];
        VALUES
    }
}
