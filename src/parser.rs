// mod token;

use pest::error::Error;
use pest::Parser;
use pest::iterators::Pair;
use pest_derive::Parser;
use std::collections::HashMap;
// use token::Token;

#[derive(Parser)]
#[grammar = "grammar_rules.pest"]
struct GrammarParser;

#[derive(Debug)]
enum Literal {
    Number(f64),
    Bool(bool),
    Char(char)
}


pub fn parse(input: &str) -> Result<(), Error<Rule>> {
    let mut ast: Vec<&str> = vec![];
    let pairs = GrammarParser::parse(Rule::program, input)?;
    
    for pair in pairs {
        match pair.as_rule() {
            Rule::program => {
                parse_program(pair);
            },
            _ => {}
        }
    }
    println!("{:?}", ast);

    // let result = AbstractSyntaxTree::Program();
    Ok(())
}

fn parse_program(pair: Pair<Rule>) {
    for p in pair.into_inner() {
        // println!("---{:?}---", p.as_rule());

        match p.as_rule() {
            Rule::literal => {
                parse_literal(p);
            },
            _ => {}
        }

    }
}

fn parse_literal(pair: Pair<Rule>) {
    for p in pair.into_inner() {

        let result = match p.as_rule() {
            Rule::boolean => Literal::Bool(p.as_str().parse().unwrap()),
            Rule::number => Literal::Number(p.as_str().parse().unwrap()),
            Rule::char => Literal::Char(p.as_str().parse().unwrap()),
            _ => unreachable!()
        };

        println!("{result:?}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_whitespace() {
        assert!(GrammarParser::parse(Rule::WHITESPACE, " ").is_ok());
    }

    #[test]
    fn detect_boolean_literal() {
        assert!(GrammarParser::parse(Rule::literal, "false").is_ok());
        assert!(GrammarParser::parse(Rule::literal, "true").is_ok());
    }

    #[test]
    fn detect_numerical_literals() {
        assert!(GrammarParser::parse(Rule::literal, "0.1").is_ok());
        assert!(GrammarParser::parse(Rule::literal, "1").is_ok());
        assert!(GrammarParser::parse(Rule::literal, "-0.3242").is_ok());
        assert!(GrammarParser::parse(Rule::literal, "3240").is_ok());
        assert!(GrammarParser::parse(Rule::literal, "-23432").is_ok());
        assert!(GrammarParser::parse(Rule::literal, "0").is_ok());
        assert!(GrammarParser::parse(Rule::literal, "35.7674.456").is_err());
    }
}
