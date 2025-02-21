mod parser;

use crate::opcode::Opcode;
use parser::{ASTNode, BinOperator, Value};

pub struct Compiler {
    register_stack: Vec<u8>,
    operand_stack: Vec<Operand>,
}

impl Compiler {
    pub fn new() -> Self {
        let register_stack = Vec::with_capacity(256);

        Compiler {
            register_stack,
            operand_stack: Vec::new(),
        }
    }

    fn reset(&mut self) {
        self.operand_stack.clear();
        self.register_stack.clear();
        for i in (0..=255).rev() {
            self.register_stack.push(i);
        }
    }

    pub fn compile(&mut self, source_code: &str, _filename: &str) -> Result<Vec<Opcode>, String> {
        let ast = parser::parse(source_code)?;
        let mut bytecode = Vec::new();
        self.reset();
        self.compile_recursive(&ast, &mut bytecode);

        let reg = self.get_register(&mut bytecode);
        bytecode.push(Opcode::Save(reg.value()));

        Ok(bytecode)
    }

    pub fn compile_recursive(&mut self, node: &ASTNode, bytecode: &mut Vec<Opcode>) {
        match node {
            ASTNode::Value(value) => {
                self.operand_stack.push(Operand::Value(value.clone()));
            }
            ASTNode::BinaryOp(left, operator, right) => {
                self.compile_recursive(left, bytecode);
                self.compile_recursive(right, bytecode);

                let right_register = self.get_register(bytecode);
                let left_register = self.get_register(bytecode);

                match left_register {
                    Register::Int(left_reg) => match right_register {
                        Register::Int(right_reg) => match operator {
                            BinOperator::Equal => {
                                bytecode.push(Opcode::EqualInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::NotEqual => {
                                bytecode.push(Opcode::NotEqualInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessThan => {
                                bytecode.push(Opcode::LessThanInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessEq => {
                                bytecode.push(Opcode::LessEqInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::GreaterThan => {
                                bytecode
                                    .push(Opcode::GreaterThanInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::GreaterEq => {
                                bytecode.push(Opcode::GreaterEqInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::Add => {
                                bytecode.push(Opcode::AddInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinOperator::Sub => {
                                bytecode.push(Opcode::SubInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinOperator::Mul => {
                                bytecode.push(Opcode::MulInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinOperator::Div => {
                                bytecode.push(Opcode::DivInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinOperator::Mod => {
                                bytecode.push(Opcode::ModInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinOperator::Or | BinOperator::And => panic!("Compile time type error"),
                        },
                        _ => panic!("Compile time type error"),
                    },
                    Register::Uint(left_reg) => match right_register {
                        Register::Uint(right_reg) => match operator {
                            BinOperator::Equal => {
                                bytecode.push(Opcode::EqualUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::NotEqual => {
                                bytecode.push(Opcode::NotEqualUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessThan => {
                                bytecode.push(Opcode::LessThanUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessEq => {
                                bytecode.push(Opcode::LessEqUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::GreaterThan => {
                                bytecode
                                    .push(Opcode::GreaterThanUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::GreaterEq => {
                                bytecode
                                    .push(Opcode::GreaterEqUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::Add => {
                                bytecode.push(Opcode::AddUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Uint(right_reg)));
                            }
                            BinOperator::Sub => {
                                bytecode.push(Opcode::SubUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Uint(right_reg)));
                            }
                            BinOperator::Mul => {
                                bytecode.push(Opcode::MulUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Uint(right_reg)));
                            }
                            BinOperator::Div => {
                                bytecode.push(Opcode::DivUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Uint(right_reg)));
                            }
                            BinOperator::Mod => {
                                bytecode.push(Opcode::ModUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Uint(right_reg)));
                            }
                            BinOperator::Or | BinOperator::And => panic!("Compile time type error"),
                        },
                        Register::Str(right_reg) => match operator {
                            BinOperator::Mul => {
                                bytecode.push(Opcode::MulStr(right_reg, left_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Str(right_reg)));
                            }
                            _ => panic!("Compile time type error"),
                        },
                        _ => panic!("Compile time type error"),
                    },
                    Register::Float(left_reg) => match right_register {
                        Register::Float(right_reg) => match operator {
                            BinOperator::Equal => {
                                bytecode.push(Opcode::EqualFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::NotEqual => {
                                bytecode
                                    .push(Opcode::NotEqualFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessThan => {
                                bytecode
                                    .push(Opcode::LessThanFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessEq => {
                                bytecode.push(Opcode::LessEqFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::GreaterThan => {
                                bytecode
                                    .push(Opcode::GreaterThanFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::GreaterEq => {
                                bytecode
                                    .push(Opcode::GreaterEqFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::Add => {
                                bytecode.push(Opcode::AddFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinOperator::Sub => {
                                bytecode.push(Opcode::SubFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinOperator::Mul => {
                                bytecode.push(Opcode::MulFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinOperator::Div => {
                                bytecode.push(Opcode::DivFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinOperator::Mod => {
                                bytecode.push(Opcode::ModFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinOperator::Or | BinOperator::And => panic!("Compile time type error"),
                        },
                        _ => panic!("Compile time type error"),
                    },
                    Register::Bool(left_reg) => match right_register {
                        Register::Bool(right_reg) => match operator {
                            BinOperator::Or => {
                                bytecode.push(Opcode::Or(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::And => {
                                bytecode.push(Opcode::And(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::Equal => {
                                bytecode.push(Opcode::EqualBool(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::NotEqual => {
                                bytecode.push(Opcode::NotEqualBool(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessThan
                            | BinOperator::LessEq
                            | BinOperator::GreaterThan
                            | BinOperator::GreaterEq
                            | BinOperator::Add
                            | BinOperator::Sub
                            | BinOperator::Mul
                            | BinOperator::Div
                            | BinOperator::Mod => panic!("Compile time type error"),
                        },
                        _ => panic!("Compile time type error"),
                    },
                    Register::Str(left_reg) => match right_register {
                        Register::Str(right_reg) => match operator {
                            BinOperator::Equal => {
                                bytecode.push(Opcode::EqualStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::NotEqual => {
                                bytecode.push(Opcode::NotEqualStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessThan => {
                                bytecode.push(Opcode::LessThanStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessEq => {
                                bytecode.push(Opcode::LessEqStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::GreaterThan => {
                                bytecode
                                    .push(Opcode::GreaterThanStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::GreaterEq => {
                                bytecode.push(Opcode::GreaterEqStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::Add => {
                                bytecode.push(Opcode::AddStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Str(right_reg)));
                            }
                            BinOperator::Or
                            | BinOperator::And
                            | BinOperator::Sub
                            | BinOperator::Mul
                            | BinOperator::Div
                            | BinOperator::Mod => panic!("Compile time type error"),
                        },
                        Register::Uint(right_reg) => match operator {
                            BinOperator::Mul => {
                                bytecode.push(Opcode::MulStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Str(right_reg)));
                            }
                            _ => panic!("Compile time type error"),
                        },
                        _ => panic!("Compile time type error"),
                    },
                    Register::Char(left_reg) => match right_register {
                        Register::Char(right_reg) => match operator {
                            BinOperator::Equal => {
                                bytecode.push(Opcode::EqualChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::NotEqual => {
                                bytecode.push(Opcode::NotEqualChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessThan => {
                                bytecode.push(Opcode::LessThanChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::LessEq => {
                                bytecode.push(Opcode::LessEqChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::GreaterThan => {
                                bytecode
                                    .push(Opcode::GreaterThanChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::GreaterEq => {
                                bytecode
                                    .push(Opcode::GreaterEqChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinOperator::Or
                            | BinOperator::And
                            | BinOperator::Add
                            | BinOperator::Sub
                            | BinOperator::Mul
                            | BinOperator::Div
                            | BinOperator::Mod => panic!("Compile time type error"),
                        },
                        _ => panic!("Compile time type error"),
                    },
                }

                self.register_stack.push(left_register.value());
            }
        }
    }

    #[inline]
    fn get_register(&mut self, bytecode: &mut Vec<Opcode>) -> Register {
        match self.operand_stack.pop().unwrap() {
            Operand::Register(reg) => reg,
            Operand::Value(value) => {
                let reg = self.register_stack.pop().expect("Ran out of registers");
                match value {
                    Value::Int(val) => {
                        bytecode.push(Opcode::LoadInt(reg, val));
                        Register::Int(reg)
                    }
                    Value::Uint(val) => {
                        bytecode.push(Opcode::LoadUint(reg, val));
                        Register::Uint(reg)
                    }
                    Value::Float(val) => {
                        bytecode.push(Opcode::LoadFloat(reg, val));
                        Register::Float(reg)
                    }
                    Value::Bool(val) => {
                        bytecode.push(Opcode::LoadBool(reg, val));
                        Register::Bool(reg)
                    }
                    Value::Str(val) => {
                        bytecode.push(Opcode::LoadStr(reg, val));
                        Register::Str(reg)
                    }
                    Value::Char(val) => {
                        bytecode.push(Opcode::LoadChar(reg, val));
                        Register::Char(reg)
                    }
                }
            }
        }
    }
}

enum Operand {
    Value(Value),
    Register(Register),
}

enum Register {
    Int(u8),
    Uint(u8),
    Float(u8),
    Bool(u8),
    Str(u8),
    Char(u8),
}

impl Register {
    fn value(&self) -> u8 {
        match self {
            Register::Int(reg) => *reg,
            Register::Uint(reg) => *reg,
            Register::Float(reg) => *reg,
            Register::Bool(reg) => *reg,
            Register::Str(reg) => *reg,
            Register::Char(reg) => *reg,
        }
    }
}
