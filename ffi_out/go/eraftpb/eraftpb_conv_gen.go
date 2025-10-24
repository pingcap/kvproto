//go:build kvffi_gen
// +build kvffi_gen

package eraftpb

/*
#cgo CFLAGS: -I../../c
#include "kvproto_abi.h"
*/
import "C"

import (
	"unsafe"
	runtime "github.com/pingcap/kvproto/ffi_out/go/runtime"
	eraftpbproto "github.com/pingcap/kvproto/pkg/eraftpb"
)

func NewReprConfChangeGenerated(arena *runtime.Arena, src *eraftpbproto.ConfChange) *ConfChange {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ConfChange)(arena.AllocZero(uintptr(C.sizeof_eraftpb_ConfChange)))
	IntoReprConfChangeGenerated(arena, ptr, src)
	return ptr
}

func IntoReprConfChangeGenerated(arena *runtime.Arena, dst *ConfChange, src *eraftpbproto.ConfChange) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.id = C.uint64_t(src.GetId())
	dst.change_type = C.int32_t(int32(src.GetChangeType()))
	dst.node_id = C.uint64_t(src.GetNodeId())
	if data, length := arena.AllocBytes(src.GetContext()); length > 0 {
		dst.context.data = (*C.uint8_t)(data)
		dst.context.len = C.size_t(length)
	}
}

func FromReprConfChangeGenerated(src *ConfChange) *eraftpbproto.ConfChange {
	if src == nil {
		return nil
	}
	out := &eraftpbproto.ConfChange{}
	out.Id = uint64(src.id)
	out.ChangeType = eraftpbproto.ConfChangeType(int32(src.change_type))
	out.NodeId = uint64(src.node_id)
	out.Context = runtime.BytesFrom(unsafe.Pointer(src.context.data), int(src.context.len))
	return out
}

func NewReprConfChangeSingleGenerated(arena *runtime.Arena, src *eraftpbproto.ConfChangeSingle) *ConfChangeSingle {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ConfChangeSingle)(arena.AllocZero(uintptr(C.sizeof_eraftpb_ConfChangeSingle)))
	IntoReprConfChangeSingleGenerated(arena, ptr, src)
	return ptr
}

func IntoReprConfChangeSingleGenerated(arena *runtime.Arena, dst *ConfChangeSingle, src *eraftpbproto.ConfChangeSingle) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.change_type = C.int32_t(int32(src.GetChangeType()))
	dst.node_id = C.uint64_t(src.GetNodeId())
}

func FromReprConfChangeSingleGenerated(src *ConfChangeSingle) *eraftpbproto.ConfChangeSingle {
	if src == nil {
		return nil
	}
	out := &eraftpbproto.ConfChangeSingle{}
	out.ChangeType = eraftpbproto.ConfChangeType(int32(src.change_type))
	out.NodeId = uint64(src.node_id)
	return out
}

func NewReprConfChangeV2Generated(arena *runtime.Arena, src *eraftpbproto.ConfChangeV2) *ConfChangeV2 {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ConfChangeV2)(arena.AllocZero(uintptr(C.sizeof_eraftpb_ConfChangeV2)))
	IntoReprConfChangeV2Generated(arena, ptr, src)
	return ptr
}

func IntoReprConfChangeV2Generated(arena *runtime.Arena, dst *ConfChangeV2, src *eraftpbproto.ConfChangeV2) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.transition = C.int32_t(int32(src.GetTransition()))
	if values := src.GetChanges(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*ConfChangeSingle)(nil)))
		array := unsafe.Slice((**ConfChangeSingle)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprConfChangeSingleGenerated(arena, value)
		}
		dst.changes.data = (**ConfChangeSingle)(ptr)
		dst.changes.len = C.size_t(len(values))
		dst.changes.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocBytes(src.GetContext()); length > 0 {
		dst.context.data = (*C.uint8_t)(data)
		dst.context.len = C.size_t(length)
	}
}

func FromReprConfChangeV2Generated(src *ConfChangeV2) *eraftpbproto.ConfChangeV2 {
	if src == nil {
		return nil
	}
	out := &eraftpbproto.ConfChangeV2{}
	out.Transition = eraftpbproto.ConfChangeTransition(int32(src.transition))
	if src.changes.data != nil && src.changes.len > 0 {
		length := int(src.changes.len)
		ptrs := unsafe.Slice((**ConfChangeSingle)(unsafe.Pointer(src.changes.data)), length)
		out.Changes = make([]*eraftpbproto.ConfChangeSingle, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Changes = append(out.Changes, FromReprConfChangeSingleGenerated(ptr))
		}
	}
	out.Context = runtime.BytesFrom(unsafe.Pointer(src.context.data), int(src.context.len))
	return out
}

func NewReprConfStateGenerated(arena *runtime.Arena, src *eraftpbproto.ConfState) *ConfState {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ConfState)(arena.AllocZero(uintptr(C.sizeof_eraftpb_ConfState)))
	IntoReprConfStateGenerated(arena, ptr, src)
	return ptr
}

func IntoReprConfStateGenerated(arena *runtime.Arena, dst *ConfState, src *eraftpbproto.ConfState) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetVoters(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.voters.data = (*C.uint64_t)(ptr)
		dst.voters.len = C.size_t(len(values))
		dst.voters.cap = C.size_t(len(values))
	}
	if values := src.GetLearners(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.learners.data = (*C.uint64_t)(ptr)
		dst.learners.len = C.size_t(len(values))
		dst.learners.cap = C.size_t(len(values))
	}
	if values := src.GetVotersOutgoing(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.voters_outgoing.data = (*C.uint64_t)(ptr)
		dst.voters_outgoing.len = C.size_t(len(values))
		dst.voters_outgoing.cap = C.size_t(len(values))
	}
	if values := src.GetLearnersNext(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.learners_next.data = (*C.uint64_t)(ptr)
		dst.learners_next.len = C.size_t(len(values))
		dst.learners_next.cap = C.size_t(len(values))
	}
	dst.auto_leave = C.bool(src.GetAutoLeave())
}

func FromReprConfStateGenerated(src *ConfState) *eraftpbproto.ConfState {
	if src == nil {
		return nil
	}
	out := &eraftpbproto.ConfState{}
	if src.voters.data != nil && src.voters.len > 0 {
		length := int(src.voters.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.voters.data)), length)
		out.Voters = make([]uint64, 0, length)
		for _, value := range values {
			out.Voters = append(out.Voters, uint64(value))
		}
	}
	if src.learners.data != nil && src.learners.len > 0 {
		length := int(src.learners.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.learners.data)), length)
		out.Learners = make([]uint64, 0, length)
		for _, value := range values {
			out.Learners = append(out.Learners, uint64(value))
		}
	}
	if src.voters_outgoing.data != nil && src.voters_outgoing.len > 0 {
		length := int(src.voters_outgoing.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.voters_outgoing.data)), length)
		out.VotersOutgoing = make([]uint64, 0, length)
		for _, value := range values {
			out.VotersOutgoing = append(out.VotersOutgoing, uint64(value))
		}
	}
	if src.learners_next.data != nil && src.learners_next.len > 0 {
		length := int(src.learners_next.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.learners_next.data)), length)
		out.LearnersNext = make([]uint64, 0, length)
		for _, value := range values {
			out.LearnersNext = append(out.LearnersNext, uint64(value))
		}
	}
	out.AutoLeave = bool(src.auto_leave)
	return out
}

func NewReprEntryGenerated(arena *runtime.Arena, src *eraftpbproto.Entry) *Entry {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Entry)(arena.AllocZero(uintptr(C.sizeof_eraftpb_Entry)))
	IntoReprEntryGenerated(arena, ptr, src)
	return ptr
}

func IntoReprEntryGenerated(arena *runtime.Arena, dst *Entry, src *eraftpbproto.Entry) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.entry_type = C.int32_t(int32(src.GetEntryType()))
	dst.term = C.uint64_t(src.GetTerm())
	dst.index = C.uint64_t(src.GetIndex())
	if data, length := arena.AllocBytes(src.GetData()); length > 0 {
		dst.data.data = (*C.uint8_t)(data)
		dst.data.len = C.size_t(length)
	}
	dst.sync_log = C.bool(src.GetSyncLog())
	if data, length := arena.AllocBytes(src.GetContext()); length > 0 {
		dst.context.data = (*C.uint8_t)(data)
		dst.context.len = C.size_t(length)
	}
}

func FromReprEntryGenerated(src *Entry) *eraftpbproto.Entry {
	if src == nil {
		return nil
	}
	out := &eraftpbproto.Entry{}
	out.EntryType = eraftpbproto.EntryType(int32(src.entry_type))
	out.Term = uint64(src.term)
	out.Index = uint64(src.index)
	out.Data = runtime.BytesFrom(unsafe.Pointer(src.data.data), int(src.data.len))
	out.SyncLog = bool(src.sync_log)
	out.Context = runtime.BytesFrom(unsafe.Pointer(src.context.data), int(src.context.len))
	return out
}

func NewReprHardStateGenerated(arena *runtime.Arena, src *eraftpbproto.HardState) *HardState {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*HardState)(arena.AllocZero(uintptr(C.sizeof_eraftpb_HardState)))
	IntoReprHardStateGenerated(arena, ptr, src)
	return ptr
}

func IntoReprHardStateGenerated(arena *runtime.Arena, dst *HardState, src *eraftpbproto.HardState) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.term = C.uint64_t(src.GetTerm())
	dst.vote = C.uint64_t(src.GetVote())
	dst.commit = C.uint64_t(src.GetCommit())
}

func FromReprHardStateGenerated(src *HardState) *eraftpbproto.HardState {
	if src == nil {
		return nil
	}
	out := &eraftpbproto.HardState{}
	out.Term = uint64(src.term)
	out.Vote = uint64(src.vote)
	out.Commit = uint64(src.commit)
	return out
}

func NewReprMessageGenerated(arena *runtime.Arena, src *eraftpbproto.Message) *Message {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Message)(arena.AllocZero(uintptr(C.sizeof_eraftpb_Message)))
	IntoReprMessageGenerated(arena, ptr, src)
	return ptr
}

func IntoReprMessageGenerated(arena *runtime.Arena, dst *Message, src *eraftpbproto.Message) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.msg_type = C.int32_t(int32(src.GetMsgType()))
	dst.to = C.uint64_t(src.GetTo())
	dst.from = C.uint64_t(src.GetFrom())
	dst.term = C.uint64_t(src.GetTerm())
	dst.log_term = C.uint64_t(src.GetLogTerm())
	dst.index = C.uint64_t(src.GetIndex())
	if values := src.GetEntries(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*Entry)(nil)))
		array := unsafe.Slice((**Entry)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprEntryGenerated(arena, value)
		}
		dst.entries.data = (**Entry)(ptr)
		dst.entries.len = C.size_t(len(values))
		dst.entries.cap = C.size_t(len(values))
	}
	dst.commit = C.uint64_t(src.GetCommit())
	if value := src.GetSnapshot(); value != nil {
		dst.snapshot = NewReprSnapshotGenerated(arena, value)
	} else {
		dst.snapshot = nil
	}
	dst.reject = C.bool(src.GetReject())
	dst.reject_hint = C.uint64_t(src.GetRejectHint())
	if data, length := arena.AllocBytes(src.GetContext()); length > 0 {
		dst.context.data = (*C.uint8_t)(data)
		dst.context.len = C.size_t(length)
	}
	dst.request_snapshot = C.uint64_t(src.GetRequestSnapshot())
	dst.deprecated_priority = C.uint64_t(src.GetDeprecatedPriority())
	dst.priority = C.int64_t(src.GetPriority())
}

func FromReprMessageGenerated(src *Message) *eraftpbproto.Message {
	if src == nil {
		return nil
	}
	out := &eraftpbproto.Message{}
	out.MsgType = eraftpbproto.MessageType(int32(src.msg_type))
	out.To = uint64(src.to)
	out.From = uint64(src.from)
	out.Term = uint64(src.term)
	out.LogTerm = uint64(src.log_term)
	out.Index = uint64(src.index)
	if src.entries.data != nil && src.entries.len > 0 {
		length := int(src.entries.len)
		ptrs := unsafe.Slice((**Entry)(unsafe.Pointer(src.entries.data)), length)
		out.Entries = make([]*eraftpbproto.Entry, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Entries = append(out.Entries, FromReprEntryGenerated(ptr))
		}
	}
	out.Commit = uint64(src.commit)
	if src.snapshot != nil {
		out.Snapshot = FromReprSnapshotGenerated(src.snapshot)
	}
	out.Reject = bool(src.reject)
	out.RejectHint = uint64(src.reject_hint)
	out.Context = runtime.BytesFrom(unsafe.Pointer(src.context.data), int(src.context.len))
	out.RequestSnapshot = uint64(src.request_snapshot)
	out.DeprecatedPriority = uint64(src.deprecated_priority)
	out.Priority = int64(src.priority)
	return out
}

func NewReprSnapshotGenerated(arena *runtime.Arena, src *eraftpbproto.Snapshot) *Snapshot {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Snapshot)(arena.AllocZero(uintptr(C.sizeof_eraftpb_Snapshot)))
	IntoReprSnapshotGenerated(arena, ptr, src)
	return ptr
}

func IntoReprSnapshotGenerated(arena *runtime.Arena, dst *Snapshot, src *eraftpbproto.Snapshot) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocBytes(src.GetData()); length > 0 {
		dst.data.data = (*C.uint8_t)(data)
		dst.data.len = C.size_t(length)
	}
	if value := src.GetMetadata(); value != nil {
		dst.metadata = NewReprSnapshotMetadataGenerated(arena, value)
	} else {
		dst.metadata = nil
	}
}

func FromReprSnapshotGenerated(src *Snapshot) *eraftpbproto.Snapshot {
	if src == nil {
		return nil
	}
	out := &eraftpbproto.Snapshot{}
	out.Data = runtime.BytesFrom(unsafe.Pointer(src.data.data), int(src.data.len))
	if src.metadata != nil {
		out.Metadata = FromReprSnapshotMetadataGenerated(src.metadata)
	}
	return out
}

func NewReprSnapshotMetadataGenerated(arena *runtime.Arena, src *eraftpbproto.SnapshotMetadata) *SnapshotMetadata {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*SnapshotMetadata)(arena.AllocZero(uintptr(C.sizeof_eraftpb_SnapshotMetadata)))
	IntoReprSnapshotMetadataGenerated(arena, ptr, src)
	return ptr
}

func IntoReprSnapshotMetadataGenerated(arena *runtime.Arena, dst *SnapshotMetadata, src *eraftpbproto.SnapshotMetadata) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetConfState(); value != nil {
		dst.conf_state = NewReprConfStateGenerated(arena, value)
	} else {
		dst.conf_state = nil
	}
	dst.index = C.uint64_t(src.GetIndex())
	dst.term = C.uint64_t(src.GetTerm())
}

func FromReprSnapshotMetadataGenerated(src *SnapshotMetadata) *eraftpbproto.SnapshotMetadata {
	if src == nil {
		return nil
	}
	out := &eraftpbproto.SnapshotMetadata{}
	if src.conf_state != nil {
		out.ConfState = FromReprConfStateGenerated(src.conf_state)
	}
	out.Index = uint64(src.index)
	out.Term = uint64(src.term)
	return out
}
