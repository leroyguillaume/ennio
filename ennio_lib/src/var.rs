use std::collections::HashMap;

macro_rules! value_from {
    ($type:ty, $value:expr) => {
        impl From<$type> for Value {
            fn from(val: $type) -> Self {
                $value(val)
            }
        }
    };

    ($type:ty, $value:expr, $target_type:ty) => {
        impl From<$type> for Value {
            fn from(val: $type) -> Self {
                $value(val as $target_type)
            }
        }
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Bool(bool),
    PositiveInt(u64),
    NegativeInt(i64),
    String(String),
    List(Vec<Value>),
    Hash(Hash),
}

value_from!(bool, Value::Bool, bool);
value_from!(u8, Value::PositiveInt, u64);
value_from!(u16, Value::PositiveInt, u64);
value_from!(u32, Value::PositiveInt, u64);
value_from!(u64, Value::PositiveInt);
value_from!(i8, Value::NegativeInt, i64);
value_from!(i16, Value::NegativeInt, i64);
value_from!(i32, Value::NegativeInt, i64);
value_from!(i64, Value::NegativeInt);
value_from!(String, Value::String);

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Self::String(val.into())
    }
}

pub type Hash = HashMap<String, Value>;

#[cfg(test)]
mod test {
    use super::*;

    mod value {
        use super::*;

        mod from {
            use super::*;

            macro_rules! test {
                ($name:ident, $entry:expr, $expected:expr) => {
                    #[test]
                    fn $name() {
                        let val = Value::from($entry);
                        assert_eq!(val, $expected);
                    }
                };
            }

            test!(bool, true, Value::Bool(true));
            test!(u8, 1u8, Value::PositiveInt(1));
            test!(u16, 1u16, Value::PositiveInt(1));
            test!(u32, 1u32, Value::PositiveInt(1));
            test!(u64, 1u64, Value::PositiveInt(1));
            test!(i8, -1i8, Value::NegativeInt(-1));
            test!(i16, -1i16, Value::NegativeInt(-1));
            test!(i32, -1i32, Value::NegativeInt(-1));
            test!(i64, -1i64, Value::NegativeInt(-1));
            test!(
                string,
                String::from("val"),
                Value::String(String::from("val"))
            );
            test!(str, "val", Value::String(String::from("val")));
        }
    }
}
