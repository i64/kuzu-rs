#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct PtrContainer<T: ?Sized>(pub *mut T);
#[macro_export]
macro_rules! convert_inner_to_owned_string {
    ($inner:expr) => {{
        let _inner = $inner;
        let cstr = unsafe { ::std::ffi::CStr::from_ptr(_inner) };
        cstr.to_str().unwrap().to_owned()
    }};
}

#[macro_export]
macro_rules! into_cstr {
    ($inner:expr) => {{
        let cstr = ::std::ffi::CString::new($inner).unwrap();
        Box::leak(Box::new(cstr)).as_ptr()
    }};
}
