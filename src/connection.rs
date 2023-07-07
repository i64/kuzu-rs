use crate::{error, ptrc::PtrContainer};
use std::{
    cell::RefCell,
    ops::{Deref, DerefMut},
};

use super::database;
use crate::ffi;

/// Represents a connection to a database.

pub struct Connection {
    /// The inner reference cell containing the pointer to the Kuzu connection.
    inner: RefCell<PtrContainer<ffi::kuzu_connection>>,
    /// Transaction statistics for the connection.
    tx_stats: TransactionStats,
}

impl Connection {
    /// Creates a new connection to the specified database.
    pub fn new(database: &mut database::Database) -> error::Result<Self> {
        unsafe {
            let this = PtrContainer::try_new(ffi::kuzu_connection_init(database.inner.0))?;
            Ok(Self {
                inner: RefCell::new(this),
                tx_stats: TransactionStats::default(),
            })
        }
    }

    /// Sets the maximum number of threads to use for executing queries.
    pub fn set_max_num_thread_for_exec(&mut self, num_threads: u64) {
        unsafe { ffi::kuzu_connection_set_max_num_thread_for_exec(self.to_inner(), num_threads) }
    }

    /// Returns the maximum number of threads used for executing queries.
    pub fn max_num_thread_for_exec(&mut self) -> u64 {
        unsafe { ffi::kuzu_connection_get_max_num_thread_for_exec(self.to_inner()) }
    }

    /// Returns the inner pointer to the `kuzu_connection` struct.
    pub(crate) fn to_inner(&self) -> *mut ffi::kuzu_connection {
        self.inner.borrow().0
    }

    /// Begins a new transaction with the specified transaction type.
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

/// Represents the type of a transaction.
pub enum TransactionType {
    ///  Indicates a read-only transaction. This type of transaction is used for executing read operations on the database without modifying the data.
    Readonly,
    /// Indicates a read-write transaction. This type of transaction allows both read and write operations, including modifications to the database.
    ReadWrite,
}

/// Represents the behavior when dropping a transaction.
pub enum DropBehaviour {
    /// Roll back the changes made in the transaction when it is dropped.
    RollBack,
    /// Commit the changes made in the transaction when it is dropped.
    Commit,
    /// Ignore the changes made in the transaction when it is dropped.
    Ignore,
    /// Panic when the transaction is dropped, indicating an unexpected situation.
    Panic,
}

/// Holds transaction statistics.
#[derive(Default)]
pub struct TransactionStats {
    /// The number of read-only transactions.
    r: usize,
    /// The number of read-write transactions.
    rw: usize,
}

/// Represents a transaction within a connection.
pub struct Transaction<'conn> {
    conn: &'conn mut Connection,
    transaction_type: TransactionType,
    on_drop: DropBehaviour,
    commited: bool,
}

impl<'c> Transaction<'c> {
    /// Creates a new transaction with the specified connection and transaction type.
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

    /// Commits the transaction.
    pub fn commit(&mut self) {
        self.commited = true;
        unsafe { ffi::kuzu_connection_commit(self.conn.to_inner()) }
    }

    /// Rolls back the transaction.
    pub fn rollback(&self) {
        unsafe { ffi::kuzu_connection_rollback(self.conn.to_inner()) }
    }

    /// Finishes the transaction, either by committing or rolling back.
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
