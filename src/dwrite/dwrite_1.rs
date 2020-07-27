#![allow(dead_code)]

use crate::utility::to_BOOL;
use winapi::um::dwrite_1;
use winapi::um::dwrite_1::*;

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
pub enum OrientationAngle {
    _0Degrees = DWRITE_GLYPH_ORIENTATION_ANGLE_0_DEGREES,
    _90Degrees = DWRITE_GLYPH_ORIENTATION_ANGLE_90_DEGREES,
    _180Degrees = DWRITE_GLYPH_ORIENTATION_ANGLE_180_DEGREES,
    _270Degrees = DWRITE_GLYPH_ORIENTATION_ANGLE_270_DEGREES,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum OutlineThreshold {
    Antialiased = winapi::um::dwrite_1::DWRITE_OUTLINE_THRESHOLD_ANTIALIASED,
    Aliased = winapi::um::dwrite_1::DWRITE_OUTLINE_THRESHOLD_ALIASED,
}

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
#[allow(non_upper_case_globals)]
impl PanoseFamily {
    pub const Pictorial: Self = Self::Symbol;
}

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum PanoseSpacing {
    Any = DWRITE_PANOSE_SPACING_ANY,
    NoFit = DWRITE_PANOSE_SPACING_NO_FIT,
    ProportionalSpaced = DWRITE_PANOSE_SPACING_PROPORTIONAL_SPACED,
    Monospaced = DWRITE_PANOSE_SPACING_MONOSPACED,
}

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
#[allow(non_upper_case_globals)]
impl PanoseXHeight {
    pub const ConstantStd: Self = Self::ConstantStandard;
    pub const DuckingStd: Self = Self::DuckingStandard;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum TextAntialiasMode {
    ClearType = DWRITE_TEXT_ANTIALIAS_MODE_CLEARTYPE,
    GrayScale = DWRITE_TEXT_ANTIALIAS_MODE_GRAYSCALE,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum VerticalGlyphOrientation {
    Default = DWRITE_VERTICAL_GLYPH_ORIENTATION_DEFAULT,
    Stacked = DWRITE_VERTICAL_GLYPH_ORIENTATION_STACKED,
}

#[derive(Clone, Debug)]
pub struct CaretMetrics {
    pub slope_rise: i16,
    pub slope_run: i16,
    pub offset: i16,
}
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
pub struct UnicodeRange {
    pub first: u32,
    pub last: u32,
}
impl UnicodeRange {
    fn to_c_struct(&self) -> dwrite_1::DWRITE_UNICODE_RANGE {
        dwrite_1::DWRITE_UNICODE_RANGE {
            first: self.first,
            last: self.last,
        }
    }
}
