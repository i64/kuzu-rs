use crate::helper::PtrContainer;

use crate::{error, ffi};

/// Represents the logical type ids used in Kuzu.
#[repr(u32)]
#[derive(Debug, PartialEq)]
pub enum LogicalTypeID {
    /// Represents any logical type.
    Any = ffi::kuzu_data_type_id_KUZU_ANY,
    /// Represents a node logical type.
    Node = ffi::kuzu_data_type_id_KUZU_NODE,
    /// Represents a relationship logical type.
    Rel = ffi::kuzu_data_type_id_KUZU_REL,
    /// Represents a boolean logical type.
    Bool = ffi::kuzu_data_type_id_KUZU_BOOL,
    /// Represents a 64-bit integer logical type.
    Int64 = ffi::kuzu_data_type_id_KUZU_INT64,
    /// Represents a 32-bit integer logical type.
    Int32 = ffi::kuzu_data_type_id_KUZU_INT32,
    /// Represents a 16-bit integer logical type.
    Int16 = ffi::kuzu_data_type_id_KUZU_INT16,
    /// Represents a double-precision floating-point logical type.
    Double = ffi::kuzu_data_type_id_KUZU_DOUBLE,
    /// Represents a single-precision floating-point logical type.
    Float = ffi::kuzu_data_type_id_KUZU_FLOAT,
    /// Represents a date logical type.
    Date = ffi::kuzu_data_type_id_KUZU_DATE,
    /// Represents a timestamp logical type.
    Timestamp = ffi::kuzu_data_type_id_KUZU_TIMESTAMP,
    /// Represents an interval logical type.
    Interval = ffi::kuzu_data_type_id_KUZU_INTERVAL,
    /// Represents a fixed list logical type.
    FixedList = ffi::kuzu_data_type_id_KUZU_FIXED_LIST,
    /// Represents an internal ID logical type.
    InternalId = ffi::kuzu_data_type_id_KUZU_INTERNAL_ID,
    /// Represents a string logical type.
    String = ffi::kuzu_data_type_id_KUZU_STRING,
    /// Represents a variable-length list logical type.
    VarList = ffi::kuzu_data_type_id_KUZU_VAR_LIST,
    /// Represents a struct logical type.
    Struct = ffi::kuzu_data_type_id_KUZU_STRUCT,
}

impl TryFrom<u32> for LogicalTypeID {
    type Error = error::Error;

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            x if x == LogicalTypeID::Any as u32 => Ok(LogicalTypeID::Any),
            x if x == LogicalTypeID::Node as u32 => Ok(LogicalTypeID::Node),
            x if x == LogicalTypeID::Rel as u32 => Ok(LogicalTypeID::Rel),
            x if x == LogicalTypeID::Bool as u32 => Ok(LogicalTypeID::Bool),
            x if x == LogicalTypeID::Int64 as u32 => Ok(LogicalTypeID::Int64),
            x if x == LogicalTypeID::Int32 as u32 => Ok(LogicalTypeID::Int32),
            x if x == LogicalTypeID::Int16 as u32 => Ok(LogicalTypeID::Int16),
            x if x == LogicalTypeID::Double as u32 => Ok(LogicalTypeID::Double),
            x if x == LogicalTypeID::Float as u32 => Ok(LogicalTypeID::Float),
            x if x == LogicalTypeID::Date as u32 => Ok(LogicalTypeID::Date),
            x if x == LogicalTypeID::Timestamp as u32 => Ok(LogicalTypeID::Timestamp),
            x if x == LogicalTypeID::Interval as u32 => Ok(LogicalTypeID::Interval),
            x if x == LogicalTypeID::FixedList as u32 => Ok(LogicalTypeID::FixedList),
            x if x == LogicalTypeID::InternalId as u32 => Ok(LogicalTypeID::InternalId),
            x if x == LogicalTypeID::String as u32 => Ok(LogicalTypeID::String),
            x if x == LogicalTypeID::VarList as u32 => Ok(LogicalTypeID::VarList),
            x if x == LogicalTypeID::Struct as u32 => Ok(LogicalTypeID::Struct),
            _ => unreachable!(),
        }
    }
}

/// Represents a logical type used in Kuzu.
#[derive(Debug, PartialEq)]
pub(crate) struct LogicaType {
    /// The LogicalTypeID of the LogicaType.
    pub(crate) tid: LogicalTypeID,
    /// The fixed number of elements in a list.
    pub(crate) fixed_num_elements_in_list: u64,
}

impl LogicaType {
    /// Creates a new `LogicaType` with the provided logical type ID, inner pointer,
    /// and fixed number of elements in the list.
    ///
    /// Returns a the newly created `LogicaType` if successful, or
    /// an `Error::FFIGotNull` if the inner pointer is null.
    fn new_with_id(
        tid: LogicalTypeID,
        inner: *mut ffi::kuzu_logical_type,
        fixed_num_elements_in_list: u64,
    ) -> error::Result<Self> {
        match inner.is_null() {
            true => Err(error::Error::FFIGotNull(std::any::type_name::<Self>())),
            false => Ok(Self {
                tid,
                fixed_num_elements_in_list,
            }),
        }
    }
}

impl TryFrom<&PtrContainer<ffi::kuzu_value>> for LogicaType {
    type Error = error::Error;
    fn try_from(value: &PtrContainer<ffi::kuzu_value>) -> Result<Self, Self::Error> {
        if value.0.is_null() {
            return Err(error::Error::FFIGotNull("LogicalType"));
        }
        let logical_type =
            PtrContainer::try_new(unsafe { ffi::kuzu_value_get_data_type(value.0) })?;
        logical_type.try_into()
    }
}

impl TryFrom<PtrContainer<ffi::kuzu_logical_type>> for LogicaType {
    type Error = error::Error;
    fn try_from(value: PtrContainer<ffi::kuzu_logical_type>) -> Result<Self, Self::Error> {
        let value = value;
        let tid = {
            let _tid = unsafe { ffi::kuzu_data_type_get_id(value.0) };
            LogicalTypeID::try_from(_tid)?
        };

        let fixed_num_elements_in_list =
            unsafe { ffi::kuzu_data_type_get_fixed_num_elements_in_list(value.0) };

        Self::new_with_id(tid, value.0, fixed_num_elements_in_list)
    }
}
