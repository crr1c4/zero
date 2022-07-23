use std::env;
use zero::read_args;
use zero::Lexer;

fn main() {
    let content = read_args(env::args());
    Lexer::lex(content);
}
