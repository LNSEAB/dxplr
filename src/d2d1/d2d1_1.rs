#![allow(dead_code)]

use super::d2d1::*;
use crate::impl_bitflag_operators;
use winapi::um::d2d1_1::*;
use winapi::um::d2d1effectauthor::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct BitmapOptions(u32);
#[allow(non_upper_case_globals)]
impl BitmapOptions {
    pub const None: Self = Self(D2D1_BITMAP_OPTIONS_NONE);
    pub const Target: Self = Self(D2D1_BITMAP_OPTIONS_TARGET);
    pub const CannotDraw: Self = Self(D2D1_BITMAP_OPTIONS_CANNOT_DRAW);
    pub const CPURead: Self = Self(D2D1_BITMAP_OPTIONS_CPU_READ);
    pub const GDICompatible: Self = Self(D2D1_BITMAP_OPTIONS_GDI_COMPATIBLE);
}
impl_bitflag_operators!(BitmapOptions);

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
pub enum ColorSpace {
    Custom = D2D1_COLOR_SPACE_CUSTOM,
    SRGB = D2D1_COLOR_SPACE_SRGB,
    ScRGB = D2D1_COLOR_SPACE_SCRGB,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ColorInterpolationMode {
    Straight = D2D1_COLOR_INTERPOLATION_MODE_STRAIGHT,
    Premultiplied = D2D1_COLOR_INTERPOLATION_MODE_PREMULTIPLIED,
}

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
pub enum DeviceContextOptions {
    None = D2D1_DEVICE_CONTEXT_OPTIONS_NONE,
    EnableMultiThreadedOptimizations =
        D2D1_DEVICE_CONTEXT_OPTIONS_ENABLE_MULTITHREADED_OPTIMIZATIONS,
}

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
pub enum LayerOptions1 {
    None = D2D1_LAYER_OPTIONS1_NONE,
    InitializeFromBackground = D2D1_LAYER_OPTIONS1_INITIALIZE_FROM_BACKGROUND,
    IgnoreAlpha = D2D1_LAYER_OPTIONS1_IGNORE_ALPHA,
}

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
pub enum PrimitiveBlend {
    SourceOver = D2D1_PRIMITIVE_BLEND_SOURCE_OVER,
    Copy = D2D1_PRIMITIVE_BLEND_COPY,
    Min = D2D1_PRIMITIVE_BLEND_MIN,
    Add = D2D1_PRIMITIVE_BLEND_ADD,
    // Max = D2D1_PRIMITIVE_BLEND_MAX,
    Max = 4,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PrintFontSubsetMode {
    Default = D2D1_PRINT_FONT_SUBSET_MODE_DEFAULT,
    EachPage = D2D1_PRINT_FONT_SUBSET_MODE_EACHPAGE,
    None = D2D1_PRINT_FONT_SUBSET_MODE_NONE,
}

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum StrokeTransformType {
    Normal = D2D1_STROKE_TRANSFORM_TYPE_NORMAL,
    Fiexed = D2D1_STROKE_TRANSFORM_TYPE_FIXED,
    HairLine = D2D1_STROKE_TRANSFORM_TYPE_HAIRLINE,
}

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
pub enum ThreadingMode {
    SingleThreaded = D2D1_THREADING_MODE_SINGLE_THREADED,
    MultiThreaded = D2D1_THREADING_MODE_MULTI_THREADED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum UnitMode {
    DIPs = D2D1_UNIT_MODE_DIPS,
    Pixels = D2D1_UNIT_MODE_PIXELS,
}

#[derive(Clone, Debug)]
pub struct BitmapBrushProperties1 {
    pub extend_mode_x: ExtendMode,
    pub extend_mode_y: ExtendMode,
    pub interpolation_mode: BitmapInterpolationMode,
}
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
pub struct CreationProperties {
    pub threading_mode: ThreadingMode,
    pub debug_level: DebugLevel,
    pub options: Option<DeviceContextOptions>,
}
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
pub struct DrawingStateDescription1 {
    pub antialias_mode: AntialiasMode,
    pub text_antialias_mode: TextAntialiasMode,
    pub tag1: Tag,
    pub tag2: Tag,
    pub transform: Matrix3x2F,
    pub primitive_blend: PrimitiveBlend,
    pub unit_mode: UnitMode,
}
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
pub struct PointDescription {
    pub point: Point2F,
    pub unit_tangent_vector: Point2F,
    pub end_segment: u32,
    pub end_figure: u32,
    pub length_to_end_segment: f32,
}
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

#[derive(Clone, Debug)]
pub struct PrintControlProperties {
    pub font_subset: PrintFontSubsetMode,
    pub raster_dpi: f32,
    pub color_space: ColorSpace,
}
impl PrintControlProperties {
    fn to_c_struct(&self) -> D2D1_PRINT_CONTROL_PROPERTIES {
        D2D1_PRINT_CONTROL_PROPERTIES {
            fontSubset: self.font_subset as u32,
            rasterDPI: self.raster_dpi,
            colorSpace: self.color_space as u32,
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

#[derive(Clone, Debug)]
pub struct RenderingControls {
    pub buffer_precision: BufferPrecision,
    pub tile_size: SizeU,
}
impl RenderingControls {
    fn to_c_struct(&self) -> D2D1_RENDERING_CONTROLS {
        D2D1_RENDERING_CONTROLS {
            bufferPrecision: self.buffer_precision as u32,
            tileSize: self.tile_size.into(),
        }
    }
}

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

/*
#[derive(Clone, Debug)]
pub struct BitmapProperties1 {
    pub pixel_format: PixelFormat,
    pub dpi_x: f32,
    pub dpi_y: f32,
    pub bitmap_options: BitmapOptions,
    pub color_context: Option<ColorContext>,
}
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

/*
#[derive(Clone, Debug)]
pub struct EffectInputDescription {
    pub effect: Effect,
    pub input_index: u32,
    pub input_rectangle: RectF,
}
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

/*
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
