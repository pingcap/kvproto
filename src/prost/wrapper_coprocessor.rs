impl KeyRange {
    pub fn new_() -> KeyRange {
        ::std::default::Default::default()
    }
    pub fn clear_start(&mut self) {
        self.start.clear();
    }
    pub fn set_start(&mut self, v: Vec<u8>) {
        self.start = v;
    }
    pub fn get_start(&self) -> &[u8] {
        &self.start
    }
    pub fn mut_start(&mut self) -> &mut Vec<u8> {
        &mut self.start
    }
    pub fn take_start(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.start, ::std::vec::Vec::new())
    }
    pub fn clear_end(&mut self) {
        self.end.clear();
    }
    pub fn set_end(&mut self, v: Vec<u8>) {
        self.end = v;
    }
    pub fn get_end(&self) -> &[u8] {
        &self.end
    }
    pub fn mut_end(&mut self) -> &mut Vec<u8> {
        &mut self.end
    }
    pub fn take_end(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.end, ::std::vec::Vec::new())
    }
}
impl ::protobuf::Clear for KeyRange {
    fn clear(&mut self) {
        ::prost::Message::clear(self);
    }
}
impl ::protobuf::Message for KeyRange {
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
    fn default_instance() -> &'static KeyRange {
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
    pub fn has_context(&self) -> bool {
        self.context.is_some()
    }
    pub fn clear_context(&mut self) {
        self.context = ::std::option::Option::None
    }
    pub fn set_context(&mut self, v: super::kvrpcpb::Context) {
        self.context = ::std::option::Option::Some(v);;    }
    pub fn get_context(&self) -> &super::kvrpcpb::Context {
        match self.context.as_ref() {
            Some(v) => v,
            None => <super::kvrpcpb::Context as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_context(&mut self) -> &mut super::kvrpcpb::Context {
        if self.context.is_none() {
            self.context = ::std::option::Option::Some(super::kvrpcpb::Context::default());
        }
        self.context.as_mut().unwrap()
    }
    pub fn take_context(&mut self) -> super::kvrpcpb::Context {
        self.context
            .take()
            .unwrap_or_else(|| super::kvrpcpb::Context::default())
    }
    pub fn clear_tp(&mut self) {
        self.tp = 0
    }
    pub fn set_tp(&mut self, v: i64) {
        self.tp = v;
    }
    pub fn get_tp(&self) -> i64 {
        self.tp
    }
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
    pub fn set_data(&mut self, v: Vec<u8>) {
        self.data = v;
    }
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn mut_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    pub fn take_data(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
    pub fn clear_ranges(&mut self) {
        self.ranges.clear();
    }
    pub fn set_ranges(&mut self, v: ::std::vec::Vec<KeyRange>) {
        self.ranges = v;
    }
    pub fn get_ranges(&self) -> &::std::vec::Vec<KeyRange> {
        &self.ranges
    }
    pub fn mut_ranges(&mut self) -> &mut ::std::vec::Vec<KeyRange> {
        &mut self.ranges
    }
    pub fn take_ranges(&mut self) -> ::std::vec::Vec<KeyRange> {
        ::std::mem::replace(&mut self.ranges, ::std::vec::Vec::new())
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
    pub fn clear_data(&mut self) {
        self.data.clear();
    }
    pub fn set_data(&mut self, v: Vec<u8>) {
        self.data = v;
    }
    pub fn get_data(&self) -> &[u8] {
        &self.data
    }
    pub fn mut_data(&mut self) -> &mut Vec<u8> {
        &mut self.data
    }
    pub fn take_data(&mut self) -> Vec<u8> {
        ::std::mem::replace(&mut self.data, ::std::vec::Vec::new())
    }
    pub fn has_region_error(&self) -> bool {
        self.region_error.is_some()
    }
    pub fn clear_region_error(&mut self) {
        self.region_error = ::std::option::Option::None
    }
    pub fn set_region_error(&mut self, v: super::errorpb::Error) {
        self.region_error = ::std::option::Option::Some(v);;    }
    pub fn get_region_error(&self) -> &super::errorpb::Error {
        match self.region_error.as_ref() {
            Some(v) => v,
            None => <super::errorpb::Error as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_region_error(&mut self) -> &mut super::errorpb::Error {
        if self.region_error.is_none() {
            self.region_error = ::std::option::Option::Some(super::errorpb::Error::default());
        }
        self.region_error.as_mut().unwrap()
    }
    pub fn take_region_error(&mut self) -> super::errorpb::Error {
        self.region_error
            .take()
            .unwrap_or_else(|| super::errorpb::Error::default())
    }
    pub fn has_locked(&self) -> bool {
        self.locked.is_some()
    }
    pub fn clear_locked(&mut self) {
        self.locked = ::std::option::Option::None
    }
    pub fn set_locked(&mut self, v: super::kvrpcpb::LockInfo) {
        self.locked = ::std::option::Option::Some(v);;    }
    pub fn get_locked(&self) -> &super::kvrpcpb::LockInfo {
        match self.locked.as_ref() {
            Some(v) => v,
            None => <super::kvrpcpb::LockInfo as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_locked(&mut self) -> &mut super::kvrpcpb::LockInfo {
        if self.locked.is_none() {
            self.locked = ::std::option::Option::Some(super::kvrpcpb::LockInfo::default());
        }
        self.locked.as_mut().unwrap()
    }
    pub fn take_locked(&mut self) -> super::kvrpcpb::LockInfo {
        self.locked
            .take()
            .unwrap_or_else(|| super::kvrpcpb::LockInfo::default())
    }
    pub fn clear_other_error(&mut self) {
        self.other_error.clear();
    }
    pub fn set_other_error(&mut self, v: String) {
        self.other_error = v;
    }
    pub fn get_other_error(&self) -> &str {
        &self.other_error
    }
    pub fn mut_other_error(&mut self) -> &mut String {
        &mut self.other_error
    }
    pub fn take_other_error(&mut self) -> String {
        ::std::mem::replace(&mut self.other_error, ::std::string::String::new())
    }
    pub fn has_range(&self) -> bool {
        self.range.is_some()
    }
    pub fn clear_range(&mut self) {
        self.range = ::std::option::Option::None
    }
    pub fn set_range(&mut self, v: KeyRange) {
        self.range = ::std::option::Option::Some(v);;    }
    pub fn get_range(&self) -> &KeyRange {
        match self.range.as_ref() {
            Some(v) => v,
            None => <KeyRange as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_range(&mut self) -> &mut KeyRange {
        if self.range.is_none() {
            self.range = ::std::option::Option::Some(KeyRange::default());
        }
        self.range.as_mut().unwrap()
    }
    pub fn take_range(&mut self) -> KeyRange {
        self.range.take().unwrap_or_else(|| KeyRange::default())
    }
    pub fn has_exec_details(&self) -> bool {
        self.exec_details.is_some()
    }
    pub fn clear_exec_details(&mut self) {
        self.exec_details = ::std::option::Option::None
    }
    pub fn set_exec_details(&mut self, v: super::kvrpcpb::ExecDetails) {
        self.exec_details = ::std::option::Option::Some(v);;    }
    pub fn get_exec_details(&self) -> &super::kvrpcpb::ExecDetails {
        match self.exec_details.as_ref() {
            Some(v) => v,
            None => <super::kvrpcpb::ExecDetails as ::protobuf::Message>::default_instance(),
        }
    }
    pub fn mut_exec_details(&mut self) -> &mut super::kvrpcpb::ExecDetails {
        if self.exec_details.is_none() {
            self.exec_details = ::std::option::Option::Some(super::kvrpcpb::ExecDetails::default());
        }
        self.exec_details.as_mut().unwrap()
    }
    pub fn take_exec_details(&mut self) -> super::kvrpcpb::ExecDetails {
        self.exec_details
            .take()
            .unwrap_or_else(|| super::kvrpcpb::ExecDetails::default())
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
