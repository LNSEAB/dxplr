#![allow(dead_code)]

use crate::utility::*;
use winapi::um::d2d1svg::*;

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
