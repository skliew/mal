use fnv::FnvHashMap;
use itertools::Itertools;

use crate::printer::pr_str;

#[derive(Debug, Clone)]
pub enum MalType {
    Int(i64),
    Sym(String),
    List(Vec<MalType>),
    Vector(Vec<MalType>),
    Hash(FnvHashMap<String, MalType>),
    Func(fn(MalArgs) -> MalRet)
}

pub type MalArgs = Vec<MalType>;
pub type MalRet = Result<MalType, String>;

pub type Env = FnvHashMap<String, MalType>;

pub fn to_hashmap(seq: Vec<MalType>) -> Result<MalType, String> {
    let mut hm : FnvHashMap<String, MalType> = FnvHashMap::default();
    for (k, v) in seq.iter().tuples() {
        match k {
            MalType::Sym(s) => {
                hm.insert(s.to_string(), v.clone());
            },
            _ => return Err("Invalid key".to_string()),
        }
    }
    Ok(MalType::Hash(hm))
}

#[macro_export]
macro_rules! list {
    ( $($x:expr),* ) => {{
        let mut result : Vec<MalType> = vec![];
        $(
            result.push($x);
        )*
        MalType::List(result)
    }}
}

impl MalType {
    pub fn apply(&self, args: MalArgs) -> MalRet {
        match self {
            MalType::Func(f) => f(args),
            _ => Err("Not a function".to_string())
        }
    }
}
