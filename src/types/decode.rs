use crate::{convert_inner_to_owned_string, helper::PtrContainer};

use super::{logical_type::LogicalTypeID, value::KuzuVal, custom_types::{rel::Relation, node::Node}};


impl From<KuzuVal> for bool {
    fn from(value: KuzuVal) -> Self {
        // assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_bool(value.val.0) }
    }

    // fn check_type(kuzu_val: &KuzuVal) -> bool {
    //     kuzu_val.logical_type.tid == LogicalTypeID::Bool
    // }
}

impl From<KuzuVal> for i16 {
    fn from(value: KuzuVal) -> Self {
        // assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_int16(value.val.0) }
    }

    // fn check_type(kuzu_val: &KuzuVal) -> bool {
    //     kuzu_val.logical_type.tid == LogicalTypeID::Int16
    // }
}

impl From<KuzuVal> for i32 {
    fn from(value: KuzuVal) -> Self {
        // assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_int32(value.val.0) }
    }

    // fn check_type(kuzu_val: &KuzuVal) -> bool {
    //     kuzu_val.logical_type.tid == LogicalTypeID::Int32
    // }
}

impl From<KuzuVal> for i64 {
    fn from(value: KuzuVal) -> Self {
        // assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_int64(value.val.0) }
    }

    // fn check_type(kuzu_val: &KuzuVal) -> bool {
    //     kuzu_val.logical_type.tid == LogicalTypeID::Int64
    // }
}

impl From<KuzuVal> for f32 {
    fn from(value: KuzuVal) -> Self {
        // assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_float(value.val.0) }
    }

    // fn check_type(kuzu_val: &KuzuVal) -> bool {
    //     kuzu_val.logical_type.tid == LogicalTypeID::Float
    // }
}

impl From<KuzuVal> for f64 {
    fn from(value: KuzuVal) -> Self {
        // assert!(Self::check_type(kuzu_val));
        unsafe { ffi::kuzu_value_get_double(value.val.0) }
    }

    // fn check_type(kuzu_val: &KuzuVal) -> bool {
    //     kuzu_val.logical_type.tid == LogicalTypeID::Double
    // }
}

impl From<KuzuVal> for String {
    fn from(value: KuzuVal) -> Self {
        let str_ptr = unsafe { ffi::kuzu_value_get_string(value.val.0) };
        convert_inner_to_owned_string!(str_ptr)
    }
}

impl From<KuzuVal> for Node {
    fn from(value: KuzuVal) -> Self {
        let rel_val = unsafe { ffi::kuzu_value_get_node_val(value.val.0) };
        PtrContainer(rel_val).into()
    }
}

impl From<KuzuVal> for Relation {
    fn from(value: KuzuVal) -> Self {
        let rel_val = unsafe { ffi::kuzu_value_get_rel_val(value.val.0) };
        PtrContainer(rel_val).into()
    }
}
mod ffi {
    use crate::types::{value::ffi::kuzu_value, custom_types::{rel::ffi::kuzu_rel_val, node::ffi::kuzu_node_val}};

    extern "C" {
        pub fn kuzu_value_get_bool(value: *mut kuzu_value) -> bool;
        pub fn kuzu_value_get_int16(value: *mut kuzu_value) -> i16;
        pub fn kuzu_value_get_int32(value: *mut kuzu_value) -> i32;
        pub fn kuzu_value_get_int64(value: *mut kuzu_value) -> i64;
        pub fn kuzu_value_get_float(value: *mut kuzu_value) -> f32;
        pub fn kuzu_value_get_double(value: *mut kuzu_value) -> f64;
        // pub fn kuzu_value_get_internal_id(value: *mut kuzu_value) -> kuzu_internal_id_t;
        // pub fn kuzu_value_get_date(value: *mut kuzu_value) -> kuzu_date_t;
        // pub fn kuzu_value_get_timestamp(value: *mut kuzu_value) -> kuzu_timestamp_t;
        // pub fn kuzu_value_get_interval(value: *mut kuzu_value) -> kuzu_interval_t;
        pub fn kuzu_value_get_string(value: *mut kuzu_value) -> *mut ::std::os::raw::c_char;
        pub fn kuzu_value_get_rel_val(value: *mut kuzu_value) -> *mut kuzu_rel_val;
        pub fn kuzu_value_get_node_val(value: *mut kuzu_value) -> *mut kuzu_node_val;

    }
}
