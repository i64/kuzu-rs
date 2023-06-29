use std::{collections::HashMap, marker::PhantomData, rc::Rc};

use crate::{
    connection::Connection,
    error,
    helper::{convert_inner_to_owned_string, PtrContainer},
    into_cstr,
    types::{row::Row, value::KuzuValue},
};

use crate::ffi;

pub struct QueryResult(*mut ffi::kuzu_query_result);

impl TryFrom<PtrContainer<ffi::kuzu_query_result>> for QueryResult {
    type Error = error::Error;

    fn try_from(value: PtrContainer<ffi::kuzu_query_result>) -> Result<Self, Self::Error> {
        let is_success = unsafe { ffi::kuzu_query_result_is_success(value.validate()?.0) };

        if !is_success {
            let s = convert_inner_to_owned_string(unsafe {
                ffi::kuzu_query_result_get_error_message(value.0)
            })?;
            return Err(error::Error::QueryResultError(s));
        }

        Ok(Self(value.0))
    }
}

impl QueryResult {
    pub fn iter<R: From<Row>>(&self) -> error::Result<Iter<R>> {
        let len = unsafe { ffi::kuzu_query_result_get_num_tuples(self.0) } as usize;
        let column_len = unsafe { ffi::kuzu_query_result_get_num_columns(self.0) };

        let columns = (0..column_len)
            .map(|idx| {
                convert_inner_to_owned_string(unsafe {
                    ffi::kuzu_query_result_get_column_name(self.0, idx)
                })
                .map(|res| (res, idx as usize))
            })
            .collect::<Result<_, _>>()?;

        Ok(Iter {
            _m: PhantomData,
            inner: self,
            columns: Rc::new(columns),
            len,
        })
    }
}

impl Drop for QueryResult {
    fn drop(&mut self) {
        unsafe { ffi::kuzu_query_result_destroy(self.0) }
    }
}

pub struct Iter<'qr, R: From<Row>> {
    inner: &'qr QueryResult,
    columns: Rc<HashMap<String, usize>>,
    len: usize,
    _m: PhantomData<R>,
}

impl<'a, R> Iterator for Iter<'a, R>
where
    R: From<Row>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        let has_next = unsafe { ffi::kuzu_query_result_has_next(self.inner.0) };
        if !has_next {
            return None;
        }

        let _row = unsafe { ffi::kuzu_query_result_get_next(self.inner.0) };
        if _row.is_null() {
            return None;
        }

        let values: Vec<KuzuValue> = (0..self.columns.len())
            .map(|idx| {
                let inner = unsafe { ffi::kuzu_flat_tuple_get_value(_row, idx as u64) };
                PtrContainer(inner).try_into()
            })
            .collect::<Result<_, _>>()
            .ok()?;

        let row = Row::new(Rc::clone(&self.columns), values);
        self.len -= 1;
        Some(Self::Item::from(row))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl Connection {
    pub fn query<S: AsRef<str>>(&self, query: S) -> error::Result<QueryResult> {
        let cst = into_cstr!(query.as_ref())?;
        let raw_result = unsafe { ffi::kuzu_connection_query(self.to_inner(), cst.as_ptr()) };
        PtrContainer(raw_result).try_into()
    }
}
