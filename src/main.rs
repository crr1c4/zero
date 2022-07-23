use std::env;
use zero::Interpreter;

fn main() {
    let content = Interpreter::read_args(env::args());
    Interpreter::parse(&content);
}
