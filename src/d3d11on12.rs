#![cfg(feature = "d3d11on12")]

use winapi::um::d3d11on12::*;
use crate::impl_interface;
use crate::Interface;
use crate::d3d;
use crate::d3d11;
use crate::d3d12;
use crate::result::{HResult, hresult};
use winapi::Interface as _;
use com_ptr::ComPtr;
use winapi::um::d3d11::*;
use winapi::um::unknwnbase::IUnknown;

pub struct D3D11ResourceFlags {
    pub bind_flags: d3d11::BindFlags,
    pub misc_flags: Option<d3d11::ResourceMiscFlags>,
    pub cpu_access_flags: Option<d3d11::CPUAccessFlags>,
    pub structure_byte_stride: u32,
}
impl D3D11ResourceFlags {
    fn to_c_struct(&self) -> D3D11_RESOURCE_FLAGS {
        D3D11_RESOURCE_FLAGS {
            BindFlags: self.bind_flags.0,
            MiscFlags: self.misc_flags.map_or(0, |f| f.0),
            CPUAccessFlags: self.cpu_access_flags.map_or(0, |f| f.0),
            StructureByteStride: self.structure_byte_stride,
        }
    }
}

pub trait IDevice: Interface {
    fn acquire_wrapped_resources(&self, resources: &[&impl d3d11::IResource]);
    fn create_wrapped_resource(
        &self,
        resource12: &impl d3d12::IResource,
        flags11: D3D11ResourceFlags,
        in_state: d3d12::ResourceStates,
        out_state: d3d12::ResourceStates
    ) -> Result<d3d11::Resource, HResult>;
    fn release_wrapped_resources(&self, resources: &[&impl d3d11::IResource]);
}
pub trait IDevice1: IDevice {
    fn get_d3d12_device<T: d3d12::IDevice>(&self) -> Result<T, HResult>;
}
#[derive(Clone, Debug)]
pub struct Device(ComPtr<ID3D11On12Device>);
/*
#[derive(Clone, Debug)]
pub struct Device1(ComPtr<ID3D11On12Device1>);
*/
macro_rules! impl_device {
    ($s: ident, $interface: ident, Device) => {
        impl_interface!($s, $interface);
        impl IDevice for $s {
            fn acquire_wrapped_resources(&self, resources: &[&impl d3d11::IResource]) {
                unsafe {
                    let mut c_resources = resources.iter().map(|r| r.as_ptr()).collect::<Vec<_>>();
                    self.0.AcquireWrappedResources(c_resources.as_mut_ptr() as *mut *mut ID3D11Resource, c_resources.len() as u32);
                }
            }
            fn create_wrapped_resource(
                &self,
                resource12: &impl d3d12::IResource,
                flags11: D3D11ResourceFlags,
                in_state: d3d12::ResourceStates,
                out_state: d3d12::ResourceStates,
            ) -> Result<d3d11::Resource, HResult> {
                Ok(d3d11::Resource::new(ComPtr::new(|| {
                    let mut p = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateWrappedResource(
                            resource12.as_ptr() as *mut IUnknown,
                            &flags11.to_c_struct(),
                            in_state.0,
                            out_state.0,
                            &d3d11::Resource::uuidof().into(),
                            &mut p
                        )
                    };
                    hresult(p as *mut ID3D11Resource, res)
                })?))
            }
            fn release_wrapped_resources(&self, resources: &[&impl d3d11::IResource]) {
                unsafe {
                    let mut c_resources = resources.iter().map(|r| r.as_ptr()).collect::<Vec<_>>();
                    self.0.ReleaseWrappedResources(c_resources.as_mut_ptr() as *mut *mut ID3D11Resource, c_resources.len() as u32);
                }
            }
        }
    };
    ($s: ident, $interface: ident, Device1) => {
        impl_interface!($s, $interface);
        pub impl IDevice1 for $s {
            fn get_d3d12_device<T: d3d12::IDevice>(&self) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut p = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.GetD3D12Device(
                            &T::uuidof().into(),
                            &mut p
                        )
                    };
                    hresult(p as *mut <T as Interface>::APIType, res)
                })?))
            }
        }
    };
}
impl_device!(Device, ID3D11On12Device, Device);

pub fn create_device(
    device12: &impl d3d12::IDevice,
    flags: Option<d3d11::CreateDeviceFlags>,
    feature_levels: &[d3d::FeatureLevel],
    command_queues: &[&impl d3d12::ICommandQueue],
    node_mask: u32
) -> Result<(d3d11::Device, d3d11::DeviceContext, d3d::FeatureLevel), HResult> {
    unsafe {
        let mut device = std::ptr::null_mut();
        let mut device_context = std::ptr::null_mut();
        let mut feature_level = 0;
        let c_feature_levels = feature_levels.iter().map(|&l| l.into()).collect::<Vec<_>>();
        let mut c_command_queues = command_queues.iter().map(|c| c.as_ptr() as *mut IUnknown).collect::<Vec<_>>();
        let res = D3D11On12CreateDevice(
            device12.as_ptr() as *mut IUnknown,
            flags.map_or(0, |f| f.0),
            c_feature_levels.as_ptr(),
            c_feature_levels.len() as u32,
            c_command_queues.as_mut_ptr(),
            c_command_queues.len() as u32,
            node_mask,
            &mut device,
            &mut device_context,
            &mut feature_level,
        );
        if res < 0 {
            return Err(res.into());
        }
        Ok((
            d3d11::Device::new(ComPtr::from_raw(device)),
            d3d11::DeviceContext::new(ComPtr::from_raw(device_context)),
            feature_level.into(),
        ))
    }
}
