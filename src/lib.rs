use std::env;
use std::fs::read_to_string;

pub fn read_args(mut args: env::Args) -> String {
    args.next();

    let file_name = match args.next() {
        Some(name) => name,
        None => panic!("Failed to read line"),
    };

    if !file_name.ends_with(".ze") {
        panic!("Invalid file type")
    }

    let file_content = match read_to_string(file_name) {
        Ok(content) => content,
        Err(e) => panic!("Problem reading to file: {e}"),
    };

    file_content
}

pub struct Lexer {

}

impl Lexer {
    pub fn lex(input: String) {
        input.chars().for_each(|c| println!("{c:?}"))
    }
}

enum Token {
    Keyword(String, u64),
    Identifier(String, u64),
    Operator(String, u64),
    Separators(char, u64)
}

