//! Auto-generated conversions (feature `kvffi_gen`).
#![cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
#![allow(unused_imports, unused_variables, unused_mut, non_snake_case)]

use std::ptr;
use std::os::raw::c_char;

use protobuf::Message;
use protobuf::ProtobufEnum;
use crate::ffi_runtime::arena::{Arena, bytes_from, string_from};
use crate::ffi_runtime::abi::{ErrorpbBucketVersionNotMatch, ErrorpbDataIsNotReady, ErrorpbDiskFull, ErrorpbEpochNotMatch, ErrorpbError, ErrorpbFlashbackInProgress, ErrorpbFlashbackNotPrepared, ErrorpbIsWitness, ErrorpbKeyNotInRegion, ErrorpbMaxTimestampNotSynced, ErrorpbMismatchPeerId, ErrorpbNotLeader, ErrorpbProposalInMergingMode, ErrorpbRaftEntryTooLarge, ErrorpbReadIndexNotReady, ErrorpbRecoveryInProgress, ErrorpbRegionNotFound, ErrorpbRegionNotInitialized, ErrorpbServerIsBusy, ErrorpbStaleCommand, ErrorpbStoreNotMatch, ErrorpbUndeterminedResult, KvprotoBytesView, KvprotoSliceKvprotoBytesView, KvprotoSliceMetapbRegionPtr, KvprotoSliceUint64T, KvprotoStringView, MetapbPeer, MetapbRegion};
use crate::errorpb as pb;
use crate::metapb;

pub fn bucket_version_not_match_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BucketVersionNotMatch) -> &'a mut ErrorpbBucketVersionNotMatch {
    let mut repr = ErrorpbBucketVersionNotMatch {
        version: Default::default(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    repr.version = src.get_version();
    {
        let values = src.get_keys();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_bytes(value);
                views.push(KvprotoBytesView { data: ptr, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.keys.data = ptr;
                repr.keys.len = len;
                repr.keys.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn bucket_version_not_match_from_repr_generated(src: *const ErrorpbBucketVersionNotMatch) -> Option<pb::BucketVersionNotMatch> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BucketVersionNotMatch::new();
    out.set_version(repr.version);
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    Some(out)
}

pub fn data_is_not_ready_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::DataIsNotReady) -> &'a mut ErrorpbDataIsNotReady {
    let mut repr = ErrorpbDataIsNotReady {
        region_id: Default::default(),
        peer_id: Default::default(),
        safe_ts: Default::default(),
    };
    repr.region_id = src.get_region_id();
    repr.peer_id = src.get_peer_id();
    repr.safe_ts = src.get_safe_ts();
    arena.alloc_struct(repr)
}

pub fn data_is_not_ready_from_repr_generated(src: *const ErrorpbDataIsNotReady) -> Option<pb::DataIsNotReady> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::DataIsNotReady::new();
    out.set_region_id(repr.region_id);
    out.set_peer_id(repr.peer_id);
    out.set_safe_ts(repr.safe_ts);
    Some(out)
}

pub fn disk_full_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::DiskFull) -> &'a mut ErrorpbDiskFull {
    let mut repr = ErrorpbDiskFull {
        store_id: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        reason: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    {
        let values = src.get_store_id();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.store_id.data = ptr;
            repr.store_id.len = len;
            repr.store_id.cap = len;
        }
    }
    if !src.get_reason().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_reason());
        repr.reason.data = ptr as *const c_char;
        repr.reason.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn disk_full_from_repr_generated(src: *const ErrorpbDiskFull) -> Option<pb::DiskFull> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::DiskFull::new();
    if !repr.store_id.data.is_null() && repr.store_id.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.store_id.data, repr.store_id.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_store_id(values);
    }
    out.set_reason(string_from(repr.reason.data as *const u8, repr.reason.len));
    Some(out)
}

pub fn epoch_not_match_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::EpochNotMatch) -> &'a mut ErrorpbEpochNotMatch {
    let mut repr = ErrorpbEpochNotMatch {
        current_regions: KvprotoSliceMetapbRegionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_current_regions();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut MetapbRegion> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(crate::ffi_runtime::metapb::region_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.current_regions.data = ptr;
                repr.current_regions.len = len;
                repr.current_regions.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn epoch_not_match_from_repr_generated(src: *const ErrorpbEpochNotMatch) -> Option<pb::EpochNotMatch> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::EpochNotMatch::new();
    if !repr.current_regions.data.is_null() && repr.current_regions.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.current_regions.data, repr.current_regions.len) };
        let mut values: Vec<metapb::Region> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = crate::ffi_runtime::metapb::region_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_current_regions(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn error_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Error) -> &'a mut ErrorpbError {
    let mut repr = ErrorpbError {
        message: KvprotoStringView { data: ptr::null(), len: 0 },
        not_leader: ptr::null_mut(),
        region_not_found: ptr::null_mut(),
        key_not_in_region: ptr::null_mut(),
        epoch_not_match: ptr::null_mut(),
        server_is_busy: ptr::null_mut(),
        stale_command: ptr::null_mut(),
        store_not_match: ptr::null_mut(),
        raft_entry_too_large: ptr::null_mut(),
        max_timestamp_not_synced: ptr::null_mut(),
        read_index_not_ready: ptr::null_mut(),
        proposal_in_merging_mode: ptr::null_mut(),
        data_is_not_ready: ptr::null_mut(),
        region_not_initialized: ptr::null_mut(),
        disk_full: ptr::null_mut(),
        RecoveryInProgress: ptr::null_mut(),
        FlashbackInProgress: ptr::null_mut(),
        FlashbackNotPrepared: ptr::null_mut(),
        is_witness: ptr::null_mut(),
        mismatch_peer_id: ptr::null_mut(),
        bucket_version_not_match: ptr::null_mut(),
        undetermined_result: ptr::null_mut(),
    };
    if !src.get_message().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_message());
        repr.message.data = ptr as *const c_char;
        repr.message.len = len;
    }
    if src.has_not_leader() {
        repr.not_leader = not_leader_to_repr_generated(arena, src.get_not_leader()) as *mut _;
    } else {
        repr.not_leader = ptr::null_mut();
    }
    if src.has_region_not_found() {
        repr.region_not_found = region_not_found_to_repr_generated(arena, src.get_region_not_found()) as *mut _;
    } else {
        repr.region_not_found = ptr::null_mut();
    }
    if src.has_key_not_in_region() {
        repr.key_not_in_region = key_not_in_region_to_repr_generated(arena, src.get_key_not_in_region()) as *mut _;
    } else {
        repr.key_not_in_region = ptr::null_mut();
    }
    if src.has_epoch_not_match() {
        repr.epoch_not_match = epoch_not_match_to_repr_generated(arena, src.get_epoch_not_match()) as *mut _;
    } else {
        repr.epoch_not_match = ptr::null_mut();
    }
    if src.has_server_is_busy() {
        repr.server_is_busy = server_is_busy_to_repr_generated(arena, src.get_server_is_busy()) as *mut _;
    } else {
        repr.server_is_busy = ptr::null_mut();
    }
    if src.has_stale_command() {
        repr.stale_command = stale_command_to_repr_generated(arena, src.get_stale_command()) as *mut _;
    } else {
        repr.stale_command = ptr::null_mut();
    }
    if src.has_store_not_match() {
        repr.store_not_match = store_not_match_to_repr_generated(arena, src.get_store_not_match()) as *mut _;
    } else {
        repr.store_not_match = ptr::null_mut();
    }
    if src.has_raft_entry_too_large() {
        repr.raft_entry_too_large = raft_entry_too_large_to_repr_generated(arena, src.get_raft_entry_too_large()) as *mut _;
    } else {
        repr.raft_entry_too_large = ptr::null_mut();
    }
    if src.has_max_timestamp_not_synced() {
        repr.max_timestamp_not_synced = max_timestamp_not_synced_to_repr_generated(arena, src.get_max_timestamp_not_synced()) as *mut _;
    } else {
        repr.max_timestamp_not_synced = ptr::null_mut();
    }
    if src.has_read_index_not_ready() {
        repr.read_index_not_ready = read_index_not_ready_to_repr_generated(arena, src.get_read_index_not_ready()) as *mut _;
    } else {
        repr.read_index_not_ready = ptr::null_mut();
    }
    if src.has_proposal_in_merging_mode() {
        repr.proposal_in_merging_mode = proposal_in_merging_mode_to_repr_generated(arena, src.get_proposal_in_merging_mode()) as *mut _;
    } else {
        repr.proposal_in_merging_mode = ptr::null_mut();
    }
    if src.has_data_is_not_ready() {
        repr.data_is_not_ready = data_is_not_ready_to_repr_generated(arena, src.get_data_is_not_ready()) as *mut _;
    } else {
        repr.data_is_not_ready = ptr::null_mut();
    }
    if src.has_region_not_initialized() {
        repr.region_not_initialized = region_not_initialized_to_repr_generated(arena, src.get_region_not_initialized()) as *mut _;
    } else {
        repr.region_not_initialized = ptr::null_mut();
    }
    if src.has_disk_full() {
        repr.disk_full = disk_full_to_repr_generated(arena, src.get_disk_full()) as *mut _;
    } else {
        repr.disk_full = ptr::null_mut();
    }
    if src.has_recovery_in_progress() {
        repr.RecoveryInProgress = recovery_in_progress_to_repr_generated(arena, src.get_recovery_in_progress()) as *mut _;
    } else {
        repr.RecoveryInProgress = ptr::null_mut();
    }
    if src.has_flashback_in_progress() {
        repr.FlashbackInProgress = flashback_in_progress_to_repr_generated(arena, src.get_flashback_in_progress()) as *mut _;
    } else {
        repr.FlashbackInProgress = ptr::null_mut();
    }
    if src.has_flashback_not_prepared() {
        repr.FlashbackNotPrepared = flashback_not_prepared_to_repr_generated(arena, src.get_flashback_not_prepared()) as *mut _;
    } else {
        repr.FlashbackNotPrepared = ptr::null_mut();
    }
    if src.has_is_witness() {
        repr.is_witness = is_witness_to_repr_generated(arena, src.get_is_witness()) as *mut _;
    } else {
        repr.is_witness = ptr::null_mut();
    }
    if src.has_mismatch_peer_id() {
        repr.mismatch_peer_id = mismatch_peer_id_to_repr_generated(arena, src.get_mismatch_peer_id()) as *mut _;
    } else {
        repr.mismatch_peer_id = ptr::null_mut();
    }
    if src.has_bucket_version_not_match() {
        repr.bucket_version_not_match = bucket_version_not_match_to_repr_generated(arena, src.get_bucket_version_not_match()) as *mut _;
    } else {
        repr.bucket_version_not_match = ptr::null_mut();
    }
    if src.has_undetermined_result() {
        repr.undetermined_result = undetermined_result_to_repr_generated(arena, src.get_undetermined_result()) as *mut _;
    } else {
        repr.undetermined_result = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn error_from_repr_generated(src: *const ErrorpbError) -> Option<pb::Error> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Error::new();
    out.set_message(string_from(repr.message.data as *const u8, repr.message.len));
    if !repr.not_leader.is_null() {
        if let Some(value) = not_leader_from_repr_generated(repr.not_leader) {
            out.set_not_leader(value);
        }
    }
    if !repr.region_not_found.is_null() {
        if let Some(value) = region_not_found_from_repr_generated(repr.region_not_found) {
            out.set_region_not_found(value);
        }
    }
    if !repr.key_not_in_region.is_null() {
        if let Some(value) = key_not_in_region_from_repr_generated(repr.key_not_in_region) {
            out.set_key_not_in_region(value);
        }
    }
    if !repr.epoch_not_match.is_null() {
        if let Some(value) = epoch_not_match_from_repr_generated(repr.epoch_not_match) {
            out.set_epoch_not_match(value);
        }
    }
    if !repr.server_is_busy.is_null() {
        if let Some(value) = server_is_busy_from_repr_generated(repr.server_is_busy) {
            out.set_server_is_busy(value);
        }
    }
    if !repr.stale_command.is_null() {
        if let Some(value) = stale_command_from_repr_generated(repr.stale_command) {
            out.set_stale_command(value);
        }
    }
    if !repr.store_not_match.is_null() {
        if let Some(value) = store_not_match_from_repr_generated(repr.store_not_match) {
            out.set_store_not_match(value);
        }
    }
    if !repr.raft_entry_too_large.is_null() {
        if let Some(value) = raft_entry_too_large_from_repr_generated(repr.raft_entry_too_large) {
            out.set_raft_entry_too_large(value);
        }
    }
    if !repr.max_timestamp_not_synced.is_null() {
        if let Some(value) = max_timestamp_not_synced_from_repr_generated(repr.max_timestamp_not_synced) {
            out.set_max_timestamp_not_synced(value);
        }
    }
    if !repr.read_index_not_ready.is_null() {
        if let Some(value) = read_index_not_ready_from_repr_generated(repr.read_index_not_ready) {
            out.set_read_index_not_ready(value);
        }
    }
    if !repr.proposal_in_merging_mode.is_null() {
        if let Some(value) = proposal_in_merging_mode_from_repr_generated(repr.proposal_in_merging_mode) {
            out.set_proposal_in_merging_mode(value);
        }
    }
    if !repr.data_is_not_ready.is_null() {
        if let Some(value) = data_is_not_ready_from_repr_generated(repr.data_is_not_ready) {
            out.set_data_is_not_ready(value);
        }
    }
    if !repr.region_not_initialized.is_null() {
        if let Some(value) = region_not_initialized_from_repr_generated(repr.region_not_initialized) {
            out.set_region_not_initialized(value);
        }
    }
    if !repr.disk_full.is_null() {
        if let Some(value) = disk_full_from_repr_generated(repr.disk_full) {
            out.set_disk_full(value);
        }
    }
    if !repr.RecoveryInProgress.is_null() {
        if let Some(value) = recovery_in_progress_from_repr_generated(repr.RecoveryInProgress) {
            out.set_recovery_in_progress(value);
        }
    }
    if !repr.FlashbackInProgress.is_null() {
        if let Some(value) = flashback_in_progress_from_repr_generated(repr.FlashbackInProgress) {
            out.set_flashback_in_progress(value);
        }
    }
    if !repr.FlashbackNotPrepared.is_null() {
        if let Some(value) = flashback_not_prepared_from_repr_generated(repr.FlashbackNotPrepared) {
            out.set_flashback_not_prepared(value);
        }
    }
    if !repr.is_witness.is_null() {
        if let Some(value) = is_witness_from_repr_generated(repr.is_witness) {
            out.set_is_witness(value);
        }
    }
    if !repr.mismatch_peer_id.is_null() {
        if let Some(value) = mismatch_peer_id_from_repr_generated(repr.mismatch_peer_id) {
            out.set_mismatch_peer_id(value);
        }
    }
    if !repr.bucket_version_not_match.is_null() {
        if let Some(value) = bucket_version_not_match_from_repr_generated(repr.bucket_version_not_match) {
            out.set_bucket_version_not_match(value);
        }
    }
    if !repr.undetermined_result.is_null() {
        if let Some(value) = undetermined_result_from_repr_generated(repr.undetermined_result) {
            out.set_undetermined_result(value);
        }
    }
    Some(out)
}

pub fn flashback_in_progress_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::FlashbackInProgress) -> &'a mut ErrorpbFlashbackInProgress {
    let mut repr = ErrorpbFlashbackInProgress {
        region_id: Default::default(),
        flashback_start_ts: Default::default(),
    };
    repr.region_id = src.get_region_id();
    repr.flashback_start_ts = src.get_flashback_start_ts();
    arena.alloc_struct(repr)
}

pub fn flashback_in_progress_from_repr_generated(src: *const ErrorpbFlashbackInProgress) -> Option<pb::FlashbackInProgress> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::FlashbackInProgress::new();
    out.set_region_id(repr.region_id);
    out.set_flashback_start_ts(repr.flashback_start_ts);
    Some(out)
}

pub fn flashback_not_prepared_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::FlashbackNotPrepared) -> &'a mut ErrorpbFlashbackNotPrepared {
    let mut repr = ErrorpbFlashbackNotPrepared {
        region_id: Default::default(),
    };
    repr.region_id = src.get_region_id();
    arena.alloc_struct(repr)
}

pub fn flashback_not_prepared_from_repr_generated(src: *const ErrorpbFlashbackNotPrepared) -> Option<pb::FlashbackNotPrepared> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::FlashbackNotPrepared::new();
    out.set_region_id(repr.region_id);
    Some(out)
}

pub fn is_witness_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::IsWitness) -> &'a mut ErrorpbIsWitness {
    let mut repr = ErrorpbIsWitness {
        region_id: Default::default(),
    };
    repr.region_id = src.get_region_id();
    arena.alloc_struct(repr)
}

pub fn is_witness_from_repr_generated(src: *const ErrorpbIsWitness) -> Option<pb::IsWitness> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::IsWitness::new();
    out.set_region_id(repr.region_id);
    Some(out)
}

pub fn key_not_in_region_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::KeyNotInRegion) -> &'a mut ErrorpbKeyNotInRegion {
    let mut repr = ErrorpbKeyNotInRegion {
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        region_id: Default::default(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    repr.region_id = src.get_region_id();
    if !src.get_start_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_start_key());
        repr.start_key.data = ptr;
        repr.start_key.len = len;
    }
    if !src.get_end_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_end_key());
        repr.end_key.data = ptr;
        repr.end_key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn key_not_in_region_from_repr_generated(src: *const ErrorpbKeyNotInRegion) -> Option<pb::KeyNotInRegion> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::KeyNotInRegion::new();
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_region_id(repr.region_id);
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    Some(out)
}

pub fn max_timestamp_not_synced_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MaxTimestampNotSynced) -> &'a mut ErrorpbMaxTimestampNotSynced {
    let mut repr = ErrorpbMaxTimestampNotSynced {
    };
    arena.alloc_struct(repr)
}

pub fn max_timestamp_not_synced_from_repr_generated(src: *const ErrorpbMaxTimestampNotSynced) -> Option<pb::MaxTimestampNotSynced> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MaxTimestampNotSynced::new();
    Some(out)
}

pub fn mismatch_peer_id_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MismatchPeerId) -> &'a mut ErrorpbMismatchPeerId {
    let mut repr = ErrorpbMismatchPeerId {
        request_peer_id: Default::default(),
        store_peer_id: Default::default(),
    };
    repr.request_peer_id = src.get_request_peer_id();
    repr.store_peer_id = src.get_store_peer_id();
    arena.alloc_struct(repr)
}

pub fn mismatch_peer_id_from_repr_generated(src: *const ErrorpbMismatchPeerId) -> Option<pb::MismatchPeerId> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MismatchPeerId::new();
    out.set_request_peer_id(repr.request_peer_id);
    out.set_store_peer_id(repr.store_peer_id);
    Some(out)
}

pub fn not_leader_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::NotLeader) -> &'a mut ErrorpbNotLeader {
    let mut repr = ErrorpbNotLeader {
        region_id: Default::default(),
        leader: ptr::null_mut(),
    };
    repr.region_id = src.get_region_id();
    if src.has_leader() {
        repr.leader = crate::ffi_runtime::metapb::peer_to_repr_generated(arena, src.get_leader()) as *mut _;
    } else {
        repr.leader = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn not_leader_from_repr_generated(src: *const ErrorpbNotLeader) -> Option<pb::NotLeader> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::NotLeader::new();
    out.set_region_id(repr.region_id);
    if !repr.leader.is_null() {
        if let Some(value) = crate::ffi_runtime::metapb::peer_from_repr_generated(repr.leader) {
            out.set_leader(value);
        }
    }
    Some(out)
}

pub fn proposal_in_merging_mode_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ProposalInMergingMode) -> &'a mut ErrorpbProposalInMergingMode {
    let mut repr = ErrorpbProposalInMergingMode {
        region_id: Default::default(),
    };
    repr.region_id = src.get_region_id();
    arena.alloc_struct(repr)
}

pub fn proposal_in_merging_mode_from_repr_generated(src: *const ErrorpbProposalInMergingMode) -> Option<pb::ProposalInMergingMode> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ProposalInMergingMode::new();
    out.set_region_id(repr.region_id);
    Some(out)
}

pub fn raft_entry_too_large_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RaftEntryTooLarge) -> &'a mut ErrorpbRaftEntryTooLarge {
    let mut repr = ErrorpbRaftEntryTooLarge {
        region_id: Default::default(),
        entry_size: Default::default(),
    };
    repr.region_id = src.get_region_id();
    repr.entry_size = src.get_entry_size();
    arena.alloc_struct(repr)
}

pub fn raft_entry_too_large_from_repr_generated(src: *const ErrorpbRaftEntryTooLarge) -> Option<pb::RaftEntryTooLarge> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RaftEntryTooLarge::new();
    out.set_region_id(repr.region_id);
    out.set_entry_size(repr.entry_size);
    Some(out)
}

pub fn read_index_not_ready_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ReadIndexNotReady) -> &'a mut ErrorpbReadIndexNotReady {
    let mut repr = ErrorpbReadIndexNotReady {
        reason: KvprotoStringView { data: ptr::null(), len: 0 },
        region_id: Default::default(),
    };
    if !src.get_reason().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_reason());
        repr.reason.data = ptr as *const c_char;
        repr.reason.len = len;
    }
    repr.region_id = src.get_region_id();
    arena.alloc_struct(repr)
}

pub fn read_index_not_ready_from_repr_generated(src: *const ErrorpbReadIndexNotReady) -> Option<pb::ReadIndexNotReady> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ReadIndexNotReady::new();
    out.set_reason(string_from(repr.reason.data as *const u8, repr.reason.len));
    out.set_region_id(repr.region_id);
    Some(out)
}

pub fn recovery_in_progress_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RecoveryInProgress) -> &'a mut ErrorpbRecoveryInProgress {
    let mut repr = ErrorpbRecoveryInProgress {
        region_id: Default::default(),
    };
    repr.region_id = src.get_region_id();
    arena.alloc_struct(repr)
}

pub fn recovery_in_progress_from_repr_generated(src: *const ErrorpbRecoveryInProgress) -> Option<pb::RecoveryInProgress> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RecoveryInProgress::new();
    out.set_region_id(repr.region_id);
    Some(out)
}

pub fn region_not_found_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RegionNotFound) -> &'a mut ErrorpbRegionNotFound {
    let mut repr = ErrorpbRegionNotFound {
        region_id: Default::default(),
    };
    repr.region_id = src.get_region_id();
    arena.alloc_struct(repr)
}

pub fn region_not_found_from_repr_generated(src: *const ErrorpbRegionNotFound) -> Option<pb::RegionNotFound> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RegionNotFound::new();
    out.set_region_id(repr.region_id);
    Some(out)
}

pub fn region_not_initialized_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RegionNotInitialized) -> &'a mut ErrorpbRegionNotInitialized {
    let mut repr = ErrorpbRegionNotInitialized {
        region_id: Default::default(),
    };
    repr.region_id = src.get_region_id();
    arena.alloc_struct(repr)
}

pub fn region_not_initialized_from_repr_generated(src: *const ErrorpbRegionNotInitialized) -> Option<pb::RegionNotInitialized> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RegionNotInitialized::new();
    out.set_region_id(repr.region_id);
    Some(out)
}

pub fn server_is_busy_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ServerIsBusy) -> &'a mut ErrorpbServerIsBusy {
    let mut repr = ErrorpbServerIsBusy {
        reason: KvprotoStringView { data: ptr::null(), len: 0 },
        backoff_ms: Default::default(),
        estimated_wait_ms: Default::default(),
        applied_index: Default::default(),
    };
    if !src.get_reason().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_reason());
        repr.reason.data = ptr as *const c_char;
        repr.reason.len = len;
    }
    repr.backoff_ms = src.get_backoff_ms();
    repr.estimated_wait_ms = src.get_estimated_wait_ms();
    repr.applied_index = src.get_applied_index();
    arena.alloc_struct(repr)
}

pub fn server_is_busy_from_repr_generated(src: *const ErrorpbServerIsBusy) -> Option<pb::ServerIsBusy> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ServerIsBusy::new();
    out.set_reason(string_from(repr.reason.data as *const u8, repr.reason.len));
    out.set_backoff_ms(repr.backoff_ms);
    out.set_estimated_wait_ms(repr.estimated_wait_ms);
    out.set_applied_index(repr.applied_index);
    Some(out)
}

pub fn stale_command_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::StaleCommand) -> &'a mut ErrorpbStaleCommand {
    let mut repr = ErrorpbStaleCommand {
    };
    arena.alloc_struct(repr)
}

pub fn stale_command_from_repr_generated(src: *const ErrorpbStaleCommand) -> Option<pb::StaleCommand> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::StaleCommand::new();
    Some(out)
}

pub fn store_not_match_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::StoreNotMatch) -> &'a mut ErrorpbStoreNotMatch {
    let mut repr = ErrorpbStoreNotMatch {
        request_store_id: Default::default(),
        actual_store_id: Default::default(),
    };
    repr.request_store_id = src.get_request_store_id();
    repr.actual_store_id = src.get_actual_store_id();
    arena.alloc_struct(repr)
}

pub fn store_not_match_from_repr_generated(src: *const ErrorpbStoreNotMatch) -> Option<pb::StoreNotMatch> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::StoreNotMatch::new();
    out.set_request_store_id(repr.request_store_id);
    out.set_actual_store_id(repr.actual_store_id);
    Some(out)
}

pub fn undetermined_result_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::UndeterminedResult) -> &'a mut ErrorpbUndeterminedResult {
    let mut repr = ErrorpbUndeterminedResult {
        message: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_message().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_message());
        repr.message.data = ptr as *const c_char;
        repr.message.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn undetermined_result_from_repr_generated(src: *const ErrorpbUndeterminedResult) -> Option<pb::UndeterminedResult> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::UndeterminedResult::new();
    out.set_message(string_from(repr.message.data as *const u8, repr.message.len));
    Some(out)
}

