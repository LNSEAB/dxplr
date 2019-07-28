use winapi::um::d3d12sdklayers::*;
use winapi::um::unknwnbase::*;
use winapi::Interface as _;
use crate::Interface;
use crate::impl_interface;
use com_ptr::ComPtr;

pub trait IDebug: Interface {
    fn enable_debug_layer(&self);
}
macro_rules! impl_debug {
    ($s: ident, $interface: ident, Debug) => {
        impl_interface!($s, $interface);
        impl IDebug for $s {
            fn enable_debug_layer(&self) {
                unsafe { self.0.EnableDebugLayer() }
            }
        }
    };
}
pub struct Debug(ComPtr<ID3D12Debug>);
impl_debug!(Debug, ID3D12Debug, Debug);
