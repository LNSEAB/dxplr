use crate::d3d::{Blob, IBlob};
use com_ptr::ComPtr;
use winapi::um::d3dcommon::ID3DBlob;
use winapi::um::winnt::HRESULT;
use thiserror::Error;

#[derive(Clone, PartialEq, Eq, Debug, Error)]
pub enum HResult {
    #[error("0x{0:x}")]
    Successed(HRESULT),
    #[error("0x{0:x}")]
    Failed(HRESULT),
}
impl HResult {
    pub fn is_successed(&self) -> bool {
        match self {
            HResult::Successed(_) => true,
            HResult::Failed(_) => false,
        }
    }
    pub fn is_failed(&self) -> bool {
        !self.is_successed()
    }
    pub fn code(&self) -> HRESULT {
        match self {
            HResult::Successed(v) => v,
            HResult::Failed(v) => v,
        }
        .clone()
    }
}
impl From<HRESULT> for HResult {
    fn from(src: HRESULT) -> HResult {
        if src < 0 {
            HResult::Failed(src)
        } else {
            HResult::Successed(src)
        }
    }
}

pub fn hresult<T>(obj: T, res: HRESULT) -> Result<T, HResult> {
    com_ptr::hresult(obj, res).map_err(|res| res.into())
}

#[derive(Debug, Error)]
pub struct ErrorMessage {
    hresult: HResult,
    message: Option<String>,
}
impl ErrorMessage {
    #[allow(dead_code)]
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
impl std::fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(s) = &self.message {
            write!(f, "{}", s)
        } else {
            write!(f, "{}", self.hresult)
        }
    }
}
