#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_database {
    pub _database: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_connection {
    pub _connection: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_prepared_statement {
    pub _prepared_statement: *mut ::std::os::raw::c_void,
    pub _bound_values: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_query_result {
    pub _query_result: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_flat_tuple {
    pub _flat_tuple: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_logical_type {
    pub _data_type: *mut ::std::os::raw::c_void,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_value {
    pub _value: *mut ::std::os::raw::c_void,
    pub _is_owned_by_cpp: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_node_val {
    pub _node_val: *mut ::std::os::raw::c_void,
    pub _is_owned_by_cpp: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_rel_val {
    pub _rel_val: *mut ::std::os::raw::c_void,
    pub _is_owned_by_cpp: bool,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_internal_id_t {
    pub table_id: u64,
    pub offset: u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_date_t {
    pub days: i32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_timestamp_t {
    pub value: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct kuzu_interval_t {
    pub months: i32,
    pub days: i32,
    pub micros: i64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
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
pub type kuzu_data_type_id = ::std::os::raw::c_uint;

extern "C" {
    pub fn kuzu_database_init(
        database_path: *const ::std::os::raw::c_char,
        buffer_pool_size: u64,
    ) -> *mut kuzu_database;

    pub fn kuzu_database_destroy(database: *mut kuzu_database);

    pub fn kuzu_database_set_logging_level(logging_level: *const ::std::os::raw::c_char);

    pub fn kuzu_connection_init(database: *mut kuzu_database) -> *mut kuzu_connection;

    pub fn kuzu_connection_destroy(connection: *mut kuzu_connection);

    pub fn kuzu_connection_begin_read_only_transaction(connection: *mut kuzu_connection);

    pub fn kuzu_connection_begin_write_transaction(connection: *mut kuzu_connection);

    pub fn kuzu_connection_commit(connection: *mut kuzu_connection);

    pub fn kuzu_connection_rollback(connection: *mut kuzu_connection);

    pub fn kuzu_connection_set_max_num_thread_for_exec(
        connection: *mut kuzu_connection,
        num_threads: u64,
    );

    pub fn kuzu_connection_get_max_num_thread_for_exec(connection: *mut kuzu_connection) -> u64;

    pub fn kuzu_connection_query(
        connection: *mut kuzu_connection,
        query: *const ::std::os::raw::c_char,
    ) -> *mut kuzu_query_result;

    pub fn kuzu_connection_prepare(
        connection: *mut kuzu_connection,
        query: *const ::std::os::raw::c_char,
    ) -> *mut kuzu_prepared_statement;

    pub fn kuzu_connection_execute(
        connection: *mut kuzu_connection,
        prepared_statement: *mut kuzu_prepared_statement,
    ) -> *mut kuzu_query_result;

    pub fn kuzu_connection_get_node_table_names(
        connection: *mut kuzu_connection,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_connection_get_rel_table_names(
        connection: *mut kuzu_connection,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_connection_get_node_property_names(
        connection: *mut kuzu_connection,
        table_name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_connection_get_rel_property_names(
        connection: *mut kuzu_connection,
        table_name: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_connection_interrupt(connection: *mut kuzu_connection);

    pub fn kuzu_connection_set_query_timeout(connection: *mut kuzu_connection, timeout_in_ms: u64);

    pub fn kuzu_prepared_statement_destroy(prepared_statement: *mut kuzu_prepared_statement);

    pub fn kuzu_prepared_statement_allow_active_transaction(
        prepared_statement: *mut kuzu_prepared_statement,
    ) -> bool;

    pub fn kuzu_prepared_statement_is_success(
        prepared_statement: *mut kuzu_prepared_statement,
    ) -> bool;

    pub fn kuzu_prepared_statement_get_error_message(
        prepared_statement: *mut kuzu_prepared_statement,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_prepared_statement_bind_bool(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *mut ::std::os::raw::c_char,
        value: bool,
    );

    pub fn kuzu_prepared_statement_bind_int64(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *mut ::std::os::raw::c_char,
        value: i64,
    );

    pub fn kuzu_prepared_statement_bind_int32(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *mut ::std::os::raw::c_char,
        value: i32,
    );

    pub fn kuzu_prepared_statement_bind_int16(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *mut ::std::os::raw::c_char,
        value: i16,
    );

    pub fn kuzu_prepared_statement_bind_double(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *mut ::std::os::raw::c_char,
        value: f64,
    );

    pub fn kuzu_prepared_statement_bind_float(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *mut ::std::os::raw::c_char,
        value: f32,
    );

    pub fn kuzu_prepared_statement_bind_date(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *mut ::std::os::raw::c_char,
        value: kuzu_date_t,
    );

    pub fn kuzu_prepared_statement_bind_timestamp(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *mut ::std::os::raw::c_char,
        value: kuzu_timestamp_t,
    );

    pub fn kuzu_prepared_statement_bind_interval(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *mut ::std::os::raw::c_char,
        value: kuzu_interval_t,
    );

    pub fn kuzu_prepared_statement_bind_string(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *mut ::std::os::raw::c_char,
        value: *mut ::std::os::raw::c_char,
    );

    pub fn kuzu_prepared_statement_bind_value(
        prepared_statement: *mut kuzu_prepared_statement,
        param_name: *const ::std::os::raw::c_char,
        value: *mut kuzu_value,
    );

    pub fn kuzu_query_result_destroy(query_result: *mut kuzu_query_result);

    pub fn kuzu_query_result_is_success(query_result: *mut kuzu_query_result) -> bool;

    pub fn kuzu_query_result_get_error_message(
        query_result: *mut kuzu_query_result,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_query_result_get_num_columns(query_result: *mut kuzu_query_result) -> u64;

    pub fn kuzu_query_result_get_column_name(
        query_result: *mut kuzu_query_result,
        index: u64,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_query_result_get_column_data_type(
        query_result: *mut kuzu_query_result,
        index: u64,
    ) -> *mut kuzu_logical_type;

    pub fn kuzu_query_result_get_num_tuples(query_result: *mut kuzu_query_result) -> u64;

    pub fn kuzu_query_result_get_query_summary(
        query_result: *mut kuzu_query_result,
    ) -> *mut kuzu_query_summary;

    pub fn kuzu_query_result_has_next(query_result: *mut kuzu_query_result) -> bool;

    pub fn kuzu_query_result_get_next(query_result: *mut kuzu_query_result)
        -> *mut kuzu_flat_tuple;

    pub fn kuzu_query_result_to_string(
        query_result: *mut kuzu_query_result,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_query_result_write_to_csv(
        query_result: *mut kuzu_query_result,
        file_path: *const ::std::os::raw::c_char,
        delimiter: ::std::os::raw::c_char,
        escape_char: ::std::os::raw::c_char,
        new_line: ::std::os::raw::c_char,
    );

    pub fn kuzu_query_result_reset_iterator(query_result: *mut kuzu_query_result);

    pub fn kuzu_flat_tuple_destroy(flat_tuple: *mut kuzu_flat_tuple);

    pub fn kuzu_flat_tuple_get_value(
        flat_tuple: *mut kuzu_flat_tuple,
        index: u64,
    ) -> *mut kuzu_value;

    pub fn kuzu_flat_tuple_to_string(
        flat_tuple: *mut kuzu_flat_tuple,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_data_type_create(
        id: kuzu_data_type_id,
        child_type: *mut kuzu_logical_type,
        fixed_num_elements_in_list: u64,
    ) -> *mut kuzu_logical_type;

    pub fn kuzu_data_type_clone(data_type: *mut kuzu_logical_type) -> *mut kuzu_logical_type;

    pub fn kuzu_data_type_destroy(data_type: *mut kuzu_logical_type);

    pub fn kuzu_data_type_equals(
        data_type1: *mut kuzu_logical_type,
        data_type2: *mut kuzu_logical_type,
    ) -> bool;

    pub fn kuzu_data_type_get_id(data_type: *mut kuzu_logical_type) -> kuzu_data_type_id;

    pub fn kuzu_data_type_get_fixed_num_elements_in_list(data_type: *mut kuzu_logical_type) -> u64;

    pub fn kuzu_value_create_null() -> *mut kuzu_value;

    pub fn kuzu_value_create_null_with_data_type(
        data_type: *mut kuzu_logical_type,
    ) -> *mut kuzu_value;

    pub fn kuzu_value_is_null(value: *mut kuzu_value) -> bool;

    pub fn kuzu_value_set_null(value: *mut kuzu_value, is_null: bool);

    pub fn kuzu_value_create_default(data_type: *mut kuzu_logical_type) -> *mut kuzu_value;

    pub fn kuzu_value_create_bool(val_: bool) -> *mut kuzu_value;

    pub fn kuzu_value_create_int16(val_: i16) -> *mut kuzu_value;

    pub fn kuzu_value_create_int32(val_: i32) -> *mut kuzu_value;

    pub fn kuzu_value_create_int64(val_: i64) -> *mut kuzu_value;

    pub fn kuzu_value_create_float(val_: f32) -> *mut kuzu_value;

    pub fn kuzu_value_create_double(val_: f64) -> *mut kuzu_value;

    pub fn kuzu_value_create_internal_id(val_: kuzu_internal_id_t) -> *mut kuzu_value;

    pub fn kuzu_value_create_node_val(val_: *mut kuzu_node_val) -> *mut kuzu_value;

    pub fn kuzu_value_create_rel_val(val_: *mut kuzu_rel_val) -> *mut kuzu_value;

    pub fn kuzu_value_create_date(val_: kuzu_date_t) -> *mut kuzu_value;

    pub fn kuzu_value_create_timestamp(val_: kuzu_timestamp_t) -> *mut kuzu_value;

    pub fn kuzu_value_create_interval(val_: kuzu_interval_t) -> *mut kuzu_value;

    pub fn kuzu_value_create_string(val_: *const ::std::os::raw::c_char) -> *mut kuzu_value;

    pub fn kuzu_value_clone(value: *mut kuzu_value) -> *mut kuzu_value;

    pub fn kuzu_value_copy(value: *mut kuzu_value, other: *mut kuzu_value);

    pub fn kuzu_value_destroy(value: *mut kuzu_value);

    pub fn kuzu_value_get_list_size(value: *mut kuzu_value) -> u64;

    pub fn kuzu_value_get_list_element(value: *mut kuzu_value, index: u64) -> *mut kuzu_value;

    pub fn kuzu_value_get_struct_num_fields(value: *mut kuzu_value) -> u64;

    pub fn kuzu_value_get_struct_field_name(
        value: *mut kuzu_value,
        index: u64,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_value_get_struct_field_value(value: *mut kuzu_value, index: u64)
        -> *mut kuzu_value;

    pub fn kuzu_value_get_data_type(value: *mut kuzu_value) -> *mut kuzu_logical_type;

    pub fn kuzu_value_get_bool(value: *mut kuzu_value) -> bool;

    pub fn kuzu_value_get_int16(value: *mut kuzu_value) -> i16;

    pub fn kuzu_value_get_int32(value: *mut kuzu_value) -> i32;

    pub fn kuzu_value_get_int64(value: *mut kuzu_value) -> i64;

    pub fn kuzu_value_get_float(value: *mut kuzu_value) -> f32;

    pub fn kuzu_value_get_double(value: *mut kuzu_value) -> f64;

    pub fn kuzu_value_get_internal_id(value: *mut kuzu_value) -> kuzu_internal_id_t;

    pub fn kuzu_value_get_node_val(value: *mut kuzu_value) -> *mut kuzu_node_val;

    pub fn kuzu_value_get_rel_val(value: *mut kuzu_value) -> *mut kuzu_rel_val;

    pub fn kuzu_value_get_date(value: *mut kuzu_value) -> kuzu_date_t;

    pub fn kuzu_value_get_timestamp(value: *mut kuzu_value) -> kuzu_timestamp_t;

    pub fn kuzu_value_get_interval(value: *mut kuzu_value) -> kuzu_interval_t;

    pub fn kuzu_value_get_string(value: *mut kuzu_value) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_value_to_string(value: *mut kuzu_value) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_node_val_create(
        id: kuzu_internal_id_t,
        label: *mut ::std::os::raw::c_char,
    ) -> *mut kuzu_node_val;

    pub fn kuzu_node_val_clone(node_val: *mut kuzu_node_val) -> *mut kuzu_node_val;

    pub fn kuzu_node_val_destroy(node_val: *mut kuzu_node_val);

    pub fn kuzu_node_val_get_id_val(node_val: *mut kuzu_node_val) -> *mut kuzu_value;

    pub fn kuzu_node_val_get_label_val(node_val: *mut kuzu_node_val) -> *mut kuzu_value;

    pub fn kuzu_node_val_get_id(node_val: *mut kuzu_node_val) -> kuzu_internal_id_t;

    pub fn kuzu_node_val_get_label_name(
        node_val: *mut kuzu_node_val,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_node_val_get_property_size(node_val: *mut kuzu_node_val) -> u64;

    pub fn kuzu_node_val_get_property_name_at(
        node_val: *mut kuzu_node_val,
        index: u64,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_node_val_get_property_value_at(
        node_val: *mut kuzu_node_val,
        index: u64,
    ) -> *mut kuzu_value;

    pub fn kuzu_node_val_add_property(
        node_val: *mut kuzu_node_val,
        name: *const ::std::os::raw::c_char,
        property: *mut kuzu_value,
    );

    pub fn kuzu_node_val_to_string(node_val: *mut kuzu_node_val) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_rel_val_create(
        src_id: kuzu_internal_id_t,
        dst_id: kuzu_internal_id_t,
        label: *mut ::std::os::raw::c_char,
    ) -> *mut kuzu_rel_val;

    pub fn kuzu_rel_val_clone(rel_val: *mut kuzu_rel_val) -> *mut kuzu_rel_val;

    pub fn kuzu_rel_val_destroy(rel_val: *mut kuzu_rel_val);

    pub fn kuzu_rel_val_get_src_id_val(rel_val: *mut kuzu_rel_val) -> *mut kuzu_value;

    pub fn kuzu_rel_val_get_dst_id_val(rel_val: *mut kuzu_rel_val) -> *mut kuzu_value;

    pub fn kuzu_rel_val_get_src_id(rel_val: *mut kuzu_rel_val) -> kuzu_internal_id_t;

    pub fn kuzu_rel_val_get_dst_id(rel_val: *mut kuzu_rel_val) -> kuzu_internal_id_t;

    pub fn kuzu_rel_val_get_label_name(rel_val: *mut kuzu_rel_val) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_rel_val_get_property_size(rel_val: *mut kuzu_rel_val) -> u64;

    pub fn kuzu_rel_val_get_property_name_at(
        rel_val: *mut kuzu_rel_val,
        index: u64,
    ) -> *mut ::std::os::raw::c_char;

    pub fn kuzu_rel_val_get_property_value_at(
        rel_val: *mut kuzu_rel_val,
        index: u64,
    ) -> *mut kuzu_value;

    pub fn kuzu_rel_val_add_property(
        rel_val: *mut kuzu_rel_val,
        name: *mut ::std::os::raw::c_char,
        property: *mut kuzu_value,
    );

    pub fn kuzu_rel_val_to_string(rel_val: *mut kuzu_rel_val) -> *mut ::std::os::raw::c_char;
    pub fn kuzu_query_summary_destroy(query_summary: *mut kuzu_query_summary);

    pub fn kuzu_query_summary_get_compiling_time(query_summary: *mut kuzu_query_summary) -> f64;

    pub fn kuzu_query_summary_get_execution_time(query_summary: *mut kuzu_query_summary) -> f64;
}
