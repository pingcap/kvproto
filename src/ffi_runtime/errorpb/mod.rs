#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
#[path = "errorpb_conv_gen.rs"]
mod errorpb_conv_gen;
#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
pub use errorpb_conv_gen::*;
