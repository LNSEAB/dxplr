use std::time::Duration;
use winapi::shared::guiddef::GUID;
use winapi::shared::windef::{HWND, POINT, RECT};
use winapi::um::handleapi::*;
use winapi::um::synchapi::*;
use winapi::um::winbase::INFINITE;
use winapi::um::winnt::{HANDLE, LUID};

/// Represents a globally unique identifier (GUID).
#[derive(Clone, Copy)]
pub struct Guid(pub GUID);
impl Guid {
    /// Creates a GUID.
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
impl std::fmt::Display for Guid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Debug::fmt(self, f)
    }
}

/// Represents a universally unique identifier (UUID).
pub type Uuid = Guid;

/// Represents a local identifier for an adapter.
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

/// Represents a point in a two dimensions plane.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
impl Point {
    /// Creates a new `Point`.
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

/// Represents a rectangle in two dimensions plane.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl Rect {
    /// Creates a new `Rect`.
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

/// Wrapped around HANDLE.
/// This struct close a HANDLE automatically when dropped.
pub struct Handle(HANDLE);
impl Handle {
    pub fn new(p: HANDLE) -> Self {
        Self(p)
    }
    pub fn as_raw_handle(&self) -> HANDLE {
        self.0
    }
}
impl Drop for Handle {
    fn drop(&mut self) {
        unsafe {
            CloseHandle(self.0);
        }
    }
}
unsafe impl Send for Handle {}
unsafe impl Sync for Handle {}

/// Wrapped around a event object.
pub struct EventHandle(std::sync::Arc<Handle>);
impl EventHandle {
    /// Creates a event object.
    pub fn new() -> Self {
        unsafe {
            let h = CreateEventW(std::ptr::null_mut(), 0, 0, std::ptr::null());
            assert!(h != std::ptr::null_mut());
            Self(std::sync::Arc::new(Handle::new(h)))
        }
    }

    /// Waits until the signaled state or the timeout (a unit of millseconds).
    pub fn wait(&self, timeout: Option<Duration>) {
        unsafe {
            WaitForSingleObject(
                self.0.as_raw_handle(),
                timeout.map_or(INFINITE, |d| d.as_millis() as u32),
            );
        }
    }

    /// Returns a `HANDLE`.
    pub fn as_raw_handle(&self) -> HANDLE {
        self.0.as_raw_handle()
    }
}
unsafe impl Send for EventHandle {}
unsafe impl Sync for EventHandle {}

/// A trait for converting a value to a 'HWND' and a `c_void` pointer.
pub trait WindowHandle {
    fn as_hwnd(&self) -> HWND;
    fn as_ptr(&self) -> *const std::ffi::c_void;
}
impl WindowHandle for HWND {
    fn as_hwnd(&self) -> HWND {
        self.clone()
    }
    fn as_ptr(&self) -> *const std::ffi::c_void {
        self.clone() as *const _
    }
}
impl WindowHandle for *const std::ffi::c_void {
    fn as_hwnd(&self) -> HWND {
        self.clone() as HWND
    }
    fn as_ptr(&self) -> *const std::ffi::c_void {
        self.clone()
    }
}
impl WindowHandle for *mut std::ffi::c_void {
    fn as_hwnd(&self) -> HWND {
        self.clone() as HWND
    }
    fn as_ptr(&self) -> *const std::ffi::c_void {
        self.clone()
    }
}
