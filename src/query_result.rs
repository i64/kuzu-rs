use std::{ffi::CStr, marker::PhantomData};

use crate::{
    connection::Connection,
    helper::PtrContainer,
    into_cstr,
    types::row::{FromRow, Row},
};

use crate::ffi;

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
