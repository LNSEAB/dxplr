#![allow(dead_code)]

use winapi::um::dwrite_2::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum GridFitMode {
    Default = DWRITE_GRID_FIT_MODE_DEFAULT,
    Disabled = DWRITE_GRID_FIT_MODE_DISABLED,
    Enabled = DWRITE_GRID_FIT_MODE_ENABLED,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum OpticalAlignment {
    None = DWRITE_OPTICAL_ALIGNMENT_NONE,
    NoSideBearings = DWRITE_OPTICAL_ALIGNMENT_NO_SIDE_BEARINGS,
}

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