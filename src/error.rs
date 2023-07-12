use std::ffi::c_char;

use thiserror::Error;

pub type Result<T, E = Error> = ::std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    /// Error that occurs while converting a string into a `CString`.
    #[error("Error while converting string {0} into CString")]
    CStringEncodeError(String),

    /// Error that occurs while converting a C string into a Rust string.
    #[error("Error while converting string {0:p} from CStr")]
    CStringDecodeError(*const c_char),

    /// Error that occurs when a variant cannot be converted to a specific type.
    #[error("Cannot convert variant {0} to type {1}")]
    DecodeError(&'static str, &'static str),

    /// Error that occurs when a column is not found.
    #[error("Column {0} not found. ({1:?})")]
    ColumnNotFound(String, Vec<String>),

    /// Error that occurs while getting a query result.
    #[error("Error while getting query result: {0}")]
    QueryResultError(String),

    /// Error that occurs while trying to create a connection.
    #[error("Error while trying to create a connection: {0}")]
    ConnectionError(String),

    /// FFI error indicating that a null pointer was received instead of an expected pointer type.
    #[error("FFI error: expected {0} ptr got null ptr")]
    FFIGotNull(&'static str),

    /// Error indicating that transactions are not allowed in DDL and COPY statements.
    #[error("Transactions are not allowed in DDL and COPY statements")]
    TxNotAllowed,

    /// Error indicating that multiple write transactions are not allowed at the same time.
    #[error(
        "At any point in time, there can be multiple read transactions but one write transaction"
    )]
    MultipleWriteTxNotAllowed,

    /// Error indicating that a list can only have one type inside it.
    #[error("A list can only have one type inside it")]
    ListTypeError,
    // // Nothing
    // Infallible,
}
