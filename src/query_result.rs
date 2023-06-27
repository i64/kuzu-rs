use std::{ffi::CStr, marker::PhantomData};

use crate::{
    connection::Connection,
    helper::PtrContainer,
    into_cstr,
    types::row::{FromRow, Row},
};

pub struct QueryResult(*mut ffi::kuzu_query_result);

impl From<PtrContainer<ffi::kuzu_query_result>> for QueryResult {
    fn from(value: PtrContainer<ffi::kuzu_query_result>) -> Self {
        if value.0.is_null() {
            // return null
        }

        unsafe {
            if !ffi::kuzu_query_result_is_success(value.0) {
                let s = CStr::from_ptr(ffi::kuzu_query_result_get_error_message(value.0)).to_str();
                panic!("{}", s.unwrap())
            }
        }

        Self(value.0)
    }
}

impl QueryResult {
    pub fn iter<'a, R: FromRow<'a>>(&'a self) -> Iter<'a, R> {
        let column_len = unsafe { ffi::kuzu_query_result_get_num_columns(self.0) };
        let len = unsafe { ffi::kuzu_query_result_get_num_tuples(self.0) } as usize;
        Iter {
            _m: PhantomData,
            inner: self,
            column_len,
            len,
        }
    }
}

impl Drop for QueryResult {
    fn drop(&mut self) {
        unsafe { ffi::kuzu_query_result_destroy(self.0) }
    }
}

pub struct Iter<'qr, R: FromRow<'qr>> {
    inner: &'qr QueryResult,
    column_len: u64,
    len: usize,
    _m: PhantomData<R>,
}

impl<R> Iterator for Iter<'_, R>
where
    R: for<'a> FromRow<'a>,
{
    type Item = R;
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if ffi::kuzu_query_result_has_next(self.inner.0) {
                let _row = ffi::kuzu_query_result_get_next(self.inner.0);
                assert!(!_row.is_null());
                let row = Row::new(_row, self.column_len);
                self.len -= 1;
                return Self::Item::from_row(&row);
            }
        }
        None
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl Connection {
    pub fn query<S: AsRef<str>>(&self, query: S) -> QueryResult {
        let cst = into_cstr!(query.as_ref());
        let raw_result = unsafe { ffi::kuzu_connection_query(self.to_inner(), cst) };
        PtrContainer(raw_result).into()
    }
}

pub(crate) mod ffi {
    use crate::{connection::ffi::kuzu_connection, types::value::ffi::kuzu_value};

    #[repr(C)]
    pub struct kuzu_query_result {
        _query_result: *mut ::std::os::raw::c_void,
    }

    #[repr(C)]
    pub struct kuzu_flat_tuple {
        _flat_tuple: *mut ::std::os::raw::c_void,
    }

    extern "C" {
        pub fn kuzu_connection_query(
            connection: *mut kuzu_connection,
            query: *const ::std::os::raw::c_char,
        ) -> *mut kuzu_query_result;

        pub fn kuzu_query_result_destroy(query_result: *mut kuzu_query_result);

        pub fn kuzu_query_result_is_success(query_result: *mut kuzu_query_result) -> bool;

        pub fn kuzu_query_result_get_error_message(
            query_result: *mut kuzu_query_result,
        ) -> *mut ::std::os::raw::c_char;

        pub fn kuzu_query_result_get_num_columns(query_result: *mut kuzu_query_result) -> u64;

        // pub fn kuzu_query_result_get_column_data_type(
        //     query_result: *mut kuzu_query_result,
        //     index: u64,
        // ) -> *mut kuzu_logical_type;

        pub fn kuzu_query_result_get_num_tuples(query_result: *mut kuzu_query_result) -> u64;

        // pub fn kuzu_query_result_get_query_summary(
        //     query_result: *mut kuzu_query_result,
        // ) -> *mut kuzu_query_summary;

        pub fn kuzu_query_result_has_next(query_result: *mut kuzu_query_result) -> bool;

        pub fn kuzu_query_result_get_next(
            query_result: *mut kuzu_query_result,
        ) -> *mut kuzu_flat_tuple;

        pub fn kuzu_flat_tuple_get_value(
            flat_tuple: *mut kuzu_flat_tuple,
            index: u64,
        ) -> *mut kuzu_value;

        // pub fn kuzu_query_result_to_string(
        //     query_result: *mut kuzu_query_result,
        // ) -> *mut ::std::os::raw::c_char;

        // pub fn kuzu_query_result_write_to_csv(
        //     query_result: *mut kuzu_query_result,
        //     file_path: *const ::std::os::raw::c_char,
        //     delimiter: ::std::os::raw::c_char,
        //     escape_char: ::std::os::raw::c_char,
        //     new_line: ::std::os::raw::c_char,
        // );

        // pub fn kuzu_query_result_reset_iterator(query_result: *mut kuzu_query_result);
    }
}
