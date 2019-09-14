#![cfg(feature = "d3dcompiler")]

use crate::d3d::{Blob, ShaderMacro};
use crate::impl_bitflag_operators;
use crate::result::{ErrorMessage, ErrorMessageObject};
use com_ptr::ComPtr;
use winapi::ctypes::c_void;
use winapi::um::d3dcompiler::*;

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

pub fn compile(
    src_data: &[u8],
    src_name: Option<&str>,
    macros: Option<&[ShaderMacro]>,
    entry_point: &str,
    target: &str,
    flags1: Option<CompileFlags>,
    flags2: Option<CompileEffectFlags>,
) -> Result<Blob, ErrorMessage> {
    let c_src_name = src_name.map(|name| std::ffi::CString::new(name).unwrap());
    let c_macros: Option<(Vec<_>, Vec<_>)> =
        macros.map(|ms| ms.iter().map(|m| m.to_c_struct()).unzip());
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
            D3D_COMPILE_STANDARD_FILE_INCLUDE,
            c_entry_point.as_ptr(),
            c_target.as_ptr(),
            flags1.map_or(0, |f| f.0),
            flags2.map_or(0, |f| f.0),
            &mut blob,
            &mut err_blob,
        );
        if res < 0 {
            Err(ErrorMessageObject::new(res.into(), err_blob).into())
        } else {
            Ok(Blob(ComPtr::from_raw(blob)))
        }
    }
}
