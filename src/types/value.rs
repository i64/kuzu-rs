use crate::{convert_inner_to_owned_string, helper::PtrContainer, into_cstr};

use super::{
    custom_types::{
        node::{InternalId, Node},
        rel::Relation,
    },
    logical_type::{LogicaType, LogicalTypeID},
};

use crate::ffi;

#[derive(Debug)]
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
    // Date,
    // Timestamp,
    // Interval,
    // FixedList,
    // VarList,
    // Struct,
}

impl From<PtrContainer<ffi::kuzu_value>> for KuzuValue {
    fn from(value: PtrContainer<ffi::kuzu_value>) -> Self {
        let logical_type = LogicaType::from(&value);

        match logical_type.tid {
            LogicalTypeID::Node => {
                let rel_val = PtrContainer(unsafe { ffi::kuzu_value_get_node_val(value.0) });
                KuzuValue::Node(rel_val.into())
            }
            LogicalTypeID::Rel => {
                let rel_val = PtrContainer(unsafe { ffi::kuzu_value_get_rel_val(value.0) });
                KuzuValue::Rel(rel_val.into())
            }
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
            LogicalTypeID::InternalId => {
                let internal_id = unsafe { ffi::kuzu_value_get_internal_id(value.0) }.into();
                KuzuValue::InternalId(internal_id)
            }
            // LogicalTypeID::Date => todo!(),
            // LogicalTypeID::Timestamp => todo!(),
            // LogicalTypeID::Interval => todo!(),
            // LogicalTypeID::FixedList => todo!(),
            // LogicalTypeID::VarList => todo!(),
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
