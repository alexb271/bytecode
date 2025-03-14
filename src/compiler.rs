mod error;
mod language_components;
mod parser;

use crate::opcode::Opcode;
use error::{Error, ErrorKind};
use language_components::*;

use std::collections::HashMap;

const KEYWORDS: [&'static str; 2] = ["let", "return"];

#[derive(Debug)]
enum Operand {
    Value(Value),
    Register(Register),
}

#[derive(Clone, Copy, Debug)]
struct Register {
    value: u8,
    data_type: DataType,
    is_temporary: bool,
}

impl Register {
    fn new(value: u8, data_type: DataType, is_temporary: bool) -> Register {
        Register {
            value,
            data_type,
            is_temporary,
        }
    }
}

#[derive(Debug)]
pub struct Compiler {
    filename: String,
    source_code: String,

    register_stack: Vec<u8>,
    operand_stack: Vec<Operand>,
    variables: HashMap<String, Register>,
    bytecode: Vec<Opcode>,
}

impl Compiler {
    pub fn new() -> Self {
        let register_stack = Vec::with_capacity(256);

        Compiler {
            filename: String::new(),
            source_code: String::new(),
            register_stack,
            operand_stack: Vec::new(),
            variables: HashMap::new(),
            bytecode: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.register_stack.clear();
        for i in (0..=255).rev() {
            self.register_stack.push(i);
        }
        self.operand_stack.clear();
        self.variables.clear();
        self.bytecode.clear();
    }

    pub fn compile(&mut self, source_code: &str, filename: &str) -> Result<Vec<Opcode>, String> {
        let function_body = parser::parse(source_code)?;

        self.reset();
        self.source_code = source_code.to_owned();
        self.filename = filename.to_owned();
        for control_flow in function_body.control_flow_structures() {
            match self.compile_control_flow(control_flow) {
                Ok(()) => (),
                Err(e) => {
                    return Err(e.to_string());
                }
            }
        }
        let result = std::mem::take(&mut self.bytecode);
        Ok(result)
    }

    fn compile_control_flow(&mut self, control_flow: &ControlFlow) -> Result<(), Error> {
        match control_flow {
            ControlFlow::WhileLoop(while_loop) => self.compile_while_loop(while_loop)?,
            ControlFlow::BasicBlock(basic_block) => self.compile_basic_block(basic_block)?,
        }
        Ok(())
    }

    fn compile_while_loop(&mut self, while_loop: &WhileLoop) -> Result<(), Error> {
        let start_idx = self.bytecode.len();
        self.compile_expression(while_loop.condition(), None)?;
        let result_register = self.get_register();
        if result_register.data_type != DataType::Bool {
            return Err(Error::new(
                self.filename.clone(),
                self.source_code.clone(),
                ErrorKind::ArgumentInvalidType(
                    DataType::Bool.typename(),
                    result_register.data_type.typename(),
                ),
                while_loop.span(),
                while_loop.condition().span(),
            ));
        }

        let conditional_jump_opcode_idx = self.bytecode.len();
        // Placeholder to be replaced later when we know the actual index where the loop ends
        self.bytecode.push(Opcode::Error);

        for statement in while_loop.body().statements() {
            self.compile_statement(statement)?;
        }
        // We are inside the loop, we need to jump backwards to before the conditional expression.
        // The +1 is needed because the program counter is expected to increment after each operation as well,
        // including this jump, so in thought we have to be one operation ahead already.
        let offset = self.bytecode.len() - start_idx + 1;
        if offset > i16::MAX as usize {
            panic!("Jump offset cannot fit into i16");
        }
        let offset = offset as i16 * -1;
        self.bytecode.push(Opcode::Jump(offset));

        // jump to after the loop is over in case the conditional expression evaluates to 'false'
        // the -1 is needed because the program counter is expected to increment after each operation as well
        let offset = self.bytecode.len() - conditional_jump_opcode_idx - 1;
        if offset > i16::MAX as usize {
            panic!("Jump offset cannot fit into i16");
        }
        self.bytecode[conditional_jump_opcode_idx] =
            Opcode::JumpCond(result_register.value, offset as i16);

        Ok(())
    }

    fn compile_basic_block(&mut self, basic_block: &BasicBlock) -> Result<(), Error> {
        for statement in basic_block.statements() {
            self.compile_statement(statement)?
        }
        Ok(())
    }

    fn compile_statement(&mut self, statement: &Statement) -> Result<(), Error> {
        match statement {
            Statement::LetStatement(let_statement) => self.compile_let_statement(let_statement),
            Statement::Assignment(assignemnt) => self.compile_assignment(assignemnt),
            Statement::ReturnStatement(return_statement) => {
                self.compile_return_statement(return_statement)
            }
            Statement::Expression(expression) => {
                self.compile_expression(expression, None)?;

                // Discard the value of standalone expressions and release the register holding it
                let result = self.get_register();
                self.register_stack.push(result.value);
                Ok(())
            }
        }
    }

    fn compile_let_statement(&mut self, let_statement: &LetStatement) -> Result<(), Error> {
        if Self::is_keyword(let_statement.identifier().name()) {
            return Err(Error::new(
                self.filename.clone(),
                self.source_code.clone(),
                ErrorKind::IdentifierIsKeyword,
                let_statement.span(),
                let_statement.identifier().span(),
            ));
        }
        self.compile_expression(let_statement.expression(), None)?;
        let mut result_register = self.get_register();
        result_register.is_temporary = false;
        self.variables.insert(
            let_statement.identifier().name().to_owned(),
            result_register,
        );
        Ok(())
    }

    fn compile_assignment(&mut self, assignment: &Assignment) -> Result<(), Error> {
        let lhs_reg = match self.variables.get(assignment.lhs().name()) {
            Some(reg) => reg,
            None => {
                return Err(
                    self.new_identifier_not_found_error(assignment.lhs(), assignment.span())
                );
            }
        };

        match assignment.rhs() {
            Expression::Literal(literal) => {
                if lhs_reg.data_type == literal.value().data_type() {
                    match literal.value().clone() {
                        Value::Int(val) => {
                            self.bytecode.push(Opcode::LoadInt(lhs_reg.value, val));
                        }
                        Value::Float(val) => {
                            self.bytecode.push(Opcode::LoadFloat(lhs_reg.value, val));
                        }
                        Value::Bool(val) => {
                            self.bytecode.push(Opcode::LoadBool(lhs_reg.value, val));
                        }
                        Value::Str(val) => {
                            self.bytecode.push(Opcode::LoadStr(lhs_reg.value, val));
                        }
                        Value::Char(val) => {
                            self.bytecode.push(Opcode::LoadChar(lhs_reg.value, val));
                        }
                    }
                } else {
                    return Err(self.new_invalid_assignment_error(
                        lhs_reg.data_type.typename(),
                        literal.value().data_type().typename(),
                        assignment.span(),
                        assignment.operator_span(),
                    ));
                }
            }
            Expression::Identifier(identifier) => {
                let rhs_reg = match self.variables.get(identifier.name()) {
                    Some(reg) => reg,
                    None => {
                        return Err(
                            self.new_identifier_not_found_error(identifier, assignment.span())
                        );
                    }
                };

                if lhs_reg.data_type == rhs_reg.data_type {
                    self.bytecode
                        .push(Opcode::Copy(rhs_reg.value, lhs_reg.value));
                } else {
                    return Err(self.new_invalid_assignment_error(
                        lhs_reg.data_type.typename(),
                        rhs_reg.data_type.typename(),
                        assignment.span(),
                        assignment.operator_span(),
                    ));
                }
            }
            Expression::BinaryOperation(_) | Expression::UnaryOperation(_) => {
                let lhs_data_type = lhs_reg.data_type;
                self.compile_expression(assignment.rhs(), Some(lhs_reg.value))?;
                let expression_result = self.get_register();

                if lhs_data_type != expression_result.data_type {
                    return Err(self.new_invalid_assignment_error(
                        lhs_data_type.typename(),
                        expression_result.data_type.typename(),
                        assignment.span(),
                        assignment.operator_span(),
                    ));
                }
            }
        }
        Ok(())
    }

    fn compile_return_statement(
        &mut self,
        return_statement: &ReturnStatement,
    ) -> Result<(), Error> {
        self.compile_expression(return_statement.expression(), None)?;
        let result_register = self.get_register();
        self.bytecode.push(Opcode::Save(result_register.value));
        Ok(())
    }

    #[inline]
    fn get_register(&mut self) -> Register {
        match self.operand_stack.pop().unwrap() {
            Operand::Register(reg) => reg,
            Operand::Value(value) => {
                let reg = self.register_stack.pop().expect("Ran out of registers");
                match value {
                    Value::Int(val) => {
                        self.bytecode.push(Opcode::LoadInt(reg, val));
                        Register::new(reg, DataType::Int, true)
                    }
                    Value::Float(val) => {
                        self.bytecode.push(Opcode::LoadFloat(reg, val));
                        Register::new(reg, DataType::Float, true)
                    }
                    Value::Bool(val) => {
                        self.bytecode.push(Opcode::LoadBool(reg, val));
                        Register::new(reg, DataType::Bool, true)
                    }
                    Value::Str(val) => {
                        self.bytecode.push(Opcode::LoadStr(reg, val));
                        Register::new(reg, DataType::Str, true)
                    }
                    Value::Char(val) => {
                        self.bytecode.push(Opcode::LoadChar(reg, val));
                        Register::new(reg, DataType::Char, true)
                    }
                }
            }
        }
    }

    pub fn compile_expression(
        &mut self,
        expression: &Expression,
        target_register: Option<u8>,
    ) -> Result<(), Error> {
        match expression {
            Expression::Literal(literal) => {
                self.operand_stack
                    .push(Operand::Value(literal.value().clone()));
                Ok(())
            }
            Expression::Identifier(identifier) => {
                let register = match self.variables.get(identifier.name()) {
                    Some(register) => register,
                    None => {
                        return Err(
                            self.new_identifier_not_found_error(identifier, expression.span())
                        );
                    }
                };
                self.operand_stack.push(Operand::Register(*register));
                Ok(())
            }
            Expression::BinaryOperation(binop) => {
                self.compile_expression(binop.left(), target_register)?;
                self.compile_expression(binop.right(), target_register)?;

                let right_register = self.get_register();
                let left_register = self.get_register();
                let target_register = match target_register {
                    Some(reg) => reg,
                    None => {
                        if right_register.is_temporary {
                            right_register.value
                        } else if left_register.is_temporary {
                            left_register.value
                        } else {
                            self.register_stack.pop().expect("Ran out of registers")
                        }
                    }
                };

                match left_register.data_type {
                    DataType::Int => match right_register.data_type {
                        DataType::Int => match binop.operator() {
                            BinaryOperator::Equal => {
                                self.bytecode.push(Opcode::EqualInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::NotEqual => {
                                self.bytecode.push(Opcode::NotEqualInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::LessThan => {
                                self.bytecode.push(Opcode::LessThanInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::LessEq => {
                                self.bytecode.push(Opcode::LessEqInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::GreaterThan => {
                                self.bytecode.push(Opcode::GreaterThanInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::GreaterEq => {
                                self.bytecode.push(Opcode::GreaterEqInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::Add => {
                                self.bytecode.push(Opcode::AddInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Int,
                                    true,
                                )));
                            }
                            BinaryOperator::Sub => {
                                self.bytecode.push(Opcode::SubInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Int,
                                    true,
                                )));
                            }
                            BinaryOperator::Mul => {
                                self.bytecode.push(Opcode::MulInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Int,
                                    true,
                                )));
                            }
                            BinaryOperator::Div => {
                                self.bytecode.push(Opcode::DivInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Int,
                                    true,
                                )));
                            }
                            BinaryOperator::Mod => {
                                self.bytecode.push(Opcode::ModInt(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Int,
                                    true,
                                )));
                            }
                            BinaryOperator::Or | BinaryOperator::And => {
                                return Err(self.new_binary_operation_error(
                                    binop,
                                    &right_register,
                                    &left_register,
                                ));
                            }
                        },
                        DataType::Str => match binop.operator() {
                            BinaryOperator::Mul => {
                                self.bytecode.push(Opcode::MulStr(
                                    right_register.value,
                                    left_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Str,
                                    true,
                                )));
                            }
                            _ => {
                                return Err(self.new_binary_operation_error(
                                    binop,
                                    &right_register,
                                    &left_register,
                                ));
                            }
                        },
                        _ => {
                            return Err(self.new_binary_operation_error(
                                binop,
                                &right_register,
                                &left_register,
                            ));
                        }
                    },
                    DataType::Float => match right_register.data_type {
                        DataType::Float => match binop.operator() {
                            BinaryOperator::Equal => {
                                self.bytecode.push(Opcode::EqualFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::NotEqual => {
                                self.bytecode.push(Opcode::NotEqualFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::LessThan => {
                                self.bytecode.push(Opcode::LessThanFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::LessEq => {
                                self.bytecode.push(Opcode::LessEqFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::GreaterThan => {
                                self.bytecode.push(Opcode::GreaterThanFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::GreaterEq => {
                                self.bytecode.push(Opcode::GreaterEqFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::Add => {
                                self.bytecode.push(Opcode::AddFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Float,
                                    true,
                                )));
                            }
                            BinaryOperator::Sub => {
                                self.bytecode.push(Opcode::SubFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Float,
                                    true,
                                )));
                            }
                            BinaryOperator::Mul => {
                                self.bytecode.push(Opcode::MulFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Float,
                                    true,
                                )));
                            }
                            BinaryOperator::Div => {
                                self.bytecode.push(Opcode::DivFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Float,
                                    true,
                                )));
                            }
                            BinaryOperator::Mod => {
                                self.bytecode.push(Opcode::ModFloat(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Float,
                                    true,
                                )));
                            }
                            BinaryOperator::Or | BinaryOperator::And => {
                                return Err(self.new_binary_operation_error(
                                    binop,
                                    &right_register,
                                    &left_register,
                                ));
                            }
                        },
                        _ => {
                            return Err(self.new_binary_operation_error(
                                binop,
                                &right_register,
                                &left_register,
                            ));
                        }
                    },
                    DataType::Bool => match right_register.data_type {
                        DataType::Bool => match binop.operator() {
                            BinaryOperator::Or => {
                                self.bytecode.push(Opcode::Or(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::And => {
                                self.bytecode.push(Opcode::And(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::Equal => {
                                self.bytecode.push(Opcode::EqualBool(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::NotEqual => {
                                self.bytecode.push(Opcode::NotEqualBool(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::LessThan
                            | BinaryOperator::LessEq
                            | BinaryOperator::GreaterThan
                            | BinaryOperator::GreaterEq
                            | BinaryOperator::Add
                            | BinaryOperator::Sub
                            | BinaryOperator::Mul
                            | BinaryOperator::Div
                            | BinaryOperator::Mod => {
                                return Err(self.new_binary_operation_error(
                                    binop,
                                    &right_register,
                                    &left_register,
                                ));
                            }
                        },
                        _ => {
                            return Err(self.new_binary_operation_error(
                                binop,
                                &right_register,
                                &left_register,
                            ));
                        }
                    },
                    DataType::Str => match right_register.data_type {
                        DataType::Str => match binop.operator() {
                            BinaryOperator::Equal => {
                                self.bytecode.push(Opcode::EqualStr(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::NotEqual => {
                                self.bytecode.push(Opcode::NotEqualStr(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::LessThan => {
                                self.bytecode.push(Opcode::LessThanStr(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::LessEq => {
                                self.bytecode.push(Opcode::LessEqStr(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::GreaterThan => {
                                self.bytecode.push(Opcode::GreaterThanStr(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::GreaterEq => {
                                self.bytecode.push(Opcode::GreaterEqStr(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::Add => {
                                self.bytecode.push(Opcode::AddStr(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Str,
                                    true,
                                )));
                            }
                            BinaryOperator::Or
                            | BinaryOperator::And
                            | BinaryOperator::Sub
                            | BinaryOperator::Mul
                            | BinaryOperator::Div
                            | BinaryOperator::Mod => {
                                return Err(self.new_binary_operation_error(
                                    binop,
                                    &right_register,
                                    &left_register,
                                ));
                            }
                        },
                        DataType::Int => match binop.operator() {
                            BinaryOperator::Mul => {
                                self.bytecode.push(Opcode::MulStr(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Str,
                                    true,
                                )));
                            }
                            _ => {
                                return Err(self.new_binary_operation_error(
                                    binop,
                                    &right_register,
                                    &left_register,
                                ));
                            }
                        },
                        _ => {
                            return Err(self.new_binary_operation_error(
                                binop,
                                &right_register,
                                &left_register,
                            ));
                        }
                    },
                    DataType::Char => match right_register.data_type {
                        DataType::Char => match binop.operator() {
                            BinaryOperator::Equal => {
                                self.bytecode.push(Opcode::EqualChar(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::NotEqual => {
                                self.bytecode.push(Opcode::NotEqualChar(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::LessThan => {
                                self.bytecode.push(Opcode::LessThanChar(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::LessEq => {
                                self.bytecode.push(Opcode::LessEqChar(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::GreaterThan => {
                                self.bytecode.push(Opcode::GreaterThanChar(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::GreaterEq => {
                                self.bytecode.push(Opcode::GreaterEqChar(
                                    left_register.value,
                                    right_register.value,
                                    target_register,
                                ));
                                self.operand_stack.push(Operand::Register(Register::new(
                                    target_register,
                                    DataType::Bool,
                                    true,
                                )));
                            }
                            BinaryOperator::Or
                            | BinaryOperator::And
                            | BinaryOperator::Add
                            | BinaryOperator::Sub
                            | BinaryOperator::Mul
                            | BinaryOperator::Div
                            | BinaryOperator::Mod => {
                                return Err(self.new_binary_operation_error(
                                    binop,
                                    &right_register,
                                    &left_register,
                                ));
                            }
                        },
                        _ => {
                            return Err(self.new_binary_operation_error(
                                binop,
                                &right_register,
                                &left_register,
                            ));
                        }
                    },
                }

                if left_register.is_temporary && left_register.value != target_register {
                    self.register_stack.push(left_register.value);
                }
                if right_register.is_temporary && right_register.value != target_register {
                    self.register_stack.push(right_register.value);
                }

                Ok(())
            }
            Expression::UnaryOperation(unop) => {
                self.compile_expression(unop.operand(), target_register)?;
                let register = self.get_register();
                let target_register = match target_register {
                    Some(reg) => reg,
                    None => {
                        if register.is_temporary {
                            register.value
                        } else {
                            self.register_stack.pop().expect("Ran out of registers")
                        }
                    }
                };

                match register.data_type {
                    DataType::Int => match unop.operator() {
                        UnaryOperator::Neg => {
                            self.bytecode
                                .push(Opcode::NegInt(register.value, target_register));
                            self.operand_stack.push(Operand::Register(Register::new(
                                target_register,
                                DataType::Int,
                                true,
                            )));
                        }
                        UnaryOperator::Not => {
                            return Err(self.new_unary_operation_error(unop, &register));
                        }
                    },
                    DataType::Float => match unop.operator() {
                        UnaryOperator::Neg => {
                            self.bytecode
                                .push(Opcode::NegFloat(register.value, target_register));
                            self.operand_stack.push(Operand::Register(Register::new(
                                target_register,
                                DataType::Float,
                                true,
                            )));
                        }
                        UnaryOperator::Not => {
                            return Err(self.new_unary_operation_error(unop, &register));
                        }
                    },
                    DataType::Bool => match unop.operator() {
                        UnaryOperator::Not => {
                            self.bytecode
                                .push(Opcode::NegBool(register.value, target_register));
                            self.operand_stack.push(Operand::Register(Register::new(
                                target_register,
                                DataType::Bool,
                                true,
                            )));
                        }
                        UnaryOperator::Neg => {
                            return Err(self.new_unary_operation_error(unop, &register));
                        }
                    },
                    _ => return Err(self.new_unary_operation_error(unop, &register)),
                }

                if register.is_temporary && register.value != target_register {
                    self.register_stack.push(register.value);
                }
                Ok(())
            }
        }
    }

    #[inline]
    fn new_binary_operation_error(
        &self,
        binop: &BinaryOperation,
        right_register: &Register,
        left_register: &Register,
    ) -> Error {
        Error::new(
            self.filename.clone(),
            self.source_code.clone(),
            ErrorKind::InvalidBinaryOperation(
                binop.operator(),
                left_register.data_type.typename(),
                right_register.data_type.typename(),
            ),
            binop.left().span(),
            binop.operator_span(),
        )
    }

    #[inline]
    fn new_unary_operation_error(&self, unop: &UnaryOperation, register: &Register) -> Error {
        Error::new(
            self.filename.clone(),
            self.source_code.clone(),
            ErrorKind::InvalidUnaryOperation(unop.operator(), register.data_type.typename()),
            unop.span(),
            unop.operator_span(),
        )
    }

    #[inline]
    fn new_identifier_not_found_error(&self, identifier: &Identifier, context: Span) -> Error {
        Error::new(
            self.filename.clone(),
            self.source_code.clone(),
            ErrorKind::IdentifierNotFound,
            context,
            identifier.span(),
        )
    }

    #[inline]
    fn new_invalid_assignment_error(
        &self,
        lhs_type: String,
        rhs_type: String,
        context: Span,
        error: Span,
    ) -> Error {
        Error::new(
            self.filename.clone(),
            self.source_code.clone(),
            ErrorKind::InvalidAssignment(lhs_type, rhs_type),
            context,
            error,
        )
    }

    #[inline]
    fn is_keyword(input: &str) -> bool {
        for keyword in KEYWORDS {
            if input == keyword {
                return true;
            }
        }
        return false;
    }
}
