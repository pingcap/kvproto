#[allow(dead_code)]
#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(renamed_and_removed_lints)]
#[allow(bare_trait_objects)]
mod protos {
    include!(concat!(env!("OUT_DIR"), "/protos/mod.rs"));

    use raft_proto::eraftpb;
}

pub use protos::*;

#[cfg(feature = "prost-codec")]
pub mod prost_adapt {
    use crate::backup::{error, ClusterIdError, Error};
    use crate::import_kvpb::{write_engine_request, WriteBatch, WriteEngineRequest, WriteHead};
    use crate::import_sstpb::{upload_request, SstMeta, UploadRequest};
    use crate::{errorpb, kvrpcpb};

    impl UploadRequest {
        pub fn set_data(&mut self, v: Vec<u8>) {
            self.chunk = Some(upload_request::Chunk::Data(v));
        }
        pub fn get_data(&self) -> &[u8] {
            match &self.chunk {
                Some(upload_request::Chunk::Data(v)) => v,
                _ => &[],
            }
        }
        pub fn set_meta(&mut self, v: SstMeta) {
            self.chunk = Some(upload_request::Chunk::Meta(v));
        }
        pub fn get_meta(&self) -> &SstMeta {
            match &self.chunk {
                Some(upload_request::Chunk::Meta(v)) => v,
                _ => SstMeta::default_ref(),
            }
        }
        pub fn has_meta(&self) -> bool {
            match self.chunk {
                Some(upload_request::Chunk::Meta(_)) => true,
                _ => false,
            }
        }
    }

    impl WriteEngineRequest {
        pub fn set_head(&mut self, v: WriteHead) {
            self.chunk = Some(write_engine_request::Chunk::Head(v));
        }
        pub fn get_head(&self) -> &WriteHead {
            match &self.chunk {
                Some(write_engine_request::Chunk::Head(v)) => v,
                _ => WriteHead::default_ref(),
            }
        }
        pub fn has_head(&self) -> bool {
            match self.chunk {
                Some(write_engine_request::Chunk::Head(_)) => true,
                _ => false,
            }
        }
        pub fn set_batch(&mut self, v: WriteBatch) {
            self.chunk = Some(write_engine_request::Chunk::Batch(v));
        }
        pub fn get_batch(&self) -> &WriteBatch {
            match &self.chunk {
                Some(write_engine_request::Chunk::Batch(v)) => v,
                _ => WriteBatch::default_ref(),
            }
        }
        pub fn has_batch(&self) -> bool {
            match self.chunk {
                Some(write_engine_request::Chunk::Batch(_)) => true,
                _ => false,
            }
        }
        pub fn take_batch(&mut self) -> WriteBatch {
            if self.has_batch() {
                match self.chunk.take() {
                    Some(write_engine_request::Chunk::Batch(v)) => v,
                    _ => unreachable!(),
                }
            } else {
                WriteBatch::default()
            }
        }
    }

    impl Error {
        pub fn set_region_error(&mut self, v: errorpb::Error) {
            self.detail = Some(error::Detail::RegionError(v));
        }

        pub fn set_kv_error(&mut self, v: kvrpcpb::KeyError) {
            self.detail = Some(error::Detail::KvError(v));
        }

        pub fn set_cluster_id_error(&mut self, v: ClusterIdError) {
            self.detail = Some(error::Detail::ClusterIdError(v));
        }

        pub fn get_region_error(&self) -> &errorpb::Error {
            match &self.detail {
                Some(error::Detail::RegionError(v)) => v,
                _ => errorpb::Error::default_ref(),
            }
        }

        pub fn get_kv_error(&self) -> &kvrpcpb::KeyError {
            match &self.detail {
                Some(error::Detail::KvError(v)) => v,
                _ => kvrpcpb::KeyError::default_ref(),
            }
        }

        pub fn get_cluster_id_error(&self) -> &ClusterIdError {
            match &self.detail {
                Some(error::Detail::ClusterIdError(v)) => v,
                _ => ClusterIdError::default_ref(),
            }
        }

        pub fn has_region_error(&self) -> bool {
            match self.detail {
                Some(error::Detail::RegionError(_)) => true,
                _ => false,
            }
        }

        pub fn has_kv_error(&self) -> bool {
            match self.detail {
                Some(error::Detail::KvError(_)) => true,
                _ => false,
            }
        }

        pub fn has_cluster_id_error(&self) -> bool {
            match self.detail {
                Some(error::Detail::ClusterIdError(_)) => true,
                _ => false,
            }
        }

        pub fn mut_region_error(&mut self) -> &mut errorpb::Error {
            if let Some(error::Detail::RegionError(_)) = self.detail {
            } else {
                self.detail = Some(error::Detail::RegionError(errorpb::Error::default()));
            }
            match self.detail {
                Some(error::Detail::RegionError(ref mut v)) => v,
                _ => unreachable!(),
            }
        }

        pub fn mut_kv_error(&mut self) -> &mut kvrpcpb::KeyError {
            if let Some(error::Detail::KvError(_)) = self.detail {
            } else {
                self.detail = Some(error::Detail::KvError(kvrpcpb::KeyError::default()));
            }
            match self.detail {
                Some(error::Detail::KvError(ref mut v)) => v,
                _ => unreachable!(),
            }
        }

        pub fn mut_cluster_id_error(&mut self) -> &mut ClusterIdError {
            if let Some(error::Detail::ClusterIdError(_)) = self.detail {
            } else {
                self.detail = Some(error::Detail::ClusterIdError(ClusterIdError::default()));
            }
            match self.detail {
                Some(error::Detail::ClusterIdError(ref mut v)) => v,
                _ => unreachable!(),
            }
        }
    }
}
