//! (unstable) A thin Rust wrapper around D3D1, D3D12 and DXGI.

pub mod api;
pub mod d2d1;
pub mod d3d;
#[cfg(feature = "d3d11")]
pub mod d3d11;
#[cfg(all(feature = "d3d11on12", feature = "d3d11", feature = "d3d12"))]
pub mod d3d11on12;
#[cfg(feature = "d3d11sdklayers")]
mod d3d11sdklayers;
#[cfg(feature = "d3d12")]
pub mod d3d12;
#[cfg(feature = "d3d12sdklayers")]
mod d3d12sdklayers;
#[cfg(feature = "d3dcompiler")]
pub mod d3dcompiler;
#[cfg(feature = "dwrite")]
pub mod dwrite;
pub mod dxgi;
pub mod result;
mod utility;

#[doc(inline)]
pub use api::{EventHandle, Guid, Luid, Point, Rect, WindowHandle};
#[doc(inline)]
pub use result::HResult;

use winapi::um::unknwnbase::IUnknown;

/// Defines the `IUnknown` interface and utility methods for Rust.
pub trait Interface {
    type APIType: winapi::Interface;
    fn new(p: com_ptr::ComPtr<Self::APIType>) -> Self;
    fn uuidof() -> api::Guid;
    fn as_ptr(&self) -> *mut Self::APIType;
    fn as_com_ptr(&self) -> &com_ptr::ComPtr<Self::APIType>;
    fn as_unknown(&self) -> *mut IUnknown;
    fn from_com_ptr(p: com_ptr::ComPtr<Self::APIType>) -> Self;
    fn query_interface<T: Interface>(&self) -> Result<T, result::HResult>;
}

#[derive(Clone, Debug)]
pub struct Unknown(com_ptr::ComPtr<IUnknown>);
impl_interface!(Unknown, IUnknown);

impl Unknown {
    pub fn from_interface(interface: &impl Interface) -> Unknown {
        Unknown(
            interface
                .as_com_ptr()
                .query_interface::<IUnknown>()
                .unwrap(),
        )
    }
}
