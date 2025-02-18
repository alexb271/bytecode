use crate::opcode::Word;
use pest::{iterators::Pair, Parser};
use pest_derive::Parser;

const MAX_PRECEDENCE_DEPTH: u8 = 2;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

pub fn parse(input: &str) -> Result<ASTNode, String> {
    let parse_result = MyParser::parse(Rule::start_symbol, &input);
    match parse_result {
        Ok(mut parse_content) => {
            let ast = parse_recursive(
                parse_content.next().unwrap().into_inner().next().unwrap(),
                0,
            );
            Ok(ast)
        }
        Err(e) => Err(e.to_string()),
    }
}

fn parse_recursive(pair: Pair<Rule>, level: u8) -> ASTNode {
    let mut inner_rules = pair.into_inner();

    if level < MAX_PRECEDENCE_DEPTH {
        let mut left = parse_recursive(inner_rules.next().unwrap(), level + 1);

        while let Some(pair) = inner_rules.next() {
            let operator = parse_bin_operator(pair);
            let right = parse_recursive(inner_rules.next().unwrap(), level + 1);
            left = ASTNode::BinaryOp(Box::new(left), operator, Box::new(right))
        }
        left
    } else {
        let pair = inner_rules.next().unwrap();
        match pair.as_rule() {
            Rule::number => ASTNode::Number(pair.as_str().parse::<Word>().unwrap()),
            Rule::expression => parse_recursive(pair, 0),
            _ => {
                dbg!(pair);
                unreachable!()
            }
        }
    }
}

fn parse_bin_operator(pair: Pair<Rule>) -> BinOperator {
    match pair.as_rule() {
        Rule::add => BinOperator::Add,
        Rule::sub => BinOperator::Sub,
        Rule::mul => BinOperator::Mul,
        Rule::div => BinOperator::Div,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub enum BinOperator {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum ASTNode {
    Number(Word),
    BinaryOp(Box<ASTNode>, BinOperator, Box<ASTNode>),
}

impl ASTNode {
    #[allow(dead_code)]
    pub fn print(&self) {
        self.print_recursive();
        println!();
    }

    fn print_recursive(&self) {
        match self {
            ASTNode::Number(value) => {
                print!("{}", value);
            }
            ASTNode::BinaryOp(left, operator, right) => {
                print!("(");
                left.print_recursive();

                match operator {
                    BinOperator::Add => print!("+"),
                    BinOperator::Sub => print!("-"),
                    BinOperator::Mul => print!("*"),
                    BinOperator::Div => print!("/"),
                };

                right.print_recursive();
                print!(")");
            }
        }
    }
}
