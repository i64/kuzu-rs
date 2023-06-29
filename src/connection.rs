use crate::{error, helper::PtrContainer};
use std::cell::RefCell;

use super::database;
use crate::ffi;
#[repr(u8)]
pub(crate) enum ConnectionTransactionMode {
    AutoCommit = 0,
    Manual = 1,
}

pub struct Connection(RefCell<PtrContainer<ffi::kuzu_connection>>);

impl Connection {
    pub fn new(database: &mut database::Database) -> error::Result<Self> {
        unsafe {
            let this = PtrContainer(ffi::kuzu_connection_init(database.0));
            Ok(Self(RefCell::new(this.validate()?)))
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
