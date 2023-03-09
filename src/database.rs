use std::{
    ffi::{CStr, CString},
    io::Seek,
};

use super::opaque_types;
use crate::{
    helper::{self, into_cstr},
    opaque_types::Opaque,
};

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
    system_config: SystemConfig,
    log_level: LogLevel,
}

impl DatabaseBuilder {
    pub fn new<S: AsRef<str>>(database_path: S) -> Self {
        Self {
            database_path: database_path.as_ref().to_owned(),
            system_config: SystemConfig::default(),
            log_level: LogLevel::Error,
        }
    }

    pub fn with_log_level(&mut self, log_level: LogLevel) -> &mut Self {
        self.log_level = log_level;
        self
    }
    pub unsafe fn build(&self) -> Database {
        // Database::set_logging_level(self.log_level);
        let mut db = Database::new_with_options(&self.database_path, self.system_config);
        db
    }
    pub fn with_default_page_buffer_pool_size(
        &mut self,
        default_page_buffer_pool_size: u64,
    ) -> &mut Self {
        self.system_config.default_page_buffer_pool_size = default_page_buffer_pool_size;
        self
    }
    pub fn with_large_page_buffer_pool_size(
        &mut self,
        large_page_buffer_pool_size: u64,
    ) -> &mut Self {
        self.system_config.large_page_buffer_pool_size = large_page_buffer_pool_size;
        self
    }
    pub fn with_max_num_threads(&mut self, max_num_threads: u64) -> &mut Self {
        self.system_config.max_num_threads = max_num_threads;
        self
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct SystemConfig {
    pub default_page_buffer_pool_size: u64,
    pub large_page_buffer_pool_size: u64,
    pub max_num_threads: u64,
}

impl Default for SystemConfig {
    fn default() -> Self {
        const DEFAULT_BUFFER_POOL_SIZE: u64 = 1 << 30;
        const DEFAULT_PAGES_BUFFER_RATIO: f64 = 0.75;
        const LARGE_PAGES_BUFFER_RATIO: f64 = 1.0 - DEFAULT_PAGES_BUFFER_RATIO;

        SystemConfig {
            default_page_buffer_pool_size: ((DEFAULT_BUFFER_POOL_SIZE as f64)
                * DEFAULT_PAGES_BUFFER_RATIO) as u64,
            large_page_buffer_pool_size: ((DEFAULT_BUFFER_POOL_SIZE as f64)
                * LARGE_PAGES_BUFFER_RATIO) as u64,
            max_num_threads: 1,
        }
    }
}

#[repr(C)]
pub struct Database(pub Opaque<128>);

impl Database {
    pub fn new<S: AsRef<str>>(database_path: S) -> Self {
        let (cstring_path, cstring_path_len) = into_cstr(database_path).unwrap();
        dbg!(&cstring_path.as_bytes(), cstring_path_len);

        let mut __this = ::std::mem::MaybeUninit::uninit();
        unsafe {
            ffi::kuzu_main_Database_Database(
                __this.as_mut_ptr(),
                cstring_path.as_ptr(),
                cstring_path_len,
            );
            let k = __this.assume_init();
            println!("inner {:p}", &k);
            println!("inner.0.0 {:p}", k.0 .0.as_ptr());

            k
        }
    }

    fn new_with_options<S: AsRef<str>>(database_path: S, system_config: SystemConfig) -> Self {
        let (cstring_path, cstring_path_len) = into_cstr(database_path).unwrap();

        dbg!(&cstring_path.as_bytes(), cstring_path_len);

        let mut __this = ::std::mem::MaybeUninit::uninit();

        unsafe {
            ffi::kuzu_main_Database_Database1(
                __this.as_mut_ptr(),
                cstring_path.as_ptr(),
                cstring_path_len,
                system_config,
            );
            __this.assume_init()
        }
    }

    unsafe fn resize_buffer_manager(&mut self, new_size: u64) {
        ffi::kuzu_main_Database_resizeBufferManager(self, new_size);
    }

    unsafe fn set_logging_level(log_level: LogLevel) {
        let (cstring_log_level, cstring_log_level_len) = into_cstr(log_level.as_str()).unwrap();

        dbg!(&cstring_log_level.as_bytes(), cstring_log_level_len);

        ffi::kuzu_main_Database_setLoggingLevel(cstring_log_level.as_ptr(), cstring_log_level_len);
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        helper::drop_logger(helper::LoggerEnum::Database);
        helper::drop_logger(helper::LoggerEnum::CsvReader);
        helper::drop_logger(helper::LoggerEnum::Loader);
        helper::drop_logger(helper::LoggerEnum::Processor);
        helper::drop_logger(helper::LoggerEnum::BufferManager);
        helper::drop_logger(helper::LoggerEnum::Catalog);
        helper::drop_logger(helper::LoggerEnum::Storage);
        helper::drop_logger(helper::LoggerEnum::TransactionManager);
        helper::drop_logger(helper::LoggerEnum::Wal);
    }
}

mod ffi {
    extern "C" {
        #[link_name = "\u{1}_ZN4kuzu4main8DatabaseC1EPKcm"]
        pub fn kuzu_main_Database_Database(
            this: *mut super::Database,
            database_path: *const ::std::os::raw::c_char,
            path_size: usize,
        );

        #[link_name = "\u{1}_ZN4kuzu4main8DatabaseC1EPKcmNS0_12SystemConfigE"]
        pub fn kuzu_main_Database_Database1(
            this: *mut super::Database,
            database_path: *const ::std::os::raw::c_char,
            path_size: usize,
            system_config: super::SystemConfig,
        );

        #[link_name = "\u{1}_ZN4kuzu4main8Database19resizeBufferManagerEm"]
        pub fn kuzu_main_Database_resizeBufferManager(this: *mut super::Database, new_size: u64);

        #[link_name = "\u{1}_ZN4kuzu4main8DatabaseD1Ev"]
        pub fn kuzu_main_Database_Database_destructor(this: *mut super::Database);

        #[link_name = "\u{1}_ZN4kuzu4main8Database15setLoggingLevelEPKcm"]
        pub fn kuzu_main_Database_setLoggingLevel(
            loggingLevel: *const ::std::os::raw::c_char,
            loggingLevelSize: usize,
        );
    }
}
