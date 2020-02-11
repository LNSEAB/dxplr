use crate::d3d11::IDeviceContext;
use crate::dxgi::{ISwapChain, SwapChain};
use crate::result::{hresult, HResult};
use crate::{impl_bitflag_operators, impl_interface, Interface};
use com_ptr::ComPtr;
use winapi::shared::dxgi::*;
use winapi::um::d3d11::*;
use winapi::um::d3d11sdklayers::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DebugFeature(u32);
#[allow(non_upper_case_globals)]
impl DebugFeature {
    pub const FlushPerRenderOp: Self = Self(D3D11_DEBUG_FEATURE_FLUSH_PER_RENDER_OP);
    pub const FinishPerRenderOp: Self = Self(D3D11_DEBUG_FEATURE_FINISH_PER_RENDER_OP);
    pub const PresentPerRenderOp: Self = Self(D3D11_DEBUG_FEATURE_PRESENT_PER_RENDER_OP);
    pub const AlwaysDiscardOfferedResource: Self =
        Self(D3D11_DEBUG_FEATURE_ALWAYS_DISCARD_OFFERED_RESOURCE);
    pub const NeverDiscardOfferedResource: Self =
        Self(D3D11_DEBUG_FEATURE_NEVER_DISCARD_OFFERED_RESOURCE);
    pub const AvoidBehaviorChangingDebugAids: Self =
        Self(D3D11_DEBUG_FEATURE_AVOID_BEHAVIOR_CHANGING_DEBUG_AIDS);
    pub const DisableTiledResourceMappingTrackingAndValidation: Self =
        Self(D3D11_DEBUG_FEATURE_DISABLE_TILED_RESOURCE_MAPPING_TRACKING_AND_VALIDATION);
}
impl_bitflag_operators!(DebugFeature);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MessageCategory {
    ApplicationDefined = D3D11_MESSAGE_CATEGORY_APPLICATION_DEFINED,
    Miscellaneous = D3D11_MESSAGE_CATEGORY_MISCELLANEOUS,
    Initialization = D3D11_MESSAGE_CATEGORY_INITIALIZATION,
    Cleanup = D3D11_MESSAGE_CATEGORY_CLEANUP,
    Compilation = D3D11_MESSAGE_CATEGORY_COMPILATION,
    StateCreation = D3D11_MESSAGE_CATEGORY_STATE_CREATION,
    StateSetting = D3D11_MESSAGE_CATEGORY_STATE_SETTING,
    StateGetting = D3D11_MESSAGE_CATEGORY_STATE_GETTING,
    ResourceManipulation = D3D11_MESSAGE_CATEGORY_RESOURCE_MANIPULATION,
    Execution = D3D11_MESSAGE_CATEGORY_EXECUTION,
    Shader = D3D11_MESSAGE_CATEGORY_SHADER,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageID(pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum MessageSeverity {
    Corruption = D3D11_MESSAGE_SEVERITY_CORRUPTION,
    Error = D3D11_MESSAGE_SEVERITY_ERROR,
    Warning = D3D11_MESSAGE_SEVERITY_WARNING,
    Info = D3D11_MESSAGE_SEVERITY_INFO,
    Message = D3D11_MESSAGE_SEVERITY_MESSAGE,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct RLDOFlags(u32);
#[allow(non_upper_case_globals)]
impl RLDOFlags {
    pub const Summary: Self = Self(D3D11_RLDO_SUMMARY);
    pub const Detail: Self = Self(D3D11_RLDO_DETAIL);
    pub const IgnoreInternal: Self = Self(D3D11_RLDO_IGNORE_INTERNAL);
}
impl_bitflag_operators!(RLDOFlags);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ShaderTrackingOptions(u32);
#[allow(non_upper_case_globals)]
impl ShaderTrackingOptions {
    pub const Ignore: Self = Self(D3D11_SHADER_TRACKING_OPTION_IGNORE);
    pub const TrackUninitialized: Self = Self(D3D11_SHADER_TRACKING_OPTION_TRACK_UNINITIALIZED);
    pub const TrackRAW: Self = Self(D3D11_SHADER_TRACKING_OPTION_TRACK_RAW);
    pub const TrackWAR: Self = Self(D3D11_SHADER_TRACKING_OPTION_TRACK_WAR);
    pub const TrackWAW: Self = Self(D3D11_SHADER_TRACKING_OPTION_TRACK_WAW);
    pub const AllowSame: Self = Self(D3D11_SHADER_TRACKING_OPTION_ALLOW_SAME);
    pub const TrackAtomicConsistency: Self =
        Self(D3D11_SHADER_TRACKING_OPTION_TRACK_ATOMIC_CONSISTENCY);
    pub const TrackRAWAcrossThreadGroups: Self =
        Self(D3D11_SHADER_TRACKING_OPTION_TRACK_RAW_ACROSS_THREADGROUPS);
    pub const TrackWARAcrossThreadGroups: Self =
        Self(D3D11_SHADER_TRACKING_OPTION_TRACK_WAR_ACROSS_THREADGROUPS);
    pub const TrackWAWAcrossThreadGroups: Self =
        Self(D3D11_SHADER_TRACKING_OPTION_TRACK_WAW_ACROSS_THREADGROUPS);
    pub const TrackAtomicConsistencyAcrossThreadGroups: Self =
        Self(D3D11_SHADER_TRACKING_OPTION_TRACK_ATOMIC_CONSISTENCY_ACROSS_THREADGROUPS);
    pub const UAVSpecificFlags: Self = Self(D3D11_SHADER_TRACKING_OPTION_UAV_SPECIFIC_FLAGS);
    pub const AllHazards: Self = Self(D3D11_SHADER_TRACKING_OPTION_ALL_HAZARDS);
    pub const AllHazardsAllowingSame: Self =
        Self(D3D11_SHADER_TRACKING_OPTION_ALL_HAZARDS_ALLOWING_SAME);
    pub const AllOptions: Self = Self(D3D11_SHADER_TRACKING_OPTION_ALL_OPTIONS);
}
impl_bitflag_operators!(ShaderTrackingOptions);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ShaderTrackingResourceType {
    None = D3D11_SHADER_TRACKING_RESOURCE_TYPE_NONE,
    UAVDeviceMemory = D3D11_SHADER_TRACKING_RESOURCE_TYPE_UAV_DEVICEMEMORY,
    NonUAVDeviceMemory = D3D11_SHADER_TRACKING_RESOURCE_TYPE_NON_UAV_DEVICEMEMORY,
    AllDeviceMemory = D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL_DEVICEMEMORY,
    GroupSharedMemory = D3D11_SHADER_TRACKING_RESOURCE_TYPE_GROUPSHARED_MEMORY,
    AllSharedMemory = D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL_SHARED_MEMORY,
    GroupSharedNonUAV = D3D11_SHADER_TRACKING_RESOURCE_TYPE_GROUPSHARED_NON_UAV,
    All = D3D11_SHADER_TRACKING_RESOURCE_TYPE_ALL,
}

#[derive(Debug, Clone)]
pub struct InfoQueueFilter {
    allow_list: InfoQueueFilterDesc,
    deny_list: InfoQueueFilterDesc,
}

#[derive(Debug, Clone)]
pub struct InfoQueueFilterDesc {
    category_list: Vec<MessageCategory>,
    severity_list: Vec<MessageSeverity>,
    id_list: Vec<MessageID>,
}

#[derive(Debug, Clone)]
pub struct Message {
    category: MessageCategory,
    severity: MessageSeverity,
    id: MessageID,
    description: String,
}

pub trait IDebug: Interface {
    fn get_feature_mask(&self) -> Option<DebugFeature>;
    fn get_present_per_render_op_delay(&self) -> u32;
    fn get_swap_chain(&self) -> Result<SwapChain, HResult>;
    fn report_live_device_objects(&self, flags: RLDOFlags) -> Result<(), HResult>;
    fn set_feature_mask(&self, mask: Option<DebugFeature>) -> Result<(), HResult>;
    fn set_present_per_render_op_delay(&self, milliseconds: u32) -> Result<(), HResult>;
    fn set_swap_chain(&self, swap_chain: &impl ISwapChain) -> Result<(), HResult>;
    fn validate_context(&self, context: &impl IDeviceContext) -> Result<(), HResult>;
    fn validate_context_for_dispatch(&self, context: &impl IDeviceContext) -> Result<(), HResult>;
}
#[derive(Debug, Clone)]
pub struct Debug(ComPtr<ID3D11Debug>);
macro_rules! impl_debug {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IDebug for $s {
            fn get_feature_mask(&self) -> Option<DebugFeature> {
                unsafe {
                    let mask = self.0.GetFeatureMask();
                    if mask == 0 {
                        None
                    } else {
                        Some(DebugFeature(mask))
                    }
                }
            }
            fn get_present_per_render_op_delay(&self) -> u32 {
                unsafe { self.0.GetPresentPerRenderOpDelay() }
            }
            fn get_swap_chain(&self) -> Result<SwapChain, HResult> {
                unsafe {
                    Ok(SwapChain(ComPtr::new(|| {
                        let mut obj = std::ptr::null_mut();
                        let res = self.0.GetSwapChain(&mut obj);
                        hresult(obj, res)
                    })?))
                }
            }
            fn report_live_device_objects(&self, flags: RLDOFlags) -> Result<(), HResult> {
                unsafe {
                    let res = self.0.ReportLiveDeviceObjects(flags.0);
                    hresult((), res)
                }
            }
            fn set_feature_mask(&self, mask: Option<DebugFeature>) -> Result<(), HResult> {
                unsafe {
                    let res = self.0.SetFeatureMask(mask.map_or(0, |m| m.0));
                    hresult((), res)
                }
            }
            fn set_present_per_render_op_delay(&self, milliseconds: u32) -> Result<(), HResult> {
                unsafe {
                    let res = self.0.SetPresentPerRenderOpDelay(milliseconds);
                    hresult((), res)
                }
            }
            fn set_swap_chain(&self, swap_chain: &impl ISwapChain) -> Result<(), HResult> {
                unsafe {
                    let res = self
                        .0
                        .SetSwapChain(swap_chain.as_ptr() as *mut IDXGISwapChain);
                    hresult((), res)
                }
            }
            fn validate_context(&self, context: &impl IDeviceContext) -> Result<(), HResult> {
                unsafe {
                    let res = self
                        .0
                        .ValidateContext(context.as_ptr() as *mut ID3D11DeviceContext);
                    hresult((), res)
                }
            }
            fn validate_context_for_dispatch(
                &self,
                context: &impl IDeviceContext,
            ) -> Result<(), HResult> {
                unsafe {
                    let res = self
                        .0
                        .ValidateContextForDispatch(context.as_ptr() as *mut ID3D11DeviceContext);
                    hresult((), res)
                }
            }
        }
    };
}
impl_debug!(Debug, ID3D11Debug);
