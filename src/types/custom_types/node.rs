use crate::helper::PtrContainer;
use std::{collections::HashMap};

use crate::convert_inner_to_owned_string;
use crate::types::value::KuzuVal;

#[derive(Debug)]
pub struct InternalId {
    offset: u64,
    table_id: u64,
}

impl From<ffi::kuzu_internal_id_t> for InternalId {
    fn from(value: ffi::kuzu_internal_id_t) -> Self {
        Self {
            offset: value.offset,
            table_id: value.table_id,
        }
    }
}

#[derive(Debug)]
pub struct Node {
    id: InternalId,
    label: String,
    properties: HashMap<String, KuzuVal>,
}

impl From<PtrContainer<ffi::kuzu_node_val>> for Node {
    fn from(value: PtrContainer<ffi::kuzu_node_val>) -> Self {
        let id = unsafe { ffi::kuzu_node_val_get_id(value.0) }.into();

        let label = {
            let inner = unsafe { ffi::kuzu_node_val_get_label_name(value.0) };
            convert_inner_to_owned_string!(inner)
        };

        let properties = {
            let property_size = unsafe { ffi::kuzu_node_val_get_property_size(value.0) };
            (0..property_size)
                .map(|idx| {
                    let key = {
                        let inner =
                            unsafe { ffi::kuzu_node_val_get_property_name_at(value.0, idx) };
                        convert_inner_to_owned_string!(inner)
                    };
                    let val = unsafe { ffi::kuzu_node_val_get_property_value_at(value.0, idx) };

                    (key, (PtrContainer(val)).into())
                })
                .collect()
        };

        Self {
            id,
            label,
            properties,
        }
    }
}

pub(crate) mod ffi {
    use crate::types::value::ffi::kuzu_value;

    #[repr(C)]
    pub struct kuzu_node_val {
        pub _node_val: *mut ::std::os::raw::c_void,
        pub _is_owned_by_cpp: bool,
    }

    #[repr(C)]
    pub struct kuzu_internal_id_t {
        pub table_id: u64,
        pub offset: u64,
    }

    extern "C" {
        pub fn kuzu_node_val_get_id(node_val: *mut kuzu_node_val) -> kuzu_internal_id_t;
        pub fn kuzu_node_val_get_label_name(
            node_val: *mut kuzu_node_val,
        ) -> *const ::std::os::raw::c_char;
        pub fn kuzu_node_val_get_property_size(node_val: *mut kuzu_node_val) -> u64;

        pub fn kuzu_node_val_get_property_name_at(
            node_val: *mut kuzu_node_val,
            index: u64,
        ) -> *mut ::std::os::raw::c_char;
        pub fn kuzu_node_val_get_property_value_at(
            node_val: *mut kuzu_node_val,
            index: u64,
        ) -> *mut kuzu_value;
    }
}
