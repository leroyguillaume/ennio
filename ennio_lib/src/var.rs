use std::collections::HashMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Var {
    Boolean(bool),
    Float(String),
    Hash(Vars),
    Integer(i64),
    List(Vec<Var>),
    String(String),
}

pub type Vars = HashMap<String, Var>;
