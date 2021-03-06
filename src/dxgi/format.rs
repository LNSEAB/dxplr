use winapi::shared::dxgiformat::*;

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
impl Default for Format {
    fn default() -> Self {
        Format::Unknown
    }
}
