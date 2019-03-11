#[cfg(feature = "protobuf-codec")]
pub use crate::protobuf::*;
#[cfg(feature = "prost-codec")]
pub use crate::prost::*;

#[cfg(feature = "prost-codec")]
mod prost;
#[cfg(feature = "protobuf-codec")]
mod protobuf;
