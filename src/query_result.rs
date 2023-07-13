use std::{collections::HashMap, marker::PhantomData, rc::Rc};

use crate::{
    connection::Connection,
    error,
    helper::convert_inner_to_owned_string,
    into_cstr,
    ptrc::PtrContainer,
    types::{row::Row, value::KuzuValue},
};

use crate::ffi;

/// Represents the result of a query execution in Kuzu.
pub struct QueryResult(PtrContainer<ffi::kuzu_query_result>);

impl TryFrom<PtrContainer<ffi::kuzu_query_result>> for QueryResult {
    type Error = error::Error;

    fn try_from(value: PtrContainer<ffi::kuzu_query_result>) -> Result<Self, Self::Error> {
        let value = value;
        let is_success = unsafe { ffi::kuzu_query_result_is_success(value.0) };

        if !is_success {
            let s = convert_inner_to_owned_string(unsafe {
                ffi::kuzu_query_result_get_error_message(value.0)
            })?;
            return Err(error::Error::QueryResultError(s));
        }

        Ok(Self(value))
    }
}

impl QueryResult {
    /// Returns an iterator over the rows of the query result.
    ///
    /// Each iteration produces a `TryFrom<Row>` object, which represents a single row of the result set.
    ///
    /// Returns an error if there is an issue retrieving the rows from the query result.
    pub fn iter<R: TryFrom<Row>>(self) -> error::Result<Iter<R>> {
        let len = unsafe { ffi::kuzu_query_result_get_num_tuples(self.0 .0) } as usize;
        let column_len = unsafe { ffi::kuzu_query_result_get_num_columns(self.0 .0) };

        let columns = (0..column_len)
            .map(|idx| {
                convert_inner_to_owned_string(unsafe {
                    ffi::kuzu_query_result_get_column_name(self.0 .0, idx)
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

/// Iterator over the rows of a query result.
pub struct Iter<R: TryFrom<Row>> {
    inner: QueryResult,
    columns: Rc<HashMap<String, usize>>,
    len: usize,
    _m: PhantomData<R>,
}

impl<R> Iterator for Iter<R>
where
    R: TryFrom<Row>,
{
    type Item = R;

    fn next(&mut self) -> Option<Self::Item> {
        let has_next = unsafe { ffi::kuzu_query_result_has_next(self.inner.0 .0) };
        if !has_next {
            return None;
        }

        let _row = unsafe { ffi::kuzu_query_result_get_next(self.inner.0 .0) };
        if _row.is_null() {
            return None;
        }

        let values: Vec<KuzuValue> = (0..self.columns.len())
            .map(|idx| {
                let inner = unsafe { ffi::kuzu_flat_tuple_get_value(_row, idx as u64) };
                PtrContainer::try_new(inner)?.try_into()
            })
            .collect::<Result<_, _>>()
            .ok()?;

        let row = Row::new(Rc::clone(&self.columns), values);
        self.len -= 1;
        Self::Item::try_from(row).ok()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len, Some(self.len))
    }
}

impl Connection {
    /// Executes a query on the connection and returns the query result.
    /// Returns an error if there is an issue executing the query or retrieving the query result.
    pub fn query<S: AsRef<str>>(&self, query: S) -> error::Result<QueryResult> {
        let cst = into_cstr!(query.as_ref())?;
        let raw_result = unsafe { ffi::kuzu_connection_query(self.to_inner(), cst.as_ptr()) };
        PtrContainer::try_new(raw_result)?.try_into()
    }
}
