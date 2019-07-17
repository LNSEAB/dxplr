use crate::api::*;
use crate::result::HResult;
use crate::utility::*;
use crate::Interface;
use crate::{impl_bitflag_operators, impl_interface};
use com_ptr;
use com_ptr::ComPtr;
#[cfg(feature = "dxgi1_2")]
use winapi::ctypes::c_void;
use winapi::shared::dxgi::*;
#[cfg(feature = "dxgi1_2")]
use winapi::shared::dxgi1_2::*;
#[cfg(feature = "dxgi1_3")]
use winapi::shared::dxgi1_3::*;
#[cfg(feature = "dxgi1_4")]
use winapi::shared::dxgi1_4::*;
#[cfg(feature = "dxgi1_5")]
use winapi::shared::dxgi1_5::*;
#[cfg(feature = "dxgi1_6")]
use winapi::shared::dxgi1_6::*;
use winapi::shared::dxgiformat::*;
use winapi::shared::dxgitype::*;
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::*;
use winapi::shared::windef::*;
use winapi::shared::winerror::*;
use winapi::um::dxgidebug::*;
#[cfg(feature = "dxgi1_2")]
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::winnt::HANDLE;
use winapi::um::winnt::HRESULT;
use winapi::Interface as _;

fn hresult<T>(obj: T, res: HRESULT) -> Result<T, HResult> {
    com_ptr::hresult(obj, res).map_err(|res| res.into())
}

#[derive(Clone, Copy, Debug)]
pub struct DebugID(Guid);
#[allow(non_upper_case_globals)]
impl DebugID {
    pub const All: Self = Self(Guid::new(
        0xe48ae283,
        0xda80,
        0x490b,
        [0x87, 0xe6, 0x43, 0xe9, 0xa9, 0xcf, 0xda, 0x8],
    ));
    pub const DX: Self = Self(Guid::new(
        0x35cdd7fc,
        0x13b2,
        0x421d,
        [0xa5, 0xd7, 0x7e, 0x44, 0x51, 0x28, 0x7d, 0x64],
    ));
    pub const DXGI: Self = Self(Guid::new(
        0x25cddaa4,
        0xb1c6,
        0x47e1,
        [0xac, 0x3e, 0x98, 0x87, 0x5b, 0x5a, 0x2e, 0x2a],
    ));
    pub const App: Self = Self(Guid::new(
        0x6cd6e01,
        0x4219,
        0x4ebd,
        [0x87, 0x9, 0x27, 0xed, 0x23, 0x36, 0xc, 0x62],
    ));
    pub const D3D11: Self = Self(Guid::new(
        0x4b99317b,
        0xac39,
        0x4aa6,
        [0xbb, 0xb, 0xba, 0xa0, 0x47, 0x84, 0x79, 0x8f],
    ));
}
impl From<DebugID> for GUID {
    fn from(src: DebugID) -> GUID {
        src.0.into()
    }
}
impl From<GUID> for DebugID {
    fn from(src: GUID) -> DebugID {
        DebugID(src.into())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct EnumModes(u32);
#[allow(non_upper_case_globals)]
impl EnumModes {
    pub const Interlaced: Self = Self(DXGI_ENUM_MODES_INTERLACED);
    pub const Scaling: Self = Self(DXGI_ENUM_MODES_SCALING);
    #[cfg(feature = "dxgi1_2")]
    pub const Stereo: Self = Self(DXGI_ENUM_MODES_STEREO);
    #[cfg(feature = "dxgi1_2")]
    pub const DisabledStereo: Self = Self(DXGI_ENUM_MODES_DISABLED_STEREO);
}
impl_bitflag_operators!(EnumModes);

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Present(u32);
#[allow(non_upper_case_globals)]
impl Present {
    pub const DoNotSequence: Self = Self(DXGI_PRESENT_DO_NOT_SEQUENCE);
    pub const Test: Self = Self(DXGI_PRESENT_TEST);
    pub const Restart: Self = Self(DXGI_PRESENT_RESTART);
    pub const DoNotWait: Self = Self(DXGI_PRESENT_DO_NOT_WAIT);
    pub const RestrictToOutput: Self = Self(DXGI_PRESENT_RESTRICT_TO_OUTPUT);
    pub const StreaoPreferRight: Self = Self(DXGI_PRESENT_STEREO_PREFER_RIGHT);
    pub const StereoTemporaryMono: Self = Self(DXGI_PRESENT_STEREO_TEMPORARY_MONO);
    pub const UseDuration: Self = Self(DXGI_PRESENT_USE_DURATION);
    pub const AllowTearing: Self = Self(DXGI_PRESENT_ALLOW_TEARING);
}
impl_bitflag_operators!(Present);

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct SharedResourceRW(u32);
#[cfg(feature = "dxgi1_2")]
#[allow(non_upper_case_globals)]
impl SharedResourceRW {
    pub const Read: Self = Self(DXGI_SHARED_RESOURCE_READ);
    pub const Write: Self = Self(DXGI_SHARED_RESOURCE_WRITE);
}
#[cfg(feature = "dxgi1_2")]
impl_bitflag_operators!(SharedResourceRW);

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct Usage(u32);
#[allow(non_upper_case_globals)]
impl Usage {
    pub const ShaderInput: Self = Self(DXGI_USAGE_SHADER_INPUT);
    pub const RenderTargetOutput: Self = Self(DXGI_USAGE_RENDER_TARGET_OUTPUT);
    pub const BackBuffer: Self = Self(DXGI_USAGE_BACK_BUFFER);
    pub const Shared: Self = Self(DXGI_USAGE_SHARED);
    pub const ReadOnly: Self = Self(DXGI_USAGE_READ_ONLY);
    pub const DiscardOnPresent: Self = Self(DXGI_USAGE_DISCARD_ON_PRESENT);
    pub const UnorderedAccess: Self = Self(DXGI_USAGE_UNORDERED_ACCESS);
}
impl_bitflag_operators!(Usage);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CPUAccess {
    None = DXGI_CPU_ACCESS_NONE,
    Dynamic = DXGI_CPU_ACCESS_DYNAMIC,
    ReadWrite = DXGI_CPU_ACCESS_READ_WRITE,
    Scratch = DXGI_CPU_ACCESS_SCRATCH,
    Field = DXGI_CPU_ACCESS_FIELD,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct MapFlags(u32);
#[allow(non_upper_case_globals)]
impl MapFlags {
    pub const Read: u32 = DXGI_MAP_READ;
    pub const Write: u32 = DXGI_MAP_WRITE;
    pub const Discard: u32 = DXGI_MAP_DISCARD;
}
impl_bitflag_operators!(MapFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Status {
    Occluded = DXGI_STATUS_OCCLUDED,
    ModeChanged = DXGI_STATUS_MODE_CHANGED,
    ModeChangeInProgress = DXGI_STATUS_MODE_CHANGE_IN_PROGRESS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AdapterFlag {
    None = DXGI_ADAPTER_FLAG_NONE,
    Remote = DXGI_ADAPTER_FLAG_REMOTE,
    Software = DXGI_ADAPTER_FLAG_SOFTWARE,
}

#[cfg(feature = "dxgi1_6")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AdapterFlag3 {
    None = DXGI_ADAPTER_FLAG3_NONE,
    Remote = DXGI_ADAPTER_FLAG3_REMOTE,
    Software = DXGI_ADAPTER_FLAG3_SOFTWARE,
    ACGCompatible = DXGI_ADAPTER_FLAG3_ACG_COMPATIBLE,
    SupportMonitoredFences = DXGI_ADAPTER_FLAG3_SUPPORT_MONITORED_FENCES,
    SupportNonMonitoredFences = DXGI_ADAPTER_FLAG3_SUPPORT_NON_MONITORED_FENCES,
    KeyedMutexConformance = DXGI_ADAPTER_FLAG3_KEYED_MUTEX_CONFORMANCE,
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AlphaMode {
    Unspecified = DXGI_ALPHA_MODE_UNSPECIFIED,
    Premultiplied = DXGI_ALPHA_MODE_PREMULTIPLIED,
    Straight = DXGI_ALPHA_MODE_STRAIGHT,
    Ignore = DXGI_ALPHA_MODE_IGNORE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorSpaceType {
    RGBFullG22NoneP709 = DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P709,
    RGBFullG10NoneP709 = DXGI_COLOR_SPACE_RGB_FULL_G10_NONE_P709,
    RGBStudioG22NoneP709 = DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P709,
    RGBStudioG22NoneP2020 = DXGI_COLOR_SPACE_RGB_STUDIO_G22_NONE_P2020,
    Reserved = DXGI_COLOR_SPACE_RESERVED,
    YCbCrFullG22NoneP709X601 = DXGI_COLOR_SPACE_YCBCR_FULL_G22_NONE_P709_X601,
    YCbCrStudioG22LeftP601 = DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P601,
    YCbCrFullG22LeftP601 = DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P601,
    YCbCrStudioG22LeftP709 = DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P709,
    YCbCrFullG22LeftP709 = DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P709,
    YCbCrStudioG22LeftP2020 = DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_LEFT_P2020,
    YCbCrFullG22LeftP2020 = DXGI_COLOR_SPACE_YCBCR_FULL_G22_LEFT_P2020,
    RGBFullG2084NoneP2020 = DXGI_COLOR_SPACE_RGB_FULL_G2084_NONE_P2020,
    YCbCrStudioG2084LeftP2020 = DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_LEFT_P2020,
    RGBStudioG2084NoneP2020 = DXGI_COLOR_SPACE_RGB_STUDIO_G2084_NONE_P2020,
    YCbCrStudioG22TopleftP2020 = DXGI_COLOR_SPACE_YCBCR_STUDIO_G22_TOPLEFT_P2020,
    YCbCrStudioG2084TopleftP2020 = DXGI_COLOR_SPACE_YCBCR_STUDIO_G2084_TOPLEFT_P2020,
    RGBFullG22NoneP2020 = DXGI_COLOR_SPACE_RGB_FULL_G22_NONE_P2020,
    // YCbCrStudioGhlgTopleftP2020 = DXGI_COLOR_SPACE_YCBCR_STUDIO_GHLG_TOPLEFT_P2020,
    // YCbCrFullGhlgTopleftP2020 = DXGI_COLOR_SPACE_YCBCR_FULL_GHLG_TOPLEFT_P2020,
    // RGBStudioG24NoneP709 = DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P709,
    // RGBStudioG24NoneP2020 = DXGI_COLOR_SPACE_RGB_STUDIO_G24_NONE_P2020,
    // YCbCrStudioG24LeftP709 = DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P709,
    // YCbCrStudioG24LeftP2020 = DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_LEFT_P2020,
    // YCbCrStuioG24TopleftP2020 = DXGI_COLOR_SPACE_YCBCR_STUDIO_G24_TOPLEFT_P2020,
    Custom = DXGI_COLOR_SPACE_CUSTOM,
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ComputePreemptionGranularity {
    DMABufferBoundary = DXGI_COMPUTE_PREEMPTION_DMA_BUFFER_BOUNDARY,
    DispatchBoundary = DXGI_COMPUTE_PREEMPTION_DISPATCH_BOUNDARY,
    ThreadGroupBoundary = DXGI_COMPUTE_PREEMPTION_THREAD_GROUP_BOUNDARY,
    ThreadBoundary = DXGI_COMPUTE_PREEMPTION_THREAD_BOUNDARY,
    InstructionBoundary = DXGI_COMPUTE_PREEMPTION_INSTRUCTION_BOUNDARY,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DebugRLOFlags {
    Summary = DXGI_DEBUG_RLO_SUMMARY,
    Detail = DXGI_DEBUG_RLO_DETAIL,
    IgnoreInternal = DXGI_DEBUG_RLO_IGNORE_INTERNAL,
    All = DXGI_DEBUG_RLO_ALL,
}

#[cfg(feature = "dxgi1_5")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Feature {
    PresentAllowTearing = DXGI_FEATURE_PRESENT_ALLOW_TEARING,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Format {
    Unknown = DXGI_FORMAT_UNKNOWN,
    R32G32B32A32Typeless = DXGI_FORMAT_R32G32B32A32_TYPELESS,
    R32G32B32A32Float = DXGI_FORMAT_R32G32B32A32_FLOAT,
    R32G32B32A32Uint = DXGI_FORMAT_R32G32B32A32_UINT,
    R32G32B32A32Sint = DXGI_FORMAT_R32G32B32A32_SINT,
    R32G32B32Typeless = DXGI_FORMAT_R32G32B32_TYPELESS,
    R32G32B32Float = DXGI_FORMAT_R32G32B32_FLOAT,
    R32G32B32Uint = DXGI_FORMAT_R32G32B32_UINT,
    R32G32B32Sint = DXGI_FORMAT_R32G32B32_SINT,
    R16G16B16A16Typeless = DXGI_FORMAT_R16G16B16A16_TYPELESS,
    R16G16B16A16Float = DXGI_FORMAT_R16G16B16A16_FLOAT,
    R16G16B16A16Unorm = DXGI_FORMAT_R16G16B16A16_UNORM,
    R16G16B16A16Uint = DXGI_FORMAT_R16G16B16A16_UINT,
    R16G16B16A16Snorm = DXGI_FORMAT_R16G16B16A16_SNORM,
    R16G16B16A16Sint = DXGI_FORMAT_R16G16B16A16_SINT,
    R32G32Typeless = DXGI_FORMAT_R32G32_TYPELESS,
    R32G32Float = DXGI_FORMAT_R32G32_FLOAT,
    R32G32Uint = DXGI_FORMAT_R32G32_UINT,
    R32G32Sint = DXGI_FORMAT_R32G32_SINT,
    R32G8X24Typeless = DXGI_FORMAT_R32G8X24_TYPELESS,
    D32FloatS8X24Uint = DXGI_FORMAT_D32_FLOAT_S8X24_UINT,
    R32FloatX8X24Typeless = DXGI_FORMAT_R32_FLOAT_X8X24_TYPELESS,
    X32TypelessG8X24Uint = DXGI_FORMAT_X32_TYPELESS_G8X24_UINT,
    R10G10B10A2Typeless = DXGI_FORMAT_R10G10B10A2_TYPELESS,
    R10G10B10A2Unorm = DXGI_FORMAT_R10G10B10A2_UNORM,
    R10G10B10A2Uint = DXGI_FORMAT_R10G10B10A2_UINT,
    R8G8B8A8Typeless = DXGI_FORMAT_R8G8B8A8_TYPELESS,
    R8G8B8A8Unorm = DXGI_FORMAT_R8G8B8A8_UNORM,
    R8G8B8A8UnormSRGB = DXGI_FORMAT_R8G8B8A8_UNORM_SRGB,
    R8G8B8A8Uint = DXGI_FORMAT_R8G8B8A8_UINT,
    R8G8B8A8Snorm = DXGI_FORMAT_R8G8B8A8_SNORM,
    R8G8B8A8Sint = DXGI_FORMAT_R8G8B8A8_SINT,
    R16G16Typeless = DXGI_FORMAT_R16G16_TYPELESS,
    R16G16Float = DXGI_FORMAT_R16G16_FLOAT,
    R16G16Unorm = DXGI_FORMAT_R16G16_UNORM,
    R16G16Uint = DXGI_FORMAT_R16G16_UINT,
    R16G16Snorm = DXGI_FORMAT_R16G16_SNORM,
    R16G16Sint = DXGI_FORMAT_R16G16_SINT,
    R32Typeless = DXGI_FORMAT_R32_TYPELESS,
    D32Float = DXGI_FORMAT_D32_FLOAT,
    R32Float = DXGI_FORMAT_R32_FLOAT,
    R32Uint = DXGI_FORMAT_R32_UINT,
    R32Sint = DXGI_FORMAT_R32_SINT,
    R24G8Typeless = DXGI_FORMAT_R24G8_TYPELESS,
    D24UnormS8Uint = DXGI_FORMAT_D24_UNORM_S8_UINT,
    R24UnormX8Typeless = DXGI_FORMAT_R24_UNORM_X8_TYPELESS,
    X24TypelessG8Uint = DXGI_FORMAT_X24_TYPELESS_G8_UINT,
    R8G8Typeless = DXGI_FORMAT_R8G8_TYPELESS,
    R8G8Unorm = DXGI_FORMAT_R8G8_UNORM,
    R8G8Uint = DXGI_FORMAT_R8G8_UINT,
    R8G8Snorm = DXGI_FORMAT_R8G8_SNORM,
    R8G8Sint = DXGI_FORMAT_R8G8_SINT,
    R16Typeless = DXGI_FORMAT_R16_TYPELESS,
    R16Float = DXGI_FORMAT_R16_FLOAT,
    D16Unorm = DXGI_FORMAT_D16_UNORM,
    R16Unorm = DXGI_FORMAT_R16_UNORM,
    R16Uint = DXGI_FORMAT_R16_UINT,
    R16Snorm = DXGI_FORMAT_R16_SNORM,
    R16Sint = DXGI_FORMAT_R16_SINT,
    R8Typeless = DXGI_FORMAT_R8_TYPELESS,
    R8Unorm = DXGI_FORMAT_R8_UNORM,
    R8Uint = DXGI_FORMAT_R8_UINT,
    R8Snorm = DXGI_FORMAT_R8_SNORM,
    R8Sint = DXGI_FORMAT_R8_SINT,
    A8Unorm = DXGI_FORMAT_A8_UNORM,
    R1Unorm = DXGI_FORMAT_R1_UNORM,
    R9G9B9E5SharedExp = DXGI_FORMAT_R9G9B9E5_SHAREDEXP,
    R8G8B8G8Unorm = DXGI_FORMAT_R8G8_B8G8_UNORM,
    G8R8G8B8Unorm = DXGI_FORMAT_G8R8_G8B8_UNORM,
    BC1Typeless = DXGI_FORMAT_BC1_TYPELESS,
    BC1Unorm = DXGI_FORMAT_BC1_UNORM,
    BC1UnormSRGB = DXGI_FORMAT_BC1_UNORM_SRGB,
    BC2Typeless = DXGI_FORMAT_BC2_TYPELESS,
    BC2Unorm = DXGI_FORMAT_BC2_UNORM,
    BC2UnormSRGB = DXGI_FORMAT_BC2_UNORM_SRGB,
    BC3Typeless = DXGI_FORMAT_BC3_TYPELESS,
    BC3Unorm = DXGI_FORMAT_BC3_UNORM,
    BC3UnormSRGB = DXGI_FORMAT_BC3_UNORM_SRGB,
    BC4Typeless = DXGI_FORMAT_BC4_TYPELESS,
    BC4Unorm = DXGI_FORMAT_BC4_UNORM,
    BC4Snorm = DXGI_FORMAT_BC4_SNORM,
    BC5Typeless = DXGI_FORMAT_BC5_TYPELESS,
    BC5Unorm = DXGI_FORMAT_BC5_UNORM,
    BC5Snorm = DXGI_FORMAT_BC5_SNORM,
    B5G6R5Unorm = DXGI_FORMAT_B5G6R5_UNORM,
    B5G5R5A1Unorm = DXGI_FORMAT_B5G5R5A1_UNORM,
    B8G8R8A8Unorm = DXGI_FORMAT_B8G8R8A8_UNORM,
    B8G8R8X8Unorm = DXGI_FORMAT_B8G8R8X8_UNORM,
    R10G10B10XrBiasA2Unorm = DXGI_FORMAT_R10G10B10_XR_BIAS_A2_UNORM,
    B8G8R8A8Typeless = DXGI_FORMAT_B8G8R8A8_TYPELESS,
    B8G8R8A8UnormSRGB = DXGI_FORMAT_B8G8R8A8_UNORM_SRGB,
    B8G8R8X8Typeless = DXGI_FORMAT_B8G8R8X8_TYPELESS,
    B8G8R8X8UnormSRGB = DXGI_FORMAT_B8G8R8X8_UNORM_SRGB,
    BC6HTypeless = DXGI_FORMAT_BC6H_TYPELESS,
    BC6HUf16 = DXGI_FORMAT_BC6H_UF16,
    BC6HSf16 = DXGI_FORMAT_BC6H_SF16,
    BC7Typeless = DXGI_FORMAT_BC7_TYPELESS,
    BC7Unorm = DXGI_FORMAT_BC7_UNORM,
    BC7UnormSRGB = DXGI_FORMAT_BC7_UNORM_SRGB,
    AYUV = DXGI_FORMAT_AYUV,
    Y410 = DXGI_FORMAT_Y410,
    Y416 = DXGI_FORMAT_Y416,
    NV12 = DXGI_FORMAT_NV12,
    P010 = DXGI_FORMAT_P010,
    P016 = DXGI_FORMAT_P016,
    _420Opaque = DXGI_FORMAT_420_OPAQUE,
    YUY2 = DXGI_FORMAT_YUY2,
    Y210 = DXGI_FORMAT_Y210,
    Y216 = DXGI_FORMAT_Y216,
    NV11 = DXGI_FORMAT_NV11,
    AI44 = DXGI_FORMAT_AI44,
    IA44 = DXGI_FORMAT_IA44,
    P8 = DXGI_FORMAT_P8,
    A8P8 = DXGI_FORMAT_A8P8,
    B4G4R4A4Unorm = DXGI_FORMAT_B4G4R4A4_UNORM,
    P208 = DXGI_FORMAT_P208,
    V208 = DXGI_FORMAT_V208,
    V408 = DXGI_FORMAT_V408,
}

#[cfg(feature = "dxgi1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FramePresentationMode {
    Composed = DXGI_FRAME_PRESENTATION_MODE_COMPOSED,
    Overlay = DXGI_FRAME_PRESENTATION_MODE_OVERLAY,
    None = DXGI_FRAME_PRESENTATION_MODE_NONE,
    CompositionFailure = DXGI_FRAME_PRESENTATION_MODE_COMPOSITION_FAILURE,
}

#[cfg(feature = "dxgi1_6")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GPUPreference {
    Unspecified = DXGI_GPU_PREFERENCE_UNSPECIFIED,
    MinimumPower = DXGI_GPU_PREFERENCE_MINIMUM_POWER,
    HighPerformance = DXGI_GPU_PREFERENCE_HIGH_PERFORMANCE,
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GraphicsPreemptionGranularity {
    DMABufferBoundary = DXGI_GRAPHICS_PREEMPTION_DMA_BUFFER_BOUNDARY,
    PrimitiveBoundary = DXGI_GRAPHICS_PREEMPTION_PRIMITIVE_BOUNDARY,
    TriangleBoundary = DXGI_GRAPHICS_PREEMPTION_TRIANGLE_BOUNDARY,
    PixelBoundary = DXGI_GRAPHICS_PREEMPTION_PIXEL_BOUNDARY,
    InstructionBoundary = DXGI_GRAPHICS_PREEMPTION_INSTRUCTION_BOUNDARY,
}

#[cfg(feature = "dxgi1_6")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct HardwareCompositionSupportFlags(u32);
#[cfg(feature = "dxgi1_6")]
#[allow(non_upper_case_globals)]
impl HardwareCompositionSupportFlags {
    pub const Fullscreen: Self = Self(DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_FULLSCREEN);
    pub const Windowed: Self = Self(DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_WINDOWED);
    pub const CursorStretched: Self = Self(DXGI_HARDWARE_COMPOSITION_SUPPORT_FLAG_CURSOR_STRETCHED);
}
#[cfg(feature = "dxgi1_6")]
impl_bitflag_operators!(HardwareCompositionSupportFlags);

#[cfg(feature = "dxgi1_5")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HDRMetadataType {
    None = DXGI_HDR_METADATA_TYPE_NONE,
    HDR10 = DXGI_HDR_METADATA_TYPE_HDR10,
    // HDR10Plus = DXGI_HDR_METADATA_TYPE_HDR10PLUS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum InfoQueueMessageCategory {
    Unknown = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_UNKNOWN,
    Miscellaneous = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_MISCELLANEOUS,
    Initialization = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_INITIALIZATION,
    Cleanup = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_CLEANUP,
    Compilation = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_COMPILATION,
    StateCreation = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_CREATION,
    StateSetting = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_SETTING,
    StateGetting = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_STATE_GETTING,
    ResourceManipulation = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_RESOURCE_MANIPULATION,
    Execution = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_EXECUTION,
    Shader = DXGI_INFO_QUEUE_MESSAGE_CATEGORY_SHADER,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum InfoQueueMessageSeverity {
    Corruption = DXGI_INFO_QUEUE_MESSAGE_SEVERITY_CORRUPTION,
    Error = DXGI_INFO_QUEUE_MESSAGE_SEVERITY_ERROR,
    Warning = DXGI_INFO_QUEUE_MESSAGE_SEVERITY_WARNING,
    Info = DXGI_INFO_QUEUE_MESSAGE_SEVERITY_INFO,
    Message = DXGI_INFO_QUEUE_MESSAGE_SEVERITY_MESSAGE,
}

#[cfg(feature = "dxgi1_4")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum MemorySegmentGroup {
    Local = DXGI_MEMORY_SEGMENT_GROUP_LOCAL,
    NonLocal = DXGI_MEMORY_SEGMENT_GROUP_NON_LOCAL,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ModeRotation {
    Unspecified = DXGI_MODE_ROTATION_UNSPECIFIED,
    Identity = DXGI_MODE_ROTATION_IDENTITY,
    Rotate90 = DXGI_MODE_ROTATION_ROTATE90,
    Rotate180 = DXGI_MODE_ROTATION_ROTATE180,
    Rotate270 = DXGI_MODE_ROTATION_ROTATE270,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ModeScaling {
    Unspecified = DXGI_MODE_SCALING_UNSPECIFIED,
    Centered = DXGI_MODE_SCALING_CENTERED,
    Stretched = DXGI_MODE_SCALING_STRETCHED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ScanlineOrder {
    Unspecified = DXGI_MODE_SCANLINE_ORDER_UNSPECIFIED,
    Progressive = DXGI_MODE_SCANLINE_ORDER_PROGRESSIVE,
    UpperFieldFirst = DXGI_MODE_SCANLINE_ORDER_UPPER_FIELD_FIRST,
    LowerFieldFirst = DXGI_MODE_SCANLINE_ORDER_LOWER_FIELD_FIRST,
}

#[cfg(feature = "dxgi1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MultiplaneOverlayYCbCrFlags(u32);
#[cfg(feature = "dxgi1_3")]
#[allow(non_upper_case_globals)]
impl MultiplaneOverlayYCbCrFlags {
    pub const NominalRange: Self = Self(DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_NOMINAL_RANGE);
    pub const BT709: Self = Self(DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_BT709);
    pub const XvYCC: Self = Self(DXGI_MULTIPLANE_OVERLAY_YCbCr_FLAG_xvYCC);
}
#[cfg(feature = "dxgi1_3")]
impl_bitflag_operators!(MultiplaneOverlayYCbCrFlags);

#[cfg(feature = "dxgi1_5")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum OfferResourceFlags {
    AllowDecommit = DXGI_OFFER_RESOURCE_FLAG_ALLOW_DECOMMIT,
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum OfferResourcePriority {
    Low = DXGI_OFFER_RESOURCE_PRIORITY_LOW,
    Normal = DXGI_OFFER_RESOURCE_PRIORITY_NORMAL,
    Hight = DXGI_OFFER_RESOURCE_PRIORITY_HIGH,
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum OutduplPointerShapeType {
    Monochrome = DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MONOCHROME,
    Color = DXGI_OUTDUPL_POINTER_SHAPE_TYPE_COLOR,
    MaskedColor = DXGI_OUTDUPL_POINTER_SHAPE_TYPE_MASKED_COLOR,
}

#[cfg(feature = "dxgi1_4")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OverlayColorSpaceSupportFlag(u32);
#[cfg(feature = "dxgi1_4")]
#[allow(non_upper_case_globals)]
impl OverlayColorSpaceSupportFlag {
    pub const Present: Self = Self(DXGI_OVERLAY_COLOR_SPACE_SUPPORT_FLAG_PRESENT);
}
#[cfg(feature = "dxgi1_4")]
impl_bitflag_operators!(OverlayColorSpaceSupportFlag);

#[cfg(feature = "dxgi1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct OverlaySupportFlag(u32);
#[cfg(feature = "dxgi1_3")]
#[allow(non_upper_case_globals)]
impl OverlaySupportFlag {
    pub const Direct: Self = Self(DXGI_OVERLAY_SUPPORT_FLAG_DIRECT);
    pub const Scaling: Self = Self(DXGI_OVERLAY_SUPPORT_FLAG_SCALING);
}
#[cfg(feature = "dxgi1_3")]
impl_bitflag_operators!(OverlaySupportFlag);

#[cfg(feature = "dxgi1_5")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ReclaimResourceResults {
    Ok = DXGI_RECLAIM_RESOURCE_RESULT_OK,
    Discarded = DXGI_RECLAIM_RESOURCE_RESULT_DISCARDED,
    NotCommitted = DXGI_RECLAIM_RESOURCE_RESULT_NOT_COMMITTED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Residency {
    FullyResident = DXGI_RESIDENCY_FULLY_RESIDENT,
    InSharedMemory = DXGI_RESIDENCY_RESIDENT_IN_SHARED_MEMORY,
    EvictedToDisk = DXGI_RESIDENCY_EVICTED_TO_DISK,
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Scaling {
    Stretch = DXGI_SCALING_STRETCH,
    None = DXGI_SCALING_NONE,
    AspectRatioStretch = DXGI_SCALING_ASPECT_RATIO_STRETCH,
}

#[cfg(feature = "dxgi1_4")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct SwapChainColorSpaceSupportFlag(u32);
#[cfg(feature = "dxgi1_4")]
#[allow(non_upper_case_globals)]
impl SwapChainColorSpaceSupportFlag {
    pub const Present: Self = Self(DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_PRESENT);
    pub const OverlayPresent: Self = Self(DXGI_SWAP_CHAIN_COLOR_SPACE_SUPPORT_FLAG_OVERLAY_PRESENT);
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Debug)]
pub struct SwapChainFlag(u32);
#[allow(non_upper_case_globals)]
impl SwapChainFlag {
    pub const NonPrerotated: Self = Self(DXGI_SWAP_CHAIN_FLAG_NONPREROTATED);
    pub const AllowModeSwitch: Self = Self(DXGI_SWAP_CHAIN_FLAG_ALLOW_MODE_SWITCH);
    pub const GDICompatible: Self = Self(DXGI_SWAP_CHAIN_FLAG_GDI_COMPATIBLE);
    pub const RestrictedContent: Self = Self(DXGI_SWAP_CHAIN_FLAG_RESTRICTED_CONTENT);
    pub const RestrictedSharedResourceDriver: Self =
        Self(DXGI_SWAP_CHAIN_FLAG_RESTRICT_SHARED_RESOURCE_DRIVER);
    pub const DisplayOnly: Self = Self(DXGI_SWAP_CHAIN_FLAG_DISPLAY_ONLY);
    pub const FrameLatencyWaitableObject: Self =
        Self(DXGI_SWAP_CHAIN_FLAG_FRAME_LATENCY_WAITABLE_OBJECT);
    pub const ForegroundLayer: Self = Self(DXGI_SWAP_CHAIN_FLAG_FOREGROUND_LAYER);
    pub const FullscreenVideo: Self = Self(DXGI_SWAP_CHAIN_FLAG_FULLSCREEN_VIDEO);
    pub const YUVVideo: Self = Self(DXGI_SWAP_CHAIN_FLAG_YUV_VIDEO);
    pub const HWProtected: Self = Self(DXGI_SWAP_CHAIN_FLAG_HW_PROTECTED);
    pub const AllowTearing: Self = Self(DXGI_SWAP_CHAIN_FLAG_ALLOW_TEARING);
    // pub const RestrictedToAllHolographicsDisplays: Self = Self(DXGI_SWAP_CHAIN_FLAG_RESTRICTED_TO_ALL_HOLOGRAPHIC_DISPLAYS);
}
impl_bitflag_operators!(SwapChainFlag);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SwapEffect {
    Discard = DXGI_SWAP_EFFECT_DISCARD,
    Sequential = DXGI_SWAP_EFFECT_SEQUENTIAL,
    FlipSequential = DXGI_SWAP_EFFECT_FLIP_SEQUENTIAL,
    FlipDiscard = DXGI_SWAP_EFFECT_FLIP_DISCARD,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct MakeWindowAssociationFlag(u32);
#[allow(non_upper_case_globals)]
impl MakeWindowAssociationFlag {
    pub const NoWindowChanges: Self = Self(1 << 0);
    pub const NoAltEnter: Self = Self(1 << 1);
    pub const NoPrintScreen: Self = Self(1 << 2);
    pub const Valid: Self = Self(0x7);
}
impl_bitflag_operators!(MakeWindowAssociationFlag);

#[cfg(feature = "dxgi1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct CreateFactoryFlag(u32);
#[cfg(feature = "dxgi1_3")]
#[allow(non_upper_case_globals)]
impl CreateFactoryFlag {
    pub const Debug: Self = Self(DXGI_CREATE_FACTORY_DEBUG);
}
#[cfg(feature = "dxgi1_3")]
impl_bitflag_operators!(CreateFactoryFlag);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ResourcePriority {
    Minimum = 0x28000000,
    Low = 0x50000000,
    Normal = 0x78000000,
    High = 0xa0000000,
    Maximum = 0xc8000000,
}

#[derive(Clone, Debug)]
pub struct AdapterDesc {
    pub description: String,
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: Luid,
}
impl From<DXGI_ADAPTER_DESC> for AdapterDesc {
    fn from(src: DXGI_ADAPTER_DESC) -> AdapterDesc {
        AdapterDesc {
            description: String::from_utf16(&src.Description).unwrap(),
            vendor_id: src.VendorId,
            device_id: src.DeviceId,
            sub_sys_id: src.SubSysId,
            revision: src.Revision,
            dedicated_video_memory: src.DedicatedVideoMemory,
            dedicated_system_memory: src.DedicatedSystemMemory,
            shared_system_memory: src.SharedSystemMemory,
            adapter_luid: src.AdapterLuid.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct AdapterDesc1 {
    pub description: String,
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: Luid,
    pub flags: AdapterFlag,
}
impl From<DXGI_ADAPTER_DESC1> for AdapterDesc1 {
    fn from(src: DXGI_ADAPTER_DESC1) -> AdapterDesc1 {
        AdapterDesc1 {
            description: String::from_utf16(&src.Description).unwrap(),
            vendor_id: src.VendorId,
            device_id: src.DeviceId,
            sub_sys_id: src.SubSysId,
            revision: src.Revision,
            dedicated_video_memory: src.DedicatedVideoMemory,
            dedicated_system_memory: src.DedicatedSystemMemory,
            shared_system_memory: src.SharedSystemMemory,
            adapter_luid: src.AdapterLuid.into(),
            flags: unsafe { std::mem::transmute(src.Flags) },
        }
    }
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Debug)]
pub struct AdapterDesc2 {
    pub description: String,
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: Luid,
    pub flags: AdapterFlag,
    pub graphics_preemption_granularity: GraphicsPreemptionGranularity,
    pub compute_preemption_granularity: ComputePreemptionGranularity,
}
#[cfg(feature = "dxgi1_2")]
impl From<DXGI_ADAPTER_DESC2> for AdapterDesc2 {
    fn from(src: DXGI_ADAPTER_DESC2) -> AdapterDesc2 {
        AdapterDesc2 {
            description: String::from_utf16(&src.Description).unwrap(),
            vendor_id: src.VendorId,
            device_id: src.DeviceId,
            sub_sys_id: src.SubSysId,
            revision: src.Revision,
            dedicated_video_memory: src.DedicatedVideoMemory,
            dedicated_system_memory: src.DedicatedSystemMemory,
            shared_system_memory: src.SharedSystemMemory,
            adapter_luid: src.AdapterLuid.into(),
            flags: unsafe { std::mem::transmute(src.Flags) },
            graphics_preemption_granularity: unsafe {
                std::mem::transmute(src.GraphicsPreemptionGranularity)
            },
            compute_preemption_granularity: unsafe {
                std::mem::transmute(src.ComputePreemptionGranularity)
            },
        }
    }
}

#[cfg(feature = "dxgi1_6")]
#[derive(Clone, Debug)]
pub struct AdapterDesc3 {
    pub description: String,
    pub vendor_id: u32,
    pub device_id: u32,
    pub sub_sys_id: u32,
    pub revision: u32,
    pub dedicated_video_memory: usize,
    pub dedicated_system_memory: usize,
    pub shared_system_memory: usize,
    pub adapter_luid: Luid,
    pub flags: AdapterFlag3,
    pub graphics_preemption_granularity: GraphicsPreemptionGranularity,
    pub compute_preemption_granularity: ComputePreemptionGranularity,
}
#[cfg(feature = "dxgi1_6")]
impl From<DXGI_ADAPTER_DESC3> for AdapterDesc3 {
    fn from(src: DXGI_ADAPTER_DESC3) -> AdapterDesc3 {
        AdapterDesc3 {
            description: String::from_utf16(&src.Description).unwrap(),
            vendor_id: src.VendorID,
            device_id: src.DeviceID,
            sub_sys_id: src.SubSysID,
            revision: src.Revision,
            dedicated_video_memory: src.DedicatedVideoMemory,
            dedicated_system_memory: src.DedicatedSystemMemory,
            shared_system_memory: src.SharedSystemMemory,
            adapter_luid: src.AdapterLuid.into(),
            flags: unsafe { std::mem::transmute(src.Flags) },
            graphics_preemption_granularity: unsafe {
                std::mem::transmute(src.GraphicsPreemptionGranularity)
            },
            compute_preemption_granularity: unsafe {
                std::mem::transmute(src.ComputePreemptionGranularity)
            },
        }
    }
}

#[cfg(feature = "dxgi1_3")]
#[derive(Clone, Debug, Default)]
pub struct DecodeSwapChainDesc {
    pub flags: u32,
}
#[cfg(feature = "dxgi1_3")]
impl From<DecodeSwapChainDesc> for DXGI_DECODE_SWAP_CHAIN_DESC {
    fn from(src: DecodeSwapChainDesc) -> DXGI_DECODE_SWAP_CHAIN_DESC {
        DXGI_DECODE_SWAP_CHAIN_DESC { Flags: src.flags }
    }
}

#[derive(Clone, Debug)]
pub struct DisplayColorSpace {
    pub primary_coordinates: [f32; 8],
    pub white_points: [f32; 16],
}

#[derive(Clone, Debug)]
pub struct FrameStatistics {
    pub present_count: u32,
    pub present_refresh_count: u32,
    pub sync_refresh_count: u32,
    pub sync_qpc_time: i64,
    pub sync_gpu_time: i64,
}
impl From<DXGI_FRAME_STATISTICS> for FrameStatistics {
    fn from(src: DXGI_FRAME_STATISTICS) -> FrameStatistics {
        FrameStatistics {
            present_count: src.PresentCount,
            present_refresh_count: src.PresentRefreshCount,
            sync_refresh_count: src.SyncRefreshCount,
            sync_qpc_time: unsafe { *src.SyncQPCTime.QuadPart() },
            sync_gpu_time: unsafe { *src.SyncGPUTime.QuadPart() },
        }
    }
}

#[cfg(feature = "dxgi1_3")]
#[derive(Clone, Debug)]
pub struct FrameStatisticsMedia {
    pub present_count: u32,
    pub present_refresh_count: u32,
    pub sync_refresh_count: u32,
    pub sync_qpc_time: i64,
    pub sync_gpu_time: i64,
    pub composition_mode: FramePresentationMode,
    pub approved_present_duration: u32,
}
#[cfg(feature = "dxgi1_3")]
impl From<DXGI_FRAME_STATISTICS_MEDIA> for FrameStatisticsMedia {
    fn from(src: DXGI_FRAME_STATISTICS_MEDIA) -> FrameStatisticsMedia {
        FrameStatisticsMedia {
            present_count: src.PresentCount,
            present_refresh_count: src.PresentRefreshCount,
            sync_refresh_count: src.SyncRefreshCount,
            sync_qpc_time: unsafe { *src.SyncQPCTime.QuadPart() },
            sync_gpu_time: unsafe { *src.SyncGPUTime.QuadPart() },
            composition_mode: unsafe { std::mem::transmute(src.CompositionMode) },
            approved_present_duration: src.ApprovedPresentDuration,
        }
    }
}

#[derive(Clone)]
pub struct GammaControl {
    pub scale: RGB,
    pub offset: RGB,
    pub gamma_curve: [RGB; 1025],
}
impl From<DXGI_GAMMA_CONTROL> for GammaControl {
    fn from(src: DXGI_GAMMA_CONTROL) -> GammaControl {
        GammaControl {
            scale: src.Scale.into(),
            offset: src.Offset.into(),
            gamma_curve: {
                let mut curve = [Default::default(); 1025];
                for i in 0..1025 {
                    curve[i] = src.GammaCurve[i].into();
                }
                curve
            },
        }
    }
}
impl From<GammaControl> for DXGI_GAMMA_CONTROL {
    fn from(src: GammaControl) -> DXGI_GAMMA_CONTROL {
        DXGI_GAMMA_CONTROL {
            Scale: src.scale.into(),
            Offset: src.offset.into(),
            GammaCurve: {
                let mut curve = [Default::default(); 1025];
                for i in 0..1025 {
                    curve[i] = src.gamma_curve[i].into();
                }
                curve
            },
        }
    }
}
impl std::fmt::Debug for GammaControl {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{scale: {:?}, offset: {:?}, gamma_curve: [",
            self.scale, self.offset
        )?;
        for i in self.gamma_curve.iter() {
            write!(f, "({:?}),", i)?;
        }
        write!(f, "]}}")
    }
}

#[derive(Clone)]
pub struct GammaControlCapabilities {
    pub scale_and_offset_supported: bool,
    pub max_converted_value: f32,
    pub min_converted_value: f32,
    pub control_point_positions: Vec<f32>,
}
impl From<DXGI_GAMMA_CONTROL_CAPABILITIES> for GammaControlCapabilities {
    fn from(src: DXGI_GAMMA_CONTROL_CAPABILITIES) -> GammaControlCapabilities {
        GammaControlCapabilities {
            scale_and_offset_supported: src.ScaleAndOffsetSupported == TRUE,
            max_converted_value: src.MaxConvertedValue,
            min_converted_value: src.MinConvertedValue,
            control_point_positions: src.ControlPointPositions
                [..src.NumGammaControlPoints as usize]
                .iter()
                .cloned()
                .collect::<Vec<_>>(),
        }
    }
}
impl From<GammaControlCapabilities> for DXGI_GAMMA_CONTROL_CAPABILITIES {
    fn from(src: GammaControlCapabilities) -> DXGI_GAMMA_CONTROL_CAPABILITIES {
        DXGI_GAMMA_CONTROL_CAPABILITIES {
            ScaleAndOffsetSupported: to_BOOL(src.scale_and_offset_supported),
            MaxConvertedValue: src.max_converted_value,
            MinConvertedValue: src.min_converted_value,
            NumGammaControlPoints: src.control_point_positions.len() as u32,
            ControlPointPositions: {
                assert!(src.control_point_positions.len() <= 1025);
                let mut a = [0.0; 1025];
                for i in 0..src.control_point_positions.len() {
                    a[i] = src.control_point_positions[i].into();
                }
                a
            },
        }
    }
}

#[cfg(feature = "dxgi1_5")]
#[derive(Clone, Debug)]
pub struct HDRMetadataHDR10 {
    pub red_primary: [u16; 2],
    pub green_primary: [u16; 2],
    pub blue_primary: [u16; 2],
    pub white_point: [u16; 2],
    pub max_mastering_luminance: u32,
    pub min_mastering_luminance: u32,
    pub max_content_light_level: u16,
    pub max_frame_average_light_level: u16,
}
#[cfg(feature = "dxgi1_5")]
impl From<HDRMetadataHDR10> for DXGI_HDR_METADATA_HDR10 {
    fn from(src: HDRMetadataHDR10) -> DXGI_HDR_METADATA_HDR10 {
        DXGI_HDR_METADATA_HDR10 {
            RedPrimary: src.red_primary,
            GreenPrimary: src.green_primary,
            BluePrimary: src.blue_primary,
            WhitePoint: src.white_point,
            MaxMasteringLuminance: src.max_mastering_luminance,
            MinMasteringLuminance: src.min_mastering_luminance,
            MaxContentLightLevel: src.max_content_light_level,
            MaxFrameAverageLightLevel: src.max_frame_average_light_level,
        }
    }
}

pub type InfoQueueMessageID = DXGI_INFO_QUEUE_MESSAGE_ID;

#[derive(Clone, Debug)]
pub struct InfoQueueFilter {
    pub allow_list: InfoQueueFilterDesc,
    pub deny_list: InfoQueueFilterDesc,
}
impl From<DXGI_INFO_QUEUE_FILTER> for InfoQueueFilter {
    fn from(src: DXGI_INFO_QUEUE_FILTER) -> InfoQueueFilter {
        InfoQueueFilter {
            allow_list: src.AllowList.into(),
            deny_list: src.DenyList.into(),
        }
    }
}
impl From<InfoQueueFilter> for DXGI_INFO_QUEUE_FILTER {
    fn from(src: InfoQueueFilter) -> DXGI_INFO_QUEUE_FILTER {
        DXGI_INFO_QUEUE_FILTER {
            AllowList: src.allow_list.into(),
            DenyList: src.deny_list.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InfoQueueFilterDesc {
    pub category_list: Vec<InfoQueueMessageCategory>,
    pub severity_list: Vec<InfoQueueMessageSeverity>,
    pub id_list: Vec<InfoQueueMessageID>,
}
impl From<DXGI_INFO_QUEUE_FILTER_DESC> for InfoQueueFilterDesc {
    fn from(src: DXGI_INFO_QUEUE_FILTER_DESC) -> InfoQueueFilterDesc {
        InfoQueueFilterDesc {
            category_list: unsafe {
                std::slice::from_raw_parts(src.pCategoryList, src.NumCategories as usize)
                    .iter()
                    .map(|i| std::mem::transmute(*i))
                    .collect::<Vec<_>>()
            },
            severity_list: unsafe {
                std::slice::from_raw_parts(src.pSeverityList, src.NumSeverities as usize)
                    .iter()
                    .map(|i| std::mem::transmute(*i))
                    .collect::<Vec<_>>()
            },
            id_list: unsafe {
                std::slice::from_raw_parts(src.pIDList, src.NumIDs as usize)
                    .iter()
                    .map(|i| *i)
                    .collect::<Vec<_>>()
            },
        }
    }
}
impl From<InfoQueueFilterDesc> for DXGI_INFO_QUEUE_FILTER_DESC {
    fn from(src: InfoQueueFilterDesc) -> DXGI_INFO_QUEUE_FILTER_DESC {
        let mut category_list = src
            .category_list
            .iter()
            .map(|&category| category as u32)
            .collect::<Vec<_>>();
        let mut severity_list = src
            .severity_list
            .iter()
            .map(|&severity| severity as u32)
            .collect::<Vec<_>>();
        let mut id_list = src.id_list.clone();
        DXGI_INFO_QUEUE_FILTER_DESC {
            NumCategories: category_list.len() as u32,
            pCategoryList: category_list.as_mut_ptr(),
            NumSeverities: severity_list.len() as u32,
            pSeverityList: severity_list.as_mut_ptr(),
            NumIDs: id_list.len() as u32,
            pIDList: id_list.as_mut_ptr(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct InfoQueueMessage {
    pub producer: DebugID,
    pub category: InfoQueueMessageCategory,
    pub severity: InfoQueueMessageSeverity,
    pub id: InfoQueueMessageID,
    pub description: String,
}
impl From<DXGI_INFO_QUEUE_MESSAGE> for InfoQueueMessage {
    fn from(src: DXGI_INFO_QUEUE_MESSAGE) -> InfoQueueMessage {
        InfoQueueMessage {
            producer: unsafe { std::mem::transmute(src.Producer) },
            category: unsafe { std::mem::transmute(src.Category) },
            severity: unsafe { std::mem::transmute(src.Severity) },
            id: src.ID,
            description: unsafe {
                String::from_utf8(
                    std::slice::from_raw_parts(
                        src.pDescription as *const u8,
                        src.DescriptionByteLength,
                    )
                    .to_vec(),
                )
                .unwrap_or_default()
            },
        }
    }
}

#[derive(Clone)]
pub struct JpegAcHuffmanTable {
    pub code_counts: [u8; 16],
    pub code_values: [u8; 162],
}

#[derive(Clone)]
pub struct JpegDcHuffmanTable {
    pub code_counts: [u8; 12],
    pub code_values: [u8; 12],
}

#[derive(Clone)]
pub struct JpegQuantizationTable {
    pub elements: [u8; 64],
}

#[cfg(feature = "dxgi1_3")]
pub type Matrix3x2f = DXGI_MATRIX_3X2_F;

pub struct MappedRect {
    pub pitch: i32,
    pub bits: *mut u8,
}
impl From<DXGI_MAPPED_RECT> for MappedRect {
    fn from(src: DXGI_MAPPED_RECT) -> MappedRect {
        MappedRect {
            pitch: src.Pitch,
            bits: src.pBits,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ModeDesc {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: Rational,
    pub format: Format,
    pub scanline_ordering: ScanlineOrder,
    pub scaling: ModeScaling,
}
impl From<DXGI_MODE_DESC> for ModeDesc {
    fn from(src: DXGI_MODE_DESC) -> ModeDesc {
        ModeDesc {
            width: src.Width,
            height: src.Height,
            refresh_rate: src.RefreshRate.into(),
            format: unsafe { std::mem::transmute(src.Format) },
            scanline_ordering: unsafe { std::mem::transmute(src.ScanlineOrdering) },
            scaling: unsafe { std::mem::transmute(src.Scaling) },
        }
    }
}
impl From<ModeDesc> for DXGI_MODE_DESC {
    fn from(src: ModeDesc) -> DXGI_MODE_DESC {
        DXGI_MODE_DESC {
            Width: src.width,
            Height: src.height,
            RefreshRate: src.refresh_rate.into(),
            Format: src.format as u32,
            ScanlineOrdering: src.scanline_ordering as u32,
            Scaling: src.scaling as u32,
        }
    }
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Debug)]
pub struct ModeDesc1 {
    pub width: u32,
    pub height: u32,
    pub refresh_rate: Rational,
    pub format: Format,
    pub scanline_ordering: ScanlineOrder,
    pub scaling: Scaling,
    pub stereo: bool,
}
#[cfg(feature = "dxgi1_2")]
impl From<DXGI_MODE_DESC1> for ModeDesc1 {
    fn from(src: DXGI_MODE_DESC1) -> ModeDesc1 {
        ModeDesc1 {
            width: src.Width,
            height: src.Height,
            refresh_rate: src.RefreshRate.into(),
            format: unsafe { std::mem::transmute(src.Format) },
            scanline_ordering: unsafe { std::mem::transmute(src.ScanlineOrdering) },
            scaling: unsafe { std::mem::transmute(src.Scaling) },
            stereo: src.Stereo == TRUE,
        }
    }
}
#[cfg(feature = "dxgi1_2")]
impl From<ModeDesc1> for DXGI_MODE_DESC1 {
    fn from(src: ModeDesc1) -> DXGI_MODE_DESC1 {
        DXGI_MODE_DESC1 {
            Width: src.width,
            Height: src.height,
            RefreshRate: src.refresh_rate.into(),
            Format: src.format as u32,
            ScanlineOrdering: src.scanline_ordering as u32,
            Scaling: src.scaling as u32,
            Stereo: to_BOOL(src.stereo),
        }
    }
}

#[derive(Clone, Debug)]
pub struct OutputDesc {
    pub device_name: String,
    pub desktop_coordinates: Rect,
    pub attached_to_desktop: bool,
    pub rotation: ModeRotation,
    pub monitor: HMONITOR,
}
impl From<DXGI_OUTPUT_DESC> for OutputDesc {
    fn from(src: DXGI_OUTPUT_DESC) -> OutputDesc {
        OutputDesc {
            device_name: String::from_utf16(&src.DeviceName).unwrap_or_default(),
            desktop_coordinates: src.DesktopCoordinates.into(),
            attached_to_desktop: src.AttachedToDesktop == TRUE,
            rotation: unsafe { std::mem::transmute(src.Rotation) },
            monitor: src.Monitor,
        }
    }
}

#[cfg(feature = "dxgi1_6")]
#[derive(Clone, Debug)]
pub struct OutputDesc1 {
    pub device_name: String,
    pub desktop_coordinates: Rect,
    pub attached_to_desktop: bool,
    pub rotation: ModeRotation,
    pub monitor: HMONITOR,
    pub bits_per_color: u32,
    pub red_primary: [f32; 2],
    pub green_primary: [f32; 2],
    pub blue_primary: [f32; 2],
    pub white_point: [f32; 2],
    pub min_luminance: f32,
    pub max_luminance: f32,
    pub max_full_frame_luminance: f32,
}
#[cfg(feature = "dxgi1_6")]
impl From<DXGI_OUTPUT_DESC1> for OutputDesc1 {
    fn from(src: DXGI_OUTPUT_DESC1) -> OutputDesc1 {
        OutputDesc1 {
            device_name: String::from_utf16(&src.DeviceName).unwrap_or_default(),
            desktop_coordinates: src.DesktopCoordinates.into(),
            attached_to_desktop: src.AttachedToDesktop == TRUE,
            rotation: unsafe { std::mem::transmute(src.Rotation) },
            monitor: src.Monitor,
            bits_per_color: src.BitsPerColor,
            red_primary: src.RedPrimary,
            green_primary: src.GreenPrimary,
            blue_primary: src.BluePrimary,
            white_point: src.WhitePoint,
            min_luminance: src.MinLuminance,
            max_luminance: src.MaxLuminance,
            max_full_frame_luminance: src.MaxFullFrameLuminance,
        }
    }
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Debug)]
pub struct OutduplDesc {
    pub mode_desc: ModeDesc,
    pub rotation: ModeRotation,
    pub desktop_image_in_system_memory: bool,
}
#[cfg(feature = "dxgi1_2")]
impl From<DXGI_OUTDUPL_DESC> for OutduplDesc {
    fn from(src: DXGI_OUTDUPL_DESC) -> OutduplDesc {
        OutduplDesc {
            mode_desc: src.ModeDesc.into(),
            rotation: unsafe { std::mem::transmute(src.Rotation) },
            desktop_image_in_system_memory: src.DesktopImageInSystemMemory == TRUE,
        }
    }
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Debug)]
pub struct OutduplFrameInfo {
    pub last_present_time: i64,
    pub last_mouse_update_time: i64,
    pub accumulated_frames: u32,
    pub rects_coalesced: bool,
    pub protected_content_masked_out: bool,
    pub pointer_position: OutduplPointerPosition,
    pub total_metadata_buffer_size: u32,
    pub pointer_shape_buffer_size: u32,
}
#[cfg(feature = "dxgi1_2")]
impl From<DXGI_OUTDUPL_FRAME_INFO> for OutduplFrameInfo {
    fn from(src: DXGI_OUTDUPL_FRAME_INFO) -> OutduplFrameInfo {
        OutduplFrameInfo {
            last_present_time: unsafe { *src.LastPresentTime.QuadPart() },
            last_mouse_update_time: unsafe { *src.LastMouseUpdateTime.QuadPart() },
            accumulated_frames: src.AccumulatedFrames,
            rects_coalesced: src.RectsCoalesced == TRUE,
            protected_content_masked_out: src.ProtectedContentMaskedOut == TRUE,
            pointer_position: src.PointerPosition.into(),
            total_metadata_buffer_size: src.TotalMetadataBufferSize,
            pointer_shape_buffer_size: src.PointerShapeBufferSize,
        }
    }
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct OutduplMoveRect {
    pub source_point: Point,
    pub destination_rect: Rect,
}
#[cfg(feature = "dxgi1_2")]
impl From<DXGI_OUTDUPL_MOVE_RECT> for OutduplMoveRect {
    fn from(src: DXGI_OUTDUPL_MOVE_RECT) -> OutduplMoveRect {
        OutduplMoveRect {
            source_point: src.SourcePoint.into(),
            destination_rect: src.DestinationRect.into(),
        }
    }
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Debug)]
pub struct OutduplPointerPosition {
    position: Point,
    visible: bool,
}
#[cfg(feature = "dxgi1_2")]
impl From<DXGI_OUTDUPL_POINTER_POSITION> for OutduplPointerPosition {
    fn from(src: DXGI_OUTDUPL_POINTER_POSITION) -> OutduplPointerPosition {
        OutduplPointerPosition {
            position: src.Position.into(),
            visible: src.Visible == TRUE,
        }
    }
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Debug)]
pub struct OutduplPointerShapeInfo {
    pub ty: u32,
    pub width: u32,
    pub height: u32,
    pub pitch: u32,
    pub hot_spot: Point,
}
#[cfg(feature = "dxgi1_2")]
impl From<DXGI_OUTDUPL_POINTER_SHAPE_INFO> for OutduplPointerShapeInfo {
    fn from(src: DXGI_OUTDUPL_POINTER_SHAPE_INFO) -> OutduplPointerShapeInfo {
        OutduplPointerShapeInfo {
            ty: src.Type,
            width: src.Width,
            height: src.Height,
            pitch: src.Pitch,
            hot_spot: src.HotSpot.into(),
        }
    }
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Default, Debug)]
pub struct PresentParameters<'a> {
    pub dirty_rects: Option<&'a [Rect]>,
    pub scroll_rect: Option<Rect>,
    pub scroll_offset: Option<Point>,
}

#[cfg(feature = "dxgi1_4")]
#[derive(Clone, Debug)]
pub struct QueryVideoMemoryInfo {
    pub budget: u64,
    pub current_usage: u64,
    pub available_for_reservation: u64,
    pub current_reservation: u64,
}
#[cfg(feature = "dxgi1_4")]
impl From<DXGI_QUERY_VIDEO_MEMORY_INFO> for QueryVideoMemoryInfo {
    fn from(src: DXGI_QUERY_VIDEO_MEMORY_INFO) -> QueryVideoMemoryInfo {
        QueryVideoMemoryInfo {
            budget: src.Budget,
            current_usage: src.CurrentUsage,
            available_for_reservation: src.AvailableForReservation,
            current_reservation: src.CurrentReservation,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Rational {
    numerator: u32,
    denominator: u32,
}
impl From<DXGI_RATIONAL> for Rational {
    fn from(src: DXGI_RATIONAL) -> Rational {
        Rational {
            numerator: src.Numerator,
            denominator: src.Denominator,
        }
    }
}
impl From<Rational> for DXGI_RATIONAL {
    fn from(src: Rational) -> DXGI_RATIONAL {
        DXGI_RATIONAL {
            Numerator: src.numerator,
            Denominator: src.denominator,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct RGB {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}
impl From<DXGI_RGB> for RGB {
    fn from(src: DXGI_RGB) -> RGB {
        RGB {
            r: src.Red,
            g: src.Green,
            b: src.Blue,
        }
    }
}
impl From<RGB> for DXGI_RGB {
    fn from(src: RGB) -> DXGI_RGB {
        DXGI_RGB {
            Red: src.r,
            Green: src.g,
            Blue: src.b,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl From<DXGI_RGBA> for RGBA {
    fn from(src: DXGI_RGBA) -> RGBA {
        RGBA {
            r: src.r,
            g: src.g,
            b: src.b,
            a: src.a,
        }
    }
}
impl From<RGBA> for DXGI_RGBA {
    fn from(src: RGBA) -> DXGI_RGBA {
        DXGI_RGBA {
            r: src.r,
            g: src.g,
            b: src.b,
            a: src.a,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SampleDesc {
    count: u32,
    quality: u32,
}
impl Default for SampleDesc {
    fn default() -> Self {
        Self {
            count: 1,
            quality: 0,
        }
    }
}
impl From<DXGI_SAMPLE_DESC> for SampleDesc {
    fn from(src: DXGI_SAMPLE_DESC) -> SampleDesc {
        SampleDesc {
            count: src.Count,
            quality: src.Quality,
        }
    }
}
impl From<SampleDesc> for DXGI_SAMPLE_DESC {
    fn from(src: SampleDesc) -> DXGI_SAMPLE_DESC {
        DXGI_SAMPLE_DESC {
            Count: src.count,
            Quality: src.quality,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct SharedResource {
    handle: HANDLE,
}
impl From<DXGI_SHARED_RESOURCE> for SharedResource {
    fn from(src: DXGI_SHARED_RESOURCE) -> SharedResource {
        SharedResource { handle: src.Handle }
    }
}
impl From<SharedResource> for DXGI_SHARED_RESOURCE {
    fn from(src: SharedResource) -> DXGI_SHARED_RESOURCE {
        DXGI_SHARED_RESOURCE { Handle: src.handle }
    }
}

#[derive(Clone, Debug)]
pub struct SurfaceDesc {
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub sample_desc: SampleDesc,
}
impl From<DXGI_SURFACE_DESC> for SurfaceDesc {
    fn from(src: DXGI_SURFACE_DESC) -> SurfaceDesc {
        SurfaceDesc {
            width: src.Width,
            height: src.Height,
            format: unsafe { std::mem::transmute(src.Format) },
            sample_desc: src.SampleDesc.into(),
        }
    }
}
impl From<SurfaceDesc> for DXGI_SURFACE_DESC {
    fn from(src: SurfaceDesc) -> DXGI_SURFACE_DESC {
        DXGI_SURFACE_DESC {
            Width: src.width,
            Height: src.height,
            Format: src.format as u32,
            SampleDesc: src.sample_desc.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct SwapChainDesc {
    pub buffer_desc: ModeDesc,
    pub sample_desc: SampleDesc,
    pub buffer_usage: Usage,
    pub buffer_count: u32,
    pub output_window: HWND,
    pub windowed: bool,
    pub swap_effect: SwapEffect,
    pub flags: SwapChainFlag,
}
impl From<DXGI_SWAP_CHAIN_DESC> for SwapChainDesc {
    fn from(src: DXGI_SWAP_CHAIN_DESC) -> SwapChainDesc {
        SwapChainDesc {
            buffer_desc: src.BufferDesc.into(),
            sample_desc: src.SampleDesc.into(),
            buffer_usage: unsafe { std::mem::transmute(src.BufferUsage) },
            buffer_count: src.BufferCount,
            output_window: src.OutputWindow,
            windowed: src.Windowed == TRUE,
            swap_effect: unsafe { std::mem::transmute(src.SwapEffect) },
            flags: SwapChainFlag(src.Flags),
        }
    }
}
impl From<SwapChainDesc> for DXGI_SWAP_CHAIN_DESC {
    fn from(src: SwapChainDesc) -> DXGI_SWAP_CHAIN_DESC {
        DXGI_SWAP_CHAIN_DESC {
            BufferDesc: src.buffer_desc.into(),
            SampleDesc: src.sample_desc.into(),
            BufferUsage: src.buffer_usage.0,
            BufferCount: src.buffer_count,
            OutputWindow: src.output_window,
            Windowed: to_BOOL(src.windowed),
            SwapEffect: src.swap_effect as u32,
            Flags: src.flags.0,
        }
    }
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Debug)]
pub struct SwapChainDesc1 {
    pub width: u32,
    pub height: u32,
    pub format: Format,
    pub stereo: bool,
    pub sample_desc: SampleDesc,
    pub buffer_usage: Usage,
    pub buffer_count: u32,
    pub scaling: Scaling,
    pub swap_effect: SwapEffect,
    pub alpha_mode: AlphaMode,
    pub flags: SwapChainFlag,
}
#[cfg(feature = "dxgi1_2")]
impl From<DXGI_SWAP_CHAIN_DESC1> for SwapChainDesc1 {
    fn from(src: DXGI_SWAP_CHAIN_DESC1) -> SwapChainDesc1 {
        SwapChainDesc1 {
            width: src.Width,
            height: src.Height,
            format: unsafe { std::mem::transmute(src.Format) },
            stereo: src.Stereo == TRUE,
            sample_desc: src.SampleDesc.into(),
            buffer_usage: unsafe { std::mem::transmute(src.BufferUsage) },
            buffer_count: src.BufferCount,
            scaling: unsafe { std::mem::transmute(src.Scaling) },
            swap_effect: unsafe { std::mem::transmute(src.SwapEffect) },
            alpha_mode: unsafe { std::mem::transmute(src.AlphaMode) },
            flags: SwapChainFlag(src.Flags),
        }
    }
}
#[cfg(feature = "dxgi1_2")]
impl From<SwapChainDesc1> for DXGI_SWAP_CHAIN_DESC1 {
    fn from(src: SwapChainDesc1) -> DXGI_SWAP_CHAIN_DESC1 {
        DXGI_SWAP_CHAIN_DESC1 {
            Width: src.width,
            Height: src.height,
            Format: src.format as u32,
            Stereo: to_BOOL(src.stereo),
            SampleDesc: src.sample_desc.into(),
            BufferUsage: src.buffer_usage.0,
            BufferCount: src.buffer_count,
            Scaling: src.scaling as u32,
            SwapEffect: src.swap_effect as u32,
            AlphaMode: src.alpha_mode as u32,
            Flags: src.flags.0,
        }
    }
}

#[cfg(feature = "dxgi1_2")]
#[derive(Clone, Debug)]
pub struct SwapChainFullscreenDesc {
    pub refresh_rate: Rational,
    pub scanline_ordering: ScanlineOrder,
    pub scaling: Scaling,
    pub windowed: bool,
}
#[cfg(feature = "dxgi1_2")]
impl From<DXGI_SWAP_CHAIN_FULLSCREEN_DESC> for SwapChainFullscreenDesc {
    fn from(src: DXGI_SWAP_CHAIN_FULLSCREEN_DESC) -> SwapChainFullscreenDesc {
        SwapChainFullscreenDesc {
            refresh_rate: src.RefreshRate.into(),
            scanline_ordering: unsafe { std::mem::transmute(src.ScanlineOrdering) },
            scaling: unsafe { std::mem::transmute(src.Scaling) },
            windowed: src.Windowed == TRUE,
        }
    }
}
#[cfg(feature = "dxgi1_2")]
impl From<SwapChainFullscreenDesc> for DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
    fn from(src: SwapChainFullscreenDesc) -> DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
        DXGI_SWAP_CHAIN_FULLSCREEN_DESC {
            RefreshRate: src.refresh_rate.into(),
            ScanlineOrdering: src.scanline_ordering as u32,
            Scaling: src.scaling as u32,
            Windowed: to_BOOL(src.windowed),
        }
    }
}

pub trait IAdapter: Interface {
    fn check_interface_support<T: Interface>(&self) -> Result<i64, HResult>;
    fn enum_outputs(&self) -> Result<Vec<Output>, HResult>;
    fn get_desc(&self) -> Result<AdapterDesc, HResult>;
}
pub trait IAdapter1: IAdapter {
    fn get_desc1(&self) -> Result<AdapterDesc1, HResult>;
}
#[cfg(feature = "dxgi1_2")]
pub trait IAdapter2: IAdapter1 {
    fn get_desc2(&self) -> Result<AdapterDesc2, HResult>;
}
#[cfg(feature = "dxgi1_4")]
pub trait IAdapter3: IAdapter2 {
    fn query_video_memory_info(
        &self,
        node_index: u32,
        memory_segment_group: MemorySegmentGroup,
    ) -> Result<QueryVideoMemoryInfo, HResult>;
    fn register_hardware_content_protection_teardown_status_event(
        &self,
        hevent: HANDLE,
    ) -> Result<u32, HResult>;
    fn register_video_memory_budget_change_notification_event(
        &self,
        hevent: HANDLE,
    ) -> Result<u32, HResult>;
    fn set_video_memory_reservation(
        &self,
        node_index: u32,
        memory_segment_group: MemorySegmentGroup,
        reservation: u64,
    ) -> Result<(), HResult>;
    fn unregister_hardware_content_protection_teardown_status(&self, cookie: u32);
    fn unregister_video_memory_budget_change_notification(&self, cookie: u32);
}
#[cfg(feature = "dxgi1_6")]
pub trait IAdapter4: IAdapter3 {
    fn get_desc3(&self) -> Result<AdapterDesc3, HResult>;
}
macro_rules! impl_adapter {
    ($s: ident, $interface: ident, Adapter) => {
        impl_interface!($s, $interface);
        impl IAdapter for $s {
            fn check_interface_support<T: Interface>(&self) -> Result<i64, HResult> {
                let mut umd_version = Default::default();
                unsafe {
                    let res = self
                        .0
                        .CheckInterfaceSupport(&T::uuidof().into(), &mut umd_version);
                    hresult(*umd_version.QuadPart(), res)
                }
            }
            fn enum_outputs(&self) -> Result<Vec<Output>, HResult> {
                enum_function(DXGI_ERROR_NOT_FOUND.into(), |i| {
                    Ok(Output(ComPtr::new(|| {
                        let mut obj = std::ptr::null_mut();
                        let res = unsafe { self.0.EnumOutputs(i, &mut obj) };
                        hresult(obj, res.into())
                    })?))
                })
            }
            fn get_desc(&self) -> Result<AdapterDesc, HResult> {
                let mut desc = Default::default();
                let res = unsafe { self.0.GetDesc(&mut desc) };
                hresult(desc.into(), res.into())
            }
        }
    };
    ($s: ident, $interface: ident, Adapter1) => {
        impl_adapter!($s, $interface, Adapter);
        impl $s {
            pub fn as_adapter(&self) -> Adapter {
                Adapter(self.0.query_interface::<IDXGIAdapter>().unwrap())
            }
        }
        impl IAdapter1 for $s {
            fn get_desc1(&self) -> Result<AdapterDesc1, HResult> {
                let mut desc = Default::default();
                let res = unsafe { self.0.GetDesc1(&mut desc) };
                hresult(desc.into(), res.into())
            }
        }
    };
    ($s: ident, $interface: ident, Adapter2) => {
        impl_adapter!($s, $interface, Adapter1);
        impl $s {
            pub fn as_adapter1(&self) -> Adapter1 {
                Adapter1(self.0.query_interface::<IDXGIAdapter1>().unwrap())
            }
        }
        impl IAdapter2 for $s {
            fn get_desc2(&self) -> Result<AdapterDesc2, HResult> {
                let mut desc = Default::default();
                let res = unsafe { self.0.GetDesc2(&mut desc) };
                hresult(desc.into(), res.into())
            }
        }
    };
    ($s: ident, $interface: ident, Adapter3) => {
        impl_adapter!($s, $interface, Adapter2);
        impl $s {
            pub fn as_adapter2(&self) -> Adapter2 {
                Adapter2(self.0.query_interface::<IDXGIAdapter2>().unwrap())
            }
        }
        impl IAdapter3 for $s {
            fn query_video_memory_info(
                &self,
                node_index: u32,
                memory_segment_group: MemorySegmentGroup,
            ) -> Result<QueryVideoMemoryInfo, HResult> {
                let mut info = Default::default();
                let res = unsafe {
                    self.0
                        .QueryVideoMemoryInfo(node_index, memory_segment_group as u32, &mut info)
                };
                hresult(info.into(), res.into())
            }
            fn register_hardware_content_protection_teardown_status_event(
                &self,
                hevent: HANDLE,
            ) -> Result<u32, HResult> {
                let mut cookie = Default::default();
                let res = unsafe {
                    self.0
                        .RegisterHardwareContentProtectionTeardownStatusEvent(hevent, &mut cookie)
                };
                hresult(cookie, res.into())
            }
            fn register_video_memory_budget_change_notification_event(
                &self,
                hevent: HANDLE,
            ) -> Result<u32, HResult> {
                let mut cookie = Default::default();
                let res = unsafe {
                    self.0
                        .RegisterVideoMemoryBudgetChangeNotificationEvent(hevent, &mut cookie)
                };
                hresult(cookie, res.into())
            }
            fn set_video_memory_reservation(
                &self,
                node_index: u32,
                memory_segment_group: MemorySegmentGroup,
                reservation: u64,
            ) -> Result<(), HResult> {
                let res = unsafe {
                    self.0.SetVideoMemoryReservation(
                        node_index,
                        memory_segment_group as u32,
                        reservation,
                    )
                };
                hresult((), res.into())
            }
            fn unregister_hardware_content_protection_teardown_status(&self, cookie: u32) {
                unsafe {
                    self.0
                        .UnregisterHardwareContentProtectionTeardownStatus(cookie)
                };
            }
            fn unregister_video_memory_budget_change_notification(&self, cookie: u32) {
                unsafe { self.0.UnregisterVideoMemoryBudgetChangeNotification(cookie) };
            }
        }
    };
    ($s: ident, $interface: ident, Adapter4) => {
        impl_adapter!($s, $interface, Adapter3);
        impl $s {
            pub fn as_adapter3(&self) -> Adapter3 {
                Adapter3(self.0.query_interface::<IDXGIAdapter3>().unwrap())
            }
        }
        impl IAdapter4 for $s {
            fn get_desc3(&self) -> Result<AdapterDesc3, HResult> {
                let mut desc = Default::default();
                let res = unsafe { self.0.GetDesc3(&mut desc) };
                hresult(desc.into(), res.into())
            }
        }
    };
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Adapter(ComPtr<IDXGIAdapter>);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Adapter1(ComPtr<IDXGIAdapter1>);
#[cfg(feature = "dxgi1_2")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Adapter2(ComPtr<IDXGIAdapter2>);
#[cfg(feature = "dxgi1_4")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Adapter3(ComPtr<IDXGIAdapter3>);
#[cfg(feature = "dxgi1_6")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Adapter4(ComPtr<IDXGIAdapter4>);
impl_adapter!(Adapter, IDXGIAdapter, Adapter);
impl_adapter!(Adapter1, IDXGIAdapter1, Adapter1);
#[cfg(feature = "dxgi1_2")]
impl_adapter!(Adapter2, IDXGIAdapter2, Adapter2);
#[cfg(feature = "dxgi1_4")]
impl_adapter!(Adapter3, IDXGIAdapter3, Adapter3);
#[cfg(feature = "dxgi1_6")]
impl_adapter!(Adapter4, IDXGIAdapter4, Adapter4);

pub trait IDebug: Interface {
    fn report_live_objects(&self, apiid: DebugID, flags: DebugRLOFlags) -> Result<(), HResult>;
}
pub trait IDebug1: IDebug {
    fn disable_leak_tracking_for_thread(&self);
    fn enable_leak_tracking_for_thread(&self);
    fn is_leak_tracking_enabled_for_thread(&self) -> bool;
}
macro_rules! impl_debug {
    ($s: ident, $interface: ident, Debug) => {
        impl_interface!($s, $interface);
        impl IDebug for $s {
            fn report_live_objects(
                &self,
                apiid: DebugID,
                flags: DebugRLOFlags,
            ) -> Result<(), HResult> {
                let res = unsafe { self.0.ReportLiveObjects(apiid.into(), flags as u32) };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, Debug1) => {
        impl_debug!($s, $interface, Debug);
        impl $s {
            pub fn as_debug(&self) -> Debug {
                Debug(self.0.query_interface::<IDXGIDebug>().unwrap())
            }
        }
        impl IDebug1 for $s {
            fn disable_leak_tracking_for_thread(&self) {
                unsafe { self.0.DisableLeakTrackingForThread() };
            }
            fn enable_leak_tracking_for_thread(&self) {
                unsafe { self.0.EnableLeakTrackingForThread() };
            }
            fn is_leak_tracking_enabled_for_thread(&self) -> bool {
                unsafe { self.0.IsLeakTrackingEnabledForThread() == TRUE }
            }
        }
    };
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Debug(ComPtr<IDXGIDebug>);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Debug1(ComPtr<IDXGIDebug1>);
impl_debug!(Debug, IDXGIDebug, Debug);
impl_debug!(Debug1, IDXGIDebug1, Debug1);

#[cfg(feature = "dxgi1_3")]
#[derive(Clone, Copy, Default, Debug)]
pub struct DestSize {
    pub width: u32,
    pub height: u32,
}
#[cfg(feature = "dxgi1_3")]
pub trait IDecodeSwapChain: Interface {
    fn get_color_space(&self) -> MultiplaneOverlayYCbCrFlags;
    fn get_dest_size(&self) -> Result<DestSize, HResult>;
    fn get_source_rect(&self) -> Result<Rect, HResult>;
    fn get_target_rect(&self) -> Result<Rect, HResult>;
    fn present_buffer(
        &self,
        buffe_to_present: u32,
        sync_interval: u32,
        flags: Present,
    ) -> Result<(), HResult>;
    fn set_color_space(&self, color_space: MultiplaneOverlayYCbCrFlags) -> Result<(), HResult>;
    fn set_dest_size(&self, dest_size: DestSize) -> Result<(), HResult>;
    fn set_source_rect(&self, rect: Rect) -> Result<(), HResult>;
    fn set_target_rect(&self, rect: Rect) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_3")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DecodeSwapChain(ComPtr<IDXGIDecodeSwapChain>);
#[cfg(feature = "dxgi1_3")]
impl_interface!(DecodeSwapChain, IDXGIDecodeSwapChain);
#[cfg(feature = "dxgi1_3")]
impl IDecodeSwapChain for DecodeSwapChain {
    fn get_color_space(&self) -> MultiplaneOverlayYCbCrFlags {
        unsafe { std::mem::transmute(self.0.GetColorSpace()) }
    }
    fn get_dest_size(&self) -> Result<DestSize, HResult> {
        let mut sz = DestSize::default();
        let res = unsafe { self.0.GetDestSize(&mut sz.width, &mut sz.height) };
        hresult(sz, res.into())
    }
    fn get_source_rect(&self) -> Result<Rect, HResult> {
        let mut rc = Default::default();
        let res = unsafe { self.0.GetSourceRect(&mut rc) };
        hresult(rc.into(), res.into())
    }
    fn get_target_rect(&self) -> Result<Rect, HResult> {
        let mut rc = Default::default();
        let res = unsafe { self.0.GetTargetRect(&mut rc) };
        hresult(rc.into(), res.into())
    }
    fn present_buffer(
        &self,
        buffe_to_present: u32,
        sync_interval: u32,
        flags: Present,
    ) -> Result<(), HResult> {
        let res = unsafe {
            self.0
                .PresentBuffer(buffe_to_present, sync_interval, flags.0)
        };
        hresult((), res.into())
    }
    fn set_color_space(&self, color_space: MultiplaneOverlayYCbCrFlags) -> Result<(), HResult> {
        let res = unsafe { self.0.SetColorSpace(color_space.0) };
        hresult((), res.into())
    }
    fn set_dest_size(&self, dest_size: DestSize) -> Result<(), HResult> {
        let res = unsafe { self.0.SetDestSize(dest_size.width, dest_size.height) };
        hresult((), res.into())
    }
    fn set_source_rect(&self, rect: Rect) -> Result<(), HResult> {
        let rc = rect.into();
        let res = unsafe { self.0.SetSourceRect(&rc) };
        hresult((), res.into())
    }
    fn set_target_rect(&self, rect: Rect) -> Result<(), HResult> {
        let rc = rect.into();
        let res = unsafe { self.0.SetTargetRect(&rc) };
        hresult((), res.into())
    }
}

pub trait IDevice: Interface {
    fn create_surface(
        &self,
        desc: &SurfaceDesc,
        num_surfaces: u32,
        usage: Usage,
        shared_resource: &SharedResource,
    ) -> Result<Surface, HResult>;
    fn get_adapter(&self) -> Result<Adapter, HResult>;
    fn get_gpu_thread_priority(&self) -> Result<i32, HResult>;
    fn query_resource_residency(&self, resources: &[&Resource]) -> Result<Vec<Residency>, HResult>;
    fn set_gpu_thread_priority(&self, priority: i32) -> Result<(), HResult>;
}
pub trait IDevice1: IDevice {
    fn get_maximum_frame_latency(&self) -> Result<u32, HResult>;
    fn set_maximum_frame_latency(&self, max_latency: u32) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_2")]
pub trait IDevice2: IDevice1 {
    fn enqueue_set_event(&self, hevent: HANDLE) -> Result<(), HResult>;
    fn offer_resources(
        &self,
        resources: &[&Resource],
        priority: OfferResourcePriority,
    ) -> Result<(), HResult>;
    fn reclaim_resources(&self, resources: &[&Resource]) -> Result<Vec<bool>, HResult>;
}
#[cfg(feature = "dxgi1_3")]
pub trait IDevice3: IDevice2 {
    fn trim(&self);
}
#[cfg(feature = "dxgi1_5")]
pub trait IDevice4: IDevice3 {
    fn offer_resources1(
        &self,
        resources: &[&Resource],
        priority: OfferResourcePriority,
        flags: Option<OfferResourceFlags>,
    ) -> Result<(), HResult>;
    fn reclaim_resources1(
        &self,
        resources: &[&Resource],
    ) -> Result<Vec<ReclaimResourceResults>, HResult>;
}
macro_rules! impl_device {
    ($s: ident, $interface: ident, Device) => {
        impl_interface!($s, $interface);
        impl IDevice for $s {
            fn create_surface(
                &self,
                desc: &SurfaceDesc,
                num_surfaces: u32,
                usage: Usage,
                shared_resource: &SharedResource,
            ) -> Result<Surface, HResult> {
                Ok(Surface(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateSurface(
                            &desc.clone().into(),
                            num_surfaces,
                            usage.0,
                            &shared_resource.clone().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj, res.into())
                })?))
            }
            fn get_adapter(&self) -> Result<Adapter, HResult> {
                Ok(Adapter(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.GetAdapter(&mut obj) };
                    hresult(obj, res.into())
                })?))
            }
            fn get_gpu_thread_priority(&self) -> Result<i32, HResult> {
                let mut priority = 0;
                let res = unsafe { self.0.GetGPUThreadPriority(&mut priority) };
                hresult(priority, res.into())
            }
            fn query_resource_residency(
                &self,
                resources: &[&Resource],
            ) -> Result<Vec<Residency>, HResult> {
                let mut status = Vec::with_capacity(resources.len());
                unsafe {
                    status.set_len(resources.len());
                    let resource_ptrs = resources
                        .iter()
                        .map(|r| r.0.as_ptr() as *mut IUnknown)
                        .collect::<Vec<_>>();
                    let res = self.0.QueryResourceResidency(
                        resource_ptrs.as_ptr(),
                        status.as_mut_ptr(),
                        resources.len() as u32,
                    );
                    hresult(
                        status
                            .into_iter()
                            .map(|s| std::mem::transmute(s))
                            .collect::<Vec<_>>(),
                        res.into(),
                    )
                }
            }
            fn set_gpu_thread_priority(&self, priority: i32) -> Result<(), HResult> {
                let res = unsafe { self.0.SetGPUThreadPriority(priority) };
                hresult((), res.into())
            }
        }
    };
    ($s: ident, $interface: ident, Device1) => {
        impl_device!($s, $interface, Device);
        impl $s {
            pub fn as_device(&self) -> Device {
                Device(self.0.query_interface::<IDXGIDevice>().unwrap())
            }
        }
        impl IDevice1 for $s {
            fn get_maximum_frame_latency(&self) -> Result<u32, HResult> {
                let mut max_latency = 0;
                let res = unsafe { self.0.GetMaximumFrameLatency(&mut max_latency) };
                hresult(max_latency, res.into())
            }
            fn set_maximum_frame_latency(&self, max_latency: u32) -> Result<(), HResult> {
                let res = unsafe { self.0.SetMaximumFrameLatency(max_latency) };
                hresult((), res.into())
            }
        }
    };
    ($s: ident, $interface: ident, Device2) => {
        impl_device!($s, $interface, Device1);
        impl $s {
            pub fn as_device1(&self) -> Device1 {
                Device1(self.0.query_interface::<IDXGIDevice1>().unwrap())
            }
        }
        impl IDevice2 for $s {
            fn enqueue_set_event(&self, hevent: HANDLE) -> Result<(), HResult> {
                let res = unsafe { self.0.EnqueueSetEvent(hevent) };
                hresult((), res.into())
            }
            fn offer_resources(
                &self,
                resources: &[&Resource],
                priority: OfferResourcePriority,
            ) -> Result<(), HResult> {
                let mut resource_ptrs = resources.iter().map(|r| r.0.as_ptr()).collect::<Vec<_>>();
                let res = unsafe {
                    self.0.OfferResources(
                        resources.len() as u32,
                        resource_ptrs.as_mut_ptr(),
                        priority as u32,
                    )
                };
                hresult((), res.into())
            }
            fn reclaim_resources(&self, resources: &[&Resource]) -> Result<Vec<bool>, HResult> {
                let mut resource_ptrs = resources.iter().map(|r| r.0.as_ptr()).collect::<Vec<_>>();
                let mut discards = Vec::with_capacity(resources.len());
                discards.resize(resources.len(), Default::default());
                let res = unsafe {
                    self.0.ReclaimResources(
                        resources.len() as u32,
                        resource_ptrs.as_mut_ptr(),
                        discards.as_mut_ptr(),
                    )
                };
                hresult(
                    discards.into_iter().map(|b| b == TRUE).collect::<Vec<_>>(),
                    res.into(),
                )
            }
        }
    };
    ($s: ident, $interface: ident, Device3) => {
        impl_device!($s, $interface, Device2);
        impl $s {
            pub fn as_device2(&self) -> Device2 {
                Device2(self.0.query_interface::<IDXGIDevice2>().unwrap())
            }
        }
        impl IDevice3 for $s {
            fn trim(&self) {
                unsafe { self.0.Trim() };
            }
        }
    };
    ($s: ident, $interface: ident, Device4) => {
        impl_device!($s, $interface, Device3);
        impl $s {
            pub fn as_device3(&self) -> Device3 {
                Device3(self.0.query_interface::<IDXGIDevice3>().unwrap())
            }
        }
        impl IDevice4 for $s {
            fn offer_resources1(
                &self,
                resources: &[&Resource],
                priority: OfferResourcePriority,
                flags: Option<OfferResourceFlags>,
            ) -> Result<(), HResult> {
                let mut resource_ptrs = resources.iter().map(|r| r.0.as_ptr()).collect::<Vec<_>>();
                let res = unsafe {
                    self.0.OfferResources1(
                        resources.len() as u32,
                        resource_ptrs.as_mut_ptr(),
                        priority as u32,
                        flags.map_or(0, |f| f as u32),
                    )
                };
                hresult((), res.into())
            }
            fn reclaim_resources1(
                &self,
                resources: &[&Resource],
            ) -> Result<Vec<ReclaimResourceResults>, HResult> {
                let mut resource_ptrs = resources.iter().map(|r| r.0.as_ptr()).collect::<Vec<_>>();
                let mut results = Vec::with_capacity(resources.len());
                unsafe {
                    results.set_len(resources.len());
                    let res = self.0.ReclaimResources1(
                        resources.len() as u32,
                        resource_ptrs.as_mut_ptr(),
                        results.as_mut_ptr(),
                    );
                    hresult(
                        results
                            .into_iter()
                            .map(|r| std::mem::transmute(r))
                            .collect::<Vec<_>>(),
                        res,
                    )
                }
            }
        }
    };
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Device(ComPtr<IDXGIDevice>);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Device1(ComPtr<IDXGIDevice1>);
#[cfg(feature = "dxgi1_2")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Device2(ComPtr<IDXGIDevice2>);
#[cfg(feature = "dxgi1_3")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Device3(ComPtr<IDXGIDevice3>);
#[cfg(feature = "dxgi1_5")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Device4(ComPtr<IDXGIDevice4>);
impl_device!(Device, IDXGIDevice, Device);
impl_device!(Device1, IDXGIDevice1, Device1);
#[cfg(feature = "dxgi1_2")]
impl_device!(Device2, IDXGIDevice2, Device2);
#[cfg(feature = "dxgi1_3")]
impl_device!(Device3, IDXGIDevice3, Device3);
#[cfg(feature = "dxgi1_5")]
impl_device!(Device4, IDXGIDevice4, Device4);

#[cfg(feature = "dxgi1_2")]
pub trait IDisplayControl: Interface {
    fn is_stereo_enabled(&self) -> bool;
    fn set_stereo_enabled(&self, enabled: bool);
}
#[cfg(feature = "dxgi1_2")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct DisplayControl(ComPtr<IDXGIDisplayControl>);
#[cfg(feature = "dxgi1_2")]
impl_interface!(DisplayControl, IDXGIDisplayControl);
#[cfg(feature = "dxgi1_2")]
impl IDisplayControl for DisplayControl {
    fn is_stereo_enabled(&self) -> bool {
        unsafe { self.0.IsStereoEnabled() == TRUE }
    }
    fn set_stereo_enabled(&self, enabled: bool) {
        unsafe { self.0.SetStereoEnabled(to_BOOL(enabled)) }
    }
}

#[cfg(feature = "dxgi1_5")]
pub enum FeatureSupoortData {
    Bool(bool),
}

pub trait IFactory: Interface {
    fn create_software_adapter(&self, module: HMODULE) -> Result<Adapter, HResult>;
    fn create_swap_chain<T: Interface>(
        &self,
        device: &T,
        desc: &SwapChainDesc,
    ) -> Result<SwapChain, HResult>;
    fn enum_adapters(&self) -> Result<Vec<Adapter>, HResult>;
    fn get_window_association(&self) -> Result<HWND, HResult>;
    fn make_window_association(
        &self,
        hwnd: HWND,
        flags: Option<MakeWindowAssociationFlag>,
    ) -> Result<(), HResult>;
}
pub trait IFactory1: IFactory {
    fn enum_adapters1(&self) -> Result<Vec<Adapter1>, HResult>;
    fn is_current(&self) -> bool;
}
#[cfg(feature = "dxgi1_2")]
pub trait IFactory2: IFactory1 {
    fn create_swap_chain_for_composition<T: Interface>(
        &self,
        device: &T,
        desc: &SwapChainDesc1,
        restrict_to_output: Option<&Output>,
    ) -> Result<SwapChain1, HResult>;
    fn create_swap_chain_for_core_window<T: Interface, U: Interface>(
        &self,
        device: &T,
        window: &U,
        desc: &SwapChainDesc1,
        restrict_to_output: Option<&Output>,
    ) -> Result<SwapChain1, HResult>;
    fn create_swap_chain_for_hwnd<T: Interface>(
        &self,
        device: &T,
        hwnd: HWND,
        desc: &SwapChainDesc1,
        fullscreen_desc: Option<&SwapChainFullscreenDesc>,
        restrict_to_output: Option<&Output>,
    ) -> Result<SwapChain1, HResult>;
    fn get_shared_resource_adapter_luid(&self, resource: HANDLE) -> Result<Luid, HResult>;
    fn is_windowed_stereo_enabled(&self) -> bool;
    fn register_occlusion_status_event(&self, hevent: HANDLE) -> Result<u32, HResult>;
    fn register_occlusion_status_window(&self, hwnd: HWND, msg: UINT) -> Result<u32, HResult>;
    fn register_stereo_status_event(&self, hevent: HANDLE) -> Result<u32, HResult>;
    fn register_stereo_status_window(&self, hwnd: HWND, msg: UINT) -> Result<u32, HResult>;
    fn unregister_occlusion_status(&self, cookie: u32);
    fn unregister_stereo_status(&self, cookie: u32);
}
#[cfg(feature = "dxgi1_3")]
pub trait IFactory3: IFactory2 {
    fn get_creation_flags(&self) -> CreateFactoryFlag;
}
#[cfg(feature = "dxgi1_4")]
pub trait IFactory4: IFactory3 {
    fn enum_adapter_by_luid<T: IAdapter>(&self, adapter_luid: Luid) -> Result<T, HResult>;
    fn enum_warp_adapter<T: IAdapter>(&self) -> Result<T, HResult>;
}
#[cfg(feature = "dxgi1_5")]
pub trait IFactory5: IFactory4 {
    fn check_feature_support(&self, feature: Feature) -> Result<FeatureSupoortData, HResult>;
}
#[cfg(feature = "dxgi1_6")]
pub trait IFactory6: IFactory5 {
    fn enum_adapter_by_gpu_preference<T: IAdapter>(
        &self,
        gpu_preference: GPUPreference,
    ) -> Result<Vec<T>, HResult>;
}
macro_rules! impl_factory {
    ($s: ident, $interface: ident, Factory) => {
        impl_interface!($s, $interface);
        impl IFactory for $s {
            fn create_software_adapter(&self, module: HMODULE) -> Result<Adapter, HResult> {
                Ok(Adapter(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.CreateSoftwareAdapter(module, &mut obj) };
                    hresult(obj, res.into())
                })?))
            }
            fn create_swap_chain<T: Interface>(
                &self,
                device: &T,
                desc: &SwapChainDesc,
            ) -> Result<SwapChain, HResult> {
                Ok(SwapChain(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let mut desc = desc.clone().into();
                    let res = unsafe {
                        self.0
                            .CreateSwapChain(device.as_unknown(), &mut desc, &mut obj)
                    };
                    hresult(obj, res.into())
                })?))
            }
            fn enum_adapters(&self) -> Result<Vec<Adapter>, HResult> {
                enum_function(DXGI_ERROR_NOT_FOUND.into(), |i| {
                    Ok(Adapter(ComPtr::new(|| {
                        let mut obj = std::ptr::null_mut();
                        let res = unsafe { self.0.EnumAdapters(i, &mut obj) };
                        hresult(obj, res.into())
                    })?))
                })
            }
            fn get_window_association(&self) -> Result<HWND, HResult> {
                let mut hwnd = std::ptr::null_mut();
                let res = unsafe { self.0.GetWindowAssociation(&mut hwnd) };
                hresult(hwnd, res.into())
            }
            fn make_window_association(
                &self,
                hwnd: HWND,
                flags: Option<MakeWindowAssociationFlag>,
            ) -> Result<(), HResult> {
                let res = unsafe { self.0.MakeWindowAssociation(hwnd, flags.map_or(0, |f| f.0)) };
                hresult((), res.into())
            }
        }
    };
    ($s: ident, $interface: ident, Factory1) => {
        impl_factory!($s, $interface, Factory);
        impl $s {
            pub fn as_factory(&self) -> Factory {
                Factory(self.0.query_interface::<IDXGIFactory>().unwrap())
            }
        }
        impl IFactory1 for $s {
            fn enum_adapters1(&self) -> Result<Vec<Adapter1>, HResult> {
                enum_function(DXGI_ERROR_NOT_FOUND.into(), |i| {
                    Ok(Adapter1(ComPtr::new(|| {
                        let mut obj = std::ptr::null_mut();
                        let res = unsafe { self.0.EnumAdapters1(i, &mut obj) };
                        hresult(obj, res.into())
                    })?))
                })
            }
            fn is_current(&self) -> bool {
                unsafe { self.0.IsCurrent() == TRUE }
            }
        }
    };
    ($s: ident, $interface: ident, Factory2) => {
        impl_factory!($s, $interface, Factory1);
        impl $s {
            pub fn as_factory1(&self) -> Factory1 {
                Factory1(self.0.query_interface::<IDXGIFactory1>().unwrap())
            }
        }
        impl IFactory2 for $s {
            fn create_swap_chain_for_composition<T: Interface>(
                &self,
                device: &T,
                desc: &SwapChainDesc1,
                restrict_to_output: Option<&Output>,
            ) -> Result<SwapChain1, HResult> {
                Ok(SwapChain1(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let desc = desc.clone().into();
                    let res = unsafe {
                        self.0.CreateSwapChainForComposition(
                            device.as_unknown(),
                            &desc,
                            restrict_to_output
                                .map_or(std::ptr::null_mut(), |output| output.0.as_ptr()),
                            &mut obj,
                        )
                    };
                    hresult(obj, res.into())
                })?))
            }
            fn create_swap_chain_for_core_window<T: Interface, U: Interface>(
                &self,
                device: &T,
                window: &U,
                desc: &SwapChainDesc1,
                restrict_to_output: Option<&Output>,
            ) -> Result<SwapChain1, HResult> {
                Ok(SwapChain1(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let desc = desc.clone().into();
                    let res = unsafe {
                        self.0.CreateSwapChainForCoreWindow(
                            device.as_unknown(),
                            window.as_unknown(),
                            &desc,
                            restrict_to_output
                                .map_or(std::ptr::null_mut(), |output| output.0.as_ptr()),
                            &mut obj,
                        )
                    };
                    hresult(obj, res.into())
                })?))
            }
            fn create_swap_chain_for_hwnd<T: Interface>(
                &self,
                device: &T,
                hwnd: HWND,
                desc: &SwapChainDesc1,
                fullscreen_desc: Option<&SwapChainFullscreenDesc>,
                restrict_to_output: Option<&Output>,
            ) -> Result<SwapChain1, HResult> {
                Ok(SwapChain1(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let desc = desc.clone().into();
                    let fullscreen_desc = fullscreen_desc.map(|fd| fd.clone().into());
                    let res = unsafe {
                        self.0.CreateSwapChainForHwnd(
                            device.as_unknown(),
                            hwnd,
                            &desc,
                            fullscreen_desc.map_or(std::ptr::null(), |fd| &fd),
                            restrict_to_output
                                .map_or(std::ptr::null_mut(), |output| output.0.as_ptr()),
                            &mut obj,
                        )
                    };
                    hresult(obj, res.into())
                })?))
            }
            fn get_shared_resource_adapter_luid(&self, resource: HANDLE) -> Result<Luid, HResult> {
                let mut luid = Default::default();
                let res = unsafe { self.0.GetSharedResourceAdapterLuid(resource, &mut luid) };
                hresult(luid.into(), res.into())
            }
            fn is_windowed_stereo_enabled(&self) -> bool {
                unsafe { self.0.IsWindowedStereoEnabled() == TRUE }
            }
            fn register_occlusion_status_event(&self, hevent: HANDLE) -> Result<u32, HResult> {
                let mut cookie = 0;
                let res = unsafe { self.0.RegisterOcclusionStatusEvent(hevent, &mut cookie) };
                hresult(cookie, res.into())
            }
            fn register_occlusion_status_window(
                &self,
                hwnd: HWND,
                msg: UINT,
            ) -> Result<u32, HResult> {
                let mut cookie = 0;
                let res = unsafe { self.0.RegisterOcclusionStatusWindow(hwnd, msg, &mut cookie) };
                hresult(cookie, res.into())
            }
            fn register_stereo_status_event(&self, hevent: HANDLE) -> Result<u32, HResult> {
                let mut cookie = 0;
                let res = unsafe { self.0.RegisterStereoStatusEvent(hevent, &mut cookie) };
                hresult(cookie, res.into())
            }
            fn register_stereo_status_window(&self, hwnd: HWND, msg: UINT) -> Result<u32, HResult> {
                let mut cookie = 0;
                let res = unsafe { self.0.RegisterStereoStatusWindow(hwnd, msg, &mut cookie) };
                hresult(cookie, res.into())
            }
            fn unregister_occlusion_status(&self, cookie: u32) {
                unsafe { self.0.UnregisterOcclusionStatus(cookie) }
            }
            fn unregister_stereo_status(&self, cookie: u32) {
                unsafe { self.0.UnregisterStereoStatus(cookie) }
            }
        }
    };
    ($s: ident, $interface: ident, Factory3) => {
        impl_factory!($s, $interface, Factory2);
        impl $s {
            pub fn as_factory2(&self) -> Factory2 {
                Factory2(self.0.query_interface::<IDXGIFactory2>().unwrap())
            }
        }
        impl IFactory3 for $s {
            fn get_creation_flags(&self) -> CreateFactoryFlag {
                unsafe { CreateFactoryFlag(self.0.GetCreationFlags()) }
            }
        }
    };
    ($s: ident, $interface: ident, Factory4) => {
        impl_factory!($s, $interface, Factory3);
        impl $s {
            pub fn as_factory3(&self) -> Factory3 {
                Factory3(self.0.query_interface::<IDXGIFactory3>().unwrap())
            }
        }
        impl IFactory4 for $s {
            fn enum_adapter_by_luid<T: IAdapter>(&self, adapter_luid: Luid) -> Result<T, HResult> {
                Ok(T::from_com_ptr(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0
                            .EnumAdapterByLuid(adapter_luid.into(), &T::uuidof().into(), &mut obj)
                    };
                    hresult(obj as *mut T::APIType, res.into())
                })?))
            }
            fn enum_warp_adapter<T: IAdapter>(&self) -> Result<T, HResult> {
                Ok(T::from_com_ptr(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.EnumWarpAdapter(&T::uuidof().into(), &mut obj) };
                    hresult(obj as *mut T::APIType, res.into())
                })?))
            }
        }
    };
    ($s: ident, $interface: ident, Factory5) => {
        impl_factory!($s, $interface, Factory4);
        impl $s {
            pub fn as_fatory4(&self) -> Factory4 {
                Factory4(self.0.query_interface::<IDXGIFactory4>().unwrap())
            }
        }
        impl IFactory5 for $s {
            fn check_feature_support(
                &self,
                feature: Feature,
            ) -> Result<FeatureSupoortData, HResult> {
                match feature {
                    Feature::PresentAllowTearing => {
                        let mut value: BOOL = FALSE;
                        let res = unsafe {
                            self.0.CheckFeatureSupport(
                                feature as u32,
                                &mut value as *mut i32 as *mut c_void,
                                std::mem::size_of::<BOOL>() as u32,
                            )
                        };
                        hresult(FeatureSupoortData::Bool(value == TRUE), res)
                    }
                }
            }
        }
    };
    ($s: ident, $interface: ident, Factory6) => {
        impl_factory!($s, $interface, Factory5);
        impl $s {
            pub fn as_factory5(&self) -> Factory5 {
                Factory5(self.0.query_interface::<IDXGIFactory5>().unwrap())
            }
        }
        impl IFactory6 for $s {
            fn enum_adapter_by_gpu_preference<T: IAdapter>(
                &self,
                gpu_preference: GPUPreference,
            ) -> Result<Vec<T>, HResult> {
                enum_function(DXGI_ERROR_NOT_FOUND.into(), |i| {
                    Ok(T::from_com_ptr(ComPtr::new(|| {
                        let mut obj = std::ptr::null_mut();
                        let res = unsafe {
                            self.0.EnumAdapterByGpuPreference(
                                i,
                                gpu_preference as u32,
                                &T::uuidof().into(),
                                &mut obj,
                            )
                        };
                        hresult(obj as *mut T::APIType, res)
                    })?))
                })
            }
        }
    };
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Factory(ComPtr<IDXGIFactory>);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Factory1(ComPtr<IDXGIFactory1>);
#[cfg(feature = "dxgi1_2")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Factory2(ComPtr<IDXGIFactory2>);
#[cfg(feature = "dxgi1_3")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Factory3(ComPtr<IDXGIFactory3>);
#[cfg(feature = "dxgi1_4")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Factory4(ComPtr<IDXGIFactory4>);
#[cfg(feature = "dxgi1_5")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Factory5(ComPtr<IDXGIFactory5>);
#[cfg(feature = "dxgi1_6")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Factory6(ComPtr<IDXGIFactory6>);
impl_factory!(Factory, IDXGIFactory, Factory);
impl_factory!(Factory1, IDXGIFactory1, Factory1);
#[cfg(feature = "dxgi1_2")]
impl_factory!(Factory2, IDXGIFactory2, Factory2);
#[cfg(feature = "dxgi1_3")]
impl_factory!(Factory3, IDXGIFactory3, Factory3);
#[cfg(feature = "dxgi1_4")]
impl_factory!(Factory4, IDXGIFactory4, Factory4);
#[cfg(feature = "dxgi1_5")]
impl_factory!(Factory5, IDXGIFactory5, Factory5);
#[cfg(feature = "dxgi1_6")]
impl_factory!(Factory6, IDXGIFactory6, Factory6);

#[cfg(feature = "dxgi1_3")]
pub trait IFactoryMedia {
    fn create_decode_swap_chain_for_composition_surface_handle<T: Interface>(
        &self,
        device: &T,
        surface: HANDLE,
        desc: &DecodeSwapChainDesc,
        yuv_decode_buffers: &Resource,
        restrict_to_output: Option<&Output>,
    ) -> Result<DecodeSwapChain, HResult>;
    fn create_swap_chain_for_composition_surface_handle<T: Interface>(
        &self,
        device: &T,
        surface: HANDLE,
        desc: &SwapChainDesc1,
        restrict_to_output: Option<&Output>,
    ) -> Result<SwapChain1, HResult>;
}
#[cfg(feature = "dxgi1_3")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct FactoryMedia(ComPtr<IDXGIFactoryMedia>);
#[cfg(feature = "dxgi1_3")]
impl_interface!(FactoryMedia, IDXGIFactoryMedia);
#[cfg(feature = "dxgi1_3")]
impl IFactoryMedia for FactoryMedia {
    fn create_decode_swap_chain_for_composition_surface_handle<T: Interface>(
        &self,
        device: &T,
        surface: HANDLE,
        desc: &DecodeSwapChainDesc,
        yuv_decode_buffers: &Resource,
        restrict_to_output: Option<&Output>,
    ) -> Result<DecodeSwapChain, HResult> {
        Ok(DecodeSwapChain(ComPtr::new(|| {
            let mut obj = std::ptr::null_mut();
            let mut desc = desc.clone().into();
            let res = unsafe {
                self.0.CreateDecodeSwapChainForCompositionSurfaceHandle(
                    device.as_unknown(),
                    surface,
                    &mut desc,
                    yuv_decode_buffers.0.as_ptr(),
                    restrict_to_output.map_or(std::ptr::null_mut(), |output| output.0.as_ptr()),
                    &mut obj,
                )
            };
            hresult(obj, res.into())
        })?))
    }
    fn create_swap_chain_for_composition_surface_handle<T: Interface>(
        &self,
        device: &T,
        surface: HANDLE,
        desc: &SwapChainDesc1,
        restrict_to_output: Option<&Output>,
    ) -> Result<SwapChain1, HResult> {
        Ok(SwapChain1(ComPtr::new(|| {
            let mut obj = std::ptr::null_mut();
            let mut desc = desc.clone().into();
            let res = unsafe {
                self.0.CreateSwapChainForCompositionSurfaceHandle(
                    device.as_unknown(),
                    surface,
                    &mut desc,
                    restrict_to_output.map_or(std::ptr::null_mut(), |output| output.0.as_ptr()),
                    &mut obj,
                )
            };
            hresult(obj, res.into())
        })?))
    }
}

pub trait IInfoQueue {
    fn add_application_message<T: AsRef<str>>(
        &self,
        severity: InfoQueueMessageSeverity,
        description: T,
    ) -> Result<(), HResult>;
    fn add_message<T: AsRef<str>>(
        &self,
        producer: DebugID,
        category: InfoQueueMessageCategory,
        severity: InfoQueueMessageSeverity,
        id: InfoQueueMessageID,
        description: T,
    ) -> Result<(), HResult>;
    fn add_retrieval_filter_entries(
        &self,
        producer: DebugID,
        filter: &[InfoQueueFilter],
    ) -> Result<(), HResult>;
    fn add_storage_filter_entries(
        &self,
        producer: DebugID,
        filter: &[InfoQueueFilter],
    ) -> Result<(), HResult>;
    fn clear_retrieval_filter(&self, producer: DebugID);
    fn clear_storage_filter(&self, producer: DebugID);
    fn clear_stored_messages(&self, producer: DebugID);
    fn get_break_on_category(&self, producer: DebugID, category: InfoQueueMessageCategory) -> bool;
    fn get_break_on_id(&self, producer: DebugID, id: InfoQueueMessageID) -> bool;
    fn get_break_on_severity(&self, producer: DebugID, severity: InfoQueueMessageSeverity) -> bool;
    fn get_message(
        &self,
        producer: DebugID,
        message_index: u64,
    ) -> Result<InfoQueueMessage, HResult>;
    fn get_message_count_limit(&self, producer: DebugID) -> u64;
    fn get_mute_debug_output(&self, producer: DebugID) -> bool;
    fn get_num_messages_allowed_by_storage_filter(&self, producer: DebugID) -> u64;
    fn get_num_messages_denied_by_storage_filter(&self, producer: DebugID) -> u64;
    fn get_num_messages_discarded_by_message_count_limit(&self, producer: DebugID) -> u64;
    fn get_num_stored_messages(&self, producer: DebugID) -> u64;
    fn get_num_stored_messages_allowed_by_retrieval_filters(&self, producer: DebugID) -> u64;
    fn get_retrieval_filter(&self, producer: DebugID) -> Result<InfoQueueFilter, HResult>;
    fn get_retrieval_filter_stack_size(&self, producer: DebugID) -> u32;
    fn get_storage_filter(&self, producer: DebugID) -> Result<InfoQueueFilter, HResult>;
    fn get_storage_filter_stack_size(&self, producer: DebugID) -> u32;
    fn pop_retrieval_filter(&self, producer: DebugID);
    fn pop_storage_filter(&self, producer: DebugID);
    fn push_copy_of_retrieval_filter(&self, producer: DebugID) -> Result<(), HResult>;
    fn push_copy_of_storage_filter(&self, producer: DebugID) -> Result<(), HResult>;
    fn push_deny_all_retrieval_filter(&self, producer: DebugID) -> Result<(), HResult>;
    fn push_deny_all_storage_filter(&self, producer: DebugID) -> Result<(), HResult>;
    fn push_empty_retrieval_filter(&self, producer: DebugID) -> Result<(), HResult>;
    fn push_empty_storage_filter(&self, producer: DebugID) -> Result<(), HResult>;
    fn push_retrieval_filter(
        &self,
        producer: DebugID,
        filter: &InfoQueueFilter,
    ) -> Result<(), HResult>;
    fn push_storage_filter(
        &self,
        producer: DebugID,
        filter: &InfoQueueFilter,
    ) -> Result<(), HResult>;
    fn set_break_on_category(
        &self,
        producer: DebugID,
        category: InfoQueueMessageCategory,
        enable: bool,
    ) -> Result<(), HResult>;
    fn set_break_on_id(
        &self,
        producer: DebugID,
        id: InfoQueueMessageID,
        enable: bool,
    ) -> Result<(), HResult>;
    fn set_break_on_severity(
        &self,
        producer: DebugID,
        severity: InfoQueueMessageSeverity,
        enable: bool,
    ) -> Result<(), HResult>;
    fn set_message_count_limit(
        &self,
        producer: DebugID,
        message_count_limit: u64,
    ) -> Result<(), HResult>;
    fn set_mute_debug_output(&self, producer: DebugID, mute: bool);
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct InfoQueue(ComPtr<IDXGIInfoQueue>);
impl IInfoQueue for InfoQueue {
    fn add_application_message<T: AsRef<str>>(
        &self,
        severity: InfoQueueMessageSeverity,
        description: T,
    ) -> Result<(), HResult> {
        let description = std::ffi::CString::new(description.as_ref().as_bytes()).unwrap();
        let res = unsafe {
            self.0
                .AddApplicationMessage(severity as u32, description.as_ptr())
        };
        hresult((), res)
    }
    fn add_message<T: AsRef<str>>(
        &self,
        producer: DebugID,
        category: InfoQueueMessageCategory,
        severity: InfoQueueMessageSeverity,
        id: InfoQueueMessageID,
        description: T,
    ) -> Result<(), HResult> {
        let description = std::ffi::CString::new(description.as_ref().as_bytes()).unwrap();
        let res = unsafe {
            self.0.AddMessage(
                producer.into(),
                category as u32,
                severity as u32,
                id,
                description.as_ptr(),
            )
        };
        hresult((), res)
    }
    fn add_retrieval_filter_entries(
        &self,
        producer: DebugID,
        filter: &[InfoQueueFilter],
    ) -> Result<(), HResult> {
        let filter = filter.iter().cloned().map(|f| f.into()).collect::<Vec<_>>();
        let res = unsafe {
            self.0
                .AddRetrievalFilterEntries(producer.into(), filter.as_ptr())
        };
        hresult((), res)
    }
    fn add_storage_filter_entries(
        &self,
        producer: DebugID,
        filter: &[InfoQueueFilter],
    ) -> Result<(), HResult> {
        let filter = filter.iter().cloned().map(|f| f.into()).collect::<Vec<_>>();
        let res = unsafe {
            self.0
                .AddStorageFilterEntries(producer.into(), filter.as_ptr())
        };
        hresult((), res)
    }
    fn clear_retrieval_filter(&self, producer: DebugID) {
        unsafe { self.0.ClearRetrievalFilter(producer.into()) }
    }
    fn clear_storage_filter(&self, producer: DebugID) {
        unsafe { self.0.ClearRetrievalFilter(producer.into()) }
    }
    fn clear_stored_messages(&self, producer: DebugID) {
        unsafe { self.0.ClearStoredMessages(producer.into()) }
    }
    fn get_break_on_category(&self, producer: DebugID, category: InfoQueueMessageCategory) -> bool {
        unsafe { self.0.GetBreakOnCategory(producer.into(), category as u32) == TRUE }
    }
    fn get_break_on_id(&self, producer: DebugID, id: InfoQueueMessageID) -> bool {
        unsafe { self.0.GetBreakOnID(producer.into(), id) == TRUE }
    }
    fn get_break_on_severity(&self, producer: DebugID, severity: InfoQueueMessageSeverity) -> bool {
        unsafe { self.0.GetBreakOnSeverity(producer.into(), severity as u32) == TRUE }
    }
    fn get_message(&self, producer: DebugID, index: u64) -> Result<InfoQueueMessage, HResult> {
        let mut len = 0;
        let res = unsafe {
            self.0
                .GetMessage(producer.into(), index, std::ptr::null_mut(), &mut len)
        };
        let mut len = hresult(len, res.into())?;
        unsafe {
            let mut msg: Vec<u8> = Vec::with_capacity(len);
            msg.set_len(len);
            let res = self.0.GetMessage(
                producer.into(),
                index,
                msg.as_mut_ptr() as *mut DXGI_INFO_QUEUE_MESSAGE,
                &mut len,
            );
            let src = msg.as_mut_ptr() as *const DXGI_INFO_QUEUE_MESSAGE;
            hresult(
                InfoQueueMessage {
                    producer: (*src).Producer.into(),
                    category: std::mem::transmute((*src).Category),
                    severity: std::mem::transmute((*src).Severity),
                    id: (*src).ID,
                    description: {
                        let d = std::slice::from_raw_parts(
                            (*src).pDescription as *const u8,
                            (*src).DescriptionByteLength,
                        );
                        let cstr = std::ffi::CStr::from_bytes_with_nul_unchecked(d);
                        cstr.to_str()
                            .map(|s| s.to_string())
                            .unwrap_or(String::new())
                    },
                },
                res,
            )
        }
    }
    fn get_message_count_limit(&self, producer: DebugID) -> u64 {
        unsafe { self.0.GetMessageCountLimit(producer.into()) }
    }
    fn get_mute_debug_output(&self, producer: DebugID) -> bool {
        unsafe { self.0.GetMuteDebugOutput(producer.into()) == TRUE }
    }
    fn get_num_messages_allowed_by_storage_filter(&self, producer: DebugID) -> u64 {
        unsafe { self.0.GetNumMessagesAllowedByStorageFilter(producer.into()) }
    }
    fn get_num_messages_denied_by_storage_filter(&self, producer: DebugID) -> u64 {
        unsafe { self.0.GetNumMessagesDeniedByStorageFilter(producer.into()) }
    }
    fn get_num_messages_discarded_by_message_count_limit(&self, producer: DebugID) -> u64 {
        unsafe {
            self.0
                .GetNumMessagesDiscardedByMessageCountLimit(producer.into())
        }
    }
    fn get_num_stored_messages(&self, producer: DebugID) -> u64 {
        unsafe { self.0.GetNumStoredMessages(producer.into()) }
    }
    fn get_num_stored_messages_allowed_by_retrieval_filters(&self, producer: DebugID) -> u64 {
        unsafe {
            self.0
                .GetNumStoredMessagesAllowedByRetrievalFilters(producer.into())
        }
    }
    fn get_retrieval_filter(&self, producer: DebugID) -> Result<InfoQueueFilter, HResult> {
        let mut len = 0;
        let res = unsafe {
            self.0
                .GetRetrievalFilter(producer.into(), std::ptr::null_mut(), &mut len)
        };
        let mut len = hresult(len, res)?;
        unsafe {
            let mut buf: Vec<u8> = Vec::with_capacity(len);
            buf.set_len(len);
            let res = self.0.GetRetrievalFilter(
                producer.into(),
                buf.as_mut_ptr() as *mut DXGI_INFO_QUEUE_FILTER,
                &mut len,
            );
            let p = buf.as_ptr() as *const DXGI_INFO_QUEUE_FILTER;
            hresult((*p).into(), res.into())
        }
    }
    fn get_retrieval_filter_stack_size(&self, producer: DebugID) -> u32 {
        unsafe { self.0.GetRetrievalFilterStackSize(producer.into()) }
    }
    fn get_storage_filter(&self, producer: DebugID) -> Result<InfoQueueFilter, HResult> {
        let mut len = 0;
        let res = unsafe {
            self.0
                .GetStorageFilter(producer.into(), std::ptr::null_mut(), &mut len)
        };
        let mut len = hresult(len, res)?;
        unsafe {
            let mut buf: Vec<u8> = Vec::with_capacity(len);
            buf.set_len(len);
            let res = self.0.GetStorageFilter(
                producer.into(),
                buf.as_mut_ptr() as *mut DXGI_INFO_QUEUE_FILTER,
                &mut len,
            );
            let p = buf.as_ptr() as *const DXGI_INFO_QUEUE_FILTER;
            hresult((*p).into(), res.into())
        }
    }
    fn get_storage_filter_stack_size(&self, producer: DebugID) -> u32 {
        unsafe { self.0.GetStorageFilterStackSize(producer.into()) }
    }
    fn pop_retrieval_filter(&self, producer: DebugID) {
        unsafe { self.0.PopRetrievalFilter(producer.into()) }
    }
    fn pop_storage_filter(&self, producer: DebugID) {
        unsafe { self.0.PopStorageFilter(producer.into()) }
    }
    fn push_copy_of_retrieval_filter(&self, producer: DebugID) -> Result<(), HResult> {
        let res = unsafe { self.0.PushCopyOfRetrievalFilter(producer.into()) };
        hresult((), res.into())
    }
    fn push_copy_of_storage_filter(&self, producer: DebugID) -> Result<(), HResult> {
        let res = unsafe { self.0.PushCopyOfStorageFilter(producer.into()) };
        hresult((), res.into())
    }
    fn push_deny_all_retrieval_filter(&self, producer: DebugID) -> Result<(), HResult> {
        let res = unsafe { self.0.PushDenyAllRetrievalFilter(producer.into()) };
        hresult((), res.into())
    }
    fn push_deny_all_storage_filter(&self, producer: DebugID) -> Result<(), HResult> {
        let res = unsafe { self.0.PushDenyAllStorageFilter(producer.into()) };
        hresult((), res.into())
    }
    fn push_empty_retrieval_filter(&self, producer: DebugID) -> Result<(), HResult> {
        let res = unsafe { self.0.PushEmptyRetrievalFilter(producer.into()) };
        hresult((), res.into())
    }
    fn push_empty_storage_filter(&self, producer: DebugID) -> Result<(), HResult> {
        let res = unsafe { self.0.PushEmptyStorageFilter(producer.into()) };
        hresult((), res.into())
    }
    fn push_retrieval_filter(
        &self,
        producer: DebugID,
        filter: &InfoQueueFilter,
    ) -> Result<(), HResult> {
        let res = unsafe {
            self.0
                .PushRetrievalFilter(producer.into(), &filter.clone().into())
        };
        hresult((), res.into())
    }
    fn push_storage_filter(
        &self,
        producer: DebugID,
        filter: &InfoQueueFilter,
    ) -> Result<(), HResult> {
        let res = unsafe {
            self.0
                .PushStorageFilter(producer.into(), &filter.clone().into())
        };
        hresult((), res.into())
    }
    fn set_break_on_category(
        &self,
        producer: DebugID,
        category: InfoQueueMessageCategory,
        enable: bool,
    ) -> Result<(), HResult> {
        let res = unsafe {
            self.0
                .SetBreakOnCategory(producer.into(), category as u32, to_BOOL(enable))
        };
        hresult((), res.into())
    }
    fn set_break_on_id(
        &self,
        producer: DebugID,
        id: InfoQueueMessageID,
        enable: bool,
    ) -> Result<(), HResult> {
        let res = unsafe { self.0.SetBreakOnID(producer.into(), id, to_BOOL(enable)) };
        hresult((), res.into())
    }
    fn set_break_on_severity(
        &self,
        producer: DebugID,
        severity: InfoQueueMessageSeverity,
        enable: bool,
    ) -> Result<(), HResult> {
        let res = unsafe {
            self.0
                .SetBreakOnSeverity(producer.into(), severity as u32, to_BOOL(enable))
        };
        hresult((), res.into())
    }
    fn set_message_count_limit(
        &self,
        producer: DebugID,
        message_count_limit: u64,
    ) -> Result<(), HResult> {
        let res = unsafe {
            self.0
                .SetMessageCountLimit(producer.into(), message_count_limit)
        };
        hresult((), res.into())
    }
    fn set_mute_debug_output(&self, producer: DebugID, mute: bool) {
        unsafe { self.0.SetMuteDebugOutput(producer.into(), to_BOOL(mute)) }
    }
}

pub trait IKeyedMutex {
    fn acquire_sync(&self, key: u64, ms: u32) -> Result<(), HResult>;
    fn release_sync(&self, key: u64) -> Result<(), HResult>;
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct KeyedMutex(ComPtr<IDXGIKeyedMutex>);
impl_interface!(KeyedMutex, IDXGIKeyedMutex);
impl IKeyedMutex for KeyedMutex {
    fn acquire_sync(&self, key: u64, ms: u32) -> Result<(), HResult> {
        let res = unsafe { self.0.AcquireSync(key, ms) };
        hresult((), res.into())
    }
    fn release_sync(&self, key: u64) -> Result<(), HResult> {
        let res = unsafe { self.0.ReleaseSync(key) };
        hresult((), res.into())
    }
}

pub trait IOutput {
    fn find_closest_matching_mode<T: Interface>(
        &self,
        mode_to_match: &ModeDesc,
        concerned_device: Option<&T>,
    ) -> Result<ModeDesc, HResult>;
    fn get_desc(&self) -> Result<OutputDesc, HResult>;
    fn get_display_mode_list(
        &self,
        enum_format: Format,
        flags: Option<EnumModes>,
    ) -> Result<Vec<ModeDesc>, HResult>;
    fn get_display_surface_data(&self, destination: &Surface) -> Result<(), HResult>;
    fn get_frame_statistics(&self) -> Result<FrameStatistics, HResult>;
    fn get_gamma_control(&self) -> Result<GammaControl, HResult>;
    fn get_gamma_control_capabilities(&self) -> Result<GammaControlCapabilities, HResult>;
    fn release_ownership(&self);
    fn set_display_surface(&self, scanout_surface: &Surface) -> Result<(), HResult>;
    fn set_gamma_control(&self, array: &GammaControl) -> Result<(), HResult>;
    fn take_ownership<T: Interface>(&self, device: &T, exclusive: bool) -> Result<(), HResult>;
    fn wait_for_vblank(&self) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_2")]
pub trait IOutput1: IOutput {
    fn duplicate_output<T: Interface>(&self, device: &T) -> Result<OutputDuplication, HResult>;
    fn find_closest_matching_mode1<T: Interface>(
        &self,
        mode_to_match: &ModeDesc1,
        concerned_device: Option<&T>,
    ) -> Result<ModeDesc1, HResult>;
    fn get_display_mode_list1(
        &self,
        enum_format: Format,
        flags: Option<EnumModes>,
    ) -> Result<Vec<ModeDesc1>, HResult>;
    fn get_display_surface_data1(&self, destination: &Resource) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_3")]
pub trait IOutput2: IOutput1 {
    fn supports_overlays(&self) -> bool;
}
#[cfg(feature = "dxgi1_3")]
pub trait IOutput3: IOutput2 {
    fn check_overlay_support<T: Interface>(
        &self,
        format: Format,
        concerned_device: &T,
    ) -> Result<OverlaySupportFlag, HResult>;
}
#[cfg(feature = "dxgi1_4")]
pub trait IOutput4: IOutput3 {
    fn check_overlay_color_space_support<T: Interface>(
        &self,
        format: Format,
        color_space: ColorSpaceType,
        concerned_device: &T,
    ) -> Result<OverlayColorSpaceSupportFlag, HResult>;
}
#[cfg(feature = "dxgi1_5")]
pub trait IOutput5: IOutput4 {
    fn duplicate_output1<T: Interface>(
        &self,
        device: &T,
        flags: u32,
        formats: &[Format],
    ) -> Result<OutputDuplication, HResult>;
}
#[cfg(feature = "dxgi1_6")]
pub trait IOutput6: IOutput5 {
    fn check_hardware_composition_support(
        &self,
    ) -> Result<HardwareCompositionSupportFlags, HResult>;
    fn get_desc1(&self) -> Result<OutputDesc1, HResult>;
}
macro_rules! impl_output {
    ($s: ident, $interface: ident, Output) => {
        impl_interface!($s, $interface);
        impl IOutput for $s {
            fn find_closest_matching_mode<T: Interface>(
                &self,
                mode_to_match: &ModeDesc,
                concerned_device: Option<&T>,
            ) -> Result<ModeDesc, HResult> {
                let mut desc = Default::default();
                let res = unsafe {
                    self.0.FindClosestMatchingMode(
                        &mode_to_match.clone().into(),
                        &mut desc,
                        concerned_device.map_or(std::ptr::null_mut(), |p| p.as_unknown()),
                    )
                };
                hresult(desc.into(), res.into())
            }
            fn get_desc(&self) -> Result<OutputDesc, HResult> {
                let mut desc = Default::default();
                let res = unsafe { self.0.GetDesc(&mut desc) };
                hresult(desc.into(), res.into())
            }
            fn get_display_mode_list(
                &self,
                format: Format,
                flags: Option<EnumModes>,
            ) -> Result<Vec<ModeDesc>, HResult> {
                let mut len = 0;
                let flags = flags.map_or(0, |f| f.0);
                let res = unsafe {
                    self.0
                        .GetDisplayModeList(format as u32, flags, &mut len, std::ptr::null_mut())
                };
                let mut len = hresult(len, res.into())?;
                unsafe {
                    let mut descs = Vec::with_capacity(len as usize);
                    descs.set_len(len as usize);
                    let res = self.0.GetDisplayModeList(
                        format as u32,
                        flags,
                        &mut len,
                        descs.as_mut_ptr(),
                    );
                    hresult(
                        descs
                            .into_iter()
                            .map(|desc| desc.into())
                            .collect::<Vec<_>>(),
                        res.into(),
                    )
                }
            }
            fn get_display_surface_data(&self, destination: &Surface) -> Result<(), HResult> {
                let res = unsafe { self.0.GetDisplaySurfaceData(destination.0.as_ptr()) };
                hresult((), res.into())
            }
            fn get_frame_statistics(&self) -> Result<FrameStatistics, HResult> {
                let mut obj = Default::default();
                let res = unsafe { self.0.GetFrameStatistics(&mut obj) };
                hresult(obj.into(), res.into())
            }
            fn get_gamma_control(&self) -> Result<GammaControl, HResult> {
                let mut obj = Default::default();
                let res = unsafe { self.0.GetGammaControl(&mut obj) };
                hresult(obj.into(), res.into())
            }
            fn get_gamma_control_capabilities(&self) -> Result<GammaControlCapabilities, HResult> {
                let mut obj = Default::default();
                let res = unsafe { self.0.GetGammaControlCapabilities(&mut obj) };
                hresult(obj.into(), res.into())
            }
            fn release_ownership(&self) {
                unsafe { self.0.ReleaseOwnership() };
            }
            fn set_display_surface(&self, scanout_surface: &Surface) -> Result<(), HResult> {
                let res = unsafe { self.0.SetDisplaySurface(scanout_surface.0.as_ptr()) };
                hresult((), res)
            }
            fn set_gamma_control(&self, array: &GammaControl) -> Result<(), HResult> {
                let res = unsafe { self.0.SetGammaControl(&array.clone().into()) };
                hresult((), res)
            }
            fn take_ownership<T: Interface>(
                &self,
                device: &T,
                exclusive: bool,
            ) -> Result<(), HResult> {
                let res = unsafe {
                    self.0
                        .TakeOwnership(device.as_unknown(), to_BOOL(exclusive))
                };
                hresult((), res)
            }
            fn wait_for_vblank(&self) -> Result<(), HResult> {
                let res = unsafe { self.0.WaitForVBlank() };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, Output1) => {
        impl_output!($s, $interface, Output);
        impl $s {
            pub fn as_output(&self) -> Output {
                Output(self.0.query_interface::<IDXGIOutput>().unwrap())
            }
        }
        impl IOutput1 for $s {
            fn duplicate_output<T: Interface>(
                &self,
                device: &T,
            ) -> Result<OutputDuplication, HResult> {
                Ok(OutputDuplication(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.DuplicateOutput(device.as_unknown(), &mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn find_closest_matching_mode1<T: Interface>(
                &self,
                mode_to_match: &ModeDesc1,
                concerned_device: Option<&T>,
            ) -> Result<ModeDesc1, HResult> {
                let mut desc = Default::default();
                let concerned_device =
                    concerned_device.map_or(std::ptr::null_mut(), |cd| cd.as_unknown());
                let res = unsafe {
                    self.0.FindClosestMatchingMode1(
                        &mode_to_match.clone().into(),
                        &mut desc,
                        concerned_device,
                    )
                };
                hresult(desc.into(), res)
            }
            fn get_display_mode_list1(
                &self,
                enum_format: Format,
                flags: Option<EnumModes>,
            ) -> Result<Vec<ModeDesc1>, HResult> {
                let mut len = 0;
                let flags = flags.map_or(0, |f| f.0);
                let res = unsafe {
                    self.0.GetDisplayModeList1(
                        enum_format as u32,
                        flags,
                        &mut len,
                        std::ptr::null_mut(),
                    )
                };
                let mut len = hresult(len, res)?;
                unsafe {
                    let mut descs = Vec::with_capacity(len as usize);
                    descs.set_len(len as usize);
                    let res = self.0.GetDisplayModeList1(
                        enum_format as u32,
                        flags,
                        &mut len,
                        descs.as_mut_ptr(),
                    );
                    hresult(
                        descs
                            .into_iter()
                            .map(|desc| desc.into())
                            .collect::<Vec<_>>(),
                        res,
                    )
                }
            }
            fn get_display_surface_data1(&self, destination: &Resource) -> Result<(), HResult> {
                let res = unsafe { self.0.GetDisplaySurfaceData1(destination.0.as_ptr()) };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, Output2) => {
        impl_output!($s, $interface, Output1);
        impl $s {
            pub fn as_output1(&self) -> Output1 {
                Output1(self.0.query_interface::<IDXGIOutput1>().unwrap())
            }
        }
        impl IOutput2 for $s {
            fn supports_overlays(&self) -> bool {
                unsafe { self.0.SupportsOverlays() == TRUE }
            }
        }
    };
    ($s: ident, $interface: ident, Output3) => {
        impl_output!($s, $interface, Output2);
        impl $s {
            pub fn as_output2(&self) -> Output2 {
                Output2(self.0.query_interface::<IDXGIOutput2>().unwrap())
            }
        }
        impl IOutput3 for $s {
            fn check_overlay_support<T: Interface>(
                &self,
                format: Format,
                concerned_device: &T,
            ) -> Result<OverlaySupportFlag, HResult> {
                let mut flags = 0;
                let res = unsafe {
                    self.0.CheckOverlaySupport(
                        format as u32,
                        concerned_device.as_unknown(),
                        &mut flags,
                    )
                };
                hresult(OverlaySupportFlag(flags), res)
            }
        }
    };
    ($s: ident, $interface: ident, Output4) => {
        impl_output!($s, $interface, Output3);
        impl $s {
            pub fn as_output3(&self) -> Output3 {
                Output3(self.0.query_interface::<IDXGIOutput3>().unwrap())
            }
        }
        impl IOutput4 for $s {
            fn check_overlay_color_space_support<T: Interface>(
                &self,
                format: Format,
                color_space: ColorSpaceType,
                concerned_device: &T,
            ) -> Result<OverlayColorSpaceSupportFlag, HResult> {
                let mut flags = 0;
                let res = unsafe {
                    self.0.CheckOverlayColorSpaceSupport(
                        format as u32,
                        color_space as u32,
                        concerned_device.as_unknown(),
                        &mut flags,
                    )
                };
                hresult(OverlayColorSpaceSupportFlag(flags), res)
            }
        }
    };
    ($s: ident, $interface: ident, Output5) => {
        impl_output!($s, $interface, Output4);
        impl $s {
            pub fn as_output4(&self) -> Output4 {
                Output4(self.0.query_interface::<IDXGIOutput4>().unwrap())
            }
        }
        impl IOutput5 for $s {
            fn duplicate_output1<T: Interface>(
                &self,
                device: &T,
                flags: u32,
                formats: &[Format],
            ) -> Result<OutputDuplication, HResult> {
                Ok(OutputDuplication(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let formats = formats
                        .iter()
                        .map(|&format| format as u32)
                        .collect::<Vec<_>>();
                    let res = unsafe {
                        self.0.DuplicateOutput1(
                            device.as_unknown(),
                            flags,
                            formats.len() as u32,
                            formats.as_ptr(),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
        }
    };
    ($s: ident, $interface: ident, Output6) => {
        impl_output!($s, $interface, Output5);
        impl $s {
            pub fn as_output5(&self) -> Output5 {
                Output5(self.0.query_interface::<IDXGIOutput5>().unwrap())
            }
        }
        impl IOutput6 for $s {
            fn check_hardware_composition_support(
                &self,
            ) -> Result<HardwareCompositionSupportFlags, HResult> {
                let mut flags = 0;
                let res = unsafe { self.0.CheckHardwareCompositionSupport(&mut flags) };
                hresult(HardwareCompositionSupportFlags(flags), res)
            }
            fn get_desc1(&self) -> Result<OutputDesc1, HResult> {
                let mut desc = Default::default();
                let res = unsafe { self.0.GetDesc1(&mut desc) };
                hresult(desc.into(), res)
            }
        }
    };
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Output(ComPtr<IDXGIOutput>);
#[cfg(feature = "dxgi1_2")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Output1(ComPtr<IDXGIOutput1>);
#[cfg(feature = "dxgi1_3")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Output2(ComPtr<IDXGIOutput2>);
#[cfg(feature = "dxgi1_3")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Output3(ComPtr<IDXGIOutput3>);
#[cfg(feature = "dxgi1_4")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Output4(ComPtr<IDXGIOutput4>);
#[cfg(feature = "dxgi1_5")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Output5(ComPtr<IDXGIOutput5>);
#[cfg(feature = "dxgi1_6")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Output6(ComPtr<IDXGIOutput6>);
impl_output!(Output, IDXGIOutput, Output);
#[cfg(feature = "dxgi1_2")]
impl_output!(Output1, IDXGIOutput1, Output1);
#[cfg(feature = "dxgi1_3")]
impl_output!(Output2, IDXGIOutput2, Output2);
#[cfg(feature = "dxgi1_3")]
impl_output!(Output3, IDXGIOutput3, Output3);
#[cfg(feature = "dxgi1_4")]
impl_output!(Output4, IDXGIOutput4, Output4);
#[cfg(feature = "dxgi1_5")]
impl_output!(Output5, IDXGIOutput5, Output5);
#[cfg(feature = "dxgi1_6")]
impl_output!(Output6, IDXGIOutput6, Output6);

#[cfg(feature = "dxgi1_2")]
pub trait IOutputDuplication {
    fn acquire_next_frame(
        &self,
        timeout_in_ms: u32,
    ) -> Result<(OutduplFrameInfo, Resource), HResult>;
    fn get_desc(&self) -> OutduplDesc;
    fn get_frame_dirty_rects(&self, buffer: &mut Vec<Rect>) -> Result<(), HResult>;
    fn get_frame_move_rects(&self, buffer: &mut Vec<OutduplMoveRect>) -> Result<(), HResult>;
    fn get_frame_pointer_shape(
        &self,
        buffer: &mut Vec<u8>,
    ) -> Result<OutduplPointerShapeInfo, HResult>;
    fn map_desktop_surface(&self) -> Result<MappedRect, HResult>;
    fn release_frame(&self) -> Result<(), HResult>;
    fn unmap_desktop_surface(&self) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_2")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct OutputDuplication(ComPtr<IDXGIOutputDuplication>);
#[cfg(feature = "dxgi1_2")]
impl_interface!(OutputDuplication, IDXGIOutputDuplication);
#[cfg(feature = "dxgi1_2")]
impl IOutputDuplication for OutputDuplication {
    fn acquire_next_frame(
        &self,
        timeout_in_ms: u32,
    ) -> Result<(OutduplFrameInfo, Resource), HResult> {
        let mut info = Default::default();
        let mut obj = std::ptr::null_mut();
        unsafe {
            let res = self.0.AcquireNextFrame(timeout_in_ms, &mut info, &mut obj);
            hresult((info.into(), Resource(ComPtr::from_raw(obj))), res)
        }
    }
    fn get_desc(&self) -> OutduplDesc {
        let mut desc = Default::default();
        unsafe { self.0.GetDesc(&mut desc) };
        desc.into()
    }
    fn get_frame_dirty_rects(&self, buffer: &mut Vec<Rect>) -> Result<(), HResult> {
        if buffer.is_empty() {
            buffer.push(Default::default());
        }
        loop {
            let mut required_size = 0;
            let res = unsafe {
                self.0.GetFrameDirtyRects(
                    buffer.len() as u32,
                    buffer.as_mut_ptr() as *mut RECT,
                    &mut required_size,
                )
            };
            if res != DXGI_ERROR_MORE_DATA {
                break;
            }
            let rc_size = required_size as usize / std::mem::size_of::<RECT>();
            buffer.resize(rc_size, Default::default());
        }
        Ok(())
    }
    fn get_frame_move_rects(&self, buffer: &mut Vec<OutduplMoveRect>) -> Result<(), HResult> {
        if buffer.is_empty() {
            buffer.push(Default::default());
        }
        loop {
            let mut required_size = 0;
            let res = unsafe {
                self.0.GetFrameMoveRects(
                    buffer.len() as u32,
                    buffer.as_mut_ptr() as *mut DXGI_OUTDUPL_MOVE_RECT,
                    &mut required_size,
                )
            };
            if res != DXGI_ERROR_MORE_DATA {
                if res == S_OK {
                    break;
                } else {
                    return Err(res.into());
                }
            }
            let rc_size = required_size as usize / std::mem::size_of::<DXGI_OUTDUPL_MOVE_RECT>();
            buffer.resize(rc_size, Default::default());
        }
        Ok(())
    }
    fn get_frame_pointer_shape(
        &self,
        buffer: &mut Vec<u8>,
    ) -> Result<OutduplPointerShapeInfo, HResult> {
        if buffer.is_empty() {
            buffer.push(Default::default());
        }
        let mut info = Default::default();
        loop {
            let mut required_size = 0;
            let res = unsafe {
                self.0.GetFramePointerShape(
                    buffer.len() as u32,
                    buffer.as_mut_ptr() as *mut c_void,
                    &mut required_size,
                    &mut info,
                )
            };
            if res != DXGI_ERROR_MORE_DATA {
                if res == S_OK {
                    break;
                } else {
                    return Err(res.into());
                }
            }
            buffer.resize(required_size as usize, Default::default());
        }
        Ok(info.into())
    }
    fn map_desktop_surface(&self) -> Result<MappedRect, HResult> {
        let mut rc = Default::default();
        let res = unsafe { self.0.MapDesktopSurface(&mut rc) };
        hresult(rc.into(), res)
    }
    fn release_frame(&self) -> Result<(), HResult> {
        hresult((), unsafe { self.0.ReleaseFrame() })
    }
    fn unmap_desktop_surface(&self) -> Result<(), HResult> {
        hresult((), unsafe { self.0.UnMapDesktopSurface() })
    }
}

pub trait IResource: Interface {
    fn get_eviction_priority(&self) -> Result<ResourcePriority, HResult>;
    fn get_shared_handle(&self) -> Result<HANDLE, HResult>;
    fn get_usage(&self) -> Result<Usage, HResult>;
    fn set_eviction_priority(&self, priority: ResourcePriority) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_2")]
pub trait IResource1: IResource {
    fn create_shared_handle(
        &self,
        attr: Option<&SECURITY_ATTRIBUTES>,
        access: SharedResourceRW,
    ) -> Result<HANDLE, HResult>;
    fn create_subresource_surface(&self, index: u32) -> Result<Surface2, HResult>;
}
macro_rules! impl_resource {
    ($s: ident, $interface: ident, Resource) => {
        impl_interface!($s, $interface);
        impl IResource for $s {
            fn get_eviction_priority(&self) -> Result<ResourcePriority, HResult> {
                let mut priority = 0;
                unsafe {
                    let res = self.0.GetEvictionPriority(&mut priority);
                    hresult(std::mem::transmute(priority), res)
                }
            }
            fn get_shared_handle(&self) -> Result<HANDLE, HResult> {
                let mut handle = std::ptr::null_mut();
                let res = unsafe { self.0.GetSharedHandle(&mut handle) };
                hresult(handle, res)
            }
            fn get_usage(&self) -> Result<Usage, HResult> {
                let mut usage = 0;
                let res = unsafe { self.0.GetUsage(&mut usage) };
                hresult(Usage(usage), res)
            }
            fn set_eviction_priority(&self, priority: ResourcePriority) -> Result<(), HResult> {
                let res = unsafe { self.0.SetEvictionPriority(priority as u32) };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, Resource1) => {
        impl_resource!($s, $interface, Resource);
        impl $s {
            pub fn as_resource(&self) -> Resource {
                Resource(self.0.query_interface::<IDXGIResource>().unwrap())
            }
        }
    };
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Resource(ComPtr<IDXGIResource>);
#[cfg(feature = "dxgi1_2")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Resource1(ComPtr<IDXGIResource1>);
impl_resource!(Resource, IDXGIResource, Resource);
#[cfg(feature = "dxgi1_2")]
impl_resource!(Resource1, IDXGIResource1, Resource1);

pub trait ISurface: Interface {
    fn get_desc(&self) -> Result<SurfaceDesc, HResult>;
    fn map(&self, flags: MapFlags) -> Result<MappedRect, HResult>;
    fn unmap(&self) -> Result<(), HResult>;
}
pub trait ISurface1: ISurface {
    fn get_dc(&self, discard: bool) -> Result<HDC, HResult>;
    fn release_dc(&self, dirty_rect: Option<Rect>) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_2")]
pub trait ISurface2: ISurface1 {
    fn get_resource<T: IResource>(&self) -> Result<(T, u32), HResult>;
}
macro_rules! impl_surface {
    ($s: ident, $interface: ident, Surface) => {
        impl_interface!($s, $interface);
        impl ISurface for $s {
            fn get_desc(&self) -> Result<SurfaceDesc, HResult> {
                let mut desc = Default::default();
                let res = unsafe { self.0.GetDesc(&mut desc) };
                hresult(desc.into(), res)
            }
            fn map(&self, flags: MapFlags) -> Result<MappedRect, HResult> {
                let mut rc = Default::default();
                let res = unsafe { self.0.Map(&mut rc, flags.0) };
                hresult(rc.into(), res)
            }
            fn unmap(&self) -> Result<(), HResult> {
                let res = unsafe { self.0.Unmap() };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, Surface1) => {
        impl_surface!($s, $interface, Surface);
        impl $s {
            pub fn as_surface(&self) -> Surface {
                Surface(self.0.query_interface::<IDXGISurface>().unwrap())
            }
        }
        impl ISurface1 for $s {
            fn get_dc(&self, discard: bool) -> Result<HDC, HResult> {
                let mut dc = std::ptr::null_mut();
                let res = unsafe { self.0.GetDC(to_BOOL(discard), &mut dc) };
                hresult(dc, res)
            }
            fn release_dc(&self, dirty_rect: Option<Rect>) -> Result<(), HResult> {
                let mut rc = dirty_rect.map(|rc| rc.into());
                let res = unsafe {
                    self.0
                        .ReleaseDC(rc.as_mut().map_or(std::ptr::null_mut(), |r| r as *mut RECT))
                };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, Surface2) => {
        impl_surface!($s, $interface, Surface1);
        impl ISurface2 for $s {
            fn get_resource<T: IResource>(&self) -> Result<(T, u32), HResult> {
                let mut obj = std::ptr::null_mut();
                let mut index = 0;
                unsafe {
                    let res = self
                        .0
                        .GetResource(&T::uuidof().into(), &mut obj, &mut index);
                    hresult(
                        (
                            T::from_com_ptr(ComPtr::from_raw(obj as *mut T::APIType)),
                            index,
                        ),
                        res,
                    )
                }
            }
        }
    };
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Surface(ComPtr<IDXGISurface>);
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Surface1(ComPtr<IDXGISurface1>);
#[cfg(feature = "dxgi1_2")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Surface2(ComPtr<IDXGISurface2>);
impl_surface!(Surface, IDXGISurface, Surface);
impl_surface!(Surface1, IDXGISurface1, Surface1);
#[cfg(feature = "dxgi1_2")]
impl_surface!(Surface2, IDXGISurface2, Surface2);

#[cfg(feature = "dxgi1_3")]
#[derive(Clone, Copy, Default, Debug)]
pub struct SourceSize {
    pub width: u32,
    pub height: u32,
}

pub trait ISwapChain: Interface {
    fn get_buffer<T: Interface>(&self, index: u32) -> Result<T, HResult>;
    fn get_containing_output(&self) -> Result<Output, HResult>;
    fn get_desc(&self) -> Result<SwapChainDesc, HResult>;
    fn get_frame_statistics(&self) -> Result<FrameStatistics, HResult>;
    fn get_fullscreen_state(&self) -> Result<(bool, Option<Output>), HResult>;
    fn get_last_present_count(&self) -> Result<u32, HResult>;
    fn present(&self, interval: u32, flags: Present) -> Result<(), HResult>;
    fn resize_buffers(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        format: Format,
        flags: Option<SwapChainFlag>,
    ) -> Result<(), HResult>;
    fn set_fullscreen_state(
        &self,
        fullscreen: bool,
        target: Option<&Output>,
    ) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_2")]
pub trait ISwapChain1: ISwapChain {
    fn get_background_color(&self) -> Result<RGBA, HResult>;
    fn get_core_window<T: Interface>(&self) -> Result<T, HResult>;
    fn get_desc1(&self) -> Result<SwapChainDesc1, HResult>;
    fn get_fullscreen_desc(&self) -> Result<SwapChainFullscreenDesc, HResult>;
    fn get_hwnd(&self) -> Result<HWND, HResult>;
    fn get_restrict_to_output(&self) -> Result<Output, HResult>;
    fn get_rotation(&self) -> Result<ModeRotation, HResult>;
    fn is_temporary_mono_supported(&self) -> bool;
    fn present1(
        &self,
        interval: u32,
        flags: Present,
        parameters: &PresentParameters,
    ) -> Result<(), HResult>;
    fn set_background_color(&self, rgba: RGBA) -> Result<(), HResult>;
    fn set_rotation(&self, rotation: ModeRotation) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_3")]
pub trait ISwapChain2: ISwapChain1 {
    fn get_frame_latency_waitable_object(&self) -> HANDLE;
    fn get_matrix_transform(&self) -> Result<Matrix3x2f, HResult>;
    fn get_maximum_frame_latency(&self) -> Result<u32, HResult>;
    fn get_source_size(&self) -> Result<SourceSize, HResult>;
    fn set_matrix_transform(&self, m: Matrix3x2f) -> Result<(), HResult>;
    fn set_maximum_frame_latency(&self, latency: u32) -> Result<(), HResult>;
    fn set_source_size(&self, source_size: SourceSize) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_4")]
pub trait ISwapChain3: ISwapChain2 {
    fn check_color_space_support(
        &self,
        color_space: ColorSpaceType,
    ) -> Result<SwapChainColorSpaceSupportFlag, HResult>;
    fn get_current_backbuffer_index(&self) -> u32;
    fn resize_buffers1<T: Interface>(
        &self,
        buffer_count: u32,
        width: u32,
        height: u32,
        format: Format,
        flags: Option<SwapChainFlag>,
        node_mask: &[u32],
        present_queue: &[T],
    ) -> Result<(), HResult>;
    fn set_color_space1(&self, color_space: ColorSpaceType) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_5")]
pub trait ISwapChain4: ISwapChain3 {
    fn set_hdr_metadata(&self, ty: HDRMetadataType, data: &mut [u8]) -> Result<(), HResult>;
}
macro_rules! impl_swapchain {
    ($s: ident, $interface: ident, SwapChain) => {
        impl_interface!($s, $interface);
        impl ISwapChain for $s {
            fn get_buffer<T: Interface>(&self, index: u32) -> Result<T, HResult> {
                Ok(T::from_com_ptr(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.GetBuffer(index, &T::uuidof().into(), &mut obj) };
                    hresult(obj as *mut T::APIType, res)
                })?))
            }
            fn get_containing_output(&self) -> Result<Output, HResult> {
                Ok(Output(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.GetContainingOutput(&mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn get_desc(&self) -> Result<SwapChainDesc, HResult> {
                let mut desc = Default::default();
                let res = unsafe { self.0.GetDesc(&mut desc) };
                hresult(desc.into(), res)
            }
            fn get_frame_statistics(&self) -> Result<FrameStatistics, HResult> {
                let mut stats = Default::default();
                let res = unsafe { self.0.GetFrameStatistics(&mut stats) };
                hresult(stats.into(), res)
            }
            fn get_fullscreen_state(&self) -> Result<(bool, Option<Output>), HResult> {
                let mut fullscreen = 0;
                let mut p = std::ptr::null_mut();
                unsafe {
                    let res = self.0.GetFullscreenState(&mut fullscreen, &mut p);
                    let obj = if p == std::ptr::null_mut() {
                        None
                    } else {
                        Some(Output(ComPtr::from_raw(p)))
                    };
                    hresult((fullscreen == TRUE, obj), res)
                }
            }
            fn get_last_present_count(&self) -> Result<u32, HResult> {
                let mut count = 0;
                let res = unsafe { self.0.GetLastPresentCount(&mut count) };
                hresult(count, res)
            }
            fn present(&self, interval: u32, flags: Present) -> Result<(), HResult> {
                let res = unsafe { self.0.Present(interval, flags.0) };
                hresult((), res)
            }
            fn resize_buffers(
                &self,
                buffer_count: u32,
                width: u32,
                height: u32,
                format: Format,
                flags: Option<SwapChainFlag>,
            ) -> Result<(), HResult> {
                let res = unsafe {
                    self.0.ResizeBuffers(
                        buffer_count,
                        width,
                        height,
                        format as u32,
                        flags.map_or(0, |f| f.0),
                    )
                };
                hresult((), res)
            }
            fn set_fullscreen_state(
                &self,
                fullscreen: bool,
                target: Option<&Output>,
            ) -> Result<(), HResult> {
                let output = if fullscreen {
                    target.map_or(std::ptr::null_mut(), |p| p.0.as_ptr())
                } else {
                    std::ptr::null_mut()
                };
                let res = unsafe { self.0.SetFullscreenState(to_BOOL(fullscreen), output) };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, SwapChain1) => {
        impl_swapchain!($s, $interface, SwapChain);
        impl $s {
            pub fn as_swapchain(&self) -> SwapChain {
                SwapChain(self.0.query_interface::<IDXGISwapChain>().unwrap())
            }
        }
        impl ISwapChain1 for $s {
            fn get_background_color(&self) -> Result<RGBA, HResult> {
                let mut rgba = Default::default();
                let res = unsafe { self.0.GetBackgroundColor(&mut rgba) };
                hresult(rgba.into(), res)
            }
            fn get_core_window<T: Interface>(&self) -> Result<T, HResult> {
                Ok(T::from_com_ptr(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.GetCoreWindow(&T::uuidof().into(), &mut obj) };
                    hresult(obj as *mut T::APIType, res)
                })?))
            }
            fn get_desc1(&self) -> Result<SwapChainDesc1, HResult> {
                let mut desc = Default::default();
                let res = unsafe { self.0.GetDesc1(&mut desc) };
                hresult(desc.into(), res)
            }
            fn get_fullscreen_desc(&self) -> Result<SwapChainFullscreenDesc, HResult> {
                let mut desc = Default::default();
                let res = unsafe { self.0.GetFullscreenDesc(&mut desc) };
                hresult(desc.into(), res)
            }
            fn get_hwnd(&self) -> Result<HWND, HResult> {
                let mut hwnd = std::ptr::null_mut();
                let res = unsafe { self.0.GetHwnd(&mut hwnd) };
                hresult(hwnd, res)
            }
            fn get_restrict_to_output(&self) -> Result<Output, HResult> {
                Ok(Output(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.GetRestrictToOutput(&mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn get_rotation(&self) -> Result<ModeRotation, HResult> {
                let mut rotation = Default::default();
                unsafe {
                    let res = self.0.GetRotation(&mut rotation);
                    hresult(std::mem::transmute(rotation), res)
                }
            }
            fn is_temporary_mono_supported(&self) -> bool {
                unsafe { self.0.IsTemporaryMonoSupported() == TRUE }
            }
            fn present1(
                &self,
                interval: u32,
                flags: Present,
                parameters: &PresentParameters,
            ) -> Result<(), HResult> {
                let parameters = parameters.clone();
                let dirty_rects = parameters
                    .dirty_rects
                    .map(|rcs| rcs.iter().collect::<Vec<_>>());
                let mut scroll_rect = parameters.scroll_rect.map(|rc| rc.into());
                let mut scroll_offset = parameters.scroll_offset.map(|pt| pt.into());
                let params = DXGI_PRESENT_PARAMETERS {
                    DirtyRectsCount: parameters
                        .dirty_rects
                        .as_ref()
                        .map_or(0, |rc| rc.len() as u32),
                    pDirtyRects: dirty_rects
                        .map_or(std::ptr::null_mut(), |mut rc| rc.as_mut_ptr() as *mut RECT),
                    pScrollRect: scroll_rect
                        .as_mut()
                        .map_or(std::ptr::null_mut(), |rc| rc as *mut RECT),
                    pScrollOffset: scroll_offset
                        .as_mut()
                        .map_or(std::ptr::null_mut(), |pt| pt as *mut POINT),
                };
                let res = unsafe { self.0.Present1(interval, flags.0, &params) };
                hresult((), res)
            }
            fn set_background_color(&self, rgba: RGBA) -> Result<(), HResult> {
                let res = unsafe { self.0.SetBackgroundColor(&rgba.into()) };
                hresult((), res)
            }
            fn set_rotation(&self, rotation: ModeRotation) -> Result<(), HResult> {
                let res = unsafe { self.0.SetRotation(rotation as u32) };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, SwapChain2) => {
        impl_swapchain!($s, $interface, SwapChain1);
        impl $s {
            pub fn as_swapchain1(&self) -> SwapChain1 {
                SwapChain1(self.0.query_interface::<IDXGISwapChain1>().unwrap())
            }
        }
        impl ISwapChain2 for $s {
            fn get_frame_latency_waitable_object(&self) -> HANDLE {
                unsafe { self.0.GetFrameLatencyWaitableObject() }
            }
            fn get_matrix_transform(&self) -> Result<Matrix3x2f, HResult> {
                let mut m = Default::default();
                let res = unsafe { self.0.GetMatrixTransform(&mut m) };
                hresult(m, res)
            }
            fn get_maximum_frame_latency(&self) -> Result<u32, HResult> {
                let mut latency = 0;
                let res = unsafe { self.0.GetMaximumFrameLatency(&mut latency) };
                hresult(latency, res)
            }
            fn get_source_size(&self) -> Result<SourceSize, HResult> {
                let mut sz = SourceSize::default();
                let res = unsafe { self.0.GetSourceSize(&mut sz.width, &mut sz.height) };
                hresult(sz, res)
            }
            fn set_matrix_transform(&self, m: Matrix3x2f) -> Result<(), HResult> {
                let res = unsafe { self.0.SetMatrixTransform(&m.into()) };
                hresult((), res)
            }
            fn set_maximum_frame_latency(&self, latency: u32) -> Result<(), HResult> {
                let res = unsafe { self.0.SetMaximumFrameLatency(latency) };
                hresult((), res)
            }
            fn set_source_size(&self, source_size: SourceSize) -> Result<(), HResult> {
                let res = unsafe { self.0.SetSourceSize(source_size.width, source_size.height) };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, SwapChain3) => {
        impl_swapchain!($s, $interface, SwapChain2);
        impl $s {
            pub fn as_swapchain2(&self) -> SwapChain2 {
                SwapChain2(self.0.query_interface::<IDXGISwapChain2>().unwrap())
            }
        }
        impl ISwapChain3 for $s {
            fn check_color_space_support(
                &self,
                color_space: ColorSpaceType,
            ) -> Result<SwapChainColorSpaceSupportFlag, HResult> {
                let mut support = 0;
                let res = unsafe {
                    self.0
                        .CheckColorSpaceSupport(color_space as u32, &mut support)
                };
                hresult(SwapChainColorSpaceSupportFlag(support), res)
            }
            fn get_current_backbuffer_index(&self) -> u32 {
                unsafe { self.0.GetCurrentBackBufferIndex() }
            }
            fn resize_buffers1<T: Interface>(
                &self,
                buffer_count: u32,
                width: u32,
                height: u32,
                format: Format,
                flags: Option<SwapChainFlag>,
                node_mask: &[u32],
                present_queue: &[T],
            ) -> Result<(), HResult> {
                let mut queue = present_queue
                    .iter()
                    .map(|q| q.as_unknown())
                    .collect::<Vec<_>>();
                let res = unsafe {
                    self.0.ResizeBuffers1(
                        buffer_count,
                        width,
                        height,
                        format as u32,
                        flags.map_or(0, |f| f.0),
                        node_mask.as_ptr(),
                        queue.as_mut_ptr(),
                    )
                };
                hresult((), res)
            }
            fn set_color_space1(&self, color_space: ColorSpaceType) -> Result<(), HResult> {
                let res = unsafe { self.0.SetColorSpace1(color_space as u32) };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, SwapChain4) => {
        impl_swapchain!($s, $interface, SwapChain3);
        impl $s {
            pub fn as_swapchain3(&self) -> SwapChain3 {
                SwapChain3(self.0.query_interface::<IDXGISwapChain3>().unwrap())
            }
        }
        impl ISwapChain4 for $s {
            fn set_hdr_metadata(
                &self,
                ty: HDRMetadataType,
                data: &mut [u8],
            ) -> Result<(), HResult> {
                let res = unsafe {
                    self.0.SetHDRMetaData(
                        ty as u32,
                        data.len() as u32,
                        data.as_mut_ptr() as *mut c_void,
                    )
                };
                hresult((), res)
            }
        }
    };
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwapChain(ComPtr<IDXGISwapChain>);
#[cfg(feature = "dxgi1_2")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwapChain1(ComPtr<IDXGISwapChain1>);
#[cfg(feature = "dxgi1_3")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwapChain2(ComPtr<IDXGISwapChain2>);
#[cfg(feature = "dxgi1_4")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwapChain3(ComPtr<IDXGISwapChain3>);
#[cfg(feature = "dxgi1_5")]
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct SwapChain4(ComPtr<IDXGISwapChain4>);
impl_swapchain!(SwapChain, IDXGISwapChain, SwapChain);
#[cfg(feature = "dxgi1_2")]
impl_swapchain!(SwapChain1, IDXGISwapChain1, SwapChain1);
#[cfg(feature = "dxgi1_3")]
impl_swapchain!(SwapChain2, IDXGISwapChain2, SwapChain2);
#[cfg(feature = "dxgi1_4")]
impl_swapchain!(SwapChain3, IDXGISwapChain3, SwapChain3);
#[cfg(feature = "dxgi1_5")]
impl_swapchain!(SwapChain4, IDXGISwapChain4, SwapChain4);

#[cfg(feature = "dxgi1_3")]
#[derive(Clone, Default, Debug)]
pub struct PresentDuration {
    small: u32,
    large: u32,
}
#[cfg(feature = "dxgi1_3")]
pub trait ISwapChainMedia {
    fn check_present_duration_support(&self, desired: u32) -> Result<PresentDuration, HResult>;
    fn get_frame_statistics_media(&self) -> Result<FrameStatisticsMedia, HResult>;
    fn set_present_duration(&self, duration: u32) -> Result<(), HResult>;
}
#[cfg(feature = "dxgi1_3")]
pub struct SwapChainMedia(ComPtr<IDXGISwapChainMedia>);
#[cfg(feature = "dxgi1_3")]
impl ISwapChainMedia for SwapChainMedia {
    fn check_present_duration_support(&self, desired: u32) -> Result<PresentDuration, HResult> {
        let mut duration = PresentDuration::default();
        let res = unsafe { self.0.CheckPresentDurationSupport(desired, &mut duration.small, &mut duration.large) };
        hresult(duration, res)
    }
    fn get_frame_statistics_media(&self) -> Result<FrameStatisticsMedia, HResult> {
        let mut stats = Default::default();
        let res = unsafe { self.0.GetFrameStatisticsMedia(&mut stats) };
        hresult(stats.into(), res)
    }
    fn set_present_duration(&self, duration: u32) -> Result<(), HResult> {
        let res = unsafe { self.0.SetPresentDuration(duration) };
        hresult((), res)
    }
}

pub fn create_dxgi_factory<T: IFactory>() -> Result<T, HResult> {
    Ok(T::from_com_ptr(ComPtr::new(|| {
        let mut obj = std::ptr::null_mut();
        let res = unsafe { CreateDXGIFactory(&T::uuidof().into(), &mut obj) };
        hresult(obj as *mut T::APIType, res)
    })?))
}
pub fn create_dxgi_factory1<T: IFactory>() -> Result<T, HResult> {
    Ok(T::from_com_ptr(ComPtr::new(|| {
        let mut obj = std::ptr::null_mut();
        let res = unsafe { CreateDXGIFactory1(&T::uuidof().into(), &mut obj) };
        hresult(obj as *mut T::APIType, res)
    })?))
}
#[cfg(feature = "dxgi1_3")]
pub fn create_dxgi_factory2<T: IFactory>(flags: Option<CreateFactoryFlag>) -> Result<T, HResult> {
    Ok(T::from_com_ptr(ComPtr::new(|| {
        let mut obj = std::ptr::null_mut();
        let res =
            unsafe { CreateDXGIFactory2(flags.map_or(0, |f| f.0), &T::uuidof().into(), &mut obj) };
        hresult(obj as *mut T::APIType, res)
    })?))
}

extern "system" {
    #[cfg(feature = "dxgi1_6")]
    fn DXGIDeclareAdapterRemovalSupport() -> HRESULT;
}
#[cfg(feature = "dxgi1_6")]
pub fn declare_adapter_removal_support() -> Result<(), HResult> {
    unsafe { hresult((), DXGIDeclareAdapterRemovalSupport().into()) }
}

pub fn get_debug_interface<T: Interface>() -> Result<T, HResult> {
    Ok(T::from_com_ptr(ComPtr::new(|| {
        let mut obj = std::ptr::null_mut();
        let res = unsafe { DXGIGetDebugInterface(&T::uuidof().into(), &mut obj) };
        hresult(obj as *mut T::APIType, res)
    })?))
}
#[cfg(feature = "dxgi1_3")]
pub fn get_debug_interface1<T: Interface>(flags: UINT) -> Result<T, HResult> {
    Ok(T::from_com_ptr(ComPtr::new(|| {
        let mut obj = std::ptr::null_mut();
        let res = unsafe { DXGIGetDebugInterface1(flags, &T::uuidof().into(), &mut obj) };
        hresult(obj as *mut T::APIType, res)
    })?))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_factory1_test() {
        let res = create_dxgi_factory1::<Factory1>();
        assert!(res.is_ok());
    }
    #[test]
    #[cfg(feature = "dxgi1_6")]
    fn declare_adapter_removal_support_test() {
        let _ = declare_adapter_removal_support();
    }
}
