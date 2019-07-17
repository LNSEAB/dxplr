use winapi::shared::guiddef::GUID;
use winapi::shared::windef::{POINT, RECT};
use winapi::um::winnt::LUID;

#[derive(Clone, Copy)]
pub struct Guid(pub GUID);
impl Guid {
    pub const fn new(data1: u32, data2: u16, data3: u16, data4: [u8; 8]) -> Self {
        Self(GUID {
            Data1: data1,
            Data2: data2,
            Data3: data3,
            Data4: data4,
        })
    }
}
impl From<GUID> for Guid {
    fn from(src: GUID) -> Guid {
        Guid(src)
    }
}
impl From<Guid> for GUID {
    fn from(src: Guid) -> GUID {
        src.0
    }
}
impl std::fmt::Debug for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{{:x}-{:x}-{:x}-{:x}-",
            self.0.Data1, self.0.Data2, self.0.Data3, self.0.Data4[0]
        )?;
        for i in 1..8 {
            write!(f, "{:x}", self.0.Data4[i])?;
        }
        write!(f, "}}")
    }
}
pub type Uuid = Guid;

#[derive(Clone, Debug)]
pub struct Luid(pub u64);
impl From<LUID> for Luid {
    fn from(src: LUID) -> Luid {
        Luid(src.LowPart as u64 | ((src.HighPart as u64) << 32))
    }
}
impl From<Luid> for LUID {
    fn from(src: Luid) -> LUID {
        LUID {
            LowPart: (src.0 & 0xffffffff) as u32,
            HighPart: (src.0 >> 32) as i32,
        }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Point {
    x: i32,
    y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}
impl From<POINT> for Point {
    fn from(src: POINT) -> Point {
        Point { x: src.x, y: src.y }
    }
}
impl From<Point> for POINT {
    fn from(src: Point) -> POINT {
        POINT { x: src.x, y: src.y }
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
impl Rect {
    pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}
impl From<RECT> for Rect {
    fn from(src: RECT) -> Rect {
        Rect {
            left: src.left,
            top: src.top,
            right: src.right,
            bottom: src.bottom,
        }
    }
}
impl From<Rect> for RECT {
    fn from(src: Rect) -> RECT {
        RECT {
            left: src.left,
            top: src.top,
            right: src.right,
            bottom: src.bottom,
        }
    }
}
