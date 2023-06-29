use crate::error;

use super::value::{KuzuValue, Node, Relation};

macro_rules! impl_decode {
    ($ty:ty, $inner:ident) => {
        impl TryFrom<KuzuValue> for $ty {
            type Error = error::Error;
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
