#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
#[path = "metapb/metapb_conv_gen.rs"]
mod metapb_conv_gen;
#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
pub use metapb_conv_gen::*;
