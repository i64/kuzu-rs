use std::{collections::HashMap, rc::Rc};

use crate::error;

use super::value::KuzuValue;

/// Represents a row in Kuzu.
#[derive(Debug)]
pub struct Row {
    /// The mapping of keys (column names) to their respective indices in the `values` vector.
    keys: Rc<HashMap<String, usize>>,
    /// The actual values of the row.
    values: Vec<KuzuValue>,
}

impl Row {
    
    pub(crate) fn new(keys: Rc<HashMap<String, usize>>, values: Vec<KuzuValue>) -> Self {
        Self { keys, values }
    }

    /// Returns a reference to the `KuzuValue` if it exists, or an `Error::ColumnNotFound` if the index is out of bounds.
    pub fn get_ref(&self, idx: usize) -> error::Result<&KuzuValue> {
        self.values
            .get(idx)
            .ok_or(error::Error::ColumnNotFound(idx.to_string()))
    }

    
    /// Returns the converted value if it exists, or an `Error::ColumnNotFound`  if the index is out of bounds, or `Error::DecodeError` if the the wrong type is specified.
    pub fn get_val<T: TryFrom<KuzuValue, Error = error::Error>>(
        &self,
        idx: usize,
    ) -> error::Result<T> {
        let val = self
            .values
            .get(idx)
            .ok_or(error::Error::ColumnNotFound(idx.to_string()))?;

        val.clone().try_into()
    }

    /// Returns a reference to the `KuzuValue` if it exists, or an `Error::ColumnNotFound` if the column not found.
    pub fn get_ref_by_column<S: AsRef<str>>(&self, column_name: S) -> error::Result<&KuzuValue> {
        let key_idx = self
            .keys
            .get(column_name.as_ref())
            .ok_or(error::Error::ColumnNotFound(column_name.as_ref().into()))?;
        let inner_val = unsafe { self.values.get_unchecked(*key_idx) };

        Ok(inner_val)
    }

    /// Returns the converted value if it exists, or an `Error::ColumnNotFound`  if the column not found, or `Error::DecodeError` if the the wrong type is specified.
    pub fn get_val_by_column<T: TryFrom<KuzuValue, Error = error::Error>, S: AsRef<str>>(
        &self,
        column_name: S,
    ) -> error::Result<T> {
        let key_idx = self
            .keys
            .get(column_name.as_ref())
            .ok_or(error::Error::ColumnNotFound(column_name.as_ref().into()))?;

        let inner_val = unsafe { self.values.get_unchecked(*key_idx) };
        inner_val.clone().try_into()
    }
}

// Macro to generate TryFrom<Row> implementations for tuples of varying lengths
macro_rules! impl_from_row_for_tuple {
    ($( ($idx:tt) -> $T:ident );+;) => {
        impl<$($T,)+> TryFrom<Row> for ($($T,)+)
        where
            $($T: TryFrom<KuzuValue,  Error = error::Error>,)+

        {
            type Error = error::Error;

            #[inline]
            fn try_from(row: Row) -> Result<Self, Self::Error> {
                Ok(($(row.get_val::<$T>($idx)?,)+))
             }
        }
    };
}

impl_from_row_for_tuple!(
    (0) -> T1;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
    (11) -> T12;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
    (11) -> T12;
    (12) -> T13;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
    (11) -> T12;
    (12) -> T13;
    (13) -> T14;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
    (11) -> T12;
    (12) -> T13;
    (13) -> T14;
    (14) -> T15;
);

impl_from_row_for_tuple!(
    (0) -> T1;
    (1) -> T2;
    (2) -> T3;
    (3) -> T4;
    (4) -> T5;
    (5) -> T6;
    (6) -> T7;
    (7) -> T8;
    (8) -> T9;
    (9) -> T10;
    (10) -> T11;
    (11) -> T12;
    (12) -> T13;
    (13) -> T14;
    (14) -> T15;
    (15) -> T16;
);
