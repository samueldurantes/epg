pub mod ast;
pub mod grammar;

use std::collections::HashMap;
use ast::{
  Name,
  Context,
  Expr,
  Value,
  Expr::*,
  Value::*,
  TopLevel,
};

fn parse(input: &str) -> TopLevel {
  grammar::TopLevelListParser::new().parse(input).unwrap()
}

fn find_variable(name: &Name, ctx: &Context) -> Value {
  match HashMap::get(ctx, name) {
    Some(var) => var.clone(),
    None => panic!("Variable \"{}\" not found!", name),
  }
}

fn eval(expr: &Expr, ctx: Context) -> Value {
  match expr {
    Int(i) => VInt(i.to_owned()),
    Var(name) => find_variable(name, &ctx),
    Lam(func, arg) =>
      VClosure(ctx, func.to_owned(), arg.clone()),
    App(func, arg) => {
      let a = ctx.clone();
      let arg = eval(arg.as_ref(), ctx);
      match eval(func.as_ref(), a) {
        VClosure(c, p, b) => {
          let mut c = c.clone();
          c.insert(p, arg);
          eval(&b, c.to_owned())
        }
        _ => todo!(),
      }
    },
    Let(n, e, x) => {
      let mut a = ctx.clone();
      let b = eval(e.as_ref(), ctx);
      a.insert(n.to_string(), b);

      eval(x, a)
    },
    Add(arg1, arg2) => {
      let a = ctx.clone();
      match (eval(arg1, ctx), eval(arg2, a)) {
        (VInt(i), VInt(k)) => VInt(i + k),
        _ => todo!(),
      }
    },
  }
}

fn eval_toplevel(t: &TopLevel, ctx: Context) -> Value {
  let mut ctx = ctx.clone();

  if let TopLevel::TopLoad(xs) = t {
    for x in xs {
      match x.as_ref() {
        TopLevel::TopDef(n, e) => {
          ctx.insert(n.to_string(), eval(&e, ctx.clone()));
        },
        _ => todo!(),
      }
    }
  }

  match ctx.get("main").clone() {
    Some(e) => e.clone(),
    None => panic!("main not defined"),
  }
}

fn main() {
  let ctx: Context = HashMap::new();

  let ex: &str = "
    pls = #x => x + 1
    cti = #n => n(pls)(0)
    two = #f => #x => f(f(x))
    exp = #m => #n => n(m)

    id = #x => x

    main = cti(exp(two)(two))
  ";

  println!("{}", eval_toplevel(&parse(ex), ctx.clone()));
}
