#![allow(warnings, unused)]
/// Kuzu database manages all database components.
#[repr(C)]
pub struct kuzu_database {
    pub _database: *mut ::std::os::raw::c_void,
}

/// `kuzu_connection` is used to interact with a `Database` instance. Each connection is
/// thread-safe. Multiple connections can connect to the same `Database` instance in a multi-threaded
/// environment.
#[repr(C)]
pub struct kuzu_connection {
    pub _connection: *mut ::std::os::raw::c_void,
}

/// `kuzu_prepared_statement` is a parameterized query which can avoid planning the same query
/// for repeated execution.
#[repr(C)]
pub struct kuzu_prepared_statement {
    pub _prepared_statement: *mut ::std::os::raw::c_void,
    pub _bound_values: *mut ::std::os::raw::c_void,
}

/// `kuzu_query_result` stores the result of a query.
#[repr(C)]
pub struct kuzu_query_result {
    pub _query_result: *mut ::std::os::raw::c_void,
}

/// `kuzu_flat_tuple` stores a vector of values.
#[repr(C)]
pub struct kuzu_flat_tuple {
    pub _flat_tuple: *mut ::std::os::raw::c_void,
}

/// `kuzu_logical_type` is the kuzu internal representation of data types.
#[repr(C)]
pub struct kuzu_logical_type {
    pub _data_type: *mut ::std::os::raw::c_void,
}

/// `kuzu_value` is used to represent a value with any kuzu internal dataType.
#[repr(C)]
pub struct kuzu_value {
    pub _value: *mut ::std::os::raw::c_void,
    pub _is_owned_by_cpp: bool,
}

/// `kuzu_node_val` is a kuzu internal node type which stores the nodeID, label, and properties of a node in a graph.
#[repr(C)]
pub struct kuzu_node_val {
    pub _node_val: *mut ::std::os::raw::c_void,
    pub _is_owned_by_cpp: bool,
}

/// `kuzu_rel_val` is a kuzu internal rel type which stores the relID, src/dst nodes, and properties of a rel in a graph.
#[repr(C)]
pub struct kuzu_rel_val {
    pub _rel_val: *mut ::std::os::raw::c_void,
    pub _is_owned_by_cpp: bool,
}

/// `kuzu_internal_id_t` is a kuzu internal internal_id type which stores the table_id and offset of a node/rel.
#[repr(C)]
pub struct kuzu_internal_id_t {
    pub table_id: u64,
    pub offset: u64,
}

/// `kuzu_date_t` is a kuzu internal date type which stores the number of days since 1970-01-01 00:00:00 UTC.
#[repr(C)]
pub struct kuzu_date_t {
    pub days: i32,
}

/// `kuzu_timestamp_t` is a kuzu internal timestamp type which stores the number of microseconds since 1970-01-01 00:00:00 UTC.
#[repr(C)]
pub struct kuzu_timestamp_t {
    pub value: i64,
}

/// `kuzu_interval_t` is a kuzu internal interval type which stores the months, days, and microseconds.
#[repr(C)]
pub struct kuzu_interval_t {
    pub months: i32,
    pub days: i32,
    pub micros: i64,
}

/// `kuzu_query_summary` stores the execution time, plan, compiling time, and query options of a query.
#[repr(C)]
pub struct kuzu_query_summary {
    pub _query_summary: *mut ::std::os::raw::c_void,
}

pub const kuzu_data_type_id_KUZU_ANY: kuzu_data_type_id = 0;
pub const kuzu_data_type_id_KUZU_NODE: kuzu_data_type_id = 10;
pub const kuzu_data_type_id_KUZU_REL: kuzu_data_type_id = 11;
pub const kuzu_data_type_id_KUZU_BOOL: kuzu_data_type_id = 22;
pub const kuzu_data_type_id_KUZU_INT64: kuzu_data_type_id = 23;
pub const kuzu_data_type_id_KUZU_INT32: kuzu_data_type_id = 24;
pub const kuzu_data_type_id_KUZU_INT16: kuzu_data_type_id = 25;
pub const kuzu_data_type_id_KUZU_DOUBLE: kuzu_data_type_id = 26;
pub const kuzu_data_type_id_KUZU_FLOAT: kuzu_data_type_id = 27;
pub const kuzu_data_type_id_KUZU_DATE: kuzu_data_type_id = 28;
pub const kuzu_data_type_id_KUZU_TIMESTAMP: kuzu_data_type_id = 29;
pub const kuzu_data_type_id_KUZU_INTERVAL: kuzu_data_type_id = 30;
pub const kuzu_data_type_id_KUZU_FIXED_LIST: kuzu_data_type_id = 31;
pub const kuzu_data_type_id_KUZU_INTERNAL_ID: kuzu_data_type_id = 40;
pub const kuzu_data_type_id_KUZU_STRING: kuzu_data_type_id = 50;
pub const kuzu_data_type_id_KUZU_VAR_LIST: kuzu_data_type_id = 52;
pub const kuzu_data_type_id_KUZU_STRUCT: kuzu_data_type_id = 53;

/// Enum class for kuzu internal dataTypes.
pub type kuzu_data_type_id = ::std::os::raw::c_uint;

extern "C" {
    /// Allocates memory and creates a kuzu database instance at `database_path` with
    /// `bufferPoolSize=buffer_pool_size`. Caller is responsible for calling `kuzu_database_destroy()` to
    /// release the allocated memory.
    ///
    /// # Arguments
    ///
    /// * `database_path` - The path to the database.
    /// * `buffer_pool_size` - The size of the buffer pool in bytes.
    ///
    /// # Returns
    ///
    /// The database instance.
    pub fn kuzu_database_init(
        database_path: *const ::std::os::raw::c_char,
        buffer_pool_size: u64,
    ) -> *mut kuzu_database;

    /// Destroys the kuzu database instance and frees the allocated memory.
    ///
    /// # Arguments
    ///
    /// * `database` - The database instance to destroy.
    pub fn kuzu_database_destroy(database: *mut kuzu_database);

    /// Sets the logging level of the database.
    ///
    /// # Arguments
    ///
    /// * `logging_level` - The logging level to set. Supported logging levels are: "info", "debug", "err".
    pub fn kuzu_database_set_logging_level(logging_level: *const ::std::os::raw::c_char);

    /// Allocates memory and creates a connection to the database. Caller is responsible for
    /// calling `kuzu_connection_destroy()` to release the allocated memory.
    ///
    /// # Arguments
    ///
    /// * `database` - The database instance to connect to.
    ///
    /// # Returns
    ///
    /// The connection instance.
    pub fn kuzu_connection_init(database: *mut kuzu_database) -> *mut kuzu_connection;

    /// Destroys the connection instance and frees the allocated memory.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to destroy.
    pub fn kuzu_connection_destroy(connection: *mut kuzu_connection);

    /// Begins a read-only transaction in the given connection.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to begin read-only transaction.
    pub fn kuzu_connection_begin_read_only_transaction(connection: *mut kuzu_connection);

    /// Begins a write transaction in the given connection.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to begin write transaction.
    pub fn kuzu_connection_begin_write_transaction(connection: *mut kuzu_connection);

    /// Commits the current transaction.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to commit transaction.
    pub fn kuzu_connection_commit(connection: *mut kuzu_connection);

    /// Rollbacks the current transaction.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to rollback transaction.
    pub fn kuzu_connection_rollback(connection: *mut kuzu_connection);

    /// Sets the maximum number of threads to use for executing queries.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to set max number of threads for execution.
    /// * `num_threads` - The maximum number of threads to use for executing queries.
    pub fn kuzu_connection_set_max_num_thread_for_exec(
        connection: *mut kuzu_connection,
        num_threads: u64,
    );

    /// Returns the maximum number of threads of the connection to use for executing queries.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to return max number of threads for execution.
    ///
    /// # Returns
    ///
    /// The maximum number of threads for executing queries.
    pub fn kuzu_connection_get_max_num_thread_for_exec(connection: *mut kuzu_connection) -> u64;

    /// Executes the given query and returns the result.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to execute the query.
    /// * `query` - The query to execute.
    ///
    /// # Returns
    ///
    /// The result of the query.
    pub fn kuzu_connection_query(
        connection: *mut kuzu_connection,
        query: *const ::std::os::raw::c_char,
    ) -> *mut kuzu_query_result;

    /// Prepares the given query and returns the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to prepare the query.
    /// * `query` - The query to prepare.
    pub fn kuzu_connection_prepare(
        connection: *mut kuzu_connection,
        query: *const ::std::os::raw::c_char,
    ) -> *mut kuzu_prepared_statement;

    /// Executes the prepared statement using the connection.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to execute the prepared statement.
    /// * `prepared_statement` - The prepared statement to execute.
    pub fn kuzu_connection_execute(
        connection: *mut kuzu_connection,
        prepared_statement: *mut kuzu_prepared_statement,
    ) -> *mut kuzu_query_result;

    /// Returns all node table names of the database.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to return all node table names.
    ///
    /// # Returns
    ///
    /// A string containing all node table names.
    pub fn kuzu_connection_get_node_table_names(
        connection: *mut kuzu_connection,
    ) -> *const ::std::os::raw::c_char;

    /// Returns all rel table names of the database.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to return all rel table names.
    ///
    /// # Returns
    ///
    /// A string containing all rel table names.
    pub fn kuzu_connection_get_rel_table_names(
        connection: *mut kuzu_connection,
    ) -> *const ::std::os::raw::c_char;

    /// Returns all property names of the given node table.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to return all property names.
    /// * `table_name` - The table name to return all property names.
    ///
    /// # Returns
    ///
    /// A string containing all property names of the given node table.
    pub fn kuzu_connection_get_node_property_names(
        connection: *mut kuzu_connection,
        table_name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;

    /// Returns all property names of the given rel table.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to return all property names.
    /// * `table_name` - The table name to return all property names.
    ///
    /// # Returns
    ///
    /// A string containing all property names of the given rel table.
    pub fn kuzu_connection_get_rel_property_names(
        connection: *mut kuzu_connection,
        table_name: *const ::std::os::raw::c_char,
    ) -> *const ::std::os::raw::c_char;

    /// Interrupts the current query execution in the connection.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to interrupt.
    pub fn kuzu_connection_interrupt(connection: *mut kuzu_connection);

    /// Sets the query timeout value in milliseconds for the connection.
    ///
    /// # Arguments
    ///
    /// * `connection` - The connection instance to set query timeout value.
    /// * `timeout_in_ms` - The timeout value in milliseconds.
    pub fn kuzu_connection_set_query_timeout(connection: *mut kuzu_connection, timeout_in_ms: u64);

    /// Destroys the prepared statement instance and frees the allocated memory.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to destroy.
    pub fn kuzu_prepared_statement_destroy(prepared_statement: *mut kuzu_prepared_statement);

    /// Checks if the prepared statement is allowed to be part of an active transaction.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to check.
    ///
    /// # Returns
    ///
    /// `true` if the prepared statement is allowed to be part of an active transaction, `false` otherwise.
    pub fn kuzu_prepared_statement_allow_active_transaction(
        prepared_statement: *mut kuzu_prepared_statement,
    ) -> bool;

    /// Checks if the query is prepared successfully.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to check.
    ///
    /// # Returns
    ///
    /// `true` if the query is prepared successfully, `false` otherwise.
    pub fn kuzu_prepared_statement_is_success(
        prepared_statement: *mut kuzu_prepared_statement,
    ) -> bool;

    /// Returns the error message if the statement is not prepared successfully.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to check.
    ///
    /// # Returns
    ///
    /// The error message if the statement is not prepared successfully.
    pub fn kuzu_prepared_statement_get_error_message(
        prepared_statement: *mut kuzu_prepared_statement,
    ) -> *const ::std::os::raw::c_char;

    /// Binds the given boolean value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The boolean value to bind.
    pub fn kuzu_prepared_statement_bind_bool(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: bool,
    );

    /// Binds the given int64_t value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The int64_t value to bind.
    pub fn kuzu_prepared_statement_bind_int64(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: i64,
    );

    /// Binds the given int32_t value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The int32_t value to bind.
    pub fn kuzu_prepared_statement_bind_int32(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: i32,
    );

    /// Binds the given int16_t value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The int16_t value to bind.
    pub fn kuzu_prepared_statement_bind_int16(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: i16,
    );

    /// Binds the given double value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The double value to bind.
    pub fn kuzu_prepared_statement_bind_double(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: f64,
    );

    /// Binds the given float value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The float value to bind.
    pub fn kuzu_prepared_statement_bind_float(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: f32,
    );

    /// Binds the given date value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The date value to bind.
    pub fn kuzu_prepared_statement_bind_date(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: kuzu_date_t,
    );

    /// Binds the given timestamp value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The timestamp value to bind.
    pub fn kuzu_prepared_statement_bind_timestamp(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: kuzu_timestamp_t,
    );

    /// Binds the given interval value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The interval value to bind.
    pub fn kuzu_prepared_statement_bind_interval(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: kuzu_interval_t,
    );

    /// Binds the given string value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The string value to bind.
    pub fn kuzu_prepared_statement_bind_string(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: *const ::std::os::raw::c_char,
    );

    /// Binds the given kuzu value to the given parameter name in the prepared statement.
    ///
    /// # Arguments
    ///
    /// * `prepared_statement` - The prepared statement instance to bind the value.
    /// * `param_name` - The parameter name to bind the value.
    /// * `value` - The kuzu value to bind.
    pub fn kuzu_prepared_statement_bind_value(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: *mut kuzu_value,
    );

    /// Destroys the given query result instance.
    ///
    /// # Arguments
    ///
    /// * `query_result` - The query result instance to destroy.
    pub fn kuzu_query_result_destroy(query_result: *mut kuzu_query_result);

    /// Returns true if the query is executed successfully, false otherwise.
    ///
    /// # Arguments
    ///
    /// * `query_result` - The query result instance to check.
    pub fn kuzu_query_result_is_success(query_result: *mut kuzu_query_result) -> bool;

    /// Returns the error message if the query is failed.
    ///
    /// # Arguments
    ///
    /// * `query_result` - The query result instance to check and return the error message.
    pub fn kuzu_query_result_get_error_message(
        query_result: *mut kuzu_query_result,
    ) -> *const ::std::os::raw::c_char;

    /// Returns the number of columns in the query result.
    ///
    /// # Arguments
    ///
    /// * `query_result` - The query result instance to return.
    pub fn kuzu_query_result_get_num_columns(query_result: *mut kuzu_query_result) -> u64;

    /// Returns the column name at the given index.
    ///
    /// # Arguments
    ///
    /// * `query_result` - The query result instance to return.
    /// * `index` - The index of the column to return the name.
    pub fn kuzu_query_result_get_column_name(
        query_result: *mut kuzu_query_result,
        index: u64,
    ) -> *const ::std::os::raw::c_char;

    /// Returns the data type of the column at the given index.
    ///
    /// # Arguments
    ///
    /// * `query_result` - The query result instance to return.
    /// * `index` - The index of the column to return the data type.
    pub fn kuzu_query_result_get_column_data_type(
        query_result: *mut kuzu_query_result,
        index: u64,
    ) -> *mut kuzu_logical_type;

    /// Returns the number of tuples in the query result.
    ///
    /// # Arguments
    ///
    /// * `query_result` - The query result instance to return.
    pub fn kuzu_query_result_get_num_tuples(query_result: *mut kuzu_query_result) -> u64;

    /// Returns the query summary of the query result.
    ///
    /// # Arguments
    ///
    /// * `query_result` - The query result instance to return.
    pub fn kuzu_query_result_get_query_summary(
        query_result: *mut kuzu_query_result,
    ) -> *mut kuzu_query_summary;

    /// Returns true if we have not consumed all tuples in the query result, false otherwise.
    ///
    /// # Arguments
    /// * `query_result` - The query result instance to check.
    pub fn kuzu_query_result_has_next(query_result: *mut kuzu_query_result) -> bool;

    /// Returns the next tuple in the query result. Throws an exception if there is no more tuple.
    ///
    /// # Arguments
    /// * `query_result` - The query result instance to return.
    pub fn kuzu_query_result_get_next(query_result: *mut kuzu_query_result)
        -> *mut kuzu_flat_tuple;

    /// Returns the query result as a string.
    ///
    /// # Arguments
    /// * `query_result` - The query result instance to return.
    pub fn kuzu_query_result_to_string(
        query_result: *mut kuzu_query_result,
    ) -> *const ::std::os::raw::c_char;

    /// Writes the query result to the given file path as CSV.
    ///
    /// # Arguments
    /// * `query_result` - The query result instance to write.
    /// * `file_path` - The file path to write the query result.
    /// * `delimiter` - The delimiter character to use when writing the CSV file.
    /// * `escape_char` - The escape character to use when writing the CSV file.
    /// * `new_line` - The new line character to use when writing the CSV file.
    pub fn kuzu_query_result_write_to_csv(
        query_result: *mut kuzu_query_result,
        file_path: *const ::std::os::raw::c_char,
        delimiter: ::std::os::raw::c_char,
        escape_char: ::std::os::raw::c_char,
        new_line: ::std::os::raw::c_char,
    );

    /// Resets the iterator of the query result to the beginning of the query result.
    ///
    /// # Arguments
    /// * `query_result` - The query result instance to reset the iterator.
    pub fn kuzu_query_result_reset_iterator(query_result: *mut kuzu_query_result);

    /// Destroys the given flat tuple instance.
    ///
    /// # Arguments
    /// * `flat_tuple` - The flat tuple instance to destroy.
    pub fn kuzu_flat_tuple_destroy(flat_tuple: *mut kuzu_flat_tuple);

    /// Returns the value at the index of the flat tuple.
    ///
    /// # Arguments
    /// * `flat_tuple` - The flat tuple instance to return.
    /// * `index` - The index of the value to return.
    pub fn kuzu_flat_tuple_get_value(
        flat_tuple: *mut kuzu_flat_tuple,
        index: u64,
    ) -> *mut kuzu_value;

    /// Converts the flat tuple to a string.
    ///
    /// # Arguments
    /// * `flat_tuple` - The flat tuple instance to convert.
    pub fn kuzu_flat_tuple_to_string(
        flat_tuple: *mut kuzu_flat_tuple,
    ) -> *const ::std::os::raw::c_char;

    /// Creates a data type instance with the given id, childType, and fixed_num_elements_in_list.
    /// Caller is responsible for destroying the returned data type instance.
    ///
    /// # Arguments
    /// * `id` - The enum type id of the datatype to create.
    /// * `child_type` - The child type of the datatype to create (only used for nested dataTypes).
    /// * `fixed_num_elements_in_list` - The fixed number of elements in the list (only used for FIXED_LIST).
    pub fn kuzu_data_type_create(
        id: kuzu_data_type_id,
        child_type: *mut kuzu_logical_type,
        fixed_num_elements_in_list: u64,
    ) -> *mut kuzu_logical_type;

    /// Creates a new data type instance by cloning the given data type instance.
    ///
    /// # Arguments
    /// * `data_type` - The data type instance to clone.
    pub fn kuzu_data_type_clone(data_type: *mut kuzu_logical_type) -> *mut kuzu_logical_type;

    /// Destroys the given data type instance.
    ///
    /// # Arguments
    /// * `data_type` - The data type instance to destroy.
    pub fn kuzu_data_type_destroy(data_type: *mut kuzu_logical_type);

    /// Returns true if the given data type is equal to the other data type, false otherwise.
    ///
    /// # Arguments
    /// * `data_type1` - The first data type instance to compare.
    /// * `data_type2` - The second data type instance to compare.
    pub fn kuzu_data_type_equals(
        data_type1: *mut kuzu_logical_type,
        data_type2: *mut kuzu_logical_type,
    ) -> bool;

    /// Returns the enum type id of the given data type.
    ///
    /// # Arguments
    /// * `data_type` - The data type instance to return.
    pub fn kuzu_data_type_get_id(data_type: *mut kuzu_logical_type) -> kuzu_data_type_id;

    /// Returns the number of elements per list for fixedSizeList.
    ///
    /// # Arguments
    /// * `data_type` - The data type instance to return.
    pub fn kuzu_data_type_get_fixed_num_elements_in_list(data_type: *mut kuzu_logical_type) -> u64;

    /// Creates a NULL value of ANY type. Caller is responsible for destroying the returned value.
    pub fn kuzu_value_create_null() -> *mut kuzu_value;

    /// Creates a value of the given data type. Caller is responsible for destroying the returned value.
    ///
    /// # Arguments
    /// * `data_type` - The data type of the value to create.
    pub fn kuzu_value_create_null_with_data_type(
        data_type: *mut kuzu_logical_type,
    ) -> *mut kuzu_value;

    /// Returns true if the given value is NULL, false otherwise.
    ///
    /// # Arguments
    /// * `value` - The value instance to check.
    pub fn kuzu_value_is_null(value: *mut kuzu_value) -> bool;

    /// Sets the given value to NULL or not.
    ///
    /// # Arguments
    /// * `value` - The value instance to set.
    /// * `is_null` - True if sets the value to NULL, false otherwise.
    pub fn kuzu_value_set_null(value: *mut kuzu_value, is_null: bool);

    /// Creates a value of the given data type with default non-NULL value. Caller is responsible
    /// for destroying the returned value.
    ///
    /// # Arguments
    /// * `data_type` - The data type of the value to create.
    pub fn kuzu_value_create_default(data_type: *mut kuzu_logical_type) -> *mut kuzu_value;

    /// Creates a value with boolean type and the given bool value. Caller is responsible for
    /// destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The bool value of the value to create.
    pub fn kuzu_value_create_bool(val_: bool) -> *mut kuzu_value;

    /// Creates a value with int16 type and the given int16 value. Caller is responsible for
    /// destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The int16 value of the value to create.
    pub fn kuzu_value_create_int16(val_: i16) -> *mut kuzu_value;

    /// Creates a value with int32 type and the given int32 value. Caller is responsible for
    /// destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The int32 value of the value to create.
    pub fn kuzu_value_create_int32(val_: i32) -> *mut kuzu_value;

    /// Creates a value with int64 type and the given int64 value. Caller is responsible for
    /// destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The int64 value of the value to create.
    pub fn kuzu_value_create_int64(val_: i64) -> *mut kuzu_value;

    /// Creates a value with float type and the given float value. Caller is responsible for
    /// destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The float value of the value to create.
    pub fn kuzu_value_create_float(val_: f32) -> *mut kuzu_value;

    /// Creates a value with double type and the given double value. Caller is responsible for
    /// destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The double value of the value to create.
    pub fn kuzu_value_create_double(val_: f64) -> *mut kuzu_value;

    /// Creates a value with internal_id type and the given internal_id value. Caller is
    /// responsible for destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The internal_id value of the value to create.
    pub fn kuzu_value_create_internal_id(val_: kuzu_internal_id_t) -> *mut kuzu_value;

    /// Creates a value with nodeVal type and the given nodeVal value. Caller is responsible for
    /// destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The nodeVal value of the value to create.
    pub fn kuzu_value_create_node_val(val_: *mut kuzu_node_val) -> *mut kuzu_value;

    /// Creates a value with relVal type and the given relVal value. Caller is responsible for
    /// destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The relVal value of the value to create.
    pub fn kuzu_value_create_rel_val(val_: *mut kuzu_rel_val) -> *mut kuzu_value;

    /// Creates a value with date type and the given date value. Caller is responsible for
    /// destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The date value of the value to create.
    pub fn kuzu_value_create_date(val_: kuzu_date_t) -> *mut kuzu_value;

    /// Creates a value with timestamp type and the given timestamp value. Caller is responsible
    /// for destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The timestamp value of the value to create.
    pub fn kuzu_value_create_timestamp(val_: kuzu_timestamp_t) -> *mut kuzu_value;

    /// Creates a value with interval type and the given interval value. Caller is responsible
    /// for destroying the returned value.
    ///
    /// # Arguments
    /// * `val_` - The interval value of the value to create.
    pub fn kuzu_value_create_interval(val_: kuzu_interval_t) -> *mut kuzu_value;

    /// Creates a value with string type and the given string value. Caller is responsible for
    /// destroying the returned value.
    ///
    /// # Arguments
    ///
    /// * `val_` - The string value of the value to create.
    ///
    /// # Safety
    ///
    /// The caller is responsible for ensuring that the `val_` parameter is a valid pointer.
    pub fn kuzu_value_create_string(val_: *const ::std::os::raw::c_char) -> *mut kuzu_value;

    /// Creates a new value based on the given value. Caller is responsible for destroying the
    /// returned value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to create from.
    pub fn kuzu_value_clone(value: *mut kuzu_value) -> *mut kuzu_value;

    /// Copies the other value to the value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to copy to.
    /// * `other` - The value to copy from.
    pub fn kuzu_value_copy(value: *mut kuzu_value, other: *mut kuzu_value);

    /// Destroys the value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to destroy.
    pub fn kuzu_value_destroy(value: *mut kuzu_value);

    /// Returns the number of elements per list of the given value. The value must be of type
    /// FIXED_LIST.
    ///
    /// # Arguments
    ///
    /// * `value` - The FIXED_LIST value to get list size.
    pub fn kuzu_value_get_list_size(value: *mut kuzu_value) -> u64;

    /// Returns the element at index of the given value. The value must be of type VAR_LIST.
    ///
    /// # Arguments
    ///
    /// * `value` - The VAR_LIST value to return.
    /// * `index` - The index of the element to return.
    pub fn kuzu_value_get_list_element(value: *mut kuzu_value, index: u64) -> *mut kuzu_value;

    /// Returns the number of fields of the given struct value. The value must be of type STRUCT.
    ///
    /// # Arguments
    ///
    /// * `value` - The STRUCT value to get number of fields.
    pub fn kuzu_value_get_struct_num_fields(value: *mut kuzu_value) -> u64;

    /// Returns the field name at index of the given struct value. The value must be of type
    /// STRUCT.
    ///
    /// # Arguments
    ///
    /// * `value` - The STRUCT value to get field name.
    /// * `index` - The index of the field name to return.
    pub fn kuzu_value_get_struct_field_name(
        value: *mut kuzu_value,
        index: u64,
    ) -> *const ::std::os::raw::c_char;

    /// Returns the field value at index of the given struct value. The value must be of type
    /// STRUCT.
    ///
    /// # Arguments
    ///
    /// * `value` - The STRUCT value to get field value.
    /// * `index` - The index of the field value to return.
    pub fn kuzu_value_get_struct_field_value(value: *mut kuzu_value, index: u64)
        -> *mut kuzu_value;

    /// Returns internal type of the given value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_data_type(value: *mut kuzu_value) -> *mut kuzu_logical_type;

    /// Returns the boolean value of the given value. The value must be of type BOOL.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_bool(value: *mut kuzu_value) -> bool;

    /// Returns the int16 value of the given value. The value must be of type INT16.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_int16(value: *mut kuzu_value) -> i16;

    /// Returns the int32 value of the given value. The value must be of type INT32.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_int32(value: *mut kuzu_value) -> i32;

    /// Returns the int64 value of the given value. The value must be of type INT64.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_int64(value: *mut kuzu_value) -> i64;

    /// Returns the float value of the given value. The value must be of type FLOAT.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_float(value: *mut kuzu_value) -> f32;

    /// Returns the double value of the given value. The value must be of type DOUBLE.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_double(value: *mut kuzu_value) -> f64;

    /// Returns the internal id value of the given value. The value must be of type INTERNAL_ID.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_internal_id(value: *mut kuzu_value) -> kuzu_internal_id_t;

    /// Returns the node value of the given value. The value must be of type NODE.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_node_val(value: *mut kuzu_value) -> *mut kuzu_node_val;

    /// Returns the rel value of the given value. The value must be of type REL.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_rel_val(value: *mut kuzu_value) -> *mut kuzu_rel_val;

    /// Returns the date value of the given value. The value must be of type DATE.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_date(value: *mut kuzu_value) -> kuzu_date_t;

    /// Returns the timestamp value of the given value. The value must be of type TIMESTAMP.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_timestamp(value: *mut kuzu_value) -> kuzu_timestamp_t;

    /// Returns the interval value of the given value. The value must be of type INTERVAL.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_interval(value: *mut kuzu_value) -> kuzu_interval_t;

    /// Returns the string value of the given value. The value must be of type STRING.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to return.
    pub fn kuzu_value_get_string(value: *mut kuzu_value) -> *const ::std::os::raw::c_char;

    /// Converts the given value to string.
    ///
    /// # Arguments
    ///
    /// * `value` - The value to convert.
    pub fn kuzu_value_to_string(value: *mut kuzu_value) -> *const ::std::os::raw::c_char;

    /// Creates a new node value.
    ///
    /// # Arguments
    ///
    /// * `id` - The internal id of the node.
    /// * `label` - The label of the node.
    pub fn kuzu_node_val_create(
        id: kuzu_internal_id_t,
        label: *const ::std::os::raw::c_char,
    ) -> *mut kuzu_node_val;

    /// Creates a new node value from the given node value.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to clone.
    pub fn kuzu_node_val_clone(node_val: *mut kuzu_node_val) -> *mut kuzu_node_val;

    /// Destroys the given node value.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to destroy.
    pub fn kuzu_node_val_destroy(node_val: *mut kuzu_node_val);

    /// Returns the internal id value of the given node value as a kuzu value.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to return.
    pub fn kuzu_node_val_get_id_val(node_val: *mut kuzu_node_val) -> *mut kuzu_value;

    /// Returns the label value of the given node value as a label value.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to return.
    pub fn kuzu_node_val_get_label_val(node_val: *mut kuzu_node_val) -> *mut kuzu_value;

    /// Returns the internal id value of the given node value as internal_id.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to return.
    pub fn kuzu_node_val_get_id(node_val: *mut kuzu_node_val) -> kuzu_internal_id_t;

    /// Returns the label value of the given node value as string.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to return.
    pub fn kuzu_node_val_get_label_name(
        node_val: *mut kuzu_node_val,
    ) -> *const ::std::os::raw::c_char;

    /// Returns the number of properties of the given node value.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to return.
    pub fn kuzu_node_val_get_property_size(node_val: *mut kuzu_node_val) -> u64;

    /// Returns the property name of the given node value at the given index.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to return.
    /// * `index` - The index of the property.
    pub fn kuzu_node_val_get_property_name_at(
        node_val: *mut kuzu_node_val,
        index: u64,
    ) -> *const ::std::os::raw::c_char;

    /// Returns the property value of the given node value at the given index.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to return.
    /// * `index` - The index of the property.
    pub fn kuzu_node_val_get_property_value_at(
        node_val: *mut kuzu_node_val,
        index: u64,
    ) -> *mut kuzu_value;

    /// Adds the property with name to the given node value.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to add to.
    /// * `name` - The name of the property.
    /// * `property` - The property(in value format) to add.
    pub fn kuzu_node_val_add_property(
        node_val: *mut kuzu_node_val,
        name: *const ::std::os::raw::c_char,
        property: *mut kuzu_value,
    );

    /// Converts the given node value to string.
    ///
    /// # Arguments
    ///
    /// * `node_val` - The node value to convert.
    pub fn kuzu_node_val_to_string(node_val: *mut kuzu_node_val) -> *const ::std::os::raw::c_char;

    /// Creates a new rel value. Caller is responsible for destroying the rel value.
    ///
    /// # Arguments
    ///
    /// * `src_id` - The internal id of the source node.
    /// * `dst_id` - The internal id of the destination node.
    /// * `label` - The label of the rel.
    pub fn kuzu_rel_val_create(
        src_id: kuzu_internal_id_t,
        dst_id: kuzu_internal_id_t,
        label: *const ::std::os::raw::c_char,
    ) -> *mut kuzu_rel_val;

    /// Creates a new rel value from the given rel value.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to clone.
    pub fn kuzu_rel_val_clone(rel_val: *mut kuzu_rel_val) -> *mut kuzu_rel_val;

    /// Destroys the given rel value.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to destroy.
    pub fn kuzu_rel_val_destroy(rel_val: *mut kuzu_rel_val);

    /// Returns the internal id value of the source node of the given rel value as a kuzu value.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to return.
    pub fn kuzu_rel_val_get_src_id_val(rel_val: *mut kuzu_rel_val) -> *mut kuzu_value;

    /// Returns the internal id value of the destination node of the given rel value as a kuzu value.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to return.
    pub fn kuzu_rel_val_get_dst_id_val(rel_val: *mut kuzu_rel_val) -> *mut kuzu_value;

    /// Returns the internal id value of the source node of the given rel value.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to return.
    pub fn kuzu_rel_val_get_src_id(rel_val: *mut kuzu_rel_val) -> kuzu_internal_id_t;

    /// Returns the internal id value of the destination node of the given rel value.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to return.
    pub fn kuzu_rel_val_get_dst_id(rel_val: *mut kuzu_rel_val) -> kuzu_internal_id_t;

    /// Returns the label of the given rel value.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to return.
    pub fn kuzu_rel_val_get_label_name(rel_val: *mut kuzu_rel_val) -> *const ::std::os::raw::c_char;

    /// Returns the number of properties of the given rel value.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to return.
    pub fn kuzu_rel_val_get_property_size(rel_val: *mut kuzu_rel_val) -> u64;

    /// Returns the property name of the given rel value at the given index.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to return.
    /// * `index` - The index of the property.
    pub fn kuzu_rel_val_get_property_name_at(
        rel_val: *mut kuzu_rel_val,
        index: u64,
    ) -> *const ::std::os::raw::c_char;

    /// Returns the property of the given rel value at the given index as kuzu value.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to return.
    /// * `index` - The index of the property.
    pub fn kuzu_rel_val_get_property_value_at(
        rel_val: *mut kuzu_rel_val,
        index: u64,
    ) -> *mut kuzu_value;

    /// Adds the property with name to the given rel value.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to add to.
    /// * `name` - The name of the property.
    /// * `property` - The property(in value format) to add.
    pub fn kuzu_rel_val_add_property(
        rel_val: *mut kuzu_rel_val,
        name: *const ::std::os::raw::c_char,
        property: *mut kuzu_value,
    );

    /// Converts the given rel value to string.
    ///
    /// # Arguments
    ///
    /// * `rel_val` - The rel value to convert.
    pub fn kuzu_rel_val_to_string(rel_val: *mut kuzu_rel_val) -> *const ::std::os::raw::c_char;

    /// Destroys the given query summary.
    ///
    /// # Arguments
    ///
    /// * `query_summary` - The query summary to destroy.
    pub fn kuzu_query_summary_destroy(query_summary: *mut kuzu_query_summary);

    /// Returns the compilation time of the given query summary.
    ///
    /// # Arguments
    ///
    /// * `query_summary` - The query summary to get compilation time.
    pub fn kuzu_query_summary_get_compiling_time(query_summary: *mut kuzu_query_summary) -> f64;

    /// Returns the execution time of the given query summary.
    ///
    /// # Arguments
    ///
    /// * `query_summary` - The query summary to get execution time.
    pub fn kuzu_query_summary_get_execution_time(query_summary: *mut kuzu_query_summary) -> f64;

}
