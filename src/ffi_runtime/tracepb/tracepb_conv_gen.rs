//! Auto-generated conversions (feature `kvffi_gen`).
#![cfg(feature = "kvffi_gen")]
#![allow(unused_imports, unused_variables, unused_mut, non_snake_case)]

use std::ptr;
use std::os::raw::c_char;

use protobuf::Message;
use protobuf::ProtobufEnum;
use crate::ffi_runtime::arena::{Arena, string_from};
use crate::ffi_runtime::abi::{KvprotoSliceTracepbPropertyPtr, KvprotoSliceTracepbRemoteParentSpanPtr, KvprotoSliceTracepbSpanPtr, KvprotoStringView, TracepbNotifyCollect, TracepbProperty, TracepbRemoteParentSpan, TracepbReport, TracepbSpan, TracepbTraceContext, TracepbTraceRecord, TracepbTraceRecordRequest};
use crate::tracepb as pb;

pub fn notify_collect_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::NotifyCollect) -> &'a mut TracepbNotifyCollect {
    let mut repr = TracepbNotifyCollect {
        trace_id: Default::default(),
    };
    repr.trace_id = src.get_trace_id();
    arena.alloc_struct(repr)
}

pub fn notify_collect_from_repr_generated(src: *const TracepbNotifyCollect) -> Option<pb::NotifyCollect> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::NotifyCollect::new();
    out.set_trace_id(repr.trace_id);
    Some(out)
}

pub fn property_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Property) -> &'a mut TracepbProperty {
    let mut repr = TracepbProperty {
        key: KvprotoStringView { data: ptr::null(), len: 0 },
        value: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_key().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_key());
        repr.key.data = ptr as *const c_char;
        repr.key.len = len;
    }
    if !src.get_value().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_value());
        repr.value.data = ptr as *const c_char;
        repr.value.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn property_from_repr_generated(src: *const TracepbProperty) -> Option<pb::Property> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Property::new();
    out.set_key(string_from(repr.key.data as *const u8, repr.key.len));
    out.set_value(string_from(repr.value.data as *const u8, repr.value.len));
    Some(out)
}

pub fn remote_parent_span_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RemoteParentSpan) -> &'a mut TracepbRemoteParentSpan {
    let mut repr = TracepbRemoteParentSpan {
        trace_id: Default::default(),
        span_id: Default::default(),
    };
    repr.trace_id = src.get_trace_id();
    repr.span_id = src.get_span_id();
    arena.alloc_struct(repr)
}

pub fn remote_parent_span_from_repr_generated(src: *const TracepbRemoteParentSpan) -> Option<pb::RemoteParentSpan> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RemoteParentSpan::new();
    out.set_trace_id(repr.trace_id);
    out.set_span_id(repr.span_id);
    Some(out)
}

pub fn report_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Report) -> &'a mut TracepbReport {
    let mut repr = TracepbReport {
        remote_parent_spans: KvprotoSliceTracepbRemoteParentSpanPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        spans: KvprotoSliceTracepbSpanPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_remote_parent_spans();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut TracepbRemoteParentSpan> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(remote_parent_span_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.remote_parent_spans.data = ptr;
                repr.remote_parent_spans.len = len;
                repr.remote_parent_spans.cap = len;
            }
        }
    }
    {
        let values = src.get_spans();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut TracepbSpan> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(span_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.spans.data = ptr;
                repr.spans.len = len;
                repr.spans.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn report_from_repr_generated(src: *const TracepbReport) -> Option<pb::Report> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Report::new();
    if !repr.remote_parent_spans.data.is_null() && repr.remote_parent_spans.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.remote_parent_spans.data, repr.remote_parent_spans.len) };
        let mut values: Vec<pb::RemoteParentSpan> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = remote_parent_span_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_remote_parent_spans(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.spans.data.is_null() && repr.spans.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.spans.data, repr.spans.len) };
        let mut values: Vec<pb::Span> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = span_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_spans(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn span_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Span) -> &'a mut TracepbSpan {
    let mut repr = TracepbSpan {
        span_id: Default::default(),
        parent_id: Default::default(),
        begin_unix_ns: Default::default(),
        duration_ns: Default::default(),
        event: KvprotoStringView { data: ptr::null(), len: 0 },
        properties: KvprotoSliceTracepbPropertyPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    repr.span_id = src.get_span_id();
    repr.parent_id = src.get_parent_id();
    repr.begin_unix_ns = src.get_begin_unix_ns();
    repr.duration_ns = src.get_duration_ns();
    if !src.get_event().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_event());
        repr.event.data = ptr as *const c_char;
        repr.event.len = len;
    }
    {
        let values = src.get_properties();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut TracepbProperty> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(property_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.properties.data = ptr;
                repr.properties.len = len;
                repr.properties.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn span_from_repr_generated(src: *const TracepbSpan) -> Option<pb::Span> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Span::new();
    out.set_span_id(repr.span_id);
    out.set_parent_id(repr.parent_id);
    out.set_begin_unix_ns(repr.begin_unix_ns);
    out.set_duration_ns(repr.duration_ns);
    out.set_event(string_from(repr.event.data as *const u8, repr.event.len));
    if !repr.properties.data.is_null() && repr.properties.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.properties.data, repr.properties.len) };
        let mut values: Vec<pb::Property> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = property_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_properties(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn trace_context_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TraceContext) -> &'a mut TracepbTraceContext {
    let mut repr = TracepbTraceContext {
        remote_parent_spans: KvprotoSliceTracepbRemoteParentSpanPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        duration_threshold_ms: Default::default(),
    };
    {
        let values = src.get_remote_parent_spans();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut TracepbRemoteParentSpan> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(remote_parent_span_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.remote_parent_spans.data = ptr;
                repr.remote_parent_spans.len = len;
                repr.remote_parent_spans.cap = len;
            }
        }
    }
    repr.duration_threshold_ms = src.get_duration_threshold_ms();
    arena.alloc_struct(repr)
}

pub fn trace_context_from_repr_generated(src: *const TracepbTraceContext) -> Option<pb::TraceContext> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TraceContext::new();
    if !repr.remote_parent_spans.data.is_null() && repr.remote_parent_spans.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.remote_parent_spans.data, repr.remote_parent_spans.len) };
        let mut values: Vec<pb::RemoteParentSpan> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = remote_parent_span_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_remote_parent_spans(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_duration_threshold_ms(repr.duration_threshold_ms);
    Some(out)
}

pub fn trace_record_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TraceRecord) -> &'a mut TracepbTraceRecord {
    let mut repr = TracepbTraceRecord {
        record_oneof_case: Default::default(),
        report: ptr::null_mut(),
        notify_collect: ptr::null_mut(),
    };
    arena.alloc_struct(repr)
}

pub fn trace_record_from_repr_generated(src: *const TracepbTraceRecord) -> Option<pb::TraceRecord> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TraceRecord::new();
    Some(out)
}

pub fn trace_record_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TraceRecordRequest) -> &'a mut TracepbTraceRecordRequest {
    let mut repr = TracepbTraceRecordRequest {
    };
    arena.alloc_struct(repr)
}

pub fn trace_record_request_from_repr_generated(src: *const TracepbTraceRecordRequest) -> Option<pb::TraceRecordRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TraceRecordRequest::new();
    Some(out)
}

