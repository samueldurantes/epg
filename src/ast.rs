use std::{
  collections::HashMap,
  rc::Rc,
  fmt,
};

pub type Name = String;
pub type BExpr = Rc<Expr>;

#[derive(Clone, Debug)]
pub enum Expr {
  Var(Name),
  Lam(Name, BExpr),
  App(BExpr, BExpr),
  Let(Name, BExpr, BExpr),
  Int(i32),
  Add(BExpr, BExpr)
}

pub type Context = HashMap<Name, Value>;

#[derive(Clone, Debug)]
pub enum Value {
  VInt(i32),
  VClosure(Context, String, BExpr),
}

#[derive(Clone, Debug)]
pub enum TopLevel {
  TopLoad(Vec<Rc<TopLevel>>),
  TopDef(Name, Expr),
}

impl fmt::Display for Value {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match *self {
      Value::VInt(i) => i.fmt(f),
      Value::VClosure(_, _, _) => write!(f, "(function)"),
    }
  }
}
