use crate::api::Rect;
use crate::api::*;
use crate::d3d;
use crate::dxgi;
use crate::result::{hresult, HResult};
use crate::utility::*;
use crate::Interface;
use crate::{impl_bitflag_operators, impl_interface};
use com_ptr::ComPtr;
use winapi::ctypes::c_void;
use winapi::shared::windef::RECT;
use winapi::um::d3d11::*;
use winapi::um::d3dcommon::*;
use winapi::Interface as _;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct AsyncGetDataFlag(u32);
#[allow(non_upper_case_globals)]
impl AsyncGetDataFlag {
    pub const DoNotFlush: Self = Self(D3D11_ASYNC_GETDATA_DONOTFLUSH);
}
impl_bitflag_operators!(AsyncGetDataFlag);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AuthenticatedChannelType {
    D3D11 = D3D11_AUTHENTICATED_CHANNEL_D3D11,
    DriverSoftware = D3D11_AUTHENTICATED_CHANNEL_DRIVER_SOFTWARE,
    DriverHardware = D3D11_AUTHENTICATED_CHANNEL_DRIVER_HARDWARE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AuthenticatedProcessIdentifierType {
    // Typo in winapi-rs
    Unknown = DD3D11_PROCESSIDTYPE_UNKNOWN,
    DWM = DD3D11_PROCESSIDTYPE_DWM,
    Handle = DD3D11_PROCESSIDTYPE_HANDLE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BindFlags(u32);
#[allow(non_upper_case_globals)]
impl BindFlags {
    pub const VertexBuffer: Self = Self(D3D11_BIND_VERTEX_BUFFER);
    pub const IndexBuffer: Self = Self(D3D11_BIND_INDEX_BUFFER);
    pub const ConstantBuffer: Self = Self(D3D11_BIND_CONSTANT_BUFFER);
    pub const ShaderResource: Self = Self(D3D11_BIND_SHADER_RESOURCE);
    pub const StreamOutput: Self = Self(D3D11_BIND_STREAM_OUTPUT);
    pub const RenderTarget: Self = Self(D3D11_BIND_RENDER_TARGET);
    pub const DepthStencil: Self = Self(D3D11_BIND_DEPTH_STENCIL);
    pub const UnorderedAccess: Self = Self(D3D11_BIND_UNORDERED_ACCESS);
    pub const Decoder: Self = Self(D3D11_BIND_DECODER);
    pub const VideoEncoder: Self = Self(D3D11_BIND_VIDEO_ENCODER);
}
impl_bitflag_operators!(BindFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Blend {
    Zero = D3D11_BLEND_ZERO,
    One = D3D11_BLEND_ONE,
    SrcColor = D3D11_BLEND_SRC_COLOR,
    InvSrcColor = D3D11_BLEND_INV_SRC_COLOR,
    SrcAlpha = D3D11_BLEND_SRC_ALPHA,
    InvSrcAlpha = D3D11_BLEND_INV_SRC_ALPHA,
    DestAlpha = D3D11_BLEND_DEST_ALPHA,
    InvDestAlpha = D3D11_BLEND_INV_DEST_ALPHA,
    DestColor = D3D11_BLEND_DEST_COLOR,
    InvDestColor = D3D11_BLEND_INV_DEST_COLOR,
    SrcAlphaSat = D3D11_BLEND_SRC_ALPHA_SAT,
    BlendFactor = D3D11_BLEND_BLEND_FACTOR,
    InvBlendFactor = D3D11_BLEND_INV_BLEND_FACTOR,
    Src1Color = D3D11_BLEND_SRC1_COLOR,
    InvSrc1Color = D3D11_BLEND_INV_SRC1_COLOR,
    Src1Alpha = D3D11_BLEND_SRC1_ALPHA,
    InvSrc1Alpha = D3D11_BLEND_INV_SRC1_ALPHA,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BlendOp {
    Add = D3D11_BLEND_OP_ADD,
    Subtract = D3D11_BLEND_OP_SUBTRACT,
    RevSubtract = D3D11_BLEND_OP_REV_SUBTRACT,
    Min = D3D11_BLEND_OP_MIN,
    Max = D3D11_BLEND_OP_MAX,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BufferUAVFlags(u32);
#[allow(non_upper_case_globals)]
impl BufferUAVFlags {
    pub const Raw: Self = Self(D3D11_BUFFER_UAV_FLAG_RAW);
    pub const Append: Self = Self(D3D11_BUFFER_UAV_FLAG_APPEND);
    pub const Counter: Self = Self(D3D11_BUFFER_UAV_FLAG_COUNTER);
}
impl_bitflag_operators!(BufferUAVFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BufferExSRVFlags(u32);
#[allow(non_upper_case_globals)]
impl BufferExSRVFlags {
    pub const Raw: Self = Self(D3D11_BUFFEREX_SRV_FLAG_RAW);
}
impl_bitflag_operators!(BufferExSRVFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BusType {
    Other = D3D11_BUS_TYPE_OTHER,
    PCI = D3D11_BUS_TYPE_PCI,
    PCIX = D3D11_BUS_TYPE_PCIX,
    PCIExpress = D3D11_BUS_TYPE_PCIEXPRESS,
    AGP = D3D11_BUS_TYPE_AGP,
    ModifierInsideOfChipset = D3D11_BUS_IMPL_MODIFIER_INSIDE_OF_CHIPSET,
    ModifierTracksOnMotherBoardToChip = D3D11_BUS_IMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_CHIP,
    ModifierTracksOnMotherBoardToSocket = D3D11_BUS_IMPL_MODIFIER_TRACKS_ON_MOTHER_BOARD_TO_SOCKET,
    ModifierDaughterBoardConnector = D3D11_BUS_IMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR,
    ModifierDaughterBoardConnectorInsideOfNuae =
        D3D11_BUS_IMPL_MODIFIER_DAUGHTER_BOARD_CONNECTOR_INSIDE_OF_NUAE,
    ModifierNonStandard = D3D11_BUS_IMPL_MODIFIER_NON_STANDARD,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ClearFlags(u32);
#[allow(non_upper_case_globals)]
impl ClearFlags {
    pub const Depth: Self = Self(D3D11_CLEAR_DEPTH);
    pub const Stencil: Self = Self(D3D11_CLEAR_STENCIL);
}
impl_bitflag_operators!(ClearFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ColorWriteEnable(u32);
#[allow(non_upper_case_globals)]
impl ColorWriteEnable {
    pub const Red: Self = Self(D3D11_COLOR_WRITE_ENABLE_RED);
    pub const Green: Self = Self(D3D11_COLOR_WRITE_ENABLE_GREEN);
    pub const Blue: Self = Self(D3D11_COLOR_WRITE_ENABLE_BLUE);
    pub const Alpha: Self = Self(D3D11_COLOR_WRITE_ENABLE_ALPHA);
    pub const All: Self = Self(D3D11_COLOR_WRITE_ENABLE_ALL);
}
impl_bitflag_operators!(ColorWriteEnable);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ComparisonFunc {
    Never = D3D11_COMPARISON_NEVER,
    Less = D3D11_COMPARISON_LESS,
    Equal = D3D11_COMPARISON_EQUAL,
    LessEqual = D3D11_COMPARISON_LESS_EQUAL,
    Greater = D3D11_COMPARISON_GREATER,
    NotEqual = D3D11_COMPARISON_NOT_EQUAL,
    GreaterEqual = D3D11_COMPARISON_GREATER_EQUAL,
    Always = D3D11_COMPARISON_ALWAYS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ConservativeRasterizationTier {
    NotSupported = D3D11_CONSERVATIVE_RASTERIZATION_NOT_SUPPORTED,
    Tier1 = D3D11_CONSERVATIVE_RASTERIZATION_TIER_1,
    Tier2 = D3D11_CONSERVATIVE_RASTERIZATION_TIER_2,
    Tier3 = D3D11_CONSERVATIVE_RASTERIZATION_TIER_3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ContentProtectionCaps(u32);
#[allow(non_upper_case_globals)]
impl ContentProtectionCaps {
    pub const Software: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_SOFTWARE);
    pub const Hardware: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_HARDWARE);
    pub const AlwaysOn: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_PROTECTION_ALWAYS_ON);
    pub const PartialDecryption: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_PARTIAL_DECRYPTION);
    pub const ContentKey: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_CONTENT_KEY);
    pub const FreshenSessionKey: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_FRESHEN_SESSION_KEY);
    pub const EncryptedReadBack: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_ENCRYPTED_READ_BACK);
    pub const EncryptedReadBackKey: Self =
        Self(D3D11_CONTENT_PROTECTION_CAPS_ENCRYPTED_READ_BACK_KEY);
    pub const SequentialCTRIV: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_SEQUENTIAL_CTR_IV);
    pub const EncryptSliceDataOnly: Self =
        Self(D3D11_CONTENT_PROTECTION_CAPS_ENCRYPT_SLICEDATA_ONLY);
    pub const DecryptionBlt: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_DECRYPTION_BLT);
    pub const HardwareProtectUncompressed: Self =
        Self(D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_PROTECT_UNCOMPRESSED);
    pub const HardwareProtectedMemoryPageable: Self =
        Self(D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_PROTECTED_MEMORY_PAGEABLE);
    pub const HardwareTeardown: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_TEARDOWN);
    pub const HardwareDRMCommunication: Self =
        Self(D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_DRM_COMMUNICATION);
    // pub const HardwareDRMCommunicationMultiThreaded: Self = Self(D3D11_CONTENT_PROTECTION_CAPS_HARDWARE_DRM_COMMUNICATION_MULTI_THREADED);
}
impl_bitflag_operators!(ContentProtectionCaps);

pub const COUNTER_DEVICE_DEPENDENT_0: u32 = D3D11_COUNTER_DEVICE_DEPENDENT_0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CounterType {
    Float32 = D3D11_COUNTER_TYPE_FLOAT32,
    Uint16 = D3D11_COUNTER_TYPE_UINT16,
    Uint32 = D3D11_COUNTER_TYPE_UINT32,
    Uint64 = D3D11_COUNTER_TYPE_UINT64,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CPUAccessFlags(u32);
#[allow(non_upper_case_globals)]
impl CPUAccessFlags {
    pub const Write: Self = Self(D3D11_CPU_ACCESS_WRITE);
    pub const Read: Self = Self(D3D11_CPU_ACCESS_READ);
}
impl_bitflag_operators!(CPUAccessFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CreateDeviceFlags(u32);
#[allow(non_upper_case_globals)]
impl CreateDeviceFlags {
    pub const SingleThreaded: Self = Self(D3D11_CREATE_DEVICE_SINGLETHREADED);
    pub const Debug: Self = Self(D3D11_CREATE_DEVICE_DEBUG);
    pub const SwitchToRef: Self = Self(D3D11_CREATE_DEVICE_SWITCH_TO_REF);
    pub const PreventInternalThreadingOptimizations: Self =
        Self(D3D11_CREATE_DEVICE_PREVENT_INTERNAL_THREADING_OPTIMIZATIONS);
    pub const BGRASupport: Self = Self(D3D11_CREATE_DEVICE_BGRA_SUPPORT);
    pub const Debuggable: Self = Self(D3D11_CREATE_DEVICE_DEBUGGABLE);
    pub const PreventAlteringLayerSettingsFromRegistry: Self =
        Self(D3D11_CREATE_DEVICE_PREVENT_ALTERING_LAYER_SETTINGS_FROM_REGISTRY);
    pub const DisableGPUTimeout: Self = Self(D3D11_CREATE_DEVICE_DISABLE_GPU_TIMEOUT);
    pub const VideoSupport: Self = Self(D3D11_CREATE_DEVICE_VIDEO_SUPPORT);
}
impl_bitflag_operators!(CreateDeviceFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CullMode {
    None = D3D11_CULL_NONE,
    Front = D3D11_CULL_FRONT,
    Back = D3D11_CULL_BACK,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DepthWriteMask {
    Zero = D3D11_DEPTH_WRITE_MASK_ZERO,
    All = D3D11_DEPTH_WRITE_MASK_ALL,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DeviceContextType {
    Immediate = D3D11_DEVICE_CONTEXT_IMMEDIATE,
    Deferred = D3D11_DEVICE_CONTEXT_DEFERRED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DSVDimension {
    Unknown = D3D11_DSV_DIMENSION_UNKNOWN,
    Texture1D = D3D11_DSV_DIMENSION_TEXTURE1D,
    Texture1DArray = D3D11_DSV_DIMENSION_TEXTURE1DARRAY,
    Texture2D = D3D11_DSV_DIMENSION_TEXTURE2D,
    Texture2DArray = D3D11_DSV_DIMENSION_TEXTURE2DARRAY,
    Texture2DMS = D3D11_DSV_DIMENSION_TEXTURE2DMS,
    Texture2DMSArray = D3D11_DSV_DIMENSION_TEXTURE2DMSARRAY,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DSVFlags(u32);
#[allow(non_upper_case_globals)]
impl DSVFlags {
    pub const ReadOnlyDepth: Self = Self(D3D11_DSV_READ_ONLY_DEPTH);
    pub const ReadOnlyStencil: Self = Self(D3D11_DSV_READ_ONLY_STENCIL);
}
impl_bitflag_operators!(DSVFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FillMode {
    Wireframe = D3D11_FILL_WIREFRAME,
    Solid = D3D11_FILL_SOLID,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Filter {
    MinMagMipPoint = D3D11_FILTER_MIN_MAG_MIP_POINT,
    MinMagPointMipLinear = D3D11_FILTER_MIN_MAG_POINT_MIP_LINEAR,
    MinPointMagLinearMipPoint = D3D11_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT,
    MinPointMagMipLinear = D3D11_FILTER_MIN_POINT_MAG_MIP_LINEAR,
    MinLinearMagMipPoint = D3D11_FILTER_MIN_LINEAR_MAG_MIP_POINT,
    MinLinearMagPointMipLinear = D3D11_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
    MinMagLinearMipPoint = D3D11_FILTER_MIN_MAG_LINEAR_MIP_POINT,
    MinMagMipLinear = D3D11_FILTER_MIN_MAG_MIP_LINEAR,
    Anisotropic = D3D11_FILTER_ANISOTROPIC,
    ComparisonMinMagMipPoint = D3D11_FILTER_COMPARISON_MIN_MAG_MIP_POINT,
    ComparisonMinMagPointMipLinear = D3D11_FILTER_COMPARISON_MIN_MAG_POINT_MIP_LINEAR,
    ComparisonMinPointMagLinearMipPoint = D3D11_FILTER_COMPARISON_MIN_POINT_MAG_LINEAR_MIP_POINT,
    ComparisonMinPointMagMipLinear = D3D11_FILTER_COMPARISON_MIN_POINT_MAG_MIP_LINEAR,
    ComparisonMinLinearMagMipPoint = D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_MIP_POINT,
    ComparisonMinLinearMagPointMipLinear = D3D11_FILTER_COMPARISON_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
    ComparisonMinMagLinearMipPoint = D3D11_FILTER_COMPARISON_MIN_MAG_LINEAR_MIP_POINT,
    ComparisonMinMagMipLinear = D3D11_FILTER_COMPARISON_MIN_MAG_MIP_LINEAR,
    ComparisonAnisotropic = D3D11_FILTER_COMPARISON_ANISOTROPIC,
    MinimumMinMagMipPoint = D3D11_FILTER_MINIMUM_MIN_MAG_MIP_POINT,
    MinimumMinMagPointMipLinear = D3D11_FILTER_MINIMUM_MIN_MAG_POINT_MIP_LINEAR,
    MinimumMinPointMagLinearMipPoint = D3D11_FILTER_MINIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT,
    MinimumMinPointMagMipLinear = D3D11_FILTER_MINIMUM_MIN_POINT_MAG_MIP_LINEAR,
    MinimumMinLinearMagMipPoint = D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_MIP_POINT,
    MinimumMinLinearMagPointMipLinear = D3D11_FILTER_MINIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
    MinimumMinMagLinearMipPoint = D3D11_FILTER_MINIMUM_MIN_MAG_LINEAR_MIP_POINT,
    MinimumMinMagMipLinear = D3D11_FILTER_MINIMUM_MIN_MAG_MIP_LINEAR,
    MinimumAnisotropic = D3D11_FILTER_MINIMUM_ANISOTROPIC,
    MaximumMinMagMipPoint = D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_POINT,
    MaximumMinMagPointMipLinear = D3D11_FILTER_MAXIMUM_MIN_MAG_POINT_MIP_LINEAR,
    MaximumPointMagLinearMipPoint = D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_LINEAR_MIP_POINT,
    MaximumMinPointMagMipLinear = D3D11_FILTER_MAXIMUM_MIN_POINT_MAG_MIP_LINEAR,
    MaximumMinLinearMagMipPoint = D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_MIP_POINT,
    MaximumMinLinearMagPointMipLinear = D3D11_FILTER_MAXIMUM_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
    MaximumMinMagLinearMipPoint = D3D11_FILTER_MAXIMUM_MIN_MAG_LINEAR_MIP_POINT,
    MaximumMinMagMipLinear = D3D11_FILTER_MAXIMUM_MIN_MAG_MIP_LINEAR,
    MaximumAnisotropic = D3D11_FILTER_MAXIMUM_ANISOTROPIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FilterReductionType {
    Standard = D3D11_FILTER_REDUCTION_TYPE_STANDARD,
    Comparison = D3D11_FILTER_REDUCTION_TYPE_COMPARISON,
    Minimum = D3D11_FILTER_REDUCTION_TYPE_MINIMUM,
    Maximum = D3D11_FILTER_REDUCTION_TYPE_MAXIMUM,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FilterType {
    Point = D3D11_FILTER_TYPE_POINT,
    Linear = D3D11_FILTER_TYPE_LINEAR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FormatSupport(u32);
#[allow(non_upper_case_globals)]
impl FormatSupport {
    pub const Buffer: Self = Self(D3D11_FORMAT_SUPPORT_BUFFER);
    pub const IAVertexBuffer: Self = Self(D3D11_FORMAT_SUPPORT_IA_VERTEX_BUFFER);
    pub const IAIndxBuffer: Self = Self(D3D11_FORMAT_SUPPORT_IA_INDEX_BUFFER);
    pub const SOBuffer: Self = Self(D3D11_FORMAT_SUPPORT_SO_BUFFER);
    pub const Texture1D: Self = Self(D3D11_FORMAT_SUPPORT_TEXTURE1D);
    pub const Texture2D: Self = Self(D3D11_FORMAT_SUPPORT_TEXTURE2D);
    pub const Texture3D: Self = Self(D3D11_FORMAT_SUPPORT_TEXTURE3D);
    pub const TextureCube: Self = Self(D3D11_FORMAT_SUPPORT_TEXTURECUBE);
    pub const ShaderLoad: Self = Self(D3D11_FORMAT_SUPPORT_SHADER_LOAD);
    pub const Sample: Self = Self(D3D11_FORMAT_SUPPORT_SHADER_SAMPLE);
    pub const SampleComparison: Self = Self(D3D11_FORMAT_SUPPORT_SHADER_SAMPLE_COMPARISON);
    pub const ShaderSampleMonoText: Self = Self(D3D11_FORMAT_SUPPORT_SHADER_SAMPLE_MONO_TEXT);
    pub const Mip: Self = Self(D3D11_FORMAT_SUPPORT_MIP);
    pub const MipAutogen: Self = Self(D3D11_FORMAT_SUPPORT_MIP_AUTOGEN);
    pub const RenderTarget: Self = Self(D3D11_FORMAT_SUPPORT_RENDER_TARGET);
    pub const Blendable: Self = Self(D3D11_FORMAT_SUPPORT_BLENDABLE);
    pub const DepthStencil: Self = Self(D3D11_FORMAT_SUPPORT_DEPTH_STENCIL);
    pub const CPULockable: Self = Self(D3D11_FORMAT_SUPPORT_CPU_LOCKABLE);
    pub const MultisampleResolve: Self = Self(D3D11_FORMAT_SUPPORT_MULTISAMPLE_RESOLVE);
    pub const Display: Self = Self(D3D11_FORMAT_SUPPORT_DISPLAY);
    pub const CastWithinBitLayout: Self = Self(D3D11_FORMAT_SUPPORT_CAST_WITHIN_BIT_LAYOUT);
    pub const MultisampleRenderTarget: Self = Self(D3D11_FORMAT_SUPPORT_MULTISAMPLE_RENDERTARGET);
    pub const MultisampleLoad: Self = Self(D3D11_FORMAT_SUPPORT_MULTISAMPLE_LOAD);
    pub const ShaderGather: Self = Self(D3D11_FORMAT_SUPPORT_SHADER_GATHER);
    pub const BackBufferCast: Self = Self(D3D11_FORMAT_SUPPORT_BACK_BUFFER_CAST);
    pub const TypedUnorderedAccessView: Self =
        Self(D3D11_FORMAT_SUPPORT_TYPED_UNORDERED_ACCESS_VIEW);
    pub const ShaderGatherComparison: Self = Self(D3D11_FORMAT_SUPPORT_SHADER_GATHER_COMPARISON);
    pub const DecodeOutput: Self = Self(D3D11_FORMAT_SUPPORT_DECODER_OUTPUT);
    pub const VideoProcessorOutput: Self = Self(D3D11_FORMAT_SUPPORT_VIDEO_PROCESSOR_OUTPUT);
    pub const VideoProcessorInput: Self = Self(D3D11_FORMAT_SUPPORT_VIDEO_PROCESSOR_INPUT);
    pub const VideoEncoder: Self = Self(D3D11_FORMAT_SUPPORT_VIDEO_ENCODER);
}
impl_bitflag_operators!(FormatSupport);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FormatSupport2(u32);
#[allow(non_upper_case_globals)]
impl FormatSupport2 {
    pub const UAVAtomicAdd: Self = Self(D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_ADD);
    pub const UAVAtomicBitwiseOps: Self = Self(D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_BITWISE_OPS);
    pub const UAVAtomicCompareStoreOrCompareExchange: Self =
        Self(D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_COMPARE_STORE_OR_COMPARE_EXCHANGE);
    pub const UAVAtomicExchange: Self = Self(D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_EXCHANGE);
    pub const UAVAtomicSignedMinOrMax: Self =
        Self(D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_SIGNED_MIN_OR_MAX);
    pub const UAVAtomicUnsignedMinOrMax: Self =
        Self(D3D11_FORMAT_SUPPORT2_UAV_ATOMIC_UNSIGNED_MIN_OR_MAX);
    pub const UAVTypedLoad: Self = Self(D3D11_FORMAT_SUPPORT2_UAV_TYPED_LOAD);
    pub const UAVTypedStore: Self = Self(D3D11_FORMAT_SUPPORT2_UAV_TYPED_STORE);
    pub const OutputMergerLogicOp: Self = Self(D3D11_FORMAT_SUPPORT2_OUTPUT_MERGER_LOGIC_OP);
    pub const Tiled: Self = Self(D3D11_FORMAT_SUPPORT2_TILED);
    pub const Shareable: Self = Self(D3D11_FORMAT_SUPPORT2_SHAREABLE);
    pub const MultiplaneOverlay: Self = Self(D3D11_FORMAT_SUPPORT2_MULTIPLANE_OVERLAY);
}
impl_bitflag_operators!(FormatSupport2);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum InputClassification {
    PerVertexData = D3D11_INPUT_PER_VERTEX_DATA,
    PerInstanceData = D3D11_INPUT_PER_INSTANCE_DATA,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum MapType {
    Read = D3D11_MAP_READ,
    Write = D3D11_MAP_WRITE,
    ReadWrite = D3D11_MAP_READ_WRITE,
    WriteDiscard = D3D11_MAP_WRITE_DISCARD,
    WriteNoOverwrite = D3D11_MAP_WRITE_NO_OVERWRITE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct MapFlag(u32);
#[allow(non_upper_case_globals)]
impl MapFlag {
    pub const DoNotWait: MapFlag = MapFlag(D3D11_MAP_FLAG_DO_NOT_WAIT);
}
impl_bitflag_operators!(MapFlag);

pub type Primitive = d3d::Primitive;
pub type PrimitiveTopology = d3d::PrimitiveTopology;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum QueryType {
    Event = D3D11_QUERY_EVENT,
    Occlusion = D3D11_QUERY_OCCLUSION,
    Timestamp = D3D11_QUERY_TIMESTAMP,
    TimestampDisjoint = D3D11_QUERY_TIMESTAMP_DISJOINT,
    PipelineStatistics = D3D11_QUERY_PIPELINE_STATISTICS,
    OcclustionPredicate = D3D11_QUERY_OCCLUSION_PREDICATE,
    SOStatistics = D3D11_QUERY_SO_STATISTICS,
    SOOverflowPredicate = D3D11_QUERY_SO_OVERFLOW_PREDICATE,
    SOStatisticsStream0 = D3D11_QUERY_SO_STATISTICS_STREAM0,
    SOOverflowPredicateStream0 = D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM0,
    SOStatisticsStream1 = D3D11_QUERY_SO_STATISTICS_STREAM1,
    SOOverflowPredicateStream1 = D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM1,
    SOStatisticsStream2 = D3D11_QUERY_SO_STATISTICS_STREAM2,
    SOOverflowPredicateStream2 = D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM2,
    SOStatisticsStream3 = D3D11_QUERY_SO_STATISTICS_STREAM3,
    SOOverflowPredicateStream3 = D3D11_QUERY_SO_OVERFLOW_PREDICATE_STREAM3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct QueryMiscFlags(u32);
#[allow(non_upper_case_globals)]
impl QueryMiscFlags {
    pub const PredicateHint: Self = Self(D3D11_QUERY_MISC_PREDICATEHINT);
}
impl_bitflag_operators!(QueryMiscFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct RaiseFlags(u32);
#[allow(non_upper_case_globals)]
impl RaiseFlags {
    pub const DriverInternalError: Self = Self(D3D11_RAISE_FLAG_DRIVER_INTERNAL_ERROR);
}
impl_bitflag_operators!(RaiseFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ResourceDimension {
    Unknown = D3D11_RESOURCE_DIMENSION_UNKNOWN,
    Buffer = D3D11_RESOURCE_DIMENSION_BUFFER,
    Texture1D = D3D11_RESOURCE_DIMENSION_TEXTURE1D,
    Texture2D = D3D11_RESOURCE_DIMENSION_TEXTURE2D,
    Texture3D = D3D11_RESOURCE_DIMENSION_TEXTURE3D,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ResourceMiscFlags(u32);
#[allow(non_upper_case_globals)]
impl ResourceMiscFlags {
    pub const GenerateMips: Self = Self(D3D11_RESOURCE_MISC_GENERATE_MIPS);
    pub const Shared: Self = Self(D3D11_RESOURCE_MISC_SHARED);
    pub const TextureCube: Self = Self(D3D11_RESOURCE_MISC_TEXTURECUBE);
    pub const DrawIndirectArgs: Self = Self(D3D11_RESOURCE_MISC_DRAWINDIRECT_ARGS);
    pub const BufferAllowRawViews: Self = Self(D3D11_RESOURCE_MISC_BUFFER_ALLOW_RAW_VIEWS);
    pub const BufferStructured: Self = Self(D3D11_RESOURCE_MISC_BUFFER_STRUCTURED);
    pub const ResourceClamp: Self = Self(D3D11_RESOURCE_MISC_RESOURCE_CLAMP);
    pub const SharedKeyedMutex: Self = Self(D3D11_RESOURCE_MISC_SHARED_KEYEDMUTEX);
    pub const GDICompatible: Self = Self(D3D11_RESOURCE_MISC_GDI_COMPATIBLE);
    pub const SharedNTHandle: Self = Self(D3D11_RESOURCE_MISC_SHARED_NTHANDLE);
    pub const RestrictedContent: Self = Self(D3D11_RESOURCE_MISC_RESTRICTED_CONTENT);
    pub const RestrictSharedResource: Self = Self(D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE);
    pub const RestrictSharedResourceDriver: Self =
        Self(D3D11_RESOURCE_MISC_RESTRICT_SHARED_RESOURCE_DRIVER);
    pub const Guarded: Self = Self(D3D11_RESOURCE_MISC_GUARDED);
    pub const TilePool: Self = Self(D3D11_RESOURCE_MISC_TILE_POOL);
    pub const Tiled: Self = Self(D3D11_RESOURCE_MISC_TILED);
    pub const HWProtected: Self = Self(D3D11_RESOURCE_MISC_HW_PROTECTED);
}
impl_bitflag_operators!(ResourceMiscFlags);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RTVDimension {
    Unknown = D3D11_RTV_DIMENSION_UNKNOWN,
    Buffer = D3D11_RTV_DIMENSION_BUFFER,
    Texture1D = D3D11_RTV_DIMENSION_TEXTURE1D,
    Texture1DArray = D3D11_RTV_DIMENSION_TEXTURE1DARRAY,
    Texture2D = D3D11_RTV_DIMENSION_TEXTURE2D,
    Texture2DArray = D3D11_RTV_DIMENSION_TEXTURE2DARRAY,
    Texture2DMS = D3D11_RTV_DIMENSION_TEXTURE2DMS,
    Texture2DMSArray = D3D11_RTV_DIMENSION_TEXTURE2DMSARRAY,
    Texture3D = D3D11_RTV_DIMENSION_TEXTURE3D,
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ShaderCacheSupportFlags(u32);
#[allow(non_upper_case_globals)]
impl ShaderCacheSupportFlags {
    pub const None: Self = Self(D3D11_SHADER_CACHE_SUPPORT_NONE);
    pub const AutomaticInprocCache: Self = Self(D3D11_SHADER_CACHE_SUPPORT_AUTOMATIC_INPROC_CACHE);
    pub const AutomaticDiskCache: Self = Self(D3D11_SHADER_CACHE_SUPPORT_AUTOMATIC_DISK_CACHE);
}
impl_bitflag_operators!(ShaderCacheSupportFlags);
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ShaderMinPrecisionSupport {
    _10Bit = D3D11_SHADER_MIN_PRECISION_10_BIT,
    _16Bit = D3D11_SHADER_MIN_PRECISION_16_BIT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StandardMultisampleQualityLevels {
    StandardMultisamplePattern = D3D11_STANDARD_MULTISAMPLE_PATTERN,
    CenterMultisamplePattern = D3D11_CENTER_MULTISAMPLE_PATTERN,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StencilOp {
    Keep = D3D11_STENCIL_OP_KEEP,
    Zero = D3D11_STENCIL_OP_ZERO,
    Replace = D3D11_STENCIL_OP_REPLACE,
    IncrSat = D3D11_STENCIL_OP_INCR_SAT,
    DecrSat = D3D11_STENCIL_OP_DECR_SAT,
    Invert = D3D11_STENCIL_OP_INVERT,
    Incr = D3D11_STENCIL_OP_INCR,
    Decr = D3D11_STENCIL_OP_DECR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextureAddressMode {
    Wrap = D3D11_TEXTURE_ADDRESS_WRAP,
    Mirror = D3D11_TEXTURE_ADDRESS_MIRROR,
    Clamp = D3D11_TEXTURE_ADDRESS_CLAMP,
    Border = D3D11_TEXTURE_ADDRESS_BORDER,
    Once = D3D11_TEXTURE_ADDRESS_MIRROR_ONCE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextureCubeFace {
    PositiveX = D3D11_TEXTURECUBE_FACE_POSITIVE_X,
    NegativeX = D3D11_TEXTURECUBE_FACE_NEGATIVE_X,
    PositiveY = D3D11_TEXTURECUBE_FACE_POSITIVE_Y,
    NegativeY = D3D11_TEXTURECUBE_FACE_NEGATIVE_Y,
    PositiveZ = D3D11_TEXTURECUBE_FACE_POSITIVE_Z,
    NegativeZ = D3D11_TEXTURECUBE_FACE_NEGATIVE_Z,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TiledResourcesTier {
    NotSupported = D3D11_TILED_RESOURCES_NOT_SUPPORTED,
    Tier1 = D3D11_TILED_RESOURCES_TIER_1,
    Tier2 = D3D11_TILED_RESOURCES_TIER_2,
    Tier3 = D3D11_TILED_RESOURCES_TIER_3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum UAVDimension {
    Unknown = D3D11_UAV_DIMENSION_UNKNOWN,
    Buffer = D3D11_UAV_DIMENSION_BUFFER,
    Texture1D = D3D11_UAV_DIMENSION_TEXTURE1D,
    Texture1DArray = D3D11_UAV_DIMENSION_TEXTURE1DARRAY,
    Texture2D = D3D11_UAV_DIMENSION_TEXTURE2D,
    Texture2DArray = D3D11_UAV_DIMENSION_TEXTURE2DARRAY,
    Texture3D = D3D11_UAV_DIMENSION_TEXTURE3D,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Usage {
    Default = D3D11_USAGE_DEFAULT,
    Immutable = D3D11_USAGE_IMMUTABLE,
    Dynamic = D3D11_USAGE_DYNAMIC,
    Staging = D3D11_USAGE_STAGING,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VDOVDimension {
    Unknown = D3D11_VDOV_DIMENSION_UNKNOWN,
    Texture2D = D3D11_VDOV_DIMENSION_TEXTURE2D,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VideoDecoderBufferType {
    PictureParameters = D3D11_VIDEO_DECODER_BUFFER_PICTURE_PARAMETERS,
    MacroblockControl = D3D11_VIDEO_DECODER_BUFFER_MACROBLOCK_CONTROL,
    ResidualDifference = D3D11_VIDEO_DECODER_BUFFER_RESIDUAL_DIFFERENCE,
    DeblockingControl = D3D11_VIDEO_DECODER_BUFFER_DEBLOCKING_CONTROL,
    InverseQuantizationMatrix = D3D11_VIDEO_DECODER_BUFFER_INVERSE_QUANTIZATION_MATRIX,
    SliceControl = D3D11_VIDEO_DECODER_BUFFER_SLICE_CONTROL,
    Bitstream = D3D11_VIDEO_DECODER_BUFFER_BITSTREAM,
    MotionVector = D3D11_VIDEO_DECODER_BUFFER_MOTION_VECTOR,
    FilmGrain = D3D11_VIDEO_DECODER_BUFFER_FILM_GRAIN,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VideoFrameFormat {
    Progressive = D3D11_VIDEO_FRAME_FORMAT_PROGRESSIVE,
    InterlacedTopFieldFirst = D3D11_VIDEO_FRAME_FORMAT_INTERLACED_TOP_FIELD_FIRST,
    InterlacedBottomFieldFirst = D3D11_VIDEO_FRAME_FORMAT_INTERLACED_BOTTOM_FIELD_FIRST,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VideoProcessorAlphaFillMode {
    Opaque = D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_OPAQUE,
    Background = D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_BACKGROUND,
    Destination = D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_DESTINATION,
    SourceStream = D3D11_VIDEO_PROCESSOR_ALPHA_FILL_MODE_SOURCE_STREAM,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoProcessorAutoStreamCaps(u32);
#[allow(non_upper_case_globals)]
impl VideoProcessorAutoStreamCaps {
    pub const Denoise: Self = Self(D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_DENOISE);
    pub const Deringing: Self = Self(D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_DERINGING);
    pub const EdgeEnhancement: Self = Self(D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_EDGE_ENHANCEMENT);
    pub const ColorCorrection: Self = Self(D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_COLOR_CORRECTION);
    pub const FleshToneMapping: Self =
        Self(D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_FLESH_TONE_MAPPING);
    pub const ImageStabilization: Self =
        Self(D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_IMAGE_STABILIZATION);
    pub const SuperResolution: Self = Self(D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_SUPER_RESOLUTION);
    pub const AnamorphicScaling: Self =
        Self(D3D11_VIDEO_PROCESSOR_AUTO_STREAM_CAPS_ANAMORPHIC_SCALING);
}
impl_bitflag_operators!(VideoProcessorAutoStreamCaps);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoProcessorDeviceCaps(u32);
#[allow(non_upper_case_globals)]
impl VideoProcessorDeviceCaps {
    pub const LinearSpace: Self = Self(D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_LINEAR_SPACE);
    pub const xvYCC: Self = Self(D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_xvYCC);
    pub const RGBRangeConversion: Self =
        Self(D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_RGB_RANGE_CONVERSION);
    pub const YCbCrMatrixConversion: Self =
        Self(D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_YCbCr_MATRIX_CONVERSION);
    pub const NominalRange: Self = Self(D3D11_VIDEO_PROCESSOR_DEVICE_CAPS_NOMINAL_RANGE);
}
impl_bitflag_operators!(VideoProcessorDeviceCaps);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoProcessorFeatureCaps(u32);
#[allow(non_upper_case_globals)]
impl VideoProcessorFeatureCaps {
    pub const AlphaFill: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_FILL);
    pub const Constriction: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_CONSTRICTION);
    pub const LumaKey: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_LUMA_KEY);
    pub const AlphaPalette: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_PALETTE);
    pub const Legacy: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_LEGACY);
    pub const Stereo: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_STEREO);
    pub const Rotation: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ROTATION);
    pub const AlphaStream: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_ALPHA_STREAM);
    pub const PixelAspectRatio: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_PIXEL_ASPECT_RATIO);
    pub const Mirror: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_MIRROR);
    pub const ShaderUsage: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_SHADER_USAGE);
    // pub const MetadataHDR10: Self = Self(D3D11_VIDEO_PROCESSOR_FEATURE_CAPS_METADATA_HDR10);
}
impl_bitflag_operators!(VideoProcessorFeatureCaps);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VideoProcessorFilter {
    Brightness = D3D11_VIDEO_PROCESSOR_FILTER_BRIGHTNESS,
    Contrast = D3D11_VIDEO_PROCESSOR_FILTER_CONTRAST,
    Hue = D3D11_VIDEO_PROCESSOR_FILTER_HUE,
    Saturation = D3D11_VIDEO_PROCESSOR_FILTER_SATURATION,
    NoiseReduction = D3D11_VIDEO_PROCESSOR_FILTER_NOISE_REDUCTION,
    EdgeEnhancement = D3D11_VIDEO_PROCESSOR_FILTER_EDGE_ENHANCEMENT,
    AnamorphicScaling = D3D11_VIDEO_PROCESSOR_FILTER_ANAMORPHIC_SCALING,
    StereoAdjustment = D3D11_VIDEO_PROCESSOR_FILTER_STEREO_ADJUSTMENT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoProcessorFilterCaps(u32);
#[allow(non_upper_case_globals)]
impl VideoProcessorFilterCaps {
    pub const Brightness: Self = Self(D3D11_VIDEO_PROCESSOR_FILTER_CAPS_BRIGHTNESS);
    pub const Contrast: Self = Self(D3D11_VIDEO_PROCESSOR_FILTER_CAPS_CONTRAST);
    pub const Hue: Self = Self(D3D11_VIDEO_PROCESSOR_FILTER_CAPS_HUE);
    pub const Saturation: Self = Self(D3D11_VIDEO_PROCESSOR_FILTER_CAPS_SATURATION);
    pub const NoiseReduction: Self = Self(D3D11_VIDEO_PROCESSOR_FILTER_CAPS_NOISE_REDUCTION);
    pub const EdgeEnhancement: Self = Self(D3D11_VIDEO_PROCESSOR_FILTER_CAPS_EDGE_ENHANCEMENT);
    pub const AnamorphicScaling: Self = Self(D3D11_VIDEO_PROCESSOR_FILTER_CAPS_ANAMORPHIC_SCALING);
    pub const StereoAdjustment: Self = Self(D3D11_VIDEO_PROCESSOR_FILTER_CAPS_STEREO_ADJUSTMENT);
}
impl_bitflag_operators!(VideoProcessorFilterCaps);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoProcessorFormatCaps(u32);
#[allow(non_upper_case_globals)]
impl VideoProcessorFormatCaps {
    pub const RGBInterlaced: Self = Self(D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_INTERLACED);
    pub const RGBProcmap: Self = Self(D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_PROCAMP);
    pub const RGBLumaKey: Self = Self(D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_RGB_LUMA_KEY);
    pub const PaletteInterlaced: Self = Self(D3D11_VIDEO_PROCESSOR_FORMAT_CAPS_PALETTE_INTERLACED);
}
impl_bitflag_operators!(VideoProcessorFormatCaps);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VideoProcessorFormatSupport {
    Input = D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT_INPUT,
    Output = D3D11_VIDEO_PROCESSOR_FORMAT_SUPPORT_OUTPUT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoProcessorItelecineCaps(u32);
#[allow(non_upper_case_globals)]
impl VideoProcessorItelecineCaps {
    pub const _32: Self = Self(D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_32);
    pub const _22: Self = Self(D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_22);
    pub const _2224: Self = Self(D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_2224);
    pub const _2332: Self = Self(D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_2332);
    pub const _32322: Self = Self(D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_32322);
    pub const _55: Self = Self(D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_55);
    pub const _64: Self = Self(D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_64);
    pub const _87: Self = Self(D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_87);
    pub const _222222222223: Self = Self(D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_222222222223);
    pub const Other: Self = Self(D3D11_VIDEO_PROCESSOR_ITELECINE_CAPS_OTHER);
}
impl_bitflag_operators!(VideoProcessorItelecineCaps);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VideoProcessorNominalRange {
    Undefined = D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_UNDEFINED,
    _16_235 = D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_16_235,
    _0_255 = D3D11_VIDEO_PROCESSOR_NOMINAL_RANGE_0_255,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VideoProcessorOutputRate {
    Normal = D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_NORMAL,
    Half = D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_HALF,
    Custom = D3D11_VIDEO_PROCESSOR_OUTPUT_RATE_CUSTOM,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoProcessorProcessorCaps(u32);
#[allow(non_upper_case_globals)]
impl VideoProcessorProcessorCaps {
    pub const DeinterlaceBlend: Self = Self(D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_BLEND);
    pub const DeinterlaceBob: Self = Self(D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_BOB);
    pub const DeinterlaceAdaptive: Self =
        Self(D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_ADAPTIVE);
    pub const DeinterlaceMotionCompensation: Self =
        Self(D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_DEINTERLACE_MOTION_COMPENSATION);
    pub const InverseTelecine: Self = Self(D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_INVERSE_TELECINE);
    pub const FrameRateConversion: Self =
        Self(D3D11_VIDEO_PROCESSOR_PROCESSOR_CAPS_FRAME_RATE_CONVERSION);
}
impl_bitflag_operators!(VideoProcessorProcessorCaps);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ProcessorRotation {
    Identity = D3D11_VIDEO_PROCESSOR_ROTATION_IDENTITY,
    _90 = D3D11_VIDEO_PROCESSOR_ROTATION_90,
    _180 = D3D11_VIDEO_PROCESSOR_ROTATION_180,
    _270 = D3D11_VIDEO_PROCESSOR_ROTATION_270,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct VideoProcessorStereoCaps(u32);
#[allow(non_upper_case_globals)]
impl VideoProcessorStereoCaps {
    pub const MonoOffset: Self = Self(D3D11_VIDEO_PROCESSOR_STEREO_CAPS_MONO_OFFSET);
    pub const RowInterleaved: Self = Self(D3D11_VIDEO_PROCESSOR_STEREO_CAPS_ROW_INTERLEAVED);
    pub const ColumnInterleaved: Self = Self(D3D11_VIDEO_PROCESSOR_STEREO_CAPS_COLUMN_INTERLEAVED);
    pub const CapsCheckerBoard: Self = Self(D3D11_VIDEO_PROCESSOR_STEREO_CAPS_CHECKERBOARD);
    pub const FlipMode: Self = Self(D3D11_VIDEO_PROCESSOR_STEREO_CAPS_FLIP_MODE);
}
impl_bitflag_operators!(VideoProcessorStereoCaps);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VideoProcessorStereoFlipMode {
    None = D3D11_VIDEO_PROCESSOR_STEREO_FLIP_NONE,
    Frame0 = D3D11_VIDEO_PROCESSOR_STEREO_FLIP_FRAME0,
    Frame1 = D3D11_VIDEO_PROCESSOR_STEREO_FLIP_FRAME1,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VideoProcessorStereoFormat {
    Mono = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_MONO,
    Horizontal = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_HORIZONTAL,
    Vertical = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_VERTICAL,
    Separate = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_SEPARATE,
    MonoOffset = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_MONO_OFFSET,
    RowInterleaved = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_ROW_INTERLEAVED,
    ColumnInterleaved = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_COLUMN_INTERLEAVED,
    CheckerBoard = D3D11_VIDEO_PROCESSOR_STEREO_FORMAT_CHECKERBOARD,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VideoUsage {
    PlaybackNormal = D3D11_VIDEO_USAGE_PLAYBACK_NORMAL,
    OptimalSpeed = D3D11_VIDEO_USAGE_OPTIMAL_SPEED,
    OptimalQuality = D3D11_VIDEO_USAGE_OPTIMAL_QUALITY,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VPIVDimension {
    Unknown = D3D11_VPIV_DIMENSION_UNKNOWN,
    Texture2D = D3D11_VPIV_DIMENSION_TEXTURE2D,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VPOVDimension {
    Unknown = D3D11_VPOV_DIMENSION_UNKNOWN,
    Texture2D = D3D11_VPOV_DIMENSION_TEXTURE2D,
    Texture2DArray = D3D11_VPOV_DIMENSION_TEXTURE2DARRAY,
}

#[derive(Clone, Debug)]
#[repr(C)]
#[allow(non_snake_case)]
pub struct AESCTRIV {
    pub IV: u64,
    pub Count: u64,
}

#[derive(Clone, Debug)]
pub struct BlendDesc {
    pub alpha_to_coverage_enable: bool,
    pub independent_blend_enable: bool,
    pub render_target: Vec<RenderTargetBlendDesc>,
}
impl BlendDesc {
    fn to_c_struct(&self) -> D3D11_BLEND_DESC {
        assert!(self.render_target.len() <= 8);
        let mut render_target: [D3D11_RENDER_TARGET_BLEND_DESC; 8] = Default::default();
        for i in 0..self.render_target.len() {
            render_target[i] = self.render_target[i].to_c_struct();
        }
        for i in self.render_target.len()..8 {
            render_target[i] = RenderTargetBlendDesc::default().to_c_struct();
        }
        D3D11_BLEND_DESC {
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
impl From<D3D11_BLEND_DESC> for BlendDesc {
    fn from(src: D3D11_BLEND_DESC) -> BlendDesc {
        BlendDesc {
            alpha_to_coverage_enable: src.AlphaToCoverageEnable != 0,
            independent_blend_enable: src.IndependentBlendEnable != 0,
            render_target: src
                .RenderTarget
                .iter()
                .map(|&rt| rt.into())
                .collect::<Vec<_>>(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub struct Box3D {
    pub left: u32,
    pub top: u32,
    pub fromt: u32,
    pub right: u32,
    pub bottom: u32,
    pub back: u32,
}
impl Box3D {
    pub fn as_c_ptr(&self) -> *const D3D11_BOX {
        (self as *const Box3D).cast::<D3D11_BOX>()
    }
}

#[derive(Clone, Debug)]
pub struct BufferDesc<Bw, U, Bf> {
    pub byte_width: Bw,
    pub usage: U,
    pub bind_flags: Bf,
    pub cpu_access_flags: Option<CPUAccessFlags>,
    pub misc_flags: Option<ResourceMiscFlags>,
    pub structure_byte_stride: u32,
}
impl BufferDesc<(), (), ()> {
    pub fn new() -> Self {
        Self {
            byte_width: (),
            usage: (),
            bind_flags: (),
            cpu_access_flags: None,
            misc_flags: None,
            structure_byte_stride: 0,
        }
    }
}
impl<Bw, U, Bf> BufferDesc<Bw, U, Bf> {
    pub fn byte_width(self, byte_width: u32) -> BufferDesc<u32, U, Bf> {
        BufferDesc {
            byte_width,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
            structure_byte_stride: self.structure_byte_stride,
        }
    }
    pub fn usage(self, usage: Usage) -> BufferDesc<Bw, Usage, Bf> {
        BufferDesc {
            byte_width: self.byte_width,
            usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
            structure_byte_stride: self.structure_byte_stride,
        }
    }
    pub fn bind_flags(self, bind_flags: BindFlags) -> BufferDesc<Bw, U, BindFlags> {
        BufferDesc {
            byte_width: self.byte_width,
            usage: self.usage,
            bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
            structure_byte_stride: self.structure_byte_stride,
        }
    }
    pub fn cpu_access_flags(mut self, cpu_access_flags: CPUAccessFlags) -> Self {
        self.cpu_access_flags = Some(cpu_access_flags);
        self
    }
    pub fn misc_flags(mut self, misc_flags: ResourceMiscFlags) -> Self {
        self.misc_flags = Some(misc_flags);
        self
    }
    pub fn structure_byte_stride(mut self, structure_byte_stride: u32) -> Self {
        self.structure_byte_stride = structure_byte_stride;
        self
    }
}
impl BufferDesc<u32, Usage, BindFlags> {
    fn to_c_struct(&self) -> D3D11_BUFFER_DESC {
        D3D11_BUFFER_DESC {
            ByteWidth: self.byte_width,
            Usage: self.usage as u32,
            BindFlags: self.bind_flags.0,
            CPUAccessFlags: self.cpu_access_flags.map_or(0, |f| f.0),
            MiscFlags: self.misc_flags.map_or(0, |f| f.0),
            StructureByteStride: self.structure_byte_stride,
        }
    }
}
impl From<D3D11_BUFFER_DESC> for BufferDesc<u32, Usage, BindFlags> {
    fn from(src: D3D11_BUFFER_DESC) -> BufferDesc<u32, Usage, BindFlags> {
        unsafe {
            BufferDesc {
                byte_width: src.ByteWidth,
                usage: std::mem::transmute(src.Usage),
                bind_flags: BindFlags(src.BindFlags),
                cpu_access_flags: if src.CPUAccessFlags == 0 {
                    None
                } else {
                    Some(CPUAccessFlags(src.CPUAccessFlags))
                },
                misc_flags: if src.MiscFlags == 0 {
                    None
                } else {
                    Some(ResourceMiscFlags(src.MiscFlags))
                },
                structure_byte_stride: src.StructureByteStride,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClassInstanceDesc {
    pub instance_id: u32,
    pub instance_index: u32,
    pub type_id: u32,
    pub constant_buffer: u32,
    pub base_constant_buffer_offset: u32,
    pub base_texture: u32,
    pub base_sampler: u32,
    pub created: bool,
}
impl From<D3D11_CLASS_INSTANCE_DESC> for ClassInstanceDesc {
    fn from(src: D3D11_CLASS_INSTANCE_DESC) -> ClassInstanceDesc {
        ClassInstanceDesc {
            instance_id: src.InstanceId,
            instance_index: src.InstanceIndex,
            type_id: src.TypeId,
            constant_buffer: src.ConstantBuffer,
            base_constant_buffer_offset: src.BaseConstantBufferOffset,
            base_texture: src.BaseTexture,
            base_sampler: src.BaseSampler,
            created: src.Created != 0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CounterDesc {
    pub counter: u32,
    pub misc_flags: u32,
}
impl CounterDesc {
    fn to_c_struct(&self) -> D3D11_COUNTER_DESC {
        D3D11_COUNTER_DESC {
            Counter: self.counter as u32,
            MiscFlags: self.misc_flags,
        }
    }
}
impl From<D3D11_COUNTER_DESC> for CounterDesc {
    fn from(src: D3D11_COUNTER_DESC) -> CounterDesc {
        CounterDesc {
            counter: unsafe { std::mem::transmute(src.Counter) },
            misc_flags: src.MiscFlags,
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct CounterInfo {
    pub last_device_dependent_counter: u32,
    pub num_simultaneous_counters: u32,
    pub num_detectable_parallel_units: u8,
}
impl From<D3D11_COUNTER_INFO> for CounterInfo {
    fn from(src: D3D11_COUNTER_INFO) -> CounterInfo {
        CounterInfo {
            last_device_dependent_counter: src.LastDeviceDependentCounter,
            num_simultaneous_counters: src.NumSimultaneousCounters,
            num_detectable_parallel_units: src.NumDetectableParallelUnits,
        }
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
    fn to_c_struct(&self) -> D3D11_DEPTH_STENCIL_DESC {
        D3D11_DEPTH_STENCIL_DESC {
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
impl From<D3D11_DEPTH_STENCIL_DESC> for DepthStencilDesc {
    fn from(src: D3D11_DEPTH_STENCIL_DESC) -> DepthStencilDesc {
        unsafe {
            DepthStencilDesc {
                depth_enable: src.DepthEnable != 0,
                depth_write_mask: std::mem::transmute(src.DepthWriteMask),
                depth_func: std::mem::transmute(src.DepthFunc),
                stencil_enable: src.StencilEnable != 0,
                stencil_read_mask: src.StencilReadMask,
                stencil_write_mask: src.StencilWriteMask,
                front_face: src.FrontFace.into(),
                back_face: src.BackFace.into(),
            }
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
    fn to_c_struct(&self) -> D3D11_DEPTH_STENCIL_VIEW_DESC {
        let mut obj = D3D11_DEPTH_STENCIL_VIEW_DESC::default();
        match self {
            &DepthStencilViewDesc::Texture1D {
                format,
                flags,
                mip_slice,
            } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = DSVDimension::Texture1D as u32;
                obj.Flags = flags.map_or(0, |f| f.0);
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
                obj.Flags = flags.map_or(0, |f| f.0);
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
                obj.Flags = flags.map_or(0, |f| f.0);
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
                obj.Flags = flags.map_or(0, |f| f.0);
                obj.u.Texture2DArray_mut().MipSlice = mip_slice;
                obj.u.Texture2DArray_mut().FirstArraySlice = first_array_slice;
                obj.u.Texture2DArray_mut().ArraySize = array_size;
            },
            &DepthStencilViewDesc::Texture2DMS { format, flags } => unsafe {
                obj.Format = format as u32;
                obj.ViewDimension = DSVDimension::Texture2DMS as u32;
                obj.Flags = flags.map_or(0, |f| f.0);
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
                obj.Flags = flags.map_or(0, |f| f.0);
                obj.u.Texture2DMSArray_mut().FirstArraySlice = first_array_slice;
                obj.u.Texture2DMSArray_mut().ArraySize = array_size;
            },
        }
        obj
    }
}
impl From<D3D11_DEPTH_STENCIL_VIEW_DESC> for DepthStencilViewDesc {
    fn from(src: D3D11_DEPTH_STENCIL_VIEW_DESC) -> DepthStencilViewDesc {
        unsafe {
            match src.ViewDimension {
                D3D11_DSV_DIMENSION_TEXTURE1D => DepthStencilViewDesc::Texture1D {
                    format: std::mem::transmute(src.Format),
                    flags: if src.Flags == 0 {
                        None
                    } else {
                        Some(DSVFlags(src.Flags))
                    },
                    mip_slice: src.u.Texture1D().MipSlice,
                },
                D3D11_DSV_DIMENSION_TEXTURE1DARRAY => DepthStencilViewDesc::Texture1DArray {
                    format: std::mem::transmute(src.Format),
                    flags: if src.Flags == 0 {
                        None
                    } else {
                        Some(DSVFlags(src.Flags))
                    },
                    mip_slice: src.u.Texture1DArray().MipSlice,
                    first_array_slice: src.u.Texture1DArray().FirstArraySlice,
                    array_size: src.u.Texture1DArray().ArraySize,
                },
                D3D11_DSV_DIMENSION_TEXTURE2D => DepthStencilViewDesc::Texture2D {
                    format: std::mem::transmute(src.Format),
                    flags: if src.Flags == 0 {
                        None
                    } else {
                        Some(DSVFlags(src.Flags))
                    },
                    mip_slice: src.u.Texture1D().MipSlice,
                },
                D3D11_DSV_DIMENSION_TEXTURE2DARRAY => DepthStencilViewDesc::Texture2DArray {
                    format: std::mem::transmute(src.Format),
                    flags: if src.Flags == 0 {
                        None
                    } else {
                        Some(DSVFlags(src.Flags))
                    },
                    mip_slice: src.u.Texture2DArray().MipSlice,
                    first_array_slice: src.u.Texture2DArray().FirstArraySlice,
                    array_size: src.u.Texture2DArray().ArraySize,
                },
                D3D11_DSV_DIMENSION_TEXTURE2DMS => DepthStencilViewDesc::Texture2DMS {
                    format: std::mem::transmute(src.Format),
                    flags: if src.Flags == 0 {
                        None
                    } else {
                        Some(DSVFlags(src.Flags))
                    },
                },
                D3D11_DSV_DIMENSION_TEXTURE2DMSARRAY => DepthStencilViewDesc::Texture2DMSArray {
                    format: std::mem::transmute(src.Format),
                    flags: if src.Flags == 0 {
                        None
                    } else {
                        Some(DSVFlags(src.Flags))
                    },
                    first_array_slice: src.u.Texture2DMSArray().FirstArraySlice,
                    array_size: src.u.Texture2DMSArray().ArraySize,
                },
                _ => unreachable!(),
            }
        }
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
    fn to_c_struct(&self) -> D3D11_DEPTH_STENCILOP_DESC {
        D3D11_DEPTH_STENCILOP_DESC {
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
impl From<D3D11_DEPTH_STENCILOP_DESC> for DepthStencilOpDesc {
    fn from(src: D3D11_DEPTH_STENCILOP_DESC) -> DepthStencilOpDesc {
        unsafe {
            DepthStencilOpDesc {
                stencil_fail_op: std::mem::transmute(src.StencilFailOp),
                stencil_depth_fail_op: std::mem::transmute(src.StencilDepthFailOp),
                stencil_pass_op: std::mem::transmute(src.StencilPassOp),
                stencil_func: std::mem::transmute(src.StencilFunc),
            }
        }
    }
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct DrawIndexedInstancedIndirectArgs {
    pub index_count_per_instance: u32,
    pub instance_count: u32,
    pub start_index_location: u32,
    pub base_vertex_location: i32,
    pub start_instance_location: u32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct DrawInstancedIndirectArgs {
    pub vertex_count_per_instance: u32,
    pub instance_count: u32,
    pub start_vertex_location: u32,
    pub start_instance_location: u32,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct EncryptedBlockInfo {
    pub num_encrypted_bytes_at_beginning: u32,
    pub num_bytes_in_skip_pattern: u32,
    pub num_bytes_in_encrypt_pattern: u32,
}

pub trait CheckFeatureSupport
where
    Self: Sized,
{
    type Args;
    fn check_feature_support(device: *mut ID3D11Device, args: Self::Args) -> Result<Self, HResult>;
}

pub mod feature_data {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct ArchitectureInfo {
        pub tile_based_deferred_renderer: bool,
    }
    impl From<D3D11_FEATURE_DATA_ARCHITECTURE_INFO> for ArchitectureInfo {
        fn from(src: D3D11_FEATURE_DATA_ARCHITECTURE_INFO) -> ArchitectureInfo {
            ArchitectureInfo {
                tile_based_deferred_renderer: src.TileBasedDeferredRenderer != 0,
            }
        }
    }
    impl CheckFeatureSupport for ArchitectureInfo {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_ARCHITECTURE_INFO::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_ARCHITECTURE_INFO,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct D3D10XHardwareOptions {
        pub compute_shaders_plus_raw_and_structured_buffers_via_shader_4_x: bool,
    }
    impl From<D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS> for D3D10XHardwareOptions {
        fn from(src: D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS) -> D3D10XHardwareOptions {
            D3D10XHardwareOptions {
                compute_shaders_plus_raw_and_structured_buffers_via_shader_4_x: src
                    .ComputeShaders_Plus_RawAndStructuredBuffers_Via_Shader_4_x
                    != 0,
            }
        }
    }
    impl CheckFeatureSupport for D3D10XHardwareOptions {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_D3D10_X_HARDWARE_OPTIONS::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_D3D10_X_HARDWARE_OPTIONS,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct D3D11Options {
        pub output_merger_logic_op: bool,
        pub uav_only_rendering_forced_sample_count: bool,
        pub discard_apis_seen_by_driver: bool,
        pub flags_for_update_and_copy_seen_by_driver: bool,
        pub clear_view: bool,
        pub copy_with_overlap: bool,
        pub constant_buffer_partial_update: bool,
        pub constant_buffer_offsetting: bool,
        pub map_no_overwirte_on_dynamic_constant_buffer: bool,
        pub map_no_overwrite_on_dynamic_buffer_srv: bool,
        pub multisample_rtv_with_forced_sample_count_one: bool,
        pub sad_4_shader_instructions: bool,
        pub extended_doubles_shader_instructions: bool,
        pub extended_resource_sharing: bool,
    }
    impl From<D3D11_FEATURE_DATA_D3D11_OPTIONS> for D3D11Options {
        fn from(src: D3D11_FEATURE_DATA_D3D11_OPTIONS) -> D3D11Options {
            D3D11Options {
                output_merger_logic_op: src.OutputMergerLogicOp != 0,
                uav_only_rendering_forced_sample_count: src.UAVOnlyRenderingForcedSampleCount != 0,
                discard_apis_seen_by_driver: src.DiscardAPIsSeenByDriver != 0,
                flags_for_update_and_copy_seen_by_driver: src.FlagsForUpdateAndCopySeenByDriver
                    != 0,
                clear_view: src.ClearView != 0,
                copy_with_overlap: src.CopyWithOverlap != 0,
                constant_buffer_partial_update: src.ConstantBufferPartialUpdate != 0,
                constant_buffer_offsetting: src.ConstantBufferOffsetting != 0,
                map_no_overwirte_on_dynamic_constant_buffer: src
                    .MapNoOverwriteOnDynamicConstantBuffer
                    != 0,
                map_no_overwrite_on_dynamic_buffer_srv: src.MapNoOverwriteOnDynamicBufferSRV != 0,
                multisample_rtv_with_forced_sample_count_one: src
                    .MultisampleRTVWithForcedSampleCountOne
                    != 0,
                sad_4_shader_instructions: src.SAD4ShaderInstructions != 0,
                extended_doubles_shader_instructions: src.ExtendedDoublesShaderInstructions != 0,
                extended_resource_sharing: src.ExtendedResourceSharing != 0,
            }
        }
    }
    impl CheckFeatureSupport for D3D11Options {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_D3D11_OPTIONS::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_D3D11_OPTIONS,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct D3D11Options2 {
        pub ps_specified_stencil_ref_supported: bool,
        pub typed_uav_load_additional_formats: bool,
        pub rovs_supported: bool,
        pub conservative_rasterization_iter: ConservativeRasterizationTier,
        pub tiled_resources_tier: TiledResourcesTier,
        pub map_on_default_textures: bool,
        pub standard_swizzle: bool,
        pub unified_memory_architecture: bool,
    }
    impl From<D3D11_FEATURE_DATA_D3D11_OPTIONS2> for D3D11Options2 {
        fn from(src: D3D11_FEATURE_DATA_D3D11_OPTIONS2) -> D3D11Options2 {
            D3D11Options2 {
                ps_specified_stencil_ref_supported: src.PSSpecifiedStencilRefSupported != 0,
                typed_uav_load_additional_formats: src.TypedUAVLoadAdditionalFormats != 0,
                rovs_supported: src.ROVsSupported != 0,
                conservative_rasterization_iter: unsafe {
                    std::mem::transmute(src.ConservativeRasterizationTier)
                },
                tiled_resources_tier: unsafe { std::mem::transmute(src.TiledResourcesTier) },
                map_on_default_textures: src.MapOnDefaultTextures != 0,
                standard_swizzle: src.StandardSwizzle != 0,
                unified_memory_architecture: src.UnifiedMemoryArchitecture != 0,
            }
        }
    }
    impl CheckFeatureSupport for D3D11Options2 {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_D3D11_OPTIONS2::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_D3D11_OPTIONS2,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct D3D11Options3 {
        pub vp_and_rt_array_index_from_any_shader_feeding_resterizer: bool,
    }
    impl From<D3D11_FEATURE_DATA_D3D11_OPTIONS3> for D3D11Options3 {
        fn from(src: D3D11_FEATURE_DATA_D3D11_OPTIONS3) -> D3D11Options3 {
            D3D11Options3 {
                vp_and_rt_array_index_from_any_shader_feeding_resterizer: src
                    .VPAndRTArrayIndexFromAnyShaderFeedingRasterizer
                    != 0,
            }
        }
    }
    impl CheckFeatureSupport for D3D11Options3 {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_D3D11_OPTIONS3::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_D3D11_OPTIONS3,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct D3D9Options {
        pub full_non_pow2_texture_support: bool,
    }
    impl From<D3D11_FEATURE_DATA_D3D9_OPTIONS> for D3D9Options {
        fn from(src: D3D11_FEATURE_DATA_D3D9_OPTIONS) -> D3D9Options {
            D3D9Options {
                full_non_pow2_texture_support: src.FullNonPow2TextureSupport != 0,
            }
        }
    }
    impl CheckFeatureSupport for D3D9Options {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_D3D9_OPTIONS::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_D3D9_OPTIONS,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct D3D9Options1 {
        pub full_non_pow2_texture_supported: bool,
        pub depth_as_texture_with_less_equal_comparison_filter_supported: bool,
        pub simple_instancing_supported: bool,
        pub texture_cube_face_render_target_with_non_cube_depth_stencil_supported: bool,
    }
    impl From<D3D11_FEATURE_DATA_D3D9_OPTIONS1> for D3D9Options1 {
        fn from(src: D3D11_FEATURE_DATA_D3D9_OPTIONS1) -> D3D9Options1 {
            D3D9Options1 {
                full_non_pow2_texture_supported: src.FullNonPow2TextureSupported != 0,
                depth_as_texture_with_less_equal_comparison_filter_supported: src
                    .DepthAsTextureWithLessEqualComparisonFilterSupported
                    != 0,
                simple_instancing_supported: src.SimpleInstancingSupported != 0,
                texture_cube_face_render_target_with_non_cube_depth_stencil_supported: src
                    .TextureCubeFaceRenderTargetWithNonCubeDepthStencilSupported
                    != 0,
            }
        }
    }
    impl CheckFeatureSupport for D3D9Options1 {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_D3D9_OPTIONS1::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_D3D9_OPTIONS1,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct D3D9ShadowSupport {
        pub supports_depth_as_texture_with_less_equal_comparison_filter: bool,
    }
    impl From<D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT> for D3D9ShadowSupport {
        fn from(src: D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT) -> D3D9ShadowSupport {
            D3D9ShadowSupport {
                supports_depth_as_texture_with_less_equal_comparison_filter: src
                    .SupportsDepthAsTextureWithLessEqualComparisonFilter
                    != 0,
            }
        }
    }
    impl CheckFeatureSupport for D3D9ShadowSupport {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_D3D9_SHADOW_SUPPORT::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_D3D9_SHADOW_SUPPORT,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct D3D9SimpleInstancingSupport {
        pub simple_instancing_supported: bool,
    }
    impl From<D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT> for D3D9SimpleInstancingSupport {
        fn from(
            src: D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT,
        ) -> D3D9SimpleInstancingSupport {
            D3D9SimpleInstancingSupport {
                simple_instancing_supported: src.SimpleInstancingSupported != 0,
            }
        }
    }
    impl CheckFeatureSupport for D3D9SimpleInstancingSupport {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_D3D9_SIMPLE_INSTANCING_SUPPORT::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_D3D9_SIMPLE_INSTANCING_SUPPORT,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Doubles {
        pub double_precision_float_shader_ops: bool,
    }
    impl From<D3D11_FEATURE_DATA_DOUBLES> for Doubles {
        fn from(src: D3D11_FEATURE_DATA_DOUBLES) -> Doubles {
            Doubles {
                double_precision_float_shader_ops: src.DoublePrecisionFloatShaderOps != 0,
            }
        }
    }
    impl CheckFeatureSupport for Doubles {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_DOUBLES::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_DOUBLES,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct FormatSupport {
        pub in_format: dxgi::Format,
        pub out_format_support: super::FormatSupport,
    }
    impl From<D3D11_FEATURE_DATA_FORMAT_SUPPORT> for FormatSupport {
        fn from(src: D3D11_FEATURE_DATA_FORMAT_SUPPORT) -> FormatSupport {
            FormatSupport {
                in_format: unsafe { std::mem::transmute(src.InFormat) },
                out_format_support: super::FormatSupport(src.OutFormatSupport),
            }
        }
    }
    impl CheckFeatureSupport for FormatSupport {
        type Args = dxgi::Format;
        fn check_feature_support(
            device: *mut ID3D11Device,
            in_format: dxgi::Format,
        ) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_FORMAT_SUPPORT::default();
            obj.InFormat = in_format as u32;
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_FORMAT_SUPPORT,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct FormatSupport2 {
        pub in_format: dxgi::Format,
        pub out_format_support2: super::FormatSupport2,
    }
    impl From<D3D11_FEATURE_DATA_FORMAT_SUPPORT2> for FormatSupport2 {
        fn from(src: D3D11_FEATURE_DATA_FORMAT_SUPPORT2) -> FormatSupport2 {
            FormatSupport2 {
                in_format: unsafe { std::mem::transmute(src.InFormat) },
                out_format_support2: super::FormatSupport2(src.OutFormatSupport2),
            }
        }
    }
    impl CheckFeatureSupport for FormatSupport2 {
        type Args = dxgi::Format;
        fn check_feature_support(
            device: *mut ID3D11Device,
            in_format: dxgi::Format,
        ) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_FORMAT_SUPPORT2::default();
            obj.InFormat = in_format as u32;
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_FORMAT_SUPPORT2,
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
    impl From<D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT> for GPUVirtualAddressSupport {
        fn from(src: D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT) -> GPUVirtualAddressSupport {
            GPUVirtualAddressSupport {
                max_gpu_virtual_address_bits_per_resource: src.MaxGPUVirtualAddressBitsPerResource,
                max_gpu_virtual_address_bits_per_process: src.MaxGPUVirtualAddressBitsPerProcess,
            }
        }
    }
    impl CheckFeatureSupport for GPUVirtualAddressSupport {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_GPU_VIRTUAL_ADDRESS_SUPPORT::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_GPU_VIRTUAL_ADDRESS_SUPPORT,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct MarkerSupport {
        pub profile: bool,
    }
    impl From<D3D11_FEATURE_DATA_MARKER_SUPPORT> for MarkerSupport {
        fn from(src: D3D11_FEATURE_DATA_MARKER_SUPPORT) -> MarkerSupport {
            MarkerSupport {
                profile: src.Profile != 0,
            }
        }
    }
    impl CheckFeatureSupport for MarkerSupport {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_MARKER_SUPPORT::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_MARKER_SUPPORT,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct ShaderMinPrecisionSupport {
        pub pixel_shader_min_precition: u32,
        pub all_other_shader_stages_min_precision: u32,
    }
    impl From<D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT> for ShaderMinPrecisionSupport {
        fn from(src: D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT) -> ShaderMinPrecisionSupport {
            ShaderMinPrecisionSupport {
                pixel_shader_min_precition: src.PixelShaderMinPrecision,
                all_other_shader_stages_min_precision: src.AllOtherShaderStagesMinPrecision,
            }
        }
    }
    impl CheckFeatureSupport for ShaderMinPrecisionSupport {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_SHADER_MIN_PRECISION_SUPPORT::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_SHADER_MIN_PRECISION_SUPPORT,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }

    #[derive(Clone, Debug)]
    pub struct Threading {
        pub driver_concurrent_creates: bool,
        pub driver_command_lists: bool,
    }
    impl From<D3D11_FEATURE_DATA_THREADING> for Threading {
        fn from(src: D3D11_FEATURE_DATA_THREADING) -> Threading {
            Threading {
                driver_concurrent_creates: src.DriverConcurrentCreates != 0,
                driver_command_lists: src.DriverCommandLists != 0,
            }
        }
    }
    impl CheckFeatureSupport for Threading {
        type Args = ();
        fn check_feature_support(device: *mut ID3D11Device, _args: ()) -> Result<Self, HResult> {
            let mut obj = D3D11_FEATURE_DATA_THREADING::default();
            let res = unsafe {
                (*device).CheckFeatureSupport(
                    D3D11_FEATURE_THREADING,
                    as_c_void_mut(&mut obj),
                    std::mem::size_of_val(&obj) as u32,
                )
            };
            hresult(obj.into(), res)
        }
    }
}

pub const APPEND_ALIGNED_ELEMENT: u32 = D3D11_APPEND_ALIGNED_ELEMENT;

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
    fn to_c_struct(&self) -> (D3D11_INPUT_ELEMENT_DESC, std::ffi::CString) {
        let name = std::ffi::CString::new(self.semantic_name).unwrap();
        (
            D3D11_INPUT_ELEMENT_DESC {
                SemanticName: name.as_ptr(),
                SemanticIndex: self.semantic_index,
                Format: self.format as u32,
                InputSlot: self.input_slot,
                AlignedByteOffset: self.aligned_byte_offset,
                InputSlotClass: self.input_slot_class as u32,
                InstanceDataStepRate: self.instance_data_step_rate,
            },
            name,
        )
    }
}

#[macro_export]
macro_rules! d3d11_input_element_descs {
    ($({$name: expr, $index: expr, $format: expr, $slot: expr, $offset: expr, $class: expr, $rate: expr},)*) => {
        vec![
            $(
                $crate::d3d11::InputElementDesc {
                    semantic_name: $name,
                    semantic_index: $index,
                    format: $format,
                    input_slot: $slot,
                    aligned_byte_offset: $offset,
                    input_slot_class: $class,
                    instance_data_step_rate: $rate,
                },
            )*
        ]
    };
}

#[derive(Clone, Debug)]
pub struct MappedSubresource {
    data: *mut c_void,
    row_pitch: u32,
    depth_pitch: u32,
}
impl From<D3D11_MAPPED_SUBRESOURCE> for MappedSubresource {
    fn from(src: D3D11_MAPPED_SUBRESOURCE) -> MappedSubresource {
        MappedSubresource {
            data: src.pData,
            row_pitch: src.RowPitch,
            depth_pitch: src.DepthPitch,
        }
    }
}

/*
#[derive(Clone, Debug)]
pub struct OMAC {
    omac: [u8; 16],
}
impl OMAC {
    fn to_c_struct(&self) -> D3D11_OMAC {
        D3D11_OMAC {
            Omac: self.omac.clone(),
        }
    }
}
*/

pub mod query_data {
    use super::*;

    #[derive(Clone, Debug)]
    pub struct PipelineStatistics {
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
    impl From<D3D11_QUERY_DATA_PIPELINE_STATISTICS> for PipelineStatistics {
        fn from(src: D3D11_QUERY_DATA_PIPELINE_STATISTICS) -> PipelineStatistics {
            PipelineStatistics {
                ia_vertices: src.IAVertices,
                ia_primitives: src.IAPrimitives,
                vs_invocations: src.VSInvocations,
                gs_invocations: src.GSInvocations,
                gs_primitives: src.GSPrimitives,
                c_invocations: src.CInvocations,
                c_primitives: src.CPrimitives,
                ps_invocations: src.PSInvocations,
                hs_invocations: src.HSInvocations,
                ds_invocations: src.DSInvocations,
                cs_invocations: src.CSInvocations,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct SOStatistics {
        pub num_primitives_written: u64,
        pub primitives_storage_needed: u64,
    }
    impl From<D3D11_QUERY_DATA_SO_STATISTICS> for SOStatistics {
        fn from(src: D3D11_QUERY_DATA_SO_STATISTICS) -> SOStatistics {
            SOStatistics {
                num_primitives_written: src.NumPrimitivesWritten,
                primitives_storage_needed: src.PrimitivesStorageNeeded,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct TimestampDisjoint {
        pub frequency: u64,
        pub disjoint: bool,
    }
    impl From<D3D11_QUERY_DATA_TIMESTAMP_DISJOINT> for TimestampDisjoint {
        fn from(src: D3D11_QUERY_DATA_TIMESTAMP_DISJOINT) -> TimestampDisjoint {
            TimestampDisjoint {
                frequency: src.Frequency,
                disjoint: src.Disjoint != 0,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct QueryDesc {
    pub query: QueryType,
    pub misc_flags: Option<QueryMiscFlags>,
}
impl QueryDesc {
    fn to_c_struct(&self) -> D3D11_QUERY_DESC {
        D3D11_QUERY_DESC {
            Query: self.query as u32,
            MiscFlags: self.misc_flags.map_or(0, |f| f.0),
        }
    }
}
impl From<D3D11_QUERY_DESC> for QueryDesc {
    fn from(src: D3D11_QUERY_DESC) -> QueryDesc {
        QueryDesc {
            query: unsafe { std::mem::transmute(src.Query) },
            misc_flags: if src.MiscFlags == 0 {
                None
            } else {
                Some(QueryMiscFlags(src.MiscFlags))
            },
        }
    }
}

pub const DEFAULT_DEPTH_BIAS: u32 = D3D11_DEFAULT_DEPTH_BIAS;
pub const DEFAULT_DEPTH_BIAS_CLAMP: f32 = D3D11_DEFAULT_DEPTH_BIAS_CLAMP;
pub const DEFAULT_SLOPE_SCALED_DEPTH_BIAS: f32 = D3D11_DEFAULT_SLOPE_SCALED_DEPTH_BIAS;

#[derive(Clone, Debug)]
pub struct RasterizerDesc {
    pub fill_mode: FillMode,
    pub cull_mode: CullMode,
    pub front_counter_clockwise: bool,
    pub depth_bias: i32,
    pub depth_bias_clamp: f32,
    pub slope_scaled_depth_bias: f32,
    pub depth_clip_enable: bool,
    pub scissor_enable: bool,
    pub multisample_enable: bool,
    pub antialiased_line_enable: bool,
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
    pub fn scissor_enable(mut self, scissor_enable: bool) -> Self {
        self.scissor_enable = scissor_enable;
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
    fn to_c_struct(&self) -> D3D11_RASTERIZER_DESC {
        D3D11_RASTERIZER_DESC {
            FillMode: self.fill_mode as u32,
            CullMode: self.cull_mode as u32,
            FrontCounterClockwise: to_BOOL(self.front_counter_clockwise),
            DepthBias: self.depth_bias,
            DepthBiasClamp: self.depth_bias_clamp,
            SlopeScaledDepthBias: self.slope_scaled_depth_bias,
            DepthClipEnable: to_BOOL(self.depth_clip_enable),
            ScissorEnable: to_BOOL(self.scissor_enable),
            MultisampleEnable: to_BOOL(self.multisample_enable),
            AntialiasedLineEnable: to_BOOL(self.antialiased_line_enable),
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
            scissor_enable: false,
            multisample_enable: false,
            antialiased_line_enable: false,
        }
    }
}
impl From<D3D11_RASTERIZER_DESC> for RasterizerDesc {
    fn from(src: D3D11_RASTERIZER_DESC) -> RasterizerDesc {
        unsafe {
            RasterizerDesc {
                fill_mode: std::mem::transmute(src.FillMode),
                cull_mode: std::mem::transmute(src.CullMode),
                front_counter_clockwise: src.FrontCounterClockwise != 0,
                depth_bias: src.DepthBias,
                depth_bias_clamp: src.DepthBiasClamp,
                slope_scaled_depth_bias: src.SlopeScaledDepthBias,
                depth_clip_enable: src.DepthClipEnable != 0,
                scissor_enable: src.ScissorEnable != 0,
                multisample_enable: src.MultisampleEnable != 0,
                antialiased_line_enable: src.AntialiasedLineEnable != 0,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct RenderTargetBlendDesc {
    pub blend_enable: bool,
    pub src_blend: Blend,
    pub dest_blend: Blend,
    pub blend_op: BlendOp,
    pub src_blend_alpha: Blend,
    pub dest_blend_alpha: Blend,
    pub blend_op_alpha: BlendOp,
    pub render_target_write_mask: ColorWriteEnable,
}
impl RenderTargetBlendDesc {
    fn to_c_struct(&self) -> D3D11_RENDER_TARGET_BLEND_DESC {
        D3D11_RENDER_TARGET_BLEND_DESC {
            BlendEnable: to_BOOL(self.blend_enable),
            SrcBlend: self.src_blend as u32,
            DestBlend: self.dest_blend as u32,
            BlendOp: self.blend_op as u32,
            SrcBlendAlpha: self.src_blend_alpha as u32,
            DestBlendAlpha: self.dest_blend_alpha as u32,
            BlendOpAlpha: self.blend_op_alpha as u32,
            RenderTargetWriteMask: self.render_target_write_mask.0 as u8,
        }
    }
}
impl Default for RenderTargetBlendDesc {
    fn default() -> Self {
        Self {
            blend_enable: false,
            src_blend: Blend::One,
            dest_blend: Blend::Zero,
            blend_op: BlendOp::Add,
            src_blend_alpha: Blend::One,
            dest_blend_alpha: Blend::Zero,
            blend_op_alpha: BlendOp::Add,
            render_target_write_mask: ColorWriteEnable::All,
        }
    }
}
impl From<D3D11_RENDER_TARGET_BLEND_DESC> for RenderTargetBlendDesc {
    fn from(src: D3D11_RENDER_TARGET_BLEND_DESC) -> RenderTargetBlendDesc {
        unsafe {
            RenderTargetBlendDesc {
                blend_enable: src.BlendEnable != 0,
                src_blend: std::mem::transmute(src.SrcBlend),
                dest_blend: std::mem::transmute(src.DestBlend),
                blend_op: std::mem::transmute(src.BlendOp),
                src_blend_alpha: std::mem::transmute(src.SrcBlendAlpha),
                dest_blend_alpha: std::mem::transmute(src.DestBlendAlpha),
                blend_op_alpha: std::mem::transmute(src.BlendOpAlpha),
                render_target_write_mask: ColorWriteEnable(src.RenderTargetWriteMask as u32),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum RenderTargetViewDesc {
    Buffer {
        format: dxgi::Format,
        first_element: u32,
        num_elements: u32,
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
    },
    Texture2DArray {
        format: dxgi::Format,
        mip_slice: u32,
        first_array_slice: u32,
        array_size: u32,
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
    fn to_c_struct(&self) -> D3D11_RENDER_TARGET_VIEW_DESC {
        let mut desc = D3D11_RENDER_TARGET_VIEW_DESC::default();
        match self {
            &RenderTargetViewDesc::Buffer {
                format,
                first_element,
                num_elements,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_RTV_DIMENSION_BUFFER;
                *desc.u.Buffer_mut().u1.FirstElement_mut() = first_element;
                *desc.u.Buffer_mut().u2.NumElements_mut() = num_elements;
            },
            &RenderTargetViewDesc::Texture1D { format, mip_slice } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_RTV_DIMENSION_TEXTURE1D;
                desc.u.Texture1D_mut().MipSlice = mip_slice;
            },
            &RenderTargetViewDesc::Texture1DArray {
                format,
                mip_slice,
                first_array_slice,
                array_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_RTV_DIMENSION_TEXTURE1DARRAY;
                desc.u.Texture1DArray_mut().MipSlice = mip_slice;
                desc.u.Texture1DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture1DArray_mut().ArraySize = array_size;
            },
            &RenderTargetViewDesc::Texture2D { format, mip_slice } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_RTV_DIMENSION_TEXTURE2D;
                desc.u.Texture2D_mut().MipSlice = mip_slice;
            },
            &RenderTargetViewDesc::Texture2DArray {
                format,
                mip_slice,
                first_array_slice,
                array_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_RTV_DIMENSION_TEXTURE2DARRAY;
                desc.u.Texture2DArray_mut().MipSlice = mip_slice;
                desc.u.Texture2DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture2DArray_mut().ArraySize = array_size;
            },
            &RenderTargetViewDesc::Texture2DMS { format } => {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_RTV_DIMENSION_TEXTURE2DMS;
            }
            &RenderTargetViewDesc::Texture2DMSArray {
                format,
                first_array_slice,
                array_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_RTV_DIMENSION_TEXTURE2DMSARRAY;
                desc.u.Texture2DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture2DArray_mut().ArraySize = array_size;
            },
            &RenderTargetViewDesc::Texture3D {
                format,
                mip_slice,
                first_w_slice,
                w_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_RTV_DIMENSION_TEXTURE3D;
                desc.u.Texture3D_mut().MipSlice = mip_slice;
                desc.u.Texture3D_mut().FirstWSlice = first_w_slice;
                desc.u.Texture3D_mut().WSize = w_size;
            },
        }
        desc
    }
}
impl From<D3D11_RENDER_TARGET_VIEW_DESC> for RenderTargetViewDesc {
    fn from(src: D3D11_RENDER_TARGET_VIEW_DESC) -> RenderTargetViewDesc {
        unsafe {
            match src.ViewDimension {
                D3D11_RTV_DIMENSION_BUFFER => RenderTargetViewDesc::Buffer {
                    format: std::mem::transmute(src.Format),
                    first_element: *src.u.Buffer().u1.FirstElement(),
                    num_elements: *src.u.Buffer().u2.NumElements(),
                },
                D3D11_RTV_DIMENSION_TEXTURE1D => RenderTargetViewDesc::Texture1D {
                    format: std::mem::transmute(src.Format),
                    mip_slice: src.u.Texture1D().MipSlice,
                },
                D3D11_RTV_DIMENSION_TEXTURE1DARRAY => RenderTargetViewDesc::Texture1DArray {
                    format: std::mem::transmute(src.Format),
                    mip_slice: src.u.Texture1DArray().MipSlice,
                    first_array_slice: src.u.Texture1DArray().FirstArraySlice,
                    array_size: src.u.Texture1DArray().ArraySize,
                },
                D3D11_RTV_DIMENSION_TEXTURE2D => RenderTargetViewDesc::Texture2D {
                    format: std::mem::transmute(src.Format),
                    mip_slice: src.u.Texture2D().MipSlice,
                },
                D3D11_RTV_DIMENSION_TEXTURE2DARRAY => RenderTargetViewDesc::Texture2DArray {
                    format: std::mem::transmute(src.Format),
                    mip_slice: src.u.Texture2DArray().MipSlice,
                    first_array_slice: src.u.Texture2DArray().FirstArraySlice,
                    array_size: src.u.Texture2DArray().ArraySize,
                },
                D3D11_RTV_DIMENSION_TEXTURE2DMS => RenderTargetViewDesc::Texture2DMS {
                    format: std::mem::transmute(src.Format),
                },
                D3D11_RTV_DIMENSION_TEXTURE2DMSARRAY => RenderTargetViewDesc::Texture2DMSArray {
                    format: std::mem::transmute(src.Format),
                    first_array_slice: src.u.Texture2DMSArray().FirstArraySlice,
                    array_size: src.u.Texture2DMSArray().ArraySize,
                },
                D3D11_RTV_DIMENSION_TEXTURE3D => RenderTargetViewDesc::Texture3D {
                    format: std::mem::transmute(src.Format),
                    mip_slice: src.u.Texture3D().MipSlice,
                    first_w_slice: src.u.Texture3D().FirstWSlice,
                    w_size: src.u.Texture3D().WSize,
                },
                _ => unreachable!(),
            }
        }
    }
}

pub const MAX_MAXANISOTROPY: u32 = D3D11_MAX_MAXANISOTROPY;

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
    fn to_c_struct(&self) -> D3D11_SAMPLER_DESC {
        D3D11_SAMPLER_DESC {
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
impl From<D3D11_SAMPLER_DESC> for SamplerDesc<Filter> {
    fn from(src: D3D11_SAMPLER_DESC) -> SamplerDesc<Filter> {
        unsafe {
            SamplerDesc {
                filter: std::mem::transmute(src.Filter),
                address_u: std::mem::transmute(src.AddressU),
                address_v: std::mem::transmute(src.AddressV),
                address_w: std::mem::transmute(src.AddressW),
                mip_lod_bias: src.MipLODBias,
                max_anisotropy: src.MaxAnisotropy,
                comparison_func: std::mem::transmute(src.ComparisonFunc),
                border_color: dxgi::RGBA {
                    r: src.BorderColor[0],
                    g: src.BorderColor[1],
                    b: src.BorderColor[2],
                    a: src.BorderColor[3],
                },
                min_lod: src.MinLOD,
                max_lod: src.MaxLOD,
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum ShaderResourceViewDesc {
    Buffer {
        format: dxgi::Format,
        first_element: u32,
        num_elements: u32,
    },
    Texture1D {
        format: dxgi::Format,
        most_detailed_mip: u32,
        mip_levels: u32,
    },
    Texture1DArray {
        format: dxgi::Format,
        most_detailed_mip: u32,
        mip_levels: u32,
        first_array_slice: u32,
        array_size: u32,
    },
    Texture2D {
        format: dxgi::Format,
        most_detailed_mip: u32,
        mip_levels: u32,
    },
    Texture2DArray {
        format: dxgi::Format,
        most_detailed_mip: u32,
        mip_levels: u32,
        first_array_slice: u32,
        array_size: u32,
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
        most_detailed_mip: u32,
        mip_levels: u32,
    },
    TextureCube {
        format: dxgi::Format,
        most_detailed_mip: u32,
        mip_levels: u32,
    },
    TextureCubeArray {
        format: dxgi::Format,
        most_detailed_mip: u32,
        mip_levels: u32,
        first_2d_array_face: u32,
        num_cubes: u32,
    },
    BufferEx {
        format: dxgi::Format,
        first_element: u32,
        num_elements: u32,
        flags: Option<BufferExSRVFlags>,
    },
}
impl ShaderResourceViewDesc {
    fn to_c_struct(&self) -> D3D11_SHADER_RESOURCE_VIEW_DESC {
        let mut desc = D3D11_SHADER_RESOURCE_VIEW_DESC::default();
        match self {
            &ShaderResourceViewDesc::Buffer {
                format,
                first_element,
                num_elements,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_BUFFER;
                *desc.u.Buffer_mut().u1.FirstElement_mut() = first_element;
                *desc.u.Buffer_mut().u2.NumElements_mut() = num_elements;
            },
            &ShaderResourceViewDesc::Texture1D {
                format,
                most_detailed_mip,
                mip_levels,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE1D;
                desc.u.Texture1D_mut().MostDetailedMip = most_detailed_mip;
                desc.u.Texture1D_mut().MipLevels = mip_levels;
            },
            &ShaderResourceViewDesc::Texture1DArray {
                format,
                most_detailed_mip,
                mip_levels,
                first_array_slice,
                array_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE1DARRAY;
                desc.u.Texture1DArray_mut().MostDetailedMip = most_detailed_mip;
                desc.u.Texture1DArray_mut().MipLevels = mip_levels;
                desc.u.Texture1DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture1DArray_mut().ArraySize = array_size;
            },
            &ShaderResourceViewDesc::Texture2D {
                format,
                most_detailed_mip,
                mip_levels,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2D;
                desc.u.Texture2D_mut().MostDetailedMip = most_detailed_mip;
                desc.u.Texture2D_mut().MipLevels = mip_levels;
            },
            &ShaderResourceViewDesc::Texture2DArray {
                format,
                most_detailed_mip,
                mip_levels,
                first_array_slice,
                array_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2DARRAY;
                desc.u.Texture2DArray_mut().MostDetailedMip = most_detailed_mip;
                desc.u.Texture2DArray_mut().MipLevels = mip_levels;
                desc.u.Texture2DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture2DArray_mut().ArraySize = array_size;
            },
            &ShaderResourceViewDesc::Texture2DMS { format } => {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2DMS;
            }
            &ShaderResourceViewDesc::Texture2DMSArray {
                format,
                first_array_slice,
                array_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE2DMSARRAY;
                desc.u.Texture2DMSArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture2DMSArray_mut().ArraySize = array_size;
            },
            &ShaderResourceViewDesc::Texture3D {
                format,
                most_detailed_mip,
                mip_levels,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURE3D;
                desc.u.Texture3D_mut().MostDetailedMip = most_detailed_mip;
                desc.u.Texture3D_mut().MipLevels = mip_levels;
            },
            &ShaderResourceViewDesc::TextureCube {
                format,
                most_detailed_mip,
                mip_levels,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURECUBE;
                desc.u.TextureCube_mut().MostDetailedMip = most_detailed_mip;
                desc.u.TextureCube_mut().MipLevels = mip_levels;
            },
            &ShaderResourceViewDesc::TextureCubeArray {
                format,
                most_detailed_mip,
                mip_levels,
                first_2d_array_face,
                num_cubes,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_TEXTURECUBEARRAY;
                desc.u.TextureCubeArray_mut().MostDetailedMip = most_detailed_mip;
                desc.u.TextureCubeArray_mut().MipLevels = mip_levels;
                desc.u.TextureCubeArray_mut().First2DArrayFace = first_2d_array_face;
                desc.u.TextureCubeArray_mut().NumCubes = num_cubes;
            },
            &ShaderResourceViewDesc::BufferEx {
                format,
                first_element,
                num_elements,
                flags,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_SRV_DIMENSION_BUFFEREX;
                desc.u.BufferEx_mut().FirstElement = first_element;
                desc.u.BufferEx_mut().NumElements = num_elements;
                desc.u.BufferEx_mut().Flags = flags.map_or(0, |f| f.0);
            },
        }
        desc
    }
}
impl From<D3D11_SHADER_RESOURCE_VIEW_DESC> for ShaderResourceViewDesc {
    fn from(src: D3D11_SHADER_RESOURCE_VIEW_DESC) -> ShaderResourceViewDesc {
        unsafe {
            match src.ViewDimension {
                D3D11_SRV_DIMENSION_BUFFER => ShaderResourceViewDesc::Buffer {
                    format: std::mem::transmute(src.Format),
                    first_element: *src.u.Buffer().u1.FirstElement(),
                    num_elements: *src.u.Buffer().u2.NumElements(),
                },
                D3D11_SRV_DIMENSION_TEXTURE1D => ShaderResourceViewDesc::Texture1D {
                    format: std::mem::transmute(src.Format),
                    most_detailed_mip: src.u.Texture1D().MostDetailedMip,
                    mip_levels: src.u.Texture1D().MipLevels,
                },
                D3D11_SRV_DIMENSION_TEXTURE1DARRAY => ShaderResourceViewDesc::Texture1DArray {
                    format: std::mem::transmute(src.Format),
                    most_detailed_mip: src.u.Texture1DArray().MostDetailedMip,
                    mip_levels: src.u.Texture1DArray().MipLevels,
                    first_array_slice: src.u.Texture1DArray().FirstArraySlice,
                    array_size: src.u.Texture1DArray().ArraySize,
                },
                D3D11_SRV_DIMENSION_TEXTURE2D => ShaderResourceViewDesc::Texture2D {
                    format: std::mem::transmute(src.Format),
                    most_detailed_mip: src.u.Texture2D().MostDetailedMip,
                    mip_levels: src.u.Texture2D().MipLevels,
                },
                D3D11_SRV_DIMENSION_TEXTURE2DARRAY => ShaderResourceViewDesc::Texture2DArray {
                    format: std::mem::transmute(src.Format),
                    most_detailed_mip: src.u.Texture2DArray().MostDetailedMip,
                    mip_levels: src.u.Texture2DArray().MipLevels,
                    first_array_slice: src.u.Texture2DArray().FirstArraySlice,
                    array_size: src.u.Texture2DArray().ArraySize,
                },
                D3D11_SRV_DIMENSION_TEXTURE2DMS => ShaderResourceViewDesc::Texture2DMS {
                    format: std::mem::transmute(src.Format),
                },
                D3D11_SRV_DIMENSION_TEXTURE2DMSARRAY => ShaderResourceViewDesc::Texture2DMSArray {
                    format: std::mem::transmute(src.Format),
                    first_array_slice: src.u.Texture2DMSArray().FirstArraySlice,
                    array_size: src.u.Texture2DMSArray().ArraySize,
                },
                D3D11_SRV_DIMENSION_TEXTURE3D => ShaderResourceViewDesc::Texture3D {
                    format: std::mem::transmute(src.Format),
                    most_detailed_mip: src.u.Texture3D().MostDetailedMip,
                    mip_levels: src.u.Texture3D().MipLevels,
                },
                D3D11_SRV_DIMENSION_TEXTURECUBE => ShaderResourceViewDesc::TextureCube {
                    format: std::mem::transmute(src.Format),
                    most_detailed_mip: src.u.TextureCube().MostDetailedMip,
                    mip_levels: src.u.TextureCube().MipLevels,
                },
                D3D11_SRV_DIMENSION_TEXTURECUBEARRAY => ShaderResourceViewDesc::TextureCubeArray {
                    format: std::mem::transmute(src.Format),
                    most_detailed_mip: src.u.TextureCubeArray().MostDetailedMip,
                    mip_levels: src.u.TextureCubeArray().MipLevels,
                    first_2d_array_face: src.u.TextureCubeArray().First2DArrayFace,
                    num_cubes: src.u.TextureCubeArray().NumCubes,
                },
                D3D11_SRV_DIMENSION_BUFFEREX => ShaderResourceViewDesc::BufferEx {
                    format: std::mem::transmute(src.Format),
                    first_element: src.u.BufferEx().FirstElement,
                    num_elements: src.u.BufferEx().NumElements,
                    flags: if src.u.BufferEx().Flags == 0 {
                        None
                    } else {
                        Some(BufferExSRVFlags(src.u.BufferEx().Flags))
                    },
                },
                _ => unreachable!(),
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct SODeclarationEntry<'a> {
    pub stream: u32,
    pub semantic_name: Option<&'a str>,
    pub semantic_index: u32,
    pub start_component: u8,
    pub component_count: u8,
    pub output_slot: u8,
}
impl<'a> SODeclarationEntry<'a> {
    fn to_c_struct(&self) -> (D3D11_SO_DECLARATION_ENTRY, Option<std::ffi::CString>) {
        let name = self
            .semantic_name
            .map_or(None, |s| Some(std::ffi::CString::new(s).unwrap()));
        (
            D3D11_SO_DECLARATION_ENTRY {
                Stream: self.stream,
                SemanticName: name.as_ref().map_or(std::ptr::null(), |n| n.as_ptr()),
                SemanticIndex: self.semantic_index,
                StartComponent: self.start_component,
                ComponentCount: self.component_count,
                OutputSlot: self.output_slot,
            },
            name,
        )
    }
}

#[derive(Clone, Debug)]
pub struct SubresourceData {
    pub sys_mem: *const c_void,
    pub sys_mem_pitch: u32,
    pub sys_mem_slice_pitch: u32,
}
impl SubresourceData {
    fn to_c_struct(&self) -> D3D11_SUBRESOURCE_DATA {
        D3D11_SUBRESOURCE_DATA {
            pSysMem: self.sys_mem,
            SysMemPitch: self.sys_mem_pitch,
            SysMemSlicePitch: self.sys_mem_slice_pitch,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Texture1DDesc<W, F, U> {
    pub width: W,
    pub mip_levels: u32,
    pub array_size: u32,
    pub format: F,
    pub usage: U,
    pub bind_flags: BindFlags,
    pub cpu_access_flags: Option<CPUAccessFlags>,
    pub misc_flags: Option<ResourceMiscFlags>,
}
impl Texture1DDesc<(), (), ()> {
    pub fn new() -> Self {
        Self {
            width: (),
            mip_levels: 0,
            array_size: 1,
            format: (),
            usage: (),
            bind_flags: BindFlags::ShaderResource,
            cpu_access_flags: None,
            misc_flags: None,
        }
    }
}
impl<W, F, U> Texture1DDesc<W, F, U> {
    pub fn width(self, width: u32) -> Texture1DDesc<u32, F, U> {
        Texture1DDesc {
            width,
            mip_levels: self.mip_levels,
            array_size: self.array_size,
            format: self.format,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.mip_levels = mip_levels;
        self
    }
    pub fn array_size(mut self, array_size: u32) -> Self {
        self.array_size = array_size;
        self
    }
    pub fn format(self, format: dxgi::Format) -> Texture1DDesc<W, dxgi::Format, U> {
        Texture1DDesc {
            width: self.width,
            mip_levels: self.mip_levels,
            array_size: self.array_size,
            format,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn usage(self, usage: Usage) -> Texture1DDesc<W, F, Usage> {
        Texture1DDesc {
            width: self.width,
            mip_levels: self.mip_levels,
            array_size: self.array_size,
            format: self.format,
            usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn bind_flags(mut self, bind_flags: BindFlags) -> Self {
        self.bind_flags = bind_flags;
        self
    }
    pub fn cpu_access_flags(mut self, cpu_access_flags: CPUAccessFlags) -> Self {
        self.cpu_access_flags = Some(cpu_access_flags);
        self
    }
    pub fn misc_flags(mut self, misc_flags: ResourceMiscFlags) -> Self {
        self.misc_flags = Some(misc_flags);
        self
    }
}
impl Texture1DDesc<u32, dxgi::Format, Usage> {
    fn to_c_struct(&self) -> D3D11_TEXTURE1D_DESC {
        D3D11_TEXTURE1D_DESC {
            Width: self.width,
            MipLevels: self.mip_levels,
            ArraySize: self.array_size,
            Format: self.format as u32,
            Usage: self.usage as u32,
            BindFlags: self.bind_flags.0,
            CPUAccessFlags: self.cpu_access_flags.map_or(0, |f| f.0),
            MiscFlags: self.misc_flags.map_or(0, |f| f.0),
        }
    }
}
impl From<D3D11_TEXTURE1D_DESC> for Texture1DDesc<u32, dxgi::Format, Usage> {
    fn from(src: D3D11_TEXTURE1D_DESC) -> Texture1DDesc<u32, dxgi::Format, Usage> {
        unsafe {
            Texture1DDesc {
                width: src.Width,
                mip_levels: src.MipLevels,
                array_size: src.ArraySize,
                format: std::mem::transmute(src.Format),
                usage: std::mem::transmute(src.Usage),
                bind_flags: BindFlags(src.BindFlags),
                cpu_access_flags: if src.CPUAccessFlags == 0 {
                    None
                } else {
                    Some(CPUAccessFlags(src.CPUAccessFlags))
                },
                misc_flags: if src.MiscFlags == 0 {
                    None
                } else {
                    Some(ResourceMiscFlags(src.MiscFlags))
                },
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Texture2DDesc<W, H, F, U> {
    pub width: W,
    pub height: H,
    pub mip_levels: u32,
    pub array_size: u32,
    pub format: F,
    pub sample_desc: dxgi::SampleDesc,
    pub usage: U,
    pub bind_flags: BindFlags,
    pub cpu_access_flags: Option<CPUAccessFlags>,
    pub misc_flags: Option<ResourceMiscFlags>,
}
impl Texture2DDesc<(), (), (), ()> {
    pub fn new() -> Self {
        Self {
            width: (),
            height: (),
            mip_levels: 1,
            array_size: 1,
            format: (),
            sample_desc: Default::default(),
            usage: (),
            bind_flags: BindFlags::ShaderResource,
            cpu_access_flags: None,
            misc_flags: None,
        }
    }
}
impl<W, H, F, U> Texture2DDesc<W, H, F, U> {
    pub fn width(self, width: u32) -> Texture2DDesc<u32, H, F, U> {
        Texture2DDesc {
            width,
            height: self.height,
            mip_levels: self.mip_levels,
            array_size: self.array_size,
            format: self.format,
            sample_desc: self.sample_desc,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn height(self, height: u32) -> Texture2DDesc<W, u32, F, U> {
        Texture2DDesc {
            width: self.width,
            height,
            mip_levels: self.mip_levels,
            array_size: self.array_size,
            format: self.format,
            sample_desc: self.sample_desc,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.mip_levels = mip_levels;
        self
    }
    pub fn array_size(mut self, array_size: u32) -> Self {
        self.array_size = array_size;
        self
    }
    pub fn format(self, format: dxgi::Format) -> Texture2DDesc<W, H, dxgi::Format, U> {
        Texture2DDesc {
            width: self.width,
            height: self.height,
            mip_levels: self.mip_levels,
            array_size: self.array_size,
            format,
            sample_desc: self.sample_desc,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn sample_desc(mut self, sample_desc: dxgi::SampleDesc) -> Self {
        self.sample_desc = sample_desc;
        self
    }
    pub fn usage(self, usage: Usage) -> Texture2DDesc<W, H, F, Usage> {
        Texture2DDesc {
            width: self.width,
            height: self.height,
            mip_levels: self.mip_levels,
            array_size: self.array_size,
            format: self.format,
            sample_desc: self.sample_desc,
            usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn bind_flags(mut self, bind_flags: BindFlags) -> Self {
        self.bind_flags = bind_flags;
        self
    }
    pub fn cpu_access_flags(mut self, cpu_access_flags: CPUAccessFlags) -> Self {
        self.cpu_access_flags = Some(cpu_access_flags);
        self
    }
    pub fn misc_flags(mut self, misc_flags: ResourceMiscFlags) -> Self {
        self.misc_flags = Some(misc_flags);
        self
    }
}
impl Texture2DDesc<u32, u32, dxgi::Format, Usage> {
    fn to_c_struct(&self) -> D3D11_TEXTURE2D_DESC {
        D3D11_TEXTURE2D_DESC {
            Width: self.width,
            Height: self.height,
            MipLevels: self.mip_levels,
            ArraySize: self.array_size,
            Format: self.format as u32,
            SampleDesc: self.sample_desc.to_c_struct(),
            Usage: self.usage as u32,
            BindFlags: self.bind_flags.0,
            CPUAccessFlags: self.cpu_access_flags.map_or(0, |f| f.0),
            MiscFlags: self.misc_flags.map_or(0, |f| f.0),
        }
    }
}
impl From<D3D11_TEXTURE2D_DESC> for Texture2DDesc<u32, u32, dxgi::Format, Usage> {
    fn from(src: D3D11_TEXTURE2D_DESC) -> Texture2DDesc<u32, u32, dxgi::Format, Usage> {
        unsafe {
            Texture2DDesc {
                width: src.Width,
                height: src.Height,
                mip_levels: src.MipLevels,
                array_size: src.ArraySize,
                format: std::mem::transmute(src.Format),
                sample_desc: src.SampleDesc.into(),
                usage: std::mem::transmute(src.Usage),
                bind_flags: BindFlags(src.BindFlags),
                cpu_access_flags: if src.CPUAccessFlags == 0 {
                    None
                } else {
                    Some(CPUAccessFlags(src.CPUAccessFlags))
                },
                misc_flags: if src.MiscFlags == 0 {
                    None
                } else {
                    Some(ResourceMiscFlags(src.MiscFlags))
                },
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Texture3DDesc<W, H, D, F, U> {
    pub width: W,
    pub height: H,
    pub depth: D,
    pub mip_levels: u32,
    pub format: F,
    pub usage: U,
    pub bind_flags: BindFlags,
    pub cpu_access_flags: Option<CPUAccessFlags>,
    pub misc_flags: Option<ResourceMiscFlags>,
}
impl Texture3DDesc<(), (), (), (), ()> {
    pub fn new() -> Self {
        Self {
            width: (),
            height: (),
            depth: (),
            mip_levels: 0,
            format: (),
            usage: (),
            bind_flags: BindFlags::ShaderResource,
            cpu_access_flags: None,
            misc_flags: None,
        }
    }
}
impl<W, H, D, F, U> Texture3DDesc<W, H, D, F, U> {
    pub fn width(self, width: u32) -> Texture3DDesc<u32, H, D, F, U> {
        Texture3DDesc {
            width,
            height: self.height,
            depth: self.depth,
            mip_levels: self.mip_levels,
            format: self.format,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn height(self, height: u32) -> Texture3DDesc<W, u32, D, F, U> {
        Texture3DDesc {
            width: self.width,
            height,
            depth: self.depth,
            mip_levels: self.mip_levels,
            format: self.format,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn depth(self, depth: u32) -> Texture3DDesc<W, H, u32, F, U> {
        Texture3DDesc {
            width: self.width,
            height: self.height,
            depth,
            mip_levels: self.mip_levels,
            format: self.format,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn mip_levels(mut self, mip_levels: u32) -> Self {
        self.mip_levels = mip_levels;
        self
    }
    pub fn format(self, format: dxgi::Format) -> Texture3DDesc<W, H, D, dxgi::Format, U> {
        Texture3DDesc {
            width: self.width,
            height: self.height,
            depth: self.depth,
            mip_levels: self.mip_levels,
            format,
            usage: self.usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn usage(self, usage: Usage) -> Texture3DDesc<W, H, D, F, Usage> {
        Texture3DDesc {
            width: self.width,
            height: self.height,
            depth: self.depth,
            mip_levels: self.mip_levels,
            format: self.format,
            usage,
            bind_flags: self.bind_flags,
            cpu_access_flags: self.cpu_access_flags,
            misc_flags: self.misc_flags,
        }
    }
    pub fn bind_flags(mut self, bind_flags: BindFlags) -> Self {
        self.bind_flags = bind_flags;
        self
    }
    pub fn cpu_access_flags(mut self, cpu_access_flags: CPUAccessFlags) -> Self {
        self.cpu_access_flags = Some(cpu_access_flags);
        self
    }
    pub fn misc_flags(mut self, misc_flags: ResourceMiscFlags) -> Self {
        self.misc_flags = Some(misc_flags);
        self
    }
}
impl Texture3DDesc<u32, u32, u32, dxgi::Format, Usage> {
    fn to_c_struct(&self) -> D3D11_TEXTURE3D_DESC {
        D3D11_TEXTURE3D_DESC {
            Width: self.width,
            Height: self.height,
            Depth: self.depth,
            MipLevels: self.mip_levels,
            Format: self.format as u32,
            Usage: self.usage as u32,
            BindFlags: self.bind_flags.0,
            CPUAccessFlags: self.cpu_access_flags.map_or(0, |f| f.0),
            MiscFlags: self.misc_flags.map_or(0, |f| f.0),
        }
    }
}
impl From<D3D11_TEXTURE3D_DESC> for Texture3DDesc<u32, u32, u32, dxgi::Format, Usage> {
    fn from(src: D3D11_TEXTURE3D_DESC) -> Texture3DDesc<u32, u32, u32, dxgi::Format, Usage> {
        unsafe {
            Texture3DDesc {
                width: src.Width,
                height: src.Height,
                depth: src.Depth,
                mip_levels: src.MipLevels,
                format: std::mem::transmute(src.Format),
                usage: std::mem::transmute(src.Usage),
                bind_flags: BindFlags(src.BindFlags),
                cpu_access_flags: if src.CPUAccessFlags == 0 {
                    None
                } else {
                    Some(CPUAccessFlags(src.CPUAccessFlags))
                },
                misc_flags: if src.MiscFlags == 0 {
                    None
                } else {
                    Some(ResourceMiscFlags(src.MiscFlags))
                },
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum UnorderedAccessViewDesc {
    Buffer {
        format: dxgi::Format,
        first_element: u32,
        num_elements: u32,
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
    },
    Texture2DArray {
        format: dxgi::Format,
        mip_slice: u32,
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
impl UnorderedAccessViewDesc {
    fn to_c_struct(&self) -> D3D11_UNORDERED_ACCESS_VIEW_DESC {
        let mut desc = D3D11_UNORDERED_ACCESS_VIEW_DESC::default();
        match self {
            &UnorderedAccessViewDesc::Buffer {
                format,
                first_element,
                num_elements,
                flags,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_UAV_DIMENSION_BUFFER;
                desc.u.Buffer_mut().FirstElement = first_element;
                desc.u.Buffer_mut().NumElements = num_elements;
                desc.u.Buffer_mut().Flags = flags.map_or(0, |f| f.0);
            },
            &UnorderedAccessViewDesc::Texture1D { format, mip_slice } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_UAV_DIMENSION_TEXTURE1D;
                desc.u.Texture1D_mut().MipSlice = mip_slice;
            },
            &UnorderedAccessViewDesc::Texture1DArray {
                format,
                mip_slice,
                first_array_slice,
                array_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_UAV_DIMENSION_TEXTURE1DARRAY;
                desc.u.Texture1DArray_mut().MipSlice = mip_slice;
                desc.u.Texture1DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture1DArray_mut().ArraySize = array_size;
            },
            &UnorderedAccessViewDesc::Texture2D { format, mip_slice } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_UAV_DIMENSION_TEXTURE2D;
                desc.u.Texture2D_mut().MipSlice = mip_slice;
            },
            &UnorderedAccessViewDesc::Texture2DArray {
                format,
                mip_slice,
                first_array_slice,
                array_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_UAV_DIMENSION_TEXTURE2DARRAY;
                desc.u.Texture2DArray_mut().MipSlice = mip_slice;
                desc.u.Texture2DArray_mut().FirstArraySlice = first_array_slice;
                desc.u.Texture2DArray_mut().ArraySize = array_size;
            },
            &UnorderedAccessViewDesc::Texture3D {
                format,
                mip_slice,
                first_w_slice,
                w_size,
            } => unsafe {
                desc.Format = format as u32;
                desc.ViewDimension = D3D11_UAV_DIMENSION_TEXTURE3D;
                desc.u.Texture3D_mut().MipSlice = mip_slice;
                desc.u.Texture3D_mut().FirstWSlice = first_w_slice;
                desc.u.Texture3D_mut().WSize = w_size;
            },
        }
        desc
    }
}
impl From<D3D11_UNORDERED_ACCESS_VIEW_DESC> for UnorderedAccessViewDesc {
    fn from(src: D3D11_UNORDERED_ACCESS_VIEW_DESC) -> UnorderedAccessViewDesc {
        unsafe {
            match src.ViewDimension {
                D3D11_UAV_DIMENSION_BUFFER => UnorderedAccessViewDesc::Buffer {
                    format: std::mem::transmute(src.Format),
                    first_element: src.u.Buffer().FirstElement,
                    num_elements: src.u.Buffer().NumElements,
                    flags: if src.u.Buffer().Flags == 0 {
                        None
                    } else {
                        Some(BufferUAVFlags(src.u.Buffer().Flags))
                    },
                },
                D3D11_UAV_DIMENSION_TEXTURE1D => UnorderedAccessViewDesc::Texture1D {
                    format: std::mem::transmute(src.Format),
                    mip_slice: src.u.Texture1D().MipSlice,
                },
                D3D11_UAV_DIMENSION_TEXTURE1DARRAY => UnorderedAccessViewDesc::Texture1DArray {
                    format: std::mem::transmute(src.Format),
                    mip_slice: src.u.Texture1DArray().MipSlice,
                    first_array_slice: src.u.Texture1DArray().FirstArraySlice,
                    array_size: src.u.Texture1DArray().ArraySize,
                },
                D3D11_UAV_DIMENSION_TEXTURE2D => UnorderedAccessViewDesc::Texture2D {
                    format: std::mem::transmute(src.Format),
                    mip_slice: src.u.Texture2D().MipSlice,
                },
                D3D11_UAV_DIMENSION_TEXTURE2DARRAY => UnorderedAccessViewDesc::Texture2DArray {
                    format: std::mem::transmute(src.Format),
                    mip_slice: src.u.Texture2DArray().MipSlice,
                    first_array_slice: src.u.Texture2DArray().FirstArraySlice,
                    array_size: src.u.Texture2DArray().ArraySize,
                },
                D3D11_UAV_DIMENSION_TEXTURE3D => UnorderedAccessViewDesc::Texture3D {
                    format: std::mem::transmute(src.Format),
                    mip_slice: src.u.Texture3D().MipSlice,
                    first_w_slice: src.u.Texture3D().FirstWSlice,
                    w_size: src.u.Texture3D().WSize,
                },
                _ => unreachable!(),
            }
        }
    }
}

/*
#[derive(Clone, Debug)]
pub enum VideoColor {
    YCbCr {
        y: f32,
        cb: f32,
        cr: f32,
        a: f32,
    },
    RGBA {
        r: f32,
        g: f32,
        b: f32,
        a: f32,
    },
}
impl VideoColor {
    fn to_c_struct(&self) -> D3D11_VIDEO_COLOR {
        let mut obj = D3D11_VIDEO_COLOR::default();
        match self {
            &VideoColor::YCbCr { y, cb, cr, a } => unsafe {
                obj.YCbCr_mut().Y = y;
                obj.YCbCr_mut().Cb = cb;
                obj.YCbCr_mut().Cr = cr;
                obj.YCbCr_mut().A = a;
            },
            &VideoColor::RGBA { r, g, b, a } => unsafe {
                obj.RGBA_mut().R = r;
                obj.RGBA_mut().G = g;
                obj.RGBA_mut().B = b;
                obj.RGBA_mut().A = a;
            },
        }
        obj
    }
}
*/

#[derive(Clone, Debug)]
#[repr(C)]
pub struct VideoContentProtectionCaps {
    caps: u32,
    key_exchange_type_count: u32,
    block_alignment_size: u32,
    protected_memory_size: u64,
}

#[derive(Clone, Debug)]
#[repr(C)]
pub struct Viewport {
    top_left_x: f32,
    top_left_y: f32,
    width: f32,
    height: f32,
    min_depth: f32,
    max_depth: f32,
}
impl Viewport {
    pub fn new() -> Self {
        Self {
            top_left_x: 0.0,
            top_left_y: 0.0,
            width: 0.0,
            height: 0.0,
            min_depth: 0.0,
            max_depth: 1.0,
        }
    }
    pub fn top_left_x(mut self, top_left_x: f32) -> Self {
        self.top_left_x = top_left_x;
        self
    }
    pub fn top_left_y(mut self, top_left_y: f32) -> Self {
        self.top_left_y = top_left_y;
        self
    }
    pub fn width(mut self, width: f32) -> Self {
        self.width = width;
        self
    }
    pub fn height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }
    pub fn min_depth(mut self, min_depth: f32) -> Self {
        self.min_depth = min_depth;
        self
    }
    pub fn max_depth(mut self, max_depth: f32) -> Self {
        self.max_depth = max_depth;
        self
    }
}
impl From<D3D11_VIEWPORT> for Viewport {
    fn from(src: D3D11_VIEWPORT) -> Viewport {
        Viewport {
            top_left_x: src.TopLeftX,
            top_left_y: src.TopLeftY,
            width: src.Width,
            height: src.Height,
            min_depth: src.MinDepth,
            max_depth: src.MaxDepth,
        }
    }
}

pub trait IDeviceChild: Interface {
    fn get_device(&self) -> Device;
}
macro_rules! impl_devicechild {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IDeviceChild for $s {
            fn get_device(&self) -> Device {
                let mut obj = std::ptr::null_mut();
                unsafe {
                    self.0.GetDevice(&mut obj);
                    Device(ComPtr::from_raw(obj))
                }
            }
        }
    };
}

pub trait IResource: IDeviceChild {
    fn get_eviction_priority(&self) -> u32;
    fn get_type(&self) -> ResourceDimension;
    fn set_eviction_priority(&self, priority: u32);
}
#[derive(Clone, Debug)]
pub struct Resource(ComPtr<ID3D11Resource>);
macro_rules! impl_resource {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IResource for $s {
            fn get_eviction_priority(&self) -> u32 {
                unsafe { self.0.GetEvictionPriority() }
            }
            fn get_type(&self) -> ResourceDimension {
                let mut value = 0;
                unsafe {
                    self.0.GetType(&mut value);
                    std::mem::transmute(value)
                }
            }
            fn set_eviction_priority(&self, priority: u32) {
                unsafe { self.0.SetEvictionPriority(priority) }
            }
        }
    };
}
impl_resource!(Resource, ID3D11Resource);

pub trait IView: IDeviceChild {
    fn get_resource(&self) -> Resource;
}
#[derive(Clone, Debug)]
pub struct View(ComPtr<ID3D11View>);
macro_rules! impl_view {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IView for $s {
            fn get_resource(&self) -> Resource {
                unsafe {
                    let mut r = std::ptr::null_mut();
                    self.0.GetResource(&mut r);
                    Resource(ComPtr::from_raw(r))
                }
            }
        }
    };
}
impl_view!(View, ID3D11View);

pub trait IAsynchronous: IDeviceChild {
    fn get_data_size(&self) -> u32;
}
#[derive(Clone, Debug)]
pub struct Asynchronous(ComPtr<ID3D11Asynchronous>);
macro_rules! impl_asynchronous {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IAsynchronous for $s {
            fn get_data_size(&self) -> u32 {
                unsafe { self.0.GetDataSize() }
            }
        }
    };
}
impl_asynchronous!(Asynchronous, ID3D11Asynchronous);

pub trait IAuthenticatedChannel: IDeviceChild {
    fn get_certificate(&self) -> Result<Vec<u8>, HResult>;
    fn get_channel_handle(&self) -> Handle;
}
#[derive(Clone, Debug)]
pub struct AuthenticatedChannel(ComPtr<ID3D11AuthenticatedChannel>);
impl_devicechild!(AuthenticatedChannel, ID3D11AuthenticatedChannel);
impl IAuthenticatedChannel for AuthenticatedChannel {
    fn get_certificate(&self) -> Result<Vec<u8>, HResult> {
        unsafe {
            let mut sz = 0;
            let res = self.0.GetCertificateSize(&mut sz);
            let sz = hresult(sz, res)?;
            let mut data = Vec::with_capacity(sz as usize);
            data.set_len(sz as usize);
            let res = self.0.GetCertificate(sz, data.as_mut_ptr());
            hresult(data, res)
        }
    }
    fn get_channel_handle(&self) -> Handle {
        unsafe {
            let mut handle = std::ptr::null_mut();
            self.0.GetChannelHandle(&mut handle);
            Handle::new(handle)
        }
    }
}

pub trait IBlendState: IDeviceChild {
    fn get_desc(&self) -> BlendDesc;
}
#[derive(Clone, Debug)]
pub struct BlendState(ComPtr<ID3D11BlendState>);
macro_rules! impl_blendstate {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IBlendState for $s {
            fn get_desc(&self) -> BlendDesc {
                unsafe {
                    let mut desc = Default::default();
                    self.0.GetDesc(&mut desc);
                    desc.into()
                }
            }
        }
    };
}
impl_blendstate!(BlendState, ID3D11BlendState);

pub trait IBuffer: IResource {
    fn get_desc(&self) -> BufferDesc<u32, Usage, BindFlags>;
}
#[derive(Clone, Debug)]
pub struct Buffer(ComPtr<ID3D11Buffer>);
macro_rules! impl_buffer {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl IBuffer for $s {
            fn get_desc(&self) -> BufferDesc<u32, Usage, BindFlags> {
                unsafe {
                    let mut desc = Default::default();
                    self.0.GetDesc(&mut desc);
                    desc.into()
                }
            }
        }
    };
}
impl_buffer!(Buffer, ID3D11Buffer);

pub trait IClassInstance: IDeviceChild {
    fn get_class_linkage(&self) -> ClassLinkage;
    fn get_desc(&self) -> ClassInstanceDesc;
    fn get_instance_name(&self) -> String;
    fn get_type_name(&self) -> String;
}
#[derive(Clone, Debug)]
pub struct ClassInstance(ComPtr<ID3D11ClassInstance>);
macro_rules! impl_classinstance {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IClassInstance for $s {
            fn get_class_linkage(&self) -> ClassLinkage {
                unsafe {
                    let mut linkage = std::ptr::null_mut();
                    self.0.GetClassLinkage(&mut linkage);
                    ClassLinkage(ComPtr::from_raw(linkage))
                }
            }
            fn get_desc(&self) -> ClassInstanceDesc {
                unsafe {
                    let mut desc = Default::default();
                    self.0.GetDesc(&mut desc);
                    desc.into()
                }
            }
            fn get_instance_name(&self) -> String {
                unsafe {
                    let mut buf = Vec::with_capacity(1024);
                    buf.set_len(1024);
                    let mut len = 0;
                    self.0.GetInstanceName(buf.as_mut_ptr(), &mut len);
                    String::from_raw_parts(buf.as_mut_ptr() as *mut u8, len, len)
                }
            }
            fn get_type_name(&self) -> String {
                unsafe {
                    let mut buf = Vec::with_capacity(1024);
                    buf.set_len(1024);
                    let mut len = 0;
                    self.0.GetTypeName(buf.as_mut_ptr(), &mut len);
                    String::from_raw_parts(buf.as_mut_ptr() as *mut u8, len, len)
                }
            }
        }
    };
}
impl_classinstance!(ClassInstance, ID3D11ClassInstance);

pub trait IClassLinkage: IDeviceChild {
    fn create_class_instance(
        &self,
        class_type_name: &str,
        constant_buffer_offset: u32,
        constant_vector_offset: u32,
        texture_offset: u32,
        sampler_offset: u32,
    ) -> Result<ClassInstance, HResult>;
    fn get_class_instance(
        &self,
        class_name_instance: &str,
        instance_index: u32,
    ) -> Result<ClassInstance, HResult>;
}
#[derive(Clone, Debug)]
pub struct ClassLinkage(ComPtr<ID3D11ClassLinkage>);
macro_rules! impl_classlinkage {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IClassLinkage for $s {
            fn create_class_instance(
                &self,
                class_type_name: &str,
                constant_buffer_offset: u32,
                constant_vector_offset: u32,
                texture_offset: u32,
                sampler_offset: u32,
            ) -> Result<ClassInstance, HResult> {
                Ok(ClassInstance(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let name = std::ffi::CString::new(class_type_name).unwrap();
                    let res = unsafe {
                        self.0.CreateClassInstance(
                            name.as_ptr(),
                            constant_buffer_offset,
                            constant_vector_offset,
                            texture_offset,
                            sampler_offset,
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn get_class_instance(
                &self,
                class_name_instance: &str,
                instance_index: u32,
            ) -> Result<ClassInstance, HResult> {
                Ok(ClassInstance(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let name = std::ffi::CString::new(class_name_instance).unwrap();
                    let res = unsafe {
                        self.0
                            .GetClassInstance(name.as_ptr(), instance_index, &mut obj)
                    };
                    hresult(obj, res)
                })?))
            }
        }
    };
}
impl_classlinkage!(ClassLinkage, ID3D11ClassLinkage);

pub trait ICommandList: IDeviceChild {
    fn get_context_flags(&self) -> u32;
}
#[derive(Clone, Debug)]
pub struct CommandList(ComPtr<ID3D11CommandList>);
macro_rules! impl_commandlist {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl ICommandList for $s {
            fn get_context_flags(&self) -> u32 {
                unsafe { self.0.GetContextFlags() }
            }
        }
    };
}
impl_commandlist!(CommandList, ID3D11CommandList);

pub trait IComputeShader: IDeviceChild {}
#[derive(Clone, Debug)]
pub struct ComputeShader(ComPtr<ID3D11ComputeShader>);
macro_rules! impl_computeshader {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IComputeShader for $s {}
    };
}
impl_computeshader!(ComputeShader, ID3D11ComputeShader);

pub trait ICounter: IAsynchronous {
    fn get_desc(&self) -> CounterDesc;
}
#[derive(Clone, Debug)]
pub struct Counter(ComPtr<ID3D11Counter>);
macro_rules! impl_counter {
    ($s: ident, $interface: ident) => {
        impl_asynchronous!($s, $interface);
        impl ICounter for $s {
            fn get_desc(&self) -> CounterDesc {
                unsafe {
                    let mut desc = Default::default();
                    self.0.GetDesc(&mut desc);
                    desc.into()
                }
            }
        }
    };
}
impl_counter!(Counter, ID3D11Counter);

pub trait ICryptoSession: IDeviceChild {
    fn get_certificate(&self) -> Result<Vec<u8>, HResult>;
    fn get_crypto_session_handle(&self) -> Handle;
    fn get_crypto_type(&self) -> Guid;
    fn get_decoder_profile(&self) -> Guid;
}
pub struct CryptoSession(ComPtr<ID3D11CryptoSession>);
macro_rules! impl_cryptosession {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl ICryptoSession for $s {
            fn get_certificate(&self) -> Result<Vec<u8>, HResult> {
                unsafe {
                    let mut sz = 0;
                    let res = self.0.GetCertificateSize(&mut sz);
                    let sz = hresult(sz, res)?;
                    let mut data = Vec::with_capacity(sz as usize);
                    data.set_len(sz as usize);
                    let res = self.0.GetCertificate(sz, data.as_mut_ptr());
                    hresult(data, res)
                }
            }
            fn get_crypto_session_handle(&self) -> Handle {
                let mut handle = std::ptr::null_mut();
                unsafe { self.0.GetCryptoSessionHandle(&mut handle) };
                Handle::new(handle)
            }
            fn get_crypto_type(&self) -> Guid {
                let mut guid = Default::default();
                unsafe { self.0.GetCryptoType(&mut guid) };
                guid.into()
            }
            fn get_decoder_profile(&self) -> Guid {
                let mut guid = Default::default();
                unsafe { self.0.GetDecoderProfile(&mut guid) };
                guid.into()
            }
        }
    };
}
impl_cryptosession!(CryptoSession, ID3D11CryptoSession);

pub trait IDepthStencilState: IDeviceChild {
    fn get_desc(&self) -> DepthStencilDesc;
}
#[derive(Clone, Debug)]
pub struct DepthStencilState(ComPtr<ID3D11DepthStencilState>);
macro_rules! impl_depthstencilstate {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IDepthStencilState for $s {
            fn get_desc(&self) -> DepthStencilDesc {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_depthstencilstate!(DepthStencilState, ID3D11DepthStencilState);

pub trait IDepthStencilView: IView {
    fn get_desc(&self) -> DepthStencilViewDesc;
}
#[derive(Clone, Debug)]
pub struct DepthStencilView(ComPtr<ID3D11DepthStencilView>);
macro_rules! impl_depthstencilview {
    ($s: ident, $interface: ident) => {
        impl_view!($s, $interface);
        impl IDepthStencilView for $s {
            fn get_desc(&self) -> DepthStencilViewDesc {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_depthstencilview!(DepthStencilView, ID3D11DepthStencilView);

#[derive(Clone, Debug)]
pub struct CheckCounterResult {
    pub ty: CounterType,
    pub active_counters: u32,
    pub name: Option<String>,
    pub units: Option<String>,
    pub description: Option<String>,
}

pub trait IDevice: Interface {
    fn check_counter(&self, desc: &CounterDesc) -> Result<CheckCounterResult, HResult>;
    fn check_counter_info(&self) -> CounterInfo;
    fn check_feature_support<T: CheckFeatureSupport>(&self, args: T::Args) -> Result<T, HResult>;
    fn check_format_support(&self, format: dxgi::Format) -> Result<FormatSupport, HResult>;
    fn check_multisample_quality_levels(
        &self,
        format: dxgi::Format,
        sample_count: u32,
    ) -> Result<u32, HResult>;
    fn create_blend_state(&self, desc: &BlendDesc) -> Result<BlendState, HResult>;
    fn create_buffer(
        &self,
        desc: &BufferDesc<u32, Usage, BindFlags>,
        initial_data: Option<&SubresourceData>,
    ) -> Result<Buffer, HResult>;
    fn create_class_linkage(&self) -> Result<ClassLinkage, HResult>;
    fn create_compute_shader(
        &self,
        bytecode: &[u8],
        class_linkage: Option<&ClassLinkage>,
    ) -> Result<ComputeShader, HResult>;
    fn create_counter(&self, desc: &CounterDesc) -> Result<Counter, HResult>;
    fn create_deferred_context(&self) -> Result<DeviceContext, HResult>;
    fn create_depth_stencil_state(
        &self,
        desc: &DepthStencilDesc,
    ) -> Result<DepthStencilState, HResult>;
    fn create_depth_stencil_view(
        &self,
        resource: &impl IResource,
        desc: Option<&DepthStencilViewDesc>,
    ) -> Result<DepthStencilView, HResult>;
    fn create_domain_shader(
        &self,
        bytecode: &[u8],
        class_linkage: Option<&ClassLinkage>,
    ) -> Result<DomainShader, HResult>;
    fn create_geometry_shader(
        &self,
        bytecode: &[u8],
        class_linkage: Option<&ClassLinkage>,
    ) -> Result<GeometryShader, HResult>;
    fn create_geometry_shader_with_stream_output(
        &self,
        bytecode: &[u8],
        so_declaration: Option<&[&SODeclarationEntry]>,
        buffer_stride: &[u32],
        rasterized_stream: u32,
        class_linkage: Option<&ClassLinkage>,
    ) -> Result<GeometryShader, HResult>;
    fn create_hull_shader(
        &self,
        bytecode: &[u8],
        class_linkage: Option<&ClassLinkage>,
    ) -> Result<HullShader, HResult>;
    fn create_input_layout<'a>(
        &self,
        descs: &[InputElementDesc<'a>],
        bytecode: &[u8],
    ) -> Result<InputLayout, HResult>;
    fn create_pixel_shader(
        &self,
        bytecode: &[u8],
        class_linkage: Option<&ClassLinkage>,
    ) -> Result<PixelShader, HResult>;
    fn create_predicate(&self, desc: &QueryDesc) -> Result<Predicate, HResult>;
    fn create_query(&self, desc: &QueryDesc) -> Result<Query, HResult>;
    fn create_rasterizer_state(&self, desc: &RasterizerDesc) -> Result<RasterizerState, HResult>;
    fn create_render_target_view(
        &self,
        resource: &impl IResource,
        desc: Option<&RenderTargetViewDesc>,
    ) -> Result<RenderTargetView, HResult>;
    fn create_sampler_state(&self, desc: &SamplerDesc<Filter>) -> Result<SamplerState, HResult>;
    fn create_shader_resource_view(
        &self,
        resource: &impl IResource,
        desc: Option<&ShaderResourceViewDesc>,
    ) -> Result<ShaderResourceView, HResult>;
    fn create_texture1d(
        &self,
        desc: &Texture1DDesc<u32, dxgi::Format, Usage>,
        initial_data: Option<&SubresourceData>,
    ) -> Result<Texture1D, HResult>;
    fn create_texture2d(
        &self,
        desc: &Texture2DDesc<u32, u32, dxgi::Format, Usage>,
        initial_data: Option<&SubresourceData>,
    ) -> Result<Texture2D, HResult>;
    fn create_texture3d(
        &self,
        desc: &Texture3DDesc<u32, u32, u32, dxgi::Format, Usage>,
        initial_data: Option<&SubresourceData>,
    ) -> Result<Texture3D, HResult>;
    fn create_unordered_access_view(
        &self,
        resource: &impl IResource,
        desc: Option<&UnorderedAccessViewDesc>,
    ) -> Result<UnorderedAccessView, HResult>;
    fn create_vertex_shader(
        &self,
        bytecode: &[u8],
        class_linkage: Option<&ClassLinkage>,
    ) -> Result<VertexShader, HResult>;
    fn get_creation_flags(&self) -> Option<CreateDeviceFlags>;
    fn get_device_removed_reason(&self) -> HResult;
    fn get_exception_mode(&self) -> Option<RaiseFlags>;
    fn get_feature_level(&self) -> d3d::FeatureLevel;
    fn get_immediate_context(&self) -> DeviceContext;
    fn open_shared_resource<T: Interface>(&self, resource: &Handle) -> Result<T, HResult>;
    fn set_exception_mode(&self, flags: Option<RaiseFlags>) -> Result<(), HResult>;
}
#[derive(Clone, Debug)]
pub struct Device(ComPtr<ID3D11Device>);
macro_rules! impl_device {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IDevice for $s {
            fn check_counter(&self, desc: &CounterDesc) -> Result<CheckCounterResult, HResult> {
                let c_desc = desc.to_c_struct();
                let mut ty = 0;
                let mut active_counters = 0;
                let mut name_len = 0;
                let mut units_len = 0;
                let mut description_len = 0;
                let res = unsafe {
                    self.0.CheckCounter(
                        &c_desc,
                        &mut ty,
                        &mut active_counters,
                        std::ptr::null_mut(),
                        &mut name_len,
                        std::ptr::null_mut(),
                        &mut units_len,
                        std::ptr::null_mut(),
                        &mut description_len,
                    )
                };
                hresult((), res)?;
                unsafe {
                    let mut name = Vec::<u8>::with_capacity(name_len as usize);
                    name.set_len(name_len as usize);
                    let mut units = Vec::<u8>::with_capacity(units_len as usize);
                    units.set_len(units_len as usize);
                    let mut description = Vec::<u8>::with_capacity(description_len as usize);
                    description.set_len(description_len as usize);
                    let res = self.0.CheckCounter(
                        &c_desc,
                        &mut ty,
                        &mut active_counters,
                        name.as_mut_ptr() as *mut i8,
                        &mut name_len,
                        units.as_mut_ptr() as *mut i8,
                        &mut units_len,
                        description.as_mut_ptr() as *mut i8,
                        &mut description_len,
                    );
                    let obj = CheckCounterResult {
                        ty: std::mem::transmute(ty),
                        active_counters,
                        name: String::from_utf8(name).ok(),
                        units: String::from_utf8(units).ok(),
                        description: String::from_utf8(description).ok(),
                    };
                    hresult(obj, res)
                }
            }
            fn check_counter_info(&self) -> CounterInfo {
                let mut obj = Default::default();
                unsafe { self.0.CheckCounterInfo(&mut obj) };
                obj.into()
            }
            fn check_feature_support<T: CheckFeatureSupport>(
                &self,
                args: T::Args,
            ) -> Result<T, HResult> {
                T::check_feature_support(self.0.as_ptr() as *mut ID3D11Device, args)
            }
            fn check_format_support(&self, format: dxgi::Format) -> Result<FormatSupport, HResult> {
                let mut val = 0;
                let res = unsafe { self.0.CheckFormatSupport(format as u32, &mut val) };
                hresult(FormatSupport(val), res)
            }
            fn check_multisample_quality_levels(
                &self,
                format: dxgi::Format,
                sample_count: u32,
            ) -> Result<u32, HResult> {
                let mut levels = 0;
                let res = unsafe {
                    self.0
                        .CheckMultisampleQualityLevels(format as u32, sample_count, &mut levels)
                };
                hresult(levels, res)
            }
            fn create_blend_state(&self, desc: &BlendDesc) -> Result<BlendState, HResult> {
                Ok(BlendState(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.CreateBlendState(&desc.to_c_struct(), &mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn create_buffer(
                &self,
                desc: &BufferDesc<u32, Usage, BindFlags>,
                initial_data: Option<&SubresourceData>,
            ) -> Result<Buffer, HResult> {
                Ok(Buffer(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let c_initial_data = initial_data.as_ref().map(|d| d.to_c_struct());
                    let res = unsafe {
                        self.0.CreateBuffer(
                            &desc.to_c_struct(),
                            c_initial_data.map_or(std::ptr::null(), |d| &d),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_class_linkage(&self) -> Result<ClassLinkage, HResult> {
                Ok(ClassLinkage(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.CreateClassLinkage(&mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn create_compute_shader(
                &self,
                bytecode: &[u8],
                class_linkage: Option<&ClassLinkage>,
            ) -> Result<ComputeShader, HResult> {
                Ok(ComputeShader(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateComputeShader(
                            bytecode.as_ptr() as *const c_void,
                            bytecode.len(),
                            class_linkage.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_counter(&self, desc: &CounterDesc) -> Result<Counter, HResult> {
                Ok(Counter(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.CreateCounter(&desc.to_c_struct(), &mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn create_deferred_context(&self) -> Result<DeviceContext, HResult> {
                Ok(DeviceContext(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.CreateDeferredContext(0, &mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn create_depth_stencil_state(
                &self,
                desc: &DepthStencilDesc,
            ) -> Result<DepthStencilState, HResult> {
                Ok(DepthStencilState(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0
                            .CreateDepthStencilState(&desc.to_c_struct(), &mut obj)
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_depth_stencil_view(
                &self,
                resource: &impl IResource,
                desc: Option<&DepthStencilViewDesc>,
            ) -> Result<DepthStencilView, HResult> {
                Ok(DepthStencilView(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let c_desc = desc.as_ref().map(|d| d.to_c_struct());
                    let res = unsafe {
                        self.0.CreateDepthStencilView(
                            resource.as_ptr() as *mut _,
                            c_desc.map_or(std::ptr::null(), |d| &d),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_domain_shader(
                &self,
                bytecode: &[u8],
                class_linkage: Option<&ClassLinkage>,
            ) -> Result<DomainShader, HResult> {
                Ok(DomainShader(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateDomainShader(
                            bytecode.as_ptr() as *const c_void,
                            bytecode.len(),
                            class_linkage.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_geometry_shader(
                &self,
                bytecode: &[u8],
                class_linkage: Option<&ClassLinkage>,
            ) -> Result<GeometryShader, HResult> {
                Ok(GeometryShader(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateGeometryShader(
                            bytecode.as_ptr() as *const c_void,
                            bytecode.len(),
                            class_linkage.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_geometry_shader_with_stream_output(
                &self,
                bytecode: &[u8],
                so_declaration: Option<&[&SODeclarationEntry]>,
                buffer_stride: &[u32],
                rasterized_stream: u32,
                class_linkage: Option<&ClassLinkage>,
            ) -> Result<GeometryShader, HResult> {
                Ok(GeometryShader(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let (c_so, _tmp): (Vec<_>, Vec<_>) = so_declaration
                        .map_or((Vec::new(), Vec::new()), |d| {
                            d.iter().map(|entry| entry.to_c_struct()).unzip()
                        });
                    let res = unsafe {
                        self.0.CreateGeometryShaderWithStreamOutput(
                            bytecode.as_ptr() as *const c_void,
                            bytecode.len(),
                            if c_so.is_empty() {
                                std::ptr::null()
                            } else {
                                c_so.as_ptr()
                            },
                            c_so.len() as u32,
                            buffer_stride.as_ptr(),
                            buffer_stride.len() as u32,
                            rasterized_stream,
                            class_linkage.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_hull_shader(
                &self,
                bytecode: &[u8],
                class_linkage: Option<&ClassLinkage>,
            ) -> Result<HullShader, HResult> {
                Ok(HullShader(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateHullShader(
                            bytecode.as_ptr() as *const c_void,
                            bytecode.len(),
                            class_linkage.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_input_layout<'a>(
                &self,
                descs: &[InputElementDesc<'a>],
                bytecode: &[u8],
            ) -> Result<InputLayout, HResult> {
                Ok(InputLayout(ComPtr::new(|| {
                    let (c_descs, _tmp): (Vec<_>, Vec<_>) =
                        descs.iter().map(|d| d.to_c_struct()).unzip();
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateInputLayout(
                            c_descs.as_ptr(),
                            c_descs.len() as u32,
                            bytecode.as_ptr() as *const c_void,
                            bytecode.len(),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_pixel_shader(
                &self,
                bytecode: &[u8],
                class_linkage: Option<&ClassLinkage>,
            ) -> Result<PixelShader, HResult> {
                Ok(PixelShader(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreatePixelShader(
                            bytecode.as_ptr() as *const c_void,
                            bytecode.len(),
                            class_linkage.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_predicate(&self, desc: &QueryDesc) -> Result<Predicate, HResult> {
                Ok(Predicate(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.CreatePredicate(&desc.to_c_struct(), &mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn create_query(&self, desc: &QueryDesc) -> Result<Query, HResult> {
                Ok(Query(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.CreateQuery(&desc.to_c_struct(), &mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn create_rasterizer_state(
                &self,
                desc: &RasterizerDesc,
            ) -> Result<RasterizerState, HResult> {
                Ok(RasterizerState(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res =
                        unsafe { self.0.CreateRasterizerState(&desc.to_c_struct(), &mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn create_render_target_view(
                &self,
                resource: &impl IResource,
                desc: Option<&RenderTargetViewDesc>,
            ) -> Result<RenderTargetView, HResult> {
                Ok(RenderTargetView(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let c_desc = desc.map(|d| d.to_c_struct());
                    let res = unsafe {
                        self.0.CreateRenderTargetView(
                            resource.as_ptr() as *mut _,
                            c_desc.map_or(std::ptr::null(), |d| &d),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_sampler_state(
                &self,
                desc: &SamplerDesc<Filter>,
            ) -> Result<SamplerState, HResult> {
                Ok(SamplerState(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe { self.0.CreateSamplerState(&desc.to_c_struct(), &mut obj) };
                    hresult(obj, res)
                })?))
            }
            fn create_shader_resource_view(
                &self,
                resource: &impl IResource,
                desc: Option<&ShaderResourceViewDesc>,
            ) -> Result<ShaderResourceView, HResult> {
                Ok(ShaderResourceView(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let c_desc = desc.map(|d| d.to_c_struct());
                    let res = unsafe {
                        self.0.CreateShaderResourceView(
                            resource.as_ptr() as *mut _,
                            c_desc.map_or(std::ptr::null(), |d| &d),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_texture1d(
                &self,
                desc: &Texture1DDesc<u32, dxgi::Format, Usage>,
                initial_data: Option<&SubresourceData>,
            ) -> Result<Texture1D, HResult> {
                let c_data = initial_data
                    .as_ref()
                    .map_or(None, |d| Some(d.to_c_struct()));
                Ok(Texture1D(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateTexture1D(
                            &desc.to_c_struct(),
                            c_data.as_ref().map_or(std::ptr::null(), |d| d),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_texture2d(
                &self,
                desc: &Texture2DDesc<u32, u32, dxgi::Format, Usage>,
                initial_data: Option<&SubresourceData>,
            ) -> Result<Texture2D, HResult> {
                let c_data = initial_data
                    .as_ref()
                    .map_or(None, |d| Some(d.to_c_struct()));
                Ok(Texture2D(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateTexture2D(
                            &desc.to_c_struct(),
                            c_data.as_ref().map_or(std::ptr::null(), |d| d),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_texture3d(
                &self,
                desc: &Texture3DDesc<u32, u32, u32, dxgi::Format, Usage>,
                initial_data: Option<&SubresourceData>,
            ) -> Result<Texture3D, HResult> {
                let c_data = initial_data
                    .as_ref()
                    .map_or(None, |d| Some(d.to_c_struct()));
                Ok(Texture3D(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateTexture3D(
                            &desc.to_c_struct(),
                            c_data.as_ref().map_or(std::ptr::null(), |d| d),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_unordered_access_view(
                &self,
                resource: &impl IResource,
                desc: Option<&UnorderedAccessViewDesc>,
            ) -> Result<UnorderedAccessView, HResult> {
                Ok(UnorderedAccessView(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let c_desc = desc.map(|d| d.to_c_struct());
                    let res = unsafe {
                        self.0.CreateUnorderedAccessView(
                            resource.as_ptr() as *mut _,
                            c_desc.as_ref().map_or(std::ptr::null(), |d| d),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn create_vertex_shader(
                &self,
                bytecode: &[u8],
                class_linkage: Option<&ClassLinkage>,
            ) -> Result<VertexShader, HResult> {
                Ok(VertexShader(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.CreateVertexShader(
                            bytecode.as_ptr() as *const c_void,
                            bytecode.len(),
                            class_linkage.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
                            &mut obj,
                        )
                    };
                    hresult(obj, res)
                })?))
            }
            fn get_creation_flags(&self) -> Option<CreateDeviceFlags> {
                let value = unsafe { self.0.GetCreationFlags() };
                if value == 0 {
                    None
                } else {
                    Some(CreateDeviceFlags(value))
                }
            }
            fn get_device_removed_reason(&self) -> HResult {
                unsafe { self.0.GetDeviceRemovedReason().into() }
            }
            fn get_exception_mode(&self) -> Option<RaiseFlags> {
                let value = unsafe { self.0.GetExceptionMode() };
                if value == 0 {
                    None
                } else {
                    Some(RaiseFlags(value))
                }
            }
            fn get_feature_level(&self) -> d3d::FeatureLevel {
                let level = unsafe { self.0.GetFeatureLevel() };
                level.into()
            }
            fn get_immediate_context(&self) -> DeviceContext {
                unsafe {
                    let mut obj = std::ptr::null_mut();
                    self.0.GetImmediateContext(&mut obj);
                    DeviceContext(ComPtr::from_raw(obj))
                }
            }
            fn open_shared_resource<T: Interface>(&self, resource: &Handle) -> Result<T, HResult> {
                Ok(T::new(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0.OpenSharedResource(
                            resource.as_raw_handle(),
                            &T::uuidof().into(),
                            &mut obj,
                        )
                    };
                    hresult(obj as *mut <T as Interface>::APIType, res)
                })?))
            }
            fn set_exception_mode(&self, flags: Option<RaiseFlags>) -> Result<(), HResult> {
                let res = unsafe { self.0.SetExceptionMode(flags.map_or(0, |f| f.0)) };
                hresult((), res)
            }
        }
    };
}
impl_device!(Device, ID3D11Device);

fn shader_get_objects<F, T: Interface>(f: F, start_slot: u32, num: u32) -> Vec<Option<T>>
where
    F: FnOnce(u32, u32, *mut *mut T::APIType),
{
    unsafe {
        let mut objs = Vec::with_capacity(num as usize);
        objs.set_len(num as usize);
        f(start_slot, num, objs.as_mut_ptr());
        objs.into_iter()
            .map(|p| {
                if p == std::ptr::null_mut() {
                    None
                } else {
                    Some(T::new(ComPtr::from_raw(p)))
                }
            })
            .collect::<Vec<_>>()
    }
}

fn get_shader<F, T: Interface>(f: F) -> (Option<T>, Option<Vec<ClassInstance>>)
where
    F: FnOnce(*mut *mut T::APIType, *mut *mut ID3D11ClassInstance, &mut u32),
{
    unsafe {
        let mut shader = std::ptr::null_mut();
        let mut instances = std::ptr::null_mut();
        let mut instances_num = 0;
        f(&mut shader, &mut instances, &mut instances_num);
        let shader = if shader == std::ptr::null_mut() {
            None
        } else {
            Some(T::new(ComPtr::from_raw(shader)))
        };
        let instances = if instances == std::ptr::null_mut() {
            None
        } else {
            let c_instances = Vec::from_raw_parts(
                &mut instances,
                instances_num as usize,
                instances_num as usize,
            );
            Some(
                c_instances
                    .into_iter()
                    .map(|p| ClassInstance(ComPtr::from_raw(p)))
                    .collect::<Vec<_>>(),
            )
        };
        (shader, instances)
    }
}

fn shader_set_objects<F, T: Interface>(f: F, start_slot: u32, objs: &[&T])
where
    F: FnOnce(u32, u32, *const *mut T::APIType),
{
    let c_objs = objs.iter().map(|o| o.as_ptr()).collect::<Vec<_>>();
    f(start_slot, c_objs.len() as u32, c_objs.as_ptr());
}

fn set_shader<F, T: Interface>(f: F, shader: &T, instances: Option<&[&ClassInstance]>)
where
    F: FnOnce(*mut T::APIType, *const *mut ID3D11ClassInstance, u32),
{
    let c_instances = instances.map(|a| a.iter().map(|i| i.as_ptr()).collect::<Vec<_>>());
    f(
        shader.as_ptr(),
        c_instances
            .as_ref()
            .map_or(std::ptr::null(), |ci| ci.as_ptr()),
        c_instances.as_ref().map_or(0, |ci| ci.len() as u32),
    );
}

pub trait IDeviceContext: IDeviceChild {
    fn begin(&self, obj: &impl IAsynchronous);
    fn clear_depth_stencil_view(
        &self,
        view: &DepthStencilView,
        clear_flags: ClearFlags,
        depth: f32,
        stencil: u8,
    );
    fn clear_render_target_view(&self, view: &RenderTargetView, color: dxgi::RGBA);
    fn clear_state(&self);
    fn clear_unordered_access_view_float(&self, view: &UnorderedAccessView, values: [f32; 4]);
    fn clear_unordered_access_view_uint(&self, view: &UnorderedAccessView, values: [u32; 4]);
    fn copy_resource(&self, dest: &impl IResource, src: &impl IResource);
    fn copy_structure_count(
        &self,
        buffer: &impl IBuffer,
        dest_aligned_byte_offset: u32,
        src_view: &UnorderedAccessView,
    );
    fn copy_subresource_region(
        &self,
        dest: &impl IResource,
        dest_subresource: u32,
        dest_x: u32,
        dest_y: u32,
        dest_z: u32,
        src: &impl IResource,
        src_subresource: u32,
        src_box: Option<&Box3D>,
    );
    fn cs_get_constant_buffers(&self, start_slot: u32, num_buffers: u32) -> Vec<Option<Buffer>>;
    fn cs_get_samplers(&self, start_slot: u32, num_samplers: u32) -> Vec<Option<SamplerState>>;
    fn cs_get_shader(&self) -> (Option<ComputeShader>, Option<Vec<ClassInstance>>);
    fn cs_get_shader_resources(
        &self,
        start_slot: u32,
        num_views: u32,
    ) -> Vec<Option<ShaderResourceView>>;
    fn cs_get_unordered_access_views(
        &self,
        start_slot: u32,
        num_uavs: u32,
    ) -> Vec<Option<UnorderedAccessView>>;
    fn cs_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]);
    fn cs_set_samplers(&self, start_slot: u32, samplers: &[&impl ISamplerState]);
    fn cs_set_shader(&self, shader: &ComputeShader, instances: Option<&[&ClassInstance]>);
    fn cs_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]);
    fn cs_set_unordered_access_views(
        &self,
        start_slot: u32,
        vies: &[&UnorderedAccessView],
        initial_counts: &[u32],
    );
    fn dispatch(
        &self,
        thread_group_count_x: u32,
        thread_group_count_y: u32,
        thraed_group_count_z: u32,
    );
    fn dispatch_indirect(&self, args: &Buffer, byte_offset: u32);
    fn draw(&self, vertex_count: u32, start_vertex_location: u32);
    fn draw_auto(&self);
    fn draw_indexed(&self, index_count: u32, start_index_location: u32, base_vertex_location: i32);
    fn draw_indexed_instanced(
        &self,
        index_count_per_instance: u32,
        instance_count: u32,
        start_index_location: u32,
        base_vertex_location: i32,
        start_instance_location: u32,
    );
    fn draw_indexed_instanced_indirect(&self, args: &impl IBuffer, byte_offset: u32);
    fn draw_instanced(
        &self,
        vertex_count_per_instance: u32,
        instance_count: u32,
        start_vertex_location: u32,
        start_instance_location: u32,
    );
    fn draw_instanced_indirect(&self, args: &impl IBuffer, byte_offset: u32);
    fn ds_get_constant_buffers(&self, start_slot: u32, num_buffers: u32) -> Vec<Option<Buffer>>;
    fn ds_get_samplers(&self, start_slot: u32, num_samplers: u32) -> Vec<Option<SamplerState>>;
    fn ds_get_shader(&self) -> (Option<DomainShader>, Option<Vec<ClassInstance>>);
    fn ds_get_shader_resources(
        &self,
        start_slot: u32,
        num_views: u32,
    ) -> Vec<Option<ShaderResourceView>>;
    fn ds_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]);
    fn ds_set_samplers(&self, start_slot: u32, samplers: &[&impl ISamplerState]);
    fn ds_set_shader(&self, shader: &DomainShader, instances: Option<&[&ClassInstance]>);
    fn ds_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]);
    fn end(&self, obj: &impl IAsynchronous);
    fn execute_command_list(&self, cmd_list: &impl ICommandList, restore_context_state: bool);
    fn finish_command_list(
        &self,
        restore_deferred_context_state: bool,
    ) -> Result<CommandList, HResult>;
    fn flush(&self);
    fn generate_mips(&self, view: &impl IShaderResourceView);
    fn get_context_flags(&self) -> u32;
    fn get_data(
        &self,
        async_obj: &impl IAsynchronous,
        flags: Option<AsyncGetDataFlag>,
    ) -> Result<Vec<u8>, HResult>;
    fn get_predication(&self) -> Option<(Predicate, bool)>;
    fn get_resource_min_lod(&self, resource: &impl IResource) -> f32;
    fn get_type(&self) -> DeviceContextType;
    fn gs_get_constant_buffers(&self, start_slot: u32, num_buffers: u32) -> Vec<Option<Buffer>>;
    fn gs_get_samplers(&self, start_slot: u32, num_samplers: u32) -> Vec<Option<SamplerState>>;
    fn gs_get_shader(&self) -> (Option<GeometryShader>, Option<Vec<ClassInstance>>);
    fn gs_get_shader_resources(
        &self,
        start_slot: u32,
        num_views: u32,
    ) -> Vec<Option<ShaderResourceView>>;
    fn gs_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]);
    fn gs_set_samplers(&self, start_slot: u32, samplers: &[&impl ISamplerState]);
    fn gs_set_shader(&self, shader: &GeometryShader, instances: Option<&[&ClassInstance]>);
    fn gs_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]);
    fn hs_get_constant_buffers(&self, start_slot: u32, num_buffers: u32) -> Vec<Option<Buffer>>;
    fn hs_get_samplers(&self, start_slot: u32, num_samplers: u32) -> Vec<Option<SamplerState>>;
    fn hs_get_shader(&self) -> (Option<HullShader>, Option<Vec<ClassInstance>>);
    fn hs_get_shader_resources(
        &self,
        start_slot: u32,
        num_views: u32,
    ) -> Vec<Option<ShaderResourceView>>;
    fn hs_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]);
    fn hs_set_samplers(&self, start_slot: u32, samplers: &[&SamplerState]);
    fn hs_set_shader(&self, shader: &HullShader, instances: Option<&[&ClassInstance]>);
    fn hs_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]);
    fn ia_get_index_buffer(&self) -> Option<(Buffer, dxgi::Format, u32)>;
    fn ia_get_input_layout(&self) -> Option<InputLayout>;
    fn ia_get_primitive_topology(&self) -> Option<PrimitiveTopology>;
    fn ia_get_vertex_buffers(
        &self,
        start_slot: u32,
        num_buffers: u32,
    ) -> Vec<Option<(Buffer, u32, u32)>>;
    fn ia_set_index_buffer(&self, buffer: &impl IBuffer, format: dxgi::Format, offset: u32);
    fn ia_set_input_layout(&self, input_layout: &impl IInputLayout);
    fn ia_set_primitive_topology(&self, topology: PrimitiveTopology);
    fn ia_set_vertex_buffers(
        &self,
        start_slot: u32,
        buffers: &[&impl IBuffer],
        strides: &[u32],
        offset: &[u32],
    );
    fn map(
        &self,
        resource: &impl IResource,
        subresource: u32,
        map_type: MapType,
        map_flags: Option<MapFlag>,
    ) -> Result<MappedSubresource, HResult>;
    fn om_get_blend_state(&self) -> Option<(BlendState, [f32; 4], u32)>;
    fn om_get_depth_stencil_state(&self) -> (Option<DepthStencilState>, u32);
    fn om_get_render_targets(
        &self,
        num_views: u32,
    ) -> (Vec<Option<RenderTargetView>>, Option<DepthStencilView>);
    // fn om_get_render_targets_and_unordered_access_views(&self, num_rtvs: u32, uav_start_slot: u32, num_uavs: u32) -> (Vec<Option<RenderTargetView>>, Option<DepthStencilView>, Vec<Option<(UnorderedAccessView, u32)>>);
    fn om_set_blend_state(&self, state: &BlendState, factor: &[f32; 4], mask: u32);
    fn om_set_depth_stencil_state(&self, state: &DepthStencilState, stencil_ref: u32);
    fn om_set_render_targets(
        &self,
        rtvs: Option<&[&RenderTargetView]>,
        dsv: Option<&DepthStencilView>,
    );
    fn om_set_render_targets_and_unordered_access_views(
        &self,
        rtvs: Option<&[&RenderTargetView]>,
        dsv: Option<&DepthStencilView>,
        uav_start_slot: u32,
        uavs: Option<&[&UnorderedAccessView]>,
        initial_counts: Option<&[u32]>,
    );
    fn ps_get_constant_buffers(&self, start_slot: u32, num_buffers: u32) -> Vec<Option<Buffer>>;
    fn ps_get_samplers(&self, start_slot: u32, num_samplers: u32) -> Vec<Option<SamplerState>>;
    fn ps_get_shader(&self) -> (Option<PixelShader>, Option<Vec<ClassInstance>>);
    fn ps_get_shader_resources(
        &self,
        start_slot: u32,
        num_views: u32,
    ) -> Vec<Option<ShaderResourceView>>;
    fn ps_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]);
    fn ps_set_samplers(&self, start_slot: u32, samplers: &[&SamplerState]);
    fn ps_set_shader(&self, shader: &PixelShader, instances: Option<&[&ClassInstance]>);
    fn ps_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]);
    fn resolve_subresource(
        &self,
        dest: &impl IResource,
        dest_subresource: u32,
        src: &impl IResource,
        src_subresource: u32,
        format: dxgi::Format,
    );
    fn rs_get_scissor_rects(&self) -> Vec<Rect>;
    fn rs_get_state(&self) -> Option<RasterizerState>;
    fn rs_get_viewports(&self) -> Vec<Viewport>;
    fn rs_set_scissor_rects(&self, rects: &[Rect]);
    fn rs_set_state(&self, rasterizer_state: &RasterizerState);
    fn rs_set_viewports(&self, viewports: &[Viewport]);
    fn set_predication(&self, predicate: Option<&Predicate>, value: bool);
    fn set_resource_min_lod(&self, resource: &impl IResource, min_lod: f32);
    fn so_get_targets(&self, num: u32) -> Vec<Option<Buffer>>;
    fn so_set_targets(&self, targets: &[&impl IBuffer], offset: &[u32]);
    fn unmap(&self, resource: &impl IResource, subresource: u32);
    fn update_subresource(
        &self,
        dest: &impl IResource,
        dest_subresource: u32,
        dest_box: Option<&Box3D>,
        src: &[u8],
        src_row_pitch: u32,
        src_depth_pitch: u32,
    );
    fn vs_get_constant_buffers(&self, start_slot: u32, num_buffers: u32) -> Vec<Option<Buffer>>;
    fn vs_get_samplers(&self, start_slot: u32, num_samplers: u32) -> Vec<Option<SamplerState>>;
    fn vs_get_shader(&self) -> (Option<VertexShader>, Option<Vec<ClassInstance>>);
    fn vs_get_shader_resources(
        &self,
        start_slot: u32,
        num_views: u32,
    ) -> Vec<Option<ShaderResourceView>>;
    fn vs_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]);
    fn vs_set_samplers(&self, start_slot: u32, samplers: &[&SamplerState]);
    fn vs_set_shader(&self, shader: &VertexShader, instances: Option<&[&ClassInstance]>);
    fn vs_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]);
}
pub struct DeviceContext(ComPtr<ID3D11DeviceContext>);
macro_rules! impl_devicecontext {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IDeviceContext for $s {
            fn begin(&self, obj: &impl IAsynchronous) {
                unsafe {
                    self.0.Begin(obj.as_ptr() as *mut _);
                }
            }
            fn clear_depth_stencil_view(
                &self,
                view: &DepthStencilView,
                clear_flags: ClearFlags,
                depth: f32,
                stencil: u8,
            ) {
                unsafe {
                    self.0
                        .ClearDepthStencilView(view.as_ptr(), clear_flags.0, depth, stencil);
                }
            }
            fn clear_render_target_view(&self, view: &RenderTargetView, color: dxgi::RGBA) {
                unsafe {
                    self.0.ClearRenderTargetView(
                        view.as_ptr(),
                        &[color.r, color.g, color.b, color.a],
                    );
                }
            }
            fn clear_state(&self) {
                unsafe {
                    self.0.ClearState();
                }
            }
            fn clear_unordered_access_view_float(
                &self,
                view: &UnorderedAccessView,
                values: [f32; 4],
            ) {
                unsafe {
                    self.0.ClearUnorderedAccessViewFloat(view.as_ptr(), &values);
                }
            }
            fn clear_unordered_access_view_uint(
                &self,
                view: &UnorderedAccessView,
                values: [u32; 4],
            ) {
                unsafe {
                    self.0.ClearUnorderedAccessViewUint(view.as_ptr(), &values);
                }
            }
            fn copy_resource(&self, dest: &impl IResource, src: &impl IResource) {
                unsafe {
                    self.0
                        .CopyResource(dest.as_ptr() as *mut _, src.as_ptr() as *mut _);
                }
            }
            fn copy_structure_count(
                &self,
                dest: &impl IBuffer,
                byte_offset: u32,
                src_view: &UnorderedAccessView,
            ) {
                unsafe {
                    self.0.CopyStructureCount(
                        dest.as_ptr() as *mut _,
                        byte_offset,
                        src_view.as_ptr(),
                    );
                }
            }
            fn copy_subresource_region(
                &self,
                dest: &impl IResource,
                dest_resource: u32,
                dest_x: u32,
                dest_y: u32,
                dest_z: u32,
                src: &impl IResource,
                src_subresource: u32,
                src_box: Option<&Box3D>,
            ) {
                unsafe {
                    self.0.CopySubresourceRegion(
                        dest.as_ptr() as *mut _,
                        dest_resource,
                        dest_x,
                        dest_y,
                        dest_z,
                        src.as_ptr() as *mut _,
                        src_subresource,
                        src_box.map_or(std::ptr::null(), |b| b.as_c_ptr()),
                    );
                }
            }
            fn cs_get_constant_buffers(
                &self,
                start_slot: u32,
                num_buffers: u32,
            ) -> Vec<Option<Buffer>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.CSGetConstantBuffers(s, n, p) },
                    start_slot,
                    num_buffers,
                )
            }
            fn cs_get_samplers(
                &self,
                start_slot: u32,
                num_samplers: u32,
            ) -> Vec<Option<SamplerState>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.CSGetSamplers(s, n, p) },
                    start_slot,
                    num_samplers,
                )
            }
            fn cs_get_shader(&self) -> (Option<ComputeShader>, Option<Vec<ClassInstance>>) {
                get_shader(|p, c, n| unsafe { self.0.CSGetShader(p, c, n) })
            }
            fn cs_get_shader_resources(
                &self,
                start_slot: u32,
                num_views: u32,
            ) -> Vec<Option<ShaderResourceView>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.CSGetShaderResources(s, n, p) },
                    start_slot,
                    num_views,
                )
            }
            fn cs_get_unordered_access_views(
                &self,
                start_slot: u32,
                num_uavs: u32,
            ) -> Vec<Option<UnorderedAccessView>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.CSGetUnorderedAccessViews(s, n, p) },
                    start_slot,
                    num_uavs,
                )
            }
            fn cs_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.CSSetConstantBuffers(s, n, p as *mut _) },
                    start_slot,
                    buffers,
                )
            }
            fn cs_set_samplers(&self, start_slot: u32, samplers: &[&impl ISamplerState]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.CSSetSamplers(s, n, p as *mut _) },
                    start_slot,
                    samplers,
                )
            }
            fn cs_set_shader(&self, shader: &ComputeShader, instances: Option<&[&ClassInstance]>) {
                set_shader(
                    |p, c, n| unsafe { self.0.CSSetShader(p, c, n) },
                    shader,
                    instances,
                )
            }
            fn cs_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.CSSetShaderResources(s, n, p) },
                    start_slot,
                    views,
                )
            }
            fn cs_set_unordered_access_views(
                &self,
                start_slot: u32,
                views: &[&UnorderedAccessView],
                initial_data: &[u32],
            ) {
                let c_views = views.iter().map(|o| o.as_ptr()).collect::<Vec<_>>();
                unsafe {
                    self.0.CSSetUnorderedAccessViews(
                        start_slot,
                        c_views.len() as u32,
                        c_views.as_ptr(),
                        initial_data.as_ptr(),
                    );
                }
            }
            fn dispatch(
                &self,
                thread_group_count_x: u32,
                thread_group_count_y: u32,
                thraed_group_count_z: u32,
            ) {
                unsafe {
                    self.0.Dispatch(
                        thread_group_count_x,
                        thread_group_count_y,
                        thraed_group_count_z,
                    );
                }
            }
            fn dispatch_indirect(&self, buffer: &Buffer, byte_offset: u32) {
                unsafe {
                    self.0.DispatchIndirect(buffer.as_ptr(), byte_offset);
                }
            }
            fn draw(&self, vertex_count: u32, start_vertex_location: u32) {
                unsafe {
                    self.0.Draw(vertex_count, start_vertex_location);
                }
            }
            fn draw_auto(&self) {
                unsafe { self.0.DrawAuto() };
            }
            fn draw_indexed(
                &self,
                index_count: u32,
                start_index_location: u32,
                base_vertex_location: i32,
            ) {
                unsafe {
                    self.0
                        .DrawIndexed(index_count, start_index_location, base_vertex_location);
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
                    );
                }
            }
            fn draw_indexed_instanced_indirect(&self, args: &impl IBuffer, byte_offset: u32) {
                unsafe {
                    self.0
                        .DrawIndexedInstancedIndirect(args.as_ptr() as *mut _, byte_offset);
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
                    );
                }
            }
            fn draw_instanced_indirect(&self, args: &impl IBuffer, byte_offset: u32) {
                unsafe {
                    self.0
                        .DrawInstancedIndirect(args.as_ptr() as *mut _, byte_offset);
                }
            }
            fn ds_get_constant_buffers(
                &self,
                start_slot: u32,
                num_buffers: u32,
            ) -> Vec<Option<Buffer>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.DSGetConstantBuffers(s, n, p) },
                    start_slot,
                    num_buffers,
                )
            }
            fn ds_get_samplers(
                &self,
                start_slot: u32,
                num_samplers: u32,
            ) -> Vec<Option<SamplerState>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.DSGetSamplers(s, n, p) },
                    start_slot,
                    num_samplers,
                )
            }
            fn ds_get_shader(&self) -> (Option<DomainShader>, Option<Vec<ClassInstance>>) {
                get_shader(|p, c, n| unsafe { self.0.DSGetShader(p, c, n) })
            }
            fn ds_get_shader_resources(
                &self,
                start_slot: u32,
                num_views: u32,
            ) -> Vec<Option<ShaderResourceView>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.DSGetShaderResources(s, n, p) },
                    start_slot,
                    num_views,
                )
            }
            fn ds_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.DSSetConstantBuffers(s, n, p as *mut _) },
                    start_slot,
                    buffers,
                )
            }
            fn ds_set_samplers(&self, start_slot: u32, samplers: &[&impl ISamplerState]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.DSSetSamplers(s, n, p as *mut _) },
                    start_slot,
                    samplers,
                )
            }
            fn ds_set_shader(&self, shader: &DomainShader, instances: Option<&[&ClassInstance]>) {
                set_shader(
                    |p, c, n| unsafe { self.0.DSSetShader(p, c, n) },
                    shader,
                    instances,
                )
            }
            fn ds_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.DSSetShaderResources(s, n, p) },
                    start_slot,
                    views,
                )
            }
            fn end(&self, obj: &impl IAsynchronous) {
                unsafe {
                    self.0.End(obj.as_ptr() as *mut _);
                }
            }
            fn execute_command_list(
                &self,
                cmd_list: &impl ICommandList,
                restore_context_state: bool,
            ) {
                unsafe {
                    self.0.ExecuteCommandList(
                        cmd_list.as_ptr() as *mut _,
                        to_BOOL(restore_context_state),
                    );
                }
            }
            fn finish_command_list(
                &self,
                restore_deferred_context_state: bool,
            ) -> Result<CommandList, HResult> {
                Ok(CommandList(ComPtr::new(|| {
                    let mut obj = std::ptr::null_mut();
                    let res = unsafe {
                        self.0
                            .FinishCommandList(to_BOOL(restore_deferred_context_state), &mut obj)
                    };
                    hresult(obj, res)
                })?))
            }
            fn flush(&self) {
                unsafe {
                    self.0.Flush();
                }
            }
            fn generate_mips(&self, view: &impl IShaderResourceView) {
                unsafe {
                    self.0.GenerateMips(view.as_ptr() as *mut _);
                }
            }
            fn get_context_flags(&self) -> u32 {
                unsafe { self.0.GetContextFlags() }
            }
            fn get_data(
                &self,
                async_obj: &impl IAsynchronous,
                flags: Option<AsyncGetDataFlag>,
            ) -> Result<Vec<u8>, HResult> {
                unsafe {
                    let mut data = Vec::new();
                    data.resize(async_obj.get_data_size() as usize, 0);
                    let res = self.0.GetData(
                        async_obj.as_ptr() as *mut _,
                        data.as_mut_ptr() as *mut c_void,
                        data.len() as u32,
                        flags.map_or(0, |f| f.0),
                    );
                    hresult(data, res)
                }
            }
            fn get_predication(&self) -> Option<(Predicate, bool)> {
                unsafe {
                    let mut obj = std::ptr::null_mut();
                    let mut value = 0;
                    self.0.GetPredication(&mut obj, &mut value);
                    if obj == std::ptr::null_mut() {
                        None
                    } else {
                        Some((Predicate(ComPtr::from_raw(obj)), value != 0))
                    }
                }
            }
            fn get_resource_min_lod(&self, resource: &impl IResource) -> f32 {
                unsafe { self.0.GetResourceMinLOD(resource.as_ptr() as *mut _) }
            }
            fn get_type(&self) -> DeviceContextType {
                unsafe {
                    let t = self.0.GetType();
                    std::mem::transmute(t)
                }
            }
            fn gs_get_constant_buffers(
                &self,
                start_slot: u32,
                num_buffers: u32,
            ) -> Vec<Option<Buffer>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.GSGetConstantBuffers(s, n, p) },
                    start_slot,
                    num_buffers,
                )
            }
            fn gs_get_samplers(
                &self,
                start_slot: u32,
                num_samplers: u32,
            ) -> Vec<Option<SamplerState>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.GSGetSamplers(s, n, p) },
                    start_slot,
                    num_samplers,
                )
            }
            fn gs_get_shader(&self) -> (Option<GeometryShader>, Option<Vec<ClassInstance>>) {
                get_shader(|p, c, n| unsafe { self.0.GSGetShader(p, c, n) })
            }
            fn gs_get_shader_resources(
                &self,
                start_slot: u32,
                num_views: u32,
            ) -> Vec<Option<ShaderResourceView>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.GSGetShaderResources(s, n, p) },
                    start_slot,
                    num_views,
                )
            }
            fn gs_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.GSSetConstantBuffers(s, n, p as *mut _) },
                    start_slot,
                    buffers,
                )
            }
            fn gs_set_samplers(&self, start_slot: u32, samplers: &[&impl ISamplerState]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.GSSetSamplers(s, n, p as *mut _) },
                    start_slot,
                    samplers,
                )
            }
            fn gs_set_shader(&self, shader: &GeometryShader, instances: Option<&[&ClassInstance]>) {
                set_shader(
                    |p, c, n| unsafe { self.0.GSSetShader(p, c, n) },
                    shader,
                    instances,
                )
            }
            fn gs_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.GSSetShaderResources(s, n, p) },
                    start_slot,
                    views,
                )
            }
            fn hs_get_constant_buffers(
                &self,
                start_slot: u32,
                num_buffers: u32,
            ) -> Vec<Option<Buffer>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.HSGetConstantBuffers(s, n, p) },
                    start_slot,
                    num_buffers,
                )
            }
            fn hs_get_samplers(
                &self,
                start_slot: u32,
                num_samplers: u32,
            ) -> Vec<Option<SamplerState>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.HSGetSamplers(s, n, p) },
                    start_slot,
                    num_samplers,
                )
            }
            fn hs_get_shader(&self) -> (Option<HullShader>, Option<Vec<ClassInstance>>) {
                get_shader(|p, c, n| unsafe { self.0.HSGetShader(p, c, n) })
            }
            fn hs_get_shader_resources(
                &self,
                start_slot: u32,
                num_views: u32,
            ) -> Vec<Option<ShaderResourceView>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.HSGetShaderResources(s, n, p) },
                    start_slot,
                    num_views,
                )
            }
            fn hs_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.HSSetConstantBuffers(s, n, p as *mut _) },
                    start_slot,
                    buffers,
                )
            }
            fn hs_set_samplers(&self, start_slot: u32, samplers: &[&SamplerState]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.HSSetSamplers(s, n, p) },
                    start_slot,
                    samplers,
                )
            }
            fn hs_set_shader(&self, shader: &HullShader, instances: Option<&[&ClassInstance]>) {
                set_shader(
                    |p, c, n| unsafe { self.0.HSSetShader(p, c, n) },
                    shader,
                    instances,
                )
            }
            fn hs_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.HSSetShaderResources(s, n, p) },
                    start_slot,
                    views,
                )
            }
            fn ia_get_index_buffer(&self) -> Option<(Buffer, dxgi::Format, u32)> {
                unsafe {
                    let mut obj = std::ptr::null_mut();
                    let mut format = 0;
                    let mut offset = 0;
                    self.0.IAGetIndexBuffer(&mut obj, &mut format, &mut offset);
                    if obj == std::ptr::null_mut() {
                        None
                    } else {
                        Some((
                            Buffer(ComPtr::from_raw(obj)),
                            std::mem::transmute(format),
                            offset,
                        ))
                    }
                }
            }
            fn ia_get_input_layout(&self) -> Option<InputLayout> {
                unsafe {
                    let mut obj = std::ptr::null_mut();
                    self.0.IAGetInputLayout(&mut obj);
                    if obj == std::ptr::null_mut() {
                        None
                    } else {
                        Some(InputLayout(ComPtr::from_raw(obj)))
                    }
                }
            }
            fn ia_get_primitive_topology(&self) -> Option<PrimitiveTopology> {
                unsafe {
                    let mut topology = 0;
                    self.0.IAGetPrimitiveTopology(&mut topology);
                    if topology == 0 {
                        None
                    } else {
                        Some(std::mem::transmute(topology))
                    }
                }
            }
            fn ia_get_vertex_buffers(
                &self,
                start_slot: u32,
                num_buffers: u32,
            ) -> Vec<Option<(Buffer, u32, u32)>> {
                unsafe {
                    let mut buffers = Vec::with_capacity(num_buffers as usize);
                    buffers.set_len(num_buffers as usize);
                    let mut strides = Vec::with_capacity(num_buffers as usize);
                    strides.set_len(num_buffers as usize);
                    let mut offsets = Vec::with_capacity(num_buffers as usize);
                    offsets.set_len(num_buffers as usize);
                    self.0.IAGetVertexBuffers(
                        start_slot,
                        num_buffers,
                        buffers.as_mut_ptr(),
                        strides.as_mut_ptr(),
                        offsets.as_mut_ptr(),
                    );
                    let mut data: Vec<Option<(Buffer, u32, u32)>> =
                        Vec::with_capacity(num_buffers as usize);
                    for i in 0..num_buffers as usize {
                        if buffers[i] == std::ptr::null_mut() {
                            data.push(None)
                        } else {
                            data.push(Some((
                                Buffer(ComPtr::from_raw(buffers[i])),
                                strides[i],
                                offsets[i],
                            )))
                        }
                    }
                    data
                }
            }
            fn ia_set_index_buffer(
                &self,
                buffer: &impl IBuffer,
                format: dxgi::Format,
                offset: u32,
            ) {
                unsafe {
                    self.0
                        .IASetIndexBuffer(buffer.as_ptr() as *mut _, format as u32, offset);
                }
            }
            fn ia_set_input_layout(&self, input_layout: &impl IInputLayout) {
                unsafe {
                    self.0.IASetInputLayout(input_layout.as_ptr() as *mut _);
                }
            }
            fn ia_set_primitive_topology(&self, topology: PrimitiveTopology) {
                unsafe {
                    self.0.IASetPrimitiveTopology(topology as u32);
                }
            }
            fn ia_set_vertex_buffers(
                &self,
                start_slot: u32,
                buffers: &[&impl IBuffer],
                strides: &[u32],
                offsets: &[u32],
            ) {
                unsafe {
                    let bufs = buffers
                        .iter()
                        .map(|b| b.as_ptr() as *mut _)
                        .collect::<Vec<_>>();
                    assert!(bufs.len() == strides.len());
                    assert!(bufs.len() == offsets.len());
                    self.0.IASetVertexBuffers(
                        start_slot,
                        bufs.len() as u32,
                        bufs.as_ptr(),
                        strides.as_ptr(),
                        offsets.as_ptr(),
                    );
                }
            }
            fn map(
                &self,
                resource: &impl IResource,
                subresource: u32,
                map_type: MapType,
                map_flags: Option<MapFlag>,
            ) -> Result<MappedSubresource, HResult> {
                unsafe {
                    let mut mapped_subresource = Default::default();
                    let res = self.0.Map(
                        resource.as_ptr() as *mut _,
                        subresource,
                        map_type as u32,
                        map_flags.map_or(0, |f| f.0),
                        &mut mapped_subresource,
                    );
                    hresult(mapped_subresource.into(), res)
                }
            }
            fn om_get_blend_state(&self) -> Option<(BlendState, [f32; 4], u32)> {
                let mut blend_state = std::ptr::null_mut();
                let mut factor = [0.0f32; 4];
                let mut mask = 0;
                unsafe {
                    self.0
                        .OMGetBlendState(&mut blend_state, &mut factor, &mut mask);
                    if blend_state == std::ptr::null_mut() {
                        None
                    } else {
                        Some((BlendState(ComPtr::from_raw(blend_state)), factor, mask))
                    }
                }
            }
            fn om_get_depth_stencil_state(&self) -> (Option<DepthStencilState>, u32) {
                let mut state = std::ptr::null_mut();
                let mut stencil = 0;
                unsafe {
                    self.0.OMGetDepthStencilState(&mut state, &mut stencil);
                    let state = if state == std::ptr::null_mut() {
                        None
                    } else {
                        Some(DepthStencilState(ComPtr::from_raw(state)))
                    };
                    (state, stencil)
                }
            }
            fn om_get_render_targets(
                &self,
                num_views: u32,
            ) -> (Vec<Option<RenderTargetView>>, Option<DepthStencilView>) {
                unsafe {
                    let mut rtvs = Vec::with_capacity(num_views as usize);
                    rtvs.set_len(num_views as usize);
                    let mut dsv = std::ptr::null_mut();
                    self.0
                        .OMGetRenderTargets(num_views, rtvs.as_mut_ptr(), &mut dsv);
                    let rtvs = rtvs
                        .into_iter()
                        .map(|rtv| {
                            if rtv == std::ptr::null_mut() {
                                None
                            } else {
                                Some(RenderTargetView(ComPtr::from_raw(rtv)))
                            }
                        })
                        .collect::<Vec<_>>();
                    let dsv = if dsv == std::ptr::null_mut() {
                        None
                    } else {
                        Some(DepthStencilView(ComPtr::from_raw(dsv)))
                    };
                    (rtvs, dsv)
                }
            }
            // The argument of OMGetRenderTargetsAndUnorderedAccessViews is missing.
            /*
            fn om_get_render_targets_and_unordered_access_views(&self, num_rtvs: u32, uav_start_slot: u32, num_uavs: u32) -> (Vec<Option<RenderTargetView>>, Option<DepthStencilView>, Vec<Option<UnorderedAccessView>>) {
                unsafe {
                    let mut rtvs = Vec::with_capacity(num_rtvs as usize);
                    rtvs.set_len(num_rtvs as usize);
                    let mut dsv = std::ptr::null_mut();
                    let mut uavs = Vec::with_capacity(num_uavs as usize);
                    uavs.set_len(num_uavs as usize);
                    self.0.OMGetRenderTargetsAndUnorderedAccessViews(num_rtvs, rtvs.as_mut_ptr(), &mut dsv, uav_start_slot, num_uavs, uavs.as_mut_ptr());
                    let rtvs = rtvs.into_iter().map(|rtv| {
                        if rtv == std::ptr::null_mut() {
                            None
                        } else {
                            Some(RenderTargetView(ComPtr::from_raw(rtv)))
                        }
                    }).collect::<Vec<_>>();
                    let dsv = if dsv == std::ptr::null_mut() {
                        None
                    } else {
                        Some(DepthStencilView(ComPtr::from_raw(dsv)))
                    };
                    let uavs = uavs.into_iter().map(|uav| {
                        if uav == std::ptr::null_mut() {
                            None
                        } else {
                            Some(UnorderedAccessView(ComPtr::from_raw(uav)))
                        }
                    }).collect::<Vec<_>>();
                    (rtvs, dsv, uavs)
                }
            }
            */
            fn om_set_blend_state(&self, blend_state: &BlendState, factor: &[f32; 4], mask: u32) {
                unsafe {
                    self.0.OMSetBlendState(blend_state.as_ptr(), factor, mask);
                }
            }
            fn om_set_depth_stencil_state(&self, dss: &DepthStencilState, stencil_ref: u32) {
                unsafe {
                    self.0.OMSetDepthStencilState(dss.as_ptr(), stencil_ref);
                }
            }
            fn om_set_render_targets(
                &self,
                rtvs: Option<&[&RenderTargetView]>,
                dsv: Option<&DepthStencilView>,
            ) {
                let c_rtvs = rtvs.map(|r| r.iter().map(|rtv| rtv.as_ptr()).collect::<Vec<_>>());
                unsafe {
                    self.0.OMSetRenderTargets(
                        c_rtvs.as_ref().map_or(0, |r| r.len() as u32),
                        c_rtvs.as_ref().map_or(std::ptr::null(), |r| r.as_ptr()),
                        dsv.map_or(std::ptr::null_mut(), |d| d.as_ptr()),
                    );
                }
            }
            fn om_set_render_targets_and_unordered_access_views(
                &self,
                rtvs: Option<&[&RenderTargetView]>,
                dsv: Option<&DepthStencilView>,
                uav_start_slot: u32,
                uavs: Option<&[&UnorderedAccessView]>,
                initial_counts: Option<&[u32]>,
            ) {
                let c_rtvs = rtvs.map(|r| r.iter().map(|rtv| rtv.as_ptr()).collect::<Vec<_>>());
                let c_uavs = uavs.map(|r| r.iter().map(|uav| uav.as_ptr()).collect::<Vec<_>>());
                unsafe {
                    self.0.OMSetRenderTargetsAndUnorderedAccessViews(
                        c_rtvs.as_ref().map_or(0, |r| r.len() as u32),
                        c_rtvs.as_ref().map_or(std::ptr::null(), |r| r.as_ptr()),
                        dsv.map_or(std::ptr::null_mut(), |d| d.as_ptr()),
                        uav_start_slot,
                        c_uavs.as_ref().map_or(0, |u| u.len() as u32),
                        c_uavs.as_ref().map_or(std::ptr::null(), |u| u.as_ptr()),
                        initial_counts.map_or(std::ptr::null(), |ic| ic.as_ptr()),
                    );
                }
            }
            fn ps_get_constant_buffers(
                &self,
                start_slot: u32,
                num_buffers: u32,
            ) -> Vec<Option<Buffer>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.PSGetConstantBuffers(s, n, p) },
                    start_slot,
                    num_buffers,
                )
            }
            fn ps_get_samplers(
                &self,
                start_slot: u32,
                num_samplers: u32,
            ) -> Vec<Option<SamplerState>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.PSGetSamplers(s, n, p) },
                    start_slot,
                    num_samplers,
                )
            }
            fn ps_get_shader(&self) -> (Option<PixelShader>, Option<Vec<ClassInstance>>) {
                get_shader(|p, c, n| unsafe { self.0.PSGetShader(p, c, n) })
            }
            fn ps_get_shader_resources(
                &self,
                start_slot: u32,
                num_views: u32,
            ) -> Vec<Option<ShaderResourceView>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.PSGetShaderResources(s, n, p) },
                    start_slot,
                    num_views,
                )
            }
            fn ps_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.PSSetConstantBuffers(s, n, p as *mut _) },
                    start_slot,
                    buffers,
                )
            }
            fn ps_set_samplers(&self, start_slot: u32, samplers: &[&SamplerState]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.PSSetSamplers(s, n, p) },
                    start_slot,
                    samplers,
                )
            }
            fn ps_set_shader(&self, shader: &PixelShader, instances: Option<&[&ClassInstance]>) {
                set_shader(
                    |p, c, n| unsafe { self.0.PSSetShader(p, c, n) },
                    shader,
                    instances,
                )
            }
            fn ps_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.PSSetShaderResources(s, n, p) },
                    start_slot,
                    views,
                )
            }
            fn resolve_subresource(
                &self,
                dest: &impl IResource,
                dest_subresource: u32,
                src: &impl IResource,
                src_subresource: u32,
                format: dxgi::Format,
            ) {
                unsafe {
                    self.0.ResolveSubresource(
                        dest.as_ptr() as *mut _,
                        dest_subresource,
                        src.as_ptr() as *mut _,
                        src_subresource,
                        format as u32,
                    );
                }
            }
            fn rs_get_scissor_rects(&self) -> Vec<Rect> {
                unsafe {
                    let mut sz = 0;
                    self.0.RSGetScissorRects(&mut sz, std::ptr::null_mut());
                    if sz == 0 {
                        return Vec::new();
                    }
                    let mut rcs = Vec::with_capacity(sz as usize);
                    rcs.set_len(sz as usize);
                    self.0.RSGetScissorRects(&mut sz, rcs.as_mut_ptr());
                    rcs.into_iter().map(|rc| rc.into()).collect::<Vec<_>>()
                }
            }
            fn rs_get_state(&self) -> Option<RasterizerState> {
                unsafe {
                    let mut obj = std::ptr::null_mut();
                    self.0.RSGetState(&mut obj);
                    if obj == std::ptr::null_mut() {
                        None
                    } else {
                        Some(RasterizerState(ComPtr::from_raw(obj)))
                    }
                }
            }
            fn rs_get_viewports(&self) -> Vec<Viewport> {
                unsafe {
                    let mut sz = 0;
                    self.0.RSGetViewports(&mut sz, std::ptr::null_mut());
                    if sz == 0 {
                        return Vec::new();
                    }
                    let mut vps = Vec::with_capacity(sz as usize);
                    vps.set_len(sz as usize);
                    self.0.RSGetViewports(&mut sz, vps.as_mut_ptr());
                    vps.into_iter().map(|vp| vp.into()).collect::<Vec<_>>()
                }
            }
            fn rs_set_scissor_rects(&self, rects: &[Rect]) {
                unsafe {
                    self.0
                        .RSSetScissorRects(rects.len() as u32, rects.as_ptr() as *const RECT);
                }
            }
            fn rs_set_state(&self, rasterizer_state: &RasterizerState) {
                unsafe {
                    self.0.RSSetState(rasterizer_state.as_ptr());
                }
            }
            fn rs_set_viewports(&self, viewports: &[Viewport]) {
                unsafe {
                    self.0.RSSetViewports(
                        viewports.len() as u32,
                        viewports.as_ptr() as *const D3D11_VIEWPORT,
                    );
                }
            }
            fn set_predication(&self, predicate: Option<&Predicate>, value: bool) {
                unsafe {
                    self.0.SetPredication(
                        predicate.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
                        to_BOOL(value),
                    );
                }
            }
            fn set_resource_min_lod(&self, resource: &impl IResource, min_lod: f32) {
                unsafe {
                    self.0
                        .SetResourceMinLOD(resource.as_ptr() as *mut _, min_lod);
                }
            }
            fn so_get_targets(&self, num: u32) -> Vec<Option<Buffer>> {
                unsafe {
                    let mut bufs = Vec::with_capacity(num as usize);
                    bufs.set_len(num as usize);
                    self.0.SOGetTargets(num, bufs.as_mut_ptr());
                    bufs.into_iter()
                        .map(|p| {
                            if p == std::ptr::null_mut() {
                                None
                            } else {
                                Some(Buffer(ComPtr::from_raw(p)))
                            }
                        })
                        .collect::<Vec<_>>()
                }
            }
            fn so_set_targets(&self, targets: &[&impl IBuffer], offset: &[u32]) {
                let c_targets = targets
                    .iter()
                    .map(|t| t.as_ptr() as *mut _)
                    .collect::<Vec<_>>();
                unsafe {
                    self.0.SOSetTargets(
                        c_targets.len() as u32,
                        c_targets.as_ptr(),
                        offset.as_ptr(),
                    );
                }
            }
            fn unmap(&self, resource: &impl IResource, subresource: u32) {
                unsafe {
                    self.0.Unmap(resource.as_ptr() as *mut _, subresource);
                }
            }
            fn update_subresource(
                &self,
                dest: &impl IResource,
                dest_subresource: u32,
                dest_box: Option<&Box3D>,
                src: &[u8],
                src_row_pitch: u32,
                src_depth_pitch: u32,
            ) {
                unsafe {
                    self.0.UpdateSubresource(
                        dest.as_ptr() as *mut _,
                        dest_subresource,
                        dest_box.map_or(std::ptr::null(), |b| b.as_c_ptr()),
                        src.as_ptr() as *const c_void,
                        src_row_pitch,
                        src_depth_pitch,
                    );
                }
            }
            fn vs_get_constant_buffers(
                &self,
                start_slot: u32,
                num_buffers: u32,
            ) -> Vec<Option<Buffer>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.VSGetConstantBuffers(s, n, p) },
                    start_slot,
                    num_buffers,
                )
            }
            fn vs_get_samplers(
                &self,
                start_slot: u32,
                num_samplers: u32,
            ) -> Vec<Option<SamplerState>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.VSGetSamplers(s, n, p) },
                    start_slot,
                    num_samplers,
                )
            }
            fn vs_get_shader(&self) -> (Option<VertexShader>, Option<Vec<ClassInstance>>) {
                get_shader(|p, c, n| unsafe { self.0.VSGetShader(p, c, n) })
            }
            fn vs_get_shader_resources(
                &self,
                start_slot: u32,
                num_views: u32,
            ) -> Vec<Option<ShaderResourceView>> {
                shader_get_objects(
                    |s, n, p| unsafe { self.0.VSGetShaderResources(s, n, p) },
                    start_slot,
                    num_views,
                )
            }
            fn vs_set_constant_buffers(&self, start_slot: u32, buffers: &[&impl IBuffer]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.VSSetConstantBuffers(s, n, p as *mut _) },
                    start_slot,
                    buffers,
                )
            }
            fn vs_set_samplers(&self, start_slot: u32, samplers: &[&SamplerState]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.VSSetSamplers(s, n, p) },
                    start_slot,
                    samplers,
                )
            }
            fn vs_set_shader(&self, shader: &VertexShader, instances: Option<&[&ClassInstance]>) {
                set_shader(
                    |p, c, n| unsafe { self.0.VSSetShader(p, c, n) },
                    shader,
                    instances,
                )
            }
            fn vs_set_shader_resources(&self, start_slot: u32, views: &[&ShaderResourceView]) {
                shader_set_objects(
                    |s, n, p| unsafe { self.0.VSSetShaderResources(s, n, p) },
                    start_slot,
                    views,
                )
            }
        }
    };
}
impl_devicecontext!(DeviceContext, ID3D11DeviceContext);

pub trait IDomainShader: IDeviceChild {}
#[derive(Clone, Debug)]
pub struct DomainShader(ComPtr<ID3D11DomainShader>);
macro_rules! impl_domainshader {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IDomainShader for $s {}
    };
}
impl_domainshader!(DomainShader, ID3D11DomainShader);

pub trait IGeometryShader: IDeviceChild {}
#[derive(Clone, Debug)]
pub struct GeometryShader(ComPtr<ID3D11GeometryShader>);
macro_rules! impl_geometryshader {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IGeometryShader for $s {}
    };
}
impl_geometryshader!(GeometryShader, ID3D11GeometryShader);

pub trait IHullShader: IDeviceChild {}
#[derive(Clone, Debug)]
pub struct HullShader(ComPtr<ID3D11HullShader>);
macro_rules! impl_hullshader {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IHullShader for $s {}
    };
}
impl_hullshader!(HullShader, ID3D11HullShader);

pub trait IInputLayout: IDeviceChild {}
#[derive(Clone, Debug)]
pub struct InputLayout(ComPtr<ID3D11InputLayout>);
macro_rules! impl_inputlayout {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IInputLayout for $s {}
    };
}
impl_inputlayout!(InputLayout, ID3D11InputLayout);

pub trait IPixelShader: IDeviceChild {}
#[derive(Clone, Debug)]
pub struct PixelShader(ComPtr<ID3D11PixelShader>);
macro_rules! impl_pixelshader {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IPixelShader for $s {}
    };
}
impl_pixelshader!(PixelShader, ID3D11PixelShader);

pub trait IQuery: IAsynchronous {
    fn get_desc(&self) -> QueryDesc;
}
#[derive(Clone, Debug)]
pub struct Query(ComPtr<ID3D11Query>);
macro_rules! impl_query {
    ($s: ident, $interface: ident) => {
        impl_asynchronous!($s, $interface);
        impl IQuery for $s {
            fn get_desc(&self) -> QueryDesc {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_query!(Query, ID3D11Query);

pub trait IPredicate: IQuery {}
#[derive(Clone, Debug)]
pub struct Predicate(ComPtr<ID3D11Predicate>);
macro_rules! impl_predicate {
    ($s: ident, $interface: ident) => {
        impl_query!($s, $interface);
        impl IPredicate for $s {}
    };
}
impl_predicate!(Predicate, ID3D11Predicate);

pub trait IRasterizerState: IDeviceChild {
    fn get_desc(&self) -> RasterizerDesc;
}
#[derive(Clone, Debug)]
pub struct RasterizerState(ComPtr<ID3D11RasterizerState>);
macro_rules! impl_rasterizerState {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IRasterizerState for $s {
            fn get_desc(&self) -> RasterizerDesc {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_rasterizerState!(RasterizerState, ID3D11RasterizerState);

pub trait IRenderTargetView: IView {
    fn get_desc(&self) -> RenderTargetViewDesc;
}
#[derive(Clone, Debug)]
pub struct RenderTargetView(ComPtr<ID3D11RenderTargetView>);
macro_rules! impl_rendertargetview {
    ($s: ident, $interface: ident) => {
        impl_view!($s, $interface);
        impl IRenderTargetView for $s {
            fn get_desc(&self) -> RenderTargetViewDesc {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_rendertargetview!(RenderTargetView, ID3D11RenderTargetView);

pub trait ISamplerState: IDeviceChild {
    fn get_desc(&self) -> SamplerDesc<Filter>;
}
#[derive(Clone, Debug)]
pub struct SamplerState(ComPtr<ID3D11SamplerState>);
macro_rules! impl_samplerstate {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl ISamplerState for $s {
            fn get_desc(&self) -> SamplerDesc<Filter> {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_samplerstate!(SamplerState, ID3D11SamplerState);

pub trait IShaderResourceView: IView {
    fn get_desc(&self) -> ShaderResourceViewDesc;
}
#[derive(Clone, Debug)]
pub struct ShaderResourceView(ComPtr<ID3D11ShaderResourceView>);
macro_rules! impl_shader_resource_view {
    ($s: ident, $interface: ident) => {
        impl_view!($s, $interface);
        impl IShaderResourceView for $s {
            fn get_desc(&self) -> ShaderResourceViewDesc {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_shader_resource_view!(ShaderResourceView, ID3D11ShaderResourceView);

pub trait ITexture1D: IResource {
    fn get_desc(&self) -> Texture1DDesc<u32, dxgi::Format, Usage>;
}
#[derive(Clone, Debug)]
pub struct Texture1D(ComPtr<ID3D11Texture1D>);
macro_rules! impl_texture1d {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl ITexture1D for $s {
            fn get_desc(&self) -> Texture1DDesc<u32, dxgi::Format, Usage> {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_texture1d!(Texture1D, ID3D11Texture1D);

pub trait ITexture2D: IResource {
    fn get_desc(&self) -> Texture2DDesc<u32, u32, dxgi::Format, Usage>;
}
#[derive(Clone, Debug)]
pub struct Texture2D(ComPtr<ID3D11Texture2D>);
macro_rules! impl_texture2d {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl ITexture2D for $s {
            fn get_desc(&self) -> Texture2DDesc<u32, u32, dxgi::Format, Usage> {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_texture2d!(Texture2D, ID3D11Texture2D);

pub trait ITexture3D: IResource {
    fn get_desc(&self) -> Texture3DDesc<u32, u32, u32, dxgi::Format, Usage>;
}
#[derive(Clone, Debug)]
pub struct Texture3D(ComPtr<ID3D11Texture3D>);
macro_rules! impl_texture3d {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl ITexture3D for $s {
            fn get_desc(&self) -> Texture3DDesc<u32, u32, u32, dxgi::Format, Usage> {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_texture3d!(Texture3D, ID3D11Texture3D);

pub trait IUnorderedAccessView: IView {
    fn get_desc(&self) -> UnorderedAccessViewDesc;
}
#[derive(Clone, Debug)]
pub struct UnorderedAccessView(ComPtr<ID3D11UnorderedAccessView>);
macro_rules! impl_unordered_access_view {
    ($s: ident, $interface: ident) => {
        impl_view!($s, $interface);
        impl IUnorderedAccessView for $s {
            fn get_desc(&self) -> UnorderedAccessViewDesc {
                let mut desc = Default::default();
                unsafe { self.0.GetDesc(&mut desc) };
                desc.into()
            }
        }
    };
}
impl_unordered_access_view!(UnorderedAccessView, ID3D11UnorderedAccessView);

pub trait IVertexShader: IDeviceChild {}
#[derive(Clone, Debug)]
pub struct VertexShader(ComPtr<ID3D11VertexShader>);
macro_rules! impl_vertexshader {
    ($s: ident, $interface: ident) => {
        impl_devicechild!($s, $interface);
        impl IVertexShader for $s {}
    };
}
impl_vertexshader!(VertexShader, ID3D11VertexShader);

pub fn calc_subresource(mip_slice: u32, array_slice: u32, mip_levels: u32) -> u32 {
    D3D11CalcSubresource(mip_slice, array_slice, mip_levels)
}

pub fn create_device(
    adapter: Option<&dxgi::Adapter>,
    driver_type: d3d::DriverType,
    software: Option<winapi::shared::minwindef::HMODULE>,
    flags: Option<CreateDeviceFlags>,
    feature_levels: &[d3d::FeatureLevel],
) -> Result<(Device, d3d::FeatureLevel, DeviceContext), HResult> {
    unsafe {
        let mut device = std::ptr::null_mut();
        let mut level = 0;
        let mut device_context = std::ptr::null_mut();
        let c_feature_levels = feature_levels.iter().map(|&l| l.into()).collect::<Vec<_>>();
        let res = D3D11CreateDevice(
            adapter.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
            driver_type as u32,
            software.unwrap_or(std::ptr::null_mut()),
            flags.map_or(0, |f| f.0 as u32),
            c_feature_levels.as_ptr(),
            c_feature_levels.len() as u32,
            D3D11_SDK_VERSION,
            &mut device,
            &mut level,
            &mut device_context,
        );
        hresult(
            (
                Device(ComPtr::from_raw(device)),
                level.into(),
                DeviceContext(ComPtr::from_raw(device_context)),
            ),
            res,
        )
    }
}

pub fn create_device_and_swap_chain(
    adapter: Option<&dxgi::Adapter>,
    driver_type: d3d::DriverType,
    software: Option<winapi::shared::minwindef::HMODULE>,
    flags: Option<CreateDeviceFlags>,
    feature_levels: &[d3d::FeatureLevel],
    swap_chain_desc: &dxgi::SwapChainDesc<
        dxgi::ModeDesc<u32, u32, dxgi::Rational, dxgi::Format>,
        dxgi::Usage,
        u32,
        *const c_void,
        bool,
        dxgi::SwapEffect,
    >,
) -> Result<(dxgi::SwapChain, Device, d3d::FeatureLevel, DeviceContext), HResult> {
    unsafe {
        let mut swap_chain = std::ptr::null_mut();
        let mut device = std::ptr::null_mut();
        let mut level = 0;
        let mut device_context = std::ptr::null_mut();
        let c_feature_levels = feature_levels.iter().map(|&l| l.into()).collect::<Vec<_>>();
        let res = D3D11CreateDeviceAndSwapChain(
            adapter.map_or(std::ptr::null_mut(), |p| p.as_ptr()),
            driver_type as u32,
            software.unwrap_or(std::ptr::null_mut()),
            flags.map_or(0, |f| f.0 as u32),
            c_feature_levels.as_ptr(),
            c_feature_levels.len() as u32,
            D3D11_SDK_VERSION,
            &swap_chain_desc.to_c_struct(),
            &mut swap_chain,
            &mut device,
            &mut level,
            &mut device_context,
        );
        hresult(
            (
                dxgi::SwapChain(ComPtr::from_raw(swap_chain)),
                Device(ComPtr::from_raw(device)),
                level.into(),
                DeviceContext(ComPtr::from_raw(device_context)),
            ),
            res,
        )
    }
}
