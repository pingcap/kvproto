#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
#[path = "resource_manager/resource_manager_conv_gen.rs"]
mod resource_manager_conv_gen;
#[cfg(all(feature = "kvffi_gen", feature = "protobuf-codec"))]
pub use resource_manager_conv_gen::*;
