use winapi::um::d2d1_3::*;

/*
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
pub enum ColorBitmapGlyphSnapOption {
    Default = D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DEFAULT,
    Disable = D2D1_COLOR_BITMAP_GLYPH_SNAP_OPTION_DISABLE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorContextType {
    ICC = D2D1_COLOR_CONTEXT_TYPE_ICC,
    Simple = D2D1_COLOR_CONTEXT_TYPE_SIMPLE,
    DXGI = D2D1_COLOR_CONTEXT_TYPE_DXGI,
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ContractProp {
    Contrast = D2D1_CONTRAST_PROP_CONTRAST,
    ClampInput = D2D1_CONTRAST_PROP_CLAMP_INPUT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum EdgeDetectionMode {
    Sobel = D2D1_EDGEDETECTION_MODE_SOBEL,
    Prewitt = D2D1_EDGEDETECTION_MODE_PREWITT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum EdgeDetectionProp {
    Strength = D2D1_EDGEDETECTION_PROP_STRENGTH,
    BlurRadius = D2D1_EDGEDETECTION_PROP_BLUR_RADIUS,
    Mode = D2D1_EDGEDETECTION_PROP_MODE,
    OverlayEdges = D2D1_EDGEDETECTION_PROP_OVERLAY_EDGES,
    AlphaMode = D2D1_EDGEDETECTION_PROP_ALPHA_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum EmbossProp {
    Height = D2D1_EMBOSS_PROP_HEIGHT,
    Direction = D2D1_EMBOSS_PROP_DIRECTION,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ExposureProp {
    ExposureValue = D2D1_EXPOSURE_PROP_EXPOSURE_VALUE,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Gamma1 {
    G22 = D2D1_GAMMA1_G22,
    G10 = D2D1_GAMMA1_G10,
    G2084 = D2D1_GAMMA1_G2084,
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HDRToneMapDisplayMode {
    SDR = D2D1_HDRTONEMAP_DISPLAY_MODE_SDR,
    HDR = D2D1_HDRTONEMAP_DISPLAY_MODE_HDR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HDRToneMapProp {
    InputMaxLuminance = D2D1_HDRTONEMAP_PROP_INPUT_MAX_LUMINANCE,
    OutputMaxLuminance = D2D1_HDRTONEMAP_PROP_OUTPUT_MAX_LUMINANCE,
    DisplayMode = D2D1_HDRTONEMAP_PROP_DISPLAY_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HighlightsAndShadowsInputGamma {
    Linear = D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_LINEAR,
    SRGB = D2D1_HIGHLIGHTSANDSHADOWS_INPUT_GAMMA_SRGB,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HighlightsAndShadowsProp {
    Highlights = D2D1_HIGHLIGHTSANDSHADOWS_PROP_HIGHLIGHTS,
    Shadows = D2D1_HIGHLIGHTSANDSHADOWS_PROP_SHADOWS,
    Clarity = D2D1_HIGHLIGHTSANDSHADOWS_PROP_CLARITY,
    InputGamma = D2D1_HIGHLIGHTSANDSHADOWS_PROP_INPUT_GAMMA,
    MaskBlurRadius = D2D1_HIGHLIGHTSANDSHADOWS_PROP_MASK_BLUR_RADIUS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HueToRGBInputColorSpace {
    HueSaturationValue = D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_VALUE,
    HueSaturationLightness = D2D1_HUETORGB_INPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum HueToRGBProp {
    InputColorSpace = D2D1_HUETORGB_PROP_INPUT_COLOR_SPACE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ImageSourceFromDXGIOptions {
    None = D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_NONE,
    LowQualityPrimaryConversion = D2D1_IMAGE_SOURCE_FROM_DXGI_OPTIONS_LOW_QUALITY_PRIMARY_CONVERSION,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ImageSourceLoadingOptions {
    None = D2D1_IMAGE_SOURCE_LOADING_OPTIONS_NONE,
    ReleaseSource = D2D1_IMAGE_SOURCE_LOADING_OPTIONS_RELEASE_SOURCE,
    CacheOnDepend = D2D1_IMAGE_SOURCE_LOADING_OPTIONS_CACHE_ON_DEMAND,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum InkNibShape {
    Round = D2D1_INK_NIB_SHAPE_ROUND,
    Square = D2D1_INK_NIB_SHAPE_SQUARE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LookupTable3DProp {
    LUT = D2D1_LOOKUPTABLE3D_PROP_LUT,
    AlphaMode = D2D1_LOOKUPTABLE3D_PROP_ALPHA_MODE,
}

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PatchEdgeMode {
    Aliased = D2D1_PATCH_EDGE_MODE_ALIASED,
    Antialiased = D2D1_PATCH_EDGE_MODE_ANTIALIASED,
    AliasedInflated = D2D1_PATCH_EDGE_MODE_ALIASED_INFLATED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PosterizeProp {
    RedValueCount = D2D1_POSTERIZE_PROP_RED_VALUE_COUNT,
    GreenValueCount = D2D1_POSTERIZE_PROP_GREEN_VALUE_COUNT,
    BlueValueCount = D2D1_POSTERIZE_PROP_BLUE_VALUE_COUNT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RGBToHueOutputColorSpace {
    HueSaturationValue = D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_VALUE,
    HueSaturationLightness = D2D1_RGBTOHUE_OUTPUT_COLOR_SPACE_HUE_SATURATION_LIGHTNESS,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RGBToHueProp {
    OutputColorSpace = D2D1_RGBTOHUE_PROP_OUTPUT_COLOR_SPACE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SepiaProp {
    Intensity = D2D1_SEPIA_PROP_INTENSITY,
    AlphaMode = D2D1_SEPIA_PROP_ALPHA_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Sharpen_prop {
    Sharpness = D2D1_SHARPEN_PROP_SHARPNESS,
    Threashold = D2D1_SHARPEN_PROP_THRESHOLD,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum SpriteOptions {
    None = D2D1_SPRITE_OPTIONS_NONE,
    ClampToSourceRectangle = D2D1_SPRITE_OPTIONS_CLAMP_TO_SOURCE_RECTANGLE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StraightenProp {
    Angle = D2D1_STRAIGHTEN_PROP_ANGLE,
    MaintainSize = D2D1_STRAIGHTEN_PROP_MAINTAIN_SIZE,
    ScaleMode = D2D1_STRAIGHTEN_PROP_SCALE_MODE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StraightenScaleMode {
    NearestNeighbor = D2D1_STRAIGHTEN_SCALE_MODE_NEAREST_NEIGHBOR,
    Linear = D2D1_STRAIGHTEN_SCALE_MODE_LINEAR,
    Cubic = D2D1_STRAIGHTEN_SCALE_MODE_CUBIC,
    MultiSampleLinear = D2D1_STRAIGHTEN_SCALE_MODE_MULTI_SAMPLE_LINEAR,
    Anisotropic = D2D1_STRAIGHTEN_SCALE_MODE_ANISOTROPIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TemperatureAndTintProp {
    Temperature = D2D1_TEMPERATUREANDTINT_PROP_TEMPERATURE,
    Tint = D2D1_TEMPERATUREANDTINT_PROP_TINT,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TransformedImageSourceOptions {
    None = D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_NONE,
    DisableDPIScale = D2D1_TRANSFORMED_IMAGE_SOURCE_OPTIONS_DISABLE_DPI_SCALE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VignetteProp {
    Color = D2D1_VIGNETTE_PROP_COLOR,
    TransitionSize = D2D1_VIGNETTE_PROP_TRANSITION_SIZE,
    Strength = D2D1_VIGNETTE_PROP_STRENGTH,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum WhiteLevelAdjustmentProp {
    InputWhiteLevel = D2D1_WHITELEVELADJUSTMENT_PROP_INPUT_WHITE_LEVEL,
    OutputWhiteLevel = D2D1_WHITELEVELADJUSTMENT_PROP_OUTPUT_WHITE_LEVEL,
}

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

#[derive(Clone, Debug)]
pub struct InkPoint {
    pub x: f32,
    pub y: f32,
    pub radius: f32,
}

#[derive(Clone, Debug)]
pub struct SimpleColorProfile {
    pub red_primary: Point2F,
    pub green_primary: Point2F,
    pub blue_primary: Point2F,
    pub white_point_xz: Point2F,
    pub gamma: Gamma1,
}
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

#[derive(Clone, Debug)]
pub struct TransformedImageSourceProperties {
    pub orientation: Orientation,
    pub scale_x: f32,
    pub scale_y: f32,
    pub interpolation_mode: InterpolationMode,
    pub options: Option<TransformedImageSourceOptions>,
}
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
