mod lexical_analyzer;
use std::env;
use std::fs::read_to_string;
use lexical_analyzer::LexicalAnalyzer;
pub struct Interpreter;

impl Interpreter {
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

    pub fn parse(input: String) {
        let analyzer = LexicalAnalyzer::new(input);
        let tokens = analyzer.parse().expect("Something went wrong during the lexical analysis.");
        println!("{:?}", tokens);
    }
}
