use std::{
  collections::HashMap,
  rc::Rc,
};
use crate::Expr::*;
use crate::Value::*;

type Name = String;
type BExpr = Rc<Expr>;

#[derive(Clone, Debug)]
enum Expr {
  Var(Name),
  Lam(Name, BExpr),
  App(BExpr, BExpr),
  // Let(Name, BExpr, BExpr),
}

type Context = HashMap<Name, Value>;

#[derive(Clone, Debug)]
enum Value {
  Int(i64),
  Closure(Context, String, BExpr), // ctx, param, body
}

enum TopLevel {
  Def(Name, Expr),
}

fn find_variable(name: &Name, ctx: &Context) -> Value {
  match HashMap::get(ctx, name) {
    Some(var) => var.clone(),
    None => panic!("Variable \"{}\" not found!", name),
  }
}

fn eval(expr: &Expr, ctx: Context) -> Value {
  match expr {
    Var(name) => find_variable(name, &ctx),
    Lam(func, arg) =>
      Closure(ctx, func.to_owned(), arg.clone()),
    App(func, arg) => {
      let a = ctx.clone();
      let arg = eval(arg.as_ref(), ctx);
      match eval(func.as_ref(), a) {
        Closure(c, p, b) => {
          let mut c = c.clone();
          c.insert(p, arg);
          eval(&b, c.to_owned())
        }
        _ => todo!(),
      }
    },
  }
}

fn main() {
  // exp = \m -> \n -> n m
  let exp: Expr = Lam(
    "m".to_owned(),
    Rc::new(
      Lam(
        "n".to_owned(),
        Rc::new(
          App(
            Rc::new(Var("n".to_owned())),
            Rc::new(Var("m".to_owned())),
          ),
        )
      )
    )
  );

  let two: Expr = Lam(
    "f".to_owned(),
    Rc::new(
      Lam(
        "x".to_owned(),
        Rc::new(
          App(
            Rc::new(Var("f".to_owned())),
            Rc::new(
              App(
                Rc::new(Var("f".to_owned())),
                Rc::new(Var("x".to_owned())),
              ),
            )
          ),
        )
      )
    )
  );

  let a = two.clone();

  // App(App("exp", "two"), two)
  let app = Rc::new(
    App(
      Rc::new(
        App(
          Rc::new(exp),
          Rc::new(two),
        )
      ),
      Rc::new(a),
    )
  );

  let data = vec![];
  let ctx: Context = HashMap::from_iter(data.into_iter());

  // let v = eval(&Var("one".to_owned()), ctx);
  let f = eval(&app, ctx);

  // println!("{v:?}");
  println!("{f:?}");
}
