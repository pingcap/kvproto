extern crate futures;
extern crate grpcio;
extern crate bytes;
extern crate prost;
#[macro_use]
extern crate prost_derive;
extern crate raft;
use raft::eraftpb;
pub mod raft_serverpb { include!(concat!(env!("OUT_DIR"), "/raft_serverpb.rs")); }
pub mod errorpb { include!(concat!(env!("OUT_DIR"), "/errorpb.rs")); }
pub mod metapb { include!(concat!(env!("OUT_DIR"), "/metapb.rs")); }
pub mod raft_cmdpb { include!(concat!(env!("OUT_DIR"), "/raft_cmdpb.rs")); }
pub mod pdpb { include!(concat!(env!("OUT_DIR"), "/pdpb.rs")); }
pub mod kvrpcpb { include!(concat!(env!("OUT_DIR"), "/kvrpcpb.rs")); }
pub mod tikvpb { include!(concat!(env!("OUT_DIR"), "/tikvpb.rs")); }
pub mod debugpb { include!(concat!(env!("OUT_DIR"), "/debugpb.rs")); }
pub mod coprocessor { include!(concat!(env!("OUT_DIR"), "/coprocessor.rs")); }
pub mod import_sstpb { include!(concat!(env!("OUT_DIR"), "/import_sstpb.rs")); }
pub mod import_kvpb { include!(concat!(env!("OUT_DIR"), "/import_kvpb.rs")); }
