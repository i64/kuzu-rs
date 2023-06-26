use std::ops::Deref;

use crate::connection::{self, Connection};

pub enum TransactionType {
    Readonly,
    Writeonly,
}

pub enum DropBehaviour {
    RollBack,
    Commit,
    Ignore,
    Panic,
}

pub struct Transaction<'conn> {
    conn: &'conn connection::Connection,
    transaction_type: TransactionType,
    on_drop: DropBehaviour,
    commited: bool,
}

impl<'c> Transaction<'c> {
    fn new(conn: &'c connection::Connection, transaction_type: TransactionType) -> Transaction<'_> {
        unsafe {
            match transaction_type {
                TransactionType::Readonly => {
                    ffi::kuzu_connection_begin_read_only_transaction(conn.to_inner())
                }
                TransactionType::Writeonly => {
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

    fn commit(&mut self) {
        self.commited = true;
        unsafe { ffi::kuzu_connection_commit(self.conn.to_inner()) }
    }

    fn rollback(&self) {
        unsafe { ffi::kuzu_connection_rollback(self.conn.to_inner()) }
    }

    fn finish(&self) {
        if !self.commited {
            self.rollback()
        }
    }
}

impl Deref for Transaction<'_> {
    type Target = Connection;

    #[inline]
    fn deref(&self) -> &Connection {
        self.conn
    }
}

impl Drop for Transaction<'_> {
    #[inline]
    fn drop(&mut self) {
        self.finish();
    }
}

impl Connection {
    pub fn transaction(&self, transaction_type: TransactionType) -> Transaction {
        Transaction::new(self, transaction_type)
    }
}

pub(crate) mod ffi {
    extern "C" {
        pub fn kuzu_connection_begin_read_only_transaction(
            connection: *mut super::connection::ffi::kuzu_connection,
        );
        pub fn kuzu_connection_begin_write_transaction(
            connection: *mut super::connection::ffi::kuzu_connection,
        );
        pub fn kuzu_connection_commit(connection: *mut super::connection::ffi::kuzu_connection);
        pub fn kuzu_connection_rollback(connection: *mut super::connection::ffi::kuzu_connection);
    }
}
