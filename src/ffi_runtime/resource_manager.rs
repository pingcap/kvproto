#[cfg(feature = "kvffi_gen")]
#[path = "resource_manager/resource_manager_conv_gen.rs"]
mod resource_manager_conv_gen;
#[cfg(feature = "kvffi_gen")]
pub use resource_manager_conv_gen::*;
