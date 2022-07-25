use std::env;
use zero::Interpreter;

fn main() {
    let input = Interpreter::read_args(env::args());
    Interpreter::parse(input);
}
