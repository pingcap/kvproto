//go:build kvffi_gen
// +build kvffi_gen

package kvrpcpb

/*
#cgo CFLAGS: -I../../c
#include "kvproto_abi.h"
*/
import "C"

import (
	"unsafe"
	deadlockffi "github.com/pingcap/kvproto/ffi_out/go/deadlock"
	errorpbffi "github.com/pingcap/kvproto/ffi_out/go/errorpb"
	metapbffi "github.com/pingcap/kvproto/ffi_out/go/metapb"
	resource_managerffi "github.com/pingcap/kvproto/ffi_out/go/resource_manager"
	runtime "github.com/pingcap/kvproto/ffi_out/go/runtime"
	tracepbffi "github.com/pingcap/kvproto/ffi_out/go/tracepb"
	deadlockproto "github.com/pingcap/kvproto/pkg/deadlock"
	kvrpcpbproto "github.com/pingcap/kvproto/pkg/kvrpcpb"
	metapbproto "github.com/pingcap/kvproto/pkg/metapb"
	sharedbytes "github.com/pingcap/kvproto/pkg/sharedbytes"
)

func NewReprAlreadyExistGenerated(arena *runtime.Arena, src *kvrpcpbproto.AlreadyExist) *AlreadyExist {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*AlreadyExist)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_AlreadyExist)))
	IntoReprAlreadyExistGenerated(arena, ptr, src)
	return ptr
}

func IntoReprAlreadyExistGenerated(arena *runtime.Arena, dst *AlreadyExist, src *kvrpcpbproto.AlreadyExist) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
}

func FromReprAlreadyExistGenerated(src *AlreadyExist) *kvrpcpbproto.AlreadyExist {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.AlreadyExist{}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	return out
}

func NewReprAssertionFailedGenerated(arena *runtime.Arena, src *kvrpcpbproto.AssertionFailed) *AssertionFailed {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*AssertionFailed)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_AssertionFailed)))
	IntoReprAssertionFailedGenerated(arena, ptr, src)
	return ptr
}

func IntoReprAssertionFailedGenerated(arena *runtime.Arena, dst *AssertionFailed, src *kvrpcpbproto.AssertionFailed) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	dst.assertion = C.int32_t(int32(src.GetAssertion()))
	dst.existing_start_ts = C.uint64_t(src.GetExistingStartTs())
	dst.existing_commit_ts = C.uint64_t(src.GetExistingCommitTs())
}

func FromReprAssertionFailedGenerated(src *AssertionFailed) *kvrpcpbproto.AssertionFailed {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.AssertionFailed{}
	out.StartTs = uint64(src.start_ts)
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Assertion = kvrpcpbproto.Assertion(int32(src.assertion))
	out.ExistingStartTs = uint64(src.existing_start_ts)
	out.ExistingCommitTs = uint64(src.existing_commit_ts)
	return out
}

func NewReprBatchGetRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.BatchGetRequest) *BatchGetRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BatchGetRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_BatchGetRequest)))
	IntoReprBatchGetRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBatchGetRequestGenerated(arena *runtime.Arena, dst *BatchGetRequest, src *kvrpcpbproto.BatchGetRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
	dst.version = C.uint64_t(src.GetVersion())
	dst.need_commit_ts = C.bool(src.GetNeedCommitTs())
}

func FromReprBatchGetRequestGenerated(src *BatchGetRequest) *kvrpcpbproto.BatchGetRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.BatchGetRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	out.Version = uint64(src.version)
	out.NeedCommitTs = bool(src.need_commit_ts)
	return out
}

func NewReprBatchGetResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.BatchGetResponse) *BatchGetResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BatchGetResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_BatchGetResponse)))
	IntoReprBatchGetResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBatchGetResponseGenerated(arena *runtime.Arena, dst *BatchGetResponse, src *kvrpcpbproto.BatchGetResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetPairs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KvPair)(nil)))
		array := unsafe.Slice((**KvPair)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKvPairGenerated(arena, value)
		}
		dst.pairs.data = (**KvPair)(ptr)
		dst.pairs.len = C.size_t(len(values))
		dst.pairs.cap = C.size_t(len(values))
	}
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
}

func FromReprBatchGetResponseGenerated(src *BatchGetResponse) *kvrpcpbproto.BatchGetResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.BatchGetResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.pairs.data != nil && src.pairs.len > 0 {
		length := int(src.pairs.len)
		ptrs := unsafe.Slice((**KvPair)(unsafe.Pointer(src.pairs.data)), length)
		out.Pairs = make([]*kvrpcpbproto.KvPair, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Pairs = append(out.Pairs, FromReprKvPairGenerated(ptr))
		}
	}
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	return out
}

func NewReprBatchRollbackRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.BatchRollbackRequest) *BatchRollbackRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BatchRollbackRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_BatchRollbackRequest)))
	IntoReprBatchRollbackRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBatchRollbackRequestGenerated(arena *runtime.Arena, dst *BatchRollbackRequest, src *kvrpcpbproto.BatchRollbackRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.start_version = C.uint64_t(src.GetStartVersion())
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
	dst.is_txn_file = C.bool(src.GetIsTxnFile())
}

func FromReprBatchRollbackRequestGenerated(src *BatchRollbackRequest) *kvrpcpbproto.BatchRollbackRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.BatchRollbackRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartVersion = uint64(src.start_version)
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	out.IsTxnFile = bool(src.is_txn_file)
	return out
}

func NewReprBatchRollbackResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.BatchRollbackResponse) *BatchRollbackResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BatchRollbackResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_BatchRollbackResponse)))
	IntoReprBatchRollbackResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBatchRollbackResponseGenerated(arena *runtime.Arena, dst *BatchRollbackResponse, src *kvrpcpbproto.BatchRollbackResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprBatchRollbackResponseGenerated(src *BatchRollbackResponse) *kvrpcpbproto.BatchRollbackResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.BatchRollbackResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprBroadcastTxnStatusRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.BroadcastTxnStatusRequest) *BroadcastTxnStatusRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BroadcastTxnStatusRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_BroadcastTxnStatusRequest)))
	IntoReprBroadcastTxnStatusRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBroadcastTxnStatusRequestGenerated(arena *runtime.Arena, dst *BroadcastTxnStatusRequest, src *kvrpcpbproto.BroadcastTxnStatusRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if values := src.GetTxnStatus(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*TxnStatus)(nil)))
		array := unsafe.Slice((**TxnStatus)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprTxnStatusGenerated(arena, value)
		}
		dst.txn_status.data = (**TxnStatus)(ptr)
		dst.txn_status.len = C.size_t(len(values))
		dst.txn_status.cap = C.size_t(len(values))
	}
}

func FromReprBroadcastTxnStatusRequestGenerated(src *BroadcastTxnStatusRequest) *kvrpcpbproto.BroadcastTxnStatusRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.BroadcastTxnStatusRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	if src.txn_status.data != nil && src.txn_status.len > 0 {
		length := int(src.txn_status.len)
		ptrs := unsafe.Slice((**TxnStatus)(unsafe.Pointer(src.txn_status.data)), length)
		out.TxnStatus = make([]*kvrpcpbproto.TxnStatus, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.TxnStatus = append(out.TxnStatus, FromReprTxnStatusGenerated(ptr))
		}
	}
	return out
}

func NewReprBroadcastTxnStatusResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.BroadcastTxnStatusResponse) *BroadcastTxnStatusResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BroadcastTxnStatusResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_BroadcastTxnStatusResponse)))
	IntoReprBroadcastTxnStatusResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBroadcastTxnStatusResponseGenerated(arena *runtime.Arena, dst *BroadcastTxnStatusResponse, src *kvrpcpbproto.BroadcastTxnStatusResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprBroadcastTxnStatusResponseGenerated(src *BroadcastTxnStatusResponse) *kvrpcpbproto.BroadcastTxnStatusResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.BroadcastTxnStatusResponse{}
	return out
}

func NewReprBufferBatchGetRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.BufferBatchGetRequest) *BufferBatchGetRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BufferBatchGetRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_BufferBatchGetRequest)))
	IntoReprBufferBatchGetRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBufferBatchGetRequestGenerated(arena *runtime.Arena, dst *BufferBatchGetRequest, src *kvrpcpbproto.BufferBatchGetRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
	dst.version = C.uint64_t(src.GetVersion())
}

func FromReprBufferBatchGetRequestGenerated(src *BufferBatchGetRequest) *kvrpcpbproto.BufferBatchGetRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.BufferBatchGetRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	out.Version = uint64(src.version)
	return out
}

func NewReprBufferBatchGetResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.BufferBatchGetResponse) *BufferBatchGetResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BufferBatchGetResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_BufferBatchGetResponse)))
	IntoReprBufferBatchGetResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBufferBatchGetResponseGenerated(arena *runtime.Arena, dst *BufferBatchGetResponse, src *kvrpcpbproto.BufferBatchGetResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if values := src.GetPairs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KvPair)(nil)))
		array := unsafe.Slice((**KvPair)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKvPairGenerated(arena, value)
		}
		dst.pairs.data = (**KvPair)(ptr)
		dst.pairs.len = C.size_t(len(values))
		dst.pairs.cap = C.size_t(len(values))
	}
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprBufferBatchGetResponseGenerated(src *BufferBatchGetResponse) *kvrpcpbproto.BufferBatchGetResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.BufferBatchGetResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	if src.pairs.data != nil && src.pairs.len > 0 {
		length := int(src.pairs.len)
		ptrs := unsafe.Slice((**KvPair)(unsafe.Pointer(src.pairs.data)), length)
		out.Pairs = make([]*kvrpcpbproto.KvPair, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Pairs = append(out.Pairs, FromReprKvPairGenerated(ptr))
		}
	}
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprChangedEntryGenerated(arena *runtime.Arena, src *kvrpcpbproto.ChangedEntry) *ChangedEntry {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ChangedEntry)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ChangedEntry)))
	IntoReprChangedEntryGenerated(arena, ptr, src)
	return ptr
}

func IntoReprChangedEntryGenerated(arena *runtime.Arena, dst *ChangedEntry, src *kvrpcpbproto.ChangedEntry) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetValue()); length > 0 {
		dst.value.data = (*C.uint8_t)(data)
		dst.value.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetOldValue()); length > 0 {
		dst.old_value.data = (*C.uint8_t)(data)
		dst.old_value.len = C.size_t(length)
	}
	dst.commit_ts = C.uint64_t(src.GetCommitTs())
}

func FromReprChangedEntryGenerated(src *ChangedEntry) *kvrpcpbproto.ChangedEntry {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ChangedEntry{}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Value = runtime.BytesFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	out.OldValue = runtime.BytesFrom(unsafe.Pointer(src.old_value.data), int(src.old_value.len))
	out.CommitTs = uint64(src.commit_ts)
	return out
}

func NewReprCheckLeaderRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.CheckLeaderRequest) *CheckLeaderRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CheckLeaderRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CheckLeaderRequest)))
	IntoReprCheckLeaderRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCheckLeaderRequestGenerated(arena *runtime.Arena, dst *CheckLeaderRequest, src *kvrpcpbproto.CheckLeaderRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetRegions(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*LeaderInfo)(nil)))
		array := unsafe.Slice((**LeaderInfo)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprLeaderInfoGenerated(arena, value)
		}
		dst.regions.data = (**LeaderInfo)(ptr)
		dst.regions.len = C.size_t(len(values))
		dst.regions.cap = C.size_t(len(values))
	}
	dst.ts = C.uint64_t(src.GetTs())
}

func FromReprCheckLeaderRequestGenerated(src *CheckLeaderRequest) *kvrpcpbproto.CheckLeaderRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CheckLeaderRequest{}
	if src.regions.data != nil && src.regions.len > 0 {
		length := int(src.regions.len)
		ptrs := unsafe.Slice((**LeaderInfo)(unsafe.Pointer(src.regions.data)), length)
		out.Regions = make([]*kvrpcpbproto.LeaderInfo, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Regions = append(out.Regions, FromReprLeaderInfoGenerated(ptr))
		}
	}
	out.Ts = uint64(src.ts)
	return out
}

func NewReprCheckLeaderResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.CheckLeaderResponse) *CheckLeaderResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CheckLeaderResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CheckLeaderResponse)))
	IntoReprCheckLeaderResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCheckLeaderResponseGenerated(arena *runtime.Arena, dst *CheckLeaderResponse, src *kvrpcpbproto.CheckLeaderResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetRegions(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.regions.data = (*C.uint64_t)(ptr)
		dst.regions.len = C.size_t(len(values))
		dst.regions.cap = C.size_t(len(values))
	}
	dst.ts = C.uint64_t(src.GetTs())
}

func FromReprCheckLeaderResponseGenerated(src *CheckLeaderResponse) *kvrpcpbproto.CheckLeaderResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CheckLeaderResponse{}
	if src.regions.data != nil && src.regions.len > 0 {
		length := int(src.regions.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.regions.data)), length)
		out.Regions = make([]uint64, 0, length)
		for _, value := range values {
			out.Regions = append(out.Regions, uint64(value))
		}
	}
	out.Ts = uint64(src.ts)
	return out
}

func NewReprCheckLockObserverRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.CheckLockObserverRequest) *CheckLockObserverRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CheckLockObserverRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CheckLockObserverRequest)))
	IntoReprCheckLockObserverRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCheckLockObserverRequestGenerated(arena *runtime.Arena, dst *CheckLockObserverRequest, src *kvrpcpbproto.CheckLockObserverRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.max_ts = C.uint64_t(src.GetMaxTs())
}

func FromReprCheckLockObserverRequestGenerated(src *CheckLockObserverRequest) *kvrpcpbproto.CheckLockObserverRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CheckLockObserverRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.MaxTs = uint64(src.max_ts)
	return out
}

func NewReprCheckLockObserverResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.CheckLockObserverResponse) *CheckLockObserverResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CheckLockObserverResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CheckLockObserverResponse)))
	IntoReprCheckLockObserverResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCheckLockObserverResponseGenerated(arena *runtime.Arena, dst *CheckLockObserverResponse, src *kvrpcpbproto.CheckLockObserverResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	dst.is_clean = C.bool(src.GetIsClean())
	if values := src.GetLocks(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*LockInfo)(nil)))
		array := unsafe.Slice((**LockInfo)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprLockInfoGenerated(arena, value)
		}
		dst.locks.data = (**LockInfo)(ptr)
		dst.locks.len = C.size_t(len(values))
		dst.locks.cap = C.size_t(len(values))
	}
}

func FromReprCheckLockObserverResponseGenerated(src *CheckLockObserverResponse) *kvrpcpbproto.CheckLockObserverResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CheckLockObserverResponse{}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	out.IsClean = bool(src.is_clean)
	if src.locks.data != nil && src.locks.len > 0 {
		length := int(src.locks.len)
		ptrs := unsafe.Slice((**LockInfo)(unsafe.Pointer(src.locks.data)), length)
		out.Locks = make([]*kvrpcpbproto.LockInfo, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Locks = append(out.Locks, FromReprLockInfoGenerated(ptr))
		}
	}
	return out
}

func NewReprCheckSecondaryLocksRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.CheckSecondaryLocksRequest) *CheckSecondaryLocksRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CheckSecondaryLocksRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CheckSecondaryLocksRequest)))
	IntoReprCheckSecondaryLocksRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCheckSecondaryLocksRequestGenerated(arena *runtime.Arena, dst *CheckSecondaryLocksRequest, src *kvrpcpbproto.CheckSecondaryLocksRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
	dst.start_version = C.uint64_t(src.GetStartVersion())
}

func FromReprCheckSecondaryLocksRequestGenerated(src *CheckSecondaryLocksRequest) *kvrpcpbproto.CheckSecondaryLocksRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CheckSecondaryLocksRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	out.StartVersion = uint64(src.start_version)
	return out
}

func NewReprCheckSecondaryLocksResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.CheckSecondaryLocksResponse) *CheckSecondaryLocksResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CheckSecondaryLocksResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CheckSecondaryLocksResponse)))
	IntoReprCheckSecondaryLocksResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCheckSecondaryLocksResponseGenerated(arena *runtime.Arena, dst *CheckSecondaryLocksResponse, src *kvrpcpbproto.CheckSecondaryLocksResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if values := src.GetLocks(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*LockInfo)(nil)))
		array := unsafe.Slice((**LockInfo)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprLockInfoGenerated(arena, value)
		}
		dst.locks.data = (**LockInfo)(ptr)
		dst.locks.len = C.size_t(len(values))
		dst.locks.cap = C.size_t(len(values))
	}
	dst.commit_ts = C.uint64_t(src.GetCommitTs())
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprCheckSecondaryLocksResponseGenerated(src *CheckSecondaryLocksResponse) *kvrpcpbproto.CheckSecondaryLocksResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CheckSecondaryLocksResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	if src.locks.data != nil && src.locks.len > 0 {
		length := int(src.locks.len)
		ptrs := unsafe.Slice((**LockInfo)(unsafe.Pointer(src.locks.data)), length)
		out.Locks = make([]*kvrpcpbproto.LockInfo, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Locks = append(out.Locks, FromReprLockInfoGenerated(ptr))
		}
	}
	out.CommitTs = uint64(src.commit_ts)
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprCheckTxnStatusRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.CheckTxnStatusRequest) *CheckTxnStatusRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CheckTxnStatusRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CheckTxnStatusRequest)))
	IntoReprCheckTxnStatusRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCheckTxnStatusRequestGenerated(arena *runtime.Arena, dst *CheckTxnStatusRequest, src *kvrpcpbproto.CheckTxnStatusRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetPrimaryKey()); length > 0 {
		dst.primary_key.data = (*C.uint8_t)(data)
		dst.primary_key.len = C.size_t(length)
	}
	dst.lock_ts = C.uint64_t(src.GetLockTs())
	dst.caller_start_ts = C.uint64_t(src.GetCallerStartTs())
	dst.current_ts = C.uint64_t(src.GetCurrentTs())
	dst.rollback_if_not_exist = C.bool(src.GetRollbackIfNotExist())
	dst.force_sync_commit = C.bool(src.GetForceSyncCommit())
	dst.resolving_pessimistic_lock = C.bool(src.GetResolvingPessimisticLock())
	dst.verify_is_primary = C.bool(src.GetVerifyIsPrimary())
	dst.is_txn_file = C.bool(src.GetIsTxnFile())
}

func FromReprCheckTxnStatusRequestGenerated(src *CheckTxnStatusRequest) *kvrpcpbproto.CheckTxnStatusRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CheckTxnStatusRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.PrimaryKey = runtime.BytesFrom(unsafe.Pointer(src.primary_key.data), int(src.primary_key.len))
	out.LockTs = uint64(src.lock_ts)
	out.CallerStartTs = uint64(src.caller_start_ts)
	out.CurrentTs = uint64(src.current_ts)
	out.RollbackIfNotExist = bool(src.rollback_if_not_exist)
	out.ForceSyncCommit = bool(src.force_sync_commit)
	out.ResolvingPessimisticLock = bool(src.resolving_pessimistic_lock)
	out.VerifyIsPrimary = bool(src.verify_is_primary)
	out.IsTxnFile = bool(src.is_txn_file)
	return out
}

func NewReprCheckTxnStatusResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.CheckTxnStatusResponse) *CheckTxnStatusResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CheckTxnStatusResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CheckTxnStatusResponse)))
	IntoReprCheckTxnStatusResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCheckTxnStatusResponseGenerated(arena *runtime.Arena, dst *CheckTxnStatusResponse, src *kvrpcpbproto.CheckTxnStatusResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	dst.lock_ttl = C.uint64_t(src.GetLockTtl())
	dst.commit_version = C.uint64_t(src.GetCommitVersion())
	dst.action = C.int32_t(int32(src.GetAction()))
	if value := src.GetLockInfo(); value != nil {
		dst.lock_info = NewReprLockInfoGenerated(arena, value)
	} else {
		dst.lock_info = nil
	}
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprCheckTxnStatusResponseGenerated(src *CheckTxnStatusResponse) *kvrpcpbproto.CheckTxnStatusResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CheckTxnStatusResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	out.LockTtl = uint64(src.lock_ttl)
	out.CommitVersion = uint64(src.commit_version)
	out.Action = kvrpcpbproto.Action(int32(src.action))
	if src.lock_info != nil {
		out.LockInfo = FromReprLockInfoGenerated(src.lock_info)
	}
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprCleanupRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.CleanupRequest) *CleanupRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CleanupRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CleanupRequest)))
	IntoReprCleanupRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCleanupRequestGenerated(arena *runtime.Arena, dst *CleanupRequest, src *kvrpcpbproto.CleanupRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	dst.start_version = C.uint64_t(src.GetStartVersion())
	dst.current_ts = C.uint64_t(src.GetCurrentTs())
}

func FromReprCleanupRequestGenerated(src *CleanupRequest) *kvrpcpbproto.CleanupRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CleanupRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.StartVersion = uint64(src.start_version)
	out.CurrentTs = uint64(src.current_ts)
	return out
}

func NewReprCleanupResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.CleanupResponse) *CleanupResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CleanupResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CleanupResponse)))
	IntoReprCleanupResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCleanupResponseGenerated(arena *runtime.Arena, dst *CleanupResponse, src *kvrpcpbproto.CleanupResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	dst.commit_version = C.uint64_t(src.GetCommitVersion())
}

func FromReprCleanupResponseGenerated(src *CleanupResponse) *kvrpcpbproto.CleanupResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CleanupResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	out.CommitVersion = uint64(src.commit_version)
	return out
}

func NewReprCommitRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.CommitRequest) *CommitRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CommitRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CommitRequest)))
	IntoReprCommitRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCommitRequestGenerated(arena *runtime.Arena, dst *CommitRequest, src *kvrpcpbproto.CommitRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.start_version = C.uint64_t(src.GetStartVersion())
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
	dst.commit_version = C.uint64_t(src.GetCommitVersion())
	dst.commit_role = C.int32_t(int32(src.GetCommitRole()))
	if data, length := arena.AllocBytes(src.GetPrimaryKey()); length > 0 {
		dst.primary_key.data = (*C.uint8_t)(data)
		dst.primary_key.len = C.size_t(length)
	}
	dst.use_async_commit = C.bool(src.GetUseAsyncCommit())
	dst.is_txn_file = C.bool(src.GetIsTxnFile())
}

func FromReprCommitRequestGenerated(src *CommitRequest) *kvrpcpbproto.CommitRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CommitRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartVersion = uint64(src.start_version)
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	out.CommitVersion = uint64(src.commit_version)
	out.CommitRole = kvrpcpbproto.CommitRole(int32(src.commit_role))
	out.PrimaryKey = runtime.BytesFrom(unsafe.Pointer(src.primary_key.data), int(src.primary_key.len))
	out.UseAsyncCommit = bool(src.use_async_commit)
	out.IsTxnFile = bool(src.is_txn_file)
	return out
}

func NewReprCommitResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.CommitResponse) *CommitResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CommitResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CommitResponse)))
	IntoReprCommitResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCommitResponseGenerated(arena *runtime.Arena, dst *CommitResponse, src *kvrpcpbproto.CommitResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	dst.commit_version = C.uint64_t(src.GetCommitVersion())
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprCommitResponseGenerated(src *CommitResponse) *kvrpcpbproto.CommitResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CommitResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	out.CommitVersion = uint64(src.commit_version)
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprCommitTsExpiredGenerated(arena *runtime.Arena, src *kvrpcpbproto.CommitTsExpired) *CommitTsExpired {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CommitTsExpired)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CommitTsExpired)))
	IntoReprCommitTsExpiredGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCommitTsExpiredGenerated(arena *runtime.Arena, dst *CommitTsExpired, src *kvrpcpbproto.CommitTsExpired) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	dst.attempted_commit_ts = C.uint64_t(src.GetAttemptedCommitTs())
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	dst.min_commit_ts = C.uint64_t(src.GetMinCommitTs())
}

func FromReprCommitTsExpiredGenerated(src *CommitTsExpired) *kvrpcpbproto.CommitTsExpired {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CommitTsExpired{}
	out.StartTs = uint64(src.start_ts)
	out.AttemptedCommitTs = uint64(src.attempted_commit_ts)
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.MinCommitTs = uint64(src.min_commit_ts)
	return out
}

func NewReprCommitTsTooLargeGenerated(arena *runtime.Arena, src *kvrpcpbproto.CommitTsTooLarge) *CommitTsTooLarge {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CommitTsTooLarge)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CommitTsTooLarge)))
	IntoReprCommitTsTooLargeGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCommitTsTooLargeGenerated(arena *runtime.Arena, dst *CommitTsTooLarge, src *kvrpcpbproto.CommitTsTooLarge) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.commit_ts = C.uint64_t(src.GetCommitTs())
}

func FromReprCommitTsTooLargeGenerated(src *CommitTsTooLarge) *kvrpcpbproto.CommitTsTooLarge {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CommitTsTooLarge{}
	out.CommitTs = uint64(src.commit_ts)
	return out
}

func NewReprCommitTxnRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.CommitTxnRequest) *CommitTxnRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CommitTxnRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CommitTxnRequest)))
	IntoReprCommitTxnRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCommitTxnRequestGenerated(arena *runtime.Arena, dst *CommitTxnRequest, src *kvrpcpbproto.CommitTxnRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.start_version = C.uint64_t(src.GetStartVersion())
	dst.max_txn_time_use_ms = C.uint64_t(src.GetMaxTxnTimeUseMs())
	dst.latest_schema_expire_ms = C.uint64_t(src.GetLatestSchemaExpireMs())
	if values := src.GetPrewriteReqs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*PrewriteRequest)(nil)))
		array := unsafe.Slice((**PrewriteRequest)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprPrewriteRequestGenerated(arena, value)
		}
		dst.prewrite_reqs.data = (**PrewriteRequest)(ptr)
		dst.prewrite_reqs.len = C.size_t(len(values))
		dst.prewrite_reqs.cap = C.size_t(len(values))
	}
}

func FromReprCommitTxnRequestGenerated(src *CommitTxnRequest) *kvrpcpbproto.CommitTxnRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CommitTxnRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartVersion = uint64(src.start_version)
	out.MaxTxnTimeUseMs = uint64(src.max_txn_time_use_ms)
	out.LatestSchemaExpireMs = uint64(src.latest_schema_expire_ms)
	if src.prewrite_reqs.data != nil && src.prewrite_reqs.len > 0 {
		length := int(src.prewrite_reqs.len)
		ptrs := unsafe.Slice((**PrewriteRequest)(unsafe.Pointer(src.prewrite_reqs.data)), length)
		out.PrewriteReqs = make([]*kvrpcpbproto.PrewriteRequest, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.PrewriteReqs = append(out.PrewriteReqs, FromReprPrewriteRequestGenerated(ptr))
		}
	}
	return out
}

func NewReprCommitTxnResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.CommitTxnResponse) *CommitTxnResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CommitTxnResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CommitTxnResponse)))
	IntoReprCommitTxnResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCommitTxnResponseGenerated(arena *runtime.Arena, dst *CommitTxnResponse, src *kvrpcpbproto.CommitTxnResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if values := src.GetPrewriteResps(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*PrewriteResponse)(nil)))
		array := unsafe.Slice((**PrewriteResponse)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprPrewriteResponseGenerated(arena, value)
		}
		dst.prewrite_resps.data = (**PrewriteResponse)(ptr)
		dst.prewrite_resps.len = C.size_t(len(values))
		dst.prewrite_resps.cap = C.size_t(len(values))
	}
	if value := src.GetCommitResp(); value != nil {
		dst.commit_resp = NewReprCommitResponseGenerated(arena, value)
	} else {
		dst.commit_resp = nil
	}
	dst.prewrite_success = C.bool(src.GetPrewriteSuccess())
	dst.commit_ts = C.uint64_t(src.GetCommitTs())
	dst.fallback_to_store_two_phase_commit = C.bool(src.GetFallbackToStoreTwoPhaseCommit())
}

func FromReprCommitTxnResponseGenerated(src *CommitTxnResponse) *kvrpcpbproto.CommitTxnResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CommitTxnResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	if src.prewrite_resps.data != nil && src.prewrite_resps.len > 0 {
		length := int(src.prewrite_resps.len)
		ptrs := unsafe.Slice((**PrewriteResponse)(unsafe.Pointer(src.prewrite_resps.data)), length)
		out.PrewriteResps = make([]*kvrpcpbproto.PrewriteResponse, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.PrewriteResps = append(out.PrewriteResps, FromReprPrewriteResponseGenerated(ptr))
		}
	}
	if src.commit_resp != nil {
		out.CommitResp = FromReprCommitResponseGenerated(src.commit_resp)
	}
	out.PrewriteSuccess = bool(src.prewrite_success)
	out.CommitTs = uint64(src.commit_ts)
	out.FallbackToStoreTwoPhaseCommit = bool(src.fallback_to_store_two_phase_commit)
	return out
}

func NewReprCompactErrorGenerated(arena *runtime.Arena, src *kvrpcpbproto.CompactError) *CompactError {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CompactError)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CompactError)))
	IntoReprCompactErrorGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCompactErrorGenerated(arena *runtime.Arena, dst *CompactError, src *kvrpcpbproto.CompactError) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.error_case = 0
	switch value := src.GetError().(type) {
	case *kvrpcpbproto.CompactError_ErrInvalidStartKey:
		dst.error_case = C.int32_t(1)
		if value.ErrInvalidStartKey != nil {
			dst.err_invalid_start_key = NewReprCompactErrorInvalidStartKeyGenerated(arena, value.ErrInvalidStartKey)
		} else {
			dst.err_invalid_start_key = nil
		}
	case *kvrpcpbproto.CompactError_ErrPhysicalTableNotExist:
		dst.error_case = C.int32_t(2)
		if value.ErrPhysicalTableNotExist != nil {
			dst.err_physical_table_not_exist = NewReprCompactErrorPhysicalTableNotExistGenerated(arena, value.ErrPhysicalTableNotExist)
		} else {
			dst.err_physical_table_not_exist = nil
		}
	case *kvrpcpbproto.CompactError_ErrCompactInProgress:
		dst.error_case = C.int32_t(3)
		if value.ErrCompactInProgress != nil {
			dst.err_compact_in_progress = NewReprCompactErrorCompactInProgressGenerated(arena, value.ErrCompactInProgress)
		} else {
			dst.err_compact_in_progress = nil
		}
	case *kvrpcpbproto.CompactError_ErrTooManyPendingTasks:
		dst.error_case = C.int32_t(4)
		if value.ErrTooManyPendingTasks != nil {
			dst.err_too_many_pending_tasks = NewReprCompactErrorTooManyPendingTasksGenerated(arena, value.ErrTooManyPendingTasks)
		} else {
			dst.err_too_many_pending_tasks = nil
		}
	default:
		dst.error_case = 0
	}
}

func FromReprCompactErrorGenerated(src *CompactError) *kvrpcpbproto.CompactError {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CompactError{}
	switch int32(src.error_case) {
	case 1:
		out.Error = &kvrpcpbproto.CompactError_ErrInvalidStartKey{
			ErrInvalidStartKey: FromReprCompactErrorInvalidStartKeyGenerated(src.err_invalid_start_key),
		}
	case 2:
		out.Error = &kvrpcpbproto.CompactError_ErrPhysicalTableNotExist{
			ErrPhysicalTableNotExist: FromReprCompactErrorPhysicalTableNotExistGenerated(src.err_physical_table_not_exist),
		}
	case 3:
		out.Error = &kvrpcpbproto.CompactError_ErrCompactInProgress{
			ErrCompactInProgress: FromReprCompactErrorCompactInProgressGenerated(src.err_compact_in_progress),
		}
	case 4:
		out.Error = &kvrpcpbproto.CompactError_ErrTooManyPendingTasks{
			ErrTooManyPendingTasks: FromReprCompactErrorTooManyPendingTasksGenerated(src.err_too_many_pending_tasks),
		}
	}
	return out
}

func NewReprCompactErrorCompactInProgressGenerated(arena *runtime.Arena, src *kvrpcpbproto.CompactErrorCompactInProgress) *CompactErrorCompactInProgress {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CompactErrorCompactInProgress)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CompactErrorCompactInProgress)))
	IntoReprCompactErrorCompactInProgressGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCompactErrorCompactInProgressGenerated(arena *runtime.Arena, dst *CompactErrorCompactInProgress, src *kvrpcpbproto.CompactErrorCompactInProgress) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprCompactErrorCompactInProgressGenerated(src *CompactErrorCompactInProgress) *kvrpcpbproto.CompactErrorCompactInProgress {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CompactErrorCompactInProgress{}
	return out
}

func NewReprCompactErrorInvalidStartKeyGenerated(arena *runtime.Arena, src *kvrpcpbproto.CompactErrorInvalidStartKey) *CompactErrorInvalidStartKey {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CompactErrorInvalidStartKey)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CompactErrorInvalidStartKey)))
	IntoReprCompactErrorInvalidStartKeyGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCompactErrorInvalidStartKeyGenerated(arena *runtime.Arena, dst *CompactErrorInvalidStartKey, src *kvrpcpbproto.CompactErrorInvalidStartKey) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprCompactErrorInvalidStartKeyGenerated(src *CompactErrorInvalidStartKey) *kvrpcpbproto.CompactErrorInvalidStartKey {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CompactErrorInvalidStartKey{}
	return out
}

func NewReprCompactErrorPhysicalTableNotExistGenerated(arena *runtime.Arena, src *kvrpcpbproto.CompactErrorPhysicalTableNotExist) *CompactErrorPhysicalTableNotExist {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CompactErrorPhysicalTableNotExist)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CompactErrorPhysicalTableNotExist)))
	IntoReprCompactErrorPhysicalTableNotExistGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCompactErrorPhysicalTableNotExistGenerated(arena *runtime.Arena, dst *CompactErrorPhysicalTableNotExist, src *kvrpcpbproto.CompactErrorPhysicalTableNotExist) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprCompactErrorPhysicalTableNotExistGenerated(src *CompactErrorPhysicalTableNotExist) *kvrpcpbproto.CompactErrorPhysicalTableNotExist {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CompactErrorPhysicalTableNotExist{}
	return out
}

func NewReprCompactErrorTooManyPendingTasksGenerated(arena *runtime.Arena, src *kvrpcpbproto.CompactErrorTooManyPendingTasks) *CompactErrorTooManyPendingTasks {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CompactErrorTooManyPendingTasks)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CompactErrorTooManyPendingTasks)))
	IntoReprCompactErrorTooManyPendingTasksGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCompactErrorTooManyPendingTasksGenerated(arena *runtime.Arena, dst *CompactErrorTooManyPendingTasks, src *kvrpcpbproto.CompactErrorTooManyPendingTasks) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprCompactErrorTooManyPendingTasksGenerated(src *CompactErrorTooManyPendingTasks) *kvrpcpbproto.CompactErrorTooManyPendingTasks {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CompactErrorTooManyPendingTasks{}
	return out
}

func NewReprCompactRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.CompactRequest) *CompactRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CompactRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CompactRequest)))
	IntoReprCompactRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCompactRequestGenerated(arena *runtime.Arena, dst *CompactRequest, src *kvrpcpbproto.CompactRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	dst.physical_table_id = C.int64_t(src.GetPhysicalTableId())
	dst.logical_table_id = C.int64_t(src.GetLogicalTableId())
	dst.api_version = C.int32_t(int32(src.GetApiVersion()))
	dst.keyspace_id = C.uint32_t(src.GetKeyspaceId())
}

func FromReprCompactRequestGenerated(src *CompactRequest) *kvrpcpbproto.CompactRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CompactRequest{}
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.PhysicalTableId = int64(src.physical_table_id)
	out.LogicalTableId = int64(src.logical_table_id)
	out.ApiVersion = kvrpcpbproto.APIVersion(int32(src.api_version))
	out.KeyspaceId = uint32(src.keyspace_id)
	return out
}

func NewReprCompactResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.CompactResponse) *CompactResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*CompactResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_CompactResponse)))
	IntoReprCompactResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprCompactResponseGenerated(arena *runtime.Arena, dst *CompactResponse, src *kvrpcpbproto.CompactResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprCompactErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	dst.has_remaining = C.bool(src.GetHasRemaining())
	if data, length := arena.AllocBytes(src.GetCompactedStartKey()); length > 0 {
		dst.compacted_start_key.data = (*C.uint8_t)(data)
		dst.compacted_start_key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetCompactedEndKey()); length > 0 {
		dst.compacted_end_key.data = (*C.uint8_t)(data)
		dst.compacted_end_key.len = C.size_t(length)
	}
}

func FromReprCompactResponseGenerated(src *CompactResponse) *kvrpcpbproto.CompactResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.CompactResponse{}
	if src.error != nil {
		out.Error = FromReprCompactErrorGenerated(src.error)
	}
	out.HasRemaining = bool(src.has_remaining)
	out.CompactedStartKey = runtime.BytesFrom(unsafe.Pointer(src.compacted_start_key.data), int(src.compacted_start_key.len))
	out.CompactedEndKey = runtime.BytesFrom(unsafe.Pointer(src.compacted_end_key.data), int(src.compacted_end_key.len))
	return out
}

func NewReprContextGenerated(arena *runtime.Arena, src *kvrpcpbproto.Context) *Context {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Context)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_Context)))
	IntoReprContextGenerated(arena, ptr, src)
	return ptr
}

func IntoReprContextGenerated(arena *runtime.Arena, dst *Context, src *kvrpcpbproto.Context) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
	if value := src.GetRegionEpoch(); value != nil {
		dst.region_epoch = (*C.metapb_RegionEpoch)(unsafe.Pointer(metapbffi.NewReprRegionEpochGenerated(arena, value)))
	} else {
		dst.region_epoch = nil
	}
	if value := src.GetPeer(); value != nil {
		dst.peer = (*C.metapb_Peer)(unsafe.Pointer(metapbffi.NewReprPeerGenerated(arena, value)))
	} else {
		dst.peer = nil
	}
	dst.term = C.uint64_t(src.GetTerm())
	dst.priority = C.int32_t(int32(src.GetPriority()))
	dst.isolation_level = C.int32_t(int32(src.GetIsolationLevel()))
	dst.not_fill_cache = C.bool(src.GetNotFillCache())
	dst.sync_log = C.bool(src.GetSyncLog())
	dst.record_time_stat = C.bool(src.GetRecordTimeStat())
	dst.record_scan_stat = C.bool(src.GetRecordScanStat())
	dst.replica_read = C.bool(src.GetReplicaRead())
	if values := src.GetResolvedLocks(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.resolved_locks.data = (*C.uint64_t)(ptr)
		dst.resolved_locks.len = C.size_t(len(values))
		dst.resolved_locks.cap = C.size_t(len(values))
	}
	dst.max_execution_duration_ms = C.uint64_t(src.GetMaxExecutionDurationMs())
	dst.applied_index = C.uint64_t(src.GetAppliedIndex())
	dst.task_id = C.uint64_t(src.GetTaskId())
	dst.stale_read = C.bool(src.GetStaleRead())
	if data, length := arena.AllocBytes(src.GetResourceGroupTag()); length > 0 {
		dst.resource_group_tag.data = (*C.uint8_t)(data)
		dst.resource_group_tag.len = C.size_t(length)
	}
	dst.disk_full_opt = C.int32_t(int32(src.GetDiskFullOpt()))
	dst.is_retry_request = C.bool(src.GetIsRetryRequest())
	dst.api_version = C.int32_t(int32(src.GetApiVersion()))
	if values := src.GetCommittedLocks(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.committed_locks.data = (*C.uint64_t)(ptr)
		dst.committed_locks.len = C.size_t(len(values))
		dst.committed_locks.cap = C.size_t(len(values))
	}
	if value := src.GetTraceContext(); value != nil {
		dst.trace_context = (*C.tracepb_TraceContext)(unsafe.Pointer(tracepbffi.NewReprTraceContextGenerated(arena, value)))
	} else {
		dst.trace_context = nil
	}
	if data, length := arena.AllocString(src.GetRequestSource()); length > 0 {
		dst.request_source.data = (*C.char)(data)
		dst.request_source.len = C.size_t(length)
	}
	dst.txn_source = C.uint64_t(src.GetTxnSource())
	dst.busy_threshold_ms = C.uint32_t(src.GetBusyThresholdMs())
	if value := src.GetResourceControlContext(); value != nil {
		dst.resource_control_context = NewReprResourceControlContextGenerated(arena, value)
	} else {
		dst.resource_control_context = nil
	}
	dst.request_origin = C.int32_t(int32(src.GetRequestOrigin()))
	if data, length := arena.AllocString(src.GetKeyspaceName()); length > 0 {
		dst.keyspace_name.data = (*C.char)(data)
		dst.keyspace_name.len = C.size_t(length)
	}
	dst.keyspace_id = C.uint32_t(src.GetKeyspaceId())
	dst.buckets_version = C.uint64_t(src.GetBucketsVersion())
	if value := src.GetSourceStmt(); value != nil {
		dst.source_stmt = NewReprSourceStmtGenerated(arena, value)
	} else {
		dst.source_stmt = nil
	}
	dst.cluster_id = C.uint64_t(src.GetClusterId())
	if data, length := arena.AllocBytes(src.GetTraceId()); length > 0 {
		dst.trace_id.data = (*C.uint8_t)(data)
		dst.trace_id.len = C.size_t(length)
	}
	dst.trace_control_flags = C.uint64_t(src.GetTraceControlFlags())
}

func FromReprContextGenerated(src *Context) *kvrpcpbproto.Context {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.Context{}
	out.RegionId = uint64(src.region_id)
	if src.region_epoch != nil {
		out.RegionEpoch = metapbffi.FromReprRegionEpochGenerated((*metapbffi.RegionEpoch)(unsafe.Pointer(src.region_epoch)))
	}
	if src.peer != nil {
		out.Peer = metapbffi.FromReprPeerGenerated((*metapbffi.Peer)(unsafe.Pointer(src.peer)))
	}
	out.Term = uint64(src.term)
	out.Priority = kvrpcpbproto.CommandPri(int32(src.priority))
	out.IsolationLevel = kvrpcpbproto.IsolationLevel(int32(src.isolation_level))
	out.NotFillCache = bool(src.not_fill_cache)
	out.SyncLog = bool(src.sync_log)
	out.RecordTimeStat = bool(src.record_time_stat)
	out.RecordScanStat = bool(src.record_scan_stat)
	out.ReplicaRead = bool(src.replica_read)
	if src.resolved_locks.data != nil && src.resolved_locks.len > 0 {
		length := int(src.resolved_locks.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.resolved_locks.data)), length)
		out.ResolvedLocks = make([]uint64, 0, length)
		for _, value := range values {
			out.ResolvedLocks = append(out.ResolvedLocks, uint64(value))
		}
	}
	out.MaxExecutionDurationMs = uint64(src.max_execution_duration_ms)
	out.AppliedIndex = uint64(src.applied_index)
	out.TaskId = uint64(src.task_id)
	out.StaleRead = bool(src.stale_read)
	out.ResourceGroupTag = runtime.BytesFrom(unsafe.Pointer(src.resource_group_tag.data), int(src.resource_group_tag.len))
	out.DiskFullOpt = kvrpcpbproto.DiskFullOpt(int32(src.disk_full_opt))
	out.IsRetryRequest = bool(src.is_retry_request)
	out.ApiVersion = kvrpcpbproto.APIVersion(int32(src.api_version))
	if src.committed_locks.data != nil && src.committed_locks.len > 0 {
		length := int(src.committed_locks.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.committed_locks.data)), length)
		out.CommittedLocks = make([]uint64, 0, length)
		for _, value := range values {
			out.CommittedLocks = append(out.CommittedLocks, uint64(value))
		}
	}
	if src.trace_context != nil {
		out.TraceContext = tracepbffi.FromReprTraceContextGenerated((*tracepbffi.TraceContext)(unsafe.Pointer(src.trace_context)))
	}
	out.RequestSource = runtime.StringFrom(unsafe.Pointer(src.request_source.data), int(src.request_source.len))
	out.TxnSource = uint64(src.txn_source)
	out.BusyThresholdMs = uint32(src.busy_threshold_ms)
	if src.resource_control_context != nil {
		out.ResourceControlContext = FromReprResourceControlContextGenerated(src.resource_control_context)
	}
	out.RequestOrigin = kvrpcpbproto.RequestOrigin(int32(src.request_origin))
	out.KeyspaceName = runtime.StringFrom(unsafe.Pointer(src.keyspace_name.data), int(src.keyspace_name.len))
	out.KeyspaceId = uint32(src.keyspace_id)
	out.BucketsVersion = uint64(src.buckets_version)
	if src.source_stmt != nil {
		out.SourceStmt = FromReprSourceStmtGenerated(src.source_stmt)
	}
	out.ClusterId = uint64(src.cluster_id)
	out.TraceId = runtime.BytesFrom(unsafe.Pointer(src.trace_id.data), int(src.trace_id.len))
	out.TraceControlFlags = uint64(src.trace_control_flags)
	return out
}

func NewReprDeadlockGenerated(arena *runtime.Arena, src *kvrpcpbproto.Deadlock) *Deadlock {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Deadlock)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_Deadlock)))
	IntoReprDeadlockGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDeadlockGenerated(arena *runtime.Arena, dst *Deadlock, src *kvrpcpbproto.Deadlock) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.lock_ts = C.uint64_t(src.GetLockTs())
	if data, length := arena.AllocBytes(src.GetLockKey()); length > 0 {
		dst.lock_key.data = (*C.uint8_t)(data)
		dst.lock_key.len = C.size_t(length)
	}
	dst.deadlock_key_hash = C.uint64_t(src.GetDeadlockKeyHash())
	if values := src.GetWaitChain(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*C.deadlock_WaitForEntry)(nil)))
		array := unsafe.Slice((**C.deadlock_WaitForEntry)(ptr), len(values))
		for i, value := range values {
			array[i] = (*C.deadlock_WaitForEntry)(unsafe.Pointer(deadlockffi.NewReprWaitForEntryGenerated(arena, value)))
		}
		dst.wait_chain.data = (**C.deadlock_WaitForEntry)(ptr)
		dst.wait_chain.len = C.size_t(len(values))
		dst.wait_chain.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocBytes(src.GetDeadlockKey()); length > 0 {
		dst.deadlock_key.data = (*C.uint8_t)(data)
		dst.deadlock_key.len = C.size_t(length)
	}
}

func FromReprDeadlockGenerated(src *Deadlock) *kvrpcpbproto.Deadlock {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.Deadlock{}
	out.LockTs = uint64(src.lock_ts)
	out.LockKey = runtime.BytesFrom(unsafe.Pointer(src.lock_key.data), int(src.lock_key.len))
	out.DeadlockKeyHash = uint64(src.deadlock_key_hash)
	if src.wait_chain.data != nil && src.wait_chain.len > 0 {
		length := int(src.wait_chain.len)
		ptrs := unsafe.Slice((**C.deadlock_WaitForEntry)(unsafe.Pointer(src.wait_chain.data)), length)
		out.WaitChain = make([]*deadlockproto.WaitForEntry, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.WaitChain = append(out.WaitChain, deadlockffi.FromReprWaitForEntryGenerated((*deadlockffi.WaitForEntry)(unsafe.Pointer(ptr))))
		}
	}
	out.DeadlockKey = runtime.BytesFrom(unsafe.Pointer(src.deadlock_key.data), int(src.deadlock_key.len))
	return out
}

func NewReprDebugInfoGenerated(arena *runtime.Arena, src *kvrpcpbproto.DebugInfo) *DebugInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DebugInfo)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_DebugInfo)))
	IntoReprDebugInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDebugInfoGenerated(arena *runtime.Arena, dst *DebugInfo, src *kvrpcpbproto.DebugInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetMvccInfo(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*MvccDebugInfo)(nil)))
		array := unsafe.Slice((**MvccDebugInfo)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprMvccDebugInfoGenerated(arena, value)
		}
		dst.mvcc_info.data = (**MvccDebugInfo)(ptr)
		dst.mvcc_info.len = C.size_t(len(values))
		dst.mvcc_info.cap = C.size_t(len(values))
	}
}

func FromReprDebugInfoGenerated(src *DebugInfo) *kvrpcpbproto.DebugInfo {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.DebugInfo{}
	if src.mvcc_info.data != nil && src.mvcc_info.len > 0 {
		length := int(src.mvcc_info.len)
		ptrs := unsafe.Slice((**MvccDebugInfo)(unsafe.Pointer(src.mvcc_info.data)), length)
		out.MvccInfo = make([]*kvrpcpbproto.MvccDebugInfo, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.MvccInfo = append(out.MvccInfo, FromReprMvccDebugInfoGenerated(ptr))
		}
	}
	return out
}

func NewReprDeleteRangeRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.DeleteRangeRequest) *DeleteRangeRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DeleteRangeRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_DeleteRangeRequest)))
	IntoReprDeleteRangeRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDeleteRangeRequestGenerated(arena *runtime.Arena, dst *DeleteRangeRequest, src *kvrpcpbproto.DeleteRangeRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
	dst.notify_only = C.bool(src.GetNotifyOnly())
}

func FromReprDeleteRangeRequestGenerated(src *DeleteRangeRequest) *kvrpcpbproto.DeleteRangeRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.DeleteRangeRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	out.NotifyOnly = bool(src.notify_only)
	return out
}

func NewReprDeleteRangeResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.DeleteRangeResponse) *DeleteRangeResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DeleteRangeResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_DeleteRangeResponse)))
	IntoReprDeleteRangeResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDeleteRangeResponseGenerated(arena *runtime.Arena, dst *DeleteRangeResponse, src *kvrpcpbproto.DeleteRangeResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprDeleteRangeResponseGenerated(src *DeleteRangeResponse) *kvrpcpbproto.DeleteRangeResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.DeleteRangeResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprExecDetailsGenerated(arena *runtime.Arena, src *kvrpcpbproto.ExecDetails) *ExecDetails {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ExecDetails)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ExecDetails)))
	IntoReprExecDetailsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprExecDetailsGenerated(arena *runtime.Arena, dst *ExecDetails, src *kvrpcpbproto.ExecDetails) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetTimeDetail(); value != nil {
		dst.time_detail = NewReprTimeDetailGenerated(arena, value)
	} else {
		dst.time_detail = nil
	}
	if value := src.GetScanDetail(); value != nil {
		dst.scan_detail = NewReprScanDetailGenerated(arena, value)
	} else {
		dst.scan_detail = nil
	}
}

func FromReprExecDetailsGenerated(src *ExecDetails) *kvrpcpbproto.ExecDetails {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ExecDetails{}
	if src.time_detail != nil {
		out.TimeDetail = FromReprTimeDetailGenerated(src.time_detail)
	}
	if src.scan_detail != nil {
		out.ScanDetail = FromReprScanDetailGenerated(src.scan_detail)
	}
	return out
}

func NewReprExecDetailsV2Generated(arena *runtime.Arena, src *kvrpcpbproto.ExecDetailsV2) *ExecDetailsV2 {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ExecDetailsV2)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ExecDetailsV2)))
	IntoReprExecDetailsV2Generated(arena, ptr, src)
	return ptr
}

func IntoReprExecDetailsV2Generated(arena *runtime.Arena, dst *ExecDetailsV2, src *kvrpcpbproto.ExecDetailsV2) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetTimeDetail(); value != nil {
		dst.time_detail = NewReprTimeDetailGenerated(arena, value)
	} else {
		dst.time_detail = nil
	}
	if value := src.GetScanDetailV2(); value != nil {
		dst.scan_detail_v2 = NewReprScanDetailV2Generated(arena, value)
	} else {
		dst.scan_detail_v2 = nil
	}
	if value := src.GetWriteDetail(); value != nil {
		dst.write_detail = NewReprWriteDetailGenerated(arena, value)
	} else {
		dst.write_detail = nil
	}
	if value := src.GetTimeDetailV2(); value != nil {
		dst.time_detail_v2 = NewReprTimeDetailV2Generated(arena, value)
	} else {
		dst.time_detail_v2 = nil
	}
	if value := src.GetRuV2(); value != nil {
		dst.ru_v2 = NewReprRUV2Generated(arena, value)
	} else {
		dst.ru_v2 = nil
	}
}

func FromReprExecDetailsV2Generated(src *ExecDetailsV2) *kvrpcpbproto.ExecDetailsV2 {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ExecDetailsV2{}
	if src.time_detail != nil {
		out.TimeDetail = FromReprTimeDetailGenerated(src.time_detail)
	}
	if src.scan_detail_v2 != nil {
		out.ScanDetailV2 = FromReprScanDetailV2Generated(src.scan_detail_v2)
	}
	if src.write_detail != nil {
		out.WriteDetail = FromReprWriteDetailGenerated(src.write_detail)
	}
	if src.time_detail_v2 != nil {
		out.TimeDetailV2 = FromReprTimeDetailV2Generated(src.time_detail_v2)
	}
	if src.ru_v2 != nil {
		out.RuV2 = FromReprRUV2Generated(src.ru_v2)
	}
	return out
}

func NewReprExecutorInputsGenerated(arena *runtime.Arena, src *kvrpcpbproto.ExecutorInputs) *ExecutorInputs {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ExecutorInputs)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ExecutorInputs)))
	IntoReprExecutorInputsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprExecutorInputsGenerated(arena *runtime.Arena, dst *ExecutorInputs, src *kvrpcpbproto.ExecutorInputs) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.tikv_coprocessor_executor_work_total_batch_index_scan = C.uint64_t(src.GetTikvCoprocessorExecutorWorkTotalBatchIndexScan())
	dst.tikv_coprocessor_executor_work_total_batch_table_scan = C.uint64_t(src.GetTikvCoprocessorExecutorWorkTotalBatchTableScan())
	dst.tikv_coprocessor_executor_work_total_batch_selection = C.uint64_t(src.GetTikvCoprocessorExecutorWorkTotalBatchSelection())
	dst.tikv_coprocessor_executor_work_total_batch_top_n = C.uint64_t(src.GetTikvCoprocessorExecutorWorkTotalBatchTopN())
	dst.tikv_coprocessor_executor_work_total_batch_limit = C.uint64_t(src.GetTikvCoprocessorExecutorWorkTotalBatchLimit())
	dst.tikv_coprocessor_executor_work_total_batch_simple_aggr = C.uint64_t(src.GetTikvCoprocessorExecutorWorkTotalBatchSimpleAggr())
	dst.tikv_coprocessor_executor_work_total_batch_fast_hash_aggr = C.uint64_t(src.GetTikvCoprocessorExecutorWorkTotalBatchFastHashAggr())
}

func FromReprExecutorInputsGenerated(src *ExecutorInputs) *kvrpcpbproto.ExecutorInputs {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ExecutorInputs{}
	out.TikvCoprocessorExecutorWorkTotalBatchIndexScan = uint64(src.tikv_coprocessor_executor_work_total_batch_index_scan)
	out.TikvCoprocessorExecutorWorkTotalBatchTableScan = uint64(src.tikv_coprocessor_executor_work_total_batch_table_scan)
	out.TikvCoprocessorExecutorWorkTotalBatchSelection = uint64(src.tikv_coprocessor_executor_work_total_batch_selection)
	out.TikvCoprocessorExecutorWorkTotalBatchTopN = uint64(src.tikv_coprocessor_executor_work_total_batch_top_n)
	out.TikvCoprocessorExecutorWorkTotalBatchLimit = uint64(src.tikv_coprocessor_executor_work_total_batch_limit)
	out.TikvCoprocessorExecutorWorkTotalBatchSimpleAggr = uint64(src.tikv_coprocessor_executor_work_total_batch_simple_aggr)
	out.TikvCoprocessorExecutorWorkTotalBatchFastHashAggr = uint64(src.tikv_coprocessor_executor_work_total_batch_fast_hash_aggr)
	return out
}

func NewReprFlashbackToVersionRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.FlashbackToVersionRequest) *FlashbackToVersionRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FlashbackToVersionRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_FlashbackToVersionRequest)))
	IntoReprFlashbackToVersionRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFlashbackToVersionRequestGenerated(arena *runtime.Arena, dst *FlashbackToVersionRequest, src *kvrpcpbproto.FlashbackToVersionRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.version = C.uint64_t(src.GetVersion())
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	dst.commit_ts = C.uint64_t(src.GetCommitTs())
}

func FromReprFlashbackToVersionRequestGenerated(src *FlashbackToVersionRequest) *kvrpcpbproto.FlashbackToVersionRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.FlashbackToVersionRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Version = uint64(src.version)
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	out.StartTs = uint64(src.start_ts)
	out.CommitTs = uint64(src.commit_ts)
	return out
}

func NewReprFlashbackToVersionResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.FlashbackToVersionResponse) *FlashbackToVersionResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FlashbackToVersionResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_FlashbackToVersionResponse)))
	IntoReprFlashbackToVersionResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFlashbackToVersionResponseGenerated(arena *runtime.Arena, dst *FlashbackToVersionResponse, src *kvrpcpbproto.FlashbackToVersionResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprFlashbackToVersionResponseGenerated(src *FlashbackToVersionResponse) *kvrpcpbproto.FlashbackToVersionResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.FlashbackToVersionResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprFlushRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.FlushRequest) *FlushRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FlushRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_FlushRequest)))
	IntoReprFlushRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFlushRequestGenerated(arena *runtime.Arena, dst *FlushRequest, src *kvrpcpbproto.FlushRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if values := src.GetMutations(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*Mutation)(nil)))
		array := unsafe.Slice((**Mutation)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprMutationGenerated(arena, value)
		}
		dst.mutations.data = (**Mutation)(ptr)
		dst.mutations.len = C.size_t(len(values))
		dst.mutations.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocBytes(src.GetPrimaryKey()); length > 0 {
		dst.primary_key.data = (*C.uint8_t)(data)
		dst.primary_key.len = C.size_t(length)
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	dst.min_commit_ts = C.uint64_t(src.GetMinCommitTs())
	dst.generation = C.uint64_t(src.GetGeneration())
	dst.lock_ttl = C.uint64_t(src.GetLockTtl())
	dst.assertion_level = C.int32_t(int32(src.GetAssertionLevel()))
}

func FromReprFlushRequestGenerated(src *FlushRequest) *kvrpcpbproto.FlushRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.FlushRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	if src.mutations.data != nil && src.mutations.len > 0 {
		length := int(src.mutations.len)
		ptrs := unsafe.Slice((**Mutation)(unsafe.Pointer(src.mutations.data)), length)
		out.Mutations = make([]*kvrpcpbproto.Mutation, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Mutations = append(out.Mutations, FromReprMutationGenerated(ptr))
		}
	}
	out.PrimaryKey = runtime.BytesFrom(unsafe.Pointer(src.primary_key.data), int(src.primary_key.len))
	out.StartTs = uint64(src.start_ts)
	out.MinCommitTs = uint64(src.min_commit_ts)
	out.Generation = uint64(src.generation)
	out.LockTtl = uint64(src.lock_ttl)
	out.AssertionLevel = kvrpcpbproto.AssertionLevel(int32(src.assertion_level))
	return out
}

func NewReprFlushResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.FlushResponse) *FlushResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FlushResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_FlushResponse)))
	IntoReprFlushResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFlushResponseGenerated(arena *runtime.Arena, dst *FlushResponse, src *kvrpcpbproto.FlushResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetErrors(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KeyError)(nil)))
		array := unsafe.Slice((**KeyError)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKeyErrorGenerated(arena, value)
		}
		dst.errors.data = (**KeyError)(ptr)
		dst.errors.len = C.size_t(len(values))
		dst.errors.cap = C.size_t(len(values))
	}
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprFlushResponseGenerated(src *FlushResponse) *kvrpcpbproto.FlushResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.FlushResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.errors.data != nil && src.errors.len > 0 {
		length := int(src.errors.len)
		ptrs := unsafe.Slice((**KeyError)(unsafe.Pointer(src.errors.data)), length)
		out.Errors = make([]*kvrpcpbproto.KeyError, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Errors = append(out.Errors, FromReprKeyErrorGenerated(ptr))
		}
	}
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprGCRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.GCRequest) *GCRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GCRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_GCRequest)))
	IntoReprGCRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGCRequestGenerated(arena *runtime.Arena, dst *GCRequest, src *kvrpcpbproto.GCRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.safe_point = C.uint64_t(src.GetSafePoint())
}

func FromReprGCRequestGenerated(src *GCRequest) *kvrpcpbproto.GCRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.GCRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.SafePoint = uint64(src.safe_point)
	return out
}

func NewReprGCResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.GCResponse) *GCResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GCResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_GCResponse)))
	IntoReprGCResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGCResponseGenerated(arena *runtime.Arena, dst *GCResponse, src *kvrpcpbproto.GCResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
}

func FromReprGCResponseGenerated(src *GCResponse) *kvrpcpbproto.GCResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.GCResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	return out
}

func NewReprGetHealthFeedbackRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.GetHealthFeedbackRequest) *GetHealthFeedbackRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GetHealthFeedbackRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_GetHealthFeedbackRequest)))
	IntoReprGetHealthFeedbackRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGetHealthFeedbackRequestGenerated(arena *runtime.Arena, dst *GetHealthFeedbackRequest, src *kvrpcpbproto.GetHealthFeedbackRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
}

func FromReprGetHealthFeedbackRequestGenerated(src *GetHealthFeedbackRequest) *kvrpcpbproto.GetHealthFeedbackRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.GetHealthFeedbackRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	return out
}

func NewReprGetHealthFeedbackResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.GetHealthFeedbackResponse) *GetHealthFeedbackResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GetHealthFeedbackResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_GetHealthFeedbackResponse)))
	IntoReprGetHealthFeedbackResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGetHealthFeedbackResponseGenerated(arena *runtime.Arena, dst *GetHealthFeedbackResponse, src *kvrpcpbproto.GetHealthFeedbackResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetHealthFeedback(); value != nil {
		dst.health_feedback = NewReprHealthFeedbackGenerated(arena, value)
	} else {
		dst.health_feedback = nil
	}
}

func FromReprGetHealthFeedbackResponseGenerated(src *GetHealthFeedbackResponse) *kvrpcpbproto.GetHealthFeedbackResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.GetHealthFeedbackResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.health_feedback != nil {
		out.HealthFeedback = FromReprHealthFeedbackGenerated(src.health_feedback)
	}
	return out
}

func NewReprGetLockWaitHistoryRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.GetLockWaitHistoryRequest) *GetLockWaitHistoryRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GetLockWaitHistoryRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_GetLockWaitHistoryRequest)))
	IntoReprGetLockWaitHistoryRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGetLockWaitHistoryRequestGenerated(arena *runtime.Arena, dst *GetLockWaitHistoryRequest, src *kvrpcpbproto.GetLockWaitHistoryRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
}

func FromReprGetLockWaitHistoryRequestGenerated(src *GetLockWaitHistoryRequest) *kvrpcpbproto.GetLockWaitHistoryRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.GetLockWaitHistoryRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	return out
}

func NewReprGetLockWaitHistoryResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.GetLockWaitHistoryResponse) *GetLockWaitHistoryResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GetLockWaitHistoryResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_GetLockWaitHistoryResponse)))
	IntoReprGetLockWaitHistoryResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGetLockWaitHistoryResponseGenerated(arena *runtime.Arena, dst *GetLockWaitHistoryResponse, src *kvrpcpbproto.GetLockWaitHistoryResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	if values := src.GetEntries(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*C.deadlock_WaitForEntry)(nil)))
		array := unsafe.Slice((**C.deadlock_WaitForEntry)(ptr), len(values))
		for i, value := range values {
			array[i] = (*C.deadlock_WaitForEntry)(unsafe.Pointer(deadlockffi.NewReprWaitForEntryGenerated(arena, value)))
		}
		dst.entries.data = (**C.deadlock_WaitForEntry)(ptr)
		dst.entries.len = C.size_t(len(values))
		dst.entries.cap = C.size_t(len(values))
	}
}

func FromReprGetLockWaitHistoryResponseGenerated(src *GetLockWaitHistoryResponse) *kvrpcpbproto.GetLockWaitHistoryResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.GetLockWaitHistoryResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	if src.entries.data != nil && src.entries.len > 0 {
		length := int(src.entries.len)
		ptrs := unsafe.Slice((**C.deadlock_WaitForEntry)(unsafe.Pointer(src.entries.data)), length)
		out.Entries = make([]*deadlockproto.WaitForEntry, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Entries = append(out.Entries, deadlockffi.FromReprWaitForEntryGenerated((*deadlockffi.WaitForEntry)(unsafe.Pointer(ptr))))
		}
	}
	return out
}

func NewReprGetLockWaitInfoRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.GetLockWaitInfoRequest) *GetLockWaitInfoRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GetLockWaitInfoRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_GetLockWaitInfoRequest)))
	IntoReprGetLockWaitInfoRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGetLockWaitInfoRequestGenerated(arena *runtime.Arena, dst *GetLockWaitInfoRequest, src *kvrpcpbproto.GetLockWaitInfoRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
}

func FromReprGetLockWaitInfoRequestGenerated(src *GetLockWaitInfoRequest) *kvrpcpbproto.GetLockWaitInfoRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.GetLockWaitInfoRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	return out
}

func NewReprGetLockWaitInfoResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.GetLockWaitInfoResponse) *GetLockWaitInfoResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GetLockWaitInfoResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_GetLockWaitInfoResponse)))
	IntoReprGetLockWaitInfoResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGetLockWaitInfoResponseGenerated(arena *runtime.Arena, dst *GetLockWaitInfoResponse, src *kvrpcpbproto.GetLockWaitInfoResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	if values := src.GetEntries(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*C.deadlock_WaitForEntry)(nil)))
		array := unsafe.Slice((**C.deadlock_WaitForEntry)(ptr), len(values))
		for i, value := range values {
			array[i] = (*C.deadlock_WaitForEntry)(unsafe.Pointer(deadlockffi.NewReprWaitForEntryGenerated(arena, value)))
		}
		dst.entries.data = (**C.deadlock_WaitForEntry)(ptr)
		dst.entries.len = C.size_t(len(values))
		dst.entries.cap = C.size_t(len(values))
	}
}

func FromReprGetLockWaitInfoResponseGenerated(src *GetLockWaitInfoResponse) *kvrpcpbproto.GetLockWaitInfoResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.GetLockWaitInfoResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	if src.entries.data != nil && src.entries.len > 0 {
		length := int(src.entries.len)
		ptrs := unsafe.Slice((**C.deadlock_WaitForEntry)(unsafe.Pointer(src.entries.data)), length)
		out.Entries = make([]*deadlockproto.WaitForEntry, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Entries = append(out.Entries, deadlockffi.FromReprWaitForEntryGenerated((*deadlockffi.WaitForEntry)(unsafe.Pointer(ptr))))
		}
	}
	return out
}

func NewReprGetRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.GetRequest) *GetRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GetRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_GetRequest)))
	IntoReprGetRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGetRequestGenerated(arena *runtime.Arena, dst *GetRequest, src *kvrpcpbproto.GetRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	dst.version = C.uint64_t(src.GetVersion())
	dst.need_commit_ts = C.bool(src.GetNeedCommitTs())
}

func FromReprGetRequestGenerated(src *GetRequest) *kvrpcpbproto.GetRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.GetRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Version = uint64(src.version)
	out.NeedCommitTs = bool(src.need_commit_ts)
	return out
}

func NewReprGetResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.GetResponse) *GetResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GetResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_GetResponse)))
	IntoReprGetResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGetResponseGenerated(arena *runtime.Arena, dst *GetResponse, src *kvrpcpbproto.GetResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if data, length := arena.AllocBytes(src.GetValue()); length > 0 {
		dst.value.data = (*C.uint8_t)(data)
		dst.value.len = C.size_t(length)
	}
	dst.not_found = C.bool(src.GetNotFound())
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
	dst.commit_ts = C.uint64_t(src.GetCommitTs())
}

func FromReprGetResponseGenerated(src *GetResponse) *kvrpcpbproto.GetResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.GetResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	out.Value = runtime.BytesFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	out.NotFound = bool(src.not_found)
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	out.CommitTs = uint64(src.commit_ts)
	return out
}

func NewReprHealthFeedbackGenerated(arena *runtime.Arena, src *kvrpcpbproto.HealthFeedback) *HealthFeedback {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*HealthFeedback)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_HealthFeedback)))
	IntoReprHealthFeedbackGenerated(arena, ptr, src)
	return ptr
}

func IntoReprHealthFeedbackGenerated(arena *runtime.Arena, dst *HealthFeedback, src *kvrpcpbproto.HealthFeedback) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.store_id = C.uint64_t(src.GetStoreId())
	dst.feedback_seq_no = C.uint64_t(src.GetFeedbackSeqNo())
	dst.slow_score = C.int32_t(src.GetSlowScore())
}

func FromReprHealthFeedbackGenerated(src *HealthFeedback) *kvrpcpbproto.HealthFeedback {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.HealthFeedback{}
	out.StoreId = uint64(src.store_id)
	out.FeedbackSeqNo = uint64(src.feedback_seq_no)
	out.SlowScore = int32(src.slow_score)
	return out
}

func NewReprImportRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.ImportRequest) *ImportRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ImportRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ImportRequest)))
	IntoReprImportRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprImportRequestGenerated(arena *runtime.Arena, dst *ImportRequest, src *kvrpcpbproto.ImportRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetMutations(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*Mutation)(nil)))
		array := unsafe.Slice((**Mutation)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprMutationGenerated(arena, value)
		}
		dst.mutations.data = (**Mutation)(ptr)
		dst.mutations.len = C.size_t(len(values))
		dst.mutations.cap = C.size_t(len(values))
	}
	dst.commit_version = C.uint64_t(src.GetCommitVersion())
}

func FromReprImportRequestGenerated(src *ImportRequest) *kvrpcpbproto.ImportRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ImportRequest{}
	if src.mutations.data != nil && src.mutations.len > 0 {
		length := int(src.mutations.len)
		ptrs := unsafe.Slice((**Mutation)(unsafe.Pointer(src.mutations.data)), length)
		out.Mutations = make([]*kvrpcpbproto.Mutation, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Mutations = append(out.Mutations, FromReprMutationGenerated(ptr))
		}
	}
	out.CommitVersion = uint64(src.commit_version)
	return out
}

func NewReprImportResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.ImportResponse) *ImportResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ImportResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ImportResponse)))
	IntoReprImportResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprImportResponseGenerated(arena *runtime.Arena, dst *ImportResponse, src *kvrpcpbproto.ImportResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprImportResponseGenerated(src *ImportResponse) *kvrpcpbproto.ImportResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ImportResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprKeyErrorGenerated(arena *runtime.Arena, src *kvrpcpbproto.KeyError) *KeyError {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*KeyError)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_KeyError)))
	IntoReprKeyErrorGenerated(arena, ptr, src)
	return ptr
}

func IntoReprKeyErrorGenerated(arena *runtime.Arena, dst *KeyError, src *kvrpcpbproto.KeyError) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetLocked(); value != nil {
		dst.locked = NewReprLockInfoGenerated(arena, value)
	} else {
		dst.locked = nil
	}
	if data, length := arena.AllocString(src.GetRetryable()); length > 0 {
		dst.retryable.data = (*C.char)(data)
		dst.retryable.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetAbort()); length > 0 {
		dst.abort.data = (*C.char)(data)
		dst.abort.len = C.size_t(length)
	}
	if value := src.GetConflict(); value != nil {
		dst.conflict = NewReprWriteConflictGenerated(arena, value)
	} else {
		dst.conflict = nil
	}
	if value := src.GetAlreadyExist(); value != nil {
		dst.already_exist = NewReprAlreadyExistGenerated(arena, value)
	} else {
		dst.already_exist = nil
	}
	if value := src.GetDeadlock(); value != nil {
		dst.deadlock = NewReprDeadlockGenerated(arena, value)
	} else {
		dst.deadlock = nil
	}
	if value := src.GetCommitTsExpired(); value != nil {
		dst.commit_ts_expired = NewReprCommitTsExpiredGenerated(arena, value)
	} else {
		dst.commit_ts_expired = nil
	}
	if value := src.GetTxnNotFound(); value != nil {
		dst.txn_not_found = NewReprTxnNotFoundGenerated(arena, value)
	} else {
		dst.txn_not_found = nil
	}
	if value := src.GetCommitTsTooLarge(); value != nil {
		dst.commit_ts_too_large = NewReprCommitTsTooLargeGenerated(arena, value)
	} else {
		dst.commit_ts_too_large = nil
	}
	if value := src.GetAssertionFailed(); value != nil {
		dst.assertion_failed = NewReprAssertionFailedGenerated(arena, value)
	} else {
		dst.assertion_failed = nil
	}
	if value := src.GetPrimaryMismatch(); value != nil {
		dst.primary_mismatch = NewReprPrimaryMismatchGenerated(arena, value)
	} else {
		dst.primary_mismatch = nil
	}
	if value := src.GetTxnLockNotFound(); value != nil {
		dst.txn_lock_not_found = NewReprTxnLockNotFoundGenerated(arena, value)
	} else {
		dst.txn_lock_not_found = nil
	}
	if value := src.GetDebugInfo(); value != nil {
		dst.debug_info = NewReprDebugInfoGenerated(arena, value)
	} else {
		dst.debug_info = nil
	}
}

func FromReprKeyErrorGenerated(src *KeyError) *kvrpcpbproto.KeyError {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.KeyError{}
	if src.locked != nil {
		out.Locked = FromReprLockInfoGenerated(src.locked)
	}
	out.Retryable = runtime.StringFrom(unsafe.Pointer(src.retryable.data), int(src.retryable.len))
	out.Abort = runtime.StringFrom(unsafe.Pointer(src.abort.data), int(src.abort.len))
	if src.conflict != nil {
		out.Conflict = FromReprWriteConflictGenerated(src.conflict)
	}
	if src.already_exist != nil {
		out.AlreadyExist = FromReprAlreadyExistGenerated(src.already_exist)
	}
	if src.deadlock != nil {
		out.Deadlock = FromReprDeadlockGenerated(src.deadlock)
	}
	if src.commit_ts_expired != nil {
		out.CommitTsExpired = FromReprCommitTsExpiredGenerated(src.commit_ts_expired)
	}
	if src.txn_not_found != nil {
		out.TxnNotFound = FromReprTxnNotFoundGenerated(src.txn_not_found)
	}
	if src.commit_ts_too_large != nil {
		out.CommitTsTooLarge = FromReprCommitTsTooLargeGenerated(src.commit_ts_too_large)
	}
	if src.assertion_failed != nil {
		out.AssertionFailed = FromReprAssertionFailedGenerated(src.assertion_failed)
	}
	if src.primary_mismatch != nil {
		out.PrimaryMismatch = FromReprPrimaryMismatchGenerated(src.primary_mismatch)
	}
	if src.txn_lock_not_found != nil {
		out.TxnLockNotFound = FromReprTxnLockNotFoundGenerated(src.txn_lock_not_found)
	}
	if src.debug_info != nil {
		out.DebugInfo = FromReprDebugInfoGenerated(src.debug_info)
	}
	return out
}

func NewReprKeyRangeGenerated(arena *runtime.Arena, src *kvrpcpbproto.KeyRange) *KeyRange {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*KeyRange)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_KeyRange)))
	IntoReprKeyRangeGenerated(arena, ptr, src)
	return ptr
}

func IntoReprKeyRangeGenerated(arena *runtime.Arena, dst *KeyRange, src *kvrpcpbproto.KeyRange) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
}

func FromReprKeyRangeGenerated(src *KeyRange) *kvrpcpbproto.KeyRange {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.KeyRange{}
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	return out
}

func NewReprKvPairGenerated(arena *runtime.Arena, src *kvrpcpbproto.KvPair) *KvPair {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*KvPair)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_KvPair)))
	IntoReprKvPairGenerated(arena, ptr, src)
	return ptr
}

func IntoReprKvPairGenerated(arena *runtime.Arena, dst *KvPair, src *kvrpcpbproto.KvPair) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetValue()); length > 0 {
		dst.value.data = (*C.uint8_t)(data)
		dst.value.len = C.size_t(length)
	}
	dst.commit_ts = C.uint64_t(src.GetCommitTs())
}

func FromReprKvPairGenerated(src *KvPair) *kvrpcpbproto.KvPair {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.KvPair{}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Value = runtime.BytesFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	out.CommitTs = uint64(src.commit_ts)
	return out
}

func NewReprLeaderInfoGenerated(arena *runtime.Arena, src *kvrpcpbproto.LeaderInfo) *LeaderInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*LeaderInfo)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_LeaderInfo)))
	IntoReprLeaderInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprLeaderInfoGenerated(arena *runtime.Arena, dst *LeaderInfo, src *kvrpcpbproto.LeaderInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
	dst.peer_id = C.uint64_t(src.GetPeerId())
	dst.term = C.uint64_t(src.GetTerm())
	if value := src.GetRegionEpoch(); value != nil {
		dst.region_epoch = (*C.metapb_RegionEpoch)(unsafe.Pointer(metapbffi.NewReprRegionEpochGenerated(arena, value)))
	} else {
		dst.region_epoch = nil
	}
	if value := src.GetReadState(); value != nil {
		dst.read_state = NewReprReadStateGenerated(arena, value)
	} else {
		dst.read_state = nil
	}
}

func FromReprLeaderInfoGenerated(src *LeaderInfo) *kvrpcpbproto.LeaderInfo {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.LeaderInfo{}
	out.RegionId = uint64(src.region_id)
	out.PeerId = uint64(src.peer_id)
	out.Term = uint64(src.term)
	if src.region_epoch != nil {
		out.RegionEpoch = metapbffi.FromReprRegionEpochGenerated((*metapbffi.RegionEpoch)(unsafe.Pointer(src.region_epoch)))
	}
	if src.read_state != nil {
		out.ReadState = FromReprReadStateGenerated(src.read_state)
	}
	return out
}

func NewReprLockEntryGenerated(arena *runtime.Arena, src *kvrpcpbproto.LockEntry) *LockEntry {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*LockEntry)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_LockEntry)))
	IntoReprLockEntryGenerated(arena, ptr, src)
	return ptr
}

func IntoReprLockEntryGenerated(arena *runtime.Arena, dst *LockEntry, src *kvrpcpbproto.LockEntry) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	if data, length := arena.AllocBytes(src.GetPrimaryKey()); length > 0 {
		dst.primary_key.data = (*C.uint8_t)(data)
		dst.primary_key.len = C.size_t(length)
	}
	dst.lock_ttl = C.uint64_t(src.GetLockTtl())
}

func FromReprLockEntryGenerated(src *LockEntry) *kvrpcpbproto.LockEntry {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.LockEntry{}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.StartTs = uint64(src.start_ts)
	out.PrimaryKey = runtime.BytesFrom(unsafe.Pointer(src.primary_key.data), int(src.primary_key.len))
	out.LockTtl = uint64(src.lock_ttl)
	return out
}

func NewReprLockInfoGenerated(arena *runtime.Arena, src *kvrpcpbproto.LockInfo) *LockInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*LockInfo)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_LockInfo)))
	IntoReprLockInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprLockInfoGenerated(arena *runtime.Arena, dst *LockInfo, src *kvrpcpbproto.LockInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetPrimaryLock()); length > 0 {
		dst.primary_lock.data = (*C.uint8_t)(data)
		dst.primary_lock.len = C.size_t(length)
	}
	dst.lock_version = C.uint64_t(src.GetLockVersion())
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	dst.lock_ttl = C.uint64_t(src.GetLockTtl())
	dst.txn_size = C.uint64_t(src.GetTxnSize())
	dst.lock_type = C.int32_t(int32(src.GetLockType()))
	dst.lock_for_update_ts = C.uint64_t(src.GetLockForUpdateTs())
	dst.use_async_commit = C.bool(src.GetUseAsyncCommit())
	dst.min_commit_ts = C.uint64_t(src.GetMinCommitTs())
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.secondaries), src.GetSecondaries())
	dst.duration_to_last_update_ms = C.uint64_t(src.GetDurationToLastUpdateMs())
	if values := src.GetSharedLockInfos(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*LockInfo)(nil)))
		array := unsafe.Slice((**LockInfo)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprLockInfoGenerated(arena, value)
		}
		dst.shared_lock_infos.data = (**LockInfo)(ptr)
		dst.shared_lock_infos.len = C.size_t(len(values))
		dst.shared_lock_infos.cap = C.size_t(len(values))
	}
	dst.is_txn_file = C.bool(src.GetIsTxnFile())
}

func FromReprLockInfoGenerated(src *LockInfo) *kvrpcpbproto.LockInfo {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.LockInfo{}
	out.PrimaryLock = runtime.BytesFrom(unsafe.Pointer(src.primary_lock.data), int(src.primary_lock.len))
	out.LockVersion = uint64(src.lock_version)
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.LockTtl = uint64(src.lock_ttl)
	out.TxnSize = uint64(src.txn_size)
	out.LockType = kvrpcpbproto.Op(int32(src.lock_type))
	out.LockForUpdateTs = uint64(src.lock_for_update_ts)
	out.UseAsyncCommit = bool(src.use_async_commit)
	out.MinCommitTs = uint64(src.min_commit_ts)
	out.Secondaries = runtime.CopyBytesSlice(unsafe.Pointer(&src.secondaries))
	out.DurationToLastUpdateMs = uint64(src.duration_to_last_update_ms)
	if src.shared_lock_infos.data != nil && src.shared_lock_infos.len > 0 {
		length := int(src.shared_lock_infos.len)
		ptrs := unsafe.Slice((**LockInfo)(unsafe.Pointer(src.shared_lock_infos.data)), length)
		out.SharedLockInfos = make([]*kvrpcpbproto.LockInfo, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.SharedLockInfos = append(out.SharedLockInfos, FromReprLockInfoGenerated(ptr))
		}
	}
	out.IsTxnFile = bool(src.is_txn_file)
	return out
}

func NewReprMutationGenerated(arena *runtime.Arena, src *kvrpcpbproto.Mutation) *Mutation {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Mutation)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_Mutation)))
	IntoReprMutationGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMutationGenerated(arena *runtime.Arena, dst *Mutation, src *kvrpcpbproto.Mutation) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.op = C.int32_t(int32(src.GetOp()))
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetValue()); length > 0 {
		dst.value.data = (*C.uint8_t)(data)
		dst.value.len = C.size_t(length)
	}
	dst.assertion = C.int32_t(int32(src.GetAssertion()))
}

func FromReprMutationGenerated(src *Mutation) *kvrpcpbproto.Mutation {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.Mutation{}
	out.Op = kvrpcpbproto.Op(int32(src.op))
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Value = runtime.BytesFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	out.Assertion = kvrpcpbproto.Assertion(int32(src.assertion))
	return out
}

func NewReprMvccDebugInfoGenerated(arena *runtime.Arena, src *kvrpcpbproto.MvccDebugInfo) *MvccDebugInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MvccDebugInfo)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_MvccDebugInfo)))
	IntoReprMvccDebugInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMvccDebugInfoGenerated(arena *runtime.Arena, dst *MvccDebugInfo, src *kvrpcpbproto.MvccDebugInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if value := src.GetMvcc(); value != nil {
		dst.mvcc = NewReprMvccInfoGenerated(arena, value)
	} else {
		dst.mvcc = nil
	}
}

func FromReprMvccDebugInfoGenerated(src *MvccDebugInfo) *kvrpcpbproto.MvccDebugInfo {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.MvccDebugInfo{}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	if src.mvcc != nil {
		out.Mvcc = FromReprMvccInfoGenerated(src.mvcc)
	}
	return out
}

func NewReprMvccGetByKeyRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.MvccGetByKeyRequest) *MvccGetByKeyRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MvccGetByKeyRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_MvccGetByKeyRequest)))
	IntoReprMvccGetByKeyRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMvccGetByKeyRequestGenerated(arena *runtime.Arena, dst *MvccGetByKeyRequest, src *kvrpcpbproto.MvccGetByKeyRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
}

func FromReprMvccGetByKeyRequestGenerated(src *MvccGetByKeyRequest) *kvrpcpbproto.MvccGetByKeyRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.MvccGetByKeyRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	return out
}

func NewReprMvccGetByKeyResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.MvccGetByKeyResponse) *MvccGetByKeyResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MvccGetByKeyResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_MvccGetByKeyResponse)))
	IntoReprMvccGetByKeyResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMvccGetByKeyResponseGenerated(arena *runtime.Arena, dst *MvccGetByKeyResponse, src *kvrpcpbproto.MvccGetByKeyResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	if value := src.GetInfo(); value != nil {
		dst.info = NewReprMvccInfoGenerated(arena, value)
	} else {
		dst.info = nil
	}
}

func FromReprMvccGetByKeyResponseGenerated(src *MvccGetByKeyResponse) *kvrpcpbproto.MvccGetByKeyResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.MvccGetByKeyResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	if src.info != nil {
		out.Info = FromReprMvccInfoGenerated(src.info)
	}
	return out
}

func NewReprMvccGetByStartTsRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.MvccGetByStartTsRequest) *MvccGetByStartTsRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MvccGetByStartTsRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_MvccGetByStartTsRequest)))
	IntoReprMvccGetByStartTsRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMvccGetByStartTsRequestGenerated(arena *runtime.Arena, dst *MvccGetByStartTsRequest, src *kvrpcpbproto.MvccGetByStartTsRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
}

func FromReprMvccGetByStartTsRequestGenerated(src *MvccGetByStartTsRequest) *kvrpcpbproto.MvccGetByStartTsRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.MvccGetByStartTsRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartTs = uint64(src.start_ts)
	return out
}

func NewReprMvccGetByStartTsResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.MvccGetByStartTsResponse) *MvccGetByStartTsResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MvccGetByStartTsResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_MvccGetByStartTsResponse)))
	IntoReprMvccGetByStartTsResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMvccGetByStartTsResponseGenerated(arena *runtime.Arena, dst *MvccGetByStartTsResponse, src *kvrpcpbproto.MvccGetByStartTsResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if value := src.GetInfo(); value != nil {
		dst.info = NewReprMvccInfoGenerated(arena, value)
	} else {
		dst.info = nil
	}
}

func FromReprMvccGetByStartTsResponseGenerated(src *MvccGetByStartTsResponse) *kvrpcpbproto.MvccGetByStartTsResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.MvccGetByStartTsResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	if src.info != nil {
		out.Info = FromReprMvccInfoGenerated(src.info)
	}
	return out
}

func NewReprMvccInfoGenerated(arena *runtime.Arena, src *kvrpcpbproto.MvccInfo) *MvccInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MvccInfo)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_MvccInfo)))
	IntoReprMvccInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMvccInfoGenerated(arena *runtime.Arena, dst *MvccInfo, src *kvrpcpbproto.MvccInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetLock(); value != nil {
		dst.lock = NewReprMvccLockGenerated(arena, value)
	} else {
		dst.lock = nil
	}
	if values := src.GetWrites(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*MvccWrite)(nil)))
		array := unsafe.Slice((**MvccWrite)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprMvccWriteGenerated(arena, value)
		}
		dst.writes.data = (**MvccWrite)(ptr)
		dst.writes.len = C.size_t(len(values))
		dst.writes.cap = C.size_t(len(values))
	}
	if values := src.GetValues(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*MvccValue)(nil)))
		array := unsafe.Slice((**MvccValue)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprMvccValueGenerated(arena, value)
		}
		dst.values.data = (**MvccValue)(ptr)
		dst.values.len = C.size_t(len(values))
		dst.values.cap = C.size_t(len(values))
	}
}

func FromReprMvccInfoGenerated(src *MvccInfo) *kvrpcpbproto.MvccInfo {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.MvccInfo{}
	if src.lock != nil {
		out.Lock = FromReprMvccLockGenerated(src.lock)
	}
	if src.writes.data != nil && src.writes.len > 0 {
		length := int(src.writes.len)
		ptrs := unsafe.Slice((**MvccWrite)(unsafe.Pointer(src.writes.data)), length)
		out.Writes = make([]*kvrpcpbproto.MvccWrite, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Writes = append(out.Writes, FromReprMvccWriteGenerated(ptr))
		}
	}
	if src.values.data != nil && src.values.len > 0 {
		length := int(src.values.len)
		ptrs := unsafe.Slice((**MvccValue)(unsafe.Pointer(src.values.data)), length)
		out.Values = make([]*kvrpcpbproto.MvccValue, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Values = append(out.Values, FromReprMvccValueGenerated(ptr))
		}
	}
	return out
}

func NewReprMvccLockGenerated(arena *runtime.Arena, src *kvrpcpbproto.MvccLock) *MvccLock {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MvccLock)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_MvccLock)))
	IntoReprMvccLockGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMvccLockGenerated(arena *runtime.Arena, dst *MvccLock, src *kvrpcpbproto.MvccLock) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.type_field = C.int32_t(int32(src.GetType()))
	dst.start_ts = C.uint64_t(src.GetStartTs())
	if data, length := arena.AllocBytes(src.GetPrimary()); length > 0 {
		dst.primary.data = (*C.uint8_t)(data)
		dst.primary.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetShortValue()); length > 0 {
		dst.short_value.data = (*C.uint8_t)(data)
		dst.short_value.len = C.size_t(length)
	}
	dst.ttl = C.uint64_t(src.GetTtl())
	dst.for_update_ts = C.uint64_t(src.GetForUpdateTs())
	dst.txn_size = C.uint64_t(src.GetTxnSize())
	dst.use_async_commit = C.bool(src.GetUseAsyncCommit())
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.secondaries), src.GetSecondaries())
	if values := src.GetRollbackTs(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.rollback_ts.data = (*C.uint64_t)(ptr)
		dst.rollback_ts.len = C.size_t(len(values))
		dst.rollback_ts.cap = C.size_t(len(values))
	}
	dst.last_change_ts = C.uint64_t(src.GetLastChangeTs())
	dst.versions_to_last_change = C.uint64_t(src.GetVersionsToLastChange())
}

func FromReprMvccLockGenerated(src *MvccLock) *kvrpcpbproto.MvccLock {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.MvccLock{}
	out.Type = kvrpcpbproto.Op(int32(src.type_field))
	out.StartTs = uint64(src.start_ts)
	out.Primary = runtime.BytesFrom(unsafe.Pointer(src.primary.data), int(src.primary.len))
	out.ShortValue = runtime.BytesFrom(unsafe.Pointer(src.short_value.data), int(src.short_value.len))
	out.Ttl = uint64(src.ttl)
	out.ForUpdateTs = uint64(src.for_update_ts)
	out.TxnSize = uint64(src.txn_size)
	out.UseAsyncCommit = bool(src.use_async_commit)
	out.Secondaries = runtime.CopyBytesSlice(unsafe.Pointer(&src.secondaries))
	if src.rollback_ts.data != nil && src.rollback_ts.len > 0 {
		length := int(src.rollback_ts.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.rollback_ts.data)), length)
		out.RollbackTs = make([]uint64, 0, length)
		for _, value := range values {
			out.RollbackTs = append(out.RollbackTs, uint64(value))
		}
	}
	out.LastChangeTs = uint64(src.last_change_ts)
	out.VersionsToLastChange = uint64(src.versions_to_last_change)
	return out
}

func NewReprMvccValueGenerated(arena *runtime.Arena, src *kvrpcpbproto.MvccValue) *MvccValue {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MvccValue)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_MvccValue)))
	IntoReprMvccValueGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMvccValueGenerated(arena *runtime.Arena, dst *MvccValue, src *kvrpcpbproto.MvccValue) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	if data, length := arena.AllocBytes(src.GetValue()); length > 0 {
		dst.value.data = (*C.uint8_t)(data)
		dst.value.len = C.size_t(length)
	}
}

func FromReprMvccValueGenerated(src *MvccValue) *kvrpcpbproto.MvccValue {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.MvccValue{}
	out.StartTs = uint64(src.start_ts)
	out.Value = runtime.BytesFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	return out
}

func NewReprMvccWriteGenerated(arena *runtime.Arena, src *kvrpcpbproto.MvccWrite) *MvccWrite {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MvccWrite)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_MvccWrite)))
	IntoReprMvccWriteGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMvccWriteGenerated(arena *runtime.Arena, dst *MvccWrite, src *kvrpcpbproto.MvccWrite) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.type_field = C.int32_t(int32(src.GetType()))
	dst.start_ts = C.uint64_t(src.GetStartTs())
	dst.commit_ts = C.uint64_t(src.GetCommitTs())
	if data, length := arena.AllocBytes(src.GetShortValue()); length > 0 {
		dst.short_value.data = (*C.uint8_t)(data)
		dst.short_value.len = C.size_t(length)
	}
	dst.has_overlapped_rollback = C.bool(src.GetHasOverlappedRollback())
	dst.has_gc_fence = C.bool(src.GetHasGcFence())
	dst.gc_fence = C.uint64_t(src.GetGcFence())
	dst.last_change_ts = C.uint64_t(src.GetLastChangeTs())
	dst.versions_to_last_change = C.uint64_t(src.GetVersionsToLastChange())
}

func FromReprMvccWriteGenerated(src *MvccWrite) *kvrpcpbproto.MvccWrite {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.MvccWrite{}
	out.Type = kvrpcpbproto.Op(int32(src.type_field))
	out.StartTs = uint64(src.start_ts)
	out.CommitTs = uint64(src.commit_ts)
	out.ShortValue = runtime.BytesFrom(unsafe.Pointer(src.short_value.data), int(src.short_value.len))
	out.HasOverlappedRollback = bool(src.has_overlapped_rollback)
	out.HasGcFence = bool(src.has_gc_fence)
	out.GcFence = uint64(src.gc_fence)
	out.LastChangeTs = uint64(src.last_change_ts)
	out.VersionsToLastChange = uint64(src.versions_to_last_change)
	return out
}

func NewReprPessimisticLockKeyResultGenerated(arena *runtime.Arena, src *kvrpcpbproto.PessimisticLockKeyResult) *PessimisticLockKeyResult {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PessimisticLockKeyResult)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PessimisticLockKeyResult)))
	IntoReprPessimisticLockKeyResultGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPessimisticLockKeyResultGenerated(arena *runtime.Arena, dst *PessimisticLockKeyResult, src *kvrpcpbproto.PessimisticLockKeyResult) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.type_field = C.int32_t(int32(src.GetType()))
	if data, length := arena.AllocBytes(src.GetValue()); length > 0 {
		dst.value.data = (*C.uint8_t)(data)
		dst.value.len = C.size_t(length)
	}
	dst.existence = C.bool(src.GetExistence())
	dst.locked_with_conflict_ts = C.uint64_t(src.GetLockedWithConflictTs())
	dst.skip_resolving_lock = C.bool(src.GetSkipResolvingLock())
}

func FromReprPessimisticLockKeyResultGenerated(src *PessimisticLockKeyResult) *kvrpcpbproto.PessimisticLockKeyResult {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PessimisticLockKeyResult{}
	out.Type = kvrpcpbproto.PessimisticLockKeyResultType(int32(src.type_field))
	out.Value = runtime.BytesFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	out.Existence = bool(src.existence)
	out.LockedWithConflictTs = uint64(src.locked_with_conflict_ts)
	out.SkipResolvingLock = bool(src.skip_resolving_lock)
	return out
}

func NewReprPessimisticLockRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.PessimisticLockRequest) *PessimisticLockRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PessimisticLockRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PessimisticLockRequest)))
	IntoReprPessimisticLockRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPessimisticLockRequestGenerated(arena *runtime.Arena, dst *PessimisticLockRequest, src *kvrpcpbproto.PessimisticLockRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if values := src.GetMutations(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*Mutation)(nil)))
		array := unsafe.Slice((**Mutation)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprMutationGenerated(arena, value)
		}
		dst.mutations.data = (**Mutation)(ptr)
		dst.mutations.len = C.size_t(len(values))
		dst.mutations.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocBytes(src.GetPrimaryLock()); length > 0 {
		dst.primary_lock.data = (*C.uint8_t)(data)
		dst.primary_lock.len = C.size_t(length)
	}
	dst.start_version = C.uint64_t(src.GetStartVersion())
	dst.lock_ttl = C.uint64_t(src.GetLockTtl())
	dst.for_update_ts = C.uint64_t(src.GetForUpdateTs())
	dst.is_first_lock = C.bool(src.GetIsFirstLock())
	dst.wait_timeout = C.int64_t(src.GetWaitTimeout())
	dst.force = C.bool(src.GetForce())
	dst.return_values = C.bool(src.GetReturnValues())
	dst.min_commit_ts = C.uint64_t(src.GetMinCommitTs())
	dst.check_existence = C.bool(src.GetCheckExistence())
	dst.lock_only_if_exists = C.bool(src.GetLockOnlyIfExists())
	dst.wake_up_mode = C.int32_t(int32(src.GetWakeUpMode()))
}

func FromReprPessimisticLockRequestGenerated(src *PessimisticLockRequest) *kvrpcpbproto.PessimisticLockRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PessimisticLockRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	if src.mutations.data != nil && src.mutations.len > 0 {
		length := int(src.mutations.len)
		ptrs := unsafe.Slice((**Mutation)(unsafe.Pointer(src.mutations.data)), length)
		out.Mutations = make([]*kvrpcpbproto.Mutation, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Mutations = append(out.Mutations, FromReprMutationGenerated(ptr))
		}
	}
	out.PrimaryLock = runtime.BytesFrom(unsafe.Pointer(src.primary_lock.data), int(src.primary_lock.len))
	out.StartVersion = uint64(src.start_version)
	out.LockTtl = uint64(src.lock_ttl)
	out.ForUpdateTs = uint64(src.for_update_ts)
	out.IsFirstLock = bool(src.is_first_lock)
	out.WaitTimeout = int64(src.wait_timeout)
	out.Force = bool(src.force)
	out.ReturnValues = bool(src.return_values)
	out.MinCommitTs = uint64(src.min_commit_ts)
	out.CheckExistence = bool(src.check_existence)
	out.LockOnlyIfExists = bool(src.lock_only_if_exists)
	out.WakeUpMode = kvrpcpbproto.PessimisticLockWakeUpMode(int32(src.wake_up_mode))
	return out
}

func NewReprPessimisticLockResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.PessimisticLockResponse) *PessimisticLockResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PessimisticLockResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PessimisticLockResponse)))
	IntoReprPessimisticLockResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPessimisticLockResponseGenerated(arena *runtime.Arena, dst *PessimisticLockResponse, src *kvrpcpbproto.PessimisticLockResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetErrors(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KeyError)(nil)))
		array := unsafe.Slice((**KeyError)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKeyErrorGenerated(arena, value)
		}
		dst.errors.data = (**KeyError)(ptr)
		dst.errors.len = C.size_t(len(values))
		dst.errors.cap = C.size_t(len(values))
	}
	dst.commit_ts = C.uint64_t(src.GetCommitTs())
	if data, length := arena.AllocBytes(src.GetValue()); length > 0 {
		dst.value.data = (*C.uint8_t)(data)
		dst.value.len = C.size_t(length)
	}
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.values), src.GetValues())
	if values := src.GetNotFounds(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.bool(false)))
		array := unsafe.Slice((*C.bool)(ptr), len(values))
		for i, value := range values {
			if value {
				array[i] = C.bool(true)
			} else {
				array[i] = C.bool(false)
			}
		}
		dst.not_founds.data = (*C.bool)(ptr)
		dst.not_founds.len = C.size_t(len(values))
		dst.not_founds.cap = C.size_t(len(values))
	}
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
	if values := src.GetResults(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*PessimisticLockKeyResult)(nil)))
		array := unsafe.Slice((**PessimisticLockKeyResult)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprPessimisticLockKeyResultGenerated(arena, value)
		}
		dst.results.data = (**PessimisticLockKeyResult)(ptr)
		dst.results.len = C.size_t(len(values))
		dst.results.cap = C.size_t(len(values))
	}
}

func FromReprPessimisticLockResponseGenerated(src *PessimisticLockResponse) *kvrpcpbproto.PessimisticLockResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PessimisticLockResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.errors.data != nil && src.errors.len > 0 {
		length := int(src.errors.len)
		ptrs := unsafe.Slice((**KeyError)(unsafe.Pointer(src.errors.data)), length)
		out.Errors = make([]*kvrpcpbproto.KeyError, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Errors = append(out.Errors, FromReprKeyErrorGenerated(ptr))
		}
	}
	out.CommitTs = uint64(src.commit_ts)
	out.Value = runtime.BytesFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	out.Values = runtime.CopyBytesSlice(unsafe.Pointer(&src.values))
	if src.not_founds.data != nil && src.not_founds.len > 0 {
		length := int(src.not_founds.len)
		values := unsafe.Slice((*C.bool)(unsafe.Pointer(src.not_founds.data)), length)
		out.NotFounds = make([]bool, 0, length)
		for _, value := range values {
			out.NotFounds = append(out.NotFounds, bool(value))
		}
	}
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	if src.results.data != nil && src.results.len > 0 {
		length := int(src.results.len)
		ptrs := unsafe.Slice((**PessimisticLockKeyResult)(unsafe.Pointer(src.results.data)), length)
		out.Results = make([]*kvrpcpbproto.PessimisticLockKeyResult, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Results = append(out.Results, FromReprPessimisticLockKeyResultGenerated(ptr))
		}
	}
	return out
}

func NewReprPessimisticRollbackRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.PessimisticRollbackRequest) *PessimisticRollbackRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PessimisticRollbackRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PessimisticRollbackRequest)))
	IntoReprPessimisticRollbackRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPessimisticRollbackRequestGenerated(arena *runtime.Arena, dst *PessimisticRollbackRequest, src *kvrpcpbproto.PessimisticRollbackRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.start_version = C.uint64_t(src.GetStartVersion())
	dst.for_update_ts = C.uint64_t(src.GetForUpdateTs())
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
}

func FromReprPessimisticRollbackRequestGenerated(src *PessimisticRollbackRequest) *kvrpcpbproto.PessimisticRollbackRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PessimisticRollbackRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartVersion = uint64(src.start_version)
	out.ForUpdateTs = uint64(src.for_update_ts)
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	return out
}

func NewReprPessimisticRollbackResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.PessimisticRollbackResponse) *PessimisticRollbackResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PessimisticRollbackResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PessimisticRollbackResponse)))
	IntoReprPessimisticRollbackResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPessimisticRollbackResponseGenerated(arena *runtime.Arena, dst *PessimisticRollbackResponse, src *kvrpcpbproto.PessimisticRollbackResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetErrors(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KeyError)(nil)))
		array := unsafe.Slice((**KeyError)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKeyErrorGenerated(arena, value)
		}
		dst.errors.data = (**KeyError)(ptr)
		dst.errors.len = C.size_t(len(values))
		dst.errors.cap = C.size_t(len(values))
	}
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprPessimisticRollbackResponseGenerated(src *PessimisticRollbackResponse) *kvrpcpbproto.PessimisticRollbackResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PessimisticRollbackResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.errors.data != nil && src.errors.len > 0 {
		length := int(src.errors.len)
		ptrs := unsafe.Slice((**KeyError)(unsafe.Pointer(src.errors.data)), length)
		out.Errors = make([]*kvrpcpbproto.KeyError, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Errors = append(out.Errors, FromReprKeyErrorGenerated(ptr))
		}
	}
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprPhysicalScanLockRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.PhysicalScanLockRequest) *PhysicalScanLockRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PhysicalScanLockRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PhysicalScanLockRequest)))
	IntoReprPhysicalScanLockRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPhysicalScanLockRequestGenerated(arena *runtime.Arena, dst *PhysicalScanLockRequest, src *kvrpcpbproto.PhysicalScanLockRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.max_ts = C.uint64_t(src.GetMaxTs())
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	dst.limit = C.uint32_t(src.GetLimit())
}

func FromReprPhysicalScanLockRequestGenerated(src *PhysicalScanLockRequest) *kvrpcpbproto.PhysicalScanLockRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PhysicalScanLockRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.MaxTs = uint64(src.max_ts)
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.Limit = uint32(src.limit)
	return out
}

func NewReprPhysicalScanLockResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.PhysicalScanLockResponse) *PhysicalScanLockResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PhysicalScanLockResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PhysicalScanLockResponse)))
	IntoReprPhysicalScanLockResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPhysicalScanLockResponseGenerated(arena *runtime.Arena, dst *PhysicalScanLockResponse, src *kvrpcpbproto.PhysicalScanLockResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	if values := src.GetLocks(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*LockInfo)(nil)))
		array := unsafe.Slice((**LockInfo)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprLockInfoGenerated(arena, value)
		}
		dst.locks.data = (**LockInfo)(ptr)
		dst.locks.len = C.size_t(len(values))
		dst.locks.cap = C.size_t(len(values))
	}
}

func FromReprPhysicalScanLockResponseGenerated(src *PhysicalScanLockResponse) *kvrpcpbproto.PhysicalScanLockResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PhysicalScanLockResponse{}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	if src.locks.data != nil && src.locks.len > 0 {
		length := int(src.locks.len)
		ptrs := unsafe.Slice((**LockInfo)(unsafe.Pointer(src.locks.data)), length)
		out.Locks = make([]*kvrpcpbproto.LockInfo, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Locks = append(out.Locks, FromReprLockInfoGenerated(ptr))
		}
	}
	return out
}

func NewReprPrepareFlashbackToVersionRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.PrepareFlashbackToVersionRequest) *PrepareFlashbackToVersionRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PrepareFlashbackToVersionRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PrepareFlashbackToVersionRequest)))
	IntoReprPrepareFlashbackToVersionRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPrepareFlashbackToVersionRequestGenerated(arena *runtime.Arena, dst *PrepareFlashbackToVersionRequest, src *kvrpcpbproto.PrepareFlashbackToVersionRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	dst.version = C.uint64_t(src.GetVersion())
}

func FromReprPrepareFlashbackToVersionRequestGenerated(src *PrepareFlashbackToVersionRequest) *kvrpcpbproto.PrepareFlashbackToVersionRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PrepareFlashbackToVersionRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	out.StartTs = uint64(src.start_ts)
	out.Version = uint64(src.version)
	return out
}

func NewReprPrepareFlashbackToVersionResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.PrepareFlashbackToVersionResponse) *PrepareFlashbackToVersionResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PrepareFlashbackToVersionResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PrepareFlashbackToVersionResponse)))
	IntoReprPrepareFlashbackToVersionResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPrepareFlashbackToVersionResponseGenerated(arena *runtime.Arena, dst *PrepareFlashbackToVersionResponse, src *kvrpcpbproto.PrepareFlashbackToVersionResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprPrepareFlashbackToVersionResponseGenerated(src *PrepareFlashbackToVersionResponse) *kvrpcpbproto.PrepareFlashbackToVersionResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PrepareFlashbackToVersionResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprPrewriteRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.PrewriteRequest) *PrewriteRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PrewriteRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PrewriteRequest)))
	IntoReprPrewriteRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPrewriteRequestGenerated(arena *runtime.Arena, dst *PrewriteRequest, src *kvrpcpbproto.PrewriteRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if values := src.GetMutations(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*Mutation)(nil)))
		array := unsafe.Slice((**Mutation)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprMutationGenerated(arena, value)
		}
		dst.mutations.data = (**Mutation)(ptr)
		dst.mutations.len = C.size_t(len(values))
		dst.mutations.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocBytes(src.GetPrimaryLock()); length > 0 {
		dst.primary_lock.data = (*C.uint8_t)(data)
		dst.primary_lock.len = C.size_t(length)
	}
	dst.start_version = C.uint64_t(src.GetStartVersion())
	dst.lock_ttl = C.uint64_t(src.GetLockTtl())
	dst.skip_constraint_check = C.bool(src.GetSkipConstraintCheck())
	if values := src.GetPessimisticActions(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.int32_t(0)))
		array := unsafe.Slice((*C.int32_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.int32_t(int32(value))
		}
		dst.pessimistic_actions.data = (*C.int32_t)(ptr)
		dst.pessimistic_actions.len = C.size_t(len(values))
		dst.pessimistic_actions.cap = C.size_t(len(values))
	}
	dst.txn_size = C.uint64_t(src.GetTxnSize())
	dst.for_update_ts = C.uint64_t(src.GetForUpdateTs())
	dst.min_commit_ts = C.uint64_t(src.GetMinCommitTs())
	dst.use_async_commit = C.bool(src.GetUseAsyncCommit())
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.secondaries), src.GetSecondaries())
	dst.try_one_pc = C.bool(src.GetTryOnePc())
	dst.max_commit_ts = C.uint64_t(src.GetMaxCommitTs())
	dst.assertion_level = C.int32_t(int32(src.GetAssertionLevel()))
	if values := src.GetForUpdateTsConstraints(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*PrewriteRequest_ForUpdateTSConstraint)(nil)))
		array := unsafe.Slice((**PrewriteRequest_ForUpdateTSConstraint)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprPrewriteRequest_ForUpdateTSConstraintGenerated(arena, value)
		}
		dst.for_update_ts_constraints.data = (**PrewriteRequest_ForUpdateTSConstraint)(ptr)
		dst.for_update_ts_constraints.len = C.size_t(len(values))
		dst.for_update_ts_constraints.cap = C.size_t(len(values))
	}
	if values := src.GetTxnFileChunks(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.txn_file_chunks.data = (*C.uint64_t)(ptr)
		dst.txn_file_chunks.len = C.size_t(len(values))
		dst.txn_file_chunks.cap = C.size_t(len(values))
	}
}

func FromReprPrewriteRequestGenerated(src *PrewriteRequest) *kvrpcpbproto.PrewriteRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PrewriteRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	if src.mutations.data != nil && src.mutations.len > 0 {
		length := int(src.mutations.len)
		ptrs := unsafe.Slice((**Mutation)(unsafe.Pointer(src.mutations.data)), length)
		out.Mutations = make([]*kvrpcpbproto.Mutation, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Mutations = append(out.Mutations, FromReprMutationGenerated(ptr))
		}
	}
	out.PrimaryLock = runtime.BytesFrom(unsafe.Pointer(src.primary_lock.data), int(src.primary_lock.len))
	out.StartVersion = uint64(src.start_version)
	out.LockTtl = uint64(src.lock_ttl)
	out.SkipConstraintCheck = bool(src.skip_constraint_check)
	if src.pessimistic_actions.data != nil && src.pessimistic_actions.len > 0 {
		length := int(src.pessimistic_actions.len)
		values := unsafe.Slice((*C.int32_t)(unsafe.Pointer(src.pessimistic_actions.data)), length)
		out.PessimisticActions = make([]kvrpcpbproto.PrewriteRequest_PessimisticAction, 0, length)
		for _, value := range values {
			out.PessimisticActions = append(out.PessimisticActions, kvrpcpbproto.PrewriteRequest_PessimisticAction(int32(value)))
		}
	}
	out.TxnSize = uint64(src.txn_size)
	out.ForUpdateTs = uint64(src.for_update_ts)
	out.MinCommitTs = uint64(src.min_commit_ts)
	out.UseAsyncCommit = bool(src.use_async_commit)
	out.Secondaries = runtime.CopyBytesSlice(unsafe.Pointer(&src.secondaries))
	out.TryOnePc = bool(src.try_one_pc)
	out.MaxCommitTs = uint64(src.max_commit_ts)
	out.AssertionLevel = kvrpcpbproto.AssertionLevel(int32(src.assertion_level))
	if src.for_update_ts_constraints.data != nil && src.for_update_ts_constraints.len > 0 {
		length := int(src.for_update_ts_constraints.len)
		ptrs := unsafe.Slice((**PrewriteRequest_ForUpdateTSConstraint)(unsafe.Pointer(src.for_update_ts_constraints.data)), length)
		out.ForUpdateTsConstraints = make([]*kvrpcpbproto.PrewriteRequest_ForUpdateTSConstraint, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.ForUpdateTsConstraints = append(out.ForUpdateTsConstraints, FromReprPrewriteRequest_ForUpdateTSConstraintGenerated(ptr))
		}
	}
	if src.txn_file_chunks.data != nil && src.txn_file_chunks.len > 0 {
		length := int(src.txn_file_chunks.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.txn_file_chunks.data)), length)
		out.TxnFileChunks = make([]uint64, 0, length)
		for _, value := range values {
			out.TxnFileChunks = append(out.TxnFileChunks, uint64(value))
		}
	}
	return out
}

func NewReprPrewriteRequest_ForUpdateTSConstraintGenerated(arena *runtime.Arena, src *kvrpcpbproto.PrewriteRequest_ForUpdateTSConstraint) *PrewriteRequest_ForUpdateTSConstraint {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PrewriteRequest_ForUpdateTSConstraint)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PrewriteRequest_ForUpdateTSConstraint)))
	IntoReprPrewriteRequest_ForUpdateTSConstraintGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPrewriteRequest_ForUpdateTSConstraintGenerated(arena *runtime.Arena, dst *PrewriteRequest_ForUpdateTSConstraint, src *kvrpcpbproto.PrewriteRequest_ForUpdateTSConstraint) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.index = C.uint32_t(src.GetIndex())
	dst.expected_for_update_ts = C.uint64_t(src.GetExpectedForUpdateTs())
}

func FromReprPrewriteRequest_ForUpdateTSConstraintGenerated(src *PrewriteRequest_ForUpdateTSConstraint) *kvrpcpbproto.PrewriteRequest_ForUpdateTSConstraint {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PrewriteRequest_ForUpdateTSConstraint{}
	out.Index = uint32(src.index)
	out.ExpectedForUpdateTs = uint64(src.expected_for_update_ts)
	return out
}

func NewReprPrewriteResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.PrewriteResponse) *PrewriteResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PrewriteResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PrewriteResponse)))
	IntoReprPrewriteResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPrewriteResponseGenerated(arena *runtime.Arena, dst *PrewriteResponse, src *kvrpcpbproto.PrewriteResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetErrors(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KeyError)(nil)))
		array := unsafe.Slice((**KeyError)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKeyErrorGenerated(arena, value)
		}
		dst.errors.data = (**KeyError)(ptr)
		dst.errors.len = C.size_t(len(values))
		dst.errors.cap = C.size_t(len(values))
	}
	dst.min_commit_ts = C.uint64_t(src.GetMinCommitTs())
	dst.one_pc_commit_ts = C.uint64_t(src.GetOnePcCommitTs())
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprPrewriteResponseGenerated(src *PrewriteResponse) *kvrpcpbproto.PrewriteResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PrewriteResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.errors.data != nil && src.errors.len > 0 {
		length := int(src.errors.len)
		ptrs := unsafe.Slice((**KeyError)(unsafe.Pointer(src.errors.data)), length)
		out.Errors = make([]*kvrpcpbproto.KeyError, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Errors = append(out.Errors, FromReprKeyErrorGenerated(ptr))
		}
	}
	out.MinCommitTs = uint64(src.min_commit_ts)
	out.OnePcCommitTs = uint64(src.one_pc_commit_ts)
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprPrimaryMismatchGenerated(arena *runtime.Arena, src *kvrpcpbproto.PrimaryMismatch) *PrimaryMismatch {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PrimaryMismatch)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_PrimaryMismatch)))
	IntoReprPrimaryMismatchGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPrimaryMismatchGenerated(arena *runtime.Arena, dst *PrimaryMismatch, src *kvrpcpbproto.PrimaryMismatch) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetLockInfo(); value != nil {
		dst.lock_info = NewReprLockInfoGenerated(arena, value)
	} else {
		dst.lock_info = nil
	}
}

func FromReprPrimaryMismatchGenerated(src *PrimaryMismatch) *kvrpcpbproto.PrimaryMismatch {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.PrimaryMismatch{}
	if src.lock_info != nil {
		out.LockInfo = FromReprLockInfoGenerated(src.lock_info)
	}
	return out
}

func NewReprRUV2Generated(arena *runtime.Arena, src *kvrpcpbproto.RUV2) *RUV2 {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RUV2)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RUV2)))
	IntoReprRUV2Generated(arena, ptr, src)
	return ptr
}

func IntoReprRUV2Generated(arena *runtime.Arena, dst *RUV2, src *kvrpcpbproto.RUV2) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.kv_engine_cache_miss = C.uint64_t(src.GetKvEngineCacheMiss())
	if value := src.GetExecutorInputs(); value != nil {
		dst.executor_inputs = NewReprExecutorInputsGenerated(arena, value)
	} else {
		dst.executor_inputs = nil
	}
	dst.coprocessor_executor_iterations = C.uint64_t(src.GetCoprocessorExecutorIterations())
	dst.coprocessor_response_bytes = C.uint64_t(src.GetCoprocessorResponseBytes())
	dst.raftstore_store_write_trigger_wb_bytes = C.uint64_t(src.GetRaftstoreStoreWriteTriggerWbBytes())
	dst.storage_processed_keys_batch_get = C.uint64_t(src.GetStorageProcessedKeysBatchGet())
	dst.storage_processed_keys_get = C.uint64_t(src.GetStorageProcessedKeysGet())
	dst.read_rpc_count = C.uint64_t(src.GetReadRpcCount())
	dst.write_rpc_count = C.uint64_t(src.GetWriteRpcCount())
}

func FromReprRUV2Generated(src *RUV2) *kvrpcpbproto.RUV2 {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RUV2{}
	out.KvEngineCacheMiss = uint64(src.kv_engine_cache_miss)
	if src.executor_inputs != nil {
		out.ExecutorInputs = FromReprExecutorInputsGenerated(src.executor_inputs)
	}
	out.CoprocessorExecutorIterations = uint64(src.coprocessor_executor_iterations)
	out.CoprocessorResponseBytes = uint64(src.coprocessor_response_bytes)
	out.RaftstoreStoreWriteTriggerWbBytes = uint64(src.raftstore_store_write_trigger_wb_bytes)
	out.StorageProcessedKeysBatchGet = uint64(src.storage_processed_keys_batch_get)
	out.StorageProcessedKeysGet = uint64(src.storage_processed_keys_get)
	out.ReadRpcCount = uint64(src.read_rpc_count)
	out.WriteRpcCount = uint64(src.write_rpc_count)
	return out
}

func NewReprRawBatchDeleteRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawBatchDeleteRequest) *RawBatchDeleteRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawBatchDeleteRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawBatchDeleteRequest)))
	IntoReprRawBatchDeleteRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawBatchDeleteRequestGenerated(arena *runtime.Arena, dst *RawBatchDeleteRequest, src *kvrpcpbproto.RawBatchDeleteRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
	dst.for_cas = C.bool(src.GetForCas())
}

func FromReprRawBatchDeleteRequestGenerated(src *RawBatchDeleteRequest) *kvrpcpbproto.RawBatchDeleteRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawBatchDeleteRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	out.ForCas = bool(src.for_cas)
	return out
}

func NewReprRawBatchDeleteResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawBatchDeleteResponse) *RawBatchDeleteResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawBatchDeleteResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawBatchDeleteResponse)))
	IntoReprRawBatchDeleteResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawBatchDeleteResponseGenerated(arena *runtime.Arena, dst *RawBatchDeleteResponse, src *kvrpcpbproto.RawBatchDeleteResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprRawBatchDeleteResponseGenerated(src *RawBatchDeleteResponse) *kvrpcpbproto.RawBatchDeleteResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawBatchDeleteResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprRawBatchGetRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawBatchGetRequest) *RawBatchGetRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawBatchGetRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawBatchGetRequest)))
	IntoReprRawBatchGetRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawBatchGetRequestGenerated(arena *runtime.Arena, dst *RawBatchGetRequest, src *kvrpcpbproto.RawBatchGetRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
}

func FromReprRawBatchGetRequestGenerated(src *RawBatchGetRequest) *kvrpcpbproto.RawBatchGetRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawBatchGetRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	return out
}

func NewReprRawBatchGetResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawBatchGetResponse) *RawBatchGetResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawBatchGetResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawBatchGetResponse)))
	IntoReprRawBatchGetResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawBatchGetResponseGenerated(arena *runtime.Arena, dst *RawBatchGetResponse, src *kvrpcpbproto.RawBatchGetResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetPairs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KvPair)(nil)))
		array := unsafe.Slice((**KvPair)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKvPairGenerated(arena, value)
		}
		dst.pairs.data = (**KvPair)(ptr)
		dst.pairs.len = C.size_t(len(values))
		dst.pairs.cap = C.size_t(len(values))
	}
}

func FromReprRawBatchGetResponseGenerated(src *RawBatchGetResponse) *kvrpcpbproto.RawBatchGetResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawBatchGetResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.pairs.data != nil && src.pairs.len > 0 {
		length := int(src.pairs.len)
		ptrs := unsafe.Slice((**KvPair)(unsafe.Pointer(src.pairs.data)), length)
		out.Pairs = make([]*kvrpcpbproto.KvPair, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Pairs = append(out.Pairs, FromReprKvPairGenerated(ptr))
		}
	}
	return out
}

func NewReprRawBatchPutRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawBatchPutRequest) *RawBatchPutRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawBatchPutRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawBatchPutRequest)))
	IntoReprRawBatchPutRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawBatchPutRequestGenerated(arena *runtime.Arena, dst *RawBatchPutRequest, src *kvrpcpbproto.RawBatchPutRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if values := src.GetPairs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KvPair)(nil)))
		array := unsafe.Slice((**KvPair)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKvPairGenerated(arena, value)
		}
		dst.pairs.data = (**KvPair)(ptr)
		dst.pairs.len = C.size_t(len(values))
		dst.pairs.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
	dst.ttl = C.uint64_t(src.GetTtl())
	dst.for_cas = C.bool(src.GetForCas())
	if values := src.GetTtls(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.ttls.data = (*C.uint64_t)(ptr)
		dst.ttls.len = C.size_t(len(values))
		dst.ttls.cap = C.size_t(len(values))
	}
}

func FromReprRawBatchPutRequestGenerated(src *RawBatchPutRequest) *kvrpcpbproto.RawBatchPutRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawBatchPutRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	if src.pairs.data != nil && src.pairs.len > 0 {
		length := int(src.pairs.len)
		ptrs := unsafe.Slice((**KvPair)(unsafe.Pointer(src.pairs.data)), length)
		out.Pairs = make([]*kvrpcpbproto.KvPair, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Pairs = append(out.Pairs, FromReprKvPairGenerated(ptr))
		}
	}
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	out.Ttl = uint64(src.ttl)
	out.ForCas = bool(src.for_cas)
	if src.ttls.data != nil && src.ttls.len > 0 {
		length := int(src.ttls.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.ttls.data)), length)
		out.Ttls = make([]uint64, 0, length)
		for _, value := range values {
			out.Ttls = append(out.Ttls, uint64(value))
		}
	}
	return out
}

func NewReprRawBatchPutResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawBatchPutResponse) *RawBatchPutResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawBatchPutResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawBatchPutResponse)))
	IntoReprRawBatchPutResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawBatchPutResponseGenerated(arena *runtime.Arena, dst *RawBatchPutResponse, src *kvrpcpbproto.RawBatchPutResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprRawBatchPutResponseGenerated(src *RawBatchPutResponse) *kvrpcpbproto.RawBatchPutResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawBatchPutResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprRawBatchScanRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawBatchScanRequest) *RawBatchScanRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawBatchScanRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawBatchScanRequest)))
	IntoReprRawBatchScanRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawBatchScanRequestGenerated(arena *runtime.Arena, dst *RawBatchScanRequest, src *kvrpcpbproto.RawBatchScanRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if values := src.GetRanges(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KeyRange)(nil)))
		array := unsafe.Slice((**KeyRange)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKeyRangeGenerated(arena, value)
		}
		dst.ranges.data = (**KeyRange)(ptr)
		dst.ranges.len = C.size_t(len(values))
		dst.ranges.cap = C.size_t(len(values))
	}
	dst.each_limit = C.uint32_t(src.GetEachLimit())
	dst.key_only = C.bool(src.GetKeyOnly())
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
	dst.reverse = C.bool(src.GetReverse())
}

func FromReprRawBatchScanRequestGenerated(src *RawBatchScanRequest) *kvrpcpbproto.RawBatchScanRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawBatchScanRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	if src.ranges.data != nil && src.ranges.len > 0 {
		length := int(src.ranges.len)
		ptrs := unsafe.Slice((**KeyRange)(unsafe.Pointer(src.ranges.data)), length)
		out.Ranges = make([]*kvrpcpbproto.KeyRange, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Ranges = append(out.Ranges, FromReprKeyRangeGenerated(ptr))
		}
	}
	out.EachLimit = uint32(src.each_limit)
	out.KeyOnly = bool(src.key_only)
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	out.Reverse = bool(src.reverse)
	return out
}

func NewReprRawBatchScanResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawBatchScanResponse) *RawBatchScanResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawBatchScanResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawBatchScanResponse)))
	IntoReprRawBatchScanResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawBatchScanResponseGenerated(arena *runtime.Arena, dst *RawBatchScanResponse, src *kvrpcpbproto.RawBatchScanResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetKvs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KvPair)(nil)))
		array := unsafe.Slice((**KvPair)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKvPairGenerated(arena, value)
		}
		dst.kvs.data = (**KvPair)(ptr)
		dst.kvs.len = C.size_t(len(values))
		dst.kvs.cap = C.size_t(len(values))
	}
}

func FromReprRawBatchScanResponseGenerated(src *RawBatchScanResponse) *kvrpcpbproto.RawBatchScanResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawBatchScanResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.kvs.data != nil && src.kvs.len > 0 {
		length := int(src.kvs.len)
		ptrs := unsafe.Slice((**KvPair)(unsafe.Pointer(src.kvs.data)), length)
		out.Kvs = make([]*kvrpcpbproto.KvPair, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Kvs = append(out.Kvs, FromReprKvPairGenerated(ptr))
		}
	}
	return out
}

func NewReprRawCASRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawCASRequest) *RawCASRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawCASRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawCASRequest)))
	IntoReprRawCASRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawCASRequestGenerated(arena *runtime.Arena, dst *RawCASRequest, src *kvrpcpbproto.RawCASRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetValue()); length > 0 {
		dst.value.data = (*C.uint8_t)(data)
		dst.value.len = C.size_t(length)
	}
	dst.previous_not_exist = C.bool(src.GetPreviousNotExist())
	if data, length := arena.AllocBytes(src.GetPreviousValue()); length > 0 {
		dst.previous_value.data = (*C.uint8_t)(data)
		dst.previous_value.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
	dst.ttl = C.uint64_t(src.GetTtl())
	dst.delete = C.bool(src.GetDelete())
}

func FromReprRawCASRequestGenerated(src *RawCASRequest) *kvrpcpbproto.RawCASRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawCASRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Value = runtime.BytesFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	out.PreviousNotExist = bool(src.previous_not_exist)
	out.PreviousValue = runtime.BytesFrom(unsafe.Pointer(src.previous_value.data), int(src.previous_value.len))
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	out.Ttl = uint64(src.ttl)
	out.Delete = bool(src.delete)
	return out
}

func NewReprRawCASResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawCASResponse) *RawCASResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawCASResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawCASResponse)))
	IntoReprRawCASResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawCASResponseGenerated(arena *runtime.Arena, dst *RawCASResponse, src *kvrpcpbproto.RawCASResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	dst.succeed = C.bool(src.GetSucceed())
	dst.previous_not_exist = C.bool(src.GetPreviousNotExist())
	if data, length := arena.AllocBytes(src.GetPreviousValue()); length > 0 {
		dst.previous_value.data = (*C.uint8_t)(data)
		dst.previous_value.len = C.size_t(length)
	}
}

func FromReprRawCASResponseGenerated(src *RawCASResponse) *kvrpcpbproto.RawCASResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawCASResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	out.Succeed = bool(src.succeed)
	out.PreviousNotExist = bool(src.previous_not_exist)
	out.PreviousValue = runtime.BytesFrom(unsafe.Pointer(src.previous_value.data), int(src.previous_value.len))
	return out
}

func NewReprRawChecksumRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawChecksumRequest) *RawChecksumRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawChecksumRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawChecksumRequest)))
	IntoReprRawChecksumRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawChecksumRequestGenerated(arena *runtime.Arena, dst *RawChecksumRequest, src *kvrpcpbproto.RawChecksumRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.algorithm = C.int32_t(int32(src.GetAlgorithm()))
	if values := src.GetRanges(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KeyRange)(nil)))
		array := unsafe.Slice((**KeyRange)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKeyRangeGenerated(arena, value)
		}
		dst.ranges.data = (**KeyRange)(ptr)
		dst.ranges.len = C.size_t(len(values))
		dst.ranges.cap = C.size_t(len(values))
	}
}

func FromReprRawChecksumRequestGenerated(src *RawChecksumRequest) *kvrpcpbproto.RawChecksumRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawChecksumRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Algorithm = kvrpcpbproto.ChecksumAlgorithm(int32(src.algorithm))
	if src.ranges.data != nil && src.ranges.len > 0 {
		length := int(src.ranges.len)
		ptrs := unsafe.Slice((**KeyRange)(unsafe.Pointer(src.ranges.data)), length)
		out.Ranges = make([]*kvrpcpbproto.KeyRange, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Ranges = append(out.Ranges, FromReprKeyRangeGenerated(ptr))
		}
	}
	return out
}

func NewReprRawChecksumResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawChecksumResponse) *RawChecksumResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawChecksumResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawChecksumResponse)))
	IntoReprRawChecksumResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawChecksumResponseGenerated(arena *runtime.Arena, dst *RawChecksumResponse, src *kvrpcpbproto.RawChecksumResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	dst.checksum = C.uint64_t(src.GetChecksum())
	dst.total_kvs = C.uint64_t(src.GetTotalKvs())
	dst.total_bytes = C.uint64_t(src.GetTotalBytes())
}

func FromReprRawChecksumResponseGenerated(src *RawChecksumResponse) *kvrpcpbproto.RawChecksumResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawChecksumResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	out.Checksum = uint64(src.checksum)
	out.TotalKvs = uint64(src.total_kvs)
	out.TotalBytes = uint64(src.total_bytes)
	return out
}

func NewReprRawCoprocessorRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawCoprocessorRequest) *RawCoprocessorRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawCoprocessorRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawCoprocessorRequest)))
	IntoReprRawCoprocessorRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawCoprocessorRequestGenerated(arena *runtime.Arena, dst *RawCoprocessorRequest, src *kvrpcpbproto.RawCoprocessorRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocString(src.GetCoprName()); length > 0 {
		dst.copr_name.data = (*C.char)(data)
		dst.copr_name.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetCoprVersionReq()); length > 0 {
		dst.copr_version_req.data = (*C.char)(data)
		dst.copr_version_req.len = C.size_t(length)
	}
	if values := src.GetRanges(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KeyRange)(nil)))
		array := unsafe.Slice((**KeyRange)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKeyRangeGenerated(arena, value)
		}
		dst.ranges.data = (**KeyRange)(ptr)
		dst.ranges.len = C.size_t(len(values))
		dst.ranges.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocBytes(src.GetData()); length > 0 {
		dst.data.data = (*C.uint8_t)(data)
		dst.data.len = C.size_t(length)
	}
}

func FromReprRawCoprocessorRequestGenerated(src *RawCoprocessorRequest) *kvrpcpbproto.RawCoprocessorRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawCoprocessorRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.CoprName = runtime.StringFrom(unsafe.Pointer(src.copr_name.data), int(src.copr_name.len))
	out.CoprVersionReq = runtime.StringFrom(unsafe.Pointer(src.copr_version_req.data), int(src.copr_version_req.len))
	if src.ranges.data != nil && src.ranges.len > 0 {
		length := int(src.ranges.len)
		ptrs := unsafe.Slice((**KeyRange)(unsafe.Pointer(src.ranges.data)), length)
		out.Ranges = make([]*kvrpcpbproto.KeyRange, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Ranges = append(out.Ranges, FromReprKeyRangeGenerated(ptr))
		}
	}
	out.Data = runtime.BytesFrom(unsafe.Pointer(src.data.data), int(src.data.len))
	return out
}

func NewReprRawCoprocessorResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawCoprocessorResponse) *RawCoprocessorResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawCoprocessorResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawCoprocessorResponse)))
	IntoReprRawCoprocessorResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawCoprocessorResponseGenerated(arena *runtime.Arena, dst *RawCoprocessorResponse, src *kvrpcpbproto.RawCoprocessorResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetData()); length > 0 {
		dst.data.data = (*C.uint8_t)(data)
		dst.data.len = C.size_t(length)
	}
}

func FromReprRawCoprocessorResponseGenerated(src *RawCoprocessorResponse) *kvrpcpbproto.RawCoprocessorResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawCoprocessorResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	out.Data = runtime.BytesFrom(unsafe.Pointer(src.data.data), int(src.data.len))
	return out
}

func NewReprRawDeleteRangeRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawDeleteRangeRequest) *RawDeleteRangeRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawDeleteRangeRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawDeleteRangeRequest)))
	IntoReprRawDeleteRangeRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawDeleteRangeRequestGenerated(arena *runtime.Arena, dst *RawDeleteRangeRequest, src *kvrpcpbproto.RawDeleteRangeRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
}

func FromReprRawDeleteRangeRequestGenerated(src *RawDeleteRangeRequest) *kvrpcpbproto.RawDeleteRangeRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawDeleteRangeRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	return out
}

func NewReprRawDeleteRangeResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawDeleteRangeResponse) *RawDeleteRangeResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawDeleteRangeResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawDeleteRangeResponse)))
	IntoReprRawDeleteRangeResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawDeleteRangeResponseGenerated(arena *runtime.Arena, dst *RawDeleteRangeResponse, src *kvrpcpbproto.RawDeleteRangeResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprRawDeleteRangeResponseGenerated(src *RawDeleteRangeResponse) *kvrpcpbproto.RawDeleteRangeResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawDeleteRangeResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprRawDeleteRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawDeleteRequest) *RawDeleteRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawDeleteRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawDeleteRequest)))
	IntoReprRawDeleteRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawDeleteRequestGenerated(arena *runtime.Arena, dst *RawDeleteRequest, src *kvrpcpbproto.RawDeleteRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
	dst.for_cas = C.bool(src.GetForCas())
}

func FromReprRawDeleteRequestGenerated(src *RawDeleteRequest) *kvrpcpbproto.RawDeleteRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawDeleteRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	out.ForCas = bool(src.for_cas)
	return out
}

func NewReprRawDeleteResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawDeleteResponse) *RawDeleteResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawDeleteResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawDeleteResponse)))
	IntoReprRawDeleteResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawDeleteResponseGenerated(arena *runtime.Arena, dst *RawDeleteResponse, src *kvrpcpbproto.RawDeleteResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprRawDeleteResponseGenerated(src *RawDeleteResponse) *kvrpcpbproto.RawDeleteResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawDeleteResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprRawGetKeyTTLRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawGetKeyTTLRequest) *RawGetKeyTTLRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawGetKeyTTLRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawGetKeyTTLRequest)))
	IntoReprRawGetKeyTTLRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawGetKeyTTLRequestGenerated(arena *runtime.Arena, dst *RawGetKeyTTLRequest, src *kvrpcpbproto.RawGetKeyTTLRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
}

func FromReprRawGetKeyTTLRequestGenerated(src *RawGetKeyTTLRequest) *kvrpcpbproto.RawGetKeyTTLRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawGetKeyTTLRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	return out
}

func NewReprRawGetKeyTTLResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawGetKeyTTLResponse) *RawGetKeyTTLResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawGetKeyTTLResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawGetKeyTTLResponse)))
	IntoReprRawGetKeyTTLResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawGetKeyTTLResponseGenerated(arena *runtime.Arena, dst *RawGetKeyTTLResponse, src *kvrpcpbproto.RawGetKeyTTLResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	dst.ttl = C.uint64_t(src.GetTtl())
	dst.not_found = C.bool(src.GetNotFound())
}

func FromReprRawGetKeyTTLResponseGenerated(src *RawGetKeyTTLResponse) *kvrpcpbproto.RawGetKeyTTLResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawGetKeyTTLResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	out.Ttl = uint64(src.ttl)
	out.NotFound = bool(src.not_found)
	return out
}

func NewReprRawGetRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawGetRequest) *RawGetRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawGetRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawGetRequest)))
	IntoReprRawGetRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawGetRequestGenerated(arena *runtime.Arena, dst *RawGetRequest, src *kvrpcpbproto.RawGetRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
}

func FromReprRawGetRequestGenerated(src *RawGetRequest) *kvrpcpbproto.RawGetRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawGetRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	return out
}

func NewReprRawGetResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawGetResponse) *RawGetResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawGetResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawGetResponse)))
	IntoReprRawGetResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawGetResponseGenerated(arena *runtime.Arena, dst *RawGetResponse, src *kvrpcpbproto.RawGetResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetValue()); length > 0 {
		dst.value.data = (*C.uint8_t)(data)
		dst.value.len = C.size_t(length)
	}
	dst.not_found = C.bool(src.GetNotFound())
}

func FromReprRawGetResponseGenerated(src *RawGetResponse) *kvrpcpbproto.RawGetResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawGetResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	out.Value = runtime.BytesFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	out.NotFound = bool(src.not_found)
	return out
}

func NewReprRawPutRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawPutRequest) *RawPutRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawPutRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawPutRequest)))
	IntoReprRawPutRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawPutRequestGenerated(arena *runtime.Arena, dst *RawPutRequest, src *kvrpcpbproto.RawPutRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetValue()); length > 0 {
		dst.value.data = (*C.uint8_t)(data)
		dst.value.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
	dst.ttl = C.uint64_t(src.GetTtl())
	dst.for_cas = C.bool(src.GetForCas())
}

func FromReprRawPutRequestGenerated(src *RawPutRequest) *kvrpcpbproto.RawPutRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawPutRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Value = runtime.BytesFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	out.Ttl = uint64(src.ttl)
	out.ForCas = bool(src.for_cas)
	return out
}

func NewReprRawPutResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawPutResponse) *RawPutResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawPutResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawPutResponse)))
	IntoReprRawPutResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawPutResponseGenerated(arena *runtime.Arena, dst *RawPutResponse, src *kvrpcpbproto.RawPutResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprRawPutResponseGenerated(src *RawPutResponse) *kvrpcpbproto.RawPutResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawPutResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprRawScanRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawScanRequest) *RawScanRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawScanRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawScanRequest)))
	IntoReprRawScanRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawScanRequestGenerated(arena *runtime.Arena, dst *RawScanRequest, src *kvrpcpbproto.RawScanRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	dst.limit = C.uint32_t(src.GetLimit())
	dst.key_only = C.bool(src.GetKeyOnly())
	if data, length := arena.AllocString(src.GetCf()); length > 0 {
		dst.cf.data = (*C.char)(data)
		dst.cf.len = C.size_t(length)
	}
	dst.reverse = C.bool(src.GetReverse())
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
}

func FromReprRawScanRequestGenerated(src *RawScanRequest) *kvrpcpbproto.RawScanRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawScanRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.Limit = uint32(src.limit)
	out.KeyOnly = bool(src.key_only)
	out.Cf = runtime.StringFrom(unsafe.Pointer(src.cf.data), int(src.cf.len))
	out.Reverse = bool(src.reverse)
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	return out
}

func NewReprRawScanResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RawScanResponse) *RawScanResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawScanResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RawScanResponse)))
	IntoReprRawScanResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawScanResponseGenerated(arena *runtime.Arena, dst *RawScanResponse, src *kvrpcpbproto.RawScanResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetKvs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KvPair)(nil)))
		array := unsafe.Slice((**KvPair)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKvPairGenerated(arena, value)
		}
		dst.kvs.data = (**KvPair)(ptr)
		dst.kvs.len = C.size_t(len(values))
		dst.kvs.cap = C.size_t(len(values))
	}
}

func FromReprRawScanResponseGenerated(src *RawScanResponse) *kvrpcpbproto.RawScanResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RawScanResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.kvs.data != nil && src.kvs.len > 0 {
		length := int(src.kvs.len)
		ptrs := unsafe.Slice((**KvPair)(unsafe.Pointer(src.kvs.data)), length)
		out.Kvs = make([]*kvrpcpbproto.KvPair, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Kvs = append(out.Kvs, FromReprKvPairGenerated(ptr))
		}
	}
	return out
}

func NewReprReadIndexRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.ReadIndexRequest) *ReadIndexRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ReadIndexRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ReadIndexRequest)))
	IntoReprReadIndexRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprReadIndexRequestGenerated(arena *runtime.Arena, dst *ReadIndexRequest, src *kvrpcpbproto.ReadIndexRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	if values := src.GetRanges(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KeyRange)(nil)))
		array := unsafe.Slice((**KeyRange)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKeyRangeGenerated(arena, value)
		}
		dst.ranges.data = (**KeyRange)(ptr)
		dst.ranges.len = C.size_t(len(values))
		dst.ranges.cap = C.size_t(len(values))
	}
}

func FromReprReadIndexRequestGenerated(src *ReadIndexRequest) *kvrpcpbproto.ReadIndexRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ReadIndexRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartTs = uint64(src.start_ts)
	if src.ranges.data != nil && src.ranges.len > 0 {
		length := int(src.ranges.len)
		ptrs := unsafe.Slice((**KeyRange)(unsafe.Pointer(src.ranges.data)), length)
		out.Ranges = make([]*kvrpcpbproto.KeyRange, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Ranges = append(out.Ranges, FromReprKeyRangeGenerated(ptr))
		}
	}
	return out
}

func NewReprReadIndexResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.ReadIndexResponse) *ReadIndexResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ReadIndexResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ReadIndexResponse)))
	IntoReprReadIndexResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprReadIndexResponseGenerated(arena *runtime.Arena, dst *ReadIndexResponse, src *kvrpcpbproto.ReadIndexResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	dst.read_index = C.uint64_t(src.GetReadIndex())
	if value := src.GetLocked(); value != nil {
		dst.locked = NewReprLockInfoGenerated(arena, value)
	} else {
		dst.locked = nil
	}
}

func FromReprReadIndexResponseGenerated(src *ReadIndexResponse) *kvrpcpbproto.ReadIndexResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ReadIndexResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.ReadIndex = uint64(src.read_index)
	if src.locked != nil {
		out.Locked = FromReprLockInfoGenerated(src.locked)
	}
	return out
}

func NewReprReadStateGenerated(arena *runtime.Arena, src *kvrpcpbproto.ReadState) *ReadState {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ReadState)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ReadState)))
	IntoReprReadStateGenerated(arena, ptr, src)
	return ptr
}

func IntoReprReadStateGenerated(arena *runtime.Arena, dst *ReadState, src *kvrpcpbproto.ReadState) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.applied_index = C.uint64_t(src.GetAppliedIndex())
	dst.safe_ts = C.uint64_t(src.GetSafeTs())
}

func FromReprReadStateGenerated(src *ReadState) *kvrpcpbproto.ReadState {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ReadState{}
	out.AppliedIndex = uint64(src.applied_index)
	out.SafeTs = uint64(src.safe_ts)
	return out
}

func NewReprRegisterLockObserverRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RegisterLockObserverRequest) *RegisterLockObserverRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RegisterLockObserverRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RegisterLockObserverRequest)))
	IntoReprRegisterLockObserverRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRegisterLockObserverRequestGenerated(arena *runtime.Arena, dst *RegisterLockObserverRequest, src *kvrpcpbproto.RegisterLockObserverRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.max_ts = C.uint64_t(src.GetMaxTs())
}

func FromReprRegisterLockObserverRequestGenerated(src *RegisterLockObserverRequest) *kvrpcpbproto.RegisterLockObserverRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RegisterLockObserverRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.MaxTs = uint64(src.max_ts)
	return out
}

func NewReprRegisterLockObserverResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RegisterLockObserverResponse) *RegisterLockObserverResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RegisterLockObserverResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RegisterLockObserverResponse)))
	IntoReprRegisterLockObserverResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRegisterLockObserverResponseGenerated(arena *runtime.Arena, dst *RegisterLockObserverResponse, src *kvrpcpbproto.RegisterLockObserverResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprRegisterLockObserverResponseGenerated(src *RegisterLockObserverResponse) *kvrpcpbproto.RegisterLockObserverResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RegisterLockObserverResponse{}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprRemoveLockObserverRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.RemoveLockObserverRequest) *RemoveLockObserverRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RemoveLockObserverRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RemoveLockObserverRequest)))
	IntoReprRemoveLockObserverRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRemoveLockObserverRequestGenerated(arena *runtime.Arena, dst *RemoveLockObserverRequest, src *kvrpcpbproto.RemoveLockObserverRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.max_ts = C.uint64_t(src.GetMaxTs())
}

func FromReprRemoveLockObserverRequestGenerated(src *RemoveLockObserverRequest) *kvrpcpbproto.RemoveLockObserverRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RemoveLockObserverRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.MaxTs = uint64(src.max_ts)
	return out
}

func NewReprRemoveLockObserverResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.RemoveLockObserverResponse) *RemoveLockObserverResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RemoveLockObserverResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_RemoveLockObserverResponse)))
	IntoReprRemoveLockObserverResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRemoveLockObserverResponseGenerated(arena *runtime.Arena, dst *RemoveLockObserverResponse, src *kvrpcpbproto.RemoveLockObserverResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprRemoveLockObserverResponseGenerated(src *RemoveLockObserverResponse) *kvrpcpbproto.RemoveLockObserverResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.RemoveLockObserverResponse{}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprResolveLockRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.ResolveLockRequest) *ResolveLockRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ResolveLockRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ResolveLockRequest)))
	IntoReprResolveLockRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprResolveLockRequestGenerated(arena *runtime.Arena, dst *ResolveLockRequest, src *kvrpcpbproto.ResolveLockRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.start_version = C.uint64_t(src.GetStartVersion())
	dst.commit_version = C.uint64_t(src.GetCommitVersion())
	if values := src.GetTxnInfos(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*TxnInfo)(nil)))
		array := unsafe.Slice((**TxnInfo)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprTxnInfoGenerated(arena, value)
		}
		dst.txn_infos.data = (**TxnInfo)(ptr)
		dst.txn_infos.len = C.size_t(len(values))
		dst.txn_infos.cap = C.size_t(len(values))
	}
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
	dst.is_async = C.bool(src.GetIsAsync())
	dst.is_txn_file = C.bool(src.GetIsTxnFile())
}

func FromReprResolveLockRequestGenerated(src *ResolveLockRequest) *kvrpcpbproto.ResolveLockRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ResolveLockRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartVersion = uint64(src.start_version)
	out.CommitVersion = uint64(src.commit_version)
	if src.txn_infos.data != nil && src.txn_infos.len > 0 {
		length := int(src.txn_infos.len)
		ptrs := unsafe.Slice((**TxnInfo)(unsafe.Pointer(src.txn_infos.data)), length)
		out.TxnInfos = make([]*kvrpcpbproto.TxnInfo, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.TxnInfos = append(out.TxnInfos, FromReprTxnInfoGenerated(ptr))
		}
	}
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	out.IsAsync = bool(src.is_async)
	out.IsTxnFile = bool(src.is_txn_file)
	return out
}

func NewReprResolveLockResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.ResolveLockResponse) *ResolveLockResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ResolveLockResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ResolveLockResponse)))
	IntoReprResolveLockResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprResolveLockResponseGenerated(arena *runtime.Arena, dst *ResolveLockResponse, src *kvrpcpbproto.ResolveLockResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprResolveLockResponseGenerated(src *ResolveLockResponse) *kvrpcpbproto.ResolveLockResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ResolveLockResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprResourceControlContextGenerated(arena *runtime.Arena, src *kvrpcpbproto.ResourceControlContext) *ResourceControlContext {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ResourceControlContext)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ResourceControlContext)))
	IntoReprResourceControlContextGenerated(arena, ptr, src)
	return ptr
}

func IntoReprResourceControlContextGenerated(arena *runtime.Arena, dst *ResourceControlContext, src *kvrpcpbproto.ResourceControlContext) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetResourceGroupName()); length > 0 {
		dst.resource_group_name.data = (*C.char)(data)
		dst.resource_group_name.len = C.size_t(length)
	}
	if value := src.GetPenalty(); value != nil {
		dst.penalty = (*C.resource_manager_Consumption)(unsafe.Pointer(resource_managerffi.NewReprConsumptionGenerated(arena, value)))
	} else {
		dst.penalty = nil
	}
	dst.override_priority = C.uint64_t(src.GetOverridePriority())
}

func FromReprResourceControlContextGenerated(src *ResourceControlContext) *kvrpcpbproto.ResourceControlContext {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ResourceControlContext{}
	out.ResourceGroupName = runtime.StringFrom(unsafe.Pointer(src.resource_group_name.data), int(src.resource_group_name.len))
	if src.penalty != nil {
		out.Penalty = resource_managerffi.FromReprConsumptionGenerated((*resource_managerffi.Consumption)(unsafe.Pointer(src.penalty)))
	}
	out.OverridePriority = uint64(src.override_priority)
	return out
}

func NewReprScanChangesRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.ScanChangesRequest) *ScanChangesRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ScanChangesRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ScanChangesRequest)))
	IntoReprScanChangesRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprScanChangesRequestGenerated(arena *runtime.Arena, dst *ScanChangesRequest, src *kvrpcpbproto.ScanChangesRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
	dst.after_ts = C.uint64_t(src.GetAfterTs())
	dst.up_to_ts = C.uint64_t(src.GetUpToTs())
	dst.limit = C.uint32_t(src.GetLimit())
	dst.need_value = C.bool(src.GetNeedValue())
	dst.need_old_value = C.bool(src.GetNeedOldValue())
}

func FromReprScanChangesRequestGenerated(src *ScanChangesRequest) *kvrpcpbproto.ScanChangesRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ScanChangesRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	out.AfterTs = uint64(src.after_ts)
	out.UpToTs = uint64(src.up_to_ts)
	out.Limit = uint32(src.limit)
	out.NeedValue = bool(src.need_value)
	out.NeedOldValue = bool(src.need_old_value)
	return out
}

func NewReprScanChangesResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.ScanChangesResponse) *ScanChangesResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ScanChangesResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ScanChangesResponse)))
	IntoReprScanChangesResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprScanChangesResponseGenerated(arena *runtime.Arena, dst *ScanChangesResponse, src *kvrpcpbproto.ScanChangesResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetEntries(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*ChangedEntry)(nil)))
		array := unsafe.Slice((**ChangedEntry)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprChangedEntryGenerated(arena, value)
		}
		dst.entries.data = (**ChangedEntry)(ptr)
		dst.entries.len = C.size_t(len(values))
		dst.entries.cap = C.size_t(len(values))
	}
	if values := src.GetLocks(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*LockEntry)(nil)))
		array := unsafe.Slice((**LockEntry)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprLockEntryGenerated(arena, value)
		}
		dst.locks.data = (**LockEntry)(ptr)
		dst.locks.len = C.size_t(len(values))
		dst.locks.cap = C.size_t(len(values))
	}
	dst.has_more = C.bool(src.GetHasMore())
	if data, length := arena.AllocBytes(src.GetNextStartKey()); length > 0 {
		dst.next_start_key.data = (*C.uint8_t)(data)
		dst.next_start_key.len = C.size_t(length)
	}
}

func FromReprScanChangesResponseGenerated(src *ScanChangesResponse) *kvrpcpbproto.ScanChangesResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ScanChangesResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.entries.data != nil && src.entries.len > 0 {
		length := int(src.entries.len)
		ptrs := unsafe.Slice((**ChangedEntry)(unsafe.Pointer(src.entries.data)), length)
		out.Entries = make([]*kvrpcpbproto.ChangedEntry, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Entries = append(out.Entries, FromReprChangedEntryGenerated(ptr))
		}
	}
	if src.locks.data != nil && src.locks.len > 0 {
		length := int(src.locks.len)
		ptrs := unsafe.Slice((**LockEntry)(unsafe.Pointer(src.locks.data)), length)
		out.Locks = make([]*kvrpcpbproto.LockEntry, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Locks = append(out.Locks, FromReprLockEntryGenerated(ptr))
		}
	}
	out.HasMore = bool(src.has_more)
	out.NextStartKey = runtime.BytesFrom(unsafe.Pointer(src.next_start_key.data), int(src.next_start_key.len))
	return out
}

func NewReprScanDetailGenerated(arena *runtime.Arena, src *kvrpcpbproto.ScanDetail) *ScanDetail {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ScanDetail)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ScanDetail)))
	IntoReprScanDetailGenerated(arena, ptr, src)
	return ptr
}

func IntoReprScanDetailGenerated(arena *runtime.Arena, dst *ScanDetail, src *kvrpcpbproto.ScanDetail) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetWrite(); value != nil {
		dst.write = NewReprScanInfoGenerated(arena, value)
	} else {
		dst.write = nil
	}
	if value := src.GetLock(); value != nil {
		dst.lock = NewReprScanInfoGenerated(arena, value)
	} else {
		dst.lock = nil
	}
	if value := src.GetData(); value != nil {
		dst.data = NewReprScanInfoGenerated(arena, value)
	} else {
		dst.data = nil
	}
}

func FromReprScanDetailGenerated(src *ScanDetail) *kvrpcpbproto.ScanDetail {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ScanDetail{}
	if src.write != nil {
		out.Write = FromReprScanInfoGenerated(src.write)
	}
	if src.lock != nil {
		out.Lock = FromReprScanInfoGenerated(src.lock)
	}
	if src.data != nil {
		out.Data = FromReprScanInfoGenerated(src.data)
	}
	return out
}

func NewReprScanDetailV2Generated(arena *runtime.Arena, src *kvrpcpbproto.ScanDetailV2) *ScanDetailV2 {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ScanDetailV2)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ScanDetailV2)))
	IntoReprScanDetailV2Generated(arena, ptr, src)
	return ptr
}

func IntoReprScanDetailV2Generated(arena *runtime.Arena, dst *ScanDetailV2, src *kvrpcpbproto.ScanDetailV2) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.processed_versions = C.uint64_t(src.GetProcessedVersions())
	dst.total_versions = C.uint64_t(src.GetTotalVersions())
	dst.rocksdb_delete_skipped_count = C.uint64_t(src.GetRocksdbDeleteSkippedCount())
	dst.rocksdb_key_skipped_count = C.uint64_t(src.GetRocksdbKeySkippedCount())
	dst.rocksdb_block_cache_hit_count = C.uint64_t(src.GetRocksdbBlockCacheHitCount())
	dst.rocksdb_block_read_count = C.uint64_t(src.GetRocksdbBlockReadCount())
	dst.rocksdb_block_read_byte = C.uint64_t(src.GetRocksdbBlockReadByte())
	dst.processed_versions_size = C.uint64_t(src.GetProcessedVersionsSize())
	dst.rocksdb_block_read_nanos = C.uint64_t(src.GetRocksdbBlockReadNanos())
	dst.get_snapshot_nanos = C.uint64_t(src.GetGetSnapshotNanos())
	dst.read_index_propose_wait_nanos = C.uint64_t(src.GetReadIndexProposeWaitNanos())
	dst.read_index_confirm_wait_nanos = C.uint64_t(src.GetReadIndexConfirmWaitNanos())
	dst.read_pool_schedule_wait_nanos = C.uint64_t(src.GetReadPoolScheduleWaitNanos())
	dst.total_versions_size = C.uint64_t(src.GetTotalVersionsSize())
	dst.ia_cache_hit_count = C.uint64_t(src.GetIaCacheHitCount())
	dst.ia_remote_read_segment_count = C.uint64_t(src.GetIaRemoteReadSegmentCount())
	dst.ia_remote_read_segment_bytes = C.uint64_t(src.GetIaRemoteReadSegmentBytes())
	dst.ia_remote_read_segment_nanos = C.uint64_t(src.GetIaRemoteReadSegmentNanos())
}

func FromReprScanDetailV2Generated(src *ScanDetailV2) *kvrpcpbproto.ScanDetailV2 {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ScanDetailV2{}
	out.ProcessedVersions = uint64(src.processed_versions)
	out.TotalVersions = uint64(src.total_versions)
	out.RocksdbDeleteSkippedCount = uint64(src.rocksdb_delete_skipped_count)
	out.RocksdbKeySkippedCount = uint64(src.rocksdb_key_skipped_count)
	out.RocksdbBlockCacheHitCount = uint64(src.rocksdb_block_cache_hit_count)
	out.RocksdbBlockReadCount = uint64(src.rocksdb_block_read_count)
	out.RocksdbBlockReadByte = uint64(src.rocksdb_block_read_byte)
	out.ProcessedVersionsSize = uint64(src.processed_versions_size)
	out.RocksdbBlockReadNanos = uint64(src.rocksdb_block_read_nanos)
	out.GetSnapshotNanos = uint64(src.get_snapshot_nanos)
	out.ReadIndexProposeWaitNanos = uint64(src.read_index_propose_wait_nanos)
	out.ReadIndexConfirmWaitNanos = uint64(src.read_index_confirm_wait_nanos)
	out.ReadPoolScheduleWaitNanos = uint64(src.read_pool_schedule_wait_nanos)
	out.TotalVersionsSize = uint64(src.total_versions_size)
	out.IaCacheHitCount = uint64(src.ia_cache_hit_count)
	out.IaRemoteReadSegmentCount = uint64(src.ia_remote_read_segment_count)
	out.IaRemoteReadSegmentBytes = uint64(src.ia_remote_read_segment_bytes)
	out.IaRemoteReadSegmentNanos = uint64(src.ia_remote_read_segment_nanos)
	return out
}

func NewReprScanInfoGenerated(arena *runtime.Arena, src *kvrpcpbproto.ScanInfo) *ScanInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ScanInfo)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ScanInfo)))
	IntoReprScanInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprScanInfoGenerated(arena *runtime.Arena, dst *ScanInfo, src *kvrpcpbproto.ScanInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.total = C.int64_t(src.GetTotal())
	dst.processed = C.int64_t(src.GetProcessed())
	dst.read_bytes = C.int64_t(src.GetReadBytes())
}

func FromReprScanInfoGenerated(src *ScanInfo) *kvrpcpbproto.ScanInfo {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ScanInfo{}
	out.Total = int64(src.total)
	out.Processed = int64(src.processed)
	out.ReadBytes = int64(src.read_bytes)
	return out
}

func NewReprScanLockRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.ScanLockRequest) *ScanLockRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ScanLockRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ScanLockRequest)))
	IntoReprScanLockRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprScanLockRequestGenerated(arena *runtime.Arena, dst *ScanLockRequest, src *kvrpcpbproto.ScanLockRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	dst.max_version = C.uint64_t(src.GetMaxVersion())
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	dst.limit = C.uint32_t(src.GetLimit())
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
}

func FromReprScanLockRequestGenerated(src *ScanLockRequest) *kvrpcpbproto.ScanLockRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ScanLockRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.MaxVersion = uint64(src.max_version)
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.Limit = uint32(src.limit)
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	return out
}

func NewReprScanLockResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.ScanLockResponse) *ScanLockResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ScanLockResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ScanLockResponse)))
	IntoReprScanLockResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprScanLockResponseGenerated(arena *runtime.Arena, dst *ScanLockResponse, src *kvrpcpbproto.ScanLockResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if values := src.GetLocks(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*LockInfo)(nil)))
		array := unsafe.Slice((**LockInfo)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprLockInfoGenerated(arena, value)
		}
		dst.locks.data = (**LockInfo)(ptr)
		dst.locks.len = C.size_t(len(values))
		dst.locks.cap = C.size_t(len(values))
	}
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprScanLockResponseGenerated(src *ScanLockResponse) *kvrpcpbproto.ScanLockResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ScanLockResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	if src.locks.data != nil && src.locks.len > 0 {
		length := int(src.locks.len)
		ptrs := unsafe.Slice((**LockInfo)(unsafe.Pointer(src.locks.data)), length)
		out.Locks = make([]*kvrpcpbproto.LockInfo, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Locks = append(out.Locks, FromReprLockInfoGenerated(ptr))
		}
	}
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprScanRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.ScanRequest) *ScanRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ScanRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ScanRequest)))
	IntoReprScanRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprScanRequestGenerated(arena *runtime.Arena, dst *ScanRequest, src *kvrpcpbproto.ScanRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	dst.limit = C.uint32_t(src.GetLimit())
	dst.version = C.uint64_t(src.GetVersion())
	dst.key_only = C.bool(src.GetKeyOnly())
	dst.reverse = C.bool(src.GetReverse())
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
	dst.sample_step = C.uint32_t(src.GetSampleStep())
}

func FromReprScanRequestGenerated(src *ScanRequest) *kvrpcpbproto.ScanRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ScanRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.Limit = uint32(src.limit)
	out.Version = uint64(src.version)
	out.KeyOnly = bool(src.key_only)
	out.Reverse = bool(src.reverse)
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	out.SampleStep = uint32(src.sample_step)
	return out
}

func NewReprScanResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.ScanResponse) *ScanResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ScanResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_ScanResponse)))
	IntoReprScanResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprScanResponseGenerated(arena *runtime.Arena, dst *ScanResponse, src *kvrpcpbproto.ScanResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetPairs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KvPair)(nil)))
		array := unsafe.Slice((**KvPair)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKvPairGenerated(arena, value)
		}
		dst.pairs.data = (**KvPair)(ptr)
		dst.pairs.len = C.size_t(len(values))
		dst.pairs.cap = C.size_t(len(values))
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
}

func FromReprScanResponseGenerated(src *ScanResponse) *kvrpcpbproto.ScanResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.ScanResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.pairs.data != nil && src.pairs.len > 0 {
		length := int(src.pairs.len)
		ptrs := unsafe.Slice((**KvPair)(unsafe.Pointer(src.pairs.data)), length)
		out.Pairs = make([]*kvrpcpbproto.KvPair, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Pairs = append(out.Pairs, FromReprKvPairGenerated(ptr))
		}
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	return out
}

func NewReprSourceStmtGenerated(arena *runtime.Arena, src *kvrpcpbproto.SourceStmt) *SourceStmt {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*SourceStmt)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_SourceStmt)))
	IntoReprSourceStmtGenerated(arena, ptr, src)
	return ptr
}

func IntoReprSourceStmtGenerated(arena *runtime.Arena, dst *SourceStmt, src *kvrpcpbproto.SourceStmt) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	dst.connection_id = C.uint64_t(src.GetConnectionId())
	dst.stmt_id = C.uint64_t(src.GetStmtId())
	if data, length := arena.AllocString(src.GetSessionAlias()); length > 0 {
		dst.session_alias.data = (*C.char)(data)
		dst.session_alias.len = C.size_t(length)
	}
}

func FromReprSourceStmtGenerated(src *SourceStmt) *kvrpcpbproto.SourceStmt {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.SourceStmt{}
	out.StartTs = uint64(src.start_ts)
	out.ConnectionId = uint64(src.connection_id)
	out.StmtId = uint64(src.stmt_id)
	out.SessionAlias = runtime.StringFrom(unsafe.Pointer(src.session_alias.data), int(src.session_alias.len))
	return out
}

func NewReprSplitRegionRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.SplitRegionRequest) *SplitRegionRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*SplitRegionRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_SplitRegionRequest)))
	IntoReprSplitRegionRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprSplitRegionRequestGenerated(arena *runtime.Arena, dst *SplitRegionRequest, src *kvrpcpbproto.SplitRegionRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetSplitKey()); length > 0 {
		dst.split_key.data = (*C.uint8_t)(data)
		dst.split_key.len = C.size_t(length)
	}
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.split_keys), src.GetSplitKeys())
	dst.is_raw_kv = C.bool(src.GetIsRawKv())
}

func FromReprSplitRegionRequestGenerated(src *SplitRegionRequest) *kvrpcpbproto.SplitRegionRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.SplitRegionRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.SplitKey = runtime.BytesFrom(unsafe.Pointer(src.split_key.data), int(src.split_key.len))
	out.SplitKeys = runtime.CopyBytesSlice(unsafe.Pointer(&src.split_keys))
	out.IsRawKv = bool(src.is_raw_kv)
	return out
}

func NewReprSplitRegionResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.SplitRegionResponse) *SplitRegionResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*SplitRegionResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_SplitRegionResponse)))
	IntoReprSplitRegionResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprSplitRegionResponseGenerated(arena *runtime.Arena, dst *SplitRegionResponse, src *kvrpcpbproto.SplitRegionResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetLeft(); value != nil {
		dst.left = (*C.metapb_Region)(unsafe.Pointer(metapbffi.NewReprRegionGenerated(arena, value)))
	} else {
		dst.left = nil
	}
	if value := src.GetRight(); value != nil {
		dst.right = (*C.metapb_Region)(unsafe.Pointer(metapbffi.NewReprRegionGenerated(arena, value)))
	} else {
		dst.right = nil
	}
	if values := src.GetRegions(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*C.metapb_Region)(nil)))
		array := unsafe.Slice((**C.metapb_Region)(ptr), len(values))
		for i, value := range values {
			array[i] = (*C.metapb_Region)(unsafe.Pointer(metapbffi.NewReprRegionGenerated(arena, value)))
		}
		dst.regions.data = (**C.metapb_Region)(ptr)
		dst.regions.len = C.size_t(len(values))
		dst.regions.cap = C.size_t(len(values))
	}
	if values := src.GetErrors(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KeyError)(nil)))
		array := unsafe.Slice((**KeyError)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKeyErrorGenerated(arena, value)
		}
		dst.errors.data = (**KeyError)(ptr)
		dst.errors.len = C.size_t(len(values))
		dst.errors.cap = C.size_t(len(values))
	}
}

func FromReprSplitRegionResponseGenerated(src *SplitRegionResponse) *kvrpcpbproto.SplitRegionResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.SplitRegionResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.left != nil {
		out.Left = metapbffi.FromReprRegionGenerated((*metapbffi.Region)(unsafe.Pointer(src.left)))
	}
	if src.right != nil {
		out.Right = metapbffi.FromReprRegionGenerated((*metapbffi.Region)(unsafe.Pointer(src.right)))
	}
	if src.regions.data != nil && src.regions.len > 0 {
		length := int(src.regions.len)
		ptrs := unsafe.Slice((**C.metapb_Region)(unsafe.Pointer(src.regions.data)), length)
		out.Regions = make([]*metapbproto.Region, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Regions = append(out.Regions, metapbffi.FromReprRegionGenerated((*metapbffi.Region)(unsafe.Pointer(ptr))))
		}
	}
	if src.errors.data != nil && src.errors.len > 0 {
		length := int(src.errors.len)
		ptrs := unsafe.Slice((**KeyError)(unsafe.Pointer(src.errors.data)), length)
		out.Errors = make([]*kvrpcpbproto.KeyError, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Errors = append(out.Errors, FromReprKeyErrorGenerated(ptr))
		}
	}
	return out
}

func NewReprStoreBatchGetRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.StoreBatchGetRequest) *StoreBatchGetRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StoreBatchGetRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_StoreBatchGetRequest)))
	IntoReprStoreBatchGetRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStoreBatchGetRequestGenerated(arena *runtime.Arena, dst *StoreBatchGetRequest, src *kvrpcpbproto.StoreBatchGetRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if values := src.GetSubReqs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*StoreBatchGetSubRequest)(nil)))
		array := unsafe.Slice((**StoreBatchGetSubRequest)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprStoreBatchGetSubRequestGenerated(arena, value)
		}
		dst.sub_reqs.data = (**StoreBatchGetSubRequest)(ptr)
		dst.sub_reqs.len = C.size_t(len(values))
		dst.sub_reqs.cap = C.size_t(len(values))
	}
	dst.version = C.uint64_t(src.GetVersion())
}

func FromReprStoreBatchGetRequestGenerated(src *StoreBatchGetRequest) *kvrpcpbproto.StoreBatchGetRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.StoreBatchGetRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	if src.sub_reqs.data != nil && src.sub_reqs.len > 0 {
		length := int(src.sub_reqs.len)
		ptrs := unsafe.Slice((**StoreBatchGetSubRequest)(unsafe.Pointer(src.sub_reqs.data)), length)
		out.SubReqs = make([]*kvrpcpbproto.StoreBatchGetSubRequest, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.SubReqs = append(out.SubReqs, FromReprStoreBatchGetSubRequestGenerated(ptr))
		}
	}
	out.Version = uint64(src.version)
	return out
}

func NewReprStoreBatchGetResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.StoreBatchGetResponse) *StoreBatchGetResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StoreBatchGetResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_StoreBatchGetResponse)))
	IntoReprStoreBatchGetResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStoreBatchGetResponseGenerated(arena *runtime.Arena, dst *StoreBatchGetResponse, src *kvrpcpbproto.StoreBatchGetResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if values := src.GetPairs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*KvPair)(nil)))
		array := unsafe.Slice((**KvPair)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprKvPairGenerated(arena, value)
		}
		dst.pairs.data = (**KvPair)(ptr)
		dst.pairs.len = C.size_t(len(values))
		dst.pairs.cap = C.size_t(len(values))
	}
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
}

func FromReprStoreBatchGetResponseGenerated(src *StoreBatchGetResponse) *kvrpcpbproto.StoreBatchGetResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.StoreBatchGetResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.pairs.data != nil && src.pairs.len > 0 {
		length := int(src.pairs.len)
		ptrs := unsafe.Slice((**KvPair)(unsafe.Pointer(src.pairs.data)), length)
		out.Pairs = make([]*kvrpcpbproto.KvPair, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Pairs = append(out.Pairs, FromReprKvPairGenerated(ptr))
		}
	}
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	return out
}

func NewReprStoreBatchGetSubRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.StoreBatchGetSubRequest) *StoreBatchGetSubRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StoreBatchGetSubRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_StoreBatchGetSubRequest)))
	IntoReprStoreBatchGetSubRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStoreBatchGetSubRequestGenerated(arena *runtime.Arena, dst *StoreBatchGetSubRequest, src *kvrpcpbproto.StoreBatchGetSubRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
	if value := src.GetRegionEpoch(); value != nil {
		dst.region_epoch = (*C.metapb_RegionEpoch)(unsafe.Pointer(metapbffi.NewReprRegionEpochGenerated(arena, value)))
	} else {
		dst.region_epoch = nil
	}
	if value := src.GetPeer(); value != nil {
		dst.peer = (*C.metapb_Peer)(unsafe.Pointer(metapbffi.NewReprPeerGenerated(arena, value)))
	} else {
		dst.peer = nil
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
}

func FromReprStoreBatchGetSubRequestGenerated(src *StoreBatchGetSubRequest) *kvrpcpbproto.StoreBatchGetSubRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.StoreBatchGetSubRequest{}
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	if src.region_epoch != nil {
		out.RegionEpoch = metapbffi.FromReprRegionEpochGenerated((*metapbffi.RegionEpoch)(unsafe.Pointer(src.region_epoch)))
	}
	if src.peer != nil {
		out.Peer = metapbffi.FromReprPeerGenerated((*metapbffi.Peer)(unsafe.Pointer(src.peer)))
	}
	out.RegionId = uint64(src.region_id)
	return out
}

func NewReprStoreCommitRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.StoreCommitRequest) *StoreCommitRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StoreCommitRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_StoreCommitRequest)))
	IntoReprStoreCommitRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStoreCommitRequestGenerated(arena *runtime.Arena, dst *StoreCommitRequest, src *kvrpcpbproto.StoreCommitRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if values := src.GetCommitReqs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*CommitRequest)(nil)))
		array := unsafe.Slice((**CommitRequest)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprCommitRequestGenerated(arena, value)
		}
		dst.commit_reqs.data = (**CommitRequest)(ptr)
		dst.commit_reqs.len = C.size_t(len(values))
		dst.commit_reqs.cap = C.size_t(len(values))
	}
}

func FromReprStoreCommitRequestGenerated(src *StoreCommitRequest) *kvrpcpbproto.StoreCommitRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.StoreCommitRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	if src.commit_reqs.data != nil && src.commit_reqs.len > 0 {
		length := int(src.commit_reqs.len)
		ptrs := unsafe.Slice((**CommitRequest)(unsafe.Pointer(src.commit_reqs.data)), length)
		out.CommitReqs = make([]*kvrpcpbproto.CommitRequest, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.CommitReqs = append(out.CommitReqs, FromReprCommitRequestGenerated(ptr))
		}
	}
	return out
}

func NewReprStoreCommitResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.StoreCommitResponse) *StoreCommitResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StoreCommitResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_StoreCommitResponse)))
	IntoReprStoreCommitResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStoreCommitResponseGenerated(arena *runtime.Arena, dst *StoreCommitResponse, src *kvrpcpbproto.StoreCommitResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if value := src.GetPrimaryCommitResp(); value != nil {
		dst.primary_commit_resp = NewReprCommitResponseGenerated(arena, value)
	} else {
		dst.primary_commit_resp = nil
	}
	if values := src.GetCommitResps(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*CommitResponse)(nil)))
		array := unsafe.Slice((**CommitResponse)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprCommitResponseGenerated(arena, value)
		}
		dst.commit_resps.data = (**CommitResponse)(ptr)
		dst.commit_resps.len = C.size_t(len(values))
		dst.commit_resps.cap = C.size_t(len(values))
	}
}

func FromReprStoreCommitResponseGenerated(src *StoreCommitResponse) *kvrpcpbproto.StoreCommitResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.StoreCommitResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	if src.primary_commit_resp != nil {
		out.PrimaryCommitResp = FromReprCommitResponseGenerated(src.primary_commit_resp)
	}
	if src.commit_resps.data != nil && src.commit_resps.len > 0 {
		length := int(src.commit_resps.len)
		ptrs := unsafe.Slice((**CommitResponse)(unsafe.Pointer(src.commit_resps.data)), length)
		out.CommitResps = make([]*kvrpcpbproto.CommitResponse, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.CommitResps = append(out.CommitResps, FromReprCommitResponseGenerated(ptr))
		}
	}
	return out
}

func NewReprStorePrewriteRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.StorePrewriteRequest) *StorePrewriteRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StorePrewriteRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_StorePrewriteRequest)))
	IntoReprStorePrewriteRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStorePrewriteRequestGenerated(arena *runtime.Arena, dst *StorePrewriteRequest, src *kvrpcpbproto.StorePrewriteRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if values := src.GetPrewriteReqs(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*PrewriteRequest)(nil)))
		array := unsafe.Slice((**PrewriteRequest)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprPrewriteRequestGenerated(arena, value)
		}
		dst.prewrite_reqs.data = (**PrewriteRequest)(ptr)
		dst.prewrite_reqs.len = C.size_t(len(values))
		dst.prewrite_reqs.cap = C.size_t(len(values))
	}
}

func FromReprStorePrewriteRequestGenerated(src *StorePrewriteRequest) *kvrpcpbproto.StorePrewriteRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.StorePrewriteRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	if src.prewrite_reqs.data != nil && src.prewrite_reqs.len > 0 {
		length := int(src.prewrite_reqs.len)
		ptrs := unsafe.Slice((**PrewriteRequest)(unsafe.Pointer(src.prewrite_reqs.data)), length)
		out.PrewriteReqs = make([]*kvrpcpbproto.PrewriteRequest, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.PrewriteReqs = append(out.PrewriteReqs, FromReprPrewriteRequestGenerated(ptr))
		}
	}
	return out
}

func NewReprStorePrewriteResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.StorePrewriteResponse) *StorePrewriteResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StorePrewriteResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_StorePrewriteResponse)))
	IntoReprStorePrewriteResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStorePrewriteResponseGenerated(arena *runtime.Arena, dst *StorePrewriteResponse, src *kvrpcpbproto.StorePrewriteResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if values := src.GetPrewriteResps(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*PrewriteResponse)(nil)))
		array := unsafe.Slice((**PrewriteResponse)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprPrewriteResponseGenerated(arena, value)
		}
		dst.prewrite_resps.data = (**PrewriteResponse)(ptr)
		dst.prewrite_resps.len = C.size_t(len(values))
		dst.prewrite_resps.cap = C.size_t(len(values))
	}
}

func FromReprStorePrewriteResponseGenerated(src *StorePrewriteResponse) *kvrpcpbproto.StorePrewriteResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.StorePrewriteResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	if src.prewrite_resps.data != nil && src.prewrite_resps.len > 0 {
		length := int(src.prewrite_resps.len)
		ptrs := unsafe.Slice((**PrewriteResponse)(unsafe.Pointer(src.prewrite_resps.data)), length)
		out.PrewriteResps = make([]*kvrpcpbproto.PrewriteResponse, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.PrewriteResps = append(out.PrewriteResps, FromReprPrewriteResponseGenerated(ptr))
		}
	}
	return out
}

func NewReprStoreSafeTSRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.StoreSafeTSRequest) *StoreSafeTSRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StoreSafeTSRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_StoreSafeTSRequest)))
	IntoReprStoreSafeTSRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStoreSafeTSRequestGenerated(arena *runtime.Arena, dst *StoreSafeTSRequest, src *kvrpcpbproto.StoreSafeTSRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetKeyRange(); value != nil {
		dst.key_range = NewReprKeyRangeGenerated(arena, value)
	} else {
		dst.key_range = nil
	}
}

func FromReprStoreSafeTSRequestGenerated(src *StoreSafeTSRequest) *kvrpcpbproto.StoreSafeTSRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.StoreSafeTSRequest{}
	if src.key_range != nil {
		out.KeyRange = FromReprKeyRangeGenerated(src.key_range)
	}
	return out
}

func NewReprStoreSafeTSResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.StoreSafeTSResponse) *StoreSafeTSResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StoreSafeTSResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_StoreSafeTSResponse)))
	IntoReprStoreSafeTSResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStoreSafeTSResponseGenerated(arena *runtime.Arena, dst *StoreSafeTSResponse, src *kvrpcpbproto.StoreSafeTSResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.safe_ts = C.uint64_t(src.GetSafeTs())
}

func FromReprStoreSafeTSResponseGenerated(src *StoreSafeTSResponse) *kvrpcpbproto.StoreSafeTSResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.StoreSafeTSResponse{}
	out.SafeTs = uint64(src.safe_ts)
	return out
}

func NewReprTiFlashSystemTableRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.TiFlashSystemTableRequest) *TiFlashSystemTableRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TiFlashSystemTableRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_TiFlashSystemTableRequest)))
	IntoReprTiFlashSystemTableRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTiFlashSystemTableRequestGenerated(arena *runtime.Arena, dst *TiFlashSystemTableRequest, src *kvrpcpbproto.TiFlashSystemTableRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetSql()); length > 0 {
		dst.sql.data = (*C.char)(data)
		dst.sql.len = C.size_t(length)
	}
}

func FromReprTiFlashSystemTableRequestGenerated(src *TiFlashSystemTableRequest) *kvrpcpbproto.TiFlashSystemTableRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.TiFlashSystemTableRequest{}
	out.Sql = runtime.StringFrom(unsafe.Pointer(src.sql.data), int(src.sql.len))
	return out
}

func NewReprTiFlashSystemTableResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.TiFlashSystemTableResponse) *TiFlashSystemTableResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TiFlashSystemTableResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_TiFlashSystemTableResponse)))
	IntoReprTiFlashSystemTableResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTiFlashSystemTableResponseGenerated(arena *runtime.Arena, dst *TiFlashSystemTableResponse, src *kvrpcpbproto.TiFlashSystemTableResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes([]byte(src.Data)); length > 0 {
		dst.data.data = (*C.uint8_t)(data)
		dst.data.len = C.size_t(length)
	}
}

func FromReprTiFlashSystemTableResponseGenerated(src *TiFlashSystemTableResponse) *kvrpcpbproto.TiFlashSystemTableResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.TiFlashSystemTableResponse{}
	out.Data = sharedbytes.SharedBytes(runtime.BytesFrom(unsafe.Pointer(src.data.data), int(src.data.len)))
	return out
}

func NewReprTimeDetailGenerated(arena *runtime.Arena, src *kvrpcpbproto.TimeDetail) *TimeDetail {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TimeDetail)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_TimeDetail)))
	IntoReprTimeDetailGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTimeDetailGenerated(arena *runtime.Arena, dst *TimeDetail, src *kvrpcpbproto.TimeDetail) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.wait_wall_time_ms = C.uint64_t(src.GetWaitWallTimeMs())
	dst.process_wall_time_ms = C.uint64_t(src.GetProcessWallTimeMs())
	dst.kv_read_wall_time_ms = C.uint64_t(src.GetKvReadWallTimeMs())
	dst.total_rpc_wall_time_ns = C.uint64_t(src.GetTotalRpcWallTimeNs())
}

func FromReprTimeDetailGenerated(src *TimeDetail) *kvrpcpbproto.TimeDetail {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.TimeDetail{}
	out.WaitWallTimeMs = uint64(src.wait_wall_time_ms)
	out.ProcessWallTimeMs = uint64(src.process_wall_time_ms)
	out.KvReadWallTimeMs = uint64(src.kv_read_wall_time_ms)
	out.TotalRpcWallTimeNs = uint64(src.total_rpc_wall_time_ns)
	return out
}

func NewReprTimeDetailV2Generated(arena *runtime.Arena, src *kvrpcpbproto.TimeDetailV2) *TimeDetailV2 {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TimeDetailV2)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_TimeDetailV2)))
	IntoReprTimeDetailV2Generated(arena, ptr, src)
	return ptr
}

func IntoReprTimeDetailV2Generated(arena *runtime.Arena, dst *TimeDetailV2, src *kvrpcpbproto.TimeDetailV2) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.wait_wall_time_ns = C.uint64_t(src.GetWaitWallTimeNs())
	dst.process_wall_time_ns = C.uint64_t(src.GetProcessWallTimeNs())
	dst.process_suspend_wall_time_ns = C.uint64_t(src.GetProcessSuspendWallTimeNs())
	dst.kv_read_wall_time_ns = C.uint64_t(src.GetKvReadWallTimeNs())
	dst.total_rpc_wall_time_ns = C.uint64_t(src.GetTotalRpcWallTimeNs())
	dst.kv_grpc_process_time_ns = C.uint64_t(src.GetKvGrpcProcessTimeNs())
	dst.kv_grpc_wait_time_ns = C.uint64_t(src.GetKvGrpcWaitTimeNs())
}

func FromReprTimeDetailV2Generated(src *TimeDetailV2) *kvrpcpbproto.TimeDetailV2 {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.TimeDetailV2{}
	out.WaitWallTimeNs = uint64(src.wait_wall_time_ns)
	out.ProcessWallTimeNs = uint64(src.process_wall_time_ns)
	out.ProcessSuspendWallTimeNs = uint64(src.process_suspend_wall_time_ns)
	out.KvReadWallTimeNs = uint64(src.kv_read_wall_time_ns)
	out.TotalRpcWallTimeNs = uint64(src.total_rpc_wall_time_ns)
	out.KvGrpcProcessTimeNs = uint64(src.kv_grpc_process_time_ns)
	out.KvGrpcWaitTimeNs = uint64(src.kv_grpc_wait_time_ns)
	return out
}

func NewReprTxnHeartBeatRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.TxnHeartBeatRequest) *TxnHeartBeatRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TxnHeartBeatRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_TxnHeartBeatRequest)))
	IntoReprTxnHeartBeatRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTxnHeartBeatRequestGenerated(arena *runtime.Arena, dst *TxnHeartBeatRequest, src *kvrpcpbproto.TxnHeartBeatRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetPrimaryLock()); length > 0 {
		dst.primary_lock.data = (*C.uint8_t)(data)
		dst.primary_lock.len = C.size_t(length)
	}
	dst.start_version = C.uint64_t(src.GetStartVersion())
	dst.advise_lock_ttl = C.uint64_t(src.GetAdviseLockTtl())
	dst.min_commit_ts = C.uint64_t(src.GetMinCommitTs())
	dst.is_txn_file = C.bool(src.GetIsTxnFile())
}

func FromReprTxnHeartBeatRequestGenerated(src *TxnHeartBeatRequest) *kvrpcpbproto.TxnHeartBeatRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.TxnHeartBeatRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.PrimaryLock = runtime.BytesFrom(unsafe.Pointer(src.primary_lock.data), int(src.primary_lock.len))
	out.StartVersion = uint64(src.start_version)
	out.AdviseLockTtl = uint64(src.advise_lock_ttl)
	out.MinCommitTs = uint64(src.min_commit_ts)
	out.IsTxnFile = bool(src.is_txn_file)
	return out
}

func NewReprTxnHeartBeatResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.TxnHeartBeatResponse) *TxnHeartBeatResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TxnHeartBeatResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_TxnHeartBeatResponse)))
	IntoReprTxnHeartBeatResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTxnHeartBeatResponseGenerated(arena *runtime.Arena, dst *TxnHeartBeatResponse, src *kvrpcpbproto.TxnHeartBeatResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprKeyErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	dst.lock_ttl = C.uint64_t(src.GetLockTtl())
	if value := src.GetExecDetailsV2(); value != nil {
		dst.exec_details_v2 = NewReprExecDetailsV2Generated(arena, value)
	} else {
		dst.exec_details_v2 = nil
	}
}

func FromReprTxnHeartBeatResponseGenerated(src *TxnHeartBeatResponse) *kvrpcpbproto.TxnHeartBeatResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.TxnHeartBeatResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	if src.error != nil {
		out.Error = FromReprKeyErrorGenerated(src.error)
	}
	out.LockTtl = uint64(src.lock_ttl)
	if src.exec_details_v2 != nil {
		out.ExecDetailsV2 = FromReprExecDetailsV2Generated(src.exec_details_v2)
	}
	return out
}

func NewReprTxnInfoGenerated(arena *runtime.Arena, src *kvrpcpbproto.TxnInfo) *TxnInfo {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TxnInfo)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_TxnInfo)))
	IntoReprTxnInfoGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTxnInfoGenerated(arena *runtime.Arena, dst *TxnInfo, src *kvrpcpbproto.TxnInfo) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.txn = C.uint64_t(src.GetTxn())
	dst.status = C.uint64_t(src.GetStatus())
	dst.is_txn_file = C.bool(src.GetIsTxnFile())
}

func FromReprTxnInfoGenerated(src *TxnInfo) *kvrpcpbproto.TxnInfo {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.TxnInfo{}
	out.Txn = uint64(src.txn)
	out.Status = uint64(src.status)
	out.IsTxnFile = bool(src.is_txn_file)
	return out
}

func NewReprTxnLockNotFoundGenerated(arena *runtime.Arena, src *kvrpcpbproto.TxnLockNotFound) *TxnLockNotFound {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TxnLockNotFound)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_TxnLockNotFound)))
	IntoReprTxnLockNotFoundGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTxnLockNotFoundGenerated(arena *runtime.Arena, dst *TxnLockNotFound, src *kvrpcpbproto.TxnLockNotFound) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
}

func FromReprTxnLockNotFoundGenerated(src *TxnLockNotFound) *kvrpcpbproto.TxnLockNotFound {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.TxnLockNotFound{}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	return out
}

func NewReprTxnNotFoundGenerated(arena *runtime.Arena, src *kvrpcpbproto.TxnNotFound) *TxnNotFound {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TxnNotFound)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_TxnNotFound)))
	IntoReprTxnNotFoundGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTxnNotFoundGenerated(arena *runtime.Arena, dst *TxnNotFound, src *kvrpcpbproto.TxnNotFound) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	if data, length := arena.AllocBytes(src.GetPrimaryKey()); length > 0 {
		dst.primary_key.data = (*C.uint8_t)(data)
		dst.primary_key.len = C.size_t(length)
	}
}

func FromReprTxnNotFoundGenerated(src *TxnNotFound) *kvrpcpbproto.TxnNotFound {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.TxnNotFound{}
	out.StartTs = uint64(src.start_ts)
	out.PrimaryKey = runtime.BytesFrom(unsafe.Pointer(src.primary_key.data), int(src.primary_key.len))
	return out
}

func NewReprTxnStatusGenerated(arena *runtime.Arena, src *kvrpcpbproto.TxnStatus) *TxnStatus {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TxnStatus)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_TxnStatus)))
	IntoReprTxnStatusGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTxnStatusGenerated(arena *runtime.Arena, dst *TxnStatus, src *kvrpcpbproto.TxnStatus) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	dst.min_commit_ts = C.uint64_t(src.GetMinCommitTs())
	dst.commit_ts = C.uint64_t(src.GetCommitTs())
	dst.rolled_back = C.bool(src.GetRolledBack())
	dst.is_completed = C.bool(src.GetIsCompleted())
}

func FromReprTxnStatusGenerated(src *TxnStatus) *kvrpcpbproto.TxnStatus {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.TxnStatus{}
	out.StartTs = uint64(src.start_ts)
	out.MinCommitTs = uint64(src.min_commit_ts)
	out.CommitTs = uint64(src.commit_ts)
	out.RolledBack = bool(src.rolled_back)
	out.IsCompleted = bool(src.is_completed)
	return out
}

func NewReprUnsafeDestroyRangeRequestGenerated(arena *runtime.Arena, src *kvrpcpbproto.UnsafeDestroyRangeRequest) *UnsafeDestroyRangeRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*UnsafeDestroyRangeRequest)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_UnsafeDestroyRangeRequest)))
	IntoReprUnsafeDestroyRangeRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprUnsafeDestroyRangeRequestGenerated(arena *runtime.Arena, dst *UnsafeDestroyRangeRequest, src *kvrpcpbproto.UnsafeDestroyRangeRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetContext(); value != nil {
		dst.context = NewReprContextGenerated(arena, value)
	} else {
		dst.context = nil
	}
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
}

func FromReprUnsafeDestroyRangeRequestGenerated(src *UnsafeDestroyRangeRequest) *kvrpcpbproto.UnsafeDestroyRangeRequest {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.UnsafeDestroyRangeRequest{}
	if src.context != nil {
		out.Context = FromReprContextGenerated(src.context)
	}
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	return out
}

func NewReprUnsafeDestroyRangeResponseGenerated(arena *runtime.Arena, src *kvrpcpbproto.UnsafeDestroyRangeResponse) *UnsafeDestroyRangeResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*UnsafeDestroyRangeResponse)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_UnsafeDestroyRangeResponse)))
	IntoReprUnsafeDestroyRangeResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprUnsafeDestroyRangeResponseGenerated(arena *runtime.Arena, dst *UnsafeDestroyRangeResponse, src *kvrpcpbproto.UnsafeDestroyRangeResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRegionError(); value != nil {
		dst.region_error = (*C.errorpb_Error)(unsafe.Pointer(errorpbffi.NewReprErrorGenerated(arena, value)))
	} else {
		dst.region_error = nil
	}
	if data, length := arena.AllocString(src.GetError()); length > 0 {
		dst.error.data = (*C.char)(data)
		dst.error.len = C.size_t(length)
	}
}

func FromReprUnsafeDestroyRangeResponseGenerated(src *UnsafeDestroyRangeResponse) *kvrpcpbproto.UnsafeDestroyRangeResponse {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.UnsafeDestroyRangeResponse{}
	if src.region_error != nil {
		out.RegionError = errorpbffi.FromReprErrorGenerated((*errorpbffi.Error)(unsafe.Pointer(src.region_error)))
	}
	out.Error = runtime.StringFrom(unsafe.Pointer(src.error.data), int(src.error.len))
	return out
}

func NewReprWriteConflictGenerated(arena *runtime.Arena, src *kvrpcpbproto.WriteConflict) *WriteConflict {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*WriteConflict)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_WriteConflict)))
	IntoReprWriteConflictGenerated(arena, ptr, src)
	return ptr
}

func IntoReprWriteConflictGenerated(arena *runtime.Arena, dst *WriteConflict, src *kvrpcpbproto.WriteConflict) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.start_ts = C.uint64_t(src.GetStartTs())
	dst.conflict_ts = C.uint64_t(src.GetConflictTs())
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetPrimary()); length > 0 {
		dst.primary.data = (*C.uint8_t)(data)
		dst.primary.len = C.size_t(length)
	}
	dst.conflict_commit_ts = C.uint64_t(src.GetConflictCommitTs())
	dst.reason = C.int32_t(int32(src.GetReason()))
}

func FromReprWriteConflictGenerated(src *WriteConflict) *kvrpcpbproto.WriteConflict {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.WriteConflict{}
	out.StartTs = uint64(src.start_ts)
	out.ConflictTs = uint64(src.conflict_ts)
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Primary = runtime.BytesFrom(unsafe.Pointer(src.primary.data), int(src.primary.len))
	out.ConflictCommitTs = uint64(src.conflict_commit_ts)
	out.Reason = kvrpcpbproto.WriteConflict_Reason(int32(src.reason))
	return out
}

func NewReprWriteDetailGenerated(arena *runtime.Arena, src *kvrpcpbproto.WriteDetail) *WriteDetail {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*WriteDetail)(arena.AllocZero(uintptr(C.sizeof_kvrpcpb_WriteDetail)))
	IntoReprWriteDetailGenerated(arena, ptr, src)
	return ptr
}

func IntoReprWriteDetailGenerated(arena *runtime.Arena, dst *WriteDetail, src *kvrpcpbproto.WriteDetail) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.store_batch_wait_nanos = C.uint64_t(src.GetStoreBatchWaitNanos())
	dst.propose_send_wait_nanos = C.uint64_t(src.GetProposeSendWaitNanos())
	dst.persist_log_nanos = C.uint64_t(src.GetPersistLogNanos())
	dst.raft_db_write_leader_wait_nanos = C.uint64_t(src.GetRaftDbWriteLeaderWaitNanos())
	dst.raft_db_sync_log_nanos = C.uint64_t(src.GetRaftDbSyncLogNanos())
	dst.raft_db_write_memtable_nanos = C.uint64_t(src.GetRaftDbWriteMemtableNanos())
	dst.commit_log_nanos = C.uint64_t(src.GetCommitLogNanos())
	dst.apply_batch_wait_nanos = C.uint64_t(src.GetApplyBatchWaitNanos())
	dst.apply_log_nanos = C.uint64_t(src.GetApplyLogNanos())
	dst.apply_mutex_lock_nanos = C.uint64_t(src.GetApplyMutexLockNanos())
	dst.apply_write_leader_wait_nanos = C.uint64_t(src.GetApplyWriteLeaderWaitNanos())
	dst.apply_write_wal_nanos = C.uint64_t(src.GetApplyWriteWalNanos())
	dst.apply_write_memtable_nanos = C.uint64_t(src.GetApplyWriteMemtableNanos())
	dst.latch_wait_nanos = C.uint64_t(src.GetLatchWaitNanos())
	dst.process_nanos = C.uint64_t(src.GetProcessNanos())
	dst.throttle_nanos = C.uint64_t(src.GetThrottleNanos())
	dst.pessimistic_lock_wait_nanos = C.uint64_t(src.GetPessimisticLockWaitNanos())
}

func FromReprWriteDetailGenerated(src *WriteDetail) *kvrpcpbproto.WriteDetail {
	if src == nil {
		return nil
	}
	out := &kvrpcpbproto.WriteDetail{}
	out.StoreBatchWaitNanos = uint64(src.store_batch_wait_nanos)
	out.ProposeSendWaitNanos = uint64(src.propose_send_wait_nanos)
	out.PersistLogNanos = uint64(src.persist_log_nanos)
	out.RaftDbWriteLeaderWaitNanos = uint64(src.raft_db_write_leader_wait_nanos)
	out.RaftDbSyncLogNanos = uint64(src.raft_db_sync_log_nanos)
	out.RaftDbWriteMemtableNanos = uint64(src.raft_db_write_memtable_nanos)
	out.CommitLogNanos = uint64(src.commit_log_nanos)
	out.ApplyBatchWaitNanos = uint64(src.apply_batch_wait_nanos)
	out.ApplyLogNanos = uint64(src.apply_log_nanos)
	out.ApplyMutexLockNanos = uint64(src.apply_mutex_lock_nanos)
	out.ApplyWriteLeaderWaitNanos = uint64(src.apply_write_leader_wait_nanos)
	out.ApplyWriteWalNanos = uint64(src.apply_write_wal_nanos)
	out.ApplyWriteMemtableNanos = uint64(src.apply_write_memtable_nanos)
	out.LatchWaitNanos = uint64(src.latch_wait_nanos)
	out.ProcessNanos = uint64(src.process_nanos)
	out.ThrottleNanos = uint64(src.throttle_nanos)
	out.PessimisticLockWaitNanos = uint64(src.pessimistic_lock_wait_nanos)
	return out
}
