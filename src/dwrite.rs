use crate::api::*;
use crate::result::*;
use crate::utility::*;
use crate::Interface;
use crate::Unknown;
use crate::{impl_bitflag_operators, impl_interface};
use com_ptr::ComPtr;
use std::path::{Path, PathBuf};
use winapi::shared::minwindef::TRUE;
use winapi::shared::windef::{RECT, SIZE};
use winapi::shared::winerror::*;
use winapi::um::dcommon::*;
use winapi::um::dwrite::*;
#[cfg(feature = "dwrite_1")]
use winapi::um::dwrite_1;
#[cfg(feature = "dwrite_1")]
use winapi::um::dwrite_1::*;
#[cfg(feature = "dwrite_2")]
use winapi::um::dwrite_2::*;
#[cfg(feature = "dwrite_3")]
use winapi::um::dwrite_3::*;

/*
#[cfg(feature = "dwrite_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum AutomaticFontAxes {
    None = DWRITE_AUTOMATIC_FONT_AXES_NONE,
    OpticalSize = DWRITE_AUTOMATIC_FONT_AXES_OPTICAL_SIZE,
}
*/

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BaseLine {
    Default = DWRITE_BASELINE_DEFAULT,
    Roman = DWRITE_BASELINE_ROMAN,
    Central = DWRITE_BASELINE_CENTRAL,
    Math = DWRITE_BASELINE_MATH,
    Hanging = DWRITE_BASELINE_HANGING,
    Bottom = DWRITE_BASELINE_IDEOGRAPHIC_BOTTOM,
    Top = DWRITE_BASELINE_IDEOGRAPHIC_TOP,
    Minimum = DWRITE_BASELINE_MINIMUM,
    Maximum = DWRITE_BASELINE_MAXIMUM,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum BreakCondition {
    Neutral = DWRITE_BREAK_CONDITION_NEUTRAL,
    CanBreak = DWRITE_BREAK_CONDITION_CAN_BREAK,
    MayNotBreak = DWRITE_BREAK_CONDITION_MAY_NOT_BREAK,
    MustBreak = DWRITE_BREAK_CONDITION_MUST_BREAK,
}

/*
#[cfg(feature = "dwrite_3")]
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
pub enum FactoryType {
    Shared = DWRITE_FACTORY_TYPE_SHARED,
    Isolated = DWRITE_FACTORY_TYPE_ISOLATED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FlowDirection {
    TopToBottom = DWRITE_FLOW_DIRECTION_TOP_TO_BOTTOM,
    BottomToTop = DWRITE_FLOW_DIRECTION_BOTTOM_TO_TOP,
    LeftToRight = DWRITE_FLOW_DIRECTION_LEFT_TO_RIGHT,
    RightToLeft = DWRITE_FLOW_DIRECTION_RIGHT_TO_LEFT,
}

#[cfg(feature = "dwrite_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontAxisAttributes {
    None = DWRITE_FONT_AXIS_ATTRIBUTES_NONE,
    Variable = DWRITE_FONT_AXIS_ATTRIBUTES_VARIABLE,
    Hiden = DWRITE_FONT_AXIS_ATTRIBUTES_HIDDEN,
}

#[cfg(feature = "dwrite_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontAxisTag {
    Weight = DWRITE_FONT_AXIS_TAG_WEIGHT,
    Width = DWRITE_FONT_AXIS_TAG_WIDTH,
    Slant = DWRITE_FONT_AXIS_TAG_SLANT,
    OpticalSize = DWRITE_FONT_AXIS_TAG_OPTICAL_SIZE,
    Italic = DWRITE_FONT_AXIS_TAG_ITALIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontFaceType {
    CFF = DWRITE_FONT_FACE_TYPE_CFF,
    TrueType = DWRITE_FONT_FACE_TYPE_TRUETYPE,
    OpenTypeCollection = DWRITE_FONT_FACE_TYPE_OPENTYPE_COLLECTION,
    Type1 = DWRITE_FONT_FACE_TYPE_TYPE1,
    Vector = DWRITE_FONT_FACE_TYPE_VECTOR,
    Bitmap = DWRITE_FONT_FACE_TYPE_BITMAP,
    Unknown = DWRITE_FONT_FACE_TYPE_UNKNOWN,
    RawCFF = DWRITE_FONT_FACE_TYPE_RAW_CFF,
}
#[allow(non_upper_case_globals)]
impl FontFaceType {
    pub const TrueTypeCollection: Self = Self::OpenTypeCollection;
}

/*
#[cfg(feature = "dwrite_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontFamilyModel {
    Typographics = DWRITE_FONT_FAMILY_MODEL_TYPOGRAPHIC,
    WeightStretchStyle = DWRITE_FONT_FAMILY_MODEL_WEIGHT_STRETCH_STYLE
}
*/

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontFeatureTag {
    AlternativeFractions = DWRITE_FONT_FEATURE_TAG_ALTERNATIVE_FRACTIONS,
    PetiteCapitalsFromCapitals = DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS_FROM_CAPITALS,
    SmallCapitalsFromCapitals = DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS_FROM_CAPITALS,
    ContextualAlternates = DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_ALTERNATES,
    CaseSensitiveForms = DWRITE_FONT_FEATURE_TAG_CASE_SENSITIVE_FORMS,
    GlyphCompositionDecomposition = DWRITE_FONT_FEATURE_TAG_GLYPH_COMPOSITION_DECOMPOSITION,
    ContextualLigatures = DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_LIGATURES,
    CapitalSpacing = DWRITE_FONT_FEATURE_TAG_CAPITAL_SPACING,
    ContextualSwash = DWRITE_FONT_FEATURE_TAG_CONTEXTUAL_SWASH,
    CursivePositioning = DWRITE_FONT_FEATURE_TAG_CURSIVE_POSITIONING,
    Default = DWRITE_FONT_FEATURE_TAG_DEFAULT,
    DiscretionaryLigatures = DWRITE_FONT_FEATURE_TAG_DISCRETIONARY_LIGATURES,
    ExpertForms = DWRITE_FONT_FEATURE_TAG_EXPERT_FORMS,
    Fractions = DWRITE_FONT_FEATURE_TAG_FRACTIONS,
    FullWidth = DWRITE_FONT_FEATURE_TAG_FULL_WIDTH,
    HalfForms = DWRITE_FONT_FEATURE_TAG_HALF_FORMS,
    HalantForms = DWRITE_FONT_FEATURE_TAG_HALANT_FORMS,
    AlternateHalfWidth = DWRITE_FONT_FEATURE_TAG_ALTERNATE_HALF_WIDTH,
    HistoricalForms = DWRITE_FONT_FEATURE_TAG_HISTORICAL_FORMS,
    HorizontalKanaAlternates = DWRITE_FONT_FEATURE_TAG_HORIZONTAL_KANA_ALTERNATES,
    HistoricalLigatures = DWRITE_FONT_FEATURE_TAG_HISTORICAL_LIGATURES,
    HalfWidth = DWRITE_FONT_FEATURE_TAG_HALF_WIDTH,
    HojoKanjiForms = DWRITE_FONT_FEATURE_TAG_HOJO_KANJI_FORMS,
    JIS04Forms = DWRITE_FONT_FEATURE_TAG_JIS04_FORMS,
    JIS78Forms = DWRITE_FONT_FEATURE_TAG_JIS78_FORMS,
    JIS83Forms = DWRITE_FONT_FEATURE_TAG_JIS83_FORMS,
    JIS90Forms = DWRITE_FONT_FEATURE_TAG_JIS90_FORMS,
    Kerning = DWRITE_FONT_FEATURE_TAG_KERNING,
    StandardLigatures = DWRITE_FONT_FEATURE_TAG_STANDARD_LIGATURES,
    LiningFigures = DWRITE_FONT_FEATURE_TAG_LINING_FIGURES,
    LocalizedForms = DWRITE_FONT_FEATURE_TAG_LOCALIZED_FORMS,
    MaskPositioning = DWRITE_FONT_FEATURE_TAG_MARK_POSITIONING,
    MathematicalGreek = DWRITE_FONT_FEATURE_TAG_MATHEMATICAL_GREEK,
    MaskToMaskPositioning = DWRITE_FONT_FEATURE_TAG_MARK_TO_MARK_POSITIONING,
    AlternateAnnotationForms = DWRITE_FONT_FEATURE_TAG_ALTERNATE_ANNOTATION_FORMS,
    NLCKanjiForms = DWRITE_FONT_FEATURE_TAG_NLC_KANJI_FORMS,
    OldStyleFigures = DWRITE_FONT_FEATURE_TAG_OLD_STYLE_FIGURES,
    Ordinals = DWRITE_FONT_FEATURE_TAG_ORDINALS,
    ProportionalAlternateWidth = DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_ALTERNATE_WIDTH,
    PetiteCapitals = DWRITE_FONT_FEATURE_TAG_PETITE_CAPITALS,
    ProportionalFigures = DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_FIGURES,
    ProportionalWidths = DWRITE_FONT_FEATURE_TAG_PROPORTIONAL_WIDTHS,
    QuarterWidths = DWRITE_FONT_FEATURE_TAG_QUARTER_WIDTHS,
    RequiredLigatures = DWRITE_FONT_FEATURE_TAG_REQUIRED_LIGATURES,
    RubyNotationForms = DWRITE_FONT_FEATURE_TAG_RUBY_NOTATION_FORMS,
    StylisticAlternates = DWRITE_FONT_FEATURE_TAG_STYLISTIC_ALTERNATES,
    ScientificInferiors = DWRITE_FONT_FEATURE_TAG_SCIENTIFIC_INFERIORS,
    SmallCapitals = DWRITE_FONT_FEATURE_TAG_SMALL_CAPITALS,
    SimplifiedForms = DWRITE_FONT_FEATURE_TAG_SIMPLIFIED_FORMS,
    StylisticSet1 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_1,
    StylisticSet2 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_2,
    StylisticSet3 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_3,
    StylisticSet4 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_4,
    StylisticSet5 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_5,
    StylisticSet6 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_6,
    StylisticSet7 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_7,
    StylisticSet8 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_8,
    StylisticSet9 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_9,
    StylisticSet10 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_10,
    StylisticSet11 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_11,
    StylisticSet12 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_12,
    StylisticSet13 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_13,
    StylisticSet14 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_14,
    StylisticSet15 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_15,
    StylisticSet16 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_16,
    StylisticSet17 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_17,
    StylisticSet18 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_18,
    StylisticSet19 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_19,
    StylisticSet20 = DWRITE_FONT_FEATURE_TAG_STYLISTIC_SET_20,
    Subscript = DWRITE_FONT_FEATURE_TAG_SUBSCRIPT,
    Superscript = DWRITE_FONT_FEATURE_TAG_SUPERSCRIPT,
    Swash = DWRITE_FONT_FEATURE_TAG_SWASH,
    Titling = DWRITE_FONT_FEATURE_TAG_TITLING,
    TraditionalNameForms = DWRITE_FONT_FEATURE_TAG_TRADITIONAL_NAME_FORMS,
    TabularFigures = DWRITE_FONT_FEATURE_TAG_TABULAR_FIGURES,
    TraditionalForms = DWRITE_FONT_FEATURE_TAG_TRADITIONAL_FORMS,
    ThirdWidths = DWRITE_FONT_FEATURE_TAG_THIRD_WIDTHS,
    Unicase = DWRITE_FONT_FEATURE_TAG_UNICASE,
    Writing = DWRITE_FONT_FEATURE_TAG_VERTICAL_WRITING,
    VerticalAlternatesAndRotation = DWRITE_FONT_FEATURE_TAG_VERTICAL_ALTERNATES_AND_ROTATION,
    TagSlashedZero = DWRITE_FONT_FEATURE_TAG_SLASHED_ZERO,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontFileType {
    Unknown = DWRITE_FONT_FILE_TYPE_UNKNOWN,
    CFF = DWRITE_FONT_FILE_TYPE_CFF,
    TrueType = DWRITE_FONT_FILE_TYPE_TRUETYPE,
    OpenTypeCollection = DWRITE_FONT_FILE_TYPE_OPENTYPE_COLLECTION,
    Type1PFM = DWRITE_FONT_FILE_TYPE_TYPE1_PFM,
    Type1PFB = DWRITE_FONT_FILE_TYPE_TYPE1_PFB,
    Vector = DWRITE_FONT_FILE_TYPE_VECTOR,
    Bitmap = DWRITE_FONT_FILE_TYPE_BITMAP,
}
#[allow(non_upper_case_globals)]
impl FontFileType {
    pub const TrueTypeCollection: Self = Self::OpenTypeCollection;
}

#[cfg(feature = "dwrite_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontLineGapUsage {
    Default = DWRITE_FONT_LINE_GAP_USAGE_DEFAULT,
    Disabled = DWRITE_FONT_LINE_GAP_USAGE_DISABLED,
    Enabled = DWRITE_FONT_LINE_GAP_USAGE_ENABLED,
}

#[cfg(feature = "dwrite_3")]
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
#[cfg(feature = "dwrite_3")]
impl From<Option<FontPropertyID>> for FontPropertyID {
    fn from(src: Option<FontPropertyID>) -> FontPropertyID {
        match src {
            Some(id) => id,
            None => FontPropertyID::None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct FontSimulations(u32);
#[allow(non_upper_case_globals)]
impl FontSimulations {
    pub const None: FontSimulations = FontSimulations(DWRITE_FONT_SIMULATIONS_NONE);
    pub const Bold: FontSimulations = FontSimulations(DWRITE_FONT_SIMULATIONS_BOLD);
    pub const Oblique: FontSimulations = FontSimulations(DWRITE_FONT_SIMULATIONS_OBLIQUE);
}
impl_bitflag_operators!(FontSimulations);

/*
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontStretch {
    Undefined = DWRITE_FONT_STRETCH_UNDEFINED,
    UltraCondensed = DWRITE_FONT_STRETCH_ULTRA_CONDENSED,
    ExtraCondensed = DWRITE_FONT_STRETCH_EXTRA_CONDENSED,
    Condensed = DWRITE_FONT_STRETCH_CONDENSED,
    SemiCondensed = DWRITE_FONT_STRETCH_SEMI_CONDENSED,
    Normal = DWRITE_FONT_STRETCH_NORMAL,
    SemiExpanded = DWRITE_FONT_STRETCH_SEMI_EXPANDED,
    Expanded = DWRITE_FONT_STRETCH_EXPANDED,
    ExtraExpanded = DWRITE_FONT_STRETCH_EXTRA_EXPANDED,
    UltraExpanded = DWRITE_FONT_STRETCH_ULTRA_EXPANDED,
}
#[allow(non_upper_case_globals)]
impl FontStretch {
    pub const Medium: Self = Self::Normal;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontStyle {
    Normal = DWRITE_FONT_STYLE_NORMAL,
    Oblique = DWRITE_FONT_STYLE_OBLIQUE,
    Italic = DWRITE_FONT_STYLE_ITALIC,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum FontWeight {
    Thin = DWRITE_FONT_WEIGHT_THIN,
    ExtraLight = DWRITE_FONT_WEIGHT_EXTRA_LIGHT,
    Light = DWRITE_FONT_WEIGHT_LIGHT,
    SemiLight = DWRITE_FONT_WEIGHT_SEMI_LIGHT,
    Normal = DWRITE_FONT_WEIGHT_NORMAL,
    Medium = DWRITE_FONT_WEIGHT_MEDIUM,
    DemiBold = DWRITE_FONT_WEIGHT_DEMI_BOLD,
    Bold = DWRITE_FONT_WEIGHT_BOLD,
    ExtraBold = DWRITE_FONT_WEIGHT_EXTRA_BOLD,
    Black = DWRITE_FONT_WEIGHT_BLACK,
    ExtraBlack = DWRITE_FONT_WEIGHT_EXTRA_BLACK,
}
#[allow(non_upper_case_globals)]
impl FontWeight {
    pub const UltraLight: Self = Self::ExtraLight;
    pub const Normal: Self = Self::Normal;
    pub const SemiBold: Self = Self::DemiBold;
    pub const UltraBold: Self = Self::ExtraBold;
    pub const Heavy: Self = Self::Black;
    pub const UltraBlack: Self = Self::ExtraBlack;
}

#[cfg(feature = "dwrite_3")]
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

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum OrientationAngle {
    _0Degrees = DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES,
    _90Degrees = DWRITE_GLYPH_ORIENTATION_ANGLE_90_DEGREES,
    _180Degrees = DWRITE_GLYPH_ORIENTATION_ANGLE_180_DEGREES,
    _270Degrees = DWRITE_GLYPH_ORIENTATION_ANGLE_270_DEGREES,
}

#[cfg(feature = "dwrite_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GridFitMode {
    Default = DWRITE_GRID_FIT_MODE_DEFAULT,
    Disabled = DWRITE_GRID_FIT_MODE_DISABLED,
    Enabled = DWRITE_GRID_FIT_MODE_ENABLED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum InformationStringID {
    None = DWRITE_INFORMATIONAL_STRING_NONE,
    CopyrightNotice = DWRITE_INFORMATIONAL_STRING_COPYRIGHT_NOTICE,
    VersionStrings = DWRITE_INFORMATIONAL_STRING_VERSION_STRINGS,
    Trademark = DWRITE_INFORMATIONAL_STRING_TRADEMARK,
    Manufacturer = DWRITE_INFORMATIONAL_STRING_MANUFACTURER,
    Designer = DWRITE_INFORMATIONAL_STRING_DESIGNER,
    DesignerURL = DWRITE_INFORMATIONAL_STRING_DESIGNER_URL,
    Description = DWRITE_INFORMATIONAL_STRING_DESCRIPTION,
    FontVendorURL = DWRITE_INFORMATIONAL_STRING_FONT_VENDOR_URL,
    LicenseDescription = DWRITE_INFORMATIONAL_STRING_LICENSE_DESCRIPTION,
    LicenseInfoURL = DWRITE_INFORMATIONAL_STRING_LICENSE_INFO_URL,
    Win32FamilyNames = DWRITE_INFORMATIONAL_STRING_WIN32_FAMILY_NAMES,
    Win32SubFamilyNames = DWRITE_INFORMATIONAL_STRING_WIN32_SUBFAMILY_NAMES,
    // TypographicFamilyNames = DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_FAMILY_NAMES,
    // TypographicSubFamilyNames = DWRITE_INFORMATIONAL_STRING_TYPOGRAPHIC_SUBFAMILY_NAMES,
    SampleText = DWRITE_INFORMATIONAL_STRING_SAMPLE_TEXT,
    FullName = DWRITE_INFORMATIONAL_STRING_FULL_NAME,
    PostScriptName = DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_NAME,
    PostScriptCIDName = DWRITE_INFORMATIONAL_STRING_POSTSCRIPT_CID_NAME,
    // StyleFamilyName = DWRITE_INFORMATIONAL_STRING_WEIGHT_STRETCH_STYLE_FAMILY_NAME,
    DesignScriptLanguageTag = DWRITE_INFORMATIONAL_STRING_DESIGN_SCRIPT_LANGUAGE_TAG,
    SupportedScriptLanguageTag = DWRITE_INFORMATIONAL_STRING_SUPPORTED_SCRIPT_LANGUAGE_TAG,
    PreferredFamilyNames = DWRITE_INFORMATIONAL_STRING_PREFERRED_FAMILY_NAMES,
    PreferredSubFamilyNames = DWRITE_INFORMATIONAL_STRING_PREFERRED_SUBFAMILY_NAMES,
    WWSFamilyName = DWRITE_INFORMATIONAL_STRING_WWS_FAMILY_NAME,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum LineSpacingMethod {
    Default = DWRITE_LINE_SPACING_METHOD_DEFAULT,
    Uniform = DWRITE_LINE_SPACING_METHOD_UNIFORM,
    Proportional = DWRITE_LINE_SPACING_METHOD_PROPORTIONAL,
}

#[cfg(feature = "dwrite_3")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum Locality {
    Remote = DWRITE_LOCALITY_REMOTE,
    Partial = DWRITE_LOCALITY_PARTIAL,
    Local = DWRITE_LOCALITY_LOCAL,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum MeasuringMode {
    Natural = DWRITE_MEASURING_MODE_NATURAL,
    GDIClassic = DWRITE_MEASURING_MODE_GDI_CLASSIC,
    GDINatural = DWRITE_MEASURING_MODE_GDI_NATURAL,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum NumberSubstitutionMethod {
    FromCulture = DWRITE_NUMBER_SUBSTITUTION_METHOD_FROM_CULTURE,
    Contextual = DWRITE_NUMBER_SUBSTITUTION_METHOD_CONTEXTUAL,
    None = DWRITE_NUMBER_SUBSTITUTION_METHOD_NONE,
    National = DWRITE_NUMBER_SUBSTITUTION_METHOD_NATIONAL,
    Traditional = DWRITE_NUMBER_SUBSTITUTION_METHOD_TRADITIONAL,
}

#[cfg(feature = "dwrite_2")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum OpticalAlignment {
    None = DWRITE_OPTICAL_ALIGNMENT_NONE,
    NoSideBearings = DWRITE_OPTICAL_ALIGNMENT_NO_SIDE_BEARINGS,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum OutlineThreshold {
    Antialiased = winapi::um::dwrite_1::DWRITE_OUTLINE_THRESHOLD_ANTIALIASED,
    Aliased = winapi::um::dwrite_1::DWRITE_OUTLINE_THRESHOLD_ALIASED,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseArmStyle {
    Any = DWRITE_PANOSE_ARM_STYLE_ANY,
    NoFit = DWRITE_PANOSE_ARM_STYLE_NO_FIT,
    StraightArmsHorizontal = DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_HORIZONTAL,
    StraightArmsWedge = DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_WEDGE,
    StraightArmsVertical = DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_VERTICAL,
    StraightArmsSingleSerif = DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_SINGLE_SERIF,
    StraightArmsDoubleSerif = DWRITE_PANOSE_ARM_STYLE_STRAIGHT_ARMS_DOUBLE_SERIF,
    NonStraightArmsHorizontal = DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_HORIZONTAL,
    NonStraightArmsWedge = DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_WEDGE,
    NonStraightArmsVertical = DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_VERTICAL,
    NonStraightArmsSingleSerif = DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_SINGLE_SERIF,
    NonStraightArmsDoubleSerif = DWRITE_PANOSE_ARM_STYLE_NONSTRAIGHT_ARMS_DOUBLE_SERIF,
}
#[cfg(feature = "dwrite_1")]
#[allow(non_upper_case_globals)]
impl PanoseArmStyle {
    pub const StraightArmsHorz: Self = Self::StraightArmsHorizontal;
    pub const StraightArmsVert: Self = Self::StraightArmsVertical;
    pub const BentArmsHorz: Self = Self::NonStraightArmsHorizontal;
    pub const BentArmsWedge: Self = Self::NonStraightArmsWedge;
    pub const BentArmsVert: Self = Self::NonStraightArmsVertical;
    pub const BentArmsSingleSerif: Self = Self::NonStraightArmsSingleSerif;
    pub const BentArmsDoubleSerif: Self = Self::NonStraightArmsDoubleSerif;
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseAspect {
    Any = DWRITE_PANOSE_ASPECT_ANY,
    NoFit = DWRITE_PANOSE_ASPECT_NO_FIT,
    SuperCondensed = DWRITE_PANOSE_ASPECT_SUPER_CONDENSED,
    VeryCondensed = DWRITE_PANOSE_ASPECT_VERY_CONDENSED,
    Condensed = DWRITE_PANOSE_ASPECT_CONDENSED,
    Normal = DWRITE_PANOSE_ASPECT_NORMAL,
    Extended = DWRITE_PANOSE_ASPECT_EXTENDED,
    VeryExtended = DWRITE_PANOSE_ASPECT_VERY_EXTENDED,
    SuperExtended = DWRITE_PANOSE_ASPECT_SUPER_EXTENDED,
    Monospaced = DWRITE_PANOSE_ASPECT_MONOSPACED,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseAspectRatio {
    Any = DWRITE_PANOSE_ASPECT_RATIO_ANY,
    NoFit = DWRITE_PANOSE_ASPECT_RATIO_NO_FIT,
    VeryCondensed = DWRITE_PANOSE_ASPECT_RATIO_VERY_CONDENSED,
    Condensed = DWRITE_PANOSE_ASPECT_RATIO_CONDENSED,
    Normal = DWRITE_PANOSE_ASPECT_RATIO_NORMAL,
    Expanded = DWRITE_PANOSE_ASPECT_RATIO_EXPANDED,
    VeryExpanded = DWRITE_PANOSE_ASPECT_RATIO_VERY_EXPANDED,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseCharacterRanges {
    Any = DWRITE_PANOSE_CHARACTER_RANGES_ANY,
    NoFit = DWRITE_PANOSE_CHARACTER_RANGES_NO_FIT,
    ExtendedCollection = DWRITE_PANOSE_CHARACTER_RANGES_EXTENDED_COLLECTION,
    Literals = DWRITE_PANOSE_CHARACTER_RANGES_LITERALS,
    NoLowerCase = DWRITE_PANOSE_CHARACTER_RANGES_NO_LOWER_CASE,
    SmallCaps = DWRITE_PANOSE_CHARACTER_RANGES_SMALL_CAPS,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseContrast {
    Any = DWRITE_PANOSE_CONTRAST_ANY,
    NoFit = DWRITE_PANOSE_CONTRAST_NO_FIT,
    None = DWRITE_PANOSE_CONTRAST_NONE,
    VeryLow = DWRITE_PANOSE_CONTRAST_VERY_LOW,
    Low = DWRITE_PANOSE_CONTRAST_LOW,
    MediumLow = DWRITE_PANOSE_CONTRAST_MEDIUM_LOW,
    Medium = DWRITE_PANOSE_CONTRAST_MEDIUM,
    MediumHigh = DWRITE_PANOSE_CONTRAST_MEDIUM_HIGH,
    High = DWRITE_PANOSE_CONTRAST_HIGH,
    VeryHigh = DWRITE_PANOSE_CONTRAST_VERY_HIGH,
    HorizontalLow = DWRITE_PANOSE_CONTRAST_HORIZONTAL_LOW,
    HorizontalMedium = DWRITE_PANOSE_CONTRAST_HORIZONTAL_MEDIUM,
    HorizontalHigh = DWRITE_PANOSE_CONTRAST_HORIZONTAL_HIGH,
    Broken = DWRITE_PANOSE_CONTRAST_BROKEN,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseDecorativeClass {
    Any = DWRITE_PANOSE_DECORATIVE_CLASS_ANY,
    NoFit = DWRITE_PANOSE_DECORATIVE_CLASS_NO_FIT,
    Derivative = DWRITE_PANOSE_DECORATIVE_CLASS_DERIVATIVE,
    NonStandardTopology = DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_TOPOLOGY,
    NonStandardElements = DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ELEMENTS,
    NonStandardAspect = DWRITE_PANOSE_DECORATIVE_CLASS_NONSTANDARD_ASPECT,
    Initials = DWRITE_PANOSE_DECORATIVE_CLASS_INITIALS,
    Cartoon = DWRITE_PANOSE_DECORATIVE_CLASS_CARTOON,
    PictureStems = DWRITE_PANOSE_DECORATIVE_CLASS_PICTURE_STEMS,
    Ornamented = DWRITE_PANOSE_DECORATIVE_CLASS_ORNAMENTED,
    TextAndBackground = DWRITE_PANOSE_DECORATIVE_CLASS_TEXT_AND_BACKGROUND,
    Collage = DWRITE_PANOSE_DECORATIVE_CLASS_COLLAGE,
    Montage = DWRITE_PANOSE_DECORATIVE_CLASS_MONTAGE,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseDecorativeTopology {
    Any = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ANY,
    NoFit = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_NO_FIT,
    Standard = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_STANDARD,
    Square = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SQUARE,
    MultipleSegment = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_MULTIPLE_SEGMENT,
    ArtDeco = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_ART_DECO,
    UnevenWeighting = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UNEVEN_WEIGHTING,
    DiverseArms = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_ARMS,
    DiverseForms = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_DIVERSE_FORMS,
    LombardicForms = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_LOMBARDIC_FORMS,
    UpperCaseInLowerCase = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_UPPER_CASE_IN_LOWER_CASE,
    ImpliedTopology = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_IMPLIED_TOPOLOGY,
    HorseshoeEAndA = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_HORSESHOE_E_AND_A,
    Cursive = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_CURSIVE,
    Blackletter = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_BLACKLETTER,
    SwashVariance = DWRITE_PANOSE_DECORATIVE_TOPOLOGY_SWASH_VARIANCE,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseFamily {
    Any = DWRITE_PANOSE_FAMILY_ANY,
    NoFit = DWRITE_PANOSE_FAMILY_NO_FIT,
    TextDisplay = DWRITE_PANOSE_FAMILY_TEXT_DISPLAY,
    Script = DWRITE_PANOSE_FAMILY_SCRIPT,
    Decorative = DWRITE_PANOSE_FAMILY_DECORATIVE,
    Symbol = DWRITE_PANOSE_FAMILY_SYMBOL,
}
#[cfg(feature = "dwrite_1")]
#[allow(non_upper_case_globals)]
impl PanoseFamily {
    pub const Pictorial: Self = Self::Symbol;
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseFill {
    Any = DWRITE_PANOSE_FILL_ANY,
    NoFit = DWRITE_PANOSE_FILL_NO_FIT,
    StandardSolidFill = DWRITE_PANOSE_FILL_STANDARD_SOLID_FILL,
    NoFill = DWRITE_PANOSE_FILL_NO_FILL,
    PatternedFill = DWRITE_PANOSE_FILL_PATTERNED_FILL,
    ComplexFill = DWRITE_PANOSE_FILL_COMPLEX_FILL,
    ShapedFill = DWRITE_PANOSE_FILL_SHAPED_FILL,
    DrawnDistressed = DWRITE_PANOSE_FILL_DRAWN_DISTRESSED,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseFinials {
    Any = DWRITE_PANOSE_FINIALS_ANY,
    NoFit = DWRITE_PANOSE_FINIALS_NO_FIT,
    NoLoops = DWRITE_PANOSE_FINIALS_NONE_NO_LOOPS,
    NoneClosedLoops = DWRITE_PANOSE_FINIALS_NONE_CLOSED_LOOPS,
    NoneOpenLoops = DWRITE_PANOSE_FINIALS_NONE_OPEN_LOOPS,
    SharpNoLoops = DWRITE_PANOSE_FINIALS_SHARP_NO_LOOPS,
    SharpClosedLoops = DWRITE_PANOSE_FINIALS_SHARP_CLOSED_LOOPS,
    SharpOpenLoops = DWRITE_PANOSE_FINIALS_SHARP_OPEN_LOOPS,
    TaperedNoLoops = DWRITE_PANOSE_FINIALS_TAPERED_NO_LOOPS,
    TaperedClosedLoops = DWRITE_PANOSE_FINIALS_TAPERED_CLOSED_LOOPS,
    TaperedOpenLoops = DWRITE_PANOSE_FINIALS_TAPERED_OPEN_LOOPS,
    RoundNoLoops = DWRITE_PANOSE_FINIALS_ROUND_NO_LOOPS,
    RoundClosedLoops = DWRITE_PANOSE_FINIALS_ROUND_CLOSED_LOOPS,
    RoundOpenLoops = DWRITE_PANOSE_FINIALS_ROUND_OPEN_LOOPS,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseLetterForm {
    Any = DWRITE_PANOSE_LETTERFORM_ANY,
    NoFit = DWRITE_PANOSE_LETTERFORM_NO_FIT,
    NormalContact = DWRITE_PANOSE_LETTERFORM_NORMAL_CONTACT,
    NormalWeighted = DWRITE_PANOSE_LETTERFORM_NORMAL_WEIGHTED,
    NormalBoxed = DWRITE_PANOSE_LETTERFORM_NORMAL_BOXED,
    NormalFlattened = DWRITE_PANOSE_LETTERFORM_NORMAL_FLATTENED,
    NormalRounded = DWRITE_PANOSE_LETTERFORM_NORMAL_ROUNDED,
    NormalOffCenter = DWRITE_PANOSE_LETTERFORM_NORMAL_OFF_CENTER,
    NormalSquare = DWRITE_PANOSE_LETTERFORM_NORMAL_SQUARE,
    ObliqueContact = DWRITE_PANOSE_LETTERFORM_OBLIQUE_CONTACT,
    ObliqueWeighted = DWRITE_PANOSE_LETTERFORM_OBLIQUE_WEIGHTED,
    ObliqueBoxed = DWRITE_PANOSE_LETTERFORM_OBLIQUE_BOXED,
    ObliqueFlattened = DWRITE_PANOSE_LETTERFORM_OBLIQUE_FLATTENED,
    ObliqueRounded = DWRITE_PANOSE_LETTERFORM_OBLIQUE_ROUNDED,
    ObliqueOffCenter = DWRITE_PANOSE_LETTERFORM_OBLIQUE_OFF_CENTER,
    ObliqueSquare = DWRITE_PANOSE_LETTERFORM_OBLIQUE_SQUARE,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseLining {
    Any = DWRITE_PANOSE_LINING_ANY,
    NoFit = DWRITE_PANOSE_LINING_NO_FIT,
    None = DWRITE_PANOSE_LINING_NONE,
    Inline = DWRITE_PANOSE_LINING_INLINE,
    Outline = DWRITE_PANOSE_LINING_OUTLINE,
    Engraved = DWRITE_PANOSE_LINING_ENGRAVED,
    Shadow = DWRITE_PANOSE_LINING_SHADOW,
    Relief = DWRITE_PANOSE_LINING_RELIEF,
    Backdrop = DWRITE_PANOSE_LINING_BACKDROP,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseMidline {
    Any = DWRITE_PANOSE_MIDLINE_ANY,
    NoFit = DWRITE_PANOSE_MIDLINE_NO_FIT,
    StandardTrimmed = DWRITE_PANOSE_MIDLINE_STANDARD_TRIMMED,
    StandardPointed = DWRITE_PANOSE_MIDLINE_STANDARD_POINTED,
    StandardSerifed = DWRITE_PANOSE_MIDLINE_STANDARD_SERIFED,
    HighTrimmed = DWRITE_PANOSE_MIDLINE_HIGH_TRIMMED,
    HighPointed = DWRITE_PANOSE_MIDLINE_HIGH_POINTED,
    HighSerifed = DWRITE_PANOSE_MIDLINE_HIGH_SERIFED,
    ConstantTrimmed = DWRITE_PANOSE_MIDLINE_CONSTANT_TRIMMED,
    ConstantPointed = DWRITE_PANOSE_MIDLINE_CONSTANT_POINTED,
    ConstantSerifed = DWRITE_PANOSE_MIDLINE_CONSTANT_SERIFED,
    LowTrimmed = DWRITE_PANOSE_MIDLINE_LOW_TRIMMED,
    LowPointed = DWRITE_PANOSE_MIDLINE_LOW_POINTED,
    LowSerifed = DWRITE_PANOSE_MIDLINE_LOW_SERIFED,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseProportion {
    Any = DWRITE_PANOSE_PROPORTION_ANY,
    NoFit = DWRITE_PANOSE_PROPORTION_NO_FIT,
    OldStyle = DWRITE_PANOSE_PROPORTION_OLD_STYLE,
    Modern = DWRITE_PANOSE_PROPORTION_MODERN,
    EvenWidth = DWRITE_PANOSE_PROPORTION_EVEN_WIDTH,
    Expanded = DWRITE_PANOSE_PROPORTION_EXPANDED,
    Condensed = DWRITE_PANOSE_PROPORTION_CONDENSED,
    VeryExpanded = DWRITE_PANOSE_PROPORTION_VERY_EXPANDED,
    VeryCondensed = DWRITE_PANOSE_PROPORTION_VERY_CONDENSED,
    Monospaced = DWRITE_PANOSE_PROPORTION_MONOSPACED,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseScriptForm {
    Any = DWRITE_PANOSE_SCRIPT_FORM_ANY,
    NoFit = DWRITE_PANOSE_SCRIPT_FORM_NO_FIT,
    UprightNoWrapping = DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_NO_WRAPPING,
    UprightSomeWrapping = DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_SOME_WRAPPING,
    UprightMoreWrapping = DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_MORE_WRAPPING,
    UprightExtremeWrapping = DWRITE_PANOSE_SCRIPT_FORM_UPRIGHT_EXTREME_WRAPPING,
    ObliqueNoWrapping = DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_NO_WRAPPING,
    ObliqueSomeWrapping = DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_SOME_WRAPPING,
    ObliqueMoreWrapping = DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_MORE_WRAPPING,
    ObliqueExtremeWrapping = DWRITE_PANOSE_SCRIPT_FORM_OBLIQUE_EXTREME_WRAPPING,
    ExaggeratedNoWrapping = DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_NO_WRAPPING,
    ExaggeratedSomeWrapping = DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_SOME_WRAPPING,
    ExaggeratedMoreWrapping = DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_MORE_WRAPPING,
    ExaggeratedExtremeWrapping = DWRITE_PANOSE_SCRIPT_FORM_EXAGGERATED_EXTREME_WRAPPING,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseScriptTopology {
    Any = DWRITE_PANOSE_SCRIPT_TOPOLOGY_ANY,
    NoFit = DWRITE_PANOSE_SCRIPT_TOPOLOGY_NO_FIT,
    RomanDisconnected = DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_DISCONNECTED,
    RomanTrailing = DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_TRAILING,
    RomanConnected = DWRITE_PANOSE_SCRIPT_TOPOLOGY_ROMAN_CONNECTED,
    CursiveDisconnected = DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_DISCONNECTED,
    CursiveTrailing = DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_TRAILING,
    CursiveConnected = DWRITE_PANOSE_SCRIPT_TOPOLOGY_CURSIVE_CONNECTED,
    BlackletterDisconnected = DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_DISCONNECTED,
    BlackletterTrailing = DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_TRAILING,
    BlackletterConnected = DWRITE_PANOSE_SCRIPT_TOPOLOGY_BLACKLETTER_CONNECTED,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseSerifStyle {
    Any = DWRITE_PANOSE_SERIF_STYLE_ANY,
    NoFit = DWRITE_PANOSE_SERIF_STYLE_NO_FIT,
    Cove = DWRITE_PANOSE_SERIF_STYLE_COVE,
    ObtuseCove = DWRITE_PANOSE_SERIF_STYLE_OBTUSE_COVE,
    SquareCove = DWRITE_PANOSE_SERIF_STYLE_SQUARE_COVE,
    ObtuseSquareCove = DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SQUARE_COVE,
    Square = DWRITE_PANOSE_SERIF_STYLE_SQUARE,
    Thin = DWRITE_PANOSE_SERIF_STYLE_THIN,
    Oval = DWRITE_PANOSE_SERIF_STYLE_OVAL,
    Exaggerated = DWRITE_PANOSE_SERIF_STYLE_EXAGGERATED,
    Triangle = DWRITE_PANOSE_SERIF_STYLE_TRIANGLE,
    NormalSans = DWRITE_PANOSE_SERIF_STYLE_NORMAL_SANS,
    ObtuseSans = DWRITE_PANOSE_SERIF_STYLE_OBTUSE_SANS,
    PerpendicularSans = DWRITE_PANOSE_SERIF_STYLE_PERPENDICULAR_SANS,
    Flared = DWRITE_PANOSE_SERIF_STYLE_FLARED,
    Rounded = DWRITE_PANOSE_SERIF_STYLE_ROUNDED,
    Script = DWRITE_PANOSE_SERIF_STYLE_SCRIPT,
}
#[allow(non_upper_case_globals)]
impl PanoseSerifStyle {
    pub const PerpSans: Self = Self::PerpendicularSans;
    pub const Bone: Self = Self::Oval;
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseSpacing {
    Any = DWRITE_PANOSE_SPACING_ANY,
    NoFit = DWRITE_PANOSE_SPACING_NO_FIT,
    ProportionalSpaced = DWRITE_PANOSE_SPACING_PROPORTIONAL_SPACED,
    Monospaced = DWRITE_PANOSE_SPACING_MONOSPACED,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseStrokeVariation {
    Any = DWRITE_PANOSE_STROKE_VARIATION_ANY,
    NoFit = DWRITE_PANOSE_STROKE_VARIATION_NO_FIT,
    NoVariation = DWRITE_PANOSE_STROKE_VARIATION_NO_VARIATION,
    GradualDiagonal = DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_DIAGONAL,
    GradualTransitional = DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_TRANSITIONAL,
    GradualVertical = DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_VERTICAL,
    GradualHorizontal = DWRITE_PANOSE_STROKE_VARIATION_GRADUAL_HORIZONTAL,
    RapidVertical = DWRITE_PANOSE_STROKE_VARIATION_RAPID_VERTICAL,
    RapidHorizontal = DWRITE_PANOSE_STROKE_VARIATION_RAPID_HORIZONTAL,
    InstantVertical = DWRITE_PANOSE_STROKE_VARIATION_INSTANT_VERTICAL,
    InstantHorizontal = DWRITE_PANOSE_STROKE_VARIATION_INSTANT_HORIZONTAL,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseSymbolAspectRatio {
    Any = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_ANY,
    NoFit = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_FIT,
    NoWidth = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NO_WIDTH,
    ExceptionallyWide = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_EXCEPTIONALLY_WIDE,
    SuperWide = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_SUPER_WIDE,
    VeryWide = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_WIDE,
    Wide = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_WIDE,
    Normal = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NORMAL,
    Narrow = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_NARROW,
    VeryNarrow = DWRITE_PANOSE_SYMBOL_ASPECT_RATIO_VERY_NARROW,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseSymbolKind {
    Any = DWRITE_PANOSE_SYMBOL_KIND_ANY,
    NoFit = DWRITE_PANOSE_SYMBOL_KIND_NO_FIT,
    Montages = DWRITE_PANOSE_SYMBOL_KIND_MONTAGES,
    Pictures = DWRITE_PANOSE_SYMBOL_KIND_PICTURES,
    Shapes = DWRITE_PANOSE_SYMBOL_KIND_SHAPES,
    Scientific = DWRITE_PANOSE_SYMBOL_KIND_SCIENTIFIC,
    Music = DWRITE_PANOSE_SYMBOL_KIND_MUSIC,
    Expert = DWRITE_PANOSE_SYMBOL_KIND_EXPERT,
    Patterns = DWRITE_PANOSE_SYMBOL_KIND_PATTERNS,
    Boarders = DWRITE_PANOSE_SYMBOL_KIND_BOARDERS,
    Icons = DWRITE_PANOSE_SYMBOL_KIND_ICONS,
    Logos = DWRITE_PANOSE_SYMBOL_KIND_LOGOS,
    IndustrySpecific = DWRITE_PANOSE_SYMBOL_KIND_INDUSTRY_SPECIFIC,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PansoeToolKind {
    Any = DWRITE_PANOSE_TOOL_KIND_ANY,
    NoFit = DWRITE_PANOSE_TOOL_KIND_NO_FIT,
    FlatNIB = DWRITE_PANOSE_TOOL_KIND_FLAT_NIB,
    PressurePoint = DWRITE_PANOSE_TOOL_KIND_PRESSURE_POINT,
    Engraved = DWRITE_PANOSE_TOOL_KIND_ENGRAVED,
    Ball = DWRITE_PANOSE_TOOL_KIND_BALL,
    Brush = DWRITE_PANOSE_TOOL_KIND_BRUSH,
    Rough = DWRITE_PANOSE_TOOL_KIND_ROUGH,
    FeltPenBrushTip = DWRITE_PANOSE_TOOL_KIND_FELT_PEN_BRUSH_TIP,
    WildBrush = DWRITE_PANOSE_TOOL_KIND_WILD_BRUSH,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseWeight {
    Any = DWRITE_PANOSE_WEIGHT_ANY,
    NoFit = DWRITE_PANOSE_WEIGHT_NO_FIT,
    VeryLight = DWRITE_PANOSE_WEIGHT_VERY_LIGHT,
    Light = DWRITE_PANOSE_WEIGHT_LIGHT,
    Thin = DWRITE_PANOSE_WEIGHT_THIN,
    Book = DWRITE_PANOSE_WEIGHT_BOOK,
    Medium = DWRITE_PANOSE_WEIGHT_MEDIUM,
    Demi = DWRITE_PANOSE_WEIGHT_DEMI,
    Bold = DWRITE_PANOSE_WEIGHT_BOLD,
    Heavy = DWRITE_PANOSE_WEIGHT_HEAVY,
    Black = DWRITE_PANOSE_WEIGHT_BLACK,
    ExtraBlack = DWRITE_PANOSE_WEIGHT_EXTRA_BLACK,
}
#[allow(non_upper_case_globals)]
impl PanoseWeight {
    pub const WeightNord: Self = Self::ExtraBlack;
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseXAscent {
    Any = DWRITE_PANOSE_XASCENT_ANY,
    NoFit = DWRITE_PANOSE_XASCENT_NO_FIT,
    VeryLow = DWRITE_PANOSE_XASCENT_VERY_LOW,
    Low = DWRITE_PANOSE_XASCENT_LOW,
    Medium = DWRITE_PANOSE_XASCENT_MEDIUM,
    High = DWRITE_PANOSE_XASCENT_HIGH,
    VeryHigh = DWRITE_PANOSE_XASCENT_VERY_HIGH,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseXHeight {
    Any = DWRITE_PANOSE_XHEIGHT_ANY,
    NoFit = DWRITE_PANOSE_XHEIGHT_NO_FIT,
    ConstantSmall = DWRITE_PANOSE_XHEIGHT_CONSTANT_SMALL,
    ConstantStandard = DWRITE_PANOSE_XHEIGHT_CONSTANT_STANDARD,
    ConstantLarge = DWRITE_PANOSE_XHEIGHT_CONSTANT_LARGE,
    DuckingSmall = DWRITE_PANOSE_XHEIGHT_DUCKING_SMALL,
    DuckingStandard = DWRITE_PANOSE_XHEIGHT_DUCKING_STANDARD,
    DuckingLarge = DWRITE_PANOSE_XHEIGHT_DUCKING_LARGE,
}
#[cfg(feature = "dwrite_1")]
#[allow(non_upper_case_globals)]
impl PanoseXHeight {
    pub const ConstantStd: Self = Self::ConstantStandard;
    pub const DuckingStd: Self = Self::DuckingStandard;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ParagraphAlignment {
    Near = DWRITE_PARAGRAPH_ALIGNMENT_NEAR,
    Far = DWRITE_PARAGRAPH_ALIGNMENT_FAR,
    Center = DWRITE_PARAGRAPH_ALIGNMENT_CENTER,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PixelGeometry {
    Flat = DWRITE_PIXEL_GEOMETRY_FLAT,
    RGB = DWRITE_PIXEL_GEOMETRY_RGB,
    BGR = DWRITE_PIXEL_GEOMETRY_BGR,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ReadingDirection {
    LeftToRight = DWRITE_READING_DIRECTION_LEFT_TO_RIGHT,
    RightToLeft = DWRITE_READING_DIRECTION_RIGHT_TO_LEFT,
    TopTopBottom = DWRITE_READING_DIRECTION_TOP_TO_BOTTOM,
    BottomToTop = DWRITE_READING_DIRECTION_BOTTOM_TO_TOP,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RenderingMode {
    Default = DWRITE_RENDERING_MODE_DEFAULT,
    Aliased = DWRITE_RENDERING_MODE_ALIASED,
    GDIClassic = DWRITE_RENDERING_MODE_GDI_CLASSIC,
    GDINatural = DWRITE_RENDERING_MODE_GDI_NATURAL,
    Natural = DWRITE_RENDERING_MODE_NATURAL,
    NaturalSymmetric = DWRITE_RENDERING_MODE_NATURAL_SYMMETRIC,
    Outline = DWRITE_RENDERING_MODE_OUTLINE,
}
#[allow(non_upper_case_globals)]
impl RenderingMode {
    pub const ClearTypeGDIClassic: Self = Self::GDIClassic;
    pub const ClearTypeGDINatural: Self = Self::GDINatural;
    pub const ClearTypeNatural: Self = Self::Natural;
    pub const ClearTypeNaturalSymmetric: Self = Self::NaturalSymmetric;
}

#[cfg(feature = "dwrite_3")]
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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum ScriptShapes {
    Default = DWRITE_SCRIPT_SHAPES_DEFAULT,
    NoVisual = DWRITE_SCRIPT_SHAPES_NO_VISUAL,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextAlignment {
    Leading = DWRITE_TEXT_ALIGNMENT_LEADING,
    Trailing = DWRITE_TEXT_ALIGNMENT_TRAILING,
    Center = DWRITE_TEXT_ALIGNMENT_CENTER,
    Justified = DWRITE_TEXT_ALIGNMENT_JUSTIFIED,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextAntialiasMode {
    ClearType = DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE,
    GrayScale = DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextureType {
    Aliased1x1 = DWRITE_TEXTURE_ALIASED_1x1,
    ClearType3x1 = DWRITE_TEXTURE_CLEARTYPE_3x1,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TrimmingGranularity {
    None = DWRITE_TRIMMING_GRANULARITY_NONE,
    Character = DWRITE_TRIMMING_GRANULARITY_CHARACTER,
    Word = DWRITE_TRIMMING_GRANULARITY_WORD,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VerticalGlyphOrientation {
    Default = DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT,
    Stacked = DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum WordWrapping {
    Wrap = DWRITE_WORD_WRAPPING_WRAP,
    NoWrap = DWRITE_WORD_WRAPPING_NO_WRAP,
    EmergencyBreak = DWRITE_WORD_WRAPPING_EMERGENCY_BREAK,
    WholeWord = DWRITE_WORD_WRAPPING_WHOLE_WORD,
    Character = DWRITE_WORD_WRAPPING_CHARACTER,
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Debug)]
pub struct CaretMetrics {
    pub slope_rise: i16,
    pub slope_run: i16,
    pub offset: i16,
}
#[cfg(feature = "dwrite_1")]
impl CaretMetrics {
    fn to_c_struct(&self) -> dwrite_1::DWRITE_CARET_METRICS {
        dwrite_1::DWRITE_CARET_METRICS {
            slopeRise: self.slope_rise,
            slopeRun: self.slope_run,
            offset: self.offset,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ClusterMetrics {
    pub width: f32,
    pub length: u16,
    pub can_wrap_line_after: bool,
    pub is_whitespace: bool,
    pub is_newline: bool,
    pub is_soft_hyphen: bool,
    pub is_right_to_left: bool,
}
impl ClusterMetrics {
    fn to_c_struct(&self) -> DWRITE_CLUSTER_METRICS {
        let mut obj = DWRITE_CLUSTER_METRICS::default();
        obj.width = self.width;
        obj.length = self.length;
        obj.set_canWrapLineAfter(self.can_wrap_line_after as u16);
        obj.set_isWhitespace(self.is_whitespace as u16);
        obj.set_isNewline(self.is_newline as u16);
        obj.set_isSoftHyphen(self.is_soft_hyphen as u16);
        obj.set_isRightToLeft(self.is_right_to_left as u16);
        obj
    }
}
impl From<DWRITE_CLUSTER_METRICS> for ClusterMetrics {
    fn from(src: DWRITE_CLUSTER_METRICS) -> ClusterMetrics {
        ClusterMetrics {
            width: src.width,
            length: src.length,
            can_wrap_line_after: src.canWrapLineAfter() != 0,
            is_whitespace: src.isWhitespace() != 0,
            is_newline: src.isNewline() != 0,
            is_soft_hyphen: src.isSoftHyphen() != 0,
            is_right_to_left: src.isRightToLeft() != 0,
        }
    }
}

pub type ColorF = crate::dxgitype::RGBA;

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

#[cfg(feature = "dwrite_3")]
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
#[cfg(feature = "dwrite_3")]
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

#[cfg(feature = "dwrite_3")]
#[derive(Clone, Debug)]
pub struct FileFragment {
    pub file_offset: u64,
    pub fragment_size: u64,
}
#[cfg(feature = "dwrite_3")]
impl FileFragment {
    fn to_c_struct(&self) -> DWRITE_FILE_FRAGMENT {
        DWRITE_FILE_FRAGMENT {
            fileOffset: self.file_offset,
            fragmentSize: self.fragment_size,
        }
    }
}
*/

#[cfg(feature = "dwrite_3")]
#[derive(Clone, Debug)]
pub struct FontAxisRange {
    pub axis_tag: FontAxisTag,
    pub min_value: f32,
    pub max_value: f32,
}
#[cfg(feature = "dwrite_3")]
impl FontAxisRange {
    fn to_c_struct(&self) -> DWRITE_FONT_AXIS_RANGE {
        DWRITE_FONT_AXIS_RANGE {
            axisTag: self.axis_tag as u32,
            minValue: self.min_value,
            maxValue: self.max_value,
        }
    }
}

#[cfg(feature = "dwrite_3")]
#[derive(Clone, Debug)]
pub struct FontAxisValue {
    pub axis_tag: FontAxisTag,
    pub value: f32,
}
#[cfg(feature = "dwrite_3")]
impl FontAxisValue {
    fn to_c_struct(&self) -> DWRITE_FONT_AXIS_VALUE {
        DWRITE_FONT_AXIS_VALUE {
            axisTag: self.axis_tag as u32,
            value: self.value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FontFeature {
    pub name_tag: FontFeatureTag,
    pub parameter: u32,
}
impl FontFeature {
    fn to_c_struct(&self) -> DWRITE_FONT_FEATURE {
        DWRITE_FONT_FEATURE {
            nameTag: self.name_tag as u32,
            parameter: self.parameter,
        }
    }
}
impl From<DWRITE_FONT_FEATURE> for FontFeature {
    fn from(src: DWRITE_FONT_FEATURE) -> FontFeature {
        FontFeature {
            name_tag: unsafe { std::mem::transmute(src.nameTag) },
            parameter: src.parameter,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FontMetrics {
    pub design_units_per_em: u16,
    pub ascent: u16,
    pub descent: u16,
    pub line_gap: i16,
    pub cap_height: u16,
    pub x_height: u16,
    pub underline_position: i16,
    pub underline_thickness: u16,
    pub strikethrough_position: i16,
    pub strikethrough_thickness: u16,
}
impl FontMetrics {
    fn to_c_struct(&self) -> DWRITE_FONT_METRICS {
        DWRITE_FONT_METRICS {
            designUnitsPerEm: self.design_units_per_em,
            ascent: self.ascent,
            descent: self.descent,
            lineGap: self.line_gap,
            capHeight: self.cap_height,
            xHeight: self.x_height,
            underlinePosition: self.underline_position,
            underlineThickness: self.underline_thickness,
            strikethroughPosition: self.strikethrough_position,
            strikethroughThickness: self.strikethrough_thickness,
        }
    }
}
impl From<DWRITE_FONT_METRICS> for FontMetrics {
    fn from(src: DWRITE_FONT_METRICS) -> FontMetrics {
        FontMetrics {
            design_units_per_em: src.designUnitsPerEm,
            ascent: src.ascent,
            descent: src.descent,
            line_gap: src.lineGap,
            cap_height: src.capHeight,
            x_height: src.xHeight,
            underline_position: src.underlinePosition,
            underline_thickness: src.underlineThickness,
            strikethrough_position: src.strikethroughPosition,
            strikethrough_thickness: src.strikethroughThickness,
        }
    }
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Debug)]
pub struct FontMetrics1 {
    pub design_units_per_em: u16,
    pub ascent: u16,
    pub descent: u16,
    pub line_gap: i16,
    pub cap_height: u16,
    pub x_height: u16,
    pub underline_position: i16,
    pub underline_thickness: u16,
    pub strikethrough_position: i16,
    pub strikethrough_thickness: u16,
    pub glyph_box_left: i16,
    pub glyph_box_top: i16,
    pub glyph_box_right: i16,
    pub glyph_box_bottom: i16,
    pub subscript_position_x: i16,
    pub subscript_position_y: i16,
    pub subscript_size_x: i16,
    pub subscript_size_y: i16,
    pub superscript_position_x: i16,
    pub superscript_position_y: i16,
    pub superscript_size_x: i16,
    pub superscript_size_y: i16,
    pub has_typographic_metrics: bool,
}
#[cfg(feature = "dwrite_1")]
impl FontMetrics1 {
    fn to_c_struct(&self) -> dwrite_1::DWRITE_FONT_METRICS1 {
        dwrite_1::DWRITE_FONT_METRICS1 {
            designUnitsPerEm: self.design_units_per_em,
            ascent: self.ascent,
            descent: self.descent,
            lineGap: self.line_gap,
            capHeight: self.cap_height,
            xHeight: self.x_height,
            underlinePosition: self.underline_position,
            underlineThickness: self.underline_thickness,
            strikethroughPosition: self.strikethrough_position,
            strikethroughThickness: self.strikethrough_thickness,
            glyphBoxLeft: self.glyph_box_left,
            glyphBoxTop: self.glyph_box_top,
            glyphBoxRight: self.glyph_box_right,
            glyphBoxBottom: self.glyph_box_bottom,
            subscriptPositionX: self.subscript_position_x,
            subscriptPositionY: self.subscript_position_y,
            subscriptSizeX: self.subscript_size_x,
            subscriptSizeY: self.subscript_size_y,
            superscriptPositionX: self.superscript_position_x,
            superscriptPositionY: self.superscript_position_y,
            superscriptSizeX: self.superscript_size_x,
            superscriptSizeY: self.superscript_size_y,
            hasTypographicMetrics: to_BOOL(self.has_typographic_metrics),
        }
    }
}

#[cfg(feature = "dwrite_3")]
#[derive(Clone, Debug)]
pub struct FontProperty {
    pub property_id: FontPropertyID,
    pub property_value: String,
    pub locale_name: String,
}
#[cfg(feature = "dwrite_3")]
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

#[cfg(all(feature = "dwrite_3", feature = "d2d1"))]
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
#[cfg(all(feature = "dwrite_3", feature = "d2d1"))]
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

#[derive(Clone, Debug)]
pub struct GlyphMetrics {
    pub left_side_bearing: i32,
    pub advance_width: u32,
    pub right_side_bearing: i32,
    pub top_side_bearing: i32,
    pub advance_height: u32,
    pub bottom_side_bearing: i32,
    pub vertical_origin_y: i32,
}
impl GlyphMetrics {
    fn to_c_struct(&self) -> DWRITE_GLYPH_METRICS {
        DWRITE_GLYPH_METRICS {
            leftSideBearing: self.left_side_bearing,
            advanceWidth: self.advance_width,
            rightSideBearing: self.right_side_bearing,
            topSideBearing: self.top_side_bearing,
            advanceHeight: self.advance_height,
            bottomSideBearing: self.bottom_side_bearing,
            verticalOriginY: self.vertical_origin_y,
        }
    }
}
impl From<DWRITE_GLYPH_METRICS> for GlyphMetrics {
    fn from(src: DWRITE_GLYPH_METRICS) -> GlyphMetrics {
        GlyphMetrics {
            left_side_bearing: src.leftSideBearing,
            advance_width: src.advanceWidth,
            right_side_bearing: src.rightSideBearing,
            top_side_bearing: src.topSideBearing,
            advance_height: src.advanceHeight,
            bottom_side_bearing: src.bottomSideBearing,
            vertical_origin_y: src.verticalOriginY,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GlyphOffset {
    pub advance_offset: f32,
    pub ascender_offset: f32,
}
impl GlyphOffset {
    fn to_c_struct(&self) -> DWRITE_GLYPH_OFFSET {
        DWRITE_GLYPH_OFFSET {
            advanceOffset: self.advance_offset,
            ascenderOffset: self.ascender_offset,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GlyphRun<'a, 'b, 'c> {
    pub font_face: FontFace,
    pub font_em_size: f32,
    pub glyph_indices: &'a [u16],
    pub glyph_advances: &'b [f32],
    pub glyph_offsets: &'c [GlyphOffset],
    pub is_sideways: bool,
    pub bidi_level: u32,
}
impl<'a, 'b, 'c> GlyphRun<'a, 'b, 'c> {
    pub(crate) fn to_c_struct(&self) -> (DWRITE_GLYPH_RUN, Vec<DWRITE_GLYPH_OFFSET>) {
        let offsets = self
            .glyph_offsets
            .iter()
            .map(|offset| offset.to_c_struct())
            .collect::<Vec<_>>();
        (
            DWRITE_GLYPH_RUN {
                fontFace: self.font_face.as_ptr(),
                fontEmSize: self.font_em_size,
                glyphCount: self.glyph_indices.len() as u32,
                glyphIndices: self.glyph_indices.as_ptr(),
                glyphAdvances: self.glyph_advances.as_ptr(),
                glyphOffsets: offsets.as_ptr(),
                isSideways: to_BOOL(self.is_sideways),
                bidiLevel: self.bidi_level,
            },
            offsets,
        )
    }
}

#[derive(Clone, Debug)]
pub struct GlyphRunDescription<'a> {
    pub locale_name: String,
    pub string: String,
    pub cluster_map: &'a [u16],
    pub text_position: u32,
}
impl<'a> GlyphRunDescription<'a> {
    fn to_c_struct(&self) -> (DWRITE_GLYPH_RUN_DESCRIPTION, Vec<u16>, Vec<u16>) {
        let locale_name = self
            .locale_name
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>();
        let string = self
            .string
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>();
        (
            DWRITE_GLYPH_RUN_DESCRIPTION {
                localeName: locale_name.as_ptr(),
                string: string.as_ptr(),
                stringLength: string.len() as u32,
                clusterMap: self.cluster_map.as_ptr(),
                textPosition: self.text_position,
            },
            locale_name,
            string,
        )
    }
}

#[derive(Clone, Debug)]
pub struct HitTestMetrics {
    pub text_position: u32,
    pub length: u32,
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32,
    pub bidi_level: u32,
    pub is_text: bool,
    pub is_trimmed: bool,
}
impl HitTestMetrics {
    fn to_c_struct(&self) -> DWRITE_HIT_TEST_METRICS {
        DWRITE_HIT_TEST_METRICS {
            textPosition: self.text_position,
            length: self.length,
            left: self.left,
            top: self.top,
            width: self.width,
            height: self.height,
            bidiLevel: self.bidi_level,
            isText: to_BOOL(self.is_text),
            isTrimmed: to_BOOL(self.is_trimmed),
        }
    }
}
impl From<DWRITE_HIT_TEST_METRICS> for HitTestMetrics {
    fn from(src: DWRITE_HIT_TEST_METRICS) -> HitTestMetrics {
        HitTestMetrics {
            text_position: src.textPosition,
            length: src.length,
            left: src.left,
            top: src.top,
            width: src.width,
            height: src.height,
            bidi_level: src.bidiLevel,
            is_text: src.isText == TRUE,
            is_trimmed: src.isTrimmed == TRUE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct InlineObjectMetrics {
    pub width: f32,
    pub height: f32,
    pub baseline: f32,
    pub supports_sideways: bool,
}
impl InlineObjectMetrics {
    fn to_c_struct(&self) -> DWRITE_INLINE_OBJECT_METRICS {
        DWRITE_INLINE_OBJECT_METRICS {
            width: self.width,
            height: self.height,
            baseline: self.baseline,
            supportsSideways: to_BOOL(self.supports_sideways),
        }
    }
}
impl From<DWRITE_INLINE_OBJECT_METRICS> for InlineObjectMetrics {
    fn from(src: DWRITE_INLINE_OBJECT_METRICS) -> InlineObjectMetrics {
        InlineObjectMetrics {
            width: src.width,
            height: src.height,
            baseline: src.baseline,
            supports_sideways: src.supportsSideways == TRUE,
        }
    }
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Debug)]
pub struct JustificationOpportunity {
    pub expansion_minimum: f32,
    pub expansion_maximum: f32,
    pub compression_maximum: f32,
    pub expansion_priority: u8,
    pub compression_priority: u8,
    pub allow_residual_expansion: bool,
    pub allow_residual_compression: bool,
    pub apply_to_leading_edge: bool,
    pub apply_to_trailing_edge: bool,
}
#[cfg(feature = "dwrite_1")]
impl JustificationOpportunity {
    fn to_c_struct(&self) -> DWRITE_JUSTIFICATION_OPPORTUNITY {
        let mut obj = DWRITE_JUSTIFICATION_OPPORTUNITY::default();
        obj.expansionMinimum = self.expansion_minimum;
        obj.expansionMaximum = self.expansion_maximum;
        obj.compressionMaximum = self.compression_maximum;
        obj.set_expansionPriority(self.expansion_priority as u32);
        obj.set_compressionPriority(self.compression_priority as u32);
        obj.set_allowResidualExpansion(self.allow_residual_expansion as u32);
        obj.set_allowResidualCompression(self.allow_residual_compression as u32);
        obj.set_applyToLeadingEdge(self.apply_to_leading_edge as u32);
        obj.set_applyToTrailingEdge(self.apply_to_trailing_edge as u32);
        obj
    }
}

#[derive(Clone, Debug)]
pub struct LineBreakpoint {
    pub break_condition_before: u8,
    pub break_condition_after: u8,
    pub is_whitespace: bool,
    pub is_soft_hyphen: bool,
}
impl LineBreakpoint {
    fn to_c_struct(&self) -> DWRITE_LINE_BREAKPOINT {
        let mut obj = DWRITE_LINE_BREAKPOINT::default();
        obj.set_breakConditionBefore(self.break_condition_before);
        obj.set_breakConditionAfter(self.break_condition_after);
        obj.set_isWhitespace(self.is_whitespace as u8);
        obj.set_isSoftHyphen(self.is_soft_hyphen as u8);
        obj
    }
}

#[derive(Clone, Debug)]
pub struct LineMetrics {
    pub length: u32,
    pub trailing_whitespace_length: u32,
    pub newline_length: u32,
    pub height: f32,
    pub baseline: f32,
    pub is_trimmed: bool,
}
impl LineMetrics {
    fn to_c_struct(&self) -> DWRITE_LINE_METRICS {
        DWRITE_LINE_METRICS {
            length: self.length,
            trailingWhitespaceLength: self.trailing_whitespace_length,
            newlineLength: self.newline_length,
            height: self.height,
            baseline: self.baseline,
            isTrimmed: to_BOOL(self.is_trimmed),
        }
    }
}
impl From<DWRITE_LINE_METRICS> for LineMetrics {
    fn from(src: DWRITE_LINE_METRICS) -> LineMetrics {
        LineMetrics {
            length: src.length,
            trailing_whitespace_length: src.trailingWhitespaceLength,
            newline_length: src.newlineLength,
            height: src.height,
            baseline: src.baseline,
            is_trimmed: src.isTrimmed == TRUE,
        }
    }
}

#[cfg(feature = "dwrite_3")]
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
#[cfg(feature = "dwrite_3")]
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

#[cfg(feature = "dwrite_3")]
pub struct LineSpacing {
    pub method: LineSpacingMethod,
    pub height: f32,
    pub baseline: f32,
    pub leading_before: f32,
    pub font_line_cap_usage: FontLineGapUsage,
}
#[cfg(feature = "dwrite_3")]
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

#[derive(Clone, Debug)]
pub struct Matrix {
    pub m11: f32,
    pub m12: f32,
    pub m21: f32,
    pub m22: f32,
    pub dx: f32,
    pub dy: f32,
}
impl Matrix {
    fn to_c_struct(&self) -> DWRITE_MATRIX {
        DWRITE_MATRIX {
            m11: self.m11,
            m12: self.m11,
            m21: self.m21,
            m22: self.m22,
            dx: self.dx,
            dy: self.dy,
        }
    }
}
impl From<DWRITE_MATRIX> for Matrix {
    fn from(src: DWRITE_MATRIX) -> Matrix {
        Matrix {
            m11: src.m11,
            m12: src.m12,
            m21: src.m21,
            m22: src.m22,
            dx: src.dx,
            dy: src.dy,
        }
    }
}

#[derive(Clone, Debug)]
pub struct OverhangMetrics {
    pub left: f32,
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
}
impl OverhangMetrics {
    fn to_c_struct(&self) -> DWRITE_OVERHANG_METRICS {
        DWRITE_OVERHANG_METRICS {
            left: self.left,
            top: self.top,
            right: self.right,
            bottom: self.bottom,
        }
    }
}
impl From<DWRITE_OVERHANG_METRICS> for OverhangMetrics {
    fn from(src: DWRITE_OVERHANG_METRICS) -> OverhangMetrics {
        OverhangMetrics {
            left: src.left,
            top: src.top,
            right: src.right,
            bottom: src.bottom,
        }
    }
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Debug)]
pub enum Panose {
    Text {
        family_kind: u8,
        serif_style: u8,
        weight: u8,
        proportion: u8,
        contrast: u8,
        stroke_variation: u8,
        arm_style: u8,
        letter_form: u8,
        midline: u8,
        x_height: u8,
    },
    Script {
        family_kind: u8,
        tool_kind: u8,
        weight: u8,
        spacing: u8,
        aspect_ratio: u8,
        contrast: u8,
        script_topology: u8,
        script_form: u8,
        finials: u8,
        x_ascent: u8,
    },
    Decorative {
        family_kind: u8,
        decorative_class: u8,
        weight: u8,
        aspect: u8,
        contrast: u8,
        serif_variant: u8,
        fill: u8,
        lining: u8,
        decorative_topology: u8,
        character_range: u8,
    },
    Symbol {
        family_kind: u8,
        symbol_kind: u8,
        weight: u8,
        spacing: u8,
        aspect_ratio_and_contrast: u8,
        aspect_ratio_94: u8,
        aspect_ratio_119: u8,
        aspect_ratio_157: u8,
        aspect_ratio_163: u8,
        aspect_ratio_211: u8,
    },
}
#[cfg(feature = "dwrite_1")]
impl Panose {
    pub fn family_kind(&self) -> u8 {
        match self {
            Self::Text { family_kind, .. } => *family_kind,
            Self::Script { family_kind, .. } => *family_kind,
            Self::Decorative { family_kind, .. } => *family_kind,
            Self::Symbol { family_kind, .. } => *family_kind,
        }
    }

    fn to_c_struct(&self) -> DWRITE_PANOSE {
        let mut obj = DWRITE_PANOSE::default();
        match self {
            Self::Text {
                family_kind,
                serif_style,
                weight,
                proportion,
                contrast,
                stroke_variation,
                arm_style,
                letter_form,
                midline,
                x_height,
            } => unsafe {
                obj.text_mut().familyKind = *family_kind;
                obj.text_mut().serifStyle = *serif_style;
                obj.text_mut().weight = *weight;
                obj.text_mut().proportion = *proportion;
                obj.text_mut().contrast = *contrast;
                obj.text_mut().strokeVariation = *stroke_variation;
                obj.text_mut().armStyle = *arm_style;
                obj.text_mut().letterform = *letter_form;
                obj.text_mut().midline = *midline;
                obj.text_mut().xHeight = *x_height;
            },
            Self::Script {
                family_kind,
                tool_kind,
                weight,
                spacing,
                aspect_ratio,
                contrast,
                script_topology,
                script_form,
                finials,
                x_ascent,
            } => unsafe {
                obj.script_mut().familyKind = *family_kind;
                obj.script_mut().toolKind = *tool_kind;
                obj.script_mut().weight = *weight;
                obj.script_mut().spacing = *spacing;
                obj.script_mut().aspectRatio = *aspect_ratio;
                obj.script_mut().contrast = *contrast;
                obj.script_mut().scriptTopology = *script_topology;
                obj.script_mut().scriptForm = *script_form;
                obj.script_mut().finials = *finials;
                obj.script_mut().xAscent = *x_ascent;
            },
            Self::Decorative {
                family_kind,
                decorative_class,
                weight,
                aspect,
                contrast,
                serif_variant,
                fill,
                lining,
                decorative_topology,
                character_range,
            } => unsafe {
                obj.decorative_mut().familyKind = *family_kind;
                obj.decorative_mut().decorativeClass = *decorative_class;
                obj.decorative_mut().weight = *weight;
                obj.decorative_mut().aspect = *aspect;
                obj.decorative_mut().contrast = *contrast;
                obj.decorative_mut().serifVariant = *serif_variant;
                obj.decorative_mut().fill = *fill;
                obj.decorative_mut().lining = *lining;
                obj.decorative_mut().decorativeTopology = *decorative_topology;
                obj.decorative_mut().characterRange = *character_range;
            },
            Self::Symbol {
                family_kind,
                symbol_kind,
                weight,
                spacing,
                aspect_ratio_and_contrast,
                aspect_ratio_94,
                aspect_ratio_119,
                aspect_ratio_157,
                aspect_ratio_163,
                aspect_ratio_211,
            } => unsafe {
                obj.symbol_mut().familyKind = *family_kind;
                obj.symbol_mut().symbolKind = *symbol_kind;
                obj.symbol_mut().weight = *weight;
                obj.symbol_mut().spacing = *spacing;
                obj.symbol_mut().aspectRatioAndContrast = *aspect_ratio_and_contrast;
                obj.symbol_mut().aspectRatio94 = *aspect_ratio_94;
                obj.symbol_mut().aspectRatio119 = *aspect_ratio_119;
                obj.symbol_mut().aspectRatio157 = *aspect_ratio_157;
                obj.symbol_mut().aspectRatio163 = *aspect_ratio_163;
                obj.symbol_mut().aspectRatio211 = *aspect_ratio_211;
            },
        }
        obj
    }
}

#[derive(Clone, Debug)]
pub struct ScriptAnalysis {
    pub script: u16,
    pub shapes: ScriptShapes,
}
impl ScriptAnalysis {
    fn to_c_struct(&self) -> DWRITE_SCRIPT_ANALYSIS {
        DWRITE_SCRIPT_ANALYSIS {
            script: self.script,
            shapes: self.shapes as u32,
        }
    }
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Debug)]
pub struct ScriptProperties {
    pub iso_script_code: u32,
    pub iso_script_number: u32,
    pub cluster_look_ahead: u32,
    pub justification_character: u32,
    pub restrict_caret_to_clusters: bool,
    pub uses_word_dividers: bool,
    pub is_discrete_writing: bool,
    pub is_block_writing: bool,
    pub is_distributed_within_cluster: bool,
    pub is_connected_writing: bool,
    pub is_cursive_writing: bool,
}
#[cfg(feature = "dwrite_1")]
impl ScriptProperties {
    fn to_c_struct(&self) -> DWRITE_SCRIPT_PROPERTIES {
        let mut obj = DWRITE_SCRIPT_PROPERTIES::default();
        obj.isoScriptCode = self.iso_script_code;
        obj.isoScriptNumber = self.iso_script_number;
        obj.clusterLookahead = self.cluster_look_ahead;
        obj.justificationCharacter = self.justification_character;
        obj.set_restrictCaretToClusters(self.restrict_caret_to_clusters as u32);
        obj.set_usesWordDividers(self.uses_word_dividers as u32);
        obj.set_isDiscreteWriting(self.is_discrete_writing as u32);
        obj.set_isBlockWriting(self.is_block_writing as u32);
        obj.set_isDistributedWithinCluster(self.is_distributed_within_cluster as u32);
        obj.set_isConnectedWriting(self.is_connected_writing as u32);
        obj.set_isCursiveWriting(self.is_cursive_writing as u32);
        obj
    }
}

#[derive(Clone, Debug)]
pub struct ShapingGlyphProperties {
    pub justification: u8,
    pub is_cluster_start: bool,
    pub is_diacritic: bool,
    pub is_zero_width_space: bool,
}
impl ShapingGlyphProperties {
    fn to_c_struct(&self) -> DWRITE_SHAPING_GLYPH_PROPERTIES {
        let mut obj = DWRITE_SHAPING_GLYPH_PROPERTIES::default();
        obj.set_justification(self.justification as u16);
        obj.set_isClusterStart(self.is_cluster_start as u16);
        obj.set_isDiacritic(self.is_diacritic as u16);
        obj.set_isZeroWidthSpace(self.is_zero_width_space as u16);
        obj
    }
}

#[derive(Clone, Debug)]
pub struct ShapingTextProperties {
    pub is_shaped_alone: bool,
    // pub can_break_shaping_after: bool,
}
impl ShapingTextProperties {
    fn to_c_struct(&self) -> DWRITE_SHAPING_TEXT_PROPERTIES {
        let mut obj = DWRITE_SHAPING_TEXT_PROPERTIES::default();
        obj.set_isShapedAlone(self.is_shaped_alone as u16);
        // obj.set_canBreakShapingAfter(self.can_break_shaping_after as u16);
        obj
    }
}

#[derive(Clone, Debug)]
pub struct Strikethrough {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub reading_direction: ReadingDirection,
    pub flow_direction: FlowDirection,
    pub locale_name: String,
    pub measuring_mode: MeasuringMode,
}
impl Strikethrough {
    fn to_c_struct(&self) -> (DWRITE_STRIKETHROUGH, Vec<u16>) {
        let locale_name = self
            .locale_name
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>();
        (
            DWRITE_STRIKETHROUGH {
                width: self.width,
                thickness: self.thickness,
                offset: self.offset,
                readingDirection: self.reading_direction as u32,
                flowDirection: self.flow_direction as u32,
                localeName: locale_name.as_ptr(),
                measuringMode: self.measuring_mode as u32,
            },
            locale_name,
        )
    }
}

#[derive(Clone, Debug)]
pub struct TextMetrics {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub width_including_trailing_whitespace: f32,
    pub height: f32,
    pub layout_width: f32,
    pub layout_height: f32,
    pub max_bidi_reordering_depth: u32,
    pub line_count: u32,
}
impl TextMetrics {
    fn to_c_struct(&self) -> DWRITE_TEXT_METRICS {
        DWRITE_TEXT_METRICS {
            left: self.left,
            top: self.top,
            width: self.width,
            widthIncludingTrailingWhitespace: self.width_including_trailing_whitespace,
            height: self.height,
            layoutWidth: self.layout_width,
            layoutHeight: self.layout_height,
            maxBidiReorderingDepth: self.max_bidi_reordering_depth,
            lineCount: self.line_count,
        }
    }
}
impl From<DWRITE_TEXT_METRICS> for TextMetrics {
    fn from(src: DWRITE_TEXT_METRICS) -> TextMetrics {
        TextMetrics {
            left: src.left,
            top: src.top,
            width: src.width,
            width_including_trailing_whitespace: src.widthIncludingTrailingWhitespace,
            height: src.height,
            layout_width: src.layoutWidth,
            layout_height: src.layoutHeight,
            max_bidi_reordering_depth: src.maxBidiReorderingDepth,
            line_count: src.lineCount,
        }
    }
}

#[cfg(feature = "dwrite_2")]
#[derive(Clone, Debug)]
pub struct TextMetrics1 {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub width_including_trailing_whitespace: f32,
    pub height: f32,
    pub layout_width: f32,
    pub layout_height: f32,
    pub max_bidi_reordering_depth: u32,
    pub line_count: u32,
    pub height_including_trailing_whitespace: f32,
}
#[cfg(feature = "dwrite_2")]
impl TextMetrics1 {
    fn to_c_struct(&self) -> DWRITE_TEXT_METRICS1 {
        DWRITE_TEXT_METRICS1 {
            left: self.left,
            top: self.top,
            width: self.width,
            widthIncludingTrailingWhitespace: self.width_including_trailing_whitespace,
            height: self.height,
            layoutWidth: self.layout_width,
            layoutHeight: self.layout_height,
            maxBidiReorderingDepth: self.max_bidi_reordering_depth,
            lineCount: self.line_count,
            heightIncludingTrailingWhitespace: self.height_including_trailing_whitespace,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct TextRange {
    pub start_position: u32,
    pub length: u32,
}
impl TextRange {
    fn to_c_struct(&self) -> DWRITE_TEXT_RANGE {
        DWRITE_TEXT_RANGE {
            startPosition: self.start_position,
            length: self.length,
        }
    }
}
impl From<DWRITE_TEXT_RANGE> for TextRange {
    fn from(src: DWRITE_TEXT_RANGE) -> TextRange {
        TextRange {
            start_position: src.startPosition,
            length: src.length,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Trimming {
    pub granularity: TrimmingGranularity,
    pub delimiter: u32,
    pub delimiter_count: u32,
}
impl Trimming {
    fn to_c_struct(&self) -> DWRITE_TRIMMING {
        DWRITE_TRIMMING {
            granularity: self.granularity as u32,
            delimiter: self.delimiter,
            delimiterCount: self.delimiter_count,
        }
    }
}
impl From<DWRITE_TRIMMING> for Trimming {
    fn from(src: DWRITE_TRIMMING) -> Trimming {
        Trimming {
            granularity: match src.granularity {
                DWRITE_TRIMMING_GRANULARITY_NONE => TrimmingGranularity::None,
                DWRITE_TRIMMING_GRANULARITY_CHARACTER => TrimmingGranularity::Character,
                DWRITE_TRIMMING_GRANULARITY_WORD => TrimmingGranularity::Word,
                _ => unimplemented!(),
            },
            delimiter: src.delimiter,
            delimiter_count: src.delimiterCount,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TypographicFeatures(pub Vec<FontFeature>);
impl TypographicFeatures {
    fn to_c_struct(&self) -> (DWRITE_TYPOGRAPHIC_FEATURES, Vec<DWRITE_FONT_FEATURE>) {
        let mut features = self
            .0
            .iter()
            .map(|obj| obj.to_c_struct())
            .collect::<Vec<_>>();
        (
            DWRITE_TYPOGRAPHIC_FEATURES {
                features: features.as_mut_ptr(),
                featureCount: features.len() as u32,
            },
            features,
        )
    }
}

#[derive(Clone, Debug)]
pub struct Underline {
    pub width: f32,
    pub thickness: f32,
    pub offset: f32,
    pub run_height: f32,
    pub reading_direction: ReadingDirection,
    pub flow_direction: FlowDirection,
    pub locale_name: String,
    pub measuring_mode: MeasuringMode,
}
impl Underline {
    fn to_c_struct(&self) -> (DWRITE_UNDERLINE, Vec<u16>) {
        let locale_name = self
            .locale_name
            .encode_utf16()
            .chain(Some(0))
            .collect::<Vec<_>>();
        (
            DWRITE_UNDERLINE {
                width: self.width,
                thickness: self.thickness,
                offset: self.offset,
                runHeight: self.run_height,
                readingDirection: self.reading_direction as u32,
                flowDirection: self.flow_direction as u32,
                localeName: locale_name.as_ptr(),
                measuringMode: self.measuring_mode as u32,
            },
            locale_name,
        )
    }
}

#[cfg(feature = "dwrite_1")]
#[derive(Clone, Debug)]
pub struct UnicodeRange {
    pub first: u32,
    pub last: u32,
}
#[cfg(feature = "dwrite_1")]
impl UnicodeRange {
    fn to_c_struct(&self) -> dwrite_1::DWRITE_UNICODE_RANGE {
        dwrite_1::DWRITE_UNICODE_RANGE {
            first: self.first,
            last: self.last,
        }
    }
}

macro_rules! impl_bitmap_render_target {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IBitmapRenderTarget for $s {
            fn draw_glyph_run(
                &self,
                baseline_origin_x: f32,
                baseline_origin_y: f32,
                measuring_mode: MeasuringMode,
                glyph_run: &GlyphRun,
                rendering_params: &impl IRenderingParams,
                text_color: u32,
            ) -> Result<Rect<i32>, HResult> {
                unsafe {
                    let mut rc = RECT::default();
                    let (glyph_run, _) = glyph_run.to_c_struct();
                    let ret = self.0.DrawGlyphRun(
                        baseline_origin_x,
                        baseline_origin_y,
                        measuring_mode as u32,
                        &glyph_run,
                        rendering_params.as_ptr() as *mut IDWriteRenderingParams,
                        text_color,
                        &mut rc,
                    );
                    hresult(rc.into(), ret)
                }
            }
            fn get_current_transform(&self) -> Result<Matrix, HResult> {
                unsafe {
                    let mut m = DWRITE_MATRIX::default();
                    let ret = self.0.GetCurrentTransform(&mut m);
                    hresult(m.into(), ret)
                }
            }
            fn get_memory_dc(&self) -> *mut std::ffi::c_void {
                unsafe { self.0.GetMemoryDC() as *mut std::ffi::c_void }
            }
            fn get_pixels_per_dip(&self) -> f32 {
                unsafe { self.0.GetPixelsPerDip() }
            }
            fn get_size(&self) -> Result<Size<i32>, HResult> {
                unsafe {
                    let mut sz = SIZE::default();
                    let ret = self.0.GetSize(&mut sz);
                    hresult(sz.into(), ret)
                }
            }
            fn resize(&self, width: u32, height: u32) -> Result<(), HResult> {
                unsafe {
                    let ret = self.0.Resize(width, height);
                    hresult((), ret)
                }
            }
            fn set_current_transform(&self, m: Option<&Matrix>) -> Result<(), HResult> {
                unsafe {
                    let m = m.map(|n| n.to_c_struct());
                    let ret = self.0.SetCurrentTransform(
                        m.as_ref().map_or(std::ptr::null(), |n| n as *const _),
                    );
                    hresult((), ret)
                }
            }
            fn set_pixels_per_dip(&self, pixels_per_dip: f32) -> Result<(), HResult> {
                unsafe {
                    let ret = self.0.SetPixelsPerDip(pixels_per_dip);
                    hresult((), ret)
                }
            }
        }
    };
}

macro_rules! impl_factory {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFactory for $s {
            fn create_custom_font_collection(
                &self,
                loader: &impl IFontCollectionLoader,
                key: &[u8],
            ) -> Result<FontCollection, HResult> {
                Ok(FontCollection(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateCustomFontCollection(
                        loader.as_ptr() as *mut _,
                        key.as_ptr() as *const _,
                        key.len() as u32,
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_custom_font_file_reference(
                &self,
                key: &[u8],
                loader: &impl IFontFileLoader,
            ) -> Result<FontFile, HResult> {
                Ok(FontFile(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateCustomFontFileReference(
                        key.as_ptr() as *const _,
                        key.len() as u32,
                        loader.as_ptr() as *mut _,
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_custom_rendering_params(
                &self,
                gamma: f32,
                enchanced_contrast: f32,
                clear_type_level: f32,
                pixel_geometry: PixelGeometry,
                rendering_mode: RenderingMode,
            ) -> Result<RenderingParams, HResult> {
                Ok(RenderingParams(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateCustomRenderingParams(
                        gamma,
                        enchanced_contrast,
                        clear_type_level,
                        pixel_geometry as u32,
                        rendering_mode as u32,
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_ellipsis_trimming_sign(
                &self,
                format: &impl ITextFormat,
            ) -> Result<InlineObject, HResult> {
                Ok(InlineObject(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self
                        .0
                        .CreateEllipsisTrimmingSign(format.as_ptr() as *mut _, &mut p);
                    hresult(p, ret)
                })?))
            }
            fn create_font_face(
                &self,
                font_face_type: FontFaceType,
                font_files: &[&impl IFontFile],
                face_index: u32,
                flags: Option<FontSimulations>,
            ) -> Result<FontFace, HResult> {
                let font_files = font_files.iter().map(|p| p.as_ptr()).collect::<Vec<_>>();
                Ok(FontFace(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateFontFace(
                        font_face_type as u32,
                        font_files.len() as u32,
                        font_files.as_ptr() as *mut _,
                        face_index,
                        flags.map_or(0, |f| f.0),
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_font_file_reference(
                &self,
                path: impl AsRef<Path>,
                file_time: Option<&FileTime>,
            ) -> Result<FontFile, HResult> {
                let path = to_wstring(path.as_ref().to_str().unwrap());
                let file_time = file_time.map(|ft| ft.to_c_struct());
                Ok(FontFile(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateFontFileReference(
                        path.as_ptr(),
                        file_time
                            .as_ref()
                            .map_or(std::ptr::null(), |ft| ft as *const _),
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_gdi_compatible_text_layout(
                &self,
                string: impl AsRef<str>,
                format: &impl ITextFormat,
                width: f32,
                height: f32,
                pixels_per_dip: f32,
                transform: Option<&Matrix>,
                use_gdi_natural: bool,
            ) -> Result<TextLayout, HResult> {
                let string = to_wstring(string);
                let transform = transform.map(|t| t.to_c_struct());
                Ok(TextLayout(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateGdiCompatibleTextLayout(
                        string.as_ptr(),
                        string.len() as u32,
                        format.as_ptr() as *mut _,
                        width,
                        height,
                        pixels_per_dip,
                        transform
                            .as_ref()
                            .map_or(std::ptr::null(), |t| t as *const _),
                        to_BOOL(use_gdi_natural),
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_glyph_run_analysis(
                &self,
                glyph_run: &GlyphRun,
                pixels_per_dip: f32,
                transform: Option<&Matrix>,
                rendering_mode: RenderingMode,
                measuring_mode: MeasuringMode,
                baseline_origin_x: f32,
                baseline_origin_y: f32,
            ) -> Result<GlyphRunAnalysis, HResult> {
                let transform = transform.map(|t| t.to_c_struct());
                let (glyph_run, _) = glyph_run.to_c_struct();
                Ok(GlyphRunAnalysis(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateGlyphRunAnalysis(
                        &glyph_run,
                        pixels_per_dip,
                        transform
                            .as_ref()
                            .map_or(std::ptr::null(), |t| t as *const _),
                        rendering_mode as u32,
                        measuring_mode as u32,
                        baseline_origin_x,
                        baseline_origin_y,
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_monitor_rendering_params(
                &self,
                monitor: *mut std::ffi::c_void,
            ) -> Result<RenderingParams, HResult> {
                Ok(RenderingParams(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self
                        .0
                        .CreateMonitorRenderingParams(monitor as *mut _, &mut p);
                    hresult(p, ret)
                })?))
            }
            fn create_number_substitution(
                &self,
                substitution_method: NumberSubstitutionMethod,
                locale_name: impl AsRef<str>,
                ignore_user_override: bool,
            ) -> Result<NumberSubstitution, HResult> {
                let locale_name = to_wstring(locale_name);
                Ok(NumberSubstitution(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateNumberSubstitution(
                        substitution_method as u32,
                        locale_name.as_ptr(),
                        to_BOOL(ignore_user_override),
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_rendering_params(&self) -> Result<RenderingParams, HResult> {
                Ok(RenderingParams(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateRenderingParams(&mut p);
                    hresult(p, ret)
                })?))
            }
            /*
            fn create_text_analyzer(&self) -> Result<TextAnalyzer, HResult> {
                Ok(TextAnalyzer(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateTextAnalyzer(&mut p);
                    hresult(p, ret)
                })?))
            }
            */
            fn create_text_format(
                &self,
                family_name: impl AsRef<str>,
                collection: &impl IFontCollection,
                weight: FontWeight,
                style: FontStyle,
                stretch: FontStretch,
                size: f32,
                locale_name: impl AsRef<str>,
            ) -> Result<TextFormat, HResult> {
                let family_name = to_wstring(family_name);
                let locale_name = to_wstring(locale_name);
                Ok(TextFormat(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateTextFormat(
                        family_name.as_ptr(),
                        collection.as_ptr() as *mut _,
                        weight as u32,
                        style as u32,
                        stretch as u32,
                        size,
                        locale_name.as_ptr(),
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_text_layout(
                &self,
                string: impl AsRef<str>,
                text_format: &impl ITextFormat,
                max_width: f32,
                max_height: f32,
            ) -> Result<TextLayout, HResult> {
                let string = to_wstring(string);
                Ok(TextLayout(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateTextLayout(
                        string.as_ptr(),
                        string.len() as u32,
                        text_format.as_ptr() as *mut _,
                        max_width,
                        max_height,
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn create_typography(&self) -> Result<Typography, HResult> {
                Ok(Typography(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateTypography(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_gdi_interop(&self) -> Result<GdiInterop, HResult> {
                Ok(GdiInterop(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetGdiInterop(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_system_font_collection(
                &self,
                check_for_updates: bool,
            ) -> Result<FontCollection, HResult> {
                Ok(FontCollection(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self
                        .0
                        .GetSystemFontCollection(&mut p, to_BOOL(check_for_updates));
                    hresult(p, ret)
                })?))
            }
            fn register_font_collection_loader(
                &self,
                loader: &impl IFontCollectionLoader,
            ) -> Result<(), HResult> {
                unsafe {
                    let ret = self
                        .0
                        .RegisterFontCollectionLoader(loader.as_ptr() as *mut _);
                    hresult((), ret)
                }
            }
            fn register_font_file_loader(
                &self,
                loader: &impl IFontFileLoader,
            ) -> Result<(), HResult> {
                unsafe {
                    let ret = self.0.RegisterFontFileLoader(loader.as_ptr() as *mut _);
                    hresult((), ret)
                }
            }
            fn unregister_font_collection_loader(
                &self,
                loader: &impl IFontCollectionLoader,
            ) -> Result<(), HResult> {
                unsafe {
                    let ret = self
                        .0
                        .UnregisterFontCollectionLoader(loader.as_ptr() as *mut _);
                    hresult((), ret)
                }
            }
            fn unregister_font_file_loader(
                &self,
                loader: &impl IFontFileLoader,
            ) -> Result<(), HResult> {
                unsafe {
                    let ret = self.0.UnregisterFontFileLoader(loader.as_ptr() as *mut _);
                    hresult((), ret)
                }
            }
        }
    };
}

macro_rules! impl_font {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFont for $s {
            fn create_font_face(&self) -> Result<FontFace, HResult> {
                Ok(FontFace(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateFontFace(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_face_names(&self) -> Result<LocalizedStrings, HResult> {
                Ok(LocalizedStrings(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetFaceNames(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_font_family(&self) -> Result<FontFamily, HResult> {
                Ok(FontFamily(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetFontFamily(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_informational_strings(
                &self,
                id: InformationStringID,
            ) -> Result<(LocalizedStrings, bool), HResult> {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    let mut exists = 0;
                    let ret = self
                        .0
                        .GetInformationalStrings(id as u32, &mut p, &mut exists);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok((LocalizedStrings(ComPtr::from_raw(p)), exists == TRUE))
                }
            }
            fn get_metrics(&self) -> FontMetrics {
                unsafe {
                    let mut metrics = Default::default();
                    self.0.GetMetrics(&mut metrics);
                    metrics.into()
                }
            }
            fn get_simulations(&self) -> FontSimulations {
                unsafe { std::mem::transmute(self.0.GetSimulations()) }
            }
            fn get_stretch(&self) -> FontStretch {
                unsafe { std::mem::transmute(self.0.GetStretch()) }
            }
            fn get_style(&self) -> FontStyle {
                unsafe { std::mem::transmute(self.0.GetStyle()) }
            }
            fn get_weight(&self) -> FontWeight {
                unsafe { std::mem::transmute(self.0.GetWeight()) }
            }
            fn has_character(&self, unicode_value: u32) -> Result<bool, HResult> {
                unsafe {
                    let mut exists = 0;
                    let ret = self.0.HasCharacter(unicode_value, &mut exists);
                    hresult(exists == TRUE, ret)
                }
            }
            fn is_symbol_font(&self) -> bool {
                unsafe { self.0.IsSymbolFont() == TRUE }
            }
        }
    };
}

macro_rules! impl_font_face {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFontFace for $s {
            fn get_design_glyph_metrics(
                &self,
                indices: &[u16],
                is_sideways: bool,
            ) -> Result<Vec<GlyphMetrics>, HResult> {
                let mut metrics = Vec::new();
                metrics.resize_with(indices.len(), || Default::default());
                unsafe {
                    let ret = self.0.GetDesignGlyphMetrics(
                        indices.as_ptr(),
                        indices.len() as u32,
                        metrics.as_mut_ptr(),
                        to_BOOL(is_sideways),
                    );
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok(metrics.into_iter().map(|m| m.into()).collect::<Vec<_>>())
                }
            }
            fn get_files(&self) -> Result<Vec<FontFile>, HResult> {
                let mut len = unsafe {
                    let mut len = 0;
                    let ret = self.0.GetFiles(&mut len, std::ptr::null_mut());
                    hresult(len, ret)
                }?;
                let ptrs = unsafe {
                    let mut ptrs = vec![std::ptr::null_mut(); len as usize];
                    let ret = self.0.GetFiles(&mut len, ptrs.as_mut_ptr());
                    hresult(ptrs, ret)
                }?;
                Ok(ptrs
                    .into_iter()
                    .map(|p| unsafe { FontFile(ComPtr::from_raw(p)) })
                    .collect::<Vec<_>>())
            }
            fn get_gdi_compatible_glyph_metrics(
                &self,
                em_size: f32,
                pixels_per_dip: f32,
                transform: Option<&Matrix>,
                use_gdi_natural: bool,
                indices: &[u16],
                is_sideways: bool,
            ) -> Result<Vec<GlyphMetrics>, HResult> {
                let transform = transform.map(|t| t.to_c_struct());
                let mut metrics = vec![Default::default(); indices.len()];
                unsafe {
                    let ret = self.0.GetGdiCompatibleGlyphMetrics(
                        em_size,
                        pixels_per_dip,
                        transform
                            .as_ref()
                            .map_or(std::ptr::null(), |t| t as *const _),
                        to_BOOL(use_gdi_natural),
                        indices.as_ptr(),
                        indices.len() as u32,
                        metrics.as_mut_ptr(),
                        to_BOOL(is_sideways),
                    );
                    hresult(
                        metrics.into_iter().map(|m| m.into()).collect::<Vec<_>>(),
                        ret,
                    )
                }
            }
            fn get_gdi_compatbile_metrics(
                &self,
                em_size: f32,
                pixels_per_dip: f32,
                transform: Option<&Matrix>,
            ) -> Result<FontMetrics, HResult> {
                let transform = transform.map(|t| t.to_c_struct());
                unsafe {
                    let mut metrics = Default::default();
                    let ret = self.0.GetGdiCompatibleMetrics(
                        em_size,
                        pixels_per_dip,
                        transform
                            .as_ref()
                            .map_or(std::ptr::null(), |t| t as *const _),
                        &mut metrics,
                    );
                    hresult(metrics.into(), ret)
                }
            }
            fn get_glyph_count(&self) -> u16 {
                unsafe { self.0.GetGlyphCount() }
            }
            fn get_glyph_indices(&self, code_points: &[u32]) -> Result<Vec<u16>, HResult> {
                let mut indices = vec![0u16; code_points.len()];
                unsafe {
                    let ret = self.0.GetGlyphIndices(
                        code_points.as_ptr(),
                        code_points.len() as u32,
                        indices.as_mut_ptr(),
                    );
                    hresult(indices, ret)
                }
            }
            fn get_glyph_run_outline(
                &self,
                em_size: f32,
                indices: &[u16],
                advances: Option<&[f32]>,
                offsets: Option<&[GlyphOffset]>,
                is_sideways: bool,
                is_right_to_left: bool,
                sink: &impl IGeometrySink,
            ) -> Result<(), HResult> {
                let offsets =
                    offsets.map(|ofs| ofs.iter().map(|i| i.to_c_struct()).collect::<Vec<_>>());
                unsafe {
                    let ret = self.0.GetGlyphRunOutline(
                        em_size,
                        indices.as_ptr(),
                        advances.map_or(std::ptr::null(), |a| a.as_ptr()),
                        offsets
                            .as_ref()
                            .map_or(std::ptr::null(), |o| o.as_ptr() as *const _),
                        indices.len() as u32,
                        to_BOOL(is_sideways),
                        to_BOOL(is_right_to_left),
                        sink.as_ptr() as *mut _,
                    );
                    hresult((), ret)
                }
            }
            fn get_index(&self) -> u32 {
                unsafe { self.0.GetIndex() }
            }
            fn get_metrics(&self) -> FontMetrics {
                unsafe {
                    let mut metrics = Default::default();
                    self.0.GetMetrics(&mut metrics);
                    metrics.into()
                }
            }
            fn get_recommended_rendering_mode(
                &self,
                em_size: f32,
                pixels_per_dip: f32,
                measuring_mode: MeasuringMode,
                params: &impl IRenderingParams,
            ) -> Result<RenderingMode, HResult> {
                unsafe {
                    let mut mode = 0;
                    let ret = self.0.GetRecommendedRenderingMode(
                        em_size,
                        pixels_per_dip,
                        measuring_mode as u32,
                        params.as_ptr() as *mut _,
                        &mut mode,
                    );
                    hresult(std::mem::transmute(mode), ret)
                }
            }
            fn get_simulations(&self) -> FontSimulations {
                unsafe { FontSimulations(self.0.GetSimulations()) }
            }
            fn get_type(&self) -> FontFaceType {
                unsafe { std::mem::transmute(self.0.GetType()) }
            }
            fn is_symbol_font(&self) -> bool {
                unsafe { self.0.IsSymbolFont() == TRUE }
            }
            fn try_get_font_table(&self, tag: u32) -> Result<Option<Vec<u8>>, HResult> {
                unsafe {
                    let mut data = std::ptr::null();
                    let mut size = 0;
                    let mut context = std::ptr::null_mut();
                    let mut exists = 0;
                    let ret = self.0.TryGetFontTable(
                        tag,
                        &mut data,
                        &mut size,
                        &mut context,
                        &mut exists,
                    );
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    if exists != TRUE {
                        return Ok(None);
                    }
                    let p = std::slice::from_raw_parts(data as *const u8, size as usize);
                    let v = p.to_vec();
                    self.0.ReleaseFontTable(context);
                    Ok(Some(v))
                }
            }
        }
    };
}

macro_rules! impl_font_collection {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFontCollection for $s {
            fn find_family_name(
                &self,
                family_name: impl AsRef<str>,
            ) -> Result<(u32, bool), HResult> {
                let family_name = to_wstring(family_name);
                unsafe {
                    let mut index = 0;
                    let mut exists = 0;
                    let ret = self
                        .0
                        .FindFamilyName(family_name.as_ptr(), &mut index, &mut exists);
                    hresult((index, exists == TRUE), ret)
                }
            }
            fn get_font_family(&self, index: u32) -> Result<FontFamily, HResult> {
                Ok(FontFamily(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetFontFamily(index, &mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_font_family_count(&self) -> u32 {
                unsafe { self.0.GetFontFamilyCount() }
            }
            fn get_font_from_font_face(&self, font_face: &impl IFontFace) -> Result<Font, HResult> {
                Ok(Font(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self
                        .0
                        .GetFontFromFontFace(font_face.as_ptr() as *mut _, &mut p);
                    hresult(p, ret)
                })?))
            }
        }
    };
}

macro_rules! impl_font_collection_loader {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFontCollectionLoader for $s {
            fn create_enumerator_from_key(
                &self,
                factory: &impl IFactory,
                key: &[u8],
            ) -> Result<FontFileEnumerator, HResult> {
                Ok(FontFileEnumerator(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateEnumeratorFromKey(
                        factory.as_ptr() as *mut _,
                        key.as_ptr() as *const _,
                        key.len() as u32,
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
        }
    };
}

macro_rules! impl_font_family {
    ($s: ident, $interface: ident) => {
        impl_font_list!($s, $interface);
        impl IFontFamily for $s {
            fn get_family_names(&self) -> Result<LocalizedStrings, HResult> {
                Ok(LocalizedStrings(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetFamilyNames(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_first_matching_font(
                &self,
                weight: FontWeight,
                stretch: FontStretch,
                style: FontStyle,
            ) -> Result<Font, HResult> {
                Ok(Font(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetFirstMatchingFont(
                        weight as u32,
                        stretch as u32,
                        style as u32,
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
            fn get_matching_fonts(
                &self,
                weight: FontWeight,
                stretch: FontStretch,
                style: FontStyle,
            ) -> Result<FontList, HResult> {
                Ok(FontList(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetMatchingFonts(
                        weight as u32,
                        stretch as u32,
                        style as u32,
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
        }
    };
}

macro_rules! impl_font_file {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFontFile for $s {
            fn analyze(&self) -> Result<AnalyzeResult, HResult> {
                unsafe {
                    let mut is_supported_font_type = 0;
                    let mut font_file_type = 0;
                    let mut font_face_type = 0;
                    let mut number_of_faces = 0;
                    let ret = self.0.Analyze(
                        &mut is_supported_font_type,
                        &mut font_file_type,
                        &mut font_face_type,
                        &mut number_of_faces,
                    );
                    hresult(
                        AnalyzeResult {
                            is_supported_font_type: is_supported_font_type == TRUE,
                            font_file_type: std::mem::transmute(font_file_type),
                            font_face_type: std::mem::transmute(font_face_type),
                            number_of_faces,
                        },
                        ret,
                    )
                }
            }
            fn get_loader(&self) -> Result<FontFileLoader, HResult> {
                Ok(FontFileLoader(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetLoader(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_reference_key(&self) -> Result<Vec<u8>, HResult> {
                unsafe {
                    let mut p = std::ptr::null();
                    let mut size = 0;
                    let ret = self.0.GetReferenceKey(&mut p, &mut size);
                    hresult(
                        std::slice::from_raw_parts(p as *const u8, size as usize).to_vec(),
                        ret,
                    )
                }
            }
        }
    };
}

macro_rules! impl_font_file_enumerator {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFontFileEnumerator for $s {
            fn get_current_font_file(&self) -> Result<FontFile, HResult> {
                Ok(FontFile(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetCurrentFontFile(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn move_next(&self) -> Result<bool, HResult> {
                unsafe {
                    let mut has_current_file = 0;
                    let ret = self.0.MoveNext(&mut has_current_file);
                    hresult(has_current_file == TRUE, ret)
                }
            }
        }
    };
}

macro_rules! impl_font_file_loader {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFontFileLoader for $s {
            fn create_stream_from_key(&self, key: &[u8]) -> Result<FontFileStream, HResult> {
                Ok(FontFileStream(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.CreateStreamFromKey(
                        key.as_ptr() as *const _,
                        key.len() as u32,
                        &mut p,
                    );
                    hresult(p, ret)
                })?))
            }
        }
    };
}

macro_rules! impl_font_file_stream {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFontFileStream for $s {
            fn get_file_size(&self) -> Result<u64, HResult> {
                unsafe {
                    let mut size = 0;
                    let ret = self.0.GetFileSize(&mut size);
                    hresult(size, ret)
                }
            }
            fn get_last_write_time(&self) -> Result<u64, HResult> {
                unsafe {
                    let mut size = 0;
                    let ret = self.0.GetLastWriteTime(&mut size);
                    hresult(size, ret)
                }
            }
            fn read_file_fragment(&self, offset: u64, size: u64) -> Result<Vec<u8>, HResult> {
                unsafe {
                    let mut start = std::ptr::null();
                    let mut context = std::ptr::null_mut();
                    let ret = self
                        .0
                        .ReadFileFragment(&mut start, offset, size, &mut context);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    let v = std::slice::from_raw_parts(start as *const u8, size as usize).to_vec();
                    self.0.ReleaseFileFragment(context);
                    Ok(v)
                }
            }
        }
    };
}

macro_rules! impl_font_list {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IFontList for $s {
            fn get_font(&self, index: u32) -> Result<Font, HResult> {
                Ok(Font(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetFont(index, &mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_font_collection(&self) -> Result<FontCollection, HResult> {
                Ok(FontCollection(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetFontCollection(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_font_count(&self) -> u32 {
                unsafe { self.0.GetFontCount() }
            }
        }
    };
}

macro_rules! impl_gdi_interop {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IGdiInterop for $s {}
    };
}

macro_rules! impl_glyph_run_analysis {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IGlyphRunAnalysis for $s {
            fn create_alpha_texture(
                &self,
                texture_type: TextureType,
                bounds: impl Into<Rect<i32>>,
            ) -> Result<Vec<u8>, HResult> {
                let bounds = bounds.into();
                let width = bounds.right - bounds.left;
                let height = bounds.bottom - bounds.top;
                let len = width
                    * height
                    * match texture_type {
                        TextureType::Aliased1x1 => 1,
                        TextureType::ClearType3x1 => 3,
                    };
                let mut alpha_values = vec![0; len as usize];
                unsafe {
                    let ret = self.0.CreateAlphaTexture(
                        texture_type as u32,
                        bounds.as_ref() as *const _,
                        alpha_values.as_mut_ptr(),
                        len as u32,
                    );
                    hresult(alpha_values, ret)
                }
            }
            fn get_alpha_blend_params(
                &self,
                params: &impl IRenderingParams,
            ) -> Result<AlphaBlendParams, HResult> {
                unsafe {
                    let mut gamma = 0.0;
                    let mut enhanced_contrast = 0.0;
                    let mut clear_type_level = 0.0;
                    let ret = self.0.GetAlphaBlendParams(
                        params.as_ptr() as *mut _,
                        &mut gamma,
                        &mut enhanced_contrast,
                        &mut clear_type_level,
                    );
                    hresult(
                        AlphaBlendParams {
                            gamma,
                            enhanced_contrast,
                            clear_type_level,
                        },
                        ret,
                    )
                }
            }
            fn get_alpha_texture_bounds(&self, texture_type: TextureType) -> Result<Rect<i32>, HResult> {
                unsafe {
                    let mut rc = Default::default();
                    let ret = self.0.GetAlphaTextureBounds(texture_type as u32, &mut rc);
                    hresult(rc.into(), ret)
                }
            }
        }
    };
}

macro_rules! impl_inline_object {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IInlineObject for $s {
            fn draw(
                &self,
                context: Option<*mut std::ffi::c_void>,
                renderer: &impl ITextRenderer,
                origin_x: f32,
                origin_y: f32,
                is_sideways: bool,
                is_right_to_left: bool,
                effect: Option<Unknown>,
            ) -> Result<(), HResult> {
                let context = context.unwrap_or(std::ptr::null_mut());
                let effect = effect.map_or(std::ptr::null_mut(), |e| e.as_ptr());
                unsafe {
                    let ret = self.0.Draw(
                        context,
                        renderer.as_ptr() as *mut _,
                        origin_x,
                        origin_y,
                        to_BOOL(is_sideways),
                        to_BOOL(is_right_to_left),
                        effect,
                    );
                    hresult((), ret)
                }
            }
            fn get_break_conditions(&self) -> Result<GetBreakConditionsResult, HResult> {
                unsafe {
                    let mut before = 0;
                    let mut after = 0;
                    let ret = self.0.GetBreakConditions(&mut before, &mut after);
                    hresult(
                        GetBreakConditionsResult {
                            before: std::mem::transmute(before),
                            after: std::mem::transmute(after),
                        },
                        ret,
                    )
                }
            }
            fn get_metrics(&self) -> Result<InlineObjectMetrics, HResult> {
                let mut metrics = Default::default();
                unsafe {
                    let ret = self.0.GetMetrics(&mut metrics);
                    hresult(metrics.into(), ret)
                }
            }
            fn get_overhang_metrics(&self) -> Result<OverhangMetrics, HResult> {
                unsafe {
                    let mut overhangs = Default::default();
                    let ret = self.0.GetOverhangMetrics(&mut overhangs);
                    hresult(overhangs.into(), ret)
                }
            }
        }
    };
}

macro_rules! impl_local_font_file_loader {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl ILocalFontFileLoader for $s {
            fn get_file_path_from_key(&self, key: &[u8]) -> Result<PathBuf, HResult> {
                let len = self.get_file_path_length_from_key(key)?;
                let mut wstring = vec![0; len as usize];
                unsafe {
                    let ret = self.0.GetFilePathFromKey(
                        key.as_ptr() as *const _,
                        key.len() as u32,
                        wstring.as_mut_ptr(),
                        wstring.len() as u32,
                    );
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok(to_string(&wstring).into())
                }
            }
            fn get_file_path_length_from_key(&self, key: &[u8]) -> Result<u32, HResult> {
                unsafe {
                    let mut len = 0;
                    let ret = self.0.GetFilePathLengthFromKey(
                        key.as_ptr() as *const _,
                        key.len() as u32,
                        &mut len,
                    );
                    hresult(len, ret)
                }
            }
            fn get_last_write_time_from_key(&self, key: &[u8]) -> Result<FileTime, HResult> {
                unsafe {
                    let mut ft = Default::default();
                    let ret = self.0.GetLastWriteTimeFromKey(
                        key.as_ptr() as *const _,
                        key.len() as u32,
                        &mut ft,
                    );
                    hresult(ft.into(), ret)
                }
            }
        }
    };
}

macro_rules! impl_localized_strings {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl ILocalizedStrings for $s {
            fn find_locale_name(
                &self,
                locale_name: impl AsRef<str>,
            ) -> Result<Option<u32>, HResult> {
                let locale_name = to_wstring(locale_name);
                unsafe {
                    let mut index = 0;
                    let mut exists = 0;
                    let ret = self
                        .0
                        .FindLocaleName(locale_name.as_ptr(), &mut index, &mut exists);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    if exists != TRUE {
                        return Ok(None);
                    }
                    Ok(Some(index))
                }
            }
            fn get_count(&self) -> u32 {
                unsafe { self.0.GetCount() }
            }
            fn get_locale_name(&self, index: u32) -> Result<String, HResult> {
                let len = self.get_locale_name_length(index)? + 1;
                let mut wstring = vec![0; len as usize];
                unsafe {
                    let ret = self.0.GetLocaleName(index, wstring.as_mut_ptr(), len);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok(to_string(&wstring))
                }
            }
            fn get_locale_name_length(&self, index: u32) -> Result<u32, HResult> {
                unsafe {
                    let mut len = 0;
                    let ret = self.0.GetLocaleNameLength(index, &mut len);
                    hresult(len, ret)
                }
            }
            fn get_string(&self, index: u32) -> Result<String, HResult> {
                let len = self.get_string_length(index)? + 1;
                let mut wstring = vec![0; len as usize];
                unsafe {
                    let ret = self.0.GetString(index, wstring.as_mut_ptr(), len);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok(to_string(&wstring))
                }
            }
            fn get_string_length(&self, index: u32) -> Result<u32, HResult> {
                unsafe {
                    let mut len = 0;
                    let ret = self.0.GetStringLength(index, &mut len);
                    hresult(len, ret)
                }
            }
        }
    };
}

macro_rules! impl_number_substitution {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl INumberSubstitution for $s {}
    };
}

macro_rules! impl_pixel_snapping {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IPixelSnapping for $s {
            fn get_current_transform(
                &self,
                context: Option<*mut std::ffi::c_void>,
            ) -> Result<Matrix, HResult> {
                let context = context.unwrap_or(std::ptr::null_mut());
                unsafe {
                    let mut m = Default::default();
                    let ret = self.0.GetCurrentTransform(context, &mut m);
                    hresult(m.into(), ret)
                }
            }
            fn get_pixels_per_dip(
                &self,
                context: Option<*mut std::ffi::c_void>,
            ) -> Result<f32, HResult> {
                let context = context.unwrap_or(std::ptr::null_mut());
                unsafe {
                    let mut ppd = 0.0;
                    let ret = self.0.GetPixelsPerDip(context, &mut ppd);
                    hresult(ppd, ret)
                }
            }
        }
    };
}

macro_rules! impl_rendering_params {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl IRenderingParams for $s {
            fn get_clear_type_level(&self) -> f32 {
                unsafe { self.0.GetClearTypeLevel() }
            }
            fn get_enhanced_contrast(&self) -> f32 {
                unsafe { self.0.GetEnhancedContrast() }
            }
            fn get_gamma(&self) -> f32 {
                unsafe { self.0.GetGamma() }
            }
            fn get_pixel_geometry(&self) -> PixelGeometry {
                unsafe { std::mem::transmute(self.0.GetPixelGeometry()) }
            }
            fn get_rendering_mode(&self) -> RenderingMode {
                unsafe { std::mem::transmute(self.0.GetRenderingMode()) }
            }
        }
    };
}

/*
macro_rules! impl_text_analysis_sink {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl ITextAnalysisSink for $s {
            fn set_bidi_level(&self, text_position: u32, text_length: u32, explicit_level: u8, resolved_level: u8) -> Result<(), HResult> {
                unsafe {
                    let ret = self.0.SetBidiLevel(text_position, text_length, explicit_level, resolved_level);
                    hresult((), ret)
                }
            }
            fn set_line_breakpoints(&self, text_position: u32, text_length: u32, line_breakpoints: &LineBreakpoint) -> Result<(), HResult> {
                unsafe {
                    let ret = self.0.SetLineBreakpoints(text_position, text_length, &line_breakpoints.to_c_struct());
                    hresult((), ret)
                }
            }
            fn set_number_substitution(&self, text_position: u32, text_length: u32, number_substitution: &impl INumberSubstitution) -> Result<(), HResult> {
                unsafe {
                    let ret = self.0.SetNumberSubstitution(text_position, text_length, number_substitution.as_ptr() as *mut _);
                    hresult((), ret)
                }
            }
            fn set_script_analysis(&self, text_position: u32, text_length: u32, script_analysis: &ScriptAnalysis) -> Result<(), HResult> {
                unsafe {
                    let ret = self.0.SetScriptAnalysis(text_position, text_length, &script_analysis.to_c_struct());
                    hresult((), ret)
                }
            }
        }
    };
}
*/

#[inline]
fn as_text_format(p: &impl Interface) -> &IDWriteTextFormat {
    unsafe { (p.as_ptr() as *mut IDWriteTextFormat).as_ref().unwrap() }
}

macro_rules! impl_text_format {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl ITextFormat for $s {
            fn get_flow_direction(&self) -> FlowDirection {
                unsafe { std::mem::transmute(self.0.GetFlowDirection()) }
            }
            fn get_font_collection(&self) -> Result<FontCollection, HResult> {
                Ok(FontCollection(ComPtr::new(|| unsafe {
                    let mut p = std::ptr::null_mut();
                    let ret = as_text_format(self).GetFontCollection(&mut p);
                    hresult(p, ret)
                })?))
            }
            fn get_font_family_name(&self) -> Result<String, HResult> {
                let len = <Self as ITextFormat>::get_font_family_name_length(self) + 1;
                unsafe {
                    let mut name = vec![0; len as usize];
                    let ret = as_text_format(self).GetFontFamilyName(name.as_mut_ptr(), len);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    name.pop();
                    Ok(to_string(&name))
                }
            }
            fn get_font_family_name_length(&self) -> u32 {
                unsafe { as_text_format(self).GetFontFamilyNameLength() }
            }
            fn get_font_size(&self) -> f32 {
                unsafe { as_text_format(self).GetFontSize() }
            }
            fn get_font_stretch(&self) -> FontStretch {
                unsafe { std::mem::transmute(as_text_format(self).GetFontStretch()) }
            }
            fn get_font_style(&self) -> FontStyle {
                unsafe { std::mem::transmute(as_text_format(self).GetFontStyle()) }
            }
            fn get_font_weight(&self) -> FontWeight {
                unsafe { std::mem::transmute(as_text_format(self).GetFontWeight()) }
            }
            fn get_incremental_tab_stop(&self) -> f32 {
                unsafe { as_text_format(self).GetIncrementalTabStop() }
            }
            fn get_line_spacing(&self) -> Result<GetLineSpacingResult, HResult> {
                unsafe {
                    let mut method = 0;
                    let mut line_spacing = 0.0;
                    let mut baseline = 0.0;
                    let ret = as_text_format(self).GetLineSpacing(
                        &mut method,
                        &mut line_spacing,
                        &mut baseline,
                    );
                    hresult(
                        GetLineSpacingResult {
                            method: std::mem::transmute(method),
                            line_spacing,
                            baseline,
                        },
                        ret,
                    )
                }
            }
            fn get_locale_name(&self) -> Result<String, HResult> {
                let len = <Self as ITextFormat>::get_locale_name_length(self) + 1;
                unsafe {
                    let mut name = vec![0; len as usize];
                    let ret = as_text_format(self).GetLocaleName(name.as_mut_ptr(), len as u32);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    name.pop();
                    Ok(to_string(&name))
                }
            }
            fn get_locale_name_length(&self) -> u32 {
                unsafe { as_text_format(self).GetLocaleNameLength() as u32 }
            }
            fn get_paragraph_alignment(&self) -> ParagraphAlignment {
                unsafe { std::mem::transmute(self.0.GetParagraphAlignment()) }
            }
            fn get_reading_direction(&self) -> ReadingDirection {
                unsafe { std::mem::transmute(self.0.GetReadingDirection()) }
            }
            fn get_text_alignment(&self) -> TextAlignment {
                unsafe { std::mem::transmute(self.0.GetTextAlignment()) }
            }
            fn get_trimming(&self) -> Result<GetTrimmingResult, HResult> {
                unsafe {
                    let mut trimming = Default::default();
                    let mut p = std::ptr::null_mut();
                    let ret = self.0.GetTrimming(&mut trimming, &mut p);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok(GetTrimmingResult {
                        trimming: trimming.into(),
                        inline_object: InlineObject(ComPtr::from_raw(p)),
                    })
                }
            }
            fn get_word_wrapping(&self) -> WordWrapping {
                unsafe { std::mem::transmute(self.0.GetWordWrapping()) }
            }
            fn set_flow_direction(&self, flow_direction: FlowDirection) -> Result<(), HResult> {
                unsafe { hresult((), self.0.SetFlowDirection(flow_direction as u32)) }
            }
            fn set_incremental_tab_stop(&self, tab_stop: f32) -> Result<(), HResult> {
                unsafe { hresult((), self.0.SetIncrementalTabStop(tab_stop)) }
            }
            fn set_line_spacing(
                &self,
                method: LineSpacingMethod,
                line_spacing: f32,
                baseline: f32,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0.SetLineSpacing(method as u32, line_spacing, baseline),
                    )
                }
            }
            fn set_paragraph_alignment(
                &self,
                alignment: ParagraphAlignment,
            ) -> Result<(), HResult> {
                unsafe { hresult((), self.0.SetParagraphAlignment(alignment as u32)) }
            }
            fn set_text_alignment(&self, alignment: TextAlignment) -> Result<(), HResult> {
                unsafe { hresult((), self.0.SetTextAlignment(alignment as u32)) }
            }
            fn set_trimming(
                &self,
                options: &Trimming,
                sign: &impl IInlineObject,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0
                            .SetTrimming(&options.to_c_struct(), sign.as_ptr() as *mut _),
                    )
                }
            }
            fn set_word_wrapping(&self, word_wrapping: WordWrapping) -> Result<(), HResult> {
                unsafe { hresult((), self.0.SetWordWrapping(word_wrapping as u32)) }
            }
        }
    };
}

#[inline]
fn as_text_layout(p: &impl Interface) -> &IDWriteTextLayout {
    unsafe { (p.as_ptr() as *mut IDWriteTextLayout).as_ref().unwrap() }
}

macro_rules! impl_text_layout {
    ($s: ident, $interface: ident) => {
        impl_text_format!($s, $interface);
        impl ITextLayout for $s {
            fn determine_min_width(&self) -> Result<f32, HResult> {
                unsafe {
                    let mut width = 0.0;
                    let ret = self.0.DetermineMinWidth(&mut width);
                    hresult(width, ret)
                }
            }
            fn draw(
                &self,
                context: Option<*mut std::ffi::c_void>,
                renderer: &impl ITextRenderer,
                origin_x: f32,
                origin_y: f32,
            ) -> Result<(), HResult> {
                let context = context.unwrap_or(std::ptr::null_mut());
                unsafe {
                    let ret = self
                        .0
                        .Draw(context, renderer.as_ptr() as *mut _, origin_x, origin_y);
                    hresult((), ret)
                }
            }
            fn get_cluster_metrics(&self) -> Result<Vec<ClusterMetrics>, HResult> {
                unsafe {
                    let mut metrics = vec![Default::default(); 1];
                    let mut actual = 0;
                    let ret = self
                        .0
                        .GetClusterMetrics(metrics.as_mut_ptr(), 1, &mut actual);
                    if ret == S_OK {
                        return Ok(metrics.into_iter().map(|m| m.into()).collect::<Vec<_>>());
                    }
                    if ret != HRESULT_FROM_WIN32(ERROR_INSUFFICIENT_BUFFER) {
                        return Err(ret.into());
                    }
                    let mut metrics = vec![Default::default(); actual as usize];
                    let ret = self
                        .0
                        .GetClusterMetrics(metrics.as_mut_ptr(), actual, &mut actual);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok(metrics.into_iter().map(|m| m.into()).collect::<Vec<_>>())
                }
            }
            fn get_drawing_effect(
                &self,
                current_position: u32,
            ) -> Result<(Unknown, TextRange), HResult> {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    let mut range = Default::default();
                    let ret = self
                        .0
                        .GetDrawingEffect(current_position, &mut p, &mut range);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok((
                        <Unknown as Interface>::new(ComPtr::from_raw(p)),
                        range.into(),
                    ))
                }
            }
            fn get_font_collection(
                &self,
                current_position: u32,
            ) -> Result<(FontCollection, TextRange), HResult> {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    let mut range = Default::default();
                    let ret = self
                        .0
                        .GetFontCollection(current_position, &mut p, &mut range);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok((FontCollection(ComPtr::from_raw(p)), range.into()))
                }
            }
            fn get_font_family_name(
                &self,
                current_position: u32,
            ) -> Result<(String, TextRange), HResult> {
                let (len, _) =
                    <Self as ITextLayout>::get_font_family_name_length(self, current_position)?;
                unsafe {
                    let mut wstring = vec![0; len as usize + 1];
                    let mut range = Default::default();
                    let ret = as_text_layout(self).GetFontFamilyName(
                        current_position,
                        wstring.as_mut_ptr(),
                        wstring.len() as u32,
                        &mut range,
                    );
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    wstring.pop();
                    Ok((to_string(&wstring), range.into()))
                }
            }
            fn get_font_family_name_length(
                &self,
                current_position: u32,
            ) -> Result<(u32, TextRange), HResult> {
                unsafe {
                    let mut len = 0;
                    let mut range = Default::default();
                    let ret = as_text_layout(self).GetFontFamilyNameLength(
                        current_position,
                        &mut len,
                        &mut range,
                    );
                    hresult((len, range.into()), ret)
                }
            }
            fn get_font_size(&self, current_position: u32) -> Result<(f32, TextRange), HResult> {
                unsafe {
                    let mut size = 0.0;
                    let mut range = Default::default();
                    let ret =
                        as_text_layout(self).GetFontSize(current_position, &mut size, &mut range);
                    hresult((size, range.into()), ret)
                }
            }
            fn get_font_stretch(
                &self,
                current_position: u32,
            ) -> Result<(FontStretch, TextRange), HResult> {
                unsafe {
                    let mut stretch = 0;
                    let mut range = Default::default();
                    let ret = as_text_layout(self).GetFontStretch(
                        current_position,
                        &mut stretch,
                        &mut range,
                    );
                    hresult((std::mem::transmute(stretch), range.into()), ret)
                }
            }
            fn get_font_style(
                &self,
                current_position: u32,
            ) -> Result<(FontStyle, TextRange), HResult> {
                unsafe {
                    let mut style = 0;
                    let mut range = Default::default();
                    let ret =
                        as_text_layout(self).GetFontStyle(current_position, &mut style, &mut range);
                    hresult((std::mem::transmute(style), range.into()), ret)
                }
            }
            fn get_font_weight(
                &self,
                current_position: u32,
            ) -> Result<(FontWeight, TextRange), HResult> {
                unsafe {
                    let mut weight = 0;
                    let mut range = Default::default();
                    let ret = as_text_layout(self).GetFontStyle(
                        current_position,
                        &mut weight,
                        &mut range,
                    );
                    hresult((std::mem::transmute(weight), range.into()), ret)
                }
            }
            fn get_inline_object(
                &self,
                current_position: u32,
            ) -> Result<(InlineObject, TextRange), HResult> {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    let mut range = Default::default();
                    let ret = self.0.GetInlineObject(current_position, &mut p, &mut range);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok((InlineObject(ComPtr::from_raw(p)), range.into()))
                }
            }
            fn get_line_metrics(&self) -> Result<Vec<LineMetrics>, HResult> {
                unsafe {
                    let mut metrics = vec![Default::default(); 1];
                    let mut actual = 0;
                    let ret = self.0.GetLineMetrics(metrics.as_mut_ptr(), 1, &mut actual);
                    if ret == S_OK {
                        return Ok(metrics.into_iter().map(|m| m.into()).collect::<Vec<_>>());
                    }
                    if ret != HRESULT_FROM_WIN32(ERROR_INSUFFICIENT_BUFFER) {
                        return Err(ret.into());
                    }
                    let mut metrics = vec![Default::default(); actual as usize];
                    let ret = self
                        .0
                        .GetLineMetrics(metrics.as_mut_ptr(), actual, &mut actual);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok(metrics.into_iter().map(|m| m.into()).collect::<Vec<_>>())
                }
            }
            fn get_locale_name(
                &self,
                current_position: u32,
            ) -> Result<(String, TextRange), HResult> {
                let (len, _) =
                    <Self as ITextLayout>::get_locale_name_length(self, current_position)?;
                unsafe {
                    let mut wstring = vec![0; len as usize + 1];
                    let mut range = Default::default();
                    let ret = as_text_layout(self).GetLocaleName(
                        current_position,
                        wstring.as_mut_ptr(),
                        wstring.len() as u32,
                        &mut range,
                    );
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok((to_string(&wstring), range.into()))
                }
            }
            fn get_locale_name_length(
                &self,
                current_position: u32,
            ) -> Result<(u32, TextRange), HResult> {
                unsafe {
                    let mut len = 0;
                    let mut range = Default::default();
                    let ret = as_text_layout(self).GetLocaleNameLength(
                        current_position,
                        &mut len,
                        &mut range,
                    );
                    hresult((len, range.into()), ret)
                }
            }
            fn get_max_height(&self) -> f32 {
                unsafe { self.0.GetMaxHeight() }
            }
            fn get_max_width(&self) -> f32 {
                unsafe { self.0.GetMaxWidth() }
            }
            fn get_metrics(&self) -> Result<TextMetrics, HResult> {
                unsafe {
                    let mut metrics = Default::default();
                    let ret = self.0.GetMetrics(&mut metrics);
                    hresult(metrics.into(), ret)
                }
            }
            fn get_overhang_metrics(&self) -> Result<OverhangMetrics, HResult> {
                unsafe {
                    let mut metrics = Default::default();
                    let ret = self.0.GetOverhangMetrics(&mut metrics);
                    hresult(metrics.into(), ret)
                }
            }
            fn get_strikethrough(
                &self,
                current_position: u32,
            ) -> Result<(bool, TextRange), HResult> {
                unsafe {
                    let mut has = 0;
                    let mut range = Default::default();
                    let ret = self
                        .0
                        .GetStrikethrough(current_position, &mut has, &mut range);
                    hresult((has == TRUE, range.into()), ret)
                }
            }
            fn get_typography(
                &self,
                current_position: u32,
            ) -> Result<(Typography, TextRange), HResult> {
                unsafe {
                    let mut p = std::ptr::null_mut();
                    let mut range = Default::default();
                    let ret = self.0.GetTypography(current_position, &mut p, &mut range);
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok((Typography(ComPtr::from_raw(p)), range.into()))
                }
            }
            fn get_underline(&self, current_position: u32) -> Result<(bool, TextRange), HResult> {
                unsafe {
                    let mut has = 0;
                    let mut range = Default::default();
                    let ret = self.0.GetUnderline(current_position, &mut has, &mut range);
                    hresult((has == TRUE, range.into()), ret)
                }
            }
            fn hit_test_point(&self, x: f32, y: f32) -> Result<HitTestPointResult, HResult> {
                unsafe {
                    let mut is_trailing_hit = 0;
                    let mut is_inside = 0;
                    let mut metrics = Default::default();
                    let ret = self.0.HitTestPoint(
                        x,
                        y,
                        &mut is_trailing_hit,
                        &mut is_inside,
                        &mut metrics,
                    );
                    hresult(
                        HitTestPointResult {
                            is_trailing_hit: is_trailing_hit == TRUE,
                            is_inside: is_inside == TRUE,
                            metrics: metrics.into(),
                        },
                        ret,
                    )
                }
            }
            fn hit_test_text_position(
                &self,
                text_position: u32,
                is_trailing_hit: bool,
            ) -> Result<HitTestTextPositionResult, HResult> {
                unsafe {
                    let mut x = 0.0;
                    let mut y = 0.0;
                    let mut metrics = Default::default();
                    let ret = self.0.HitTestTextPosition(
                        text_position,
                        to_BOOL(is_trailing_hit),
                        &mut x,
                        &mut y,
                        &mut metrics,
                    );
                    hresult(
                        HitTestTextPositionResult {
                            x,
                            y,
                            metrics: metrics.into(),
                        },
                        ret,
                    )
                }
            }
            fn hit_test_text_range(
                &self,
                text_position: u32,
                text_length: u32,
                origin_x: f32,
                origin_y: f32,
            ) -> Result<Vec<HitTestMetrics>, HResult> {
                unsafe {
                    let mut metrics = vec![Default::default(); 1];
                    let mut actual = 0;
                    let ret = self.0.HitTestTextRange(
                        text_position,
                        text_length,
                        origin_x,
                        origin_y,
                        metrics.as_mut_ptr(),
                        1,
                        &mut actual,
                    );
                    if ret == S_OK {
                        return Ok(metrics.into_iter().map(|m| m.into()).collect::<Vec<_>>());
                    }
                    if ret != HRESULT_FROM_WIN32(ERROR_INSUFFICIENT_BUFFER) {
                        return Err(ret.into());
                    }
                    let mut metrics = vec![Default::default(); actual as usize];
                    let ret = self.0.HitTestTextRange(
                        text_position,
                        text_length,
                        origin_x,
                        origin_y,
                        metrics.as_mut_ptr(),
                        actual,
                        &mut actual,
                    );
                    if ret != S_OK {
                        return Err(ret.into());
                    }
                    Ok(metrics.into_iter().map(|m| m.into()).collect::<Vec<_>>())
                }
            }
            fn set_drawing_effect(
                &self,
                effect: &impl Interface,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0
                            .SetDrawingEffect(effect.as_ptr() as *mut _, text_range.to_c_struct()),
                    )
                }
            }
            fn set_font_collection(
                &self,
                collection: &impl IFontCollection,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0.SetFontCollection(
                            collection.as_ptr() as *mut _,
                            text_range.to_c_struct(),
                        ),
                    )
                }
            }
            fn set_font_family_name(
                &self,
                family_name: impl AsRef<str>,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                let family_name = to_wstring(family_name);
                unsafe {
                    hresult(
                        (),
                        self.0
                            .SetFontFamilyName(family_name.as_ptr(), text_range.to_c_struct()),
                    )
                }
            }
            fn set_font_size(&self, size: f32, text_range: TextRange) -> Result<(), HResult> {
                unsafe { hresult((), self.0.SetFontSize(size, text_range.to_c_struct())) }
            }
            fn set_font_stretch(
                &self,
                stretch: FontStretch,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0
                            .SetFontStretch(stretch as u32, text_range.to_c_struct()),
                    )
                }
            }
            fn set_font_style(
                &self,
                style: FontStyle,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0.SetFontStyle(style as u32, text_range.to_c_struct()),
                    )
                }
            }
            fn set_font_weight(
                &self,
                weight: FontWeight,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0
                            .SetFontWeight(weight as u32, text_range.to_c_struct()),
                    )
                }
            }
            fn set_inline_object(
                &self,
                inline_object: &impl IInlineObject,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0.SetInlineObject(
                            inline_object.as_ptr() as *mut _,
                            text_range.to_c_struct(),
                        ),
                    )
                }
            }
            fn set_locale_name(
                &self,
                locale_name: impl AsRef<str>,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                let locale_name = to_wstring(&locale_name);
                unsafe {
                    hresult(
                        (),
                        self.0
                            .SetLocaleName(locale_name.as_ptr(), text_range.to_c_struct()),
                    )
                }
            }
            fn set_max_height(&self, height: f32) -> Result<(), HResult> {
                unsafe { hresult((), self.0.SetMaxHeight(height)) }
            }
            fn set_max_width(&self, width: f32) -> Result<(), HResult> {
                unsafe { hresult((), self.0.SetMaxWidth(width)) }
            }
            fn set_strikethrough(
                &self,
                has_strikethrough: bool,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0
                            .SetStrikethrough(to_BOOL(has_strikethrough), text_range.to_c_struct()),
                    )
                }
            }
            fn set_typography(
                &self,
                typography: &impl ITypography,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0
                            .SetTypography(typography.as_ptr() as *mut _, text_range.to_c_struct()),
                    )
                }
            }
            fn set_underline(
                &self,
                has_underline: bool,
                text_range: TextRange,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0
                            .SetUnderline(to_BOOL(has_underline), text_range.to_c_struct()),
                    )
                }
            }
        }
    };
}

macro_rules! impl_text_renderer {
    ($s: ident, $interface: ident) => {
        impl_pixel_snapping!($s, $interface);
        impl ITextRenderer for $s {
            fn draw_glyph_run(
                &self,
                context: Option<*mut std::ffi::c_void>,
                baseline_origin_x: f32,
                baseline_origin_y: f32,
                measuring_mode: MeasuringMode,
                glyph_run: &GlyphRun,
                glyph_run_description: &GlyphRunDescription,
                effect: Option<Unknown>,
            ) -> Result<(), HResult> {
                let (glyph_run, _) = glyph_run.to_c_struct();
                let (glyph_run_description, _, _) = glyph_run_description.to_c_struct();
                unsafe {
                    hresult(
                        (),
                        self.0.DrawGlyphRun(
                            context.unwrap_or(std::ptr::null_mut()),
                            baseline_origin_x,
                            baseline_origin_y,
                            measuring_mode as u32,
                            &glyph_run,
                            &glyph_run_description,
                            effect
                                .as_ref()
                                .map_or(std::ptr::null_mut(), |e| e.as_ptr() as *mut _),
                        ),
                    )
                }
            }
            fn draw_inline_object(
                &self,
                context: Option<*mut std::ffi::c_void>,
                baseline_origin_x: f32,
                baseline_origin_y: f32,
                inline_object: &impl IInlineObject,
                is_sideways: bool,
                is_right_to_left: bool,
                effect: Option<Unknown>,
            ) -> Result<(), HResult> {
                unsafe {
                    hresult(
                        (),
                        self.0.DrawInlineObject(
                            context.unwrap_or(std::ptr::null_mut()),
                            baseline_origin_x,
                            baseline_origin_y,
                            inline_object.as_ptr() as *mut _,
                            to_BOOL(is_sideways),
                            to_BOOL(is_right_to_left),
                            effect
                                .as_ref()
                                .map_or(std::ptr::null_mut(), |e| e.as_ptr() as *mut _),
                        ),
                    )
                }
            }
            fn draw_strikethrough(
                &self,
                context: Option<*mut std::ffi::c_void>,
                baseline_origin_x: f32,
                baseline_origin_y: f32,
                striketthrough: &Strikethrough,
                effect: Option<Unknown>,
            ) -> Result<(), HResult> {
                let (strikethrough, _) = striketthrough.to_c_struct();
                unsafe {
                    hresult(
                        (),
                        self.0.DrawStrikethrough(
                            context.unwrap_or(std::ptr::null_mut()),
                            baseline_origin_x,
                            baseline_origin_y,
                            &strikethrough,
                            effect
                                .as_ref()
                                .map_or(std::ptr::null_mut(), |e| e.as_ptr() as *mut _),
                        ),
                    )
                }
            }
            fn draw_underline(
                &self,
                context: Option<*mut std::ffi::c_void>,
                baseline_origin_x: f32,
                baseline_origin_y: f32,
                underline: &Underline,
                effect: Option<Unknown>,
            ) -> Result<(), HResult> {
                let (underline, _) = underline.to_c_struct();
                unsafe {
                    hresult(
                        (),
                        self.0.DrawUnderline(
                            context.unwrap_or(std::ptr::null_mut()),
                            baseline_origin_x,
                            baseline_origin_y,
                            &underline,
                            effect
                                .as_ref()
                                .map_or(std::ptr::null_mut(), |e| e.as_ptr() as *mut _),
                        ),
                    )
                }
            }
        }
    };
}

macro_rules! impl_typography {
    ($s: ident, $interface: ident) => {
        impl_interface!($s, $interface);
        impl ITypography for $s {
            fn add_font_feature(&self, feature: FontFeature) -> Result<(), HResult> {
                unsafe { hresult((), self.0.AddFontFeature(feature.to_c_struct())) }
            }
            fn get_font_feature(&self, index: u32) -> Result<FontFeature, HResult> {
                unsafe {
                    let mut feature = Default::default();
                    let ret = self.0.GetFontFeature(index, &mut feature);
                    hresult(feature.into(), ret)
                }
            }
            fn get_font_feature_count(&self) -> u32 {
                unsafe { self.0.GetFontFeatureCount() }
            }
        }
    };
}

pub trait IBitmapRenderTarget: Interface {
    fn draw_glyph_run(
        &self,
        baseline_origin_x: f32,
        baseline_origin_y: f32,
        measuring_mode: MeasuringMode,
        glyph_run: &GlyphRun,
        rendering_params: &impl IRenderingParams,
        text_color: u32,
    ) -> Result<Rect<i32>, HResult>;
    fn get_current_transform(&self) -> Result<Matrix, HResult>;
    fn get_memory_dc(&self) -> *mut std::ffi::c_void;
    fn get_pixels_per_dip(&self) -> f32;
    fn get_size(&self) -> Result<Size<i32>, HResult>;
    fn resize(&self, width: u32, height: u32) -> Result<(), HResult>;
    fn set_current_transform(&self, m: Option<&Matrix>) -> Result<(), HResult>;
    fn set_pixels_per_dip(&self, pixels_per_dip: f32) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct BitmapRenderTarget(ComPtr<IDWriteBitmapRenderTarget>);
impl_bitmap_render_target!(BitmapRenderTarget, IDWriteBitmapRenderTarget);

pub trait IFactory: Interface {
    fn create_custom_font_collection(
        &self,
        loader: &impl IFontCollectionLoader,
        key: &[u8],
    ) -> Result<FontCollection, HResult>;
    fn create_custom_font_file_reference(
        &self,
        key: &[u8],
        loader: &impl IFontFileLoader,
    ) -> Result<FontFile, HResult>;
    fn create_custom_rendering_params(
        &self,
        gamma: f32,
        enchanced_contrast: f32,
        clear_type_level: f32,
        pixel_geometry: PixelGeometry,
        rendering_mode: RenderingMode,
    ) -> Result<RenderingParams, HResult>;
    fn create_ellipsis_trimming_sign(
        &self,
        format: &impl ITextFormat,
    ) -> Result<InlineObject, HResult>;
    fn create_font_face(
        &self,
        font_face_type: FontFaceType,
        font_files: &[&impl IFontFile],
        face_index: u32,
        flags: Option<FontSimulations>,
    ) -> Result<FontFace, HResult>;
    fn create_font_file_reference(
        &self,
        path: impl AsRef<Path>,
        file_time: Option<&FileTime>,
    ) -> Result<FontFile, HResult>;
    fn create_gdi_compatible_text_layout(
        &self,
        string: impl AsRef<str>,
        format: &impl ITextFormat,
        width: f32,
        height: f32,
        pixels_per_dip: f32,
        transform: Option<&Matrix>,
        use_gdi_natural: bool,
    ) -> Result<TextLayout, HResult>;
    fn create_glyph_run_analysis(
        &self,
        glyph_run: &GlyphRun,
        pixels_per_dip: f32,
        transform: Option<&Matrix>,
        rendering_mode: RenderingMode,
        measuring_mode: MeasuringMode,
        baseline_origin_x: f32,
        baseline_origin_y: f32,
    ) -> Result<GlyphRunAnalysis, HResult>;
    fn create_monitor_rendering_params(
        &self,
        monitor: *mut std::ffi::c_void,
    ) -> Result<RenderingParams, HResult>;
    fn create_number_substitution(
        &self,
        substitution_method: NumberSubstitutionMethod,
        locale_name: impl AsRef<str>,
        ignore_user_override: bool,
    ) -> Result<NumberSubstitution, HResult>;
    fn create_rendering_params(&self) -> Result<RenderingParams, HResult>;
    // fn create_text_analyzer(&self) -> Result<TextAnalyzer, HResult>;
    fn create_text_format(
        &self,
        family_name: impl AsRef<str>,
        collection: &impl IFontCollection,
        weight: FontWeight,
        style: FontStyle,
        stretch: FontStretch,
        size: f32,
        locale_name: impl AsRef<str>,
    ) -> Result<TextFormat, HResult>;
    fn create_text_layout(
        &self,
        string: impl AsRef<str>,
        text_format: &impl ITextFormat,
        max_width: f32,
        max_height: f32,
    ) -> Result<TextLayout, HResult>;
    fn create_typography(&self) -> Result<Typography, HResult>;
    fn get_gdi_interop(&self) -> Result<GdiInterop, HResult>;
    fn get_system_font_collection(
        &self,
        check_for_updates: bool,
    ) -> Result<FontCollection, HResult>;
    fn register_font_collection_loader(
        &self,
        loader: &impl IFontCollectionLoader,
    ) -> Result<(), HResult>;
    fn register_font_file_loader(&self, loader: &impl IFontFileLoader) -> Result<(), HResult>;
    fn unregister_font_collection_loader(
        &self,
        loader: &impl IFontCollectionLoader,
    ) -> Result<(), HResult>;
    fn unregister_font_file_loader(&self, loader: &impl IFontFileLoader) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct Factory(ComPtr<IDWriteFactory>);
impl_factory!(Factory, IDWriteFactory);

pub fn create_factory<T: IFactory>(factory_type: FactoryType) -> Result<T, HResult> {
    Ok(T::new(ComPtr::new(|| unsafe {
        let mut p = std::ptr::null_mut();
        let ret = DWriteCreateFactory(factory_type as u32, &T::uuidof().into(), &mut p);
        hresult(p as *mut _, ret)
    })?))
}

pub trait IFont: Interface {
    fn create_font_face(&self) -> Result<FontFace, HResult>;
    fn get_face_names(&self) -> Result<LocalizedStrings, HResult>;
    fn get_font_family(&self) -> Result<FontFamily, HResult>;
    fn get_informational_strings(
        &self,
        id: InformationStringID,
    ) -> Result<(LocalizedStrings, bool), HResult>;
    fn get_metrics(&self) -> FontMetrics;
    fn get_simulations(&self) -> FontSimulations;
    fn get_stretch(&self) -> FontStretch;
    fn get_style(&self) -> FontStyle;
    fn get_weight(&self) -> FontWeight;
    fn has_character(&self, unicode_value: u32) -> Result<bool, HResult>;
    fn is_symbol_font(&self) -> bool;
}

#[derive(Clone, Debug)]
pub struct Font(ComPtr<IDWriteFont>);
impl_font!(Font, IDWriteFont);

pub trait IFontCollection: Interface {
    fn find_family_name(&self, family_name: impl AsRef<str>) -> Result<(u32, bool), HResult>;
    fn get_font_family(&self, index: u32) -> Result<FontFamily, HResult>;
    fn get_font_family_count(&self) -> u32;
    fn get_font_from_font_face(&self, font_face: &impl IFontFace) -> Result<Font, HResult>;
}

#[derive(Clone, Debug)]
pub struct FontCollection(ComPtr<IDWriteFontCollection>);
impl_font_collection!(FontCollection, IDWriteFontCollection);

pub trait IFontCollectionLoader: Interface {
    fn create_enumerator_from_key(
        &self,
        factory: &impl IFactory,
        key: &[u8],
    ) -> Result<FontFileEnumerator, HResult>;
}

#[derive(Clone, Debug)]
pub struct FontCollectionLoader(ComPtr<IDWriteFontCollectionLoader>);
impl_font_collection_loader!(FontCollectionLoader, IDWriteFontCollectionLoader);

pub trait IFontFace: Interface {
    fn get_design_glyph_metrics(
        &self,
        indices: &[u16],
        is_sideways: bool,
    ) -> Result<Vec<GlyphMetrics>, HResult>;
    fn get_files(&self) -> Result<Vec<FontFile>, HResult>;
    fn get_gdi_compatible_glyph_metrics(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        transform: Option<&Matrix>,
        use_gdi_natural: bool,
        indices: &[u16],
        is_sideways: bool,
    ) -> Result<Vec<GlyphMetrics>, HResult>;
    fn get_gdi_compatbile_metrics(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        transform: Option<&Matrix>,
    ) -> Result<FontMetrics, HResult>;
    fn get_glyph_count(&self) -> u16;
    fn get_glyph_indices(&self, code_points: &[u32]) -> Result<Vec<u16>, HResult>;
    #[cfg(feature = "d2d1")]
    fn get_glyph_run_outline(
        &self,
        em_size: f32,
        indices: &[u16],
        advances: Option<&[f32]>,
        offsets: Option<&[GlyphOffset]>,
        is_sideways: bool,
        is_right_to_left: bool,
        sink: &impl IGeometrySink,
    ) -> Result<(), HResult>;
    fn get_index(&self) -> u32;
    fn get_metrics(&self) -> FontMetrics;
    fn get_recommended_rendering_mode(
        &self,
        em_size: f32,
        pixels_per_dip: f32,
        measuring_mode: MeasuringMode,
        params: &impl IRenderingParams,
    ) -> Result<RenderingMode, HResult>;
    fn get_simulations(&self) -> FontSimulations;
    fn get_type(&self) -> FontFaceType;
    fn is_symbol_font(&self) -> bool;
    fn try_get_font_table(&self, tag: u32) -> Result<Option<Vec<u8>>, HResult>;
}

#[derive(Clone, Debug)]
pub struct FontFace(ComPtr<IDWriteFontFace>);
impl_font_face!(FontFace, IDWriteFontFace);

pub trait IFontFamily: IFontList {
    fn get_family_names(&self) -> Result<LocalizedStrings, HResult>;
    fn get_first_matching_font(
        &self,
        weight: FontWeight,
        stretch: FontStretch,
        style: FontStyle,
    ) -> Result<Font, HResult>;
    fn get_matching_fonts(
        &self,
        weight: FontWeight,
        stretch: FontStretch,
        style: FontStyle,
    ) -> Result<FontList, HResult>;
}

#[derive(Clone, Debug)]
pub struct FontFamily(ComPtr<IDWriteFontFamily>);
impl_font_family!(FontFamily, IDWriteFontFamily);

#[derive(Clone, Debug)]
pub struct AnalyzeResult {
    pub is_supported_font_type: bool,
    pub font_file_type: FontFileType,
    pub font_face_type: FontFaceType,
    pub number_of_faces: u32,
}

pub trait IFontFile: Interface {
    fn analyze(&self) -> Result<AnalyzeResult, HResult>;
    fn get_loader(&self) -> Result<FontFileLoader, HResult>;
    fn get_reference_key(&self) -> Result<Vec<u8>, HResult>;
}

#[derive(Clone, Debug)]
pub struct FontFile(ComPtr<IDWriteFontFile>);
impl_font_file!(FontFile, IDWriteFontFile);

pub trait IFontFileEnumerator: Interface {
    fn get_current_font_file(&self) -> Result<FontFile, HResult>;
    fn move_next(&self) -> Result<bool, HResult>;
}

#[derive(Clone, Debug)]
pub struct FontFileEnumerator(ComPtr<IDWriteFontFileEnumerator>);
impl_font_file_enumerator!(FontFileEnumerator, IDWriteFontFileEnumerator);

pub trait IFontFileLoader: Interface {
    fn create_stream_from_key(&self, key: &[u8]) -> Result<FontFileStream, HResult>;
}

#[derive(Clone, Debug)]
pub struct FontFileLoader(ComPtr<IDWriteFontFileLoader>);
impl_font_file_loader!(FontFileLoader, IDWriteFontFileLoader);

pub trait IFontFileStream: Interface {
    fn get_file_size(&self) -> Result<u64, HResult>;
    fn get_last_write_time(&self) -> Result<u64, HResult>;
    fn read_file_fragment(&self, offset: u64, size: u64) -> Result<Vec<u8>, HResult>;
}

#[derive(Clone, Debug)]
pub struct FontFileStream(ComPtr<IDWriteFontFileStream>);
impl_font_file_stream!(FontFileStream, IDWriteFontFileStream);

pub trait IFontList: Interface {
    fn get_font(&self, index: u32) -> Result<Font, HResult>;
    fn get_font_collection(&self) -> Result<FontCollection, HResult>;
    fn get_font_count(&self) -> u32;
}

#[derive(Clone, Debug)]
pub struct FontList(ComPtr<IDWriteFontList>);
impl_font_list!(FontList, IDWriteFontList);

#[cfg(feature = "d2d1")]
pub use crate::d2d1::ISimplifiedGeometrySink as IGeometrySink;

pub trait IGdiInterop: Interface {}

#[derive(Clone, Debug)]
pub struct GdiInterop(ComPtr<IDWriteGdiInterop>);
impl_gdi_interop!(GdiInterop, IDWriteGdiInterop);

#[derive(Clone, Debug)]
pub struct AlphaBlendParams {
    pub gamma: f32,
    pub enhanced_contrast: f32,
    pub clear_type_level: f32,
}

pub trait IGlyphRunAnalysis: Interface {
    fn create_alpha_texture(
        &self,
        texture_type: TextureType,
        bounds: impl Into<Rect<i32>>,
    ) -> Result<Vec<u8>, HResult>;
    fn get_alpha_blend_params(
        &self,
        params: &impl IRenderingParams,
    ) -> Result<AlphaBlendParams, HResult>;
    fn get_alpha_texture_bounds(&self, texture_type: TextureType) -> Result<Rect<i32>, HResult>;
}

#[derive(Clone, Debug)]
pub struct GlyphRunAnalysis(ComPtr<IDWriteGlyphRunAnalysis>);
impl_glyph_run_analysis!(GlyphRunAnalysis, IDWriteGlyphRunAnalysis);

#[derive(Clone, Debug)]
pub struct GetBreakConditionsResult {
    pub before: BreakCondition,
    pub after: BreakCondition,
}

pub trait IInlineObject: Interface {
    fn draw(
        &self,
        context: Option<*mut std::ffi::c_void>,
        renderer: &impl ITextRenderer,
        origin_x: f32,
        origin_y: f32,
        is_sideways: bool,
        is_right_to_left: bool,
        effect: Option<Unknown>,
    ) -> Result<(), HResult>;
    fn get_break_conditions(&self) -> Result<GetBreakConditionsResult, HResult>;
    fn get_metrics(&self) -> Result<InlineObjectMetrics, HResult>;
    fn get_overhang_metrics(&self) -> Result<OverhangMetrics, HResult>;
}

#[derive(Clone, Debug)]
pub struct InlineObject(ComPtr<IDWriteInlineObject>);
impl_inline_object!(InlineObject, IDWriteInlineObject);

pub trait ILocalFontFileLoader: Interface {
    fn get_file_path_from_key(&self, key: &[u8]) -> Result<PathBuf, HResult>;
    fn get_file_path_length_from_key(&self, key: &[u8]) -> Result<u32, HResult>;
    fn get_last_write_time_from_key(&self, key: &[u8]) -> Result<FileTime, HResult>;
}

#[derive(Clone, Debug)]
pub struct LocalFontFileLoader(ComPtr<IDWriteLocalFontFileLoader>);
impl_local_font_file_loader!(LocalFontFileLoader, IDWriteLocalFontFileLoader);

pub trait ILocalizedStrings: Interface {
    fn find_locale_name(&self, locale_name: impl AsRef<str>) -> Result<Option<u32>, HResult>;
    fn get_count(&self) -> u32;
    fn get_locale_name(&self, index: u32) -> Result<String, HResult>;
    fn get_locale_name_length(&self, index: u32) -> Result<u32, HResult>;
    fn get_string(&self, index: u32) -> Result<String, HResult>;
    fn get_string_length(&self, index: u32) -> Result<u32, HResult>;
}

#[derive(Clone, Debug)]
pub struct LocalizedStrings(ComPtr<IDWriteLocalizedStrings>);
impl_localized_strings!(LocalizedStrings, IDWriteLocalizedStrings);

pub trait INumberSubstitution: Interface {}

#[derive(Clone, Debug)]
pub struct NumberSubstitution(ComPtr<IDWriteNumberSubstitution>);
impl_number_substitution!(NumberSubstitution, IDWriteNumberSubstitution);

pub trait IPixelSnapping: Interface {
    fn get_current_transform(
        &self,
        context: Option<*mut std::ffi::c_void>,
    ) -> Result<Matrix, HResult>;
    fn get_pixels_per_dip(&self, context: Option<*mut std::ffi::c_void>) -> Result<f32, HResult>;
}

#[derive(Clone, Debug)]
pub struct PixelSnapping(ComPtr<IDWritePixelSnapping>);
impl_pixel_snapping!(PixelSnapping, IDWritePixelSnapping);

pub trait IRenderingParams: Interface {
    fn get_clear_type_level(&self) -> f32;
    fn get_enhanced_contrast(&self) -> f32;
    fn get_gamma(&self) -> f32;
    fn get_pixel_geometry(&self) -> PixelGeometry;
    fn get_rendering_mode(&self) -> RenderingMode;
}

#[derive(Clone, Debug)]
pub struct RenderingParams(pub(crate) ComPtr<IDWriteRenderingParams>);
impl_rendering_params!(RenderingParams, IDWriteRenderingParams);

/*
pub trait ITextAnalysisSink: Interface {
    fn set_bidi_level(
        &self,
        text_position: u32,
        text_length: u32,
        explicit_level: u8,
        resolved_level: u8,
    ) -> Result<(), HResult>;
    fn set_line_breakpoints(
        &self,
        text_position: u32,
        text_length: u32,
        line_breakpoints: &LineBreakpoint,
    ) -> Result<(), HResult>;
    fn set_number_substitution(
        &self,
        text_position: u32,
        text_length: u32,
        number_substitution: &impl INumberSubstitution,
    ) -> Result<(), HResult>;
    fn set_script_analysis(
        &self,
        text_position: u32,
        text_length: u32,
        script_analysis: &ScriptAnalysis,
    ) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct TextAnalysisSink(ComPtr<IDWriteTextAnalysisSink>);
impl_text_analysis_sink!(TextAnalysisSink, IDWriteTextAnalysisSink);

pub trait ITextAnalysisSource: Interface {
}

pub trait ITextAnalyzer: Interface {
    fn analyze_bidi(
        &self,
        source: &impl ITextAnalysisSource,
        text_position: u32,
        length: u32,
    ) -> Result<TextAnalysisSink, HResult>;
    fn analyze_line_breakpoints(
        &self,
        source: &impl ITextAnalysisSource,
        text_position: u32,
        text_length: u32,
    ) -> Result<TextAnalysisSink, HResult>;
    fn analyze_number_substitution(
        &self,
        source: &impl ITextAnalysisSource,
        text_position: u32,
        text_length: u32,
    ) -> Result<TextAnalysisSink, HResult>;
    fn analyze_script(
        &self,
        source: &impl ITextAnalysisSource,
        text_position: u32,
        text_length: u32,
    ) -> Result<TextAnalysisSink, HResult>;
    // fn get_gdi_compatible_glyph_placements;
    // fn get_glyph_placements;
    // fn get_glyphs;
}

#[derive(Clone, Debug)]
pub struct TextAnalyzer(ComPtr<IDWriteTextAnalyzer>);
*/

#[derive(Clone, Debug)]
pub struct GetLineSpacingResult {
    pub method: LineSpacingMethod,
    pub line_spacing: f32,
    pub baseline: f32,
}

#[derive(Clone, Debug)]
pub struct GetTrimmingResult {
    pub trimming: Trimming,
    pub inline_object: InlineObject,
}

pub trait ITextFormat: Interface {
    fn get_flow_direction(&self) -> FlowDirection;
    fn get_font_collection(&self) -> Result<FontCollection, HResult>;
    fn get_font_family_name(&self) -> Result<String, HResult>;
    fn get_font_family_name_length(&self) -> u32;
    fn get_font_size(&self) -> f32;
    fn get_font_stretch(&self) -> FontStretch;
    fn get_font_style(&self) -> FontStyle;
    fn get_font_weight(&self) -> FontWeight;
    fn get_incremental_tab_stop(&self) -> f32;
    fn get_line_spacing(&self) -> Result<GetLineSpacingResult, HResult>;
    fn get_locale_name(&self) -> Result<String, HResult>;
    fn get_locale_name_length(&self) -> u32;
    fn get_paragraph_alignment(&self) -> ParagraphAlignment;
    fn get_reading_direction(&self) -> ReadingDirection;
    fn get_text_alignment(&self) -> TextAlignment;
    fn get_trimming(&self) -> Result<GetTrimmingResult, HResult>;
    fn get_word_wrapping(&self) -> WordWrapping;
    fn set_flow_direction(&self, flow_direction: FlowDirection) -> Result<(), HResult>;
    fn set_incremental_tab_stop(&self, tab_stop: f32) -> Result<(), HResult>;
    fn set_line_spacing(
        &self,
        method: LineSpacingMethod,
        line_spacing: f32,
        baseline: f32,
    ) -> Result<(), HResult>;
    fn set_paragraph_alignment(&self, alignment: ParagraphAlignment) -> Result<(), HResult>;
    fn set_text_alignment(&self, alignment: TextAlignment) -> Result<(), HResult>;
    fn set_trimming(&self, options: &Trimming, sign: &impl IInlineObject) -> Result<(), HResult>;
    fn set_word_wrapping(&self, word_wrapping: WordWrapping) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct TextFormat(ComPtr<IDWriteTextFormat>);
impl_text_format!(TextFormat, IDWriteTextFormat);

#[derive(Clone, Debug)]
pub struct HitTestPointResult {
    pub is_trailing_hit: bool,
    pub is_inside: bool,
    pub metrics: HitTestMetrics,
}

#[derive(Clone, Debug)]
pub struct HitTestTextPositionResult {
    pub x: f32,
    pub y: f32,
    pub metrics: HitTestMetrics,
}

pub trait ITextLayout: ITextFormat {
    fn determine_min_width(&self) -> Result<f32, HResult>;
    fn draw(
        &self,
        context: Option<*mut std::ffi::c_void>,
        renderer: &impl ITextRenderer,
        origin_x: f32,
        origin_y: f32,
    ) -> Result<(), HResult>;
    fn get_cluster_metrics(&self) -> Result<Vec<ClusterMetrics>, HResult>;
    fn get_drawing_effect(&self, current_position: u32) -> Result<(Unknown, TextRange), HResult>;
    fn get_font_collection(
        &self,
        current_position: u32,
    ) -> Result<(FontCollection, TextRange), HResult>;
    fn get_font_family_name(&self, current_position: u32) -> Result<(String, TextRange), HResult>;
    fn get_font_family_name_length(
        &self,
        current_position: u32,
    ) -> Result<(u32, TextRange), HResult>;
    fn get_font_size(&self, current_position: u32) -> Result<(f32, TextRange), HResult>;
    fn get_font_stretch(&self, current_position: u32) -> Result<(FontStretch, TextRange), HResult>;
    fn get_font_style(&self, current_position: u32) -> Result<(FontStyle, TextRange), HResult>;
    fn get_font_weight(&self, current_position: u32) -> Result<(FontWeight, TextRange), HResult>;
    fn get_inline_object(
        &self,
        current_position: u32,
    ) -> Result<(InlineObject, TextRange), HResult>;
    fn get_line_metrics(&self) -> Result<Vec<LineMetrics>, HResult>;
    fn get_locale_name(&self, current_position: u32) -> Result<(String, TextRange), HResult>;
    fn get_locale_name_length(&self, current_position: u32) -> Result<(u32, TextRange), HResult>;
    fn get_max_height(&self) -> f32;
    fn get_max_width(&self) -> f32;
    fn get_metrics(&self) -> Result<TextMetrics, HResult>;
    fn get_overhang_metrics(&self) -> Result<OverhangMetrics, HResult>;
    fn get_strikethrough(&self, current_position: u32) -> Result<(bool, TextRange), HResult>;
    fn get_typography(&self, current_position: u32) -> Result<(Typography, TextRange), HResult>;
    fn get_underline(&self, current_position: u32) -> Result<(bool, TextRange), HResult>;
    fn hit_test_point(&self, x: f32, y: f32) -> Result<HitTestPointResult, HResult>;
    fn hit_test_text_position(
        &self,
        text_position: u32,
        is_trailing_hit: bool,
    ) -> Result<HitTestTextPositionResult, HResult>;
    fn hit_test_text_range(
        &self,
        text_position: u32,
        text_length: u32,
        origin_x: f32,
        origin_y: f32,
    ) -> Result<Vec<HitTestMetrics>, HResult>;
    fn set_drawing_effect(
        &self,
        effect: &impl Interface,
        text_range: TextRange,
    ) -> Result<(), HResult>;
    fn set_font_collection(
        &self,
        collection: &impl IFontCollection,
        text_range: TextRange,
    ) -> Result<(), HResult>;
    fn set_font_family_name(
        &self,
        family_name: impl AsRef<str>,
        text_range: TextRange,
    ) -> Result<(), HResult>;
    fn set_font_size(&self, size: f32, text_range: TextRange) -> Result<(), HResult>;
    fn set_font_stretch(&self, stretch: FontStretch, text_range: TextRange) -> Result<(), HResult>;
    fn set_font_style(&self, style: FontStyle, text_range: TextRange) -> Result<(), HResult>;
    fn set_font_weight(&self, weight: FontWeight, text_range: TextRange) -> Result<(), HResult>;
    fn set_inline_object(
        &self,
        inline_object: &impl IInlineObject,
        text_range: TextRange,
    ) -> Result<(), HResult>;
    fn set_locale_name(
        &self,
        locale_name: impl AsRef<str>,
        text_range: TextRange,
    ) -> Result<(), HResult>;
    fn set_max_height(&self, height: f32) -> Result<(), HResult>;
    fn set_max_width(&self, width: f32) -> Result<(), HResult>;
    fn set_strikethrough(
        &self,
        has_strikethrough: bool,
        text_range: TextRange,
    ) -> Result<(), HResult>;
    fn set_typography(
        &self,
        typography: &impl ITypography,
        text_range: TextRange,
    ) -> Result<(), HResult>;
    fn set_underline(&self, has_underline: bool, text_range: TextRange) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct TextLayout(ComPtr<IDWriteTextLayout>);
impl_text_layout!(TextLayout, IDWriteTextLayout);

pub trait ITextRenderer: IPixelSnapping {
    fn draw_glyph_run(
        &self,
        context: Option<*mut std::ffi::c_void>,
        baseline_origin_x: f32,
        baseline_origin_y: f32,
        measuring_mode: MeasuringMode,
        glyph_run: &GlyphRun,
        glyph_run_description: &GlyphRunDescription,
        effect: Option<Unknown>,
    ) -> Result<(), HResult>;
    fn draw_inline_object(
        &self,
        context: Option<*mut std::ffi::c_void>,
        baseline_origin_x: f32,
        baseline_origin_y: f32,
        inline_object: &impl IInlineObject,
        is_sideways: bool,
        is_right_to_left: bool,
        effect: Option<Unknown>,
    ) -> Result<(), HResult>;
    fn draw_strikethrough(
        &self,
        context: Option<*mut std::ffi::c_void>,
        baseline_origin_x: f32,
        baseline_origin_y: f32,
        striketthrough: &Strikethrough,
        effect: Option<Unknown>,
    ) -> Result<(), HResult>;
    fn draw_underline(
        &self,
        context: Option<*mut std::ffi::c_void>,
        baseline_origin_x: f32,
        baseline_origin_y: f32,
        underline: &Underline,
        effect: Option<Unknown>,
    ) -> Result<(), HResult>;
}

#[derive(Clone, Debug)]
pub struct TextRenderer(ComPtr<IDWriteTextRenderer>);
impl_text_renderer!(TextRenderer, IDWriteTextRenderer);

pub trait ITypography: Interface {
    fn add_font_feature(&self, feature: FontFeature) -> Result<(), HResult>;
    fn get_font_feature(&self, index: u32) -> Result<FontFeature, HResult>;
    fn get_font_feature_count(&self) -> u32;
}

#[derive(Clone, Debug)]
pub struct Typography(ComPtr<IDWriteTypography>);
impl_typography!(Typography, IDWriteTypography);
