pub mod api;
pub mod d3d;
pub mod d3d12;
mod d3d12sdklayers;
pub mod dxgi;
pub mod result;
mod utility;
pub mod d3dcompiler;

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
