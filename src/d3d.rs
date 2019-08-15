use crate::impl_interface;
use crate::Interface;
use com_ptr::ComPtr;
use winapi::ctypes::c_void;
use winapi::um::d3d12::{
    D3D_ROOT_SIGNATURE_VERSION, D3D_ROOT_SIGNATURE_VERSION_1_0, D3D_ROOT_SIGNATURE_VERSION_1_1,
    D3D_SHADER_MODEL,
};
use winapi::um::d3dcommon::*;
use winapi::Interface as _;

#[cfg(feature = "d3dcompiler")]
pub use crate::d3dcompiler::*;

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum DriverType {
    Unknown = D3D_DRIVER_TYPE_UNKNOWN,
    Hardware = D3D_DRIVER_TYPE_HARDWARE,
    Reference = D3D_DRIVER_TYPE_REFERENCE,
    Null = D3D_DRIVER_TYPE_NULL,
    Software = D3D_DRIVER_TYPE_SOFTWARE,
    Warp = D3D_DRIVER_TYPE_WARP,
}

#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum PrimitiveTopology {
    Undefined = D3D_PRIMITIVE_TOPOLOGY_UNDEFINED,
    PointList = D3D_PRIMITIVE_TOPOLOGY_POINTLIST,
    LineList = D3D_PRIMITIVE_TOPOLOGY_LINELIST,
    LineStrip = D3D_PRIMITIVE_TOPOLOGY_LINESTRIP,
    TriangleList = D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST,
    TriangleStrip = D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP,
    LineListAdj = D3D_PRIMITIVE_TOPOLOGY_LINELIST_ADJ,
    LineStripAdj = D3D_PRIMITIVE_TOPOLOGY_LINESTRIP_ADJ,
    TriangleListAdj = D3D_PRIMITIVE_TOPOLOGY_TRIANGLELIST_ADJ,
    TriangleStripAdj = D3D_PRIMITIVE_TOPOLOGY_TRIANGLESTRIP_ADJ,
    ControlPointPatchlist1 = D3D_PRIMITIVE_TOPOLOGY_1_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist2 = D3D_PRIMITIVE_TOPOLOGY_2_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist3 = D3D_PRIMITIVE_TOPOLOGY_3_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist4 = D3D_PRIMITIVE_TOPOLOGY_4_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist5 = D3D_PRIMITIVE_TOPOLOGY_5_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist6 = D3D_PRIMITIVE_TOPOLOGY_6_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist7 = D3D_PRIMITIVE_TOPOLOGY_7_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist8 = D3D_PRIMITIVE_TOPOLOGY_8_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist9 = D3D_PRIMITIVE_TOPOLOGY_9_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist10 = D3D_PRIMITIVE_TOPOLOGY_10_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist11 = D3D_PRIMITIVE_TOPOLOGY_11_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist12 = D3D_PRIMITIVE_TOPOLOGY_12_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist13 = D3D_PRIMITIVE_TOPOLOGY_13_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist14 = D3D_PRIMITIVE_TOPOLOGY_14_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist15 = D3D_PRIMITIVE_TOPOLOGY_15_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist16 = D3D_PRIMITIVE_TOPOLOGY_16_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist17 = D3D_PRIMITIVE_TOPOLOGY_17_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist18 = D3D_PRIMITIVE_TOPOLOGY_18_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist19 = D3D_PRIMITIVE_TOPOLOGY_19_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist20 = D3D_PRIMITIVE_TOPOLOGY_20_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist21 = D3D_PRIMITIVE_TOPOLOGY_21_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist22 = D3D_PRIMITIVE_TOPOLOGY_22_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist23 = D3D_PRIMITIVE_TOPOLOGY_23_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist24 = D3D_PRIMITIVE_TOPOLOGY_24_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist25 = D3D_PRIMITIVE_TOPOLOGY_25_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist26 = D3D_PRIMITIVE_TOPOLOGY_26_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist27 = D3D_PRIMITIVE_TOPOLOGY_27_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist28 = D3D_PRIMITIVE_TOPOLOGY_28_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist29 = D3D_PRIMITIVE_TOPOLOGY_29_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist30 = D3D_PRIMITIVE_TOPOLOGY_30_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist31 = D3D_PRIMITIVE_TOPOLOGY_31_CONTROL_POINT_PATCHLIST,
    ControlPointPatchlist32 = D3D_PRIMITIVE_TOPOLOGY_32_CONTROL_POINT_PATCHLIST,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FeatureLevel(pub u32, pub u32);
impl From<FeatureLevel> for D3D_FEATURE_LEVEL {
    fn from(src: FeatureLevel) -> D3D_FEATURE_LEVEL {
        (src.0 << 12) | (src.1 << 8)
    }
}
impl From<D3D_FEATURE_LEVEL> for FeatureLevel {
    fn from(src: D3D_FEATURE_LEVEL) -> FeatureLevel {
        FeatureLevel((src >> 12) & 0xf, (src >> 8) & 0xf)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RootSignatureVersion(pub u32, pub u32);
impl From<RootSignatureVersion> for D3D_ROOT_SIGNATURE_VERSION {
    fn from(src: RootSignatureVersion) -> D3D_ROOT_SIGNATURE_VERSION {
        match src {
            RootSignatureVersion(1, 0) => D3D_ROOT_SIGNATURE_VERSION_1_0,
            RootSignatureVersion(1, 1) => D3D_ROOT_SIGNATURE_VERSION_1_1,
            _ => unimplemented!(),
        }
    }
}
impl From<D3D_ROOT_SIGNATURE_VERSION> for RootSignatureVersion {
    fn from(src: D3D_ROOT_SIGNATURE_VERSION) -> RootSignatureVersion {
        match src {
            D3D_ROOT_SIGNATURE_VERSION_1_0 => RootSignatureVersion(1, 0),
            D3D_ROOT_SIGNATURE_VERSION_1_1 => RootSignatureVersion(1, 1),
            _ => unimplemented!(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ShaderModel(pub u32, pub u32);
impl From<ShaderModel> for D3D_SHADER_MODEL {
    fn from(src: ShaderModel) -> D3D_SHADER_MODEL {
        (src.0 << 4) | src.1
    }
}
impl From<D3D_SHADER_MODEL> for ShaderModel {
    fn from(src: D3D_SHADER_MODEL) -> ShaderModel {
        ShaderModel((src >> 4) & 0xf, src & 0xf)
    }
}

#[derive(Clone, Debug)]
pub struct ShaderMacro<'a, 'b> {
    name: &'a str,
    definition: &'b str,
}
impl<'a, 'b> ShaderMacro<'a, 'b> {
    // This function is used in d3dcompiler.rs.
    #[allow(dead_code)]
    pub(crate) fn to_c_struct(&self) -> (D3D_SHADER_MACRO, (std::ffi::CString, std::ffi::CString)) {
        let name = std::ffi::CString::new(self.name).unwrap();
        let definition = std::ffi::CString::new(self.name).unwrap();
        (
            D3D_SHADER_MACRO {
                Name: name.as_ptr(),
                Definition: definition.as_ptr(),
            },
            (name, definition),
        )
    }
}

pub trait IBlob: Interface {
    fn get_buffer_pointer(&self) -> *const c_void;
    fn get_buffer_pointer_mut(&mut self) -> *mut c_void;
    fn get_buffer_size(&self) -> usize;
    fn as_slice(&self) -> &[u8];
    fn as_mut_slice(&mut self) -> &mut [u8];
    fn to_vec(&self) -> Vec<u8>;
    fn as_cstr(&self) -> Result<&std::ffi::CStr, std::ffi::FromBytesWithNulError>;
}
#[derive(Clone, Debug)]
pub struct Blob(pub(crate) ComPtr<ID3DBlob>);
impl_interface!(Blob, ID3DBlob);
impl IBlob for Blob {
    fn get_buffer_pointer(&self) -> *const c_void {
        unsafe { self.0.GetBufferPointer() }
    }
    fn get_buffer_pointer_mut(&mut self) -> *mut c_void {
        unsafe { self.0.GetBufferPointer() }
    }
    fn get_buffer_size(&self) -> usize {
        unsafe { self.0.GetBufferSize() }
    }
    fn as_slice(&self) -> &[u8] {
        unsafe {
            std::slice::from_raw_parts(
                self.get_buffer_pointer() as *const u8,
                self.get_buffer_size(),
            )
        }
    }
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            std::slice::from_raw_parts_mut(
                self.get_buffer_pointer_mut() as *mut u8,
                self.get_buffer_size(),
            )
        }
    }
    fn to_vec(&self) -> Vec<u8> {
        self.as_slice().to_vec()
    }
    fn as_cstr(&self) -> Result<&std::ffi::CStr, std::ffi::FromBytesWithNulError> {
        std::ffi::CStr::from_bytes_with_nul(self.as_slice())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use winapi::um::d3d12::D3D_SHADER_MODEL_5_1;

    #[test]
    fn feature_level_test() {
        assert_eq!(
            D3D_FEATURE_LEVEL::from(FeatureLevel(12, 1)),
            D3D_FEATURE_LEVEL_12_1
        );
        assert_eq!(FeatureLevel(12, 1), D3D_FEATURE_LEVEL_12_1.into());
    }
    #[test]
    fn root_signature_version_test() {
        assert_eq!(
            D3D_ROOT_SIGNATURE_VERSION::from(RootSignatureVersion(1, 1)),
            D3D_ROOT_SIGNATURE_VERSION_1_1
        );
        assert_eq!(
            RootSignatureVersion(1, 1),
            D3D_ROOT_SIGNATURE_VERSION_1_1.into()
        );
    }
    #[test]
    fn shader_model_test() {
        assert_eq!(
            D3D_SHADER_MODEL::from(ShaderModel(5, 1)),
            D3D_SHADER_MODEL_5_1
        );
        assert_eq!(ShaderModel(5, 1), D3D_SHADER_MODEL_5_1.into());
    }
}
