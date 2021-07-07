use crate::types::{MalType, MalRet};

pub fn pr_str(mal: &MalRet) -> String {
    match mal {
        Ok(v) => pr_maltype(v),
        Err(v) => v.to_string(),
    }
}

pub fn pr_maltype(mal: &MalType) -> String {
    match mal {
        MalType::Int(i) => i.to_string(),
        MalType::Sym(s) => s.to_string(),
        MalType::List(v) => {
            let mut result : Vec<String> = vec![];
            for item in v.iter() {
                result.push(pr_maltype(&item));
            }
            let mut result_string = String::new(); 
            result_string.push_str("(");
            result_string.push_str(&result.join(" "));
            result_string.push_str(")");
            return result_string;
        },
        MalType::Vector(v) => {
            let mut result : Vec<String> = vec![];
            for item in v.iter() {
                result.push(pr_maltype(&item));
            }
            let mut result_string = String::new(); 
            result_string.push_str("[");
            result_string.push_str(&result.join(" "));
            result_string.push_str("]");
            return result_string;
        },
        MalType::Hash(h) => {
            let mut result : Vec<String> = vec![];
            for (k, v) in h.iter() {
                result.push(k.clone());
                result.push(pr_maltype(v));
            }
            let mut result_string = String::new(); 
            result_string.push_str("{");
            result_string.push_str(&result.join(" "));
            result_string.push_str("}");
            return result_string;
        }
    }
}
