use crate::ffi;
use crate::into_cstr;
#[derive(Clone, Copy)]
pub enum LogLevel {
    Info,
    Debug,
    Error,
}

impl LogLevel {
    fn as_str(&self) -> &'static str {
        match self {
            LogLevel::Info => "info",
            LogLevel::Debug => "debug",
            LogLevel::Error => "err",
        }
    }
}

pub struct DatabaseBuilder {
    database_path: String,
    buffer_pool_size: u64,
    //     system_config: SystemConfig,
    log_level: LogLevel,
}

impl DatabaseBuilder {
    pub fn new<S: AsRef<str>>(database_path: S) -> Self {
        Self {
            database_path: database_path.as_ref().to_owned(),
            buffer_pool_size: 0,
            // system_config: SystemConfig::default(),
            log_level: LogLevel::Error,
        }
    }

    pub fn with_log_level(&mut self, log_level: LogLevel) -> &mut Self {
        self.log_level = log_level;
        self
    }
    pub fn build(&self) -> Database {
        let db = Database::new(&self.database_path, self.buffer_pool_size);
        unsafe { Database::set_logging_level(self.log_level) };
        db
    }
    pub fn with_page_buffer_pool_size(&mut self, buffer_pool_size: u64) -> &mut Self {
        self.buffer_pool_size = buffer_pool_size;
        self
    }
    //     pub fn with_large_page_buffer_pool_size(
    //         &mut self,
    //         large_page_buffer_pool_size: u64,
    //     ) -> &mut Self {
    //         self.system_config.large_page_buffer_pool_size = large_page_buffer_pool_size;
    //         self
    //     }
    //     pub fn with_max_num_threads(&mut self, max_num_threads: u64) -> &mut Self {
    //         self.system_config.max_num_threads = max_num_threads;
    //         self
    //     }
}

#[repr(C)]
pub struct Database(pub *mut ffi::kuzu_database);

impl Database {
    pub fn builder<S: AsRef<str>>(database_path: S) -> DatabaseBuilder {
        DatabaseBuilder::new(database_path)
    }
    pub fn new<S: AsRef<str>>(database_path: S, buffer_pool_size: u64) -> Database {
        let cstring_path = into_cstr!(database_path.as_ref());
        let this = unsafe { ffi::kuzu_database_init(cstring_path, buffer_pool_size) };
        Self(this)
    }

    unsafe fn set_logging_level(log_level: LogLevel) {
        let cstring_log_level = into_cstr!(log_level.as_str());
        ffi::kuzu_database_set_logging_level(cstring_log_level);
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        unsafe { ffi::kuzu_database_destroy(self.0) }
    }
}
