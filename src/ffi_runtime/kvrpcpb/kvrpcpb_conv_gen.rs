//! Auto-generated conversions (feature `kvffi_gen`).
#![cfg(feature = "kvffi_gen")]

use std::ptr;
use std::os::raw::c_char;

use protobuf::ProtobufEnum;
use crate::ffi_runtime::arena::{Arena, bytes_from, string_from};
use crate::ffi_runtime::abi::{DeadlockWaitForEntry, ErrorpbError, KvprotoBytesView, KvprotoSliceBool, KvprotoSliceDeadlockWaitForEntryPtr, KvprotoSliceInt32T, KvprotoSliceKvprotoBytesView, KvprotoSliceKvrpcpbKeyErrorPtr, KvprotoSliceKvrpcpbKeyRangePtr, KvprotoSliceKvrpcpbKvPairPtr, KvprotoSliceKvrpcpbLeaderInfoPtr, KvprotoSliceKvrpcpbLockInfoPtr, KvprotoSliceKvrpcpbMutationPtr, KvprotoSliceKvrpcpbMvccDebugInfoPtr, KvprotoSliceKvrpcpbMvccValuePtr, KvprotoSliceKvrpcpbMvccWritePtr, KvprotoSliceKvrpcpbPessimisticLockKeyResultPtr, KvprotoSliceKvrpcpbPrewriteRequestForUpdateTSConstraintPtr, KvprotoSliceKvrpcpbTxnInfoPtr, KvprotoSliceKvrpcpbTxnStatusPtr, KvprotoSliceMetapbRegionPtr, KvprotoSliceUint64T, KvprotoStringView, KvrpcpbAlreadyExist, KvrpcpbAssertionFailed, KvrpcpbBatchGetRequest, KvrpcpbBatchGetResponse, KvrpcpbBatchRollbackRequest, KvrpcpbBatchRollbackResponse, KvrpcpbBroadcastTxnStatusRequest, KvrpcpbBroadcastTxnStatusResponse, KvrpcpbBufferBatchGetRequest, KvrpcpbBufferBatchGetResponse, KvrpcpbCheckLeaderRequest, KvrpcpbCheckLeaderResponse, KvrpcpbCheckLockObserverRequest, KvrpcpbCheckLockObserverResponse, KvrpcpbCheckSecondaryLocksRequest, KvrpcpbCheckSecondaryLocksResponse, KvrpcpbCheckTxnStatusRequest, KvrpcpbCheckTxnStatusResponse, KvrpcpbCleanupRequest, KvrpcpbCleanupResponse, KvrpcpbCommitRequest, KvrpcpbCommitResponse, KvrpcpbCommitTsExpired, KvrpcpbCommitTsTooLarge, KvrpcpbCompactError, KvrpcpbCompactErrorCompactInProgress, KvrpcpbCompactErrorInvalidStartKey, KvrpcpbCompactErrorPhysicalTableNotExist, KvrpcpbCompactErrorTooManyPendingTasks, KvrpcpbCompactRequest, KvrpcpbCompactResponse, KvrpcpbContext, KvrpcpbDeadlock, KvrpcpbDebugInfo, KvrpcpbDeleteRangeRequest, KvrpcpbDeleteRangeResponse, KvrpcpbExecDetails, KvrpcpbExecDetailsV2, KvrpcpbFlashbackToVersionRequest, KvrpcpbFlashbackToVersionResponse, KvrpcpbFlushRequest, KvrpcpbFlushResponse, KvrpcpbGCRequest, KvrpcpbGCResponse, KvrpcpbGetHealthFeedbackRequest, KvrpcpbGetHealthFeedbackResponse, KvrpcpbGetLockWaitHistoryRequest, KvrpcpbGetLockWaitHistoryResponse, KvrpcpbGetLockWaitInfoRequest, KvrpcpbGetLockWaitInfoResponse, KvrpcpbGetRequest, KvrpcpbGetResponse, KvrpcpbHealthFeedback, KvrpcpbImportRequest, KvrpcpbImportResponse, KvrpcpbKeyError, KvrpcpbKeyRange, KvrpcpbKvPair, KvrpcpbLeaderInfo, KvrpcpbLockInfo, KvrpcpbMutation, KvrpcpbMvccDebugInfo, KvrpcpbMvccGetByKeyRequest, KvrpcpbMvccGetByKeyResponse, KvrpcpbMvccGetByStartTsRequest, KvrpcpbMvccGetByStartTsResponse, KvrpcpbMvccInfo, KvrpcpbMvccLock, KvrpcpbMvccValue, KvrpcpbMvccWrite, KvrpcpbPessimisticLockKeyResult, KvrpcpbPessimisticLockRequest, KvrpcpbPessimisticLockResponse, KvrpcpbPessimisticRollbackRequest, KvrpcpbPessimisticRollbackResponse, KvrpcpbPhysicalScanLockRequest, KvrpcpbPhysicalScanLockResponse, KvrpcpbPrepareFlashbackToVersionRequest, KvrpcpbPrepareFlashbackToVersionResponse, KvrpcpbPrewriteRequest, KvrpcpbPrewriteRequestForUpdateTSConstraint, KvrpcpbPrewriteResponse, KvrpcpbPrimaryMismatch, KvrpcpbRawBatchDeleteRequest, KvrpcpbRawBatchDeleteResponse, KvrpcpbRawBatchGetRequest, KvrpcpbRawBatchGetResponse, KvrpcpbRawBatchPutRequest, KvrpcpbRawBatchPutResponse, KvrpcpbRawBatchScanRequest, KvrpcpbRawBatchScanResponse, KvrpcpbRawCASRequest, KvrpcpbRawCASResponse, KvrpcpbRawChecksumRequest, KvrpcpbRawChecksumResponse, KvrpcpbRawCoprocessorRequest, KvrpcpbRawCoprocessorResponse, KvrpcpbRawDeleteRangeRequest, KvrpcpbRawDeleteRangeResponse, KvrpcpbRawDeleteRequest, KvrpcpbRawDeleteResponse, KvrpcpbRawGetKeyTTLRequest, KvrpcpbRawGetKeyTTLResponse, KvrpcpbRawGetRequest, KvrpcpbRawGetResponse, KvrpcpbRawPutRequest, KvrpcpbRawPutResponse, KvrpcpbRawScanRequest, KvrpcpbRawScanResponse, KvrpcpbReadIndexRequest, KvrpcpbReadIndexResponse, KvrpcpbReadState, KvrpcpbRegisterLockObserverRequest, KvrpcpbRegisterLockObserverResponse, KvrpcpbRemoveLockObserverRequest, KvrpcpbRemoveLockObserverResponse, KvrpcpbResolveLockRequest, KvrpcpbResolveLockResponse, KvrpcpbResourceControlContext, KvrpcpbScanDetail, KvrpcpbScanDetailV2, KvrpcpbScanInfo, KvrpcpbScanLockRequest, KvrpcpbScanLockResponse, KvrpcpbScanRequest, KvrpcpbScanResponse, KvrpcpbSourceStmt, KvrpcpbSplitRegionRequest, KvrpcpbSplitRegionResponse, KvrpcpbStoreSafeTSRequest, KvrpcpbStoreSafeTSResponse, KvrpcpbTiFlashSystemTableRequest, KvrpcpbTiFlashSystemTableResponse, KvrpcpbTimeDetail, KvrpcpbTimeDetailV2, KvrpcpbTxnHeartBeatRequest, KvrpcpbTxnHeartBeatResponse, KvrpcpbTxnInfo, KvrpcpbTxnLockNotFound, KvrpcpbTxnNotFound, KvrpcpbTxnStatus, KvrpcpbUnsafeDestroyRangeRequest, KvrpcpbUnsafeDestroyRangeResponse, KvrpcpbWriteConflict, KvrpcpbWriteDetail, MetapbPeer, MetapbRegion, MetapbRegionEpoch, ResourceManagerConsumption, TracepbTraceContext};
use crate::kvrpcpb as pb;
use crate::deadlock;
use crate::errorpb;
use crate::metapb;
use crate::resource_manager;
use crate::tracepb;

pub fn already_exist_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::AlreadyExist) -> &'a mut KvrpcpbAlreadyExist {
    let mut repr = KvrpcpbAlreadyExist {
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn already_exist_from_repr_generated(src: *const KvrpcpbAlreadyExist) -> Option<pb::AlreadyExist> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::AlreadyExist::new();
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    Some(out)
}

pub fn assertion_failed_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::AssertionFailed) -> &'a mut KvrpcpbAssertionFailed {
    let mut repr = KvrpcpbAssertionFailed {
        start_ts: Default::default(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        assertion: Default::default(),
        existing_start_ts: Default::default(),
        existing_commit_ts: Default::default(),
    };
    repr.start_ts = src.get_start_ts();
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    repr.assertion = src.get_assertion() as i32;
    repr.existing_start_ts = src.get_existing_start_ts();
    repr.existing_commit_ts = src.get_existing_commit_ts();
    arena.alloc_struct(repr)
}

pub fn assertion_failed_from_repr_generated(src: *const KvrpcpbAssertionFailed) -> Option<pb::AssertionFailed> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::AssertionFailed::new();
    out.set_start_ts(repr.start_ts);
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_assertion(pb::Assertion::from_i32(repr.assertion).unwrap_or_default());
    out.set_existing_start_ts(repr.existing_start_ts);
    out.set_existing_commit_ts(repr.existing_commit_ts);
    Some(out)
}

pub fn batch_get_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BatchGetRequest) -> &'a mut KvrpcpbBatchGetRequest {
    let mut repr = KvrpcpbBatchGetRequest {
        context: ptr::null_mut(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        version: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
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
    repr.version = src.get_version();
    arena.alloc_struct(repr)
}

pub fn batch_get_request_from_repr_generated(src: *const KvrpcpbBatchGetRequest) -> Option<pb::BatchGetRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BatchGetRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_version(repr.version);
    Some(out)
}

pub fn batch_get_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BatchGetResponse) -> &'a mut KvrpcpbBatchGetResponse {
    let mut repr = KvrpcpbBatchGetResponse {
        region_error: ptr::null_mut(),
        pairs: KvprotoSliceKvrpcpbKvPairPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        exec_details_v2: ptr::null_mut(),
        error: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    {
        let values = src.get_pairs();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKvPair> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(kv_pair_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.pairs.data = ptr;
                repr.pairs.len = len;
                repr.pairs.cap = len;
            }
        }
    }
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn batch_get_response_from_repr_generated(src: *const KvrpcpbBatchGetResponse) -> Option<pb::BatchGetResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BatchGetResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.pairs.data.is_null() && repr.pairs.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.pairs.data, repr.pairs.len) };
        let mut values: Vec<pb::KvPair> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = kv_pair_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_pairs(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    Some(out)
}

pub fn batch_rollback_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BatchRollbackRequest) -> &'a mut KvrpcpbBatchRollbackRequest {
    let mut repr = KvrpcpbBatchRollbackRequest {
        context: ptr::null_mut(),
        start_version: Default::default(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        is_txn_file: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.start_version = src.get_start_version();
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
    repr.is_txn_file = src.get_is_txn_file();
    arena.alloc_struct(repr)
}

pub fn batch_rollback_request_from_repr_generated(src: *const KvrpcpbBatchRollbackRequest) -> Option<pb::BatchRollbackRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BatchRollbackRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_version(repr.start_version);
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_is_txn_file(repr.is_txn_file);
    Some(out)
}

pub fn batch_rollback_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BatchRollbackResponse) -> &'a mut KvrpcpbBatchRollbackResponse {
    let mut repr = KvrpcpbBatchRollbackResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn batch_rollback_response_from_repr_generated(src: *const KvrpcpbBatchRollbackResponse) -> Option<pb::BatchRollbackResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BatchRollbackResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn broadcast_txn_status_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BroadcastTxnStatusRequest) -> &'a mut KvrpcpbBroadcastTxnStatusRequest {
    let mut repr = KvrpcpbBroadcastTxnStatusRequest {
        context: ptr::null_mut(),
        txn_status: KvprotoSliceKvrpcpbTxnStatusPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    {
        let values = src.get_txn_status();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbTxnStatus> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(txn_status_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.txn_status.data = ptr;
                repr.txn_status.len = len;
                repr.txn_status.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn broadcast_txn_status_request_from_repr_generated(src: *const KvrpcpbBroadcastTxnStatusRequest) -> Option<pb::BroadcastTxnStatusRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BroadcastTxnStatusRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.txn_status.data.is_null() && repr.txn_status.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.txn_status.data, repr.txn_status.len) };
        let mut values: Vec<pb::TxnStatus> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = txn_status_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_txn_status(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn broadcast_txn_status_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BroadcastTxnStatusResponse) -> &'a mut KvrpcpbBroadcastTxnStatusResponse {
    let mut repr = KvrpcpbBroadcastTxnStatusResponse {
    };
    arena.alloc_struct(repr)
}

pub fn broadcast_txn_status_response_from_repr_generated(src: *const KvrpcpbBroadcastTxnStatusResponse) -> Option<pb::BroadcastTxnStatusResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BroadcastTxnStatusResponse::new();
    Some(out)
}

pub fn buffer_batch_get_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BufferBatchGetRequest) -> &'a mut KvrpcpbBufferBatchGetRequest {
    let mut repr = KvrpcpbBufferBatchGetRequest {
        context: ptr::null_mut(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        version: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
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
    repr.version = src.get_version();
    arena.alloc_struct(repr)
}

pub fn buffer_batch_get_request_from_repr_generated(src: *const KvrpcpbBufferBatchGetRequest) -> Option<pb::BufferBatchGetRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BufferBatchGetRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_version(repr.version);
    Some(out)
}

pub fn buffer_batch_get_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BufferBatchGetResponse) -> &'a mut KvrpcpbBufferBatchGetResponse {
    let mut repr = KvrpcpbBufferBatchGetResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
        pairs: KvprotoSliceKvrpcpbKvPairPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    {
        let values = src.get_pairs();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKvPair> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(kv_pair_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.pairs.data = ptr;
                repr.pairs.len = len;
                repr.pairs.cap = len;
            }
        }
    }
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn buffer_batch_get_response_from_repr_generated(src: *const KvrpcpbBufferBatchGetResponse) -> Option<pb::BufferBatchGetResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BufferBatchGetResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    if !repr.pairs.data.is_null() && repr.pairs.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.pairs.data, repr.pairs.len) };
        let mut values: Vec<pb::KvPair> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = kv_pair_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_pairs(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn check_leader_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CheckLeaderRequest) -> &'a mut KvrpcpbCheckLeaderRequest {
    let mut repr = KvrpcpbCheckLeaderRequest {
        regions: KvprotoSliceKvrpcpbLeaderInfoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        ts: Default::default(),
    };
    {
        let values = src.get_regions();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbLeaderInfo> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(leader_info_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.regions.data = ptr;
                repr.regions.len = len;
                repr.regions.cap = len;
            }
        }
    }
    repr.ts = src.get_ts();
    arena.alloc_struct(repr)
}

pub fn check_leader_request_from_repr_generated(src: *const KvrpcpbCheckLeaderRequest) -> Option<pb::CheckLeaderRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CheckLeaderRequest::new();
    if !repr.regions.data.is_null() && repr.regions.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.regions.data, repr.regions.len) };
        let mut values: Vec<pb::LeaderInfo> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = leader_info_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_regions(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_ts(repr.ts);
    Some(out)
}

pub fn check_leader_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CheckLeaderResponse) -> &'a mut KvrpcpbCheckLeaderResponse {
    let mut repr = KvrpcpbCheckLeaderResponse {
        regions: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        ts: Default::default(),
    };
    {
        let values = src.get_regions();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.regions.data = ptr;
            repr.regions.len = len;
            repr.regions.cap = len;
        }
    }
    repr.ts = src.get_ts();
    arena.alloc_struct(repr)
}

pub fn check_leader_response_from_repr_generated(src: *const KvrpcpbCheckLeaderResponse) -> Option<pb::CheckLeaderResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CheckLeaderResponse::new();
    if !repr.regions.data.is_null() && repr.regions.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.regions.data, repr.regions.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_regions(values);
    }
    out.set_ts(repr.ts);
    Some(out)
}

pub fn check_lock_observer_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CheckLockObserverRequest) -> &'a mut KvrpcpbCheckLockObserverRequest {
    let mut repr = KvrpcpbCheckLockObserverRequest {
        context: ptr::null_mut(),
        max_ts: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.max_ts = src.get_max_ts();
    arena.alloc_struct(repr)
}

pub fn check_lock_observer_request_from_repr_generated(src: *const KvrpcpbCheckLockObserverRequest) -> Option<pb::CheckLockObserverRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CheckLockObserverRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_max_ts(repr.max_ts);
    Some(out)
}

pub fn check_lock_observer_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CheckLockObserverResponse) -> &'a mut KvrpcpbCheckLockObserverResponse {
    let mut repr = KvrpcpbCheckLockObserverResponse {
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        is_clean: Default::default(),
        locks: KvprotoSliceKvrpcpbLockInfoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    repr.is_clean = src.get_is_clean();
    {
        let values = src.get_locks();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbLockInfo> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(lock_info_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.locks.data = ptr;
                repr.locks.len = len;
                repr.locks.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn check_lock_observer_response_from_repr_generated(src: *const KvrpcpbCheckLockObserverResponse) -> Option<pb::CheckLockObserverResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CheckLockObserverResponse::new();
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    out.set_is_clean(repr.is_clean);
    if !repr.locks.data.is_null() && repr.locks.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.locks.data, repr.locks.len) };
        let mut values: Vec<pb::LockInfo> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = lock_info_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_locks(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn check_secondary_locks_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CheckSecondaryLocksRequest) -> &'a mut KvrpcpbCheckSecondaryLocksRequest {
    let mut repr = KvrpcpbCheckSecondaryLocksRequest {
        context: ptr::null_mut(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        start_version: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
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
    repr.start_version = src.get_start_version();
    arena.alloc_struct(repr)
}

pub fn check_secondary_locks_request_from_repr_generated(src: *const KvrpcpbCheckSecondaryLocksRequest) -> Option<pb::CheckSecondaryLocksRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CheckSecondaryLocksRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_start_version(repr.start_version);
    Some(out)
}

pub fn check_secondary_locks_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CheckSecondaryLocksResponse) -> &'a mut KvrpcpbCheckSecondaryLocksResponse {
    let mut repr = KvrpcpbCheckSecondaryLocksResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
        locks: KvprotoSliceKvrpcpbLockInfoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        commit_ts: Default::default(),
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    {
        let values = src.get_locks();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbLockInfo> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(lock_info_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.locks.data = ptr;
                repr.locks.len = len;
                repr.locks.cap = len;
            }
        }
    }
    repr.commit_ts = src.get_commit_ts();
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn check_secondary_locks_response_from_repr_generated(src: *const KvrpcpbCheckSecondaryLocksResponse) -> Option<pb::CheckSecondaryLocksResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CheckSecondaryLocksResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    if !repr.locks.data.is_null() && repr.locks.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.locks.data, repr.locks.len) };
        let mut values: Vec<pb::LockInfo> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = lock_info_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_locks(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_commit_ts(repr.commit_ts);
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn check_txn_status_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CheckTxnStatusRequest) -> &'a mut KvrpcpbCheckTxnStatusRequest {
    let mut repr = KvrpcpbCheckTxnStatusRequest {
        context: ptr::null_mut(),
        primary_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        lock_ts: Default::default(),
        caller_start_ts: Default::default(),
        current_ts: Default::default(),
        rollback_if_not_exist: Default::default(),
        force_sync_commit: Default::default(),
        resolving_pessimistic_lock: Default::default(),
        verify_is_primary: Default::default(),
        is_txn_file: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_primary_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_primary_key());
        repr.primary_key.data = ptr;
        repr.primary_key.len = len;
    }
    repr.lock_ts = src.get_lock_ts();
    repr.caller_start_ts = src.get_caller_start_ts();
    repr.current_ts = src.get_current_ts();
    repr.rollback_if_not_exist = src.get_rollback_if_not_exist();
    repr.force_sync_commit = src.get_force_sync_commit();
    repr.resolving_pessimistic_lock = src.get_resolving_pessimistic_lock();
    repr.verify_is_primary = src.get_verify_is_primary();
    repr.is_txn_file = src.get_is_txn_file();
    arena.alloc_struct(repr)
}

pub fn check_txn_status_request_from_repr_generated(src: *const KvrpcpbCheckTxnStatusRequest) -> Option<pb::CheckTxnStatusRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CheckTxnStatusRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_primary_key(bytes_from(repr.primary_key.data, repr.primary_key.len).into());
    out.set_lock_ts(repr.lock_ts);
    out.set_caller_start_ts(repr.caller_start_ts);
    out.set_current_ts(repr.current_ts);
    out.set_rollback_if_not_exist(repr.rollback_if_not_exist);
    out.set_force_sync_commit(repr.force_sync_commit);
    out.set_resolving_pessimistic_lock(repr.resolving_pessimistic_lock);
    out.set_verify_is_primary(repr.verify_is_primary);
    out.set_is_txn_file(repr.is_txn_file);
    Some(out)
}

pub fn check_txn_status_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CheckTxnStatusResponse) -> &'a mut KvrpcpbCheckTxnStatusResponse {
    let mut repr = KvrpcpbCheckTxnStatusResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
        lock_ttl: Default::default(),
        commit_version: Default::default(),
        action: Default::default(),
        lock_info: ptr::null_mut(),
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    repr.lock_ttl = src.get_lock_ttl();
    repr.commit_version = src.get_commit_version();
    repr.action = src.get_action() as i32;
    if src.has_lock_info() {
        repr.lock_info = lock_info_to_repr_generated(arena, src.get_lock_info()) as *mut _;
    } else {
        repr.lock_info = ptr::null_mut();
    }
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn check_txn_status_response_from_repr_generated(src: *const KvrpcpbCheckTxnStatusResponse) -> Option<pb::CheckTxnStatusResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CheckTxnStatusResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    out.set_lock_ttl(repr.lock_ttl);
    out.set_commit_version(repr.commit_version);
    out.set_action(pb::Action::from_i32(repr.action).unwrap_or_default());
    if !repr.lock_info.is_null() {
        if let Some(value) = lock_info_from_repr_generated(repr.lock_info) {
            out.set_lock_info(value);
        }
    }
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn cleanup_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CleanupRequest) -> &'a mut KvrpcpbCleanupRequest {
    let mut repr = KvrpcpbCleanupRequest {
        context: ptr::null_mut(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        start_version: Default::default(),
        current_ts: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    repr.start_version = src.get_start_version();
    repr.current_ts = src.get_current_ts();
    arena.alloc_struct(repr)
}

pub fn cleanup_request_from_repr_generated(src: *const KvrpcpbCleanupRequest) -> Option<pb::CleanupRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CleanupRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_start_version(repr.start_version);
    out.set_current_ts(repr.current_ts);
    Some(out)
}

pub fn cleanup_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CleanupResponse) -> &'a mut KvrpcpbCleanupResponse {
    let mut repr = KvrpcpbCleanupResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
        commit_version: Default::default(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    repr.commit_version = src.get_commit_version();
    arena.alloc_struct(repr)
}

pub fn cleanup_response_from_repr_generated(src: *const KvrpcpbCleanupResponse) -> Option<pb::CleanupResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CleanupResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    out.set_commit_version(repr.commit_version);
    Some(out)
}

pub fn commit_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CommitRequest) -> &'a mut KvrpcpbCommitRequest {
    let mut repr = KvrpcpbCommitRequest {
        context: ptr::null_mut(),
        start_version: Default::default(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        commit_version: Default::default(),
        commit_role: Default::default(),
        primary_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        is_txn_file: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.start_version = src.get_start_version();
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
    repr.commit_version = src.get_commit_version();
    repr.commit_role = src.get_commit_role() as i32;
    if !src.get_primary_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_primary_key());
        repr.primary_key.data = ptr;
        repr.primary_key.len = len;
    }
    repr.is_txn_file = src.get_is_txn_file();
    arena.alloc_struct(repr)
}

pub fn commit_request_from_repr_generated(src: *const KvrpcpbCommitRequest) -> Option<pb::CommitRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CommitRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_version(repr.start_version);
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_commit_version(repr.commit_version);
    out.set_commit_role(pb::CommitRole::from_i32(repr.commit_role).unwrap_or_default());
    out.set_primary_key(bytes_from(repr.primary_key.data, repr.primary_key.len).into());
    out.set_is_txn_file(repr.is_txn_file);
    Some(out)
}

pub fn commit_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CommitResponse) -> &'a mut KvrpcpbCommitResponse {
    let mut repr = KvrpcpbCommitResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
        commit_version: Default::default(),
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    repr.commit_version = src.get_commit_version();
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn commit_response_from_repr_generated(src: *const KvrpcpbCommitResponse) -> Option<pb::CommitResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CommitResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    out.set_commit_version(repr.commit_version);
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn commit_ts_expired_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CommitTsExpired) -> &'a mut KvrpcpbCommitTsExpired {
    let mut repr = KvrpcpbCommitTsExpired {
        start_ts: Default::default(),
        attempted_commit_ts: Default::default(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        min_commit_ts: Default::default(),
    };
    repr.start_ts = src.get_start_ts();
    repr.attempted_commit_ts = src.get_attempted_commit_ts();
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    repr.min_commit_ts = src.get_min_commit_ts();
    arena.alloc_struct(repr)
}

pub fn commit_ts_expired_from_repr_generated(src: *const KvrpcpbCommitTsExpired) -> Option<pb::CommitTsExpired> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CommitTsExpired::new();
    out.set_start_ts(repr.start_ts);
    out.set_attempted_commit_ts(repr.attempted_commit_ts);
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_min_commit_ts(repr.min_commit_ts);
    Some(out)
}

pub fn commit_ts_too_large_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CommitTsTooLarge) -> &'a mut KvrpcpbCommitTsTooLarge {
    let mut repr = KvrpcpbCommitTsTooLarge {
        commit_ts: Default::default(),
    };
    repr.commit_ts = src.get_commit_ts();
    arena.alloc_struct(repr)
}

pub fn commit_ts_too_large_from_repr_generated(src: *const KvrpcpbCommitTsTooLarge) -> Option<pb::CommitTsTooLarge> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CommitTsTooLarge::new();
    out.set_commit_ts(repr.commit_ts);
    Some(out)
}

pub fn compact_error_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CompactError) -> &'a mut KvrpcpbCompactError {
    let mut repr = KvrpcpbCompactError {
        error_case: Default::default(),
        err_invalid_start_key: ptr::null_mut(),
        err_physical_table_not_exist: ptr::null_mut(),
        err_compact_in_progress: ptr::null_mut(),
        err_too_many_pending_tasks: ptr::null_mut(),
    };
    arena.alloc_struct(repr)
}

pub fn compact_error_from_repr_generated(src: *const KvrpcpbCompactError) -> Option<pb::CompactError> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CompactError::new();
    Some(out)
}

pub fn compact_error_compact_in_progress_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CompactErrorCompactInProgress) -> &'a mut KvrpcpbCompactErrorCompactInProgress {
    let mut repr = KvrpcpbCompactErrorCompactInProgress {
    };
    arena.alloc_struct(repr)
}

pub fn compact_error_compact_in_progress_from_repr_generated(src: *const KvrpcpbCompactErrorCompactInProgress) -> Option<pb::CompactErrorCompactInProgress> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CompactErrorCompactInProgress::new();
    Some(out)
}

pub fn compact_error_invalid_start_key_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CompactErrorInvalidStartKey) -> &'a mut KvrpcpbCompactErrorInvalidStartKey {
    let mut repr = KvrpcpbCompactErrorInvalidStartKey {
    };
    arena.alloc_struct(repr)
}

pub fn compact_error_invalid_start_key_from_repr_generated(src: *const KvrpcpbCompactErrorInvalidStartKey) -> Option<pb::CompactErrorInvalidStartKey> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CompactErrorInvalidStartKey::new();
    Some(out)
}

pub fn compact_error_physical_table_not_exist_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CompactErrorPhysicalTableNotExist) -> &'a mut KvrpcpbCompactErrorPhysicalTableNotExist {
    let mut repr = KvrpcpbCompactErrorPhysicalTableNotExist {
    };
    arena.alloc_struct(repr)
}

pub fn compact_error_physical_table_not_exist_from_repr_generated(src: *const KvrpcpbCompactErrorPhysicalTableNotExist) -> Option<pb::CompactErrorPhysicalTableNotExist> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CompactErrorPhysicalTableNotExist::new();
    Some(out)
}

pub fn compact_error_too_many_pending_tasks_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CompactErrorTooManyPendingTasks) -> &'a mut KvrpcpbCompactErrorTooManyPendingTasks {
    let mut repr = KvrpcpbCompactErrorTooManyPendingTasks {
    };
    arena.alloc_struct(repr)
}

pub fn compact_error_too_many_pending_tasks_from_repr_generated(src: *const KvrpcpbCompactErrorTooManyPendingTasks) -> Option<pb::CompactErrorTooManyPendingTasks> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CompactErrorTooManyPendingTasks::new();
    Some(out)
}

pub fn compact_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CompactRequest) -> &'a mut KvrpcpbCompactRequest {
    let mut repr = KvrpcpbCompactRequest {
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        physical_table_id: Default::default(),
        logical_table_id: Default::default(),
        api_version: Default::default(),
        keyspace_id: Default::default(),
    };
    if !src.get_start_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_start_key());
        repr.start_key.data = ptr;
        repr.start_key.len = len;
    }
    repr.physical_table_id = src.get_physical_table_id();
    repr.logical_table_id = src.get_logical_table_id();
    repr.api_version = src.get_api_version() as i32;
    repr.keyspace_id = src.get_keyspace_id();
    arena.alloc_struct(repr)
}

pub fn compact_request_from_repr_generated(src: *const KvrpcpbCompactRequest) -> Option<pb::CompactRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CompactRequest::new();
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_physical_table_id(repr.physical_table_id);
    out.set_logical_table_id(repr.logical_table_id);
    out.set_api_version(pb::ApiVersion::from_i32(repr.api_version).unwrap_or_default());
    out.set_keyspace_id(repr.keyspace_id);
    Some(out)
}

pub fn compact_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::CompactResponse) -> &'a mut KvrpcpbCompactResponse {
    let mut repr = KvrpcpbCompactResponse {
        error: ptr::null_mut(),
        has_remaining: Default::default(),
        compacted_start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        compacted_end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if src.has_error() {
        repr.error = compact_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    repr.has_remaining = src.get_has_remaining();
    if !src.get_compacted_start_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_compacted_start_key());
        repr.compacted_start_key.data = ptr;
        repr.compacted_start_key.len = len;
    }
    if !src.get_compacted_end_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_compacted_end_key());
        repr.compacted_end_key.data = ptr;
        repr.compacted_end_key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn compact_response_from_repr_generated(src: *const KvrpcpbCompactResponse) -> Option<pb::CompactResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::CompactResponse::new();
    if !repr.error.is_null() {
        if let Some(value) = compact_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    out.set_has_remaining(repr.has_remaining);
    out.set_compacted_start_key(bytes_from(repr.compacted_start_key.data, repr.compacted_start_key.len).into());
    out.set_compacted_end_key(bytes_from(repr.compacted_end_key.data, repr.compacted_end_key.len).into());
    Some(out)
}

pub fn context_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Context) -> &'a mut KvrpcpbContext {
    let mut repr = KvrpcpbContext {
        region_id: Default::default(),
        region_epoch: ptr::null_mut(),
        peer: ptr::null_mut(),
        term: Default::default(),
        priority: Default::default(),
        isolation_level: Default::default(),
        not_fill_cache: Default::default(),
        sync_log: Default::default(),
        record_time_stat: Default::default(),
        record_scan_stat: Default::default(),
        replica_read: Default::default(),
        resolved_locks: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        max_execution_duration_ms: Default::default(),
        applied_index: Default::default(),
        task_id: Default::default(),
        stale_read: Default::default(),
        resource_group_tag: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        disk_full_opt: Default::default(),
        is_retry_request: Default::default(),
        api_version: Default::default(),
        committed_locks: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        trace_context: ptr::null_mut(),
        request_source: KvprotoStringView { data: ptr::null(), len: 0 },
        txn_source: Default::default(),
        busy_threshold_ms: Default::default(),
        resource_control_context: ptr::null_mut(),
        keyspace_id: Default::default(),
        buckets_version: Default::default(),
        source_stmt: ptr::null_mut(),
        cluster_id: Default::default(),
    };
    repr.region_id = src.get_region_id();
    if src.has_region_epoch() {
        repr.region_epoch = crate::ffi_runtime::metapb::region_epoch_to_repr_generated(arena, src.get_region_epoch()) as *mut _;
    } else {
        repr.region_epoch = ptr::null_mut();
    }
    if src.has_peer() {
        repr.peer = crate::ffi_runtime::metapb::peer_to_repr_generated(arena, src.get_peer()) as *mut _;
    } else {
        repr.peer = ptr::null_mut();
    }
    repr.term = src.get_term();
    repr.priority = src.get_priority() as i32;
    repr.isolation_level = src.get_isolation_level() as i32;
    repr.not_fill_cache = src.get_not_fill_cache();
    repr.sync_log = src.get_sync_log();
    repr.record_time_stat = src.get_record_time_stat();
    repr.record_scan_stat = src.get_record_scan_stat();
    repr.replica_read = src.get_replica_read();
    {
        let values = src.get_resolved_locks();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.resolved_locks.data = ptr;
            repr.resolved_locks.len = len;
            repr.resolved_locks.cap = len;
        }
    }
    repr.max_execution_duration_ms = src.get_max_execution_duration_ms();
    repr.applied_index = src.get_applied_index();
    repr.task_id = src.get_task_id();
    repr.stale_read = src.get_stale_read();
    if !src.get_resource_group_tag().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_resource_group_tag());
        repr.resource_group_tag.data = ptr;
        repr.resource_group_tag.len = len;
    }
    repr.disk_full_opt = src.get_disk_full_opt() as i32;
    repr.is_retry_request = src.get_is_retry_request();
    repr.api_version = src.get_api_version() as i32;
    {
        let values = src.get_committed_locks();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.committed_locks.data = ptr;
            repr.committed_locks.len = len;
            repr.committed_locks.cap = len;
        }
    }
    if src.has_trace_context() {
        repr.trace_context = crate::ffi_runtime::tracepb::trace_context_to_repr_generated(arena, src.get_trace_context()) as *mut _;
    } else {
        repr.trace_context = ptr::null_mut();
    }
    if !src.get_request_source().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_request_source());
        repr.request_source.data = ptr as *const c_char;
        repr.request_source.len = len;
    }
    repr.txn_source = src.get_txn_source();
    repr.busy_threshold_ms = src.get_busy_threshold_ms();
    if src.has_resource_control_context() {
        repr.resource_control_context = resource_control_context_to_repr_generated(arena, src.get_resource_control_context()) as *mut _;
    } else {
        repr.resource_control_context = ptr::null_mut();
    }
    repr.keyspace_id = src.get_keyspace_id();
    repr.buckets_version = src.get_buckets_version();
    if src.has_source_stmt() {
        repr.source_stmt = source_stmt_to_repr_generated(arena, src.get_source_stmt()) as *mut _;
    } else {
        repr.source_stmt = ptr::null_mut();
    }
    repr.cluster_id = src.get_cluster_id();
    arena.alloc_struct(repr)
}

pub fn context_from_repr_generated(src: *const KvrpcpbContext) -> Option<pb::Context> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Context::new();
    out.set_region_id(repr.region_id);
    if !repr.region_epoch.is_null() {
        if let Some(value) = crate::ffi_runtime::metapb::region_epoch_from_repr_generated(repr.region_epoch) {
            out.set_region_epoch(value);
        }
    }
    if !repr.peer.is_null() {
        if let Some(value) = crate::ffi_runtime::metapb::peer_from_repr_generated(repr.peer) {
            out.set_peer(value);
        }
    }
    out.set_term(repr.term);
    out.set_priority(pb::CommandPri::from_i32(repr.priority).unwrap_or_default());
    out.set_isolation_level(pb::IsolationLevel::from_i32(repr.isolation_level).unwrap_or_default());
    out.set_not_fill_cache(repr.not_fill_cache);
    out.set_sync_log(repr.sync_log);
    out.set_record_time_stat(repr.record_time_stat);
    out.set_record_scan_stat(repr.record_scan_stat);
    out.set_replica_read(repr.replica_read);
    if !repr.resolved_locks.data.is_null() && repr.resolved_locks.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.resolved_locks.data, repr.resolved_locks.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_resolved_locks(values);
    }
    out.set_max_execution_duration_ms(repr.max_execution_duration_ms);
    out.set_applied_index(repr.applied_index);
    out.set_task_id(repr.task_id);
    out.set_stale_read(repr.stale_read);
    out.set_resource_group_tag(bytes_from(repr.resource_group_tag.data, repr.resource_group_tag.len).into());
    out.set_disk_full_opt(pb::DiskFullOpt::from_i32(repr.disk_full_opt).unwrap_or_default());
    out.set_is_retry_request(repr.is_retry_request);
    out.set_api_version(pb::ApiVersion::from_i32(repr.api_version).unwrap_or_default());
    if !repr.committed_locks.data.is_null() && repr.committed_locks.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.committed_locks.data, repr.committed_locks.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_committed_locks(values);
    }
    if !repr.trace_context.is_null() {
        if let Some(value) = crate::ffi_runtime::tracepb::trace_context_from_repr_generated(repr.trace_context) {
            out.set_trace_context(value);
        }
    }
    out.set_request_source(string_from(repr.request_source.data as *const u8, repr.request_source.len));
    out.set_txn_source(repr.txn_source);
    out.set_busy_threshold_ms(repr.busy_threshold_ms);
    if !repr.resource_control_context.is_null() {
        if let Some(value) = resource_control_context_from_repr_generated(repr.resource_control_context) {
            out.set_resource_control_context(value);
        }
    }
    out.set_keyspace_id(repr.keyspace_id);
    out.set_buckets_version(repr.buckets_version);
    if !repr.source_stmt.is_null() {
        if let Some(value) = source_stmt_from_repr_generated(repr.source_stmt) {
            out.set_source_stmt(value);
        }
    }
    out.set_cluster_id(repr.cluster_id);
    Some(out)
}

pub fn deadlock_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Deadlock) -> &'a mut KvrpcpbDeadlock {
    let mut repr = KvrpcpbDeadlock {
        lock_ts: Default::default(),
        lock_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        deadlock_key_hash: Default::default(),
        wait_chain: KvprotoSliceDeadlockWaitForEntryPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        deadlock_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    repr.lock_ts = src.get_lock_ts();
    if !src.get_lock_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_lock_key());
        repr.lock_key.data = ptr;
        repr.lock_key.len = len;
    }
    repr.deadlock_key_hash = src.get_deadlock_key_hash();
    {
        let values = src.get_wait_chain();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut DeadlockWaitForEntry> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(crate::ffi_runtime::deadlock::wait_for_entry_to_repr_generated(arena, value) as *mut _);
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

pub fn deadlock_from_repr_generated(src: *const KvrpcpbDeadlock) -> Option<pb::Deadlock> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Deadlock::new();
    out.set_lock_ts(repr.lock_ts);
    out.set_lock_key(bytes_from(repr.lock_key.data, repr.lock_key.len).into());
    out.set_deadlock_key_hash(repr.deadlock_key_hash);
    if !repr.wait_chain.data.is_null() && repr.wait_chain.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.wait_chain.data, repr.wait_chain.len) };
        let mut values: Vec<deadlock::WaitForEntry> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = crate::ffi_runtime::deadlock::wait_for_entry_from_repr_generated(ptr) {
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

pub fn debug_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::DebugInfo) -> &'a mut KvrpcpbDebugInfo {
    let mut repr = KvrpcpbDebugInfo {
        mvcc_info: KvprotoSliceKvrpcpbMvccDebugInfoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_mvcc_info();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbMvccDebugInfo> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(mvcc_debug_info_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.mvcc_info.data = ptr;
                repr.mvcc_info.len = len;
                repr.mvcc_info.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn debug_info_from_repr_generated(src: *const KvrpcpbDebugInfo) -> Option<pb::DebugInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::DebugInfo::new();
    if !repr.mvcc_info.data.is_null() && repr.mvcc_info.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.mvcc_info.data, repr.mvcc_info.len) };
        let mut values: Vec<pb::MvccDebugInfo> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = mvcc_debug_info_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_mvcc_info(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn delete_range_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::DeleteRangeRequest) -> &'a mut KvrpcpbDeleteRangeRequest {
    let mut repr = KvrpcpbDeleteRangeRequest {
        context: ptr::null_mut(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        notify_only: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
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
    repr.notify_only = src.get_notify_only();
    arena.alloc_struct(repr)
}

pub fn delete_range_request_from_repr_generated(src: *const KvrpcpbDeleteRangeRequest) -> Option<pb::DeleteRangeRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::DeleteRangeRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    out.set_notify_only(repr.notify_only);
    Some(out)
}

pub fn delete_range_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::DeleteRangeResponse) -> &'a mut KvrpcpbDeleteRangeResponse {
    let mut repr = KvrpcpbDeleteRangeResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn delete_range_response_from_repr_generated(src: *const KvrpcpbDeleteRangeResponse) -> Option<pb::DeleteRangeResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::DeleteRangeResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn exec_details_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ExecDetails) -> &'a mut KvrpcpbExecDetails {
    let mut repr = KvrpcpbExecDetails {
        time_detail: ptr::null_mut(),
        scan_detail: ptr::null_mut(),
    };
    if src.has_time_detail() {
        repr.time_detail = time_detail_to_repr_generated(arena, src.get_time_detail()) as *mut _;
    } else {
        repr.time_detail = ptr::null_mut();
    }
    if src.has_scan_detail() {
        repr.scan_detail = scan_detail_to_repr_generated(arena, src.get_scan_detail()) as *mut _;
    } else {
        repr.scan_detail = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn exec_details_from_repr_generated(src: *const KvrpcpbExecDetails) -> Option<pb::ExecDetails> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ExecDetails::new();
    if !repr.time_detail.is_null() {
        if let Some(value) = time_detail_from_repr_generated(repr.time_detail) {
            out.set_time_detail(value);
        }
    }
    if !repr.scan_detail.is_null() {
        if let Some(value) = scan_detail_from_repr_generated(repr.scan_detail) {
            out.set_scan_detail(value);
        }
    }
    Some(out)
}

pub fn exec_details_v2_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ExecDetailsV2) -> &'a mut KvrpcpbExecDetailsV2 {
    let mut repr = KvrpcpbExecDetailsV2 {
        time_detail: ptr::null_mut(),
        scan_detail_v2: ptr::null_mut(),
        write_detail: ptr::null_mut(),
        time_detail_v2: ptr::null_mut(),
    };
    if src.has_time_detail() {
        repr.time_detail = time_detail_to_repr_generated(arena, src.get_time_detail()) as *mut _;
    } else {
        repr.time_detail = ptr::null_mut();
    }
    if src.has_scan_detail_v2() {
        repr.scan_detail_v2 = scan_detail_v2_to_repr_generated(arena, src.get_scan_detail_v2()) as *mut _;
    } else {
        repr.scan_detail_v2 = ptr::null_mut();
    }
    if src.has_write_detail() {
        repr.write_detail = write_detail_to_repr_generated(arena, src.get_write_detail()) as *mut _;
    } else {
        repr.write_detail = ptr::null_mut();
    }
    if src.has_time_detail_v2() {
        repr.time_detail_v2 = time_detail_v2_to_repr_generated(arena, src.get_time_detail_v2()) as *mut _;
    } else {
        repr.time_detail_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn exec_details_v2_from_repr_generated(src: *const KvrpcpbExecDetailsV2) -> Option<pb::ExecDetailsV2> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ExecDetailsV2::new();
    if !repr.time_detail.is_null() {
        if let Some(value) = time_detail_from_repr_generated(repr.time_detail) {
            out.set_time_detail(value);
        }
    }
    if !repr.scan_detail_v2.is_null() {
        if let Some(value) = scan_detail_v2_from_repr_generated(repr.scan_detail_v2) {
            out.set_scan_detail_v2(value);
        }
    }
    if !repr.write_detail.is_null() {
        if let Some(value) = write_detail_from_repr_generated(repr.write_detail) {
            out.set_write_detail(value);
        }
    }
    if !repr.time_detail_v2.is_null() {
        if let Some(value) = time_detail_v2_from_repr_generated(repr.time_detail_v2) {
            out.set_time_detail_v2(value);
        }
    }
    Some(out)
}

pub fn flashback_to_version_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::FlashbackToVersionRequest) -> &'a mut KvrpcpbFlashbackToVersionRequest {
    let mut repr = KvrpcpbFlashbackToVersionRequest {
        context: ptr::null_mut(),
        version: Default::default(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        start_ts: Default::default(),
        commit_ts: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.version = src.get_version();
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
    repr.start_ts = src.get_start_ts();
    repr.commit_ts = src.get_commit_ts();
    arena.alloc_struct(repr)
}

pub fn flashback_to_version_request_from_repr_generated(src: *const KvrpcpbFlashbackToVersionRequest) -> Option<pb::FlashbackToVersionRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::FlashbackToVersionRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_version(repr.version);
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    out.set_start_ts(repr.start_ts);
    out.set_commit_ts(repr.commit_ts);
    Some(out)
}

pub fn flashback_to_version_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::FlashbackToVersionResponse) -> &'a mut KvrpcpbFlashbackToVersionResponse {
    let mut repr = KvrpcpbFlashbackToVersionResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn flashback_to_version_response_from_repr_generated(src: *const KvrpcpbFlashbackToVersionResponse) -> Option<pb::FlashbackToVersionResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::FlashbackToVersionResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn flush_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::FlushRequest) -> &'a mut KvrpcpbFlushRequest {
    let mut repr = KvrpcpbFlushRequest {
        context: ptr::null_mut(),
        mutations: KvprotoSliceKvrpcpbMutationPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        primary_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        start_ts: Default::default(),
        min_commit_ts: Default::default(),
        generation: Default::default(),
        lock_ttl: Default::default(),
        assertion_level: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    {
        let values = src.get_mutations();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbMutation> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(mutation_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.mutations.data = ptr;
                repr.mutations.len = len;
                repr.mutations.cap = len;
            }
        }
    }
    if !src.get_primary_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_primary_key());
        repr.primary_key.data = ptr;
        repr.primary_key.len = len;
    }
    repr.start_ts = src.get_start_ts();
    repr.min_commit_ts = src.get_min_commit_ts();
    repr.generation = src.get_generation();
    repr.lock_ttl = src.get_lock_ttl();
    repr.assertion_level = src.get_assertion_level() as i32;
    arena.alloc_struct(repr)
}

pub fn flush_request_from_repr_generated(src: *const KvrpcpbFlushRequest) -> Option<pb::FlushRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::FlushRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.mutations.data.is_null() && repr.mutations.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.mutations.data, repr.mutations.len) };
        let mut values: Vec<pb::Mutation> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = mutation_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_mutations(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_primary_key(bytes_from(repr.primary_key.data, repr.primary_key.len).into());
    out.set_start_ts(repr.start_ts);
    out.set_min_commit_ts(repr.min_commit_ts);
    out.set_generation(repr.generation);
    out.set_lock_ttl(repr.lock_ttl);
    out.set_assertion_level(pb::AssertionLevel::from_i32(repr.assertion_level).unwrap_or_default());
    Some(out)
}

pub fn flush_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::FlushResponse) -> &'a mut KvrpcpbFlushResponse {
    let mut repr = KvrpcpbFlushResponse {
        region_error: ptr::null_mut(),
        errors: KvprotoSliceKvrpcpbKeyErrorPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    {
        let values = src.get_errors();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKeyError> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(key_error_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.errors.data = ptr;
                repr.errors.len = len;
                repr.errors.cap = len;
            }
        }
    }
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn flush_response_from_repr_generated(src: *const KvrpcpbFlushResponse) -> Option<pb::FlushResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::FlushResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.errors.data.is_null() && repr.errors.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.errors.data, repr.errors.len) };
        let mut values: Vec<pb::KeyError> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = key_error_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_errors(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn g_c_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GcRequest) -> &'a mut KvrpcpbGCRequest {
    let mut repr = KvrpcpbGCRequest {
        context: ptr::null_mut(),
        safe_point: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.safe_point = src.get_safe_point();
    arena.alloc_struct(repr)
}

pub fn g_c_request_from_repr_generated(src: *const KvrpcpbGCRequest) -> Option<pb::GcRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GcRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_safe_point(repr.safe_point);
    Some(out)
}

pub fn g_c_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GcResponse) -> &'a mut KvrpcpbGCResponse {
    let mut repr = KvrpcpbGCResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn g_c_response_from_repr_generated(src: *const KvrpcpbGCResponse) -> Option<pb::GcResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GcResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    Some(out)
}

pub fn get_health_feedback_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GetHealthFeedbackRequest) -> &'a mut KvrpcpbGetHealthFeedbackRequest {
    let mut repr = KvrpcpbGetHealthFeedbackRequest {
        context: ptr::null_mut(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn get_health_feedback_request_from_repr_generated(src: *const KvrpcpbGetHealthFeedbackRequest) -> Option<pb::GetHealthFeedbackRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GetHealthFeedbackRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    Some(out)
}

pub fn get_health_feedback_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GetHealthFeedbackResponse) -> &'a mut KvrpcpbGetHealthFeedbackResponse {
    let mut repr = KvrpcpbGetHealthFeedbackResponse {
        region_error: ptr::null_mut(),
        health_feedback: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_health_feedback() {
        repr.health_feedback = health_feedback_to_repr_generated(arena, src.get_health_feedback()) as *mut _;
    } else {
        repr.health_feedback = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn get_health_feedback_response_from_repr_generated(src: *const KvrpcpbGetHealthFeedbackResponse) -> Option<pb::GetHealthFeedbackResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GetHealthFeedbackResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.health_feedback.is_null() {
        if let Some(value) = health_feedback_from_repr_generated(repr.health_feedback) {
            out.set_health_feedback(value);
        }
    }
    Some(out)
}

pub fn get_lock_wait_history_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GetLockWaitHistoryRequest) -> &'a mut KvrpcpbGetLockWaitHistoryRequest {
    let mut repr = KvrpcpbGetLockWaitHistoryRequest {
        context: ptr::null_mut(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn get_lock_wait_history_request_from_repr_generated(src: *const KvrpcpbGetLockWaitHistoryRequest) -> Option<pb::GetLockWaitHistoryRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GetLockWaitHistoryRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    Some(out)
}

pub fn get_lock_wait_history_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GetLockWaitHistoryResponse) -> &'a mut KvrpcpbGetLockWaitHistoryResponse {
    let mut repr = KvrpcpbGetLockWaitHistoryResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        entries: KvprotoSliceDeadlockWaitForEntryPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    {
        let values = src.get_entries();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut DeadlockWaitForEntry> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(crate::ffi_runtime::deadlock::wait_for_entry_to_repr_generated(arena, value) as *mut _);
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

pub fn get_lock_wait_history_response_from_repr_generated(src: *const KvrpcpbGetLockWaitHistoryResponse) -> Option<pb::GetLockWaitHistoryResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GetLockWaitHistoryResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    if !repr.entries.data.is_null() && repr.entries.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.entries.data, repr.entries.len) };
        let mut values: Vec<deadlock::WaitForEntry> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = crate::ffi_runtime::deadlock::wait_for_entry_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_entries(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn get_lock_wait_info_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GetLockWaitInfoRequest) -> &'a mut KvrpcpbGetLockWaitInfoRequest {
    let mut repr = KvrpcpbGetLockWaitInfoRequest {
        context: ptr::null_mut(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn get_lock_wait_info_request_from_repr_generated(src: *const KvrpcpbGetLockWaitInfoRequest) -> Option<pb::GetLockWaitInfoRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GetLockWaitInfoRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    Some(out)
}

pub fn get_lock_wait_info_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GetLockWaitInfoResponse) -> &'a mut KvrpcpbGetLockWaitInfoResponse {
    let mut repr = KvrpcpbGetLockWaitInfoResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        entries: KvprotoSliceDeadlockWaitForEntryPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    {
        let values = src.get_entries();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut DeadlockWaitForEntry> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(crate::ffi_runtime::deadlock::wait_for_entry_to_repr_generated(arena, value) as *mut _);
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

pub fn get_lock_wait_info_response_from_repr_generated(src: *const KvrpcpbGetLockWaitInfoResponse) -> Option<pb::GetLockWaitInfoResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GetLockWaitInfoResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    if !repr.entries.data.is_null() && repr.entries.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.entries.data, repr.entries.len) };
        let mut values: Vec<deadlock::WaitForEntry> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = crate::ffi_runtime::deadlock::wait_for_entry_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_entries(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn get_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GetRequest) -> &'a mut KvrpcpbGetRequest {
    let mut repr = KvrpcpbGetRequest {
        context: ptr::null_mut(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        version: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    repr.version = src.get_version();
    arena.alloc_struct(repr)
}

pub fn get_request_from_repr_generated(src: *const KvrpcpbGetRequest) -> Option<pb::GetRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GetRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_version(repr.version);
    Some(out)
}

pub fn get_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GetResponse) -> &'a mut KvrpcpbGetResponse {
    let mut repr = KvrpcpbGetResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
        value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        not_found: Default::default(),
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    if !src.get_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_value());
        repr.value.data = ptr;
        repr.value.len = len;
    }
    repr.not_found = src.get_not_found();
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn get_response_from_repr_generated(src: *const KvrpcpbGetResponse) -> Option<pb::GetResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GetResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    out.set_value(bytes_from(repr.value.data, repr.value.len).into());
    out.set_not_found(repr.not_found);
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn health_feedback_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::HealthFeedback) -> &'a mut KvrpcpbHealthFeedback {
    let mut repr = KvrpcpbHealthFeedback {
        store_id: Default::default(),
        feedback_seq_no: Default::default(),
        slow_score: Default::default(),
    };
    repr.store_id = src.get_store_id();
    repr.feedback_seq_no = src.get_feedback_seq_no();
    repr.slow_score = src.get_slow_score();
    arena.alloc_struct(repr)
}

pub fn health_feedback_from_repr_generated(src: *const KvrpcpbHealthFeedback) -> Option<pb::HealthFeedback> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::HealthFeedback::new();
    out.set_store_id(repr.store_id);
    out.set_feedback_seq_no(repr.feedback_seq_no);
    out.set_slow_score(repr.slow_score);
    Some(out)
}

pub fn import_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ImportRequest) -> &'a mut KvrpcpbImportRequest {
    let mut repr = KvrpcpbImportRequest {
        mutations: KvprotoSliceKvrpcpbMutationPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        commit_version: Default::default(),
    };
    {
        let values = src.get_mutations();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbMutation> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(mutation_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.mutations.data = ptr;
                repr.mutations.len = len;
                repr.mutations.cap = len;
            }
        }
    }
    repr.commit_version = src.get_commit_version();
    arena.alloc_struct(repr)
}

pub fn import_request_from_repr_generated(src: *const KvrpcpbImportRequest) -> Option<pb::ImportRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ImportRequest::new();
    if !repr.mutations.data.is_null() && repr.mutations.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.mutations.data, repr.mutations.len) };
        let mut values: Vec<pb::Mutation> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = mutation_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_mutations(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_commit_version(repr.commit_version);
    Some(out)
}

pub fn import_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ImportResponse) -> &'a mut KvrpcpbImportResponse {
    let mut repr = KvrpcpbImportResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn import_response_from_repr_generated(src: *const KvrpcpbImportResponse) -> Option<pb::ImportResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ImportResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn key_error_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::KeyError) -> &'a mut KvrpcpbKeyError {
    let mut repr = KvrpcpbKeyError {
        locked: ptr::null_mut(),
        retryable: KvprotoStringView { data: ptr::null(), len: 0 },
        abort: KvprotoStringView { data: ptr::null(), len: 0 },
        conflict: ptr::null_mut(),
        already_exist: ptr::null_mut(),
        deadlock: ptr::null_mut(),
        commit_ts_expired: ptr::null_mut(),
        txn_not_found: ptr::null_mut(),
        commit_ts_too_large: ptr::null_mut(),
        assertion_failed: ptr::null_mut(),
        primary_mismatch: ptr::null_mut(),
        txn_lock_not_found: ptr::null_mut(),
        debug_info: ptr::null_mut(),
    };
    if src.has_locked() {
        repr.locked = lock_info_to_repr_generated(arena, src.get_locked()) as *mut _;
    } else {
        repr.locked = ptr::null_mut();
    }
    if !src.get_retryable().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_retryable());
        repr.retryable.data = ptr as *const c_char;
        repr.retryable.len = len;
    }
    if !src.get_abort().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_abort());
        repr.abort.data = ptr as *const c_char;
        repr.abort.len = len;
    }
    if src.has_conflict() {
        repr.conflict = write_conflict_to_repr_generated(arena, src.get_conflict()) as *mut _;
    } else {
        repr.conflict = ptr::null_mut();
    }
    if src.has_already_exist() {
        repr.already_exist = already_exist_to_repr_generated(arena, src.get_already_exist()) as *mut _;
    } else {
        repr.already_exist = ptr::null_mut();
    }
    if src.has_deadlock() {
        repr.deadlock = deadlock_to_repr_generated(arena, src.get_deadlock()) as *mut _;
    } else {
        repr.deadlock = ptr::null_mut();
    }
    if src.has_commit_ts_expired() {
        repr.commit_ts_expired = commit_ts_expired_to_repr_generated(arena, src.get_commit_ts_expired()) as *mut _;
    } else {
        repr.commit_ts_expired = ptr::null_mut();
    }
    if src.has_txn_not_found() {
        repr.txn_not_found = txn_not_found_to_repr_generated(arena, src.get_txn_not_found()) as *mut _;
    } else {
        repr.txn_not_found = ptr::null_mut();
    }
    if src.has_commit_ts_too_large() {
        repr.commit_ts_too_large = commit_ts_too_large_to_repr_generated(arena, src.get_commit_ts_too_large()) as *mut _;
    } else {
        repr.commit_ts_too_large = ptr::null_mut();
    }
    if src.has_assertion_failed() {
        repr.assertion_failed = assertion_failed_to_repr_generated(arena, src.get_assertion_failed()) as *mut _;
    } else {
        repr.assertion_failed = ptr::null_mut();
    }
    if src.has_primary_mismatch() {
        repr.primary_mismatch = primary_mismatch_to_repr_generated(arena, src.get_primary_mismatch()) as *mut _;
    } else {
        repr.primary_mismatch = ptr::null_mut();
    }
    if src.has_txn_lock_not_found() {
        repr.txn_lock_not_found = txn_lock_not_found_to_repr_generated(arena, src.get_txn_lock_not_found()) as *mut _;
    } else {
        repr.txn_lock_not_found = ptr::null_mut();
    }
    if src.has_debug_info() {
        repr.debug_info = debug_info_to_repr_generated(arena, src.get_debug_info()) as *mut _;
    } else {
        repr.debug_info = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn key_error_from_repr_generated(src: *const KvrpcpbKeyError) -> Option<pb::KeyError> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::KeyError::new();
    if !repr.locked.is_null() {
        if let Some(value) = lock_info_from_repr_generated(repr.locked) {
            out.set_locked(value);
        }
    }
    out.set_retryable(string_from(repr.retryable.data as *const u8, repr.retryable.len));
    out.set_abort(string_from(repr.abort.data as *const u8, repr.abort.len));
    if !repr.conflict.is_null() {
        if let Some(value) = write_conflict_from_repr_generated(repr.conflict) {
            out.set_conflict(value);
        }
    }
    if !repr.already_exist.is_null() {
        if let Some(value) = already_exist_from_repr_generated(repr.already_exist) {
            out.set_already_exist(value);
        }
    }
    if !repr.deadlock.is_null() {
        if let Some(value) = deadlock_from_repr_generated(repr.deadlock) {
            out.set_deadlock(value);
        }
    }
    if !repr.commit_ts_expired.is_null() {
        if let Some(value) = commit_ts_expired_from_repr_generated(repr.commit_ts_expired) {
            out.set_commit_ts_expired(value);
        }
    }
    if !repr.txn_not_found.is_null() {
        if let Some(value) = txn_not_found_from_repr_generated(repr.txn_not_found) {
            out.set_txn_not_found(value);
        }
    }
    if !repr.commit_ts_too_large.is_null() {
        if let Some(value) = commit_ts_too_large_from_repr_generated(repr.commit_ts_too_large) {
            out.set_commit_ts_too_large(value);
        }
    }
    if !repr.assertion_failed.is_null() {
        if let Some(value) = assertion_failed_from_repr_generated(repr.assertion_failed) {
            out.set_assertion_failed(value);
        }
    }
    if !repr.primary_mismatch.is_null() {
        if let Some(value) = primary_mismatch_from_repr_generated(repr.primary_mismatch) {
            out.set_primary_mismatch(value);
        }
    }
    if !repr.txn_lock_not_found.is_null() {
        if let Some(value) = txn_lock_not_found_from_repr_generated(repr.txn_lock_not_found) {
            out.set_txn_lock_not_found(value);
        }
    }
    if !repr.debug_info.is_null() {
        if let Some(value) = debug_info_from_repr_generated(repr.debug_info) {
            out.set_debug_info(value);
        }
    }
    Some(out)
}

pub fn key_range_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::KeyRange) -> &'a mut KvrpcpbKeyRange {
    let mut repr = KvrpcpbKeyRange {
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
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

pub fn key_range_from_repr_generated(src: *const KvrpcpbKeyRange) -> Option<pb::KeyRange> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::KeyRange::new();
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    Some(out)
}

pub fn kv_pair_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::KvPair) -> &'a mut KvrpcpbKvPair {
    let mut repr = KvrpcpbKvPair {
        error: ptr::null_mut(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if !src.get_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_value());
        repr.value.data = ptr;
        repr.value.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn kv_pair_from_repr_generated(src: *const KvrpcpbKvPair) -> Option<pb::KvPair> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::KvPair::new();
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_value(bytes_from(repr.value.data, repr.value.len).into());
    Some(out)
}

pub fn leader_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::LeaderInfo) -> &'a mut KvrpcpbLeaderInfo {
    let mut repr = KvrpcpbLeaderInfo {
        region_id: Default::default(),
        peer_id: Default::default(),
        term: Default::default(),
        region_epoch: ptr::null_mut(),
        read_state: ptr::null_mut(),
    };
    repr.region_id = src.get_region_id();
    repr.peer_id = src.get_peer_id();
    repr.term = src.get_term();
    if src.has_region_epoch() {
        repr.region_epoch = crate::ffi_runtime::metapb::region_epoch_to_repr_generated(arena, src.get_region_epoch()) as *mut _;
    } else {
        repr.region_epoch = ptr::null_mut();
    }
    if src.has_read_state() {
        repr.read_state = read_state_to_repr_generated(arena, src.get_read_state()) as *mut _;
    } else {
        repr.read_state = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn leader_info_from_repr_generated(src: *const KvrpcpbLeaderInfo) -> Option<pb::LeaderInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::LeaderInfo::new();
    out.set_region_id(repr.region_id);
    out.set_peer_id(repr.peer_id);
    out.set_term(repr.term);
    if !repr.region_epoch.is_null() {
        if let Some(value) = crate::ffi_runtime::metapb::region_epoch_from_repr_generated(repr.region_epoch) {
            out.set_region_epoch(value);
        }
    }
    if !repr.read_state.is_null() {
        if let Some(value) = read_state_from_repr_generated(repr.read_state) {
            out.set_read_state(value);
        }
    }
    Some(out)
}

pub fn lock_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::LockInfo) -> &'a mut KvrpcpbLockInfo {
    let mut repr = KvrpcpbLockInfo {
        primary_lock: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        lock_version: Default::default(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        lock_ttl: Default::default(),
        txn_size: Default::default(),
        lock_type: Default::default(),
        lock_for_update_ts: Default::default(),
        use_async_commit: Default::default(),
        min_commit_ts: Default::default(),
        secondaries: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        duration_to_last_update_ms: Default::default(),
        is_txn_file: Default::default(),
    };
    if !src.get_primary_lock().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_primary_lock());
        repr.primary_lock.data = ptr;
        repr.primary_lock.len = len;
    }
    repr.lock_version = src.get_lock_version();
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    repr.lock_ttl = src.get_lock_ttl();
    repr.txn_size = src.get_txn_size();
    repr.lock_type = src.get_lock_type() as i32;
    repr.lock_for_update_ts = src.get_lock_for_update_ts();
    repr.use_async_commit = src.get_use_async_commit();
    repr.min_commit_ts = src.get_min_commit_ts();
    {
        let values = src.get_secondaries();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_bytes(value);
                views.push(KvprotoBytesView { data: ptr, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.secondaries.data = ptr;
                repr.secondaries.len = len;
                repr.secondaries.cap = len;
            }
        }
    }
    repr.duration_to_last_update_ms = src.get_duration_to_last_update_ms();
    repr.is_txn_file = src.get_is_txn_file();
    arena.alloc_struct(repr)
}

pub fn lock_info_from_repr_generated(src: *const KvrpcpbLockInfo) -> Option<pb::LockInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::LockInfo::new();
    out.set_primary_lock(bytes_from(repr.primary_lock.data, repr.primary_lock.len).into());
    out.set_lock_version(repr.lock_version);
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_lock_ttl(repr.lock_ttl);
    out.set_txn_size(repr.txn_size);
    out.set_lock_type(pb::Op::from_i32(repr.lock_type).unwrap_or_default());
    out.set_lock_for_update_ts(repr.lock_for_update_ts);
    out.set_use_async_commit(repr.use_async_commit);
    out.set_min_commit_ts(repr.min_commit_ts);
    if !repr.secondaries.data.is_null() && repr.secondaries.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.secondaries.data, repr.secondaries.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_secondaries(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_duration_to_last_update_ms(repr.duration_to_last_update_ms);
    out.set_is_txn_file(repr.is_txn_file);
    Some(out)
}

pub fn mutation_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Mutation) -> &'a mut KvrpcpbMutation {
    let mut repr = KvrpcpbMutation {
        op: Default::default(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        assertion: Default::default(),
    };
    repr.op = src.get_op() as i32;
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if !src.get_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_value());
        repr.value.data = ptr;
        repr.value.len = len;
    }
    repr.assertion = src.get_assertion() as i32;
    arena.alloc_struct(repr)
}

pub fn mutation_from_repr_generated(src: *const KvrpcpbMutation) -> Option<pb::Mutation> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Mutation::new();
    out.set_op(pb::Op::from_i32(repr.op).unwrap_or_default());
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_value(bytes_from(repr.value.data, repr.value.len).into());
    out.set_assertion(pb::Assertion::from_i32(repr.assertion).unwrap_or_default());
    Some(out)
}

pub fn mvcc_debug_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MvccDebugInfo) -> &'a mut KvrpcpbMvccDebugInfo {
    let mut repr = KvrpcpbMvccDebugInfo {
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        mvcc: ptr::null_mut(),
    };
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if src.has_mvcc() {
        repr.mvcc = mvcc_info_to_repr_generated(arena, src.get_mvcc()) as *mut _;
    } else {
        repr.mvcc = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn mvcc_debug_info_from_repr_generated(src: *const KvrpcpbMvccDebugInfo) -> Option<pb::MvccDebugInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MvccDebugInfo::new();
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    if !repr.mvcc.is_null() {
        if let Some(value) = mvcc_info_from_repr_generated(repr.mvcc) {
            out.set_mvcc(value);
        }
    }
    Some(out)
}

pub fn mvcc_get_by_key_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MvccGetByKeyRequest) -> &'a mut KvrpcpbMvccGetByKeyRequest {
    let mut repr = KvrpcpbMvccGetByKeyRequest {
        context: ptr::null_mut(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn mvcc_get_by_key_request_from_repr_generated(src: *const KvrpcpbMvccGetByKeyRequest) -> Option<pb::MvccGetByKeyRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MvccGetByKeyRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    Some(out)
}

pub fn mvcc_get_by_key_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MvccGetByKeyResponse) -> &'a mut KvrpcpbMvccGetByKeyResponse {
    let mut repr = KvrpcpbMvccGetByKeyResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        info: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    if src.has_info() {
        repr.info = mvcc_info_to_repr_generated(arena, src.get_info()) as *mut _;
    } else {
        repr.info = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn mvcc_get_by_key_response_from_repr_generated(src: *const KvrpcpbMvccGetByKeyResponse) -> Option<pb::MvccGetByKeyResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MvccGetByKeyResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    if !repr.info.is_null() {
        if let Some(value) = mvcc_info_from_repr_generated(repr.info) {
            out.set_info(value);
        }
    }
    Some(out)
}

pub fn mvcc_get_by_start_ts_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MvccGetByStartTsRequest) -> &'a mut KvrpcpbMvccGetByStartTsRequest {
    let mut repr = KvrpcpbMvccGetByStartTsRequest {
        context: ptr::null_mut(),
        start_ts: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.start_ts = src.get_start_ts();
    arena.alloc_struct(repr)
}

pub fn mvcc_get_by_start_ts_request_from_repr_generated(src: *const KvrpcpbMvccGetByStartTsRequest) -> Option<pb::MvccGetByStartTsRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MvccGetByStartTsRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_ts(repr.start_ts);
    Some(out)
}

pub fn mvcc_get_by_start_ts_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MvccGetByStartTsResponse) -> &'a mut KvrpcpbMvccGetByStartTsResponse {
    let mut repr = KvrpcpbMvccGetByStartTsResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        info: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if src.has_info() {
        repr.info = mvcc_info_to_repr_generated(arena, src.get_info()) as *mut _;
    } else {
        repr.info = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn mvcc_get_by_start_ts_response_from_repr_generated(src: *const KvrpcpbMvccGetByStartTsResponse) -> Option<pb::MvccGetByStartTsResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MvccGetByStartTsResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    if !repr.info.is_null() {
        if let Some(value) = mvcc_info_from_repr_generated(repr.info) {
            out.set_info(value);
        }
    }
    Some(out)
}

pub fn mvcc_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MvccInfo) -> &'a mut KvrpcpbMvccInfo {
    let mut repr = KvrpcpbMvccInfo {
        lock: ptr::null_mut(),
        writes: KvprotoSliceKvrpcpbMvccWritePtr { data: ptr::null_mut(), len: 0, cap: 0 },
        values: KvprotoSliceKvrpcpbMvccValuePtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_lock() {
        repr.lock = mvcc_lock_to_repr_generated(arena, src.get_lock()) as *mut _;
    } else {
        repr.lock = ptr::null_mut();
    }
    {
        let values = src.get_writes();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbMvccWrite> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(mvcc_write_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.writes.data = ptr;
                repr.writes.len = len;
                repr.writes.cap = len;
            }
        }
    }
    {
        let values = src.get_values();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbMvccValue> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(mvcc_value_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.values.data = ptr;
                repr.values.len = len;
                repr.values.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn mvcc_info_from_repr_generated(src: *const KvrpcpbMvccInfo) -> Option<pb::MvccInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MvccInfo::new();
    if !repr.lock.is_null() {
        if let Some(value) = mvcc_lock_from_repr_generated(repr.lock) {
            out.set_lock(value);
        }
    }
    if !repr.writes.data.is_null() && repr.writes.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.writes.data, repr.writes.len) };
        let mut values: Vec<pb::MvccWrite> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = mvcc_write_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_writes(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.values.data.is_null() && repr.values.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.values.data, repr.values.len) };
        let mut values: Vec<pb::MvccValue> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = mvcc_value_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_values(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn mvcc_lock_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MvccLock) -> &'a mut KvrpcpbMvccLock {
    let mut repr = KvrpcpbMvccLock {
        type_field: Default::default(),
        start_ts: Default::default(),
        primary: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        short_value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        ttl: Default::default(),
        for_update_ts: Default::default(),
        txn_size: Default::default(),
        use_async_commit: Default::default(),
        secondaries: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        rollback_ts: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        last_change_ts: Default::default(),
        versions_to_last_change: Default::default(),
    };
    repr.type_field = src.get_type() as i32;
    repr.start_ts = src.get_start_ts();
    if !src.get_primary().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_primary());
        repr.primary.data = ptr;
        repr.primary.len = len;
    }
    if !src.get_short_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_short_value());
        repr.short_value.data = ptr;
        repr.short_value.len = len;
    }
    repr.ttl = src.get_ttl();
    repr.for_update_ts = src.get_for_update_ts();
    repr.txn_size = src.get_txn_size();
    repr.use_async_commit = src.get_use_async_commit();
    {
        let values = src.get_secondaries();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_bytes(value);
                views.push(KvprotoBytesView { data: ptr, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.secondaries.data = ptr;
                repr.secondaries.len = len;
                repr.secondaries.cap = len;
            }
        }
    }
    {
        let values = src.get_rollback_ts();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.rollback_ts.data = ptr;
            repr.rollback_ts.len = len;
            repr.rollback_ts.cap = len;
        }
    }
    repr.last_change_ts = src.get_last_change_ts();
    repr.versions_to_last_change = src.get_versions_to_last_change();
    arena.alloc_struct(repr)
}

pub fn mvcc_lock_from_repr_generated(src: *const KvrpcpbMvccLock) -> Option<pb::MvccLock> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MvccLock::new();
    out.set_type(pb::Op::from_i32(repr.type_field).unwrap_or_default());
    out.set_start_ts(repr.start_ts);
    out.set_primary(bytes_from(repr.primary.data, repr.primary.len).into());
    out.set_short_value(bytes_from(repr.short_value.data, repr.short_value.len).into());
    out.set_ttl(repr.ttl);
    out.set_for_update_ts(repr.for_update_ts);
    out.set_txn_size(repr.txn_size);
    out.set_use_async_commit(repr.use_async_commit);
    if !repr.secondaries.data.is_null() && repr.secondaries.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.secondaries.data, repr.secondaries.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_secondaries(::protobuf::RepeatedField::from_vec(values));
    }
    if !repr.rollback_ts.data.is_null() && repr.rollback_ts.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.rollback_ts.data, repr.rollback_ts.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_rollback_ts(values);
    }
    out.set_last_change_ts(repr.last_change_ts);
    out.set_versions_to_last_change(repr.versions_to_last_change);
    Some(out)
}

pub fn mvcc_value_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MvccValue) -> &'a mut KvrpcpbMvccValue {
    let mut repr = KvrpcpbMvccValue {
        start_ts: Default::default(),
        value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    repr.start_ts = src.get_start_ts();
    if !src.get_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_value());
        repr.value.data = ptr;
        repr.value.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn mvcc_value_from_repr_generated(src: *const KvrpcpbMvccValue) -> Option<pb::MvccValue> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MvccValue::new();
    out.set_start_ts(repr.start_ts);
    out.set_value(bytes_from(repr.value.data, repr.value.len).into());
    Some(out)
}

pub fn mvcc_write_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::MvccWrite) -> &'a mut KvrpcpbMvccWrite {
    let mut repr = KvrpcpbMvccWrite {
        type_field: Default::default(),
        start_ts: Default::default(),
        commit_ts: Default::default(),
        short_value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        has_overlapped_rollback: Default::default(),
        has_gc_fence: Default::default(),
        gc_fence: Default::default(),
        last_change_ts: Default::default(),
        versions_to_last_change: Default::default(),
    };
    repr.type_field = src.get_type() as i32;
    repr.start_ts = src.get_start_ts();
    repr.commit_ts = src.get_commit_ts();
    if !src.get_short_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_short_value());
        repr.short_value.data = ptr;
        repr.short_value.len = len;
    }
    repr.has_overlapped_rollback = src.get_has_overlapped_rollback();
    repr.has_gc_fence = src.get_has_gc_fence();
    repr.gc_fence = src.get_gc_fence();
    repr.last_change_ts = src.get_last_change_ts();
    repr.versions_to_last_change = src.get_versions_to_last_change();
    arena.alloc_struct(repr)
}

pub fn mvcc_write_from_repr_generated(src: *const KvrpcpbMvccWrite) -> Option<pb::MvccWrite> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::MvccWrite::new();
    out.set_type(pb::Op::from_i32(repr.type_field).unwrap_or_default());
    out.set_start_ts(repr.start_ts);
    out.set_commit_ts(repr.commit_ts);
    out.set_short_value(bytes_from(repr.short_value.data, repr.short_value.len).into());
    out.set_has_overlapped_rollback(repr.has_overlapped_rollback);
    out.set_has_gc_fence(repr.has_gc_fence);
    out.set_gc_fence(repr.gc_fence);
    out.set_last_change_ts(repr.last_change_ts);
    out.set_versions_to_last_change(repr.versions_to_last_change);
    Some(out)
}

pub fn pessimistic_lock_key_result_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PessimisticLockKeyResult) -> &'a mut KvrpcpbPessimisticLockKeyResult {
    let mut repr = KvrpcpbPessimisticLockKeyResult {
        type_field: Default::default(),
        value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        existence: Default::default(),
        locked_with_conflict_ts: Default::default(),
        skip_resolving_lock: Default::default(),
    };
    repr.type_field = src.get_type() as i32;
    if !src.get_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_value());
        repr.value.data = ptr;
        repr.value.len = len;
    }
    repr.existence = src.get_existence();
    repr.locked_with_conflict_ts = src.get_locked_with_conflict_ts();
    repr.skip_resolving_lock = src.get_skip_resolving_lock();
    arena.alloc_struct(repr)
}

pub fn pessimistic_lock_key_result_from_repr_generated(src: *const KvrpcpbPessimisticLockKeyResult) -> Option<pb::PessimisticLockKeyResult> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PessimisticLockKeyResult::new();
    out.set_type(pb::PessimisticLockKeyResultType::from_i32(repr.type_field).unwrap_or_default());
    out.set_value(bytes_from(repr.value.data, repr.value.len).into());
    out.set_existence(repr.existence);
    out.set_locked_with_conflict_ts(repr.locked_with_conflict_ts);
    out.set_skip_resolving_lock(repr.skip_resolving_lock);
    Some(out)
}

pub fn pessimistic_lock_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PessimisticLockRequest) -> &'a mut KvrpcpbPessimisticLockRequest {
    let mut repr = KvrpcpbPessimisticLockRequest {
        context: ptr::null_mut(),
        mutations: KvprotoSliceKvrpcpbMutationPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        primary_lock: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        start_version: Default::default(),
        lock_ttl: Default::default(),
        for_update_ts: Default::default(),
        is_first_lock: Default::default(),
        wait_timeout: Default::default(),
        force: Default::default(),
        return_values: Default::default(),
        min_commit_ts: Default::default(),
        check_existence: Default::default(),
        lock_only_if_exists: Default::default(),
        wake_up_mode: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    {
        let values = src.get_mutations();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbMutation> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(mutation_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.mutations.data = ptr;
                repr.mutations.len = len;
                repr.mutations.cap = len;
            }
        }
    }
    if !src.get_primary_lock().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_primary_lock());
        repr.primary_lock.data = ptr;
        repr.primary_lock.len = len;
    }
    repr.start_version = src.get_start_version();
    repr.lock_ttl = src.get_lock_ttl();
    repr.for_update_ts = src.get_for_update_ts();
    repr.is_first_lock = src.get_is_first_lock();
    repr.wait_timeout = src.get_wait_timeout();
    repr.force = src.get_force();
    repr.return_values = src.get_return_values();
    repr.min_commit_ts = src.get_min_commit_ts();
    repr.check_existence = src.get_check_existence();
    repr.lock_only_if_exists = src.get_lock_only_if_exists();
    repr.wake_up_mode = src.get_wake_up_mode() as i32;
    arena.alloc_struct(repr)
}

pub fn pessimistic_lock_request_from_repr_generated(src: *const KvrpcpbPessimisticLockRequest) -> Option<pb::PessimisticLockRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PessimisticLockRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.mutations.data.is_null() && repr.mutations.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.mutations.data, repr.mutations.len) };
        let mut values: Vec<pb::Mutation> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = mutation_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_mutations(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_primary_lock(bytes_from(repr.primary_lock.data, repr.primary_lock.len).into());
    out.set_start_version(repr.start_version);
    out.set_lock_ttl(repr.lock_ttl);
    out.set_for_update_ts(repr.for_update_ts);
    out.set_is_first_lock(repr.is_first_lock);
    out.set_wait_timeout(repr.wait_timeout);
    out.set_force(repr.force);
    out.set_return_values(repr.return_values);
    out.set_min_commit_ts(repr.min_commit_ts);
    out.set_check_existence(repr.check_existence);
    out.set_lock_only_if_exists(repr.lock_only_if_exists);
    out.set_wake_up_mode(pb::PessimisticLockWakeUpMode::from_i32(repr.wake_up_mode).unwrap_or_default());
    Some(out)
}

pub fn pessimistic_lock_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PessimisticLockResponse) -> &'a mut KvrpcpbPessimisticLockResponse {
    let mut repr = KvrpcpbPessimisticLockResponse {
        region_error: ptr::null_mut(),
        errors: KvprotoSliceKvrpcpbKeyErrorPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        commit_ts: Default::default(),
        value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        values: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        not_founds: KvprotoSliceBool { data: ptr::null_mut(), len: 0, cap: 0 },
        exec_details_v2: ptr::null_mut(),
        results: KvprotoSliceKvrpcpbPessimisticLockKeyResultPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    {
        let values = src.get_errors();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKeyError> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(key_error_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.errors.data = ptr;
                repr.errors.len = len;
                repr.errors.cap = len;
            }
        }
    }
    repr.commit_ts = src.get_commit_ts();
    if !src.get_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_value());
        repr.value.data = ptr;
        repr.value.len = len;
    }
    {
        let values = src.get_values();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_bytes(value);
                views.push(KvprotoBytesView { data: ptr, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.values.data = ptr;
                repr.values.len = len;
                repr.values.cap = len;
            }
        }
    }
    {
        let values = src.get_not_founds();
        if !values.is_empty() {
            let mut vec: Vec<bool> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.not_founds.data = ptr;
            repr.not_founds.len = len;
            repr.not_founds.cap = len;
        }
    }
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    {
        let values = src.get_results();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbPessimisticLockKeyResult> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(pessimistic_lock_key_result_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.results.data = ptr;
                repr.results.len = len;
                repr.results.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn pessimistic_lock_response_from_repr_generated(src: *const KvrpcpbPessimisticLockResponse) -> Option<pb::PessimisticLockResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PessimisticLockResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.errors.data.is_null() && repr.errors.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.errors.data, repr.errors.len) };
        let mut values: Vec<pb::KeyError> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = key_error_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_errors(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_commit_ts(repr.commit_ts);
    out.set_value(bytes_from(repr.value.data, repr.value.len).into());
    if !repr.values.data.is_null() && repr.values.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.values.data, repr.values.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_values(::protobuf::RepeatedField::from_vec(values));
    }
    if !repr.not_founds.data.is_null() && repr.not_founds.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.not_founds.data, repr.not_founds.len) };
        let mut values: Vec<bool> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_not_founds(values);
    }
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    if !repr.results.data.is_null() && repr.results.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.results.data, repr.results.len) };
        let mut values: Vec<pb::PessimisticLockKeyResult> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = pessimistic_lock_key_result_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_results(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn pessimistic_rollback_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PessimisticRollbackRequest) -> &'a mut KvrpcpbPessimisticRollbackRequest {
    let mut repr = KvrpcpbPessimisticRollbackRequest {
        context: ptr::null_mut(),
        start_version: Default::default(),
        for_update_ts: Default::default(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.start_version = src.get_start_version();
    repr.for_update_ts = src.get_for_update_ts();
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

pub fn pessimistic_rollback_request_from_repr_generated(src: *const KvrpcpbPessimisticRollbackRequest) -> Option<pb::PessimisticRollbackRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PessimisticRollbackRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_version(repr.start_version);
    out.set_for_update_ts(repr.for_update_ts);
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

pub fn pessimistic_rollback_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PessimisticRollbackResponse) -> &'a mut KvrpcpbPessimisticRollbackResponse {
    let mut repr = KvrpcpbPessimisticRollbackResponse {
        region_error: ptr::null_mut(),
        errors: KvprotoSliceKvrpcpbKeyErrorPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    {
        let values = src.get_errors();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKeyError> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(key_error_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.errors.data = ptr;
                repr.errors.len = len;
                repr.errors.cap = len;
            }
        }
    }
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn pessimistic_rollback_response_from_repr_generated(src: *const KvrpcpbPessimisticRollbackResponse) -> Option<pb::PessimisticRollbackResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PessimisticRollbackResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.errors.data.is_null() && repr.errors.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.errors.data, repr.errors.len) };
        let mut values: Vec<pb::KeyError> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = key_error_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_errors(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn physical_scan_lock_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PhysicalScanLockRequest) -> &'a mut KvrpcpbPhysicalScanLockRequest {
    let mut repr = KvrpcpbPhysicalScanLockRequest {
        context: ptr::null_mut(),
        max_ts: Default::default(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        limit: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.max_ts = src.get_max_ts();
    if !src.get_start_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_start_key());
        repr.start_key.data = ptr;
        repr.start_key.len = len;
    }
    repr.limit = src.get_limit();
    arena.alloc_struct(repr)
}

pub fn physical_scan_lock_request_from_repr_generated(src: *const KvrpcpbPhysicalScanLockRequest) -> Option<pb::PhysicalScanLockRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PhysicalScanLockRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_max_ts(repr.max_ts);
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_limit(repr.limit);
    Some(out)
}

pub fn physical_scan_lock_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PhysicalScanLockResponse) -> &'a mut KvrpcpbPhysicalScanLockResponse {
    let mut repr = KvrpcpbPhysicalScanLockResponse {
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        locks: KvprotoSliceKvrpcpbLockInfoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    {
        let values = src.get_locks();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbLockInfo> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(lock_info_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.locks.data = ptr;
                repr.locks.len = len;
                repr.locks.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn physical_scan_lock_response_from_repr_generated(src: *const KvrpcpbPhysicalScanLockResponse) -> Option<pb::PhysicalScanLockResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PhysicalScanLockResponse::new();
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    if !repr.locks.data.is_null() && repr.locks.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.locks.data, repr.locks.len) };
        let mut values: Vec<pb::LockInfo> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = lock_info_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_locks(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn prepare_flashback_to_version_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PrepareFlashbackToVersionRequest) -> &'a mut KvrpcpbPrepareFlashbackToVersionRequest {
    let mut repr = KvrpcpbPrepareFlashbackToVersionRequest {
        context: ptr::null_mut(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        start_ts: Default::default(),
        version: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
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
    repr.start_ts = src.get_start_ts();
    repr.version = src.get_version();
    arena.alloc_struct(repr)
}

pub fn prepare_flashback_to_version_request_from_repr_generated(src: *const KvrpcpbPrepareFlashbackToVersionRequest) -> Option<pb::PrepareFlashbackToVersionRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PrepareFlashbackToVersionRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    out.set_start_ts(repr.start_ts);
    out.set_version(repr.version);
    Some(out)
}

pub fn prepare_flashback_to_version_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PrepareFlashbackToVersionResponse) -> &'a mut KvrpcpbPrepareFlashbackToVersionResponse {
    let mut repr = KvrpcpbPrepareFlashbackToVersionResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn prepare_flashback_to_version_response_from_repr_generated(src: *const KvrpcpbPrepareFlashbackToVersionResponse) -> Option<pb::PrepareFlashbackToVersionResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PrepareFlashbackToVersionResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn prewrite_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PrewriteRequest) -> &'a mut KvrpcpbPrewriteRequest {
    let mut repr = KvrpcpbPrewriteRequest {
        context: ptr::null_mut(),
        mutations: KvprotoSliceKvrpcpbMutationPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        primary_lock: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        start_version: Default::default(),
        lock_ttl: Default::default(),
        skip_constraint_check: Default::default(),
        pessimistic_actions: KvprotoSliceInt32T { data: ptr::null_mut(), len: 0, cap: 0 },
        txn_size: Default::default(),
        for_update_ts: Default::default(),
        min_commit_ts: Default::default(),
        use_async_commit: Default::default(),
        secondaries: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        try_one_pc: Default::default(),
        max_commit_ts: Default::default(),
        assertion_level: Default::default(),
        for_update_ts_constraints: KvprotoSliceKvrpcpbPrewriteRequestForUpdateTSConstraintPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        txn_file_chunks: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    {
        let values = src.get_mutations();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbMutation> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(mutation_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.mutations.data = ptr;
                repr.mutations.len = len;
                repr.mutations.cap = len;
            }
        }
    }
    if !src.get_primary_lock().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_primary_lock());
        repr.primary_lock.data = ptr;
        repr.primary_lock.len = len;
    }
    repr.start_version = src.get_start_version();
    repr.lock_ttl = src.get_lock_ttl();
    repr.skip_constraint_check = src.get_skip_constraint_check();
    {
        let values = src.get_pessimistic_actions();
        if !values.is_empty() {
            let mut vec: Vec<i32> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(value.value());
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.pessimistic_actions.data = ptr;
            repr.pessimistic_actions.len = len;
            repr.pessimistic_actions.cap = len;
        }
    }
    repr.txn_size = src.get_txn_size();
    repr.for_update_ts = src.get_for_update_ts();
    repr.min_commit_ts = src.get_min_commit_ts();
    repr.use_async_commit = src.get_use_async_commit();
    {
        let values = src.get_secondaries();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_bytes(value);
                views.push(KvprotoBytesView { data: ptr, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.secondaries.data = ptr;
                repr.secondaries.len = len;
                repr.secondaries.cap = len;
            }
        }
    }
    repr.try_one_pc = src.get_try_one_pc();
    repr.max_commit_ts = src.get_max_commit_ts();
    repr.assertion_level = src.get_assertion_level() as i32;
    {
        let values = src.get_for_update_ts_constraints();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbPrewriteRequestForUpdateTSConstraint> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(prewrite_request__for_update_t_s_constraint_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.for_update_ts_constraints.data = ptr;
                repr.for_update_ts_constraints.len = len;
                repr.for_update_ts_constraints.cap = len;
            }
        }
    }
    {
        let values = src.get_txn_file_chunks();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.txn_file_chunks.data = ptr;
            repr.txn_file_chunks.len = len;
            repr.txn_file_chunks.cap = len;
        }
    }
    arena.alloc_struct(repr)
}

pub fn prewrite_request_from_repr_generated(src: *const KvrpcpbPrewriteRequest) -> Option<pb::PrewriteRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PrewriteRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.mutations.data.is_null() && repr.mutations.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.mutations.data, repr.mutations.len) };
        let mut values: Vec<pb::Mutation> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = mutation_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_mutations(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_primary_lock(bytes_from(repr.primary_lock.data, repr.primary_lock.len).into());
    out.set_start_version(repr.start_version);
    out.set_lock_ttl(repr.lock_ttl);
    out.set_skip_constraint_check(repr.skip_constraint_check);
    if !repr.pessimistic_actions.data.is_null() && repr.pessimistic_actions.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.pessimistic_actions.data, repr.pessimistic_actions.len) };
        let mut values: Vec<pb::PrewriteRequestPessimisticAction> = Vec::with_capacity(slice.len());
        for &value in slice {
            values.push(pb::PrewriteRequestPessimisticAction::from_i32(value).unwrap_or_default());
        }
        out.set_pessimistic_actions(values);
    }
    out.set_txn_size(repr.txn_size);
    out.set_for_update_ts(repr.for_update_ts);
    out.set_min_commit_ts(repr.min_commit_ts);
    out.set_use_async_commit(repr.use_async_commit);
    if !repr.secondaries.data.is_null() && repr.secondaries.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.secondaries.data, repr.secondaries.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_secondaries(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_try_one_pc(repr.try_one_pc);
    out.set_max_commit_ts(repr.max_commit_ts);
    out.set_assertion_level(pb::AssertionLevel::from_i32(repr.assertion_level).unwrap_or_default());
    if !repr.for_update_ts_constraints.data.is_null() && repr.for_update_ts_constraints.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.for_update_ts_constraints.data, repr.for_update_ts_constraints.len) };
        let mut values: Vec<pb::PrewriteRequestForUpdateTsConstraint> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = prewrite_request__for_update_t_s_constraint_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_for_update_ts_constraints(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.txn_file_chunks.data.is_null() && repr.txn_file_chunks.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.txn_file_chunks.data, repr.txn_file_chunks.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_txn_file_chunks(values);
    }
    Some(out)
}

pub fn prewrite_request__for_update_t_s_constraint_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PrewriteRequestForUpdateTsConstraint) -> &'a mut KvrpcpbPrewriteRequestForUpdateTSConstraint {
    let mut repr = KvrpcpbPrewriteRequestForUpdateTSConstraint {
        index: Default::default(),
        expected_for_update_ts: Default::default(),
    };
    repr.index = src.get_index();
    repr.expected_for_update_ts = src.get_expected_for_update_ts();
    arena.alloc_struct(repr)
}

pub fn prewrite_request__for_update_t_s_constraint_from_repr_generated(src: *const KvrpcpbPrewriteRequestForUpdateTSConstraint) -> Option<pb::PrewriteRequestForUpdateTsConstraint> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PrewriteRequestForUpdateTsConstraint::new();
    out.set_index(repr.index);
    out.set_expected_for_update_ts(repr.expected_for_update_ts);
    Some(out)
}

pub fn prewrite_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PrewriteResponse) -> &'a mut KvrpcpbPrewriteResponse {
    let mut repr = KvrpcpbPrewriteResponse {
        region_error: ptr::null_mut(),
        errors: KvprotoSliceKvrpcpbKeyErrorPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        min_commit_ts: Default::default(),
        one_pc_commit_ts: Default::default(),
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    {
        let values = src.get_errors();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKeyError> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(key_error_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.errors.data = ptr;
                repr.errors.len = len;
                repr.errors.cap = len;
            }
        }
    }
    repr.min_commit_ts = src.get_min_commit_ts();
    repr.one_pc_commit_ts = src.get_one_pc_commit_ts();
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn prewrite_response_from_repr_generated(src: *const KvrpcpbPrewriteResponse) -> Option<pb::PrewriteResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PrewriteResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.errors.data.is_null() && repr.errors.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.errors.data, repr.errors.len) };
        let mut values: Vec<pb::KeyError> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = key_error_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_errors(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_min_commit_ts(repr.min_commit_ts);
    out.set_one_pc_commit_ts(repr.one_pc_commit_ts);
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn primary_mismatch_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PrimaryMismatch) -> &'a mut KvrpcpbPrimaryMismatch {
    let mut repr = KvrpcpbPrimaryMismatch {
        lock_info: ptr::null_mut(),
    };
    if src.has_lock_info() {
        repr.lock_info = lock_info_to_repr_generated(arena, src.get_lock_info()) as *mut _;
    } else {
        repr.lock_info = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn primary_mismatch_from_repr_generated(src: *const KvrpcpbPrimaryMismatch) -> Option<pb::PrimaryMismatch> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PrimaryMismatch::new();
    if !repr.lock_info.is_null() {
        if let Some(value) = lock_info_from_repr_generated(repr.lock_info) {
            out.set_lock_info(value);
        }
    }
    Some(out)
}

pub fn raw_batch_delete_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawBatchDeleteRequest) -> &'a mut KvrpcpbRawBatchDeleteRequest {
    let mut repr = KvrpcpbRawBatchDeleteRequest {
        context: ptr::null_mut(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
        for_cas: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
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
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    repr.for_cas = src.get_for_cas();
    arena.alloc_struct(repr)
}

pub fn raw_batch_delete_request_from_repr_generated(src: *const KvrpcpbRawBatchDeleteRequest) -> Option<pb::RawBatchDeleteRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawBatchDeleteRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    out.set_for_cas(repr.for_cas);
    Some(out)
}

pub fn raw_batch_delete_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawBatchDeleteResponse) -> &'a mut KvrpcpbRawBatchDeleteResponse {
    let mut repr = KvrpcpbRawBatchDeleteResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_batch_delete_response_from_repr_generated(src: *const KvrpcpbRawBatchDeleteResponse) -> Option<pb::RawBatchDeleteResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawBatchDeleteResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn raw_batch_get_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawBatchGetRequest) -> &'a mut KvrpcpbRawBatchGetRequest {
    let mut repr = KvrpcpbRawBatchGetRequest {
        context: ptr::null_mut(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
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
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_batch_get_request_from_repr_generated(src: *const KvrpcpbRawBatchGetRequest) -> Option<pb::RawBatchGetRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawBatchGetRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    Some(out)
}

pub fn raw_batch_get_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawBatchGetResponse) -> &'a mut KvrpcpbRawBatchGetResponse {
    let mut repr = KvrpcpbRawBatchGetResponse {
        region_error: ptr::null_mut(),
        pairs: KvprotoSliceKvrpcpbKvPairPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    {
        let values = src.get_pairs();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKvPair> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(kv_pair_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.pairs.data = ptr;
                repr.pairs.len = len;
                repr.pairs.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn raw_batch_get_response_from_repr_generated(src: *const KvrpcpbRawBatchGetResponse) -> Option<pb::RawBatchGetResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawBatchGetResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.pairs.data.is_null() && repr.pairs.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.pairs.data, repr.pairs.len) };
        let mut values: Vec<pb::KvPair> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = kv_pair_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_pairs(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn raw_batch_put_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawBatchPutRequest) -> &'a mut KvrpcpbRawBatchPutRequest {
    let mut repr = KvrpcpbRawBatchPutRequest {
        context: ptr::null_mut(),
        pairs: KvprotoSliceKvrpcpbKvPairPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
        ttl: Default::default(),
        for_cas: Default::default(),
        ttls: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    {
        let values = src.get_pairs();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKvPair> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(kv_pair_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.pairs.data = ptr;
                repr.pairs.len = len;
                repr.pairs.cap = len;
            }
        }
    }
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    repr.ttl = src.get_ttl();
    repr.for_cas = src.get_for_cas();
    {
        let values = src.get_ttls();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.ttls.data = ptr;
            repr.ttls.len = len;
            repr.ttls.cap = len;
        }
    }
    arena.alloc_struct(repr)
}

pub fn raw_batch_put_request_from_repr_generated(src: *const KvrpcpbRawBatchPutRequest) -> Option<pb::RawBatchPutRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawBatchPutRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.pairs.data.is_null() && repr.pairs.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.pairs.data, repr.pairs.len) };
        let mut values: Vec<pb::KvPair> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = kv_pair_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_pairs(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    out.set_ttl(repr.ttl);
    out.set_for_cas(repr.for_cas);
    if !repr.ttls.data.is_null() && repr.ttls.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.ttls.data, repr.ttls.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_ttls(values);
    }
    Some(out)
}

pub fn raw_batch_put_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawBatchPutResponse) -> &'a mut KvrpcpbRawBatchPutResponse {
    let mut repr = KvrpcpbRawBatchPutResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_batch_put_response_from_repr_generated(src: *const KvrpcpbRawBatchPutResponse) -> Option<pb::RawBatchPutResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawBatchPutResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn raw_batch_scan_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawBatchScanRequest) -> &'a mut KvrpcpbRawBatchScanRequest {
    let mut repr = KvrpcpbRawBatchScanRequest {
        context: ptr::null_mut(),
        ranges: KvprotoSliceKvrpcpbKeyRangePtr { data: ptr::null_mut(), len: 0, cap: 0 },
        each_limit: Default::default(),
        key_only: Default::default(),
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
        reverse: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    {
        let values = src.get_ranges();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKeyRange> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(key_range_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.ranges.data = ptr;
                repr.ranges.len = len;
                repr.ranges.cap = len;
            }
        }
    }
    repr.each_limit = src.get_each_limit();
    repr.key_only = src.get_key_only();
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    repr.reverse = src.get_reverse();
    arena.alloc_struct(repr)
}

pub fn raw_batch_scan_request_from_repr_generated(src: *const KvrpcpbRawBatchScanRequest) -> Option<pb::RawBatchScanRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawBatchScanRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    if !repr.ranges.data.is_null() && repr.ranges.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.ranges.data, repr.ranges.len) };
        let mut values: Vec<pb::KeyRange> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = key_range_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_ranges(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_each_limit(repr.each_limit);
    out.set_key_only(repr.key_only);
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    out.set_reverse(repr.reverse);
    Some(out)
}

pub fn raw_batch_scan_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawBatchScanResponse) -> &'a mut KvrpcpbRawBatchScanResponse {
    let mut repr = KvrpcpbRawBatchScanResponse {
        region_error: ptr::null_mut(),
        kvs: KvprotoSliceKvrpcpbKvPairPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    {
        let values = src.get_kvs();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKvPair> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(kv_pair_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.kvs.data = ptr;
                repr.kvs.len = len;
                repr.kvs.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn raw_batch_scan_response_from_repr_generated(src: *const KvrpcpbRawBatchScanResponse) -> Option<pb::RawBatchScanResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawBatchScanResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.kvs.data.is_null() && repr.kvs.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.kvs.data, repr.kvs.len) };
        let mut values: Vec<pb::KvPair> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = kv_pair_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_kvs(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn raw_c_a_s_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawCasRequest) -> &'a mut KvrpcpbRawCASRequest {
    let mut repr = KvrpcpbRawCASRequest {
        context: ptr::null_mut(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        previous_not_exist: Default::default(),
        previous_value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
        ttl: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if !src.get_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_value());
        repr.value.data = ptr;
        repr.value.len = len;
    }
    repr.previous_not_exist = src.get_previous_not_exist();
    if !src.get_previous_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_previous_value());
        repr.previous_value.data = ptr;
        repr.previous_value.len = len;
    }
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    repr.ttl = src.get_ttl();
    arena.alloc_struct(repr)
}

pub fn raw_c_a_s_request_from_repr_generated(src: *const KvrpcpbRawCASRequest) -> Option<pb::RawCasRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawCasRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_value(bytes_from(repr.value.data, repr.value.len).into());
    out.set_previous_not_exist(repr.previous_not_exist);
    out.set_previous_value(bytes_from(repr.previous_value.data, repr.previous_value.len).into());
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    out.set_ttl(repr.ttl);
    Some(out)
}

pub fn raw_c_a_s_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawCasResponse) -> &'a mut KvrpcpbRawCASResponse {
    let mut repr = KvrpcpbRawCASResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        succeed: Default::default(),
        previous_not_exist: Default::default(),
        previous_value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    repr.succeed = src.get_succeed();
    repr.previous_not_exist = src.get_previous_not_exist();
    if !src.get_previous_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_previous_value());
        repr.previous_value.data = ptr;
        repr.previous_value.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_c_a_s_response_from_repr_generated(src: *const KvrpcpbRawCASResponse) -> Option<pb::RawCasResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawCasResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    out.set_succeed(repr.succeed);
    out.set_previous_not_exist(repr.previous_not_exist);
    out.set_previous_value(bytes_from(repr.previous_value.data, repr.previous_value.len).into());
    Some(out)
}

pub fn raw_checksum_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawChecksumRequest) -> &'a mut KvrpcpbRawChecksumRequest {
    let mut repr = KvrpcpbRawChecksumRequest {
        context: ptr::null_mut(),
        algorithm: Default::default(),
        ranges: KvprotoSliceKvrpcpbKeyRangePtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.algorithm = src.get_algorithm() as i32;
    {
        let values = src.get_ranges();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKeyRange> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(key_range_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.ranges.data = ptr;
                repr.ranges.len = len;
                repr.ranges.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn raw_checksum_request_from_repr_generated(src: *const KvrpcpbRawChecksumRequest) -> Option<pb::RawChecksumRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawChecksumRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_algorithm(pb::ChecksumAlgorithm::from_i32(repr.algorithm).unwrap_or_default());
    if !repr.ranges.data.is_null() && repr.ranges.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.ranges.data, repr.ranges.len) };
        let mut values: Vec<pb::KeyRange> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = key_range_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_ranges(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn raw_checksum_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawChecksumResponse) -> &'a mut KvrpcpbRawChecksumResponse {
    let mut repr = KvrpcpbRawChecksumResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        checksum: Default::default(),
        total_kvs: Default::default(),
        total_bytes: Default::default(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    repr.checksum = src.get_checksum();
    repr.total_kvs = src.get_total_kvs();
    repr.total_bytes = src.get_total_bytes();
    arena.alloc_struct(repr)
}

pub fn raw_checksum_response_from_repr_generated(src: *const KvrpcpbRawChecksumResponse) -> Option<pb::RawChecksumResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawChecksumResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    out.set_checksum(repr.checksum);
    out.set_total_kvs(repr.total_kvs);
    out.set_total_bytes(repr.total_bytes);
    Some(out)
}

pub fn raw_coprocessor_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawCoprocessorRequest) -> &'a mut KvrpcpbRawCoprocessorRequest {
    let mut repr = KvrpcpbRawCoprocessorRequest {
        context: ptr::null_mut(),
        copr_name: KvprotoStringView { data: ptr::null(), len: 0 },
        copr_version_req: KvprotoStringView { data: ptr::null(), len: 0 },
        ranges: KvprotoSliceKvrpcpbKeyRangePtr { data: ptr::null_mut(), len: 0, cap: 0 },
        data: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_copr_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_copr_name());
        repr.copr_name.data = ptr as *const c_char;
        repr.copr_name.len = len;
    }
    if !src.get_copr_version_req().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_copr_version_req());
        repr.copr_version_req.data = ptr as *const c_char;
        repr.copr_version_req.len = len;
    }
    {
        let values = src.get_ranges();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKeyRange> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(key_range_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.ranges.data = ptr;
                repr.ranges.len = len;
                repr.ranges.cap = len;
            }
        }
    }
    if !src.get_data().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_data());
        repr.data.data = ptr;
        repr.data.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_coprocessor_request_from_repr_generated(src: *const KvrpcpbRawCoprocessorRequest) -> Option<pb::RawCoprocessorRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawCoprocessorRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_copr_name(string_from(repr.copr_name.data as *const u8, repr.copr_name.len));
    out.set_copr_version_req(string_from(repr.copr_version_req.data as *const u8, repr.copr_version_req.len));
    if !repr.ranges.data.is_null() && repr.ranges.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.ranges.data, repr.ranges.len) };
        let mut values: Vec<pb::KeyRange> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = key_range_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_ranges(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_data(bytes_from(repr.data.data, repr.data.len).into());
    Some(out)
}

pub fn raw_coprocessor_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawCoprocessorResponse) -> &'a mut KvrpcpbRawCoprocessorResponse {
    let mut repr = KvrpcpbRawCoprocessorResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        data: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    if !src.get_data().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_data());
        repr.data.data = ptr;
        repr.data.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_coprocessor_response_from_repr_generated(src: *const KvrpcpbRawCoprocessorResponse) -> Option<pb::RawCoprocessorResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawCoprocessorResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    out.set_data(bytes_from(repr.data.data, repr.data.len).into());
    Some(out)
}

pub fn raw_delete_range_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawDeleteRangeRequest) -> &'a mut KvrpcpbRawDeleteRangeRequest {
    let mut repr = KvrpcpbRawDeleteRangeRequest {
        context: ptr::null_mut(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
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
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_delete_range_request_from_repr_generated(src: *const KvrpcpbRawDeleteRangeRequest) -> Option<pb::RawDeleteRangeRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawDeleteRangeRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    Some(out)
}

pub fn raw_delete_range_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawDeleteRangeResponse) -> &'a mut KvrpcpbRawDeleteRangeResponse {
    let mut repr = KvrpcpbRawDeleteRangeResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_delete_range_response_from_repr_generated(src: *const KvrpcpbRawDeleteRangeResponse) -> Option<pb::RawDeleteRangeResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawDeleteRangeResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn raw_delete_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawDeleteRequest) -> &'a mut KvrpcpbRawDeleteRequest {
    let mut repr = KvrpcpbRawDeleteRequest {
        context: ptr::null_mut(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
        for_cas: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    repr.for_cas = src.get_for_cas();
    arena.alloc_struct(repr)
}

pub fn raw_delete_request_from_repr_generated(src: *const KvrpcpbRawDeleteRequest) -> Option<pb::RawDeleteRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawDeleteRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    out.set_for_cas(repr.for_cas);
    Some(out)
}

pub fn raw_delete_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawDeleteResponse) -> &'a mut KvrpcpbRawDeleteResponse {
    let mut repr = KvrpcpbRawDeleteResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_delete_response_from_repr_generated(src: *const KvrpcpbRawDeleteResponse) -> Option<pb::RawDeleteResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawDeleteResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn raw_get_key_t_t_l_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawGetKeyTtlRequest) -> &'a mut KvrpcpbRawGetKeyTTLRequest {
    let mut repr = KvrpcpbRawGetKeyTTLRequest {
        context: ptr::null_mut(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_get_key_t_t_l_request_from_repr_generated(src: *const KvrpcpbRawGetKeyTTLRequest) -> Option<pb::RawGetKeyTtlRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawGetKeyTtlRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    Some(out)
}

pub fn raw_get_key_t_t_l_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawGetKeyTtlResponse) -> &'a mut KvrpcpbRawGetKeyTTLResponse {
    let mut repr = KvrpcpbRawGetKeyTTLResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        ttl: Default::default(),
        not_found: Default::default(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    repr.ttl = src.get_ttl();
    repr.not_found = src.get_not_found();
    arena.alloc_struct(repr)
}

pub fn raw_get_key_t_t_l_response_from_repr_generated(src: *const KvrpcpbRawGetKeyTTLResponse) -> Option<pb::RawGetKeyTtlResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawGetKeyTtlResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    out.set_ttl(repr.ttl);
    out.set_not_found(repr.not_found);
    Some(out)
}

pub fn raw_get_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawGetRequest) -> &'a mut KvrpcpbRawGetRequest {
    let mut repr = KvrpcpbRawGetRequest {
        context: ptr::null_mut(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_get_request_from_repr_generated(src: *const KvrpcpbRawGetRequest) -> Option<pb::RawGetRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawGetRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    Some(out)
}

pub fn raw_get_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawGetResponse) -> &'a mut KvrpcpbRawGetResponse {
    let mut repr = KvrpcpbRawGetResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
        value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        not_found: Default::default(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    if !src.get_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_value());
        repr.value.data = ptr;
        repr.value.len = len;
    }
    repr.not_found = src.get_not_found();
    arena.alloc_struct(repr)
}

pub fn raw_get_response_from_repr_generated(src: *const KvrpcpbRawGetResponse) -> Option<pb::RawGetResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawGetResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    out.set_value(bytes_from(repr.value.data, repr.value.len).into());
    out.set_not_found(repr.not_found);
    Some(out)
}

pub fn raw_put_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawPutRequest) -> &'a mut KvrpcpbRawPutRequest {
    let mut repr = KvrpcpbRawPutRequest {
        context: ptr::null_mut(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        value: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
        ttl: Default::default(),
        for_cas: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if !src.get_value().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_value());
        repr.value.data = ptr;
        repr.value.len = len;
    }
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    repr.ttl = src.get_ttl();
    repr.for_cas = src.get_for_cas();
    arena.alloc_struct(repr)
}

pub fn raw_put_request_from_repr_generated(src: *const KvrpcpbRawPutRequest) -> Option<pb::RawPutRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawPutRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_value(bytes_from(repr.value.data, repr.value.len).into());
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    out.set_ttl(repr.ttl);
    out.set_for_cas(repr.for_cas);
    Some(out)
}

pub fn raw_put_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawPutResponse) -> &'a mut KvrpcpbRawPutResponse {
    let mut repr = KvrpcpbRawPutResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_put_response_from_repr_generated(src: *const KvrpcpbRawPutResponse) -> Option<pb::RawPutResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawPutResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn raw_scan_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawScanRequest) -> &'a mut KvrpcpbRawScanRequest {
    let mut repr = KvrpcpbRawScanRequest {
        context: ptr::null_mut(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        limit: Default::default(),
        key_only: Default::default(),
        cf: KvprotoStringView { data: ptr::null(), len: 0 },
        reverse: Default::default(),
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_start_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_start_key());
        repr.start_key.data = ptr;
        repr.start_key.len = len;
    }
    repr.limit = src.get_limit();
    repr.key_only = src.get_key_only();
    if !src.get_cf().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_cf());
        repr.cf.data = ptr as *const c_char;
        repr.cf.len = len;
    }
    repr.reverse = src.get_reverse();
    if !src.get_end_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_end_key());
        repr.end_key.data = ptr;
        repr.end_key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn raw_scan_request_from_repr_generated(src: *const KvrpcpbRawScanRequest) -> Option<pb::RawScanRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawScanRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_limit(repr.limit);
    out.set_key_only(repr.key_only);
    out.set_cf(string_from(repr.cf.data as *const u8, repr.cf.len));
    out.set_reverse(repr.reverse);
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    Some(out)
}

pub fn raw_scan_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawScanResponse) -> &'a mut KvrpcpbRawScanResponse {
    let mut repr = KvrpcpbRawScanResponse {
        region_error: ptr::null_mut(),
        kvs: KvprotoSliceKvrpcpbKvPairPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    {
        let values = src.get_kvs();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKvPair> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(kv_pair_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.kvs.data = ptr;
                repr.kvs.len = len;
                repr.kvs.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn raw_scan_response_from_repr_generated(src: *const KvrpcpbRawScanResponse) -> Option<pb::RawScanResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawScanResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.kvs.data.is_null() && repr.kvs.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.kvs.data, repr.kvs.len) };
        let mut values: Vec<pb::KvPair> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = kv_pair_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_kvs(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn read_index_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ReadIndexRequest) -> &'a mut KvrpcpbReadIndexRequest {
    let mut repr = KvrpcpbReadIndexRequest {
        context: ptr::null_mut(),
        start_ts: Default::default(),
        ranges: KvprotoSliceKvrpcpbKeyRangePtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.start_ts = src.get_start_ts();
    {
        let values = src.get_ranges();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKeyRange> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(key_range_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.ranges.data = ptr;
                repr.ranges.len = len;
                repr.ranges.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn read_index_request_from_repr_generated(src: *const KvrpcpbReadIndexRequest) -> Option<pb::ReadIndexRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ReadIndexRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_ts(repr.start_ts);
    if !repr.ranges.data.is_null() && repr.ranges.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.ranges.data, repr.ranges.len) };
        let mut values: Vec<pb::KeyRange> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = key_range_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_ranges(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn read_index_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ReadIndexResponse) -> &'a mut KvrpcpbReadIndexResponse {
    let mut repr = KvrpcpbReadIndexResponse {
        region_error: ptr::null_mut(),
        read_index: Default::default(),
        locked: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    repr.read_index = src.get_read_index();
    if src.has_locked() {
        repr.locked = lock_info_to_repr_generated(arena, src.get_locked()) as *mut _;
    } else {
        repr.locked = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn read_index_response_from_repr_generated(src: *const KvrpcpbReadIndexResponse) -> Option<pb::ReadIndexResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ReadIndexResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_read_index(repr.read_index);
    if !repr.locked.is_null() {
        if let Some(value) = lock_info_from_repr_generated(repr.locked) {
            out.set_locked(value);
        }
    }
    Some(out)
}

pub fn read_state_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ReadState) -> &'a mut KvrpcpbReadState {
    let mut repr = KvrpcpbReadState {
        applied_index: Default::default(),
        safe_ts: Default::default(),
    };
    repr.applied_index = src.get_applied_index();
    repr.safe_ts = src.get_safe_ts();
    arena.alloc_struct(repr)
}

pub fn read_state_from_repr_generated(src: *const KvrpcpbReadState) -> Option<pb::ReadState> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ReadState::new();
    out.set_applied_index(repr.applied_index);
    out.set_safe_ts(repr.safe_ts);
    Some(out)
}

pub fn register_lock_observer_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RegisterLockObserverRequest) -> &'a mut KvrpcpbRegisterLockObserverRequest {
    let mut repr = KvrpcpbRegisterLockObserverRequest {
        context: ptr::null_mut(),
        max_ts: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.max_ts = src.get_max_ts();
    arena.alloc_struct(repr)
}

pub fn register_lock_observer_request_from_repr_generated(src: *const KvrpcpbRegisterLockObserverRequest) -> Option<pb::RegisterLockObserverRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RegisterLockObserverRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_max_ts(repr.max_ts);
    Some(out)
}

pub fn register_lock_observer_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RegisterLockObserverResponse) -> &'a mut KvrpcpbRegisterLockObserverResponse {
    let mut repr = KvrpcpbRegisterLockObserverResponse {
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn register_lock_observer_response_from_repr_generated(src: *const KvrpcpbRegisterLockObserverResponse) -> Option<pb::RegisterLockObserverResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RegisterLockObserverResponse::new();
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn remove_lock_observer_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RemoveLockObserverRequest) -> &'a mut KvrpcpbRemoveLockObserverRequest {
    let mut repr = KvrpcpbRemoveLockObserverRequest {
        context: ptr::null_mut(),
        max_ts: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.max_ts = src.get_max_ts();
    arena.alloc_struct(repr)
}

pub fn remove_lock_observer_request_from_repr_generated(src: *const KvrpcpbRemoveLockObserverRequest) -> Option<pb::RemoveLockObserverRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RemoveLockObserverRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_max_ts(repr.max_ts);
    Some(out)
}

pub fn remove_lock_observer_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RemoveLockObserverResponse) -> &'a mut KvrpcpbRemoveLockObserverResponse {
    let mut repr = KvrpcpbRemoveLockObserverResponse {
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn remove_lock_observer_response_from_repr_generated(src: *const KvrpcpbRemoveLockObserverResponse) -> Option<pb::RemoveLockObserverResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RemoveLockObserverResponse::new();
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn resolve_lock_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ResolveLockRequest) -> &'a mut KvrpcpbResolveLockRequest {
    let mut repr = KvrpcpbResolveLockRequest {
        context: ptr::null_mut(),
        start_version: Default::default(),
        commit_version: Default::default(),
        txn_infos: KvprotoSliceKvrpcpbTxnInfoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        is_txn_file: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.start_version = src.get_start_version();
    repr.commit_version = src.get_commit_version();
    {
        let values = src.get_txn_infos();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbTxnInfo> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(txn_info_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.txn_infos.data = ptr;
                repr.txn_infos.len = len;
                repr.txn_infos.cap = len;
            }
        }
    }
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
    repr.is_txn_file = src.get_is_txn_file();
    arena.alloc_struct(repr)
}

pub fn resolve_lock_request_from_repr_generated(src: *const KvrpcpbResolveLockRequest) -> Option<pb::ResolveLockRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ResolveLockRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_version(repr.start_version);
    out.set_commit_version(repr.commit_version);
    if !repr.txn_infos.data.is_null() && repr.txn_infos.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.txn_infos.data, repr.txn_infos.len) };
        let mut values: Vec<pb::TxnInfo> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = txn_info_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_txn_infos(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_is_txn_file(repr.is_txn_file);
    Some(out)
}

pub fn resolve_lock_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ResolveLockResponse) -> &'a mut KvrpcpbResolveLockResponse {
    let mut repr = KvrpcpbResolveLockResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn resolve_lock_response_from_repr_generated(src: *const KvrpcpbResolveLockResponse) -> Option<pb::ResolveLockResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ResolveLockResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn resource_control_context_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ResourceControlContext) -> &'a mut KvrpcpbResourceControlContext {
    let mut repr = KvrpcpbResourceControlContext {
        resource_group_name: KvprotoStringView { data: ptr::null(), len: 0 },
        penalty: ptr::null_mut(),
        override_priority: Default::default(),
    };
    if !src.get_resource_group_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_resource_group_name());
        repr.resource_group_name.data = ptr as *const c_char;
        repr.resource_group_name.len = len;
    }
    if src.has_penalty() {
        repr.penalty = crate::ffi_runtime::resource_manager::consumption_to_repr_generated(arena, src.get_penalty()) as *mut _;
    } else {
        repr.penalty = ptr::null_mut();
    }
    repr.override_priority = src.get_override_priority();
    arena.alloc_struct(repr)
}

pub fn resource_control_context_from_repr_generated(src: *const KvrpcpbResourceControlContext) -> Option<pb::ResourceControlContext> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ResourceControlContext::new();
    out.set_resource_group_name(string_from(repr.resource_group_name.data as *const u8, repr.resource_group_name.len));
    if !repr.penalty.is_null() {
        if let Some(value) = crate::ffi_runtime::resource_manager::consumption_from_repr_generated(repr.penalty) {
            out.set_penalty(value);
        }
    }
    out.set_override_priority(repr.override_priority);
    Some(out)
}

pub fn scan_detail_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ScanDetail) -> &'a mut KvrpcpbScanDetail {
    let mut repr = KvrpcpbScanDetail {
        write: ptr::null_mut(),
        lock: ptr::null_mut(),
        data: ptr::null_mut(),
    };
    if src.has_write() {
        repr.write = scan_info_to_repr_generated(arena, src.get_write()) as *mut _;
    } else {
        repr.write = ptr::null_mut();
    }
    if src.has_lock() {
        repr.lock = scan_info_to_repr_generated(arena, src.get_lock()) as *mut _;
    } else {
        repr.lock = ptr::null_mut();
    }
    if src.has_data() {
        repr.data = scan_info_to_repr_generated(arena, src.get_data()) as *mut _;
    } else {
        repr.data = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn scan_detail_from_repr_generated(src: *const KvrpcpbScanDetail) -> Option<pb::ScanDetail> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ScanDetail::new();
    if !repr.write.is_null() {
        if let Some(value) = scan_info_from_repr_generated(repr.write) {
            out.set_write(value);
        }
    }
    if !repr.lock.is_null() {
        if let Some(value) = scan_info_from_repr_generated(repr.lock) {
            out.set_lock(value);
        }
    }
    if !repr.data.is_null() {
        if let Some(value) = scan_info_from_repr_generated(repr.data) {
            out.set_data(value);
        }
    }
    Some(out)
}

pub fn scan_detail_v2_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ScanDetailV2) -> &'a mut KvrpcpbScanDetailV2 {
    let mut repr = KvrpcpbScanDetailV2 {
        processed_versions: Default::default(),
        total_versions: Default::default(),
        rocksdb_delete_skipped_count: Default::default(),
        rocksdb_key_skipped_count: Default::default(),
        rocksdb_block_cache_hit_count: Default::default(),
        rocksdb_block_read_count: Default::default(),
        rocksdb_block_read_byte: Default::default(),
        processed_versions_size: Default::default(),
        rocksdb_block_read_nanos: Default::default(),
        get_snapshot_nanos: Default::default(),
        read_index_propose_wait_nanos: Default::default(),
        read_index_confirm_wait_nanos: Default::default(),
        read_pool_schedule_wait_nanos: Default::default(),
    };
    repr.processed_versions = src.get_processed_versions();
    repr.total_versions = src.get_total_versions();
    repr.rocksdb_delete_skipped_count = src.get_rocksdb_delete_skipped_count();
    repr.rocksdb_key_skipped_count = src.get_rocksdb_key_skipped_count();
    repr.rocksdb_block_cache_hit_count = src.get_rocksdb_block_cache_hit_count();
    repr.rocksdb_block_read_count = src.get_rocksdb_block_read_count();
    repr.rocksdb_block_read_byte = src.get_rocksdb_block_read_byte();
    repr.processed_versions_size = src.get_processed_versions_size();
    repr.rocksdb_block_read_nanos = src.get_rocksdb_block_read_nanos();
    repr.get_snapshot_nanos = src.get_get_snapshot_nanos();
    repr.read_index_propose_wait_nanos = src.get_read_index_propose_wait_nanos();
    repr.read_index_confirm_wait_nanos = src.get_read_index_confirm_wait_nanos();
    repr.read_pool_schedule_wait_nanos = src.get_read_pool_schedule_wait_nanos();
    arena.alloc_struct(repr)
}

pub fn scan_detail_v2_from_repr_generated(src: *const KvrpcpbScanDetailV2) -> Option<pb::ScanDetailV2> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ScanDetailV2::new();
    out.set_processed_versions(repr.processed_versions);
    out.set_total_versions(repr.total_versions);
    out.set_rocksdb_delete_skipped_count(repr.rocksdb_delete_skipped_count);
    out.set_rocksdb_key_skipped_count(repr.rocksdb_key_skipped_count);
    out.set_rocksdb_block_cache_hit_count(repr.rocksdb_block_cache_hit_count);
    out.set_rocksdb_block_read_count(repr.rocksdb_block_read_count);
    out.set_rocksdb_block_read_byte(repr.rocksdb_block_read_byte);
    out.set_processed_versions_size(repr.processed_versions_size);
    out.set_rocksdb_block_read_nanos(repr.rocksdb_block_read_nanos);
    out.set_get_snapshot_nanos(repr.get_snapshot_nanos);
    out.set_read_index_propose_wait_nanos(repr.read_index_propose_wait_nanos);
    out.set_read_index_confirm_wait_nanos(repr.read_index_confirm_wait_nanos);
    out.set_read_pool_schedule_wait_nanos(repr.read_pool_schedule_wait_nanos);
    Some(out)
}

pub fn scan_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ScanInfo) -> &'a mut KvrpcpbScanInfo {
    let mut repr = KvrpcpbScanInfo {
        total: Default::default(),
        processed: Default::default(),
        read_bytes: Default::default(),
    };
    repr.total = src.get_total();
    repr.processed = src.get_processed();
    repr.read_bytes = src.get_read_bytes();
    arena.alloc_struct(repr)
}

pub fn scan_info_from_repr_generated(src: *const KvrpcpbScanInfo) -> Option<pb::ScanInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ScanInfo::new();
    out.set_total(repr.total);
    out.set_processed(repr.processed);
    out.set_read_bytes(repr.read_bytes);
    Some(out)
}

pub fn scan_lock_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ScanLockRequest) -> &'a mut KvrpcpbScanLockRequest {
    let mut repr = KvrpcpbScanLockRequest {
        context: ptr::null_mut(),
        max_version: Default::default(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        limit: Default::default(),
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    repr.max_version = src.get_max_version();
    if !src.get_start_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_start_key());
        repr.start_key.data = ptr;
        repr.start_key.len = len;
    }
    repr.limit = src.get_limit();
    if !src.get_end_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_end_key());
        repr.end_key.data = ptr;
        repr.end_key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn scan_lock_request_from_repr_generated(src: *const KvrpcpbScanLockRequest) -> Option<pb::ScanLockRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ScanLockRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_max_version(repr.max_version);
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_limit(repr.limit);
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    Some(out)
}

pub fn scan_lock_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ScanLockResponse) -> &'a mut KvrpcpbScanLockResponse {
    let mut repr = KvrpcpbScanLockResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
        locks: KvprotoSliceKvrpcpbLockInfoPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    {
        let values = src.get_locks();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbLockInfo> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(lock_info_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.locks.data = ptr;
                repr.locks.len = len;
                repr.locks.cap = len;
            }
        }
    }
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn scan_lock_response_from_repr_generated(src: *const KvrpcpbScanLockResponse) -> Option<pb::ScanLockResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ScanLockResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    if !repr.locks.data.is_null() && repr.locks.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.locks.data, repr.locks.len) };
        let mut values: Vec<pb::LockInfo> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = lock_info_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_locks(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn scan_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ScanRequest) -> &'a mut KvrpcpbScanRequest {
    let mut repr = KvrpcpbScanRequest {
        context: ptr::null_mut(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        limit: Default::default(),
        version: Default::default(),
        key_only: Default::default(),
        reverse: Default::default(),
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        sample_step: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_start_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_start_key());
        repr.start_key.data = ptr;
        repr.start_key.len = len;
    }
    repr.limit = src.get_limit();
    repr.version = src.get_version();
    repr.key_only = src.get_key_only();
    repr.reverse = src.get_reverse();
    if !src.get_end_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_end_key());
        repr.end_key.data = ptr;
        repr.end_key.len = len;
    }
    repr.sample_step = src.get_sample_step();
    arena.alloc_struct(repr)
}

pub fn scan_request_from_repr_generated(src: *const KvrpcpbScanRequest) -> Option<pb::ScanRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ScanRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_limit(repr.limit);
    out.set_version(repr.version);
    out.set_key_only(repr.key_only);
    out.set_reverse(repr.reverse);
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    out.set_sample_step(repr.sample_step);
    Some(out)
}

pub fn scan_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ScanResponse) -> &'a mut KvrpcpbScanResponse {
    let mut repr = KvrpcpbScanResponse {
        region_error: ptr::null_mut(),
        pairs: KvprotoSliceKvrpcpbKvPairPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        error: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    {
        let values = src.get_pairs();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKvPair> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(kv_pair_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.pairs.data = ptr;
                repr.pairs.len = len;
                repr.pairs.cap = len;
            }
        }
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn scan_response_from_repr_generated(src: *const KvrpcpbScanResponse) -> Option<pb::ScanResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ScanResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.pairs.data.is_null() && repr.pairs.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.pairs.data, repr.pairs.len) };
        let mut values: Vec<pb::KvPair> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = kv_pair_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_pairs(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    Some(out)
}

pub fn source_stmt_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::SourceStmt) -> &'a mut KvrpcpbSourceStmt {
    let mut repr = KvrpcpbSourceStmt {
        start_ts: Default::default(),
        connection_id: Default::default(),
        stmt_id: Default::default(),
        session_alias: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    repr.start_ts = src.get_start_ts();
    repr.connection_id = src.get_connection_id();
    repr.stmt_id = src.get_stmt_id();
    if !src.get_session_alias().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_session_alias());
        repr.session_alias.data = ptr as *const c_char;
        repr.session_alias.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn source_stmt_from_repr_generated(src: *const KvrpcpbSourceStmt) -> Option<pb::SourceStmt> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::SourceStmt::new();
    out.set_start_ts(repr.start_ts);
    out.set_connection_id(repr.connection_id);
    out.set_stmt_id(repr.stmt_id);
    out.set_session_alias(string_from(repr.session_alias.data as *const u8, repr.session_alias.len));
    Some(out)
}

pub fn split_region_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::SplitRegionRequest) -> &'a mut KvrpcpbSplitRegionRequest {
    let mut repr = KvrpcpbSplitRegionRequest {
        context: ptr::null_mut(),
        split_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        split_keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        is_raw_kv: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_split_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_split_key());
        repr.split_key.data = ptr;
        repr.split_key.len = len;
    }
    {
        let values = src.get_split_keys();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_bytes(value);
                views.push(KvprotoBytesView { data: ptr, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.split_keys.data = ptr;
                repr.split_keys.len = len;
                repr.split_keys.cap = len;
            }
        }
    }
    repr.is_raw_kv = src.get_is_raw_kv();
    arena.alloc_struct(repr)
}

pub fn split_region_request_from_repr_generated(src: *const KvrpcpbSplitRegionRequest) -> Option<pb::SplitRegionRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::SplitRegionRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_split_key(bytes_from(repr.split_key.data, repr.split_key.len).into());
    if !repr.split_keys.data.is_null() && repr.split_keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.split_keys.data, repr.split_keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_split_keys(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_is_raw_kv(repr.is_raw_kv);
    Some(out)
}

pub fn split_region_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::SplitRegionResponse) -> &'a mut KvrpcpbSplitRegionResponse {
    let mut repr = KvrpcpbSplitRegionResponse {
        region_error: ptr::null_mut(),
        left: ptr::null_mut(),
        right: ptr::null_mut(),
        regions: KvprotoSliceMetapbRegionPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        errors: KvprotoSliceKvrpcpbKeyErrorPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_left() {
        repr.left = crate::ffi_runtime::metapb::region_to_repr_generated(arena, src.get_left()) as *mut _;
    } else {
        repr.left = ptr::null_mut();
    }
    if src.has_right() {
        repr.right = crate::ffi_runtime::metapb::region_to_repr_generated(arena, src.get_right()) as *mut _;
    } else {
        repr.right = ptr::null_mut();
    }
    {
        let values = src.get_regions();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut MetapbRegion> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(crate::ffi_runtime::metapb::region_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.regions.data = ptr;
                repr.regions.len = len;
                repr.regions.cap = len;
            }
        }
    }
    {
        let values = src.get_errors();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut KvrpcpbKeyError> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(key_error_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.errors.data = ptr;
                repr.errors.len = len;
                repr.errors.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn split_region_response_from_repr_generated(src: *const KvrpcpbSplitRegionResponse) -> Option<pb::SplitRegionResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::SplitRegionResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.left.is_null() {
        if let Some(value) = crate::ffi_runtime::metapb::region_from_repr_generated(repr.left) {
            out.set_left(value);
        }
    }
    if !repr.right.is_null() {
        if let Some(value) = crate::ffi_runtime::metapb::region_from_repr_generated(repr.right) {
            out.set_right(value);
        }
    }
    if !repr.regions.data.is_null() && repr.regions.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.regions.data, repr.regions.len) };
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
            out.set_regions(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.errors.data.is_null() && repr.errors.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.errors.data, repr.errors.len) };
        let mut values: Vec<pb::KeyError> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = key_error_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_errors(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn store_safe_t_s_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::StoreSafeTsRequest) -> &'a mut KvrpcpbStoreSafeTSRequest {
    let mut repr = KvrpcpbStoreSafeTSRequest {
        key_range: ptr::null_mut(),
    };
    if src.has_key_range() {
        repr.key_range = key_range_to_repr_generated(arena, src.get_key_range()) as *mut _;
    } else {
        repr.key_range = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn store_safe_t_s_request_from_repr_generated(src: *const KvrpcpbStoreSafeTSRequest) -> Option<pb::StoreSafeTsRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::StoreSafeTsRequest::new();
    if !repr.key_range.is_null() {
        if let Some(value) = key_range_from_repr_generated(repr.key_range) {
            out.set_key_range(value);
        }
    }
    Some(out)
}

pub fn store_safe_t_s_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::StoreSafeTsResponse) -> &'a mut KvrpcpbStoreSafeTSResponse {
    let mut repr = KvrpcpbStoreSafeTSResponse {
        safe_ts: Default::default(),
    };
    repr.safe_ts = src.get_safe_ts();
    arena.alloc_struct(repr)
}

pub fn store_safe_t_s_response_from_repr_generated(src: *const KvrpcpbStoreSafeTSResponse) -> Option<pb::StoreSafeTsResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::StoreSafeTsResponse::new();
    out.set_safe_ts(repr.safe_ts);
    Some(out)
}

pub fn ti_flash_system_table_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TiFlashSystemTableRequest) -> &'a mut KvrpcpbTiFlashSystemTableRequest {
    let mut repr = KvrpcpbTiFlashSystemTableRequest {
        sql: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_sql().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_sql());
        repr.sql.data = ptr as *const c_char;
        repr.sql.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn ti_flash_system_table_request_from_repr_generated(src: *const KvrpcpbTiFlashSystemTableRequest) -> Option<pb::TiFlashSystemTableRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TiFlashSystemTableRequest::new();
    out.set_sql(string_from(repr.sql.data as *const u8, repr.sql.len));
    Some(out)
}

pub fn ti_flash_system_table_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TiFlashSystemTableResponse) -> &'a mut KvrpcpbTiFlashSystemTableResponse {
    let mut repr = KvrpcpbTiFlashSystemTableResponse {
        data: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if !src.get_data().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_data());
        repr.data.data = ptr;
        repr.data.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn ti_flash_system_table_response_from_repr_generated(src: *const KvrpcpbTiFlashSystemTableResponse) -> Option<pb::TiFlashSystemTableResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TiFlashSystemTableResponse::new();
    out.set_data(bytes_from(repr.data.data, repr.data.len).into());
    Some(out)
}

pub fn time_detail_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TimeDetail) -> &'a mut KvrpcpbTimeDetail {
    let mut repr = KvrpcpbTimeDetail {
        wait_wall_time_ms: Default::default(),
        process_wall_time_ms: Default::default(),
        kv_read_wall_time_ms: Default::default(),
        total_rpc_wall_time_ns: Default::default(),
    };
    repr.wait_wall_time_ms = src.get_wait_wall_time_ms();
    repr.process_wall_time_ms = src.get_process_wall_time_ms();
    repr.kv_read_wall_time_ms = src.get_kv_read_wall_time_ms();
    repr.total_rpc_wall_time_ns = src.get_total_rpc_wall_time_ns();
    arena.alloc_struct(repr)
}

pub fn time_detail_from_repr_generated(src: *const KvrpcpbTimeDetail) -> Option<pb::TimeDetail> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TimeDetail::new();
    out.set_wait_wall_time_ms(repr.wait_wall_time_ms);
    out.set_process_wall_time_ms(repr.process_wall_time_ms);
    out.set_kv_read_wall_time_ms(repr.kv_read_wall_time_ms);
    out.set_total_rpc_wall_time_ns(repr.total_rpc_wall_time_ns);
    Some(out)
}

pub fn time_detail_v2_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TimeDetailV2) -> &'a mut KvrpcpbTimeDetailV2 {
    let mut repr = KvrpcpbTimeDetailV2 {
        wait_wall_time_ns: Default::default(),
        process_wall_time_ns: Default::default(),
        process_suspend_wall_time_ns: Default::default(),
        kv_read_wall_time_ns: Default::default(),
        total_rpc_wall_time_ns: Default::default(),
        kv_grpc_process_time_ns: Default::default(),
        kv_grpc_wait_time_ns: Default::default(),
    };
    repr.wait_wall_time_ns = src.get_wait_wall_time_ns();
    repr.process_wall_time_ns = src.get_process_wall_time_ns();
    repr.process_suspend_wall_time_ns = src.get_process_suspend_wall_time_ns();
    repr.kv_read_wall_time_ns = src.get_kv_read_wall_time_ns();
    repr.total_rpc_wall_time_ns = src.get_total_rpc_wall_time_ns();
    repr.kv_grpc_process_time_ns = src.get_kv_grpc_process_time_ns();
    repr.kv_grpc_wait_time_ns = src.get_kv_grpc_wait_time_ns();
    arena.alloc_struct(repr)
}

pub fn time_detail_v2_from_repr_generated(src: *const KvrpcpbTimeDetailV2) -> Option<pb::TimeDetailV2> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TimeDetailV2::new();
    out.set_wait_wall_time_ns(repr.wait_wall_time_ns);
    out.set_process_wall_time_ns(repr.process_wall_time_ns);
    out.set_process_suspend_wall_time_ns(repr.process_suspend_wall_time_ns);
    out.set_kv_read_wall_time_ns(repr.kv_read_wall_time_ns);
    out.set_total_rpc_wall_time_ns(repr.total_rpc_wall_time_ns);
    out.set_kv_grpc_process_time_ns(repr.kv_grpc_process_time_ns);
    out.set_kv_grpc_wait_time_ns(repr.kv_grpc_wait_time_ns);
    Some(out)
}

pub fn txn_heart_beat_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TxnHeartBeatRequest) -> &'a mut KvrpcpbTxnHeartBeatRequest {
    let mut repr = KvrpcpbTxnHeartBeatRequest {
        context: ptr::null_mut(),
        primary_lock: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        start_version: Default::default(),
        advise_lock_ttl: Default::default(),
        min_commit_ts: Default::default(),
        is_txn_file: Default::default(),
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
    if !src.get_primary_lock().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_primary_lock());
        repr.primary_lock.data = ptr;
        repr.primary_lock.len = len;
    }
    repr.start_version = src.get_start_version();
    repr.advise_lock_ttl = src.get_advise_lock_ttl();
    repr.min_commit_ts = src.get_min_commit_ts();
    repr.is_txn_file = src.get_is_txn_file();
    arena.alloc_struct(repr)
}

pub fn txn_heart_beat_request_from_repr_generated(src: *const KvrpcpbTxnHeartBeatRequest) -> Option<pb::TxnHeartBeatRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TxnHeartBeatRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_primary_lock(bytes_from(repr.primary_lock.data, repr.primary_lock.len).into());
    out.set_start_version(repr.start_version);
    out.set_advise_lock_ttl(repr.advise_lock_ttl);
    out.set_min_commit_ts(repr.min_commit_ts);
    out.set_is_txn_file(repr.is_txn_file);
    Some(out)
}

pub fn txn_heart_beat_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TxnHeartBeatResponse) -> &'a mut KvrpcpbTxnHeartBeatResponse {
    let mut repr = KvrpcpbTxnHeartBeatResponse {
        region_error: ptr::null_mut(),
        error: ptr::null_mut(),
        lock_ttl: Default::default(),
        exec_details_v2: ptr::null_mut(),
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if src.has_error() {
        repr.error = key_error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    repr.lock_ttl = src.get_lock_ttl();
    if src.has_exec_details_v2() {
        repr.exec_details_v2 = exec_details_v2_to_repr_generated(arena, src.get_exec_details_v2()) as *mut _;
    } else {
        repr.exec_details_v2 = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn txn_heart_beat_response_from_repr_generated(src: *const KvrpcpbTxnHeartBeatResponse) -> Option<pb::TxnHeartBeatResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TxnHeartBeatResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    if !repr.error.is_null() {
        if let Some(value) = key_error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    out.set_lock_ttl(repr.lock_ttl);
    if !repr.exec_details_v2.is_null() {
        if let Some(value) = exec_details_v2_from_repr_generated(repr.exec_details_v2) {
            out.set_exec_details_v2(value);
        }
    }
    Some(out)
}

pub fn txn_info_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TxnInfo) -> &'a mut KvrpcpbTxnInfo {
    let mut repr = KvrpcpbTxnInfo {
        txn: Default::default(),
        status: Default::default(),
        is_txn_file: Default::default(),
    };
    repr.txn = src.get_txn();
    repr.status = src.get_status();
    repr.is_txn_file = src.get_is_txn_file();
    arena.alloc_struct(repr)
}

pub fn txn_info_from_repr_generated(src: *const KvrpcpbTxnInfo) -> Option<pb::TxnInfo> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TxnInfo::new();
    out.set_txn(repr.txn);
    out.set_status(repr.status);
    out.set_is_txn_file(repr.is_txn_file);
    Some(out)
}

pub fn txn_lock_not_found_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TxnLockNotFound) -> &'a mut KvrpcpbTxnLockNotFound {
    let mut repr = KvrpcpbTxnLockNotFound {
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn txn_lock_not_found_from_repr_generated(src: *const KvrpcpbTxnLockNotFound) -> Option<pb::TxnLockNotFound> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TxnLockNotFound::new();
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    Some(out)
}

pub fn txn_not_found_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TxnNotFound) -> &'a mut KvrpcpbTxnNotFound {
    let mut repr = KvrpcpbTxnNotFound {
        start_ts: Default::default(),
        primary_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    repr.start_ts = src.get_start_ts();
    if !src.get_primary_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_primary_key());
        repr.primary_key.data = ptr;
        repr.primary_key.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn txn_not_found_from_repr_generated(src: *const KvrpcpbTxnNotFound) -> Option<pb::TxnNotFound> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TxnNotFound::new();
    out.set_start_ts(repr.start_ts);
    out.set_primary_key(bytes_from(repr.primary_key.data, repr.primary_key.len).into());
    Some(out)
}

pub fn txn_status_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TxnStatus) -> &'a mut KvrpcpbTxnStatus {
    let mut repr = KvrpcpbTxnStatus {
        start_ts: Default::default(),
        min_commit_ts: Default::default(),
        commit_ts: Default::default(),
        rolled_back: Default::default(),
        is_completed: Default::default(),
    };
    repr.start_ts = src.get_start_ts();
    repr.min_commit_ts = src.get_min_commit_ts();
    repr.commit_ts = src.get_commit_ts();
    repr.rolled_back = src.get_rolled_back();
    repr.is_completed = src.get_is_completed();
    arena.alloc_struct(repr)
}

pub fn txn_status_from_repr_generated(src: *const KvrpcpbTxnStatus) -> Option<pb::TxnStatus> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TxnStatus::new();
    out.set_start_ts(repr.start_ts);
    out.set_min_commit_ts(repr.min_commit_ts);
    out.set_commit_ts(repr.commit_ts);
    out.set_rolled_back(repr.rolled_back);
    out.set_is_completed(repr.is_completed);
    Some(out)
}

pub fn unsafe_destroy_range_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::UnsafeDestroyRangeRequest) -> &'a mut KvrpcpbUnsafeDestroyRangeRequest {
    let mut repr = KvrpcpbUnsafeDestroyRangeRequest {
        context: ptr::null_mut(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
    };
    if src.has_context() {
        repr.context = context_to_repr_generated(arena, src.get_context()) as *mut _;
    } else {
        repr.context = ptr::null_mut();
    }
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

pub fn unsafe_destroy_range_request_from_repr_generated(src: *const KvrpcpbUnsafeDestroyRangeRequest) -> Option<pb::UnsafeDestroyRangeRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::UnsafeDestroyRangeRequest::new();
    if !repr.context.is_null() {
        if let Some(value) = context_from_repr_generated(repr.context) {
            out.set_context(value);
        }
    }
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    Some(out)
}

pub fn unsafe_destroy_range_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::UnsafeDestroyRangeResponse) -> &'a mut KvrpcpbUnsafeDestroyRangeResponse {
    let mut repr = KvrpcpbUnsafeDestroyRangeResponse {
        region_error: ptr::null_mut(),
        error: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_region_error() {
        repr.region_error = crate::ffi_runtime::errorpb::error_to_repr_generated(arena, src.get_region_error()) as *mut _;
    } else {
        repr.region_error = ptr::null_mut();
    }
    if !src.get_error().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_error());
        repr.error.data = ptr as *const c_char;
        repr.error.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn unsafe_destroy_range_response_from_repr_generated(src: *const KvrpcpbUnsafeDestroyRangeResponse) -> Option<pb::UnsafeDestroyRangeResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::UnsafeDestroyRangeResponse::new();
    if !repr.region_error.is_null() {
        if let Some(value) = crate::ffi_runtime::errorpb::error_from_repr_generated(repr.region_error) {
            out.set_region_error(value);
        }
    }
    out.set_error(string_from(repr.error.data as *const u8, repr.error.len));
    Some(out)
}

pub fn write_conflict_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::WriteConflict) -> &'a mut KvrpcpbWriteConflict {
    let mut repr = KvrpcpbWriteConflict {
        start_ts: Default::default(),
        conflict_ts: Default::default(),
        key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        primary: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        conflict_commit_ts: Default::default(),
        reason: Default::default(),
    };
    repr.start_ts = src.get_start_ts();
    repr.conflict_ts = src.get_conflict_ts();
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_key());
        repr.key.data = ptr;
        repr.key.len = len;
    }
    if !src.get_primary().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_primary());
        repr.primary.data = ptr;
        repr.primary.len = len;
    }
    repr.conflict_commit_ts = src.get_conflict_commit_ts();
    repr.reason = src.get_reason() as i32;
    arena.alloc_struct(repr)
}

pub fn write_conflict_from_repr_generated(src: *const KvrpcpbWriteConflict) -> Option<pb::WriteConflict> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::WriteConflict::new();
    out.set_start_ts(repr.start_ts);
    out.set_conflict_ts(repr.conflict_ts);
    out.set_key(bytes_from(repr.key.data, repr.key.len).into());
    out.set_primary(bytes_from(repr.primary.data, repr.primary.len).into());
    out.set_conflict_commit_ts(repr.conflict_commit_ts);
    out.set_reason(pb::WriteConflictReason::from_i32(repr.reason).unwrap_or_default());
    Some(out)
}

pub fn write_detail_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::WriteDetail) -> &'a mut KvrpcpbWriteDetail {
    let mut repr = KvrpcpbWriteDetail {
        store_batch_wait_nanos: Default::default(),
        propose_send_wait_nanos: Default::default(),
        persist_log_nanos: Default::default(),
        raft_db_write_leader_wait_nanos: Default::default(),
        raft_db_sync_log_nanos: Default::default(),
        raft_db_write_memtable_nanos: Default::default(),
        commit_log_nanos: Default::default(),
        apply_batch_wait_nanos: Default::default(),
        apply_log_nanos: Default::default(),
        apply_mutex_lock_nanos: Default::default(),
        apply_write_leader_wait_nanos: Default::default(),
        apply_write_wal_nanos: Default::default(),
        apply_write_memtable_nanos: Default::default(),
        latch_wait_nanos: Default::default(),
        process_nanos: Default::default(),
        throttle_nanos: Default::default(),
        pessimistic_lock_wait_nanos: Default::default(),
    };
    repr.store_batch_wait_nanos = src.get_store_batch_wait_nanos();
    repr.propose_send_wait_nanos = src.get_propose_send_wait_nanos();
    repr.persist_log_nanos = src.get_persist_log_nanos();
    repr.raft_db_write_leader_wait_nanos = src.get_raft_db_write_leader_wait_nanos();
    repr.raft_db_sync_log_nanos = src.get_raft_db_sync_log_nanos();
    repr.raft_db_write_memtable_nanos = src.get_raft_db_write_memtable_nanos();
    repr.commit_log_nanos = src.get_commit_log_nanos();
    repr.apply_batch_wait_nanos = src.get_apply_batch_wait_nanos();
    repr.apply_log_nanos = src.get_apply_log_nanos();
    repr.apply_mutex_lock_nanos = src.get_apply_mutex_lock_nanos();
    repr.apply_write_leader_wait_nanos = src.get_apply_write_leader_wait_nanos();
    repr.apply_write_wal_nanos = src.get_apply_write_wal_nanos();
    repr.apply_write_memtable_nanos = src.get_apply_write_memtable_nanos();
    repr.latch_wait_nanos = src.get_latch_wait_nanos();
    repr.process_nanos = src.get_process_nanos();
    repr.throttle_nanos = src.get_throttle_nanos();
    repr.pessimistic_lock_wait_nanos = src.get_pessimistic_lock_wait_nanos();
    arena.alloc_struct(repr)
}

pub fn write_detail_from_repr_generated(src: *const KvrpcpbWriteDetail) -> Option<pb::WriteDetail> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::WriteDetail::new();
    out.set_store_batch_wait_nanos(repr.store_batch_wait_nanos);
    out.set_propose_send_wait_nanos(repr.propose_send_wait_nanos);
    out.set_persist_log_nanos(repr.persist_log_nanos);
    out.set_raft_db_write_leader_wait_nanos(repr.raft_db_write_leader_wait_nanos);
    out.set_raft_db_sync_log_nanos(repr.raft_db_sync_log_nanos);
    out.set_raft_db_write_memtable_nanos(repr.raft_db_write_memtable_nanos);
    out.set_commit_log_nanos(repr.commit_log_nanos);
    out.set_apply_batch_wait_nanos(repr.apply_batch_wait_nanos);
    out.set_apply_log_nanos(repr.apply_log_nanos);
    out.set_apply_mutex_lock_nanos(repr.apply_mutex_lock_nanos);
    out.set_apply_write_leader_wait_nanos(repr.apply_write_leader_wait_nanos);
    out.set_apply_write_wal_nanos(repr.apply_write_wal_nanos);
    out.set_apply_write_memtable_nanos(repr.apply_write_memtable_nanos);
    out.set_latch_wait_nanos(repr.latch_wait_nanos);
    out.set_process_nanos(repr.process_nanos);
    out.set_throttle_nanos(repr.throttle_nanos);
    out.set_pessimistic_lock_wait_nanos(repr.pessimistic_lock_wait_nanos);
    Some(out)
}

