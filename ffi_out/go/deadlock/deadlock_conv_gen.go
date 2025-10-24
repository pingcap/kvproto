//go:build kvffi_gen
// +build kvffi_gen

package deadlock

/*
#cgo CFLAGS: -I../../c
#include "kvproto_abi.h"
*/
import "C"

import (
	"unsafe"
	runtime "github.com/pingcap/kvproto/ffi_out/go/runtime"
	deadlockproto "github.com/pingcap/kvproto/pkg/deadlock"
)

func NewReprDeadlockRequestGenerated(arena *runtime.Arena, src *deadlockproto.DeadlockRequest) *DeadlockRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DeadlockRequest)(arena.AllocZero(uintptr(C.sizeof_deadlock_DeadlockRequest)))
	IntoReprDeadlockRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDeadlockRequestGenerated(arena *runtime.Arena, dst *DeadlockRequest, src *deadlockproto.DeadlockRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.tp = C.int32_t(int32(src.GetTp()))
	{
		value := src.GetEntry()
		dst.entry = NewReprWaitForEntryGenerated(arena, &value)
	}
	if value := src.GetReplaceLocksByKeys(); value != nil {
		dst.replace_locks_by_keys = NewReprReplaceLocksByKeysRequestGenerated(arena, value)
	} else {
		dst.replace_locks_by_keys = nil
	}
}

func FromReprDeadlockRequestGenerated(src *DeadlockRequest) *deadlockproto.DeadlockRequest {
	if src == nil {
		return nil
	}
	out := &deadlockproto.DeadlockRequest{}
	out.Tp = deadlockproto.DeadlockRequestType(int32(src.tp))
	if src.entry != nil {
		if result := FromReprWaitForEntryGenerated(src.entry); result != nil {
			out.Entry = *result
		}
	}
	if src.replace_locks_by_keys != nil {
		out.ReplaceLocksByKeys = FromReprReplaceLocksByKeysRequestGenerated(src.replace_locks_by_keys)
	}
	return out
}

func NewReprDeadlockResponseGenerated(arena *runtime.Arena, src *deadlockproto.DeadlockResponse) *DeadlockResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DeadlockResponse)(arena.AllocZero(uintptr(C.sizeof_deadlock_DeadlockResponse)))
	IntoReprDeadlockResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDeadlockResponseGenerated(arena *runtime.Arena, dst *DeadlockResponse, src *deadlockproto.DeadlockResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	{
		value := src.GetEntry()
		dst.entry = NewReprWaitForEntryGenerated(arena, &value)
	}
	dst.deadlock_key_hash = C.uint64_t(src.GetDeadlockKeyHash())
	if values := src.GetWaitChain(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*WaitForEntry)(nil)))
		array := unsafe.Slice((**WaitForEntry)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprWaitForEntryGenerated(arena, value)
		}
		dst.wait_chain.data = (**WaitForEntry)(ptr)
		dst.wait_chain.len = C.size_t(len(values))
		dst.wait_chain.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocBytes(src.GetDeadlockKey()); length > 0 {
		dst.deadlock_key.data = (*C.uint8_t)(data)
		dst.deadlock_key.len = C.size_t(length)
	}
}

func FromReprDeadlockResponseGenerated(src *DeadlockResponse) *deadlockproto.DeadlockResponse {
	if src == nil {
		return nil
	}
	out := &deadlockproto.DeadlockResponse{}
	if src.entry != nil {
		if result := FromReprWaitForEntryGenerated(src.entry); result != nil {
			out.Entry = *result
		}
	}
	out.DeadlockKeyHash = uint64(src.deadlock_key_hash)
	if src.wait_chain.data != nil && src.wait_chain.len > 0 {
		length := int(src.wait_chain.len)
		ptrs := unsafe.Slice((**WaitForEntry)(unsafe.Pointer(src.wait_chain.data)), length)
		out.WaitChain = make([]*deadlockproto.WaitForEntry, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.WaitChain = append(out.WaitChain, FromReprWaitForEntryGenerated(ptr))
		}
	}
	out.DeadlockKey = runtime.BytesFrom(unsafe.Pointer(src.deadlock_key.data), int(src.deadlock_key.len))
	return out
}

func NewReprReplaceLockByKeyItemGenerated(arena *runtime.Arena, src *deadlockproto.ReplaceLockByKeyItem) *ReplaceLockByKeyItem {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ReplaceLockByKeyItem)(arena.AllocZero(uintptr(C.sizeof_deadlock_ReplaceLockByKeyItem)))
	IntoReprReplaceLockByKeyItemGenerated(arena, ptr, src)
	return ptr
}

func IntoReprReplaceLockByKeyItemGenerated(arena *runtime.Arena, dst *ReplaceLockByKeyItem, src *deadlockproto.ReplaceLockByKeyItem) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.key_hash = C.uint64_t(src.GetKeyHash())
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	dst.old_lock_ts = C.uint64_t(src.GetOldLockTs())
	dst.new_lock_ts = C.uint64_t(src.GetNewLockTs())
}

func FromReprReplaceLockByKeyItemGenerated(src *ReplaceLockByKeyItem) *deadlockproto.ReplaceLockByKeyItem {
	if src == nil {
		return nil
	}
	out := &deadlockproto.ReplaceLockByKeyItem{}
	out.KeyHash = uint64(src.key_hash)
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.OldLockTs = uint64(src.old_lock_ts)
	out.NewLockTs = uint64(src.new_lock_ts)
	return out
}

func NewReprReplaceLocksByKeysRequestGenerated(arena *runtime.Arena, src *deadlockproto.ReplaceLocksByKeysRequest) *ReplaceLocksByKeysRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ReplaceLocksByKeysRequest)(arena.AllocZero(uintptr(C.sizeof_deadlock_ReplaceLocksByKeysRequest)))
	IntoReprReplaceLocksByKeysRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprReplaceLocksByKeysRequestGenerated(arena *runtime.Arena, dst *ReplaceLocksByKeysRequest, src *deadlockproto.ReplaceLocksByKeysRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetItems(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*ReplaceLockByKeyItem)(nil)))
		array := unsafe.Slice((**ReplaceLockByKeyItem)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprReplaceLockByKeyItemGenerated(arena, value)
		}
		dst.items.data = (**ReplaceLockByKeyItem)(ptr)
		dst.items.len = C.size_t(len(values))
		dst.items.cap = C.size_t(len(values))
	}
}

func FromReprReplaceLocksByKeysRequestGenerated(src *ReplaceLocksByKeysRequest) *deadlockproto.ReplaceLocksByKeysRequest {
	if src == nil {
		return nil
	}
	out := &deadlockproto.ReplaceLocksByKeysRequest{}
	if src.items.data != nil && src.items.len > 0 {
		length := int(src.items.len)
		ptrs := unsafe.Slice((**ReplaceLockByKeyItem)(unsafe.Pointer(src.items.data)), length)
		out.Items = make([]*deadlockproto.ReplaceLockByKeyItem, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Items = append(out.Items, FromReprReplaceLockByKeyItemGenerated(ptr))
		}
	}
	return out
}

func NewReprWaitForEntriesRequestGenerated(arena *runtime.Arena, src *deadlockproto.WaitForEntriesRequest) *WaitForEntriesRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*WaitForEntriesRequest)(arena.AllocZero(uintptr(C.sizeof_deadlock_WaitForEntriesRequest)))
	IntoReprWaitForEntriesRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprWaitForEntriesRequestGenerated(arena *runtime.Arena, dst *WaitForEntriesRequest, src *deadlockproto.WaitForEntriesRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprWaitForEntriesRequestGenerated(src *WaitForEntriesRequest) *deadlockproto.WaitForEntriesRequest {
	if src == nil {
		return nil
	}
	out := &deadlockproto.WaitForEntriesRequest{}
	return out
}

func NewReprWaitForEntriesResponseGenerated(arena *runtime.Arena, src *deadlockproto.WaitForEntriesResponse) *WaitForEntriesResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*WaitForEntriesResponse)(arena.AllocZero(uintptr(C.sizeof_deadlock_WaitForEntriesResponse)))
	IntoReprWaitForEntriesResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprWaitForEntriesResponseGenerated(arena *runtime.Arena, dst *WaitForEntriesResponse, src *deadlockproto.WaitForEntriesResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetEntries(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*WaitForEntry)(nil)))
		array := unsafe.Slice((**WaitForEntry)(ptr), len(values))
		for i := range values {
			value := values[i]
			array[i] = NewReprWaitForEntryGenerated(arena, &value)
		}
		dst.entries.data = (**WaitForEntry)(ptr)
		dst.entries.len = C.size_t(len(values))
		dst.entries.cap = C.size_t(len(values))
	}
}

func FromReprWaitForEntriesResponseGenerated(src *WaitForEntriesResponse) *deadlockproto.WaitForEntriesResponse {
	if src == nil {
		return nil
	}
	out := &deadlockproto.WaitForEntriesResponse{}
	if src.entries.data != nil && src.entries.len > 0 {
		length := int(src.entries.len)
		ptrs := unsafe.Slice((**WaitForEntry)(unsafe.Pointer(src.entries.data)), length)
		out.Entries = make([]deadlockproto.WaitForEntry, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			if result := FromReprWaitForEntryGenerated(ptr); result != nil {
				out.Entries = append(out.Entries, *result)
			}
		}
	}
	return out
}

func NewReprWaitForEntryGenerated(arena *runtime.Arena, src *deadlockproto.WaitForEntry) *WaitForEntry {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*WaitForEntry)(arena.AllocZero(uintptr(C.sizeof_deadlock_WaitForEntry)))
	IntoReprWaitForEntryGenerated(arena, ptr, src)
	return ptr
}

func IntoReprWaitForEntryGenerated(arena *runtime.Arena, dst *WaitForEntry, src *deadlockproto.WaitForEntry) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.txn = C.uint64_t(src.GetTxn())
	dst.wait_for_txn = C.uint64_t(src.GetWaitForTxn())
	dst.key_hash = C.uint64_t(src.GetKeyHash())
	if data, length := arena.AllocBytes(src.GetKey()); length > 0 {
		dst.key.data = (*C.uint8_t)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetResourceGroupTag()); length > 0 {
		dst.resource_group_tag.data = (*C.uint8_t)(data)
		dst.resource_group_tag.len = C.size_t(length)
	}
	dst.wait_time = C.uint64_t(src.GetWaitTime())
}

func FromReprWaitForEntryGenerated(src *WaitForEntry) *deadlockproto.WaitForEntry {
	if src == nil {
		return nil
	}
	out := &deadlockproto.WaitForEntry{}
	out.Txn = uint64(src.txn)
	out.WaitForTxn = uint64(src.wait_for_txn)
	out.KeyHash = uint64(src.key_hash)
	out.Key = runtime.BytesFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.ResourceGroupTag = runtime.BytesFrom(unsafe.Pointer(src.resource_group_tag.data), int(src.resource_group_tag.len))
	out.WaitTime = uint64(src.wait_time)
	return out
}
