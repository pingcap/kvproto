//! Auto-generated conversions (feature `kvffi_gen`).
#![cfg(feature = "kvffi_gen")]

use std::ptr;
use std::os::raw::c_char;

use protobuf::ProtobufEnum;
use crate::ffi_runtime::arena::{Arena, bytes_from, string_from};
use crate::ffi_runtime::abi::{EncryptionpbEncryptionMeta, KvprotoBytesView, KvprotoSliceKvprotoBytesView, KvprotoSliceMetapbPeerPtr, KvprotoSliceMetapbStoreLabelPtr, KvprotoSliceUint64T, KvprotoStringView, MetapbBucketMeta, MetapbBucketStats, MetapbBuckets, MetapbCluster, MetapbPeer, MetapbRegion, MetapbRegionEpoch, MetapbStore, MetapbStoreLabel};
use crate::metapb as pb;
use crate::encryptionpb;

pub fn bucket_meta_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BucketMeta) -> &'a mut MetapbBucketMeta {
    let mut repr = MetapbBucketMeta {
        version: Default::default(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    repr.version = src.get_version();
    {
        let values = src.get_keys();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_bytes(value);
                views.push(KvprotoBytesView { data: ptr, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.keys.data = ptr;
                repr.keys.len = len;
                repr.keys.cap = len;
            }
        }
    }
    arena.alloc_struct(repr)
}

pub fn bucket_meta_from_repr_generated(src: *const MetapbBucketMeta) -> Option<pb::BucketMeta> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BucketMeta::new();
    out.set_version(repr.version);
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    Some(out)
}

pub fn bucket_stats_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::BucketStats) -> &'a mut MetapbBucketStats {
    let mut repr = MetapbBucketStats {
        read_bytes: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        write_bytes: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        read_qps: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        write_qps: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        read_keys: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
        write_keys: KvprotoSliceUint64T { data: ptr::null_mut(), len: 0, cap: 0 },
    };
    {
        let values = src.get_read_bytes();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.read_bytes.data = ptr;
            repr.read_bytes.len = len;
            repr.read_bytes.cap = len;
        }
    }
    {
        let values = src.get_write_bytes();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.write_bytes.data = ptr;
            repr.write_bytes.len = len;
            repr.write_bytes.cap = len;
        }
    }
    {
        let values = src.get_read_qps();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.read_qps.data = ptr;
            repr.read_qps.len = len;
            repr.read_qps.cap = len;
        }
    }
    {
        let values = src.get_write_qps();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.write_qps.data = ptr;
            repr.write_qps.len = len;
            repr.write_qps.cap = len;
        }
    }
    {
        let values = src.get_read_keys();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.read_keys.data = ptr;
            repr.read_keys.len = len;
            repr.read_keys.cap = len;
        }
    }
    {
        let values = src.get_write_keys();
        if !values.is_empty() {
            let mut vec: Vec<u64> = Vec::with_capacity(values.len());
            for value in values.iter() {
                vec.push(*value);
            }
            let (ptr, len) = arena.alloc_vec(vec);
            repr.write_keys.data = ptr;
            repr.write_keys.len = len;
            repr.write_keys.cap = len;
        }
    }
    arena.alloc_struct(repr)
}

pub fn bucket_stats_from_repr_generated(src: *const MetapbBucketStats) -> Option<pb::BucketStats> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::BucketStats::new();
    if !repr.read_bytes.data.is_null() && repr.read_bytes.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.read_bytes.data, repr.read_bytes.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_read_bytes(values);
    }
    if !repr.write_bytes.data.is_null() && repr.write_bytes.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.write_bytes.data, repr.write_bytes.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_write_bytes(values);
    }
    if !repr.read_qps.data.is_null() && repr.read_qps.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.read_qps.data, repr.read_qps.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_read_qps(values);
    }
    if !repr.write_qps.data.is_null() && repr.write_qps.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.write_qps.data, repr.write_qps.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_write_qps(values);
    }
    if !repr.read_keys.data.is_null() && repr.read_keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.read_keys.data, repr.read_keys.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_read_keys(values);
    }
    if !repr.write_keys.data.is_null() && repr.write_keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.write_keys.data, repr.write_keys.len) };
        let mut values: Vec<u64> = Vec::with_capacity(slice.len());
        values.extend_from_slice(slice);
        out.set_write_keys(values);
    }
    Some(out)
}

pub fn buckets_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Buckets) -> &'a mut MetapbBuckets {
    let mut repr = MetapbBuckets {
        region_id: Default::default(),
        version: Default::default(),
        keys: KvprotoSliceKvprotoBytesView { data: ptr::null_mut(), len: 0, cap: 0 },
        stats: ptr::null_mut(),
        period_in_ms: Default::default(),
    };
    repr.region_id = src.get_region_id();
    repr.version = src.get_version();
    {
        let values = src.get_keys();
        if !values.is_empty() {
            let mut views = Vec::with_capacity(values.len());
            for value in values {
                if value.is_empty() { continue; }
                let (ptr, len) = arena.alloc_bytes(value);
                views.push(KvprotoBytesView { data: ptr, len });
            }
            if !views.is_empty() {
                let (ptr, len) = arena.alloc_vec(views);
                repr.keys.data = ptr;
                repr.keys.len = len;
                repr.keys.cap = len;
            }
        }
    }
    if src.has_stats() {
        repr.stats = bucket_stats_to_repr_generated(arena, src.get_stats()) as *mut _;
    } else {
        repr.stats = ptr::null_mut();
    }
    repr.period_in_ms = src.get_period_in_ms();
    arena.alloc_struct(repr)
}

pub fn buckets_from_repr_generated(src: *const MetapbBuckets) -> Option<pb::Buckets> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Buckets::new();
    out.set_region_id(repr.region_id);
    out.set_version(repr.version);
    if !repr.keys.data.is_null() && repr.keys.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.keys.data, repr.keys.len) };
        let mut values = Vec::with_capacity(slice.len());
        for view in slice {
            values.push(bytes_from(view.data, view.len));
        }
        out.set_keys(::protobuf::RepeatedField::from_vec(values));
    }
    if !repr.stats.is_null() {
        if let Some(value) = bucket_stats_from_repr_generated(repr.stats) {
            out.set_stats(value);
        }
    }
    out.set_period_in_ms(repr.period_in_ms);
    Some(out)
}

pub fn cluster_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Cluster) -> &'a mut MetapbCluster {
    let mut repr = MetapbCluster {
        id: Default::default(),
        max_peer_count: Default::default(),
    };
    repr.id = src.get_id();
    repr.max_peer_count = src.get_max_peer_count();
    arena.alloc_struct(repr)
}

pub fn cluster_from_repr_generated(src: *const MetapbCluster) -> Option<pb::Cluster> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Cluster::new();
    out.set_id(repr.id);
    out.set_max_peer_count(repr.max_peer_count);
    Some(out)
}

pub fn peer_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Peer) -> &'a mut MetapbPeer {
    let mut repr = MetapbPeer {
        id: Default::default(),
        store_id: Default::default(),
        role: Default::default(),
        is_witness: Default::default(),
    };
    repr.id = src.get_id();
    repr.store_id = src.get_store_id();
    repr.role = src.get_role() as i32;
    repr.is_witness = src.get_is_witness();
    arena.alloc_struct(repr)
}

pub fn peer_from_repr_generated(src: *const MetapbPeer) -> Option<pb::Peer> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Peer::new();
    out.set_id(repr.id);
    out.set_store_id(repr.store_id);
    out.set_role(pb::PeerRole::from_i32(repr.role).unwrap_or_default());
    out.set_is_witness(repr.is_witness);
    Some(out)
}

pub fn region_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Region) -> &'a mut MetapbRegion {
    let mut repr = MetapbRegion {
        id: Default::default(),
        start_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        end_key: KvprotoBytesView { data: ptr::null_mut(), len: 0 },
        region_epoch: ptr::null_mut(),
        peers: KvprotoSliceMetapbPeerPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        encryption_meta: ptr::null_mut(),
        is_in_flashback: Default::default(),
        flashback_start_ts: Default::default(),
    };
    repr.id = src.get_id();
    if !src.get_start_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_start_key());
        repr.start_key.data = ptr;
        repr.start_key.len = len;
    }
    if !src.get_end_key().is_empty() {
        let (ptr, len) = arena.alloc_bytes(src.get_end_key());
        repr.end_key.data = ptr;
        repr.end_key.len = len;
    }
    if src.has_region_epoch() {
        repr.region_epoch = region_epoch_to_repr_generated(arena, src.get_region_epoch()) as *mut _;
    } else {
        repr.region_epoch = ptr::null_mut();
    }
    {
        let values = src.get_peers();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut MetapbPeer> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(peer_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.peers.data = ptr;
                repr.peers.len = len;
                repr.peers.cap = len;
            }
        }
    }
    if src.has_encryption_meta() {
        repr.encryption_meta = crate::ffi_runtime::encryptionpb::encryption_meta_to_repr_generated(arena, src.get_encryption_meta()) as *mut _;
    } else {
        repr.encryption_meta = ptr::null_mut();
    }
    repr.is_in_flashback = src.get_is_in_flashback();
    repr.flashback_start_ts = src.get_flashback_start_ts();
    arena.alloc_struct(repr)
}

pub fn region_from_repr_generated(src: *const MetapbRegion) -> Option<pb::Region> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Region::new();
    out.set_id(repr.id);
    out.set_start_key(bytes_from(repr.start_key.data, repr.start_key.len).into());
    out.set_end_key(bytes_from(repr.end_key.data, repr.end_key.len).into());
    if !repr.region_epoch.is_null() {
        if let Some(value) = region_epoch_from_repr_generated(repr.region_epoch) {
            out.set_region_epoch(value);
        }
    }
    if !repr.peers.data.is_null() && repr.peers.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.peers.data, repr.peers.len) };
        let mut values: Vec<pb::Peer> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = peer_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_peers(::protobuf::RepeatedField::from_vec(values));
        }
    }
    if !repr.encryption_meta.is_null() {
        if let Some(value) = crate::ffi_runtime::encryptionpb::encryption_meta_from_repr_generated(repr.encryption_meta) {
            out.set_encryption_meta(value);
        }
    }
    out.set_is_in_flashback(repr.is_in_flashback);
    out.set_flashback_start_ts(repr.flashback_start_ts);
    Some(out)
}

pub fn region_epoch_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::RegionEpoch) -> &'a mut MetapbRegionEpoch {
    let mut repr = MetapbRegionEpoch {
        conf_ver: Default::default(),
        version: Default::default(),
    };
    repr.conf_ver = src.get_conf_ver();
    repr.version = src.get_version();
    arena.alloc_struct(repr)
}

pub fn region_epoch_from_repr_generated(src: *const MetapbRegionEpoch) -> Option<pb::RegionEpoch> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::RegionEpoch::new();
    out.set_conf_ver(repr.conf_ver);
    out.set_version(repr.version);
    Some(out)
}

pub fn store_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::Store) -> &'a mut MetapbStore {
    let mut repr = MetapbStore {
        id: Default::default(),
        address: KvprotoStringView { data: ptr::null(), len: 0 },
        state: Default::default(),
        labels: KvprotoSliceMetapbStoreLabelPtr { data: ptr::null_mut(), len: 0, cap: 0 },
        version: KvprotoStringView { data: ptr::null(), len: 0 },
        peer_address: KvprotoStringView { data: ptr::null(), len: 0 },
        status_address: KvprotoStringView { data: ptr::null(), len: 0 },
        git_hash: KvprotoStringView { data: ptr::null(), len: 0 },
        start_timestamp: Default::default(),
        deploy_path: KvprotoStringView { data: ptr::null(), len: 0 },
        last_heartbeat: Default::default(),
        physically_destroyed: Default::default(),
        node_state: Default::default(),
    };
    repr.id = src.get_id();
    if !src.get_address().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_address());
        repr.address.data = ptr as *const c_char;
        repr.address.len = len;
    }
    repr.state = src.get_state() as i32;
    {
        let values = src.get_labels();
        if !values.is_empty() {
            let mut ptrs: Vec<*mut MetapbStoreLabel> = Vec::with_capacity(values.len());
            for value in values.iter() {
                ptrs.push(store_label_to_repr_generated(arena, value) as *mut _);
            }
            if !ptrs.is_empty() {
                let (ptr, len) = arena.alloc_vec(ptrs);
                repr.labels.data = ptr;
                repr.labels.len = len;
                repr.labels.cap = len;
            }
        }
    }
    if !src.get_version().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_version());
        repr.version.data = ptr as *const c_char;
        repr.version.len = len;
    }
    if !src.get_peer_address().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_peer_address());
        repr.peer_address.data = ptr as *const c_char;
        repr.peer_address.len = len;
    }
    if !src.get_status_address().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_status_address());
        repr.status_address.data = ptr as *const c_char;
        repr.status_address.len = len;
    }
    if !src.get_git_hash().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_git_hash());
        repr.git_hash.data = ptr as *const c_char;
        repr.git_hash.len = len;
    }
    repr.start_timestamp = src.get_start_timestamp();
    if !src.get_deploy_path().is_empty() {
        let (ptr, len) = arena.alloc_string(src.get_deploy_path());
        repr.deploy_path.data = ptr as *const c_char;
        repr.deploy_path.len = len;
    }
    repr.last_heartbeat = src.get_last_heartbeat();
    repr.physically_destroyed = src.get_physically_destroyed();
    repr.node_state = src.get_node_state() as i32;
    arena.alloc_struct(repr)
}

pub fn store_from_repr_generated(src: *const MetapbStore) -> Option<pb::Store> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::Store::new();
    out.set_id(repr.id);
    out.set_address(string_from(repr.address.data as *const u8, repr.address.len));
    out.set_state(pb::StoreState::from_i32(repr.state).unwrap_or_default());
    if !repr.labels.data.is_null() && repr.labels.len > 0 {
        let slice = unsafe { std::slice::from_raw_parts(repr.labels.data, repr.labels.len) };
        let mut values: Vec<pb::StoreLabel> = Vec::with_capacity(slice.len());
        for &ptr in slice {
            if ptr.is_null() {
                continue;
            }
            if let Some(value) = store_label_from_repr_generated(ptr) {
                values.push(value);
            }
        }
        if !values.is_empty() {
            out.set_labels(::protobuf::RepeatedField::from_vec(values));
        }
    }
    out.set_version(string_from(repr.version.data as *const u8, repr.version.len));
    out.set_peer_address(string_from(repr.peer_address.data as *const u8, repr.peer_address.len));
    out.set_status_address(string_from(repr.status_address.data as *const u8, repr.status_address.len));
    out.set_git_hash(string_from(repr.git_hash.data as *const u8, repr.git_hash.len));
    out.set_start_timestamp(repr.start_timestamp);
    out.set_deploy_path(string_from(repr.deploy_path.data as *const u8, repr.deploy_path.len));
    out.set_last_heartbeat(repr.last_heartbeat);
    out.set_physically_destroyed(repr.physically_destroyed);
    out.set_node_state(pb::NodeState::from_i32(repr.node_state).unwrap_or_default());
    Some(out)
}

pub fn store_label_to_repr_generated<'a>(arena: &'a mut Arena, src: &pb::StoreLabel) -> &'a mut MetapbStoreLabel {
    let mut repr = MetapbStoreLabel {
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

pub fn store_label_from_repr_generated(src: *const MetapbStoreLabel) -> Option<pb::StoreLabel> {
    if src.is_null() {
        return None;
    }
    let repr = unsafe { &*src };
    let mut out = pb::StoreLabel::new();
    out.set_key(string_from(repr.key.data as *const u8, repr.key.len));
    out.set_value(string_from(repr.value.data as *const u8, repr.value.len));
    Some(out)
}

