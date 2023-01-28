pub mod ast;
pub mod cli;
pub mod grammar;

use ast::{Context, Expr, Expr::*, Name, TopLevel, Value, Value::*};
use clap::Parser;
use cli::{Cli, Command};
use std::collections::HashMap;

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
        Lam(func, arg) => VClosure(ctx, func.to_owned(), arg.clone()),
        App(func, arg) => {
            let a = ctx.clone();
            let arg = eval(arg.as_ref(), ctx);
            match eval(func.as_ref(), a) {
                VClosure(c, p, b) => {
                    let mut c = c;
                    c.insert(p, arg);
                    eval(&b, c.to_owned())
                }
                _ => todo!(),
            }
        }
        Let(n, e, x) => {
            let mut a = ctx.clone();
            let b = eval(e.as_ref(), ctx);
            a.insert(n.to_string(), b);

            eval(x, a)
        }
        Add(arg1, arg2) => {
            let a = ctx.clone();
            match (eval(arg1, ctx), eval(arg2, a)) {
                (VInt(i), VInt(k)) => VInt(i + k),
                _ => todo!(),
            }
        }
    }
}

fn eval_toplevel(t: &TopLevel, ctx: Context) -> Value {
    let mut ctx = ctx;

    if let TopLevel::TopLoad(xs) = t {
        for x in xs {
            match x.as_ref() {
                TopLevel::TopDef(n, e) => {
                    ctx.insert(n.to_string(), eval(e, ctx.clone()));
                }
                _ => todo!(),
            }
        }
    }

    match ctx.get("main") {
        Some(e) => e.clone(),
        None => panic!("main not defined"),
    }
}

fn run_code(code: &str) {
    let ctx: Context = HashMap::new();

    println!("{}", eval_toplevel(&parse(code), ctx))
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Run { file } => {
            match std::fs::read_to_string(file) {
                Ok(content) => run_code(&content),
                Err(_) => panic!("File not found!"),
            }
        },
    }
}
