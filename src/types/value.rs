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

impl LogicaType {
    fn from_kuzu_val(val: *mut ffi::kuzu_value) -> Self {
        assert!(!val.is_null());
        let logical_type = unsafe { ffi::kuzu_value_get_data_type(val) };
        assert!(!logical_type.is_null());

        Self::new(logical_type).unwrap()
    }
}

pub struct KuzuVal {
    pub(crate) val: *mut ffi::kuzu_value,
    pub(crate) logical_type: LogicaType,
}

impl KuzuVal {
    pub(crate) fn new(val: *mut ffi::kuzu_value) -> Self {
        let logical_type = LogicaType::from_kuzu_val(val);
        KuzuVal { val, logical_type }
    }
}
impl Drop for KuzuVal {
    fn drop(&mut self) {
        unsafe { ffi::kuzu_value_destroy(self.val) }
    }
}
pub trait Value {
    fn to_kuzu(&self) -> KuzuVal;
}

impl Value for bool {
    fn to_kuzu(&self) -> KuzuVal {
        let val = unsafe { ffi::kuzu_value_create_bool(*self) };
        let logical_type = LogicaType::from_kuzu_val(val);

        KuzuVal { val, logical_type }
    }
}

impl Value for i16 {
    fn to_kuzu(&self) -> KuzuVal {
        let val = unsafe { ffi::kuzu_value_create_int16(*self) };
        let logical_type = LogicaType::from_kuzu_val(val);

        KuzuVal { val, logical_type }
    }
}

impl Value for i32 {
    fn to_kuzu(&self) -> KuzuVal {
        let val = unsafe { ffi::kuzu_value_create_int32(*self) };
        let logical_type = LogicaType::from_kuzu_val(val);

        KuzuVal { val, logical_type }
    }
}

impl Value for i64 {
    fn to_kuzu(&self) -> KuzuVal {
        let val = unsafe { ffi::kuzu_value_create_int64(*self) };
        let logical_type = LogicaType::from_kuzu_val(val);

        KuzuVal { val, logical_type }
    }
}

impl Value for f32 {
    fn to_kuzu(&self) -> KuzuVal {
        let val = unsafe { ffi::kuzu_value_create_float(*self) };
        let logical_type = LogicaType::from_kuzu_val(val);

        KuzuVal { val, logical_type }
    }
}

impl Value for f64 {
    fn to_kuzu(&self) -> KuzuVal {
        let val = unsafe { ffi::kuzu_value_create_double(*self) };
        let logical_type = LogicaType::from_kuzu_val(val);

        KuzuVal { val, logical_type }
    }
}

pub(crate) mod ffi {
    #[repr(C)]
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
