use crate::opcode::Opcode;

pub struct Thread {
    instructions: Vec<Opcode>,
    program_counter: usize,

    registers: Box<[Value; 256]>,
    return_value: Option<Value>,
}

impl Thread {
    pub fn new(instructions: Vec<Opcode>) -> Self {
        let registers: [Value; 256] = [const { Value::Int(0) }; 256];

        Thread {
            instructions,
            program_counter: 0,
            registers: Box::new(registers),
            return_value: None,
        }
    }

    pub fn return_value(&self) -> &Option<Value> {
        &self.return_value
    }

    pub fn exec(&mut self) {
        self.program_counter = 0;
        while self.program_counter < self.instructions.len() {
            match &self.instructions[self.program_counter] {
                Opcode::Or(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_bool();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_bool();
                    self.registers[*res_idx as usize] = Value::Bool(lhs || rhs);
                }
                Opcode::And(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_bool();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_bool();
                    self.registers[*res_idx as usize] = Value::Bool(lhs && rhs);
                }

                Opcode::EqualInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Bool(lhs == rhs);
                }
                Opcode::EqualFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Bool(lhs == rhs);
                }
                Opcode::EqualBool(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_bool();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_bool();
                    self.registers[*res_idx as usize] = Value::Bool(lhs == rhs);
                }
                Opcode::EqualStr(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_string();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_string();
                    self.registers[*res_idx as usize] = Value::Bool(lhs == rhs);
                }
                Opcode::EqualChar(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_char();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_char();
                    self.registers[*res_idx as usize] = Value::Bool(lhs == rhs);
                }

                Opcode::NotEqualInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Bool(lhs != rhs);
                }
                Opcode::NotEqualFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Bool(lhs != rhs);
                }
                Opcode::NotEqualBool(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_bool();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_bool();
                    self.registers[*res_idx as usize] = Value::Bool(lhs != rhs);
                }
                Opcode::NotEqualStr(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_string();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_string();
                    self.registers[*res_idx as usize] = Value::Bool(lhs != rhs);
                }
                Opcode::NotEqualChar(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_char();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_char();
                    self.registers[*res_idx as usize] = Value::Bool(lhs != rhs);
                }

                Opcode::LessThanInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Bool(lhs < rhs);
                }
                Opcode::LessThanFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Bool(lhs < rhs);
                }
                Opcode::LessThanStr(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_string();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_string();
                    self.registers[*res_idx as usize] = Value::Bool(lhs < rhs);
                }
                Opcode::LessThanChar(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_char();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_char();
                    self.registers[*res_idx as usize] = Value::Bool(lhs < rhs);
                }

                Opcode::LessEqInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Bool(lhs <= rhs);
                }
                Opcode::LessEqFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Bool(lhs <= rhs);
                }
                Opcode::LessEqStr(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_string();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_string();
                    self.registers[*res_idx as usize] = Value::Bool(lhs <= rhs);
                }
                Opcode::LessEqChar(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_char();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_char();
                    self.registers[*res_idx as usize] = Value::Bool(lhs <= rhs);
                }

                Opcode::GreaterThanInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Bool(lhs > rhs);
                }
                Opcode::GreaterThanFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Bool(lhs > rhs);
                }
                Opcode::GreaterThanStr(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_string();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_string();
                    self.registers[*res_idx as usize] = Value::Bool(lhs > rhs);
                }
                Opcode::GreaterThanChar(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_char();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_char();
                    self.registers[*res_idx as usize] = Value::Bool(lhs > rhs);
                }

                Opcode::GreaterEqInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Bool(lhs >= rhs);
                }
                Opcode::GreaterEqFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Bool(lhs >= rhs);
                }
                Opcode::GreaterEqStr(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_string();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_string();
                    self.registers[*res_idx as usize] = Value::Bool(lhs >= rhs);
                }
                Opcode::GreaterEqChar(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_char();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_char();
                    self.registers[*res_idx as usize] = Value::Bool(lhs >= rhs);
                }

                Opcode::AddInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Int(lhs + rhs);
                }
                Opcode::AddFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Float(lhs + rhs);
                }
                Opcode::AddStr(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_string();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_string();
                    self.registers[*res_idx as usize] =
                        Value::Str(Box::new(String::from(lhs.as_str()) + rhs.as_str()));
                }

                Opcode::SubInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Int(lhs - rhs);
                }
                Opcode::SubFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Float(lhs - rhs);
                }

                Opcode::MulInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Int(lhs * rhs);
                }
                Opcode::MulFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Float(lhs * rhs);
                }
                Opcode::MulStr(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_string();
                    let mut rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    if rhs < 0 {
                        rhs = 0;
                    }
                    self.registers[*res_idx as usize] =
                        Value::Str(Box::new(lhs.repeat(rhs as usize)));
                }

                Opcode::DivInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Int(lhs / rhs);
                }
                Opcode::DivFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Float(lhs / rhs);
                }

                Opcode::ModInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Int(lhs % rhs);
                }
                Opcode::ModFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Float(lhs % rhs);
                }

                Opcode::NegInt(operand_idx, res_idx) => {
                    let operand = self.registers[*operand_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Int(-1 * operand);
                }
                Opcode::NegFloat(operand_idx, res_idx) => {
                    let operand = self.registers[*operand_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Float(-1.0 * operand);
                }
                Opcode::NegBool(operand_idx, res_idx) => {
                    let operand = self.registers[*operand_idx as usize].unwrap_bool();
                    self.registers[*res_idx as usize] = Value::Bool(!operand);
                }

                Opcode::LoadConst(_target_idx, _pool_idx) => {
                    todo!();
                }
                Opcode::LoadNum(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Int(*value as i64);
                }
                Opcode::Copy(source_idx, dest_idx) => {
                    self.registers[*dest_idx as usize] =
                        self.registers[*source_idx as usize].clone();
                }

                Opcode::Save(source_reg) => {
                    self.return_value = Some(self.registers[*source_reg as usize].clone());
                }
                Opcode::Jump(amount) => {
                    self.program_counter =
                        self.program_counter.wrapping_add_signed(*amount as isize);
                }
                Opcode::JumpCond(operand_idx, amount) => {
                    let operand = self.registers[*operand_idx as usize].unwrap_bool();
                    if !operand {
                        self.program_counter =
                            self.program_counter.wrapping_add_signed(*amount as isize);
                    }
                }
                Opcode::Error => {
                    panic!("Internal error");
                }

                // For WIP only
                Opcode::LoadInt(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Int(*value);
                }
                Opcode::LoadFloat(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Float(*value);
                }
                Opcode::LoadBool(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Bool(*value);
                }
                Opcode::LoadStr(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Str(Box::new(*value.clone()));
                }
                Opcode::LoadChar(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Char(*value);
                }
                Opcode::Print(target_idx) => {
                    println!("{}", self.registers[*target_idx as usize]);
                }
            }
            self.program_counter = self.program_counter.wrapping_add(1);
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(Box<String>),
    Char(char),
}

impl Value {
    #[inline]
    fn unwrap_int(&self) -> i64 {
        match self {
            Value::Int(v) => *v,
            _ => panic!("Internal type error"),
        }
    }

    #[inline]
    fn unwrap_float(&self) -> f64 {
        match self {
            Value::Float(v) => *v,
            _ => panic!("Internal type error"),
        }
    }

    #[inline]
    fn unwrap_bool(&self) -> bool {
        match self {
            Value::Bool(v) => *v,
            _ => panic!("Internal type error"),
        }
    }

    #[inline]
    fn unwrap_string(&self) -> &String {
        match self {
            Value::Str(v) => v,
            _ => panic!("Internal type error"),
        }
    }

    #[inline]
    fn unwrap_char(&self) -> char {
        match self {
            Value::Char(v) => *v,
            _ => panic!("Internal type error"),
        }
    }
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
