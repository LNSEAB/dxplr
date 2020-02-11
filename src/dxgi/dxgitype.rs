use winapi::shared::dxgitype::*;

#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct RGBA {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl From<DXGI_RGBA> for RGBA {
    fn from(src: DXGI_RGBA) -> RGBA {
        RGBA {
            r: src.r,
            g: src.g,
            b: src.b,
            a: src.a,
        }
    }
}
impl From<RGBA> for DXGI_RGBA {
    fn from(src: RGBA) -> DXGI_RGBA {
        DXGI_RGBA {
            r: src.r,
            g: src.g,
            b: src.b,
            a: src.a,
        }
    }
}
impl From<(f32, f32, f32, f32)> for RGBA {
    fn from(src: (f32, f32, f32, f32)) -> RGBA {
        RGBA {
            r: src.0,
            g: src.1,
            b: src.2,
            a: src.3,
        }
    }
}
