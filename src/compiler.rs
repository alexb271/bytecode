use crate::opcode::{Opcode, Word};
use crate::parser::{ASTNode, BinOperator};

pub struct Compiler {
    register_stack: Vec<u8>,
    stack: Vec<StackElement>,
}

impl Compiler {
    pub fn new() -> Self {
        let mut register_stack = Vec::with_capacity(256);
        for i in (0..=255).rev() {
            register_stack.push(i);
        }

        Compiler {
            register_stack,
            stack: Vec::new(),
        }
    }

    pub fn compile(mut self, ast: &ASTNode) -> Vec<Opcode> {
        let mut bytecode = Vec::new();
        self.compile_recursive(ast, &mut bytecode);

        let reg = self.get_register(&mut bytecode);
        bytecode.push(Opcode::Print(reg));

        bytecode
    }

    pub fn compile_recursive(&mut self, node: &ASTNode, bytecode: &mut Vec<Opcode>) {
        match node {
            ASTNode::Number(value) => {
                self.stack.push(StackElement::Value(*value));
            }
            ASTNode::BinaryOp(left, operator, right) => {
                self.compile_recursive(left, bytecode);
                self.compile_recursive(right, bytecode);

                let right_reg = self.get_register(bytecode);
                let left_reg = self.get_register(bytecode);

                match operator {
                    BinOperator::Add => {
                        bytecode.push(Opcode::Add(left_reg, right_reg, right_reg));
                        self.stack.push(StackElement::Register(right_reg));
                        self.register_stack.push(left_reg);
                    }
                    BinOperator::Sub => {
                        bytecode.push(Opcode::Sub(left_reg, right_reg, right_reg));
                        self.stack.push(StackElement::Register(right_reg));
                        self.register_stack.push(left_reg);
                    }
                    BinOperator::Mul => {
                        bytecode.push(Opcode::Mul(left_reg, right_reg, right_reg));
                        self.stack.push(StackElement::Register(right_reg));
                        self.register_stack.push(left_reg);
                    }
                    BinOperator::Div => {
                        bytecode.push(Opcode::Div(left_reg, right_reg, right_reg));
                        self.stack.push(StackElement::Register(right_reg));
                        self.register_stack.push(left_reg);
                    }
                };
            }
        }
    }

    #[inline]
    fn get_register(&mut self, bytecode: &mut Vec<Opcode>) -> u8 {
        match self.stack.pop().unwrap() {
            StackElement::Register(reg) => reg,
            StackElement::Value(val) => {
                let reg = self.register_stack.pop().expect("Ran out of registers");
                bytecode.push(Opcode::Load(reg, val));
                reg
            }
        }
    }
}

enum StackElement {
    Value(Word),
    Register(u8),
}
