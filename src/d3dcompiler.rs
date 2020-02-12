#![cfg(feature = "d3dcompiler")]

use crate::d3d::{Blob, IInclude, IncludeType, ShaderMacro};
use crate::impl_bitflag_operators;
use crate::result::ErrorMessage;
use com_ptr::ComPtr;
use winapi::ctypes::c_void;
use winapi::shared::minwindef::LPCVOID;
use winapi::shared::winerror::*;
use winapi::um::d3dcommon::{ID3DInclude, ID3DIncludeVtbl, D3D_INCLUDE_LOCAL, D3D_INCLUDE_SYSTEM};
use winapi::um::d3dcompiler::*;
use winapi::um::stringapiset::MultiByteToWideChar;
use winapi::um::winnls::CP_OEMCP;
use winapi::um::winnt::{HRESULT, LPCSTR};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CompileFlags(u32);
impl_bitflag_operators!(CompileFlags);
#[allow(non_upper_case_globals)]
impl CompileFlags {
    pub const Debug: Self = Self(D3DCOMPILE_DEBUG);
    pub const SkipValidation: Self = Self(D3DCOMPILE_SKIP_VALIDATION);
    pub const SkipOptimization: Self = Self(D3DCOMPILE_SKIP_OPTIMIZATION);
    pub const PackMatrixRowMajor: Self = Self(D3DCOMPILE_PACK_MATRIX_ROW_MAJOR);
    pub const PackMatrixColumnMajor: Self = Self(D3DCOMPILE_PACK_MATRIX_COLUMN_MAJOR);
    pub const PartialPrecision: Self = Self(D3DCOMPILE_PARTIAL_PRECISION);
    pub const ForceVSSoftwareNoOpt: Self = Self(D3DCOMPILE_FORCE_VS_SOFTWARE_NO_OPT);
    pub const ForcePSSoftwareNoOpt: Self = Self(D3DCOMPILE_FORCE_PS_SOFTWARE_NO_OPT);
    pub const NoPreshader: Self = Self(D3DCOMPILE_NO_PRESHADER);
    pub const AvoidFlowControl: Self = Self(D3DCOMPILE_AVOID_FLOW_CONTROL);
    pub const EnableStrictness: Self = Self(D3DCOMPILE_ENABLE_STRICTNESS);
    pub const EnableBackwardsCompatiblity: Self = Self(D3DCOMPILE_ENABLE_BACKWARDS_COMPATIBILITY);
    pub const IEEEStrictness: Self = Self(D3DCOMPILE_IEEE_STRICTNESS);
    pub const OptimizationLevel0: Self = Self(D3DCOMPILE_OPTIMIZATION_LEVEL0);
    pub const OptimizationLevel1: Self = Self(D3DCOMPILE_OPTIMIZATION_LEVEL1);
    pub const OptimizationLevel2: Self = Self(D3DCOMPILE_OPTIMIZATION_LEVEL2);
    pub const OptimizationLevel3: Self = Self(D3DCOMPILE_OPTIMIZATION_LEVEL3);
    pub const WarningsAreErrors: Self = Self(D3DCOMPILE_WARNINGS_ARE_ERRORS);
    pub const ResourcesMayAlias: Self = Self(D3DCOMPILE_RESOURCES_MAY_ALIAS);
    pub const UnboundedDescriptorTables: Self = Self(D3DCOMPILE_ENABLE_UNBOUNDED_DESCRIPTOR_TABLES);
    pub const AllResourcesBound: Self = Self(D3DCOMPILE_ALL_RESOURCES_BOUND);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CompileEffectFlags(u32);
impl_bitflag_operators!(CompileEffectFlags);
#[allow(non_upper_case_globals)]
impl CompileEffectFlags {
    pub const ChildEfect: Self = Self(D3DCOMPILE_EFFECT_CHILD_EFFECT);
    pub const AllowSlowOps: Self = Self(D3DCOMPILE_EFFECT_ALLOW_SLOW_OPS);
}

#[repr(C)]
struct IncludeObject {
    vtbl: *mut ID3DIncludeVtbl,
    obj: Box<dyn IInclude>,
    data: Vec<u8>,
}
impl IncludeObject {
    fn new(obj: Box<dyn IInclude>) -> Self {
        let vtbl = Box::new(ID3DIncludeVtbl {
            Open: Self::open,
            Close: Self::close,
        });
        Self {
            vtbl: Box::into_raw(vtbl),
            obj,
            data: Vec::new(),
        }
    }
    unsafe extern "system" fn open(
        this: *mut ID3DInclude,
        include_type: u32,
        filename: LPCSTR,
        parent_data: LPCVOID,
        pp_data: *mut LPCVOID,
        bytes: *mut u32,
    ) -> HRESULT {
        let p = this as *mut Self;
        let filename = {
            let sz =
                MultiByteToWideChar(CP_OEMCP, 0, filename, -1, std::ptr::null_mut(), 0) as usize;
            if sz == 0 {
                return E_FAIL;
            }
            let mut buf = Vec::with_capacity(sz);
            buf.set_len(sz);
            MultiByteToWideChar(
                CP_OEMCP,
                0,
                filename,
                -1,
                buf.as_mut_ptr(),
                buf.len() as i32,
            );
            buf.pop();

            match String::from_utf16(&buf) {
                Ok(s) => s,
                Err(_) => return E_FAIL,
            }
        };
        let parent_data = if parent_data == std::ptr::null() {
            None
        } else {
            Some(parent_data as *const std::ffi::c_void)
        };
        let mut data = Vec::new();
        let res = (*p).obj.open(
            match include_type {
                D3D_INCLUDE_LOCAL => IncludeType::Local,
                D3D_INCLUDE_SYSTEM => IncludeType::System,
                _ => unreachable!(),
            },
            &filename,
            parent_data,
            &mut data,
        );
        match res {
            Ok(_) => {
                (*pp_data) = data.as_ptr() as *const winapi::ctypes::c_void;
                (*bytes) = data.len() as u32;
                (*p).data = data;
                S_OK
            }
            Err(_) => E_FAIL,
        }
    }
    unsafe extern "system" fn close(_this: *mut ID3DInclude, _data: LPCVOID) -> HRESULT {
        S_OK
    }
}
impl Drop for IncludeObject {
    fn drop(&mut self) {
        unsafe {
            Box::from_raw(self.vtbl);
        }
    }
}

pub fn compile(
    src_data: &[u8],
    src_name: Option<&str>,
    macros: Option<&[ShaderMacro]>,
    include: Option<Box<dyn IInclude>>,
    entry_point: &str,
    target: &str,
    flags1: Option<CompileFlags>,
    flags2: Option<CompileEffectFlags>,
) -> Result<Blob, ErrorMessage> {
    let c_src_name = src_name.map(|name| std::ffi::CString::new(name).unwrap());
    let c_macros: Option<(Vec<_>, Vec<_>)> =
        macros.map(|ms| ms.iter().map(|m| m.to_c_struct()).unzip());
    let mut include_obj = include.map(|i| IncludeObject::new(i));
    let c_entry_point = std::ffi::CString::new(entry_point).unwrap();
    let c_target = std::ffi::CString::new(target).unwrap();
    let mut blob = std::ptr::null_mut();
    let mut err_blob = std::ptr::null_mut();
    unsafe {
        let res = D3DCompile(
            src_data.as_ptr() as *const c_void,
            src_data.len(),
            c_src_name
                .as_ref()
                .map_or(std::ptr::null(), |name| name.as_ptr()),
            if let Some((ms, _tmp)) = c_macros.as_ref() {
                ms.as_ptr()
            } else {
                std::ptr::null()
            },
            include_obj
                .as_mut()
                .map_or(D3D_COMPILE_STANDARD_FILE_INCLUDE, |i| {
                    i as *mut IncludeObject as *mut ID3DInclude
                }),
            c_entry_point.as_ptr(),
            c_target.as_ptr(),
            flags1.map_or(0, |f| f.0),
            flags2.map_or(0, |f| f.0),
            &mut blob,
            &mut err_blob,
        );
        if res < 0 {
            Err(ErrorMessage::new(res.into(), err_blob).into())
        } else {
            Ok(Blob(ComPtr::from_raw(blob)))
        }
    }
}
