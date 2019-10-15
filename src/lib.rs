//! (Experimental) A thin Rust wrapper around D3D1, D3D12 and DXGI.

pub mod api;
pub mod d3d;
pub mod d3d11;
pub mod d3d12;
mod d3d12sdklayers;
pub mod d3dcompiler;
pub mod dxgi;
pub mod result;
mod utility;

#[doc(inline)]
pub use api::{EventHandle, Guid, Luid, Point, Rect, WindowHandle};
#[doc(inline)]
pub use result::HResult;

/// Defines the `IUnknown` interface and utility methods for Rust.
pub trait Interface {
    type APIType: winapi::Interface;
    fn new(p: com_ptr::ComPtr<Self::APIType>) -> Self;
    fn uuidof() -> api::Guid;
    fn as_ptr(&self) -> *mut Self::APIType;
    fn as_com_ptr(&self) -> &com_ptr::ComPtr<Self::APIType>;
    fn as_unknown(&self) -> *mut winapi::um::unknwnbase::IUnknown;
    fn from_com_ptr(p: com_ptr::ComPtr<Self::APIType>) -> Self;
    fn query_interface<T: Interface>(&self) -> Result<T, result::HResult>;
}
