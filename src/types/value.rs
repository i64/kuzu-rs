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
    /// Custom Struct
    Struct(Struct),
    // Date,
    // Timestamp,
    // Interval,
}

impl KuzuValue {
    /// Returns the name of the `KuzuValue` variant.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Node(_) => "KuzuValue::Node",
            Self::Rel(_) => "KuzuValue::Rel",
            Self::Bool(_) => "KuzuValue::Bool",
            Self::Int64(_) => "KuzuValue::Int64",
            Self::Int32(_) => "KuzuValue::Int32",
            Self::Int16(_) => "KuzuValue::Int16",
            Self::Double(_) => "KuzuValue::Double",
            Self::Float(_) => "KuzuValue::Float",
            Self::String(_) => "KuzuValue::String",
            Self::InternalId(_) => "KuzuValue::InternalId",
            Self::FixedList(_) => "KuzuValue::FixedList",
            Self::Struct(_) => "KuzuValue::Struct",
            Self::VarList(_) => "KuzuValue::VarList",
        }
    }
}
impl TryFrom<PtrContainer<ffi::kuzu_value>> for KuzuValue {
    type Error = error::Error;

    fn try_from(value: PtrContainer<ffi::kuzu_value>) -> Result<Self, Self::Error> {
        let logical_type = LogicaType::try_from(&value)?;
        let inner_ptr = value.0;

        let res = match logical_type.tid {
            LogicalTypeID::Bool => Self::Bool(unsafe { ffi::kuzu_value_get_bool(inner_ptr) }),
            LogicalTypeID::Int16 => Self::Int16(unsafe { ffi::kuzu_value_get_int16(inner_ptr) }),
            LogicalTypeID::Int32 => Self::Int32(unsafe { ffi::kuzu_value_get_int32(inner_ptr) }),
            LogicalTypeID::Int64 => Self::Int64(unsafe { ffi::kuzu_value_get_int64(inner_ptr) }),
            LogicalTypeID::Float => Self::Float(unsafe { ffi::kuzu_value_get_float(inner_ptr) }),
            LogicalTypeID::Double => Self::Double(unsafe { ffi::kuzu_value_get_double(inner_ptr) }),
            LogicalTypeID::String => {
                let str_ptr = unsafe { ffi::kuzu_value_get_string(inner_ptr) };
                Self::String(convert_inner_to_owned_string(str_ptr)?)
            }
            LogicalTypeID::Node => {
                let rel_val: PtrContainer<ffi::kuzu_node_val> =
                    PtrContainer::try_new(unsafe { ffi::kuzu_value_get_node_val(inner_ptr) })?;
                Self::Node(rel_val.try_into()?)
            }
            LogicalTypeID::Rel => {
                let rel_val =
                    PtrContainer::try_new(unsafe { ffi::kuzu_value_get_rel_val(inner_ptr) })?;
                Self::Rel(rel_val.try_into()?)
            }
            LogicalTypeID::InternalId => {
                let internal_id = unsafe { ffi::kuzu_value_get_internal_id(inner_ptr) }.into();
                Self::InternalId(internal_id)
            }
            LogicalTypeID::FixedList => {
                let elems = (0..(logical_type.fixed_num_elements_in_list))
                    .map(|idx| {
                        PtrContainer::try_new(unsafe {
                            ffi::kuzu_value_get_list_element(inner_ptr, idx)
                        })
                        .and_then(|ptr| ptr.try_into())
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                Self::FixedList(FixedList::try_new(elems)?)
            }
            LogicalTypeID::VarList => Self::VarList(VarList::try_from(value)?),
            LogicalTypeID::Struct => Self::Struct(value.try_into()?),
            // LogicalTypeID::Date => todo!(),
            // LogicalTypeID::Timestamp => todo!(),
            // LogicalTypeID::Interval => todo!(),
            // LogicalTypeID::Any => todo!(),
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
        let inner_ptr = value.0;
        let id = unsafe { ffi::kuzu_node_val_get_id(inner_ptr) }.into();

        let label = {
            let inner = unsafe { ffi::kuzu_node_val_get_label_name(inner_ptr) };
            convert_inner_to_owned_string(inner)?
        };

        let properties = {
            let property_size = unsafe { ffi::kuzu_node_val_get_property_size(inner_ptr) };
            (0..property_size)
                .map(|idx| {
                    let _key = {
                        let inner =
                            unsafe { ffi::kuzu_node_val_get_property_name_at(inner_ptr, idx) };
                        convert_inner_to_owned_string(inner)
                    };

                    let key = match _key {
                        Ok(key) => key,
                        Err(e) => return Err(e),
                    };

                    let _val = {
                        let val =
                            unsafe { ffi::kuzu_node_val_get_property_value_at(inner_ptr, idx) };
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
        let inner_ptr = value.0;
        let label = {
            let inner = unsafe { ffi::kuzu_rel_val_get_label_name(inner_ptr) };
            convert_inner_to_owned_string(inner)?
        };
        let src = unsafe { ffi::kuzu_rel_val_get_src_id(inner_ptr) }.into();
        let dst = unsafe { ffi::kuzu_rel_val_get_dst_id(inner_ptr) }.into();

        let properties = {
            let property_size = unsafe { ffi::kuzu_rel_val_get_property_size(inner_ptr) };
            (0..property_size)
                .map(|idx| {
                    let _key = {
                        let inner =
                            unsafe { ffi::kuzu_rel_val_get_property_name_at(inner_ptr, idx) };
                        convert_inner_to_owned_string(inner)
                    };

                    let key = match _key {
                        Ok(key) => key,
                        Err(e) => return Err(e),
                    };

                    let _val = {
                        let val =
                            unsafe { ffi::kuzu_rel_val_get_property_value_at(inner_ptr, idx) };
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
impl TryFrom<PtrContainer<ffi::kuzu_value>> for VarList {
    type Error = error::Error;
    fn try_from(value: PtrContainer<ffi::kuzu_value>) -> Result<Self, Self::Error> {
        let inner_ptr = value.0;
        let list_size = unsafe { ffi::kuzu_value_get_list_size(inner_ptr) };
        let elems: Vec<KuzuValue> = (0..list_size)
            .map(|idx| {
                PtrContainer::try_new(unsafe { ffi::kuzu_value_get_list_element(inner_ptr, idx) })
                    .and_then(|ptr| ptr.try_into())
            })
            .collect::<Result<_, _>>()?;

        Self::try_new(elems)
    }
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

/// Represents the struct datatype.
#[derive(Debug, Clone, PartialEq)]
pub struct Struct {
    /// The inner hashmap of Kuzu values.
    inner: HashMap<String, KuzuValue>,
}

impl Struct {
    /// Retrieves the KuzuValue associated with the specified key, if it exists.
    pub fn get<S: AsRef<str>>(&self, key: S) -> Option<&KuzuValue> {
        let upper_key = key.as_ref().to_uppercase();
        self.inner.get(&upper_key)
    }
}

impl TryFrom<PtrContainer<ffi::kuzu_value>> for Struct {
    type Error = error::Error;
    fn try_from(value: PtrContainer<ffi::kuzu_value>) -> Result<Self, Self::Error> {
        let inner_hashmap = {
            let inner_ptr = value.0;
            let field_count = unsafe { ffi::kuzu_value_get_struct_num_fields(inner_ptr) };
            (0..field_count)
                .map(|idx| {
                    let _key = {
                        let inner =
                            unsafe { ffi::kuzu_value_get_struct_field_name(inner_ptr, idx) };
                        convert_inner_to_owned_string(inner)
                    };

                    let key = match _key {
                        Ok(key) => key,
                        Err(e) => return Err(e),
                    };

                    let _val = {
                        let val = unsafe { ffi::kuzu_value_get_struct_field_value(inner_ptr, idx) };
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
            inner: inner_hashmap,
        })
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
