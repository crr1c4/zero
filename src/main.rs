use std::env;
use zero::Interpreter;

fn main() {
    let input = Interpreter::read_args(env::args());
    Interpreter::parse(input);
}

// TODO: CLI commands projects
// Implement error logging