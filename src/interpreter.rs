use crate::opcode::Opcode;

pub struct Thread {
    instructions: Vec<Opcode>,
    registers: Box<[Value; 256]>,
    return_value: Option<Value>,
}

impl Thread {
    pub fn new(instructions: Vec<Opcode>) -> Self {
        let registers: [Value; 256] = [const { Value::Int(0) }; 256];

        Thread {
            instructions,
            registers: Box::new(registers),
            return_value: None,
        }
    }

    pub fn return_value(&self) -> &Option<Value> {
        &self.return_value
    }

    pub fn exec(&mut self) {
        for op in &self.instructions {
            match op {
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
                Opcode::EqualUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
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
                Opcode::NotEqualUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
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
                Opcode::LessThanUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
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
                Opcode::LessEqUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
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
                Opcode::GreaterThanUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
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
                Opcode::GreaterEqUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
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
                Opcode::AddUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
                    self.registers[*res_idx as usize] = Value::Uint(lhs + rhs);
                }
                Opcode::AddFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Float(lhs + rhs);
                }
                Opcode::AddStr(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_string();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_string();
                    self.registers[*res_idx as usize] = Value::Str(Box::new(*lhs + rhs.as_str()));
                }

                Opcode::SubInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Int(lhs - rhs);
                }
                Opcode::SubUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
                    self.registers[*res_idx as usize] = Value::Uint(lhs - rhs);
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
                Opcode::MulUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
                    self.registers[*res_idx as usize] = Value::Uint(lhs * rhs);
                }
                Opcode::MulFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Float(lhs * rhs);
                }
                Opcode::MulStr(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_string();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint() as usize;
                    self.registers[*res_idx as usize] = Value::Str(Box::new(lhs.repeat(rhs)));
                }

                Opcode::DivInt(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_int();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_int();
                    self.registers[*res_idx as usize] = Value::Int(lhs / rhs);
                }
                Opcode::DivUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
                    self.registers[*res_idx as usize] = Value::Uint(lhs / rhs);
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
                Opcode::ModUint(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_uint();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_uint();
                    self.registers[*res_idx as usize] = Value::Uint(lhs % rhs);
                }
                Opcode::ModFloat(lhs_idx, rhs_idx, res_idx) => {
                    let lhs = self.registers[*lhs_idx as usize].unwrap_float();
                    let rhs = self.registers[*rhs_idx as usize].unwrap_float();
                    self.registers[*res_idx as usize] = Value::Float(lhs % rhs);
                }

                Opcode::LoadConst(_target_idx, _pool_idx) => {
                    todo!();
                }
                Opcode::LoadNum(_target_idx, _value) => {
                    todo!();
                }
                Opcode::Copy(_source_idx, _dest_idx) => {
                    todo!();
                }
                Opcode::Save(source_reg) => {
                    self.return_value = Some(self.registers[*source_reg as usize].clone());
                }
                Opcode::Error => {
                    panic!("Internal error");
                }

                // For WIP only
                Opcode::LoadInt(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Int(*value);
                }
                Opcode::LoadUint(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Uint(*value);
                }
                Opcode::LoadFloat(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Float(*value);
                }
                Opcode::LoadBool(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Bool(*value);
                }
                Opcode::LoadStr(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Str(value.clone());
                }
                Opcode::LoadChar(target_idx, value) => {
                    self.registers[*target_idx as usize] = Value::Char(*value);
                }
                Opcode::Print(target_idx) => {
                    println!("{}", self.registers[*target_idx as usize]);
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Int(i64),
    Uint(u64),
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
    fn unwrap_uint(&self) -> u64 {
        match self {
            Value::Uint(v) => *v,
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
    fn unwrap_string(&self) -> Box<String> {
        match self {
            Value::Str(v) => v.clone(),
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
            Value::Uint(value) => write!(f, "{}", value),
            Value::Float(value) => write!(f, "{}", value),
            Value::Bool(value) => write!(f, "{}", value),
            Value::Str(value) => write!(f, "{}", value),
            Value::Char(value) => write!(f, "{}", value),
        }
    }
}
