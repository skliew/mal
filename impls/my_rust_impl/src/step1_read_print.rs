use std::io::Write;
mod types;
mod printer;
mod reader;

use crate::printer::pr_str;
use crate::reader::read_str;
use crate::types::{MalType, MalRet};

fn read(input: String) -> MalRet {
    read_str(input)
}

fn eval(input: MalRet) -> MalRet {
    input
}

fn print(input: MalRet) -> String {
    pr_str(&input)
}

fn rep(input: String) -> String {
    print(eval(read(input)))
}

fn main() -> Result<(), std::io::Error> {
    loop {
        let mut input = String::new();
        print!("user> ");
        std::io::stdout().flush().unwrap();
        std::io::stdin().read_line(&mut input)?;
        println!("{}", rep(input));
    }
}

