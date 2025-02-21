mod parser;

use crate::opcode::Opcode;
use parser::{ASTNode, ASTNodeContent, BinaryOperator, Value};

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
            Register::Int(reg)
            | Register::Uint(reg)
            | Register::Float(reg)
            | Register::Bool(reg)
            | Register::Str(reg)
            | Register::Char(reg) => *reg,
        }
    }

    fn typename(&self) -> String {
        match self {
            Register::Int(_) => "int".to_string(),
            Register::Uint(_) => "uint".to_string(),
            Register::Float(_) => "float".to_string(),
            Register::Bool(_) => "bool".to_string(),
            Register::Str(_) => "string".to_string(),
            Register::Char(_) => "char".to_string(),
        }
    }
}

pub struct Compiler {
    register_stack: Vec<u8>,
    operand_stack: Vec<Operand>,

    source_code: String,
    filename: String,
}

impl Compiler {
    pub fn new() -> Self {
        let register_stack = Vec::with_capacity(256);

        Compiler {
            register_stack,
            operand_stack: Vec::new(),
            source_code: String::new(),
            filename: String::new(),
        }
    }

    fn reset(&mut self) {
        self.operand_stack.clear();
        self.register_stack.clear();
        for i in (0..=255).rev() {
            self.register_stack.push(i);
        }
    }

    pub fn compile(&mut self, source_code: &str, filename: &str) -> Result<Vec<Opcode>, String> {
        let ast = parser::parse(source_code)?;
        let mut bytecode = Vec::new();

        self.reset();
        self.source_code = source_code.to_string();
        self.filename = filename.to_string();
        self.compile_recursive(&ast, &mut bytecode);

        let reg = self.get_register(&mut bytecode);
        bytecode.push(Opcode::Save(reg.value()));

        Ok(bytecode)
    }

    pub fn compile_recursive(&mut self, node: &ASTNode, bytecode: &mut Vec<Opcode>) {
        match node.content() {
            ASTNodeContent::Value(value) => {
                self.operand_stack.push(Operand::Value(value.clone()));
            }
            ASTNodeContent::BinaryOperation(binop) => {
                self.compile_recursive(binop.left(), bytecode);
                self.compile_recursive(binop.right(), bytecode);

                let right_register = self.get_register(bytecode);
                let left_register = self.get_register(bytecode);

                match left_register {
                    Register::Int(left_reg) => match right_register {
                        Register::Int(right_reg) => match binop.operator() {
                            BinaryOperator::Equal => {
                                bytecode.push(Opcode::EqualInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                bytecode.push(Opcode::NotEqualInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessThan => {
                                bytecode.push(Opcode::LessThanInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessEq => {
                                bytecode.push(Opcode::LessEqInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterThan => {
                                bytecode
                                    .push(Opcode::GreaterThanInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterEq => {
                                bytecode.push(Opcode::GreaterEqInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::Add => {
                                bytecode.push(Opcode::AddInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinaryOperator::Sub => {
                                bytecode.push(Opcode::SubInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinaryOperator::Mul => {
                                bytecode.push(Opcode::MulInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinaryOperator::Div => {
                                bytecode.push(Opcode::DivInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinaryOperator::Mod => {
                                bytecode.push(Opcode::ModInt(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Int(right_reg)));
                            }
                            BinaryOperator::Or | BinaryOperator::And => {
                                panic!("Compile time type error")
                            }
                        },
                        _ => {
                            // WIP
                            eprintln!(
                                "In file {}:\n\n{}\n{}{}\nError: Invalid operation '{}' for types '{}' and '{}'",
                                self.filename.as_str(),
                                &self.source_code[node.span_start()..node.span_end()],
                                " ".repeat(binop.operator_span_start() - node.span_start()),
                                "^".repeat(binop.operator_span_end() - binop.operator_span_start()),
                                binop.operator(),
                                left_register.typename(),
                                right_register.typename()
                            );
                            panic!("Compile time type error");
                        }
                    },
                    Register::Uint(left_reg) => match right_register {
                        Register::Uint(right_reg) => match binop.operator() {
                            BinaryOperator::Equal => {
                                bytecode.push(Opcode::EqualUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                bytecode.push(Opcode::NotEqualUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessThan => {
                                bytecode.push(Opcode::LessThanUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessEq => {
                                bytecode.push(Opcode::LessEqUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterThan => {
                                bytecode
                                    .push(Opcode::GreaterThanUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterEq => {
                                bytecode
                                    .push(Opcode::GreaterEqUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::Add => {
                                bytecode.push(Opcode::AddUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Uint(right_reg)));
                            }
                            BinaryOperator::Sub => {
                                bytecode.push(Opcode::SubUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Uint(right_reg)));
                            }
                            BinaryOperator::Mul => {
                                bytecode.push(Opcode::MulUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Uint(right_reg)));
                            }
                            BinaryOperator::Div => {
                                bytecode.push(Opcode::DivUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Uint(right_reg)));
                            }
                            BinaryOperator::Mod => {
                                bytecode.push(Opcode::ModUint(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Uint(right_reg)));
                            }
                            BinaryOperator::Or | BinaryOperator::And => {
                                panic!("Compile time type error")
                            }
                        },
                        Register::Str(right_reg) => match binop.operator() {
                            BinaryOperator::Mul => {
                                bytecode.push(Opcode::MulStr(right_reg, left_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Str(right_reg)));
                            }
                            _ => panic!("Compile time type error"),
                        },
                        _ => panic!("Compile time type error"),
                    },
                    Register::Float(left_reg) => match right_register {
                        Register::Float(right_reg) => match binop.operator() {
                            BinaryOperator::Equal => {
                                bytecode.push(Opcode::EqualFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                bytecode
                                    .push(Opcode::NotEqualFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessThan => {
                                bytecode
                                    .push(Opcode::LessThanFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessEq => {
                                bytecode.push(Opcode::LessEqFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterThan => {
                                bytecode
                                    .push(Opcode::GreaterThanFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterEq => {
                                bytecode
                                    .push(Opcode::GreaterEqFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::Add => {
                                bytecode.push(Opcode::AddFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinaryOperator::Sub => {
                                bytecode.push(Opcode::SubFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinaryOperator::Mul => {
                                bytecode.push(Opcode::MulFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinaryOperator::Div => {
                                bytecode.push(Opcode::DivFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinaryOperator::Mod => {
                                bytecode.push(Opcode::ModFloat(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Float(right_reg)));
                            }
                            BinaryOperator::Or | BinaryOperator::And => {
                                panic!("Compile time type error")
                            }
                        },
                        _ => panic!("Compile time type error"),
                    },
                    Register::Bool(left_reg) => match right_register {
                        Register::Bool(right_reg) => match binop.operator() {
                            BinaryOperator::Or => {
                                bytecode.push(Opcode::Or(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::And => {
                                bytecode.push(Opcode::And(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::Equal => {
                                bytecode.push(Opcode::EqualBool(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                bytecode.push(Opcode::NotEqualBool(left_reg, right_reg, right_reg));
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
                            | BinaryOperator::Mod => panic!("Compile time type error"),
                        },
                        _ => panic!("Compile time type error"),
                    },
                    Register::Str(left_reg) => match right_register {
                        Register::Str(right_reg) => match binop.operator() {
                            BinaryOperator::Equal => {
                                bytecode.push(Opcode::EqualStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                bytecode.push(Opcode::NotEqualStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessThan => {
                                bytecode.push(Opcode::LessThanStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessEq => {
                                bytecode.push(Opcode::LessEqStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterThan => {
                                bytecode
                                    .push(Opcode::GreaterThanStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterEq => {
                                bytecode.push(Opcode::GreaterEqStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::Add => {
                                bytecode.push(Opcode::AddStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Str(right_reg)));
                            }
                            BinaryOperator::Or
                            | BinaryOperator::And
                            | BinaryOperator::Sub
                            | BinaryOperator::Mul
                            | BinaryOperator::Div
                            | BinaryOperator::Mod => panic!("Compile time type error"),
                        },
                        Register::Uint(right_reg) => match binop.operator() {
                            BinaryOperator::Mul => {
                                bytecode.push(Opcode::MulStr(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Str(right_reg)));
                            }
                            _ => panic!("Compile time type error"),
                        },
                        _ => panic!("Compile time type error"),
                    },
                    Register::Char(left_reg) => match right_register {
                        Register::Char(right_reg) => match binop.operator() {
                            BinaryOperator::Equal => {
                                bytecode.push(Opcode::EqualChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::NotEqual => {
                                bytecode.push(Opcode::NotEqualChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessThan => {
                                bytecode.push(Opcode::LessThanChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::LessEq => {
                                bytecode.push(Opcode::LessEqChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterThan => {
                                bytecode
                                    .push(Opcode::GreaterThanChar(left_reg, right_reg, right_reg));
                                self.operand_stack
                                    .push(Operand::Register(Register::Bool(right_reg)));
                            }
                            BinaryOperator::GreaterEq => {
                                bytecode
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
                            | BinaryOperator::Mod => panic!("Compile time type error"),
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
