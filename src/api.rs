use std::time::Duration;
use winapi::shared::guiddef::GUID;
use winapi::shared::minwindef::FILETIME;
use winapi::shared::windef::{HDC, HWND, POINT, RECT, SIZE};
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
pub struct Point<T> {
    pub x: T,
    pub y: T,
}
impl<T> Point<T> {
    /// Creates a new `Point`.
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T> From<(T, T)> for Point<T> {
    fn from(src: (T, T)) -> Point<T> {
        Self::new(src.0, src.1)
    }
}
impl From<POINT> for Point<i32> {
    fn from(src: POINT) -> Point<i32> {
        Point { x: src.x, y: src.y }
    }
}
impl From<Point<i32>> for POINT {
    fn from(src: Point<i32>) -> POINT {
        POINT { x: src.x, y: src.y }
    }
}
impl AsRef<POINT> for Point<i32> {
    fn as_ref(&self) -> &POINT {
        unsafe { (self as *const Point<i32> as *const POINT).as_ref().unwrap() }
    }
}

/// Represents a size in a two dimensions plane.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Size<T> {
    pub width: T,
    pub height: T,
}
impl<T> Size<T> {
    /// Creates a new `Size`.
    pub fn new(width: T, height: T) -> Self {
        Self { width, height }
    }
}
impl<T> From<(T, T)> for Size<T> {
    fn from(src: (T, T)) -> Size<T> {
        Self::new(src.0, src.1)
    }
}
impl From<SIZE> for Size<i32> {
    fn from(src: SIZE) -> Size<i32> {
        Size {
            width: src.cx,
            height: src.cy,
        }
    }
}
impl From<Size<i32>> for SIZE {
    fn from(src: Size<i32>) -> SIZE {
        SIZE {
            cx: src.width,
            cy: src.height,
        }
    }
}
impl AsRef<SIZE> for Size<i32> {
    fn as_ref(&self) -> &SIZE {
        unsafe { (self as *const Size<i32> as *const SIZE).as_ref().unwrap() }
    }
}

/// Represents a rectangle in two dimensions plane.
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Rect<T> {
    pub left: T,
    pub top: T,
    pub right: T,
    pub bottom: T,
}
impl<T> Rect<T> {
    /// Creates a new `Rect`.
    pub fn new(left: T, top: T, right: T, bottom: T) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }
}
impl<T> From<(T, T, T, T)> for Rect<T> {
    fn from(src: (T, T, T, T)) -> Rect<T> {
        Self::new(src.0, src.1, src.2, src.3)
    }
}
impl From<RECT> for Rect<i32> {
    fn from(src: RECT) -> Rect<i32> {
        Rect {
            left: src.left,
            top: src.top,
            right: src.right,
            bottom: src.bottom,
        }
    }
}
impl From<Rect<i32>> for RECT {
    fn from(src: Rect<i32>) -> RECT {
        RECT {
            left: src.left,
            top: src.top,
            right: src.right,
            bottom: src.bottom,
        }
    }
}
impl AsRef<RECT> for Rect<i32> {
    fn as_ref(&self) -> &RECT {
        unsafe { (self as *const Rect<i32> as *const RECT).as_ref().unwrap() }
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
    fn as_ptr(&self) -> *const std::ffi::c_void {
        self.as_hwnd() as _
    }
    fn as_mut(&self) -> *mut std::ffi::c_void {
        self.as_hwnd() as _
    }
}
impl WindowHandle for HWND {
    fn as_hwnd(&self) -> HWND {
        self.clone()
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
    fn as_mut(&self) -> *mut std::ffi::c_void {
        self.clone()
    }
}

/// A trait for converting a value to a 'HDC' and a `c_void` pointer.
pub trait DeviceContextHandle {
    fn as_hdc(&self) -> HDC;
    fn as_ptr(&self) -> *const std::ffi::c_void {
        self.as_hdc() as _
    }
    fn as_mut(&self) -> *mut std::ffi::c_void {
        self.as_hdc() as _
    }
}
impl DeviceContextHandle for HDC {
    fn as_hdc(&self) -> HDC {
        self.clone()
    }
    fn as_ptr(&self) -> *const std::ffi::c_void {
        self.clone() as *const _
    }
}
impl DeviceContextHandle for *const std::ffi::c_void {
    fn as_hdc(&self) -> HDC {
        self.clone() as HDC
    }
    fn as_ptr(&self) -> *const std::ffi::c_void {
        self.clone()
    }
}
impl DeviceContextHandle for *mut std::ffi::c_void {
    fn as_hdc(&self) -> HDC {
        self.clone() as HDC
    }
    fn as_ptr(&self) -> *const std::ffi::c_void {
        self.clone()
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FileTime {
    pub low_date_time: u32,
    pub high_date_time: u32,
}

impl FileTime {
    #[allow(dead_code)]
    pub(crate) fn to_c_struct(&self) -> FILETIME {
        FILETIME {
            dwLowDateTime: self.low_date_time,
            dwHighDateTime: self.high_date_time,
        }
    }
}

impl From<FILETIME> for FileTime {
    fn from(src: FILETIME) -> FileTime {
        FileTime {
            low_date_time: src.dwLowDateTime,
            high_date_time: src.dwHighDateTime,
        }
    }
}
