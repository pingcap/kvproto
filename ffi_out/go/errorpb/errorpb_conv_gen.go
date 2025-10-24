//go:build kvffi_gen
// +build kvffi_gen

package errorpb

/*
#cgo CFLAGS: -I../../c
#include "kvproto_abi.h"
*/
import "C"

import (
	"unsafe"
	metapbffi "github.com/pingcap/kvproto/ffi_out/go/metapb"
	runtime "github.com/pingcap/kvproto/ffi_out/go/runtime"
	errorpbproto "github.com/pingcap/kvproto/pkg/errorpb"
	metapbproto "github.com/pingcap/kvproto/pkg/metapb"
)

func NewReprBucketVersionNotMatchGenerated(arena *runtime.Arena, src *errorpbproto.BucketVersionNotMatch) *BucketVersionNotMatch {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BucketVersionNotMatch)(arena.AllocZero(uintptr(C.sizeof_errorpb_BucketVersionNotMatch)))
	IntoReprBucketVersionNotMatchGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBucketVersionNotMatchGenerated(arena *runtime.Arena, dst *BucketVersionNotMatch, src *errorpbproto.BucketVersionNotMatch) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.version = C.uint64_t(src.GetVersion())
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
}

func FromReprBucketVersionNotMatchGenerated(src *BucketVersionNotMatch) *errorpbproto.BucketVersionNotMatch {
	if src == nil {
		return nil
	}
	out := &errorpbproto.BucketVersionNotMatch{}
	out.Version = uint64(src.version)
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	return out
}

func NewReprDataIsNotReadyGenerated(arena *runtime.Arena, src *errorpbproto.DataIsNotReady) *DataIsNotReady {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DataIsNotReady)(arena.AllocZero(uintptr(C.sizeof_errorpb_DataIsNotReady)))
	IntoReprDataIsNotReadyGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDataIsNotReadyGenerated(arena *runtime.Arena, dst *DataIsNotReady, src *errorpbproto.DataIsNotReady) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
	dst.peer_id = C.uint64_t(src.GetPeerId())
	dst.safe_ts = C.uint64_t(src.GetSafeTs())
}

func FromReprDataIsNotReadyGenerated(src *DataIsNotReady) *errorpbproto.DataIsNotReady {
	if src == nil {
		return nil
	}
	out := &errorpbproto.DataIsNotReady{}
	out.RegionId = uint64(src.region_id)
	out.PeerId = uint64(src.peer_id)
	out.SafeTs = uint64(src.safe_ts)
	return out
}

func NewReprDiskFullGenerated(arena *runtime.Arena, src *errorpbproto.DiskFull) *DiskFull {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DiskFull)(arena.AllocZero(uintptr(C.sizeof_errorpb_DiskFull)))
	IntoReprDiskFullGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDiskFullGenerated(arena *runtime.Arena, dst *DiskFull, src *errorpbproto.DiskFull) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetStoreId(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.store_id.data = (*C.uint64_t)(ptr)
		dst.store_id.len = C.size_t(len(values))
		dst.store_id.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocString(src.GetReason()); length > 0 {
		dst.reason.data = (*C.char)(data)
		dst.reason.len = C.size_t(length)
	}
}

func FromReprDiskFullGenerated(src *DiskFull) *errorpbproto.DiskFull {
	if src == nil {
		return nil
	}
	out := &errorpbproto.DiskFull{}
	if src.store_id.data != nil && src.store_id.len > 0 {
		length := int(src.store_id.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.store_id.data)), length)
		out.StoreId = make([]uint64, 0, length)
		for _, value := range values {
			out.StoreId = append(out.StoreId, uint64(value))
		}
	}
	out.Reason = runtime.StringFrom(unsafe.Pointer(src.reason.data), int(src.reason.len))
	return out
}

func NewReprEpochNotMatchGenerated(arena *runtime.Arena, src *errorpbproto.EpochNotMatch) *EpochNotMatch {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*EpochNotMatch)(arena.AllocZero(uintptr(C.sizeof_errorpb_EpochNotMatch)))
	IntoReprEpochNotMatchGenerated(arena, ptr, src)
	return ptr
}

func IntoReprEpochNotMatchGenerated(arena *runtime.Arena, dst *EpochNotMatch, src *errorpbproto.EpochNotMatch) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetCurrentRegions(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*C.metapb_Region)(nil)))
		array := unsafe.Slice((**C.metapb_Region)(ptr), len(values))
		for i, value := range values {
			array[i] = (*C.metapb_Region)(unsafe.Pointer(metapbffi.NewReprRegionGenerated(arena, value)))
		}
		dst.current_regions.data = (**C.metapb_Region)(ptr)
		dst.current_regions.len = C.size_t(len(values))
		dst.current_regions.cap = C.size_t(len(values))
	}
}

func FromReprEpochNotMatchGenerated(src *EpochNotMatch) *errorpbproto.EpochNotMatch {
	if src == nil {
		return nil
	}
	out := &errorpbproto.EpochNotMatch{}
	if src.current_regions.data != nil && src.current_regions.len > 0 {
		length := int(src.current_regions.len)
		ptrs := unsafe.Slice((**C.metapb_Region)(unsafe.Pointer(src.current_regions.data)), length)
		out.CurrentRegions = make([]*metapbproto.Region, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.CurrentRegions = append(out.CurrentRegions, metapbffi.FromReprRegionGenerated((*metapbffi.Region)(unsafe.Pointer(ptr))))
		}
	}
	return out
}

func NewReprErrorGenerated(arena *runtime.Arena, src *errorpbproto.Error) *Error {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Error)(arena.AllocZero(uintptr(C.sizeof_errorpb_Error)))
	IntoReprErrorGenerated(arena, ptr, src)
	return ptr
}

func IntoReprErrorGenerated(arena *runtime.Arena, dst *Error, src *errorpbproto.Error) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetMessage()); length > 0 {
		dst.message.data = (*C.char)(data)
		dst.message.len = C.size_t(length)
	}
	if value := src.GetNotLeader(); value != nil {
		dst.not_leader = NewReprNotLeaderGenerated(arena, value)
	} else {
		dst.not_leader = nil
	}
	if value := src.GetRegionNotFound(); value != nil {
		dst.region_not_found = NewReprRegionNotFoundGenerated(arena, value)
	} else {
		dst.region_not_found = nil
	}
	if value := src.GetKeyNotInRegion(); value != nil {
		dst.key_not_in_region = NewReprKeyNotInRegionGenerated(arena, value)
	} else {
		dst.key_not_in_region = nil
	}
	if value := src.GetEpochNotMatch(); value != nil {
		dst.epoch_not_match = NewReprEpochNotMatchGenerated(arena, value)
	} else {
		dst.epoch_not_match = nil
	}
	if value := src.GetServerIsBusy(); value != nil {
		dst.server_is_busy = NewReprServerIsBusyGenerated(arena, value)
	} else {
		dst.server_is_busy = nil
	}
	if value := src.GetStaleCommand(); value != nil {
		dst.stale_command = NewReprStaleCommandGenerated(arena, value)
	} else {
		dst.stale_command = nil
	}
	if value := src.GetStoreNotMatch(); value != nil {
		dst.store_not_match = NewReprStoreNotMatchGenerated(arena, value)
	} else {
		dst.store_not_match = nil
	}
	if value := src.GetRaftEntryTooLarge(); value != nil {
		dst.raft_entry_too_large = NewReprRaftEntryTooLargeGenerated(arena, value)
	} else {
		dst.raft_entry_too_large = nil
	}
	if value := src.GetMaxTimestampNotSynced(); value != nil {
		dst.max_timestamp_not_synced = NewReprMaxTimestampNotSyncedGenerated(arena, value)
	} else {
		dst.max_timestamp_not_synced = nil
	}
	if value := src.GetReadIndexNotReady(); value != nil {
		dst.read_index_not_ready = NewReprReadIndexNotReadyGenerated(arena, value)
	} else {
		dst.read_index_not_ready = nil
	}
	if value := src.GetProposalInMergingMode(); value != nil {
		dst.proposal_in_merging_mode = NewReprProposalInMergingModeGenerated(arena, value)
	} else {
		dst.proposal_in_merging_mode = nil
	}
	if value := src.GetDataIsNotReady(); value != nil {
		dst.data_is_not_ready = NewReprDataIsNotReadyGenerated(arena, value)
	} else {
		dst.data_is_not_ready = nil
	}
	if value := src.GetRegionNotInitialized(); value != nil {
		dst.region_not_initialized = NewReprRegionNotInitializedGenerated(arena, value)
	} else {
		dst.region_not_initialized = nil
	}
	if value := src.GetDiskFull(); value != nil {
		dst.disk_full = NewReprDiskFullGenerated(arena, value)
	} else {
		dst.disk_full = nil
	}
	if value := src.GetRecoveryInProgress(); value != nil {
		dst.RecoveryInProgress = NewReprRecoveryInProgressGenerated(arena, value)
	} else {
		dst.RecoveryInProgress = nil
	}
	if value := src.GetFlashbackInProgress(); value != nil {
		dst.FlashbackInProgress = NewReprFlashbackInProgressGenerated(arena, value)
	} else {
		dst.FlashbackInProgress = nil
	}
	if value := src.GetFlashbackNotPrepared(); value != nil {
		dst.FlashbackNotPrepared = NewReprFlashbackNotPreparedGenerated(arena, value)
	} else {
		dst.FlashbackNotPrepared = nil
	}
	if value := src.GetIsWitness(); value != nil {
		dst.is_witness = NewReprIsWitnessGenerated(arena, value)
	} else {
		dst.is_witness = nil
	}
	if value := src.GetMismatchPeerId(); value != nil {
		dst.mismatch_peer_id = NewReprMismatchPeerIdGenerated(arena, value)
	} else {
		dst.mismatch_peer_id = nil
	}
	if value := src.GetBucketVersionNotMatch(); value != nil {
		dst.bucket_version_not_match = NewReprBucketVersionNotMatchGenerated(arena, value)
	} else {
		dst.bucket_version_not_match = nil
	}
	if value := src.GetUndeterminedResult(); value != nil {
		dst.undetermined_result = NewReprUndeterminedResultGenerated(arena, value)
	} else {
		dst.undetermined_result = nil
	}
}

func FromReprErrorGenerated(src *Error) *errorpbproto.Error {
	if src == nil {
		return nil
	}
	out := &errorpbproto.Error{}
	out.Message = runtime.StringFrom(unsafe.Pointer(src.message.data), int(src.message.len))
	if src.not_leader != nil {
		out.NotLeader = FromReprNotLeaderGenerated(src.not_leader)
	}
	if src.region_not_found != nil {
		out.RegionNotFound = FromReprRegionNotFoundGenerated(src.region_not_found)
	}
	if src.key_not_in_region != nil {
		out.KeyNotInRegion = FromReprKeyNotInRegionGenerated(src.key_not_in_region)
	}
	if src.epoch_not_match != nil {
		out.EpochNotMatch = FromReprEpochNotMatchGenerated(src.epoch_not_match)
	}
	if src.server_is_busy != nil {
		out.ServerIsBusy = FromReprServerIsBusyGenerated(src.server_is_busy)
	}
	if src.stale_command != nil {
		out.StaleCommand = FromReprStaleCommandGenerated(src.stale_command)
	}
	if src.store_not_match != nil {
		out.StoreNotMatch = FromReprStoreNotMatchGenerated(src.store_not_match)
	}
	if src.raft_entry_too_large != nil {
		out.RaftEntryTooLarge = FromReprRaftEntryTooLargeGenerated(src.raft_entry_too_large)
	}
	if src.max_timestamp_not_synced != nil {
		out.MaxTimestampNotSynced = FromReprMaxTimestampNotSyncedGenerated(src.max_timestamp_not_synced)
	}
	if src.read_index_not_ready != nil {
		out.ReadIndexNotReady = FromReprReadIndexNotReadyGenerated(src.read_index_not_ready)
	}
	if src.proposal_in_merging_mode != nil {
		out.ProposalInMergingMode = FromReprProposalInMergingModeGenerated(src.proposal_in_merging_mode)
	}
	if src.data_is_not_ready != nil {
		out.DataIsNotReady = FromReprDataIsNotReadyGenerated(src.data_is_not_ready)
	}
	if src.region_not_initialized != nil {
		out.RegionNotInitialized = FromReprRegionNotInitializedGenerated(src.region_not_initialized)
	}
	if src.disk_full != nil {
		out.DiskFull = FromReprDiskFullGenerated(src.disk_full)
	}
	if src.RecoveryInProgress != nil {
		out.RecoveryInProgress = FromReprRecoveryInProgressGenerated(src.RecoveryInProgress)
	}
	if src.FlashbackInProgress != nil {
		out.FlashbackInProgress = FromReprFlashbackInProgressGenerated(src.FlashbackInProgress)
	}
	if src.FlashbackNotPrepared != nil {
		out.FlashbackNotPrepared = FromReprFlashbackNotPreparedGenerated(src.FlashbackNotPrepared)
	}
	if src.is_witness != nil {
		out.IsWitness = FromReprIsWitnessGenerated(src.is_witness)
	}
	if src.mismatch_peer_id != nil {
		out.MismatchPeerId = FromReprMismatchPeerIdGenerated(src.mismatch_peer_id)
	}
	if src.bucket_version_not_match != nil {
		out.BucketVersionNotMatch = FromReprBucketVersionNotMatchGenerated(src.bucket_version_not_match)
	}
	if src.undetermined_result != nil {
		out.UndeterminedResult = FromReprUndeterminedResultGenerated(src.undetermined_result)
	}
	return out
}

func NewReprFlashbackInProgressGenerated(arena *runtime.Arena, src *errorpbproto.FlashbackInProgress) *FlashbackInProgress {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FlashbackInProgress)(arena.AllocZero(uintptr(C.sizeof_errorpb_FlashbackInProgress)))
	IntoReprFlashbackInProgressGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFlashbackInProgressGenerated(arena *runtime.Arena, dst *FlashbackInProgress, src *errorpbproto.FlashbackInProgress) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
	dst.flashback_start_ts = C.uint64_t(src.GetFlashbackStartTs())
}

func FromReprFlashbackInProgressGenerated(src *FlashbackInProgress) *errorpbproto.FlashbackInProgress {
	if src == nil {
		return nil
	}
	out := &errorpbproto.FlashbackInProgress{}
	out.RegionId = uint64(src.region_id)
	out.FlashbackStartTs = uint64(src.flashback_start_ts)
	return out
}

func NewReprFlashbackNotPreparedGenerated(arena *runtime.Arena, src *errorpbproto.FlashbackNotPrepared) *FlashbackNotPrepared {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*FlashbackNotPrepared)(arena.AllocZero(uintptr(C.sizeof_errorpb_FlashbackNotPrepared)))
	IntoReprFlashbackNotPreparedGenerated(arena, ptr, src)
	return ptr
}

func IntoReprFlashbackNotPreparedGenerated(arena *runtime.Arena, dst *FlashbackNotPrepared, src *errorpbproto.FlashbackNotPrepared) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
}

func FromReprFlashbackNotPreparedGenerated(src *FlashbackNotPrepared) *errorpbproto.FlashbackNotPrepared {
	if src == nil {
		return nil
	}
	out := &errorpbproto.FlashbackNotPrepared{}
	out.RegionId = uint64(src.region_id)
	return out
}

func NewReprIsWitnessGenerated(arena *runtime.Arena, src *errorpbproto.IsWitness) *IsWitness {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*IsWitness)(arena.AllocZero(uintptr(C.sizeof_errorpb_IsWitness)))
	IntoReprIsWitnessGenerated(arena, ptr, src)
	return ptr
}

func IntoReprIsWitnessGenerated(arena *runtime.Arena, dst *IsWitness, src *errorpbproto.IsWitness) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
}

func FromReprIsWitnessGenerated(src *IsWitness) *errorpbproto.IsWitness {
	if src == nil {
		return nil
	}
	out := &errorpbproto.IsWitness{}
	out.RegionId = uint64(src.region_id)
	return out
}

func NewReprKeyNotInRegionGenerated(arena *runtime.Arena, src *errorpbproto.KeyNotInRegion) *KeyNotInRegion {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*KeyNotInRegion)(arena.AllocZero(uintptr(C.sizeof_errorpb_KeyNotInRegion)))
	IntoReprKeyNotInRegionGenerated(arena, ptr, src)
	return ptr
}

func IntoReprKeyNotInRegionGenerated(arena *runtime.Arena, dst *KeyNotInRegion, src *errorpbproto.KeyNotInRegion) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
}

func FromReprKeyNotInRegionGenerated(src *KeyNotInRegion) *errorpbproto.KeyNotInRegion {
	if src == nil {
		return nil
	}
	out := &errorpbproto.KeyNotInRegion{}
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.RegionId = uint64(src.region_id)
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	return out
}

func NewReprMaxTimestampNotSyncedGenerated(arena *runtime.Arena, src *errorpbproto.MaxTimestampNotSynced) *MaxTimestampNotSynced {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MaxTimestampNotSynced)(arena.AllocZero(uintptr(C.sizeof_errorpb_MaxTimestampNotSynced)))
	IntoReprMaxTimestampNotSyncedGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMaxTimestampNotSyncedGenerated(arena *runtime.Arena, dst *MaxTimestampNotSynced, src *errorpbproto.MaxTimestampNotSynced) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprMaxTimestampNotSyncedGenerated(src *MaxTimestampNotSynced) *errorpbproto.MaxTimestampNotSynced {
	if src == nil {
		return nil
	}
	out := &errorpbproto.MaxTimestampNotSynced{}
	return out
}

func NewReprMismatchPeerIdGenerated(arena *runtime.Arena, src *errorpbproto.MismatchPeerId) *MismatchPeerId {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*MismatchPeerId)(arena.AllocZero(uintptr(C.sizeof_errorpb_MismatchPeerId)))
	IntoReprMismatchPeerIdGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMismatchPeerIdGenerated(arena *runtime.Arena, dst *MismatchPeerId, src *errorpbproto.MismatchPeerId) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.request_peer_id = C.uint64_t(src.GetRequestPeerId())
	dst.store_peer_id = C.uint64_t(src.GetStorePeerId())
}

func FromReprMismatchPeerIdGenerated(src *MismatchPeerId) *errorpbproto.MismatchPeerId {
	if src == nil {
		return nil
	}
	out := &errorpbproto.MismatchPeerId{}
	out.RequestPeerId = uint64(src.request_peer_id)
	out.StorePeerId = uint64(src.store_peer_id)
	return out
}

func NewReprNotLeaderGenerated(arena *runtime.Arena, src *errorpbproto.NotLeader) *NotLeader {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*NotLeader)(arena.AllocZero(uintptr(C.sizeof_errorpb_NotLeader)))
	IntoReprNotLeaderGenerated(arena, ptr, src)
	return ptr
}

func IntoReprNotLeaderGenerated(arena *runtime.Arena, dst *NotLeader, src *errorpbproto.NotLeader) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
	if value := src.GetLeader(); value != nil {
		dst.leader = (*C.metapb_Peer)(unsafe.Pointer(metapbffi.NewReprPeerGenerated(arena, value)))
	} else {
		dst.leader = nil
	}
}

func FromReprNotLeaderGenerated(src *NotLeader) *errorpbproto.NotLeader {
	if src == nil {
		return nil
	}
	out := &errorpbproto.NotLeader{}
	out.RegionId = uint64(src.region_id)
	if src.leader != nil {
		out.Leader = metapbffi.FromReprPeerGenerated((*metapbffi.Peer)(unsafe.Pointer(src.leader)))
	}
	return out
}

func NewReprProposalInMergingModeGenerated(arena *runtime.Arena, src *errorpbproto.ProposalInMergingMode) *ProposalInMergingMode {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ProposalInMergingMode)(arena.AllocZero(uintptr(C.sizeof_errorpb_ProposalInMergingMode)))
	IntoReprProposalInMergingModeGenerated(arena, ptr, src)
	return ptr
}

func IntoReprProposalInMergingModeGenerated(arena *runtime.Arena, dst *ProposalInMergingMode, src *errorpbproto.ProposalInMergingMode) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
}

func FromReprProposalInMergingModeGenerated(src *ProposalInMergingMode) *errorpbproto.ProposalInMergingMode {
	if src == nil {
		return nil
	}
	out := &errorpbproto.ProposalInMergingMode{}
	out.RegionId = uint64(src.region_id)
	return out
}

func NewReprRaftEntryTooLargeGenerated(arena *runtime.Arena, src *errorpbproto.RaftEntryTooLarge) *RaftEntryTooLarge {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RaftEntryTooLarge)(arena.AllocZero(uintptr(C.sizeof_errorpb_RaftEntryTooLarge)))
	IntoReprRaftEntryTooLargeGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRaftEntryTooLargeGenerated(arena *runtime.Arena, dst *RaftEntryTooLarge, src *errorpbproto.RaftEntryTooLarge) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
	dst.entry_size = C.uint64_t(src.GetEntrySize())
}

func FromReprRaftEntryTooLargeGenerated(src *RaftEntryTooLarge) *errorpbproto.RaftEntryTooLarge {
	if src == nil {
		return nil
	}
	out := &errorpbproto.RaftEntryTooLarge{}
	out.RegionId = uint64(src.region_id)
	out.EntrySize = uint64(src.entry_size)
	return out
}

func NewReprReadIndexNotReadyGenerated(arena *runtime.Arena, src *errorpbproto.ReadIndexNotReady) *ReadIndexNotReady {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ReadIndexNotReady)(arena.AllocZero(uintptr(C.sizeof_errorpb_ReadIndexNotReady)))
	IntoReprReadIndexNotReadyGenerated(arena, ptr, src)
	return ptr
}

func IntoReprReadIndexNotReadyGenerated(arena *runtime.Arena, dst *ReadIndexNotReady, src *errorpbproto.ReadIndexNotReady) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetReason()); length > 0 {
		dst.reason.data = (*C.char)(data)
		dst.reason.len = C.size_t(length)
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
}

func FromReprReadIndexNotReadyGenerated(src *ReadIndexNotReady) *errorpbproto.ReadIndexNotReady {
	if src == nil {
		return nil
	}
	out := &errorpbproto.ReadIndexNotReady{}
	out.Reason = runtime.StringFrom(unsafe.Pointer(src.reason.data), int(src.reason.len))
	out.RegionId = uint64(src.region_id)
	return out
}

func NewReprRecoveryInProgressGenerated(arena *runtime.Arena, src *errorpbproto.RecoveryInProgress) *RecoveryInProgress {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RecoveryInProgress)(arena.AllocZero(uintptr(C.sizeof_errorpb_RecoveryInProgress)))
	IntoReprRecoveryInProgressGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRecoveryInProgressGenerated(arena *runtime.Arena, dst *RecoveryInProgress, src *errorpbproto.RecoveryInProgress) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
}

func FromReprRecoveryInProgressGenerated(src *RecoveryInProgress) *errorpbproto.RecoveryInProgress {
	if src == nil {
		return nil
	}
	out := &errorpbproto.RecoveryInProgress{}
	out.RegionId = uint64(src.region_id)
	return out
}

func NewReprRegionNotFoundGenerated(arena *runtime.Arena, src *errorpbproto.RegionNotFound) *RegionNotFound {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RegionNotFound)(arena.AllocZero(uintptr(C.sizeof_errorpb_RegionNotFound)))
	IntoReprRegionNotFoundGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRegionNotFoundGenerated(arena *runtime.Arena, dst *RegionNotFound, src *errorpbproto.RegionNotFound) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
}

func FromReprRegionNotFoundGenerated(src *RegionNotFound) *errorpbproto.RegionNotFound {
	if src == nil {
		return nil
	}
	out := &errorpbproto.RegionNotFound{}
	out.RegionId = uint64(src.region_id)
	return out
}

func NewReprRegionNotInitializedGenerated(arena *runtime.Arena, src *errorpbproto.RegionNotInitialized) *RegionNotInitialized {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RegionNotInitialized)(arena.AllocZero(uintptr(C.sizeof_errorpb_RegionNotInitialized)))
	IntoReprRegionNotInitializedGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRegionNotInitializedGenerated(arena *runtime.Arena, dst *RegionNotInitialized, src *errorpbproto.RegionNotInitialized) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
}

func FromReprRegionNotInitializedGenerated(src *RegionNotInitialized) *errorpbproto.RegionNotInitialized {
	if src == nil {
		return nil
	}
	out := &errorpbproto.RegionNotInitialized{}
	out.RegionId = uint64(src.region_id)
	return out
}

func NewReprServerIsBusyGenerated(arena *runtime.Arena, src *errorpbproto.ServerIsBusy) *ServerIsBusy {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ServerIsBusy)(arena.AllocZero(uintptr(C.sizeof_errorpb_ServerIsBusy)))
	IntoReprServerIsBusyGenerated(arena, ptr, src)
	return ptr
}

func IntoReprServerIsBusyGenerated(arena *runtime.Arena, dst *ServerIsBusy, src *errorpbproto.ServerIsBusy) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetReason()); length > 0 {
		dst.reason.data = (*C.char)(data)
		dst.reason.len = C.size_t(length)
	}
	dst.backoff_ms = C.uint64_t(src.GetBackoffMs())
	dst.estimated_wait_ms = C.uint32_t(src.GetEstimatedWaitMs())
	dst.applied_index = C.uint64_t(src.GetAppliedIndex())
}

func FromReprServerIsBusyGenerated(src *ServerIsBusy) *errorpbproto.ServerIsBusy {
	if src == nil {
		return nil
	}
	out := &errorpbproto.ServerIsBusy{}
	out.Reason = runtime.StringFrom(unsafe.Pointer(src.reason.data), int(src.reason.len))
	out.BackoffMs = uint64(src.backoff_ms)
	out.EstimatedWaitMs = uint32(src.estimated_wait_ms)
	out.AppliedIndex = uint64(src.applied_index)
	return out
}

func NewReprStaleCommandGenerated(arena *runtime.Arena, src *errorpbproto.StaleCommand) *StaleCommand {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StaleCommand)(arena.AllocZero(uintptr(C.sizeof_errorpb_StaleCommand)))
	IntoReprStaleCommandGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStaleCommandGenerated(arena *runtime.Arena, dst *StaleCommand, src *errorpbproto.StaleCommand) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprStaleCommandGenerated(src *StaleCommand) *errorpbproto.StaleCommand {
	if src == nil {
		return nil
	}
	out := &errorpbproto.StaleCommand{}
	return out
}

func NewReprStoreNotMatchGenerated(arena *runtime.Arena, src *errorpbproto.StoreNotMatch) *StoreNotMatch {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StoreNotMatch)(arena.AllocZero(uintptr(C.sizeof_errorpb_StoreNotMatch)))
	IntoReprStoreNotMatchGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStoreNotMatchGenerated(arena *runtime.Arena, dst *StoreNotMatch, src *errorpbproto.StoreNotMatch) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.request_store_id = C.uint64_t(src.GetRequestStoreId())
	dst.actual_store_id = C.uint64_t(src.GetActualStoreId())
}

func FromReprStoreNotMatchGenerated(src *StoreNotMatch) *errorpbproto.StoreNotMatch {
	if src == nil {
		return nil
	}
	out := &errorpbproto.StoreNotMatch{}
	out.RequestStoreId = uint64(src.request_store_id)
	out.ActualStoreId = uint64(src.actual_store_id)
	return out
}

func NewReprUndeterminedResultGenerated(arena *runtime.Arena, src *errorpbproto.UndeterminedResult) *UndeterminedResult {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*UndeterminedResult)(arena.AllocZero(uintptr(C.sizeof_errorpb_UndeterminedResult)))
	IntoReprUndeterminedResultGenerated(arena, ptr, src)
	return ptr
}

func IntoReprUndeterminedResultGenerated(arena *runtime.Arena, dst *UndeterminedResult, src *errorpbproto.UndeterminedResult) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetMessage()); length > 0 {
		dst.message.data = (*C.char)(data)
		dst.message.len = C.size_t(length)
	}
}

func FromReprUndeterminedResultGenerated(src *UndeterminedResult) *errorpbproto.UndeterminedResult {
	if src == nil {
		return nil
	}
	out := &errorpbproto.UndeterminedResult{}
	out.Message = runtime.StringFrom(unsafe.Pointer(src.message.data), int(src.message.len))
	return out
}
