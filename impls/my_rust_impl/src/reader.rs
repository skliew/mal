use regex::Regex;

pub fn read_str(input: String) {
}

pub fn tokenize(input: &str) -> Vec<String> {
    let re = Regex::new(
        r###"[\s,]*(~@|[\[\]{}()'`~^@]|"(?:\\.|[^\\"])*"?|;.*|[^\s\[\]{}('"`,;)]*)"###
    ).unwrap();
    let mut result = vec![];
    for cap in re.captures_iter(input) {
        result.push(cap[1].to_string());
    }
    result
}

