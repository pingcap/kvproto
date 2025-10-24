//go:build kvffi_gen
// +build kvffi_gen

package tracepb

/*
#cgo CFLAGS: -I../../c
#include "kvproto_abi.h"
*/
import "C"

import (
	"unsafe"
	runtime "github.com/pingcap/kvproto/ffi_out/go/runtime"
	tracepbproto "github.com/pingcap/kvproto/pkg/tracepb"
)

func NewReprNotifyCollectGenerated(arena *runtime.Arena, src *tracepbproto.NotifyCollect) *NotifyCollect {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*NotifyCollect)(arena.AllocZero(uintptr(C.sizeof_tracepb_NotifyCollect)))
	IntoReprNotifyCollectGenerated(arena, ptr, src)
	return ptr
}

func IntoReprNotifyCollectGenerated(arena *runtime.Arena, dst *NotifyCollect, src *tracepbproto.NotifyCollect) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.trace_id = C.uint64_t(src.GetTraceId())
}

func FromReprNotifyCollectGenerated(src *NotifyCollect) *tracepbproto.NotifyCollect {
	if src == nil {
		return nil
	}
	out := &tracepbproto.NotifyCollect{}
	out.TraceId = uint64(src.trace_id)
	return out
}

func NewReprPropertyGenerated(arena *runtime.Arena, src *tracepbproto.Property) *Property {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Property)(arena.AllocZero(uintptr(C.sizeof_tracepb_Property)))
	IntoReprPropertyGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPropertyGenerated(arena *runtime.Arena, dst *Property, src *tracepbproto.Property) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetKey()); length > 0 {
		dst.key.data = (*C.char)(data)
		dst.key.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetValue()); length > 0 {
		dst.value.data = (*C.char)(data)
		dst.value.len = C.size_t(length)
	}
}

func FromReprPropertyGenerated(src *Property) *tracepbproto.Property {
	if src == nil {
		return nil
	}
	out := &tracepbproto.Property{}
	out.Key = runtime.StringFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Value = runtime.StringFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	return out
}

func NewReprRemoteParentSpanGenerated(arena *runtime.Arena, src *tracepbproto.RemoteParentSpan) *RemoteParentSpan {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RemoteParentSpan)(arena.AllocZero(uintptr(C.sizeof_tracepb_RemoteParentSpan)))
	IntoReprRemoteParentSpanGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRemoteParentSpanGenerated(arena *runtime.Arena, dst *RemoteParentSpan, src *tracepbproto.RemoteParentSpan) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.trace_id = C.uint64_t(src.GetTraceId())
	dst.span_id = C.uint64_t(src.GetSpanId())
}

func FromReprRemoteParentSpanGenerated(src *RemoteParentSpan) *tracepbproto.RemoteParentSpan {
	if src == nil {
		return nil
	}
	out := &tracepbproto.RemoteParentSpan{}
	out.TraceId = uint64(src.trace_id)
	out.SpanId = uint64(src.span_id)
	return out
}

func NewReprReportGenerated(arena *runtime.Arena, src *tracepbproto.Report) *Report {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Report)(arena.AllocZero(uintptr(C.sizeof_tracepb_Report)))
	IntoReprReportGenerated(arena, ptr, src)
	return ptr
}

func IntoReprReportGenerated(arena *runtime.Arena, dst *Report, src *tracepbproto.Report) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetRemoteParentSpans(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*RemoteParentSpan)(nil)))
		array := unsafe.Slice((**RemoteParentSpan)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprRemoteParentSpanGenerated(arena, value)
		}
		dst.remote_parent_spans.data = (**RemoteParentSpan)(ptr)
		dst.remote_parent_spans.len = C.size_t(len(values))
		dst.remote_parent_spans.cap = C.size_t(len(values))
	}
	if values := src.GetSpans(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*Span)(nil)))
		array := unsafe.Slice((**Span)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprSpanGenerated(arena, value)
		}
		dst.spans.data = (**Span)(ptr)
		dst.spans.len = C.size_t(len(values))
		dst.spans.cap = C.size_t(len(values))
	}
}

func FromReprReportGenerated(src *Report) *tracepbproto.Report {
	if src == nil {
		return nil
	}
	out := &tracepbproto.Report{}
	if src.remote_parent_spans.data != nil && src.remote_parent_spans.len > 0 {
		length := int(src.remote_parent_spans.len)
		ptrs := unsafe.Slice((**RemoteParentSpan)(unsafe.Pointer(src.remote_parent_spans.data)), length)
		out.RemoteParentSpans = make([]*tracepbproto.RemoteParentSpan, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.RemoteParentSpans = append(out.RemoteParentSpans, FromReprRemoteParentSpanGenerated(ptr))
		}
	}
	if src.spans.data != nil && src.spans.len > 0 {
		length := int(src.spans.len)
		ptrs := unsafe.Slice((**Span)(unsafe.Pointer(src.spans.data)), length)
		out.Spans = make([]*tracepbproto.Span, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Spans = append(out.Spans, FromReprSpanGenerated(ptr))
		}
	}
	return out
}

func NewReprSpanGenerated(arena *runtime.Arena, src *tracepbproto.Span) *Span {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Span)(arena.AllocZero(uintptr(C.sizeof_tracepb_Span)))
	IntoReprSpanGenerated(arena, ptr, src)
	return ptr
}

func IntoReprSpanGenerated(arena *runtime.Arena, dst *Span, src *tracepbproto.Span) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.span_id = C.uint64_t(src.GetSpanId())
	dst.parent_id = C.uint64_t(src.GetParentId())
	dst.begin_unix_ns = C.uint64_t(src.GetBeginUnixNs())
	dst.duration_ns = C.uint64_t(src.GetDurationNs())
	if data, length := arena.AllocString(src.GetEvent()); length > 0 {
		dst.event.data = (*C.char)(data)
		dst.event.len = C.size_t(length)
	}
	if values := src.GetProperties(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*Property)(nil)))
		array := unsafe.Slice((**Property)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprPropertyGenerated(arena, value)
		}
		dst.properties.data = (**Property)(ptr)
		dst.properties.len = C.size_t(len(values))
		dst.properties.cap = C.size_t(len(values))
	}
}

func FromReprSpanGenerated(src *Span) *tracepbproto.Span {
	if src == nil {
		return nil
	}
	out := &tracepbproto.Span{}
	out.SpanId = uint64(src.span_id)
	out.ParentId = uint64(src.parent_id)
	out.BeginUnixNs = uint64(src.begin_unix_ns)
	out.DurationNs = uint64(src.duration_ns)
	out.Event = runtime.StringFrom(unsafe.Pointer(src.event.data), int(src.event.len))
	if src.properties.data != nil && src.properties.len > 0 {
		length := int(src.properties.len)
		ptrs := unsafe.Slice((**Property)(unsafe.Pointer(src.properties.data)), length)
		out.Properties = make([]*tracepbproto.Property, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Properties = append(out.Properties, FromReprPropertyGenerated(ptr))
		}
	}
	return out
}

func NewReprTraceContextGenerated(arena *runtime.Arena, src *tracepbproto.TraceContext) *TraceContext {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TraceContext)(arena.AllocZero(uintptr(C.sizeof_tracepb_TraceContext)))
	IntoReprTraceContextGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTraceContextGenerated(arena *runtime.Arena, dst *TraceContext, src *tracepbproto.TraceContext) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetRemoteParentSpans(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*RemoteParentSpan)(nil)))
		array := unsafe.Slice((**RemoteParentSpan)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprRemoteParentSpanGenerated(arena, value)
		}
		dst.remote_parent_spans.data = (**RemoteParentSpan)(ptr)
		dst.remote_parent_spans.len = C.size_t(len(values))
		dst.remote_parent_spans.cap = C.size_t(len(values))
	}
	dst.duration_threshold_ms = C.uint32_t(src.GetDurationThresholdMs())
}

func FromReprTraceContextGenerated(src *TraceContext) *tracepbproto.TraceContext {
	if src == nil {
		return nil
	}
	out := &tracepbproto.TraceContext{}
	if src.remote_parent_spans.data != nil && src.remote_parent_spans.len > 0 {
		length := int(src.remote_parent_spans.len)
		ptrs := unsafe.Slice((**RemoteParentSpan)(unsafe.Pointer(src.remote_parent_spans.data)), length)
		out.RemoteParentSpans = make([]*tracepbproto.RemoteParentSpan, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.RemoteParentSpans = append(out.RemoteParentSpans, FromReprRemoteParentSpanGenerated(ptr))
		}
	}
	out.DurationThresholdMs = uint32(src.duration_threshold_ms)
	return out
}

func NewReprTraceRecordGenerated(arena *runtime.Arena, src *tracepbproto.TraceRecord) *TraceRecord {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TraceRecord)(arena.AllocZero(uintptr(C.sizeof_tracepb_TraceRecord)))
	IntoReprTraceRecordGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTraceRecordGenerated(arena *runtime.Arena, dst *TraceRecord, src *tracepbproto.TraceRecord) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.record_oneof_case = 0
	switch value := src.GetRecordOneof().(type) {
	case *tracepbproto.TraceRecord_Report:
		dst.record_oneof_case = C.int32_t(1)
		if value.Report != nil {
			dst.report = NewReprReportGenerated(arena, value.Report)
		} else {
			dst.report = nil
		}
	case *tracepbproto.TraceRecord_NotifyCollect:
		dst.record_oneof_case = C.int32_t(2)
		if value.NotifyCollect != nil {
			dst.notify_collect = NewReprNotifyCollectGenerated(arena, value.NotifyCollect)
		} else {
			dst.notify_collect = nil
		}
	default:
		dst.record_oneof_case = 0
	}
}

func FromReprTraceRecordGenerated(src *TraceRecord) *tracepbproto.TraceRecord {
	if src == nil {
		return nil
	}
	out := &tracepbproto.TraceRecord{}
	switch int32(src.record_oneof_case) {
	case 1:
		out.RecordOneof = &tracepbproto.TraceRecord_Report{
			Report: FromReprReportGenerated(src.report),
		}
	case 2:
		out.RecordOneof = &tracepbproto.TraceRecord_NotifyCollect{
			NotifyCollect: FromReprNotifyCollectGenerated(src.notify_collect),
		}
	}
	return out
}

func NewReprTraceRecordRequestGenerated(arena *runtime.Arena, src *tracepbproto.TraceRecordRequest) *TraceRecordRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TraceRecordRequest)(arena.AllocZero(uintptr(C.sizeof_tracepb_TraceRecordRequest)))
	IntoReprTraceRecordRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTraceRecordRequestGenerated(arena *runtime.Arena, dst *TraceRecordRequest, src *tracepbproto.TraceRecordRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
}

func FromReprTraceRecordRequestGenerated(src *TraceRecordRequest) *tracepbproto.TraceRecordRequest {
	if src == nil {
		return nil
	}
	out := &tracepbproto.TraceRecordRequest{}
	return out
}
