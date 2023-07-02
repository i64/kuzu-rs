use std::ffi::{c_char, CStr, CString};

use crate::error;

pub(crate) fn convert_inner_to_owned_string(inner: *const c_char) -> error::Result<String> {
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
            .map_err(|_| $crate::error::Error::CStringEncodeError(_inner.to_owned()))?;

        Ok(cstr)
    }};
}

pub(crate) enum CCow {
    Static(&'static CStr),
    Owned(CString),
}
