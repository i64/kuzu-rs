use std::ffi::c_char;

use thiserror::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("error while converting string {0} into CString")]
    CStringEncodeError(String),

    #[error("error while converting string {0:p} from CStr")]
    CStringDecodeError(*const c_char),

    #[error("cannot convert variant {0} to type {1}")]
    DecodeError(&'static str, &'static str),

    #[error("column {0} not found")]
    ColumnNotFound(String),

    #[error("error while getting query result: {0}")]
    QueryResultError(String),

    #[error("error while trying to create a connection: {0}")]
    ConnectionError(String),

    #[error("ffi error: expected {0} ptr got null ptr")]
    FFIGotNull(&'static str),

    #[error("transactions are not allowed in DDL and COPOY statements")]
    TxNotAllowed,

    #[error(
        "at any point in time, there can be multiple read transactions but one write transaction"
    )]
    MultipleWriteTxNotAllowed,
}
