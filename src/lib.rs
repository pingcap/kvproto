#[cfg(feature = "protobuf-codec")]
#[allow(bare_trait_objects)]
mod protobuf;
mod text;

#[cfg(feature = "protobuf-codec")]
pub use crate::protobuf::*;

#[cfg(feature = "prost-codec")]
pub use crate::prost::*;
