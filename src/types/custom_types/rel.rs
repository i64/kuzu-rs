use std::collections::HashMap;

use crate::{convert_inner_to_owned_string, helper::PtrContainer, types::value::KuzuVal};

use super::node::InternalId;

struct Relation {
    label: String,
    src: InternalId,
    dst: InternalId,
    properties: HashMap<String, KuzuVal>,
}

impl From<PtrContainer<ffi::kuzu_rel_val>> for Relation {
    fn from(value: PtrContainer<ffi::kuzu_rel_val>) -> Self {
        let label = {
            let inner = unsafe { ffi::kuzu_rel_val_get_label_name(value.0) };
            convert_inner_to_owned_string!(inner)
        };
        let src = unsafe { ffi::kuzu_rel_val_get_src_id(value.0) }.into();
        let dst = unsafe { ffi::kuzu_rel_val_get_dst_id(value.0) }.into();

        let properties = {
            let property_size = unsafe { ffi::kuzu_rel_val_get_property_size(value.0) };
            (0..property_size)
                .map(|idx| {
                    let key = {
                        let inner = unsafe { ffi::kuzu_rel_val_get_property_name_at(value.0, idx) };
                        convert_inner_to_owned_string!(inner)
                    };
                    let val = unsafe { ffi::kuzu_rel_val_get_property_value_at(value.0, idx) };

                    (key, PtrContainer(val).into())
                })
                .collect()
        };

        Self {
            label,
            src,
            dst,
            properties,
        }
    }
}

mod ffi {
    use crate::types::{custom_types::node::ffi::kuzu_internal_id_t, value::ffi::kuzu_value};

    #[repr(C)]
    pub struct kuzu_rel_val {
        _rel_val: *mut ::std::os::raw::c_void,
        _is_owned_by_cpp: bool,
    }

    extern "C" {
        pub fn kuzu_rel_val_get_src_id(rel_val: *mut kuzu_rel_val) -> kuzu_internal_id_t;
        pub fn kuzu_rel_val_get_dst_id(rel_val: *mut kuzu_rel_val) -> kuzu_internal_id_t;
        pub fn kuzu_rel_val_get_label_name(
            rel_val: *mut kuzu_rel_val,
        ) -> *const ::std::os::raw::c_char;
        pub fn kuzu_rel_val_get_property_size(rel_val: *mut kuzu_rel_val) -> u64;
        pub fn kuzu_rel_val_get_property_name_at(
            rel_val: *mut kuzu_rel_val,
            index: u64,
        ) -> *mut ::std::os::raw::c_char;
        pub fn kuzu_rel_val_get_property_value_at(
            rel_val: *mut kuzu_rel_val,
            index: u64,
        ) -> *mut kuzu_value;

    }
}
