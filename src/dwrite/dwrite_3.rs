#![allow(dead_code)]

use crate::utility::*;
use winapi::um::dwrite_3::*;

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AutomaticFontAxes {
    None = DWRITE_AUTOMATIC_FONT_AXES_NONE,
    OpticalSize = DWRITE_AUTOMATIC_FONT_AXES_OPTICAL_SIZE,
}
*/

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ContainerType {
    Unknown = DWRITE_CONTAINER_TYPE_UNKNOWN,
    Woff = DWRITE_CONTAINER_TYPE_WOFF,
    Woff2 = DWRITE_CONTAINER_TYPE_WOFF2
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontAxisAttributes {
    None = DWRITE_FONT_AXIS_ATTRIBUTES_NONE,
    Variable = DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE,
    Hiden = DWRITE_FONT_AXIS_ATTRIBUTES_HIDDEN,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontAxisTag {
    Weight = DWRITE_FONT_AXIS_TAG_WEIGHT,
    Width = DWRITE_FONT_AXIS_TAG_WIDTH,
    Slant = DWRITE_FONT_AXIS_TAG_SLANT,
    OpticalSize = DWRITE_FONT_AXIS_TAG_OPTICAL_SIZE,
    Italic = DWRITE_FONT_AXIS_TAG_ITALIC,
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontFamilyModel {
    Typographics = DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC,
    WeightStretchStyle = DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontLineGapUsage {
    Default = DWRITE_FONT_LINE_GAP_USAGE_DEFAULT,
    Disabled = DWRITE_FONT_LINE_GAP_USAGE_DISABLED,
    Enabled = DWRITE_FONT_LINE_GAP_USAGE_ENABLED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontPropertyID {
    None = DWRITE_FONT_PROPERTY_ID_NONE,
    // WeightStretchStyleFamilyName = DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FAMILY_NAME,
    // TypographicsFamilyName = DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FAMILY_NAME,
    // WeightStretchStyleFaceName = DWRITE_FONT_PROPERTY_ID_WEIGHT_STRETCH_STYLE_FACE_NAME,
    FullName = DWRITE_FONT_PROPERTY_ID_FULL_NAME,
    Win32FamilyName = DWRITE_FONT_PROPERTY_ID_WIN32_FAMILY_NAME,
    PostScriptName = DWRITE_FONT_PROPERTY_ID_POSTSCRIPT_NAME,
    DesignScriptLanguageTag = DWRITE_FONT_PROPERTY_ID_DESIGN_SCRIPT_LANGUAGE_TAG,
    SupportedScriptLanguageTag = DWRITE_FONT_PROPERTY_ID_SUPPORTED_SCRIPT_LANGUAGE_TAG,
    SemanticTag = DWRITE_FONT_PROPERTY_ID_SEMANTIC_TAG,
    Weight = DWRITE_FONT_PROPERTY_ID_WEIGHT,
    Stretch = DWRITE_FONT_PROPERTY_ID_STRETCH,
    Style = DWRITE_FONT_PROPERTY_ID_STYLE,
    // TypographicsFaceName = DWRITE_FONT_PROPERTY_ID_TYPOGRAPHIC_FACE_NAME,
    Total = DWRITE_FONT_PROPERTY_ID_TOTAL,
    // TotalRS3 = DWRITE_FONT_PROPERTY_ID_TOTAL_RS3,
    PreferredFamilyName = DWRITE_FONT_PROPERTY_ID_PREFERRED_FAMILY_NAME,
    FamilyName = DWRITE_FONT_PROPERTY_ID_FAMILY_NAME,
    FaceName = DWRITE_FONT_PROPERTY_ID_FACE_NAME,
}
impl From<Option<FontPropertyID>> for FontPropertyID {
    fn from(src: Option<FontPropertyID>) -> FontPropertyID {
        match src {
            Some(id) => id,
            None => FontPropertyID::None,
        }
    }
}

/*
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GlyphImageFormats {
    None = DWRITE_GLYPH_IMAGE_FORMATS_NONE,
    TrueType = DWRITE_GLYPH_IMAGE_FORMATS_TRUETYPE,
    CFF = DWRITE_GLYPH_IMAGE_FORMATS_CFF,
    COLR = DWRITE_GLYPH_IMAGE_FORMATS_COLR,
    SVG = DWRITE_GLYPH_IMAGE_FORMATS_SVG,
    PNG = DWRITE_GLYPH_IMAGE_FORMATS_PNG,
    JPEG = DWRITE_GLYPH_IMAGE_FORMATS_JPEG,
    TIFF = DWRITE_GLYPH_IMAGE_FORMATS_TIFF,
    PremultipliedB8G8R8A8 = DWRITE_GLYPH_IMAGE_FORMATS_PREMULTIPLIED_B8G8R8A8,
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Locality {
    Remote = DWRITE_LOCALITY_REMOTE,
    Partial = DWRITE_LOCALITY_PARTIAL,
    Local = DWRITE_LOCALITY_LOCAL,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RenderingMode1 {
    Default = DWRITE_RENDERING_MODE1_DEFAULT,
    Aliased = DWRITE_RENDERING_MODE1_ALIASED,
    GDIClasssic = DWRITE_RENDERING_MODE1_GDI_CLASSIC,
    GDINatural = DWRITE_RENDERING_MODE1_GDI_NATURAL,
    Natural = DWRITE_RENDERING_MODE1_NATURAL,
    NaturalSymmetric = DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC,
    Outline = DWRITE_RENDERING_MODE1_OUTLINE,
    NaturalSymmetricDownsampled = DWRITE_RENDERING_MODE1_NATURAL_SYMMETRIC_DOWNSAMPLED,
}

/*
#[derive(Clone, Debug)]
pub struct ColorGlyphRun<'a, 'b, 'c, 'd> {
    pub glyph_run: GlyphRun<'a, 'b, 'c>,
    pub glyph_run_description: Option<GlyphRunDescription<'d>>,
    pub baseline_origin_x: f32,
    pub baseline_origin_y: f32,
    pub run_color: ColorF,
    pub palette_index: u16,
}
impl<'a, 'b, 'c, 'd> ColorGlyphRun<'a, 'b, 'c, 'd> {
    fn to_c_struct(
        &self,
    ) -> (
        DWRITE_COLOR_GLYPH_RUN,
        Vec<DWRITE_GLYPH_OFFSET>,
        Option<(DWRITE_GLYPH_RUN_DESCRIPTION, Vec<u16>, Vec<u16>)>,
    ) {
        let (glyph_run, a) = self.glyph_run.to_c_struct();
        let glyph_run_description = self.glyph_run_description.as_ref().map(|obj| obj.to_c_struct());
        (
            DWRITE_COLOR_GLYPH_RUN {
                glyphRun: glyph_run,
                glyphRunDescription: glyph_run_description
                    .as_mut()
                    .map_or(std::ptr::null_mut(), |obj| &mut obj.0 as *mut _),
                baselineOriginX: self.baseline_origin_x,
                baselineOriginY: self.baseline_origin_y,
                runColor: self.run_color.into(),
                paletteIndex: self.palette_index,
            },
            a,
            glyph_run_description,
        )
    }
}

#[derive(Clone, Debug)]
pub struct ColorGlyphRun1 {
    pub glyph_run: GlphyRun,
    pub glyph_run_description: Option<GlyphRunDescription>,
    pub baseline_origin_x: f32,
    pub baseline_origin_y: f32,
    pub run_color: ColorF,
    pub palette_index: u16,
    pub glyph_image_format: GlyphImageFormats,
    pub mesuring_mode: MeasuringMode,
}
impl ColorGlyphRun1 {
    fn to_c_struct(&self) -> DWRITE_COLOR_GLYPH_RUN1 {
        let glyph_run_description = self.glyph_run_description.map(|obj| obj.to_c_struct());
        DWRITE_COLOR_GLYPH_RUN1 {
            glyphRun: self.glyph_run.to_c_truct(),
            glyphRunDescription: glyph_run_description.map_or(std::ptr::null(), |obj| &obj as *mut _),
            baselineOriginX: self.baseline_origin_x,
            baselineOriginY: self.baseline_origin_y,
            runColor: self.run_color.into(),
            paletteIndex: self.palette_index,
            glyphImageFormat: self.glyph_image_format as u32,
            mesuringMode: self.mesuring_mode as u32,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FileFragment {
    pub file_offset: u64,
    pub fragment_size: u64,
}
impl FileFragment {
    fn to_c_struct(&self) -> DWRITE_FILE_FRAGMENT {
        DWRITE_FILE_FRAGMENT {
            fileOffset: self.file_offset,
            fragmentSize: self.fragment_size,
        }
    }
}
*/

#[derive(Clone, Debug)]
pub struct FontAxisRange {
    pub axis_tag: FontAxisTag,
    pub min_value: f32,
    pub max_value: f32,
}
impl FontAxisRange {
    fn to_c_struct(&self) -> DWRITE_FONT_AXIS_RANGE {
        DWRITE_FONT_AXIS_RANGE {
            axisTag: self.axis_tag as u32,
            minValue: self.min_value,
            maxValue: self.max_value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FontAxisValue {
    pub axis_tag: FontAxisTag,
    pub value: f32,
}
impl FontAxisValue {
    fn to_c_struct(&self) -> DWRITE_FONT_AXIS_VALUE {
        DWRITE_FONT_AXIS_VALUE {
            axisTag: self.axis_tag as u32,
            value: self.value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FontProperty {
    pub property_id: FontPropertyID,
    pub property_value: String,
    pub locale_name: String,
}
impl FontProperty {
    fn to_c_struct(&self) -> (DWRITE_FONT_PROPERTY, Vec<u16>, Vec<u16>) {
        let value = self
            .property_value
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let locale_name = self
            .locale_name
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>();
        (
            DWRITE_FONT_PROPERTY {
                propertyId: self.property_id as u32,
                propertyValue: value.as_ptr(),
                localeName: locale_name.as_ptr(),
            },
            value,
            locale_name,
        )
    }
}

/*
#[derive(Clone, Debug)]
pub struct GlyphImageData<'a> {
    pub image_data: &'a [u8],
    pub unique_data_id: u32,
    pub pixels_per_em: u32,
    pub pixel_size: crate::d2d1::SizeU,
    pub horizontal_left_origin: crate::d2d1::Point2L,
    pub horizontal_right_origin: crate::d2d1::Point2L,
    pub vertical_top_origin: crate::d2d1::Point2L,
    pub vertical_bottom_origin: crate::d2d1::Point2L,
}
impl<'a> GlyphImageData<'a> {
    fn to_c_struct(&self) -> DWRITE_GLYPH_IMAGE_DATA {
        DWRITE_GLYPH_IMAGE_DATA {
            imageData: self.image_data.as_ptr() as *const std::ffi::c_void,
            imageDataSize: self.image_data.len() as u32,
            uniqueDataId: self.unique_data_id,
            pixelsPerEm: self.pixels_per_em,
            pixelSize: self.pixel_size.into(),
            horizontalLeftOrigin: self.horizontal_left_origin.into(),
            horizontalRightOrigin: self.horizontal_right_origin.into(),
            verticalTopOrigin: self.vertical_top_origin.into(),
            verticalBottomOrigin: self.vertical_bottom_origin.into(),
        }
    }
}
*/

#[derive(Clone, Debug)]
pub struct LineMetrics1 {
    pub length: u32,
    pub trailing_whitespace_length: u32,
    pub newline_length: u32,
    pub height: f32,
    pub baseline: f32,
    pub is_trimmed: bool,
    pub leading_before: f32,
    pub leading_after: f32,
}
impl LineMetrics1 {
    fn to_c_struct(&self) -> DWRITE_LINE_METRICS1 {
        DWRITE_LINE_METRICS1 {
            length: self.length,
            trailingWhitespaceLength: self.trailing_whitespace_length,
            newlineLength: self.newline_length,
            height: self.height,
            baseline: self.baseline,
            isTrimmed: to_BOOL(self.is_trimmed),
            leadingBefore: self.leading_before,
            leadingAfter: self.leading_after,
        }
    }
}

/*
pub struct LineSpacing {
    pub method: LineSpacingMethod,
    pub height: f32,
    pub baseline: f32,
    pub leading_before: f32,
    pub font_line_cap_usage: FontLineGapUsage,
}
impl LineSpacing {
    fn to_c_struct(&self) -> DWRITE_LINE_SPACING {
        DWRITE_LINE_SPACING {
            method: self.method as u32,
            height: self.height,
            baseline: self.baseline,
            leadingBefore: self.leading_before,
            fontLineGapUsage: self.font_line_cap_usage as u32,
        }
    }
}

#[cfg(feature = "dwrite_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontSourceType {
    Unknown = DWRITE_FONT_SOURCE_TYPE_UNKNOWN,
    PerMachine = DWRITE_FONT_SOURCE_TYPE_PER_MACHINE,
    PerUser = DWRITE_FONT_SOURCE_TYPE_PER_USER,
    APPXPackage = DWRITE_FONT_SOURCE_TYPE_APPX_PACKAGE,
    RemoteFontProvider = DWRITE_FONT_SOURCE_TYPE_REMOTE_FONT_PROVIDER
}
*/
