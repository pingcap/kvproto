//! Auto-generated conversions (feature `kvffi_gen`).
#![cfg(feature = "kvffi_gen")]

use std::ptr;

use protobuf::ProtobufEnum;
use crate::ffi_runtime::arena::{Arena, bytes_from};
use crate::ffi_runtime::abi::{DeadlockDeadlockRequest, DeadlockDeadlockResponse, DeadlockReplaceLockByKeyItem, DeadlockReplaceLocksByKeysRequest, DeadlockWaitForEntriesRequest, DeadlockWaitForEntriesResponse, DeadlockWaitForEntry, KvprotoBytesView, KvprotoSliceDeadlockReplaceLockByKeyItemPtr, KvprotoSliceDeadlockWaitForEntryPtr};
use crate::deadlock as pb;

pub fn deadlock_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::DeadlockRequest) -> &'a mut DeadlockDeadlockRequest {
    let mut repr = DeadlockDeadlockRequest {
        tp: Default::default(),
        entry: ptr::null_mut(),
        replace_locks_by_keys: ptr::null_mut(),
    };
    repr.tp = src.get_tp() as i32;
    if src.has_entry() {
        repr.entry = wait_for_entry_to_repr_generated(arena, src.get_entry()) as *mut _;
    } else {
        repr.entry = ptr::null_mut();
    }
    if src.has_replace_locks_by_keys() {
        repr.replace_locks_by_keys = replace_locks_by_keys_request_to_repr_generated(arena, src.get_replace_locks_by_keys()) as *mut _;
    } else {
        repr.replace_locks_by_keys = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn deadlock_request_from_repr_generated(src: *const DeadlockDeadlockRequest) -> Option<pb::DeadlockRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::DeadlockRequest::new();
    out.set_tp(pb::DeadlockRequestType::from_i32(repr.tp).unwrap_or_default());
    if !repr.entry.is_null() {
        if let Some(value) = wait_for_entry_from_repr_generated(repr.entry) {
            out.set_entry(value);
        }
    }
    if !repr.replace_locks_by_keys.is_null() {
        if let Some(value) = replace_locks_by_keys_request_from_repr_generated(repr.replace_locks_by_keys) {
            out.set_replace_locks_by_keys(value);
        }
    }
    Some(out)
}

pub fn deadlock_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::DeadlockResponse) -> &'a mut DeadlockDeadlockResponse {
    let mut repr = DeadlockDeadlockResponse {
        entry: ptr::null_mut(),
        deadlock_key_hash: Default::default(),
        wait_chain: KvprotoSliceDeadlockWaitForEntryPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        deadlock_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if src.has_entry() {
        repr.entry = wait_for_entry_to_repr_generated(arena, src.get_entry()) as *mut _;
    } else {
        repr.entry = ptr::null_mut();
    }
    repr.deadlock_key_hash = src.get_deadlock_key_hash();
    {
        let values = src.get_wait_chain();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut DeadlockWaitForEntry> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(wait_for_entry_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.wait_chain.data = ptr;
                repr.wait_chain.len = len;
                repr.wait_chain.cap = len;
            }
        }
    }
    if !src.get_deadlock_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_deadlock_key());
        repr.deadlock_key.data = ptr;
        repr.deadlock_key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn deadlock_response_from_repr_generated(src: *const DeadlockDeadlockResponse) -> Option<pb::DeadlockResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::DeadlockResponse::new();
    if !repr.entry.is_null() {
        if let Some(value) = wait_for_entry_from_repr_generated(repr.entry) {
            out.set_entry(value);
        }
    }
    out.set_deadlock_key_hash(repr.deadlock_key_hash);
    if !repr.wait_chain.data.is_null() && repr.wait_chain.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.wait_chain.data, repr.wait_chain.len) };
        let mut values: Vec<pb::WaitForEntry> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = wait_for_entry_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_wait_chain(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_deadlock_key(bytes_from(repr.deadlock_key.data, repr.deadlock_key.len).into());
    Some(out)
}

pub fn replace_lock_by_key_item_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ReplaceLockByKeyItem) -> &'a mut DeadlockReplaceLockByKeyItem {
    let mut repr = DeadlockReplaceLockByKeyItem {
        key_hash: Default::default(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        old_lock_ts: Default::default(),
        new_lock_ts: Default::default(),
    };
    repr.key_hash = src.get_key_hash();
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    repr.old_lock_ts = src.get_old_lock_ts();
    repr.new_lock_ts = src.get_new_lock_ts();
    arena.alloc_struct(repr)
}

pub fn replace_lock_by_key_item_from_repr_generated(src: *const DeadlockReplaceLockByKeyItem) -> Option<pb::ReplaceLockByKeyItem> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ReplaceLockByKeyItem::new();
    out.set_key_hash(repr.key_hash);
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_old_lock_ts(repr.old_lock_ts);
    out.set_new_lock_ts(repr.new_lock_ts);
    Some(out)
}

pub fn replace_locks_by_keys_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ReplaceLocksByKeysRequest) -> &'a mut DeadlockReplaceLocksByKeysRequest {
    let mut repr = DeadlockReplaceLocksByKeysRequest {
        items: KvprotoSliceDeadlockReplaceLockByKeyItemPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_items();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut DeadlockReplaceLockByKeyItem> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(replace_lock_by_key_item_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.items.data = ptr;
                repr.items.len = len;
                repr.items.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn replace_locks_by_keys_request_from_repr_generated(src: *const DeadlockReplaceLocksByKeysRequest) -> Option<pb::ReplaceLocksByKeysRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ReplaceLocksByKeysRequest::new();
    if !repr.items.data.is_null() && repr.items.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.items.data, repr.items.len) };
        let mut values: Vec<pb::ReplaceLockByKeyItem> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = replace_lock_by_key_item_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_items(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn wait_for_entries_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::WaitForEntriesRequest) -> &'a mut DeadlockWaitForEntriesRequest {
    let mut repr = DeadlockWaitForEntriesRequest {
    };
    arena.alloc_struct(repr)
}

pub fn wait_for_entries_request_from_repr_generated(src: *const DeadlockWaitForEntriesRequest) -> Option<pb::WaitForEntriesRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::WaitForEntriesRequest::new();
    Some(out)
}

pub fn wait_for_entries_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::WaitForEntriesResponse) -> &'a mut DeadlockWaitForEntriesResponse {
    let mut repr = DeadlockWaitForEntriesResponse {
        entries: KvprotoSliceDeadlockWaitForEntryPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_entries();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut DeadlockWaitForEntry> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(wait_for_entry_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.entries.data = ptr;
                repr.entries.len = len;
                repr.entries.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn wait_for_entries_response_from_repr_generated(src: *const DeadlockWaitForEntriesResponse) -> Option<pb::WaitForEntriesResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::WaitForEntriesResponse::new();
    if !repr.entries.data.is_null() && repr.entries.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.entries.data, repr.entries.len) };
        let mut values: Vec<pb::WaitForEntry> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = wait_for_entry_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_entries(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn wait_for_entry_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::WaitForEntry) -> &'a mut DeadlockWaitForEntry {
    let mut repr = DeadlockWaitForEntry {
        txn: Default::default(),
        wait_for_txn: Default::default(),
        key_hash: Default::default(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        resource_group_tag: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        wait_time: Default::default(),
    };
    repr.txn = src.get_txn();
    repr.wait_for_txn = src.get_wait_for_txn();
    repr.key_hash = src.get_key_hash();
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if !src.get_resource_group_tag().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_resource_group_tag());
        repr.resource_group_tag.data = ptr;
        repr.resource_group_tag.len = len;
    }
    repr.wait_time = src.get_wait_time();
    arena.alloc_struct(repr)
}

pub fn wait_for_entry_from_repr_generated(src: *const DeadlockWaitForEntry) -> Option<pb::WaitForEntry> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::WaitForEntry::new();
    out.set_txn(repr.txn);
    out.set_wait_for_txn(repr.wait_for_txn);
    out.set_key_hash(repr.key_hash);
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_resource_group_tag(bytes_from(repr.resource_group_tag.data, repr.resource_group_tag.len).into());
    out.set_wait_time(repr.wait_time);
    Some(out)
}

