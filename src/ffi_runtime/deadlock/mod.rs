#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
#[path = "deadlock_conv_gen.rs"]
mod deadlock_conv_gen;
#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
pub use deadlock_conv_gen::*;
