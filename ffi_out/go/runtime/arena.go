package runtime

/*
#cgo CFLAGS: -I../../c
#include "kvproto_abi.h"
#include <stdlib.h>
#include <string.h>
*/
import "C"

import "unsafe"

// Arena owns memory allocated on the C heap during repr construction.
// The allocator is reusable: call Reset before putting it back into a sync.Pool
// to retain the backing chunks without keeping per-request allocations alive.
// Call Free when the arena is permanently discarded to release all C memory.
type Arena struct {
	blocks    []arenaBlock
	externals []unsafe.Pointer
}

type arenaBlock struct {
	base unsafe.Pointer
	size uintptr
	used uintptr
}

const (
	defaultArenaBlockSize = uintptr(64 << 10) // 64 KiB chunks amortize malloc overhead for small objects.
	arenaAlignment        = unsafe.Alignof(uintptr(0))
)

func alignUp(size, alignment uintptr) uintptr {
	if alignment == 0 {
		return size
	}
	if alignment&(alignment-1) == 0 {
		mask := alignment - 1
		if size&mask == 0 {
			return size
		}
		return (size + mask) & ^mask
	}
	rem := size % alignment
	if rem == 0 {
		return size
	}
	return size + alignment - rem
}

// NewArena creates a new arena allocator.
func NewArena() *Arena {
	return &Arena{}
}

// Alloc reserves raw memory that will be released when Free is called.
func (a *Arena) Alloc(size uintptr) unsafe.Pointer {
	if size == 0 {
		return nil
	}
	size = alignUp(size, arenaAlignment)
	block := a.ensureBlock(size)
	ptr := unsafe.Add(block.base, block.used)
	block.used += size
	return ptr
}

// AllocZero reserves zero-initialized memory.
func (a *Arena) AllocZero(size uintptr) unsafe.Pointer {
	return a.Alloc(size)
}

// AllocBytes copies a Go byte slice into arena managed memory.
func (a *Arena) AllocBytes(data []byte) (unsafe.Pointer, int) {
	if len(data) == 0 {
		return nil, 0
	}
	ptr := a.Alloc(uintptr(len(data)))
	buf := unsafe.Slice((*byte)(ptr), len(data))
	copy(buf, data)
	return ptr, len(data)
}

// AllocString copies a Go string into arena managed memory.
func (a *Arena) AllocString(s string) (unsafe.Pointer, int) {
	return a.AllocBytes([]byte(s))
}

// AllocUint64Slice copies a uint64 slice into arena managed memory.
func (a *Arena) AllocUint64Slice(values []uint64) (unsafe.Pointer, int) {
	if len(values) == 0 {
		return nil, 0
	}
	ptr := a.Alloc(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
	array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
	for i, v := range values {
		array[i] = C.uint64_t(v)
	}
	return ptr, len(values)
}

// AllocBoolSlice copies a bool slice into arena managed memory.
func (a *Arena) AllocBoolSlice(values []bool) (unsafe.Pointer, int) {
	if len(values) == 0 {
		return nil, 0
	}
	ptr := a.Alloc(uintptr(len(values)) * unsafe.Sizeof(C.bool(false)))
	array := unsafe.Slice((*C.bool)(ptr), len(values))
	for i, v := range values {
		if v {
			array[i] = C.bool(true)
		} else {
			array[i] = C.bool(false)
		}
	}
	return ptr, len(values)
}

// AllocPointerArray allocates zero-initialized storage for a slice of pointers.
func (a *Arena) AllocPointerArray(length int, elemSize uintptr) unsafe.Pointer {
	if length <= 0 || elemSize == 0 {
		return nil
	}
	return a.AllocZero(uintptr(length) * elemSize)
}

// SetStringSlice converts a Go string slice into a C slice of string views.
func SetStringSlice(arena *Arena, dst unsafe.Pointer, values []string) {
	if dst == nil {
		return
	}
	view := (*C.struct_kvproto_slice_kvproto_string_view)(dst)
	if len(values) == 0 {
		view.data = nil
		view.len = 0
		view.cap = 0
		return
	}
	elemSize := unsafe.Sizeof(C.kvproto_string_view{})
	ptr := arena.AllocZero(uintptr(len(values)) * elemSize)
	array := unsafe.Slice((*C.kvproto_string_view)(ptr), len(values))
	for i, s := range values {
		if data, length := arena.AllocString(s); length > 0 {
			array[i].data = (*C.char)(data)
			array[i].len = C.size_t(length)
		}
	}
	view.data = (*C.kvproto_string_view)(ptr)
	view.len = C.size_t(len(values))
	view.cap = C.size_t(len(values))
}

// SetBytesSlice converts a Go [][]byte into a C slice of bytes views.
func SetBytesSlice(arena *Arena, dst unsafe.Pointer, values [][]byte) {
	if dst == nil {
		return
	}
	view := (*C.struct_kvproto_slice_kvproto_bytes_view)(dst)
	if len(values) == 0 {
		view.data = nil
		view.len = 0
		view.cap = 0
		return
	}
	elemSize := unsafe.Sizeof(C.kvproto_bytes_view{})
	ptr := arena.AllocZero(uintptr(len(values)) * elemSize)
	array := unsafe.Slice((*C.kvproto_bytes_view)(ptr), len(values))
	for i, b := range values {
		if data, length := arena.AllocBytes(b); length > 0 {
			array[i].data = (*C.uint8_t)(data)
			array[i].len = C.size_t(length)
		}
	}
	view.data = (*C.kvproto_bytes_view)(ptr)
	view.len = C.size_t(len(values))
	view.cap = C.size_t(len(values))
}

// CopyStringSlice copies a repr string slice into a Go slice.
func CopyStringSlice(viewPtr unsafe.Pointer) []string {
	if viewPtr == nil {
		return nil
	}
	view := (*C.struct_kvproto_slice_kvproto_string_view)(viewPtr)
	if view.data == nil || view.len == 0 {
		return nil
	}
	length := int(view.len)
	array := unsafe.Slice((*C.kvproto_string_view)(unsafe.Pointer(view.data)), length)
	out := make([]string, 0, length)
	for _, v := range array {
		out = append(out, StringFrom(unsafe.Pointer(v.data), int(v.len)))
	}
	return out
}

// CopyBytesSlice copies a repr bytes slice into a Go [][]byte.
func CopyBytesSlice(viewPtr unsafe.Pointer) [][]byte {
	if viewPtr == nil {
		return nil
	}
	view := (*C.struct_kvproto_slice_kvproto_bytes_view)(viewPtr)
	if view.data == nil || view.len == 0 {
		return nil
	}
	length := int(view.len)
	array := unsafe.Slice((*C.kvproto_bytes_view)(unsafe.Pointer(view.data)), length)
	out := make([][]byte, 0, length)
	for _, v := range array {
		out = append(out, BytesFrom(unsafe.Pointer(v.data), int(v.len)))
	}
	return out
}

// Register tracks an external pointer for arena-managed freeing.
func (a *Arena) Register(ptr unsafe.Pointer) {
	if ptr != nil {
		a.externals = append(a.externals, ptr)
	}
}

// Reset releases per-request state while keeping allocated blocks for reuse.
// External registrations are freed because they usually originate outside the arena.
func (a *Arena) Reset() {
	if a == nil {
		return
	}
	for _, ptr := range a.externals {
		C.free(ptr)
	}
	a.externals = a.externals[:0]
	for i := range a.blocks {
		block := &a.blocks[i]
		if block.used > 0 {
			C.memset(block.base, 0, C.size_t(block.used))
			block.used = 0
		}
	}
}

// Free releases all allocations owned by the arena.
func (a *Arena) Free() {
	if a == nil {
		return
	}
	a.Reset()
	for _, block := range a.blocks {
		C.free(block.base)
	}
	a.blocks = nil
	a.externals = nil
}

// BytesFrom copies raw bytes referenced by pointer and length.
func BytesFrom(ptr unsafe.Pointer, length int) []byte {
	if ptr == nil || length == 0 {
		return nil
	}
	src := unsafe.Slice((*byte)(ptr), length)
	out := make([]byte, length)
	copy(out, src)
	return out
}

// StringFrom copies UTF-8 bytes referenced by pointer and length.
func StringFrom(ptr unsafe.Pointer, length int) string {
	if ptr == nil || length == 0 {
		return ""
	}
	src := unsafe.Slice((*byte)(ptr), length)
	out := make([]byte, length)
	copy(out, src)
	return string(out)
}

// Uint64SliceFrom copies a repr uint64 slice into a Go slice.
func Uint64SliceFrom(ptr unsafe.Pointer, length int) []uint64 {
	if ptr == nil || length == 0 {
		return nil
	}
	src := unsafe.Slice((*C.uint64_t)(ptr), length)
	out := make([]uint64, length)
	for i, v := range src {
		out[i] = uint64(v)
	}
	return out
}

func (a *Arena) ensureBlock(size uintptr) *arenaBlock {
	if n := len(a.blocks); n > 0 {
		last := &a.blocks[n-1]
		if last.size-last.used >= size {
			return last
		}
	}

	blockSize := defaultArenaBlockSize
	if size > blockSize {
		blockSize = alignUp(size, arenaAlignment)
	}
	ptr := C.calloc(1, C.size_t(blockSize))
	if ptr == nil {
		panic("ffi runtime: malloc failed")
	}
	a.blocks = append(a.blocks, arenaBlock{
		base: ptr,
		size: blockSize,
		used: 0,
	})
	return &a.blocks[len(a.blocks)-1]
}

// BoolSliceFrom copies a repr bool slice into a Go slice.
func BoolSliceFrom(ptr unsafe.Pointer, length int) []bool {
	if ptr == nil || length == 0 {
		return nil
	}
	src := unsafe.Slice((*C.bool)(ptr), length)
	out := make([]bool, length)
	for i, v := range src {
		out[i] = v != C.bool(false)
	}
	return out
}
