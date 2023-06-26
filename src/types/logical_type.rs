use crate::helper::PtrContainer;

#[repr(u32)]
#[derive(PartialEq, Clone, Copy)]
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
    type Error = ();

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
            _ => Err(()),
        }
    }
}

pub(crate) struct LogicaType {
    pub(crate) tid: LogicalTypeID,
    inner_ptr: *mut ffi::kuzu_logical_type,
    fixed_num_elements_in_list: u64,
}

impl PartialEq for LogicaType {
    fn eq(&self, other: &Self) -> bool {
        self.tid == other.tid
    }
}

impl Clone for LogicaType {
    fn clone(&self) -> Self {
        let new_inner = unsafe { ffi::kuzu_data_type_clone(self.inner_ptr) };

        assert!(!new_inner.is_null());

        Self {
            tid: self.tid,
            inner_ptr: new_inner,
            fixed_num_elements_in_list: self.fixed_num_elements_in_list,
        }
    }
}

impl From<PtrContainer<ffi::kuzu_logical_type>> for LogicaType {
    fn from(value: PtrContainer<ffi::kuzu_logical_type>) -> Self {
        if value.0.is_null() {
            unimplemented!()
        }

        let tid = {
            let _tid = unsafe { ffi::kuzu_data_type_get_id(value.0) };
            LogicalTypeID::try_from(_tid).unwrap()
        };

        let fixed_num_elements_in_list =
            unsafe { ffi::kuzu_data_type_get_fixed_num_elements_in_list(value.0) };

        Self::new_with_id(tid, value.0, fixed_num_elements_in_list).unwrap()
    }
}
impl LogicaType {
    fn new_with_id(
        tid: LogicalTypeID,
        inner: *mut ffi::kuzu_logical_type,
        fixed_num_elements_in_list: u64,
    ) -> Option<Self> {
        match inner.is_null() {
            true => None,
            false => Some(Self {
                inner_ptr: inner,
                tid,
                fixed_num_elements_in_list,
            }),
        }
    }

    fn new_subtype(&self, tid: LogicalTypeID, fixed_num_elements_in_list: u64) -> Option<Self> {
        let ptr = unsafe {
            ffi::kuzu_data_type_create(tid as u32, self.inner_ptr, fixed_num_elements_in_list)
        };

        Self::new_with_id(tid, ptr, fixed_num_elements_in_list)
    }
}

pub(crate) mod ffi {
    #[repr(C)]
    pub struct kuzu_logical_type {
        _data_type: *mut ::std::os::raw::c_void,
    }

    extern "C" {
        pub fn kuzu_data_type_create(
            id: u32,
            child_type: *mut kuzu_logical_type,
            fixed_num_elements_in_list: u64,
        ) -> *mut kuzu_logical_type;

        pub fn kuzu_data_type_clone(data_type: *mut kuzu_logical_type) -> *mut kuzu_logical_type;
        pub fn kuzu_data_type_destroy(data_type: *mut kuzu_logical_type);
        pub fn kuzu_data_type_equals(
            data_type1: *mut kuzu_logical_type,
            data_type2: *mut kuzu_logical_type,
        ) -> bool;
        pub fn kuzu_data_type_get_id(data_type: *mut kuzu_logical_type) -> u32;
        pub fn kuzu_data_type_get_fixed_num_elements_in_list(
            data_type: *mut kuzu_logical_type,
        ) -> u64;
    }
}
