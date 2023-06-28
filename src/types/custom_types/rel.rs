use std::collections::HashMap;

use crate::{convert_inner_to_owned_string, helper::PtrContainer, types::value::KuzuValue};

use super::node::InternalId;

use crate::ffi;

#[derive(Debug)]
pub struct Relation {
    label: String,
    src: InternalId,
    dst: InternalId,
    properties: HashMap<String, KuzuValue>,
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

                    (key, KuzuValue::from(PtrContainer(val)))
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
