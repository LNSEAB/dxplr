use crate::d3d::{Blob, IBlob};
use com_ptr::ComPtr;
use failure::{Backtrace, Context, Fail};
use winapi::ctypes::c_void;
use winapi::um::d3dcommon::ID3DBlob;
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

/// wrapped around `HRESULT`
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
    fn cause(&self) -> Option<&dyn Fail> {
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
                std::mem::transmute(self.code()),
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

pub fn hresult<T>(obj: T, res: HRESULT) -> Result<T, HResult> {
    com_ptr::hresult(obj, res).map_err(|res| res.into())
}

#[derive(Debug, Fail)]
pub struct ErrorMessageObject {
    hresult: HResult,
    message: Option<String>,
}
impl ErrorMessageObject {
    pub(crate) fn new(hresult: HResult, message: *mut ID3DBlob) -> Self {
        let msg = if message != std::ptr::null_mut() {
            unsafe { Some(Blob(ComPtr::from_raw(message))) }
        } else {
            None
        };
        Self {
            hresult,
            message: if let Some(blob) = msg {
                if let Ok(cstr) = blob.as_cstr() {
                    if let Ok(s) = cstr.to_str() {
                        Some(s.into())
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            },
        }
    }
}
impl std::fmt::Display for ErrorMessageObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(s) = &self.message {
            write!(f, "{}", s)
        } else {
            write!(f, "{}", self.hresult)
        }
    }
}

#[derive(Debug)]
pub struct ErrorMessage {
    inner: Context<ErrorMessageObject>,
}
impl ErrorMessage {
    pub fn new(inner: Context<ErrorMessageObject>) -> Self {
        Self { inner }
    }
    pub fn hresult(&self) -> &HResult {
        &self.inner.get_context().hresult
    }
    pub fn message(&self) -> Option<&str> {
        if let Some(s) = &self.inner.get_context().message {
            Some(s.as_str())
        } else {
            None
        }
    }
}
impl Fail for ErrorMessage {
    fn cause(&self) -> Option<&dyn Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}
impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.inner.get_context())
    }
}
impl From<ErrorMessageObject> for ErrorMessage {
    fn from(src: ErrorMessageObject) -> ErrorMessage {
        ErrorMessage::new(Context::new(src))
    }
}
impl From<Context<ErrorMessageObject>> for ErrorMessage {
    fn from(src: Context<ErrorMessageObject>) -> ErrorMessage {
        ErrorMessage::new(src)
    }
}
