#[cfg(feature = "kvffi_gen")]
#[path = "kvrpcpb_conv_gen.rs"]
mod kvrpcpb_conv_gen;
#[cfg(feature = "kvffi_gen")]
pub use kvrpcpb_conv_gen::*;
