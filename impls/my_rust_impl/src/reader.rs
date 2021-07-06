use lazy_static::lazy_static;
use regex::Regex;

use crate::types::{MalType, MalRet};

pub struct Reader {
    pos: usize,
    tokens: Vec<String>
}

impl Reader {
    fn peek(&self) -> Result<&str, String> {
        if (self.pos + 1) == self.tokens.len() {
            return Err("EOF".to_string());
        }
        Ok(&self.tokens[self.pos])
    }

    fn next(&mut self) -> Result<&str, String> {
        if (self.pos + 1) == self.tokens.len() {
            return Err("EOF".to_string());
        }
        let result = &self.tokens[self.pos];
        self.pos += 1;
        return Ok(result);
    }
}

pub fn read_str(input: String) -> MalRet {
    let tokens = tokenize(&input);
    let mut reader = Reader {
        pos: 0,
        tokens: tokens
    };
    read_form(&mut reader)
}

pub fn tokenize(input: &str) -> Vec<String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###
        ).unwrap();
    }
    let mut result = vec![];
    for cap in RE.captures_iter(input) {
        result.push(cap[1].to_string());
    }
    result
}

pub fn read_list(reader: &mut Reader) -> MalRet {
    reader.next();
    let mut result : Vec<MalType> = vec![];
    loop {
        let c = reader.peek();
        match c {
            Ok(value) => {
                if value == ")" {
                    reader.next();
                    return Ok(MalType::List(result));
                } else {
                    let item = read_form(reader);
                    result.push(item?);
                }
            },
            Err(v) => return Err(v),
        }
    }
}

pub fn read_atom(reader: &mut Reader) -> MalRet {
    lazy_static! {
        static ref INT_RE: Regex = Regex::new(
            r"^-?[0-9]+$"
        ).unwrap();
    }
    let c = reader.next()?;
    if INT_RE.is_match(c) {
        return Ok(MalType::Int(c.parse().unwrap()));
    } else {
        return Ok(MalType::Sym(c.to_string()));
    }
}


pub fn read_form(reader: &mut Reader) -> MalRet {
    let c = reader.peek();
    match c {
        Ok(v) => {
            match &v[..] {
                "(" => read_list(reader),
                _ => read_atom(reader)
            }
        },
        Err(v) => Err(v),
    }
}

