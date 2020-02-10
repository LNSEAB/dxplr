#[cfg(feature = "dxgi")]
use crate::dxgi;
use crate::result::{hresult, HResult};
use crate::utility::*;
use crate::Interface;
use crate::{impl_bitflag_operators, impl_interface};
use com_ptr::ComPtr;
use winapi::shared::minwindef::TRUE;
use winapi::shared::windef::{HDC, HWND};
use winapi::um::d2d1::*;
#[cfg(feature = "d2d1_1")]
use winapi::um::d2d1_1::*;
#[cfg(feature = "d2d1_3")]
use winapi::um::d2d1_3::*;
use winapi::um::d2d1effectauthor::*;
use winapi::um::d2d1effects::*;
use winapi::um::d2d1svg::*;
use winapi::um::dcommon::*;
#[cfg(feature = "d2d1_2")]
use winapi::um::{d2d1_2::*, d2d1effects_1::*};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Tag(pub u64);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum _2DAffineTransformInterpolationMode {
    NearestNeighbor = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_LINEAR,
    Cubic = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_CUBIC,
    MultiSampleLinear = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_2DAFFINETRANSFORM_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum _2DAffineTransformProp {
    InterpolationMode = D2D1_2DAFFINETRANSFORM_PROP_INTERPOLATION_MODE,
    BorderMode = D2D1_2DAFFINETRANSFORM_PROP_BORDER_MODE,
    TransformMatrix = D2D1_2DAFFINETRANSFORM_PROP_TRANSFORM_MATRIX,
    Sharpness = D2D1_2DAFFINETRANSFORM_PROP_SHARPNESS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum _3DPerspectiveTransformInterpolationMode {
    NearestNeighbor = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_LINEAR,
    Cubic = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_CUBIC,
    MultiSampleLinear = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_3DPERSPECTIVETRANSFORM_INTERPOLATION_MODE_ANISOTROPIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum _3DPerspectiveTransformProp {
    InterpolationMode = D2D1_3DPERSPECTIVETRANSFORM_PROP_INTERPOLATION_MODE,
    BorderMode = D2D1_3DPERSPECTIVETRANSFORM_PROP_BORDER_MODE,
    Depth = D2D1_3DPERSPECTIVETRANSFORM_PROP_DEPTH,
    PerspectiveOrigin = D2D1_3DPERSPECTIVETRANSFORM_PROP_PERSPECTIVE_ORIGIN,
    LocalOffset = D2D1_3DPERSPECTIVETRANSFORM_PROP_LOCAL_OFFSET,
    GlobalOffset = D2D1_3DPERSPECTIVETRANSFORM_PROP_GLOBAL_OFFSET,
    RotationOrigin = D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION_ORIGIN,
    Rotation = D2D1_3DPERSPECTIVETRANSFORM_PROP_ROTATION,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum _3DTransformInterpolationMode {
    NearestNeighbor = D2D1_3DTRANSFORM_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_3DTRANSFORM_INTERPOLATION_MODE_LINEAR,
    Cubic = D2D1_3DTRANSFORM_INTERPOLATION_MODE_CUBIC,
    MultiSampleLinear = D2D1_3DTRANSFORM_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_3DTRANSFORM_INTERPOLATION_MODE_ANISOTROPIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum _3DTransformProp {
    InterpolationMode = D2D1_3DTRANSFORM_PROP_INTERPOLATION_MODE,
    BorderMode = D2D1_3DTRANSFORM_PROP_BORDER_MODE,
    TransformMatrix = D2D1_3DTRANSFORM_PROP_TRANSFORM_MATRIX,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AlphaMode {
    Unknown = D2D1_ALPHA_MODE_UNKNOWN,
    Premultiplied = D2D1_ALPHA_MODE_PREMULTIPLIED,
    Straight = D2D1_ALPHA_MODE_STRAIGHT,
    Ignore = D2D1_ALPHA_MODE_IGNORE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AntialiasMode {
    PerPrimitive = D2D1_ANTIALIAS_MODE_PER_PRIMITIVE,
    Aliased = D2D1_ANTIALIAS_MODE_ALIASED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ArcSize {
    Small = D2D1_ARC_SIZE_SMALL,
    Large = D2D1_ARC_SIZE_LARGE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ArithmeticCompositeProp {
    Coefficients = D2D1_ARITHMETICCOMPOSITE_PROP_COEFFICIENTS,
    ClampOutput = D2D1_ARITHMETICCOMPOSITE_PROP_CLAMP_OUTPUT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AtlasProp {
    InputRect = D2D1_ATLAS_PROP_INPUT_RECT,
    InputPadingRect = D2D1_ATLAS_PROP_INPUT_PADDING_RECT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BitmapSourceAlphaMode {
    Premultiplied = D2D1_BITMAPSOURCE_ALPHA_MODE_PREMULTIPLIED,
    Straight = D2D1_BITMAPSOURCE_ALPHA_MODE_STRAIGHT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BitmapSourceInterpolationMode {
    NearstNeighbor = D2D1_BITMAPSOURCE_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_BITMAPSOURCE_INTERPOLATION_MODE_LINEAR,
    Cubic = D2D1_BITMAPSOURCE_INTERPOLATION_MODE_CUBIC,
    Fant = D2D1_BITMAPSOURCE_INTERPOLATION_MODE_FANT,
    MipmapLinear = D2D1_BITMAPSOURCE_INTERPOLATION_MODE_MIPMAP_LINEAR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BitmapSourceOrientation {
    Default = D2D1_BITMAPSOURCE_ORIENTATION_DEFAULT,
    FlipHorizontal = D2D1_BITMAPSOURCE_ORIENTATION_FLIP_HORIZONTAL,
    RotateClockwise180 = D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180,
    RotateClockwise180FlipHorizontal =
        D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL,
    RotateClockwise270FlipHorizontal =
        D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL,
    RotateClockwise90 = D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90,
    RotateClockwise90FlipHorizontal =
        D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL,
    RotateClockwise270 = D2D1_BITMAPSOURCE_ORIENTATION_ROTATE_CLOCKWISE270,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BitmapSourceProp {
    WICBitmapSource = D2D1_BITMAPSOURCE_PROP_WIC_BITMAP_SOURCE,
    Scale = D2D1_BITMAPSOURCE_PROP_SCALE,
    InterpolationMode = D2D1_BITMAPSOURCE_PROP_INTERPOLATION_MODE,
    EnableDPICorrection = D2D1_BITMAPSOURCE_PROP_ENABLE_DPI_CORRECTION,
    AlphaMode = D2D1_BITMAPSOURCE_PROP_ALPHA_MODE,
    Orientation = D2D1_BITMAPSOURCE_PROP_ORIENTATION,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BitmapInterpolationMode {
    NearestNeighbor = D2D1_BITMAP_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_BITMAP_INTERPOLATION_MODE_LINEAR,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BitmapOptions(u32);
#[cfg(feature = "d2d1_1")]
#[allow(non_upper_case_globals)]
impl BitmapOptions {
    pub const None: Self = Self(D2D1_BITMAP_OPTIONS_NONE);
    pub const Target: Self = Self(D2D1_BITMAP_OPTIONS_TARGET);
    pub const CannotDraw: Self = Self(D2D1_BITMAP_OPTIONS_CANNOT_DRAW);
    pub const CPURead: Self = Self(D2D1_BITMAP_OPTIONS_CPU_READ);
    pub const GDICompatible: Self = Self(D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE);
}
#[cfg(feature = "d2d1_1")]
impl_bitflag_operators!(BitmapOptions);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Blend {
    Zero = D2D1_BLEND_ZERO,
    One = D2D1_BLEND_ONE,
    SrcColor = D2D1_BLEND_SRC_COLOR,
    InvSrcColor = D2D1_BLEND_INV_SRC_COLOR,
    SrcAlpha = D2D1_BLEND_SRC_ALPHA,
    InvSrcAlpha = D2D1_BLEND_INV_SRC_ALPHA,
    DestAlpha = D2D1_BLEND_DEST_ALPHA,
    InvDestAlpha = D2D1_BLEND_INV_DEST_ALPHA,
    DestColor = D2D1_BLEND_DEST_COLOR,
    InvDestColor = D2D1_BLEND_INV_DEST_COLOR,
    SrcAlphaSat = D2D1_BLEND_SRC_ALPHA_SAT,
    BlendFactor = D2D1_BLEND_BLEND_FACTOR,
    InvBlendFactor = D2D1_BLEND_INV_BLEND_FACTOR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BlendMode {
    Multiply = D2D1_BLEND_MODE_MULTIPLY,
    Screen = D2D1_BLEND_MODE_SCREEN,
    Darken = D2D1_BLEND_MODE_DARKEN,
    Lighten = D2D1_BLEND_MODE_LIGHTEN,
    Dissolve = D2D1_BLEND_MODE_DISSOLVE,
    ColorBurn = D2D1_BLEND_MODE_COLOR_BURN,
    LinearBurn = D2D1_BLEND_MODE_LINEAR_BURN,
    DarkerColor = D2D1_BLEND_MODE_DARKER_COLOR,
    LighterColor = D2D1_BLEND_MODE_LIGHTER_COLOR,
    ColorDodge = D2D1_BLEND_MODE_COLOR_DODGE,
    LinearDodge = D2D1_BLEND_MODE_LINEAR_DODGE,
    Overlay = D2D1_BLEND_MODE_OVERLAY,
    SoftLight = D2D1_BLEND_MODE_SOFT_LIGHT,
    HardLight = D2D1_BLEND_MODE_HARD_LIGHT,
    VividLight = D2D1_BLEND_MODE_VIVID_LIGHT,
    LinearLight = D2D1_BLEND_MODE_LINEAR_LIGHT,
    PinLight = D2D1_BLEND_MODE_PIN_LIGHT,
    HardMix = D2D1_BLEND_MODE_HARD_MIX,
    Difference = D2D1_BLEND_MODE_DIFFERENCE,
    Exclusion = D2D1_BLEND_MODE_EXCLUSION,
    Hue = D2D1_BLEND_MODE_HUE,
    Saturation = D2D1_BLEND_MODE_SATURATION,
    Color = D2D1_BLEND_MODE_COLOR,
    Luminosity = D2D1_BLEND_MODE_LUMINOSITY,
    Subtract = D2D1_BLEND_MODE_SUBTRACT,
    Division = D2D1_BLEND_MODE_DIVISION,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BlendOperation {
    Add = D2D1_BLEND_OPERATION_ADD,
    Subtract = D2D1_BLEND_OPERATION_SUBTRACT,
    RevSubtract = D2D1_BLEND_OPERATION_REV_SUBTRACT,
    Min = D2D1_BLEND_OPERATION_MIN,
    Max = D2D1_BLEND_OPERATION_MAX,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BlendProp {
    Mode = D2D1_BLEND_PROP_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BorderEdgeMode {
    Clamp = D2D1_BORDER_EDGE_MODE_CLAMP,
    Wrap = D2D1_BORDER_EDGE_MODE_WRAP,
    Mirror = D2D1_BORDER_EDGE_MODE_MIRROR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BorderMode {
    Soft = D2D1_BORDER_MODE_SOFT,
    Hard = D2D1_BORDER_MODE_HARD,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BorderProp {
    EdgeModeX = D2D1_BORDER_PROP_EDGE_MODE_X,
    EdgeModeY = D2D1_BORDER_PROP_EDGE_MODE_Y,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BrightnessProp {
    WhitePoint = D2D1_BRIGHTNESS_PROP_WHITE_POINT,
    BlackPoint = D2D1_BRIGHTNESS_PROP_BLACK_POINT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BufferPrecision {
    Unknown = D2D1_BUFFER_PRECISION_UNKNOWN,
    _8BpcUnorm = D2D1_BUFFER_PRECISION_8BPC_UNORM,
    _8BpcUnormSRGB = D2D1_BUFFER_PRECISION_8BPC_UNORM_SRGB,
    _16BpcUnorm = D2D1_BUFFER_PRECISION_16BPC_UNORM,
    _16BpcFloat = D2D1_BUFFER_PRECISION_16BPC_FLOAT,
    _32BpcFloat = D2D1_BUFFER_PRECISION_32BPC_FLOAT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CapStyle {
    Flat = D2D1_CAP_STYLE_FLAT,
    Square = D2D1_CAP_STYLE_SQUARE,
    Round = D2D1_CAP_STYLE_ROUND,
    Triangle = D2D1_CAP_STYLE_TRIANGLE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ChangeType {
    None = D2D1_CHANGE_TYPE_NONE,
    Properties = D2D1_CHANGE_TYPE_PROPERTIES,
    Context = D2D1_CHANGE_TYPE_CONTEXT,
    Graph = D2D1_CHANGE_TYPE_GRAPH,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ChannelDepth {
    Default = D2D1_CHANNEL_DEPTH_DEFAULT,
    _1 = D2D1_CHANNEL_DEPTH_1,
    _4 = D2D1_CHANNEL_DEPTH_4,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ChannelSelector {
    R = D2D1_CHANNEL_SELECTOR_R,
    G = D2D1_CHANNEL_SELECTOR_G,
    B = D2D1_CHANNEL_SELECTOR_B,
    A = D2D1_CHANNEL_SELECTOR_A,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ChromakeyProp {
    Color = D2D1_CHROMAKEY_PROP_COLOR,
    Tolerance = D2D1_CHROMAKEY_PROP_TOLERANCE,
    InvertAlpha = D2D1_CHROMAKEY_PROP_INVERT_ALPHA,
    Feather = D2D1_CHROMAKEY_PROP_FEATHER,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorManagementAlphaMode {
    Premultiplied = D2D1_COLORMANAGEMENT_ALPHA_MODE_PREMULTIPLIED,
    Straight = D2D1_COLORMANAGEMENT_ALPHA_MODE_STRAIGHT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorManagementProp {
    SourceColorContext = D2D1_COLORMANAGEMENT_PROP_SOURCE_COLOR_CONTEXT,
    SourceRenderingIntent = D2D1_COLORMANAGEMENT_PROP_SOURCE_RENDERING_INTENT,
    DestinationColorContext = D2D1_COLORMANAGEMENT_PROP_DESTINATION_COLOR_CONTEXT,
    DestinationRenderingIntent = D2D1_COLORMANAGEMENT_PROP_DESTINATION_RENDERING_INTENT,
    AlphaMode = D2D1_COLORMANAGEMENT_PROP_ALPHA_MODE,
    Quality = D2D1_COLORMANAGEMENT_PROP_QUALITY,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorManagementQuality {
    Proof = D2D1_COLORMANAGEMENT_QUALITY_PROOF,
    Normal = D2D1_COLORMANAGEMENT_QUALITY_NORMAL,
    QualityBest = D2D1_COLORMANAGEMENT_QUALITY_BEST,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorManagementRenderingIntent {
    Perceptual = D2D1_COLORMANAGEMENT_RENDERING_INTENT_PERCEPTUAL,
    RelativeColormetric = D2D1_COLORMANAGEMENT_RENDERING_INTENT_RELATIVE_COLORIMETRIC,
    Saturation = D2D1_COLORMANAGEMENT_RENDERING_INTENT_SATURATION,
    AbsoluteColormetric = D2D1_COLORMANAGEMENT_RENDERING_INTENT_ABSOLUTE_COLORIMETRIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorMatrixAlphaMode {
    Premultiplied = D2D1_COLORMATRIX_ALPHA_MODE_PREMULTIPLIED,
    Straight = D2D1_COLORMATRIX_ALPHA_MODE_STRAIGHT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorMatrixProp {
    ColorMatrix = D2D1_COLORMATRIX_PROP_COLOR_MATRIX,
    AlphaMode = D2D1_COLORMATRIX_PROP_ALPHA_MODE,
    ClampOutput = D2D1_COLORMATRIX_PROP_CLAMP_OUTPUT,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorBitmapGlyphSnapOption {
    Default = D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT,
    Disable = D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DISABLE,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorContextType {
    ICC = D2D1_COLOR_CONTEXT_TYPE_ICC,
    Simple = D2D1_COLOR_CONTEXT_TYPE_SIMPLE,
    DXGI = D2D1_COLOR_CONTEXT_TYPE_DXGI,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorSpace {
    Custom = D2D1_COLOR_SPACE_CUSTOM,
    SRGB = D2D1_COLOR_SPACE_SRGB,
    ScRGB = D2D1_COLOR_SPACE_SCRGB,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorInterpolationMode {
    Straight = D2D1_COLOR_INTERPOLATION_MODE_STRAIGHT,
    Premultiplied = D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CombineMode {
    Union = D2D1_COMBINE_MODE_UNION,
    Intersect = D2D1_COMBINE_MODE_INTERSECT,
    Xor = D2D1_COMBINE_MODE_XOR,
    Exclude = D2D1_COMBINE_MODE_EXCLUDE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CompatibleRenderTargetOptions {
    None = D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_NONE,
    GDICompatible = D2D1_COMPATIBLE_RENDER_TARGET_OPTIONS_GDI_COMPATIBLE,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CompositeMode {
    SourceOver = D2D1_COMPOSITE_MODE_SOURCE_OVER,
    DestinationOver = D2D1_COMPOSITE_MODE_DESTINATION_OVER,
    SourceIn = D2D1_COMPOSITE_MODE_SOURCE_IN,
    DestinationIn = D2D1_COMPOSITE_MODE_DESTINATION_IN,
    SourceOut = D2D1_COMPOSITE_MODE_SOURCE_OUT,
    DestinationOut = D2D1_COMPOSITE_MODE_DESTINATION_OUT,
    SourceAtop = D2D1_COMPOSITE_MODE_SOURCE_ATOP,
    DestinationAtop = D2D1_COMPOSITE_MODE_DESTINATION_ATOP,
    Xor = D2D1_COMPOSITE_MODE_XOR,
    Plus = D2D1_COMPOSITE_MODE_PLUS,
    SourceCopy = D2D1_COMPOSITE_MODE_SOURCE_COPY,
    BoundedSourceCopy = D2D1_COMPOSITE_MODE_BOUNDED_SOURCE_COPY,
    MaskInvert = D2D1_COMPOSITE_MODE_MASK_INVERT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CompositeProp {
    Mode = D2D1_COMPOSITE_PROP_MODE,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ContractProp {
    Contrast = D2D1_CONTRAST_PROP_CONTRAST,
    ClampInput = D2D1_CONTRAST_PROP_CLAMP_INPUT,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ConvolveMatrixProp {
    KernelUnitLength = D2D1_CONVOLVEMATRIX_PROP_KERNEL_UNIT_LENGTH,
    ScaleMode = D2D1_CONVOLVEMATRIX_PROP_SCALE_MODE,
    KernelSizeX = D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_X,
    KernelSizeY = D2D1_CONVOLVEMATRIX_PROP_KERNEL_SIZE_Y,
    KernelMatrix = D2D1_CONVOLVEMATRIX_PROP_KERNEL_MATRIX,
    Divisor = D2D1_CONVOLVEMATRIX_PROP_DIVISOR,
    Bias = D2D1_CONVOLVEMATRIX_PROP_BIAS,
    KernelOffset = D2D1_CONVOLVEMATRIX_PROP_KERNEL_OFFSET,
    PreserveAlpha = D2D1_CONVOLVEMATRIX_PROP_PRESERVE_ALPHA,
    BorderMode = D2D1_CONVOLVEMATRIX_PROP_BORDER_MODE,
    ClampOutput = D2D1_CONVOLVEMATRIX_PROP_CLAMP_OUTPUT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ConvolveMatrixScaleMode {
    NearestNeighbor = D2D1_CONVOLVEMATRIX_SCALE_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_CONVOLVEMATRIX_SCALE_MODE_LINEAR,
    Cubic = D2D1_CONVOLVEMATRIX_SCALE_MODE_CUBIC,
    MultiSampleLinear = D2D1_CONVOLVEMATRIX_SCALE_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_CONVOLVEMATRIX_SCALE_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_CONVOLVEMATRIX_SCALE_MODE_HIGH_QUALITY_CUBIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum CropProp {
    Rect = D2D1_CROP_PROP_RECT,
    BorderMode = D2D1_CROP_PROP_BORDER_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DashStyle {
    Solid = D2D1_DASH_STYLE_SOLID,
    Dash = D2D1_DASH_STYLE_DASH,
    Dot = D2D1_DASH_STYLE_DOT,
    DashDot = D2D1_DASH_STYLE_DASH_DOT,
    DastDotDot = D2D1_DASH_STYLE_DASH_DOT_DOT,
    Custom = D2D1_DASH_STYLE_CUSTOM,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DCInitializeMode {
    Copy = D2D1_DC_INITIALIZE_MODE_COPY,
    Clear = D2D1_DC_INITIALIZE_MODE_CLEAR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DebugLevel {
    None = D2D1_DEBUG_LEVEL_NONE,
    Error = D2D1_DEBUG_LEVEL_ERROR,
    Warning = D2D1_DEBUG_LEVEL_WARNING,
    Information = D2D1_DEBUG_LEVEL_INFORMATION,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DeviceContextOptions {
    None = D2D1_DEVICE_CONTEXT_OPTIONS_NONE,
    EnableMultiThreadedOptimizations =
        D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DirectionalBlurOptimization {
    Speed = D2D1_DIRECTIONALBLUR_OPTIMIZATION_SPEED,
    Balanced = D2D1_DIRECTIONALBLUR_OPTIMIZATION_BALANCED,
    Quality = D2D1_DIRECTIONALBLUR_OPTIMIZATION_QUALITY,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DirectionalBlurProp {
    StandardDeviation = D2D1_DIRECTIONALBLUR_PROP_STANDARD_DEVIATION,
    Angle = D2D1_DIRECTIONALBLUR_PROP_ANGLE,
    Optimization = D2D1_DIRECTIONALBLUR_PROP_OPTIMIZATION,
    BorderMode = D2D1_DIRECTIONALBLUR_PROP_BORDER_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DiscreteTransferProp {
    RedTable = D2D1_DISCRETETRANSFER_PROP_RED_TABLE,
    RedDisable = D2D1_DISCRETETRANSFER_PROP_RED_DISABLE,
    GreenTable = D2D1_DISCRETETRANSFER_PROP_GREEN_TABLE,
    GreenDisable = D2D1_DISCRETETRANSFER_PROP_GREEN_DISABLE,
    BlueTable = D2D1_DISCRETETRANSFER_PROP_BLUE_TABLE,
    BlueDisable = D2D1_DISCRETETRANSFER_PROP_BLUE_DISABLE,
    AlphaTable = D2D1_DISCRETETRANSFER_PROP_ALPHA_TABLE,
    AlphaDisable = D2D1_DISCRETETRANSFER_PROP_ALPHA_DISABLE,
    ClampOutput = D2D1_DISCRETETRANSFER_PROP_CLAMP_OUTPUT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DisplacementMapProp {
    Scale = D2D1_DISPLACEMENTMAP_PROP_SCALE,
    XChannelSelect = D2D1_DISPLACEMENTMAP_PROP_X_CHANNEL_SELECT,
    YChannelSelect = D2D1_DISPLACEMENTMAP_PROP_Y_CHANNEL_SELECT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DistantDiffuseProp {
    Azimuth = D2D1_DISTANTDIFFUSE_PROP_AZIMUTH,
    Elevation = D2D1_DISTANTDIFFUSE_PROP_ELEVATION,
    DiffuseConstant = D2D1_DISTANTDIFFUSE_PROP_DIFFUSE_CONSTANT,
    SurfaceScale = D2D1_DISTANTDIFFUSE_PROP_SURFACE_SCALE,
    Color = D2D1_DISTANTDIFFUSE_PROP_COLOR,
    KernelUnitLength = D2D1_DISTANTDIFFUSE_PROP_KERNEL_UNIT_LENGTH,
    ScaleMode = D2D1_DISTANTDIFFUSE_PROP_SCALE_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DistantDiffuseScaleMode {
    NearestNeighbor = D2D1_DISTANTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_DISTANTDIFFUSE_SCALE_MODE_LINEAR,
    Cubic = D2D1_DISTANTDIFFUSE_SCALE_MODE_CUBIC,
    MultiSampleLinear = D2D1_DISTANTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_DISTANTDIFFUSE_SCALE_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_DISTANTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DistantSpecularProp {
    Azimuth = D2D1_DISTANTSPECULAR_PROP_AZIMUTH,
    Elevation = D2D1_DISTANTSPECULAR_PROP_ELEVATION,
    SpecularExponent = D2D1_DISTANTSPECULAR_PROP_SPECULAR_EXPONENT,
    SpecularConstant = D2D1_DISTANTSPECULAR_PROP_SPECULAR_CONSTANT,
    SurfaceScale = D2D1_DISTANTSPECULAR_PROP_SURFACE_SCALE,
    Color = D2D1_DISTANTSPECULAR_PROP_COLOR,
    KernelUnitLength = D2D1_DISTANTSPECULAR_PROP_KERNEL_UNIT_LENGTH,
    ScaleMode = D2D1_DISTANTSPECULAR_PROP_SCALE_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DistantSpecularScaleMode {
    NearestNeighbor = D2D1_DISTANTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_DISTANTSPECULAR_SCALE_MODE_LINEAR,
    Cubic = D2D1_DISTANTSPECULAR_SCALE_MODE_CUBIC,
    MultiSampleLinear = D2D1_DISTANTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_DISTANTSPECULAR_SCALE_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_DISTANTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DPICompensationInterpolationMode {
    NearestNeighbor = D2D1_DPICOMPENSATION_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_DPICOMPENSATION_INTERPOLATION_MODE_LINEAR,
    Cubic = D2D1_DPICOMPENSATION_INTERPOLATION_MODE_CUBIC,
    MultiSampleLinear = D2D1_DPICOMPENSATION_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_DPICOMPENSATION_INTERPOLATION_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_DPICOMPENSATION_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum DPICompensationProp {
    InterpolationMode = D2D1_DPICOMPENSATION_PROP_INTERPOLATION_MODE,
    BorderMode = D2D1_DPICOMPENSATION_PROP_BORDER_MODE,
    InputDPI = D2D1_DPICOMPENSATION_PROP_INPUT_DPI,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct DrawTextOptions(u32);
#[allow(non_upper_case_globals)]
impl DrawTextOptions {
    pub const NoSnap: Self = Self(D2D1_DRAW_TEXT_OPTIONS_NO_SNAP);
    pub const Clip: Self = Self(D2D1_DRAW_TEXT_OPTIONS_CLIP);
    pub const EnableColorFont: Self = Self(D2D1_DRAW_TEXT_OPTIONS_ENABLE_COLOR_FONT);
    pub const DisableColorBitmapSnapping: Self = Self(0x00000008);
    pub const None: Self = Self(D2D1_DRAW_TEXT_OPTIONS_NONE);
}
impl_bitflag_operators!(DrawTextOptions);

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum EdgeDetectionMode {
    Sobel = D2D1_EDGEDETECTION_MODE_SOBEL,
    Prewitt = D2D1_EDGEDETECTION_MODE_PREWITT,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum EdgeDetectionProp {
    Strength = D2D1_EDGEDETECTION_PROP_STRENGTH,
    BlurRadius = D2D1_EDGEDETECTION_PROP_BLUR_RADIUS,
    Mode = D2D1_EDGEDETECTION_PROP_MODE,
    OverlayEdges = D2D1_EDGEDETECTION_PROP_OVERLAY_EDGES,
    AlphaMode = D2D1_EDGEDETECTION_PROP_ALPHA_MODE,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum EmbossProp {
    Height = D2D1_EMBOSS_PROP_HEIGHT,
    Direction = D2D1_EMBOSS_PROP_DIRECTION,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ExposureProp {
    ExposureValue = D2D1_EXPOSURE_PROP_EXPOSURE_VALUE,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ExtendMode {
    Clamp = D2D1_EXTEND_MODE_CLAMP,
    Wrap = D2D1_EXTEND_MODE_WRAP,
    Mirror = D2D1_EXTEND_MODE_MIRROR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FactoryType {
    SingleThreaded = D2D1_FACTORY_TYPE_SINGLE_THREADED,
    MultiThreaded = D2D1_FACTORY_TYPE_MULTI_THREADED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Feature {
    Doubles = D2D1_FEATURE_DOUBLES,
    D3D10XHardwareOptions = D2D1_FEATURE_D3D10_X_HARDWARE_OPTIONS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FeatureLevel {
    Default = D2D1_FEATURE_LEVEL_DEFAULT,
    _9 = D2D1_FEATURE_LEVEL_9,
    _10 = D2D1_FEATURE_LEVEL_10,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FigureBegin {
    Filled = D2D1_FIGURE_BEGIN_FILLED,
    Hollow = D2D1_FIGURE_BEGIN_HOLLOW,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FigureEnd {
    Open = D2D1_FIGURE_END_OPEN,
    Closed = D2D1_FIGURE_END_CLOSED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FillMode {
    Alternate = D2D1_FILL_MODE_ALTERNATE,
    Winding = D2D1_FILL_MODE_WINDING,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Filter {
    MinMagMipPoint = D2D1_FILTER_MIN_MAG_MIP_POINT,
    MinMagPointMipLinear = D2D1_FILTER_MIN_MAG_POINT_MIP_LINEAR,
    MinPointMagLinearMipPoint = D2D1_FILTER_MIN_POINT_MAG_LINEAR_MIP_POINT,
    MinPointMagMipLinear = D2D1_FILTER_MIN_POINT_MAG_MIP_LINEAR,
    MinLinearMagMipPoint = D2D1_FILTER_MIN_LINEAR_MAG_MIP_POINT,
    MinLinearMagPointMipLinear = D2D1_FILTER_MIN_LINEAR_MAG_POINT_MIP_LINEAR,
    MinMagLinearMipPoint = D2D1_FILTER_MIN_MAG_LINEAR_MIP_POINT,
    MinMagMipLinear = D2D1_FILTER_MIN_MAG_MIP_LINEAR,
    Anisotropic = D2D1_FILTER_ANISOTROPIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FloodProp {
    Color = D2D1_FLOOD_PROP_COLOR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Gamma {
    _2_2 = D2D1_GAMMA_2_2,
    _1_0 = D2D1_GAMMA_1_0,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Gamma1 {
    G22 = D2D1_GAMMA1_G22,
    G10 = D2D1_GAMMA1_G10,
    G2084 = D2D1_GAMMA1_G2084,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GammaTransferProp {
    RedAmplitude = D2D1_GAMMATRANSFER_PROP_RED_AMPLITUDE,
    RedExponent = D2D1_GAMMATRANSFER_PROP_RED_EXPONENT,
    RedOffset = D2D1_GAMMATRANSFER_PROP_RED_OFFSET,
    RedDisable = D2D1_GAMMATRANSFER_PROP_RED_DISABLE,
    GreenAmplitude = D2D1_GAMMATRANSFER_PROP_GREEN_AMPLITUDE,
    GreenExponent = D2D1_GAMMATRANSFER_PROP_GREEN_EXPONENT,
    GreenOffset = D2D1_GAMMATRANSFER_PROP_GREEN_OFFSET,
    GreenDisable = D2D1_GAMMATRANSFER_PROP_GREEN_DISABLE,
    BlueAmplitude = D2D1_GAMMATRANSFER_PROP_BLUE_AMPLITUDE,
    BlueExponent = D2D1_GAMMATRANSFER_PROP_BLUE_EXPONENT,
    BlueOffset = D2D1_GAMMATRANSFER_PROP_BLUE_OFFSET,
    BlueDisable = D2D1_GAMMATRANSFER_PROP_BLUE_DISABLE,
    AlphaAmplitude = D2D1_GAMMATRANSFER_PROP_ALPHA_AMPLITUDE,
    AlphaExponent = D2D1_GAMMATRANSFER_PROP_ALPHA_EXPONENT,
    AlphaOffset = D2D1_GAMMATRANSFER_PROP_ALPHA_OFFSET,
    AlphaDisable = D2D1_GAMMATRANSFER_PROP_ALPHA_DISABLE,
    ClampOutput = D2D1_GAMMATRANSFER_PROP_CLAMP_OUTPUT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GaussianBlurOptimization {
    Speed = D2D1_GAUSSIANBLUR_OPTIMIZATION_SPEED,
    Balanced = D2D1_GAUSSIANBLUR_OPTIMIZATION_BALANCED,
    Quality = D2D1_GAUSSIANBLUR_OPTIMIZATION_QUALITY,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GaussianBlurProp {
    StandardDeviation = D2D1_GAUSSIANBLUR_PROP_STANDARD_DEVIATION,
    Optimization = D2D1_GAUSSIANBLUR_PROP_OPTIMIZATION,
    BorderMode = D2D1_GAUSSIANBLUR_PROP_BORDER_MODE,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
#[allow(non_camel_case_types)]
pub enum GammaConversion {
    None = 0,
    _2_2To1_0 = 1,
    _1_0To2_2 = 2,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GeometryRelation {
    Unknown = D2D1_GEOMETRY_RELATION_UNKNOWN,
    Disjoint = D2D1_GEOMETRY_RELATION_DISJOINT,
    IsContained = D2D1_GEOMETRY_RELATION_IS_CONTAINED,
    Contains = D2D1_GEOMETRY_RELATION_CONTAINS,
    Overlap = D2D1_GEOMETRY_RELATION_OVERLAP,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GeometrySimplificationOption {
    CubicsAndLines = D2D1_GEOMETRY_SIMPLIFICATION_OPTION_CUBICS_AND_LINES,
    Lines = D2D1_GEOMETRY_SIMPLIFICATION_OPTION_LINES,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HDRToneMapDisplayMode {
    SDR = D2D1_HDRTONEMAP_DISPLAY_MODE_SDR,
    HDR = D2D1_HDRTONEMAP_DISPLAY_MODE_HDR,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HDRToneMapProp {
    InputMaxLuminance = D2D1_HDRTONEMAP_PROP_INPUT_MAX_LUMINANCE,
    OutputMaxLuminance = D2D1_HDRTONEMAP_PROP_OUTPUT_MAX_LUMINANCE,
    DisplayMode = D2D1_HDRTONEMAP_PROP_DISPLAY_MODE,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HighlightsAndShadowsInputGamma {
    Linear = D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_LINEAR,
    SRGB = D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_SRGB,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HighlightsAndShadowsProp {
    Highlights = D2D1_HIGHLIGHTSANDSHADOWS_PROP_HIGHLIGHTS,
    Shadows = D2D1_HIGHLIGHTSANDSHADOWS_PROP_SHADOWS,
    Clarity = D2D1_HIGHLIGHTSANDSHADOWS_PROP_CLARITY,
    InputGamma = D2D1_HIGHLIGHTSANDSHADOWS_PROP_INPUT_GAMMA,
    MaskBlurRadius = D2D1_HIGHLIGHTSANDSHADOWS_PROP_MASK_BLUR_RADIUS,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HistogramProp {
    NumBins = D2D1_HISTOGRAM_PROP_NUM_BINS,
    ChannelSelect = D2D1_HISTOGRAM_PROP_CHANNEL_SELECT,
    HistogramOutput = D2D1_HISTOGRAM_PROP_HISTOGRAM_OUTPUT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HueRotationProp {
    Angle = D2D1_HUEROTATION_PROP_ANGLE,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HueToRGBInputColorSpace {
    HueSaturationValue = D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_VALUE,
    HueSaturationLightness = D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HueToRGBProp {
    InputColorSpace = D2D1_HUETORGB_PROP_INPUT_COLOR_SPACE,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ImageSourceFromDXGIOptions {
    None = D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE,
    LowQualityPrimaryConversion = D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ImageSourceLoadingOptions {
    None = D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE,
    ReleaseSource = D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE,
    CacheOnDepend = D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum InkNibShape {
    Round = D2D1_INK_NIB_SHAPE_ROUND,
    Square = D2D1_INK_NIB_SHAPE_SQUARE,
}
*/

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum InterpolationMode {
    NearestNeighbor = D2D1_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_INTERPOLATION_MODE_LINEAR,
    Cubic = D2D1_INTERPOLATION_MODE_CUBIC,
    MultiSampleLinear = D2D1_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_INTERPOLATION_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LayerOptions {
    None = D2D1_LAYER_OPTIONS_NONE,
    InitializeForClearType = D2D1_LAYER_OPTIONS_INITIALIZE_FOR_CLEARTYPE,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LayerOptions1 {
    None = D2D1_LAYER_OPTIONS1_NONE,
    InitializeFromBackground = D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND,
    IgnoreAlpha = D2D1_LAYER_OPTIONS1_IGNORE_ALPHA,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LinearTransferProp {
    RedYIntercept = D2D1_LINEARTRANSFER_PROP_RED_Y_INTERCEPT,
    RedSlope = D2D1_LINEARTRANSFER_PROP_RED_SLOPE,
    RedDisable = D2D1_LINEARTRANSFER_PROP_RED_DISABLE,
    GreenYIntercept = D2D1_LINEARTRANSFER_PROP_GREEN_Y_INTERCEPT,
    GreenSlope = D2D1_LINEARTRANSFER_PROP_GREEN_SLOPE,
    GreenDisable = D2D1_LINEARTRANSFER_PROP_GREEN_DISABLE,
    BlueYIntercept = D2D1_LINEARTRANSFER_PROP_BLUE_Y_INTERCEPT,
    BlueSlope = D2D1_LINEARTRANSFER_PROP_BLUE_SLOPE,
    BlueDisable = D2D1_LINEARTRANSFER_PROP_BLUE_DISABLE,
    AlphaYIntercept = D2D1_LINEARTRANSFER_PROP_ALPHA_Y_INTERCEPT,
    AlphaSlope = D2D1_LINEARTRANSFER_PROP_ALPHA_SLOPE,
    AlphaDisable = D2D1_LINEARTRANSFER_PROP_ALPHA_DISABLE,
    ClampOutput = D2D1_LINEARTRANSFER_PROP_CLAMP_OUTPUT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LineJoin {
    Miter = D2D1_LINE_JOIN_MITER,
    Bevel = D2D1_LINE_JOIN_BEVEL,
    Round = D2D1_LINE_JOIN_ROUND,
    MiterOrBevel = D2D1_LINE_JOIN_MITER_OR_BEVEL,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LookupTable3DProp {
    LUT = D2D1_LOOKUPTABLE3D_PROP_LUT,
    AlphaMode = D2D1_LOOKUPTABLE3D_PROP_ALPHA_MODE,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum MorphologyMode {
    Erode = D2D1_MORPHOLOGY_MODE_ERODE,
    Dilate = D2D1_MORPHOLOGY_MODE_DILATE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum MorphologyProp {
    Mode = D2D1_MORPHOLOGY_PROP_MODE,
    Width = D2D1_MORPHOLOGY_PROP_WIDTH,
    Height = D2D1_MORPHOLOGY_PROP_HEIGHT,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum MapOptions {
    None = D2D1_MAP_OPTIONS_NONE,
    Read = D2D1_MAP_OPTIONS_READ,
    Write = D2D1_MAP_OPTIONS_WRITE,
    Discard = D2D1_MAP_OPTIONS_DISCARD,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum OpacityMetadataProp {
    InputOpaqueRect = D2D1_OPACITYMETADATA_PROP_INPUT_OPAQUE_RECT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum OpacityMaskContent {
    Graphics = D2D1_OPACITY_MASK_CONTENT_GRAPHICS,
    Natural = D2D1_OPACITY_MASK_CONTENT_TEXT_NATURAL,
    GDICompatible = D2D1_OPACITY_MASK_CONTENT_TEXT_GDI_COMPATIBLE,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Orientation {
    Default = D2D1_ORIENTATION_DEFAULT,
    FlipHorizontal = D2D1_ORIENTATION_FLIP_HORIZONTAL,
    RotateClockwise180 = D2D1_ORIENTATION_ROTATE_CLOCKWISE180,
    RotateClockwise180FlipHorizontal = D2D1_ORIENTATION_ROTATE_CLOCKWISE180_FLIP_HORIZONTAL,
    RotateClockwise90FlipHorizontal = D2D1_ORIENTATION_ROTATE_CLOCKWISE90_FLIP_HORIZONTAL,
    RotateClockwise270 = D2D1_ORIENTATION_ROTATE_CLOCKWISE270,
    RotateClockwise270FlipHorizontal = D2D1_ORIENTATION_ROTATE_CLOCKWISE270_FLIP_HORIZONTAL,
    RotateClockwise90 = D2D1_ORIENTATION_ROTATE_CLOCKWISE90,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PatchEdgeMode {
    Aliased = D2D1_PATCH_EDGE_MODE_ALIASED,
    Antialiased = D2D1_PATCH_EDGE_MODE_ANTIALIASED,
    AliasedInflated = D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PathSegment {
    None = D2D1_PATH_SEGMENT_NONE,
    ForceUnstroked = D2D1_PATH_SEGMENT_FORCE_UNSTROKED,
    RoundLineJoin = D2D1_PATH_SEGMENT_FORCE_ROUND_LINE_JOIN,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PixelOptions {
    None = D2D1_PIXEL_OPTIONS_NONE,
    TrivialSampling = D2D1_PIXEL_OPTIONS_TRIVIAL_SAMPLING,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PointDiffuseProp {
    LightPosition = D2D1_POINTDIFFUSE_PROP_LIGHT_POSITION,
    DiffuseConstant = D2D1_POINTDIFFUSE_PROP_DIFFUSE_CONSTANT,
    SurfaceScale = D2D1_POINTDIFFUSE_PROP_SURFACE_SCALE,
    Color = D2D1_POINTDIFFUSE_PROP_COLOR,
    KernelUnitLength = D2D1_POINTDIFFUSE_PROP_KERNEL_UNIT_LENGTH,
    ScaleMode = D2D1_POINTDIFFUSE_PROP_SCALE_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PointDiffuseScaleMode {
    NearestNeighbor = D2D1_POINTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_POINTDIFFUSE_SCALE_MODE_LINEAR,
    Cubic = D2D1_POINTDIFFUSE_SCALE_MODE_CUBIC,
    MultiSampleLinear = D2D1_POINTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_POINTDIFFUSE_SCALE_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_POINTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PointSpecularProp {
    LightPosition = D2D1_POINTSPECULAR_PROP_LIGHT_POSITION,
    SpecularExponent = D2D1_POINTSPECULAR_PROP_SPECULAR_EXPONENT,
    SpecularConstant = D2D1_POINTSPECULAR_PROP_SPECULAR_CONSTANT,
    SurfaceScale = D2D1_POINTSPECULAR_PROP_SURFACE_SCALE,
    Color = D2D1_POINTSPECULAR_PROP_COLOR,
    KernelUnitLength = D2D1_POINTSPECULAR_PROP_KERNEL_UNIT_LENGTH,
    ScaleMode = D2D1_POINTSPECULAR_PROP_SCALE_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PointSpecularScaleMode {
    NearestNeighbor = D2D1_POINTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_POINTSPECULAR_SCALE_MODE_LINEAR,
    Cubic = D2D1_POINTSPECULAR_SCALE_MODE_CUBIC,
    MultiSampleLinear = D2D1_POINTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_POINTSPECULAR_SCALE_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_POINTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PosterizeProp {
    RedValueCount = D2D1_POSTERIZE_PROP_RED_VALUE_COUNT,
    GreenValueCount = D2D1_POSTERIZE_PROP_GREEN_VALUE_COUNT,
    BlueValueCount = D2D1_POSTERIZE_PROP_BLUE_VALUE_COUNT,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PresentOptions {
    None = D2D1_PRESENT_OPTIONS_NONE,
    RetainContents = D2D1_PRESENT_OPTIONS_RETAIN_CONTENTS,
    Immediately = D2D1_PRESENT_OPTIONS_IMMEDIATELY,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PrimitiveBlend {
    SourceOver = D2D1_PRIMITIVE_BLEND_SOURCE_OVER,
    Copy = D2D1_PRIMITIVE_BLEND_COPY,
    Min = D2D1_PRIMITIVE_BLEND_MIN,
    Add = D2D1_PRIMITIVE_BLEND_ADD,
    // Max = D2D1_PRIMITIVE_BLEND_MAX,
    Max = 4,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PrintFontSubsetMode {
    Default = D2D1_PRINT_FONT_SUBSET_MODE_DEFAULT,
    EachPage = D2D1_PRINT_FONT_SUBSET_MODE_EACHPAGE,
    None = D2D1_PRINT_FONT_SUBSET_MODE_NONE,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PropertyType {
    Unknown = D2D1_PROPERTY_TYPE_UNKNOWN,
    String = D2D1_PROPERTY_TYPE_STRING,
    Bool = D2D1_PROPERTY_TYPE_BOOL,
    Uint32 = D2D1_PROPERTY_TYPE_UINT32,
    Int32 = D2D1_PROPERTY_TYPE_INT32,
    Float = D2D1_PROPERTY_TYPE_FLOAT,
    Vector2 = D2D1_PROPERTY_TYPE_VECTOR2,
    Vector3 = D2D1_PROPERTY_TYPE_VECTOR3,
    Vector4 = D2D1_PROPERTY_TYPE_VECTOR4,
    Blob = D2D1_PROPERTY_TYPE_BLOB,
    IUnknown = D2D1_PROPERTY_TYPE_IUNKNOWN,
    Enum = D2D1_PROPERTY_TYPE_ENUM,
    Array = D2D1_PROPERTY_TYPE_ARRAY,
    CLSID = D2D1_PROPERTY_TYPE_CLSID,
    Matrix3x2 = D2D1_PROPERTY_TYPE_MATRIX_3X2,
    Matrix4x3 = D2D1_PROPERTY_TYPE_MATRIX_4X3,
    Matrix4x4 = D2D1_PROPERTY_TYPE_MATRIX_4X4,
    Matrix5x4 = D2D1_PROPERTY_TYPE_MATRIX_5X4,
    ColorContext = D2D1_PROPERTY_TYPE_COLOR_CONTEXT,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Property {
    CLSID = D2D1_PROPERTY_CLSID,
    DisplayName = D2D1_PROPERTY_DISPLAYNAME,
    Author = D2D1_PROPERTY_AUTHOR,
    Category = D2D1_PROPERTY_CATEGORY,
    Description = D2D1_PROPERTY_DESCRIPTION,
    Inputs = D2D1_PROPERTY_INPUTS,
    Cached = D2D1_PROPERTY_CACHED,
    Precision = D2D1_PROPERTY_PRECISION,
    MinInputs = D2D1_PROPERTY_MIN_INPUTS,
    MaxInputs = D2D1_PROPERTY_MAX_INPUTS,
}

#[cfg(feature = "d2d1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RenderingPriority {
    Normal = D2D1_RENDERING_PRIORITY_NORMAL,
    Low = D2D1_RENDERING_PRIORITY_LOW,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RenderTargetType {
    Default = D2D1_RENDER_TARGET_TYPE_DEFAULT,
    Software = D2D1_RENDER_TARGET_TYPE_SOFTWARE,
    Hardware = D2D1_RENDER_TARGET_TYPE_HARDWARE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RenderTargetUsage {
    None = D2D1_RENDER_TARGET_USAGE_NONE,
    ForceBitmapRemoting = D2D1_RENDER_TARGET_USAGE_FORCE_BITMAP_REMOTING,
    GDICompatible = D2D1_RENDER_TARGET_USAGE_GDI_COMPATIBLE,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ResourceType {
    // None = D2D1_RESOURCE_TYPE_NONE,
    // Shader = D2D1_RESOURCE_TYPE_SHADER,
    // Buffer = D2D1_RESOURCE_TYPE_BUFFER,
    None = 0,
    Shader = 1,
    Buffer = 2,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RGBToHueOutputColorSpace {
    HueSaturationValue = D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_VALUE,
    HueSaturationLightness = D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RGBToHueProp {
    OutputColorSpace = D2D1_RGBTOHUE_PROP_OUTPUT_COLOR_SPACE,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SaturationProp {
    Saturation = D2D1_SATURATION_PROP_SATURATION,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ScaleInterpolationMode {
    NearestNeighbor = D2D1_SCALE_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_SCALE_INTERPOLATION_MODE_LINEAR,
    Cubic = D2D1_SCALE_INTERPOLATION_MODE_CUBIC,
    MultiSampleLinear = D2D1_SCALE_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_SCALE_INTERPOLATION_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_SCALE_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ScaleProp {
    Scale = D2D1_SCALE_PROP_SCALE,
    CenterPoint = D2D1_SCALE_PROP_CENTER_POINT,
    InterpolationMode = D2D1_SCALE_PROP_INTERPOLATION_MODE,
    BorderMode = D2D1_SCALE_PROP_BORDER_MODE,
    Sharpness = D2D1_SCALE_PROP_SHARPNESS,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SepiaProp {
    Intensity = D2D1_SEPIA_PROP_INTENSITY,
    AlphaMode = D2D1_SEPIA_PROP_ALPHA_MODE,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ShadowOptimization {
    Speed = D2D1_SHADOW_OPTIMIZATION_SPEED,
    Balanced = D2D1_SHADOW_OPTIMIZATION_BALANCED,
    Quality = D2D1_SHADOW_OPTIMIZATION_QUALITY,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ShadowProp {
    BlurStandardDeviation = D2D1_SHADOW_PROP_BLUR_STANDARD_DEVIATION,
    Color = D2D1_SHADOW_PROP_COLOR,
    Optimization = D2D1_SHADOW_PROP_OPTIMIZATION,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Sharpen_prop {
    Sharpness = D2D1_SHARPEN_PROP_SHARPNESS,
    Threashold = D2D1_SHARPEN_PROP_THRESHOLD,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SpotDiffuseProp {
    LightPosition = D2D1_SPOTDIFFUSE_PROP_LIGHT_POSITION,
    PointsAt = D2D1_SPOTDIFFUSE_PROP_POINTS_AT,
    Focus = D2D1_SPOTDIFFUSE_PROP_FOCUS,
    LimitingConeAngle = D2D1_SPOTDIFFUSE_PROP_LIMITING_CONE_ANGLE,
    DiffuseConstant = D2D1_SPOTDIFFUSE_PROP_DIFFUSE_CONSTANT,
    SurfaceScale = D2D1_SPOTDIFFUSE_PROP_SURFACE_SCALE,
    Color = D2D1_SPOTDIFFUSE_PROP_COLOR,
    KernelUnitLength = D2D1_SPOTDIFFUSE_PROP_KERNEL_UNIT_LENGTH,
    ScaleMode = D2D1_SPOTDIFFUSE_PROP_SCALE_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SpotDiffuseScaleMode {
    NearestNeighbor = D2D1_SPOTDIFFUSE_SCALE_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_SPOTDIFFUSE_SCALE_MODE_LINEAR,
    Cubic = D2D1_SPOTDIFFUSE_SCALE_MODE_CUBIC,
    MultiSampleLinear = D2D1_SPOTDIFFUSE_SCALE_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_SPOTDIFFUSE_SCALE_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_SPOTDIFFUSE_SCALE_MODE_HIGH_QUALITY_CUBIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SpotSpecularProp {
    LightPosition = D2D1_SPOTSPECULAR_PROP_LIGHT_POSITION,
    PointsAt = D2D1_SPOTSPECULAR_PROP_POINTS_AT,
    Focus = D2D1_SPOTSPECULAR_PROP_FOCUS,
    LimitingConeAngle = D2D1_SPOTSPECULAR_PROP_LIMITING_CONE_ANGLE,
    SpecularExponent = D2D1_SPOTSPECULAR_PROP_SPECULAR_EXPONENT,
    SpecularConstant = D2D1_SPOTSPECULAR_PROP_SPECULAR_CONSTANT,
    SurfaceScale = D2D1_SPOTSPECULAR_PROP_SURFACE_SCALE,
    Color = D2D1_SPOTSPECULAR_PROP_COLOR,
    KernelUintLength = D2D1_SPOTSPECULAR_PROP_KERNEL_UNIT_LENGTH,
    ScaleMode = D2D1_SPOTSPECULAR_PROP_SCALE_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SpotSpecularScaleMode {
    NearestNeighbor = D2D1_SPOTSPECULAR_SCALE_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_SPOTSPECULAR_SCALE_MODE_LINEAR,
    Cubic = D2D1_SPOTSPECULAR_SCALE_MODE_CUBIC,
    MultiSampleLinear = D2D1_SPOTSPECULAR_SCALE_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_SPOTSPECULAR_SCALE_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_SPOTSPECULAR_SCALE_MODE_HIGH_QUALITY_CUBIC,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SpriteOptions {
    None = D2D1_SPRITE_OPTIONS_NONE,
    ClampToSourceRectangle = D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StraightenProp {
    Angle = D2D1_STRAIGHTEN_PROP_ANGLE,
    MaintainSize = D2D1_STRAIGHTEN_PROP_MAINTAIN_SIZE,
    ScaleMode = D2D1_STRAIGHTEN_PROP_SCALE_MODE,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StraightenScaleMode {
    NearestNeighbor = D2D1_STRAIGHTEN_SCALE_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_STRAIGHTEN_SCALE_MODE_LINEAR,
    Cubic = D2D1_STRAIGHTEN_SCALE_MODE_CUBIC,
    MultiSampleLinear = D2D1_STRAIGHTEN_SCALE_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_STRAIGHTEN_SCALE_MODE_ANISOTROPIC,
}
*/

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StrokeTransformType {
    Normal = D2D1_STROKE_TRANSFORM_TYPE_NORMAL,
    Fiexed = D2D1_STROKE_TRANSFORM_TYPE_FIXED,
    HairLine = D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SubProperty {
    DisplayName = D2D1_SUBPROPERTY_DISPLAYNAME,
    IsReadOnly = D2D1_SUBPROPERTY_ISREADONLY,
    Min = D2D1_SUBPROPERTY_MIN,
    Max = D2D1_SUBPROPERTY_MAX,
    Default = D2D1_SUBPROPERTY_DEFAULT,
    Fields = D2D1_SUBPROPERTY_FIELDS,
    Index = D2D1_SUBPROPERTY_INDEX,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGAspectAlign {
    None = D2D1_SVG_ASPECT_ALIGN_NONE,
    XMinYMin = D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MIN,
    XMidYMin = D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MIN,
    XMaxYMin = D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MIN,
    XMinYMid = D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MID,
    XMidYMid = D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MID,
    XMaxYMid = D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MID,
    XMinYMax = D2D1_SVG_ASPECT_ALIGN_X_MIN_Y_MAX,
    XMidYMax = D2D1_SVG_ASPECT_ALIGN_X_MID_Y_MAX,
    XMaxYMax = D2D1_SVG_ASPECT_ALIGN_X_MAX_Y_MAX,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGAspectScaling {
    Meet = D2D1_SVG_ASPECT_SCALING_MEET,
    Slice = D2D1_SVG_ASPECT_SCALING_SLICE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGAttributePODType {
    Float = D2D1_SVG_ATTRIBUTE_POD_TYPE_FLOAT,
    Color = D2D1_SVG_ATTRIBUTE_POD_TYPE_COLOR,
    FillMode = D2D1_SVG_ATTRIBUTE_POD_TYPE_FILL_MODE,
    Display = D2D1_SVG_ATTRIBUTE_POD_TYPE_DISPLAY,
    Overflow = D2D1_SVG_ATTRIBUTE_POD_TYPE_OVERFLOW,
    LineCap = D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_CAP,
    LineJoin = D2D1_SVG_ATTRIBUTE_POD_TYPE_LINE_JOIN,
    Visibility = D2D1_SVG_ATTRIBUTE_POD_TYPE_VISIBILITY,
    Matrix = D2D1_SVG_ATTRIBUTE_POD_TYPE_MATRIX,
    UnitType = D2D1_SVG_ATTRIBUTE_POD_TYPE_UNIT_TYPE,
    ExtendMode = D2D1_SVG_ATTRIBUTE_POD_TYPE_EXTEND_MODE,
    PreserveAspectRatio = D2D1_SVG_ATTRIBUTE_POD_TYPE_PRESERVE_ASPECT_RATIO,
    ViewBox = D2D1_SVG_ATTRIBUTE_POD_TYPE_VIEWBOX,
    Length = D2D1_SVG_ATTRIBUTE_POD_TYPE_LENGTH,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGAttributeStringType {
    SVG = D2D1_SVG_ATTRIBUTE_STRING_TYPE_SVG,
    ID = D2D1_SVG_ATTRIBUTE_STRING_TYPE_ID,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGDisplay {
    Inline = D2D1_SVG_DISPLAY_INLINE,
    None = D2D1_SVG_DISPLAY_NONE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGLengthUnits {
    Number = D2D1_SVG_LENGTH_UNITS_NUMBER,
    Percentage = D2D1_SVG_LENGTH_UNITS_PERCENTAGE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGLineCap {
    Butt = D2D1_SVG_LINE_CAP_BUTT,
    Square = D2D1_SVG_LINE_CAP_SQUARE,
    Round = D2D1_SVG_LINE_CAP_ROUND,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGLineJoin {
    Bevel = D2D1_SVG_LINE_JOIN_BEVEL,
    Miter = D2D1_SVG_LINE_JOIN_MITER,
    Round = D2D1_SVG_LINE_JOIN_ROUND,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGOverflow {
    Visible = D2D1_SVG_OVERFLOW_VISIBLE,
    Hidden = D2D1_SVG_OVERFLOW_HIDDEN,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGPaintType {
    None = D2D1_SVG_PAINT_TYPE_NONE,
    Color = D2D1_SVG_PAINT_TYPE_COLOR,
    CurrentColor = D2D1_SVG_PAINT_TYPE_CURRENT_COLOR,
    URI = D2D1_SVG_PAINT_TYPE_URI,
    URINone = D2D1_SVG_PAINT_TYPE_URI_NONE,
    URIColor = D2D1_SVG_PAINT_TYPE_URI_COLOR,
    URICurrentColor = D2D1_SVG_PAINT_TYPE_URI_CURRENT_COLOR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGPathCommand {
    ClosePath = D2D1_SVG_PATH_COMMAND_CLOSE_PATH,
    MoveAbsolute = D2D1_SVG_PATH_COMMAND_MOVE_ABSOLUTE,
    MoveRelative = D2D1_SVG_PATH_COMMAND_MOVE_RELATIVE,
    LineAbsolute = D2D1_SVG_PATH_COMMAND_LINE_ABSOLUTE,
    LineRelative = D2D1_SVG_PATH_COMMAND_LINE_RELATIVE,
    CubicAbsolute = D2D1_SVG_PATH_COMMAND_CUBIC_ABSOLUTE,
    CubicRelative = D2D1_SVG_PATH_COMMAND_CUBIC_RELATIVE,
    QuadradicAbsolute = D2D1_SVG_PATH_COMMAND_QUADRADIC_ABSOLUTE,
    QuadradicRelative = D2D1_SVG_PATH_COMMAND_QUADRADIC_RELATIVE,
    ArcAbsolute = D2D1_SVG_PATH_COMMAND_ARC_ABSOLUTE,
    ArcRelative = D2D1_SVG_PATH_COMMAND_ARC_RELATIVE,
    HorizontalAbsolute = D2D1_SVG_PATH_COMMAND_HORIZONTAL_ABSOLUTE,
    HorizontalRelative = D2D1_SVG_PATH_COMMAND_HORIZONTAL_RELATIVE,
    VerticalAbsolute = D2D1_SVG_PATH_COMMAND_VERTICAL_ABSOLUTE,
    VerticalRelative = D2D1_SVG_PATH_COMMAND_VERTICAL_RELATIVE,
    CubicSmoothAbsolute = D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_ABSOLUTE,
    CubicSmoothRelative = D2D1_SVG_PATH_COMMAND_CUBIC_SMOOTH_RELATIVE,
    QuadradicSmoothAbsolute = D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_ABSOLUTE,
    QuadradicSmoothRelative = D2D1_SVG_PATH_COMMAND_QUADRADIC_SMOOTH_RELATIVE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGUnitType {
    UserSpaceOnUse = D2D1_SVG_UNIT_TYPE_USER_SPACE_ON_USE,
    ObjectBoundingBox = D2D1_SVG_UNIT_TYPE_OBJECT_BOUNDING_BOX,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SVGVisibility {
    Visible = D2D1_SVG_VISIBILITY_VISIBLE,
    Hidden = D2D1_SVG_VISIBILITY_HIDDEN,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SweepDirection {
    CounterClockwise = D2D1_SWEEP_DIRECTION_COUNTER_CLOCKWISE,
    Clockwise = D2D1_SWEEP_DIRECTION_CLOCKWISE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TableTransferProp {
    RedTable = D2D1_TABLETRANSFER_PROP_RED_TABLE,
    RedDisable = D2D1_TABLETRANSFER_PROP_RED_DISABLE,
    GreenTable = D2D1_TABLETRANSFER_PROP_GREEN_TABLE,
    GreenDisable = D2D1_TABLETRANSFER_PROP_GREEN_DISABLE,
    BlueTable = D2D1_TABLETRANSFER_PROP_BLUE_TABLE,
    BlueDisable = D2D1_TABLETRANSFER_PROP_BLUE_DISABLE,
    AlphaTable = D2D1_TABLETRANSFER_PROP_ALPHA_TABLE,
    AlphaDisable = D2D1_TABLETRANSFER_PROP_ALPHA_DISABLE,
    ClampOutput = D2D1_TABLETRANSFER_PROP_CLAMP_OUTPUT,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TemperatureAndTintProp {
    Temperature = D2D1_TEMPERATUREANDTINT_PROP_TEMPERATURE,
    Tint = D2D1_TEMPERATUREANDTINT_PROP_TINT,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextAntialiasMode {
    Default = D2D1_TEXT_ANTIALIAS_MODE_DEFAULT,
    ClearType = D2D1_TEXT_ANTIALIAS_MODE_CLEARTYPE,
    GrayScale = D2D1_TEXT_ANTIALIAS_MODE_GRAYSCALE,
    Aliased = D2D1_TEXT_ANTIALIAS_MODE_ALIASED,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ThreadingMode {
    SingleThreaded = D2D1_THREADING_MODE_SINGLE_THREADED,
    MultiThreaded = D2D1_THREADING_MODE_MULTI_THREADED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TileProp {
    Rect = D2D1_TILE_PROP_RECT,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TransformedImageSourceOptions {
    None = D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE,
    DisableDPIScale = D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TurbulenceNoise {
    FractalSum = D2D1_TURBULENCE_NOISE_FRACTAL_SUM,
    Turbulence = D2D1_TURBULENCE_NOISE_TURBULENCE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TurbulenceProp {
    Offset = D2D1_TURBULENCE_PROP_OFFSET,
    Size = D2D1_TURBULENCE_PROP_SIZE,
    BaseFrequency = D2D1_TURBULENCE_PROP_BASE_FREQUENCY,
    NumOctaves = D2D1_TURBULENCE_PROP_NUM_OCTAVES,
    Seed = D2D1_TURBULENCE_PROP_SEED,
    Noise = D2D1_TURBULENCE_PROP_NOISE,
    Stitchable = D2D1_TURBULENCE_PROP_STITCHABLE,
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum UnitMode {
    DIPs = D2D1_UNIT_MODE_DIPS,
    Pixels = D2D1_UNIT_MODE_PIXELS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VertexOptions {
    None = D2D1_VERTEX_OPTIONS_NONE,
    DoNotClear = D2D1_VERTEX_OPTIONS_DO_NOT_CLEAR,
    UseDepthBuffer = D2D1_VERTEX_OPTIONS_USE_DEPTH_BUFFER,
    AssumeNoOverlap = D2D1_VERTEX_OPTIONS_ASSUME_NO_OVERLAP,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VertexUsage {
    Static = D2D1_VERTEX_USAGE_STATIC,
    Dynamic = D2D1_VERTEX_USAGE_DYNAMIC,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VignetteProp {
    Color = D2D1_VIGNETTE_PROP_COLOR,
    TransitionSize = D2D1_VIGNETTE_PROP_TRANSITION_SIZE,
    Strength = D2D1_VIGNETTE_PROP_STRENGTH,
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum WhiteLevelAdjustmentProp {
    InputWhiteLevel = D2D1_WHITELEVELADJUSTMENT_PROP_INPUT_WHITE_LEVEL,
    OutputWhiteLevel = D2D1_WHITELEVELADJUSTMENT_PROP_OUTPUT_WHITE_LEVEL,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum WindowState {
    None = D2D1_WINDOW_STATE_NONE,
    Occluded = D2D1_WINDOW_STATE_OCCLUDED,
}

#[cfg(feature = "d2d1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum YCbCrChromaSubsampling {
    Auto = D2D1_YCBCR_CHROMA_SUBSAMPLING_AUTO,
    _420 = D2D1_YCBCR_CHROMA_SUBSAMPLING_420,
    _422 = D2D1_YCBCR_CHROMA_SUBSAMPLING_422,
    _444 = D2D1_YCBCR_CHROMA_SUBSAMPLING_444,
    _440 = D2D1_YCBCR_CHROMA_SUBSAMPLING_440,
}

#[cfg(feature = "d2d1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum YCbCrInterpolationMode {
    NearestNeighbor = D2D1_YCBCR_INTERPOLATION_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_YCBCR_INTERPOLATION_MODE_LINEAR,
    Cubic = D2D1_YCBCR_INTERPOLATION_MODE_CUBIC,
    MultiSampleLinear = D2D1_YCBCR_INTERPOLATION_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_YCBCR_INTERPOLATION_MODE_ANISOTROPIC,
    HighQualityCubic = D2D1_YCBCR_INTERPOLATION_MODE_HIGH_QUALITY_CUBIC,
}

#[cfg(feature = "d2d1_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum YCbCrProp {
    ChromaSubsampling = D2D1_YCBCR_PROP_CHROMA_SUBSAMPLING,
    TransformMatrix = D2D1_YCBCR_PROP_TRANSFORM_MATRIX,
    InterpolationMode = D2D1_YCBCR_PROP_INTERPOLATION_MODE,
}

pub type ColorF = crate::dxgitype::RGBA;

macro_rules! impl_matrix {
    ($name: ident, $r: expr, $c: expr) => {
        impl $name {
            pub fn new() -> Self {
                Self { m: [0.0; $r * $c] }
            }
        }
        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
        impl std::ops::Index<(u32, u32)> for $name {
            type Output = f32;

            fn index(&self, idx: (u32, u32)) -> &f32 {
                &self.m[(idx.0 * $c + idx.1) as usize]
            }
        }
        impl std::ops::IndexMut<(u32, u32)> for $name {
            fn index_mut(&mut self, idx: (u32, u32)) -> &mut f32 {
                &mut self.m[(idx.0 * $c + idx.1) as usize]
            }
        }
    };
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Matrix3x2F {
    m: [f32; 3 * 2],
}
impl_matrix!(Matrix3x2F, 3, 2);
impl Matrix3x2F {
    fn as_c_ptr(&self) -> *const D2D_MATRIX_3X2_F {
        self.m.as_ptr() as *const D2D_MATRIX_3X2_F
    }
}
impl From<Matrix3x2F> for D2D_MATRIX_3X2_F {
    fn from(src: Matrix3x2F) -> D2D_MATRIX_3X2_F {
        Self {
            matrix: [
                [src.m[0], src.m[1]],
                [src.m[2], src.m[3]],
                [src.m[4], src.m[5]],
            ],
        }
    }
}
impl From<D2D_MATRIX_3X2_F> for Matrix3x2F {
    fn from(src: D2D_MATRIX_3X2_F) -> Matrix3x2F {
        Self {
            m: [
                src.matrix[0][0],
                src.matrix[0][1],
                src.matrix[1][0],
                src.matrix[1][1],
                src.matrix[2][0],
                src.matrix[2][1],
            ],
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Matrix4x3F {
    m: [f32; 4 * 3],
}
impl_matrix!(Matrix4x3F, 4, 3);
impl From<Matrix4x3F> for D2D_MATRIX_4X3_F {
    fn from(src: Matrix4x3F) -> D2D_MATRIX_4X3_F {
        Self {
            matrix: [
                [src.m[0], src.m[1], src.m[2]],
                [src.m[3], src.m[4], src.m[5]],
                [src.m[6], src.m[7], src.m[8]],
                [src.m[9], src.m[10], src.m[11]],
            ],
        }
    }
}
impl From<D2D_MATRIX_4X3_F> for Matrix4x3F {
    fn from(src: D2D_MATRIX_4X3_F) -> Matrix4x3F {
        Self {
            m: [
                src.matrix[0][0],
                src.matrix[0][1],
                src.matrix[0][2],
                src.matrix[1][0],
                src.matrix[1][1],
                src.matrix[1][2],
                src.matrix[2][0],
                src.matrix[2][1],
                src.matrix[2][2],
                src.matrix[3][0],
                src.matrix[3][1],
                src.matrix[3][2],
            ],
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Matrix4x4F {
    m: [f32; 4 * 4],
}
impl_matrix!(Matrix4x4F, 4, 4);
impl From<Matrix4x4F> for D2D_MATRIX_4X4_F {
    fn from(src: Matrix4x4F) -> D2D_MATRIX_4X4_F {
        Self {
            matrix: [
                [src.m[0], src.m[1], src.m[2], src.m[3]],
                [src.m[4], src.m[5], src.m[6], src.m[7]],
                [src.m[8], src.m[9], src.m[10], src.m[11]],
                [src.m[12], src.m[13], src.m[14], src.m[15]],
            ],
        }
    }
}
impl From<D2D_MATRIX_4X4_F> for Matrix4x4F {
    fn from(src: D2D_MATRIX_4X4_F) -> Matrix4x4F {
        Self {
            m: [
                src.matrix[0][0],
                src.matrix[0][1],
                src.matrix[0][2],
                src.matrix[0][3],
                src.matrix[1][0],
                src.matrix[1][1],
                src.matrix[1][2],
                src.matrix[1][3],
                src.matrix[2][0],
                src.matrix[2][1],
                src.matrix[2][2],
                src.matrix[2][3],
                src.matrix[3][0],
                src.matrix[3][1],
                src.matrix[3][2],
                src.matrix[3][3],
            ],
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Matrix5x4F {
    m: [f32; 5 * 4],
}
impl_matrix!(Matrix5x4F, 5, 4);
impl From<Matrix5x4F> for D2D_MATRIX_5X4_F {
    fn from(src: Matrix5x4F) -> D2D_MATRIX_5X4_F {
        Self {
            matrix: [
                [src.m[0], src.m[1], src.m[2], src.m[3]],
                [src.m[4], src.m[5], src.m[6], src.m[7]],
                [src.m[8], src.m[9], src.m[10], src.m[11]],
                [src.m[12], src.m[13], src.m[14], src.m[15]],
                [src.m[16], src.m[17], src.m[18], src.m[19]],
            ],
        }
    }
}
impl From<D2D_MATRIX_5X4_F> for Matrix5x4F {
    fn from(src: D2D_MATRIX_5X4_F) -> Matrix5x4F {
        Self {
            m: [
                src.matrix[0][0],
                src.matrix[0][1],
                src.matrix[0][2],
                src.matrix[0][3],
                src.matrix[1][0],
                src.matrix[1][1],
                src.matrix[1][2],
                src.matrix[1][3],
                src.matrix[2][0],
                src.matrix[2][1],
                src.matrix[2][2],
                src.matrix[2][3],
                src.matrix[3][0],
                src.matrix[3][1],
                src.matrix[3][2],
                src.matrix[3][3],
                src.matrix[4][0],
                src.matrix[4][1],
                src.matrix[4][2],
                src.matrix[4][3],
            ],
        }
    }
}

pub type Point2F = crate::api::Point<f32>;

impl From<Point2F> for D2D_POINT_2F {
    fn from(src: Point2F) -> D2D_POINT_2F {
        D2D_POINT_2F { x: src.x, y: src.y }
    }
}
impl From<D2D_POINT_2F> for Point2F {
    fn from(src: D2D_POINT_2F) -> Point2F {
        Point2F { x: src.x, y: src.y }
    }
}
impl AsRef<D2D_POINT_2F> for Point2F {
    fn as_ref(&self) -> &D2D_POINT_2F {
        unsafe { (self as *const Point2F as *const D2D_POINT_2F).as_ref().unwrap() }
    }
}

pub type Point2L = crate::api::Point<i32>;

pub type Point2U = crate::api::Point<u32>;

impl From<Point2U> for D2D_POINT_2U {
    fn from(src: Point2U) -> D2D_POINT_2U {
        D2D_POINT_2U { x: src.x, y: src.y }
    }
}
impl From<D2D_POINT_2U> for Point2U {
    fn from(src: D2D_POINT_2U) -> Point2U {
        Point2U { x: src.x, y: src.y }
    }
}
impl AsRef<D2D_POINT_2U> for Point2U {
    fn as_ref(&self) -> &D2D_POINT_2U {
        unsafe { (self as *const Point2U as *const D2D_POINT_2U).as_ref().unwrap() }
    }
}

pub type RectF = crate::api::Rect<f32>;

impl From<RectF> for D2D_RECT_F {
    fn from(src: RectF) -> D2D_RECT_F {
        D2D_RECT_F {
            left: src.left,
            top: src.top,
            right: src.right,
            bottom: src.bottom,
        }
    }
}
impl From<D2D_RECT_F> for RectF {
    fn from(src: D2D_RECT_F) -> RectF {
        RectF {
            left: src.left,
            top: src.top,
            right: src.right,
            bottom: src.bottom,
        }
    }
}
impl AsRef<D2D_RECT_F> for RectF {
    fn as_ref(&self) -> &D2D_RECT_F {
        unsafe { (self as *const RectF as *const D2D_RECT_F).as_ref().unwrap() }
    }
}

pub type RectL = crate::api::Rect<i32>;

pub type RectU = crate::api::Rect<u32>;

impl From<RectU> for D2D_RECT_U {
    fn from(src: RectU) -> D2D_RECT_U {
        D2D_RECT_U {
            left: src.left,
            top: src.top,
            right: src.right,
            bottom: src.bottom,
        }
    }
}
impl From<D2D_RECT_U> for RectU {
    fn from(src: D2D_RECT_U) -> RectU {
        RectU {
            left: src.left,
            top: src.top,
            right: src.right,
            bottom: src.bottom,
        }
    }
}
impl AsRef<D2D_RECT_U> for RectU {
    fn as_ref(&self) -> &D2D_RECT_U {
        unsafe { (self as *const RectU as *const D2D_RECT_U).as_ref().unwrap() }
    }
}

pub type SizeF = crate::api::Size<f32>;

impl From<SizeF> for D2D_SIZE_F {
    fn from(src: SizeF) -> D2D_SIZE_F {
        D2D_SIZE_F {
            width: src.width,
            height: src.height,
        }
    }
}
impl From<D2D_SIZE_F> for SizeF {
    fn from(src: D2D_SIZE_F) -> SizeF {
        SizeF {
            width: src.width,
            height: src.height,
        }
    }
}
impl AsRef<D2D_SIZE_F> for SizeF {
    fn as_ref(&self) -> &D2D_SIZE_F {
        unsafe { (self as *const SizeF as *const D2D_SIZE_F).as_ref().unwrap() }
    }
}

pub type SizeU = crate::api::Size<u32>;

impl From<SizeU> for D2D_SIZE_U {
    fn from(src: SizeU) -> D2D_SIZE_U {
        D2D_SIZE_U {
            width: src.width,
            height: src.height,
        }
    }
}
impl From<D2D_SIZE_U> for SizeU {
    fn from(src: D2D_SIZE_U) -> SizeU {
        SizeU {
            width: src.width,
            height: src.height,
        }
    }
}
impl AsRef<D2D_SIZE_U> for SizeU {
    fn as_ref(&self) -> &D2D_SIZE_U {
        unsafe { (self as *const SizeU as *const D2D_SIZE_U).as_ref().unwrap() }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector2F {
    pub x: f32,
    pub y: f32,
}
impl Vector2F {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}
impl From<Vector2F> for D2D_VECTOR_2F {
    fn from(src: Vector2F) -> D2D_VECTOR_2F {
        D2D_VECTOR_2F { x: src.x, y: src.y }
    }
}
impl From<D2D_VECTOR_2F> for Vector2F {
    fn from(src: D2D_VECTOR_2F) -> Vector2F {
        Vector2F { x: src.x, y: src.y }
    }
}
impl AsRef<D2D_VECTOR_2F> for Vector2F {
    fn as_ref(&self) -> &D2D_VECTOR_2F {
        unsafe { (self as *const Vector2F as *const D2D_VECTOR_2F).as_ref().unwrap() }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector3F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Vector3F {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}
impl From<Vector3F> for D2D_VECTOR_3F {
    fn from(src: Vector3F) -> D2D_VECTOR_3F {
        D2D_VECTOR_3F {
            x: src.x,
            y: src.y,
            z: src.z,
        }
    }
}
impl From<D2D_VECTOR_3F> for Vector3F {
    fn from(src: D2D_VECTOR_3F) -> Vector3F {
        Vector3F {
            x: src.x,
            y: src.y,
            z: src.z,
        }
    }
}
impl AsRef<D2D_VECTOR_3F> for Vector3F {
    fn as_ref(&self) -> &D2D_VECTOR_3F {
        unsafe { (self as *const Vector3F as *const D2D_VECTOR_3F).as_ref().unwrap() }
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(C)]
pub struct Vector4F {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}
impl Vector4F {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Self { x, y, z, w }
    }
}
impl From<Vector4F> for D2D_VECTOR_4F {
    fn from(src: Vector4F) -> D2D_VECTOR_4F {
        D2D_VECTOR_4F {
            x: src.x,
            y: src.y,
            z: src.z,
            w: src.w,
        }
    }
}
impl From<D2D_VECTOR_4F> for Vector4F {
    fn from(src: D2D_VECTOR_4F) -> Vector4F {
        Vector4F {
            x: src.x,
            y: src.y,
            z: src.z,
            w: src.w,
        }
    }
}
impl AsRef<D2D_VECTOR_4F> for Vector4F {
    fn as_ref(&self) -> &D2D_VECTOR_4F {
        unsafe { (self as *const Vector4F as *const D2D_VECTOR_4F).as_ref().unwrap() }
    }
}

#[derive(Clone, Debug)]
pub struct ArcSegment {
    pub point: Point2F,
    pub size: SizeF,
    pub rotation_angle: f32,
    pub sweep_direction: SweepDirection,
    pub arc_size: ArcSize,
}
impl ArcSegment {
    fn to_c_struct(&self) -> D2D1_ARC_SEGMENT {
        D2D1_ARC_SEGMENT {
            point: self.point.into(),
            size: self.size.into(),
            rotationAngle: self.rotation_angle,
            sweepDirection: self.sweep_direction as u32,
            arcSize: self.arc_size as u32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BezierSegment {
    pub point1: Point2F,
    pub point2: Point2F,
    pub point3: Point2F,
}
impl BezierSegment {
    fn to_c_struct(&self) -> D2D1_BEZIER_SEGMENT {
        D2D1_BEZIER_SEGMENT {
            point1: self.point1.into(),
            point2: self.point2.into(),
            point3: self.point3.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BitmapBrushProperties {
    pub extend_mode_x: ExtendMode,
    pub extend_mode_y: ExtendMode,
    pub interpolation_mode: BitmapInterpolationMode,
}
impl BitmapBrushProperties {
    fn to_c_struct(&self) -> D2D1_BITMAP_BRUSH_PROPERTIES {
        D2D1_BITMAP_BRUSH_PROPERTIES {
            extendModeX: self.extend_mode_x as u32,
            extendModeY: self.extend_mode_y as u32,
            interpolationMode: self.interpolation_mode as u32,
        }
    }
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct BitmapBrushProperties1 {
    pub extend_mode_x: ExtendMode,
    pub extend_mode_y: ExtendMode,
    pub interpolation_mode: BitmapInterpolationMode,
}
#[cfg(feature = "d2d1_1")]
impl BitmapBrushProperties1 {
    fn to_c_struct(&self) -> D2D1_BITMAP_BRUSH_PROPERTIES1 {
        D2D1_BITMAP_BRUSH_PROPERTIES1 {
            extendModeX: self.extend_mode_x as u32,
            extendModeY: self.extend_mode_y as u32,
            interpolationMode: self.interpolation_mode as u32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BitmapProperties {
    pub pixel_format: PixelFormat,
    pub dpi_x: f32,
    pub dpi_y: f32,
}
impl BitmapProperties {
    fn to_c_struct(&self) -> D2D1_BITMAP_PROPERTIES {
        D2D1_BITMAP_PROPERTIES {
            pixelFormat: self.pixel_format.to_c_struct(),
            dpiX: self.dpi_x,
            dpiY: self.dpi_y,
        }
    }
}

/*
#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct BitmapProperties1 {
    pub pixel_format: PixelFormat,
    pub dpi_x: f32,
    pub dpi_y: f32,
    pub bitmap_options: BitmapOptions,
    pub color_context: Option<ColorContext>,
}
#[cfg(feature = "d2d1_1")]
impl BitmapProperties1 {
    fn to_c_struct(&self) -> D2D1_BITMAP_PROPERTIES1 {
        D2D1_BITMAP_PROPERTIES1 {
            pixelFormat: self.pixel_format.to_c_struct(),
            dpiX: self.dpi_x,
            dpiY: self.dpi_y,
            bitmapOptions: self.bitmap_options.0,
            colorContext: self
                .color_context
                .map_or(std::ptr::null_mut(), |p| p.as_ptr()),
        }
    }
}
*/

#[derive(Clone, Debug)]
pub struct BlendDescription {
    pub source_blend: Blend,
    pub destination_blend: Blend,
    pub blend_operation: BlendOperation,
    pub source_blend_alpha: Blend,
    pub destination_blend_alpha: Blend,
    pub blend_operation_alpha: BlendOperation,
    pub blend_factor: [f32; 4],
}
impl BlendDescription {
    fn to_c_struct(&self) -> D2D1_BLEND_DESCRIPTION {
        D2D1_BLEND_DESCRIPTION {
            sourceBlend: self.source_blend as u32,
            destinationBlend: self.destination_blend as u32,
            blendOperation: self.blend_operation as u32,
            sourceBlendAlpha: self.source_blend_alpha as u32,
            destinationBlendAlpha: self.destination_blend_alpha as u32,
            blendOperationAlpha: self.blend_operation_alpha as u32,
            blendFactor: self.blend_factor.clone(),
        }
    }
}
impl From<D2D1_BLEND_DESCRIPTION> for BlendDescription {
    fn from(src: D2D1_BLEND_DESCRIPTION) -> BlendDescription {
        unsafe {
            BlendDescription {
                source_blend: std::mem::transmute(src.sourceBlend),
                destination_blend: std::mem::transmute(src.destinationBlend),
                blend_operation: std::mem::transmute(src.blendOperation),
                source_blend_alpha: std::mem::transmute(src.sourceBlendAlpha),
                destination_blend_alpha: std::mem::transmute(src.destinationBlendAlpha),
                blend_operation_alpha: std::mem::transmute(src.blendOperationAlpha),
                blend_factor: src.blendFactor,
            }
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct BrushProperties {
    pub opacity: f32,
    pub transform: Matrix3x2F,
}
impl BrushProperties {
    fn to_c_struct(&self) -> D2D1_BRUSH_PROPERTIES {
        D2D1_BRUSH_PROPERTIES {
            opacity: self.opacity,
            transform: self.transform.into(),
        }
    }
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct CreationProperties {
    pub threading_mode: ThreadingMode,
    pub debug_level: DebugLevel,
    pub options: Option<DeviceContextOptions>,
}
#[cfg(feature = "d2d1_1")]
impl CreationProperties {
    fn to_c_struct(&self) -> D2D1_CREATION_PROPERTIES {
        D2D1_CREATION_PROPERTIES {
            threadingMode: self.threading_mode as u32,
            debugLevel: self.debug_level as u32,
            options: self.options.map_or(0, |v| v as u32),
        }
    }
}

#[derive(Clone, Debug)]
pub struct CustomVertexBufferProperties<'a, 'b> {
    pub shader_buffer_with_input_signature: &'a [u8],
    pub input_elements: &'b [InputElementDesc<'b>],
    pub stride: u32,
}
impl<'a, 'b> CustomVertexBufferProperties<'a, 'b> {
    fn to_c_struct(
        &self,
    ) -> (
        D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES,
        Vec<D2D1_INPUT_ELEMENT_DESC>,
        Vec<std::ffi::CString>,
    ) {
        let (input_elements, names): (Vec<_>, Vec<_>) =
            self.input_elements.iter().map(|i| i.to_c_struct()).unzip();
        (
            D2D1_CUSTOM_VERTEX_BUFFER_PROPERTIES {
                shaderBufferWithInputSignature: self.shader_buffer_with_input_signature.as_ptr()
                    as *const u8,
                shaderBufferSize: self.shader_buffer_with_input_signature.len() as u32,
                inputElements: input_elements.as_ptr(),
                elementCount: input_elements.len() as u32,
                stride: self.stride,
            },
            input_elements,
            names,
        )
    }
}

#[derive(Clone, Debug)]
pub struct DrawingStateDescription {
    pub antialias_mode: AntialiasMode,
    pub text_antialias_mode: TextAntialiasMode,
    pub tag1: Tag,
    pub tag2: Tag,
    pub transform: Matrix3x2F,
}
impl DrawingStateDescription {
    fn to_c_struct(&self) -> D2D1_DRAWING_STATE_DESCRIPTION {
        D2D1_DRAWING_STATE_DESCRIPTION {
            antialiasMode: self.antialias_mode as u32,
            textAntialiasMode: self.text_antialias_mode as u32,
            tag1: self.tag1.0,
            tag2: self.tag2.0,
            transform: self.transform.into(),
        }
    }
}
impl From<D2D1_DRAWING_STATE_DESCRIPTION> for DrawingStateDescription {
    fn from(src: D2D1_DRAWING_STATE_DESCRIPTION) -> DrawingStateDescription {
        unsafe {
            DrawingStateDescription {
                antialias_mode: std::mem::transmute(src.antialiasMode),
                text_antialias_mode: std::mem::transmute(src.textAntialiasMode),
                tag1: Tag(src.tag1),
                tag2: Tag(src.tag2),
                transform: src.transform.into(),
            }
        }
    }
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct DrawingStateDescription1 {
    pub antialias_mode: AntialiasMode,
    pub text_antialias_mode: TextAntialiasMode,
    pub tag1: Tag,
    pub tag2: Tag,
    pub transform: Matrix3x2F,
    pub primitive_blend: PrimitiveBlend,
    pub unit_mode: UnitMode,
}
#[cfg(feature = "d2d1_1")]
impl DrawingStateDescription1 {
    fn to_c_struct(&self) -> D2D1_DRAWING_STATE_DESCRIPTION1 {
        D2D1_DRAWING_STATE_DESCRIPTION1 {
            antialiasMode: self.antialias_mode as u32,
            textAntialiasMode: self.text_antialias_mode as u32,
            tag1: self.tag1.0,
            tag2: self.tag2.0,
            transform: self.transform.into(),
            primitiveBlend: self.primitive_blend as u32,
            unitMode: self.unit_mode as u32,
        }
    }
}

/*
#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct EffectInputDescription {
    pub effect: Effect,
    pub input_index: u32,
    pub input_rectangle: RectF,
}
#[cfg(feature = "d2d1_1")]
impl EffectInputDescription {
    fn to_c_struct(&self) -> D2D1_EFFECT_INPUT_DESCRIPTION {
        D2D1_EFFECT_INPUT_DESCRIPTION {
            effect: self.effect.as_ptr(),
            inputIndex: self.input_index,
            inputRectangle: self.input_rectangle.into(),
        }
    }
}
*/

#[derive(Clone, Debug)]
pub struct Ellipse {
    pub point: Point2F,
    pub radius_x: f32,
    pub radius_y: f32,
}
impl Ellipse {
    fn to_c_struct(&self) -> D2D1_ELLIPSE {
        D2D1_ELLIPSE {
            point: self.point.into(),
            radiusX: self.radius_x,
            radiusY: self.radius_y,
        }
    }
}
impl From<D2D1_ELLIPSE> for Ellipse {
    fn from(src: D2D1_ELLIPSE) -> Ellipse {
        Ellipse {
            point: src.point.into(),
            radius_x: src.radiusX,
            radius_y: src.radiusY,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FactoryOptions {
    pub debug_level: DebugLevel,
}
impl FactoryOptions {
    fn to_c_struct(&self) -> D2D1_FACTORY_OPTIONS {
        D2D1_FACTORY_OPTIONS {
            debugLevel: self.debug_level as u32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FeatureDataDoubles {
    pub double_precision_float_shader_ops: bool,
}

#[derive(Clone, Debug)]
pub struct FeatureDataD3D10XHardwareOptions {
    pub compute_shaders_plus_raw_and_structured_buffers_via_shader_4_x: bool,
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Debug)]
pub struct GradientMeshPatch {
    pub point00: Point2F,
    pub point01: Point2F,
    pub point02: Point2F,
    pub point03: Point2F,
    pub point10: Point2F,
    pub point11: Point2F,
    pub point12: Point2F,
    pub point13: Point2F,
    pub point20: Point2F,
    pub point21: Point2F,
    pub point22: Point2F,
    pub point23: Point2F,
    pub point30: Point2F,
    pub point31: Point2F,
    pub point32: Point2F,
    pub point33: Point2F,
    pub color00: ColorF,
    pub color03: ColorF,
    pub color30: ColorF,
    pub color33: ColorF,
    pub top_edge_mode: PatchEdgeMode,
    pub left_edge_mode: PatchEdgeMode,
    pub bottom_edge_mode: PatchEdgeMode,
    pub right_edge_mode: PatchEdgeMode,
}
*/

#[derive(Clone, Debug)]
pub struct GradientStop {
    pub position: f32,
    pub color: ColorF,
}
impl GradientStop {
    fn to_c_struct(&self) -> D2D1_GRADIENT_STOP {
        D2D1_GRADIENT_STOP {
            position: self.position,
            color: self.color.into(),
        }
    }
}
impl From<D2D1_GRADIENT_STOP> for GradientStop {
    fn from(src: D2D1_GRADIENT_STOP) -> GradientStop {
        GradientStop {
            position: src.position,
            color: src.color.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct HwndRenderTargetPropertices {
    pub hwnd: HWND,
    pub pixel_size: SizeU,
    pub present_options: Option<PresentOptions>,
}
impl HwndRenderTargetPropertices {
    fn to_c_struct(&self) -> D2D1_HWND_RENDER_TARGET_PROPERTIES {
        D2D1_HWND_RENDER_TARGET_PROPERTIES {
            hwnd: self.hwnd,
            pixelSize: self.pixel_size.into(),
            presentOptions: self
                .present_options
                .map_or(PresentOptions::None as u32, |v| v as u32),
        }
    }
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Debug)]
pub struct InkStyleProperties {
    pub nib_shape: InkNIBShape,
    pub nib_transform: Matrix3x2F,
}
impl InkStyleProperties {
    fn to_c_struct(&self) -> D2D1_INK_STYLE_PROPERTIES {
        D2D1_INK_STYLE_PROPERTIES {
            nibShape: self.nib_shape.to_c_struct(),
            nibTrasform: self.nib_transform.into(),
        }
    }
}
*/

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct ImageBrushProperties {
    pub source_rectangle: RectF,
    pub extend_mode_x: ExtendMode,
    pub extend_mode_y: ExtendMode,
    pub interpolation_mode: InterpolationMode,
}
impl ImageBrushProperties {
    fn to_c_struct(&self) -> D2D1_IMAGE_BRUSH_PROPERTIES {
        D2D1_IMAGE_BRUSH_PROPERTIES {
            sourceRectangle: self.source_rectangle.into(),
            extendModeX: self.extend_mode_x as u32,
            extendModeY: self.extend_mode_y as u32,
            interpolationMode: self.interpolation_mode as u32,
        }
    }
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Debug)]
pub struct InkBezierSegment {
    pub point1: InkPoint,
    pub point2: InkPoint,
    pub point3: InkPoint,
}
impl InkBezierSegment {
    fn to_c_struct(&self) -> D2D1_INK_BEZIER_SEGMENT {
        D2D1_INK_BEZIER_SEGMENT {
            point1: self.point1.into(),
            point2: self.point2.into(),
            point3: self.point3.into(),
        }
    }
}

#[cfg(feature = "d2d1_3")]
#[derive(Clone, Debug)]
pub struct InkPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}
*/

#[derive(Clone, Debug)]
pub struct InputDescription {
    pub filter: Filter,
    pub level_of_detail_count: u32,
}
impl InputDescription {
    fn to_c_struct(&self) -> D2D1_INPUT_DESCRIPTION {
        D2D1_INPUT_DESCRIPTION {
            filter: self.filter as u32,
            // typo in winapi-rs
            leveOfDetailCount: self.level_of_detail_count,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InputElementDesc<'a> {
    pub semantic_name: &'a str,
    pub semantic_index: u32,
    pub format: dxgi::Format,
    pub input_slot: u32,
    pub aligned_byte_offset: u32,
}
impl<'a> InputElementDesc<'a> {
    fn to_c_struct(&self) -> (D2D1_INPUT_ELEMENT_DESC, std::ffi::CString) {
        let name = std::ffi::CString::new(self.semantic_name).unwrap();
        (
            D2D1_INPUT_ELEMENT_DESC {
                semanticName: self.semantic_name.as_ptr() as *const i8,
                semanticIndex: self.semantic_index,
                format: self.format as u32,
                inputSlot: self.input_slot,
                alignedByteOffset: self.aligned_byte_offset,
            },
            name,
        )
    }
}

#[derive(Clone, Debug)]
pub struct LayerParameters {
    pub content_bounds: RectF,
    pub geometric_mask: Geometry,
    pub mask_antialias_mode: AntialiasMode,
    pub mask_transform: Matrix3x2F,
    pub opacity: f32,
    pub opacity_brush: Brush,
    pub layer_options: Option<LayerOptions>,
}
impl LayerParameters {
    fn to_c_struct(&self) -> D2D1_LAYER_PARAMETERS {
        D2D1_LAYER_PARAMETERS {
            contentBounds: self.content_bounds.into(),
            geometricMask: self.geometric_mask.0.as_ptr() as *mut _,
            maskAntialiasMode: self.mask_antialias_mode as u32,
            maskTransform: self.mask_transform.into(),
            opacity: self.opacity,
            opacityBrush: self.opacity_brush.0.as_ptr() as *mut _,
            layerOptions: self
                .layer_options
                .map_or(LayerOptions::None as u32, |v| v as u32),
        }
    }
}

/*
#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct LayerParameters1 {
    pub content_bounds: RectF,
    pub geometric_mask: Geometry,
    pub mask_antialias_mode: AntialiasMode,
    pub mask_transform: Matrix3x2F,
    pub opacity: f32,
    pub opacity_brush: Brush,
    pub layer_options: Option<LayerOptions1>,
}
#[cfg(feature = "d2d1_1")]
impl LayerParameters1 {
    fn to_c_struct(&self) -> D2D1_LAYER_PARAMETERS1 {
        D2D1_LAYER_PARAMETERS1 {
            contentBounds: self.content_bounds.into(),
            geometricMask: self.geometric_mask.as_ptr(),
            maskAntialiasMode: self.mask_antialias_mode as u32,
            maskTransform: self.mask_transform.into(),
            opacity: self.opacity,
            opacityBrush: self.opacity_brush.as_ptr(),
            layerOptions: self
                .layer_options
                .map_or(LayerOptions1::None as u32, |v| v as u32),
        }
    }
}
*/

#[derive(Clone, Debug)]
pub struct LinearGradientBrushProperties {
    pub start_point: Point2F,
    pub end_point: Point2F,
}
impl LinearGradientBrushProperties {
    fn to_c_struct(&self) -> D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
        D2D1_LINEAR_GRADIENT_BRUSH_PROPERTIES {
            startPoint: self.start_point.into(),
            endPoint: self.end_point.into(),
        }
    }
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct MappedRect {
    pub pitch: u32,
    pub bits: *mut u8,
}
impl From<D2D1_MAPPED_RECT> for MappedRect {
    fn from(src: D2D1_MAPPED_RECT) -> MappedRect {
        MappedRect {
            pitch: src.pitch,
            bits: src.bits as *mut u8,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PixelFormat {
    pub format: dxgi::Format,
    pub alpha_mode: AlphaMode,
}
impl PixelFormat {
    fn to_c_struct(&self) -> D2D1_PIXEL_FORMAT {
        D2D1_PIXEL_FORMAT {
            format: self.format as u32,
            alphaMode: self.alpha_mode as u32,
        }
    }
}
impl From<D2D1_PIXEL_FORMAT> for PixelFormat {
    fn from(src: D2D1_PIXEL_FORMAT) -> PixelFormat {
        unsafe {
            PixelFormat {
                format: std::mem::transmute(src.format),
                alpha_mode: std::mem::transmute(src.alphaMode),
            }
        }
    }
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct PointDescription {
    pub point: Point2F,
    pub unit_tangent_vector: Point2F,
    pub end_segment: u32,
    pub end_figure: u32,
    pub length_to_end_segment: f32,
}
#[cfg(feature = "d2d1_1")]
impl PointDescription {
    fn to_c_struct(&self) -> D2D1_POINT_DESCRIPTION {
        D2D1_POINT_DESCRIPTION {
            point: self.point.into(),
            unitTangentVector: self.unit_tangent_vector.into(),
            endSegment: self.end_segment,
            endFigure: self.end_figure,
            lengthToEndSegment: self.length_to_end_segment,
        }
    }
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct PrintControlProperties {
    pub font_subset: PrintFontSubsetMode,
    pub raster_dpi: f32,
    pub color_space: ColorSpace,
}
#[cfg(feature = "d2d1_1")]
impl PrintControlProperties {
    fn to_c_struct(&self) -> D2D1_PRINT_CONTROL_PROPERTIES {
        D2D1_PRINT_CONTROL_PROPERTIES {
            fontSubset: self.font_subset as u32,
            rasterDPI: self.raster_dpi,
            colorSpace: self.color_space as u32,
        }
    }
}

/*
#[derive(Clone, Debug)]
pub struct PropertyBinding {
    pub property_name: String,
    pub set_function: fn(effect: &dyn IEffect, data: &[u8]) -> Result<(), HResult>,
    pub get_function: fn(effect: &dyn IEffect, data: &[u8], actualSize: &mut u32) -> Result<(), HResult>,
}
*/

#[derive(Clone, Debug)]
pub struct QuadraticBezierSegment {
    pub point1: Point2F,
    pub point2: Point2F,
}
impl QuadraticBezierSegment {
    fn to_c_struct(&self) -> D2D1_QUADRATIC_BEZIER_SEGMENT {
        D2D1_QUADRATIC_BEZIER_SEGMENT {
            point1: self.point1.into(),
            point2: self.point2.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RadialGradientBrushProperties {
    pub center: Point2F,
    pub gradient_origin_offset: Point2F,
    pub radius_x: f32,
    pub radius_y: f32,
}
impl RadialGradientBrushProperties {
    fn to_c_struct(&self) -> D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
        D2D1_RADIAL_GRADIENT_BRUSH_PROPERTIES {
            center: self.center.into(),
            gradientOriginOffset: self.gradient_origin_offset.into(),
            radiusX: self.radius_x,
            radiusY: self.radius_y,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ResourceTextureProperties<'a> {
    pub extents: &'a [u32],
    pub buffer_precision: BufferPrecision,
    pub channel_depth: ChannelDepth,
    pub filter: Filter,
    pub extend_modes: &'a [ExtendMode],
}
impl<'a> ResourceTextureProperties<'a> {
    fn to_c_struct(&self) -> (D2D1_RESOURCE_TEXTURE_PROPERTIES, Vec<u32>) {
        assert!(self.extents.len() >= 1 && self.extents.len() <= 3);
        assert!(self.extend_modes.len() >= 1 && self.extend_modes.len() <= 3);

        let extend_modes = self
            .extend_modes
            .iter()
            .map(|&modes| modes as u32)
            .collect::<Vec<_>>();
        (
            D2D1_RESOURCE_TEXTURE_PROPERTIES {
                extents: self.extents.as_ptr(),
                dimensions: self.extents.len() as u32,
                bufferPrecision: self.buffer_precision as u32,
                channelDepth: self.channel_depth as u32,
                filter: self.filter as u32,
                extendModes: extend_modes.as_ptr(),
            },
            extend_modes,
        )
    }
}

/*
#[derive(Clone, Debug)]
pub struct ResourceUsage {
    pub working_texture_area_memory: usize,
    pub caching_texture_area_memory: usize,
    pub shader_cache_memory: usize,
}
impl ResourceUsage {
    fn to_c_struct(&self) -> D2D1_RESOURCE_USAGE {
        D2D1_RESOURCE_USAGE {
            workingTextureAreaMemory: self.working_texture_area_memory,
            cachingTextureAreaMemory: self.caching_texture_area_memory,
            shaderCacheMemory: self.shader_cache_memory,
        }
    }
}
*/

#[derive(Clone, Debug)]
pub struct RenderTargetProperties {
    pub ty: RenderTargetType,
    pub pixel_format: PixelFormat,
    pub dpi_x: f32,
    pub dpi_y: f32,
    pub usage: RenderTargetUsage,
    pub min_level: FeatureLevel,
}
impl RenderTargetProperties {
    fn to_c_struct(&self) -> D2D1_RENDER_TARGET_PROPERTIES {
        D2D1_RENDER_TARGET_PROPERTIES {
            _type: self.ty as u32,
            pixelFormat: self.pixel_format.to_c_struct(),
            dpiX: self.dpi_x,
            dpiY: self.dpi_y,
            usage: self.usage as u32,
            minLevel: self.min_level as u32,
        }
    }
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct RenderingControls {
    pub buffer_precision: BufferPrecision,
    pub tile_size: SizeU,
}
#[cfg(feature = "d2d1_1")]
impl RenderingControls {
    fn to_c_struct(&self) -> D2D1_RENDERING_CONTROLS {
        D2D1_RENDERING_CONTROLS {
            bufferPrecision: self.buffer_precision as u32,
            tileSize: self.tile_size.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct RoundedRect {
    pub rect: RectF,
    pub radius_x: f32,
    pub radius_y: f32,
}
impl RoundedRect {
    fn to_c_struct(&self) -> D2D1_ROUNDED_RECT {
        D2D1_ROUNDED_RECT {
            rect: self.rect.into(),
            radiusX: self.radius_x,
            radiusY: self.radius_y,
        }
    }
}
impl From<D2D1_ROUNDED_RECT> for RoundedRect {
    fn from(src: D2D1_ROUNDED_RECT) -> RoundedRect {
        RoundedRect {
            rect: src.rect.into(),
            radius_x: src.radiusX,
            radius_y: src.radiusY,
        }
    }
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Debug)]
pub struct SimpleColorProfile {
    pub red_primary: Point2F,
    pub green_primary: Point2F,
    pub blue_primary: Point2F,
    pub white_point_xz: Point2F,
    pub gamma: Gamma1,
}
#[cfg(feature = "d2d1_3")]
impl SimpleColorProfile {
    fn to_c_struct(&self) -> D2D1_SIMPLE_COLOR_PROFILE {
        D2D1_SIMPLE_COLOR_PROFILE {
            redPrimary: self.red_primary.into(),
            greenPrimary: self.green_primary.into(),
            bluePrimary: self.blue_primary.into(),
            whitePointXZ: self.white_point_xz.into(),
            gamma: self.gamma as u32,
        }
    }
}
*/

#[derive(Clone, Debug)]
pub struct StrokeStyleProperties {
    pub start_cap: CapStyle,
    pub end_cap: CapStyle,
    pub dash_cap: CapStyle,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub dash_style: DashStyle,
    pub dash_offset: f32,
}
impl StrokeStyleProperties {
    fn to_c_struct(&self) -> D2D1_STROKE_STYLE_PROPERTIES {
        D2D1_STROKE_STYLE_PROPERTIES {
            startCap: self.start_cap as u32,
            endCap: self.end_cap as u32,
            dashCap: self.dash_cap as u32,
            lineJoin: self.line_join as u32,
            miterLimit: self.miter_limit,
            dashStyle: self.dash_style as u32,
            dashOffset: self.dash_offset,
        }
    }
}

#[cfg(feature = "d2d1_1")]
#[derive(Clone, Debug)]
pub struct StrokeStyleProperties1 {
    pub start_cap: CapStyle,
    pub end_cap: CapStyle,
    pub dash_cap: CapStyle,
    pub line_join: LineJoin,
    pub miter_limit: f32,
    pub dash_style: DashStyle,
    pub dash_offset: f32,
    pub transform_type: StrokeTransformType,
}
#[cfg(feature = "d2d1_1")]
impl StrokeStyleProperties1 {
    fn to_c_struct(&self) -> D2D1_STROKE_STYLE_PROPERTIES1 {
        D2D1_STROKE_STYLE_PROPERTIES1 {
            startCap: self.start_cap as u32,
            endCap: self.end_cap as u32,
            dashCap: self.dash_cap as u32,
            lineJoin: self.line_join as u32,
            miterLimit: self.miter_limit,
            dashStyle: self.dash_style as u32,
            dashOffset: self.dash_offset,
            transformType: self.transform_type as u32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SVGLength {
    pub value: f32,
    pub units: SVGLengthUnits,
}
impl SVGLength {
    fn to_c_struct(&self) -> D2D1_SVG_LENGTH {
        D2D1_SVG_LENGTH {
            value: self.value,
            units: self.units as u32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SVGPreserveAspectRatio {
    pub defer: bool,
    pub align: SVGAspectAlign,
    pub meet_or_slice: SVGAspectScaling,
}
impl SVGPreserveAspectRatio {
    fn to_c_struct(&self) -> D2D1_SVG_PRESERVE_ASPECT_RATIO {
        D2D1_SVG_PRESERVE_ASPECT_RATIO {
            defer: to_BOOL(self.defer),
            align: self.align as u32,
            meetOrSlice: self.meet_or_slice as u32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SVGViewBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}
impl SVGViewBox {
    fn to_c_struct(&self) -> D2D1_SVG_VIEWBOX {
        D2D1_SVG_VIEWBOX {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
        }
    }
}

/*
#[cfg(feature = "d2d1_3")]
#[derive(Clone, Debug)]
pub struct TransformedImageSourceProperties {
    pub orientation: Orientation,
    pub scale_x: f32,
    pub scale_y: f32,
    pub interpolation_mode: InterpolationMode,
    pub options: Option<TransformedImageSourceOptions>,
}
#[cfg(feature = "d2d1_3")]
impl TransformedImageSourceProperties {
    fn to_c_struct(&self) -> D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
        D2D1_TRANSFORMED_IMAGE_SOURCE_PROPERTIES {
            orientation: self.orientation as u32,
            scaleX: self.scale_x,
            scaleY: self.scale_y,
            interpolationMode: self.interpolation_mode as u32,
            options: self.options.map_or(TransformedImageSourceOptions::None as u32, |v| v as u32),
        }
    }
}
*/

#[derive(Clone, Debug)]
pub struct Triangle {
    pub point1: Point2F,
    pub point2: Point2F,
    pub point3: Point2F,
}
impl Triangle {
    fn to_c_struct(&self) -> D2D1_TRIANGLE {
        D2D1_TRIANGLE {
            point1: self.point1.into(),
            point2: self.point2.into(),
            point3: self.point3.into(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct VertexBufferProperties<'a> {
    pub input_count: u32,
    pub usage: VertexUsage,
    pub data: &'a [u8],
    pub byte_width: u32,
}
impl<'a> VertexBufferProperties<'a> {
    fn to_c_struct(&self) -> D2D1_VERTEX_BUFFER_PROPERTIES {
        D2D1_VERTEX_BUFFER_PROPERTIES {
            inputCount: self.input_count,
            usage: self.usage as u32,
            data: self.data.as_ptr(),
            byteWidth: self.byte_width,
        }
    }
}

#[derive(Clone, Debug)]
pub struct VertexRange {
    pub start_vertex: u32,
    pub vertex_count: u32,
}
impl VertexRange {
    fn to_c_struct(&self) -> D2D1_VERTEX_RANGE {
        D2D1_VERTEX_RANGE {
            startVertex: self.start_vertex,
            vertexCount: self.vertex_count,
        }
    }
}

macro_rules! impl_bitmap {
    ($s: ident, $interface: ident) => {
        impl_image!($s, $interface);
        impl IBitmap for $s {
            fn copy_from_bitmap(
                &self,
                dest_point: impl Into<Point2U>,
                bitmap: &impl IBitmap,
                src_rect: impl Into<RectU>,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0.CopyFromBitmap(
                            &dest_point.into().into(),
                            bitmap.as_ptr() as *mut _,
                            &src_rect.into().into(),
                        ),
                    )
                }
            }
            unsafe fn copy_from_memory<T>(
                &self,
                dest_rect: impl Into<RectU>,
                src_data: *const T,
                pitch: u32,
            ) -> Result<(), HResult> {
                hresult(
                    (),
                    self.0
                        .CopyFromMemory(&dest_rect.into().into(), src_data as *const _, pitch),
                )
            }
            fn copy_from_render_target(
                &self,
                dest_point: impl Into<Point2U>,
                render_target: &impl IRenderTarget,
                src_rect: impl Into<RectU>,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0.CopyFromRenderTarget(
                            &dest_point.into().into(),
                            render_target.as_ptr() as *mut _,
                            &src_rect.into().into(),
                        ),
                    )
                }
            }
            fn get_dpi(&self) -> Vector2F {
                unsafe {
                    let mut v = Vector2F::new(0.0, 0.0);
                    self.0.GetDpi(&mut v.x, &mut v.y);
                    v
                }
            }
            fn get_pixel_format(&self) -> PixelFormat {
                unsafe {
                    std::mem::transmute(self.0.GetPixelFormat())
                }
            }
            fn get_pixel_size(&self) -> SizeU {
                unsafe { self.0.GetPixelSize().into() }
            }
            fn get_size(&self) -> SizeF {
                unsafe { self.0.GetSize().into() }
            }
        }
    };
}

macro_rules! impl_bitmap_brush {
    ($s: ident, $interface: ident) => {
        impl_brush!($s, $interface);
        impl IBitmapBrush for $s {
            fn get_bitmap(&self) -> Option<Bitmap> {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    self.0.GetBitmap(&mut p);
                    if p == std::ptr::null_mut() {
                        None
                    } else {
                        Some(Bitmap(ComPtr::from_raw(p)))
                    }
                }
            }
            fn get_extend_mode_x(&self) -> ExtendMode {
                unsafe { std::mem::transmute(self.0.GetExtendModeX()) }
            }
            fn get_extend_mode_y(&self) -> ExtendMode {
                unsafe { std::mem::transmute(self.0.GetExtendModeY()) }
            }
            fn get_interpolation_mode(&self) -> BitmapInterpolationMode {
                unsafe { std::mem::transmute(self.0.GetInterpolationMode()) }
            }
            fn set_bitmap(&self, bitmap: &impl IBitmap) {
                unsafe {
                    self.0.SetBitmap(bitmap.as_ptr() as *mut _);
                }
            }
            fn set_extend_mode_x(&self, mode: ExtendMode) {
                unsafe {
                    self.0.SetExtendModeX(mode as u32);
                }
            }
            fn set_extend_mode_y(&self, mode: ExtendMode) {
                unsafe {
                    self.0.SetExtendModeY(mode as u32);
                }
            }
            fn set_interpolation_mode(&self, mode: BitmapInterpolationMode) {
                unsafe {
                    self.0.SetInterpolationMode(mode as u32);
                }
            }
        }
    };
}

macro_rules! impl_bitmap_render_target {
    ($s: ident, $interface: ident) => {
        impl_render_target!($s, $interface);
        impl IBitmapRenderTarget for $s {
            fn get_bitmap(&self) -> Result<Bitmap, HResult> {
                Ok(Bitmap(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetBitmap(&mut p);
                    hresult(p, ret)
                })?))
            }
        }
    };
}

macro_rules! impl_brush {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl IBrush for $s {
            fn get_opacity(&self) -> f32 {
                unsafe { self.0.GetOpacity() }
            }
            fn get_transform(&self) -> Matrix3x2F {
                unsafe {
                    let mut m = Default::default();
                    self.0.GetTransform(&mut m);
                    m.into()
                }
            }
            fn set_opacity(&self, opacity: f32) {
                unsafe {
                    self.0.SetOpacity(opacity);
                }
            }
            fn set_transform(&self, m: &Matrix3x2F) {
                unsafe {
                    self.0.SetTransform(&m.clone().into());
                }
            }
        }
    };
}

macro_rules! impl_dc_render_target {
    ($s: ident, $interface: ident) => {
        impl_render_target!($s, $interface);
        impl IDCRenderTarget for $s {
            fn bind_dc(
                &self,
                hdc: &impl crate::api::DeviceContextHandle,
                sub_rect: impl Into<RectL>,
            ) -> Result<(), HResult> {
                unsafe { hresult((), self.0.BindDC(hdc.as_hdc(), &sub_rect.into().into())) }
            }
        }
    };
}

macro_rules! impl_drawing_state_block {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl IDrawingStateBlock for $s {
            fn get_description(&self) -> DrawingStateDescription {
                unsafe {
                    let mut desc = Default::default();
                    self.0.GetDescription(&mut desc);
                    desc.into()
                }
            }
            #[cfg(feature = "dwrite")]
            fn get_text_rendering_params(&self) -> Option<crate::dwrite::RenderingParams> {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    self.0.GetTextRenderingParams(&mut p);
                    if p == std::ptr::null_mut() {
                        None
                    } else {
                        Some(crate::dwrite::RenderingParams(ComPtr::from_raw(p)))
                    }
                }
            }
            fn set_description(&self, desc: &DrawingStateDescription) {
                unsafe { self.0.SetDescription(&desc.to_c_struct()); }
            }
            #[cfg(feature = "dwrite")]
            fn set_text_rendering_params(&self, params: &impl crate::dwrite::IRenderingParams) {
                unsafe { self.0.SetTextRenderingParams(params.as_ptr() as *mut _); }
            }
        }
    };
}

macro_rules! impl_ellipse_geometry {
    ($s: ident, $interface: ident) => {
        impl_geometry!($s, $interface);
        impl IEllipseGeometry for $s {
            fn get_ellipse(&self) -> Ellipse {
                unsafe {
                    let mut ellipse = Default::default();
                    self.0.GetEllipse(&mut ellipse);
                    ellipse.into()
                }
            }
        }
    };
}

macro_rules! impl_factory {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFactory for $s {
            fn create_dc_render_target(
                &self,
                props: &RenderTargetProperties,
            ) -> Result<DCRenderTarget, HResult> {
                Ok(DCRenderTarget(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateDCRenderTarget(&props.to_c_struct(), &mut p);
                    hresult(p, ret)
                })?))
            }
            #[cfg(feature = "dwrite")]
            fn create_drawing_state_block(
                &self,
                desc: &DrawingStateDescription,
                params: Option<&crate::dwrite::RenderingParams>,
            ) -> Result<DrawingStateBlock, HResult> {
                Ok(DrawingStateBlock(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateDrawingStateBlock(
                        &desc.to_c_struct(),
                        params.map_or(std::ptr::null_mut(), |p| p.as_ptr() as *mut _),
                        &mut p
                    );
                    hresult(p, ret)
                })?))
            }
            #[cfg(feature = "dxgi")]
            fn create_dxgi_surface_render_target(
                &self,
                surface: &impl dxgi::ISurface,
                props: &RenderTargetProperties,
            ) -> Result<RenderTarget, HResult> {
                Ok(RenderTarget(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateDxgiSurfaceRenderTarget(
                        surface.as_ptr() as *mut _,
                        &props.to_c_struct(),
                        &mut p
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_ellipse_geometry(
                &self,
                ellipse: &Ellipse,
            ) -> Result<EllipseGeometry, HResult> {
                Ok(EllipseGeometry(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateEllipseGeometry(&ellipse.to_c_struct(), &mut p);
                    hresult(p, ret)
                })?))
            }
            fn create_geometry_group(
                &self,
                fill_mode: FillMode,
                geometries: &[&impl IGeometry],
            ) -> Result<GeometryGroup, HResult> {
                let mut geometries = geometries.iter().map(|p| p.as_ptr() as *mut _).collect::<Vec<_>>();
                Ok(GeometryGroup(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self
                        .0
                        .CreateGeometryGroup(fill_mode as u32, geometries.as_mut_ptr(), geometries.len() as u32, &mut p);
                    hresult(p, ret)
                })?))
            }
            fn create_hwnd_render_target(
                &self,
                props: &RenderTargetProperties,
                hwnd_props: &HwndRenderTargetPropertices,
            ) -> Result<HwndRenderTarget, HResult> {
                Ok(HwndRenderTarget(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self
                        .0
                        .CreateHwndRenderTarget(&props.to_c_struct(), &hwnd_props.to_c_struct(), &mut p);
                    hresult(p, ret)
                })?))
            }
            fn craete_path_geometry(&self) -> Result<PathGeometry, HResult> {
                Ok(PathGeometry(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreatePathGeometry(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn create_rectangle_geometry(
                &self,
                rc: impl Into<RectF>,
            ) -> Result<RectangleGeometry, HResult> {
                Ok(RectangleGeometry(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateRectangleGeometry(rc.into().as_ref(), &mut p);
                    hresult(p, ret)
                })?))
            }
            fn create_rounded_rectangle_geometry(
                &self,
                rc: &RoundedRect,
            ) -> Result<RoundedRectangleGeometry, HResult> {
                Ok(RoundedRectangleGeometry(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateRoundedRectangleGeometry(&rc.to_c_struct(), &mut p);
                    hresult(p, ret)
                })?))
            }
            fn create_stroke_style(
                &self,
                props: &StrokeStyleProperties,
                dashes: Option<&[f32]>,
            ) -> Result<StrokeStyle, HResult> {
                Ok(StrokeStyle(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateStrokeStyle(
                        &props.to_c_struct(),
                        dashes.map_or(std::ptr::null(), |d| d.as_ptr()),
                        dashes.map_or(0, |d| d.len() as u32),
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_transformed_geometry(
                &self,
                src: &impl IGeometry,
                transform: Option<&Matrix3x2F>,
            ) -> Result<TransformedGeometry, HResult> {
                let transform = transform.map(|m| m.clone().into());
                Ok(TransformedGeometry(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateTransformedGeometry(
                        src.as_ptr() as *mut _,
                        transform.as_ref().map_or(std::ptr::null(), |m| m as *const _),
                        &mut p
                    );
                    hresult(p, ret)
                })?))
            }
            fn get_desktop_dpi(&self) -> Vector2F {
                unsafe {
                    let mut v = Vector2F::new(0.0, 0.0);
                    self.0.GetDesktopDpi(&mut v.x, &mut v.y);
                    v
                }
            }
            fn reload_system_metrics(&self) -> Result<(), HResult> {
                unsafe { hresult((), self.0.ReloadSystemMetrics()) }
            }
        }
    };
}

macro_rules! impl_gdi_interop_render_target {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IGdiInteropRenderTarget for $s {
            fn get_dc(&self, mode: DCInitializeMode) -> Result<HDC, HResult> {
                unsafe {
                    let mut hdc = std::ptr::null_mut();
                    let ret = self.0.GetDC(mode as u32, &mut hdc);
                    hresult(hdc, ret)
                }
            }
            fn release_dc(&self, update: impl Into<crate::api::Rect<i32>>) -> Result<(), HResult> {
                unsafe { hresult((), self.0.ReleaseDC(update.into().as_ref())) }
            }
        }
    };
}

macro_rules! impl_geometry {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl IGeometry for $s {
            fn combine_with_geometry(
                &self,
                input: &impl IGeometry,
                mode: CombineMode,
                transform: Option<&Matrix3x2F>,
                tolerance: f32,
                sink: &impl ISimplifiedGeometrySink,
            ) -> Result<(), HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    hresult(
                        (),
                        self.0.CombineWithGeometry(
                            input.as_ptr() as *mut _,
                            mode as u32,
                            transform
                                .as_ref()
                                .map_or(std::ptr::null(), |m| m as *const _),
                            tolerance,
                            sink.as_ptr() as *mut _,
                        ),
                    )
                }
            }
            fn compute_area(
                &self,
                transform: Option<&Matrix3x2F>,
                tolerance: f32,
            ) -> Result<f32, HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    let mut area = 0.0;
                    let ret = self.0.ComputeArea(
                        transform
                            .as_ref()
                            .map_or(std::ptr::null(), |m| m as *const _),
                        tolerance,
                        &mut area,
                    );
                    hresult(area, ret)
                }
            }
            fn compute_length(
                &self,
                transform: Option<&Matrix3x2F>,
                tolerance: f32,
            ) -> Result<f32, HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    let mut len = 0.0;
                    let ret = self.0.ComputeLength(
                        transform
                            .as_ref()
                            .map_or(std::ptr::null(), |m| m as *const _),
                        tolerance,
                        &mut len,
                    );
                    hresult(len, ret)
                }
            }
            fn compute_point_at_length(
                &self,
                length: f32,
                transform: Option<&Matrix3x2F>,
                tolerance: f32,
            ) -> Result<ComputePointAtLengthResult, HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    let mut point = Default::default();
                    let mut unit_tangent_vector = Default::default();
                    let ret = self.0.ComputePointAtLength(
                        length,
                        transform
                            .as_ref()
                            .map_or(std::ptr::null(), |m| m as *const _),
                        tolerance,
                        &mut point,
                        &mut unit_tangent_vector,
                    );
                    hresult(
                        ComputePointAtLengthResult {
                            point: point.into(),
                            unit_tangent_vector: unit_tangent_vector.into(),
                        },
                        ret,
                    )
                }
            }
            fn fill_contains_point(
                &self,
                point: impl Into<Point2F>,
                transform: Option<&Matrix3x2F>,
                tolerance: f32,
            ) -> Result<bool, HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    let mut contains = 0;
                    let ret = self.0.FillContainsPoint(
                        point.into().into(),
                        transform
                            .as_ref()
                            .map_or(std::ptr::null(), |m| m as *const _),
                        tolerance,
                        &mut contains,
                    );
                    hresult(contains == TRUE, ret)
                }
            }
            fn get_bounds(&self, transform: Option<&Matrix3x2F>) -> Result<RectF, HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    let mut rc = Default::default();
                    let ret = self.0.GetBounds(
                        transform.as_ref().map_or(std::ptr::null(), |m| m as *const _),
                        &mut rc
                    );
                    hresult(rc.into(), ret)
                }
            }
            fn get_widened_bounds(&self, width: f32, stroke_style: &impl IStrokeStyle, transform: Option<&Matrix3x2F>, tolerance: f32) -> Result<RectF, HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    let mut rc = Default::default();
                    let ret = self.0.GetWidenedBounds(
                        width,
                        stroke_style.as_ptr() as *mut _,
                        transform.as_ref().map_or(std::ptr::null(), |m| m as *const _),
                        tolerance,
                        &mut rc
                    );
                    hresult(rc.into(), ret)
                }
            }
            fn outline(&self, transform: Option<&Matrix3x2F>, tolerance: f32, sink: &impl ISimplifiedGeometrySink) -> Result<(), HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    hresult((), self.0.Outline(
                        transform.as_ref().map_or(std::ptr::null(), |m| m as *const _),
                        tolerance,
                        sink.as_ptr() as *mut _
                    ))
                }
            }
            fn simplify(&self, option: GeometrySimplificationOption, transform: Option<&Matrix3x2F>, tolerance: f32, sink: &impl ISimplifiedGeometrySink) -> Result<(), HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    hresult((), self.0.Simplify(
                        option as u32,
                        transform.as_ref().map_or(std::ptr::null(), |m| m as *const _),
                        tolerance,
                        sink.as_ptr() as *mut _
                    ))
                }
            }
            fn stroke_contains_point(&self, point: impl Into<Point2F>, width: f32, stroke_style: &impl IStrokeStyle, transform: Option<&Matrix3x2F>, tolerance: f32) -> Result<bool, HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    let mut contains = 0;
                    let ret = self.0.StrokeContainsPoint(
                        point.into().into(),
                        width,
                        stroke_style.as_ptr() as *mut _,
                        transform.as_ref().map_or(std::ptr::null(), |m| m as *const _),
                        tolerance,
                        &mut contains,
                    );
                    hresult(contains == TRUE, ret)
                }
            }
            fn tessellate(&self, transform: Option<&Matrix3x2F>, tolerance: f32, sink: &impl ITessellationSink) -> Result<(), HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    hresult((), self.0.Tessellate(
                        transform.as_ref().map_or(std::ptr::null(), |m| m as *const _),
                        tolerance,
                        sink.as_ptr() as *mut _
                    ))
                }
            }
            fn widen(&self, width: f32, stroke_style: &impl IStrokeStyle, transform: Option<&Matrix3x2F>, tolerance: f32, sink: &impl ISimplifiedGeometrySink) -> Result<(), HResult> {
                let transform = transform.map(|m| m.clone().into());
                unsafe {
                    hresult((), self.0.Widen(
                        width,
                        stroke_style.as_ptr() as *mut _,
                        transform.as_ref().map_or(std::ptr::null(), |m| m as *const _),
                        tolerance,
                        sink.as_ptr() as *mut _
                    ))
                }
            }
        }
    };
}

macro_rules! impl_geometry_group {
    ($s: ident, $interface: ident) => {
        impl_geometry!($s, $interface);
        impl IGeometryGroup for $s {
            fn get_fill_mode(&self) -> FillMode {
                unsafe { std::mem::transmute(self.0.GetFillMode()) }
            }
            fn get_source_geometries(&self) -> Vec<Geometry> {
                let len = self.get_source_geometry_count();
                unsafe {
                    let mut v = vec![std::ptr::null_mut(); len as usize];
                    self.0.GetSourceGeometries(v.as_mut_ptr(), len);
                    v.into_iter().map(|p| Geometry(ComPtr::from_raw(p))).collect::<Vec<_>>()
                }
            }
            fn get_source_geometry_count(&self) -> u32 {
                unsafe { self.0.GetSourceGeometryCount() }
            }
        }
    };
}

macro_rules! impl_geometry_sink {
    ($s: ident, $interface: ident) => {
        impl_simplified_geometry_sink!($s, $interface);
        impl IGeometrySink for $s {
            fn add_arc(&self, arc: &ArcSegment) {
                unsafe { self.0.AddArc(&arc.to_c_struct()) }
            }
            fn add_bezier(&self, bezier: &BezierSegment) {
                unsafe { self.0.AddBezier(&bezier.to_c_struct()) }
            }
            fn add_line(&self, point: impl Into<Point2F>) {
                unsafe { self.0.AddLine(point.into().into()) }
            }
            fn add_quadratic_bezier(&self, bezier: &QuadraticBezierSegment) {
                unsafe { self.0.AddQuadraticBezier(&bezier.to_c_struct()) }
            }
            fn add_quadratic_beziers(&self, beziers: &[&QuadraticBezierSegment]) {
                let beziers = beziers.iter().map(|seg| seg.to_c_struct()).collect::<Vec<_>>();
                unsafe {
                    self.0.AddQuadraticBeziers(beziers.as_ptr(), beziers.len() as u32);
                }
            }
        }
    };
}

macro_rules! impl_gradient_stop_collection {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl IGradientStopCollection for $s {
            fn get_color_interpolation_gamma(&self) -> Gamma {
                unsafe { std::mem::transmute(self.0.GetColorInterpolationGamma()) }
            }
            fn get_extend_mode(&self) -> ExtendMode {
                unsafe { std::mem::transmute(self.0.GetExtendMode()) }
            }
            fn get_gradient_stop_count(&self) -> u32 {
                unsafe{ self.0.GetGradientStopCount() }
            }
            fn get_gradient_stops(&self) -> Vec<GradientStop> {
                let len = self.get_gradient_stop_count();
                unsafe {
                    let mut v = vec![Default::default(); len as usize];
                    self.0.GetGradientStops(v.as_mut_ptr(), len);
                    v.into_iter().map(|s| s.into()).collect::<Vec<_>>()
                }
            }
        }
    };
}

macro_rules! impl_hwnd_render_target {
    ($s: ident, $interface: ident) => {
        impl_render_target!($s, $interface);
        impl IHwndRenderTarget for $s {
            fn check_window_state(&self) -> WindowState {
                unsafe { std::mem::transmute(self.0.CheckWindowState()) }
            }
            fn get_hwnd(&self) -> HWND {
                unsafe { self.0.GetHwnd() }
            }
            fn resize(&self, size: impl Into<SizeU>) -> Result<(), HResult> {
                unsafe { hresult((), self.0.Resize(size.into().as_ref())) }
            }
        }
    };
}

macro_rules! impl_image {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl IImage for $s {}
    };
}

macro_rules! impl_layer {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl ILayer for $s {
            fn get_size(&self) -> SizeF {
                unsafe { self.0.GetSize().into() }
            }
        }
    };
}

macro_rules! impl_linear_gradient_brush {
    ($s: ident, $interface: ident) => {
        impl_brush!($s, $interface);
        impl ILinearGradientBrush for $s {
            fn get_end_point(&self) -> Point2F {
                unsafe { self.0.GetEndPoint().into() }
            }
            fn get_gradient_stop_collection(&self) -> GradientStopCollection {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    self.0.GetGradientStopCollection(&mut p);
                    GradientStopCollection(ComPtr::from_raw(p))
                }
            }
            fn get_start_point(&self) -> Point2F {
                unsafe { self.0.GetStartPoint().into() }
            }
            fn set_end_point(&self, point: impl Into<Point2F>) {
                unsafe { self.0.SetEndPoint(point.into().into()); }
            }
            fn set_start_point(&self, point: impl Into<Point2F>) {
                unsafe { self.0.SetStartPoint(point.into().into()); }
            }
        }
    };
}

macro_rules! impl_mesh {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl IMesh for $s {
            fn open(&self) -> Result<TessellationSink, HResult> {
                Ok(TessellationSink(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.Open(&mut p);
                    hresult(p, ret)
                })?))
            }
        }
    };
}

macro_rules! impl_path_geometry {
    ($s: ident, $interface: ident) => {
        impl_geometry!($s, $interface);
        impl IPathGeometry for $s {
            fn get_figure_count(&self) -> Result<u32, HResult> {
                unsafe {
                    let mut count = 0;
                    let ret = self.0.GetFigureCount(&mut count);
                    hresult(count, ret)
                }
            }
            fn get_segment_count(&self) -> Result<u32, HResult> {
                unsafe {
                    let mut count = 0;
                    let ret = self.0.GetSegmentCount(&mut count);
                    hresult(count, ret)
                }
            }
            fn open(&self) -> Result<GeometrySink, HResult> {
                Ok(GeometrySink(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.Open(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn stream(&self, sink: &impl IGeometrySink) -> Result<(), HResult> {
                unsafe { hresult((), self.0.Stream(sink.as_ptr() as *mut _)) }
            }
        }
    };
}

macro_rules! impl_radial_gradient_brush {
    ($s: ident, $interface: ident) => {
        impl_brush!($s, $interface);
        impl IRadialGradientBrush for $s {
            fn get_center(&self) -> Point2F {
                unsafe { self.0.GetCenter().into() }
            }
            fn get_gradient_origin_offset(&self) -> Point2F {
                unsafe { self.0.GetGradientOriginOffset().into() }
            }
            fn get_gradient_stop_collection(&self) -> GradientStopCollection {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    self.0.GetGradientStopCollection(&mut p);
                    GradientStopCollection(ComPtr::from_raw(p))
                }
            }
            fn get_radius_x(&self) -> f32 {
                unsafe { self.0.GetRadiusX() }
            }
            fn get_radius_y(&self) -> f32 {
                unsafe { self.0.GetRadiusY() }
            }
            fn set_center(&self, center: impl Into<Point2F>) {
                unsafe { self.0.SetCenter(center.into().into()); }
            }
            fn set_gradient_origin_offset(&self, offset: impl Into<Point2F>) {
                unsafe { self.0.SetGradientOriginOffset(offset.into().into()) }
            }
            fn set_radius_x(&self, x: f32) {
                unsafe { self.0.SetRadiusX(x); }
            }
            fn set_radius_y(&self, y: f32) {
                unsafe { self.0.SetRadiusY(y); }
            }
        }
    };
}

macro_rules! impl_rectangle_geometry {
    ($s: ident, $interface: ident) => {
        impl_geometry!($s, $interface);
        impl IRectangleGeometry for $s {
            fn get_rect(&self) -> RectF {
                unsafe {
                    let mut rc = Default::default();
                    self.0.GetRect(&mut rc);
                    rc.into()
                }
            }
        }
    };
}

macro_rules! impl_render_target {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl IRenderTarget for $s {
            fn begin_draw(&self) {
                unsafe { self.0.BeginDraw(); }
            }
            fn clear(&self, color: impl Into<ColorF>) {
                let color = color.into();
                unsafe { self.0.Clear(&color.into()); }
            }
            unsafe fn create_bitmap(&self, size: impl Into<SizeU>, src_data: Option<&[u8]>, pitch: u32, props: &BitmapProperties) -> Result<Bitmap, HResult> {
                Ok(Bitmap(ComPtr::new(|| {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateBitmap(
                        size.into().into(),
                        src_data.map_or(std::ptr::null(), |d| d.as_ptr() as *const _),
                        pitch,
                        &props.to_c_struct(),
                        &mut p
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_bitmap_brush(&self, bitmap: &impl IBitmap, props: Option<&BitmapBrushProperties>, brush_props: Option<&BrushProperties>) -> Result<BitmapBrush, HResult> {
                let props = props.map(|p| p.to_c_struct());
                let brush_props = brush_props.map(|p| p.to_c_struct());
                Ok(BitmapBrush(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateBitmapBrush(
                        bitmap.as_ptr() as *mut _,
                        props.as_ref().map_or(std::ptr::null(), |p| p as *const _),
                        brush_props.as_ref().map_or(std::ptr::null(), |p| p as *const _),
                        &mut p
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_compatible_render_target(&self, size: Option<SizeF>, pixel_size: Option<SizeU>, format: Option<&PixelFormat>, options: CompatibleRenderTargetOptions) -> Result<BitmapRenderTarget, HResult> {
                let size = size.map(|s| D2D_SIZE_F::from(s));
                let pixel_size = pixel_size.map(|s| D2D_SIZE_U::from(s));
                let format = format.map(|f| f.to_c_struct());
                Ok(BitmapRenderTarget(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateCompatibleRenderTarget(
                        size.as_ref().map_or(std::ptr::null(), |s| s as *const _),
                        pixel_size.as_ref().map_or(std::ptr::null(), |s| s as *const _),
                        format.as_ref().map_or(std::ptr::null(), |f| f as *const _),
                        options as u32,
                        &mut p
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_gradient_stop_collection(&self, stops: &[GradientStop], gamma: Gamma, mode: ExtendMode) -> Result<GradientStopCollection, HResult> {
                let stops = stops.iter().map(|s| s.to_c_struct()).collect::<Vec<_>>();
                Ok(GradientStopCollection(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateGradientStopCollection(
                        stops.as_ptr(),
                        stops.len() as u32,
                        gamma as u32,
                        mode as u32,
                        &mut p
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_layer(&self, size: Option<SizeF>) -> Result<Layer, HResult> {
                let size = size.map(|s| s.into());
                Ok(Layer(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateLayer(size.as_ref().map_or(std::ptr::null(), |s| s as *const _), &mut p);
                    hresult(p, ret)
                })?))
            }
            fn create_linear_gradient_brush(&self, props: &LinearGradientBrushProperties, brush_props: Option<&BrushProperties>, gradient: &impl IGradientStopCollection) -> Result<LinearGradientBrush, HResult> {
                let brush_props = brush_props.map(|p| p.to_c_struct());
                Ok(LinearGradientBrush(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateLinearGradientBrush(
                        &props.to_c_struct(),
                        brush_props.as_ref().map_or(std::ptr::null(), |p| p as *const _),
                        gradient.as_ptr() as *mut _,
                        &mut p
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_mesh(&self) -> Result<Mesh, HResult> {
                Ok(Mesh(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateMesh(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn create_radial_gradient_brush(&self, props: &RadialGradientBrushProperties, brush_props: Option<&BrushProperties>, gradient: &impl IGradientStopCollection) -> Result<RadialGradientBrush, HResult> {
                let brush_props = brush_props.map(|p| p.to_c_struct());
                Ok(RadialGradientBrush(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateRadialGradientBrush(
                        &props.to_c_struct(),
                        brush_props.as_ref().map_or(std::ptr::null(), |p| p as *const _),
                        gradient.as_ptr() as *mut _,
                        &mut p
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_solid_color_brush(&self, color: impl Into<ColorF>, brush_props: Option<&BrushProperties>) -> Result<SolidColorBrush, HResult> {
                let brush_props = brush_props.map(|p| p.to_c_struct());
                Ok(SolidColorBrush(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateSolidColorBrush(
                        &color.into().into(),
                        brush_props.as_ref().map_or(std::ptr::null(), |p| p as *const _),
                        &mut p
                    );
                    hresult(p, ret)
                })?))
            }
            fn draw_bitmap(&self, bitmap: &impl IBitmap, dest_rect: Option<&RectF>, opacity: Option<f32>, interpolation: Option<BitmapInterpolationMode>, src_rect: Option<&RectF>) {
                let dest_rect = dest_rect.map(|rc| rc.clone().into());
                let src_rect = src_rect.map(|rc| rc.clone().into());
                unsafe {
                    self.0.DrawBitmap(
                        bitmap.as_ptr() as *mut _,
                        dest_rect.as_ref().map_or(std::ptr::null(), |rc| rc as *const _),
                        opacity.unwrap_or(1.0),
                        interpolation.map_or(0, |i| i as u32),
                        src_rect.as_ref().map_or(std::ptr::null(), |rc| rc as *const _),
                    );
                }
            }
            fn draw_ellipse(&self, ellipse: &Ellipse, brush: &impl IBrush, width: Option<f32>, stroke_style: Option<&StrokeStyle>) {
                unsafe {
                    self.0.DrawEllipse(
                        &ellipse.to_c_struct(),
                        brush.as_ptr() as *mut _,
                        width.unwrap_or(1.0),
                        stroke_style.map_or(std::ptr::null_mut(), |p| p.0.as_ptr() as *mut _)
                    );
                }
            }
            fn draw_geometry(&self, geometry: &impl IGeometry, brush: &impl IBrush, width: Option<f32>, stroke_style: Option<&StrokeStyle>) {
                unsafe {
                    self.0.DrawGeometry(
                        geometry.as_ptr() as *mut _,
                        brush.as_ptr() as *mut _,
                        width.unwrap_or(1.0),
                        stroke_style.map_or(std::ptr::null_mut(), |p| p.0.as_ptr() as *mut _)
                    );
                }
            }
            #[cfg(feature ="dwrite")]
            fn draw_glyph_run(&self, baseline_origin: impl Into<Point2F>, glyph_run: &crate::dwrite::GlyphRun, brush: &impl IBrush, mode: crate::dwrite::MeasuringMode) {
                let (glyph_run, _) = glyph_run.to_c_struct();
                unsafe {
                    self.0.DrawGlyphRun(
                        baseline_origin.into().into(),
                        &glyph_run,
                        brush.as_ptr() as *mut _,
                        mode as u32
                    );
                }
            }
            fn draw_line(&self, point0: impl Into<Point2F>, point1: impl Into<Point2F>, brush: &impl IBrush, width: Option<f32>, stroke_style: Option<&StrokeStyle>) {
                unsafe {
                    self.0.DrawLine(
                        point0.into().into(),
                        point1.into().into(),
                        brush.as_ptr() as *mut _,
                        width.unwrap_or(1.0),
                        stroke_style.map_or(std::ptr::null_mut(), |p| p.0.as_ptr() as *mut _)
                    );
                }
            }
            fn draw_rectangle(&self, rect: impl Into<RectF>, brush: &impl IBrush, width: Option<f32>, stroke_style: Option<&StrokeStyle>) {
                unsafe {
                    self.0.DrawRectangle(
                        rect.into().as_ref(),
                        brush.as_ptr() as *mut _,
                        width.unwrap_or(1.0),
                        stroke_style.map_or(std::ptr::null_mut(), |p| p.0.as_ptr() as *mut _)
                    );
                }
            }
            fn draw_rounded_rectangle(&self, round: &RoundedRect, brush: &impl IBrush, width: Option<f32>, stroke_style: Option<&StrokeStyle>) {
                unsafe {
                    self.0.DrawRoundedRectangle(
                        &round.to_c_struct(),
                        brush.as_ptr() as *mut _,
                        width.unwrap_or(1.0),
                        stroke_style.map_or(std::ptr::null_mut(), |p| p.0.as_ptr() as *mut _)
                    );
                }
            }
            #[cfg(feature = "dwrite")]
            fn draw_text(&self, string: impl AsRef<str>, format: &impl crate::dwrite::ITextFormat, rect: impl Into<RectF>, fill_brush: &impl IBrush, options: Option<DrawTextOptions>, measure_mode: Option<crate::dwrite::MeasuringMode>) {
                let string = string.as_ref().encode_utf16().chain(Some(0)).collect::<Vec<_>>();
                unsafe {
                    self.0.DrawText(
                        string.as_ptr(),
                        string.len() as u32,
                        format.as_ptr() as *mut _,
                        rect.into().as_ref(),
                        fill_brush.as_ptr() as *mut _,
                        options.map_or(DrawTextOptions::None.0, |o| o.0),
                        measure_mode.map_or(crate::dwrite::MeasuringMode::Natural as u32, |m| m as u32)
                    );
                }
            }
            #[cfg(feature = "dwrite")]
            fn draw_text_layout(&self, origin: impl Into<Point2F>, layout: &impl crate::dwrite::ITextLayout, fill_brush: &impl IBrush, options: Option<DrawTextOptions>) {
                unsafe {
                    self.0.DrawTextLayout(
                        origin.into().into(),
                        layout.as_ptr() as *mut _,
                        fill_brush.as_ptr() as *mut _,
                        options.map_or(DrawTextOptions::None.0, |o| o.0)
                    );
                }
            }
            fn end_draw(&self) -> Result<(Tag, Tag), HResult> {
                unsafe {
                    let mut tag1 = 0;
                    let mut tag2 = 0;
                    let ret = self.0.EndDraw(&mut tag1, &mut tag2);
                    hresult((Tag(tag1), Tag(tag2)), ret)
                }
            }
            fn fill_ellipse(&self, ellipse: &Ellipse, brush: &impl IBrush) {
                unsafe {
                    self.0.FillEllipse(
                        &ellipse.to_c_struct(),
                        brush.as_ptr() as *mut _
                    );
                }
            }
            fn fill_geometry(&self, geometry: &impl IGeometry, brush: &impl IBrush, opacity: Option<&Brush>) {
                unsafe {
                    self.0.FillGeometry(
                        geometry.as_ptr() as *mut _,
                        brush.as_ptr() as *mut _,
                        opacity.map_or(std::ptr::null_mut(), |p| p.0.as_ptr() as *mut _)
                    );
                }
            }
            fn fill_mesh(&self, mesh: &impl IMesh, brush: &impl IBrush) {
                unsafe {
                    self.0.FillMesh(
                        mesh.as_ptr() as *mut _,
                        brush.as_ptr() as *mut _
                    );
                }
            }
            fn fill_opacity_mask(&self, mask: &impl IBitmap, brush: &impl IBrush, content: OpacityMaskContent, dest_rect: Option<&RectF>, src_rect: Option<&RectF>) {
                unsafe {
                    self.0.FillOpacityMask(
                        mask.as_ptr() as *mut _,
                        brush.as_ptr() as *mut _,
                        content as u32,
                        dest_rect.map_or(std::ptr::null(), |rc| rc.as_ref()),
                        src_rect.map_or(std::ptr::null(), |rc| rc.as_ref())
                    );
                }
            }
            fn fill_rectangle(&self, rect: impl Into<RectF>, brush: &impl IBrush) {
                unsafe {
                    self.0.FillRectangle(
                        rect.into().as_ref(),
                        brush.as_ptr() as *mut _
                    );
                }
            }
            fn fill_rounded_rectangle(&self, rounded: &RoundedRect, brush: &impl IBrush) {
                unsafe {
                    self.0.FillRoundedRectangle(
                        &rounded.to_c_struct(),
                        brush.as_ptr() as *mut _
                    );
                }
            }
            fn flush(&self) -> Result<(Tag, Tag), HResult> {
                unsafe {
                    let mut tag1 = 0;
                    let mut tag2 = 0;
                    let ret = self.0.Flush(&mut tag1, &mut tag2);
                    hresult((Tag(tag1), Tag(tag2)), ret)
                }
            }
            fn get_antialias_mode(&self) -> AntialiasMode {
                unsafe { std::mem::transmute(self.0.GetAntialiasMode()) }
            }
            fn get_dpi(&self) -> Vector2F {
                unsafe {
                    let mut x = 0.0;
                    let mut y = 0.0;
                    self.0.GetDpi(&mut x, &mut y);
                    Vector2F::new(x, y)
                }
            }
            fn get_maximum_bitmap_size(&self) -> u32 {
                unsafe { self.0.GetMaximumBitmapSize() }
            }
            fn get_pixel_format(&self) -> PixelFormat {
                unsafe { self.0.GetPixelFormat().into() }
            }
            fn get_pixel_size(&self) -> SizeU {
                unsafe { self.0.GetPixelSize().into() }
            }
            fn get_size(&self) -> SizeF {
                unsafe { self.0.GetSize().into() }
            }
            fn get_tags(&self) -> (Tag, Tag) {
                unsafe {
                    let mut tag1 = 0;
                    let mut tag2 = 0;
                    self.0.GetTags(&mut tag1, &mut tag2);
                    (Tag(tag1), Tag(tag2))
                }
            }
            fn get_text_antialias_mode(&self) -> TextAntialiasMode {
                unsafe { std::mem::transmute(self.0.GetTextAntialiasMode()) }
            }
            #[cfg(feature = "dwrite")]
            fn get_text_rendering(&self) -> Option<crate::dwrite::RenderingParams> {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    self.0.GetTextRenderingParams(&mut p);
                    if p == std::ptr::null_mut() {
                        None
                    } else {
                        Some(crate::dwrite::RenderingParams(ComPtr::from_raw(p)))
                    }
                }
            }
            fn get_transform(&self) -> Matrix3x2F {
                unsafe {
                    let mut m = Default::default();
                    self.0.GetTransform(&mut m);
                    m.into()
                }
            }
            fn is_supported(&self, props: &RenderTargetProperties) -> bool {
                unsafe { self.0.IsSupported(&props.to_c_struct()) == TRUE }
            }
            fn pop_axis_aligned_clip(&self) {
                unsafe { self.0.PopAxisAlignedClip(); }
            }
            fn pop_layer(&self) {
                unsafe { self.0.PopLayer(); }
            }
            fn push_axis_aligned_clip(&self, rect: impl Into<RectF>, mode: AntialiasMode) {
                unsafe {
                    self.0.PushAxisAlignedClip(
                        rect.into().as_ref(),
                        mode as u32
                    );
                }
            }
            fn push_layer(&self, params: &LayerParameters, layer: Option<&Layer>) {
                unsafe {
                    self.0.PushLayer(
                        &params.to_c_struct(),
                        layer.map_or(std::ptr::null_mut(), |p| p.0.as_ptr())
                    );
                }
            }
            fn restore_drawing_state(&self, block: &impl IDrawingStateBlock) {
                unsafe { self.0.RestoreDrawingState(block.as_ptr() as *mut _); }
            }
            fn save_drawing_state(&self, block: &impl IDrawingStateBlock) {
                unsafe { self.0.SaveDrawingState(block.as_ptr() as *mut _); }
            }
            fn set_antialias_mode(&self, mode: AntialiasMode) {
                unsafe { self.0.SetAntialiasMode(mode as u32); }
            }
            fn set_dpi(&self, x: f32, y: f32) {
                unsafe { self.0.SetDpi(x, y); }
            }
            fn set_tags(&self, tag1: Tag, tag2: Tag) {
                unsafe { self.0.SetTags(tag1.0, tag2.0); }
            }
            #[cfg(feature = "dwrite")]
            fn set_text_antialias_mode(&self, mode: crate::dwrite::TextAntialiasMode) {
                unsafe { self.0.SetTextAntialiasMode(mode as u32); }
            }
            #[cfg(feature = "dwrite")]
            fn set_text_rendering_params(&self, params: Option<&crate::dwrite::RenderingParams>) {
                unsafe {
                    self.0.SetTextRenderingParams(
                        params.map_or(std::ptr::null_mut(), |p| p.0.as_ptr() as *mut _)
                    );
                }
            }
            fn set_transform(&self, m: &Matrix3x2F) {
                unsafe { self.0.SetTransform(&m.clone().into()); }
            }
        }
    };
}

macro_rules! impl_resource {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IResource for $s {
            fn get_factory(&self) -> Factory {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    self.0.GetFactory(&mut p);
                    Factory(ComPtr::from_raw(p))
                }
            }
        }
    };
}

macro_rules! impl_rounded_rectangle_geometry {
    ($s: ident, $interface: ident) => {
        impl_geometry!($s, $interface);
        impl IRoundedRectangleGeometry for $s {
            fn get_rounded_rect(&self) -> RoundedRect {
                unsafe {
                    let mut rc = Default::default();
                    self.0.GetRoundedRect(&mut rc);
                    rc.into()
                }
            }
        }
    };
}

macro_rules! impl_simplified_geometry_sink {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl ISimplifiedGeometrySink for $s {
            fn add_beziers(&self, beziers: &[BezierSegment]) {
                let beziers = beziers.iter().map(|b| b.to_c_struct()).collect::<Vec<_>>();
                unsafe {
                    self.0.AddBeziers(beziers.as_ptr(), beziers.len() as u32);
                }
            }
            fn add_lines(&self, points: &[Point2F]) {
                let points = points.iter().map(|p| p.clone().into()).collect::<Vec<_>>();
                unsafe {
                    self.0.AddLines(points.as_ptr(), points.len() as u32);
                }
            }
            fn begin_figure(&self, point: impl Into<Point2F>, figure: FigureBegin) {
                unsafe {
                    self.0.BeginFigure(
                        point.into().into(),
                        figure as u32
                    );
                }
            }
            fn close(&self) -> Result<(), HResult> {
                unsafe { hresult((), self.0.Close()) }
            }
            fn end_figure(&self, figure: FigureEnd) {
                unsafe { self.0.EndFigure(figure as u32); }
            }
            fn set_fill_mode(&self, mode: FillMode) {
                unsafe { self.0.SetFillMode(mode as u32); }
            }
            fn set_segment_flags(&self, flags: PathSegment) {
                unsafe { self.0.SetSegmentFlags(flags as u32); }
            }
        }
    };
}

macro_rules! impl_solid_color_brush {
    ($s: ident, $interface: ident) => {
        impl_brush!($s, $interface);
        impl ISolidColorBrush for $s {
            fn get_color(&self) -> ColorF {
                unsafe { self.0.GetColor().into() }
            }
            fn set_color(&self, color: impl Into<ColorF>) {
                unsafe { self.0.SetColor(&color.into().into()); }
            }
        }
    };
}

macro_rules! impl_stroke_style {
    ($s: ident, $interface: ident) => {
        impl_resource!($s, $interface);
        impl IStrokeStyle for $s {
            fn get_dash_cap(&self) -> CapStyle {
                unsafe { std::mem::transmute(self.0.GetDashCap()) }
            }
            fn get_dashes(&self) -> Vec<f32> {
                let len = self.get_dashes_count();
                unsafe {
                    let mut v = vec![0.0; len as usize];
                    self.0.GetDashes(v.as_mut_ptr(), len);
                    v
                }
            }
            fn get_dashes_count(&self) -> u32 {
                unsafe { self.0.GetDashesCount() }
            }
            fn get_dash_offset(&self) -> f32 {
                unsafe { self.0.GetDashOffset() }
            }
            fn get_dash_style(&self) -> DashStyle {
                unsafe { std::mem::transmute(self.0.GetDashStyle()) }
            }
            fn get_end_cap(&self) -> CapStyle {
                unsafe { std::mem::transmute(self.0.GetEndCap()) }
            }
            fn get_line_join(&self) -> LineJoin {
                unsafe { std::mem::transmute(self.0.GetLineJoin()) }
            }
            fn get_miter_limit(&self) -> f32 {
                unsafe { self.0.GetMiterLimit() }
            }
            fn get_start_cap(&self) -> CapStyle {
                unsafe { std::mem::transmute(self.0.GetStartCap()) }
            }
        }
    };
}

macro_rules! impl_tesselation_sink {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl ITessellationSink for $s {
            fn add_triangles(&self, triangles: &[Triangle]) {
                let triangles = triangles.iter().map(|t| t.to_c_struct()).collect::<Vec<_>>();
                unsafe { self.0.AddTriangles(triangles.as_ptr(), triangles.len() as u32); }
            }
            fn close(&self) -> Result<(), HResult> {
                unsafe { hresult((), self.0.Close()) }
            }
        }
    };
}

macro_rules! impl_transformed_geometry {
    ($s: ident, $interface: ident) => {
        impl_geometry!($s, $interface);
        impl ITransformedGeometry for $s {
            fn get_source_geometry(&self) -> Geometry {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    self.0.GetSourceGeometry(&mut p);
                    Geometry(ComPtr::from_raw(p))
                }
            }
            fn get_transform(&self) -> Matrix3x2F {
                unsafe {
                    let mut m = Default::default();
                    self.0.GetTransform(&mut m);
                    m.into()
                }
            }
        }
    };
}

pub const DEFAULT_FLATTENING_TOLERANCE: f32 = 0.25;

pub trait IBitmap: IImage {
    fn copy_from_bitmap(
        &self,
        dest_point: impl Into<Point2U>,
        bitmap: &impl IBitmap,
        src_rect: impl Into<RectU>,
    ) -> Result<(), HResult>;
    unsafe fn copy_from_memory<T>(
        &self,
        dest_rect: impl Into<RectU>,
        src_data: *const T,
        pitch: u32,
    ) -> Result<(), HResult>;
    fn copy_from_render_target(
        &self,
        dest_point: impl Into<Point2U>,
        render_target: &impl IRenderTarget,
        src_rect: impl Into<RectU>,
    ) -> Result<(), HResult>;
    fn get_dpi(&self) -> Vector2F;
    fn get_pixel_format(&self) -> PixelFormat;
    fn get_pixel_size(&self) -> SizeU;
    fn get_size(&self) -> SizeF;
}

#[derive(Clone, Debug)]
pub struct Bitmap(ComPtr<ID2D1Bitmap>);
impl_bitmap!(Bitmap, ID2D1Bitmap);

pub trait IBitmapBrush: IBrush {
    fn get_bitmap(&self) -> Option<Bitmap>;
    fn get_extend_mode_x(&self) -> ExtendMode;
    fn get_extend_mode_y(&self) -> ExtendMode;
    fn get_interpolation_mode(&self) -> BitmapInterpolationMode;
    fn set_bitmap(&self, bitmap: &impl IBitmap);
    fn set_extend_mode_x(&self, mode: ExtendMode);
    fn set_extend_mode_y(&self, mode: ExtendMode);
    fn set_interpolation_mode(&self, mode: BitmapInterpolationMode);
}

#[derive(Clone, Debug)]
pub struct BitmapBrush(ComPtr<ID2D1BitmapBrush>);
impl_bitmap_brush!(BitmapBrush, ID2D1BitmapBrush);

pub trait IBitmapRenderTarget: IRenderTarget {
    fn get_bitmap(&self) -> Result<Bitmap, HResult>;
}

#[derive(Clone, Debug)]
pub struct BitmapRenderTarget(ComPtr<ID2D1BitmapRenderTarget>);
impl_bitmap_render_target!(BitmapRenderTarget, ID2D1BitmapRenderTarget);

pub trait IBrush: IResource {
    fn get_opacity(&self) -> f32;
    fn get_transform(&self) -> Matrix3x2F;
    fn set_opacity(&self, opacity: f32);
    fn set_transform(&self, m: &Matrix3x2F);
}

#[derive(Clone, Debug)]
pub struct Brush(ComPtr<ID2D1Brush>);
impl_brush!(Brush, ID2D1Brush);

pub trait IDCRenderTarget: IRenderTarget {
    fn bind_dc(
        &self,
        hdc: &impl crate::api::DeviceContextHandle,
        sub_rect: impl Into<RectL>,
    ) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct DCRenderTarget(ComPtr<ID2D1DCRenderTarget>);
impl_dc_render_target!(DCRenderTarget, ID2D1DCRenderTarget);

pub trait IDrawingStateBlock: IResource {
    fn get_description(&self) -> DrawingStateDescription;
    #[cfg(feature = "dwrite")]
    fn get_text_rendering_params(&self) -> Option<crate::dwrite::RenderingParams>;
    fn set_description(&self, desc: &DrawingStateDescription);
    #[cfg(feature = "dwrite")]
    fn set_text_rendering_params(&self, params: &impl crate::dwrite::IRenderingParams);
}

#[derive(Clone, Debug)]
pub struct DrawingStateBlock(ComPtr<ID2D1DrawingStateBlock>);
impl_drawing_state_block!(DrawingStateBlock, ID2D1DrawingStateBlock);

pub trait IEllipseGeometry: IGeometry {
    fn get_ellipse(&self) -> Ellipse;
}

#[derive(Clone, Debug)]
pub struct EllipseGeometry(ComPtr<ID2D1EllipseGeometry>);
impl_ellipse_geometry!(EllipseGeometry, ID2D1EllipseGeometry);

pub trait IFactory: Interface {
    fn create_dc_render_target(
        &self,
        props: &RenderTargetProperties,
    ) -> Result<DCRenderTarget, HResult>;
    #[cfg(feature = "dwrite")]
    fn create_drawing_state_block(
        &self,
        desc: &DrawingStateDescription,
        params: Option<&crate::dwrite::RenderingParams>,
    ) -> Result<DrawingStateBlock, HResult>;
    #[cfg(feature = "dxgi")]
    fn create_dxgi_surface_render_target(
        &self,
        surface: &impl dxgi::ISurface,
        props: &RenderTargetProperties,
    ) -> Result<RenderTarget, HResult>;
    fn create_ellipse_geometry(&self, ellipse: &Ellipse) -> Result<EllipseGeometry, HResult>;
    fn create_geometry_group(
        &self,
        fill_mode: FillMode,
        geometries: &[&impl IGeometry],
    ) -> Result<GeometryGroup, HResult>;
    fn create_hwnd_render_target(
        &self,
        props: &RenderTargetProperties,
        hwnd_props: &HwndRenderTargetPropertices,
    ) -> Result<HwndRenderTarget, HResult>;
    fn craete_path_geometry(&self) -> Result<PathGeometry, HResult>;
    fn create_rectangle_geometry(&self, rc: impl Into<RectF>)
        -> Result<RectangleGeometry, HResult>;
    fn create_rounded_rectangle_geometry(
        &self,
        rc: &RoundedRect,
    ) -> Result<RoundedRectangleGeometry, HResult>;
    fn create_stroke_style(
        &self,
        props: &StrokeStyleProperties,
        dashes: Option<&[f32]>,
    ) -> Result<StrokeStyle, HResult>;
    fn create_transformed_geometry(
        &self,
        src: &impl IGeometry,
        transform: Option<&Matrix3x2F>,
    ) -> Result<TransformedGeometry, HResult>;
    // fn create_wic_bitmap_render_target;
    fn get_desktop_dpi(&self) -> Vector2F;
    fn reload_system_metrics(&self) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct Factory(ComPtr<ID2D1Factory>);
impl_factory!(Factory, ID2D1Factory);

pub trait IGdiInteropRenderTarget: Interface {
    fn get_dc(&self, mode: DCInitializeMode) -> Result<HDC, HResult>;
    fn release_dc(&self, update: impl Into<crate::api::Rect<i32>>) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct GdiInteropRenderTarget(ComPtr<ID2D1GdiInteropRenderTarget>);
impl_gdi_interop_render_target!(GdiInteropRenderTarget, ID2D1GdiInteropRenderTarget);

#[derive(Clone, Debug)]
pub struct ComputePointAtLengthResult {
    pub point: Point2F,
    pub unit_tangent_vector: Point2F,
}

pub trait IGeometry: IResource {
    fn combine_with_geometry(
        &self,
        input: &impl IGeometry,
        mode: CombineMode,
        transform: Option<&Matrix3x2F>,
        tolerance: f32,
        sink: &impl ISimplifiedGeometrySink,
    ) -> Result<(), HResult>;
    fn compute_area(&self, transform: Option<&Matrix3x2F>, tolerance: f32) -> Result<f32, HResult>;
    fn compute_length(
        &self,
        transform: Option<&Matrix3x2F>,
        tolerance: f32,
    ) -> Result<f32, HResult>;
    fn compute_point_at_length(
        &self,
        length: f32,
        transform: Option<&Matrix3x2F>,
        tolerance: f32,
    ) -> Result<ComputePointAtLengthResult, HResult>;
    fn fill_contains_point(
        &self,
        point: impl Into<Point2F>,
        transform: Option<&Matrix3x2F>,
        tolerance: f32,
    ) -> Result<bool, HResult>;
    fn get_bounds(&self, transform: Option<&Matrix3x2F>) -> Result<RectF, HResult>;
    fn get_widened_bounds(
        &self,
        width: f32,
        stroke_style: &impl IStrokeStyle,
        transform: Option<&Matrix3x2F>,
        tolerance: f32,
    ) -> Result<RectF, HResult>;
    fn outline(
        &self,
        transform: Option<&Matrix3x2F>,
        tolerance: f32,
        sink: &impl ISimplifiedGeometrySink,
    ) -> Result<(), HResult>;
    fn simplify(
        &self,
        option: GeometrySimplificationOption,
        transform: Option<&Matrix3x2F>,
        tolerance: f32,
        sink: &impl ISimplifiedGeometrySink,
    ) -> Result<(), HResult>;
    fn stroke_contains_point(
        &self,
        point: impl Into<Point2F>,
        width: f32,
        stroke_style: &impl IStrokeStyle,
        transform: Option<&Matrix3x2F>,
        tolerance: f32,
    ) -> Result<bool, HResult>;
    fn tessellate(
        &self,
        transform: Option<&Matrix3x2F>,
        tolerance: f32,
        sink: &impl ITessellationSink,
    ) -> Result<(), HResult>;
    fn widen(
        &self,
        width: f32,
        stroke_style: &impl IStrokeStyle,
        transform: Option<&Matrix3x2F>,
        tolerance: f32,
        sink: &impl ISimplifiedGeometrySink,
    ) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct Geometry(ComPtr<ID2D1Geometry>);
impl_geometry!(Geometry, ID2D1Geometry);

pub trait IGeometryGroup: IGeometry {
    fn get_fill_mode(&self) -> FillMode;
    fn get_source_geometries(&self) -> Vec<Geometry>;
    fn get_source_geometry_count(&self) -> u32;
}

#[derive(Clone, Debug)]
pub struct GeometryGroup(ComPtr<ID2D1GeometryGroup>);
impl_geometry_group!(GeometryGroup, ID2D1GeometryGroup);

pub trait IGeometrySink: ISimplifiedGeometrySink {
    fn add_arc(&self, arc: &ArcSegment);
    fn add_bezier(&self, bezier: &BezierSegment);
    fn add_line(&self, point: impl Into<Point2F>);
    fn add_quadratic_bezier(&self, bezier: &QuadraticBezierSegment);
    fn add_quadratic_beziers(&self, beziers: &[&QuadraticBezierSegment]);
}

#[derive(Clone, Debug)]
pub struct GeometrySink(ComPtr<ID2D1GeometrySink>);
impl_geometry_sink!(GeometrySink, ID2D1GeometrySink);

pub trait IGradientStopCollection: IResource {
    fn get_color_interpolation_gamma(&self) -> Gamma;
    fn get_extend_mode(&self) -> ExtendMode;
    fn get_gradient_stop_count(&self) -> u32;
    fn get_gradient_stops(&self) -> Vec<GradientStop>;
}

#[derive(Clone, Debug)]
pub struct GradientStopCollection(ComPtr<ID2D1GradientStopCollection>);
impl_gradient_stop_collection!(GradientStopCollection, ID2D1GradientStopCollection);

pub trait IHwndRenderTarget: IRenderTarget {
    fn check_window_state(&self) -> WindowState;
    fn get_hwnd(&self) -> HWND;
    fn resize(&self, size: impl Into<SizeU>) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct HwndRenderTarget(ComPtr<ID2D1HwndRenderTarget>);
impl_hwnd_render_target!(HwndRenderTarget, ID2D1HwndRenderTarget);

pub trait IImage: IResource {}

#[derive(Clone, Debug)]
pub struct Image(ComPtr<ID2D1Image>);
impl_image!(Image, ID2D1Image);

pub trait ILayer: IResource {
    fn get_size(&self) -> SizeF;
}

#[derive(Clone, Debug)]
pub struct Layer(ComPtr<ID2D1Layer>);
impl_layer!(Layer, ID2D1Layer);

pub trait ILinearGradientBrush: IBrush {
    fn get_end_point(&self) -> Point2F;
    fn get_gradient_stop_collection(&self) -> GradientStopCollection;
    fn get_start_point(&self) -> Point2F;
    fn set_end_point(&self, point: impl Into<Point2F>);
    fn set_start_point(&self, point: impl Into<Point2F>);
}

#[derive(Clone, Debug)]
pub struct LinearGradientBrush(ComPtr<ID2D1LinearGradientBrush>);
impl_linear_gradient_brush!(LinearGradientBrush, ID2D1LinearGradientBrush);

pub trait IMesh: IResource {
    fn open(&self) -> Result<TessellationSink, HResult>;
}

#[derive(Clone, Debug)]
pub struct Mesh(ComPtr<ID2D1Mesh>);
impl_mesh!(Mesh, ID2D1Mesh);

pub trait IPathGeometry: IGeometry {
    fn get_figure_count(&self) -> Result<u32, HResult>;
    fn get_segment_count(&self) -> Result<u32, HResult>;
    fn open(&self) -> Result<GeometrySink, HResult>;
    fn stream(&self, sink: &impl IGeometrySink) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct PathGeometry(ComPtr<ID2D1PathGeometry>);
impl_path_geometry!(PathGeometry, ID2D1PathGeometry);

pub trait IRadialGradientBrush: IBrush {
    fn get_center(&self) -> Point2F;
    fn get_gradient_origin_offset(&self) -> Point2F;
    fn get_gradient_stop_collection(&self) -> GradientStopCollection;
    fn get_radius_x(&self) -> f32;
    fn get_radius_y(&self) -> f32;
    fn set_center(&self, center: impl Into<Point2F>);
    fn set_gradient_origin_offset(&self, offset: impl Into<Point2F>);
    fn set_radius_x(&self, x: f32);
    fn set_radius_y(&self, y: f32);
}

#[derive(Clone, Debug)]
pub struct RadialGradientBrush(ComPtr<ID2D1RadialGradientBrush>);
impl_radial_gradient_brush!(RadialGradientBrush, ID2D1RadialGradientBrush);

pub trait IRectangleGeometry: IGeometry {
    fn get_rect(&self) -> RectF;
}

#[derive(Clone, Debug)]
pub struct RectangleGeometry(ComPtr<ID2D1RectangleGeometry>);
impl_rectangle_geometry!(RectangleGeometry, ID2D1RectangleGeometry);

pub trait IRenderTarget: IResource {
    fn begin_draw(&self);
    fn clear(&self, color: impl Into<ColorF>);
    unsafe fn create_bitmap(
        &self,
        size: impl Into<SizeU>,
        src_data: Option<&[u8]>,
        pitch: u32,
        props: &BitmapProperties,
    ) -> Result<Bitmap, HResult>;
    fn create_bitmap_brush(
        &self,
        bitmap: &impl IBitmap,
        props: Option<&BitmapBrushProperties>,
        brush_props: Option<&BrushProperties>,
    ) -> Result<BitmapBrush, HResult>;
    // fn create_bitmap_from_wic_bitmap;
    fn create_compatible_render_target(
        &self,
        size: Option<SizeF>,
        pixel_size: Option<SizeU>,
        format: Option<&PixelFormat>,
        options: CompatibleRenderTargetOptions,
    ) -> Result<BitmapRenderTarget, HResult>;
    fn create_gradient_stop_collection(
        &self,
        stops: &[GradientStop],
        gamma: Gamma,
        mode: ExtendMode,
    ) -> Result<GradientStopCollection, HResult>;
    fn create_layer(&self, size: Option<SizeF>) -> Result<Layer, HResult>;
    fn create_linear_gradient_brush(
        &self,
        props: &LinearGradientBrushProperties,
        brush_props: Option<&BrushProperties>,
        gradient: &impl IGradientStopCollection,
    ) -> Result<LinearGradientBrush, HResult>;
    fn create_mesh(&self) -> Result<Mesh, HResult>;
    fn create_radial_gradient_brush(
        &self,
        props: &RadialGradientBrushProperties,
        brush_props: Option<&BrushProperties>,
        gradient: &impl IGradientStopCollection,
    ) -> Result<RadialGradientBrush, HResult>;
    // fn create_shared_bitmap;
    fn create_solid_color_brush(
        &self,
        color: impl Into<ColorF>,
        brush_props: Option<&BrushProperties>,
    ) -> Result<SolidColorBrush, HResult>;
    fn draw_bitmap(
        &self,
        bitmap: &impl IBitmap,
        dest_rect: Option<&RectF>,
        opacity: Option<f32>,
        interpolation: Option<BitmapInterpolationMode>,
        src_rect: Option<&RectF>,
    );
    fn draw_ellipse(
        &self,
        ellipse: &Ellipse,
        brush: &impl IBrush,
        width: Option<f32>,
        stroke_style: Option<&StrokeStyle>,
    );
    fn draw_geometry(
        &self,
        geometry: &impl IGeometry,
        brush: &impl IBrush,
        width: Option<f32>,
        stroke_style: Option<&StrokeStyle>,
    );
    #[cfg(feature = "dwrite")]
    fn draw_glyph_run(
        &self,
        baseline_origin: impl Into<Point2F>,
        glyph_run: &crate::dwrite::GlyphRun,
        brush: &impl IBrush,
        mode: crate::dwrite::MeasuringMode,
    );
    fn draw_line(
        &self,
        point0: impl Into<Point2F>,
        point1: impl Into<Point2F>,
        brush: &impl IBrush,
        width: Option<f32>,
        stroke_style: Option<&StrokeStyle>,
    );
    fn draw_rectangle(
        &self,
        rect: impl Into<RectF>,
        brush: &impl IBrush,
        width: Option<f32>,
        stroke_style: Option<&StrokeStyle>,
    );
    fn draw_rounded_rectangle(
        &self,
        round: &RoundedRect,
        brush: &impl IBrush,
        width: Option<f32>,
        stroke_style: Option<&StrokeStyle>,
    );
    #[cfg(feature = "dwrite")]
    fn draw_text(
        &self,
        string: impl AsRef<str>,
        format: &impl crate::dwrite::ITextFormat,
        rect: impl Into<RectF>,
        fill_brush: &impl IBrush,
        options: Option<DrawTextOptions>,
        measure_mode: Option<crate::dwrite::MeasuringMode>,
    );
    #[cfg(feature = "dwrite")]
    fn draw_text_layout(
        &self,
        origin: impl Into<Point2F>,
        layout: &impl crate::dwrite::ITextLayout,
        fill_brush: &impl IBrush,
        options: Option<DrawTextOptions>,
    );
    fn end_draw(&self) -> Result<(Tag, Tag), HResult>;
    fn fill_ellipse(&self, ellipse: &Ellipse, brush: &impl IBrush);
    fn fill_geometry(&self, geometry: &impl IGeometry, brush: &impl IBrush, opacity: Option<&Brush>);
    fn fill_mesh(&self, mesh: &impl IMesh, brush: &impl IBrush);
    fn fill_opacity_mask(
        &self,
        mask: &impl IBitmap,
        brush: &impl IBrush,
        content: OpacityMaskContent,
        dest_rect: Option<&RectF>,
        src_rect: Option<&RectF>,
    );
    fn fill_rectangle(&self, rect: impl Into<RectF>, brush: &impl IBrush);
    fn fill_rounded_rectangle(&self, rounded: &RoundedRect, brush: &impl IBrush);
    fn flush(&self) -> Result<(Tag, Tag), HResult>;
    fn get_antialias_mode(&self) -> AntialiasMode;
    fn get_dpi(&self) -> Vector2F;
    fn get_maximum_bitmap_size(&self) -> u32;
    fn get_pixel_format(&self) -> PixelFormat;
    fn get_pixel_size(&self) -> SizeU;
    fn get_size(&self) -> SizeF;
    fn get_tags(&self) -> (Tag, Tag);
    fn get_text_antialias_mode(&self) -> TextAntialiasMode;
    #[cfg(feature = "dwrite")]
    fn get_text_rendering(&self) -> Option<crate::dwrite::RenderingParams>;
    fn get_transform(&self) -> Matrix3x2F;
    fn is_supported(&self, props: &RenderTargetProperties) -> bool;
    fn pop_axis_aligned_clip(&self);
    fn pop_layer(&self);
    fn push_axis_aligned_clip(&self, rect: impl Into<RectF>, mode: AntialiasMode);
    fn push_layer(&self, params: &LayerParameters, layer: Option<&Layer>);
    fn restore_drawing_state(&self, block: &impl IDrawingStateBlock);
    fn save_drawing_state(&self, block: &impl IDrawingStateBlock);
    fn set_antialias_mode(&self, mode: AntialiasMode);
    fn set_dpi(&self, x: f32, y: f32);
    fn set_tags(&self, tag1: Tag, tag2: Tag);
    #[cfg(feature = "dwrite")]
    fn set_text_antialias_mode(&self, mode: crate::dwrite::TextAntialiasMode);
    #[cfg(feature = "dwrite")]
    fn set_text_rendering_params(&self, params: Option<&crate::dwrite::RenderingParams>);
    fn set_transform(&self, m: &Matrix3x2F);
}

#[derive(Clone, Debug)]
pub struct RenderTarget(ComPtr<ID2D1RenderTarget>);
impl_render_target!(RenderTarget, ID2D1RenderTarget);

pub trait IResource: Interface {
    fn get_factory(&self) -> Factory;
}

#[derive(Clone, Debug)]
pub struct Resource(ComPtr<ID2D1Resource>);
impl_resource!(Resource, ID2D1Resource);

pub trait IRoundedRectangleGeometry: IGeometry {
    fn get_rounded_rect(&self) -> RoundedRect;
}

#[derive(Clone, Debug)]
pub struct RoundedRectangleGeometry(ComPtr<ID2D1RoundedRectangleGeometry>);
impl_rounded_rectangle_geometry!(RoundedRectangleGeometry, ID2D1RoundedRectangleGeometry);

pub trait ISimplifiedGeometrySink: Interface {
    fn add_beziers(&self, beziers: &[BezierSegment]);
    fn add_lines(&self, points: &[Point2F]);
    fn begin_figure(&self, point: impl Into<Point2F>, figure: FigureBegin);
    fn close(&self) -> Result<(), HResult>;
    fn end_figure(&self, figure: FigureEnd);
    fn set_fill_mode(&self, mode: FillMode);
    fn set_segment_flags(&self, flags: PathSegment);
}

#[derive(Clone, Debug)]
pub struct SimplifiedGeometrySink(ComPtr<ID2D1SimplifiedGeometrySink>);
impl_simplified_geometry_sink!(SimplifiedGeometrySink, ID2D1SimplifiedGeometrySink);

pub trait ISolidColorBrush: IBrush {
    fn get_color(&self) -> ColorF;
    fn set_color(&self, color: impl Into<ColorF>);
}

#[derive(Clone, Debug)]
pub struct SolidColorBrush(ComPtr<ID2D1SolidColorBrush>);
impl_solid_color_brush!(SolidColorBrush, ID2D1SolidColorBrush);

pub trait IStrokeStyle: IResource {
    fn get_dash_cap(&self) -> CapStyle;
    fn get_dashes(&self) -> Vec<f32>;
    fn get_dashes_count(&self) -> u32;
    fn get_dash_offset(&self) -> f32;
    fn get_dash_style(&self) -> DashStyle;
    fn get_end_cap(&self) -> CapStyle;
    fn get_line_join(&self) -> LineJoin;
    fn get_miter_limit(&self) -> f32;
    fn get_start_cap(&self) -> CapStyle;
}

#[derive(Clone, Debug)]
pub struct StrokeStyle(ComPtr<ID2D1StrokeStyle>);
impl_stroke_style!(StrokeStyle, ID2D1StrokeStyle);

pub trait ITessellationSink: Interface {
    fn add_triangles(&self, triangles: &[Triangle]);
    fn close(&self) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct TessellationSink(ComPtr<ID2D1TessellationSink>);
impl_tesselation_sink!(TessellationSink, ID2D1TessellationSink);

pub trait ITransformedGeometry: IGeometry {
    fn get_source_geometry(&self) -> Geometry;
    fn get_transform(&self) -> Matrix3x2F;
}

#[derive(Clone, Debug)]
pub struct TransformedGeometry(ComPtr<ID2D1TransformedGeometry>);
impl_transformed_geometry!(TransformedGeometry, ID2D1TransformedGeometry);
