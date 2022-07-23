use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "parser.pest"]
pub struct GrammarParser;

pub fn parse_input(input: &str) {
    let parse = GrammarParser::parse(Rule::HELLO, &input);
    println!("{:?}", parse)
}

#[cfg(test)]
mod tests {

}