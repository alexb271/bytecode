use crate::opcode::{Opcode, Word};

pub struct Thread {
    bytecode: Vec<Opcode>,
    registers: Vec<Word>,
}

impl Thread {
    pub fn new(bytecode: Vec<Opcode>) -> Self {
        Thread {
            bytecode,
            registers: vec![0.0; 256],
        }
    }

    pub fn exec(&mut self) {
        for op in &self.bytecode {
            match op {
                Opcode::Add(lhs_idx, rhs_idx, res_idx) => {
                    self.registers[*res_idx as usize] =
                        self.registers[*lhs_idx as usize] + self.registers[*rhs_idx as usize];
                }
                Opcode::Sub(lhs_idx, rhs_idx, res_idx) => {
                    self.registers[*res_idx as usize] =
                        self.registers[*lhs_idx as usize] - self.registers[*rhs_idx as usize];
                }
                Opcode::Mul(lhs_idx, rhs_idx, res_idx) => {
                    self.registers[*res_idx as usize] =
                        self.registers[*lhs_idx as usize] * self.registers[*rhs_idx as usize];
                }
                Opcode::Div(lhs_idx, rhs_idx, res_idx) => {
                    self.registers[*res_idx as usize] =
                        self.registers[*lhs_idx as usize] / self.registers[*rhs_idx as usize];
                }
                Opcode::Load(idx, value) => {
                    self.registers[*idx as usize] = *value;
                }
                Opcode::Copy(src_idx, dst_idx) => {
                    self.registers[*dst_idx as usize] = self.registers[*src_idx as usize];
                }
                Opcode::Print(idx) => {
                    println!("{}", self.registers[*idx as usize]);
                }
            }
        }
    }
}
