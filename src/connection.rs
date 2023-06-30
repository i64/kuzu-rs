use crate::{error, helper::PtrContainer};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use super::database;
use crate::ffi;

pub struct Connection {
    inner: RefCell<PtrContainer<ffi::kuzu_connection>>,
    tx_stats: TransactionStats,
}

impl Connection {
    pub fn new(database: &mut database::Database) -> error::Result<Self> {
        unsafe {
            let this = PtrContainer(ffi::kuzu_connection_init(database.0));
            Ok(Self {
                inner: RefCell::new(this.validate()?),
                tx_stats: TransactionStats::default(),
            })
        }
    }

    pub fn set_max_num_thread_for_exec(&mut self, num_threads: u64) {
        unsafe { ffi::kuzu_connection_set_max_num_thread_for_exec(self.to_inner(), num_threads) }
    }

    pub fn max_num_thread_for_exec(&mut self) -> u64 {
        unsafe { ffi::kuzu_connection_get_max_num_thread_for_exec(self.to_inner()) }
    }

    pub(crate) fn to_inner(&self) -> *mut ffi::kuzu_connection {
        self.inner.borrow().0
    }

    pub fn transaction(&mut self, transaction_type: TransactionType) -> error::Result<Transaction> {
        match transaction_type {
            TransactionType::ReadWrite if self.tx_stats.rw > 0 => {
                return Err(error::Error::MultipleWriteTxNotAllowed)
            }
            TransactionType::ReadWrite => self.tx_stats.rw += 1,
            TransactionType::Readonly => self.tx_stats.r += 1,
        }
        Ok(Transaction::new(self, transaction_type))
    }
}

impl Drop for Connection {
    fn drop(&mut self) {
        unsafe { ffi::kuzu_connection_destroy(self.to_inner()) }
    }
}

pub enum TransactionType {
    Readonly,
    ReadWrite,
}

pub enum DropBehaviour {
    RollBack,
    Commit,
    Ignore,
    Panic,
}

#[derive(Default)]
struct TransactionStats {
    r: usize,
    rw: usize,
}
pub struct Transaction<'conn> {
    conn: &'conn mut Connection,
    transaction_type: TransactionType,
    on_drop: DropBehaviour,
    commited: bool,
}

impl<'c> Transaction<'c> {
    fn new(conn: &'c mut Connection, transaction_type: TransactionType) -> Transaction<'_> {
        unsafe {
            match transaction_type {
                TransactionType::Readonly => {
                    ffi::kuzu_connection_begin_read_only_transaction(conn.to_inner())
                }
                TransactionType::ReadWrite => {
                    ffi::kuzu_connection_begin_write_transaction(conn.to_inner())
                }
            }
        }

        Self {
            conn,
            transaction_type,
            commited: false,
            on_drop: DropBehaviour::RollBack,
        }
    }

    pub fn commit(&mut self) {
        self.commited = true;
        unsafe { ffi::kuzu_connection_commit(self.conn.to_inner()) }
    }

    pub fn rollback(&self) {
        unsafe { ffi::kuzu_connection_rollback(self.conn.to_inner()) }
    }

    pub fn finish(&mut self) {
        if !self.commited {
            self.rollback()
        }

        match self.transaction_type {
            TransactionType::ReadWrite => self.conn.tx_stats.rw -= 1,
            TransactionType::Readonly => self.conn.tx_stats.r -= 1,
        }
    }
}

impl Deref for Transaction<'_> {
    type Target = Connection;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.conn
    }
}

impl DerefMut for Transaction<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.conn
    }
}
impl Drop for Transaction<'_> {
    #[inline]
    fn drop(&mut self) {
        self.finish();
    }
}
