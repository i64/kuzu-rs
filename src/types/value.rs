use crate::helper::PtrContainer;

use self::ffi::kuzu_value;

use super::logical_type::LogicaType;

pub enum TimeUnit {
    /// Time in seconds.
    Second(i64),
    /// Time in milliseconds.
    Millisecond(i64),
    /// Time in microseconds.
    Microsecond(i64),
    /// Time in nanoseconds.
    Nanosecond(i64),
}


impl From<&PtrContainer<kuzu_value>> for LogicaType {
    fn from(value: &PtrContainer<kuzu_value>) -> Self {
        assert!(!value.0.is_null());
        let logical_type = unsafe { ffi::kuzu_value_get_data_type(value.0) };
        assert!(!logical_type.is_null());

        PtrContainer(logical_type).into() 
    }
}

#[derive(Debug)]
pub struct KuzuVal {
    pub(crate) val: PtrContainer<ffi::kuzu_value>,
    pub(crate) logical_type: LogicaType,
}

impl From<PtrContainer<ffi::kuzu_value>> for KuzuVal {
    fn from(value: PtrContainer<ffi::kuzu_value>) -> Self {
        let logical_type = LogicaType::from(&value);
        KuzuVal {
            val: value,
            logical_type,
        }
    }
}

impl Drop for KuzuVal {
    fn drop(&mut self) {
        unsafe { ffi::kuzu_value_destroy(self.val.0) }
    }
}

impl From<bool> for KuzuVal {
    fn from(value: bool) -> Self {
        let val = PtrContainer(unsafe { ffi::kuzu_value_create_bool(value) });
        let logical_type = LogicaType::from(&val);

        KuzuVal { val, logical_type }
    }
}

impl From<i16> for KuzuVal {
    fn from(value: i16) -> Self {
        let val = PtrContainer(unsafe { ffi::kuzu_value_create_int16(value) });
        let logical_type = LogicaType::from(&val);

        KuzuVal { val, logical_type }
    }
}

impl From<i32> for KuzuVal {
    fn from(value: i32) -> Self {
        let val = PtrContainer(unsafe { ffi::kuzu_value_create_int32(value) });
        let logical_type = LogicaType::from(&val);

        KuzuVal { val, logical_type }
    }
}

impl From<i64> for KuzuVal {
    fn from(value: i64) -> Self {
        let val = PtrContainer(unsafe { ffi::kuzu_value_create_int64(value) });
        let logical_type = LogicaType::from(&val);

        KuzuVal { val, logical_type }
    }
}

impl From<f32> for KuzuVal {
    fn from(value: f32) -> Self {
        let val = PtrContainer(unsafe { ffi::kuzu_value_create_float(value) });
        let logical_type = LogicaType::from(&val);

        KuzuVal { val, logical_type }
    }
}

impl From<f64> for KuzuVal {
    fn from(value: f64) -> Self {
        let val = PtrContainer(unsafe { ffi::kuzu_value_create_double(value) });
        let logical_type = LogicaType::from(&val);

        KuzuVal { val, logical_type }
    }
}

pub(crate) mod ffi {
    #[repr(C)]
    #[derive(Debug)]
    pub struct kuzu_value {
        _value: *mut ::std::os::raw::c_void,
        _is_owned_by_cpp: bool,
    }

    extern "C" {
        pub fn kuzu_value_get_data_type(
            value: *mut kuzu_value,
        ) -> *mut crate::types::logical_type::ffi::kuzu_logical_type;

        pub fn kuzu_value_destroy(value: *mut kuzu_value);
        pub fn kuzu_value_create_bool(val_: bool) -> *mut kuzu_value;
        pub fn kuzu_value_create_int16(val_: i16) -> *mut kuzu_value;
        pub fn kuzu_value_create_int32(val_: i32) -> *mut kuzu_value;
        pub fn kuzu_value_create_int64(val_: i64) -> *mut kuzu_value;
        pub fn kuzu_value_create_float(val_: f32) -> *mut kuzu_value;
        pub fn kuzu_value_create_double(val_: f64) -> *mut kuzu_value;
        // pub fn kuzu_value_create_default(data_type: *mut kuzu_logical_type) -> *mut kuzu_value;
        // pub fn kuzu_value_create_internal_id(val_: kuzu_internal_id_t) -> *mut kuzu_value;
        // pub fn kuzu_value_create_node_val(val_: *mut kuzu_node_val) -> *mut kuzu_value;
        // pub fn kuzu_value_create_rel_val(val_: *mut kuzu_rel_val) -> *mut kuzu_value;
        // pub fn kuzu_value_create_date(val_: kuzu_date_t) -> *mut kuzu_value;
        // pub fn kuzu_value_create_timestamp(val_: kuzu_timestamp_t) -> *mut kuzu_value;
        // pub fn kuzu_value_create_interval(val_: kuzu_interval_t) -> *mut kuzu_value;
        // pub fn kuzu_value_create_string(val_: *mut ::std::os::raw::c_char) -> *mut kuzu_value;
    }
}
