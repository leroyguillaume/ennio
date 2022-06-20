use std::collections::HashMap;

#[macro_export]
macro_rules! array {
    ($($items:expr),*) => {
        Array::from(vec![$($items),*])
    };
}

macro_rules! impl_primitive_from_for_array {
    ($type:ty) => {
        impl From<Vec<$type>> for Array {
            fn from(items: Vec<$type>) -> Self {
                Self(items.into_iter().map(|item| Value::from(item)).collect())
            }
        }
    };
}

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

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Array(Vec<Value>);

impl Array {
    pub fn new() -> Self {
        Self(vec![])
    }
}

impl_primitive_from_for_array!(bool);
impl_primitive_from_for_array!(u8);
impl_primitive_from_for_array!(u16);
impl_primitive_from_for_array!(u32);
impl_primitive_from_for_array!(u64);
impl_primitive_from_for_array!(i8);
impl_primitive_from_for_array!(i16);
impl_primitive_from_for_array!(i32);
impl_primitive_from_for_array!(i64);
impl_primitive_from_for_array!(String);
impl_primitive_from_for_array!(&str);

impl From<Vec<Value>> for Array {
    fn from(items: Vec<Value>) -> Self {
        Self(items)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Value {
    Bool(bool),
    PositiveInt(u64),
    NegativeInt(i64),
    String(String),
    Array(Array),
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

    mod array {
        use super::*;

        mod from {
            use super::*;

            macro_rules! test_primitive {
                ($name:ident, $expected:expr) => {
                    #[test]
                    fn $name() {
                        let expected: Vec<Value> = $expected
                            .into_iter()
                            .map(|item| Value::from(item))
                            .collect();
                        let array = Array::from($expected.to_vec());
                        assert_eq!(array.0, expected);
                    }
                };
            }

            test_primitive!(bool, [true]);
            test_primitive!(u8, [1]);
            test_primitive!(u16, [1]);
            test_primitive!(u32, [1]);
            test_primitive!(u64, [1]);
            test_primitive!(i8, [-1]);
            test_primitive!(i16, [-1]);
            test_primitive!(i32, [-1]);
            test_primitive!(i64, [-1]);
            test_primitive!(string, [String::from("val")]);
            test_primitive!(str, ["val"]);

            #[test]
            fn value() {
                let expected = vec![Value::Bool(true)];
                let array = Array::from(expected.clone());
                assert_eq!(array.0, expected);
            }
        }
    }

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
