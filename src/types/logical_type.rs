use crate::helper::PtrContainer;

use crate::{error, ffi};

/// Represents the logical type ids used in Kuzu.
#[repr(u32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogicalTypeID {
    /// Represents any logical type.
    Any = 0,
    /// Represents a node logical type.
    Node = 10,
    /// Represents a relationship logical type.
    Rel = 11,
    /// Represents a boolean logical type.
    Bool = 22,
    /// Represents a 64-bit integer logical type.
    Int64 = 23,
    /// Represents a 32-bit integer logical type.
    Int32 = 24,
    /// Represents a 16-bit integer logical type.
    Int16 = 25,
    /// Represents a double-precision floating-point logical type.
    Double = 26,
    /// Represents a single-precision floating-point logical type.
    Float = 27,
    /// Represents a date logical type.
    Date = 28,
    /// Represents a timestamp logical type.
    Timestamp = 29,
    /// Represents an interval logical type.
    Interval = 30,
    /// Represents a fixed list logical type.
    FixedList = 31,
    /// Represents an internal ID logical type.
    InternalId = 40,
    /// Represents a string logical type.
    String = 50,
    /// Represents a variable-length list logical type.
    VarList = 52,
    /// Represents a struct logical type.
    Struct = 53,
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
        let logical_type =
            PtrContainer(unsafe { ffi::kuzu_value_get_data_type(value.validate()?.0) });
        logical_type.validate()?.try_into()
    }
}

impl TryFrom<PtrContainer<ffi::kuzu_logical_type>> for LogicaType {
    type Error = error::Error;
    fn try_from(value: PtrContainer<ffi::kuzu_logical_type>) -> Result<Self, Self::Error> {
        let tid = {
            let _tid = unsafe { ffi::kuzu_data_type_get_id(value.validate()?.0) };
            LogicalTypeID::try_from(_tid)?
        };

        let fixed_num_elements_in_list =
            unsafe { ffi::kuzu_data_type_get_fixed_num_elements_in_list(value.0) };

        Self::new_with_id(tid, value.0, fixed_num_elements_in_list)
    }
}
