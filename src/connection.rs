use crate::helper::InnerContainer;
use std::cell::RefCell;

use super::database;

#[repr(u8)]
pub(crate) enum ConnectionTransactionMode {
    AutoCommit = 0,
    Manual = 1,
}

pub struct Connection(RefCell<InnerContainer<*mut ffi::kuzu_connection>>);

impl Connection {
    pub fn new(database: &mut database::Database) -> Option<Self> {
        unsafe {
            let this = ffi::kuzu_connection_init(database.0);
            if this.is_null() {
                None
            } else {
                Some(Self(RefCell::new(InnerContainer(this))))
            }
        }
    }

    pub fn set_max_num_thread_for_exec(&mut self, num_threads: u64) {
        unsafe { ffi::kuzu_connection_set_max_num_thread_for_exec(self.to_inner(), num_threads) }
    }

    pub fn max_num_thread_for_exec(&mut self) -> u64 {
        unsafe { ffi::kuzu_connection_get_max_num_thread_for_exec(self.to_inner()) }
    }

    pub(crate) fn to_inner(&self) -> *mut ffi::kuzu_connection {
        self.0.borrow().0
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { ffi::kuzu_connection_destroy(self.to_inner()) }
    }
}
pub(crate) mod ffi {
    #[repr(C)]
    pub struct kuzu_connection {
        _connection: *mut ::std::os::raw::c_void,
    }

    extern "C" {
        pub fn kuzu_connection_init(
            database: *mut super::database::ffi::kuzu_database,
        ) -> *mut kuzu_connection;
        pub fn kuzu_connection_destroy(connection: *mut kuzu_connection);
        pub fn kuzu_connection_set_max_num_thread_for_exec(
            connection: *mut kuzu_connection,
            num_threads: u64,
        );
        pub fn kuzu_connection_get_max_num_thread_for_exec(connection: *mut kuzu_connection)
            -> u64;

        // pub fn kuzu_connection_get_node_table_names(
        //     connection: *mut kuzu_connection,
        // ) -> *mut ::std::os::raw::c_char;
        // pub fn kuzu_connection_get_rel_table_names(
        //     connection: *mut kuzu_connection,
        // ) -> *mut ::std::os::raw::c_char;
        // pub fn kuzu_connection_get_node_property_names(
        //     connection: *mut kuzu_connection,
        //     table_name: *const ::std::os::raw::c_char,
        // ) -> *mut ::std::os::raw::c_char;
        // pub fn kuzu_connection_get_rel_property_names(
        //     connection: *mut kuzu_connection,
        //     table_name: *const ::std::os::raw::c_char,
        // ) -> *mut ::std::os::raw::c_char;
    }
}
