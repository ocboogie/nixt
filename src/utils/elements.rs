use crate::utils::node::Node;

#[derive(Debug, Clone)]
pub struct StackElement {
  typ: VariableType,
  value: Value,
}

#[derive(Debug, Clone)]
pub enum VariableType {
  Constant,
  Mutable,
}
#[derive(Debug, Clone)]
pub enum Value {
  Number(f32),
  Nil,
  Bool(bool),
  String(String),
  List(Vec<Value>),
  Block(Node), // Only for functions (and maybe loops)
}

impl std::fmt::Display for Value {
  fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
    match self {
      Value::Number(n) => {
        write!(f, "{}", n)?;
      }
      Value::String(s) => {
        write!(f, "{}", s)?;
      }
      Value::List(v) => {
        write!(f, "{:?}", v)?;
      }
      _ => {
        write!(f, "nil")?;
      }
    }
    Ok(())
  }
}