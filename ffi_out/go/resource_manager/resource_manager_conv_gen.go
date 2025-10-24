//go:build kvffi_gen
// +build kvffi_gen

package resource_manager

/*
#cgo CFLAGS: -I../../c
#include "kvproto_abi.h"
*/
import "C"

import (
	"unsafe"
	runtime "github.com/pingcap/kvproto/ffi_out/go/runtime"
	resource_managerproto "github.com/pingcap/kvproto/pkg/resource_manager"
)

func NewReprBackgroundSettingsGenerated(arena *runtime.Arena, src *resource_managerproto.BackgroundSettings) *BackgroundSettings {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BackgroundSettings)(arena.AllocZero(uintptr(C.sizeof_resource_manager_BackgroundSettings)))
	IntoReprBackgroundSettingsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBackgroundSettingsGenerated(arena *runtime.Arena, dst *BackgroundSettings, src *resource_managerproto.BackgroundSettings) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	runtime.SetStringSlice(arena, unsafe.Pointer(&dst.job_types), src.GetJobTypes())
	dst.utilization_limit = C.uint64_t(src.GetUtilizationLimit())
}

func FromReprBackgroundSettingsGenerated(src *BackgroundSettings) *resource_managerproto.BackgroundSettings {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.BackgroundSettings{}
	out.JobTypes = runtime.CopyStringSlice(unsafe.Pointer(&src.job_types))
	out.UtilizationLimit = uint64(src.utilization_limit)
	return out
}

func NewReprConsumptionGenerated(arena *runtime.Arena, src *resource_managerproto.Consumption) *Consumption {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Consumption)(arena.AllocZero(uintptr(C.sizeof_resource_manager_Consumption)))
	IntoReprConsumptionGenerated(arena, ptr, src)
	return ptr
}

func IntoReprConsumptionGenerated(arena *runtime.Arena, dst *Consumption, src *resource_managerproto.Consumption) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.r_r_u = C.double(src.GetRRU())
	dst.w_r_u = C.double(src.GetWRU())
	dst.read_bytes = C.double(src.GetReadBytes())
	dst.write_bytes = C.double(src.GetWriteBytes())
	dst.total_cpu_time_ms = C.double(src.GetTotalCpuTimeMs())
	dst.sql_layer_cpu_time_ms = C.double(src.GetSqlLayerCpuTimeMs())
	dst.kv_read_rpc_count = C.double(src.GetKvReadRpcCount())
	dst.kv_write_rpc_count = C.double(src.GetKvWriteRpcCount())
}

func FromReprConsumptionGenerated(src *Consumption) *resource_managerproto.Consumption {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.Consumption{}
	out.RRU = float64(src.r_r_u)
	out.WRU = float64(src.w_r_u)
	out.ReadBytes = float64(src.read_bytes)
	out.WriteBytes = float64(src.write_bytes)
	out.TotalCpuTimeMs = float64(src.total_cpu_time_ms)
	out.SqlLayerCpuTimeMs = float64(src.sql_layer_cpu_time_ms)
	out.KvReadRpcCount = float64(src.kv_read_rpc_count)
	out.KvWriteRpcCount = float64(src.kv_write_rpc_count)
	return out
}

func NewReprDeleteResourceGroupRequestGenerated(arena *runtime.Arena, src *resource_managerproto.DeleteResourceGroupRequest) *DeleteResourceGroupRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DeleteResourceGroupRequest)(arena.AllocZero(uintptr(C.sizeof_resource_manager_DeleteResourceGroupRequest)))
	IntoReprDeleteResourceGroupRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDeleteResourceGroupRequestGenerated(arena *runtime.Arena, dst *DeleteResourceGroupRequest, src *resource_managerproto.DeleteResourceGroupRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetResourceGroupName()); length > 0 {
		dst.resource_group_name.data = (*C.char)(data)
		dst.resource_group_name.len = C.size_t(length)
	}
}

func FromReprDeleteResourceGroupRequestGenerated(src *DeleteResourceGroupRequest) *resource_managerproto.DeleteResourceGroupRequest {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.DeleteResourceGroupRequest{}
	out.ResourceGroupName = runtime.StringFrom(unsafe.Pointer(src.resource_group_name.data), int(src.resource_group_name.len))
	return out
}

func NewReprDeleteResourceGroupResponseGenerated(arena *runtime.Arena, src *resource_managerproto.DeleteResourceGroupResponse) *DeleteResourceGroupResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*DeleteResourceGroupResponse)(arena.AllocZero(uintptr(C.sizeof_resource_manager_DeleteResourceGroupResponse)))
	IntoReprDeleteResourceGroupResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprDeleteResourceGroupResponseGenerated(arena *runtime.Arena, dst *DeleteResourceGroupResponse, src *resource_managerproto.DeleteResourceGroupResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if data, length := arena.AllocString(src.GetBody()); length > 0 {
		dst.body.data = (*C.char)(data)
		dst.body.len = C.size_t(length)
	}
}

func FromReprDeleteResourceGroupResponseGenerated(src *DeleteResourceGroupResponse) *resource_managerproto.DeleteResourceGroupResponse {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.DeleteResourceGroupResponse{}
	if src.error != nil {
		out.Error = FromReprErrorGenerated(src.error)
	}
	out.Body = runtime.StringFrom(unsafe.Pointer(src.body.data), int(src.body.len))
	return out
}

func NewReprErrorGenerated(arena *runtime.Arena, src *resource_managerproto.Error) *Error {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Error)(arena.AllocZero(uintptr(C.sizeof_resource_manager_Error)))
	IntoReprErrorGenerated(arena, ptr, src)
	return ptr
}

func IntoReprErrorGenerated(arena *runtime.Arena, dst *Error, src *resource_managerproto.Error) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetMessage()); length > 0 {
		dst.message.data = (*C.char)(data)
		dst.message.len = C.size_t(length)
	}
}

func FromReprErrorGenerated(src *Error) *resource_managerproto.Error {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.Error{}
	out.Message = runtime.StringFrom(unsafe.Pointer(src.message.data), int(src.message.len))
	return out
}

func NewReprGetResourceGroupRequestGenerated(arena *runtime.Arena, src *resource_managerproto.GetResourceGroupRequest) *GetResourceGroupRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GetResourceGroupRequest)(arena.AllocZero(uintptr(C.sizeof_resource_manager_GetResourceGroupRequest)))
	IntoReprGetResourceGroupRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGetResourceGroupRequestGenerated(arena *runtime.Arena, dst *GetResourceGroupRequest, src *resource_managerproto.GetResourceGroupRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetResourceGroupName()); length > 0 {
		dst.resource_group_name.data = (*C.char)(data)
		dst.resource_group_name.len = C.size_t(length)
	}
	dst.with_ru_stats = C.bool(src.GetWithRuStats())
}

func FromReprGetResourceGroupRequestGenerated(src *GetResourceGroupRequest) *resource_managerproto.GetResourceGroupRequest {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.GetResourceGroupRequest{}
	out.ResourceGroupName = runtime.StringFrom(unsafe.Pointer(src.resource_group_name.data), int(src.resource_group_name.len))
	out.WithRuStats = bool(src.with_ru_stats)
	return out
}

func NewReprGetResourceGroupResponseGenerated(arena *runtime.Arena, src *resource_managerproto.GetResourceGroupResponse) *GetResourceGroupResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GetResourceGroupResponse)(arena.AllocZero(uintptr(C.sizeof_resource_manager_GetResourceGroupResponse)))
	IntoReprGetResourceGroupResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGetResourceGroupResponseGenerated(arena *runtime.Arena, dst *GetResourceGroupResponse, src *resource_managerproto.GetResourceGroupResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if value := src.GetGroup(); value != nil {
		dst.group = NewReprResourceGroupGenerated(arena, value)
	} else {
		dst.group = nil
	}
}

func FromReprGetResourceGroupResponseGenerated(src *GetResourceGroupResponse) *resource_managerproto.GetResourceGroupResponse {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.GetResourceGroupResponse{}
	if src.error != nil {
		out.Error = FromReprErrorGenerated(src.error)
	}
	if src.group != nil {
		out.Group = FromReprResourceGroupGenerated(src.group)
	}
	return out
}

func NewReprGrantedRUTokenBucketGenerated(arena *runtime.Arena, src *resource_managerproto.GrantedRUTokenBucket) *GrantedRUTokenBucket {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GrantedRUTokenBucket)(arena.AllocZero(uintptr(C.sizeof_resource_manager_GrantedRUTokenBucket)))
	IntoReprGrantedRUTokenBucketGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGrantedRUTokenBucketGenerated(arena *runtime.Arena, dst *GrantedRUTokenBucket, src *resource_managerproto.GrantedRUTokenBucket) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.type_field = C.int32_t(int32(src.GetType()))
	if value := src.GetGrantedTokens(); value != nil {
		dst.granted_tokens = NewReprTokenBucketGenerated(arena, value)
	} else {
		dst.granted_tokens = nil
	}
	dst.trickle_time_ms = C.int64_t(src.GetTrickleTimeMs())
}

func FromReprGrantedRUTokenBucketGenerated(src *GrantedRUTokenBucket) *resource_managerproto.GrantedRUTokenBucket {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.GrantedRUTokenBucket{}
	out.Type = resource_managerproto.RequestUnitType(int32(src.type_field))
	if src.granted_tokens != nil {
		out.GrantedTokens = FromReprTokenBucketGenerated(src.granted_tokens)
	}
	out.TrickleTimeMs = int64(src.trickle_time_ms)
	return out
}

func NewReprGrantedRawResourceTokenBucketGenerated(arena *runtime.Arena, src *resource_managerproto.GrantedRawResourceTokenBucket) *GrantedRawResourceTokenBucket {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GrantedRawResourceTokenBucket)(arena.AllocZero(uintptr(C.sizeof_resource_manager_GrantedRawResourceTokenBucket)))
	IntoReprGrantedRawResourceTokenBucketGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGrantedRawResourceTokenBucketGenerated(arena *runtime.Arena, dst *GrantedRawResourceTokenBucket, src *resource_managerproto.GrantedRawResourceTokenBucket) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.type_field = C.int32_t(int32(src.GetType()))
	if value := src.GetGrantedTokens(); value != nil {
		dst.granted_tokens = NewReprTokenBucketGenerated(arena, value)
	} else {
		dst.granted_tokens = nil
	}
	dst.trickle_time_ms = C.int64_t(src.GetTrickleTimeMs())
}

func FromReprGrantedRawResourceTokenBucketGenerated(src *GrantedRawResourceTokenBucket) *resource_managerproto.GrantedRawResourceTokenBucket {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.GrantedRawResourceTokenBucket{}
	out.Type = resource_managerproto.RawResourceType(int32(src.type_field))
	if src.granted_tokens != nil {
		out.GrantedTokens = FromReprTokenBucketGenerated(src.granted_tokens)
	}
	out.TrickleTimeMs = int64(src.trickle_time_ms)
	return out
}

func NewReprGroupRawResourceSettingsGenerated(arena *runtime.Arena, src *resource_managerproto.GroupRawResourceSettings) *GroupRawResourceSettings {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GroupRawResourceSettings)(arena.AllocZero(uintptr(C.sizeof_resource_manager_GroupRawResourceSettings)))
	IntoReprGroupRawResourceSettingsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGroupRawResourceSettingsGenerated(arena *runtime.Arena, dst *GroupRawResourceSettings, src *resource_managerproto.GroupRawResourceSettings) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetCpu(); value != nil {
		dst.cpu = NewReprTokenBucketGenerated(arena, value)
	} else {
		dst.cpu = nil
	}
	if value := src.GetIoRead(); value != nil {
		dst.io_read = NewReprTokenBucketGenerated(arena, value)
	} else {
		dst.io_read = nil
	}
	if value := src.GetIoWrite(); value != nil {
		dst.io_write = NewReprTokenBucketGenerated(arena, value)
	} else {
		dst.io_write = nil
	}
}

func FromReprGroupRawResourceSettingsGenerated(src *GroupRawResourceSettings) *resource_managerproto.GroupRawResourceSettings {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.GroupRawResourceSettings{}
	if src.cpu != nil {
		out.Cpu = FromReprTokenBucketGenerated(src.cpu)
	}
	if src.io_read != nil {
		out.IoRead = FromReprTokenBucketGenerated(src.io_read)
	}
	if src.io_write != nil {
		out.IoWrite = FromReprTokenBucketGenerated(src.io_write)
	}
	return out
}

func NewReprGroupRequestUnitSettingsGenerated(arena *runtime.Arena, src *resource_managerproto.GroupRequestUnitSettings) *GroupRequestUnitSettings {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*GroupRequestUnitSettings)(arena.AllocZero(uintptr(C.sizeof_resource_manager_GroupRequestUnitSettings)))
	IntoReprGroupRequestUnitSettingsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprGroupRequestUnitSettingsGenerated(arena *runtime.Arena, dst *GroupRequestUnitSettings, src *resource_managerproto.GroupRequestUnitSettings) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRU(); value != nil {
		dst.r_u = NewReprTokenBucketGenerated(arena, value)
	} else {
		dst.r_u = nil
	}
}

func FromReprGroupRequestUnitSettingsGenerated(src *GroupRequestUnitSettings) *resource_managerproto.GroupRequestUnitSettings {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.GroupRequestUnitSettings{}
	if src.r_u != nil {
		out.RU = FromReprTokenBucketGenerated(src.r_u)
	}
	return out
}

func NewReprListResourceGroupsRequestGenerated(arena *runtime.Arena, src *resource_managerproto.ListResourceGroupsRequest) *ListResourceGroupsRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ListResourceGroupsRequest)(arena.AllocZero(uintptr(C.sizeof_resource_manager_ListResourceGroupsRequest)))
	IntoReprListResourceGroupsRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprListResourceGroupsRequestGenerated(arena *runtime.Arena, dst *ListResourceGroupsRequest, src *resource_managerproto.ListResourceGroupsRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.with_ru_stats = C.bool(src.GetWithRuStats())
}

func FromReprListResourceGroupsRequestGenerated(src *ListResourceGroupsRequest) *resource_managerproto.ListResourceGroupsRequest {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.ListResourceGroupsRequest{}
	out.WithRuStats = bool(src.with_ru_stats)
	return out
}

func NewReprListResourceGroupsResponseGenerated(arena *runtime.Arena, src *resource_managerproto.ListResourceGroupsResponse) *ListResourceGroupsResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ListResourceGroupsResponse)(arena.AllocZero(uintptr(C.sizeof_resource_manager_ListResourceGroupsResponse)))
	IntoReprListResourceGroupsResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprListResourceGroupsResponseGenerated(arena *runtime.Arena, dst *ListResourceGroupsResponse, src *resource_managerproto.ListResourceGroupsResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if values := src.GetGroups(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*ResourceGroup)(nil)))
		array := unsafe.Slice((**ResourceGroup)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprResourceGroupGenerated(arena, value)
		}
		dst.groups.data = (**ResourceGroup)(ptr)
		dst.groups.len = C.size_t(len(values))
		dst.groups.cap = C.size_t(len(values))
	}
}

func FromReprListResourceGroupsResponseGenerated(src *ListResourceGroupsResponse) *resource_managerproto.ListResourceGroupsResponse {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.ListResourceGroupsResponse{}
	if src.error != nil {
		out.Error = FromReprErrorGenerated(src.error)
	}
	if src.groups.data != nil && src.groups.len > 0 {
		length := int(src.groups.len)
		ptrs := unsafe.Slice((**ResourceGroup)(unsafe.Pointer(src.groups.data)), length)
		out.Groups = make([]*resource_managerproto.ResourceGroup, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Groups = append(out.Groups, FromReprResourceGroupGenerated(ptr))
		}
	}
	return out
}

func NewReprParticipantGenerated(arena *runtime.Arena, src *resource_managerproto.Participant) *Participant {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Participant)(arena.AllocZero(uintptr(C.sizeof_resource_manager_Participant)))
	IntoReprParticipantGenerated(arena, ptr, src)
	return ptr
}

func IntoReprParticipantGenerated(arena *runtime.Arena, dst *Participant, src *resource_managerproto.Participant) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetName()); length > 0 {
		dst.name.data = (*C.char)(data)
		dst.name.len = C.size_t(length)
	}
	dst.id = C.uint64_t(src.GetId())
	runtime.SetStringSlice(arena, unsafe.Pointer(&dst.listen_urls), src.GetListenUrls())
}

func FromReprParticipantGenerated(src *Participant) *resource_managerproto.Participant {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.Participant{}
	out.Name = runtime.StringFrom(unsafe.Pointer(src.name.data), int(src.name.len))
	out.Id = uint64(src.id)
	out.ListenUrls = runtime.CopyStringSlice(unsafe.Pointer(&src.listen_urls))
	return out
}

func NewReprPutResourceGroupRequestGenerated(arena *runtime.Arena, src *resource_managerproto.PutResourceGroupRequest) *PutResourceGroupRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PutResourceGroupRequest)(arena.AllocZero(uintptr(C.sizeof_resource_manager_PutResourceGroupRequest)))
	IntoReprPutResourceGroupRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPutResourceGroupRequestGenerated(arena *runtime.Arena, dst *PutResourceGroupRequest, src *resource_managerproto.PutResourceGroupRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetGroup(); value != nil {
		dst.group = NewReprResourceGroupGenerated(arena, value)
	} else {
		dst.group = nil
	}
}

func FromReprPutResourceGroupRequestGenerated(src *PutResourceGroupRequest) *resource_managerproto.PutResourceGroupRequest {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.PutResourceGroupRequest{}
	if src.group != nil {
		out.Group = FromReprResourceGroupGenerated(src.group)
	}
	return out
}

func NewReprPutResourceGroupResponseGenerated(arena *runtime.Arena, src *resource_managerproto.PutResourceGroupResponse) *PutResourceGroupResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*PutResourceGroupResponse)(arena.AllocZero(uintptr(C.sizeof_resource_manager_PutResourceGroupResponse)))
	IntoReprPutResourceGroupResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPutResourceGroupResponseGenerated(arena *runtime.Arena, dst *PutResourceGroupResponse, src *resource_managerproto.PutResourceGroupResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if data, length := arena.AllocString(src.GetBody()); length > 0 {
		dst.body.data = (*C.char)(data)
		dst.body.len = C.size_t(length)
	}
}

func FromReprPutResourceGroupResponseGenerated(src *PutResourceGroupResponse) *resource_managerproto.PutResourceGroupResponse {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.PutResourceGroupResponse{}
	if src.error != nil {
		out.Error = FromReprErrorGenerated(src.error)
	}
	out.Body = runtime.StringFrom(unsafe.Pointer(src.body.data), int(src.body.len))
	return out
}

func NewReprRawResourceItemGenerated(arena *runtime.Arena, src *resource_managerproto.RawResourceItem) *RawResourceItem {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RawResourceItem)(arena.AllocZero(uintptr(C.sizeof_resource_manager_RawResourceItem)))
	IntoReprRawResourceItemGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRawResourceItemGenerated(arena *runtime.Arena, dst *RawResourceItem, src *resource_managerproto.RawResourceItem) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.type_field = C.int32_t(int32(src.GetType()))
	dst.value = C.double(src.GetValue())
}

func FromReprRawResourceItemGenerated(src *RawResourceItem) *resource_managerproto.RawResourceItem {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.RawResourceItem{}
	out.Type = resource_managerproto.RawResourceType(int32(src.type_field))
	out.Value = float64(src.value)
	return out
}

func NewReprRequestUnitItemGenerated(arena *runtime.Arena, src *resource_managerproto.RequestUnitItem) *RequestUnitItem {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RequestUnitItem)(arena.AllocZero(uintptr(C.sizeof_resource_manager_RequestUnitItem)))
	IntoReprRequestUnitItemGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRequestUnitItemGenerated(arena *runtime.Arena, dst *RequestUnitItem, src *resource_managerproto.RequestUnitItem) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.type_field = C.int32_t(int32(src.GetType()))
	dst.value = C.double(src.GetValue())
}

func FromReprRequestUnitItemGenerated(src *RequestUnitItem) *resource_managerproto.RequestUnitItem {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.RequestUnitItem{}
	out.Type = resource_managerproto.RequestUnitType(int32(src.type_field))
	out.Value = float64(src.value)
	return out
}

func NewReprResourceGroupGenerated(arena *runtime.Arena, src *resource_managerproto.ResourceGroup) *ResourceGroup {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*ResourceGroup)(arena.AllocZero(uintptr(C.sizeof_resource_manager_ResourceGroup)))
	IntoReprResourceGroupGenerated(arena, ptr, src)
	return ptr
}

func IntoReprResourceGroupGenerated(arena *runtime.Arena, dst *ResourceGroup, src *resource_managerproto.ResourceGroup) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetName()); length > 0 {
		dst.name.data = (*C.char)(data)
		dst.name.len = C.size_t(length)
	}
	dst.mode = C.int32_t(int32(src.GetMode()))
	if value := src.GetRUSettings(); value != nil {
		dst.r_u_settings = NewReprGroupRequestUnitSettingsGenerated(arena, value)
	} else {
		dst.r_u_settings = nil
	}
	if value := src.GetRawResourceSettings(); value != nil {
		dst.raw_resource_settings = NewReprGroupRawResourceSettingsGenerated(arena, value)
	} else {
		dst.raw_resource_settings = nil
	}
	dst.priority = C.uint32_t(src.GetPriority())
	if value := src.GetRunawaySettings(); value != nil {
		dst.runaway_settings = NewReprRunawaySettingsGenerated(arena, value)
	} else {
		dst.runaway_settings = nil
	}
	if value := src.GetBackgroundSettings(); value != nil {
		dst.background_settings = NewReprBackgroundSettingsGenerated(arena, value)
	} else {
		dst.background_settings = nil
	}
	if value := src.GetRUStats(); value != nil {
		dst.RUStats = NewReprConsumptionGenerated(arena, value)
	} else {
		dst.RUStats = nil
	}
}

func FromReprResourceGroupGenerated(src *ResourceGroup) *resource_managerproto.ResourceGroup {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.ResourceGroup{}
	out.Name = runtime.StringFrom(unsafe.Pointer(src.name.data), int(src.name.len))
	out.Mode = resource_managerproto.GroupMode(int32(src.mode))
	if src.r_u_settings != nil {
		out.RUSettings = FromReprGroupRequestUnitSettingsGenerated(src.r_u_settings)
	}
	if src.raw_resource_settings != nil {
		out.RawResourceSettings = FromReprGroupRawResourceSettingsGenerated(src.raw_resource_settings)
	}
	out.Priority = uint32(src.priority)
	if src.runaway_settings != nil {
		out.RunawaySettings = FromReprRunawaySettingsGenerated(src.runaway_settings)
	}
	if src.background_settings != nil {
		out.BackgroundSettings = FromReprBackgroundSettingsGenerated(src.background_settings)
	}
	if src.RUStats != nil {
		out.RUStats = FromReprConsumptionGenerated(src.RUStats)
	}
	return out
}

func NewReprRunawayRuleGenerated(arena *runtime.Arena, src *resource_managerproto.RunawayRule) *RunawayRule {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RunawayRule)(arena.AllocZero(uintptr(C.sizeof_resource_manager_RunawayRule)))
	IntoReprRunawayRuleGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRunawayRuleGenerated(arena *runtime.Arena, dst *RunawayRule, src *resource_managerproto.RunawayRule) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.exec_elapsed_time_ms = C.uint64_t(src.GetExecElapsedTimeMs())
	dst.processed_keys = C.int64_t(src.GetProcessedKeys())
	dst.request_unit = C.int64_t(src.GetRequestUnit())
}

func FromReprRunawayRuleGenerated(src *RunawayRule) *resource_managerproto.RunawayRule {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.RunawayRule{}
	out.ExecElapsedTimeMs = uint64(src.exec_elapsed_time_ms)
	out.ProcessedKeys = int64(src.processed_keys)
	out.RequestUnit = int64(src.request_unit)
	return out
}

func NewReprRunawaySettingsGenerated(arena *runtime.Arena, src *resource_managerproto.RunawaySettings) *RunawaySettings {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RunawaySettings)(arena.AllocZero(uintptr(C.sizeof_resource_manager_RunawaySettings)))
	IntoReprRunawaySettingsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRunawaySettingsGenerated(arena *runtime.Arena, dst *RunawaySettings, src *resource_managerproto.RunawaySettings) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetRule(); value != nil {
		dst.rule = NewReprRunawayRuleGenerated(arena, value)
	} else {
		dst.rule = nil
	}
	dst.action = C.int32_t(int32(src.GetAction()))
	if value := src.GetWatch(); value != nil {
		dst.watch = NewReprRunawayWatchGenerated(arena, value)
	} else {
		dst.watch = nil
	}
	if data, length := arena.AllocString(src.GetSwitchGroupName()); length > 0 {
		dst.switch_group_name.data = (*C.char)(data)
		dst.switch_group_name.len = C.size_t(length)
	}
}

func FromReprRunawaySettingsGenerated(src *RunawaySettings) *resource_managerproto.RunawaySettings {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.RunawaySettings{}
	if src.rule != nil {
		out.Rule = FromReprRunawayRuleGenerated(src.rule)
	}
	out.Action = resource_managerproto.RunawayAction(int32(src.action))
	if src.watch != nil {
		out.Watch = FromReprRunawayWatchGenerated(src.watch)
	}
	out.SwitchGroupName = runtime.StringFrom(unsafe.Pointer(src.switch_group_name.data), int(src.switch_group_name.len))
	return out
}

func NewReprRunawayWatchGenerated(arena *runtime.Arena, src *resource_managerproto.RunawayWatch) *RunawayWatch {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RunawayWatch)(arena.AllocZero(uintptr(C.sizeof_resource_manager_RunawayWatch)))
	IntoReprRunawayWatchGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRunawayWatchGenerated(arena *runtime.Arena, dst *RunawayWatch, src *resource_managerproto.RunawayWatch) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.lasting_duration_ms = C.int64_t(src.GetLastingDurationMs())
	dst.type_field = C.int32_t(int32(src.GetType()))
}

func FromReprRunawayWatchGenerated(src *RunawayWatch) *resource_managerproto.RunawayWatch {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.RunawayWatch{}
	out.LastingDurationMs = int64(src.lasting_duration_ms)
	out.Type = resource_managerproto.RunawayWatchType(int32(src.type_field))
	return out
}

func NewReprTokenBucketGenerated(arena *runtime.Arena, src *resource_managerproto.TokenBucket) *TokenBucket {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TokenBucket)(arena.AllocZero(uintptr(C.sizeof_resource_manager_TokenBucket)))
	IntoReprTokenBucketGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTokenBucketGenerated(arena *runtime.Arena, dst *TokenBucket, src *resource_managerproto.TokenBucket) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetSettings(); value != nil {
		dst.settings = NewReprTokenLimitSettingsGenerated(arena, value)
	} else {
		dst.settings = nil
	}
	dst.tokens = C.double(src.GetTokens())
}

func FromReprTokenBucketGenerated(src *TokenBucket) *resource_managerproto.TokenBucket {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.TokenBucket{}
	if src.settings != nil {
		out.Settings = FromReprTokenLimitSettingsGenerated(src.settings)
	}
	out.Tokens = float64(src.tokens)
	return out
}

func NewReprTokenBucketRequestGenerated(arena *runtime.Arena, src *resource_managerproto.TokenBucketRequest) *TokenBucketRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TokenBucketRequest)(arena.AllocZero(uintptr(C.sizeof_resource_manager_TokenBucketRequest)))
	IntoReprTokenBucketRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTokenBucketRequestGenerated(arena *runtime.Arena, dst *TokenBucketRequest, src *resource_managerproto.TokenBucketRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetResourceGroupName()); length > 0 {
		dst.resource_group_name.data = (*C.char)(data)
		dst.resource_group_name.len = C.size_t(length)
	}
	if value := src.GetConsumptionSinceLastRequest(); value != nil {
		dst.consumption_since_last_request = NewReprConsumptionGenerated(arena, value)
	} else {
		dst.consumption_since_last_request = nil
	}
	dst.is_background = C.bool(src.GetIsBackground())
	dst.is_tiflash = C.bool(src.GetIsTiflash())
	dst.request_case = 0
	switch value := src.GetRequest().(type) {
	case *resource_managerproto.TokenBucketRequest_RuItems:
		dst.request_case = C.int32_t(2)
		if value.RuItems != nil {
			dst.ru_items = NewReprTokenBucketRequest_RequestRUGenerated(arena, value.RuItems)
		} else {
			dst.ru_items = nil
		}
	case *resource_managerproto.TokenBucketRequest_RawResourceItems:
		dst.request_case = C.int32_t(3)
		if value.RawResourceItems != nil {
			dst.raw_resource_items = NewReprTokenBucketRequest_RequestRawResourceGenerated(arena, value.RawResourceItems)
		} else {
			dst.raw_resource_items = nil
		}
	default:
		dst.request_case = 0
	}
}

func FromReprTokenBucketRequestGenerated(src *TokenBucketRequest) *resource_managerproto.TokenBucketRequest {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.TokenBucketRequest{}
	out.ResourceGroupName = runtime.StringFrom(unsafe.Pointer(src.resource_group_name.data), int(src.resource_group_name.len))
	if src.consumption_since_last_request != nil {
		out.ConsumptionSinceLastRequest = FromReprConsumptionGenerated(src.consumption_since_last_request)
	}
	out.IsBackground = bool(src.is_background)
	out.IsTiflash = bool(src.is_tiflash)
	switch int32(src.request_case) {
	case 2:
		out.Request = &resource_managerproto.TokenBucketRequest_RuItems{
			RuItems: FromReprTokenBucketRequest_RequestRUGenerated(src.ru_items),
		}
	case 3:
		out.Request = &resource_managerproto.TokenBucketRequest_RawResourceItems{
			RawResourceItems: FromReprTokenBucketRequest_RequestRawResourceGenerated(src.raw_resource_items),
		}
	}
	return out
}

func NewReprTokenBucketRequest_RequestRUGenerated(arena *runtime.Arena, src *resource_managerproto.TokenBucketRequest_RequestRU) *TokenBucketRequest_RequestRU {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TokenBucketRequest_RequestRU)(arena.AllocZero(uintptr(C.sizeof_resource_manager_TokenBucketRequest_RequestRU)))
	IntoReprTokenBucketRequest_RequestRUGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTokenBucketRequest_RequestRUGenerated(arena *runtime.Arena, dst *TokenBucketRequest_RequestRU, src *resource_managerproto.TokenBucketRequest_RequestRU) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetRequestRU(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*RequestUnitItem)(nil)))
		array := unsafe.Slice((**RequestUnitItem)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprRequestUnitItemGenerated(arena, value)
		}
		dst.request_r_u.data = (**RequestUnitItem)(ptr)
		dst.request_r_u.len = C.size_t(len(values))
		dst.request_r_u.cap = C.size_t(len(values))
	}
}

func FromReprTokenBucketRequest_RequestRUGenerated(src *TokenBucketRequest_RequestRU) *resource_managerproto.TokenBucketRequest_RequestRU {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.TokenBucketRequest_RequestRU{}
	if src.request_r_u.data != nil && src.request_r_u.len > 0 {
		length := int(src.request_r_u.len)
		ptrs := unsafe.Slice((**RequestUnitItem)(unsafe.Pointer(src.request_r_u.data)), length)
		out.RequestRU = make([]*resource_managerproto.RequestUnitItem, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.RequestRU = append(out.RequestRU, FromReprRequestUnitItemGenerated(ptr))
		}
	}
	return out
}

func NewReprTokenBucketRequest_RequestRawResourceGenerated(arena *runtime.Arena, src *resource_managerproto.TokenBucketRequest_RequestRawResource) *TokenBucketRequest_RequestRawResource {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TokenBucketRequest_RequestRawResource)(arena.AllocZero(uintptr(C.sizeof_resource_manager_TokenBucketRequest_RequestRawResource)))
	IntoReprTokenBucketRequest_RequestRawResourceGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTokenBucketRequest_RequestRawResourceGenerated(arena *runtime.Arena, dst *TokenBucketRequest_RequestRawResource, src *resource_managerproto.TokenBucketRequest_RequestRawResource) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetRequestRawResource(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*RawResourceItem)(nil)))
		array := unsafe.Slice((**RawResourceItem)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprRawResourceItemGenerated(arena, value)
		}
		dst.request_raw_resource.data = (**RawResourceItem)(ptr)
		dst.request_raw_resource.len = C.size_t(len(values))
		dst.request_raw_resource.cap = C.size_t(len(values))
	}
}

func FromReprTokenBucketRequest_RequestRawResourceGenerated(src *TokenBucketRequest_RequestRawResource) *resource_managerproto.TokenBucketRequest_RequestRawResource {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.TokenBucketRequest_RequestRawResource{}
	if src.request_raw_resource.data != nil && src.request_raw_resource.len > 0 {
		length := int(src.request_raw_resource.len)
		ptrs := unsafe.Slice((**RawResourceItem)(unsafe.Pointer(src.request_raw_resource.data)), length)
		out.RequestRawResource = make([]*resource_managerproto.RawResourceItem, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.RequestRawResource = append(out.RequestRawResource, FromReprRawResourceItemGenerated(ptr))
		}
	}
	return out
}

func NewReprTokenBucketResponseGenerated(arena *runtime.Arena, src *resource_managerproto.TokenBucketResponse) *TokenBucketResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TokenBucketResponse)(arena.AllocZero(uintptr(C.sizeof_resource_manager_TokenBucketResponse)))
	IntoReprTokenBucketResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTokenBucketResponseGenerated(arena *runtime.Arena, dst *TokenBucketResponse, src *resource_managerproto.TokenBucketResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if data, length := arena.AllocString(src.GetResourceGroupName()); length > 0 {
		dst.resource_group_name.data = (*C.char)(data)
		dst.resource_group_name.len = C.size_t(length)
	}
	if values := src.GetGrantedRUTokens(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*GrantedRUTokenBucket)(nil)))
		array := unsafe.Slice((**GrantedRUTokenBucket)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprGrantedRUTokenBucketGenerated(arena, value)
		}
		dst.granted_r_u_tokens.data = (**GrantedRUTokenBucket)(ptr)
		dst.granted_r_u_tokens.len = C.size_t(len(values))
		dst.granted_r_u_tokens.cap = C.size_t(len(values))
	}
	if values := src.GetGrantedResourceTokens(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*GrantedRawResourceTokenBucket)(nil)))
		array := unsafe.Slice((**GrantedRawResourceTokenBucket)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprGrantedRawResourceTokenBucketGenerated(arena, value)
		}
		dst.granted_resource_tokens.data = (**GrantedRawResourceTokenBucket)(ptr)
		dst.granted_resource_tokens.len = C.size_t(len(values))
		dst.granted_resource_tokens.cap = C.size_t(len(values))
	}
}

func FromReprTokenBucketResponseGenerated(src *TokenBucketResponse) *resource_managerproto.TokenBucketResponse {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.TokenBucketResponse{}
	out.ResourceGroupName = runtime.StringFrom(unsafe.Pointer(src.resource_group_name.data), int(src.resource_group_name.len))
	if src.granted_r_u_tokens.data != nil && src.granted_r_u_tokens.len > 0 {
		length := int(src.granted_r_u_tokens.len)
		ptrs := unsafe.Slice((**GrantedRUTokenBucket)(unsafe.Pointer(src.granted_r_u_tokens.data)), length)
		out.GrantedRUTokens = make([]*resource_managerproto.GrantedRUTokenBucket, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.GrantedRUTokens = append(out.GrantedRUTokens, FromReprGrantedRUTokenBucketGenerated(ptr))
		}
	}
	if src.granted_resource_tokens.data != nil && src.granted_resource_tokens.len > 0 {
		length := int(src.granted_resource_tokens.len)
		ptrs := unsafe.Slice((**GrantedRawResourceTokenBucket)(unsafe.Pointer(src.granted_resource_tokens.data)), length)
		out.GrantedResourceTokens = make([]*resource_managerproto.GrantedRawResourceTokenBucket, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.GrantedResourceTokens = append(out.GrantedResourceTokens, FromReprGrantedRawResourceTokenBucketGenerated(ptr))
		}
	}
	return out
}

func NewReprTokenBucketsRequestGenerated(arena *runtime.Arena, src *resource_managerproto.TokenBucketsRequest) *TokenBucketsRequest {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TokenBucketsRequest)(arena.AllocZero(uintptr(C.sizeof_resource_manager_TokenBucketsRequest)))
	IntoReprTokenBucketsRequestGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTokenBucketsRequestGenerated(arena *runtime.Arena, dst *TokenBucketsRequest, src *resource_managerproto.TokenBucketsRequest) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetRequests(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*TokenBucketRequest)(nil)))
		array := unsafe.Slice((**TokenBucketRequest)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprTokenBucketRequestGenerated(arena, value)
		}
		dst.requests.data = (**TokenBucketRequest)(ptr)
		dst.requests.len = C.size_t(len(values))
		dst.requests.cap = C.size_t(len(values))
	}
	dst.target_request_period_ms = C.uint64_t(src.GetTargetRequestPeriodMs())
	dst.client_unique_id = C.uint64_t(src.GetClientUniqueId())
}

func FromReprTokenBucketsRequestGenerated(src *TokenBucketsRequest) *resource_managerproto.TokenBucketsRequest {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.TokenBucketsRequest{}
	if src.requests.data != nil && src.requests.len > 0 {
		length := int(src.requests.len)
		ptrs := unsafe.Slice((**TokenBucketRequest)(unsafe.Pointer(src.requests.data)), length)
		out.Requests = make([]*resource_managerproto.TokenBucketRequest, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Requests = append(out.Requests, FromReprTokenBucketRequestGenerated(ptr))
		}
	}
	out.TargetRequestPeriodMs = uint64(src.target_request_period_ms)
	out.ClientUniqueId = uint64(src.client_unique_id)
	return out
}

func NewReprTokenBucketsResponseGenerated(arena *runtime.Arena, src *resource_managerproto.TokenBucketsResponse) *TokenBucketsResponse {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TokenBucketsResponse)(arena.AllocZero(uintptr(C.sizeof_resource_manager_TokenBucketsResponse)))
	IntoReprTokenBucketsResponseGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTokenBucketsResponseGenerated(arena *runtime.Arena, dst *TokenBucketsResponse, src *resource_managerproto.TokenBucketsResponse) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if value := src.GetError(); value != nil {
		dst.error = NewReprErrorGenerated(arena, value)
	} else {
		dst.error = nil
	}
	if values := src.GetResponses(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*TokenBucketResponse)(nil)))
		array := unsafe.Slice((**TokenBucketResponse)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprTokenBucketResponseGenerated(arena, value)
		}
		dst.responses.data = (**TokenBucketResponse)(ptr)
		dst.responses.len = C.size_t(len(values))
		dst.responses.cap = C.size_t(len(values))
	}
}

func FromReprTokenBucketsResponseGenerated(src *TokenBucketsResponse) *resource_managerproto.TokenBucketsResponse {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.TokenBucketsResponse{}
	if src.error != nil {
		out.Error = FromReprErrorGenerated(src.error)
	}
	if src.responses.data != nil && src.responses.len > 0 {
		length := int(src.responses.len)
		ptrs := unsafe.Slice((**TokenBucketResponse)(unsafe.Pointer(src.responses.data)), length)
		out.Responses = make([]*resource_managerproto.TokenBucketResponse, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Responses = append(out.Responses, FromReprTokenBucketResponseGenerated(ptr))
		}
	}
	return out
}

func NewReprTokenLimitSettingsGenerated(arena *runtime.Arena, src *resource_managerproto.TokenLimitSettings) *TokenLimitSettings {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*TokenLimitSettings)(arena.AllocZero(uintptr(C.sizeof_resource_manager_TokenLimitSettings)))
	IntoReprTokenLimitSettingsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprTokenLimitSettingsGenerated(arena *runtime.Arena, dst *TokenLimitSettings, src *resource_managerproto.TokenLimitSettings) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.fill_rate = C.uint64_t(src.GetFillRate())
	dst.burst_limit = C.int64_t(src.GetBurstLimit())
	dst.max_tokens = C.double(src.GetMaxTokens())
}

func FromReprTokenLimitSettingsGenerated(src *TokenLimitSettings) *resource_managerproto.TokenLimitSettings {
	if src == nil {
		return nil
	}
	out := &resource_managerproto.TokenLimitSettings{}
	out.FillRate = uint64(src.fill_rate)
	out.BurstLimit = int64(src.burst_limit)
	out.MaxTokens = float64(src.max_tokens)
	return out
}
