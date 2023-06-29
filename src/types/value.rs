use std::collections::HashMap;

use crate::{convert_inner_to_owned_string, helper::PtrContainer, into_cstr};

use super::logical_type::{LogicaType, LogicalTypeID};

use crate::ffi;

#[derive(Debug, Clone)]
pub enum KuzuValue {
    Node(Node),
    Rel(Relation),
    Bool(bool),
    Int64(i64),
    Int32(i32),
    Int16(i16),
    Double(f64),
    Float(f32),
    String(String),
    InternalId(InternalId),
    FixedList(FixedList),
    VarList(VarList),
    // Date,
    // Timestamp,
    // Interval,
    // Struct,
}

impl From<PtrContainer<ffi::kuzu_value>> for KuzuValue {
    fn from(value: PtrContainer<ffi::kuzu_value>) -> Self {
        let logical_type = LogicaType::from(&value);

        match logical_type.tid {
            LogicalTypeID::Bool => KuzuValue::Bool(unsafe { ffi::kuzu_value_get_bool(value.0) }),
            LogicalTypeID::Int16 => KuzuValue::Int16(unsafe { ffi::kuzu_value_get_int16(value.0) }),
            LogicalTypeID::Int32 => KuzuValue::Int32(unsafe { ffi::kuzu_value_get_int32(value.0) }),
            LogicalTypeID::Int64 => KuzuValue::Int64(unsafe { ffi::kuzu_value_get_int64(value.0) }),
            LogicalTypeID::Float => KuzuValue::Float(unsafe { ffi::kuzu_value_get_float(value.0) }),
            LogicalTypeID::Double => {
                KuzuValue::Double(unsafe { ffi::kuzu_value_get_double(value.0) })
            }
            LogicalTypeID::String => {
                let str_ptr = unsafe { ffi::kuzu_value_get_string(value.0) };
                KuzuValue::String(convert_inner_to_owned_string!(str_ptr))
            }
            LogicalTypeID::Node => {
                let rel_val: PtrContainer<ffi::kuzu_node_val> =
                    PtrContainer(unsafe { ffi::kuzu_value_get_node_val(value.0) });
                KuzuValue::Node(rel_val.into())
            }
            LogicalTypeID::Rel => {
                let rel_val = PtrContainer(unsafe { ffi::kuzu_value_get_rel_val(value.0) });
                KuzuValue::Rel(rel_val.into())
            }
            LogicalTypeID::InternalId => {
                let internal_id = unsafe { ffi::kuzu_value_get_internal_id(value.0) }.into();
                KuzuValue::InternalId(internal_id)
            }
            LogicalTypeID::FixedList => {
                let elems = (0..(logical_type.fixed_num_elements_in_list))
                    .map(|idx| {
                        PtrContainer(unsafe { ffi::kuzu_value_get_list_element(value.0, idx) })
                            .into()
                    })
                    .collect();

                KuzuValue::FixedList(FixedList { inner: elems })
            }
            LogicalTypeID::VarList => {
                let list_size = unsafe { ffi::kuzu_value_get_list_size(value.0) };
                let elems: Vec<KuzuValue> = (0..list_size)
                    .map(|idx| {
                        PtrContainer(unsafe { ffi::kuzu_value_get_list_element(value.0, idx) })
                            .into()
                    })
                    .collect();

                KuzuValue::VarList(VarList { inner: elems })
            }
            // LogicalTypeID::Date => todo!(),
            // LogicalTypeID::Timestamp => todo!(),
            // LogicalTypeID::Interval => todo!(),
            // LogicalTypeID::Any => todo!(),
            // LogicalTypeID::Struct => todo!(),
            ty => todo!("{:?}", ty),
        }
    }
}

impl From<&KuzuValue> for PtrContainer<ffi::kuzu_value> {
    fn from(value: &KuzuValue) -> Self {
        let res = unsafe {
            match value {
                KuzuValue::Node(_inner) => todo!(),
                KuzuValue::Rel(_inner) => todo!(),
                KuzuValue::Bool(inner) => ffi::kuzu_value_create_bool(*inner),
                KuzuValue::Int64(inner) => ffi::kuzu_value_create_int64(*inner),
                KuzuValue::Int32(inner) => ffi::kuzu_value_create_int32(*inner),
                KuzuValue::Int16(inner) => ffi::kuzu_value_create_int16(*inner),
                KuzuValue::Double(inner) => ffi::kuzu_value_create_double(*inner),
                KuzuValue::Float(inner) => ffi::kuzu_value_create_float(*inner),
                KuzuValue::String(inner) => {
                    ffi::kuzu_value_create_string(into_cstr!(inner.as_str()))
                }
                _ => todo!(),
            }
        };
        PtrContainer(res)
    }
}

#[derive(Debug, Clone, Copy)]
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

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct FixedList {
    inner: Vec<KuzuValue>,
}

#[derive(Debug, Clone)]
pub struct VarList {
    inner: Vec<KuzuValue>,
}
