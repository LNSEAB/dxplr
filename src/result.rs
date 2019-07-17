use failure::{Backtrace, Context, Fail};
use winapi::ctypes::c_void;
use winapi::um::winbase::*;
use winapi::um::winnt::HRESULT;

#[derive(Clone, PartialEq, Eq, Debug, Fail)]
pub enum HResultKind {
    #[fail(display = "0x{:x}", 0)]
    Successed(HRESULT),
    #[fail(display = "0x{:x}", 0)]
    Failed(HRESULT),
}
impl HResultKind {
    fn is_successed(&self) -> bool {
        match self {
            HResultKind::Successed(_) => true,
            HResultKind::Failed(_) => false,
        }
    }
    fn is_failed(&self) -> bool {
        !self.is_successed()
    }
    fn code(&self) -> HRESULT {
        match self {
            HResultKind::Successed(v) => v,
            HResultKind::Failed(v) => v,
        }
        .clone()
    }
}
impl From<HRESULT> for HResultKind {
    fn from(src: HRESULT) -> HResultKind {
        if src < 0 {
            HResultKind::Failed(src)
        } else {
            HResultKind::Successed(src)
        }
    }
}

#[derive(Debug)]
pub struct HResult {
    inner: Context<HResultKind>,
}
impl HResult {
    pub fn new(inner: Context<HResultKind>) -> HResult {
        HResult { inner }
    }
    pub fn kind(&self) -> &HResultKind {
        self.inner.get_context()
    }
    pub fn is_successed(&self) -> bool {
        self.kind().is_successed()
    }
    pub fn is_failed(&self) -> bool {
        self.kind().is_failed()
    }
    pub fn code(&self) -> HRESULT {
        self.kind().code()
    }
}
impl Fail for HResult {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}
impl std::fmt::Display for HResult {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let msg = unsafe {
            let mut p: *mut u16 = std::ptr::null_mut();
            let len = FormatMessageW(
                FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_ALLOCATE_BUFFER,
                std::ptr::null(),
                0,
                0,
                std::mem::transmute(&mut p),
                0,
                std::ptr::null_mut(),
            ) as usize;
            if len == 0 {
                return Err(std::fmt::Error);
            }
            let msg = String::from_utf16(std::slice::from_raw_parts(p, len - 1));
            LocalFree(p as *mut c_void);
            if let Err(_) = msg {
                return Err(std::fmt::Error);
            }
            msg.unwrap()
        };
        write!(f, "{}: {}", self.inner, msg)
    }
}
impl From<HRESULT> for HResult {
    fn from(code: HRESULT) -> HResult {
        HResult {
            inner: Context::new(code.into()),
        }
    }
}
impl From<HResultKind> for HResult {
    fn from(kind: HResultKind) -> HResult {
        HResult {
            inner: Context::new(kind),
        }
    }
}
impl From<Context<HResultKind>> for HResult {
    fn from(inner: Context<HResultKind>) -> HResult {
        HResult { inner }
    }
}
impl PartialEq for HResult {
    fn eq(&self, other: &Self) -> bool {
        self.kind() == other.kind()
    }
}
impl PartialEq<HRESULT> for HResult {
    fn eq(&self, other: &HRESULT) -> bool {
        self.code() == *other
    }
}
impl PartialEq<HResultKind> for HResult {
    fn eq(&self, other: &HResultKind) -> bool {
        self.kind() == other
    }
}
