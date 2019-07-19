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
    use crate::import_kvpb::{write_engine_request, WriteBatch, WriteEngineRequest, WriteHead};
    use crate::import_sstpb::{upload_request, SstMeta, UploadRequest};

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
}
