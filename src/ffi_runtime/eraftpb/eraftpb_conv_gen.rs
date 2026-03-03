//! Auto-generated conversions (feature `kvffi_gen`).
#![cfg(feature = "kvffi_gen")]
#![allow(unused_imports, unused_variables, unused_mut, non_snake_case)]

use std::convert::TryInto;
use std::ptr;

use protobuf::Message;
use protobuf::ProtobufEnum;
use crate::ffi_runtime::arena::{Arena, bytes_from};
use crate::ffi_runtime::abi::{EraftpbConfChange, EraftpbConfChangeSingle, EraftpbConfChangeV2, EraftpbConfState, EraftpbEntry, EraftpbHardState, EraftpbMessage, EraftpbSnapshot, EraftpbSnapshotMetadata, KvprotoBytesView, KvprotoSliceEraftpbConfChangeSinglePtr, KvprotoSliceEraftpbEntryPtr, KvprotoSliceUint64T};
use raft_proto::eraftpb as pb;

pub fn conf_change_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ConfChange) -> &'a mut EraftpbConfChange {
    let mut repr = EraftpbConfChange {
        id: Default::default(),
        change_type: Default::default(),
        node_id: Default::default(),
        context: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    repr.id = src.get_id();
    repr.change_type = src.get_change_type() as i32;
    repr.node_id = src.get_node_id();
    if !src.get_context().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_context());
        repr.context.data = ptr;
        repr.context.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn conf_change_from_repr_generated(src: *const EraftpbConfChange) -> Option<pb::ConfChange> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ConfChange::new();
    out.set_id(repr.id);
    out.set_change_type(pb::ConfChangeType::from_i32(repr.change_type).unwrap_or_default());
    out.set_node_id(repr.node_id);
    out.set_context(bytes_from(repr.context.data, repr.context.len).into());
    Some(out)
}

pub fn conf_change_single_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ConfChangeSingle) -> &'a mut EraftpbConfChangeSingle {
    let mut repr = EraftpbConfChangeSingle {
        change_type: Default::default(),
        node_id: Default::default(),
    };
    repr.change_type = src.get_change_type() as i32;
    repr.node_id = src.get_node_id();
    arena.alloc_struct(repr)
}

pub fn conf_change_single_from_repr_generated(src: *const EraftpbConfChangeSingle) -> Option<pb::ConfChangeSingle> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ConfChangeSingle::new();
    out.set_change_type(pb::ConfChangeType::from_i32(repr.change_type).unwrap_or_default());
    out.set_node_id(repr.node_id);
    Some(out)
}

pub fn conf_change_v2_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ConfChangeV2) -> &'a mut EraftpbConfChangeV2 {
    let mut repr = EraftpbConfChangeV2 {
        transition: Default::default(),
        changes: KvprotoSliceEraftpbConfChangeSinglePtr { data: ptr::null_mut(), len: 0, cap: 0 },
        context: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    repr.transition = src.get_transition() as i32;
    {
        let values = src.get_changes();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut EraftpbConfChangeSingle> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(conf_change_single_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.changes.data = ptr;
                repr.changes.len = len;
                repr.changes.cap = len;
            }
        }
    }
    if !src.get_context().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_context());
        repr.context.data = ptr;
        repr.context.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn conf_change_v2_from_repr_generated(src: *const EraftpbConfChangeV2) -> Option<pb::ConfChangeV2> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ConfChangeV2::new();
    out.set_transition(pb::ConfChangeTransition::from_i32(repr.transition).unwrap_or_default());
    if !repr.changes.data.is_null() && repr.changes.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.changes.data, repr.changes.len) };
        let mut values: Vec<pb::ConfChangeSingle> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = conf_change_single_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_changes(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_context(bytes_from(repr.context.data, repr.context.len).into());
    Some(out)
}

pub fn conf_state_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ConfState) -> &'a mut EraftpbConfState {
    let mut repr = EraftpbConfState {
        voters: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        learners: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        voters_outgoing: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        learners_next: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        auto_leave: Default::default(),
    };
    {
        let values = src.get_voters();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.voters.data = ptr;
            repr.voters.len = len;
            repr.voters.cap = len;
        }
    }
    {
        let values = src.get_learners();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.learners.data = ptr;
            repr.learners.len = len;
            repr.learners.cap = len;
        }
    }
    {
        let values = src.get_voters_outgoing();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.voters_outgoing.data = ptr;
            repr.voters_outgoing.len = len;
            repr.voters_outgoing.cap = len;
        }
    }
    {
        let values = src.get_learners_next();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.learners_next.data = ptr;
            repr.learners_next.len = len;
            repr.learners_next.cap = len;
        }
    }
    repr.auto_leave = src.get_auto_leave();
    arena.alloc_struct(repr)
}

pub fn conf_state_from_repr_generated(src: *const EraftpbConfState) -> Option<pb::ConfState> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ConfState::new();
    if !repr.voters.data.is_null() && repr.voters.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.voters.data, repr.voters.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_voters(values);
    }
    if !repr.learners.data.is_null() && repr.learners.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.learners.data, repr.learners.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_learners(values);
    }
    if !repr.voters_outgoing.data.is_null() && repr.voters_outgoing.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.voters_outgoing.data, repr.voters_outgoing.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_voters_outgoing(values);
    }
    if !repr.learners_next.data.is_null() && repr.learners_next.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.learners_next.data, repr.learners_next.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_learners_next(values);
    }
    out.set_auto_leave(repr.auto_leave);
    Some(out)
}

pub fn entry_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Entry) -> &'a mut EraftpbEntry {
    let mut repr = EraftpbEntry {
        entry_type: Default::default(),
        term: Default::default(),
        index: Default::default(),
        data: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        sync_log: Default::default(),
        context: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    repr.entry_type = src.get_entry_type() as i32;
    repr.term = src.get_term();
    repr.index = src.get_index();
    if !src.get_data().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_data());
        repr.data.data = ptr;
        repr.data.len = len;
    }
    repr.sync_log = src.get_sync_log();
    if !src.get_context().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_context());
        repr.context.data = ptr;
        repr.context.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn entry_from_repr_generated(src: *const EraftpbEntry) -> Option<pb::Entry> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Entry::new();
    out.set_entry_type(pb::EntryType::from_i32(repr.entry_type).unwrap_or_default());
    out.set_term(repr.term);
    out.set_index(repr.index);
    out.set_data(bytes_from(repr.data.data, repr.data.len).into());
    out.set_sync_log(repr.sync_log);
    out.set_context(bytes_from(repr.context.data, repr.context.len).into());
    Some(out)
}

pub fn hard_state_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::HardState) -> &'a mut EraftpbHardState {
    let mut repr = EraftpbHardState {
        term: Default::default(),
        vote: Default::default(),
        commit: Default::default(),
    };
    repr.term = src.get_term();
    repr.vote = src.get_vote();
    repr.commit = src.get_commit();
    arena.alloc_struct(repr)
}

pub fn hard_state_from_repr_generated(src: *const EraftpbHardState) -> Option<pb::HardState> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::HardState::new();
    out.set_term(repr.term);
    out.set_vote(repr.vote);
    out.set_commit(repr.commit);
    Some(out)
}

pub fn message_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Message) -> &'a mut EraftpbMessage {
    let mut repr = EraftpbMessage {
        msg_type: Default::default(),
        to: Default::default(),
        from: Default::default(),
        term: Default::default(),
        log_term: Default::default(),
        index: Default::default(),
        entries: KvprotoSliceEraftpbEntryPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        commit: Default::default(),
        snapshot: ptr::null_mut(),
        reject: Default::default(),
        reject_hint: Default::default(),
        context: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        request_snapshot: Default::default(),
        deprecated_priority: Default::default(),
        priority: Default::default(),
    };
    repr.msg_type = src.get_msg_type() as i32;
    repr.to = src.get_to();
    repr.from = src.get_from();
    repr.term = src.get_term();
    repr.log_term = src.get_log_term();
    repr.index = src.get_index();
    {
        let values = src.get_entries();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut EraftpbEntry> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(entry_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.entries.data = ptr;
                repr.entries.len = len;
                repr.entries.cap = len;
            }
        }
    }
    repr.commit = src.get_commit();
    if src.has_snapshot() {
        repr.snapshot = snapshot_to_repr_generated(arena, src.get_snapshot()) as *mut _;
    } else {
        repr.snapshot = ptr::null_mut();
    }
    repr.reject = src.get_reject();
    repr.reject_hint = src.get_reject_hint();
    if !src.get_context().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_context());
        repr.context.data = ptr;
        repr.context.len = len;
    }
    repr.request_snapshot = src.get_request_snapshot();
    repr.deprecated_priority = src.get_priority().try_into().unwrap_or_default();
    repr.priority = src.get_priority().try_into().unwrap_or_default();
    arena.alloc_struct(repr)
}

pub fn message_from_repr_generated(src: *const EraftpbMessage) -> Option<pb::Message> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Message::new();
    out.set_msg_type(pb::MessageType::from_i32(repr.msg_type).unwrap_or_default());
    out.set_to(repr.to);
    out.set_from(repr.from);
    out.set_term(repr.term);
    out.set_log_term(repr.log_term);
    out.set_index(repr.index);
    if !repr.entries.data.is_null() && repr.entries.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.entries.data, repr.entries.len) };
        let mut values: Vec<pb::Entry> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = entry_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_entries(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_commit(repr.commit);
    if !repr.snapshot.is_null() {
        if let Some(value) = snapshot_from_repr_generated(repr.snapshot) {
            out.set_snapshot(value);
        }
    }
    out.set_reject(repr.reject);
    out.set_reject_hint(repr.reject_hint);
    out.set_context(bytes_from(repr.context.data, repr.context.len).into());
    out.set_request_snapshot(repr.request_snapshot);
    out.set_priority(repr.deprecated_priority.try_into().unwrap_or_default());
    out.set_priority(repr.priority.try_into().unwrap_or_default());
    Some(out)
}

pub fn snapshot_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Snapshot) -> &'a mut EraftpbSnapshot {
    let mut repr = EraftpbSnapshot {
        data: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        metadata: ptr::null_mut(),
    };
    if !src.get_data().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_data());
        repr.data.data = ptr;
        repr.data.len = len;
    }
    if src.has_metadata() {
        repr.metadata = snapshot_metadata_to_repr_generated(arena, src.get_metadata()) as *mut _;
    } else {
        repr.metadata = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn snapshot_from_repr_generated(src: *const EraftpbSnapshot) -> Option<pb::Snapshot> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Snapshot::new();
    out.set_data(bytes_from(repr.data.data, repr.data.len).into());
    if !repr.metadata.is_null() {
        if let Some(value) = snapshot_metadata_from_repr_generated(repr.metadata) {
            out.set_metadata(value);
        }
    }
    Some(out)
}

pub fn snapshot_metadata_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::SnapshotMetadata) -> &'a mut EraftpbSnapshotMetadata {
    let mut repr = EraftpbSnapshotMetadata {
        conf_state: ptr::null_mut(),
        index: Default::default(),
        term: Default::default(),
    };
    if src.has_conf_state() {
        repr.conf_state = conf_state_to_repr_generated(arena, src.get_conf_state()) as *mut _;
    } else {
        repr.conf_state = ptr::null_mut();
    }
    repr.index = src.get_index();
    repr.term = src.get_term();
    arena.alloc_struct(repr)
}

pub fn snapshot_metadata_from_repr_generated(src: *const EraftpbSnapshotMetadata) -> Option<pb::SnapshotMetadata> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::SnapshotMetadata::new();
    if !repr.conf_state.is_null() {
        if let Some(value) = conf_state_from_repr_generated(repr.conf_state) {
            out.set_conf_state(value);
        }
    }
    out.set_index(repr.index);
    out.set_term(repr.term);
    Some(out)
}

