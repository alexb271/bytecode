use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

const MAX_PRECEDENCE_DEPTH: u8 = 6;

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
            Rule::number => parse_number(pair),
            Rule::boolean => ASTNode::Value(Value::Bool(pair.as_str().parse::<bool>().unwrap())),
            Rule::text => ASTNode::Value(Value::Str(Box::new(
                pair.as_str().trim_matches('"').to_string(),
            ))),
            Rule::char => ASTNode::Value(Value::Char(
                pair.as_str().trim_matches('\'').parse::<char>().unwrap(),
            )),
            Rule::expression => parse_recursive(pair, 0),
            _ => {
                dbg!(pair);
                unreachable!()
            }
        }
    }
}

fn parse_number(pair: Pair<Rule>) -> ASTNode {
    let number_string = pair.as_str().replace('_', "");
    if number_string.ends_with("uint") {
        ASTNode::Value(Value::Uint(
            number_string.replace("uint", "").parse::<u64>().unwrap(),
        ))
    } else if number_string.ends_with("int") {
        ASTNode::Value(Value::Int(
            number_string.replace("int", "").parse::<i64>().unwrap(),
        ))
    } else if number_string.ends_with("float") {
        ASTNode::Value(Value::Float(
            number_string.replace("float", "").parse::<f64>().unwrap(),
        ))
    } else if let Ok(result) = number_string.parse::<i64>() {
        ASTNode::Value(Value::Int(result))
    } else if let Ok(result) = number_string.parse::<u64>() {
        ASTNode::Value(Value::Uint(result))
    } else {
        ASTNode::Value(Value::Float(number_string.parse::<f64>().unwrap()))
    }
}

fn parse_bin_operator(pair: Pair<Rule>) -> BinOperator {
    match pair.as_rule() {
        Rule::or => BinOperator::Or,
        Rule::and => BinOperator::And,
        Rule::equal => BinOperator::Equal,
        Rule::not_equal => BinOperator::NotEqual,
        Rule::less_than => BinOperator::LessThan,
        Rule::less_eq => BinOperator::LessEq,
        Rule::greater_than => BinOperator::GreaterThan,
        Rule::greater_eq => BinOperator::GreaterEq,
        Rule::add => BinOperator::Add,
        Rule::sub => BinOperator::Sub,
        Rule::mul => BinOperator::Mul,
        Rule::div => BinOperator::Div,
        Rule::modulo => BinOperator::Mod,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub enum ASTNode {
    Value(Value),
    BinaryOp(Box<ASTNode>, BinOperator, Box<ASTNode>),
}

#[derive(Clone, Copy, Debug)]
pub enum BinOperator {
    Or,
    And,
    Equal,
    NotEqual,
    LessThan,
    LessEq,
    GreaterThan,
    GreaterEq,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

impl std::fmt::Display for BinOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOperator::Or => write!(f, "or"),
            BinOperator::And => write!(f, "and"),
            BinOperator::Equal => write!(f, "=="),
            BinOperator::NotEqual => write!(f, "!="),
            BinOperator::LessThan => write!(f, "<"),
            BinOperator::LessEq => write!(f, "<="),
            BinOperator::GreaterThan => write!(f, ">"),
            BinOperator::GreaterEq => write!(f, ">="),
            BinOperator::Add => write!(f, "+"),
            BinOperator::Sub => write!(f, "-"),
            BinOperator::Mul => write!(f, "*"),
            BinOperator::Div => write!(f, "/"),
            BinOperator::Mod => write!(f, "*"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Uint(u64),
    Float(f64),
    Bool(bool),
    Str(Box<String>),
    Char(char),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Uint(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Str(value) => write!(f, "{}", value),
            Value::Char(value) => write!(f, "{}", value),
        }
    }
}

#[allow(dead_code)]
impl ASTNode {
    pub fn print(&self) {
        self.print_recursive();
        println!();
    }

    fn print_recursive(&self) {
        match self {
            ASTNode::Value(value) => {
                print!("{value}");
            }
            ASTNode::BinaryOp(left, operator, right) => {
                print!("(");
                left.print_recursive();
                print!("{operator}");
                right.print_recursive();
                print!(")");
            }
        }
    }
}
