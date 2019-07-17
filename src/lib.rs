pub mod api;
pub mod dxgi;
pub mod result;
mod utility;

pub trait Interface {
    type APIType: winapi::Interface;
    fn uuidof() -> api::Guid;
    fn as_com_ptr(&self) -> &com_ptr::ComPtr<Self::APIType>;
    fn as_unknown(&self) -> *mut winapi::um::unknwnbase::IUnknown;
    fn from_com_ptr(p: com_ptr::ComPtr<Self::APIType>) -> Self;
}
