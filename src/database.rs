use crate::ffi;
use crate::into_cstr;

use crate::error;

/// Represents different log levels.
#[derive(Clone)]
pub enum LogLevel {
    /// Informational log level.
    Info,
    /// Debug log level.
    Debug,
    /// Error log level.
    Error,
}

impl LogLevel {
    /// Converts the log level to its corresponding string representation.
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Error => "err",
        }
    }
}

/// Builder for creating a database instance.
pub struct DatabaseBuilder {
    /// The path to the database directory.
    database_path: String,
    /// The size of the buffer pool in bytes.
    buffer_pool_size: u64,
    /// The log level for the database. Default value is `LogLevel::Error`.
    log_level: LogLevel,
}

impl DatabaseBuilder {
    /// Creates a new `DatabaseBuilder` instance with the specified database path.
    pub fn new<S: AsRef<str>>(database_path: S) -> Self {
        Self {
            database_path: database_path.as_ref().to_owned(),
            buffer_pool_size: 0,
            log_level: LogLevel::Error,
        }
    }
    /// Sets the log level for the database.
    pub fn with_log_level(&mut self, log_level: LogLevel) -> &mut Self {
        self.log_level = log_level;
        self
    }
    /// Builds the database instance.
    pub fn build(&self) -> error::Result<Database> {
        let db = Database::new(&self.database_path, self.buffer_pool_size);
        Database::set_logging_level(&self.log_level)?;
        db
    }
    /// Sets the page buffer pool size for the database.
    pub fn with_page_buffer_pool_size(&mut self, buffer_pool_size: u64) -> &mut Self {
        self.buffer_pool_size = buffer_pool_size;
        self
    }
}

/// Represents a database instance.
#[repr(C)]
pub struct Database(pub *mut ffi::kuzu_database);

impl Database {
    /// Creates a new `DatabaseBuilder` instance with the specified database path.
    pub fn builder<S: AsRef<str>>(database_path: S) -> DatabaseBuilder {
        DatabaseBuilder::new(database_path)
    }
    /// Creates a new database instance.
    pub fn new<S: AsRef<str>>(database_path: S, buffer_pool_size: u64) -> error::Result<Database> {
        let cstring_path = into_cstr!(database_path.as_ref())?;
        let this = unsafe { ffi::kuzu_database_init(cstring_path.as_ptr(), buffer_pool_size) };
        Ok(Self(this))
    }
    /// Sets the logging level for the database.
    pub fn set_logging_level(log_level: &LogLevel) -> error::Result<()> {
        let cstring_log_level = into_cstr!(log_level.as_str())?;
        unsafe { ffi::kuzu_database_set_logging_level(cstring_log_level.as_ptr()) };
        Ok(())
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe { ffi::kuzu_database_destroy(self.0) }
    }
}
