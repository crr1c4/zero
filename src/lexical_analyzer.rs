use pest::error::Error;
use pest::iterators::{Pair, Pairs};
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammar_rules.pest"]
struct GrammarParser;

#[derive(Debug)]
pub enum Literal {
    Number(f64),
    Bool(bool),
    Char(char),
}

#[derive(Debug)]
pub enum AST {
    Value(Literal),
    Expression(String, Literal),
}

pub struct LexicalAnalyzer {
    input: String,
}

impl LexicalAnalyzer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn parse(&self) -> Result<Vec<AST>, Error<Rule>> {
        let analysis = GrammarParser::parse(Rule::program, &self.input)?
            .next()
            .unwrap();

        // println!("{:?}", analysis);
        Ok(LexicalAnalyzer::read_rules(analysis))
    }

    fn read_rules(analysis: Pair<Rule>) -> Vec<AST> {
        let mut ast: Vec<AST> = vec![];

        analysis.into_inner().for_each(|rule| match rule.as_rule() {
            Rule::literal => rule
                .into_inner()
                .for_each(|literal| ast.push(AST::Value(LexicalAnalyzer::get_literal(literal)))),
            Rule::expression => ast.push(LexicalAnalyzer::get_expression(rule.into_inner())),
            _ => {}
        });

        ast
    }

    fn get_expression(mut expression: Pairs<Rule>) -> AST {
        let identifier = expression.next().unwrap().as_str().to_string();
        let value = expression.next().unwrap().into_inner().next().unwrap();
        let value = LexicalAnalyzer::get_literal(value);
        AST::Expression(identifier, value)
    }

    fn get_literal(literal: Pair<Rule>) -> Literal {
        match literal.as_rule() {
            Rule::boolean => Literal::Bool(literal.as_str().parse().unwrap()),
            Rule::number => Literal::Number(literal.as_str().parse().unwrap()),
            Rule::char => Literal::Char(literal.as_str().parse().unwrap()),
            _ => unreachable!(),
        }
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

    // TODO: implenent literal enum tests
}
