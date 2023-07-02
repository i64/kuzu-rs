use std::collections::HashMap;

use crate::{error, helper::convert_inner_to_owned_string, into_cstr, ptrc::PtrContainer};

use super::logical_type::{LogicaType, LogicalTypeID};

use crate::ffi;

/// Represents various types of values that can be stored in Kuzu.
#[derive(Debug, Clone, PartialEq)]
pub enum KuzuValue {
    /// Node value.
    Node(Node),
    /// Relation value.
    Rel(Relation),
    /// Boolean value.
    Bool(bool),
    /// 64-bit integer value.
    Int64(i64),
    /// 32-bit integer value.
    Int32(i32),
    /// 16-bit integer value.
    Int16(i16),
    /// Double precision floating-point value.
    Double(f64),
    /// Single precision floating-point value.
    Float(f32),
    /// String value.
    String(String),
    /// Internal ID value.
    InternalId(InternalId),
    /// Fixed-length list value.
    FixedList(FixedList),
    /// Variable-length list value.
    VarList(VarList),
    // Date,
    // Timestamp,
    // Interval,
    // Struct,
}

impl KuzuValue {
    /// Returns the name of the `KuzuValue` variant.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Node(_) => "Self::Node",
            Self::Rel(_) => "Self::Rel",
            Self::Bool(_) => "Self::Bool",
            Self::Int64(_) => "Self::Int64",
            Self::Int32(_) => "Self::Int32",
            Self::Int16(_) => "Self::Int16",
            Self::Double(_) => "Self::Double",
            Self::Float(_) => "Self::Float",
            Self::String(_) => "Self::String",
            Self::InternalId(_) => "Self::InternalId",
            Self::FixedList(_) => "Self::FixedList",
            Self::VarList(_) => "Self::VarList",
        }
    }
}
impl TryFrom<PtrContainer<ffi::kuzu_value>> for KuzuValue {
    type Error = error::Error;

    fn try_from(value: PtrContainer<ffi::kuzu_value>) -> Result<Self, Self::Error> {
        let logical_type = LogicaType::try_from(&value)?;

        let res = match logical_type.tid {
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
                KuzuValue::String(convert_inner_to_owned_string(str_ptr)?)
            }
            LogicalTypeID::Node => {
                let rel_val: PtrContainer<ffi::kuzu_node_val> =
                    PtrContainer::try_new(unsafe { ffi::kuzu_value_get_node_val(value.0) })?;
                KuzuValue::Node(rel_val.try_into()?)
            }
            LogicalTypeID::Rel => {
                let rel_val =
                    PtrContainer::try_new(unsafe { ffi::kuzu_value_get_rel_val(value.0) })?;
                KuzuValue::Rel(rel_val.try_into()?)
            }
            LogicalTypeID::InternalId => {
                let internal_id = unsafe { ffi::kuzu_value_get_internal_id(value.0) }.into();
                KuzuValue::InternalId(internal_id)
            }
            LogicalTypeID::FixedList => {
                let elems = (0..(logical_type.fixed_num_elements_in_list))
                    .map(|idx| {
                        PtrContainer::try_new(unsafe {
                            ffi::kuzu_value_get_list_element(value.0, idx)
                        })
                        .and_then(|ptr| ptr.try_into())
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                KuzuValue::FixedList(FixedList::try_new(elems)?)
            }
            LogicalTypeID::VarList => {
                let list_size = unsafe { ffi::kuzu_value_get_list_size(value.0) };
                let elems: Vec<KuzuValue> = (0..list_size)
                    .map(|idx| {
                        PtrContainer::try_new(unsafe {
                            ffi::kuzu_value_get_list_element(value.0, idx)
                        })
                        .and_then(|ptr| ptr.try_into())
                    })
                    .collect::<Result<_, _>>()?;

                KuzuValue::VarList(VarList::try_new(elems)?)
            }
            // LogicalTypeID::Date => todo!(),
            // LogicalTypeID::Timestamp => todo!(),
            // LogicalTypeID::Interval => todo!(),
            // LogicalTypeID::Any => todo!(),
            // LogicalTypeID::Struct => todo!(),
            ty => todo!("{:?}", ty),
        };

        Ok(res)
    }
}

impl TryFrom<&KuzuValue> for PtrContainer<ffi::kuzu_value> {
    type Error = error::Error;
    fn try_from(value: &KuzuValue) -> Result<Self, Self::Error> {
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
                    ffi::kuzu_value_create_string(into_cstr!(inner.as_str())?.as_ptr())
                }
                _ => todo!(),
            }
        };
        PtrContainer::try_new(res)
    }
}

/// Represents an internal ID in Kuzu.
#[derive(Debug, Clone, PartialEq)]
pub struct InternalId {
    /// Offset value.
    pub offset: usize,
    /// Table ID value.
    pub table_id: usize,
}

impl From<ffi::kuzu_internal_id_t> for InternalId {
    fn from(value: ffi::kuzu_internal_id_t) -> Self {
        Self {
            offset: value.offset as usize,
            table_id: value.table_id as usize,
        }
    }
}

/// Represents a node in Kuzu.
#[derive(Debug, Clone)]
pub struct Node {
    /// The ID of the node.
    pub id: InternalId,
    /// The label of the node.
    pub label: String,
    /// The properties of the node.
    pub properties: HashMap<String, KuzuValue>,
}

impl TryFrom<PtrContainer<ffi::kuzu_node_val>> for Node {
    type Error = error::Error;
    fn try_from(value: PtrContainer<ffi::kuzu_node_val>) -> Result<Self, Self::Error> {
        let id = unsafe { ffi::kuzu_node_val_get_id(value.0) }.into();

        let label = {
            let inner = unsafe { ffi::kuzu_node_val_get_label_name(value.0) };
            convert_inner_to_owned_string(inner)?
        };

        let properties = {
            let property_size = unsafe { ffi::kuzu_node_val_get_property_size(value.0) };
            (0..property_size)
                .map(|idx| {
                    let _key = {
                        let inner =
                            unsafe { ffi::kuzu_node_val_get_property_name_at(value.0, idx) };
                        convert_inner_to_owned_string(inner)
                    };

                    let key = match _key {
                        Ok(key) => key,
                        Err(e) => return Err(e),
                    };

                    let _val = {
                        let val = unsafe { ffi::kuzu_node_val_get_property_value_at(value.0, idx) };
                        KuzuValue::try_from(PtrContainer::try_new(val)?)
                    };

                    let val = match _val {
                        Ok(val) => val,
                        Err(e) => return Err(e),
                    };

                    Ok((key, val))
                })
                .collect::<Result<_, _>>()?
        };

        Ok(Self {
            id,
            label,
            properties,
        })
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

/// Represents a relation in Kuzu.
#[derive(Debug, Clone, PartialEq)]
pub struct Relation {
    /// The label of the relation.
    label: String,
    /// The source node ID of the relation.
    src: InternalId,
    /// The destination node ID of the relation.
    dst: InternalId,
    /// The properties of the relation.
    properties: HashMap<String, KuzuValue>,
}

impl TryFrom<PtrContainer<ffi::kuzu_rel_val>> for Relation {
    type Error = error::Error;

    fn try_from(value: PtrContainer<ffi::kuzu_rel_val>) -> Result<Self, Self::Error> {
        let label = {
            let inner = unsafe { ffi::kuzu_rel_val_get_label_name(value.0) };
            convert_inner_to_owned_string(inner)?
        };
        let src = unsafe { ffi::kuzu_rel_val_get_src_id(value.0) }.into();
        let dst = unsafe { ffi::kuzu_rel_val_get_dst_id(value.0) }.into();

        let properties = {
            let property_size = unsafe { ffi::kuzu_rel_val_get_property_size(value.0) };
            (0..property_size)
                .map(|idx| {
                    let _key = {
                        let inner = unsafe { ffi::kuzu_rel_val_get_property_name_at(value.0, idx) };
                        convert_inner_to_owned_string(inner)
                    };

                    let key = match _key {
                        Ok(key) => key,
                        Err(e) => return Err(e),
                    };

                    let _val = {
                        let val = unsafe { ffi::kuzu_rel_val_get_property_value_at(value.0, idx) };
                        KuzuValue::try_from(PtrContainer::try_new(val)?)
                    };

                    let val = match _val {
                        Ok(val) => val,
                        Err(e) => return Err(e),
                    };

                    Ok((key, val))
                })
                .collect::<Result<_, _>>()?
        };

        Ok(Self {
            label,
            src,
            dst,
            properties,
        })
    }
}

/// Represents a fixed list of values in Kuzu.
#[derive(Debug, Clone, PartialEq)]
pub struct FixedList {
    /// The inner vector of Kuzu values.
    pub inner: Vec<KuzuValue>,

    /// The length of the inner list
    pub len: usize,
}

impl FixedList {
    #[inline]
    fn try_new(inner: Vec<KuzuValue>) -> error::Result<Self> {
        if let Some(first_elem) = inner.get(0) {
            let first_elems_type = std::mem::discriminant(first_elem);
            let is_all_same = inner
                .iter()
                .all(|v| std::mem::discriminant(v) == first_elems_type);

            if !is_all_same {
                return Err(error::Error::ListTypeError);
            }
        }

        let len = inner.len();

        Ok(Self { inner, len })
    }
}

/// Represents a variable-length list of values in Kuzu.
#[derive(Debug, Clone, PartialEq)]
pub struct VarList {
    /// The inner vector of Kuzu values.
    pub inner: Vec<KuzuValue>,
}

impl VarList {
    #[inline]
    fn try_new(inner: Vec<KuzuValue>) -> error::Result<Self> {
        if let Some(first_elem) = inner.get(0) {
            let first_elems_type = std::mem::discriminant(first_elem);
            let is_all_same = inner
                .iter()
                .all(|v| std::mem::discriminant(v) == first_elems_type);

            if !is_all_same {
                return Err(error::Error::ListTypeError);
            }
        }

        Ok(Self { inner })
    }
}
#[cfg(test)]
pub mod tests {
    use super::{FixedList, InternalId, Node, Relation, VarList};

    pub fn new_internal_id(offset: usize, table_id: usize) -> InternalId {
        InternalId { offset, table_id }
    }

    pub fn new_node(offset: usize, table_id: usize) -> Node {
        Node {
            id: new_internal_id(offset, table_id),
            label: Default::default(),
            properties: Default::default(),
        }
    }

    pub fn new_rel() -> Relation {
        let src = new_internal_id(0, 0);
        let dst = new_internal_id(1, 1);
        Relation {
            src,
            dst,
            label: Default::default(),
            properties: Default::default(),
        }
    }

    pub fn new_varlist() -> VarList {
        VarList {
            inner: Default::default(),
        }
    }

    pub fn new_fixedlist() -> FixedList {
        FixedList {
            inner: Default::default(),
            len: 0,
        }
    }
}
