use super::value::{KuzuValue, Node, Relation};

impl From<KuzuValue> for bool {
    fn from(value: KuzuValue) -> Self {
        if let KuzuValue::Bool(inner) = value {
            return inner;
        }
        panic!("1222222")
    }
}

impl From<KuzuValue> for i16 {
    fn from(value: KuzuValue) -> Self {
        if let KuzuValue::Int16(inner) = value {
            return inner;
        }
        panic!("1222222")
    }
}

impl From<KuzuValue> for i32 {
    fn from(value: KuzuValue) -> Self {
        if let KuzuValue::Int32(inner) = value {
            return inner;
        }
        panic!("1222222")
    }
}

impl From<KuzuValue> for i64 {
    fn from(value: KuzuValue) -> Self {
        if let KuzuValue::Int64(inner) = value {
            return inner;
        }
        panic!("1222222")
    }
}

impl From<KuzuValue> for f32 {
    fn from(value: KuzuValue) -> Self {
        if let KuzuValue::Float(inner) = value {
            return inner;
        }
        panic!("1222222")
    }
}

impl From<KuzuValue> for f64 {
    fn from(value: KuzuValue) -> Self {
        if let KuzuValue::Double(inner) = value {
            return inner;
        }
        panic!("1222222")
    }
}

impl From<KuzuValue> for String {
    fn from(value: KuzuValue) -> Self {
        match value {
            KuzuValue::String(string) => string,
            _ => unreachable!(),
        }
    }
}

impl From<KuzuValue> for Node {
    fn from(value: KuzuValue) -> Self {
        match value {
            KuzuValue::Node(node) => node,
            _ => unreachable!(),
        }
    }
}
impl From<KuzuValue> for Relation {
    fn from(value: KuzuValue) -> Self {
        match value {
            KuzuValue::Rel(rel) => rel,
            _ => unreachable!(),
        }
    }
}
