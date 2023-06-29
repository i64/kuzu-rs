use super::value::KuzuValue;

impl From<bool> for KuzuValue {
    fn from(value: bool) -> Self {
        KuzuValue::Bool(value)
    }
}

impl From<i16> for KuzuValue {
    fn from(value: i16) -> Self {
        KuzuValue::Int16(value)
    }
}

impl From<i32> for KuzuValue {
    fn from(value: i32) -> Self {
        KuzuValue::Int32(value)
    }
}

impl From<i64> for KuzuValue {
    fn from(value: i64) -> Self {
        KuzuValue::Int64(value)
    }
}

impl From<f32> for KuzuValue {
    fn from(value: f32) -> Self {
        KuzuValue::Float(value)
    }
}

impl From<f64> for KuzuValue {
    fn from(value: f64) -> Self {
        KuzuValue::Double(value)
    }
}

impl From<&str> for KuzuValue {
    fn from(value: &str) -> Self {
        KuzuValue::String(value.to_owned())
    }
}
