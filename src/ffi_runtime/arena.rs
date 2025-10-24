use std::any::Any;
use std::ffi::c_void;
use std::ptr;

use super::abi::KvprotoSliceKvprotoStringView;

/// Owns heap allocations used when building repr structs.
pub struct Arena {
    byte_buffers: Vec<Vec<u8>>,
    u64_buffers: Vec<Vec<u64>>,
    ptr_arrays: Vec<Vec<*mut c_void>>,
    structs: Vec<Box<dyn Any>>,
}

impl Arena {
    pub fn new() -> Self {
        Self {
            byte_buffers: Vec::new(),
            u64_buffers: Vec::new(),
            ptr_arrays: Vec::new(),
            structs: Vec::new(),
        }
    }

    pub fn alloc_bytes(&mut self, data: &[u8]) -> (*mut u8, usize) {
        if data.is_empty() {
            return (ptr::null_mut(), 0);
        }
        let mut buf = Vec::with_capacity(data.len());
        buf.extend_from_slice(data);
        let ptr = buf.as_mut_ptr();
        let len = buf.len();
        self.byte_buffers.push(buf);
        (ptr, len)
    }

    pub fn alloc_string(&mut self, s: &str) -> (*mut u8, usize) {
        self.alloc_bytes(s.as_bytes())
    }

    pub fn alloc_u64_slice(&mut self, values: &[u64]) -> (*mut u64, usize) {
        if values.is_empty() {
            return (ptr::null_mut(), 0);
        }
        let mut buf = Vec::with_capacity(values.len());
        buf.extend_from_slice(values);
        let ptr = buf.as_mut_ptr();
        let len = buf.len();
        self.u64_buffers.push(buf);
        (ptr, len)
    }

    pub fn alloc_vec<T: 'static>(&mut self, vec: Vec<T>) -> (*mut T, usize) {
        let mut vec = vec;
        let ptr = vec.as_mut_ptr();
        let len = vec.len();
        self.structs.push(Box::new(vec));
        (ptr, len)
    }

    pub fn alloc_struct<T: 'static>(&mut self, value: T) -> &mut T {
        let mut boxed = Box::new(value);
        let ptr = boxed.as_mut() as *mut T;
        self.structs.push(boxed);
        unsafe { &mut *ptr }
    }

    pub fn alloc_ptr_array<T>(&mut self, items: Vec<*mut T>) -> (*mut *mut T, usize) {
        if items.is_empty() {
            return (ptr::null_mut(), 0);
        }
        let mut raw: Vec<*mut c_void> = items.into_iter().map(|p| p as *mut c_void).collect();
        let ptr = raw.as_mut_ptr() as *mut *mut T;
        let len = raw.len();
        self.ptr_arrays.push(raw);
        (ptr, len)
    }
}

impl Default for Arena {
    fn default() -> Self {
        Self::new()
    }
}

pub fn bytes_from(ptr: *const u8, len: usize) -> Vec<u8> {
    if ptr.is_null() || len == 0 {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(ptr, len) }.to_vec()
}

pub fn string_from(ptr: *const u8, len: usize) -> String {
    String::from_utf8(bytes_from(ptr, len)).unwrap_or_default()
}

pub fn u64_slice_from(ptr: *const u64, len: usize) -> Vec<u64> {
    if ptr.is_null() || len == 0 {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(ptr, len) }.to_vec()
}

pub fn string_slice_from_view(view: &KvprotoSliceKvprotoStringView) -> Vec<String> {
    if view.data.is_null() || view.len == 0 {
        return Vec::new();
    }
    unsafe { std::slice::from_raw_parts(view.data, view.len) }
        .iter()
        .map(|sv| string_from(sv.data as *const u8, sv.len))
        .collect()
}
