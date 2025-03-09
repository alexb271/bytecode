use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

const MAX_BINARY_PRECEDENCE_DEPTH: u8 = 6;
const MAX_PRECEDENCE_DEPTH: u8 = 7;

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

    if level < MAX_BINARY_PRECEDENCE_DEPTH {
        let mut left = parse_recursive(inner_rules.next().unwrap(), level + 1);

        while let Some(pair) = inner_rules.next() {
            let operator_span_start = pair.as_span().start();
            let operator_span_end = pair.as_span().end();
            let operator = parse_binary_operator(pair);

            let right = parse_recursive(inner_rules.next().unwrap(), level + 1);

            left = ASTNode::new(
                left.span_start,
                right.span_end,
                ASTNodeContent::BinaryOperation(BinaryOperation::new(
                    operator_span_start,
                    operator_span_end,
                    Box::new(left),
                    operator,
                    Box::new(right),
                )),
            );
        }
        left
    } else if level < MAX_PRECEDENCE_DEPTH {
        let pair = inner_rules.next().unwrap();

        // if there is no unary prefix operator at all, pass through
        if let Rule::level_7 = pair.as_rule() {
            return parse_recursive(pair, level + 1);
        }

        let operator_span_start = pair.as_span().start();
        let operator_span_end = pair.as_span().end();
        let operator = parse_unary_operator(pair);

        let node = parse_recursive(
            inner_rules.next().unwrap(),
            level,
        );

        ASTNode::new(
            operator_span_start,
            node.span_end,
            ASTNodeContent::UnaryOperation(UnaryOperation::new(
                operator_span_start,
                operator_span_end,
                operator,
                Box::new(node),
            )),
        )
    } else {
        let pair = inner_rules.next().unwrap();
        let span_start = pair.as_span().start();
        let span_end = pair.as_span().end();
        match pair.as_rule() {
            Rule::number => ASTNode::new(span_start, span_end, parse_number(pair)),
            Rule::boolean => ASTNode::new(
                span_start,
                span_end,
                ASTNodeContent::Value(Value::Bool(pair.as_str().parse::<bool>().unwrap())),
            ),
            Rule::text => ASTNode::new(
                span_start,
                span_end,
                ASTNodeContent::Value(Value::Str(Box::new(
                    pair.as_str().trim_matches('"').to_string(),
                ))),
            ),
            Rule::char => ASTNode::new(
                span_start,
                span_end,
                ASTNodeContent::Value(Value::Char(
                    pair.as_str().trim_matches('\'').parse::<char>().unwrap(),
                )),
            ),
            Rule::expression => parse_recursive(pair, 0),
            _ => {
                dbg!(pair);
                unreachable!()
            }
        }
    }
}

#[inline]
fn parse_number(pair: Pair<Rule>) -> ASTNodeContent {
    let number_string = pair.as_str().replace('_', "");
    if let Ok(result) = number_string.parse::<i64>() {
        ASTNodeContent::Value(Value::Int(result))
    } else {
        ASTNodeContent::Value(Value::Float(number_string.parse::<f64>().unwrap()))
    }
}

#[inline]
fn parse_binary_operator(pair: Pair<Rule>) -> BinaryOperator {
    match pair.as_rule() {
        Rule::or => BinaryOperator::Or,
        Rule::and => BinaryOperator::And,
        Rule::equal => BinaryOperator::Equal,
        Rule::not_equal => BinaryOperator::NotEqual,
        Rule::less_than => BinaryOperator::LessThan,
        Rule::less_eq => BinaryOperator::LessEq,
        Rule::greater_than => BinaryOperator::GreaterThan,
        Rule::greater_eq => BinaryOperator::GreaterEq,
        Rule::add => BinaryOperator::Add,
        Rule::sub => BinaryOperator::Sub,
        Rule::mul => BinaryOperator::Mul,
        Rule::div => BinaryOperator::Div,
        Rule::modulo => BinaryOperator::Mod,
        _ => unreachable!(),
    }
}

#[inline]
fn parse_unary_operator(pair: Pair<Rule>) -> UnaryOperator {
    match pair.as_rule() {
        Rule::not => UnaryOperator::Not,
        Rule::neg => UnaryOperator::Neg,
        _ => unreachable!(),
    }
}

#[derive(Debug)]
pub struct ASTNode {
    span_start: usize,
    span_end: usize,
    content: ASTNodeContent,
}

impl ASTNode {
    #[inline]
    pub fn new(span_start: usize, span_end: usize, content: ASTNodeContent) -> ASTNode {
        ASTNode {
            span_start,
            span_end,
            content,
        }
    }

    #[inline]
    pub fn span_start(&self) -> usize {
        self.span_start
    }

    #[inline]
    pub fn span_end(&self) -> usize {
        self.span_end
    }

    #[inline]
    pub fn content(&self) -> &ASTNodeContent {
        &self.content
    }
}

#[derive(Debug)]
pub enum ASTNodeContent {
    Value(Value),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
}

#[derive(Debug)]
pub struct BinaryOperation {
    operator_span_start: usize,
    operator_span_end: usize,
    left: Box<ASTNode>,
    operator: BinaryOperator,
    right: Box<ASTNode>,
}

impl BinaryOperation {
    #[inline]
    pub fn new(
        operator_span_start: usize,
        operator_span_end: usize,
        left: Box<ASTNode>,
        operator: BinaryOperator,
        right: Box<ASTNode>,
    ) -> BinaryOperation {
        BinaryOperation {
            operator_span_start,
            operator_span_end,
            left,
            operator,
            right,
        }
    }

    #[inline]
    pub fn operator_span_start(&self) -> usize {
        self.operator_span_start
    }
    #[inline]
    pub fn operator_span_end(&self) -> usize {
        self.operator_span_end
    }
    #[inline]
    pub fn left(&self) -> &Box<ASTNode> {
        &self.left
    }
    #[inline]
    pub fn operator(&self) -> BinaryOperator {
        self.operator
    }
    #[inline]
    pub fn right(&self) -> &Box<ASTNode> {
        &self.right
    }
}

#[derive(Clone, Copy, Debug)]
pub enum BinaryOperator {
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

impl std::fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinaryOperator::Or => write!(f, "or"),
            BinaryOperator::And => write!(f, "and"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::LessThan => write!(f, "<"),
            BinaryOperator::LessEq => write!(f, "<="),
            BinaryOperator::GreaterThan => write!(f, ">"),
            BinaryOperator::GreaterEq => write!(f, ">="),
            BinaryOperator::Add => write!(f, "+"),
            BinaryOperator::Sub => write!(f, "-"),
            BinaryOperator::Mul => write!(f, "*"),
            BinaryOperator::Div => write!(f, "/"),
            BinaryOperator::Mod => write!(f, "%"),
        }
    }
}
#[derive(Debug)]
pub struct UnaryOperation {
    operator_span_start: usize,
    operator_span_end: usize,
    operator: UnaryOperator,
    operand: Box<ASTNode>,
}

impl UnaryOperation {
    #[inline]
    pub fn new(
        operator_span_start: usize,
        operator_span_end: usize,
        operator: UnaryOperator,
        operand: Box<ASTNode>,
    ) -> UnaryOperation {
        UnaryOperation {
            operator_span_start,
            operator_span_end,
            operator,
            operand,
        }
    }

    #[inline]
    pub fn operator_span_start(&self) -> usize {
        self.operator_span_start
    }
    #[inline]
    pub fn operator_span_end(&self) -> usize {
        self.operator_span_end
    }
    #[inline]
    pub fn operator(&self) -> UnaryOperator {
        self.operator
    }
    #[inline]
    pub fn operand(&self) -> &Box<ASTNode> {
        &self.operand
    }
}

#[derive(Clone, Copy, Debug)]
pub enum UnaryOperator {
    Not,
    Neg,
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnaryOperator::Not => write!(f, "not"),
            UnaryOperator::Neg => write!(f, "-"),
        }
    }
}

#[derive(Clone, Debug)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(Box<String>),
    Char(char),
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Str(value) => write!(f, "{}", value),
            Value::Char(value) => write!(f, "{}", value),
        }
    }
}

#[allow(dead_code)]
impl ASTNodeContent {
    pub fn print(&self) {
        self.print_recursive();
        println!();
    }

    fn print_recursive(&self) {
        match self {
            ASTNodeContent::Value(value) => {
                print!("{value}");
            }
            ASTNodeContent::BinaryOperation(binop) => {
                print!("(");
                binop.left.content.print_recursive();
                print!("{}", binop.operator);
                binop.right.content.print_recursive();
                print!(")");
            }
            ASTNodeContent::UnaryOperation(unop) => {
                print!("(");
                print!("{}", unop.operator);
                unop.operand.content.print_recursive();
                print!(")");
            }
        }
    }
}
