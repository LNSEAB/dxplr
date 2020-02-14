use winapi::um::{d2d1_2::*, d2d1effects_1::*};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum RenderingPriority {
    Normal = D2D1_RENDERING_PRIORITY_NORMAL,
    Low = D2D1_RENDERING_PRIORITY_LOW,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum YCbCrChromaSubsampling {
    Auto = D2D1_YCBCR_CHROMA_SUBSAMPLING_AUTO,
    _420 = D2D1_YCBCR_CHROMA_SUBSAMPLING_420,
    _422 = D2D1_YCBCR_CHROMA_SUBSAMPLING_422,
    _444 = D2D1_YCBCR_CHROMA_SUBSAMPLING_444,
    _440 = D2D1_YCBCR_CHROMA_SUBSAMPLING_440,
}

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u32)]
pub enum YCbCrProp {
    ChromaSubsampling = D2D1_YCBCR_PROP_CHROMA_SUBSAMPLING,
    TransformMatrix = D2D1_YCBCR_PROP_TRANSFORM_MATRIX,
    InterpolationMode = D2D1_YCBCR_PROP_INTERPOLATION_MODE,
}
