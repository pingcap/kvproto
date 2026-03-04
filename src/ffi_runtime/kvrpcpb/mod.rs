#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
#[path = "kvrpcpb_conv_gen.rs"]
mod kvrpcpb_conv_gen;
#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
pub use kvrpcpb_conv_gen::*;
