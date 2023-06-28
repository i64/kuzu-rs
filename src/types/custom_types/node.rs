use crate::helper::PtrContainer;
use crate::types::value::KuzuValue;
use std::collections::HashMap;

use crate::convert_inner_to_owned_string;

use crate::ffi;
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
    properties: HashMap<String, KuzuValue>,
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
