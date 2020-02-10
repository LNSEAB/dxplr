use winapi::ctypes::c_void;
use winapi::shared::minwindef::{BOOL, FALSE, TRUE, UINT};

#[allow(non_snake_case, dead_code)]
pub(crate) fn to_BOOL(b: bool) -> BOOL {
    match b {
        true => TRUE,
        false => FALSE,
    }
}

#[allow(dead_code)]
pub(crate) fn enum_function<F, R, E>(exit_code: E, f: F) -> Result<Vec<R>, E>
where
    F: Fn(UINT) -> Result<R, E>,
    E: PartialEq + std::fmt::Debug,
{
    let mut v = Vec::new();
    let mut i = 0 as u32;
    loop {
        let res = f(i);
        if let Err(e) = res {
            if e == exit_code {
                break;
            }
            return Err(e.into());
        }
        v.push(res.unwrap());
        i += 1;
    }
    Ok(v)
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_interface {
    ($s: ident, $api_type: ident) => {
        unsafe impl Send for $s {}
        unsafe impl Sync for $s {}

        impl Interface for $s {
            type APIType = $api_type;
            fn new(p: com_ptr::ComPtr<Self::APIType>) -> Self {
                $s(p)
            }
            fn uuidof() -> $crate::api::Guid {
                use winapi::Interface as _;
                Self::APIType::uuidof().into()
            }
            fn as_ptr(&self) -> *mut Self::APIType {
                self.0.as_ptr()
            }
            fn as_com_ptr(&self) -> &com_ptr::ComPtr<Self::APIType> {
                &self.0
            }
            fn as_unknown(&self) -> *mut winapi::um::unknwnbase::IUnknown {
                Interface::as_ptr(self) as *mut winapi::um::unknwnbase::IUnknown
            }
            fn from_com_ptr(p: com_ptr::ComPtr<Self::APIType>) -> Self {
                $s(p)
            }
            fn query_interface<T: $crate::Interface>(&self) -> Result<T, $crate::result::HResult> {
                let p = self
                    .as_com_ptr()
                    .query_interface::<<T as $crate::Interface>::APIType>();
                if let Err(e) = p {
                    Err(e.into())
                } else {
                    Ok(T::new(p.unwrap()))
                }
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_bitflag_operators {
    ($s: ident) => {
        impl $s {
            pub fn enabled(&self, flag: Self) -> bool {
                Self(self.0 & flag.0) == flag
            }
            pub fn disabled(&self, flag: Self) -> bool {
                !self.enabled(flag)
            }
        }
        impl std::ops::BitAnd for $s {
            type Output = Self;
            fn bitand(self, other: Self) -> Self {
                Self(self.0 & other.0)
            }
        }
        impl std::ops::BitAndAssign for $s {
            fn bitand_assign(&mut self, other: Self) {
                self.0 &= other.0
            }
        }
        impl std::ops::BitOr for $s {
            type Output = Self;
            fn bitor(self, other: Self) -> Self {
                Self(self.0 | other.0)
            }
        }
        impl std::ops::BitOrAssign for $s {
            fn bitor_assign(&mut self, other: Self) {
                self.0 |= other.0
            }
        }
    };
}

#[allow(dead_code)]
pub(crate) fn as_c_void_mut<T>(obj: &mut T) -> *mut c_void {
    obj as *mut T as *mut c_void
}

#[allow(dead_code)]
pub(crate) fn to_wstring(src: impl AsRef<str>) -> Vec<u16> {
    src.as_ref()
        .encode_utf16()
        .chain(Some(0))
        .collect::<Vec<_>>()
}

#[allow(dead_code)]
pub(crate) fn to_string(src: &[u16]) -> String {
    String::from_utf16_lossy(src)
}
