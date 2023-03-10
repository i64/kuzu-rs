use std::ffi::{CStr, CString, NulError};

#[repr(u8)]
pub(crate) enum LoggerEnum {
    Database = 0,
    CsvReader = 1,
    Loader = 2,
    Processor = 3,
    BufferManager = 4,
    Catalog = 5,
    Storage = 6,
    TransactionManager = 7,
    Wal = 8,
}

pub(crate) fn drop_logger(logger_type: LoggerEnum) {
    unsafe { ffi::kuzu_common_LoggerUtils_dropLogger(logger_type as u8) }
}

pub(crate) fn into_cstr<S: AsRef<str>>(inp: S) -> Result<(&'static CString, usize), NulError> {
    let raw_str = inp.as_ref();
    let cstring = Box::new(CString::new(raw_str)?);

    let len = cstring.as_bytes().len();
    Ok((Box::leak(cstring), len))
}
mod ffi {
    extern "C" {
        #[link_name = "\u{1}_ZN4kuzu6common11LoggerUtils10dropLoggerENS0_15LoggerConstants10LoggerEnumE"]
        pub fn kuzu_common_LoggerUtils_dropLogger(logger_enum: u8);
    }
}
