use crate::connection::{self, Connection};
use crate::ffi::kuzu_prepared_statement;
use crate::helper::{convert_inner_to_owned_string, CCow};
use crate::ptrc::PtrContainer;
use crate::query_result::QueryResult;
use crate::types::value::KuzuValue;
use crate::{error, ffi, into_cstr};
use std::ffi::{CStr, CString};

/// Represents an argument for a prepared statement.
pub struct Argument(CCow, KuzuValue);

/// Represents a prepared statement in Kuzu
pub struct Statement<'conn> {
    /// The database connection associated with the statement.
    conn: &'conn connection::Connection,
    /// Pointer container for the underlying FFI prepared statement.
    stmt: PtrContainer<ffi::kuzu_prepared_statement>,
    /// CString representation of the statement.
    _stmt: CString,
    /// Arguments for the prepared statement.
    args: Vec<Argument>,
}

macro_rules! static_cstr {
    ($($l:expr),*) => {
        [
            $(unsafe {::std::ffi::CStr::from_bytes_with_nul_unchecked(
                concat!(stringify!($l), "\0").as_bytes()
            )}),*
        ]
    };
}

// Lookup table for static CStrings used for parameter names in prepared statements.
static STMT_LOOKUP: [&CStr; 256] = static_cstr![
    0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25,
    26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73,
    74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91, 92, 93, 94, 95, 96, 97,
    98, 99, 100, 101, 102, 103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
    117, 118, 119, 120, 121, 122, 123, 124, 125, 126, 127, 128, 129, 130, 131, 132, 133, 134, 135,
    136, 137, 138, 139, 140, 141, 142, 143, 144, 145, 146, 147, 148, 149, 150, 151, 152, 153, 154,
    155, 156, 157, 158, 159, 160, 161, 162, 163, 164, 165, 166, 167, 168, 169, 170, 171, 172, 173,
    174, 175, 176, 177, 178, 179, 180, 181, 182, 183, 184, 185, 186, 187, 188, 189, 190, 191, 192,
    193, 194, 195, 196, 197, 198, 199, 200, 201, 202, 203, 204, 205, 206, 207, 208, 209, 210, 211,
    212, 213, 214, 215, 216, 217, 218, 219, 220, 221, 222, 223, 224, 225, 226, 227, 228, 229, 230,
    231, 232, 233, 234, 235, 236, 237, 238, 239, 240, 241, 242, 243, 244, 245, 246, 247, 248, 249,
    250, 251, 252, 253, 254, 255
];

impl<'conn> Statement<'conn> {
    /// Creates a new instance of `Statement`.
    /// Returns an error if there is an issue preparing the statement or retrieving the result.
    fn new(conn: &'conn Connection, query: &str) -> error::Result<Self> {
        let cstring = into_cstr!(query)?;
        let stmt = unsafe { ffi::kuzu_connection_prepare(conn.to_inner(), cstring.as_ptr()) };
        if stmt.is_null() {
            return Err(error::Error::FFIGotNull(std::any::type_name::<
                *mut kuzu_prepared_statement,
            >()));
        }
        let is_success = unsafe { ffi::kuzu_prepared_statement_is_success(stmt) };

        if !is_success {
            let raw_error_msg = unsafe { ffi::kuzu_prepared_statement_get_error_message(stmt) };
            let error_msg = convert_inner_to_owned_string(raw_error_msg)?;
            return Err(error::Error::ConnectionError(error_msg));
        }

        let allow_active_transaction =
            unsafe { ffi::kuzu_prepared_statement_allow_active_transaction(stmt) };

        if !allow_active_transaction {
            return Err(error::Error::TxNotAllowed);
        }

        Ok(Self {
            conn,
            stmt: PtrContainer::try_new(stmt)?,
            _stmt: cstring,
            args: vec![],
        })
    }

    /// Binds a value to a parameter in the prepared statement.
    pub fn bind<K, V>(&mut self, k: K, v: V) -> error::Result<&mut Self>
    where
        (K, V): TryInto<Argument, Error = error::Error>,
    {
        self.args.push((k, v).try_into()?);
        Ok(self)
    }

    /// Executes the prepared statement.
    /// Returns an error if there is an issue executing the statement or retrieving the result.
    pub fn execute(&self) -> error::Result<QueryResult> {
        self.args.iter().try_for_each(|arg| {
            let Argument(_key, _val) = arg;

            let key = match _key {
                CCow::Owned(key_name) => key_name.as_ptr(),
                CCow::Static(key_name) => key_name.as_ptr(),
            };

            let val = PtrContainer::try_from(_val)?;

            unsafe {
                ffi::kuzu_prepared_statement_bind_value(self.stmt.0, key, val.0);
            };
            Ok(())
        })?;

        let raw_result = unsafe { ffi::kuzu_connection_execute(self.conn.to_inner(), self.stmt.0) };
        PtrContainer::try_new(raw_result)?.try_into()
    }
}

impl Connection {
    /// Prepares a Kuzu query statement for execution.
    /// Returns an error if there is an issue preparing the statement or retrieving the result.
    pub fn prepare<S: AsRef<str>>(&mut self, query: S) -> error::Result<Statement> {
        let query = query.as_ref();
        Statement::new(self, query)
    }
}

impl<V: Into<KuzuValue>> TryFrom<(usize, V)> for Argument {
    type Error = error::Error;
    fn try_from(value: (usize, V)) -> Result<Self, Self::Error> {
        let (idx, val) = value;
        let name = {
            if STMT_LOOKUP.len() > idx {
                let cstr = STMT_LOOKUP[idx];
                CCow::Static(cstr)
            } else {
                let str_idx = idx.to_string();
                let cstr_idx = into_cstr!(str_idx.as_str())?;
                CCow::Owned(cstr_idx)
            }
        };
        Ok(Argument(name, val.into()))
    }
}

impl<V: Into<KuzuValue>> TryFrom<(&str, V)> for Argument {
    type Error = error::Error;
    fn try_from(value: (&str, V)) -> Result<Self, Self::Error> {
        let (_name, val) = value;

        let name = into_cstr!(_name)?;
        Ok(Argument(CCow::Owned(name), val.into()))
    }
}
