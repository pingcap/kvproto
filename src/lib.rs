<<<<<<< HEAD
mod protobuf;

#[cfg(lib_rust_protobuf)]
=======
#[cfg(feature = "protobuf-codec")]
mod protobuf;

#[cfg(feature = "protobuf-codec")]
>>>>>>> master
pub use crate::protobuf::*;

#[cfg(feature = "prost-codec")]
pub use crate::prost::*;
