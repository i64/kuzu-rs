use crate::helper::PtrContainer;

use crate::{error, ffi};

#[repr(u32)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum LogicalTypeID {
    Any = 0,
    Node = 10,
    Rel = 11,
    Bool = 22,
    Int64 = 23,
    Int32 = 24,
    Int16 = 25,
    Double = 26,
    Float = 27,
    Date = 28,
    Timestamp = 29,
    Interval = 30,
    FixedList = 31,
    InternalId = 40,
    String = 50,
    VarList = 52,
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

#[derive(Debug)]
pub(crate) struct LogicaType {
    pub(crate) tid: LogicalTypeID,
    pub(crate) fixed_num_elements_in_list: u64,
}

impl PartialEq for LogicaType {
    fn eq(&self, other: &Self) -> bool {
        self.tid == other.tid
    }
}

impl LogicaType {
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
        Ok(logical_type.validate()?.try_into()?)
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
