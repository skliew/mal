#[derive(Debug)]
pub enum MalType {
    Int(i64),
    Sym(String),
    List(Vec<MalType>)
}

pub type MalRet = Result<MalType, String>;
