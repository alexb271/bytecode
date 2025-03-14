use super::language_components::*;
use pest::{Parser, iterators::Pair};
use pest_derive::Parser;

const MAX_BINARY_PRECEDENCE_DEPTH: u8 = 6;
const MAX_PRECEDENCE_DEPTH: u8 = 7;

#[derive(Parser)]
#[grammar = "grammar.pest"]
struct MyParser;

pub fn parse(input: &str) -> Result<FunctionBody, String> {
    let parse_result = MyParser::parse(Rule::start_symbol, &input);
    match parse_result {
        Ok(mut parse_content) => {
            let function_body_pair = parse_content.next().unwrap().into_inner().next().unwrap();
            Ok(parse_function_body(function_body_pair))
        }
        Err(e) => Err(e.to_string()),
    }
}

fn parse_function_body(pair: Pair<Rule>) -> FunctionBody {
    let mut result: Vec<ControlFlow> = Vec::new();
    let mut inner_rules = pair.into_inner();
    while let Some(control_flow_pair) = inner_rules.next() {
        result.push(parse_control_flow(control_flow_pair));
    }
    FunctionBody::new(result)
}

fn parse_control_flow(pair: Pair<Rule>) -> ControlFlow {
    let control_flow_pair = pair.into_inner().next().unwrap();
    match control_flow_pair.as_rule() {
        Rule::while_loop => ControlFlow::WhileLoop(parse_while_loop(control_flow_pair)),
        Rule::basic_block => ControlFlow::BasicBlock(parse_basic_block(control_flow_pair)),
        _ => unreachable!(),
    }
}

fn parse_while_loop(pair: Pair<Rule>) -> WhileLoop {
    let span = Span::new(pair.as_span().start(), pair.as_span().end());
    let mut inner_rules = pair.into_inner();
    let condition = parse_expression(inner_rules.next().unwrap(), 0);
    let mut body: Vec<Statement> = Vec::new();

    if let Some(basic_block_pair) = inner_rules.next() {
        let mut inner_rules = basic_block_pair.into_inner();
        while let Some(statement_pair) = inner_rules.next() {
            body.push(parse_statement(statement_pair));
        }
    }
    WhileLoop::new(span, condition, BasicBlock::new(body))
}

fn parse_basic_block(pair: Pair<Rule>) -> BasicBlock {
    let mut inner_rules = pair.into_inner();
    let mut result: Vec<Statement> = Vec::new();

    while let Some(pair) = inner_rules.next() {
        match pair.as_rule() {
            Rule::statement => {
                result.push(parse_statement(pair));
            }
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    BasicBlock::new(result)
}

fn parse_statement(pair: Pair<Rule>) -> Statement {
    match pair.as_rule() {
        Rule::statement => parse_statement(pair.into_inner().next().unwrap()),
        Rule::let_statement => Statement::LetStatement(parse_let_statement(pair)),
        Rule::assignment => Statement::Assignment(parse_assignment(pair)),
        Rule::return_statement => Statement::ReturnStatement(parse_return_statement(pair)),
        Rule::expression_statement => parse_statement(pair.into_inner().next().unwrap()),
        Rule::expression => Statement::Expression(parse_expression(pair, 0)),
        _ => unreachable!(),
    }
}

fn parse_let_statement(pair: Pair<Rule>) -> LetStatement {
    let span = Span::new(pair.as_span().start(), pair.as_span().end());
    let mut inner_rules = pair.into_inner();

    let identifier_pair = inner_rules.next().unwrap();
    let identifier_span = Span::new(
        identifier_pair.as_span().start(),
        identifier_pair.as_span().end(),
    );
    let identifier = Identifier::new(identifier_pair.as_str().to_owned(), identifier_span);

    let operator_pair = inner_rules.next().unwrap();
    let operator_span = Span::new(
        operator_pair.as_span().start(),
        operator_pair.as_span().end(),
    );

    let expression = parse_expression(inner_rules.next().unwrap(), 0);

    LetStatement::new(span, identifier, operator_span, Box::new(expression))
}

fn parse_assignment(pair: Pair<Rule>) -> Assignment {
    let span = Span::new(pair.as_span().start(), pair.as_span().end());
    let mut inner_rules = pair.into_inner();

    let lhs_pair = inner_rules.next().unwrap();
    let lhs_span = Span::new(lhs_pair.as_span().start(), lhs_pair.as_span().end());
    let lhs = Identifier::new(lhs_pair.as_str().to_owned(), lhs_span);

    let operator_pair = inner_rules.next().unwrap();
    let operator_span = Span::new(
        operator_pair.as_span().start(),
        operator_pair.as_span().end(),
    );
    let operator = parse_assignment_operator(operator_pair);

    let rhs = parse_expression(inner_rules.next().unwrap(), 0);

    Assignment::new(span, lhs, operator, operator_span, Box::new(rhs))
}

fn parse_return_statement(pair: Pair<Rule>) -> ReturnStatement {
    let span = Span::new(pair.as_span().start(), pair.as_span().end());
    let expression = parse_expression(pair.into_inner().next().unwrap(), 0);
    ReturnStatement::new(span, Box::new(expression))
}

fn parse_expression(pair: Pair<Rule>, level: u8) -> Expression {
    let mut inner_rules = pair.into_inner();

    if level < MAX_BINARY_PRECEDENCE_DEPTH {
        let mut left = parse_expression(inner_rules.next().unwrap(), level + 1);

        while let Some(pair) = inner_rules.next() {
            let operator_span = Span::new(pair.as_span().start(), pair.as_span().end());
            let operator = parse_binary_operator(pair);

            let right = parse_expression(inner_rules.next().unwrap(), level + 1);

            left = Expression::BinaryOperation(BinaryOperation::new(
                Span::new(left.span().start(), right.span().end()),
                operator_span,
                Box::new(left),
                operator,
                Box::new(right),
            ));
        }
        left
    } else if level < MAX_PRECEDENCE_DEPTH {
        let pair = inner_rules.next().unwrap();

        // if there is no unary prefix operator at all, pass through
        if let Rule::level_7 = pair.as_rule() {
            return parse_expression(pair, level + 1);
        }

        let operator_span = Span::new(pair.as_span().start(), pair.as_span().end());
        let operator = parse_unary_operator(pair);

        let operand = parse_expression(inner_rules.next().unwrap(), level);

        Expression::UnaryOperation(UnaryOperation::new(
            Span::new(operator_span.start(), operand.span().end()),
            operator_span,
            operator,
            Box::new(operand),
        ))
    } else {
        let pair = inner_rules.next().unwrap();
        let span_start = pair.as_span().start();
        let span_end = pair.as_span().end();
        match pair.as_rule() {
            Rule::number => Expression::Literal(Literal::new(
                parse_number(pair),
                Span::new(span_start, span_end),
            )),
            Rule::boolean => Expression::Literal(Literal::new(
                Value::Bool(pair.as_str().parse::<bool>().unwrap()),
                Span::new(span_start, span_end),
            )),
            Rule::string => Expression::Literal(Literal::new(
                Value::Str(Box::new(pair.as_str().trim_matches('"').to_owned())),
                Span::new(span_start, span_end),
            )),
            Rule::char => Expression::Literal(Literal::new(
                Value::Char(pair.as_str().trim_matches('\'').parse::<char>().unwrap()),
                Span::new(span_start, span_end),
            )),
            Rule::identifier => Expression::Identifier(Identifier::new(
                pair.as_str().to_owned(),
                Span::new(span_start, span_end),
            )),
            Rule::expression => parse_expression(pair, 0),
            _ => {
                dbg!(pair);
                unreachable!()
            }
        }
    }
}

#[inline]
fn parse_number(pair: Pair<Rule>) -> Value {
    let number_string = pair.as_str().replace('_', "");
    if let Ok(result) = number_string.parse::<i64>() {
        Value::Int(result)
    } else {
        Value::Float(number_string.parse::<f64>().unwrap())
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

#[inline]
fn parse_assignment_operator(pair: Pair<Rule>) -> AssignmentOperator {
    match pair.as_rule() {
        Rule::assign => AssignmentOperator::Basic,
        Rule::assign_add => AssignmentOperator::Add,
        Rule::assign_sub => AssignmentOperator::Sub,
        Rule::assign_mul => AssignmentOperator::Mul,
        Rule::assign_div => AssignmentOperator::Div,
        Rule::assign_mod => AssignmentOperator::Mod,
        _ => unreachable!(),
    }
}
