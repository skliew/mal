use std::io::Write;

#[macro_use]
mod types;
mod printer;
mod reader;

use fnv::FnvHashMap;
use crate::printer::{pr_maltype};
use crate::reader::read_str;
use crate::types::{MalType, MalRet, MalArgs, Env};

fn read(input: String) -> MalRet {
    read_str(input)
}

fn eval_ast(input: MalType, env: &Env) -> MalRet {
    match(input) {
        MalType::Sym(s) => Ok(
            env.get(&s).ok_or("not found".to_string())?.clone()
            ),
        MalType::List(v) => {
            let mut result : MalArgs = vec![];
            for i in v.iter() {
                result.push(eval(i.clone(), env)?);
            }
            // let result = v.iter().map(|i| eval(i.clone(), env)? ).collect();
            Ok(MalType::List(result))
        },
        MalType::Vector(v) => {
            let mut result : MalArgs = vec![];
            for i in v.iter() {
                result.push(eval(i.clone(), env)?);
            }
            // let result = v.iter().map(|i| eval(i.clone(), env)? ).collect();
            Ok(MalType::Vector(result))
        },
        MalType::Hash(h) => {
            let mut result : FnvHashMap<String, MalType> = FnvHashMap::default();
            for (k,v) in h.iter() {
                result.insert(k.clone(), eval(v.clone(), env)?);
            }
            Ok(MalType::Hash(result))
        },
        _ => Ok(input)
    }
}

fn eval(input: MalType, env: &Env) -> MalRet {
    match input.clone() {
        MalType::List(l) => {
            if l.is_empty() {
                return Ok(input)
            }
            match eval_ast(input, env)? {
                MalType::List(r) => {
                    let f = r[0].clone();
                    f.apply(r[1..].to_vec())
                },
                _ => Err("expected a list".to_string())
            }

        },
        MalType::Vector(l) => {
            if l.is_empty() {
                return Ok(input)
            }
            return eval_ast(input, env)
        },
        _ => eval_ast(input, env)
    }
}

fn print(input: MalType) -> String {
    pr_maltype(&input)
}

fn rep(input: String, env: &Env) -> Result<String, String> {
    let ast = read(input)?;
    let result = eval(ast, env)?;
    Ok(print(result))
}

fn int_op(f: &Fn(i64, i64) -> i64, a: MalArgs) -> MalRet {
    match(a[0].clone(), a[1].clone()) {
        (MalType::Int(a0), MalType::Int(a1)) => Ok(MalType::Int(f(a0, a1))),
        _ => Err("invalid types".to_string())
    }
}

fn main() -> Result<(), std::io::Error> {
    let mut env = Env::default();
    env.insert("+".to_string(),
               MalType::Func(|a: MalArgs| int_op(&|i, j| i + j, a)));
    env.insert("-".to_string(),
               MalType::Func(|a: MalArgs| int_op(&|i, j| i - j, a)));
    env.insert("*".to_string(),
               MalType::Func(|a: MalArgs| int_op(&|i, j| i * j, a)));
    env.insert("/".to_string(),
               MalType::Func(|a: MalArgs| int_op(&|i, j| i / j, a)));

    loop {
        let mut input = String::new();
        print!("user> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input)?;
        let result = rep(input, &env);
        match result {
            Ok(s) => println!("{}", s),
            Err(s) => println!("{:?}", s)
        }
    }
}

