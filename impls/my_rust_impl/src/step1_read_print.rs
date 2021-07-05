mod reader;

fn read(input: String) -> String {
    input
}

fn eval(input: String) -> String {
    input
}

fn print(input: String) -> String {
    input
}

fn rep(input: String) -> String {
    print(eval(read(input)))
}

fn main() -> Result<(), std::io::Error> {
    loop {
        let mut input = String::new();
        println!("user> ");
        std::io::stdin().read_line(&mut input)?;
        rep(input);
    }
}

