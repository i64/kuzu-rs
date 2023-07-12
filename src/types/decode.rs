use crate::error;

use super::value::{FixedList, KuzuValue, Node, Relation, Struct, VarList};

pub trait Decode: Sized {
    fn decode_kuzuval(value: KuzuValue) -> error::Result<Self>;
}

/// Implements the `Decode` trait for decoding a `KuzuValue` into a specific type.
macro_rules! impl_decode {
    ($ty:ty, $inner:ident) => {
        impl Decode for $ty {
            /// Tries to convert a `KuzuValue` into the specified type.
            /// If the conversion is successful, it returns the inner value.
            /// Otherwise, it returns a `DecodeError` with the type name of the value.
            fn decode_kuzuval(value: KuzuValue) -> error::Result<Self> {
                match value {
                    KuzuValue::$inner(inner) => Ok(inner),
                    ty => Err(error::Error::DecodeError(
                        ty.name(),
                        std::any::type_name::<Self>(),
                    )),
                }
            }
        }
    };
}

impl_decode!(bool, Bool);
impl_decode!(i16, Int16);
impl_decode!(i32, Int32);
impl_decode!(i64, Int64);
impl_decode!(f32, Float);
impl_decode!(f64, Double);
impl_decode!(String, String);
impl_decode!(Node, Node);
impl_decode!(Relation, Rel);
impl_decode!(VarList, VarList);
impl_decode!(FixedList, FixedList);
impl_decode!(Struct, Struct);

impl<T> Decode for Vec<T>
where
    T: Decode,
{
    fn decode_kuzuval(value: KuzuValue) -> error::Result<Self> {
        match value {
            KuzuValue::VarList(inner) => inner.try_into(),
            KuzuValue::FixedList(inner) => inner.try_into(),
            ty => Err(error::Error::DecodeError(
                ty.name(),
                std::any::type_name::<Self>(),
            )),
        }
    }
}

impl Decode for KuzuValue {
    fn decode_kuzuval(value: KuzuValue) -> error::Result<Self> {
        Ok(value)
    }
}

impl<T> TryFrom<FixedList> for Vec<T>
where
    T: Decode,
{
    type Error = error::Error;

    fn try_from(value: FixedList) -> Result<Self, Self::Error> {
        value.inner.into_iter().map(T::decode_kuzuval).collect()
    }
}

impl<T> TryFrom<VarList> for Vec<T>
where
    T: Decode,
{
    type Error = error::Error;

    fn try_from(value: VarList) -> Result<Self, Self::Error> {
        value.inner.into_iter().map(T::decode_kuzuval).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::{KuzuValue, Node, Relation};
    use crate::error;
    use crate::types::value::tests::{
        new_fixedlist, new_internal_id, new_node, new_rel, new_varlist,
    };
    use crate::types::value::{FixedList, VarList};
    use std::fmt::Debug;

    fn test_type<ST, A, B, C, D, E, F, G, H, I, J>(wrapped_val: KuzuValue, result: ST)
    where
        ST: Decode + Debug + PartialEq,
        A: Decode,
        B: Decode,
        C: Decode,
        D: Decode,
        E: Decode,
        F: Decode,
        G: Decode,
        H: Decode,
        I: Decode,
        J: Decode,
    {
        {
            let res = ST::decode_kuzuval(wrapped_val.clone());
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), result);
        }

        assert!(A::decode_kuzuval(wrapped_val.clone()).is_err());
        assert!(B::decode_kuzuval(wrapped_val.clone()).is_err());
        assert!(C::decode_kuzuval(wrapped_val.clone()).is_err());
        assert!(D::decode_kuzuval(wrapped_val.clone()).is_err());
        assert!(E::decode_kuzuval(wrapped_val.clone()).is_err());
        assert!(F::decode_kuzuval(wrapped_val.clone()).is_err());
        assert!(G::decode_kuzuval(wrapped_val.clone()).is_err());
        assert!(H::decode_kuzuval(wrapped_val.clone()).is_err());
        assert!(I::decode_kuzuval(wrapped_val.clone()).is_err());
        assert!(J::decode_kuzuval(wrapped_val.clone()).is_err());
    }

    #[test]
    fn test_decode() {
        test_type::<bool, i16, i32, i64, f32, f64, String, Node, Relation, VarList, FixedList>(
            KuzuValue::Bool(true),
            true,
        );
        test_type::<i16, bool, i32, i64, f32, f64, String, Node, Relation, VarList, FixedList>(
            KuzuValue::Int16(16),
            16,
        );
        test_type::<i32, bool, i16, i64, f32, f64, String, Node, Relation, VarList, FixedList>(
            KuzuValue::Int32(32),
            32,
        );
        test_type::<i64, bool, i16, i32, f32, f64, String, Node, Relation, VarList, FixedList>(
            KuzuValue::Int64(64),
            64,
        );
        test_type::<f32, bool, i16, i32, i64, f64, String, Node, Relation, VarList, FixedList>(
            KuzuValue::Float(0.32),
            0.32,
        );
        test_type::<f64, bool, i16, i32, i64, f32, String, Node, Relation, VarList, FixedList>(
            KuzuValue::Double(0.64),
            0.64,
        );
        test_type::<String, bool, i16, i32, i64, f32, f64, Node, Relation, VarList, FixedList>(
            KuzuValue::String("string".to_owned()),
            "string".to_owned(),
        );

        let node = new_node(0, 0);
        test_type::<Node, bool, i16, i32, i64, f32, f64, String, Relation, VarList, FixedList>(
            KuzuValue::Node(node.clone()),
            node.clone(),
        );

        let rel = new_rel();
        test_type::<Relation, bool, i16, i32, i64, f32, f64, String, Node, VarList, FixedList>(
            KuzuValue::Rel(rel.clone()),
            rel.clone(),
        );

        let fixedlist = new_fixedlist();
        test_type::<FixedList, bool, i16, i32, i64, f32, f64, String, Node, Relation, VarList>(
            KuzuValue::FixedList(fixedlist.clone()),
            fixedlist.clone(),
        );

        let varlist = new_varlist();
        test_type::<VarList, bool, i16, i32, i64, f32, f64, String, Node, Relation, FixedList>(
            KuzuValue::VarList(varlist.clone()),
            varlist.clone(),
        );
    }
}
