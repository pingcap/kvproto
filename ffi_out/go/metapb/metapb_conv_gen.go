//go:build kvffi_gen
// +build kvffi_gen

package metapb

/*
#cgo CFLAGS: -I../../c
#include "kvproto_abi.h"
*/
import "C"

import (
	"unsafe"
	encryptionpbffi "github.com/pingcap/kvproto/ffi_out/go/encryptionpb"
	runtime "github.com/pingcap/kvproto/ffi_out/go/runtime"
	metapbproto "github.com/pingcap/kvproto/pkg/metapb"
)

func NewReprBucketMetaGenerated(arena *runtime.Arena, src *metapbproto.BucketMeta) *BucketMeta {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BucketMeta)(arena.AllocZero(uintptr(C.sizeof_metapb_BucketMeta)))
	IntoReprBucketMetaGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBucketMetaGenerated(arena *runtime.Arena, dst *BucketMeta, src *metapbproto.BucketMeta) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.version = C.uint64_t(src.GetVersion())
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
}

func FromReprBucketMetaGenerated(src *BucketMeta) *metapbproto.BucketMeta {
	if src == nil {
		return nil
	}
	out := &metapbproto.BucketMeta{}
	out.Version = uint64(src.version)
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	return out
}

func NewReprBucketStatsGenerated(arena *runtime.Arena, src *metapbproto.BucketStats) *BucketStats {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*BucketStats)(arena.AllocZero(uintptr(C.sizeof_metapb_BucketStats)))
	IntoReprBucketStatsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBucketStatsGenerated(arena *runtime.Arena, dst *BucketStats, src *metapbproto.BucketStats) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	if values := src.GetReadBytes(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.read_bytes.data = (*C.uint64_t)(ptr)
		dst.read_bytes.len = C.size_t(len(values))
		dst.read_bytes.cap = C.size_t(len(values))
	}
	if values := src.GetWriteBytes(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.write_bytes.data = (*C.uint64_t)(ptr)
		dst.write_bytes.len = C.size_t(len(values))
		dst.write_bytes.cap = C.size_t(len(values))
	}
	if values := src.GetReadQps(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.read_qps.data = (*C.uint64_t)(ptr)
		dst.read_qps.len = C.size_t(len(values))
		dst.read_qps.cap = C.size_t(len(values))
	}
	if values := src.GetWriteQps(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.write_qps.data = (*C.uint64_t)(ptr)
		dst.write_qps.len = C.size_t(len(values))
		dst.write_qps.cap = C.size_t(len(values))
	}
	if values := src.GetReadKeys(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.read_keys.data = (*C.uint64_t)(ptr)
		dst.read_keys.len = C.size_t(len(values))
		dst.read_keys.cap = C.size_t(len(values))
	}
	if values := src.GetWriteKeys(); len(values) > 0 {
		ptr := arena.AllocZero(uintptr(len(values)) * unsafe.Sizeof(C.uint64_t(0)))
		array := unsafe.Slice((*C.uint64_t)(ptr), len(values))
		for i, value := range values {
			array[i] = C.uint64_t(value)
		}
		dst.write_keys.data = (*C.uint64_t)(ptr)
		dst.write_keys.len = C.size_t(len(values))
		dst.write_keys.cap = C.size_t(len(values))
	}
}

func FromReprBucketStatsGenerated(src *BucketStats) *metapbproto.BucketStats {
	if src == nil {
		return nil
	}
	out := &metapbproto.BucketStats{}
	if src.read_bytes.data != nil && src.read_bytes.len > 0 {
		length := int(src.read_bytes.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.read_bytes.data)), length)
		out.ReadBytes = make([]uint64, 0, length)
		for _, value := range values {
			out.ReadBytes = append(out.ReadBytes, uint64(value))
		}
	}
	if src.write_bytes.data != nil && src.write_bytes.len > 0 {
		length := int(src.write_bytes.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.write_bytes.data)), length)
		out.WriteBytes = make([]uint64, 0, length)
		for _, value := range values {
			out.WriteBytes = append(out.WriteBytes, uint64(value))
		}
	}
	if src.read_qps.data != nil && src.read_qps.len > 0 {
		length := int(src.read_qps.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.read_qps.data)), length)
		out.ReadQps = make([]uint64, 0, length)
		for _, value := range values {
			out.ReadQps = append(out.ReadQps, uint64(value))
		}
	}
	if src.write_qps.data != nil && src.write_qps.len > 0 {
		length := int(src.write_qps.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.write_qps.data)), length)
		out.WriteQps = make([]uint64, 0, length)
		for _, value := range values {
			out.WriteQps = append(out.WriteQps, uint64(value))
		}
	}
	if src.read_keys.data != nil && src.read_keys.len > 0 {
		length := int(src.read_keys.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.read_keys.data)), length)
		out.ReadKeys = make([]uint64, 0, length)
		for _, value := range values {
			out.ReadKeys = append(out.ReadKeys, uint64(value))
		}
	}
	if src.write_keys.data != nil && src.write_keys.len > 0 {
		length := int(src.write_keys.len)
		values := unsafe.Slice((*C.uint64_t)(unsafe.Pointer(src.write_keys.data)), length)
		out.WriteKeys = make([]uint64, 0, length)
		for _, value := range values {
			out.WriteKeys = append(out.WriteKeys, uint64(value))
		}
	}
	return out
}

func NewReprBucketsGenerated(arena *runtime.Arena, src *metapbproto.Buckets) *Buckets {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Buckets)(arena.AllocZero(uintptr(C.sizeof_metapb_Buckets)))
	IntoReprBucketsGenerated(arena, ptr, src)
	return ptr
}

func IntoReprBucketsGenerated(arena *runtime.Arena, dst *Buckets, src *metapbproto.Buckets) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.region_id = C.uint64_t(src.GetRegionId())
	dst.version = C.uint64_t(src.GetVersion())
	runtime.SetBytesSlice(arena, unsafe.Pointer(&dst.keys), src.GetKeys())
	if value := src.GetStats(); value != nil {
		dst.stats = NewReprBucketStatsGenerated(arena, value)
	} else {
		dst.stats = nil
	}
	dst.period_in_ms = C.uint64_t(src.GetPeriodInMs())
}

func FromReprBucketsGenerated(src *Buckets) *metapbproto.Buckets {
	if src == nil {
		return nil
	}
	out := &metapbproto.Buckets{}
	out.RegionId = uint64(src.region_id)
	out.Version = uint64(src.version)
	out.Keys = runtime.CopyBytesSlice(unsafe.Pointer(&src.keys))
	if src.stats != nil {
		out.Stats = FromReprBucketStatsGenerated(src.stats)
	}
	out.PeriodInMs = uint64(src.period_in_ms)
	return out
}

func NewReprClusterGenerated(arena *runtime.Arena, src *metapbproto.Cluster) *Cluster {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Cluster)(arena.AllocZero(uintptr(C.sizeof_metapb_Cluster)))
	IntoReprClusterGenerated(arena, ptr, src)
	return ptr
}

func IntoReprClusterGenerated(arena *runtime.Arena, dst *Cluster, src *metapbproto.Cluster) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.id = C.uint64_t(src.GetId())
	dst.max_peer_count = C.uint32_t(src.GetMaxPeerCount())
}

func FromReprClusterGenerated(src *Cluster) *metapbproto.Cluster {
	if src == nil {
		return nil
	}
	out := &metapbproto.Cluster{}
	out.Id = uint64(src.id)
	out.MaxPeerCount = uint32(src.max_peer_count)
	return out
}

func NewReprPeerGenerated(arena *runtime.Arena, src *metapbproto.Peer) *Peer {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Peer)(arena.AllocZero(uintptr(C.sizeof_metapb_Peer)))
	IntoReprPeerGenerated(arena, ptr, src)
	return ptr
}

func IntoReprPeerGenerated(arena *runtime.Arena, dst *Peer, src *metapbproto.Peer) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.id = C.uint64_t(src.GetId())
	dst.store_id = C.uint64_t(src.GetStoreId())
	dst.role = C.int32_t(int32(src.GetRole()))
	dst.is_witness = C.bool(src.GetIsWitness())
}

func FromReprPeerGenerated(src *Peer) *metapbproto.Peer {
	if src == nil {
		return nil
	}
	out := &metapbproto.Peer{}
	out.Id = uint64(src.id)
	out.StoreId = uint64(src.store_id)
	out.Role = metapbproto.PeerRole(int32(src.role))
	out.IsWitness = bool(src.is_witness)
	return out
}

func NewReprRegionGenerated(arena *runtime.Arena, src *metapbproto.Region) *Region {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Region)(arena.AllocZero(uintptr(C.sizeof_metapb_Region)))
	IntoReprRegionGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRegionGenerated(arena *runtime.Arena, dst *Region, src *metapbproto.Region) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.id = C.uint64_t(src.GetId())
	if data, length := arena.AllocBytes(src.GetStartKey()); length > 0 {
		dst.start_key.data = (*C.uint8_t)(data)
		dst.start_key.len = C.size_t(length)
	}
	if data, length := arena.AllocBytes(src.GetEndKey()); length > 0 {
		dst.end_key.data = (*C.uint8_t)(data)
		dst.end_key.len = C.size_t(length)
	}
	if value := src.GetRegionEpoch(); value != nil {
		dst.region_epoch = NewReprRegionEpochGenerated(arena, value)
	} else {
		dst.region_epoch = nil
	}
	if values := src.GetPeers(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*Peer)(nil)))
		array := unsafe.Slice((**Peer)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprPeerGenerated(arena, value)
		}
		dst.peers.data = (**Peer)(ptr)
		dst.peers.len = C.size_t(len(values))
		dst.peers.cap = C.size_t(len(values))
	}
	if value := src.GetEncryptionMeta(); value != nil {
		dst.encryption_meta = (*C.encryptionpb_EncryptionMeta)(unsafe.Pointer(encryptionpbffi.NewReprEncryptionMetaGenerated(arena, value)))
	} else {
		dst.encryption_meta = nil
	}
	dst.is_in_flashback = C.bool(src.GetIsInFlashback())
	dst.flashback_start_ts = C.uint64_t(src.GetFlashbackStartTs())
}

func FromReprRegionGenerated(src *Region) *metapbproto.Region {
	if src == nil {
		return nil
	}
	out := &metapbproto.Region{}
	out.Id = uint64(src.id)
	out.StartKey = runtime.BytesFrom(unsafe.Pointer(src.start_key.data), int(src.start_key.len))
	out.EndKey = runtime.BytesFrom(unsafe.Pointer(src.end_key.data), int(src.end_key.len))
	if src.region_epoch != nil {
		out.RegionEpoch = FromReprRegionEpochGenerated(src.region_epoch)
	}
	if src.peers.data != nil && src.peers.len > 0 {
		length := int(src.peers.len)
		ptrs := unsafe.Slice((**Peer)(unsafe.Pointer(src.peers.data)), length)
		out.Peers = make([]*metapbproto.Peer, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Peers = append(out.Peers, FromReprPeerGenerated(ptr))
		}
	}
	if src.encryption_meta != nil {
		out.EncryptionMeta = encryptionpbffi.FromReprEncryptionMetaGenerated((*encryptionpbffi.EncryptionMeta)(unsafe.Pointer(src.encryption_meta)))
	}
	out.IsInFlashback = bool(src.is_in_flashback)
	out.FlashbackStartTs = uint64(src.flashback_start_ts)
	return out
}

func NewReprRegionEpochGenerated(arena *runtime.Arena, src *metapbproto.RegionEpoch) *RegionEpoch {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*RegionEpoch)(arena.AllocZero(uintptr(C.sizeof_metapb_RegionEpoch)))
	IntoReprRegionEpochGenerated(arena, ptr, src)
	return ptr
}

func IntoReprRegionEpochGenerated(arena *runtime.Arena, dst *RegionEpoch, src *metapbproto.RegionEpoch) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.conf_ver = C.uint64_t(src.GetConfVer())
	dst.version = C.uint64_t(src.GetVersion())
}

func FromReprRegionEpochGenerated(src *RegionEpoch) *metapbproto.RegionEpoch {
	if src == nil {
		return nil
	}
	out := &metapbproto.RegionEpoch{}
	out.ConfVer = uint64(src.conf_ver)
	out.Version = uint64(src.version)
	return out
}

func NewReprStoreGenerated(arena *runtime.Arena, src *metapbproto.Store) *Store {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*Store)(arena.AllocZero(uintptr(C.sizeof_metapb_Store)))
	IntoReprStoreGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStoreGenerated(arena *runtime.Arena, dst *Store, src *metapbproto.Store) {
	if arena == nil || dst == nil || src == nil {
		return
	}
	dst.id = C.uint64_t(src.GetId())
	if data, length := arena.AllocString(src.GetAddress()); length > 0 {
		dst.address.data = (*C.char)(data)
		dst.address.len = C.size_t(length)
	}
	dst.state = C.int32_t(int32(src.GetState()))
	if values := src.GetLabels(); len(values) > 0 {
		ptr := arena.AllocPointerArray(len(values), unsafe.Sizeof((*StoreLabel)(nil)))
		array := unsafe.Slice((**StoreLabel)(ptr), len(values))
		for i, value := range values {
			array[i] = NewReprStoreLabelGenerated(arena, value)
		}
		dst.labels.data = (**StoreLabel)(ptr)
		dst.labels.len = C.size_t(len(values))
		dst.labels.cap = C.size_t(len(values))
	}
	if data, length := arena.AllocString(src.GetVersion()); length > 0 {
		dst.version.data = (*C.char)(data)
		dst.version.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetPeerAddress()); length > 0 {
		dst.peer_address.data = (*C.char)(data)
		dst.peer_address.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetStatusAddress()); length > 0 {
		dst.status_address.data = (*C.char)(data)
		dst.status_address.len = C.size_t(length)
	}
	if data, length := arena.AllocString(src.GetGitHash()); length > 0 {
		dst.git_hash.data = (*C.char)(data)
		dst.git_hash.len = C.size_t(length)
	}
	dst.start_timestamp = C.int64_t(src.GetStartTimestamp())
	if data, length := arena.AllocString(src.GetDeployPath()); length > 0 {
		dst.deploy_path.data = (*C.char)(data)
		dst.deploy_path.len = C.size_t(length)
	}
	dst.last_heartbeat = C.int64_t(src.GetLastHeartbeat())
	dst.physically_destroyed = C.bool(src.GetPhysicallyDestroyed())
	dst.node_state = C.int32_t(int32(src.GetNodeState()))
}

func FromReprStoreGenerated(src *Store) *metapbproto.Store {
	if src == nil {
		return nil
	}
	out := &metapbproto.Store{}
	out.Id = uint64(src.id)
	out.Address = runtime.StringFrom(unsafe.Pointer(src.address.data), int(src.address.len))
	out.State = metapbproto.StoreState(int32(src.state))
	if src.labels.data != nil && src.labels.len > 0 {
		length := int(src.labels.len)
		ptrs := unsafe.Slice((**StoreLabel)(unsafe.Pointer(src.labels.data)), length)
		out.Labels = make([]*metapbproto.StoreLabel, 0, length)
		for _, ptr := range ptrs {
			if ptr == nil {
				continue
			}
			out.Labels = append(out.Labels, FromReprStoreLabelGenerated(ptr))
		}
	}
	out.Version = runtime.StringFrom(unsafe.Pointer(src.version.data), int(src.version.len))
	out.PeerAddress = runtime.StringFrom(unsafe.Pointer(src.peer_address.data), int(src.peer_address.len))
	out.StatusAddress = runtime.StringFrom(unsafe.Pointer(src.status_address.data), int(src.status_address.len))
	out.GitHash = runtime.StringFrom(unsafe.Pointer(src.git_hash.data), int(src.git_hash.len))
	out.StartTimestamp = int64(src.start_timestamp)
	out.DeployPath = runtime.StringFrom(unsafe.Pointer(src.deploy_path.data), int(src.deploy_path.len))
	out.LastHeartbeat = int64(src.last_heartbeat)
	out.PhysicallyDestroyed = bool(src.physically_destroyed)
	out.NodeState = metapbproto.NodeState(int32(src.node_state))
	return out
}

func NewReprStoreLabelGenerated(arena *runtime.Arena, src *metapbproto.StoreLabel) *StoreLabel {
	if arena == nil || src == nil {
		return nil
	}
	ptr := (*StoreLabel)(arena.AllocZero(uintptr(C.sizeof_metapb_StoreLabel)))
	IntoReprStoreLabelGenerated(arena, ptr, src)
	return ptr
}

func IntoReprStoreLabelGenerated(arena *runtime.Arena, dst *StoreLabel, src *metapbproto.StoreLabel) {
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

func FromReprStoreLabelGenerated(src *StoreLabel) *metapbproto.StoreLabel {
	if src == nil {
		return nil
	}
	out := &metapbproto.StoreLabel{}
	out.Key = runtime.StringFrom(unsafe.Pointer(src.key.data), int(src.key.len))
	out.Value = runtime.StringFrom(unsafe.Pointer(src.value.data), int(src.value.len))
	return out
}
