use crate::error;

use super::value::{FixedList, KuzuValue, Node, Relation, VarList};

/// Implements the `TryFrom` trait for decoding a `KuzuValue` into a specific type.
macro_rules! impl_decode {
    ($ty:ty, $inner:ident) => {
        impl TryFrom<KuzuValue> for $ty {
            type Error = error::Error;

            /// Tries to convert a `KuzuValue` into the specified type.
            /// If the conversion is successful, it returns the inner value.
            /// Otherwise, it returns a `DecodeError` with the type name of the value.
            fn try_from(value: KuzuValue) -> Result<Self, Self::Error> {
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

impl<T> TryFrom<VarList> for Vec<T>
where
    T: TryFrom<KuzuValue, Error = error::Error>,
{
    type Error = error::Error;

    fn try_from(value: VarList) -> Result<Self, Self::Error> {
        value.inner.into_iter().map(|el| el.try_into()).collect()
    }
}

impl<T> TryFrom<KuzuValue> for Vec<T>
where
    T: TryFrom<KuzuValue, Error = error::Error>,
{
    type Error = error::Error;
    fn try_from(value: KuzuValue) -> Result<Self, Self::Error> {
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

impl<T> TryFrom<FixedList> for Vec<T>
where
    T: TryFrom<KuzuValue, Error = error::Error>,
{
    type Error = error::Error;

    fn try_from(value: FixedList) -> Result<Self, Self::Error> {
        value.inner.into_iter().map(|el| el.try_into()).collect()
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
        ST: TryFrom<KuzuValue, Error = error::Error> + Debug + PartialEq,
        A: TryFrom<KuzuValue>,
        B: TryFrom<KuzuValue>,
        C: TryFrom<KuzuValue>,
        D: TryFrom<KuzuValue>,
        E: TryFrom<KuzuValue>,
        F: TryFrom<KuzuValue>,
        G: TryFrom<KuzuValue>,
        H: TryFrom<KuzuValue>,
        I: TryFrom<KuzuValue>,
        J: TryFrom<KuzuValue>,
    {
        {
            let res = ST::try_from(wrapped_val.clone());
            assert!(res.is_ok());
            assert_eq!(res.unwrap(), result);
        }

        assert!(A::try_from(wrapped_val.clone()).is_err());
        assert!(B::try_from(wrapped_val.clone()).is_err());
        assert!(C::try_from(wrapped_val.clone()).is_err());
        assert!(D::try_from(wrapped_val.clone()).is_err());
        assert!(E::try_from(wrapped_val.clone()).is_err());
        assert!(F::try_from(wrapped_val.clone()).is_err());
        assert!(G::try_from(wrapped_val.clone()).is_err());
        assert!(H::try_from(wrapped_val.clone()).is_err());
        assert!(I::try_from(wrapped_val.clone()).is_err());
        assert!(J::try_from(wrapped_val.clone()).is_err());
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
