//! Auto-generated conversions (feature `kvffi_gen`).
#![cfg(feature = "kvffi_gen")]

use std::ptr;
use std::os::raw::c_char;

use protobuf::ProtobufEnum;
use crate::ffi_runtime::arena::{Arena, string_from};
use crate::ffi_runtime::abi::{KvprotoSliceKvprotoStringView, KvprotoSliceResourceManagerGrantedRUTokenBucketPtr, KvprotoSliceResourceManagerGrantedRawResourceTokenBucketPtr, KvprotoSliceResourceManagerRawResourceItemPtr, KvprotoSliceResourceManagerRequestUnitItemPtr, KvprotoSliceResourceManagerResourceGroupPtr, KvprotoSliceResourceManagerTokenBucketRequestPtr, KvprotoSliceResourceManagerTokenBucketResponsePtr, KvprotoStringView, ResourceManagerBackgroundSettings, ResourceManagerConsumption, ResourceManagerDeleteResourceGroupRequest, ResourceManagerDeleteResourceGroupResponse, ResourceManagerError, ResourceManagerGetResourceGroupRequest, ResourceManagerGetResourceGroupResponse, ResourceManagerGrantedRUTokenBucket, ResourceManagerGrantedRawResourceTokenBucket, ResourceManagerGroupRawResourceSettings, ResourceManagerGroupRequestUnitSettings, ResourceManagerListResourceGroupsRequest, ResourceManagerListResourceGroupsResponse, ResourceManagerParticipant, ResourceManagerPutResourceGroupRequest, ResourceManagerPutResourceGroupResponse, ResourceManagerRawResourceItem, ResourceManagerRequestUnitItem, ResourceManagerResourceGroup, ResourceManagerRunawayRule, ResourceManagerRunawaySettings, ResourceManagerRunawayWatch, ResourceManagerTokenBucket, ResourceManagerTokenBucketRequest, ResourceManagerTokenBucketRequestRequestRU, ResourceManagerTokenBucketRequestRequestRawResource, ResourceManagerTokenBucketResponse, ResourceManagerTokenBucketsRequest, ResourceManagerTokenBucketsResponse, ResourceManagerTokenLimitSettings};
use crate::resource_manager as pb;

pub fn background_settings_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BackgroundSettings) -> &'a mut ResourceManagerBackgroundSettings {
    let mut repr = ResourceManagerBackgroundSettings {
        job_types: KvprotoSliceKvprotoStringView { data: ptr::null_mut(), len: 0, cap: 0 },
        utilization_limit: Default::default(),
    };
    {
        let values = src.get_job_types();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_string(value);
                views.push(KvprotoStringView { data: ptr as *const c_char, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.job_types.data = ptr;
                repr.job_types.len = len;
                repr.job_types.cap = len;
            }
        }
    }
    repr.utilization_limit = src.get_utilization_limit();
    arena.alloc_struct(repr)
}

pub fn background_settings_from_repr_generated(src: *const ResourceManagerBackgroundSettings) -> Option<pb::BackgroundSettings> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BackgroundSettings::new();
    if !repr.job_types.data.is_null() && repr.job_types.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.job_types.data, repr.job_types.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(string_from(view.data as *const u8, view.len));
        }
        out.set_job_types(::protobuf::RepeatedField::from_vec(values));
    }
    out.set_utilization_limit(repr.utilization_limit);
    Some(out)
}

pub fn consumption_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Consumption) -> &'a mut ResourceManagerConsumption {
    let mut repr = ResourceManagerConsumption {
        r_r_u: Default::default(),
        w_r_u: Default::default(),
        read_bytes: Default::default(),
        write_bytes: Default::default(),
        total_cpu_time_ms: Default::default(),
        sql_layer_cpu_time_ms: Default::default(),
        kv_read_rpc_count: Default::default(),
        kv_write_rpc_count: Default::default(),
    };
    repr.r_r_u = src.get_r_r_u();
    repr.w_r_u = src.get_w_r_u();
    repr.read_bytes = src.get_read_bytes();
    repr.write_bytes = src.get_write_bytes();
    repr.total_cpu_time_ms = src.get_total_cpu_time_ms();
    repr.sql_layer_cpu_time_ms = src.get_sql_layer_cpu_time_ms();
    repr.kv_read_rpc_count = src.get_kv_read_rpc_count();
    repr.kv_write_rpc_count = src.get_kv_write_rpc_count();
    arena.alloc_struct(repr)
}

pub fn consumption_from_repr_generated(src: *const ResourceManagerConsumption) -> Option<pb::Consumption> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Consumption::new();
    out.set_r_r_u(repr.r_r_u);
    out.set_w_r_u(repr.w_r_u);
    out.set_read_bytes(repr.read_bytes);
    out.set_write_bytes(repr.write_bytes);
    out.set_total_cpu_time_ms(repr.total_cpu_time_ms);
    out.set_sql_layer_cpu_time_ms(repr.sql_layer_cpu_time_ms);
    out.set_kv_read_rpc_count(repr.kv_read_rpc_count);
    out.set_kv_write_rpc_count(repr.kv_write_rpc_count);
    Some(out)
}

pub fn delete_resource_group_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::DeleteResourceGroupRequest) -> &'a mut ResourceManagerDeleteResourceGroupRequest {
    let mut repr = ResourceManagerDeleteResourceGroupRequest {
        resource_group_name: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_resource_group_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_resource_group_name());
        repr.resource_group_name.data = ptr as *const c_char;
        repr.resource_group_name.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn delete_resource_group_request_from_repr_generated(src: *const ResourceManagerDeleteResourceGroupRequest) -> Option<pb::DeleteResourceGroupRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::DeleteResourceGroupRequest::new();
    out.set_resource_group_name(string_from(repr.resource_group_name.data as *const u8, repr.resource_group_name.len));
    Some(out)
}

pub fn delete_resource_group_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::DeleteResourceGroupResponse) -> &'a mut ResourceManagerDeleteResourceGroupResponse {
    let mut repr = ResourceManagerDeleteResourceGroupResponse {
        error: ptr::null_mut(),
        body: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_error() {
        repr.error = error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    if !src.get_body().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_body());
        repr.body.data = ptr as *const c_char;
        repr.body.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn delete_resource_group_response_from_repr_generated(src: *const ResourceManagerDeleteResourceGroupResponse) -> Option<pb::DeleteResourceGroupResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::DeleteResourceGroupResponse::new();
    if !repr.error.is_null() {
        if let Some(value) = error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    out.set_body(string_from(repr.body.data as *const u8, repr.body.len));
    Some(out)
}

pub fn error_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Error) -> &'a mut ResourceManagerError {
    let mut repr = ResourceManagerError {
        message: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if !src.get_message().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_message());
        repr.message.data = ptr as *const c_char;
        repr.message.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn error_from_repr_generated(src: *const ResourceManagerError) -> Option<pb::Error> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Error::new();
    out.set_message(string_from(repr.message.data as *const u8, repr.message.len));
    Some(out)
}

pub fn get_resource_group_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GetResourceGroupRequest) -> &'a mut ResourceManagerGetResourceGroupRequest {
    let mut repr = ResourceManagerGetResourceGroupRequest {
        resource_group_name: KvprotoStringView { data: ptr::null(), len: 0 },
        with_ru_stats: Default::default(),
    };
    if !src.get_resource_group_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_resource_group_name());
        repr.resource_group_name.data = ptr as *const c_char;
        repr.resource_group_name.len = len;
    }
    repr.with_ru_stats = src.get_with_ru_stats();
    arena.alloc_struct(repr)
}

pub fn get_resource_group_request_from_repr_generated(src: *const ResourceManagerGetResourceGroupRequest) -> Option<pb::GetResourceGroupRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GetResourceGroupRequest::new();
    out.set_resource_group_name(string_from(repr.resource_group_name.data as *const u8, repr.resource_group_name.len));
    out.set_with_ru_stats(repr.with_ru_stats);
    Some(out)
}

pub fn get_resource_group_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GetResourceGroupResponse) -> &'a mut ResourceManagerGetResourceGroupResponse {
    let mut repr = ResourceManagerGetResourceGroupResponse {
        error: ptr::null_mut(),
        group: ptr::null_mut(),
    };
    if src.has_error() {
        repr.error = error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    if src.has_group() {
        repr.group = resource_group_to_repr_generated(arena, src.get_group()) as *mut _;
    } else {
        repr.group = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn get_resource_group_response_from_repr_generated(src: *const ResourceManagerGetResourceGroupResponse) -> Option<pb::GetResourceGroupResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GetResourceGroupResponse::new();
    if !repr.error.is_null() {
        if let Some(value) = error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    if !repr.group.is_null() {
        if let Some(value) = resource_group_from_repr_generated(repr.group) {
            out.set_group(value);
        }
    }
    Some(out)
}

pub fn granted_r_u_token_bucket_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GrantedRuTokenBucket) -> &'a mut ResourceManagerGrantedRUTokenBucket {
    let mut repr = ResourceManagerGrantedRUTokenBucket {
        type_field: Default::default(),
        granted_tokens: ptr::null_mut(),
        trickle_time_ms: Default::default(),
    };
    repr.type_field = src.get_type() as i32;
    if src.has_granted_tokens() {
        repr.granted_tokens = token_bucket_to_repr_generated(arena, src.get_granted_tokens()) as *mut _;
    } else {
        repr.granted_tokens = ptr::null_mut();
    }
    repr.trickle_time_ms = src.get_trickle_time_ms();
    arena.alloc_struct(repr)
}

pub fn granted_r_u_token_bucket_from_repr_generated(src: *const ResourceManagerGrantedRUTokenBucket) -> Option<pb::GrantedRuTokenBucket> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GrantedRuTokenBucket::new();
    out.set_type(pb::RequestUnitType::from_i32(repr.type_field).unwrap_or_default());
    if !repr.granted_tokens.is_null() {
        if let Some(value) = token_bucket_from_repr_generated(repr.granted_tokens) {
            out.set_granted_tokens(value);
        }
    }
    out.set_trickle_time_ms(repr.trickle_time_ms);
    Some(out)
}

pub fn granted_raw_resource_token_bucket_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GrantedRawResourceTokenBucket) -> &'a mut ResourceManagerGrantedRawResourceTokenBucket {
    let mut repr = ResourceManagerGrantedRawResourceTokenBucket {
        type_field: Default::default(),
        granted_tokens: ptr::null_mut(),
        trickle_time_ms: Default::default(),
    };
    repr.type_field = src.get_type() as i32;
    if src.has_granted_tokens() {
        repr.granted_tokens = token_bucket_to_repr_generated(arena, src.get_granted_tokens()) as *mut _;
    } else {
        repr.granted_tokens = ptr::null_mut();
    }
    repr.trickle_time_ms = src.get_trickle_time_ms();
    arena.alloc_struct(repr)
}

pub fn granted_raw_resource_token_bucket_from_repr_generated(src: *const ResourceManagerGrantedRawResourceTokenBucket) -> Option<pb::GrantedRawResourceTokenBucket> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GrantedRawResourceTokenBucket::new();
    out.set_type(pb::RawResourceType::from_i32(repr.type_field).unwrap_or_default());
    if !repr.granted_tokens.is_null() {
        if let Some(value) = token_bucket_from_repr_generated(repr.granted_tokens) {
            out.set_granted_tokens(value);
        }
    }
    out.set_trickle_time_ms(repr.trickle_time_ms);
    Some(out)
}

pub fn group_raw_resource_settings_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GroupRawResourceSettings) -> &'a mut ResourceManagerGroupRawResourceSettings {
    let mut repr = ResourceManagerGroupRawResourceSettings {
        cpu: ptr::null_mut(),
        io_read: ptr::null_mut(),
        io_write: ptr::null_mut(),
    };
    if src.has_cpu() {
        repr.cpu = token_bucket_to_repr_generated(arena, src.get_cpu()) as *mut _;
    } else {
        repr.cpu = ptr::null_mut();
    }
    if src.has_io_read() {
        repr.io_read = token_bucket_to_repr_generated(arena, src.get_io_read()) as *mut _;
    } else {
        repr.io_read = ptr::null_mut();
    }
    if src.has_io_write() {
        repr.io_write = token_bucket_to_repr_generated(arena, src.get_io_write()) as *mut _;
    } else {
        repr.io_write = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn group_raw_resource_settings_from_repr_generated(src: *const ResourceManagerGroupRawResourceSettings) -> Option<pb::GroupRawResourceSettings> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GroupRawResourceSettings::new();
    if !repr.cpu.is_null() {
        if let Some(value) = token_bucket_from_repr_generated(repr.cpu) {
            out.set_cpu(value);
        }
    }
    if !repr.io_read.is_null() {
        if let Some(value) = token_bucket_from_repr_generated(repr.io_read) {
            out.set_io_read(value);
        }
    }
    if !repr.io_write.is_null() {
        if let Some(value) = token_bucket_from_repr_generated(repr.io_write) {
            out.set_io_write(value);
        }
    }
    Some(out)
}

pub fn group_request_unit_settings_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::GroupRequestUnitSettings) -> &'a mut ResourceManagerGroupRequestUnitSettings {
    let mut repr = ResourceManagerGroupRequestUnitSettings {
        r_u: ptr::null_mut(),
    };
    if src.has_r_u() {
        repr.r_u = token_bucket_to_repr_generated(arena, src.get_r_u()) as *mut _;
    } else {
        repr.r_u = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn group_request_unit_settings_from_repr_generated(src: *const ResourceManagerGroupRequestUnitSettings) -> Option<pb::GroupRequestUnitSettings> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::GroupRequestUnitSettings::new();
    if !repr.r_u.is_null() {
        if let Some(value) = token_bucket_from_repr_generated(repr.r_u) {
            out.set_r_u(value);
        }
    }
    Some(out)
}

pub fn list_resource_groups_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ListResourceGroupsRequest) -> &'a mut ResourceManagerListResourceGroupsRequest {
    let mut repr = ResourceManagerListResourceGroupsRequest {
        with_ru_stats: Default::default(),
    };
    repr.with_ru_stats = src.get_with_ru_stats();
    arena.alloc_struct(repr)
}

pub fn list_resource_groups_request_from_repr_generated(src: *const ResourceManagerListResourceGroupsRequest) -> Option<pb::ListResourceGroupsRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ListResourceGroupsRequest::new();
    out.set_with_ru_stats(repr.with_ru_stats);
    Some(out)
}

pub fn list_resource_groups_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ListResourceGroupsResponse) -> &'a mut ResourceManagerListResourceGroupsResponse {
    let mut repr = ResourceManagerListResourceGroupsResponse {
        error: ptr::null_mut(),
        groups: KvprotoSliceResourceManagerResourceGroupPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_error() {
        repr.error = error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    {
        let values = src.get_groups();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut ResourceManagerResourceGroup> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(resource_group_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.groups.data = ptr;
                repr.groups.len = len;
                repr.groups.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn list_resource_groups_response_from_repr_generated(src: *const ResourceManagerListResourceGroupsResponse) -> Option<pb::ListResourceGroupsResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ListResourceGroupsResponse::new();
    if !repr.error.is_null() {
        if let Some(value) = error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    if !repr.groups.data.is_null() && repr.groups.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.groups.data, repr.groups.len) };
        let mut values: Vec<pb::ResourceGroup> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = resource_group_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_groups(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn participant_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Participant) -> &'a mut ResourceManagerParticipant {
    let mut repr = ResourceManagerParticipant {
        name: KvprotoStringView { data: ptr::null(), len: 0 },
        id: Default::default(),
        listen_urls: KvprotoSliceKvprotoStringView { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if !src.get_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name());
        repr.name.data = ptr as *const c_char;
        repr.name.len = len;
    }
    repr.id = src.get_id();
    {
        let values = src.get_listen_urls();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_string(value);
                views.push(KvprotoStringView { data: ptr as *const c_char, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.listen_urls.data = ptr;
                repr.listen_urls.len = len;
                repr.listen_urls.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn participant_from_repr_generated(src: *const ResourceManagerParticipant) -> Option<pb::Participant> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Participant::new();
    out.set_name(string_from(repr.name.data as *const u8, repr.name.len));
    out.set_id(repr.id);
    if !repr.listen_urls.data.is_null() && repr.listen_urls.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.listen_urls.data, repr.listen_urls.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(string_from(view.data as *const u8, view.len));
        }
        out.set_listen_urls(::protobuf::RepeatedField::from_vec(values));
    }
    Some(out)
}

pub fn put_resource_group_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PutResourceGroupRequest) -> &'a mut ResourceManagerPutResourceGroupRequest {
    let mut repr = ResourceManagerPutResourceGroupRequest {
        group: ptr::null_mut(),
    };
    if src.has_group() {
        repr.group = resource_group_to_repr_generated(arena, src.get_group()) as *mut _;
    } else {
        repr.group = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn put_resource_group_request_from_repr_generated(src: *const ResourceManagerPutResourceGroupRequest) -> Option<pb::PutResourceGroupRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PutResourceGroupRequest::new();
    if !repr.group.is_null() {
        if let Some(value) = resource_group_from_repr_generated(repr.group) {
            out.set_group(value);
        }
    }
    Some(out)
}

pub fn put_resource_group_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::PutResourceGroupResponse) -> &'a mut ResourceManagerPutResourceGroupResponse {
    let mut repr = ResourceManagerPutResourceGroupResponse {
        error: ptr::null_mut(),
        body: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_error() {
        repr.error = error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    if !src.get_body().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_body());
        repr.body.data = ptr as *const c_char;
        repr.body.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn put_resource_group_response_from_repr_generated(src: *const ResourceManagerPutResourceGroupResponse) -> Option<pb::PutResourceGroupResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::PutResourceGroupResponse::new();
    if !repr.error.is_null() {
        if let Some(value) = error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    out.set_body(string_from(repr.body.data as *const u8, repr.body.len));
    Some(out)
}

pub fn raw_resource_item_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RawResourceItem) -> &'a mut ResourceManagerRawResourceItem {
    let mut repr = ResourceManagerRawResourceItem {
        type_field: Default::default(),
        value: Default::default(),
    };
    repr.type_field = src.get_type() as i32;
    repr.value = src.get_value();
    arena.alloc_struct(repr)
}

pub fn raw_resource_item_from_repr_generated(src: *const ResourceManagerRawResourceItem) -> Option<pb::RawResourceItem> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RawResourceItem::new();
    out.set_type(pb::RawResourceType::from_i32(repr.type_field).unwrap_or_default());
    out.set_value(repr.value);
    Some(out)
}

pub fn request_unit_item_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RequestUnitItem) -> &'a mut ResourceManagerRequestUnitItem {
    let mut repr = ResourceManagerRequestUnitItem {
        type_field: Default::default(),
        value: Default::default(),
    };
    repr.type_field = src.get_type() as i32;
    repr.value = src.get_value();
    arena.alloc_struct(repr)
}

pub fn request_unit_item_from_repr_generated(src: *const ResourceManagerRequestUnitItem) -> Option<pb::RequestUnitItem> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RequestUnitItem::new();
    out.set_type(pb::RequestUnitType::from_i32(repr.type_field).unwrap_or_default());
    out.set_value(repr.value);
    Some(out)
}

pub fn resource_group_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::ResourceGroup) -> &'a mut ResourceManagerResourceGroup {
    let mut repr = ResourceManagerResourceGroup {
        name: KvprotoStringView { data: ptr::null(), len: 0 },
        mode: Default::default(),
        r_u_settings: ptr::null_mut(),
        raw_resource_settings: ptr::null_mut(),
        priority: Default::default(),
        runaway_settings: ptr::null_mut(),
        background_settings: ptr::null_mut(),
        RUStats: ptr::null_mut(),
    };
    if !src.get_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_name());
        repr.name.data = ptr as *const c_char;
        repr.name.len = len;
    }
    repr.mode = src.get_mode() as i32;
    if src.has_r_u_settings() {
        repr.r_u_settings = group_request_unit_settings_to_repr_generated(arena, src.get_r_u_settings()) as *mut _;
    } else {
        repr.r_u_settings = ptr::null_mut();
    }
    if src.has_raw_resource_settings() {
        repr.raw_resource_settings = group_raw_resource_settings_to_repr_generated(arena, src.get_raw_resource_settings()) as *mut _;
    } else {
        repr.raw_resource_settings = ptr::null_mut();
    }
    repr.priority = src.get_priority();
    if src.has_runaway_settings() {
        repr.runaway_settings = runaway_settings_to_repr_generated(arena, src.get_runaway_settings()) as *mut _;
    } else {
        repr.runaway_settings = ptr::null_mut();
    }
    if src.has_background_settings() {
        repr.background_settings = background_settings_to_repr_generated(arena, src.get_background_settings()) as *mut _;
    } else {
        repr.background_settings = ptr::null_mut();
    }
    if src.has_ru_stats() {
        repr.RUStats = consumption_to_repr_generated(arena, src.get_ru_stats()) as *mut _;
    } else {
        repr.RUStats = ptr::null_mut();
    }
    arena.alloc_struct(repr)
}

pub fn resource_group_from_repr_generated(src: *const ResourceManagerResourceGroup) -> Option<pb::ResourceGroup> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::ResourceGroup::new();
    out.set_name(string_from(repr.name.data as *const u8, repr.name.len));
    out.set_mode(pb::GroupMode::from_i32(repr.mode).unwrap_or_default());
    if !repr.r_u_settings.is_null() {
        if let Some(value) = group_request_unit_settings_from_repr_generated(repr.r_u_settings) {
            out.set_r_u_settings(value);
        }
    }
    if !repr.raw_resource_settings.is_null() {
        if let Some(value) = group_raw_resource_settings_from_repr_generated(repr.raw_resource_settings) {
            out.set_raw_resource_settings(value);
        }
    }
    out.set_priority(repr.priority);
    if !repr.runaway_settings.is_null() {
        if let Some(value) = runaway_settings_from_repr_generated(repr.runaway_settings) {
            out.set_runaway_settings(value);
        }
    }
    if !repr.background_settings.is_null() {
        if let Some(value) = background_settings_from_repr_generated(repr.background_settings) {
            out.set_background_settings(value);
        }
    }
    if !repr.RUStats.is_null() {
        if let Some(value) = consumption_from_repr_generated(repr.RUStats) {
            out.set_ru_stats(value);
        }
    }
    Some(out)
}

pub fn runaway_rule_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RunawayRule) -> &'a mut ResourceManagerRunawayRule {
    let mut repr = ResourceManagerRunawayRule {
        exec_elapsed_time_ms: Default::default(),
        processed_keys: Default::default(),
        request_unit: Default::default(),
    };
    repr.exec_elapsed_time_ms = src.get_exec_elapsed_time_ms();
    repr.processed_keys = src.get_processed_keys();
    repr.request_unit = src.get_request_unit();
    arena.alloc_struct(repr)
}

pub fn runaway_rule_from_repr_generated(src: *const ResourceManagerRunawayRule) -> Option<pb::RunawayRule> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RunawayRule::new();
    out.set_exec_elapsed_time_ms(repr.exec_elapsed_time_ms);
    out.set_processed_keys(repr.processed_keys);
    out.set_request_unit(repr.request_unit);
    Some(out)
}

pub fn runaway_settings_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RunawaySettings) -> &'a mut ResourceManagerRunawaySettings {
    let mut repr = ResourceManagerRunawaySettings {
        rule: ptr::null_mut(),
        action: Default::default(),
        watch: ptr::null_mut(),
        switch_group_name: KvprotoStringView { data: ptr::null(), len: 0 },
    };
    if src.has_rule() {
        repr.rule = runaway_rule_to_repr_generated(arena, src.get_rule()) as *mut _;
    } else {
        repr.rule = ptr::null_mut();
    }
    repr.action = src.get_action() as i32;
    if src.has_watch() {
        repr.watch = runaway_watch_to_repr_generated(arena, src.get_watch()) as *mut _;
    } else {
        repr.watch = ptr::null_mut();
    }
    if !src.get_switch_group_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_switch_group_name());
        repr.switch_group_name.data = ptr as *const c_char;
        repr.switch_group_name.len = len;
    }
    arena.alloc_struct(repr)
}

pub fn runaway_settings_from_repr_generated(src: *const ResourceManagerRunawaySettings) -> Option<pb::RunawaySettings> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RunawaySettings::new();
    if !repr.rule.is_null() {
        if let Some(value) = runaway_rule_from_repr_generated(repr.rule) {
            out.set_rule(value);
        }
    }
    out.set_action(pb::RunawayAction::from_i32(repr.action).unwrap_or_default());
    if !repr.watch.is_null() {
        if let Some(value) = runaway_watch_from_repr_generated(repr.watch) {
            out.set_watch(value);
        }
    }
    out.set_switch_group_name(string_from(repr.switch_group_name.data as *const u8, repr.switch_group_name.len));
    Some(out)
}

pub fn runaway_watch_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RunawayWatch) -> &'a mut ResourceManagerRunawayWatch {
    let mut repr = ResourceManagerRunawayWatch {
        lasting_duration_ms: Default::default(),
        type_field: Default::default(),
    };
    repr.lasting_duration_ms = src.get_lasting_duration_ms();
    repr.type_field = src.get_type() as i32;
    arena.alloc_struct(repr)
}

pub fn runaway_watch_from_repr_generated(src: *const ResourceManagerRunawayWatch) -> Option<pb::RunawayWatch> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RunawayWatch::new();
    out.set_lasting_duration_ms(repr.lasting_duration_ms);
    out.set_type(pb::RunawayWatchType::from_i32(repr.type_field).unwrap_or_default());
    Some(out)
}

pub fn token_bucket_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TokenBucket) -> &'a mut ResourceManagerTokenBucket {
    let mut repr = ResourceManagerTokenBucket {
        settings: ptr::null_mut(),
        tokens: Default::default(),
    };
    if src.has_settings() {
        repr.settings = token_limit_settings_to_repr_generated(arena, src.get_settings()) as *mut _;
    } else {
        repr.settings = ptr::null_mut();
    }
    repr.tokens = src.get_tokens();
    arena.alloc_struct(repr)
}

pub fn token_bucket_from_repr_generated(src: *const ResourceManagerTokenBucket) -> Option<pb::TokenBucket> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TokenBucket::new();
    if !repr.settings.is_null() {
        if let Some(value) = token_limit_settings_from_repr_generated(repr.settings) {
            out.set_settings(value);
        }
    }
    out.set_tokens(repr.tokens);
    Some(out)
}

pub fn token_bucket_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TokenBucketRequest) -> &'a mut ResourceManagerTokenBucketRequest {
    let mut repr = ResourceManagerTokenBucketRequest {
        resource_group_name: KvprotoStringView { data: ptr::null(), len: 0 },
        request_case: Default::default(),
        ru_items: ptr::null_mut(),
        raw_resource_items: ptr::null_mut(),
        consumption_since_last_request: ptr::null_mut(),
        is_background: Default::default(),
        is_tiflash: Default::default(),
    };
    if !src.get_resource_group_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_resource_group_name());
        repr.resource_group_name.data = ptr as *const c_char;
        repr.resource_group_name.len = len;
    }
    if src.has_consumption_since_last_request() {
        repr.consumption_since_last_request = consumption_to_repr_generated(arena, src.get_consumption_since_last_request()) as *mut _;
    } else {
        repr.consumption_since_last_request = ptr::null_mut();
    }
    repr.is_background = src.get_is_background();
    repr.is_tiflash = src.get_is_tiflash();
    arena.alloc_struct(repr)
}

pub fn token_bucket_request_from_repr_generated(src: *const ResourceManagerTokenBucketRequest) -> Option<pb::TokenBucketRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TokenBucketRequest::new();
    out.set_resource_group_name(string_from(repr.resource_group_name.data as *const u8, repr.resource_group_name.len));
    if !repr.consumption_since_last_request.is_null() {
        if let Some(value) = consumption_from_repr_generated(repr.consumption_since_last_request) {
            out.set_consumption_since_last_request(value);
        }
    }
    out.set_is_background(repr.is_background);
    out.set_is_tiflash(repr.is_tiflash);
    Some(out)
}

pub fn token_bucket_request__request_r_u_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TokenBucketRequestRequestRu) -> &'a mut ResourceManagerTokenBucketRequestRequestRU {
    let mut repr = ResourceManagerTokenBucketRequestRequestRU {
        request_r_u: KvprotoSliceResourceManagerRequestUnitItemPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_request_r_u();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut ResourceManagerRequestUnitItem> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(request_unit_item_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.request_r_u.data = ptr;
                repr.request_r_u.len = len;
                repr.request_r_u.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn token_bucket_request__request_r_u_from_repr_generated(src: *const ResourceManagerTokenBucketRequestRequestRU) -> Option<pb::TokenBucketRequestRequestRu> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TokenBucketRequestRequestRu::new();
    if !repr.request_r_u.data.is_null() && repr.request_r_u.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.request_r_u.data, repr.request_r_u.len) };
        let mut values: Vec<pb::RequestUnitItem> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = request_unit_item_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_request_r_u(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn token_bucket_request__request_raw_resource_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TokenBucketRequestRequestRawResource) -> &'a mut ResourceManagerTokenBucketRequestRequestRawResource {
    let mut repr = ResourceManagerTokenBucketRequestRequestRawResource {
        request_raw_resource: KvprotoSliceResourceManagerRawResourceItemPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_request_raw_resource();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut ResourceManagerRawResourceItem> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(raw_resource_item_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.request_raw_resource.data = ptr;
                repr.request_raw_resource.len = len;
                repr.request_raw_resource.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn token_bucket_request__request_raw_resource_from_repr_generated(src: *const ResourceManagerTokenBucketRequestRequestRawResource) -> Option<pb::TokenBucketRequestRequestRawResource> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TokenBucketRequestRequestRawResource::new();
    if !repr.request_raw_resource.data.is_null() && repr.request_raw_resource.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.request_raw_resource.data, repr.request_raw_resource.len) };
        let mut values: Vec<pb::RawResourceItem> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = raw_resource_item_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_request_raw_resource(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn token_bucket_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TokenBucketResponse) -> &'a mut ResourceManagerTokenBucketResponse {
    let mut repr = ResourceManagerTokenBucketResponse {
        resource_group_name: KvprotoStringView { data: ptr::null(), len: 0 },
        granted_r_u_tokens: KvprotoSliceResourceManagerGrantedRUTokenBucketPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        granted_resource_tokens: KvprotoSliceResourceManagerGrantedRawResourceTokenBucketPtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if !src.get_resource_group_name().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_resource_group_name());
        repr.resource_group_name.data = ptr as *const c_char;
        repr.resource_group_name.len = len;
    }
    {
        let values = src.get_granted_r_u_tokens();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut ResourceManagerGrantedRUTokenBucket> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(granted_r_u_token_bucket_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.granted_r_u_tokens.data = ptr;
                repr.granted_r_u_tokens.len = len;
                repr.granted_r_u_tokens.cap = len;
            }
        }
    }
    {
        let values = src.get_granted_resource_tokens();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut ResourceManagerGrantedRawResourceTokenBucket> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(granted_raw_resource_token_bucket_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.granted_resource_tokens.data = ptr;
                repr.granted_resource_tokens.len = len;
                repr.granted_resource_tokens.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn token_bucket_response_from_repr_generated(src: *const ResourceManagerTokenBucketResponse) -> Option<pb::TokenBucketResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TokenBucketResponse::new();
    out.set_resource_group_name(string_from(repr.resource_group_name.data as *const u8, repr.resource_group_name.len));
    if !repr.granted_r_u_tokens.data.is_null() && repr.granted_r_u_tokens.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.granted_r_u_tokens.data, repr.granted_r_u_tokens.len) };
        let mut values: Vec<pb::GrantedRuTokenBucket> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = granted_r_u_token_bucket_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_granted_r_u_tokens(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.granted_resource_tokens.data.is_null() && repr.granted_resource_tokens.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.granted_resource_tokens.data, repr.granted_resource_tokens.len) };
        let mut values: Vec<pb::GrantedRawResourceTokenBucket> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = granted_raw_resource_token_bucket_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_granted_resource_tokens(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn token_buckets_request_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TokenBucketsRequest) -> &'a mut ResourceManagerTokenBucketsRequest {
    let mut repr = ResourceManagerTokenBucketsRequest {
        requests: KvprotoSliceResourceManagerTokenBucketRequestPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        target_request_period_ms: Default::default(),
        client_unique_id: Default::default(),
    };
    {
        let values = src.get_requests();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut ResourceManagerTokenBucketRequest> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(token_bucket_request_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.requests.data = ptr;
                repr.requests.len = len;
                repr.requests.cap = len;
            }
        }
    }
    repr.target_request_period_ms = src.get_target_request_period_ms();
    repr.client_unique_id = src.get_client_unique_id();
    arena.alloc_struct(repr)
}

pub fn token_buckets_request_from_repr_generated(src: *const ResourceManagerTokenBucketsRequest) -> Option<pb::TokenBucketsRequest> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TokenBucketsRequest::new();
    if !repr.requests.data.is_null() && repr.requests.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.requests.data, repr.requests.len) };
        let mut values: Vec<pb::TokenBucketRequest> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = token_bucket_request_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_requests(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_target_request_period_ms(repr.target_request_period_ms);
    out.set_client_unique_id(repr.client_unique_id);
    Some(out)
}

pub fn token_buckets_response_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TokenBucketsResponse) -> &'a mut ResourceManagerTokenBucketsResponse {
    let mut repr = ResourceManagerTokenBucketsResponse {
        error: ptr::null_mut(),
        responses: KvprotoSliceResourceManagerTokenBucketResponsePtr { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    if src.has_error() {
        repr.error = error_to_repr_generated(arena, src.get_error()) as *mut _;
    } else {
        repr.error = ptr::null_mut();
    }
    {
        let values = src.get_responses();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut ResourceManagerTokenBucketResponse> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(token_bucket_response_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.responses.data = ptr;
                repr.responses.len = len;
                repr.responses.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn token_buckets_response_from_repr_generated(src: *const ResourceManagerTokenBucketsResponse) -> Option<pb::TokenBucketsResponse> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TokenBucketsResponse::new();
    if !repr.error.is_null() {
        if let Some(value) = error_from_repr_generated(repr.error) {
            out.set_error(value);
        }
    }
    if !repr.responses.data.is_null() && repr.responses.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.responses.data, repr.responses.len) };
        let mut values: Vec<pb::TokenBucketResponse> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = token_bucket_response_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_responses(::protobuf::RepeatedField::from_vec(values));
        }
    }
    Some(out)
}

pub fn token_limit_settings_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::TokenLimitSettings) -> &'a mut ResourceManagerTokenLimitSettings {
    let mut repr = ResourceManagerTokenLimitSettings {
        fill_rate: Default::default(),
        burst_limit: Default::default(),
        max_tokens: Default::default(),
    };
    repr.fill_rate = src.get_fill_rate();
    repr.burst_limit = src.get_burst_limit();
    repr.max_tokens = src.get_max_tokens();
    arena.alloc_struct(repr)
}

pub fn token_limit_settings_from_repr_generated(src: *const ResourceManagerTokenLimitSettings) -> Option<pb::TokenLimitSettings> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::TokenLimitSettings::new();
    out.set_fill_rate(repr.fill_rate);
    out.set_burst_limit(repr.burst_limit);
    out.set_max_tokens(repr.max_tokens);
    Some(out)
}

