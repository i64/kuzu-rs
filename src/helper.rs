use std::ffi::{c_char, CStr, CString};

use crate::error;

/// Wrapper for a raw pointer, providing validation functionality.
///
/// The `PtrContainer` struct wraps a raw pointer `*mut T` and provides a `validate` method
/// to check if the pointer is null. If the pointer is null, it returns an `Error::FFIGotNull` with an
/// appropriate error message. Otherwise, it returns the original `PtrContainer`.
#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct PtrContainer<T: ?Sized>(pub *mut T);

impl<T: ?Sized> PtrContainer<T> {
    /// Validates the pointer and creates a `PtrContainer` if it is not null.
    /// Returns an error of type `Error::FFIGotNull` if the pointer is null.
    #[inline]
    pub fn try_new(ptr: *mut T) -> error::Result<Self> {
        match ptr.is_null() {
            true => Err(error::Error::FFIGotNull(std::any::type_name::<Self>())),
            false => Ok(Self(ptr)),
        }
    }
}

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
