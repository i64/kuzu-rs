use std::ffi::c_char;

use crate::error;

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PtrContainer<T: ?Sized>(pub *mut T);

impl<T: ?Sized> PtrContainer<T> {
    pub fn validate(self) -> error::Result<Self> {
        match self.0.is_null() {
            true => Err(error::Error::FFIGotNull(std::any::type_name::<Self>())),
            false => Ok(self),
        }
    }
}

pub fn convert_inner_to_owned_string(inner: *const c_char) -> error::Result<String> {
    if inner.is_null() {
        return Err(error::Error::FFIGotNull(std::any::type_name::<c_char>()));
    }

    let cstr = unsafe { ::std::ffi::CStr::from_ptr(inner) };

    Ok(cstr
        .to_str()
        .map_err(|_| crate::error::Error::CStringDecodeError(inner))?
        .to_owned())
}

#[macro_export]
macro_rules! into_cstr {
    ($inner:expr) => {{
        let _inner = $inner;
        let cstr = ::std::ffi::CString::new(_inner)
            .map_err(|_| crate::error::Error::CStringEncodeError(_inner.to_owned()))?;

        Ok(cstr)
    }};
}
