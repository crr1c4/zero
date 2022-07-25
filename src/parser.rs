use pest::error::Error;
use pest::iterators::Pair;
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

pub struct LexicalAnalyzer {
    input: String,
}

impl LexicalAnalyzer {
    pub fn new(input: String) -> Self {
        Self { input }
    }

    pub fn parse(&self) -> Result<Vec<Literal>, Error<Rule>> {
        let analysis = GrammarParser::parse(Rule::program, &self.input)?
            .next()
            .unwrap();
        Ok(LexicalAnalyzer::read_rules(analysis))
    }

    fn read_rules(analysis: Pair<Rule>) -> Vec<Literal> {
        let mut literals: Vec<Literal> = vec![];

        analysis.into_inner().for_each(|rule| match rule.as_rule() {
            Rule::literal => rule
                .into_inner()
                .for_each(|literal| literals.push(LexicalAnalyzer::get_literal(literal))),
            _ => {}
        });

        literals
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
