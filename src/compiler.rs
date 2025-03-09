mod error;
mod parser;

use crate::opcode::Opcode;
use error::{Error, ErrorKind};
use parser::{
    ASTNode, ASTNodeContent, BinaryOperation, BinaryOperator, UnaryOperation, UnaryOperator, Value,
};

enum Operand {
    Value(Value),
    Register(Register),
}

enum Register {
    Int(u8),
    Float(u8),
    Bool(u8),
    Str(u8),
    Char(u8),
}

impl Register {
    fn value(&self) -> u8 {
        match self {
            Register::Int(reg)
            | Register::Float(reg)
            | Register::Bool(reg)
            | Register::Str(reg)
            | Register::Char(reg) => *reg,
        }
    }

    fn typename(&self) -> String {
        match self {
            Register::Int(_) => "int".to_string(),
            Register::Float(_) => "float".to_string(),
            Register::Bool(_) => "bool".to_string(),
            Register::Str(_) => "string".to_string(),
            Register::Char(_) => "char".to_string(),
        }
    }
}

pub struct Compiler {
    filename: String,
    source_code: String,

    register_stack: Vec<u8>,
    operand_stack: Vec<Operand>,
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
            bytecode: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.register_stack.clear();
        for i in (0..=255).rev() {
            self.register_stack.push(i);
        }
        self.operand_stack.clear();
        self.bytecode.clear();
    }

    pub fn compile(&mut self, source_code: &str, filename: &str) -> Result<Vec<Opcode>, String> {
        let ast = parser::parse(source_code)?;

        self.reset();
        self.source_code = source_code.to_owned();
        self.filename = filename.to_owned();
        match self.compile_recursive(&ast) {
            Ok(()) => {
                let reg = self.get_register();
                self.bytecode.push(Opcode::Save(reg.value()));

                let result = std::mem::take(&mut self.bytecode);
                Ok(result)
            }
            Err(e) => Err(e.to_string()),
        }
    }

    pub fn compile_recursive(&mut self, node: &ASTNode) -> Result<(), Error> {
        match node.content() {
            ASTNodeContent::Value(value) => {
                self.operand_stack.push(Operand::Value(value.clone()));
                Ok(())
            }
            ASTNodeContent::BinaryOperation(binop) => {
                self.compile_recursive(binop.left())?;
                self.compile_recursive(binop.right())?;

                let right_register = self.get_register();
                let left_register = self.get_register();

                match left_register {
                    Register::Int(left_reg) => match right_register {
                        Register::Int(right_reg) => match binop.operator() {
                            BinaryOperator::Equal => {
                                self.bytecode
                                    .push(Opcode::EqualInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                self.bytecode
                                    .push(Opcode::NotEqualInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessThan => {
                                self.bytecode
                                    .push(Opcode::LessThanInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessEq => {
                                self.bytecode
                                    .push(Opcode::LessEqInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterThan => {
                                self.bytecode
                                    .push(Opcode::GreaterThanInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterEq => {
                                self.bytecode
                                    .push(Opcode::GreaterEqInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::Add => {
                                self.bytecode
                                    .push(Opcode::AddInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinaryOperator::Sub => {
                                self.bytecode
                                    .push(Opcode::SubInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinaryOperator::Mul => {
                                self.bytecode
                                    .push(Opcode::MulInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinaryOperator::Div => {
                                self.bytecode
                                    .push(Opcode::DivInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinaryOperator::Mod => {
                                self.bytecode
                                    .push(Opcode::ModInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinaryOperator::Or | BinaryOperator::And => {
                                return Err(self.new_binary_operation_error(
                                    binop,
                                    &right_register,
                                    &left_register,
                                ));
                            }
                        },
                        Register::Str(right_reg) => match binop.operator() {
                            BinaryOperator::Mul => {
                                self.bytecode
                                    .push(Opcode::MulStr(right_reg, left_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Str(right_reg)));
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
                    Register::Float(left_reg) => match right_register {
                        Register::Float(right_reg) => match binop.operator() {
                            BinaryOperator::Equal => {
                                self.bytecode
                                    .push(Opcode::EqualFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                self.bytecode
                                    .push(Opcode::NotEqualFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessThan => {
                                self.bytecode
                                    .push(Opcode::LessThanFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessEq => {
                                self.bytecode
                                    .push(Opcode::LessEqFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterThan => {
                                self.bytecode
                                    .push(Opcode::GreaterThanFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterEq => {
                                self.bytecode
                                    .push(Opcode::GreaterEqFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::Add => {
                                self.bytecode
                                    .push(Opcode::AddFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinaryOperator::Sub => {
                                self.bytecode
                                    .push(Opcode::SubFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinaryOperator::Mul => {
                                self.bytecode
                                    .push(Opcode::MulFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinaryOperator::Div => {
                                self.bytecode
                                    .push(Opcode::DivFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinaryOperator::Mod => {
                                self.bytecode
                                    .push(Opcode::ModFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
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
                    Register::Bool(left_reg) => match right_register {
                        Register::Bool(right_reg) => match binop.operator() {
                            BinaryOperator::Or => {
                                self.bytecode
                                    .push(Opcode::Or(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::And => {
                                self.bytecode
                                    .push(Opcode::And(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::Equal => {
                                self.bytecode
                                    .push(Opcode::EqualBool(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                self.bytecode
                                    .push(Opcode::NotEqualBool(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
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
                    Register::Str(left_reg) => match right_register {
                        Register::Str(right_reg) => match binop.operator() {
                            BinaryOperator::Equal => {
                                self.bytecode
                                    .push(Opcode::EqualStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                self.bytecode
                                    .push(Opcode::NotEqualStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessThan => {
                                self.bytecode
                                    .push(Opcode::LessThanStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessEq => {
                                self.bytecode
                                    .push(Opcode::LessEqStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterThan => {
                                self.bytecode
                                    .push(Opcode::GreaterThanStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterEq => {
                                self.bytecode
                                    .push(Opcode::GreaterEqStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::Add => {
                                self.bytecode
                                    .push(Opcode::AddStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Str(right_reg)));
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
                        Register::Int(right_reg) => match binop.operator() {
                            BinaryOperator::Mul => {
                                self.bytecode
                                    .push(Opcode::MulStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Str(right_reg)));
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
                    Register::Char(left_reg) => match right_register {
                        Register::Char(right_reg) => match binop.operator() {
                            BinaryOperator::Equal => {
                                self.bytecode
                                    .push(Opcode::EqualChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                self.bytecode
                                    .push(Opcode::NotEqualChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessThan => {
                                self.bytecode
                                    .push(Opcode::LessThanChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessEq => {
                                self.bytecode
                                    .push(Opcode::LessEqChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterThan => {
                                self.bytecode
                                    .push(Opcode::GreaterThanChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterEq => {
                                self.bytecode
                                    .push(Opcode::GreaterEqChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
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

                self.register_stack.push(left_register.value());
                Ok(())
            }
            ASTNodeContent::UnaryOperation(unop) => {
                self.compile_recursive(unop.operand())?;
                let register = self.get_register();

                match register {
                    Register::Int(reg)  => match unop.operator() {
                        UnaryOperator::Neg => {
                            self.bytecode.push(Opcode::NegInt(reg, reg));
                            self.operand_stack.push(Operand::Register(Register::Int(reg)));
                        }
                        UnaryOperator::Not => {
                            return Err(self.new_unary_operation_error(unop, &register));
                        }
                    }
                    Register::Float(reg)  => match unop.operator() {
                        UnaryOperator::Neg => {
                            self.bytecode.push(Opcode::NegFloat(reg, reg));
                            self.operand_stack.push(Operand::Register(Register::Float(reg)));
                        }
                        UnaryOperator::Not => {
                            return Err(self.new_unary_operation_error(unop, &register));
                        }
                    }
                    Register::Bool(reg) => match unop.operator() {
                        UnaryOperator::Not => {
                            self.bytecode.push(Opcode::NegBool(reg, reg));
                            self.operand_stack
                                .push(Operand::Register(Register::Bool(reg)));
                        }
                        UnaryOperator::Neg => {
                            return Err(self.new_unary_operation_error(unop, &register));
                        }
                    },
                    _ => return Err(self.new_unary_operation_error(unop, &register)),
                }

                Ok(())
            }
        }
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
                        Register::Int(reg)
                    }
                    Value::Float(val) => {
                        self.bytecode.push(Opcode::LoadFloat(reg, val));
                        Register::Float(reg)
                    }
                    Value::Bool(val) => {
                        self.bytecode.push(Opcode::LoadBool(reg, val));
                        Register::Bool(reg)
                    }
                    Value::Str(val) => {
                        self.bytecode.push(Opcode::LoadStr(reg, val));
                        Register::Str(reg)
                    }
                    Value::Char(val) => {
                        self.bytecode.push(Opcode::LoadChar(reg, val));
                        Register::Char(reg)
                    }
                }
            }
        }
    }

    #[inline]
    fn new_binary_operation_error(
        &mut self,
        binop: &BinaryOperation,
        right_register: &Register,
        left_register: &Register,
    ) -> Error {
        Error::new(
            std::mem::take(&mut self.filename),
            std::mem::take(&mut self.source_code),
            ErrorKind::InvalidBinaryOperation(
                binop.operator(),
                left_register.typename(),
                right_register.typename(),
            ),
            binop.left().span_start(),
            binop.right().span_end(),
            binop.operator_span_start(),
            binop.operator_span_end(),
        )
    }

    #[inline]
    fn new_unary_operation_error(&mut self, unop: &UnaryOperation, register: &Register) -> Error {
        Error::new(
            std::mem::take(&mut self.filename),
            std::mem::take(&mut self.source_code),
            ErrorKind::InvalidUnaryOperation(unop.operator(), register.typename()),
            unop.operator_span_start(),
            unop.operand().span_end(),
            unop.operator_span_start(),
            unop.operator_span_end(),
        )
    }
}
