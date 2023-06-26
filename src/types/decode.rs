use super::{logical_type::LogicalTypeID, value::KuzuVal};

pub trait Decode
where
    Self: Sized,
{
    fn decode(kuzu_val: &KuzuVal) -> Self;
    fn check_type(kuzu_val: &KuzuVal) -> bool;
}

impl Decode for bool {
    fn decode(kuzu_val: &KuzuVal) -> Self {
        assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_bool(kuzu_val.val) }
    }

    fn check_type(kuzu_val: &KuzuVal) -> bool {
        kuzu_val.logical_type.tid == LogicalTypeID::Bool
    }
}

impl Decode for i16 {
    fn decode(kuzu_val: &KuzuVal) -> Self {
        assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_int16(kuzu_val.val) }
    }

    fn check_type(kuzu_val: &KuzuVal) -> bool {
        kuzu_val.logical_type.tid == LogicalTypeID::Int16
    }
}

impl Decode for i32 {
    fn decode(kuzu_val: &KuzuVal) -> Self {
        assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_int32(kuzu_val.val) }
    }

    fn check_type(kuzu_val: &KuzuVal) -> bool {
        kuzu_val.logical_type.tid == LogicalTypeID::Int32
    }
}

impl Decode for i64 {
    fn decode(kuzu_val: &KuzuVal) -> Self {
        assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_int64(kuzu_val.val) }
    }

    fn check_type(kuzu_val: &KuzuVal) -> bool {
        kuzu_val.logical_type.tid == LogicalTypeID::Int64
    }
}

impl Decode for f32 {
    fn decode(kuzu_val: &KuzuVal) -> Self {
        assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_float(kuzu_val.val) }
    }

    fn check_type(kuzu_val: &KuzuVal) -> bool {
        kuzu_val.logical_type.tid == LogicalTypeID::Float
    }
}

impl Decode for f64 {
    fn decode(kuzu_val: &KuzuVal) -> Self {
        assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_double(kuzu_val.val) }
    }

    fn check_type(kuzu_val: &KuzuVal) -> bool {
        kuzu_val.logical_type.tid == LogicalTypeID::Double
    }
}

mod ffi {
    use crate::types::value::ffi::kuzu_value;

    extern "C" {
        pub fn kuzu_value_get_bool(value: *mut kuzu_value) -> bool;
        pub fn kuzu_value_get_int16(value: *mut kuzu_value) -> i16;
        pub fn kuzu_value_get_int32(value: *mut kuzu_value) -> i32;
        pub fn kuzu_value_get_int64(value: *mut kuzu_value) -> i64;
        pub fn kuzu_value_get_float(value: *mut kuzu_value) -> f32;
        pub fn kuzu_value_get_double(value: *mut kuzu_value) -> f64;
        // pub fn kuzu_value_get_internal_id(value: *mut kuzu_value) -> kuzu_internal_id_t;
        // pub fn kuzu_value_get_node_val(value: *mut kuzu_value) -> *mut kuzu_node_val;
        // pub fn kuzu_value_get_rel_val(value: *mut kuzu_value) -> *mut kuzu_rel_val;
        // pub fn kuzu_value_get_date(value: *mut kuzu_value) -> kuzu_date_t;
        // pub fn kuzu_value_get_timestamp(value: *mut kuzu_value) -> kuzu_timestamp_t;
        // pub fn kuzu_value_get_interval(value: *mut kuzu_value) -> kuzu_interval_t;
        // pub fn kuzu_value_get_string(value: *mut kuzu_value) -> *mut ::std::os::raw::c_char;
    }
}
