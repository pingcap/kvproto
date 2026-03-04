#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
#[path = "encryptionpb_conv_gen.rs"]
mod encryptionpb_conv_gen;
#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
pub use encryptionpb_conv_gen::*;
