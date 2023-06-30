#![doc = include_str!("../README.md")]

/// Module for handling database connections.
pub mod connection;

/// Module for working with databases.
pub mod database;

/// Module containing error types and utilities.
pub mod error;

/// Module for handling query results.
pub mod query_result;

/// Module for working with prepared statements.
pub mod prepared_statement;

/// Module defining various types used in the library.
pub mod types;

#[allow(warnings, unused)]
/// Module for handling foreign function interfaces (FFI).
mod ffi;

/// Module containing helper functions and types for internal use.
pub(crate) mod helper;
