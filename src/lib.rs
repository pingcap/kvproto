#[cfg(lib_rust_protobuf)]
mod protobuf;

#[cfg(lib_rust_protobuf)]
pub use crate::protobuf::*;

#[cfg(lib_prost)]
pub use crate::prost::*;
