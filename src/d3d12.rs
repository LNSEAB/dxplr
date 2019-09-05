use crate::api::EventHandle;
use crate::api::Rect;
use crate::api::*;
use crate::d3d;
use crate::d3d::IBlob;
pub use crate::d3d12sdklayers::*;
use crate::dxgi;
use crate::result::{hresult, HResult};
use crate::utility::*;
use crate::Interface;
use crate::{impl_bitflag_operators, impl_interface};
use com_ptr::ComPtr;
use std::any::Any;
use winapi::ctypes::c_void;
use winapi::shared::windef::RECT;
use winapi::um::d3d12::*;
use winapi::um::minwinbase::SECURITY_ATTRIBUTES;
use winapi::um::winnt::HANDLE;
use winapi::Interface as _;

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AutoBreadcrumbOp {
    SetMarker = D3D12_AUTO_BREADCRUMB_OP_SETMARKER,
    BeginEvent = D3D12_AUTO_BREADCRUMB_OP_BEGINEVENT,
    EndEvent = D3D12_AUTO_BREADCRUMB_OP_ENDEVENT,
    DrawInstanced = D3D12_AUTO_BREADCRUMB_OP_DRAWINSTANCED,
    DrawIndexedInstanced = D3D12_AUTO_BREADCRUMB_OP_DRAWINDEXEDINSTANCED,
    ExecuteIndirect = D3D12_AUTO_BREADCRUMB_OP_EXECUTEINDIRECT,
    Dispatch = D3D12_AUTO_BREADCRUMB_OP_DISPATCH,
    CopyBufferRegion = D3D12_AUTO_BREADCRUMB_OP_COPYBUFFERREGION,
    CopyTextureRegion = D3D12_AUTO_BREADCRUMB_OP_COPYTEXTUREREGION,
    CopyResource = D3D12_AUTO_BREADCRUMB_OP_COPYRESOURCE,
    CopyTiles = D3D12_AUTO_BREADCRUMB_OP_COPYTILES,
    ResolveSubresource = D3D12_AUTO_BREADCRUMB_OP_RESOLVESUBRESOURCE,
    ClearRenderTargetView = D3D12_AUTO_BREADCRUMB_OP_CLEARRENDERTARGETVIEW,
    ClearUnorderedAccessView = D3D12_AUTO_BREADCRUMB_OP_CLEARUNORDEREDACCESSVIEW,
    ClearDepthStencilView =  D3D12_AUTO_BREADCRUMB_OP_CLEARDEPTHSTENCILVIEW,
    ResourceBarrier = D3D12_AUTO_BREADCRUMB_OP_RESOURCEBARRIER,
    ExecuteBundle = D3D12_AUTO_BREADCRUMB_OP_EXECUTEBUNDLE,
    Present = D3D12_AUTO_BREADCRUMB_OP_PRESENT,
    ResolveQueryData = D3D12_AUTO_BREADCRUMB_OP_RESOLVEQUERYDATA,
    BeginSubmission = D3D12_AUTO_BREADCRUMB_OP_BEGINSUBMISSION,
    EndSubmission = D3D12_AUTO_BREADCRUMB_OP_ENDSUBMISSION,
    DecodeFrame = D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME,
    ProcessFrams = D3D12_AUTO_BREADCRUMB_OP_PROCESSFRAMES,
    AtomicCopyBufferUint = D3D12_AUTO_BREADCRUMB_OP_ATOMICCOPYBUFFERUINT,
    AtomicCopyBufferUint64 = D3D12_AUTO_BREADCRUMB_OP_ATOMICCOPYBUFFERUINT64,
    ResolveSubresourceRegion = D3D12_AUTO_BREADCRUMB_OP_RESOLVESUBRESOURCEREGION,
    WriteBufferImmediate = D3D12_AUTO_BREADCRUMB_OP_WRITEBUFFERIMMEDIATE,
    DecodeFrame1 = D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME1,
    SetProtectedResourceSession = D3D12_AUTO_BREADCRUMB_OP_SETPROTECTEDRESOURCESESSION,
    DecodeFrame2 = D3D12_AUTO_BREADCRUMB_OP_DECODEFRAME2,
    ProcessFrames1 = D3D12_AUTO_BREADCRUMB_OP_PROCESSFRAMES1,
    BuildRaytracingAccelerationStructure = D3D12_AUTO_BREADCRUMB_OP_BUILDRAYTRACINGACCELERATIONSTRUCTURE,
    EmitRaytracingAccelerationStructurePostBuildInfo = D3D12_AUTO_BREADCRUMB_OP_EMITRAYTRACINGACCELERATIONSTRUCTUREPOSTBUILDINFO,
    CopyRaytracingAccelerationStructure = D3D12_AUTO_BREADCRUMB_OP_COPYRAYTRACINGACCELERATIONSTRUCTURE,
    DispatchRays = D3D12_AUTO_BREADCRUMB_OP_DISPATCHRAYS,
    InitializeMetaCommand = D3D12_AUTO_BREADCRUMB_OP_INITIALIZEMETACOMMAND,
    ExecuteMetaCommand = D3D12_AUTO_BREADCRUMB_OP_EXECUTEMETACOMMAND,
    EstimateMotion = D3D12_AUTO_BREADCRUMB_OP_ESTIMATEMOTION,
    ResolveMotionVectorHeap = D3D12_AUTO_BREADCRUMB_OP_RESOLVEMOTIONVECTORHEAP,
    SetPipelineState1 = D3D12_AUTO_BREADCRUMB_OP_SETPIPELINESTATE1,
    InitializeExtensionCommand = D3D12_AUTO_BREADCRUMB_OP_INITIALIZEEXTENSIONCOMMAND,
    ExecuteExtensionCommand = D3D12_AUTO_BREADCRUMB_OP_EXECUTEEXTENSIONCOMMAND
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum AxisShadingRate {
    _1x = D3D12_AXIS_SHADING_RATE_1X,
    _2x = D3D12_AXIS_SHADING_RATE_2X,
    _4x = D3D12_AXIS_SHADING_RATE_4X,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Blend {
    Zero = D3D12_BLEND_ZERO,
    One = D3D12_BLEND_ONE,
    SrcColor = D3D12_BLEND_SRC_COLOR,
    InvSrcColor = D3D12_BLEND_INV_SRC_COLOR,
    SrcAlpha = D3D12_BLEND_SRC_ALPHA,
    InvSrcAlpha = D3D12_BLEND_INV_SRC_ALPHA,
    DestAlpha = D3D12_BLEND_DEST_ALPHA,
    InvDestAlpha = D3D12_BLEND_INV_DEST_ALPHA,
    DestColor = D3D12_BLEND_DEST_COLOR,
    InvDestColor = D3D12_BLEND_INV_DEST_COLOR,
    SrcAlphaSat = D3D12_BLEND_SRC_ALPHA_SAT,
    BlendFactor = D3D12_BLEND_BLEND_FACTOR,
    InvBlendFactor = D3D12_BLEND_INV_BLEND_FACTOR,
    Src1Color = D3D12_BLEND_SRC1_COLOR,
    InvSrc1Color = D3D12_BLEND_INV_SRC1_COLOR,
    Src1Alpha = D3D12_BLEND_SRC1_ALPHA,
    InvSrc1Alpha = D3D12_BLEND_INV_SRC1_ALPHA,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BlendOp {
    Add = D3D12_BLEND_OP_ADD,
    Subtract = D3D12_BLEND_OP_SUBTRACT,
    RevSubtract = D3D12_BLEND_OP_REV_SUBTRACT,
    Min = D3D12_BLEND_OP_MIN,
    Max = D3D12_BLEND_OP_MAX,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BufferSRVFlags(u32);
#[allow(non_upper_case_globals)]
impl BufferSRVFlags {
    pub const None: Self = Self(D3D12_BUFFER_SRV_FLAG_NONE);
    pub const Raw: Self = Self(D3D12_BUFFER_SRV_FLAG_RAW);
}
impl_bitflag_operators!(BufferSRVFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BufferUAVFlags(u32);
#[allow(non_upper_case_globals)]
impl BufferUAVFlags {
    pub const None: Self = Self(D3D12_BUFFER_UAV_FLAG_NONE);
    pub const Raw: Self = Self(D3D12_BUFFER_UAV_FLAG_RAW);
}
impl_bitflag_operators!(BufferUAVFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ClearFlags(u32);
#[allow(non_upper_case_globals)]
impl ClearFlags {
    pub const Depth: Self = Self(D3D12_CLEAR_FLAG_DEPTH);
    pub const Stencil: Self = Self(D3D12_CLEAR_FLAG_STENCIL);
}
impl_bitflag_operators!(ClearFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ColorWriteEnable(u32);
#[allow(non_upper_case_globals)]
impl ColorWriteEnable {
    pub const Red: Self = Self(D3D12_COLOR_WRITE_ENABLE_RED);
    pub const Green: Self = Self(D3D12_COLOR_WRITE_ENABLE_GREEN);
    pub const Blue: Self = Self(D3D12_COLOR_WRITE_ENABLE_BLUE);
    pub const Alpha: Self = Self(D3D12_COLOR_WRITE_ENABLE_ALPHA);
    pub const All: Self = Self(D3D12_COLOR_WRITE_ENABLE_ALL);
}
impl_bitflag_operators!(ColorWriteEnable);

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CommandListSupportFlags(u32);
#[allow(non_upper_case_globals)]
impl CommandListSupportFlags {
    pub const None: Self = Self(D3D12_COMMAND_LIST_SUPPORT_FLAG_NONE);
    pub const Direct: Self = Self(D3D12_COMMAND_LIST_SUPPORT_FLAG_DIRECT);
    pub const Bundle: Self = Self(D3D12_COMMAND_LIST_SUPPORT_FLAG_BUNDLE);
    pub const Compute: Self = Self(D3D12_COMMAND_LIST_SUPPORT_FLAG_COMPUTE);
    pub const Copy: Self = Self(D3D12_COMMAND_LIST_SUPPORT_FLAG_COPY);
    pub const VideoDecode: Self = Self(D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_DECODE);
    pub const VideoProcess: Self = Self(D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_PROCESS);
    pub const VideoEncode: Self = Self(D3D12_COMMAND_LIST_SUPPORT_FLAG_VIDEO_ENCODE);
}
impl_bitflag_operators!(CommandListSupportFlags);
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CommandListType {
    Direct = D3D12_COMMAND_LIST_TYPE_DIRECT,
    Bundle = D3D12_COMMAND_LIST_TYPE_BUNDLE,
    Compute = D3D12_COMMAND_LIST_TYPE_COMPUTE,
    Copy = D3D12_COMMAND_LIST_TYPE_COPY,
    // VideoDecode = D3D12_COMMAND_LIST_TYPE_VIDEO_DECODE,
    // VideoProcess = D3D12_COMMAND_LIST_TYPE_VIDEO_PROCESS,
    // VideoEncode = D3D12_COMMAND_LIST_TYPE_VIDEO_ENCODE
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CommandQueueFlags(u32);
#[allow(non_upper_case_globals)]
impl CommandQueueFlags {
    pub const None: Self = Self(D3D12_COMMAND_QUEUE_FLAG_NONE);
    pub const DisableGPUTimeout: Self = Self(D3D12_COMMAND_QUEUE_FLAG_DISABLE_GPU_TIMEOUT);
}
impl_bitflag_operators!(CommandQueueFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CommandQueuePriority {
    Normal = D3D12_COMMAND_QUEUE_PRIORITY_NORMAL,
    High = D3D12_COMMAND_QUEUE_PRIORITY_HIGH,
    GlobalRealtime = D3D12_COMMAND_QUEUE_PRIORITY_GLOBAL_REALTIME,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ComparisonFunc {
    Never = D3D12_COMPARISON_FUNC_NEVER,
    Less = D3D12_COMPARISON_FUNC_LESS,
    Equal = D3D12_COMPARISON_FUNC_EQUAL,
    LessEqual = D3D12_COMPARISON_FUNC_LESS_EQUAL,
    Greater = D3D12_COMPARISON_FUNC_GREATER,
    NotEqual = D3D12_COMPARISON_FUNC_NOT_EQUAL,
    GreaterEqual = D3D12_COMPARISON_FUNC_GREATER_EQUAL,
    Always = D3D12_COMPARISON_FUNC_ALWAYS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ConservativeRasterizationMode {
    Off = D3D12_CONSERVATIVE_RASTERIZATION_MODE_OFF,
    On = D3D12_CONSERVATIVE_RASTERIZATION_MODE_ON,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ConservativeRasterizationTier {
    NotSupported = D3D12_CONSERVATIVE_RASTERIZATION_TIER_NOT_SUPPORTED,
    Tier1 = D3D12_CONSERVATIVE_RASTERIZATION_TIER_1,
    Tier2 = D3D12_CONSERVATIVE_RASTERIZATION_TIER_2,
    Tier3 = D3D12_CONSERVATIVE_RASTERIZATION_TIER_3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CPUPageProperty {
    Unknown = D3D12_CPU_PAGE_PROPERTY_UNKNOWN,
    NotAvailable = D3D12_CPU_PAGE_PROPERTY_NOT_AVAILABLE,
    WriteCombine = D3D12_CPU_PAGE_PROPERTY_WRITE_COMBINE,
    WriteBack = D3D12_CPU_PAGE_PROPERTY_WRITE_BACK,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CrossNodeSharingTier {
    NotSupported = D3D12_CROSS_NODE_SHARING_TIER_NOT_SUPPORTED,
    Tier1Emulated = D3D12_CROSS_NODE_SHARING_TIER_1_EMULATED,
    Tier1 = D3D12_CROSS_NODE_SHARING_TIER_1,
    Tier2 = D3D12_CROSS_NODE_SHARING_TIER_2,
    // Tier3 = D3D12_CROSS_NODE_SHARING_TIER_3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CullMode {
    None = D3D12_CULL_MODE_NONE,
    Front = D3D12_CULL_MODE_FRONT,
    Back = D3D12_CULL_MODE_BACK,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DepthWriteMask {
    Zero = D3D12_DEPTH_WRITE_MASK_ZERO,
    All = D3D12_DEPTH_WRITE_MASK_ALL,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DescriptorHeapFlags(u32);
#[allow(non_upper_case_globals)]
impl DescriptorHeapFlags {
    pub const None: Self = Self(D3D12_DESCRIPTOR_HEAP_FLAG_NONE);
    pub const ShaderVisible: Self = Self(D3D12_DESCRIPTOR_HEAP_FLAG_SHADER_VISIBLE);
}
impl_bitflag_operators!(DescriptorHeapFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum DescriptorHeapType {
    CBVSRVUAV = D3D12_DESCRIPTOR_HEAP_TYPE_CBV_SRV_UAV,
    Sampler = D3D12_DESCRIPTOR_HEAP_TYPE_SAMPLER,
    RTV = D3D12_DESCRIPTOR_HEAP_TYPE_RTV,
    DSV = D3D12_DESCRIPTOR_HEAP_TYPE_DSV,
    NumTypes = D3D12_DESCRIPTOR_HEAP_TYPE_NUM_TYPES,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DescriptorRangeFlags(u32);
#[allow(non_upper_case_globals)]
impl DescriptorRangeFlags {
    pub const None: Self = Self(D3D12_DESCRIPTOR_RANGE_FLAG_NONE);
    pub const DescriptorsVolatile: Self = Self(D3D12_DESCRIPTOR_RANGE_FLAG_DESCRIPTORS_VOLATILE);
    pub const DataVolatile: Self = Self(D3D12_DESCRIPTOR_RANGE_FLAG_DATA_VOLATILE);
    pub const DataStaticWhileSetAtExecute: Self =
        Self(D3D12_DESCRIPTOR_RANGE_FLAG_DATA_STATIC_WHILE_SET_AT_EXECUTE);
    pub const DataStatic: Self = Self(D3D12_DESCRIPTOR_RANGE_FLAG_DATA_STATIC);
    // pub const DescriptorsStaticKeepingBufferBoundsChecks: Self = Self(D3D12_DESCRIPTORS_RANGE_FLAG_DESCRIPTORS_STATIC_KEEPING_BUFFER_BOUNDS_CHECKS);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DescriptorRangeType {
    SRV = D3D12_DESCRIPTOR_RANGE_TYPE_SRV,
    UAV = D3D12_DESCRIPTOR_RANGE_TYPE_UAV,
    CBV = D3D12_DESCRIPTOR_RANGE_TYPE_CBV,
    Sampler = D3D12_DESCRIPTOR_RANGE_TYPE_SAMPLER,
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DriverMatchingIdentifierStatus(u32);
#[allow(non_upper_case_globals)]
impl DriverMatchingIdentifierStatus {
    pub const CompatibleWithDevice: Self = Self(D3D12_DRIVER_MATCHING_IDENTIFIER_COMPATIBLE_WITH_DEVICE);
    pub const UnsupportedType: Self = Self(D3D12_DRIVER_MATCHING_IDENTIFIER_UNSUPPORTED_TYPE);
    pub const Unrecognized: Self = Self(D3D12_DRIVER_MATCHING_IDENTIFIER_UNRECOGNIZED);
    pub const IncompatibleVersion: Self = Self(D3D12_DRIVER_MATCHING_IDENTIFIER_INCOMPATIBLE_VERSION);
    pub const IncompatibleType: Self = Self(D3D12_DRIVER_MATCHING_IDENTIFIER_INCOMPATIBLE_TYPE);
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DSVDimension {
    Unknown = D3D12_DSV_DIMENSION_UNKNOWN,
    Texture1D = D3D12_DSV_DIMENSION_TEXTURE1D,
    Texture1DArray = D3D12_DSV_DIMENSION_TEXTURE1DARRAY,
    Texture2D = D3D12_DSV_DIMENSION_TEXTURE2D,
    Texture2DArray = D3D12_DSV_DIMENSION_TEXTURE2DARRAY,
    Texture2DMS = D3D12_DSV_DIMENSION_TEXTURE2DMS,
    Texture2DMSArray = D3D12_DSV_DIMENSION_TEXTURE2DMSARRAY,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct DSVFlags(u32);
#[allow(non_upper_case_globals)]
impl DSVFlags {
    pub const None: Self = Self(D3D12_DSV_FLAG_NONE);
    pub const ReadOnlyDepth: Self = Self(D3D12_DSV_FLAG_READ_ONLY_DEPTH);
    pub const ReadOnlyStencil: Self = Self(D3D12_DSV_FLAG_READ_ONLY_STENCIL);
}
impl_bitflag_operators!(DSVFlags);

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ElementsLayout {
    Array = D3D12_ELEMENTS_LAYOUT_ARRAY,
    ArrayOfPointers = D3D12_ELEMENTS_LAYOUT_ARRAY_OF_POINTERS,
}
*/

/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ExportFlags(u32);
#[allow(non_upper_case_globals)]
impl ExportFlags {
    pub const None: Self = Self(D3D12_EXPORT_FLAG_NONE);
}
impl_bitflag_operators!(ExportFlags);
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Feature {
    D3D12Options = D3D12_FEATURE_D3D12_OPTIONS,
    Architecture = D3D12_FEATURE_ARCHITECTURE,
    FeatureLevels = D3D12_FEATURE_FEATURE_LEVELS,
    FormatSupport = D3D12_FEATURE_FORMAT_SUPPORT,
    MultisampleQualityLevels = D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS,
    FormatInfo = D3D12_FEATURE_FORMAT_INFO,
    GPUVirtualAddressSupport = D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT,
    ShaderModel = D3D12_FEATURE_SHADER_MODEL,
    D3D12Option1 = D3D12_FEATURE_D3D12_OPTIONS1,
    // ProtectedResourceSessionSupport = D3D12_FEATURE_PROTECTED_RESOURCE_SESSION_SUPPORT,
    RootSignature = D3D12_FEATURE_ROOT_SIGNATURE,
    Architecture1 = D3D12_FEATURE_ARCHITECTURE1,
    D3D12Option2 = D3D12_FEATURE_D3D12_OPTIONS2,
    ShaderCache = D3D12_FEATURE_SHADER_CACHE,
    CommandQueuePriority = D3D12_FEATURE_COMMAND_QUEUE_PRIORITY,
    // D3D12Options3 = D3D12_FEATURE_D3D12_OPTIONS3,
    // ExistingHeaps = D3D12_FEATURE_EXISTING_HEAPS,
    // D3D12Options4 = D3D12_FEATURE_D3D12_OPTIONS4,
    // Serialization = D3D12_FEATURE_SERIALIZATION,
    // CrossNode = D3D12_FEATURE_CROSS_NODE,
    // D3D12Options5 = D3D12_FEATURE_D3D12_OPTIONS5,
    // D3D12Options6 = D3D12_FEATURE_D3D12_OPTIONS6,
    // QueryMetaCommand = D3D12_FEATURE_QUERY_META_COMMAND
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct FenceFlags(u32);
#[allow(non_upper_case_globals)]
impl FenceFlags {
    pub const None: Self = Self(D3D12_FENCE_FLAG_NONE);
    pub const Shared: Self = Self(D3D12_FENCE_FLAG_SHARED);
    pub const SharedCrossAdapter: Self = Self(D3D12_FENCE_FLAG_SHARED_CROSS_ADAPTER);
    // pub const NonMonitored: Self = Self(D3D12_FENCE_FLAG_NON_MONITORED);
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FillMode {
    Wireframe = D3D12_FILL_MODE_WIREFRAME,
    Solid = D3D12_FILL_MODE_SOLID,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Filter {
    MinMagMipPoint = D3D12_FILTER_MIN_MAG_MIP_POINT,
    MinMagPointMipLinear = D3D12_FILTER_MIN_MAG_POINT_MIP_LINEAR,
    MinPointMagLinearMipPoint = D3D12_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT,
    MinPointMagMipLinear = D3D12_FILTER_MIN_POINT_MAG_MIP_LINEAR,
    MinLinearMagMipPoint = D3D12_FILTER_MIN_LINEAR_MAG_MIP_POINT,
    MinLinearMagPointMipLinear = D3D12_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
    MinMagLinearMipPoint = D3D12_FILTER_MIN_MAG_LINEAR_MIP_POINT,
    MinMagMipLinear = D3D12_FILTER_MIN_MAG_MIP_LINEAR,
    Anisotropic = D3D12_FILTER_ANISOTROPIC,
    ComparisonMinMagMipPoint = D3D12_FILTER_COMPARISON_MIN_MAG_MIP_POINT,
    ComparisonMinMagPointMipLinear = D3D12_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR,
    ComparisonMinPointMagLinearMipPoint = D3D12_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT,
    ComparisonMinPointMagMipLinear = D3D12_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR,
    ComparisonMinLinearMagMipPoint = D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT,
    ComparisonMinLinearMagPointMipLinear = D3D12_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
    ComparisonMinMagLinearMipPoint = D3D12_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT,
    ComparisonMinMagMipLinear = D3D12_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR,
    ComparisonAnisotropic = D3D12_FILTER_COMPARISON_ANISOTROPIC,
    MinimumMinMagMipPoint = D3D12_FILTER_MINIMUM_MIN_MAG_MIP_POINT,
    MinimumMinMagPointMipLinear = D3D12_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR,
    MinimumMinPointMagLinearMipPoint = D3D12_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT,
    MinimumMinPointMagMipLinear = D3D12_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR,
    MinimumMinLinearMagMipPoint = D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT,
    MinimumMinLinearMagPointMipLinear = D3D12_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
    MinimumMinMagLinearMipPoint = D3D12_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT,
    MinimumMinMagMipLinear = D3D12_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR,
    MinimumAnisotropic = D3D12_FILTER_MINIMUM_ANISOTROPIC,
    MaximumMinMagMipPoint = D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_POINT,
    MaximumMinMagPointMipLinear = D3D12_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR,
    MaximumPointMagLinearMipPoint = D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT,
    MaximumMinPointMagMipLinear = D3D12_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR,
    MaximumMinLinearMagMipPoint = D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT,
    MaximumMinLinearMagPointMipLinear = D3D12_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
    MaximumMinMagLinearMipPoint = D3D12_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT,
    MaximumMinMagMipLinear = D3D12_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR,
    MaximumAnisotropic = D3D12_FILTER_MAXIMUM_ANISOTROPIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FilterRedutionType {
    Standard = D3D12_FILTER_REDUCTION_TYPE_STANDARD,
    Comparison = D3D12_FILTER_REDUCTION_TYPE_COMPARISON,
    Minimum = D3D12_FILTER_REDUCTION_TYPE_MINIMUM,
    Maximum = D3D12_FILTER_REDUCTION_TYPE_MAXIMUM,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FilterType {
    Point = D3D12_FILTER_TYPE_POINT,
    Linear = D3D12_FILTER_TYPE_LINEAR,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct FormatSupport1(u32);
#[allow(non_upper_case_globals)]
impl FormatSupport1 {
    pub const None: Self = Self(D3D12_FORMAT_SUPPORT1_NONE);
    pub const Buffer: Self = Self(D3D12_FORMAT_SUPPORT1_BUFFER);
    pub const IAVertexBuffer: Self = Self(D3D12_FORMAT_SUPPORT1_IA_VERTEX_BUFFER);
    pub const IAIndexBuffer: Self = Self(D3D12_FORMAT_SUPPORT1_IA_INDEX_BUFFER);
    pub const SOBuffer: Self = Self(D3D12_FORMAT_SUPPORT1_SO_BUFFER);
    pub const Texture1D: Self = Self(D3D12_FORMAT_SUPPORT1_TEXTURE1D);
    pub const Texture2D: Self = Self(D3D12_FORMAT_SUPPORT1_TEXTURE2D);
    pub const Texture3D: Self = Self(D3D12_FORMAT_SUPPORT1_TEXTURE3D);
    pub const TextureCube: Self = Self(D3D12_FORMAT_SUPPORT1_TEXTURECUBE);
    pub const ShaderLoad: Self = Self(D3D12_FORMAT_SUPPORT1_SHADER_LOAD);
    pub const ShaderSample: Self = Self(D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE);
    pub const ShaderSampleComparison: Self = Self(D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_COMPARISON);
    pub const ShaderSampleMonoText: Self = Self(D3D12_FORMAT_SUPPORT1_SHADER_SAMPLE_MONO_TEXT);
    pub const Mip: Self = Self(D3D12_FORMAT_SUPPORT1_MIP);
    pub const RenderTarget: Self = Self(D3D12_FORMAT_SUPPORT1_RENDER_TARGET);
    pub const Blendable: Self = Self(D3D12_FORMAT_SUPPORT1_BLENDABLE);
    pub const DepthStencil: Self = Self(D3D12_FORMAT_SUPPORT1_DEPTH_STENCIL);
    pub const MultisampleResolve: Self = Self(D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RESOLVE);
    pub const Display: Self = Self(D3D12_FORMAT_SUPPORT1_DISPLAY);
    pub const CastWithinBitLayout: Self = Self(D3D12_FORMAT_SUPPORT1_CAST_WITHIN_BIT_LAYOUT);
    pub const MultisampleRenderTarget: Self = Self(D3D12_FORMAT_SUPPORT1_MULTISAMPLE_RENDERTARGET);
    pub const MultisampleLoad: Self = Self(D3D12_FORMAT_SUPPORT1_MULTISAMPLE_LOAD);
    pub const ShaderGather: Self = Self(D3D12_FORMAT_SUPPORT1_SHADER_GATHER);
    pub const BackBufferCast: Self = Self(D3D12_FORMAT_SUPPORT1_BACK_BUFFER_CAST);
    pub const TypedUnorderedAccessView: Self =
        Self(D3D12_FORMAT_SUPPORT1_TYPED_UNORDERED_ACCESS_VIEW);
    pub const ShaderGatherComparison: Self = Self(D3D12_FORMAT_SUPPORT1_SHADER_GATHER_COMPARISON);
    pub const DecoderOutput: Self = Self(D3D12_FORMAT_SUPPORT1_DECODER_OUTPUT);
    pub const VideoProcessorOutput: Self = Self(D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_OUTPUT);
    pub const VideoProcessorInput: Self = Self(D3D12_FORMAT_SUPPORT1_VIDEO_PROCESSOR_INPUT);
    pub const VideoEncoder: Self = Self(D3D12_FORMAT_SUPPORT1_VIDEO_ENCODER);
}
impl_bitflag_operators!(FormatSupport1);

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct FormatSupport2(u32);
#[allow(non_upper_case_globals)]
impl FormatSupport2 {
    pub const None: Self = Self(D3D12_FORMAT_SUPPORT2_NONE);
    pub const UAVAtomicAdd: Self = Self(D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_ADD);
    pub const UAVAtomicBitwiseOps: Self = Self(D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS);
    pub const UAVAtomicCompareStoreOrCompareExchange: Self =
        Self(D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE);
    pub const UAVAtomicExchange: Self = Self(D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE);
    pub const UAVAtomicSignedMinOrMax: Self =
        Self(D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX);
    pub const UAVAtomicUnsignedMinOrMax: Self =
        Self(D3D12_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX);
    pub const UAVTypedLoad: Self = Self(D3D12_FORMAT_SUPPORT2_UAV_TYPED_LOAD);
    pub const UAVTypedStore: Self = Self(D3D12_FORMAT_SUPPORT2_UAV_TYPED_STORE);
    pub const OutputMergerLogicOp: Self = Self(D3D12_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP);
    pub const Tiled: Self = Self(D3D12_FORMAT_SUPPORT2_TILED);
    pub const MultiplaneOverlay: Self = Self(D3D12_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY);
}
impl_bitflag_operators!(FormatSupport2);

/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct GraphicsStates(u32);
#[allow(non_upper_case_globals)]
impl GraphicsStates {
    pub const None: Self = Self(D3D12_GRAPHICS_STATE_NONE);
    pub const IAVertexBuffers: Self = Self(D3D12_GRAPHICS_STATE_IA_VERTEX_BUFFERS);
    pub const IAIndexBuffer: Self = Self(D3D12_GRAPHICS_STATE_IA_INDEX_BUFFER);
    pub const IAPrimitiveTopology: Self = Self(D3D12_GRAPHICS_STATE_IA_PRIMITIVE_TOPOLOGY);
    pub const DescriptorHeap: Self = Self(D3D12_GRAPHICS_STATE_DESCRIPTOR_HEAP);
    pub const GraphicsRootSignature: Self = Self(D3D12_GRAPHICS_STATE_GRAPHICS_ROOT_SIGNATURE);
    pub const ComputeRootSignature: Self = Self(D3D12_GRAPHICS_STATE_COMPUTE_ROOT_SIGNATURE);
    pub const GraphicsStateRSViewports: Self = Self(D3D12_GRAPHICS_STATE_RS_VIEWPORTS);
    pub const RSScissorRects: Self = Self(D3D12_GRAPHICS_STATE_RS_SCISSOR_RECTS);
    pub const Predication: Self = Self(D3D12_GRAPHICS_STATE_PREDICATION);
    pub const OMRenderTargets: Self = Self(D3D12_GRAPHICS_STATE_OM_RENDER_TARGETS);
    pub const OMStencilRef: Self = Self(D3D12_GRAPHICS_STATE_OM_STENCIL_REF);
    pub const OMBlendFactor: Self = Self(D3D12_GRAPHICS_STATE_OM_BLEND_FACTOR);
    pub const PipelineState: Self = Self(D3D12_GRAPHICS_STATE_PIPELINE_STATE);
    pub const SOTargets: Self = Self(D3D12_GRAPHICS_STATE_SO_TARGETS);
    pub const OMDepthBounds: Self = Self(D3D12_GRAPHICS_STATE_OM_DEPTH_BOUNDS);
    pub const SamplePositions: Self = Self(D3D12_GRAPHICS_STATE_SAMPLE_POSITIONS);
    pub const ViewInstanceMask: Self = Self(D3D12_GRAPHICS_STATE_VIEW_INSTANCE_MASK);
}
impl_bitflag_operators!(GraphicsStates);
*/

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct HeapFlags(u32);
#[allow(non_upper_case_globals)]
impl HeapFlags {
    pub const None: Self = Self(D3D12_HEAP_FLAG_NONE);
    pub const Shared: Self = Self(D3D12_HEAP_FLAG_SHARED);
    pub const DenyBuffers: Self = Self(D3D12_HEAP_FLAG_DENY_BUFFERS);
    pub const AllowDisplay: Self = Self(D3D12_HEAP_FLAG_ALLOW_DISPLAY);
    pub const SharedCrossAdapter: Self = Self(D3D12_HEAP_FLAG_SHARED_CROSS_ADAPTER);
    pub const DenyRtDsTextures: Self = Self(D3D12_HEAP_FLAG_DENY_RT_DS_TEXTURES);
    pub const DenyNonRtDsTextures: Self = Self(D3D12_HEAP_FLAG_DENY_NON_RT_DS_TEXTURES);
    pub const HardwareProtected: Self = Self(D3D12_HEAP_FLAG_HARDWARE_PROTECTED);
    pub const AllowWriteWatch: Self = Self(D3D12_HEAP_FLAG_ALLOW_WRITE_WATCH);
    // pub const AllowShaderAtomics: Self = Self(D3D12_HEAP_FLAG_ALLOW_SHADER_ATOMICS);
    pub const AllowAllBuffersAndTextures: Self =
        Self(D3D12_HEAP_FLAG_ALLOW_ALL_BUFFERS_AND_TEXTURES);
    pub const AllowOnlyBuffers: Self = Self(D3D12_HEAP_FLAG_ALLOW_ONLY_BUFFERS);
    pub const AllowOnlyNonRtDsTextures: Self = Self(D3D12_HEAP_FLAG_ALLOW_ONLY_NON_RT_DS_TEXTURES);
    pub const AllowOnlyRtDsTextures: Self = Self(D3D12_HEAP_FLAG_ALLOW_ONLY_RT_DS_TEXTURES);
}
impl_bitflag_operators!(HeapFlags);

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HaepSerializationTier {
    Tier0 = D3D12_HEAP_SERIALIZATION_TIER_0,
    Tier10 = D3D12_HEAP_SERIALIZATION_TIER_10,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HeapType {
    Default = D3D12_HEAP_TYPE_DEFAULT,
    Upload = D3D12_HEAP_TYPE_UPLOAD,
    ReadBack = D3D12_HEAP_TYPE_READBACK,
    Custom = D3D12_HEAP_TYPE_CUSTOM,
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HitGroupType {
    Triangles = D3D12_HIT_GROUP_TYPE_TRIANGLES,
    ProceduralPrimitive = D3D12_HIT_GROUP_TYPE_PROCEDURAL_PRIMITIVE,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum IndexBufferStripCutValue {
    Disabled = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_DISABLED,
    FFFF = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFF,
    FFFFFFFF = D3D12_INDEX_BUFFER_STRIP_CUT_VALUE_0xFFFFFFFF,
}
impl Default for IndexBufferStripCutValue {
    fn default() -> Self {
        IndexBufferStripCutValue::Disabled
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum IndirectArgumentType {
    Draw = D3D12_INDIRECT_ARGUMENT_TYPE_DRAW,
    DrawIndexed = D3D12_INDIRECT_ARGUMENT_TYPE_DRAW_INDEXED,
    Dispatch = D3D12_INDIRECT_ARGUMENT_TYPE_DISPATCH,
    VertexBufferView = D3D12_INDIRECT_ARGUMENT_TYPE_VERTEX_BUFFER_VIEW,
    IndexBufferView = D3D12_INDIRECT_ARGUMENT_TYPE_INDEX_BUFFER_VIEW,
    Constant = D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT,
    ConstantBufferView = D3D12_INDIRECT_ARGUMENT_TYPE_CONSTANT_BUFFER_VIEW,
    ShaderResourceView = D3D12_INDIRECT_ARGUMENT_TYPE_SHADER_RESOURCE_VIEW,
    UnorderedAccessView = D3D12_INDIRECT_ARGUMENT_TYPE_UNORDERED_ACCESS_VIEW,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum InputClassification {
    PerVertexData = D3D12_INPUT_CLASSIFICATION_PER_VERTEX_DATA,
    PerInstancexData = D3D12_INPUT_CLASSIFICATION_PER_INSTANCE_DATA,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LogicOp {
    Clear = D3D12_LOGIC_OP_CLEAR,
    Set = D3D12_LOGIC_OP_SET,
    Copy = D3D12_LOGIC_OP_COPY,
    CopyInverted = D3D12_LOGIC_OP_COPY_INVERTED,
    Noop = D3D12_LOGIC_OP_NOOP,
    Invert = D3D12_LOGIC_OP_INVERT,
    And = D3D12_LOGIC_OP_AND,
    Nand = D3D12_LOGIC_OP_NAND,
    Or = D3D12_LOGIC_OP_OR,
    Nor = D3D12_LOGIC_OP_NOR,
    Xor = D3D12_LOGIC_OP_XOR,
    Equiv = D3D12_LOGIC_OP_EQUIV,
    AndReverse = D3D12_LOGIC_OP_AND_REVERSE,
    AndInverted = D3D12_LOGIC_OP_AND_INVERTED,
    OrReverse = D3D12_LOGIC_OP_OR_REVERSE,
    OrInverted = D3D12_LOGIC_OP_OR_INVERTED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum MemoryPool {
    Unknown = D3D12_MEMORY_POOL_UNKNOWN,
    L0 = D3D12_MEMORY_POOL_L0,
    L1 = D3D12_MEMORY_POOL_L1,
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MetaCommandParameterFlags(u32);
#[allow(non_upper_case_globals)]
impl MetaCommandParameterFlags {
    pub const Input: Self = Self(D3D12_META_COMMAND_PARAMETER_FLAG_INPUT);
    pub const Output: Self = Self(D3D12_META_COMMAND_PARAMETER_FLAG_OUTPUT);
}
impl_bitflag_operators!(MetaCommandParameterFlags);
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum MetaCommandParameterStage {
    Creation = D3D12_META_COMMAND_PARAMETER_STAGE_CREATION,
    Initialization = D3D12_META_COMMAND_PARAMETER_STAGE_INITIALIZATION,
    Execution = D3D12_META_COMMAND_PARAMETER_STAGE_EXECUTION,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum MetaCommandParameterType {
    Float = D3D12_META_COMMAND_PARAMETER_TYPE_FLOAT,
    Uint64 = D3D12_META_COMMAND_PARAMETER_TYPE_UINT64,
    GPUVirtualAddress = D3D12_META_COMMAND_PARAMETER_TYPE_GPU_VIRTUAL_ADDRESS,
    CPUDescritorHandleHeapTypeCBVSRVUAV = D3D12_META_COMMAND_PARAMETER_TYPE_CPU_DESCRIPTOR_HANDLE_HEAP_TYPE_CBV_SRV_UAV,
    GPUDescritorHandleHeapTypeCBVSRVUAV = D3D12_META_COMMAND_PARAMETER_TYPE_GPU_DESCRIPTOR_HANDLE_HEAP_TYPE_CBV_SRV_UAV,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct MultipleFenceWaitFlags(u32);
#[allow(non_upper_case_globals)]
impl MultipleFenceWaitFlags {
    pub const None: Self = Self(D3D12_MULTIPLE_FENCE_WAIT_FLAG_NONE);
    pub const Any: Self = Self(D3D12_MULTIPLE_FENCE_WAIT_FLAG_ANY);
    pub const All: Self = Self(D3D12_MULTIPLE_FENCE_WAIT_FLAG_ALL);
}
impl_bitflag_operators!(MultipleFenceWaitFlags);
/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct MultisampleQualityLevelFlags(u32);
#[allow(non_upper_case_globals)]
impl MultisampleQualityLevelFlags {
    pub const None: Self = Self(D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAG_NONE);
    pub const TiledResource: Self = Self(D3D12_MULTISAMPLE_QUALITY_LEVEL_FLAG_TILE_RESOURCE);
}
impl_bitflag_operators!(MultisampleQualityLevelFlags);
*/

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct PipelineStateFlags(u32);
#[allow(non_upper_case_globals)]
impl PipelineStateFlags {
    pub const None: Self = Self(D3D12_PIPELINE_STATE_FLAG_NONE);
    pub const ToolDebug: Self = Self(D3D12_PIPELINE_STATE_FLAG_TOOL_DEBUG);
}
impl_bitflag_operators!(PipelineStateFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PipelineStateSubObjectType {
    RootSignature = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_ROOT_SIGNATURE,
    VS = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VS,
    PS = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PS,
    DS = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DS,
    HS = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_HS,
    GS = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_GS,
    CS = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CS,
    StreamOutput = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_STREAM_OUTPUT,
    Blend = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_BLEND,
    SampleMask = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_MASK,
    Rasterizer = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RASTERIZER,
    DepthStencil = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL,
    InputLayout = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_INPUT_LAYOUT,
    IBStripCutValue = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_IB_STRIP_CUT_VALUE,
    PrimitiveTopology = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_PRIMITIVE_TOPOLOGY,
    RenderTargetFormats = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_RENDER_TARGET_FORMATS,
    DepthStencilFormat = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL_FORMAT,
    SampleDesc = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_SAMPLE_DESC,
    NodeMask = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_NODE_MASK,
    CachedPSO = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_CACHED_PSO,
    Flags = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_FLAGS,
    Stencil1 = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_DEPTH_STENCIL1,
    // ViewInstancing = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_VIEW_INSTANCING,
    // MaxValid = D3D12_PIPELINE_STATE_SUBOBJECT_TYPE_MAX_VALID,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PredicationOp {
    EqualZero = D3D12_PREDICATION_OP_EQUAL_ZERO,
    NotEqualZero = D3D12_PREDICATION_OP_NOT_EQUAL_ZERO,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PrimitiveTopologyType {
    Undefined = D3D12_PRIMITIVE_TOPOLOGY_TYPE_UNDEFINED,
    Point = D3D12_PRIMITIVE_TOPOLOGY_TYPE_POINT,
    Line = D3D12_PRIMITIVE_TOPOLOGY_TYPE_LINE,
    Triangle = D3D12_PRIMITIVE_TOPOLOGY_TYPE_TRIANGLE,
    Patch = D3D12_PRIMITIVE_TOPOLOGY_TYPE_PATCH,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ProgrammableSamplePositionsTier {
    NotSupported = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_NOT_SUPPORTED,
    Tier1 = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_1,
    Tier2 = D3D12_PROGRAMMABLE_SAMPLE_POSITIONS_TIER_2,
}
/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ProtectedResourceSessionSupportFlags(u32);
#[allow(non_upper_case_globals)]
impl ProtectedResourceSessionSupportFlags {
    pub const None: Self = Self(D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_NONE);
    pub const Supported: Self = Self(D3D12_PROTECTED_RESOURCE_SESSION_SUPPORT_FLAG_SUPPORTED);
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum QueryHeapType {
    Occlusion = D3D12_QUERY_HEAP_TYPE_OCCLUSION,
    Timestamp = D3D12_QUERY_HEAP_TYPE_TIMESTAMP,
    PipelineStatistics = D3D12_QUERY_HEAP_TYPE_PIPELINE_STATISTICS,
    SOStatistics = D3D12_QUERY_HEAP_TYPE_SO_STATISTICS,
    // VideoDecodeStatistics = D3D12_QUERY_HEAP_TYPE_VIDEO_DECODE_STATISTICS,
    // CopyQueueTimestamp = D3D12_QUERY_HEAP_TYPE_COPY_QUEUE_TIMESTAMP
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum QueryType {
    Occlusion = D3D12_QUERY_TYPE_OCCLUSION,
    BinaryOcclusion = D3D12_QUERY_TYPE_BINARY_OCCLUSION,
    Timestamp = D3D12_QUERY_TYPE_TIMESTAMP,
    PipelineStatistics = D3D12_QUERY_TYPE_PIPELINE_STATISTICS,
    SOStatisticsStream0 = D3D12_QUERY_TYPE_SO_STATISTICS_STREAM0,
    SOStatisticsStream1 = D3D12_QUERY_TYPE_SO_STATISTICS_STREAM1,
    SOStatisticsStream2 = D3D12_QUERY_TYPE_SO_STATISTICS_STREAM2,
    SOStatisticsStream3 = D3D12_QUERY_TYPE_SO_STATISTICS_STREAM3,
    // VideoDecodeStatistics = D3D12_QUERY_TYPE_VIDEO_DECODE_STATISTICS,
}
/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct RayFlags(u32);
#[allow(non_upper_case_globals)]
impl RayFlags {
    pub const None: Self = Self(D3D12_RAY_FLAG_NONE);
    pub const ForceOpaque: Self = Self(D3D12_RAY_FLAG_FORCE_OPAQUE);
    pub const ForceNonOpaque: Self = Self(D3D12_RAY_FLAG_FORCE_NON_OPAQUE);
    pub const AcceptFirstHitAndEndSearch: Self = Self(D3D12_RAY_FLAG_ACCEPT_FIRST_HIT_AND_END_SEARCH);
    pub const SkipClosestHitShader: Self = Self(D3D12_RAY_FLAG_SKIP_CLOSEST_HIT_SHADER);
    pub const BackFacingTriangles: Self = Self(D3D12_RAY_FLAG_CULL_BACK_FACING_TRIANGLES);
    pub const FrontFacingTriangles: Self = Self(D3D12_RAY_FLAG_CULL_FRONT_FACING_TRIANGLES);
    pub const CullOpaque: Self = Self(D3D12_RAY_FLAG_CULL_OPAQUE);
    pub const CullNonOpaque: Self = Self(D3D12_RAY_FLAG_CULL_NON_OPAQUE);
}
impl_bitflag_operators!(RayFlags);
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct RaytracingAccelerationStructureBuildFlags(u32);
#[allow(non_upper_case_globals)]
impl RaytracingAccelerationStructureBuildFlags {
    pub const None: Self = Self(D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_NONE);
    pub const AllowUpdate: Self = Self(D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_UPDATE);
    pub const AllowCompaction: Self = Self(D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_ALLOW_COMPACTION);
    pub const PreferFastTrace: Self = Self(D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_TRACE);
    pub const PreferFastBuild: Self = Self(D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PREFER_FAST_BUILD);
    pub const MinimizeMemory: Self = Self(D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_MINIMIZE_MEMORY);
    pub const PerformUpdate: Self = Self(D3D12_RAYTRACING_ACCELERATION_STRUCTURE_BUILD_FLAG_PERFORM_UPDATE
}
impl_bitflag_operators!(RaytracingAccelerationStructureBuildFlags);
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RaytracingAccelerationStructureCopyMode {
    Clone = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_CLONE,
    Compact = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_COMPACT,
    VisualizationDecodeForTools = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_VISUALIZATION_DECODE_FOR_TOOLS,
    Serialize = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_SERIALIZE,
    Deserialize = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_COPY_MODE_DESERIALIZE,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RaytracingAccelerationStructurePostbuildInfoType {
    CompactedSize = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_COMPACTED_SIZE,
    ToolsVisualization = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_TOOLS_VISUALIZATION,
    Serialization = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_SERIALIZATION,
    CurrentSize = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_POSTBUILD_INFO_CURRENT_SIZE,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RaytracingAccelerationStructureType {
    TopLevel = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_TOP_LEVEL,
    BottomLevel = D3D12_RAYTRACING_ACCELERATION_STRUCTURE_TYPE_BOTTOM_LEVEL,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct RaytracingGeometryFlags(u32);
#[allow(non_upper_case_globals)]
impl RaytracingGeometryFlags {
    pub const None: Self = Self(D3D12_RAYTRACING_GEOMETRY_FLAG_NONE);
    pub const Opaque: Self = Self(D3D12_RAYTRACING_GEOMETRY_FLAG_OPAQUE);
    pub const NoDeplicateAnyHitInvocation: Self = Self(D3D12_RAYTRACING_GEOMETRY_FLAG_NO_DUPLICATE_ANYHIT_INVOCATION);
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RaytracingGeoemtyType {
    Triangles = D3D12_RAYTRACING_GEOMETRY_TYPE_TRIANGLES,
    ProceduralPrimitiveAABBS = D3D12_RAYTRACING_GEOMETRY_TYPE_PROCEDURAL_PRIMITIVE_AABBS,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct RaytracingInstanceFlags(u32);
#[allow(non_upper_case_globals)]
impl RaytracingInstanceFlags {
    pub const None: Self = Self(D3D12_RAYTRACING_INSTANCE_FLAG_NONE);
    pub const TriangleCullDisable: Self = Self(D3D12_RAYTRACING_INSTANCE_FLAG_TRIANGLE_CULL_DISABLE);
    pub const TriangleFrontCounterclockwise: Self = Self(D3D12_RAYTRACING_INSTANCE_FLAG_TRIANGLE_FRONT_COUNTERCLOCKWISE);
    pub const InstanceFlagForceOpaque: Self = Self(D3D12_RAYTRACING_INSTANCE_FLAG_FORCE_OPAQUE);
    pub const InstanceFlagForceNonOpaque: Self = Self(D3D12_RAYTRACING_INSTANCE_FLAG_FORCE_NON_OPAQUE);
}
impl_bitflag_operators!(RaytracingInstanceFlags);
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RaytracingTier {
    NotSupported = D3D12_RAYTRACING_TIER_NOT_SUPPORTED,
    Tier1_0 = D3D12_RAYTRACING_TIER_1_0,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RenderPassBeginningAccessType {
    Discard = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_DISCARD,
    Preserve = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_PRESERVE,
    Clear = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_CLEAR,
    NoAccess = D3D12_RENDER_PASS_BEGINNING_ACCESS_TYPE_NO_ACCESS,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RenderPassEndingAccessType {
    Discard = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_DISCARD,
    Preserve = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_PRESERVE,
    Resolve = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_RESOLVE,
    NoAccess = D3D12_RENDER_PASS_ENDING_ACCESS_TYPE_NO_ACCESS,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct RenderPassFlags(u32);
#[allow(non_upper_case_globals)]
impl RenderPassFlags {
    pub const None: Self = Self(D3D12_RENDER_PASS_FLAG_NONE);
    pub const AllowUAVWrites: Self = Self(D3D12_RENDER_PASS_FLAG_ALLOW_UAV_WRITES);
    pub const SuspendingPass: Self = Self(D3D12_RENDER_PASS_FLAG_SUSPENDING_PASS);
    pub const ResumingPass: Self = Self(D3D12_RENDER_PASS_FLAG_RESUMING_PASS);
}
impl_bitflag_operators!(RenderPassFlags);
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum RenderPassTier {
    Tier0 = D3D12_RENDER_PASS_TIER_0,
    Tier1 = D3D12_RENDER_PASS_TIER_1,
    Tier2 = D3D12_RENDER_PASS_TIER_2,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ResidencyFlags(u32);
#[allow(non_upper_case_globals)]
impl ResidencyFlags {
    pub const None: Self = Self(D3D12_RESIDENCY_FLAG_NONE);
    pub const DenyOverbudget: Self = Self(D3D12_RESIDENCY_FLAG_DENY_OVERBUDGET);
}
impl_bitflag_operators!(ResidencyFlags);
*/
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ResidencyPriority {
    Minimum = D3D12_RESIDENCY_PRIORITY_MINIMUM,
    Low = D3D12_RESIDENCY_PRIORITY_LOW,
    Normal = D3D12_RESIDENCY_PRIORITY_NORMAL,
    High = D3D12_RESIDENCY_PRIORITY_HIGH,
    Maximum = D3D12_RESIDENCY_PRIORITY_MAXIMUM,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ResolveMode {
    Decompress = D3D12_RESOLVE_MODE_DECOMPRESS,
    Min = D3D12_RESOLVE_MODE_MIN,
    Max = D3D12_RESOLVE_MODE_MAX,
    Average = D3D12_RESOLVE_MODE_AVERAGE,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ResourceBarrierFlags(u32);
#[allow(non_upper_case_globals)]
impl ResourceBarrierFlags {
    pub const None: Self = Self(D3D12_RESOURCE_BARRIER_FLAG_NONE);
    pub const BeginOnly: Self = Self(D3D12_RESOURCE_BARRIER_FLAG_BEGIN_ONLY);
    pub const EndOnly: Self = Self(D3D12_RESOURCE_BARRIER_FLAG_END_ONLY);
}
impl_bitflag_operators!(ResourceBarrierFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ResourceBarrierType {
    Transition = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION,
    Aliasing = D3D12_RESOURCE_BARRIER_TYPE_ALIASING,
    UAV = D3D12_RESOURCE_BARRIER_TYPE_UAV,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ResourceBindingTier {
    Tier1 = D3D12_RESOURCE_BINDING_TIER_1,
    Tier2 = D3D12_RESOURCE_BINDING_TIER_2,
    Tier3 = D3D12_RESOURCE_BINDING_TIER_3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ResourceDimension {
    Unknown = D3D12_RESOURCE_DIMENSION_UNKNOWN,
    Buffer = D3D12_RESOURCE_DIMENSION_BUFFER,
    Texture1D = D3D12_RESOURCE_DIMENSION_TEXTURE1D,
    Texture2D = D3D12_RESOURCE_DIMENSION_TEXTURE2D,
    Texture3D = D3D12_RESOURCE_DIMENSION_TEXTURE3D,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ResourceFlags(u32);
#[allow(non_upper_case_globals)]
impl ResourceFlags {
    pub const None: Self = Self(D3D12_RESOURCE_FLAG_NONE);
    pub const AllowRenderTarget: Self = Self(D3D12_RESOURCE_FLAG_ALLOW_RENDER_TARGET);
    pub const AllowDepthStencil: Self = Self(D3D12_RESOURCE_FLAG_ALLOW_DEPTH_STENCIL);
    pub const AllowUnorderedAccess: Self = Self(D3D12_RESOURCE_FLAG_ALLOW_UNORDERED_ACCESS);
    pub const DenyShaderResource: Self = Self(D3D12_RESOURCE_FLAG_DENY_SHADER_RESOURCE);
    pub const AllowCrossAdapter: Self = Self(D3D12_RESOURCE_FLAG_ALLOW_CROSS_ADAPTER);
    pub const AllowSimutaneousAccess: Self = Self(D3D12_RESOURCE_FLAG_ALLOW_SIMULTANEOUS_ACCESS);
    // pub const VideoDecodeReferenceOnly: Self = Self(D3D12_RESOURCE_FLAG_VIDEO_DECODE_REFERENCE_ONLY);
}
impl_bitflag_operators!(ResourceFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ResourceHeapTier {
    Tier1 = D3D12_RESOURCE_HEAP_TIER_1,
    Tier2 = D3D12_RESOURCE_HEAP_TIER_2,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ResourceStates(u32);
#[allow(non_upper_case_globals)]
impl ResourceStates {
    pub const Common: Self = Self(D3D12_RESOURCE_STATE_COMMON);
    pub const VertexAndConstantBuffer: Self = Self(D3D12_RESOURCE_STATE_VERTEX_AND_CONSTANT_BUFFER);
    pub const IndexBuffer: Self = Self(D3D12_RESOURCE_STATE_INDEX_BUFFER);
    pub const RenderTarget: Self = Self(D3D12_RESOURCE_STATE_RENDER_TARGET);
    pub const UnorderedAccess: Self = Self(D3D12_RESOURCE_STATE_UNORDERED_ACCESS);
    pub const DepthWrite: Self = Self(D3D12_RESOURCE_STATE_DEPTH_WRITE);
    pub const DepthRead: Self = Self(D3D12_RESOURCE_STATE_DEPTH_READ);
    pub const NonPixelShaderResource: Self = Self(D3D12_RESOURCE_STATE_NON_PIXEL_SHADER_RESOURCE);
    pub const PixelShaderResource: Self = Self(D3D12_RESOURCE_STATE_PIXEL_SHADER_RESOURCE);
    pub const SteamOut: Self = Self(D3D12_RESOURCE_STATE_STREAM_OUT);
    pub const IndirectArgument: Self = Self(D3D12_RESOURCE_STATE_INDIRECT_ARGUMENT);
    pub const CopyDest: Self = Self(D3D12_RESOURCE_STATE_COPY_DEST);
    pub const CopySource: Self = Self(D3D12_RESOURCE_STATE_COPY_SOURCE);
    pub const ResolveDest: Self = Self(D3D12_RESOURCE_STATE_RESOLVE_DEST);
    pub const ResolveSource: Self = Self(D3D12_RESOURCE_STATE_RESOLVE_SOURCE);
    pub const GenericRead: Self = Self(D3D12_RESOURCE_STATE_GENERIC_READ);
    // pub const RaytracingAccelerationStructure: Self = Self(D3D12_RESOURCE_STATE_RAYTRACING_ACCELERATION_STRUCTURE);
    pub const Present: Self = Self(D3D12_RESOURCE_STATE_PRESENT);
    // pub const ShadingRateSource: Self = Self(D3D12_RESOURCE_STATE_SHADING_RATE_SOURCE);
    pub const Predication: Self = Self(D3D12_RESOURCE_STATE_PREDICATION);
    // pub const VideoDecodeRead: Self = Self(D3D12_RESOURCE_STATE_VIDEO_DECODE_READ);
    // pub const VideoDecodeWrite: Self = Self(D3D12_RESOURCE_STATE_VIDEO_DECODE_WRITE);
    // pub const VideoProcessRead: Self = Self(D3D12_RESOURCE_STATE_VIDEO_PROCESS_READ);
    // pub const VideoProcessWrite: Self = Self(D3D12_RESOURCE_STATE_VIDEO_PROCESS_WRITE);
    // pub const VideoEncodeRead: Self = Self(D3D12_RESOURCE_STATE_VIDEO_ENCODE_READ);
    // pub const VideoEncodeWrite: Self = Self(D3D12_RESOURCE_STATE_VIDEO_ENCODE_WRITE);
}
impl_bitflag_operators!(ResourceStates);

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct RootDescriptorFlags(u32);
#[allow(non_upper_case_globals)]
impl RootDescriptorFlags {
    pub const None: Self = Self(D3D12_ROOT_DESCRIPTOR_FLAG_NONE);
    pub const Volatile: Self = Self(D3D12_ROOT_DESCRIPTOR_FLAG_DATA_VOLATILE);
    pub const WhileSetAtExecute: Self =
        Self(D3D12_ROOT_DESCRIPTOR_FLAG_DATA_STATIC_WHILE_SET_AT_EXECUTE);
    pub const DataStatic: Self = Self(D3D12_ROOT_DESCRIPTOR_FLAG_DATA_STATIC);
}
impl_bitflag_operators!(RootDescriptorFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RootParameterType {
    DescriptorTable = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE,
    _32BitConstants = D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS,
    CBV = D3D12_ROOT_PARAMETER_TYPE_CBV,
    SRV = D3D12_ROOT_PARAMETER_TYPE_SRV,
    UAV = D3D12_ROOT_PARAMETER_TYPE_UAV,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct RootSignatureFlags(u32);
#[allow(non_upper_case_globals)]
impl RootSignatureFlags {
    pub const None: Self = Self(D3D12_ROOT_SIGNATURE_FLAG_NONE);
    pub const AllowInputAssemblerInputLayout: Self =
        Self(D3D12_ROOT_SIGNATURE_FLAG_ALLOW_INPUT_ASSEMBLER_INPUT_LAYOUT);
    pub const DenyVertexShaderRootAccess: Self =
        Self(D3D12_ROOT_SIGNATURE_FLAG_DENY_VERTEX_SHADER_ROOT_ACCESS);
    pub const DenyHullShaderRootAccess: Self =
        Self(D3D12_ROOT_SIGNATURE_FLAG_DENY_HULL_SHADER_ROOT_ACCESS);
    pub const DenyDomainShaderRootAccess: Self =
        Self(D3D12_ROOT_SIGNATURE_FLAG_DENY_DOMAIN_SHADER_ROOT_ACCESS);
    pub const DenyGeometryShaderRootAccess: Self =
        Self(D3D12_ROOT_SIGNATURE_FLAG_DENY_GEOMETRY_SHADER_ROOT_ACCESS);
    pub const DenyPixelShaderRootAccess: Self =
        Self(D3D12_ROOT_SIGNATURE_FLAG_DENY_PIXEL_SHADER_ROOT_ACCESS);
    pub const AllowStreamOutput: Self = Self(D3D12_ROOT_SIGNATURE_FLAG_ALLOW_STREAM_OUTPUT);
    // pub const LocalRootSignature: Self = Self(D3D12_ROOT_SIGNATURE_FLAG_LOCAL_ROOT_SIGNATURE);
}
impl_bitflag_operators!(RootSignatureFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RTVDimension {
    Unkown = D3D12_RTV_DIMENSION_UNKNOWN,
    Buffer = D3D12_RTV_DIMENSION_BUFFER,
    Texture1D = D3D12_RTV_DIMENSION_TEXTURE1D,
    Texture1DArray = D3D12_RTV_DIMENSION_TEXTURE1DARRAY,
    Texture2D = D3D12_RTV_DIMENSION_TEXTURE2D,
    Texture2DArray = D3D12_RTV_DIMENSION_TEXTURE2DARRAY,
    Texture2DMS = D3D12_RTV_DIMENSION_TEXTURE2DMS,
    Texture2DMSArray = D3D12_RTV_DIMENSION_TEXTURE2DMSARRAY,
    Texture3D = D3D12_RTV_DIMENSION_TEXTURE3D,
}
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SerializedDataType {
    RaytracingAccelerationStructure = D3D12_SERIALIZED_DATA_RAYTRACING_ACCELERATION_STRUCTURE,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ShaderCacheSupportFlags(u32);
#[allow(non_upper_case_globals)]
impl ShaderCacheSupportFlags {
    pub const None: Self = Self(D3D12_SHADER_CACHE_SUPPORT_NONE);
    pub const SinglePSO: Self = Self(D3D12_SHADER_CACHE_SUPPORT_SINGLE_PSO);
    pub const Library: Self = Self(D3D12_SHADER_CACHE_SUPPORT_LIBRARY);
    pub const AutomaticInprocCache: Self = Self(D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE);
    pub const AutomaticDiskCache: Self = Self(D3D12_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE);
}
impl_bitflag_operators!(ShaderCacheSupportFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ShaderComponentMapping(u32);
#[allow(non_upper_case_globals)]
impl ShaderComponentMapping {
    pub const FromMemoryComponent0: Self =
        Self(D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_0);
    pub const FromMemoryComponent1: Self =
        Self(D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_1);
    pub const FromMemoryComponent2: Self =
        Self(D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_2);
    pub const FromMemoryComponent3: Self =
        Self(D3D12_SHADER_COMPONENT_MAPPING_FROM_MEMORY_COMPONENT_3);
    pub const ForceValue0: Self = Self(D3D12_SHADER_COMPONENT_MAPPING_FORCE_VALUE_0);
    pub const ForceValue1: Self = Self(D3D12_SHADER_COMPONENT_MAPPING_FORCE_VALUE_1);

    const MASK: u32 = 0x7;
    const SHIFT: u32 = 3;
    const ALWAYS_SET_BIT_AVOIDING_ZEROMEM_MISTAKES: u32 = 1 << (Self::SHIFT * 4);

    pub const fn encode(src0: Self, src1: Self, src2: Self, src3: Self) -> Self {
        Self(
            (src0.0 & Self::MASK)
                | (src1.0 & Self::MASK) << Self::SHIFT
                | (src2.0 & Self::MASK) << (Self::SHIFT * 2)
                | (src3.0 & Self::MASK) << (Self::SHIFT * 3)
                | Self::ALWAYS_SET_BIT_AVOIDING_ZEROMEM_MISTAKES,
        )
    }

    pub const DEFAULT: Self = Self::encode(Self(0), Self(1), Self(2), Self(3));
}
impl Default for ShaderComponentMapping {
    fn default() -> Self {
        Self::DEFAULT
    }
}
pub const DEFAULT_4_COMPONENT_MAPPING: ShaderComponentMapping = ShaderComponentMapping::DEFAULT;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ShaderMinPrecisionSupport {
    None = D3D12_SHADER_MIN_PRECISION_SUPPORT_NONE,
    _10Bit = D3D12_SHADER_MIN_PRECISION_SUPPORT_10_BIT,
    _16Bit = D3D12_SHADER_MIN_PRECISION_SUPPORT_16_BIT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ShaderVisibility {
    All = D3D12_SHADER_VISIBILITY_ALL,
    Vertex = D3D12_SHADER_VISIBILITY_VERTEX,
    Hull = D3D12_SHADER_VISIBILITY_HULL,
    Domain = D3D12_SHADER_VISIBILITY_DOMAIN,
    Geometry = D3D12_SHADER_VISIBILITY_GEOMETRY,
    Pixel = D3D12_SHADER_VISIBILITY_PIXEL,
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ShadingRate {
    Rate1x1 = D3D12_SHADING_RATE_1X1,
    Rate1x2 = D3D12_SHADING_RATE_1X2,
    Rate2x1 = D3D12_SHADING_RATE_2X1,
    Rate2x2 = D3D12_SHADING_RATE_2X2,
    Rate2x4 = D3D12_SHADING_RATE_2X4,
    Rate4x2 = D3D12_SHADING_RATE_4X2,
    Rate4x4 = D3D12_SHADING_RATE_4X4,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ShadingRateCombiner {
    Passthrough = D3D12_SHADING_RATE_COMBINER_PASSTHROUGH,
    Override = D3D12_SHADING_RATE_COMBINER_OVERRIDE,
    Min = D3D12_SHADING_RATE_COMBINER_MIN,
    Max = D3D12_SHADING_RATE_COMBINER_MAX,
    Sum = D3D12_SHADING_RATE_COMBINER_SUM,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SharedResourceCompatibilityTier {
    Tier0 = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_0,
    Tier1 = D3D12_SHARED_RESOURCE_COMPATIBILITY_TIER_1,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SRVDimension {
    Unknown = D3D12_SRV_DIMENSION_UNKNOWN,
    Buffer = D3D12_SRV_DIMENSION_BUFFER,
    Texture1D = D3D12_SRV_DIMENSION_TEXTURE1D,
    Texture1DArray = D3D12_SRV_DIMENSION_TEXTURE1DARRAY,
    Texture2D = D3D12_SRV_DIMENSION_TEXTURE2D,
    Texture2DArray = D3D12_SRV_DIMENSION_TEXTURE2DARRAY,
    Texture2DMS = D3D12_SRV_DIMENSION_TEXTURE2DMS,
    Texture2DMSArray = D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY,
    Texture3D = D3D12_SRV_DIMENSION_TEXTURE3D,
    TextureCube = D3D12_SRV_DIMENSION_TEXTURECUBE,
    TextureCubeArray = D3D12_SRV_DIMENSION_TEXTURECUBEARRAY,
    // RaytracingAccelerationStructure = D3D12_SRV_DIMENSION_RAYTRACING_ACCELERATION_STRUCTURE,
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct StateObjectFlags(u32);
#[allow(non_upper_case_globals)]
impl StateObjectFlags {
    pub const None: Self = Self(D3D12_STATE_OBJECT_FLAG_NONE);
    pub const AllowLocalDependenciesOnExternalDefinitions: Self = Self(D3D12_STATE_OBJECT_FLAG_ALLOW_LOCAL_DEPENDENCIES_ON_EXTERNAL_DEFINITIONS);
    pub const AllowExternalDependenciesOnLocalDefinitions: Self = Self(D3D12_STATE_OBJECT_FLAG_ALLOW_EXTERNAL_DEPENDENCIES_ON_LOCAL_DEFINITIONS);
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StateObjectType {
    Collection = D3D12_STATE_OBJECT_TYPE_COLLECTION,
    RaytracingPipeline = D3D12_STATE_OBJECT_TYPE_RAYTRACING_PIPELINE
}
*/

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StateSubobjectType {
    StateObjectConfig = D3D12_STATE_SUBOBJECT_TYPE_STATE_OBJECT_CONFIG,
    GlobalRootSignature = D3D12_STATE_SUBOBJECT_TYPE_GLOBAL_ROOT_SIGNATURE,
    LocalRootSignature = D3D12_STATE_SUBOBJECT_TYPE_LOCAL_ROOT_SIGNATURE,
    NodeMask = D3D12_STATE_SUBOBJECT_TYPE_NODE_MASK,
    DXILLibrary = D3D12_STATE_SUBOBJECT_TYPE_DXIL_LIBRARY,
    ExistingCollection = D3D12_STATE_SUBOBJECT_TYPE_EXISTING_COLLECTION,
    SubobjectToExportsAssociation = D3D12_STATE_SUBOBJECT_TYPE_SUBOBJECT_TO_EXPORTS_ASSOCIATION,
    DXILSubobjectToExportsAssociation = D3D12_STATE_SUBOBJECT_TYPE_DXIL_SUBOBJECT_TO_EXPORTS_ASSOCIATION,
    RaytracingShaderConfig = D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_SHADER_CONFIG,
    RaytracingPipelineConfig = D3D12_STATE_SUBOBJECT_TYPE_RAYTRACING_PIPELINE_CONFIG,
    HitGroup = D3D12_STATE_SUBOBJECT_TYPE_HIT_GROUP,
    MaxVaild = D3D12_STATE_SUBOBJECT_TYPE_MAX_VALID,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StaticBorderColor {
    TransparentBlack = D3D12_STATIC_BORDER_COLOR_TRANSPARENT_BLACK,
    OpaqueBlack = D3D12_STATIC_BORDER_COLOR_OPAQUE_BLACK,
    OpaqueWhite = D3D12_STATIC_BORDER_COLOR_OPAQUE_WHITE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StencilOp {
    Keep = D3D12_STENCIL_OP_KEEP,
    Zero = D3D12_STENCIL_OP_ZERO,
    Replace = D3D12_STENCIL_OP_REPLACE,
    IncrSat = D3D12_STENCIL_OP_INCR_SAT,
    DecrSat = D3D12_STENCIL_OP_DECR_SAT,
    Invert = D3D12_STENCIL_OP_INVERT,
    Incr = D3D12_STENCIL_OP_INCR,
    Decr = D3D12_STENCIL_OP_DECR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextureAddressMode {
    Wrap = D3D12_TEXTURE_ADDRESS_MODE_WRAP,
    Mirror = D3D12_TEXTURE_ADDRESS_MODE_MIRROR,
    Clamp = D3D12_TEXTURE_ADDRESS_MODE_CLAMP,
    Boader = D3D12_TEXTURE_ADDRESS_MODE_BORDER,
    MirrorOnce = D3D12_TEXTURE_ADDRESS_MODE_MIRROR_ONCE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextureCopyType {
    SubresourceIndex = D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX,
    PlacedFootprint = D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextureLayout {
    Unknown = D3D12_TEXTURE_LAYOUT_UNKNOWN,
    RowMajor = D3D12_TEXTURE_LAYOUT_ROW_MAJOR,
    _64KBUndefinedSwizzle = D3D12_TEXTURE_LAYOUT_64KB_UNDEFINED_SWIZZLE,
    _64KBStandardSwizzle = D3D12_TEXTURE_LAYOUT_64KB_STANDARD_SWIZZLE,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct TileCopyFlags(u32);
#[allow(non_upper_case_globals)]
impl TileCopyFlags {
    pub const None: Self = Self(D3D12_TILE_COPY_FLAG_NONE);
    pub const NoHazard: Self = Self(D3D12_TILE_COPY_FLAG_NO_HAZARD);
    pub const LinearBufferToSwizzledTiledResource: Self =
        Self(D3D12_TILE_COPY_FLAG_LINEAR_BUFFER_TO_SWIZZLED_TILED_RESOURCE);
    pub const SwizzledTiledResourceToLinearBuffer: Self =
        Self(D3D12_TILE_COPY_FLAG_SWIZZLED_TILED_RESOURCE_TO_LINEAR_BUFFER);
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct TileMappingFlags(u32);
#[allow(non_upper_case_globals)]
impl TileMappingFlags {
    pub const None: Self = Self(D3D12_TILE_MAPPING_FLAG_NONE);
    pub const NoHazard: Self = Self(D3D12_TILE_MAPPING_FLAG_NO_HAZARD);
}
impl_bitflag_operators!(TileMappingFlags);

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct TileRangeFlags(u32);
#[allow(non_upper_case_globals)]
impl TileRangeFlags {
    pub const None: Self = Self(D3D12_TILE_RANGE_FLAG_NONE);
    pub const Null: Self = Self(D3D12_TILE_RANGE_FLAG_NULL);
    pub const Skip: Self = Self(D3D12_TILE_RANGE_FLAG_SKIP);
    pub const ReuseSingleTile: Self = Self(D3D12_TILE_RANGE_FLAG_REUSE_SINGLE_TILE);
}
impl_bitflag_operators!(TileRangeFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TiledResourcesTier {
    NotSupported = D3D12_TILED_RESOURCES_TIER_NOT_SUPPORTED,
    Tier1 = D3D12_TILED_RESOURCES_TIER_1,
    Tier2 = D3D12_TILED_RESOURCES_TIER_2,
    Tier3 = D3D12_TILED_RESOURCES_TIER_3,
    // Tier4 = D3D12_TILED_RESOURCES_TIER_4,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum UAVDimension {
    Unknown = D3D12_UAV_DIMENSION_UNKNOWN,
    Buffer = D3D12_UAV_DIMENSION_BUFFER,
    Texture1D = D3D12_UAV_DIMENSION_TEXTURE1D,
    Texture1DArray = D3D12_UAV_DIMENSION_TEXTURE1DARRAY,
    Texture2D = D3D12_UAV_DIMENSION_TEXTURE2D,
    Texture2DArray = D3D12_UAV_DIMENSION_TEXTURE2DARRAY,
    Texture3D = D3D12_UAV_DIMENSION_TEXTURE3D,
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VariableShadingRateTier {
    NotSupported = D3D12_VARIABLE_SHADING_RATE_TIER_NOT_SUPPORTED,
    Tier1 = D3D12_VARIABLE_SHADING_RATE_TIER_1,
    Tier2 = D3D12_VARIABLE_SHADING_RATE_TIER_2,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub struct ViewInstancingFlags(u32);
#[allow(non_upper_case_globals)]
impl ViewInstancingFlags {
    pub const None: Self = Self(D3D12_VIEW_INSTANCING_FLAG_NONE);
    pub const EnableViewInstanceMasking: Self = Self(D3D12_VIEW_INSTANCING_FLAG_ENABLE_VIEW_INSTANCE_MASKING);
}
impl_bitflag_operators!(ViewInstancingFlags);
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ViewInstancingTier {
    NotSupported = D3D12_VIEW_INSTANCING_TIER_NOT_SUPPORTED,
    Tier1 = D3D12_VIEW_INSTANCING_TIER_1,
    Tier2 = D3D12_VIEW_INSTANCING_TIER_2,
    Tier3 = D3D12_VIEW_INSTANCING_TIER_3,
}
*/
/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum WriteBufferImmediateMode {
    Default = D3D12_WRITEBUFFERIMMEDIATE_MODE_DEFAULT,
    MarkerIn = D3D12_WRITEBUFFERIMMEDIATE_MODE_MARKER_IN,
    MarkerOut = D3D12_WRITEBUFFERIMMEDIATE_MODE_MARKER_OUT,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, Debug)]
#[repr(C)]
pub struct GPUVirtualAddress(u64);
impl From<GPUVirtualAddress> for u64 {
    fn from(src: GPUVirtualAddress) -> u64 {
        src.0
    }
}
impl From<u64> for GPUVirtualAddress {
    fn from(src: u64) -> GPUVirtualAddress {
        GPUVirtualAddress(src)
    }
}

// pub struct AutoBreadcrumbNode;

#[derive(Clone, Debug)]
pub struct BlendDesc {
    pub alpha_to_coverage_enable: bool,
    pub independent_blend_enable: bool,
    pub render_target: Vec<RenderTargetBlendDesc>,
}
impl BlendDesc {
    fn to_c_struct(&self) -> D3D12_BLEND_DESC {
        assert!(self.render_target.len() <= 8);
        let mut render_target: [D3D12_RENDER_TARGET_BLEND_DESC; 8] = Default::default();
        for i in 0..self.render_target.len() {
            render_target[i] = self.render_target[i].to_c_struct();
        }
        for i in self.render_target.len()..8 {
            render_target[i] = RenderTargetBlendDesc::default().to_c_struct();
        }
        D3D12_BLEND_DESC {
            AlphaToCoverageEnable: to_BOOL(self.alpha_to_coverage_enable),
            IndependentBlendEnable: to_BOOL(self.independent_blend_enable),
            RenderTarget: render_target,
        }
    }
}
impl Default for BlendDesc {
    fn default() -> Self {
        Self {
            alpha_to_coverage_enable: false,
            independent_blend_enable: false,
            render_target: vec![Default::default()],
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
#[repr(C)]
pub struct Box3D {
    pub left: u32,
    pub top: u32,
    pub front: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}

// pub struct BuildRaytracingAccelerationStructureDesc;
// pub struct BuildRaytracingAccelerationStructureInputs;
// pub struct BuildRaytracingAccelerationStructureToolsVisualizationHeader;

#[derive(Clone, Default, Debug)]
pub struct CachedPipelineState {
    pub cached_blob: Vec<u8>,
}
impl CachedPipelineState {
    fn to_c_struct(&self) -> D3D12_CACHED_PIPELINE_STATE {
        D3D12_CACHED_PIPELINE_STATE {
            pCachedBlob: if self.cached_blob.is_empty() {
                std::ptr::null()
            } else {
                self.cached_blob.as_ptr() as *const c_void
            },
            CachedBlobSizeInBytes: self.cached_blob.len(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum ClearValue {
    Color(dxgi::Format, dxgi::RGBA),
    DepthStencil(dxgi::Format, DepthStencilValue),
}
impl ClearValue {
    fn to_c_struct(&self) -> D3D12_CLEAR_VALUE {
        let mut obj = D3D12_CLEAR_VALUE::default();
        match self {
            &ClearValue::Color(format, value) => unsafe {
                obj.Format = format as u32;
                obj.u.Color_mut()[0] = value.r;
                obj.u.Color_mut()[1] = value.g;
                obj.u.Color_mut()[2] = value.b;
                obj.u.Color_mut()[3] = value.a;
            },
            &ClearValue::DepthStencil(format, value) => unsafe {
                obj.Format = format as u32;
                *obj.u.DepthStencil_mut() = value.to_c_struct();
            },
        }
        obj
    }
}

#[derive(Clone, Debug)]
pub struct CommandQueueDesc<T> {
    pub ty: T,
    pub priority: CommandQueuePriority,
    pub flags: Option<CommandQueueFlags>,
    pub node_mask: u32,
}
impl CommandQueueDesc<()> {
    pub fn new() -> Self {
        Self {
            ty: (),
            priority: CommandQueuePriority::Normal,
            flags: None,
            node_mask: 0,
        }
    }
}
impl<T> CommandQueueDesc<T> {
    pub fn list_type(self, ty: CommandListType) -> CommandQueueDesc<CommandListType> {
        CommandQueueDesc {
            ty,
            priority: self.priority,
            flags: self.flags,
            node_mask: self.node_mask,
        }
    }
    pub fn priority(mut self, priority: CommandQueuePriority) -> Self {
        self.priority = priority;
        self
    }
    pub fn flags(mut self, flags: CommandQueueFlags) -> Self {
        self.flags = Some(flags);
        self
    }
    pub fn node_mask(mut self, node_mask: u32) -> Self {
        self.node_mask = node_mask;
        self
    }
}
impl CommandQueueDesc<CommandListType> {
    fn to_c_struct(&self) -> D3D12_COMMAND_QUEUE_DESC {
        D3D12_COMMAND_QUEUE_DESC {
            Type: self.ty as u32,
            Priority: self.priority as i32,
            Flags: self.flags.unwrap_or(CommandQueueFlags::None).0,
            NodeMask: self.node_mask,
        }
    }
}
impl From<D3D12_COMMAND_QUEUE_DESC> for CommandQueueDesc<CommandListType> {
    fn from(src: D3D12_COMMAND_QUEUE_DESC) -> CommandQueueDesc<CommandListType> {
        unsafe {
            CommandQueueDesc {
                ty: std::mem::transmute(src.Type),
                priority: std::mem::transmute(src.Priority),
                flags: if src.Flags == 0 {
                    None
                } else {
                    Some(CommandQueueFlags(src.Flags))
                },
                node_mask: src.NodeMask,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct CommandSignatureDesc<'a> {
    pub byte_stride: u32,
    pub argument_descs: &'a [IndirectArgumentDesc],
    pub node_mask: u32,
}

#[derive(Clone, Debug)]
pub struct ComputePipelineStateDesc {
    pub root_signature: RootSignature,
    pub cs: ShaderBytecode,
    pub node_mask: u32,
    pub cached_pso: CachedPipelineState,
    pub flags: Option<PipelineStateFlags>,
}
impl ComputePipelineStateDesc {
    fn to_c_struct(&self) -> D3D12_COMPUTE_PIPELINE_STATE_DESC {
        D3D12_COMPUTE_PIPELINE_STATE_DESC {
            pRootSignature: self.root_signature.0.as_ptr(),
            CS: self.cs.to_c_struct(),
            NodeMask: self.node_mask,
            CachedPSO: self.cached_pso.to_c_struct(),
            Flags: self.flags.map_or(0, |f| f.0),
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConstantBufferViewDesc {
    pub buffer_location: GPUVirtualAddress,
    pub size_in_bytes: u32,
}
impl ConstantBufferViewDesc {
    fn to_c_struct(&self) -> D3D12_CONSTANT_BUFFER_VIEW_DESC {
        D3D12_CONSTANT_BUFFER_VIEW_DESC {
            BufferLocation: self.buffer_location.0,
            SizeInBytes: self.size_in_bytes,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct CPUDescriptorHandle {
    pub ptr: usize,
}
impl From<CPUDescriptorHandle> for D3D12_CPU_DESCRIPTOR_HANDLE {
    fn from(src: CPUDescriptorHandle) -> D3D12_CPU_DESCRIPTOR_HANDLE {
        D3D12_CPU_DESCRIPTOR_HANDLE { ptr: src.ptr }
    }
}
impl std::ops::Add<usize> for CPUDescriptorHandle {
    type Output = Self;
    fn add(self, rhs: usize) -> Self {
        Self {
            ptr: self.ptr + rhs,
        }
    }
}
impl std::ops::Sub<usize> for CPUDescriptorHandle {
    type Output = Self;
    fn sub(self, rhs: usize) -> Self {
        Self {
            ptr: self.ptr - rhs,
        }
    }
}
impl std::ops::AddAssign<usize> for CPUDescriptorHandle {
    fn add_assign(&mut self, rhs: usize) {
        self.ptr += rhs;
    }
}
impl std::ops::SubAssign<usize> for CPUDescriptorHandle {
    fn sub_assign(&mut self, rhs: usize) {
        self.ptr -= rhs;
    }
}

pub const DEFAULT_STENCIL_READ_MASK: u8 = 0xff;
pub const DEFAULT_STENCIL_WRITE_MASK: u8 = 0xff;

#[derive(Clone, Debug)]
pub struct DepthStencilDesc {
    pub depth_enable: bool,
    pub depth_write_mask: DepthWriteMask,
    pub depth_func: ComparisonFunc,
    pub stencil_enable: bool,
    pub stencil_read_mask: u8,
    pub stencil_write_mask: u8,
    pub front_face: DepthStencilOpDesc,
    pub back_face: DepthStencilOpDesc,
}
impl DepthStencilDesc {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn depth_enable(mut self, depth_enable: bool) -> Self {
        self.depth_enable = depth_enable;
        self
    }
    pub fn depth_write_mask(mut self, depth_write_mask: DepthWriteMask) -> Self {
        self.depth_write_mask = depth_write_mask;
        self
    }
    pub fn depth_func(mut self, depth_func: ComparisonFunc) -> Self {
        self.depth_func = depth_func;
        self
    }
    pub fn stencil_enable(mut self, stencil_enable: bool) -> Self {
        self.stencil_enable = stencil_enable;
        self
    }
    pub fn stencil_read_mask(mut self, stencil_read_mask: u8) -> Self {
        self.stencil_read_mask = stencil_read_mask;
        self
    }
    pub fn stencil_write_mask(mut self, stencil_write_mask: u8) -> Self {
        self.stencil_write_mask = stencil_write_mask;
        self
    }
    pub fn front_face(mut self, front_face: DepthStencilOpDesc) -> Self {
        self.front_face = front_face;
        self
    }
    pub fn back_face(mut self, back_face: DepthStencilOpDesc) -> Self {
        self.back_face = back_face;
        self
    }
    fn to_c_struct(&self) -> D3D12_DEPTH_STENCIL_DESC {
        D3D12_DEPTH_STENCIL_DESC {
            DepthEnable: to_BOOL(self.depth_enable),
            DepthWriteMask: self.depth_write_mask as u32,
            DepthFunc: self.depth_func as u32,
            StencilEnable: to_BOOL(self.stencil_enable),
            StencilReadMask: self.stencil_read_mask,
            StencilWriteMask: self.stencil_write_mask,
            FrontFace: self.front_face.to_c_struct(),
            BackFace: self.back_face.to_c_struct(),
        }
    }
}
impl Default for DepthStencilDesc {
    fn default() -> Self {
        Self {
            depth_enable: true,
            depth_write_mask: DepthWriteMask::All,
            depth_func: ComparisonFunc::Less,
            stencil_enable: false,
            stencil_read_mask: DEFAULT_STENCIL_READ_MASK,
            stencil_write_mask: DEFAULT_STENCIL_WRITE_MASK,
            front_face: Default::default(),
            back_face: Default::default(),
        }
    }
}

/*
#[derive(Clone, Debug)]
pub struct DepthStencilDesc1 {
    depth_enable: bool,
    depth_write_mask: DepthWriteMask,
    depth_func: ComparisonFunc,
    stencil_enable: bool,
    stencil_read_mask: u8,
    stencil_write_mask: u8,
    front_face: DepthStencilOpDesc,
    back_face: DepthStencilOpDesc,
    depth_bounds_test_enable: bool,
}
impl Default for DepthStencilDesc1 {
    fn default() -> Self {
        Self {
            depth_enable: true,
            depth_write_mask: DepthWriteMask::All,
            depth_func: ComparisonFunc::Less,
            stencil_enable: false,
            stencil_read_mask: DEFAULT_STENCIL_READ_MASK,
            stencil_write_mask: DEFAULT_STENCIL_WRITE_MASK,
            front_face: Default::default(),
            back_face: Default::default(),
            depth_bounds_test_enable: false,
        }
    }
}
impl DepthStencilDesc1 {
    fn to_c_struct(&self) -> D3D12_DEPTH_STENCIL_DESC1 {
        D3D12_DEPTH_STENCIL_DESC1 {
            DepthEnable: to_BOOL(self.depth_enable),
            DepthWriteMask: self.depth_write_mask as u32,
            DepthFunc: self.depth_func as u32,
            StencilEnable: to_BOOL(self.stencil_enable),
            StencilReadMask: self.stencil_read_mask,
            StencilWriteMask: self.stencil_write_mask,
            FrontFace: self.front_face.to_c_struct(),
            BackFace: self.back_face.to_c_struct(),
            DepthBoundsTestEnable: to_BOOL(self.depth_bounds_test_enable),
        }
    }
}
*/

#[derive(Clone, Copy, Debug)]
pub struct DepthStencilValue {
    pub depth: f32,
    pub stencil: u8,
}
impl DepthStencilValue {
    fn to_c_struct(&self) -> D3D12_DEPTH_STENCIL_VALUE {
        D3D12_DEPTH_STENCIL_VALUE {
            Depth: self.depth,
            Stencil: self.stencil,
        }
    }
}

#[derive(Clone, Debug)]
pub enum DepthStencilViewDesc {
    Texture1D {
        format: dxgi::Format,
        flags: Option<DSVFlags>,
        mip_slice: u32,
    },
    Texture1DArray {
        format: dxgi::Format,
        flags: Option<DSVFlags>,
        mip_slice: u32,
        first_array_slice: u32,
        array_size: u32,
    },
    Texture2D {
        format: dxgi::Format,
        flags: Option<DSVFlags>,
        mip_slice: u32,
    },
    Texture2DArray {
        format: dxgi::Format,
        flags: Option<DSVFlags>,
        mip_slice: u32,
        first_array_slice: u32,
        array_size: u32,
    },
    Texture2DMS {
        format: dxgi::Format,
        flags: Option<DSVFlags>,
    },
    Texture2DMSArray {
        format: dxgi::Format,
        flags: Option<DSVFlags>,
        first_array_slice: u32,
        array_size: u32,
    },
}
impl DepthStencilViewDesc {
    fn to_c_struct(&self) -> D3D12_DEPTH_STENCIL_VIEW_DESC {
        let mut obj = D3D12_DEPTH_STENCIL_VIEW_DESC::default();
        match self {
            &DepthStencilViewDesc::Texture1D {
                format,
                flags,
                mip_slice,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = DSVDimension::Texture1D as u32;
                obj.Flags = flags.unwrap_or(DSVFlags::None).0;
                obj.u.Texture1D_mut().MipSlice = mip_slice;
            },
            &DepthStencilViewDesc::Texture1DArray {
                format,
                flags,
                mip_slice,
                first_array_slice,
                array_size,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = DSVDimension::Texture1DArray as u32;
                obj.Flags = flags.unwrap_or(DSVFlags::None).0;
                obj.u.Texture1DArray_mut().MipSlice = mip_slice;
                obj.u.Texture1DArray_mut().FirstArraySlice = first_array_slice;
                obj.u.Texture1DArray_mut().ArraySize = array_size;
            },
            &DepthStencilViewDesc::Texture2D {
                format,
                flags,
                mip_slice,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = DSVDimension::Texture2D as u32;
                obj.Flags = flags.unwrap_or(DSVFlags::None).0;
                obj.u.Texture2D_mut().MipSlice = mip_slice;
            },
            &DepthStencilViewDesc::Texture2DArray {
                format,
                flags,
                mip_slice,
                first_array_slice,
                array_size,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = DSVDimension::Texture2DArray as u32;
                obj.Flags = flags.unwrap_or(DSVFlags::None).0;
                obj.u.Texture2DArray_mut().MipSlice = mip_slice;
                obj.u.Texture2DArray_mut().FirstArraySlice = first_array_slice;
                obj.u.Texture2DArray_mut().ArraySize = array_size;
            },
            &DepthStencilViewDesc::Texture2DMS { format, flags } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = DSVDimension::Texture2DMS as u32;
                obj.Flags = flags.unwrap_or(DSVFlags::None).0;
                obj.u.Texture2DMS_mut().UnusedField_NothingToDefine = 0;
            },
            &DepthStencilViewDesc::Texture2DMSArray {
                format,
                flags,
                first_array_slice,
                array_size,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = DSVDimension::Texture2DMSArray as u32;
                obj.Flags = flags.unwrap_or(DSVFlags::None).0;
                obj.u.Texture2DMSArray_mut().FirstArraySlice = first_array_slice;
                obj.u.Texture2DMSArray_mut().ArraySize = array_size;
            },
        }
        obj
    }
}

#[derive(Clone, Debug)]
pub struct DepthStencilOpDesc {
    pub stencil_fail_op: StencilOp,
    pub stencil_depth_fail_op: StencilOp,
    pub stencil_pass_op: StencilOp,
    pub stencil_func: ComparisonFunc,
}
impl DepthStencilOpDesc {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn stencil_fail_op(mut self, stencil_fail_op: StencilOp) -> Self {
        self.stencil_fail_op = stencil_fail_op;
        self
    }
    pub fn stencil_depth_fail_op(mut self, stencil_depth_fail_op: StencilOp) -> Self {
        self.stencil_depth_fail_op = stencil_depth_fail_op;
        self
    }
    pub fn stencil_pass_op(mut self, stencil_pass_op: StencilOp) -> Self {
        self.stencil_pass_op = stencil_pass_op;
        self
    }
    pub fn stencil_func(mut self, stencil_func: ComparisonFunc) -> Self {
        self.stencil_func = stencil_func;
        self
    }
    fn to_c_struct(&self) -> D3D12_DEPTH_STENCILOP_DESC {
        D3D12_DEPTH_STENCILOP_DESC {
            StencilFailOp: self.stencil_fail_op as u32,
            StencilDepthFailOp: self.stencil_depth_fail_op as u32,
            StencilPassOp: self.stencil_pass_op as u32,
            StencilFunc: self.stencil_func as u32,
        }
    }
}
impl Default for DepthStencilOpDesc {
    fn default() -> Self {
        Self {
            stencil_fail_op: StencilOp::Keep,
            stencil_depth_fail_op: StencilOp::Keep,
            stencil_pass_op: StencilOp::Keep,
            stencil_func: ComparisonFunc::Always,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DescriptorHeapDesc<Ty, Nm> {
    pub ty: Ty,
    pub num_descriptors: Nm,
    pub flags: Option<DescriptorHeapFlags>,
    pub node_mask: u32,
}
impl DescriptorHeapDesc<(), ()> {
    pub fn new() -> Self {
        Self {
            ty: (),
            num_descriptors: (),
            flags: None,
            node_mask: 0,
        }
    }
}
impl<Ty, Nm> DescriptorHeapDesc<Ty, Nm> {
    pub fn flags(mut self, flags: DescriptorHeapFlags) -> Self {
        self.flags = Some(flags);
        self
    }
    pub fn node_mask(mut self, node_mask: u32) -> Self {
        self.node_mask = node_mask;
        self
    }
}
impl<Nm> DescriptorHeapDesc<(), Nm> {
    pub fn heap_type(self, ty: DescriptorHeapType) -> DescriptorHeapDesc<DescriptorHeapType, Nm> {
        DescriptorHeapDesc {
            ty,
            num_descriptors: self.num_descriptors,
            flags: self.flags,
            node_mask: self.node_mask,
        }
    }
}
impl<Ty> DescriptorHeapDesc<Ty, ()> {
    pub fn num_descriptors(self, num_descriptors: u32) -> DescriptorHeapDesc<Ty, u32> {
        DescriptorHeapDesc {
            ty: self.ty,
            num_descriptors,
            flags: self.flags,
            node_mask: self.node_mask,
        }
    }
}
impl DescriptorHeapDesc<DescriptorHeapType, u32> {
    fn to_c_struct(&self) -> D3D12_DESCRIPTOR_HEAP_DESC {
        D3D12_DESCRIPTOR_HEAP_DESC {
            Type: self.ty as u32,
            NumDescriptors: self.num_descriptors,
            Flags: self.flags.unwrap_or(DescriptorHeapFlags::None).0,
            NodeMask: self.node_mask,
        }
    }
}
impl From<D3D12_DESCRIPTOR_HEAP_DESC> for DescriptorHeapDesc<DescriptorHeapType, u32> {
    fn from(src: D3D12_DESCRIPTOR_HEAP_DESC) -> DescriptorHeapDesc<DescriptorHeapType, u32> {
        DescriptorHeapDesc {
            ty: unsafe { std::mem::transmute(src.Type) },
            num_descriptors: src.NumDescriptors,
            flags: if src.Flags == 0 {
                None
            } else {
                Some(DescriptorHeapFlags(src.Flags))
            },
            node_mask: src.NodeMask,
        }
    }
}

pub const DESCRIPTOR_RANGE_OFFSET_APPEND: u32 = DescriptorRange::OFFSET_APPEND;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct DescriptorRange {
    pub range_type: DescriptorRangeType,
    pub num_descriptors: u32,
    pub base_shader_register: u32,
    pub register_space: u32,
    pub offset_in_descriptors_from_table_start: u32,
}
impl DescriptorRange {
    pub const OFFSET_APPEND: u32 = 0xffffffff;
}
impl From<D3D12_DESCRIPTOR_RANGE> for DescriptorRange {
    fn from(src: D3D12_DESCRIPTOR_RANGE) -> DescriptorRange {
        DescriptorRange {
            range_type: unsafe { std::mem::transmute(src.RangeType) },
            num_descriptors: src.NumDescriptors,
            base_shader_register: src.BaseShaderRegister,
            register_space: src.RegisterSpace,
            offset_in_descriptors_from_table_start: src.OffsetInDescriptorsFromTableStart,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DescriptorRange1 {
    pub range_type: DescriptorRangeType,
    pub num_descriptors: u32,
    pub base_shader_register: u32,
    pub register_space: u32,
    pub flags: Option<DescriptorRangeFlags>,
    pub offset_in_descriptors_from_table_start: u32,
}
impl DescriptorRange1 {
    fn to_c_struct(&self) -> D3D12_DESCRIPTOR_RANGE1 {
        D3D12_DESCRIPTOR_RANGE1 {
            RangeType: self.range_type as u32,
            NumDescriptors: self.num_descriptors,
            BaseShaderRegister: self.base_shader_register,
            RegisterSpace: self.register_space,
            Flags: self.flags.map_or(0, |f| f.0 as u32),
            OffsetInDescriptorsFromTableStart: self.offset_in_descriptors_from_table_start,
        }
    }
}
impl From<D3D12_DESCRIPTOR_RANGE1> for DescriptorRange1 {
    fn from(src: D3D12_DESCRIPTOR_RANGE1) -> DescriptorRange1 {
        DescriptorRange1 {
            range_type: unsafe { std::mem::transmute(src.RangeType) },
            num_descriptors: src.NumDescriptors,
            base_shader_register: src.BaseShaderRegister,
            register_space: src.RegisterSpace,
            flags: if src.Flags == 0 {
                None
            } else {
                Some(DescriptorRangeFlags(src.Flags))
            },
            offset_in_descriptors_from_table_start: src.OffsetInDescriptorsFromTableStart,
        }
    }
}

// pub struct DeviceRemovedExtendedData;
// pub struct DeviceRemovedExtendedData1;

#[derive(Clone, Debug)]
pub struct DiscardRegion<'a> {
    pub rects: &'a [Rect],
    pub first_subresource: u32,
    pub num_subresources: u32,
}
impl<'a> DiscardRegion<'a> {
    fn to_c_struct(&self) -> D3D12_DISCARD_REGION {
        D3D12_DISCARD_REGION {
            NumRects: self.rects.len() as u32,
            pRects: self.rects.as_ptr() as *const RECT,
            FirstSubresource: self.first_subresource,
            NumSubresources: self.num_subresources,
        }
    }
}

#[derive(Clone, Debug)]
pub struct DispatchArguments {
    pub thread_group_count_x: u32,
    pub thread_group_count_y: u32,
    pub thread_group_count_z: u32,
}

// pub struct DispatchRaysDesc;

#[derive(Clone, Debug)]
pub struct DrawArguments {
    pub vertex_count_per_instance: u32,
    pub instance_count: u32,
    pub start_vertex_location: u32,
    pub start_instance_location: u32,
}

#[derive(Clone, Debug)]
pub struct DrawIndexedArguments {
    pub index_count_per_instance: u32,
    pub instance_count: u32,
    pub start_index_location: u32,
    pub base_vertex_location: i32,
    pub start_instance_location: u32,
}

// pub struct DREDAllocationNode;
// pub struct DREDAutoBreadcrumbsOutput;
// pub struct DREDPageFaultOutput;
// pub struct DXILLibraryDesc;
// pub struct DXILSubobjectToExportsAssociation;
// pub struct ExistingCollectionDesc;
// pub struct ExportDesc;

pub trait CheckFeatureSupport
where
    Self: Sized,
{
    type Args;
    fn check_feature_support(device: *mut ID3D12Device, args: Self::Args) -> Result<Self, HResult>;
}

pub mod feature_data {
    use super::{
        CheckFeatureSupport, CommandListType, ConservativeRasterizationTier, CrossNodeSharingTier,
        FormatSupport1, FormatSupport2, MultipleFenceWaitFlags, ProgrammableSamplePositionsTier,
        ResourceBindingTier, ResourceHeapTier, ShaderCacheSupportFlags, ShaderMinPrecisionSupport,
        TiledResourcesTier,
    };
    use crate::result::{hresult, HResult};
    use crate::utility::*;
    use crate::{d3d, dxgi};
    use winapi::shared::minwindef::TRUE;
    use winapi::um::d3d12::*;
    use winapi::um::d3dcommon::D3D_FEATURE_LEVEL;

    #[derive(Clone, Debug)]
    pub struct Architecture {
        pub node_index: u32,
        pub tile_based_renderer: bool,
        pub uma: bool,
        pub cache_coherent_uma: bool,
    }
    impl From<D3D12_FEATURE_DATA_ARCHITECTURE> for Architecture {
        fn from(src: D3D12_FEATURE_DATA_ARCHITECTURE) -> Architecture {
            Architecture {
                node_index: src.NodeIndex,
                tile_based_renderer: src.TileBasedRenderer == TRUE,
                uma: src.UMA == TRUE,
                cache_coherent_uma: src.CacheCoherentUMA == TRUE,
            }
        }
    }
    impl CheckFeatureSupport for Architecture {
        type Args = u32;
        fn check_feature_support(
            device: *mut ID3D12Device,
            node_index: u32,
        ) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_ARCHITECTURE::default();
            obj.NodeIndex = node_index;
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_ARCHITECTURE,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Architecture1 {
        pub node_index: u32,
        pub tile_based_renderer: bool,
        pub uma: bool,
        pub cache_coherent_uma: bool,
        pub isolated_mmu: bool,
    }
    impl From<D3D12_FEATURE_DATA_ARCHITECTURE1> for Architecture1 {
        fn from(src: D3D12_FEATURE_DATA_ARCHITECTURE1) -> Architecture1 {
            Architecture1 {
                node_index: src.NodeIndex,
                tile_based_renderer: src.TileBasedRenderer == TRUE,
                uma: src.UMA == TRUE,
                cache_coherent_uma: src.CacheCoherentUMA == TRUE,
                isolated_mmu: src.IsolatedMMU == TRUE,
            }
        }
    }
    impl CheckFeatureSupport for Architecture1 {
        type Args = u32;
        fn check_feature_support(
            device: *mut ID3D12Device,
            node_index: u32,
        ) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_ARCHITECTURE1::default();
            obj.NodeIndex = node_index;
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_ARCHITECTURE1,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct CommandQueuePriority {
        pub command_list_type: CommandListType,
        pub priority: u32,
        pub priority_for_type_is_supported: bool,
    }
    impl From<D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY> for CommandQueuePriority {
        fn from(src: D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY) -> CommandQueuePriority {
            CommandQueuePriority {
                command_list_type: unsafe { std::mem::transmute(src.CommandListType) },
                priority: src.Priority,
                priority_for_type_is_supported: src.PriorityForTypeIsSupported == TRUE,
            }
        }
    }
    impl CheckFeatureSupport for CommandQueuePriority {
        type Args = (CommandListType, u32);
        fn check_feature_support(
            device: *mut ID3D12Device,
            (command_list_type, priority): Self::Args,
        ) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_COMMAND_QUEUE_PRIORITY::default();
            obj.CommandListType = command_list_type as u32;
            obj.Priority = priority;
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_COMMAND_QUEUE_PRIORITY,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    /*
    #[derive(Clone, Debug)]
    pub struct CrossNode {
        pub sharing_tier: CrossNodeSharingTier,
        pub atomic_shader_instructions: bool,
    }
    impl From<D3D12_FEATURE_DATA_CROSS_NODE> for CrossNode {
        fn from(src: D3D12_FEATURE_DATA_CROSS_NODE) -> CrossNode {
            CrossNode {
                sharing_tier: unsafe { std::mem::transmute(src.SharingTier) },
                atomic_shader_instructions: src.AtomicShaderInstructions == TRUE,
            }
        }
    }
    */

    #[derive(Clone, Debug)]
    pub struct D3D12Options {
        pub double_precision_float_shader_ops: bool,
        pub output_merger_logic_op: bool,
        pub min_precision_support: ShaderMinPrecisionSupport,
        pub tiled_resources_tier: TiledResourcesTier,
        pub resource_binding_tier: ResourceBindingTier,
        pub ps_specified_stencil_ref_supported: bool,
        pub typed_uav_load_additional_formats: bool,
        pub rovs_supported: bool,
        pub conservative_rasterization_tier: ConservativeRasterizationTier,
        pub max_gpu_virtual_address_bits_per_resource: u32,
        pub standard_swizzle_64kb_supported: bool,
        pub cross_node_sharing_tier: CrossNodeSharingTier,
        pub cross_adapter_row_major_texture_supported: bool,
        pub vp_and_rt_array_indesx_from_any_shader_feeding_rasterizer_supported_without_gs_emulation:
            bool,
        pub resource_heap_tier: ResourceHeapTier,
    }
    impl From<D3D12_FEATURE_DATA_D3D12_OPTIONS> for D3D12Options {
        fn from(src: D3D12_FEATURE_DATA_D3D12_OPTIONS) -> D3D12Options {
            unsafe {
                D3D12Options {
                    double_precision_float_shader_ops: src.DoublePrecisionFloatShaderOps == TRUE,
                    output_merger_logic_op: src.OutputMergerLogicOp == TRUE,
                    min_precision_support: std::mem::transmute(src.MinPrecisionSupport),
                    tiled_resources_tier: std::mem::transmute(src.TiledResourcesTier),
                    resource_binding_tier: std::mem::transmute(src.ResourceBindingTier),
                    ps_specified_stencil_ref_supported: src.PSSpecifiedStencilRefSupported == TRUE,
                    typed_uav_load_additional_formats: src.TypedUAVLoadAdditionalFormats == TRUE,
                    rovs_supported: src.ROVsSupported == TRUE,
                    conservative_rasterization_tier: std::mem::transmute(src.ConservativeRasterizationTier),
                    max_gpu_virtual_address_bits_per_resource: src.MaxGPUVirtualAddressBitsPerResource,
                    standard_swizzle_64kb_supported: src.StandardSwizzle64KBSupported == TRUE,
                    cross_node_sharing_tier: std::mem::transmute(src.CrossNodeSharingTier),
                    cross_adapter_row_major_texture_supported: src.CrossAdapterRowMajorTextureSupported == TRUE,
                    vp_and_rt_array_indesx_from_any_shader_feeding_rasterizer_supported_without_gs_emulation: src. VPAndRTArrayIndexFromAnyShaderFeedingRasterizerSupportedWithoutGSEmulation == TRUE,
                    resource_heap_tier: std::mem::transmute(src.ResourceHeapTier),
                }
            }
        }
    }
    impl CheckFeatureSupport for D3D12Options {
        type Args = ();
        fn check_feature_support(device: *mut ID3D12Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_D3D12_OPTIONS::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_D3D12_OPTIONS,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct D3D12Options1 {
        pub wave_ops: bool,
        pub wave_lane_count_min: u32,
        pub wave_lane_count_max: u32,
        pub total_lane_count: u32,
        pub expanded_compute_resource_states: bool,
        pub int64_shader_ops: bool,
    }
    impl From<D3D12_FEATURE_DATA_D3D12_OPTIONS1> for D3D12Options1 {
        fn from(src: D3D12_FEATURE_DATA_D3D12_OPTIONS1) -> D3D12Options1 {
            D3D12Options1 {
                wave_ops: src.WaveOps == TRUE,
                wave_lane_count_min: src.WaveLaneCountMin,
                wave_lane_count_max: src.WaveLaneCountMax,
                total_lane_count: src.TotalLaneCount,
                expanded_compute_resource_states: src.ExpandedComputeResourceStates == TRUE,
                int64_shader_ops: src.Int64ShaderOps == TRUE,
            }
        }
    }
    impl CheckFeatureSupport for D3D12Options1 {
        type Args = ();
        fn check_feature_support(device: *mut ID3D12Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_D3D12_OPTIONS1::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_D3D12_OPTIONS1,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct D3D12Options2 {
        pub depth_bounds_test_supported: bool,
        pub programmable_sample_positions_tier: ProgrammableSamplePositionsTier,
    }
    impl From<D3D12_FEATURE_DATA_D3D12_OPTIONS2> for D3D12Options2 {
        fn from(src: D3D12_FEATURE_DATA_D3D12_OPTIONS2) -> D3D12Options2 {
            unsafe {
                D3D12Options2 {
                    depth_bounds_test_supported: src.DepthBoundsTestSupported == TRUE,
                    programmable_sample_positions_tier: std::mem::transmute(
                        src.ProgrammableSamplePositionsTier,
                    ),
                }
            }
        }
    }
    impl CheckFeatureSupport for D3D12Options2 {
        type Args = ();
        fn check_feature_support(device: *mut ID3D12Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_D3D12_OPTIONS2::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_D3D12_OPTIONS2,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    /*
    #[derive(Clone, Debug)]
    pub struct D3D12Option3 {
        copy_queue_timestamp_queries_supported: bool,
        casting_fully_typed_format_supported: bool,
        write_buffer_immediate_support_flags: CommandListSupportFlags,
    }
    */
    // pub strut D3D12Option4;
    // pub strut D3D12Option5;
    // pub strut D3D12Option6;

    /*
    #[derive(Clone, Debug)]
    pub struct ExistingHeaps {
        pub supported: bool,
    }
    impl From<D3D12_FEATURE_DATA_EXISTING_HEAPS> for ExistingHeaps {
        fn from(src: D3D12_FEATURE_DATA_EXISTING_HEAPS) -> ExistingHeaps {
            ExistingHeaps {
                supported: src.Supported == TRUE,
            }
        }
    }
    */

    #[derive(Clone, Debug)]
    pub struct FeatureLevels {
        pub feature_levels_requested: Vec<d3d::FeatureLevel>,
        pub max_supported_feature_level: d3d::FeatureLevel,
    }
    impl CheckFeatureSupport for FeatureLevels {
        type Args = Vec<d3d::FeatureLevel>;
        fn check_feature_support(
            device: *mut ID3D12Device,
            feature_levels: Self::Args,
        ) -> Result<Self, HResult> {
            let fls = feature_levels
                .iter()
                .map(|&fl| D3D_FEATURE_LEVEL::from(fl))
                .collect::<Vec<_>>();
            let mut obj = D3D12_FEATURE_DATA_FEATURE_LEVELS::default();
            obj.NumFeatureLevels = fls.len() as u32;
            obj.pFeatureLevelsRequested = fls.as_ptr();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_FEATURE_LEVELS,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(
                FeatureLevels {
                    feature_levels_requested: feature_levels,
                    max_supported_feature_level: obj.MaxSupportedFeatureLevel.into(),
                },
                res,
            )
        }
    }

    #[derive(Clone, Debug)]
    pub struct FormatInfo {
        pub format: dxgi::Format,
        pub plane_count: u8,
    }
    impl From<D3D12_FEATURE_DATA_FORMAT_INFO> for FormatInfo {
        fn from(src: D3D12_FEATURE_DATA_FORMAT_INFO) -> FormatInfo {
            unsafe {
                FormatInfo {
                    format: std::mem::transmute(src.Format),
                    plane_count: src.PlaneCount,
                }
            }
        }
    }
    impl CheckFeatureSupport for FormatInfo {
        type Args = dxgi::Format;
        fn check_feature_support(
            device: *mut ID3D12Device,
            format: Self::Args,
        ) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_FORMAT_INFO::default();
            obj.Format = format as u32;
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_FORMAT_INFO,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct FormatSupport {
        pub format: dxgi::Format,
        pub support1: FormatSupport1,
        pub support2: FormatSupport2,
    }
    impl From<D3D12_FEATURE_DATA_FORMAT_SUPPORT> for FormatSupport {
        fn from(src: D3D12_FEATURE_DATA_FORMAT_SUPPORT) -> FormatSupport {
            unsafe {
                FormatSupport {
                    format: std::mem::transmute(src.Format),
                    support1: std::mem::transmute(src.Support1),
                    support2: std::mem::transmute(src.Support2),
                }
            }
        }
    }
    impl CheckFeatureSupport for FormatSupport {
        type Args = dxgi::Format;
        fn check_feature_support(
            device: *mut ID3D12Device,
            format: Self::Args,
        ) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_FORMAT_SUPPORT::default();
            obj.Format = format as u32;
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_FORMAT_SUPPORT,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct GPUVirtualAddressSupport {
        pub max_gpu_virtual_address_bits_per_resource: u32,
        pub max_gpu_virtual_address_bits_per_process: u32,
    }
    impl From<D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT> for GPUVirtualAddressSupport {
        fn from(src: D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT) -> GPUVirtualAddressSupport {
            GPUVirtualAddressSupport {
                max_gpu_virtual_address_bits_per_resource: src.MaxGPUVirtualAddressBitsPerResource,
                max_gpu_virtual_address_bits_per_process: src.MaxGPUVirtualAddressBitsPerProcess,
            }
        }
    }
    impl CheckFeatureSupport for GPUVirtualAddressSupport {
        type Args = ();
        fn check_feature_support(device: *mut ID3D12Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct MultisampleQualityLevels {
        pub format: dxgi::Format,
        pub sample_count: u32,
        pub flags: Option<MultipleFenceWaitFlags>,
        pub num_quality_levels: u32,
    }
    impl From<D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS> for MultisampleQualityLevels {
        fn from(src: D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS) -> MultisampleQualityLevels {
            unsafe {
                MultisampleQualityLevels {
                    format: std::mem::transmute(src.Format),
                    sample_count: src.SampleCount,
                    flags: if src.Flags == 0 {
                        None
                    } else {
                        Some(MultipleFenceWaitFlags(src.Flags))
                    },
                    num_quality_levels: src.NumQualityLevels,
                }
            }
        }
    }
    impl CheckFeatureSupport for MultisampleQualityLevels {
        type Args = (dxgi::Format, u32, Option<MultipleFenceWaitFlags>);
        fn check_feature_support(
            device: *mut ID3D12Device,
            (format, sample_count, flags): Self::Args,
        ) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_MULTISAMPLE_QUALITY_LEVELS::default();
            obj.Format = format as u32;
            obj.SampleCount = sample_count;
            obj.Flags = flags.map_or(0, |f| f.0);
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_MULTISAMPLE_QUALITY_LEVELS,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    // pub struct ProtectedResourceSessionSupport;

    #[derive(Clone, Debug)]
    pub struct RootSignature {
        pub highest_version: d3d::RootSignatureVersion,
    }
    impl From<D3D12_FEATURE_DATA_ROOT_SIGNATURE> for RootSignature {
        fn from(src: D3D12_FEATURE_DATA_ROOT_SIGNATURE) -> RootSignature {
            RootSignature {
                highest_version: src.HighestVersion.into(),
            }
        }
    }
    impl CheckFeatureSupport for RootSignature {
        type Args = d3d::RootSignatureVersion;
        fn check_feature_support(
            device: *mut ID3D12Device,
            version: Self::Args,
        ) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_ROOT_SIGNATURE::default();
            obj.HighestVersion = version.into();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_ROOT_SIGNATURE,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    /*
    pub struct Serialization {
        pub node_index: u32,
        pub heap_serialization_tier: HeapSerializationTier,
    }
    */

    #[derive(Clone, Debug)]
    pub struct ShaderCache {
        pub support_flags: ShaderCacheSupportFlags,
    }
    impl From<D3D12_FEATURE_DATA_SHADER_CACHE> for ShaderCache {
        fn from(src: D3D12_FEATURE_DATA_SHADER_CACHE) -> ShaderCache {
            ShaderCache {
                support_flags: ShaderCacheSupportFlags(src.SupportFlags),
            }
        }
    }
    impl CheckFeatureSupport for ShaderCache {
        type Args = ();
        fn check_feature_support(
            device: *mut ID3D12Device,
            _args: Self::Args,
        ) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_SHADER_CACHE::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_SHADER_CACHE,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct ShaderModel {
        pub highest_shader_model: d3d::ShaderModel,
    }
    impl From<D3D12_FEATURE_DATA_SHADER_MODEL> for ShaderModel {
        fn from(src: D3D12_FEATURE_DATA_SHADER_MODEL) -> ShaderModel {
            ShaderModel {
                highest_shader_model: src.HighestShaderModel.into(),
            }
        }
    }
    impl CheckFeatureSupport for ShaderModel {
        type Args = d3d::ShaderModel;
        fn check_feature_support(
            device: *mut ID3D12Device,
            shader_model: Self::Args,
        ) -> Result<Self, HResult> {
            let mut obj = D3D12_FEATURE_DATA_SHADER_MODEL::default();
            obj.HighestShaderModel = shader_model.into();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D12_FEATURE_SHADER_MODEL,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }
}

#[derive(Clone, Debug)]
pub struct GlobalRootSignature {
    pub global_root_signature: RootSignature,
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct GPUDescriptorHandle {
    pub ptr: u64,
}
impl From<GPUDescriptorHandle> for D3D12_GPU_DESCRIPTOR_HANDLE {
    fn from(src: GPUDescriptorHandle) -> D3D12_GPU_DESCRIPTOR_HANDLE {
        D3D12_GPU_DESCRIPTOR_HANDLE { ptr: src.ptr }
    }
}
impl std::ops::Add<u64> for GPUDescriptorHandle {
    type Output = Self;
    fn add(self, rhs: u64) -> Self {
        Self {
            ptr: self.ptr + rhs,
        }
    }
}
impl std::ops::Sub<u64> for GPUDescriptorHandle {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self {
        Self {
            ptr: self.ptr - rhs,
        }
    }
}
impl std::ops::AddAssign<u64> for GPUDescriptorHandle {
    fn add_assign(&mut self, rhs: u64) {
        self.ptr += rhs;
    }
}
impl std::ops::SubAssign<u64> for GPUDescriptorHandle {
    fn sub_assign(&mut self, rhs: u64) {
        self.ptr -= rhs;
    }
}

/*
#[derive(Clone, Debug)]
pub struct GPUVirtualAddressAndStride {
    start_address: GPUVirtualAddress,
    stride_in_bytes: u64,
}
impl From<GPUVirtualAddressAndStride> for D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
    fn from(src: GPUVirtualAddressAndStride) -> D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
        D3D12_GPU_VIRTUAL_ADDRESS_AND_STRIDE {
            StartAddress: src.start_address,
            StrideInBytes: src.stride_in_bytes,
        }
    }
}
*/
/*
#[derive(Clone, Debug)]
pub struct GPUVirtualAddressRange {
    start_address: GPUVirtualAddress,
    size_in_bytes: u64,
}
impl From<GPUVirtualAddressRange> for D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
    fn from(src: GPUVirtualAddressRange) -> D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
        D3D12_GPU_VIRTUAL_ADDRESS_RANGE {
            StartAddress: src.start_address.0,
            SizeInBytes: src.size_in_bytes,
        }
    }
}
*/

// pub struct GPUVirtualAddressRangeAndStride;

#[derive(Clone, Debug)]
pub struct GraphicsPipelineStateDesc<'a, Il, Rf, Df> {
    pub root_signature: Option<RootSignature>,
    pub vs: Option<ShaderBytecode>,
    pub ps: Option<ShaderBytecode>,
    pub ds: Option<ShaderBytecode>,
    pub hs: Option<ShaderBytecode>,
    pub gs: Option<ShaderBytecode>,
    pub stream_output: Option<StreamOutputDesc<'a>>,
    pub blend_state: BlendDesc,
    pub sample_mask: u32,
    pub rasterizer_state: RasterizerDesc,
    pub depth_stencil_state: DepthStencilDesc,
    pub input_layout: Il,
    pub ib_strip_cut_value: IndexBufferStripCutValue,
    pub primitive_topology_type: PrimitiveTopologyType,
    pub rtv_formats: Rf,
    pub dsv_format: Df,
    pub sample_desc: dxgi::SampleDesc,
    pub node_mask: u32,
    pub cached_pso: Option<CachedPipelineState>,
    pub flags: Option<PipelineStateFlags>,
}
impl<'a> GraphicsPipelineStateDesc<'a, (), (), ()> {
    pub fn new() -> Self {
        Self {
            root_signature: None,
            vs: None,
            ps: None,
            ds: None,
            hs: None,
            gs: None,
            stream_output: None,
            blend_state: Default::default(),
            sample_mask: std::u32::MAX,
            rasterizer_state: Default::default(),
            depth_stencil_state: Default::default(),
            input_layout: (),
            ib_strip_cut_value: Default::default(),
            primitive_topology_type: PrimitiveTopologyType::Triangle,
            rtv_formats: (),
            dsv_format: (),
            sample_desc: Default::default(),
            node_mask: 0,
            cached_pso: None,
            flags: None,
        }
    }
}
impl<'a, Il, Rf, Df> GraphicsPipelineStateDesc<'a, Il, Rf, Df> {
    pub fn root_signature(mut self, root_signature: &RootSignature) -> Self {
        self.root_signature = Some(root_signature.clone());
        self
    }
    pub fn vs(mut self, code: ShaderBytecode) -> Self {
        self.vs = Some(code);
        self
    }
    pub fn ps(mut self, code: ShaderBytecode) -> Self {
        self.ps = Some(code);
        self
    }
    pub fn ds(mut self, code: ShaderBytecode) -> Self {
        self.ds = Some(code);
        self
    }
    pub fn hs(mut self, code: ShaderBytecode) -> Self {
        self.hs = Some(code);
        self
    }
    pub fn gs(mut self, code: ShaderBytecode) -> Self {
        self.gs = Some(code);
        self
    }
    pub fn stream_output(mut self, so: StreamOutputDesc<'a>) -> Self {
        self.stream_output = Some(so);
        self
    }
    pub fn blend_desc(mut self, desc: BlendDesc) -> Self {
        self.blend_state = desc;
        self
    }
    pub fn sample_mask(mut self, mask: u32) -> Self {
        self.sample_mask = mask;
        self
    }
    pub fn rasterizer_state(mut self, desc: RasterizerDesc) -> Self {
        self.rasterizer_state = desc;
        self
    }
    pub fn depth_stencil_state(mut self, desc: DepthStencilDesc) -> Self {
        self.depth_stencil_state = desc;
        self
    }
    pub fn ib_strip_cut_value(mut self, value: IndexBufferStripCutValue) -> Self {
        self.ib_strip_cut_value = value;
        self
    }
    pub fn primitive_topology_type(mut self, topology_type: PrimitiveTopologyType) -> Self {
        self.primitive_topology_type = topology_type;
        self
    }
    pub fn sample_desc(mut self, sample_desc: dxgi::SampleDesc) -> Self {
        self.sample_desc = sample_desc;
        self
    }
    pub fn node_mask(mut self, node_mask: u32) -> Self {
        self.node_mask = node_mask;
        self
    }
    pub fn cached_pso(mut self, cached_pso: CachedPipelineState) -> Self {
        self.cached_pso = Some(cached_pso);
        self
    }
    pub fn flags(mut self, flags: PipelineStateFlags) -> Self {
        self.flags = Some(flags);
        self
    }
}
impl<'a, Rf, Df> GraphicsPipelineStateDesc<'a, (), Rf, Df> {
    pub fn input_layout<'b>(
        self,
        input_layout: InputLayoutDesc<'b>,
    ) -> GraphicsPipelineStateDesc<'a, InputLayoutDesc<'b>, Rf, Df> {
        GraphicsPipelineStateDesc {
            root_signature: self.root_signature,
            vs: self.vs,
            ps: self.ps,
            ds: self.ds,
            hs: self.hs,
            gs: self.gs,
            stream_output: self.stream_output,
            blend_state: self.blend_state,
            sample_mask: self.sample_mask,
            rasterizer_state: self.rasterizer_state,
            depth_stencil_state: self.depth_stencil_state,
            input_layout,
            ib_strip_cut_value: self.ib_strip_cut_value,
            primitive_topology_type: self.primitive_topology_type,
            rtv_formats: self.rtv_formats,
            dsv_format: self.dsv_format,
            sample_desc: self.sample_desc,
            node_mask: self.node_mask,
            cached_pso: self.cached_pso,
            flags: self.flags,
        }
    }
}
impl<'a, Il, Df> GraphicsPipelineStateDesc<'a, Il, (), Df> {
    pub fn rtv_formats<'b>(
        self,
        rtv_formats: &'b [dxgi::Format],
    ) -> GraphicsPipelineStateDesc<'a, Il, &'b [dxgi::Format], Df> {
        GraphicsPipelineStateDesc {
            root_signature: self.root_signature,
            vs: self.vs,
            ps: self.ps,
            ds: self.ds,
            hs: self.hs,
            gs: self.gs,
            stream_output: self.stream_output,
            blend_state: self.blend_state,
            sample_mask: self.sample_mask,
            rasterizer_state: self.rasterizer_state,
            depth_stencil_state: self.depth_stencil_state,
            input_layout: self.input_layout,
            ib_strip_cut_value: self.ib_strip_cut_value,
            primitive_topology_type: self.primitive_topology_type,
            rtv_formats,
            dsv_format: self.dsv_format,
            sample_desc: self.sample_desc,
            node_mask: self.node_mask,
            cached_pso: self.cached_pso,
            flags: self.flags,
        }
    }
}
impl<'a, Il, Rf> GraphicsPipelineStateDesc<'a, Il, Rf, ()> {
    pub fn dsv_format(
        self,
        dsv_format: dxgi::Format,
    ) -> GraphicsPipelineStateDesc<'a, Il, Rf, dxgi::Format> {
        GraphicsPipelineStateDesc {
            root_signature: self.root_signature,
            vs: self.vs,
            ps: self.ps,
            ds: self.ds,
            hs: self.hs,
            gs: self.gs,
            stream_output: self.stream_output,
            blend_state: self.blend_state,
            sample_mask: self.sample_mask,
            rasterizer_state: self.rasterizer_state,
            depth_stencil_state: self.depth_stencil_state,
            input_layout: self.input_layout,
            ib_strip_cut_value: self.ib_strip_cut_value,
            primitive_topology_type: self.primitive_topology_type,
            rtv_formats: self.rtv_formats,
            dsv_format,
            sample_desc: self.sample_desc,
            node_mask: self.node_mask,
            cached_pso: self.cached_pso,
            flags: self.flags,
        }
    }
}
impl<'a, 'b, 'c>
    GraphicsPipelineStateDesc<'a, InputLayoutDesc<'b>, &'c [dxgi::Format], dxgi::Format>
{
    fn to_c_struct(
        &self,
    ) -> (
        D3D12_GRAPHICS_PIPELINE_STATE_DESC,
        (
            (Vec<D3D12_SO_DECLARATION_ENTRY>, Vec<std::ffi::CString>),
            (Vec<D3D12_INPUT_ELEMENT_DESC>, Vec<std::ffi::CString>),
        ),
    ) {
        let (stream_output, tmp_so) = self.stream_output.as_ref().map_or(
            (
                D3D12_STREAM_OUTPUT_DESC::default(),
                (Vec::new(), Vec::new()),
            ),
            |so| so.to_c_struct(),
        );
        let (input_layout, tmp_il) = self.input_layout.to_c_struct();
        assert!(self.rtv_formats.len() <= 8);
        let desc = D3D12_GRAPHICS_PIPELINE_STATE_DESC {
            pRootSignature: self
                .root_signature
                .as_ref()
                .map_or(std::ptr::null_mut(), |p| p.as_com_ptr().as_ptr()),
            VS: self
                .vs
                .as_ref()
                .map_or(Default::default(), |code| code.to_c_struct()),
            PS: self
                .ps
                .as_ref()
                .map_or(Default::default(), |code| code.to_c_struct()),
            DS: self
                .ds
                .as_ref()
                .map_or(Default::default(), |code| code.to_c_struct()),
            HS: self
                .hs
                .as_ref()
                .map_or(Default::default(), |code| code.to_c_struct()),
            GS: self
                .gs
                .as_ref()
                .map_or(Default::default(), |code| code.to_c_struct()),
            StreamOutput: stream_output,
            BlendState: self.blend_state.to_c_struct(),
            SampleMask: self.sample_mask,
            RasterizerState: self.rasterizer_state.to_c_struct(),
            DepthStencilState: self.depth_stencil_state.to_c_struct(),
            InputLayout: input_layout,
            IBStripCutValue: self.ib_strip_cut_value as u32,
            PrimitiveTopologyType: self.primitive_topology_type as u32,
            NumRenderTargets: self.rtv_formats.len() as u32,
            RTVFormats: {
                let mut rtv_format = [0; 8];
                for i in 0..self.rtv_formats.len() {
                    rtv_format[i] = self.rtv_formats[i] as u32;
                }
                rtv_format
            },
            DSVFormat: self.dsv_format as u32,
            SampleDesc: self.sample_desc.to_c_struct(),
            NodeMask: self.node_mask,
            CachedPSO: self
                .cached_pso
                .as_ref()
                .map_or(Default::default(), |cached| cached.to_c_struct()),
            Flags: self.flags.map_or(0, |f| f.0),
        };
        (desc, (tmp_so, tmp_il))
    }
}

#[derive(Clone, Debug)]
pub struct HeapDesc {
    pub size_in_bytes: u64,
    pub properties: HeapProperties<HeapType>,
    pub alignment: u64,
    pub flags: Option<HeapFlags>,
}
impl HeapDesc {
    fn to_c_struct(&self) -> D3D12_HEAP_DESC {
        D3D12_HEAP_DESC {
            SizeInBytes: self.size_in_bytes,
            Properties: self.properties.to_c_struct(),
            Alignment: self.alignment,
            Flags: self.flags.map_or(0, |f| f.0),
        }
    }
}
impl From<D3D12_HEAP_DESC> for HeapDesc {
    fn from(src: D3D12_HEAP_DESC) -> HeapDesc {
        HeapDesc {
            size_in_bytes: src.SizeInBytes,
            properties: src.Properties.into(),
            alignment: src.Alignment,
            flags: if src.Flags == 0 {
                None
            } else {
                Some(HeapFlags(src.Flags))
            },
        }
    }
}

#[derive(Clone, Debug)]
pub struct HeapProperties<Ht> {
    pub heap_type: Ht,
    pub cpu_page_property: CPUPageProperty,
    pub memory_pool_preference: MemoryPool,
    pub creation_node_mask: u32,
    pub visible_node_mask: u32,
}
impl HeapProperties<()> {
    pub fn new() -> Self {
        Self {
            heap_type: (),
            cpu_page_property: CPUPageProperty::Unknown,
            memory_pool_preference: MemoryPool::Unknown,
            creation_node_mask: 1,
            visible_node_mask: 1,
        }
    }
    pub fn heap_type(self, heap_type: HeapType) -> HeapProperties<HeapType> {
        HeapProperties {
            heap_type,
            cpu_page_property: self.cpu_page_property,
            memory_pool_preference: self.memory_pool_preference,
            creation_node_mask: self.creation_node_mask,
            visible_node_mask: self.visible_node_mask,
        }
    }
}
impl<Ht> HeapProperties<Ht> {
    pub fn cpu_page_property(mut self, cpu_page_property: CPUPageProperty) -> Self {
        self.cpu_page_property = cpu_page_property;
        self
    }
    pub fn memory_pool_preference(mut self, memory_pool_preference: MemoryPool) -> Self {
        self.memory_pool_preference = memory_pool_preference;
        self
    }
    pub fn creation_node_mask(mut self, mask: u32) -> Self {
        self.creation_node_mask = mask;
        self
    }
    pub fn visible_node_mask(mut self, mask: u32) -> Self {
        self.visible_node_mask = mask;
        self
    }
}
impl HeapProperties<HeapType> {
    fn to_c_struct(&self) -> D3D12_HEAP_PROPERTIES {
        D3D12_HEAP_PROPERTIES {
            Type: self.heap_type as u32,
            CPUPageProperty: self.cpu_page_property as u32,
            MemoryPoolPreference: self.memory_pool_preference as u32,
            CreationNodeMask: self.creation_node_mask,
            VisibleNodeMask: self.visible_node_mask,
        }
    }
}
impl From<D3D12_HEAP_PROPERTIES> for HeapProperties<HeapType> {
    fn from(src: D3D12_HEAP_PROPERTIES) -> HeapProperties<HeapType> {
        unsafe {
            HeapProperties {
                heap_type: std::mem::transmute(src.Type),
                cpu_page_property: std::mem::transmute(src.CPUPageProperty),
                memory_pool_preference: std::mem::transmute(src.MemoryPoolPreference),
                creation_node_mask: src.CreationNodeMask,
                visible_node_mask: src.VisibleNodeMask,
            }
        }
    }
}

// pub struct HitGroupDesc;

#[derive(Clone, Debug)]
#[repr(C)]
pub struct IndexBufferView {
    pub buffer_location: GPUVirtualAddress,
    pub size_in_bytes: u32,
    pub format: dxgi::Format,
}

#[derive(Clone, Debug)]
pub enum IndirectArgumentDesc {
    VertexBuffer {
        slot: u32,
    },
    Constant {
        root_parameter_index: u32,
        dest_offset_in_32bit_values: u32,
        num_32bit_values_to_set: u32,
    },
    ConstantBufferView {
        root_parameter_index: u32,
    },
    ShaderResourceView {
        root_parameter_index: u32,
    },
    UnorderedAccessView {
        root_parameter_index: u32,
    },
}
impl IndirectArgumentDesc {
    fn to_c_struct(&self) -> D3D12_INDIRECT_ARGUMENT_DESC {
        let mut obj = D3D12_INDIRECT_ARGUMENT_DESC::default();
        match self {
            &IndirectArgumentDesc::VertexBuffer { slot } => unsafe {
                obj.u.VertexBuffer_mut().Slot = slot;
            },
            &IndirectArgumentDesc::Constant {
                root_parameter_index,
                dest_offset_in_32bit_values,
                num_32bit_values_to_set,
            } => unsafe {
                obj.u.Constant_mut().RootParameterIndex = root_parameter_index;
                obj.u.Constant_mut().DestOffsetIn32BitValues = dest_offset_in_32bit_values;
                obj.u.Constant_mut().Num32BitValuesToSet = num_32bit_values_to_set;
            },
            &IndirectArgumentDesc::ConstantBufferView {
                root_parameter_index,
            } => unsafe {
                obj.u.ConstantBufferView_mut().RootParameterIndex = root_parameter_index;
            },
            &IndirectArgumentDesc::ShaderResourceView {
                root_parameter_index,
            } => unsafe {
                obj.u.ConstantBufferView_mut().RootParameterIndex = root_parameter_index;
            },
            &IndirectArgumentDesc::UnorderedAccessView {
                root_parameter_index,
            } => unsafe {
                obj.u.ConstantBufferView_mut().RootParameterIndex = root_parameter_index;
            },
        }
        obj
    }
}

pub const APPEND_ALIGNED_ELEMENT: u32 = 0xffffffff;

#[derive(Clone, Debug)]
pub struct InputElementDesc<'a> {
    pub semantic_name: &'a str,
    pub semantic_index: u32,
    pub format: dxgi::Format,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
    pub input_slot_class: InputClassification,
    pub instance_data_step_rate: u32,
}
impl<'a> InputElementDesc<'a> {
    fn to_c_struct(&self) -> (D3D12_INPUT_ELEMENT_DESC, std::ffi::CString) {
        let name = std::ffi::CString::new(self.semantic_name).unwrap();
        (
            D3D12_INPUT_ELEMENT_DESC {
                SemanticName: name.as_ptr(),
                SemanticIndex: self.semantic_index,
                Format: self.format as u32,
                InputSlot: self.input_slot,
                AlignedByteOffset: self.aligned_byte_offset,
                InputSlotClass: self.instance_data_step_rate as u32,
                InstanceDataStepRate: self.instance_data_step_rate,
            },
            name,
        )
    }
}

#[derive(Clone, Debug)]
pub struct InputLayoutDesc<'a>(pub Vec<InputElementDesc<'a>>);
impl<'a> InputLayoutDesc<'a> {
    fn to_c_struct(
        &self,
    ) -> (
        D3D12_INPUT_LAYOUT_DESC,
        (Vec<D3D12_INPUT_ELEMENT_DESC>, Vec<std::ffi::CString>),
    ) {
        let (elements, names): (Vec<_>, Vec<_>) =
            self.0.iter().map(|elem| elem.to_c_struct()).unzip();
        (
            D3D12_INPUT_LAYOUT_DESC {
                pInputElementDescs: elements.as_ptr(),
                NumElements: elements.len() as u32,
            },
            (elements, names),
        )
    }
}

#[macro_export]
macro_rules! d3d12_input_layout_descs {
    ($({$name: expr, $index: expr, $format: expr, $slot: expr, $offset: expr, $class: expr, $rate: expr},)*) => {
        $crate::d3d12::InputLayoutDesc(vec![
            $(
                $crate::d3d12::InputElementDesc {
                    semantic_name: $name,
                    semantic_index: $index,
                    format: $format,
                    input_slot: $slot,
                    aligned_byte_offset: $offset,
                    input_slot_class: $class,
                    instance_data_step_rate: $rate,
                },
            )*
        ])
    };
}

#[derive(Clone, Debug)]
pub struct LocalRootSignature {
    pub local_root_signature: RootSignature,
}

#[derive(Debug)]
pub struct MemcpyDest {
    pub data: *mut u8,
    pub row_pitch: usize,
    pub slice_pitch: usize,
}

// pub struct MetaCommandDesc;
// pub struct MetaCommandParameterDesc;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct NodeMask(pub u32);

#[derive(Clone, Default, Debug)]
pub struct PackedMipInfo {
    pub num_standard_mips: u8,
    pub num_packed_mips: u8,
    pub num_tiles_for_packed_mips: u32,
    pub start_tile_index_in_overall_resource: u32,
}
impl From<D3D12_PACKED_MIP_INFO> for PackedMipInfo {
    fn from(src: D3D12_PACKED_MIP_INFO) -> PackedMipInfo {
        PackedMipInfo {
            num_standard_mips: src.NumStandardMips,
            num_packed_mips: src.NumPackedMips,
            num_tiles_for_packed_mips: src.NumTilesForPackedMips,
            start_tile_index_in_overall_resource: src.StartTileIndexInOverallResource,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PipelineStateStreamDesc<'a> {
    pub pipeline_state_suboject_stream: &'a [u8],
}
impl<'a> PipelineStateStreamDesc<'a> {
    fn to_c_struct(&self) -> D3D12_PIPELINE_STATE_STREAM_DESC {
        D3D12_PIPELINE_STATE_STREAM_DESC {
            SizeInBytes: self.pipeline_state_suboject_stream.len(),
            pPipelineStateSubobjectStream: self.pipeline_state_suboject_stream.as_ptr()
                as *mut c_void,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct PlacedSubresourceFootprint {
    pub offset: u64,
    pub footprint: SubresourceFootprint,
}

#[derive(Clone, Debug)]
pub struct QueryDataPipelineStatistics {
    pub ia_vertices: u64,
    pub ia_primitives: u64,
    pub vs_invocations: u64,
    pub gs_invocations: u64,
    pub gs_primitives: u64,
    pub c_invocations: u64,
    pub c_primitives: u64,
    pub ps_invocations: u64,
    pub hs_invocations: u64,
    pub ds_invocations: u64,
    pub cs_invocations: u64,
}

#[derive(Clone, Debug)]
pub struct QueryDataSOStatistics {
    pub num_primitive_written: u64,
    pub primitives_storage_needed: u64,
}

#[derive(Clone, Debug)]
pub struct QueryHeapDesc {
    pub ty: QueryHeapType,
    pub count: u32,
    pub node_mask: u32,
}
impl QueryHeapDesc {
    fn to_c_struct(&self) -> D3D12_QUERY_HEAP_DESC {
        D3D12_QUERY_HEAP_DESC {
            Type: self.ty as u32,
            Count: self.count,
            NodeMask: self.node_mask,
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Range {
    pub begin: usize,
    pub end: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct RangeUint64 {
    pub begin: u64,
    pub end: u64,
}

pub const DEFAULT_DEPTH_BIAS: u32 = D3D12_DEFAULT_DEPTH_BIAS;
pub const DEFAULT_DEPTH_BIAS_CLAMP: f32 = D3D12_DEFAULT_DEPTH_BIAS_CLAMP;
pub const DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = D3D12_DEFAULT_SLOPE_SCALED_DEPTH_BIAS;

#[derive(Clone, Debug)]
pub struct RasterizerDesc {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
    pub front_counter_clockwise: bool,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    pub depth_clip_enable: bool,
    pub multisample_enable: bool,
    pub antialiased_line_enable: bool,
    pub forced_sample_count: u32,
    pub conservative_raster: ConservativeRasterizationMode,
}
impl RasterizerDesc {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn fill_mode(mut self, mode: FillMode) -> Self {
        self.fill_mode = mode;
        self
    }
    pub fn cull_mode(mut self, mode: CullMode) -> Self {
        self.cull_mode = mode;
        self
    }
    pub fn front_counter_clockwise(mut self, front_counter_clockwise: bool) -> Self {
        self.front_counter_clockwise = front_counter_clockwise;
        self
    }
    pub fn depth_bias(mut self, depth_bias: i32) -> Self {
        self.depth_bias = depth_bias;
        self
    }
    pub fn depth_bias_clamp(mut self, depth_bias_clamp: f32) -> Self {
        self.depth_bias_clamp = depth_bias_clamp;
        self
    }
    pub fn slope_scaled_depth_bias(mut self, slope_scaled_depth_bias: f32) -> Self {
        self.slope_scaled_depth_bias = slope_scaled_depth_bias;
        self
    }
    pub fn depth_clip_enable(mut self, depth_clip_enable: bool) -> Self {
        self.depth_clip_enable = depth_clip_enable;
        self
    }
    pub fn multisample_enable(mut self, multisample_enable: bool) -> Self {
        self.multisample_enable = multisample_enable;
        self
    }
    pub fn antialiased_line_enable(mut self, antialiased_line_enable: bool) -> Self {
        self.antialiased_line_enable = antialiased_line_enable;
        self
    }
    pub fn forced_sample_count(mut self, forced_sample_count: u32) -> Self {
        self.forced_sample_count = forced_sample_count;
        self
    }
    pub fn conservative_raster(
        mut self,
        conservative_raster: ConservativeRasterizationMode,
    ) -> Self {
        self.conservative_raster = conservative_raster;
        self
    }
    fn to_c_struct(&self) -> D3D12_RASTERIZER_DESC {
        D3D12_RASTERIZER_DESC {
            FillMode: self.fill_mode as u32,
            CullMode: self.cull_mode as u32,
            FrontCounterClockwise: to_BOOL(self.front_counter_clockwise),
            DepthBias: self.depth_bias,
            DepthBiasClamp: self.depth_bias_clamp,
            SlopeScaledDepthBias: self.slope_scaled_depth_bias,
            DepthClipEnable: to_BOOL(self.depth_clip_enable),
            MultisampleEnable: to_BOOL(self.multisample_enable),
            AntialiasedLineEnable: to_BOOL(self.antialiased_line_enable),
            ForcedSampleCount: self.forced_sample_count,
            ConservativeRaster: self.conservative_raster as u32,
        }
    }
}
impl Default for RasterizerDesc {
    fn default() -> Self {
        Self {
            fill_mode: FillMode::Solid,
            cull_mode: CullMode::Back,
            front_counter_clockwise: false,
            depth_bias: 0,
            depth_bias_clamp: 0.0,
            slope_scaled_depth_bias: 0.0,
            depth_clip_enable: true,
            multisample_enable: false,
            antialiased_line_enable: false,
            forced_sample_count: 0,
            conservative_raster: ConservativeRasterizationMode::Off,
        }
    }
}

// pub struct RaytracingAABB;
// pub struct RaytracingAccelerationStructurePostbuildInfoCompactedSizeDesc;
// pub struct RaytracingAccelerationStructurePostbuildInfoCurrentSizeDesc;
// pub struct RaytracingAccelerationStructurePostbuildInfoDesc;
// pub struct RaytracingAccelerationStructurePostbuildInfoSerializationDesc;
// pub struct RaytracingAccelerationStructurePostbuildInfoToolsVisualizationDesc;
// pub struct RaytracingAccelerationStructurePrebuildInfo;
// pub struct RaytracingAccelerationStructureSRV;
// pub struct RaytracingGeometryAABBsDesc;
// pub struct RaytracingGeometryADesc;
// pub struct RaytracingGeometryTrianglesDesc;
// pub struct RaytracingInstanceDesc;
// pub struct RaytracingShaderConfig;
// pub struct RenderPassBeginningAccess;
// pub struct RenderPassBeginningAccessClearParameters;
// pub struct RenderPassDepthStencilDesc;
// pub struct RenderPassEndingAccess;
// pub struct RenderPassEndingAccessResolveParameters;
// pub struct RenderPassEndingAccessResolveSubresourceParameters;
// pub struct RenderPassRenderTargetDesc;

#[derive(Clone, Debug)]
pub struct RenderTargetBlendDesc {
    pub blend_enable: bool,
    pub logic_op_enable: bool,
    pub src_blend: Blend,
    pub dest_blend: Blend,
    pub blend_op: BlendOp,
    pub src_blend_alpha: Blend,
    pub dest_blend_alpha: Blend,
    pub blend_op_alpha: BlendOp,
    pub logic_op: LogicOp,
    pub render_target_write_mask: ColorWriteEnable,
}
impl Default for RenderTargetBlendDesc {
    fn default() -> Self {
        Self {
            blend_enable: false,
            logic_op_enable: false,
            src_blend: Blend::One,
            dest_blend: Blend::Zero,
            blend_op: BlendOp::Add,
            src_blend_alpha: Blend::One,
            dest_blend_alpha: Blend::Zero,
            blend_op_alpha: BlendOp::Add,
            logic_op: LogicOp::Noop,
            render_target_write_mask: ColorWriteEnable::All,
        }
    }
}
impl RenderTargetBlendDesc {
    fn to_c_struct(&self) -> D3D12_RENDER_TARGET_BLEND_DESC {
        D3D12_RENDER_TARGET_BLEND_DESC {
            BlendEnable: to_BOOL(self.blend_enable),
            LogicOpEnable: to_BOOL(self.logic_op_enable),
            SrcBlend: self.src_blend as u32,
            DestBlend: self.dest_blend as u32,
            BlendOp: self.blend_op as u32,
            SrcBlendAlpha: self.src_blend_alpha as u32,
            DestBlendAlpha: self.dest_blend_alpha as u32,
            BlendOpAlpha: self.blend_op_alpha as u32,
            LogicOp: self.logic_op as u32,
            RenderTargetWriteMask: self.render_target_write_mask.0 as u8,
        }
    }
}

#[derive(Clone, Debug)]
pub enum RenderTargetViewDesc {
    Buffer {
        format: dxgi::Format,
        first_element: u64,
        num_elements: u32,
    },
    Texture1D {
        format: dxgi::Format,
        mip_slice: u32,
    },
    Texture1DArray {
        format: dxgi::Format,
        first_array_slice: u32,
        array_size: u32,
    },
    Texture2D {
        format: dxgi::Format,
        mip_slice: u32,
        plane_slice: u32,
    },
    Texture2DArray {
        format: dxgi::Format,
        first_array_slice: u32,
        array_size: u32,
        plane_slice: u32,
    },
    Texture2DMS {
        format: dxgi::Format,
    },
    Texture2DMSArray {
        format: dxgi::Format,
        first_array_slice: u32,
        array_size: u32,
    },
    Texture3D {
        format: dxgi::Format,
        mip_slice: u32,
        first_w_slice: u32,
        w_size: u32,
    },
}
impl RenderTargetViewDesc {
    fn to_c_struct(&self) -> D3D12_RENDER_TARGET_VIEW_DESC {
        let mut obj = D3D12_RENDER_TARGET_VIEW_DESC::default();
        match self {
            &RenderTargetViewDesc::Buffer {
                format,
                first_element,
                num_elements,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = RTVDimension::Buffer as u32;
                obj.u.Buffer_mut().FirstElement = first_element;
                obj.u.Buffer_mut().NumElements = num_elements;
            },
            &RenderTargetViewDesc::Texture1D { format, mip_slice } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = RTVDimension::Texture1D as u32;
                obj.u.Texture1D_mut().MipSlice = mip_slice;
            },
            &RenderTargetViewDesc::Texture1DArray {
                format,
                first_array_slice,
                array_size,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = RTVDimension::Texture1DArray as u32;
                obj.u.Texture1DArray_mut().FirstArraySlice = first_array_slice;
                obj.u.Texture1DArray_mut().ArraySize = array_size;
            },
            &RenderTargetViewDesc::Texture2D {
                format,
                mip_slice,
                plane_slice,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = RTVDimension::Texture2D as u32;
                obj.u.Texture2D_mut().MipSlice = mip_slice;
                obj.u.Texture2D_mut().PlaneSlice = plane_slice;
            },
            &RenderTargetViewDesc::Texture2DArray {
                format,
                first_array_slice,
                array_size,
                plane_slice,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = RTVDimension::Texture2DArray as u32;
                obj.u.Texture2DArray_mut().FirstArraySlice = first_array_slice;
                obj.u.Texture2DArray_mut().ArraySize = array_size;
                obj.u.Texture2DArray_mut().PlaneSlice = plane_slice;
            },
            &RenderTargetViewDesc::Texture2DMS { format } => {
                obj.Format = format as u32;
                obj.ViewDimension = RTVDimension::Texture2DMS as u32;
            }
            &RenderTargetViewDesc::Texture2DMSArray {
                format,
                first_array_slice,
                array_size,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = RTVDimension::Texture2DMSArray as u32;
                obj.u.Texture2DMSArray_mut().FirstArraySlice = first_array_slice;
                obj.u.Texture2DMSArray_mut().ArraySize = array_size;
            },
            &RenderTargetViewDesc::Texture3D {
                format,
                mip_slice,
                first_w_slice,
                w_size,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = RTVDimension::Texture3D as u32;
                obj.u.Texture3D_mut().MipSlice = mip_slice;
                obj.u.Texture3D_mut().FirstWSlice = first_w_slice;
                obj.u.Texture3D_mut().WSize = w_size;
            },
        }
        obj
    }
}

#[derive(Clone, Debug)]
pub struct ResourceAliasingBarrier {
    pub resource_before: Resource,
    pub resource_after: Resource,
}

#[derive(Clone, Debug)]
pub struct ResourceAllocationInfo {
    pub size_in_bytes: u64,
    pub alignment: u64,
}

pub const RESOURCE_BARRIER_ALL_SUBRESOURCES: u32 = 0xffffffff;

#[derive(Clone, Debug)]
pub enum ResourceBarrier<'a, T: IResource> {
    Transition {
        flags: Option<ResourceBarrierFlags>,
        resource: &'a T,
        subresource: u32,
        state_before: ResourceStates,
        state_after: ResourceStates,
    },
    Aliasing {
        flags: Option<ResourceBarrierFlags>,
        resource_before: &'a T,
        resource_after: &'a T,
    },
    UAV {
        flags: Option<ResourceBarrierFlags>,
        resource: &'a T,
    },
}
impl<'a, T: IResource> ResourceBarrier<'a, T> {
    fn to_c_struct(&self) -> D3D12_RESOURCE_BARRIER {
        let mut obj = D3D12_RESOURCE_BARRIER::default();
        match self {
            &ResourceBarrier::Transition {
                flags,
                resource,
                subresource,
                state_before,
                state_after,
            } => unsafe {
                obj.Type = D3D12_RESOURCE_BARRIER_TYPE_TRANSITION;
                obj.Flags = flags.map_or(0, |f| f.0);
                obj.u.Transition_mut().pResource = resource.as_ptr() as *mut ID3D12Resource;
                obj.u.Transition_mut().Subresource = subresource;
                obj.u.Transition_mut().StateBefore = state_before.0;
                obj.u.Transition_mut().StateAfter = state_after.0;
            },
            &ResourceBarrier::Aliasing {
                flags,
                resource_before,
                resource_after,
            } => unsafe {
                obj.Type = D3D12_RESOURCE_BARRIER_TYPE_ALIASING;
                obj.Flags = flags.map_or(0, |f| f.0);
                obj.u.Aliasing_mut().pResourceBefore =
                    resource_before.as_ptr() as *mut ID3D12Resource;
                obj.u.Aliasing_mut().pResourceAfter =
                    resource_after.as_ptr() as *mut ID3D12Resource;
            },
            &ResourceBarrier::UAV { flags, resource } => unsafe {
                obj.Type = D3D12_RESOURCE_BARRIER_TYPE_UAV;
                obj.Flags = flags.map_or(0, |f| f.0);
                obj.u.UAV_mut().pResource = resource.as_ptr() as *mut ID3D12Resource;
            },
        }
        obj
    }
}

#[derive(Clone, Debug)]
pub struct ResourceDesc<D, W, H, F, L> {
    pub dimension: D,
    pub alignment: u64,
    pub width: W,
    pub height: H,
    pub depth_or_array_size: u16,
    pub mip_levels: u16,
    pub format: F,
    pub sample_desc: dxgi::SampleDesc,
    pub layout: L,
    pub flags: Option<ResourceFlags>,
}
impl ResourceDesc<(), (), (), (), ()> {
    pub fn new() -> Self {
        Self {
            dimension: (),
            alignment: 0,
            width: (),
            height: (),
            depth_or_array_size: 1,
            mip_levels: 1,
            format: (),
            sample_desc: Default::default(),
            layout: (),
            flags: None,
        }
    }
}
impl<D, W, H, F, L> ResourceDesc<D, W, H, F, L> {
    pub fn dimension(
        self,
        dimension: ResourceDimension,
    ) -> ResourceDesc<ResourceDimension, W, H, F, L> {
        ResourceDesc {
            dimension,
            alignment: self.alignment,
            width: self.width,
            height: self.height,
            depth_or_array_size: self.depth_or_array_size,
            mip_levels: self.mip_levels,
            format: self.format,
            sample_desc: self.sample_desc,
            layout: self.layout,
            flags: self.flags,
        }
    }
    pub fn alignment(mut self, alignment: u64) -> Self {
        self.alignment = alignment;
        self
    }
    pub fn width(self, width: u64) -> ResourceDesc<D, u64, H, F, L> {
        ResourceDesc {
            dimension: self.dimension,
            alignment: self.alignment,
            width,
            height: self.height,
            depth_or_array_size: self.depth_or_array_size,
            mip_levels: self.mip_levels,
            format: self.format,
            sample_desc: self.sample_desc,
            layout: self.layout,
            flags: self.flags,
        }
    }
    pub fn height(self, height: u32) -> ResourceDesc<D, W, u32, F, L> {
        ResourceDesc {
            dimension: self.dimension,
            alignment: self.alignment,
            width: self.width,
            height,
            depth_or_array_size: self.depth_or_array_size,
            mip_levels: self.mip_levels,
            format: self.format,
            sample_desc: self.sample_desc,
            layout: self.layout,
            flags: self.flags,
        }
    }
    pub fn depth_or_array_size(mut self, size: u16) -> Self {
        self.depth_or_array_size = size;
        self
    }
    pub fn mip_levels(mut self, mip_levels: u16) -> Self {
        self.mip_levels = mip_levels;
        self
    }
    pub fn format(self, format: dxgi::Format) -> ResourceDesc<D, W, H, dxgi::Format, L> {
        ResourceDesc {
            dimension: self.dimension,
            alignment: self.alignment,
            width: self.width,
            height: self.height,
            depth_or_array_size: self.depth_or_array_size,
            mip_levels: self.mip_levels,
            format,
            sample_desc: self.sample_desc,
            layout: self.layout,
            flags: self.flags,
        }
    }
    pub fn sample_desc(mut self, desc: dxgi::SampleDesc) -> Self {
        self.sample_desc = desc;
        self
    }
    pub fn layout(self, layout: TextureLayout) -> ResourceDesc<D, W, H, F, TextureLayout> {
        ResourceDesc {
            dimension: self.dimension,
            alignment: self.alignment,
            width: self.width,
            height: self.height,
            depth_or_array_size: self.depth_or_array_size,
            mip_levels: self.mip_levels,
            format: self.format,
            sample_desc: self.sample_desc,
            layout,
            flags: self.flags,
        }
    }
    pub fn flags(mut self, flags: ResourceFlags) -> Self {
        self.flags = Some(flags);
        self
    }
}
impl ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout> {
    fn to_c_struct(&self) -> D3D12_RESOURCE_DESC {
        D3D12_RESOURCE_DESC {
            Dimension: self.dimension as u32,
            Alignment: self.alignment,
            Width: self.width,
            Height: self.height,
            DepthOrArraySize: self.depth_or_array_size,
            MipLevels: self.mip_levels,
            Format: self.format as u32,
            SampleDesc: self.sample_desc.to_c_struct(),
            Layout: self.layout as u32,
            Flags: self.flags.map_or(0, |f| f.0),
        }
    }
}
impl From<D3D12_RESOURCE_DESC>
    for ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>
{
    fn from(
        src: D3D12_RESOURCE_DESC,
    ) -> ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout> {
        unsafe {
            ResourceDesc {
                dimension: std::mem::transmute(src.Dimension),
                alignment: src.Alignment,
                width: src.Width,
                height: src.Height,
                depth_or_array_size: src.DepthOrArraySize,
                mip_levels: src.MipLevels,
                format: std::mem::transmute(src.Format),
                sample_desc: src.SampleDesc.into(),
                layout: std::mem::transmute(src.Layout),
                flags: if src.Flags == 0 {
                    None
                } else {
                    Some(ResourceFlags(src.Flags))
                },
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum RootParameter {
    DescriptorTable {
        descriptor_ranges: Vec<DescriptorRange>,
        shader_visibility: ShaderVisibility,
    },
    Constants {
        shader_register: u32,
        register_space: u32,
        num_32bit_values: u32,
        shader_visibility: ShaderVisibility,
    },
    Descriptor {
        parameter_type: RootParameterType,
        shader_register: u32,
        register_space: u32,
        shader_visibility: ShaderVisibility,
    },
}
impl RootParameter {
    fn to_c_struct(&self) -> D3D12_ROOT_PARAMETER {
        let mut obj = D3D12_ROOT_PARAMETER::default();
        match self {
            RootParameter::DescriptorTable {
                descriptor_ranges,
                shader_visibility,
            } => unsafe {
                obj.ParameterType = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE;
                obj.u.DescriptorTable_mut().NumDescriptorRanges = descriptor_ranges.len() as u32;
                obj.u.DescriptorTable_mut().pDescriptorRanges =
                    descriptor_ranges.as_ptr() as *const D3D12_DESCRIPTOR_RANGE;
                obj.ShaderVisibility = *shader_visibility as u32;
            },
            RootParameter::Constants {
                shader_register,
                register_space,
                num_32bit_values,
                shader_visibility,
            } => unsafe {
                obj.ParameterType = D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS;
                obj.u.Constants_mut().ShaderRegister = *shader_register;
                obj.u.Constants_mut().RegisterSpace = *register_space;
                obj.u.Constants_mut().Num32BitValues = *num_32bit_values;
                obj.ShaderVisibility = *shader_visibility as u32;
            },
            RootParameter::Descriptor {
                parameter_type,
                shader_register,
                register_space,
                shader_visibility,
            } => unsafe {
                obj.ParameterType = *parameter_type as u32;
                obj.u.Descriptor_mut().ShaderRegister = *shader_register;
                obj.u.Descriptor_mut().RegisterSpace = *register_space;
                obj.ShaderVisibility = *shader_visibility as u32;
            },
        }
        obj
    }
}
impl From<D3D12_ROOT_PARAMETER> for RootParameter {
    fn from(src: D3D12_ROOT_PARAMETER) -> RootParameter {
        match src.ParameterType {
            D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE => unsafe {
                let ranges = std::slice::from_raw_parts(
                    src.u.DescriptorTable().pDescriptorRanges,
                    src.u.DescriptorTable().NumDescriptorRanges as usize,
                );
                RootParameter::DescriptorTable {
                    descriptor_ranges: ranges.iter().map(|r| r.clone().into()).collect::<Vec<_>>(),
                    shader_visibility: std::mem::transmute(src.ShaderVisibility),
                }
            },
            D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS => unsafe {
                RootParameter::Constants {
                    shader_register: src.u.Constants().ShaderRegister,
                    register_space: src.u.Constants().RegisterSpace,
                    num_32bit_values: src.u.Constants().Num32BitValues,
                    shader_visibility: std::mem::transmute(src.ShaderVisibility),
                }
            },
            D3D12_ROOT_PARAMETER_TYPE_CBV
            | D3D12_ROOT_PARAMETER_TYPE_SRV
            | D3D12_ROOT_PARAMETER_TYPE_UAV => unsafe {
                RootParameter::Descriptor {
                    parameter_type: std::mem::transmute(src.ParameterType),
                    shader_register: src.u.Descriptor().ShaderRegister,
                    register_space: src.u.Descriptor().RegisterSpace,
                    shader_visibility: std::mem::transmute(src.ShaderVisibility),
                }
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum RootParameter1 {
    DescriptorTable1 {
        descriptor_ranges: Vec<DescriptorRange1>,
        shader_visibility: ShaderVisibility,
    },
    Constants {
        shader_register: u32,
        register_space: u32,
        num_32bit_values: u32,
        shader_visibility: ShaderVisibility,
    },
    Descriptor1 {
        parameter_type: RootParameterType,
        shader_register: u32,
        register_space: u32,
        flags: Option<RootDescriptorFlags>,
        shader_visibility: ShaderVisibility,
    },
}
impl RootParameter1 {
    fn to_c_struct(&self) -> (D3D12_ROOT_PARAMETER1, Vec<D3D12_DESCRIPTOR_RANGE1>) {
        let mut obj = D3D12_ROOT_PARAMETER1::default();
        match self {
            RootParameter1::DescriptorTable1 {
                descriptor_ranges,
                shader_visibility,
            } => unsafe {
                let dr = descriptor_ranges
                    .iter()
                    .map(|r| r.to_c_struct())
                    .collect::<Vec<_>>();
                obj.ParameterType = D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE;
                obj.u.DescriptorTable_mut().NumDescriptorRanges = dr.len() as u32;
                obj.u.DescriptorTable_mut().pDescriptorRanges = dr.as_ptr();
                obj.ShaderVisibility = *shader_visibility as u32;
                (obj, dr)
            },
            RootParameter1::Constants {
                shader_register,
                register_space,
                num_32bit_values,
                shader_visibility,
            } => unsafe {
                obj.ParameterType = D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS;
                obj.u.Constants_mut().ShaderRegister = *shader_register;
                obj.u.Constants_mut().RegisterSpace = *register_space;
                obj.u.Constants_mut().Num32BitValues = *num_32bit_values;
                obj.ShaderVisibility = *shader_visibility as u32;
                (obj, Vec::new())
            },
            RootParameter1::Descriptor1 {
                parameter_type,
                shader_register,
                register_space,
                flags,
                shader_visibility,
            } => unsafe {
                obj.ParameterType = *parameter_type as u32;
                obj.u.Descriptor_mut().ShaderRegister = *shader_register;
                obj.u.Descriptor_mut().RegisterSpace = *register_space;
                obj.u.Descriptor_mut().Flags = flags.map_or(0, |f| f.0 as u32);
                obj.ShaderVisibility = *shader_visibility as u32;
                (obj, Vec::new())
            },
        }
    }
}
impl From<D3D12_ROOT_PARAMETER1> for RootParameter1 {
    fn from(src: D3D12_ROOT_PARAMETER1) -> RootParameter1 {
        match src.ParameterType {
            D3D12_ROOT_PARAMETER_TYPE_DESCRIPTOR_TABLE => unsafe {
                let ranges = std::slice::from_raw_parts(
                    src.u.DescriptorTable().pDescriptorRanges,
                    src.u.DescriptorTable().NumDescriptorRanges as usize,
                );
                RootParameter1::DescriptorTable1 {
                    descriptor_ranges: ranges.iter().map(|r| r.clone().into()).collect::<Vec<_>>(),
                    shader_visibility: std::mem::transmute(src.ShaderVisibility),
                }
            },
            D3D12_ROOT_PARAMETER_TYPE_32BIT_CONSTANTS => unsafe {
                RootParameter1::Constants {
                    shader_register: src.u.Constants().ShaderRegister,
                    register_space: src.u.Constants().RegisterSpace,
                    num_32bit_values: src.u.Constants().Num32BitValues,
                    shader_visibility: std::mem::transmute(src.ShaderVisibility),
                }
            },
            D3D12_ROOT_PARAMETER_TYPE_CBV
            | D3D12_ROOT_PARAMETER_TYPE_SRV
            | D3D12_ROOT_PARAMETER_TYPE_UAV => unsafe {
                RootParameter1::Descriptor1 {
                    parameter_type: std::mem::transmute(src.ParameterType),
                    shader_register: src.u.Descriptor().ShaderRegister,
                    register_space: src.u.Descriptor().RegisterSpace,
                    flags: if src.u.Descriptor().Flags == 0 {
                        None
                    } else {
                        Some(RootDescriptorFlags(src.u.Descriptor().Flags))
                    },
                    shader_visibility: std::mem::transmute(src.ShaderVisibility),
                }
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RootSignatureDesc {
    pub parameters: Option<Vec<RootParameter>>,
    pub static_samplers: Option<Vec<StaticSamplerDesc<Filter, u32, u32, ShaderVisibility>>>,
    pub flags: Option<RootSignatureFlags>,
}
impl RootSignatureDesc {
    fn to_c_struct(
        &self,
    ) -> (
        D3D12_ROOT_SIGNATURE_DESC,
        (Vec<D3D12_ROOT_PARAMETER>, Vec<D3D12_STATIC_SAMPLER_DESC>),
    ) {
        let parameters = self.parameters.as_ref().map(|params| {
            params
                .iter()
                .map(|param| param.to_c_struct())
                .collect::<Vec<_>>()
        });
        let static_samplers = self
            .static_samplers
            .as_ref()
            .map(|sss| sss.iter().map(|ss| ss.to_c_struct()).collect::<Vec<_>>());
        (
            D3D12_ROOT_SIGNATURE_DESC {
                NumParameters: parameters.as_ref().map_or(0, |params| params.len() as u32),
                pParameters: parameters
                    .as_ref()
                    .map_or(std::ptr::null(), |params| params.as_ptr()),
                NumStaticSamplers: static_samplers.as_ref().map_or(0, |ss| ss.len() as u32),
                pStaticSamplers: static_samplers
                    .as_ref()
                    .map_or(std::ptr::null(), |ss| ss.as_ptr()),
                Flags: self.flags.map_or(0, |f| f.0),
            },
            (
                parameters.unwrap_or(Vec::new()),
                static_samplers.unwrap_or(Vec::new()),
            ),
        )
    }
}
impl From<D3D12_ROOT_SIGNATURE_DESC> for RootSignatureDesc {
    fn from(src: D3D12_ROOT_SIGNATURE_DESC) -> RootSignatureDesc {
        unsafe {
            let parameters = if src.pParameters == std::ptr::null_mut() {
                None
            } else {
                Some(
                    std::slice::from_raw_parts(src.pParameters, src.NumParameters as usize)
                        .iter()
                        .map(|param| param.clone().into())
                        .collect::<Vec<_>>(),
                )
            };
            let static_samplers = if src.pStaticSamplers == std::ptr::null_mut() {
                None
            } else {
                Some(
                    std::slice::from_raw_parts(src.pStaticSamplers, src.NumStaticSamplers as usize)
                        .iter()
                        .map(|ss| ss.clone().into())
                        .collect::<Vec<_>>(),
                )
            };
            let flags = if src.Flags == D3D12_ROOT_SIGNATURE_FLAG_NONE {
                None
            } else {
                Some(RootSignatureFlags(src.Flags))
            };
            RootSignatureDesc {
                parameters,
                static_samplers,
                flags,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct RootSignatureDesc1 {
    pub parameters: Option<Vec<RootParameter1>>,
    pub static_samplers: Option<Vec<StaticSamplerDesc<Filter, u32, u32, ShaderVisibility>>>,
    pub flags: Option<RootSignatureFlags>,
}
impl RootSignatureDesc1 {
    fn to_c_struct(
        &self,
    ) -> (
        D3D12_ROOT_SIGNATURE_DESC1,
        (
            (
                Vec<D3D12_ROOT_PARAMETER1>,
                Vec<Vec<D3D12_DESCRIPTOR_RANGE1>>,
            ),
            Vec<D3D12_STATIC_SAMPLER_DESC>,
        ),
    ) {
        let parameters: Option<(
            Vec<D3D12_ROOT_PARAMETER1>,
            Vec<Vec<D3D12_DESCRIPTOR_RANGE1>>,
        )> = self
            .parameters
            .as_ref()
            .map(|params| params.iter().map(|param| param.to_c_struct()).unzip());
        let static_samplers = self
            .static_samplers
            .as_ref()
            .map(|sss| sss.iter().map(|ss| ss.to_c_struct()).collect::<Vec<_>>());
        (
            D3D12_ROOT_SIGNATURE_DESC1 {
                NumParameters: parameters
                    .as_ref()
                    .map_or(0, |params| params.0.len() as u32),
                pParameters: parameters
                    .as_ref()
                    .map_or(std::ptr::null(), |params| params.0.as_ptr()),
                NumStaticSamplers: static_samplers.as_ref().map_or(0, |ss| ss.len() as u32),
                pStaticSamplers: static_samplers
                    .as_ref()
                    .map_or(std::ptr::null(), |ss| ss.as_ptr()),
                Flags: self.flags.map_or(0, |f| f.0),
            },
            (
                parameters.unwrap_or((Vec::new(), Vec::new())),
                static_samplers.unwrap_or(Vec::new()),
            ),
        )
    }
}
impl From<D3D12_ROOT_SIGNATURE_DESC1> for RootSignatureDesc1 {
    fn from(src: D3D12_ROOT_SIGNATURE_DESC1) -> RootSignatureDesc1 {
        unsafe {
            let parameters = if src.pParameters == std::ptr::null_mut() {
                None
            } else {
                Some(
                    std::slice::from_raw_parts(src.pParameters, src.NumParameters as usize)
                        .iter()
                        .map(|param| param.clone().into())
                        .collect::<Vec<_>>(),
                )
            };
            let static_samplers = if src.pStaticSamplers == std::ptr::null_mut() {
                None
            } else {
                Some(
                    std::slice::from_raw_parts(src.pStaticSamplers, src.NumStaticSamplers as usize)
                        .iter()
                        .map(|ss| ss.clone().into())
                        .collect::<Vec<_>>(),
                )
            };
            let flags = if src.Flags == D3D12_ROOT_SIGNATURE_FLAG_NONE {
                None
            } else {
                Some(RootSignatureFlags(src.Flags))
            };
            RootSignatureDesc1 {
                parameters,
                static_samplers,
                flags,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct RTFormatArray<'a>(&'a [dxgi::Format]);

#[derive(Clone, Copy, Debug)]
pub struct SamplePosition {
    pub x: i8,
    pub y: i8,
}

pub const MAX_MAXANISOTROPY: u32 = D3D12_MAX_MAXANISOTROPY;

#[derive(Clone, Debug)]
pub struct SamplerDesc<F> {
    pub filter: F,
    pub address_u: TextureAddressMode,
    pub address_v: TextureAddressMode,
    pub address_w: TextureAddressMode,
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub comparison_func: ComparisonFunc,
    pub border_color: dxgi::RGBA,
    pub min_lod: f32,
    pub max_lod: f32,
}
impl SamplerDesc<()> {
    pub fn new() -> Self {
        Self {
            filter: (),
            address_u: TextureAddressMode::Wrap,
            address_v: TextureAddressMode::Wrap,
            address_w: TextureAddressMode::Wrap,
            mip_lod_bias: 0.0,
            max_anisotropy: MAX_MAXANISOTROPY,
            comparison_func: ComparisonFunc::Never,
            border_color: dxgi::RGBA {
                r: 0.0,
                g: 0.0,
                b: 0.0,
                a: 0.0,
            },
            min_lod: 0.0,
            max_lod: std::f32::MAX,
        }
    }
}
impl<F> SamplerDesc<F> {
    pub fn filter(self, filter: Filter) -> SamplerDesc<Filter> {
        SamplerDesc {
            filter,
            address_u: self.address_u,
            address_v: self.address_v,
            address_w: self.address_w,
            mip_lod_bias: self.mip_lod_bias,
            max_anisotropy: self.max_anisotropy,
            comparison_func: self.comparison_func,
            border_color: self.border_color,
            min_lod: self.min_lod,
            max_lod: self.max_lod,
        }
    }
    pub fn address_u(mut self, address_u: TextureAddressMode) -> Self {
        self.address_u = address_u;
        self
    }
    pub fn address_v(mut self, address_v: TextureAddressMode) -> Self {
        self.address_v = address_v;
        self
    }
    pub fn address_w(mut self, address_w: TextureAddressMode) -> Self {
        self.address_w = address_w;
        self
    }
    pub fn mip_lod_bias(mut self, mip_lod_bias: f32) -> Self {
        self.mip_lod_bias = mip_lod_bias;
        self
    }
    pub fn max_anisotropy(mut self, max_anisotropy: u32) -> Self {
        self.max_anisotropy = max_anisotropy;
        self
    }
    pub fn comparison_func(mut self, comparison_func: ComparisonFunc) -> Self {
        self.comparison_func = comparison_func;
        self
    }
    pub fn border_color(mut self, border_color: dxgi::RGBA) -> Self {
        self.border_color = border_color;
        self
    }
    pub fn min_lod(mut self, min_lod: f32) -> Self {
        self.min_lod = min_lod;
        self
    }
    pub fn max_lod(mut self, max_lod: f32) -> Self {
        self.max_lod = max_lod;
        self
    }
}
impl SamplerDesc<Filter> {
    fn to_c_struct(&self) -> D3D12_SAMPLER_DESC {
        D3D12_SAMPLER_DESC {
            Filter: self.filter as u32,
            AddressU: self.address_u as u32,
            AddressV: self.address_v as u32,
            AddressW: self.address_w as u32,
            MipLODBias: self.mip_lod_bias,
            MaxAnisotropy: self.max_anisotropy,
            ComparisonFunc: self.comparison_func as u32,
            BorderColor: [
                self.border_color.r,
                self.border_color.g,
                self.border_color.b,
                self.border_color.a,
            ],
            MinLOD: self.min_lod,
            MaxLOD: self.max_lod,
        }
    }
}

// pub struct SerializedDataDriverMatchingIdentifier;
// pub struct SerializedRaytracingAccelerationStrucutreHeader;

#[derive(Clone, Debug)]
pub struct ShaderBytecode(Vec<u8>);
impl ShaderBytecode {
    fn to_c_struct(&self) -> D3D12_SHADER_BYTECODE {
        if self.0.is_empty() {
            D3D12_SHADER_BYTECODE {
                pShaderBytecode: std::ptr::null(),
                BytecodeLength: 0,
            }
        } else {
            D3D12_SHADER_BYTECODE {
                pShaderBytecode: self.0.as_ptr() as *const c_void,
                BytecodeLength: self.0.len(),
            }
        }
    }
}
impl Default for ShaderBytecode {
    fn default() -> Self {
        Self(Vec::new())
    }
}
impl From<d3d::Blob> for ShaderBytecode {
    fn from(src: d3d::Blob) -> ShaderBytecode {
        ShaderBytecode(src.to_vec())
    }
}

#[derive(Clone, Debug)]
pub enum ShaderResourceViewDesc {
    Buffer {
        format: dxgi::Format,
        shader_4_component_mapping: ShaderComponentMapping,
        first_element: u64,
        num_elements: u32,
        structure_byte_stride: u32,
        flags: Option<BufferSRVFlags>,
    },
    Texture1D {
        format: dxgi::Format,
        shader_4_component_mapping: ShaderComponentMapping,
        most_detailed_mip: u32,
        mip_levels: u32,
        resource_min_lod_clamp: f32,
    },
    Texture1DArray {
        format: dxgi::Format,
        shader_4_component_mapping: ShaderComponentMapping,
        most_detailed_mip: u32,
        mip_levels: u32,
        first_array_slice: u32,
        array_size: u32,
        resource_min_lod_clamp: f32,
    },
    Texture2D {
        format: dxgi::Format,
        shader_4_component_mapping: ShaderComponentMapping,
        most_detailed_mip: u32,
        mip_levels: u32,
        plane_slice: u32,
        resource_min_lod_clamp: f32,
    },
    Texture2DArray {
        format: dxgi::Format,
        shader_4_component_mapping: ShaderComponentMapping,
        most_detailed_mip: u32,
        mip_levels: u32,
        first_array_slice: u32,
        array_size: u32,
        plane_slice: u32,
        resource_min_lod_clamp: f32,
    },
    Texture2DMS {
        format: dxgi::Format,
        shader_4_component_mapping: ShaderComponentMapping,
    },
    Texture2DMSArray {
        format: dxgi::Format,
        shader_4_component_mapping: ShaderComponentMapping,
        first_array_slice: u32,
        array_size: u32,
    },
    Texture3D {
        format: dxgi::Format,
        shader_4_component_mapping: ShaderComponentMapping,
        most_detailed_mip: u32,
        mip_levels: u32,
        resource_min_lod_clamp: f32,
    },
    TextureCube {
        format: dxgi::Format,
        shader_4_component_mapping: ShaderComponentMapping,
        most_detailed_mip: u32,
        mip_levels: u32,
        resource_min_lod_clamp: f32,
    },
    TextureCubeArray {
        format: dxgi::Format,
        shader_4_component_mapping: ShaderComponentMapping,
        most_detailed_mip: u32,
        mip_levels: u32,
        first_2d_array_face: u32,
        num_cubes: u32,
        resource_min_lod_clamp: f32,
    },
    /*
    RaytracingAccelerationStructureSRV {
        location: GPUVirtualAddress,
    }
    */
}
impl ShaderResourceViewDesc {
    fn to_c_struct(&self) -> D3D12_SHADER_RESOURCE_VIEW_DESC {
        let mut desc = D3D12_SHADER_RESOURCE_VIEW_DESC::default();
        match self {
            &ShaderResourceViewDesc::Buffer {
                format,
                shader_4_component_mapping,
                first_element,
                num_elements,
                structure_byte_stride,
                flags,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_SRV_DIMENSION_BUFFER;
                desc.Shader4ComponentMapping = shader_4_component_mapping.0;
                desc.u.Buffer_mut().FirstElement = first_element;
                desc.u.Buffer_mut().NumElements = num_elements;
                desc.u.Buffer_mut().StructureByteStride = structure_byte_stride;
                desc.u.Buffer_mut().Flags = flags.map_or(0, |f| f.0);
            },
            &ShaderResourceViewDesc::Texture1D {
                format,
                shader_4_component_mapping,
                most_detailed_mip,
                mip_levels,
                resource_min_lod_clamp,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE1D;
                desc.Shader4ComponentMapping = shader_4_component_mapping.0;
                desc.u.Texture1D_mut().MostDetailedMip = most_detailed_mip;
                desc.u.Texture1D_mut().MipLevels = mip_levels;
                desc.u.Texture1D_mut().ResourceMinLODClamp = resource_min_lod_clamp;
            },
            &ShaderResourceViewDesc::Texture1DArray {
                format,
                shader_4_component_mapping,
                most_detailed_mip,
                mip_levels,
                first_array_slice,
                array_size,
                resource_min_lod_clamp,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE1DARRAY;
                desc.Shader4ComponentMapping = shader_4_component_mapping.0;
                desc.u.Texture1DArray_mut().MostDetailedMip = most_detailed_mip;
                desc.u.Texture1DArray_mut().MipLevels = mip_levels;
                desc.u.Texture1DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture1DArray_mut().ArraySize = array_size;
                desc.u.Texture1DArray_mut().ResourceMinLODClamp = resource_min_lod_clamp;
            },
            &ShaderResourceViewDesc::Texture2D {
                format,
                shader_4_component_mapping,
                most_detailed_mip,
                mip_levels,
                plane_slice,
                resource_min_lod_clamp,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2D;
                desc.Shader4ComponentMapping = shader_4_component_mapping.0;
                desc.u.Texture2D_mut().MostDetailedMip = most_detailed_mip;
                desc.u.Texture2D_mut().MipLevels = mip_levels;
                desc.u.Texture2D_mut().PlaneSlice = plane_slice;
                desc.u.Texture2D_mut().ResourceMinLODClamp = resource_min_lod_clamp;
            },
            &ShaderResourceViewDesc::Texture2DArray {
                format,
                shader_4_component_mapping,
                most_detailed_mip,
                mip_levels,
                first_array_slice,
                array_size,
                plane_slice,
                resource_min_lod_clamp,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2DARRAY;
                desc.Shader4ComponentMapping = shader_4_component_mapping.0;
                desc.u.Texture2DArray_mut().MostDetailedMip = most_detailed_mip;
                desc.u.Texture2DArray_mut().MipLevels = mip_levels;
                desc.u.Texture2DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture2DArray_mut().ArraySize = array_size;
                desc.u.Texture2DArray_mut().PlaneSlice = plane_slice;
                desc.u.Texture2DArray_mut().ResourceMinLODClamp = resource_min_lod_clamp;
            },
            &ShaderResourceViewDesc::Texture2DMS {
                format,
                shader_4_component_mapping,
            } => {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2DMS;
                desc.Shader4ComponentMapping = shader_4_component_mapping.0;
            }
            &ShaderResourceViewDesc::Texture2DMSArray {
                format,
                shader_4_component_mapping,
                first_array_slice,
                array_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE2DMSARRAY;
                desc.Shader4ComponentMapping = shader_4_component_mapping.0;
                desc.u.Texture2DMSArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture2DMSArray_mut().ArraySize = array_size;
            },
            &ShaderResourceViewDesc::Texture3D {
                format,
                shader_4_component_mapping,
                most_detailed_mip,
                mip_levels,
                resource_min_lod_clamp,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURE3D;
                desc.Shader4ComponentMapping = shader_4_component_mapping.0;
                desc.u.Texture3D_mut().MostDetailedMip = most_detailed_mip;
                desc.u.Texture3D_mut().MipLevels = mip_levels;
                desc.u.Texture3D_mut().ResourceMinLODClamp = resource_min_lod_clamp;
            },
            &ShaderResourceViewDesc::TextureCube {
                format,
                shader_4_component_mapping,
                most_detailed_mip,
                mip_levels,
                resource_min_lod_clamp,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURECUBE;
                desc.Shader4ComponentMapping = shader_4_component_mapping.0;
                desc.u.TextureCube_mut().MostDetailedMip = most_detailed_mip;
                desc.u.TextureCube_mut().MipLevels = mip_levels;
                desc.u.TextureCube_mut().ResourceMinLODClamp = resource_min_lod_clamp;
            },
            &ShaderResourceViewDesc::TextureCubeArray {
                format,
                shader_4_component_mapping,
                most_detailed_mip,
                mip_levels,
                first_2d_array_face,
                num_cubes,
                resource_min_lod_clamp,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_SRV_DIMENSION_TEXTURECUBE;
                desc.Shader4ComponentMapping = shader_4_component_mapping.0;
                desc.u.TextureCubeArray_mut().MostDetailedMip = most_detailed_mip;
                desc.u.TextureCubeArray_mut().MipLevels = mip_levels;
                desc.u.TextureCubeArray_mut().First2DArrayFace = first_2d_array_face;
                desc.u.TextureCubeArray_mut().NumCubes = num_cubes;
                desc.u.TextureCubeArray_mut().ResourceMinLODClamp = resource_min_lod_clamp;
            },
        }
        desc
    }
}

#[derive(Clone, Debug)]
pub struct SODeclarationEntry<'a> {
    pub stream: u32,
    pub semantic_name: &'a str,
    pub semantic_index: u32,
    pub start_component: u8,
    pub component_count: u8,
    pub output_slot: u8,
}
impl<'a> SODeclarationEntry<'a> {
    fn to_c_struct(&self) -> (D3D12_SO_DECLARATION_ENTRY, std::ffi::CString) {
        let name = std::ffi::CString::new(self.semantic_name).unwrap();
        (
            D3D12_SO_DECLARATION_ENTRY {
                Stream: self.stream,
                SemanticName: name.as_ptr(),
                SemanticIndex: self.semantic_index,
                StartComponent: self.start_component,
                ComponentCount: self.component_count,
                OutputSlot: self.output_slot,
            },
            name,
        )
    }
}

// pub struct StateObjectConfig;
// pub struct StateObjectDesc;
// pub struct StateSubobject;

#[derive(Clone, Debug)]
pub struct StaticSamplerDesc<F, Sr, Rs, Sv> {
    pub filter: F,
    pub address_u: TextureAddressMode,
    pub address_v: TextureAddressMode,
    pub address_w: TextureAddressMode,
    pub mip_lod_bias: f32,
    pub max_anisotropy: u32,
    pub comparison_func: ComparisonFunc,
    pub border_color: StaticBorderColor,
    pub min_lod: f32,
    pub max_lod: f32,
    pub shader_register: Sr,
    pub register_space: Rs,
    pub shader_visibility: Sv,
}
impl StaticSamplerDesc<(), (), (), ()> {
    pub fn new() -> Self {
        Self {
            filter: (),
            address_u: TextureAddressMode::Wrap,
            address_v: TextureAddressMode::Wrap,
            address_w: TextureAddressMode::Wrap,
            mip_lod_bias: 0.0,
            max_anisotropy: MAX_MAXANISOTROPY,
            comparison_func: ComparisonFunc::Never,
            border_color: StaticBorderColor::TransparentBlack,
            min_lod: 0.0,
            max_lod: std::f32::MAX,
            shader_register: (),
            register_space: (),
            shader_visibility: (),
        }
    }
}
impl<F, Sr, Rs, Sv> StaticSamplerDesc<F, Sr, Rs, Sv> {
    pub fn filter(self, filter: Filter) -> StaticSamplerDesc<Filter, Sr, Rs, Sv> {
        StaticSamplerDesc {
            filter,
            address_u: self.address_u,
            address_v: self.address_v,
            address_w: self.address_w,
            mip_lod_bias: self.mip_lod_bias,
            max_anisotropy: self.max_anisotropy,
            comparison_func: self.comparison_func,
            border_color: self.border_color,
            min_lod: self.min_lod,
            max_lod: self.max_lod,
            shader_register: self.shader_register,
            register_space: self.register_space,
            shader_visibility: self.shader_visibility,
        }
    }
    pub fn address_u(mut self, address_u: TextureAddressMode) -> Self {
        self.address_u = address_u;
        self
    }
    pub fn address_v(mut self, address_v: TextureAddressMode) -> Self {
        self.address_v = address_v;
        self
    }
    pub fn address_w(mut self, address_w: TextureAddressMode) -> Self {
        self.address_w = address_w;
        self
    }
    pub fn mip_lod_bias(mut self, bias: f32) -> Self {
        self.mip_lod_bias = bias;
        self
    }
    pub fn max_anisotropy(mut self, max_anisotropy: u32) -> Self {
        self.max_anisotropy = max_anisotropy;
        self
    }
    pub fn comparison_func(mut self, func: ComparisonFunc) -> Self {
        self.comparison_func = func;
        self
    }
    pub fn border_color(mut self, border_color: StaticBorderColor) -> Self {
        self.border_color = border_color;
        self
    }
    pub fn min_lod(mut self, min_lod: f32) -> Self {
        self.min_lod = min_lod;
        self
    }
    pub fn max_lod(mut self, max_lod: f32) -> Self {
        self.max_lod = max_lod;
        self
    }
    pub fn shader_register(self, shader_register: u32) -> StaticSamplerDesc<F, u32, Rs, Sv> {
        StaticSamplerDesc {
            filter: self.filter,
            address_u: self.address_u,
            address_v: self.address_v,
            address_w: self.address_w,
            mip_lod_bias: self.mip_lod_bias,
            max_anisotropy: self.max_anisotropy,
            comparison_func: self.comparison_func,
            border_color: self.border_color,
            min_lod: self.min_lod,
            max_lod: self.max_lod,
            shader_register,
            register_space: self.register_space,
            shader_visibility: self.shader_visibility,
        }
    }
    pub fn register_space(self, register_space: u32) -> StaticSamplerDesc<F, Sr, u32, Sv> {
        StaticSamplerDesc {
            filter: self.filter,
            address_u: self.address_u,
            address_v: self.address_v,
            address_w: self.address_w,
            mip_lod_bias: self.mip_lod_bias,
            max_anisotropy: self.max_anisotropy,
            comparison_func: self.comparison_func,
            border_color: self.border_color,
            min_lod: self.min_lod,
            max_lod: self.max_lod,
            shader_register: self.shader_register,
            register_space,
            shader_visibility: self.shader_visibility,
        }
    }
    pub fn shader_visibility(
        self,
        shader_visibility: ShaderVisibility,
    ) -> StaticSamplerDesc<F, Sr, Rs, ShaderVisibility> {
        StaticSamplerDesc {
            filter: self.filter,
            address_u: self.address_u,
            address_v: self.address_v,
            address_w: self.address_w,
            mip_lod_bias: self.mip_lod_bias,
            max_anisotropy: self.max_anisotropy,
            comparison_func: self.comparison_func,
            border_color: self.border_color,
            min_lod: self.min_lod,
            max_lod: self.max_lod,
            shader_register: self.shader_register,
            register_space: self.register_space,
            shader_visibility,
        }
    }
}
impl StaticSamplerDesc<Filter, u32, u32, ShaderVisibility> {
    fn to_c_struct(&self) -> D3D12_STATIC_SAMPLER_DESC {
        D3D12_STATIC_SAMPLER_DESC {
            Filter: self.filter as u32,
            AddressU: self.address_u as u32,
            AddressV: self.address_v as u32,
            AddressW: self.address_w as u32,
            MipLODBias: self.mip_lod_bias,
            MaxAnisotropy: self.max_anisotropy,
            ComparisonFunc: self.comparison_func as u32,
            BorderColor: self.border_color as u32,
            MinLOD: self.min_lod,
            MaxLOD: self.max_lod,
            ShaderRegister: self.shader_register,
            RegisterSpace: self.register_space,
            ShaderVisibility: self.shader_visibility as u32,
        }
    }
}
impl From<D3D12_STATIC_SAMPLER_DESC> for StaticSamplerDesc<Filter, u32, u32, ShaderVisibility> {
    fn from(
        src: D3D12_STATIC_SAMPLER_DESC,
    ) -> StaticSamplerDesc<Filter, u32, u32, ShaderVisibility> {
        unsafe {
            StaticSamplerDesc {
                filter: std::mem::transmute(src.Filter),
                address_u: std::mem::transmute(src.AddressU),
                address_v: std::mem::transmute(src.AddressV),
                address_w: std::mem::transmute(src.AddressW),
                mip_lod_bias: src.MipLODBias,
                max_anisotropy: src.MaxAnisotropy,
                comparison_func: std::mem::transmute(src.ComparisonFunc),
                border_color: std::mem::transmute(src.BorderColor),
                min_lod: src.MinLOD,
                max_lod: src.MaxLOD,
                shader_register: src.ShaderRegister,
                register_space: src.RegisterSpace,
                shader_visibility: std::mem::transmute(src.ShaderVisibility),
            }
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct StreamOutputBufferView {
    pub buffer_location: GPUVirtualAddress,
    pub size_in_bytes: u64,
    pub buffer_filled_size_location: GPUVirtualAddress,
}

#[derive(Clone, Default, Debug)]
pub struct StreamOutputDesc<'a> {
    pub so_declaration: Option<Vec<SODeclarationEntry<'a>>>,
    pub buffer_strides: Option<Vec<u32>>,
    pub rasterized_stream: u32,
}
impl<'a> StreamOutputDesc<'a> {
    fn to_c_struct(
        &self,
    ) -> (
        D3D12_STREAM_OUTPUT_DESC,
        (Vec<D3D12_SO_DECLARATION_ENTRY>, Vec<std::ffi::CString>),
    ) {
        let (sod, strs): (Vec<_>, Vec<_>) = self
            .so_declaration
            .as_ref()
            .map_or((Vec::new(), Vec::new()), |v| {
                v.iter().map(|so| so.to_c_struct()).unzip()
            });
        (
            D3D12_STREAM_OUTPUT_DESC {
                pSODeclaration: if sod.is_empty() {
                    std::ptr::null()
                } else {
                    sod.as_ptr()
                },
                NumEntries: sod.len() as u32,
                pBufferStrides: self
                    .buffer_strides
                    .as_ref()
                    .map_or(std::ptr::null(), |bs| bs.as_ptr()),
                NumStrides: self.buffer_strides.as_ref().map_or(0, |bs| bs.len() as u32),
                RasterizedStream: self.rasterized_stream,
            },
            (sod, strs),
        )
    }
}

// pub struct SubobjectToExportsAssociation;

#[derive(Debug)]
pub struct SubresourceData {
    pub data: *const u8,
    pub row_pitch: isize,
    pub slice_pitch: isize,
}

#[derive(Clone, Default, Debug)]
pub struct SubresourceFootprint {
    pub format: dxgi::Format,
    pub width: u32,
    pub height: u32,
    pub depth: u32,
    pub row_pitch: u32,
}
impl SubresourceFootprint {
    fn to_c_struct(&self) -> D3D12_SUBRESOURCE_FOOTPRINT {
        D3D12_SUBRESOURCE_FOOTPRINT {
            Format: self.format as u32,
            Width: self.width,
            Height: self.height,
            Depth: self.depth,
            RowPitch: self.row_pitch,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SubresourceInfo {
    pub offset: u64,
    pub row_pitch: u32,
    pub depth_pitch: u32,
}

#[derive(Clone, Debug)]
pub struct SubresourceRangeUint64 {
    pub subresource: u32,
    pub range: RangeUint64,
}

#[derive(Clone, Default, Debug)]
pub struct SubresourceTiling {
    pub width_in_tiles: u32,
    pub height_in_tiles: u16,
    pub depth_in_tiles: u16,
    pub start_tile_index_in_overall_resource: u32,
}
impl From<D3D12_SUBRESOURCE_TILING> for SubresourceTiling {
    fn from(src: D3D12_SUBRESOURCE_TILING) -> SubresourceTiling {
        SubresourceTiling {
            width_in_tiles: src.WidthInTiles,
            height_in_tiles: src.HeightInTiles,
            depth_in_tiles: src.DepthInTiles,
            start_tile_index_in_overall_resource: src.StartTileIndexInOverallResource,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TextureCopyLocation<'a, T: IResource> {
    PlacedFootprint {
        resource: &'a T,
        offset: u64,
        footprint: SubresourceFootprint,
    },
    SubresourceIndex {
        resource: &'a T,
        index: u32,
    },
}
impl<'a, T: IResource> TextureCopyLocation<'a, T> {
    fn to_c_struct(&self) -> D3D12_TEXTURE_COPY_LOCATION {
        let mut tcl = D3D12_TEXTURE_COPY_LOCATION::default();
        match self {
            TextureCopyLocation::PlacedFootprint {
                resource,
                offset,
                footprint,
            } => unsafe {
                tcl.pResource = resource.as_ptr() as *mut ID3D12Resource;
                tcl.Type = D3D12_TEXTURE_COPY_TYPE_PLACED_FOOTPRINT;
                tcl.u.PlacedFootprint_mut().Offset = *offset;
                tcl.u.PlacedFootprint_mut().Footprint = footprint.to_c_struct();
            },
            TextureCopyLocation::SubresourceIndex { resource, index } => unsafe {
                tcl.pResource = resource.as_ptr() as *mut ID3D12Resource;
                tcl.Type = D3D12_TEXTURE_COPY_TYPE_SUBRESOURCE_INDEX;
                *tcl.u.SubresourceIndex_mut() = *index;
            },
        }
        tcl
    }
}

#[derive(Clone, Debug)]
pub struct TileRegionSize {
    pub num_tiles: u32,
    pub use_box: bool,
    pub width: u32,
    pub height: u16,
    pub depth: u16,
}
impl TileRegionSize {
    fn to_c_struct(&self) -> D3D12_TILE_REGION_SIZE {
        D3D12_TILE_REGION_SIZE {
            NumTiles: self.num_tiles,
            UseBox: to_BOOL(self.use_box),
            Width: self.width,
            Height: self.height,
            Depth: self.depth,
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct TileShape {
    pub width_in_texels: u32,
    pub height_in_texels: u32,
    pub depth_in_texels: u32,
}
impl From<D3D12_TILE_SHAPE> for TileShape {
    fn from(src: D3D12_TILE_SHAPE) -> TileShape {
        TileShape {
            width_in_texels: src.WidthInTexels,
            height_in_texels: src.HeightInTexels,
            depth_in_texels: src.DepthInTexels,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct TiledResourceCoordinate {
    pub x: u32,
    pub y: u32,
    pub z: u32,
    pub subresource: u32,
}
impl TiledResourceCoordinate {
    fn to_c_struct(&self) -> D3D12_TILED_RESOURCE_COORDINATE {
        D3D12_TILED_RESOURCE_COORDINATE {
            X: self.x,
            Y: self.y,
            Z: self.z,
            Subresource: self.subresource,
        }
    }
}

#[derive(Clone, Debug)]
pub enum UnorderedAccessViewDesc {
    Buffer {
        format: dxgi::Format,
        first_element: u64,
        num_elements: u32,
        structure_byte_stride: u32,
        counter_offset_in_bytes: u64,
        flags: Option<BufferUAVFlags>,
    },
    Texture1D {
        format: dxgi::Format,
        mip_slice: u32,
    },
    Texture1DArray {
        format: dxgi::Format,
        mip_slice: u32,
        first_array_slice: u32,
        array_size: u32,
    },
    Texture2D {
        format: dxgi::Format,
        mip_slice: u32,
        plane_slice: u32,
    },
    Texture2DArray {
        format: dxgi::Format,
        mip_slice: u32,
        first_array_slice: u32,
        array_size: u32,
        plane_slice: u32,
    },
    Texture3D {
        format: dxgi::Format,
        mip_slice: u32,
        first_w_slice: u32,
        w_size: u32,
    },
}
impl UnorderedAccessViewDesc {
    fn to_c_struct(&self) -> D3D12_UNORDERED_ACCESS_VIEW_DESC {
        let mut desc = D3D12_UNORDERED_ACCESS_VIEW_DESC::default();
        match self {
            &UnorderedAccessViewDesc::Buffer {
                format,
                first_element,
                num_elements,
                structure_byte_stride,
                counter_offset_in_bytes,
                flags,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_UAV_DIMENSION_BUFFER;
                desc.u.Buffer_mut().FirstElement = first_element;
                desc.u.Buffer_mut().NumElements = num_elements;
                desc.u.Buffer_mut().StructureByteStride = structure_byte_stride;
                desc.u.Buffer_mut().CounterOffsetInBytes = counter_offset_in_bytes;
                desc.u.Buffer_mut().Flags = flags.map_or(0, |f| f.0);
            },
            &UnorderedAccessViewDesc::Texture1D { format, mip_slice } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_UAV_DIMENSION_TEXTURE1D;
                desc.u.Texture1D_mut().MipSlice = mip_slice;
            },
            &UnorderedAccessViewDesc::Texture1DArray {
                format,
                mip_slice,
                first_array_slice,
                array_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_UAV_DIMENSION_TEXTURE1DARRAY;
                desc.u.Texture1DArray_mut().MipSlice = mip_slice;
                desc.u.Texture1DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture1DArray_mut().ArraySize = array_size;
            },
            &UnorderedAccessViewDesc::Texture2D {
                format,
                mip_slice,
                plane_slice,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_UAV_DIMENSION_TEXTURE2D;
                desc.u.Texture2D_mut().MipSlice = mip_slice;
                desc.u.Texture2D_mut().PlaneSlice = plane_slice;
            },
            &UnorderedAccessViewDesc::Texture2DArray {
                format,
                mip_slice,
                first_array_slice,
                array_size,
                plane_slice,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_UAV_DIMENSION_TEXTURE2DARRAY;
                desc.u.Texture2DArray_mut().MipSlice = mip_slice;
                desc.u.Texture2DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture2DArray_mut().ArraySize = array_size;
                desc.u.Texture2DArray_mut().PlaneSlice = plane_slice;
            },
            &UnorderedAccessViewDesc::Texture3D {
                format,
                mip_slice,
                first_w_slice,
                w_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D12_UAV_DIMENSION_TEXTURE3D;
                desc.u.Texture3D_mut().MipSlice = mip_slice;
                desc.u.Texture3D_mut().FirstWSlice = first_w_slice;
                desc.u.Texture3D_mut().WSize = w_size;
            },
        }
        desc
    }
}

// pub struct VersionedDeviceRemovedExtendedData;

pub enum VersionedRootSignatureDesc {
    Desc1_0(RootSignatureDesc),
    Desc1_1(RootSignatureDesc1),
}
impl VersionedRootSignatureDesc {
    fn to_c_struct(&self) -> (D3D12_VERSIONED_ROOT_SIGNATURE_DESC, Box<dyn Any>) {
        let mut obj = D3D12_VERSIONED_ROOT_SIGNATURE_DESC::default();
        match self {
            VersionedRootSignatureDesc::Desc1_0(desc) => unsafe {
                let c_desc = desc.to_c_struct();
                obj.Version = D3D_ROOT_SIGNATURE_VERSION_1_0;
                *obj.u.Desc_1_0_mut() = c_desc.0;
                (obj, Box::new(c_desc.1))
            },
            VersionedRootSignatureDesc::Desc1_1(desc) => unsafe {
                let c_desc = desc.to_c_struct();
                obj.Version = D3D_ROOT_SIGNATURE_VERSION_1_1;
                *obj.u.Desc_1_1_mut() = c_desc.0;
                (obj, Box::new(c_desc.1))
            },
        }
    }
}
impl From<D3D12_VERSIONED_ROOT_SIGNATURE_DESC> for VersionedRootSignatureDesc {
    fn from(src: D3D12_VERSIONED_ROOT_SIGNATURE_DESC) -> VersionedRootSignatureDesc {
        unsafe {
            match src.Version {
                D3D_ROOT_SIGNATURE_VERSION_1_0 => {
                    VersionedRootSignatureDesc::Desc1_0(src.u.Desc_1_0().clone().into())
                }
                D3D_ROOT_SIGNATURE_VERSION_1_1 => {
                    VersionedRootSignatureDesc::Desc1_1(src.u.Desc_1_1().clone().into())
                }
                _ => unimplemented!(),
            }
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct VertexBufferView {
    pub buffer_location: GPUVirtualAddress,
    pub size_in_bytes: u32,
    pub stride_in_bytes: u32,
}

// pub struct ViewInstanceLocation;
// pub struct ViewInstancingDesc;

#[derive(Clone, Default, Debug)]
#[repr(C)]
pub struct Viewport {
    pub top_left_x: f32,
    pub top_left_y: f32,
    pub width: f32,
    pub height: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}

#[derive(Clone, Debug)]
pub struct WriteBufferImmediateParameter {
    dest: GPUVirtualAddress,
    value: u32,
}

pub trait IObject: Interface {
    fn set_name(&self, name: &str) -> Result<(), HResult>;
}
macro_rules! impl_object {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IObject for $s {
            fn set_name(&self, name: &str) -> Result<(), HResult> {
                let wname = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
                let res = unsafe { self.0.SetName(wname.as_ptr()) };
                hresult((), res)
            }
        }
    };
}

pub trait IDeviceChild: IObject {
    fn get_device<T: IDevice>(&self) -> Result<T, HResult>;
}
macro_rules! impl_devicechild {
    ($s: ident, $interface: ident) => {
        impl_object!($s, $interface);
        impl IDeviceChild for $s {
            fn get_device<T: IDevice>(&self) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.GetDevice(&T::uuidof().into(), &mut obj) };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
        }
    };
}

pub trait IPageable: IDeviceChild {}
macro_rules! impl_pageable {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IPageable for $s {}
    };
}

pub trait ICommandAllocator: IPageable {
    fn reset(&self) -> Result<(), HResult>;
}
pub struct CommandAllocator(ComPtr<ID3D12CommandAllocator>);
impl_pageable!(CommandAllocator, ID3D12CommandAllocator);
impl ICommandAllocator for CommandAllocator {
    fn reset(&self) -> Result<(), HResult> {
        unsafe { hresult((), self.0.Reset()) }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GetClockCalibrationResult {
    pub gpu_timestamp: u64,
    pub cpu_timestamp: u64,
}

pub trait ICommandList: IDeviceChild {
    fn get_type(&self) -> CommandListType;
}
macro_rules! impl_command_list {
    ($s: ident, $interface: ident, CommandList) => {
        impl_devicechild!($s, $interface);
        impl ICommandList for $s {
            fn get_type(&self) -> CommandListType {
                unsafe { std::mem::transmute(self.0.GetType()) }
            }
        }
    };
}
#[derive(Clone, Debug)]
pub struct CommandList(ComPtr<ID3D12CommandList>);
impl_command_list!(CommandList, ID3D12CommandList, CommandList);

pub trait ICommandQueue: IPageable {
    fn begin_event(&self, metadata: u32, data: *const c_void, size: u32);
    fn copy_tile_mappings(
        &self,
        dst_resource: &Resource,
        dst_region_start_coordinate: &TiledResourceCoordinate,
        src_resource: &Resource,
        src_region_start_coordinate: &TiledResourceCoordinate,
        region_size: TileRegionSize,
        flags: Option<TileMappingFlags>,
    );
    fn end_event(&self);
    fn execute_command_lists(&self, command_lists: &[CommandList]);
    fn get_clock_calibration(&self) -> Result<GetClockCalibrationResult, HResult>;
    fn get_desc(&self) -> CommandQueueDesc<CommandListType>;
    fn get_timestamp_frequency(&self) -> Result<u64, HResult>;
    fn set_marker(&self, metadata: u32, data: *const c_void, size: u32);
    fn signal(&self, fence: &Fence, value: u64) -> Result<(), HResult>;
    fn update_tile_mappings<'a, 'b, 'c, 'd, 'e>(
        &self,
        resource: &Resource,
        resource_region_start_coordinates: Option<&'a [TiledResourceCoordinate]>,
        resource_region_sizes: Option<&'b [TileRegionSize]>,
        heap: Option<&Heap>,
        range_flags: &'c [TileRangeFlags],
        heap_range_start_offsets: Option<&'d [u32]>,
        range_tile_counts: Option<&'e [u32]>,
        flags: Option<TileMappingFlags>,
    );
    fn wait(&self, fence: &Fence, value: u64) -> Result<(), HResult>;
}
#[derive(Clone, Debug)]
pub struct CommandQueue(ComPtr<ID3D12CommandQueue>);
impl_pageable!(CommandQueue, ID3D12CommandQueue);
impl ICommandQueue for CommandQueue {
    fn begin_event(&self, metadata: u32, data: *const c_void, size: u32) {
        unsafe {
            self.0.BeginEvent(metadata, data, size);
        }
    }
    fn copy_tile_mappings(
        &self,
        dst_resource: &Resource,
        dst_region_start_coordinate: &TiledResourceCoordinate,
        src_resource: &Resource,
        src_region_start_coordinate: &TiledResourceCoordinate,
        region_size: TileRegionSize,
        flags: Option<TileMappingFlags>,
    ) {
        unsafe {
            self.0.CopyTileMappings(
                dst_resource.0.as_ptr(),
                &dst_region_start_coordinate.clone().to_c_struct(),
                src_resource.0.as_ptr(),
                &src_region_start_coordinate.clone().to_c_struct(),
                &region_size.to_c_struct(),
                flags.map_or(0, |f| f.0),
            );
        }
    }
    fn end_event(&self) {
        unsafe {
            self.0.EndEvent();
        }
    }
    fn execute_command_lists(&self, command_lists: &[CommandList]) {
        let ptrs = command_lists.iter().map(|l| l.as_ptr()).collect::<Vec<_>>();
        unsafe {
            self.0.ExecuteCommandLists(ptrs.len() as u32, ptrs.as_ptr());
        }
    }
    fn get_clock_calibration(&self) -> Result<GetClockCalibrationResult, HResult> {
        let mut values = GetClockCalibrationResult::default();
        let res = unsafe {
            self.0
                .GetClockCalibration(&mut values.gpu_timestamp, &mut values.cpu_timestamp)
        };
        hresult(values, res)
    }
    fn get_desc(&self) -> CommandQueueDesc<CommandListType> {
        unsafe { self.0.GetDesc().into() }
    }
    fn get_timestamp_frequency(&self) -> Result<u64, HResult> {
        let mut value = 0;
        let res = unsafe { self.0.GetTimestampFrequency(&mut value) };
        hresult(value, res)
    }
    fn set_marker(&self, metadata: u32, data: *const c_void, size: u32) {
        unsafe {
            self.0.SetMarker(metadata, data, size);
        }
    }
    fn signal(&self, fence: &Fence, value: u64) -> Result<(), HResult> {
        unsafe { hresult((), self.0.Signal(fence.0.as_ptr(), value)) }
    }
    fn update_tile_mappings<'a, 'b, 'c, 'd, 'e>(
        &self,
        resource: &Resource,
        resource_region_start_coordinates: Option<&'a [TiledResourceCoordinate]>,
        resource_region_sizes: Option<&'b [TileRegionSize]>,
        heap: Option<&Heap>,
        range_flags: &'c [TileRangeFlags],
        heap_range_start_offsets: Option<&'d [u32]>,
        range_tile_counts: Option<&'e [u32]>,
        flags: Option<TileMappingFlags>,
    ) {
        assert!(
            resource_region_start_coordinates.map_or(0, |rrscs| rrscs.len())
                == resource_region_sizes.map_or(0, |rrss| rrss.len())
        );
        assert!(range_flags.len() == range_tile_counts.map_or(0, |rtcs| rtcs.len()));
        let rrscs = resource_region_start_coordinates.map_or(Vec::new(), |v| {
            v.iter()
                .map(|rsc| rsc.clone().to_c_struct())
                .collect::<Vec<_>>()
        });
        unsafe {
            self.0.UpdateTileMappings(
                resource.0.as_ptr(),
                rrscs.len() as u32,
                rrscs.as_ptr(),
                resource_region_sizes.map_or(std::ptr::null(), |rrss| {
                    rrss.as_ptr() as *const D3D12_TILE_REGION_SIZE
                }),
                heap.map_or(std::ptr::null_mut(), |h| h.0.as_ptr()),
                range_flags.len() as u32,
                range_flags.as_ptr() as *const D3D12_TILE_RANGE_FLAGS,
                heap_range_start_offsets.map_or(std::ptr::null(), |hrsos| hrsos.as_ptr()),
                range_tile_counts.map_or(std::ptr::null(), |rtcs| rtcs.as_ptr()),
                flags.map_or(0, |f| f.0),
            );
        }
    }
    fn wait(&self, fence: &Fence, value: u64) -> Result<(), HResult> {
        unsafe { hresult((), self.0.Wait(fence.0.as_ptr(), value)) }
    }
}

pub trait ICommandSignature: IPageable {}
#[derive(Clone, Debug)]
pub struct CommandSignature(ComPtr<ID3D12CommandSignature>);
impl_pageable!(CommandSignature, ID3D12CommandSignature);
impl ICommandSignature for CommandSignature {}

pub trait IDescriptorHeap: IPageable {
    fn get_cpu_descriptor_handle_for_heap_start(&self) -> CPUDescriptorHandle;
    fn get_desc(&self) -> DescriptorHeapDesc<DescriptorHeapType, u32>;
    fn get_gpu_descriptor_handle_for_heap_start(&self) -> GPUDescriptorHandle;
}
#[derive(Clone, Debug)]
pub struct DescriptorHeap(ComPtr<ID3D12DescriptorHeap>);
impl_pageable!(DescriptorHeap, ID3D12DescriptorHeap);
impl IDescriptorHeap for DescriptorHeap {
    fn get_cpu_descriptor_handle_for_heap_start(&self) -> CPUDescriptorHandle {
        unsafe {
            CPUDescriptorHandle {
                ptr: self.0.GetCPUDescriptorHandleForHeapStart().ptr,
            }
        }
    }
    fn get_desc(&self) -> DescriptorHeapDesc<DescriptorHeapType, u32> {
        unsafe { self.0.GetDesc().into() }
    }
    fn get_gpu_descriptor_handle_for_heap_start(&self) -> GPUDescriptorHandle {
        unsafe {
            GPUDescriptorHandle {
                ptr: self.0.GetGPUDescriptorHandleForHeapStart().ptr,
            }
        }
    }
}

#[derive(Clone, Default, Debug)]
pub struct GetCopyableFootprintsResult {
    pub layouts: Vec<PlacedSubresourceFootprint>,
    pub num_rows: Vec<u32>,
    pub row_size_in_bytes: Vec<u64>,
    pub total_bytes: u64,
}

#[derive(Clone, Debug)]
pub struct GetResourceTilingResult {
    pub num_tiles_for_entire_resource: u32,
    pub packed_mip_desc: PackedMipInfo,
    pub standard_tile_shape_for_non_packed_mips: TileShape,
    pub subresource_tilings_for_non_packed_mips: Vec<SubresourceTiling>,
}

pub trait IDevice: IObject {
    fn check_feature_support<T: CheckFeatureSupport>(&self, args: T::Args) -> Result<T, HResult>;
    unsafe fn copy_descriptors(
        &self,
        dest_descriptor_range_starts: &[CPUDescriptorHandle],
        dest_descriptor_range_sizes: &[u32],
        src_descriptor_range_start: &[CPUDescriptorHandle],
        src_descriptor_range_sizes: &[u32],
        descriptor_heaps_type: DescriptorHeapType,
    );
    unsafe fn copy_descriptors_simple(
        &self,
        num_descriptors: u32,
        dest_descriptor_range_starts: &CPUDescriptorHandle,
        src_descriptor_range_starts: &CPUDescriptorHandle,
        descriptor_heaps_type: DescriptorHeapType,
    );
    fn create_command_allocator<T: ICommandAllocator>(
        &self,
        ty: CommandListType,
    ) -> Result<T, HResult>;
    fn create_command_list<T: ICommandList>(
        &self,
        node_mask: u32,
        ty: CommandListType,
        command_allocator: &CommandAllocator,
        initial_state: Option<&PipelineState>,
    ) -> Result<T, HResult>;
    fn create_command_queue<T: ICommandQueue>(
        &self,
        desc: &CommandQueueDesc<CommandListType>,
    ) -> Result<T, HResult>;
    fn create_command_signature<T: ICommandSignature>(
        &self,
        desc: &CommandSignatureDesc,
        root_signature: &RootSignature,
    ) -> Result<T, HResult>;
    fn create_committed_resource<T: IResource>(
        &self,
        heap_properties: &HeapProperties<HeapType>,
        heap_flags: Option<HeapFlags>,
        desc: &ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>,
        initial_resource_state: ResourceStates,
        optimized_clear_value: Option<ClearValue>,
    ) -> Result<T, HResult>;
    fn create_compute_pipeline_state<T: IPipelineState>(
        &self,
        desc: &ComputePipelineStateDesc,
    ) -> Result<T, HResult>;
    unsafe fn create_constant_buffer_view(
        &self,
        desc: &ConstantBufferViewDesc,
        dest_descriptor: CPUDescriptorHandle,
    );
    unsafe fn create_depth_stencil_view(
        &self,
        resource: &Resource,
        desc: &DepthStencilViewDesc,
        dest_descriptor: CPUDescriptorHandle,
    );
    fn create_descriptor_heap<T: IDescriptorHeap>(
        &self,
        desc: &DescriptorHeapDesc<DescriptorHeapType, u32>,
    ) -> Result<T, HResult>;
    fn create_fence<T: IFence>(
        &self,
        initial_value: u64,
        flags: Option<FenceFlags>,
    ) -> Result<T, HResult>;
    fn create_graphics_pipeline_state<T: IPipelineState>(
        &self,
        desc: &GraphicsPipelineStateDesc<InputLayoutDesc<'_>, &[dxgi::Format], dxgi::Format>,
    ) -> Result<T, HResult>;
    fn create_heap<T: IHeap>(&self, desc: HeapDesc) -> Result<T, HResult>;
    fn create_placed_resource<T: IResource>(
        &self,
        heap: &Heap,
        heap_offset: u64,
        desc: &ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>,
        initial_state: ResourceStates,
        optimized_clear_value: Option<ClearValue>,
    ) -> Result<T, HResult>;
    fn create_query_heap<T: IQueryHeap>(&self, desc: QueryHeapDesc) -> Result<T, HResult>;
    unsafe fn create_render_target_view(
        &self,
        resource: &Resource,
        desc: &RenderTargetViewDesc,
        dest_descriptor: CPUDescriptorHandle,
    );
    fn create_reserved_resource<T: IResource>(
        &self,
        desc: &ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>,
        initial_state: ResourceStates,
        optimized_clear_value: Option<ClearValue>,
    ) -> Result<T, HResult>;
    fn create_root_signature<T: IRootSignature>(
        &self,
        node_mask: u32,
        data: &[u8],
    ) -> Result<T, HResult>;
    fn create_sampler(&self, desc: &SamplerDesc<Filter>, dest_descriptor: CPUDescriptorHandle);
    unsafe fn create_shader_resource_view(
        &self,
        resource: &Resource,
        desc: &ShaderResourceViewDesc,
        dest_dsecriptor: CPUDescriptorHandle,
    );
    fn create_shared_handle<T: Interface>(
        &self,
        object: &T,
        attributes: Option<&SECURITY_ATTRIBUTES>,
        access: u32,
        name: &str,
    ) -> Result<HANDLE, HResult>;
    unsafe fn create_unordered_access_view(
        &self,
        resource: &Resource,
        counter_resource: Option<&Resource>,
        desc: &UnorderedAccessViewDesc,
        dest_descriptor: CPUDescriptorHandle,
    );
    fn evict(&self, objects: &[&impl IPageable]) -> Result<(), HResult>;
    fn get_adapter_luid(&self) -> Luid;
    fn get_copyable_footprints(
        &self,
        resource: &ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>,
        first_subresource: u32,
        num_subresources: u32,
        base_offset: u64,
    ) -> GetCopyableFootprintsResult;
    fn get_custom_heap_properties(
        &self,
        node_mask: u32,
        heap_type: HeapType,
    ) -> HeapProperties<HeapType>;
    fn get_descriptor_handle_increment_size(
        &self,
        descriptor_heaps_type: DescriptorHeapType,
    ) -> u32;
    fn get_device_removed_reason(&self) -> HResult;
    fn get_node_count(&self) -> u32;
    fn get_resource_allocation_info(
        &self,
        visible_mask: u32,
        descs: &[&ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>],
    ) -> ResourceAllocationInfo;
    fn get_resource_tiling(
        &self,
        resource: &Resource,
        num_subresource_tiling: u32,
        first_subresource_tiling_to_get: u32,
    ) -> GetResourceTilingResult;
    fn make_resident(&self, objects: &[&impl IPageable]) -> Result<(), HResult>;
    fn open_shared_handle<T: Interface>(&self, nt_handle: HANDLE) -> Result<T, HResult>;
    fn open_shared_handle_by_name(&self, name: &str, access: u32) -> Result<HANDLE, HResult>;
    fn set_stable_power_state(&self, enable: bool) -> Result<(), HResult>;
}
pub trait IDevice1: IDevice {
    fn create_pipeline_library<T: Interface>(&self, data: &[u8]) -> Result<T, HResult>;
    fn set_event_on_multiple_fence_completion(
        &self,
        fences: &[&Fence],
        fence_values: &[u64],
        flags: Option<MultipleFenceWaitFlags>,
        event: &EventHandle,
    ) -> Result<(), HResult>;
    fn set_residency_priority(
        &self,
        objects: &[&impl IPageable],
        priorites: &[ResidencyPriority],
    ) -> Result<(), HResult>;
}
pub trait IDevice2: IDevice1 {
    fn create_pipeline_state<T: Interface>(
        &self,
        desc: &PipelineStateStreamDesc,
    ) -> Result<T, HResult>;
}
macro_rules! impl_device {
    ($s: ident, $interface: ident, Device) => {
        impl_object!($s, $interface);
        impl IDevice for $s {
            fn check_feature_support<T: CheckFeatureSupport>(
                &self,
                args: T::Args,
            ) -> Result<T, HResult> {
                T::check_feature_support(self.0.as_ptr() as *mut ID3D12Device, args)
            }
            unsafe fn copy_descriptors(
                &self,
                dest_descriptor_range_starts: &[CPUDescriptorHandle],
                dest_descriptor_range_sizes: &[u32],
                src_descriptor_range_start: &[CPUDescriptorHandle],
                src_descriptor_range_sizes: &[u32],
                descriptor_heaps_type: DescriptorHeapType,
            ) {
                self.0.CopyDescriptors(
                    dest_descriptor_range_starts.len() as u32,
                    dest_descriptor_range_starts.as_ptr() as *const D3D12_CPU_DESCRIPTOR_HANDLE,
                    dest_descriptor_range_sizes.as_ptr(),
                    src_descriptor_range_start.len() as u32,
                    src_descriptor_range_start.as_ptr() as *const D3D12_CPU_DESCRIPTOR_HANDLE,
                    src_descriptor_range_sizes.as_ptr(),
                    descriptor_heaps_type as u32,
                );
            }
            unsafe fn copy_descriptors_simple(
                &self,
                num_descriptors: u32,
                dest_descriptor_range_starts: &CPUDescriptorHandle,
                src_descriptor_range_starts: &CPUDescriptorHandle,
                descriptor_heaps_type: DescriptorHeapType,
            ) {
                self.0.CopyDescriptorsSimple(
                    num_descriptors,
                    D3D12_CPU_DESCRIPTOR_HANDLE {
                        ptr: dest_descriptor_range_starts.ptr,
                    },
                    D3D12_CPU_DESCRIPTOR_HANDLE {
                        ptr: src_descriptor_range_starts.ptr,
                    },
                    descriptor_heaps_type as u32,
                );
            }
            fn create_command_allocator<T: ICommandAllocator>(
                &self,
                ty: CommandListType,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0
                            .CreateCommandAllocator(ty as u32, &T::uuidof().into(), &mut obj)
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_command_list<T: ICommandList>(
                &self,
                node_mask: u32,
                ty: CommandListType,
                command_allocator: &CommandAllocator,
                initial_state: Option<&PipelineState>,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateCommandList(
                            node_mask,
                            ty as u32,
                            command_allocator.0.as_ptr(),
                            initial_state.map_or(std::ptr::null_mut(), |s| s.0.as_ptr()),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_command_queue<T: ICommandQueue>(
                &self,
                desc: &CommandQueueDesc<CommandListType>,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateCommandQueue(
                            &desc.to_c_struct(),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_command_signature<T: ICommandSignature>(
                &self,
                desc: &CommandSignatureDesc,
                root_signature: &RootSignature,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let arg_descs = desc
                        .argument_descs
                        .iter()
                        .map(|ad| ad.to_c_struct())
                        .collect::<Vec<_>>();
                    let cs_desc = D3D12_COMMAND_SIGNATURE_DESC {
                        ByteStride: desc.byte_stride,
                        NumArgumentDescs: desc.argument_descs.len() as u32,
                        pArgumentDescs: arg_descs.as_ptr(),
                        NodeMask: desc.node_mask,
                    };
                    let res = unsafe {
                        self.0.CreateCommandSignature(
                            &cs_desc,
                            root_signature.0.as_ptr(),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_committed_resource<T: IResource>(
                &self,
                heap_properties: &HeapProperties<HeapType>,
                heap_flags: Option<HeapFlags>,
                desc: &ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>,
                initial_resource_state: ResourceStates,
                optimized_clear_value: Option<ClearValue>,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let ocv = optimized_clear_value.map(|v| v.to_c_struct());
                    let res = unsafe {
                        self.0.CreateCommittedResource(
                            &heap_properties.to_c_struct(),
                            heap_flags.map_or(0, |f| f.0),
                            &desc.to_c_struct(),
                            initial_resource_state.0,
                            ocv.as_ref().map_or(std::ptr::null(), |v| v),
                            &Resource::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_compute_pipeline_state<T: IPipelineState>(
                &self,
                desc: &ComputePipelineStateDesc,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateComputePipelineState(
                            &desc.to_c_struct(),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            unsafe fn create_constant_buffer_view(
                &self,
                desc: &ConstantBufferViewDesc,
                dest_descriptor: CPUDescriptorHandle,
            ) {
                self.0.CreateConstantBufferView(&desc.to_c_struct(), dest_descriptor.into());
            }
            unsafe fn create_depth_stencil_view(
                &self,
                resource: &Resource,
                desc: &DepthStencilViewDesc,
                dest_descriptor: CPUDescriptorHandle,
            ) {
                self.0.CreateDepthStencilView(
                    resource.0.as_ptr(),
                    &desc.to_c_struct(),
                    dest_descriptor.into(),
                );
            }
            fn create_descriptor_heap<T: IDescriptorHeap>(
                &self,
                desc: &DescriptorHeapDesc<DescriptorHeapType, u32>,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateDescriptorHeap(
                            &desc.to_c_struct(),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_fence<T: IFence>(
                &self,
                initial_value: u64,
                flags: Option<FenceFlags>,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateFence(
                            initial_value,
                            flags.map_or(0, |f| f.0),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_graphics_pipeline_state<T: IPipelineState>(
                &self,
                desc: &GraphicsPipelineStateDesc<
                    InputLayoutDesc<'_>,
                    &[dxgi::Format],
                    dxgi::Format,
                >,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let (c_descs, _tmp) = desc.to_c_struct();
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0
                            .CreateGraphicsPipelineState(&c_descs, &T::uuidof().into(), &mut obj)
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_heap<T: IHeap>(&self, desc: HeapDesc) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0
                            .CreateHeap(&desc.to_c_struct(), &T::uuidof().into(), &mut obj)
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_placed_resource<T: IResource>(
                &self,
                heap: &Heap,
                heap_offset: u64,
                desc: &ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>,
                initial_state: ResourceStates,
                optimized_clear_value: Option<ClearValue>,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let ocv = optimized_clear_value.map(|v| v.to_c_struct());
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreatePlacedResource(
                            heap.0.as_ptr(),
                            heap_offset,
                            &desc.to_c_struct(),
                            initial_state.0,
                            ocv.as_ref().map_or(std::ptr::null(), |v| v),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_query_heap<T: IQueryHeap>(&self, desc: QueryHeapDesc) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0
                            .CreateQueryHeap(&desc.to_c_struct(), &T::uuidof().into(), &mut obj)
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            unsafe fn create_render_target_view(
                &self,
                resource: &Resource,
                desc: &RenderTargetViewDesc,
                dest_descriptor: CPUDescriptorHandle,
            ) {
                    self.0.CreateRenderTargetView(
                        resource.0.as_ptr(),
                        &desc.to_c_struct(),
                        D3D12_CPU_DESCRIPTOR_HANDLE {
                            ptr: dest_descriptor.ptr,
                        },
                    );
            }
            fn create_reserved_resource<T: IResource>(
                &self,
                desc: &ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>,
                initial_state: ResourceStates,
                optimized_clear_value: Option<ClearValue>,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let ocv = optimized_clear_value.map(|v| v.to_c_struct());
                    let res = unsafe {
                        self.0.CreateReservedResource(
                            &desc.to_c_struct(),
                            initial_state.0,
                            ocv.as_ref().map_or(std::ptr::null(), |v| v),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_root_signature<T: IRootSignature>(
                &self,
                node_mask: u32,
                blob: &[u8],
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateRootSignature(
                            node_mask,
                            blob.as_ptr() as *const c_void,
                            blob.len(),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn create_sampler(
                &self,
                desc: &SamplerDesc<Filter>,
                dest_descriptor: CPUDescriptorHandle,
            ) {
                unsafe {
                    self.0.CreateSampler(
                        &desc.to_c_struct(),
                        D3D12_CPU_DESCRIPTOR_HANDLE {
                            ptr: dest_descriptor.ptr,
                        },
                    );
                }
            }
            unsafe fn create_shader_resource_view(
                &self,
                resource: &Resource,
                desc: &ShaderResourceViewDesc,
                dest_descriptor: CPUDescriptorHandle,
            ) {
                self.0.CreateShaderResourceView(
                    resource.0.as_ptr(),
                    &desc.to_c_struct(),
                    D3D12_CPU_DESCRIPTOR_HANDLE {
                        ptr: dest_descriptor.ptr,
                    },
                );
            }
            fn create_shared_handle<T: Interface>(
                &self,
                object: &T,
                attributes: Option<&SECURITY_ATTRIBUTES>,
                access: u32,
                name: &str,
            ) -> Result<HANDLE, HResult> {
                let name = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
                let mut handle = std::ptr::null_mut();
                let res = unsafe {
                    self.0.CreateSharedHandle(
                        object.as_com_ptr().as_ptr() as *mut ID3D12DeviceChild,
                        attributes.map_or(std::ptr::null(), |a| a),
                        access,
                        name.as_ptr(),
                        &mut handle,
                    )
                };
                hresult(handle, res)
            }
            unsafe fn create_unordered_access_view(
                &self,
                resource: &Resource,
                counter_resource: Option<&Resource>,
                desc: &UnorderedAccessViewDesc,
                dest_descriptor: CPUDescriptorHandle,
            ) {
                self.0.CreateUnorderedAccessView(
                    resource.as_com_ptr().as_ptr(),
                    counter_resource
                        .map_or(std::ptr::null_mut(), |cr| cr.as_com_ptr().as_ptr()),
                    &desc.to_c_struct(),
                    D3D12_CPU_DESCRIPTOR_HANDLE {
                        ptr: dest_descriptor.ptr,
                    },
                );
            }
            fn evict(&self, objects: &[&impl IPageable]) -> Result<(), HResult> {
                let mut ptrs = objects
                    .iter()
                    .map(|obj| obj.as_com_ptr().as_ptr() as *mut ID3D12Pageable)
                    .collect::<Vec<_>>();
                let res = unsafe { self.0.Evict(ptrs.len() as u32, ptrs.as_mut_ptr()) };
                hresult((), res)
            }
            fn get_adapter_luid(&self) -> Luid {
                unsafe { self.0.GetAdapterLuid().into() }
            }
            fn get_copyable_footprints(
                &self,
                resource: &ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>,
                first_subresource: u32,
                num_subresources: u32,
                base_offset: u64,
            ) -> GetCopyableFootprintsResult {
                let mut layouts = Vec::with_capacity(num_subresources as usize);
                let mut num_rows = Vec::with_capacity(num_subresources as usize);
                let mut row_size_in_bytes = Vec::with_capacity(num_subresources as usize);
                let mut total_bytes = 0;
                unsafe {
                    layouts.set_len(num_subresources as usize);
                    num_rows.set_len(num_subresources as usize);
                    row_size_in_bytes.set_len(num_subresources as usize);
                    self.0.GetCopyableFootprints(
                        &resource.to_c_struct(),
                        first_subresource,
                        num_subresources,
                        base_offset,
                        layouts.as_mut_ptr(),
                        num_rows.as_mut_ptr(),
                        row_size_in_bytes.as_mut_ptr(),
                        &mut total_bytes,
                    );
                    GetCopyableFootprintsResult {
                        layouts: layouts
                            .iter()
                            .map(|l| PlacedSubresourceFootprint {
                                offset: l.Offset,
                                footprint: SubresourceFootprint {
                                    format: std::mem::transmute(l.Footprint.Format),
                                    width: l.Footprint.Width,
                                    height: l.Footprint.Height,
                                    depth: l.Footprint.Depth,
                                    row_pitch: l.Footprint.RowPitch,
                                },
                            })
                            .collect::<Vec<_>>(),
                        num_rows,
                        row_size_in_bytes,
                        total_bytes,
                    }
                }
            }
            fn get_custom_heap_properties(
                &self,
                node_mask: u32,
                heap_type: HeapType,
            ) -> HeapProperties<HeapType> {
                unsafe {
                    let props = self.0.GetCustomHeapProperties(node_mask, heap_type as u32);
                    HeapProperties {
                        heap_type: std::mem::transmute(props.Type),
                        cpu_page_property: std::mem::transmute(props.CPUPageProperty),
                        memory_pool_preference: std::mem::transmute(props.MemoryPoolPreference),
                        creation_node_mask: props.CreationNodeMask,
                        visible_node_mask: props.VisibleNodeMask,
                    }
                }
            }
            fn get_descriptor_handle_increment_size(
                &self,
                descriptor_heap_type: DescriptorHeapType,
            ) -> u32 {
                unsafe {
                    self.0
                        .GetDescriptorHandleIncrementSize(descriptor_heap_type as u32)
                }
            }
            fn get_device_removed_reason(&self) -> HResult {
                unsafe { self.0.GetDeviceRemovedReason().into() }
            }
            fn get_node_count(&self) -> u32 {
                unsafe { self.0.GetNodeCount() }
            }
            fn get_resource_allocation_info(
                &self,
                visible_mask: u32,
                descs: &[&ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>],
            ) -> ResourceAllocationInfo {
                unsafe {
                    let c_descs = descs.iter().map(|d| d.to_c_struct()).collect::<Vec<_>>();
                    let info = self.0.GetResourceAllocationInfo(
                        visible_mask,
                        c_descs.len() as u32,
                        c_descs.as_ptr(),
                    );
                    ResourceAllocationInfo {
                        size_in_bytes: info.SizeInBytes,
                        alignment: info.Alignment,
                    }
                }
            }
            fn get_resource_tiling(
                &self,
                resource: &Resource,
                num_subresource_tiling: u32,
                first_subresource_tiling_to_get: u32,
            ) -> GetResourceTilingResult {
                let mut num_tiles = 0;
                let mut packed_mip_desc = Default::default();
                let mut tile_shape = Default::default();
                let mut num_subresource = num_subresource_tiling;
                let mut tilings = Vec::with_capacity(num_subresource as usize);
                unsafe {
                    self.0.GetResourceTiling(
                        resource.as_ptr(),
                        &mut num_tiles,
                        &mut packed_mip_desc,
                        &mut tile_shape,
                        &mut num_subresource,
                        first_subresource_tiling_to_get,
                        tilings.as_mut_ptr(),
                    );
                    tilings.set_len(num_subresource as usize);
                }
                GetResourceTilingResult {
                    num_tiles_for_entire_resource: num_tiles,
                    packed_mip_desc: packed_mip_desc.into(),
                    standard_tile_shape_for_non_packed_mips: tile_shape.into(),
                    subresource_tilings_for_non_packed_mips: tilings
                        .into_iter()
                        .map(|t| t.into())
                        .collect::<Vec<_>>(),
                }
            }
            fn make_resident(&self, objects: &[&impl IPageable]) -> Result<(), HResult> {
                let mut ptrs = objects
                    .iter()
                    .map(|p| p.as_com_ptr().as_ptr() as *mut ID3D12Pageable)
                    .collect::<Vec<_>>();
                let res = unsafe { self.0.MakeResident(ptrs.len() as u32, ptrs.as_mut_ptr()) };
                hresult((), res)
            }
            fn open_shared_handle<T: Interface>(&self, nt_handle: HANDLE) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0
                            .OpenSharedHandle(nt_handle, &T::uuidof().into(), &mut obj)
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn open_shared_handle_by_name(
                &self,
                name: &str,
                access: u32,
            ) -> Result<HANDLE, HResult> {
                let wname = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
                let mut handle = std::ptr::null_mut();
                let res = unsafe {
                    self.0
                        .OpenSharedHandleByName(wname.as_ptr(), access, &mut handle)
                };
                hresult(handle, res)
            }
            fn set_stable_power_state(&self, enable: bool) -> Result<(), HResult> {
                let res = unsafe { self.0.SetStablePowerState(to_BOOL(enable)) };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, Device1) => {
        impl_device!($s, $interface, Device);
        impl $s {
            pub fn as_device(&self) -> Device {
                Device(self.0.query_interface::<ID3D12Device>().unwrap())
            }
        }
        impl IDevice1 for $s {
            fn create_pipeline_library<T: Interface>(&self, data: &[u8]) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreatePipelineLibrary(
                            data.as_ptr() as *const c_void,
                            data.len(),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn set_event_on_multiple_fence_completion(
                &self,
                fences: &[&Fence],
                fence_values: &[u64],
                flags: Option<MultipleFenceWaitFlags>,
                event: &EventHandle,
            ) -> Result<(), HResult> {
                assert!(fences.len() == fence_values.len());
                let ptrs = fences
                    .iter()
                    .map(|p| p.as_com_ptr().as_ptr())
                    .collect::<Vec<_>>();
                let res = unsafe {
                    self.0.SetEventOnMultipleFenceCompletion(
                        ptrs.as_ptr(),
                        fence_values.as_ptr(),
                        ptrs.len() as u32,
                        flags.map_or(0, |f| f.0),
                        event.as_raw_handle(),
                    )
                };
                hresult((), res)
            }
            fn set_residency_priority(
                &self,
                objects: &[&impl IPageable],
                priorites: &[ResidencyPriority],
            ) -> Result<(), HResult> {
                assert!(objects.len() == priorites.len());
                let ptrs = objects
                    .iter()
                    .map(|p| p.as_com_ptr().as_ptr() as *mut ID3D12Pageable)
                    .collect::<Vec<_>>();
                let prios = priorites
                    .iter()
                    .map(|&prio| prio as u32)
                    .collect::<Vec<_>>();
                let res = unsafe {
                    self.0
                        .SetResidencyPriority(ptrs.len() as u32, ptrs.as_ptr(), prios.as_ptr())
                };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, Device2) => {
        impl_device!($s, $interface, Device1);
        impl $s {
            pub fn as_device1(&self) -> Device1 {
                Device1(self.0.query_interface::<ID3D12Device1>().unwrap())
            }
        }
        impl IDevice2 for $s {
            fn create_pipeline_state<T: Interface>(
                &self,
                desc: &PipelineStateStreamDesc,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreatePipelineState(
                            &desc.to_c_struct(),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
        }
    };
}
#[derive(Clone, Debug)]
pub struct Device(ComPtr<ID3D12Device>);
#[derive(Clone, Debug)]
pub struct Device1(ComPtr<ID3D12Device1>);
#[derive(Clone, Debug)]
pub struct Device2(ComPtr<ID3D12Device2>);
impl_device!(Device, ID3D12Device, Device);
impl_device!(Device1, ID3D12Device1, Device1);
impl_device!(Device2, ID3D12Device2, Device2);

// pub trait IDeviceRemovedExtendedData;
// pub trait IDeviceRemovedExtendedDataSettings;

pub trait IFence: IPageable {
    fn get_completed_value(&self) -> u64;
    fn set_event_on_completion(&self, value: u64, event: &EventHandle) -> Result<(), HResult>;
    fn signal(&self, value: u64) -> Result<(), HResult>;
}
pub trait IFence1: IFence {
    fn get_creation_flags(&self) -> FenceFlags;
}
macro_rules! impl_fence {
    ($s: ident, $interface: ident, Fence) => {
        impl_pageable!($s, $interface);
        impl IFence for $s {
            fn get_completed_value(&self) -> u64 {
                unsafe { self.0.GetCompletedValue() }
            }
            fn set_event_on_completion(
                &self,
                value: u64,
                event: &EventHandle,
            ) -> Result<(), HResult> {
                let res = unsafe { self.0.SetEventOnCompletion(value, event.as_raw_handle()) };
                hresult((), res)
            }
            fn signal(&self, value: u64) -> Result<(), HResult> {
                let res = unsafe { self.0.Signal(value) };
                hresult((), res)
            }
        }
    };
    ($s: ident, $interface: ident, Fence1) => {
        impl_fence!($s, $interface, Fence);
        impl IFence1 for $s {
            fn get_creation_flags(&self) -> FenceFlags {
                unsafe { FenceFlags(self.0.GetCreationFlags()) }
            }
        }
    };
}
#[derive(Clone, Debug)]
pub struct Fence(ComPtr<ID3D12Fence>);
// #[derive(Clone, Debug)]
// pub struct Fence1(ComPtr<ID3D12Fence1>);
impl_fence!(Fence, ID3D12Fence, Fence);
// impl_fence!(Fence1, ID3D12Fence1, Fence1);

pub trait IGraphicsCommandList: ICommandList {
    fn begin_event(&self, metadata: u32, data: *const c_void, size: u32);
    fn begin_query(&self, query_heap: &QueryHeap, ty: QueryType, index: u32);
    fn clear_depth_stencil_view(
        &self,
        dsv: CPUDescriptorHandle,
        clear_flags: ClearFlags,
        depth: f32,
        stencil: u8,
        rects: Option<&[Rect]>,
    );
    fn clear_render_target_view(
        &self,
        rtv: CPUDescriptorHandle,
        clear_rgba: dxgi::RGBA,
        rects: Option<&[Rect]>,
    );
    fn clear_state(&self, pipeline_state: &PipelineState);
    fn clear_unordered_access_view_float(
        &self,
        view_gpu_handle_in_current_heap: GPUDescriptorHandle,
        view_cpu_handle: CPUDescriptorHandle,
        resource: &Resource,
        values: [f32; 4],
        rects: &[Rect],
    );
    fn clear_unordered_access_view_uint(
        &self,
        view_gpu_handle_in_current_heap: GPUDescriptorHandle,
        view_cpu_handle: CPUDescriptorHandle,
        resource: &Resource,
        values: [u32; 4],
        rects: &[Rect],
    );
    fn close(&self) -> HResult;
    fn copy_buffer_region(
        &self,
        dst_buffer: &Resource,
        dst_offset: u64,
        src_buffer: &Resource,
        src_offset: u64,
        num_bytes: u64,
    );
    fn copy_resource(&self, dst_resource: &Resource, src_resource: &Resource);
    fn copy_texture_region(
        &self,
        dst: &TextureCopyLocation<impl IResource>,
        dst_x: u32,
        dst_y: u32,
        dst_z: u32,
        src: &TextureCopyLocation<impl IResource>,
        src_box: Option<Box3D>,
    );
    fn copy_tiles(
        &self,
        tiled_resource: &Resource,
        tile_region_start_coordinate: &TiledResourceCoordinate,
        tile_region_size: &TileRegionSize,
        buffer: &Resource,
        buffer_start_offset_in_bytes: u64,
        flags: Option<TileCopyFlags>,
    );
    fn discard_resouce(&self, resource: &Resource, region: DiscardRegion);
    fn dispatch(
        &self,
        thread_group_count_x: u32,
        thread_group_count_y: u32,
        thread_group_count_z: u32,
    );
    fn draw_indexed_instanced(
        &self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    );
    fn draw_instanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    );
    fn end_event(&self);
    fn end_query(&self, query_heap: &QueryHeap, ty: QueryType, index: u32);
    fn execute_bundle(&self, command_list: &GraphicsCommandList);
    fn execute_indirect(
        &self,
        command_signature: &CommandSignature,
        max_command_count: u32,
        argument_buffer: &Resource,
        argument_buffer_offset: u64,
        count_buffer: Option<&Resource>,
        count_buffer_offset: u64,
    );
    fn ia_set_index_buffer(&self, view: &IndexBufferView);
    fn ia_set_primitive_topology(&self, primitive_topology: d3d::PrimitiveTopology);
    fn ia_set_vertex_buffers(&self, start_slot: u32, views: &[VertexBufferView]);
    fn om_set_blend_factor(&self, blend_factor: dxgi::RGBA);
    fn om_set_render_targets(
        &self,
        render_target_descriptors: &[CPUDescriptorHandle],
        rts_single_handle_to_descriptor_range: bool,
        depth_stencil_descriptor: Option<CPUDescriptorHandle>,
    );
    fn om_set_stencil_ref(&self, stencil_ref: u32);
    fn reset(
        &self,
        command_allocator: &CommandAllocator,
        pipeline_state: Option<&PipelineState>,
    ) -> Result<(), HResult>;
    fn resolve_query_data(
        &self,
        query_heap: &QueryHeap,
        ty: QueryType,
        start_index: u32,
        num_queries: u32,
        dst_buffer: &Resource,
        aligned_dst_bufer_offset: u64,
    );
    fn resolve_subresource(
        &self,
        dst_resource: &Resource,
        dst_subresource: u32,
        src_resource: &Resource,
        src_subresource: u32,
        format: dxgi::Format,
    );
    fn resource_barrier(&self, barriers: &[ResourceBarrier<impl IResource>]);
    fn rs_set_scissor_rects(&self, rects: &[Rect]);
    fn rs_set_viewports(&self, viewports: &[Viewport]);
    fn set_compute_root_32bit_constant(
        &self,
        root_parameter_index: u32,
        src_data: u32,
        dest_offset_in_32bit_value: u32,
    );
    fn set_compute_root_32bit_constants<T>(
        &self,
        root_parameter_index: u32,
        src_data: &[T],
        dest_offset_in_32bit_value: u32,
    );
    fn set_compute_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUVirtualAddress,
    );
    fn set_compute_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GPUDescriptorHandle,
    );
    fn set_compute_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUVirtualAddress,
    );
    fn set_compute_root_signature(&self, root_signature: &RootSignature);
    fn set_compute_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUVirtualAddress,
    );
    fn set_descriptor_heaps(&self, descriptor_heaps: &[&DescriptorHeap]);
    fn set_graphics_root_32bit_constant(
        &self,
        root_parameter_index: u32,
        src_data: u32,
        dest_offset_in_32bit_values: u32,
    );
    fn set_graphics_root_32bit_constants<T>(
        &self,
        root_parameter_index: u32,
        src_data: &[T],
        dest_offset_in_32bit_values: u32,
    );
    fn set_graphics_root_constant_buffer_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUVirtualAddress,
    );
    fn set_graphics_root_descriptor_table(
        &self,
        root_parameter_index: u32,
        base_descriptor: GPUDescriptorHandle,
    );
    fn set_graphics_root_shader_resource_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUVirtualAddress,
    );
    fn set_graphics_root_signature(&self, root_signature: &RootSignature);
    fn set_graphics_root_unordered_access_view(
        &self,
        root_parameter_index: u32,
        buffer_location: GPUVirtualAddress,
    );
    fn set_marker(&self, metadata: u32, data: *const c_void, size: u32);
    fn set_pipeline_state(&self, pipeline_state: &PipelineState);
    fn set_predication(
        &self,
        buffer: &Resource,
        aligned_buffer_offset: u64,
        operation: PredicationOp,
    );
    fn so_set_targets(&self, start_slot: u32, views: &[StreamOutputBufferView]);
}
macro_rules! impl_graphics_command_list {
    ($s: ident, $interface: ident, GraphicsCommandList) => {
        impl_command_list!($s, $interface, CommandList);
        impl $s {
            pub fn as_command_list(&self) -> CommandList {
                CommandList(
                    self.0
                        .query_interface::<<CommandList as Interface>::APIType>()
                        .unwrap(),
                )
            }
        }
        impl IGraphicsCommandList for $s {
            fn begin_event(&self, metadata: u32, data: *const c_void, size: u32) {
                unsafe { self.0.BeginEvent(metadata, data, size) }
            }
            fn begin_query(&self, query_heap: &QueryHeap, ty: QueryType, index: u32) {
                unsafe { self.0.BeginQuery(query_heap.as_ptr(), ty as u32, index) }
            }
            fn clear_depth_stencil_view(
                &self,
                dsv: CPUDescriptorHandle,
                clear_flags: ClearFlags,
                depth: f32,
                stencil: u8,
                rects: Option<&[Rect]>,
            ) {
                unsafe {
                    self.0.ClearDepthStencilView(
                        dsv.into(),
                        clear_flags.0,
                        depth,
                        stencil,
                        rects.map_or(0, |rcs| rcs.len() as u32),
                        rects.map_or(std::ptr::null(), |rcs| rcs.as_ptr() as *const RECT),
                    )
                }
            }
            fn clear_render_target_view(
                &self,
                rtv: CPUDescriptorHandle,
                clear_rgba: dxgi::RGBA,
                rects: Option<&[Rect]>,
            ) {
                unsafe {
                    self.0.ClearRenderTargetView(
                        rtv.into(),
                        &[clear_rgba.r, clear_rgba.g, clear_rgba.b, clear_rgba.a],
                        rects.map_or(0, |rcs| rcs.len() as u32),
                        rects.map_or(std::ptr::null(), |rcs| rcs.as_ptr() as *const RECT),
                    )
                }
            }
            fn clear_state(&self, pipeline_state: &PipelineState) {
                unsafe { self.0.ClearState(pipeline_state.as_ptr()) }
            }
            fn clear_unordered_access_view_float(
                &self,
                view_gpu_handle_in_current_heap: GPUDescriptorHandle,
                view_cpu_handle: CPUDescriptorHandle,
                resource: &Resource,
                values: [f32; 4],
                rects: &[Rect],
            ) {
                unsafe {
                    self.0.ClearUnorderedAccessViewFloat(
                        view_gpu_handle_in_current_heap.into(),
                        view_cpu_handle.into(),
                        resource.as_ptr(),
                        &values,
                        rects.len() as u32,
                        rects.as_ptr() as *const RECT,
                    )
                }
            }
            fn clear_unordered_access_view_uint(
                &self,
                view_gpu_handle_in_current_heap: GPUDescriptorHandle,
                view_cpu_handle: CPUDescriptorHandle,
                resource: &Resource,
                values: [u32; 4],
                rects: &[Rect],
            ) {
                unsafe {
                    self.0.ClearUnorderedAccessViewUint(
                        view_gpu_handle_in_current_heap.into(),
                        view_cpu_handle.into(),
                        resource.as_ptr(),
                        &values,
                        rects.len() as u32,
                        rects.as_ptr() as *const RECT,
                    )
                }
            }
            fn close(&self) -> HResult {
                unsafe { self.0.Close().into() }
            }
            fn copy_buffer_region(
                &self,
                dst_buffer: &Resource,
                dst_offset: u64,
                src_buffer: &Resource,
                src_offset: u64,
                num_bytes: u64,
            ) {
                unsafe {
                    self.0.CopyBufferRegion(
                        dst_buffer.as_ptr(),
                        dst_offset,
                        src_buffer.as_ptr(),
                        src_offset,
                        num_bytes,
                    )
                }
            }
            fn copy_resource(&self, dst_resource: &Resource, src_resource: &Resource) {
                unsafe {
                    self.0
                        .CopyResource(dst_resource.as_ptr(), src_resource.as_ptr())
                }
            }
            fn copy_texture_region(
                &self,
                dst: &TextureCopyLocation<impl IResource>,
                dst_x: u32,
                dst_y: u32,
                dst_z: u32,
                src: &TextureCopyLocation<impl IResource>,
                src_box: Option<Box3D>,
            ) {
                unsafe {
                    self.0.CopyTextureRegion(
                        &dst.to_c_struct(),
                        dst_x,
                        dst_y,
                        dst_z,
                        &src.to_c_struct(),
                        src_box
                            .as_ref()
                            .map_or(std::ptr::null(), |b| b as *const Box3D as *const D3D12_BOX),
                    )
                }
            }
            fn copy_tiles(
                &self,
                tiled_resource: &Resource,
                tile_region_start_coordinate: &TiledResourceCoordinate,
                tile_region_size: &TileRegionSize,
                buffer: &Resource,
                buffer_start_offset_in_bytes: u64,
                flags: Option<TileCopyFlags>,
            ) {
                unsafe {
                    self.0.CopyTiles(
                        tiled_resource.as_ptr(),
                        &tile_region_start_coordinate.to_c_struct(),
                        &tile_region_size.to_c_struct(),
                        buffer.as_ptr(),
                        buffer_start_offset_in_bytes,
                        flags.map_or(0, |f| f.0),
                    );
                }
            }
            fn discard_resouce(&self, resource: &Resource, region: DiscardRegion) {
                unsafe {
                    self.0
                        .DiscardResource(resource.as_ptr(), &region.to_c_struct())
                }
            }
            fn dispatch(
                &self,
                thread_group_count_x: u32,
                thread_group_count_y: u32,
                thread_group_count_z: u32,
            ) {
                unsafe {
                    self.0.Dispatch(
                        thread_group_count_x,
                        thread_group_count_y,
                        thread_group_count_z,
                    )
                }
            }
            fn draw_indexed_instanced(
                &self,
                index_count_per_instance: u32,
                instance_count: u32,
                start_index_location: u32,
                base_vertex_location: i32,
                start_instance_location: u32,
            ) {
                unsafe {
                    self.0.DrawIndexedInstanced(
                        index_count_per_instance,
                        instance_count,
                        start_index_location,
                        base_vertex_location,
                        start_instance_location,
                    )
                }
            }
            fn draw_instanced(
                &self,
                vertex_count_per_instance: u32,
                instance_count: u32,
                start_vertex_location: u32,
                start_instance_location: u32,
            ) {
                unsafe {
                    self.0.DrawInstanced(
                        vertex_count_per_instance,
                        instance_count,
                        start_vertex_location,
                        start_instance_location,
                    )
                }
            }
            fn end_event(&self) {
                unsafe { self.0.EndEvent() }
            }
            fn end_query(&self, query_heap: &QueryHeap, ty: QueryType, index: u32) {
                unsafe { self.0.EndQuery(query_heap.as_ptr(), ty as u32, index) }
            }
            fn execute_bundle(&self, command_list: &GraphicsCommandList) {
                unsafe { self.0.ExecuteBundle(command_list.as_ptr()) }
            }
            fn execute_indirect(
                &self,
                command_signature: &CommandSignature,
                max_command_count: u32,
                argument_buffer: &Resource,
                argument_buffer_offset: u64,
                count_buffer: Option<&Resource>,
                count_buffer_offset: u64,
            ) {
                unsafe {
                    self.0.ExecuteIndirect(
                        command_signature.as_ptr(),
                        max_command_count,
                        argument_buffer.as_ptr(),
                        argument_buffer_offset,
                        count_buffer
                            .as_ref()
                            .map_or(std::ptr::null_mut(), |cb| cb.as_ptr()),
                        count_buffer_offset,
                    )
                }
            }
            fn ia_set_index_buffer(&self, view: &IndexBufferView) {
                unsafe {
                    self.0.IASetIndexBuffer(
                        view as *const IndexBufferView as *const D3D12_INDEX_BUFFER_VIEW,
                    )
                }
            }
            fn ia_set_primitive_topology(&self, primitive_topology: d3d::PrimitiveTopology) {
                unsafe { self.0.IASetPrimitiveTopology(primitive_topology as u32) }
            }
            fn ia_set_vertex_buffers(&self, start_slot: u32, views: &[VertexBufferView]) {
                unsafe {
                    self.0.IASetVertexBuffers(
                        start_slot,
                        views.len() as u32,
                        views.as_ptr() as *const D3D12_VERTEX_BUFFER_VIEW,
                    )
                }
            }
            fn om_set_blend_factor(&self, blend_factor: dxgi::RGBA) {
                unsafe {
                    self.0.OMSetBlendFactor(&[
                        blend_factor.r,
                        blend_factor.g,
                        blend_factor.b,
                        blend_factor.a,
                    ])
                }
            }
            fn om_set_render_targets(
                &self,
                render_target_descriptors: &[CPUDescriptorHandle],
                rts_single_handle_to_descriptor_range: bool,
                depth_stencil_descriptor: Option<CPUDescriptorHandle>,
            ) {
                unsafe {
                    self.0.OMSetRenderTargets(
                        render_target_descriptors.len() as u32,
                        render_target_descriptors.as_ptr() as *const D3D12_CPU_DESCRIPTOR_HANDLE,
                        to_BOOL(rts_single_handle_to_descriptor_range),
                        depth_stencil_descriptor
                            .as_ref()
                            .map_or(std::ptr::null(), |d| {
                                d as *const CPUDescriptorHandle
                                    as *const D3D12_CPU_DESCRIPTOR_HANDLE
                            }),
                    )
                }
            }
            fn om_set_stencil_ref(&self, stencil_ref: u32) {
                unsafe { self.0.OMSetStencilRef(stencil_ref) }
            }
            fn reset(
                &self,
                command_allocator: &CommandAllocator,
                pipeline_state: Option<&PipelineState>,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0.Reset(
                            command_allocator.as_ptr(),
                            pipeline_state.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
                        ),
                    )
                }
            }
            fn resolve_query_data(
                &self,
                query_heap: &QueryHeap,
                ty: QueryType,
                start_index: u32,
                num_queries: u32,
                dst_buffer: &Resource,
                aligned_dst_bufer_offset: u64,
            ) {
                unsafe {
                    self.0.ResolveQueryData(
                        query_heap.as_ptr(),
                        ty as u32,
                        start_index,
                        num_queries,
                        dst_buffer.as_ptr(),
                        aligned_dst_bufer_offset,
                    )
                }
            }
            fn resolve_subresource(
                &self,
                dst_resource: &Resource,
                dst_subresource: u32,
                src_resource: &Resource,
                src_subresource: u32,
                format: dxgi::Format,
            ) {
                unsafe {
                    self.0.ResolveSubresource(
                        dst_resource.as_ptr(),
                        dst_subresource,
                        src_resource.as_ptr(),
                        src_subresource,
                        format as u32,
                    )
                }
            }
            fn resource_barrier(&self, barriers: &[ResourceBarrier<impl IResource>]) {
                let c_barriers = barriers.iter().map(|b| b.to_c_struct()).collect::<Vec<_>>();
                unsafe {
                    self.0.ResourceBarrier(
                        c_barriers.len() as u32,
                        c_barriers.as_ptr() as *const D3D12_RESOURCE_BARRIER,
                    )
                }
            }
            fn rs_set_scissor_rects(&self, rects: &[Rect]) {
                unsafe {
                    self.0
                        .RSSetScissorRects(rects.len() as u32, rects.as_ptr() as *const RECT)
                }
            }
            fn rs_set_viewports(&self, viewports: &[Viewport]) {
                unsafe {
                    self.0.RSSetViewports(
                        viewports.len() as u32,
                        viewports.as_ptr() as *const D3D12_VIEWPORT,
                    )
                }
            }
            fn set_compute_root_32bit_constant(
                &self,
                root_parameter_index: u32,
                src_data: u32,
                dest_offset_in_32bit_value: u32,
            ) {
                unsafe {
                    self.0.SetComputeRoot32BitConstant(
                        root_parameter_index,
                        src_data,
                        dest_offset_in_32bit_value,
                    )
                }
            }
            fn set_compute_root_32bit_constants<T>(
                &self,
                root_parameter_index: u32,
                src_data: &[T],
                dest_offset_in_32bit_values: u32,
            ) {
                unsafe {
                    self.0.SetComputeRoot32BitConstants(
                        root_parameter_index,
                        src_data.len() as u32,
                        src_data.as_ptr() as *const c_void,
                        dest_offset_in_32bit_values,
                    )
                }
            }
            fn set_compute_root_constant_buffer_view(
                &self,
                root_parameter_index: u32,
                buffer_location: GPUVirtualAddress,
            ) {
                unsafe {
                    self.0.SetComputeRootConstantBufferView(
                        root_parameter_index,
                        buffer_location.into(),
                    )
                }
            }
            fn set_compute_root_descriptor_table(
                &self,
                root_parameter_index: u32,
                base_descriptor: GPUDescriptorHandle,
            ) {
                unsafe {
                    self.0
                        .SetComputeRootDescriptorTable(root_parameter_index, base_descriptor.into())
                }
            }
            fn set_compute_root_shader_resource_view(
                &self,
                root_parameter_index: u32,
                buffer_location: GPUVirtualAddress,
            ) {
                unsafe {
                    self.0.SetComputeRootShaderResourceView(
                        root_parameter_index,
                        buffer_location.into(),
                    )
                }
            }
            fn set_compute_root_signature(&self, root_signature: &RootSignature) {
                unsafe { self.0.SetComputeRootSignature(root_signature.as_ptr()) }
            }
            fn set_compute_root_unordered_access_view(
                &self,
                root_parameter_index: u32,
                buffer_location: GPUVirtualAddress,
            ) {
                unsafe {
                    self.0.SetComputeRootUnorderedAccessView(
                        root_parameter_index,
                        buffer_location.into(),
                    )
                }
            }
            fn set_descriptor_heaps(&self, descriptor_heaps: &[&DescriptorHeap]) {
                let mut heap_ptrs = descriptor_heaps
                    .iter()
                    .map(|h| h.as_ptr())
                    .collect::<Vec<_>>();
                unsafe {
                    self.0
                        .SetDescriptorHeaps(heap_ptrs.len() as u32, heap_ptrs.as_mut_ptr())
                }
            }
            fn set_graphics_root_32bit_constant(
                &self,
                root_parameter_index: u32,
                src_data: u32,
                dest_offset_in_32bit_value: u32,
            ) {
                unsafe {
                    self.0.SetGraphicsRoot32BitConstant(
                        root_parameter_index,
                        src_data,
                        dest_offset_in_32bit_value,
                    )
                }
            }
            fn set_graphics_root_32bit_constants<T>(
                &self,
                root_parameter_index: u32,
                src_data: &[T],
                dest_offset_in_32bit_values: u32,
            ) {
                unsafe {
                    self.0.SetGraphicsRoot32BitConstants(
                        root_parameter_index,
                        src_data.len() as u32,
                        src_data.as_ptr() as *const c_void,
                        dest_offset_in_32bit_values,
                    )
                }
            }
            fn set_graphics_root_constant_buffer_view(
                &self,
                root_parameter_index: u32,
                buffer_location: GPUVirtualAddress,
            ) {
                unsafe {
                    self.0.SetGraphicsRootConstantBufferView(
                        root_parameter_index,
                        buffer_location.into(),
                    )
                }
            }
            fn set_graphics_root_descriptor_table(
                &self,
                root_parameter_index: u32,
                base_descriptor: GPUDescriptorHandle,
            ) {
                unsafe {
                    self.0.SetGraphicsRootDescriptorTable(
                        root_parameter_index,
                        base_descriptor.into(),
                    )
                }
            }
            fn set_graphics_root_shader_resource_view(
                &self,
                root_parameter_index: u32,
                buffer_location: GPUVirtualAddress,
            ) {
                unsafe {
                    self.0.SetGraphicsRootShaderResourceView(
                        root_parameter_index,
                        buffer_location.into(),
                    )
                }
            }
            fn set_graphics_root_signature(&self, root_signature: &RootSignature) {
                unsafe { self.0.SetGraphicsRootSignature(root_signature.as_ptr()) }
            }
            fn set_graphics_root_unordered_access_view(
                &self,
                root_parameter_index: u32,
                buffer_location: GPUVirtualAddress,
            ) {
                unsafe {
                    self.0.SetGraphicsRootUnorderedAccessView(
                        root_parameter_index,
                        buffer_location.into(),
                    )
                }
            }
            fn set_marker(&self, metadata: u32, data: *const c_void, size: u32) {
                unsafe { self.0.SetMarker(metadata, data, size) }
            }
            fn set_pipeline_state(&self, pipeline_state: &PipelineState) {
                unsafe { self.0.SetPipelineState(pipeline_state.as_ptr()) }
            }
            fn set_predication(
                &self,
                resource: &Resource,
                aligned_buffer_offset: u64,
                operation: PredicationOp,
            ) {
                unsafe {
                    self.0.SetPredication(
                        resource.as_ptr(),
                        aligned_buffer_offset,
                        operation as u32,
                    )
                }
            }
            fn so_set_targets(&self, start_slot: u32, views: &[StreamOutputBufferView]) {
                unsafe {
                    self.0.SOSetTargets(
                        start_slot,
                        views.len() as u32,
                        views.as_ptr() as *mut D3D12_STREAM_OUTPUT_BUFFER_VIEW,
                    )
                }
            }
        }
    };
}
#[derive(Clone, Debug)]
pub struct GraphicsCommandList(ComPtr<ID3D12GraphicsCommandList>);
impl_graphics_command_list!(
    GraphicsCommandList,
    ID3D12GraphicsCommandList,
    GraphicsCommandList
);

pub trait IHeap: IPageable {
    fn get_desc(&self) -> HeapDesc;
}
#[derive(Clone, Debug)]
pub struct Heap(ComPtr<ID3D12Heap>);
impl_pageable!(Heap, ID3D12Heap);
impl IHeap for Heap {
    fn get_desc(&self) -> HeapDesc {
        unsafe { self.0.GetDesc().into() }
    }
}

pub trait IPipelineLibrary: IDeviceChild {
    fn get_serialized_size(&self) -> usize;
    fn load_compute_pipeline<T: IPipelineState>(
        &self,
        name: &str,
        desc: &ComputePipelineStateDesc,
    ) -> Result<T, HResult>;
    fn load_graphics_pipeline<T: IPipelineState>(
        &self,
        name: &str,
        desc: &GraphicsPipelineStateDesc<InputLayoutDesc<'_>, &[dxgi::Format], dxgi::Format>,
    ) -> Result<T, HResult>;
    fn serialize(&self) -> Result<Vec<u8>, HResult>;
    fn store_pipeline(&self, name: &str, pipeline: &PipelineState) -> Result<(), HResult>;
}
macro_rules! impl_pipeline_library {
    ($s: ident, $interface: ident, PipelineLibrary) => {
        impl_devicechild!($s, $interface);
        impl IPipelineLibrary for PipelineLibrary {
            fn get_serialized_size(&self) -> usize {
                unsafe { self.0.GetSerializedSize() }
            }
            fn load_compute_pipeline<T: IPipelineState>(
                &self,
                name: &str,
                desc: &ComputePipelineStateDesc,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let wname = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.LoadComputePipeline(
                            wname.as_ptr(),
                            &desc.to_c_struct(),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn load_graphics_pipeline<T: IPipelineState>(
                &self,
                name: &str,
                desc: &GraphicsPipelineStateDesc<
                    InputLayoutDesc<'_>,
                    &[dxgi::Format],
                    dxgi::Format,
                >,
            ) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let wname = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
                    let (c_desc, _tmp) = desc.to_c_struct();
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.LoadGraphicsPipeline(
                            wname.as_ptr(),
                            &c_desc,
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn serialize(&self) -> Result<Vec<u8>, HResult> {
                let size = self.get_serialized_size();
                let mut v = Vec::with_capacity(size);
                unsafe {
                    v.set_len(size);
                    let res = self.0.Serialize(v.as_mut_ptr() as *mut c_void, size);
                    hresult(v, res)
                }
            }
            fn store_pipeline(&self, name: &str, pipeline: &PipelineState) -> Result<(), HResult> {
                let wname = name.encode_utf16().chain(Some(0)).collect::<Vec<_>>();
                let res = unsafe { self.0.StorePipeline(wname.as_ptr(), pipeline.as_ptr()) };
                hresult((), res)
            }
        }
    };
}
#[derive(Clone, Debug)]
pub struct PipelineLibrary(ComPtr<ID3D12PipelineLibrary>);
impl_pipeline_library!(PipelineLibrary, ID3D12PipelineLibrary, PipelineLibrary);

pub trait IPipelineState: IPageable {
    fn get_cached_blob(&self) -> Result<d3d::Blob, HResult>;
}
#[derive(Clone, Debug)]
pub struct PipelineState(ComPtr<ID3D12PipelineState>);
impl_pageable!(PipelineState, ID3D12PipelineState);
impl IPipelineState for PipelineState {
    fn get_cached_blob(&self) -> Result<d3d::Blob, HResult> {
        Ok(d3d::Blob::new(ComPtr::new(|| {
            let mut obj = std::ptr::null_mut();
            let res = unsafe { self.0.GetCachedBlob(&mut obj) };
            hresult(obj, res)
        })?))
    }
}

pub trait IQueryHeap: IPageable {}
#[derive(Clone, Debug)]
pub struct QueryHeap(ComPtr<ID3D12QueryHeap>);
impl_pageable!(QueryHeap, ID3D12QueryHeap);
impl IQueryHeap for QueryHeap {}

pub struct MappedResourceData<'a, T: IResource> {
    resource: T,
    subresource: u32,
    range: Option<Range>,
    data: &'a mut [u8],
}
impl<'a, T: IResource> MappedResourceData<'a, T> {
    pub fn as_slice(&self) -> &[u8] {
        self.data
    }
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.data
    }
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }
    pub fn as_mut_ptr(&mut self) -> *mut u8 {
        self.data.as_mut_ptr()
    }
    // pub fn read_from_subresource();
    // pub fn write_from_subresource();
}
impl<'a, T: IResource> Drop for MappedResourceData<'a, T> {
    fn drop(&mut self) {
        unsafe {
            (*(self.resource.as_ptr() as *mut ID3D12Resource)).Unmap(
                self.subresource,
                self.range.map_or(std::ptr::null(), |r| {
                    &r as *const Range as *const D3D12_RANGE
                }),
            )
        }
    }
}

pub trait IResource: IPageable
where
    Self: Sized,
{
    fn get_desc(&self) -> ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout>;
    fn get_gpu_virtual_address(&self) -> GPUVirtualAddress;
    fn get_heap_properties(&self) -> Result<(HeapProperties<HeapType>, HeapFlags), HResult>;
    fn map<'a, 'b>(
        &self,
        subresource: u32,
        range: Option<Range>,
    ) -> Result<MappedResourceData<'a, Self>, HResult>;
}
#[derive(Clone, Debug)]
pub struct Resource(ComPtr<ID3D12Resource>);
impl_pageable!(Resource, ID3D12Resource);
impl IResource for Resource {
    fn get_desc(&self) -> ResourceDesc<ResourceDimension, u64, u32, dxgi::Format, TextureLayout> {
        unsafe { self.0.GetDesc().into() }
    }
    fn get_gpu_virtual_address(&self) -> GPUVirtualAddress {
        unsafe { self.0.GetGPUVirtualAddress().into() }
    }
    fn get_heap_properties(&self) -> Result<(HeapProperties<HeapType>, HeapFlags), HResult> {
        let mut properties = D3D12_HEAP_PROPERTIES::default();
        let mut flags = 0;
        let res = unsafe { self.0.GetHeapProperties(&mut properties, &mut flags) };
        hresult((properties.into(), HeapFlags(flags)), res)
    }
    fn map<'a, 'b>(
        &self,
        subresource: u32,
        range: Option<Range>,
    ) -> Result<MappedResourceData<'a, Self>, HResult> {
        let desc = self.get_desc();
        let mut p = std::ptr::null_mut();
        let res = unsafe {
            self.0.Map(
                subresource,
                range.map_or(std::ptr::null(), |r| {
                    &r as *const Range as *const D3D12_RANGE
                }),
                &mut p,
            )
        };
        let size = range.map_or(
            desc.width as usize * desc.height as usize * desc.depth_or_array_size as usize,
            |r| r.end - r.begin,
        );
        hresult(
            MappedResourceData {
                resource: self.clone(),
                subresource,
                range,
                data: unsafe { std::slice::from_raw_parts_mut(p as *mut u8, size) },
            },
            res,
        )
    }
}

pub trait IRootSignature: IDeviceChild {}
#[derive(Clone, Debug)]
pub struct RootSignature(ComPtr<ID3D12RootSignature>);
impl_devicechild!(RootSignature, ID3D12RootSignature);
impl IRootSignature for RootSignature {}

pub trait IRootSignatureDeserializer: Interface {
    fn get_root_signature_desc(&self) -> RootSignatureDesc;
}
#[derive(Clone, Debug)]
pub struct RootSignatureDeserializer(ComPtr<ID3D12RootSignatureDeserializer>);
impl_interface!(RootSignatureDeserializer, ID3D12RootSignatureDeserializer);
impl IRootSignatureDeserializer for RootSignatureDeserializer {
    fn get_root_signature_desc(&self) -> RootSignatureDesc {
        unsafe { (*self.0.GetRootSignatureDesc()).into() }
    }
}

pub trait IVersionedRootSignatureDeserializer {
    fn get_root_signature_desc_at_version(
        &self,
        convert_to_version: d3d::RootSignatureVersion,
    ) -> Result<VersionedRootSignatureDesc, HResult>;
    fn get_unconverted_root_signature_desc(&self) -> VersionedRootSignatureDesc;
}
#[derive(Clone, Debug)]
pub struct VersionedRootSignatureDeserializer(ComPtr<ID3D12VersionedRootSignatureDeserializer>);
impl_interface!(
    VersionedRootSignatureDeserializer,
    ID3D12VersionedRootSignatureDeserializer
);
impl IVersionedRootSignatureDeserializer for VersionedRootSignatureDeserializer {
    fn get_root_signature_desc_at_version(
        &self,
        convert_to_version: d3d::RootSignatureVersion,
    ) -> Result<VersionedRootSignatureDesc, HResult> {
        let mut desc = std::ptr::null_mut();
        unsafe {
            let res = self
                .0
                .GetRootSignatureDescAtVersion(convert_to_version.into(), &mut desc);
            if res < 0 {
                Err(res.into())
            } else {
                Ok((*desc).into())
            }
        }
    }
    fn get_unconverted_root_signature_desc(&self) -> VersionedRootSignatureDesc {
        unsafe { (*self.0.GetUnconvertedRootSignatureDesc()).into() }
    }
}

/*
pub trait IStateObject: IPageable {}
#[derive(Clone, Debug)]
pub struct StateObject(ComPtr<ID3D12StateObject>);
impl_pageable!(StateObject, ID3D12StateObject);
impl IStateObject for StateObject {}
*/
// pub trait IStateObjectProperties;

pub fn create_device<T: IDevice>(
    adapter: Option<&dxgi::Adapter>,
    minimum_feature_level: d3d::FeatureLevel,
) -> Result<T, HResult> {
    Ok(T::new(ComPtr::new(|| {
        let mut obj = std::ptr::null_mut();
        let res = unsafe {
            D3D12CreateDevice(
                adapter.map_or(std::ptr::null_mut(), |a| a.as_unknown()),
                minimum_feature_level.into(),
                &T::uuidof().into(),
                &mut obj,
            )
        };
        hresult(obj as *mut <T as Interface>::APIType, res)
    })?))
}

pub fn serialize_root_signature(
    desc: &RootSignatureDesc,
    version: d3d::RootSignatureVersion,
) -> Result<d3d::Blob, (HResult, Option<d3d::Blob>)> {
    let mut obj = std::ptr::null_mut();
    let mut err = std::ptr::null_mut();
    let (c_desc, _tmp) = desc.to_c_struct();
    unsafe {
        let res = D3D12SerializeRootSignature(&c_desc, version.into(), &mut obj, &mut err);
        if res < 0 {
            Err((
                res.into(),
                if err != std::ptr::null_mut() {
                    Some(d3d::Blob::new(ComPtr::from_raw(err)))
                } else {
                    None
                },
            ))
        } else {
            Ok(d3d::Blob::new(ComPtr::from_raw(obj)))
        }
    }
}

pub fn serialize_versioned_root_signature(
    desc: &VersionedRootSignatureDesc,
) -> Result<d3d::Blob, (HResult, Option<d3d::Blob>)> {
    let mut obj = std::ptr::null_mut();
    let mut err = std::ptr::null_mut();
    let (c_desc, _tmp) = desc.to_c_struct();
    unsafe {
        let res = D3D12SerializeVersionedRootSignature(&c_desc, &mut obj, &mut err);
        if res < 0 {
            Err((
                res.into(),
                if err != std::ptr::null_mut() {
                    Some(d3d::Blob::new(ComPtr::from_raw(err)))
                } else {
                    None
                },
            ))
        } else {
            Ok(d3d::Blob::new(ComPtr::from_raw(obj)))
        }
    }
}

pub fn get_debug_interface<T: IDebug>() -> Result<T, HResult> {
    Ok(T::new(ComPtr::new(|| {
        let mut obj = std::ptr::null_mut();
        let res = unsafe { D3D12GetDebugInterface(&T::uuidof().into(), &mut obj) };
        hresult(obj as *mut <T as Interface>::APIType, res)
    })?))
}

#[cfg(test)]
mod tests {
    use super::*;
    use d3d;

    #[test]
    fn create_device_test() {
        let device = create_device::<Device>(None, d3d::FeatureLevel(12, 0));
        assert!(device.is_ok());
    }
}
