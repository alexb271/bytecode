#![allow(dead_code)]

#[derive(Debug)]
pub enum Opcode {
    Or(u8, u8, u8),  // lhs idx, rhs idx, result idx
    And(u8, u8, u8), // lhs idx, rhs idx, result idx

    EqualInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    EqualFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    EqualBool(u8, u8, u8),  // lhs idx, rhs idx, result idx
    EqualStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    EqualChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    NotEqualInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    NotEqualFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    NotEqualBool(u8, u8, u8),  // lhs idx, rhs idx, result idx
    NotEqualStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    NotEqualChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    LessThanInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    LessThanFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    LessThanStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    LessThanChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    LessEqInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    LessEqFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    LessEqStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    LessEqChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    GreaterThanInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    GreaterThanFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    GreaterThanStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    GreaterThanChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    GreaterEqInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    GreaterEqFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    GreaterEqStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    GreaterEqChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    AddInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    AddFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    AddStr(u8, u8, u8),   // lhs idx, rhs idx, result idx

    SubInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    SubFloat(u8, u8, u8), // lhs idx, rhs idx, result idx

    MulInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    MulFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    MulStr(u8, u8, u8),   // lhs idx, rhs idx, result idx

    DivInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    DivFloat(u8, u8, u8), // lhs idx, rhs idx, result idx

    ModInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    ModFloat(u8, u8, u8), // lhs idx, rhs idx, result idx

    NegInt(u8, u8),   // operand idx, result idx
    NegFloat(u8, u8), // operand idx, result idx
    NegBool(u8, u8),  // operand idx, result idx

    LoadConst(u8, u16), // target register, constant pool idx
    LoadNum(u8, i16),   // target register, small numeric constant value
    LoadBool(u8, bool), // target register, bool constant
    Copy(u8, u8),       // src idx, dst idx

    Save(u8),          // Save content of target register as thread return value
    Jump(i16),         // Offset amount
    JumpCond(u8, i16), // operand idx, amount - Conditional jump based on the content of the register
    Error,             // Malformed bytecode

    // For WIP only
    LoadInt(u8, i64),
    LoadFloat(u8, f64),
    LoadStr(u8, Box<String>),
    LoadChar(u8, char),
    Print(u8), // argument idx
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let padding = 12;
        match self {
            Opcode::Or(lhs, rhs, dst) => write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "or"),
            Opcode::And(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "and")
            }

            Opcode::EqualInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "eq_int")
            }
            Opcode::EqualFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "eq_float")
            }
            Opcode::EqualBool(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "eq_bool")
            }
            Opcode::EqualStr(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "eq_str")
            }
            Opcode::EqualChar(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "eq_char")
            }

            Opcode::NotEqualInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "neq_int")
            }
            Opcode::NotEqualFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "neq_float")
            }
            Opcode::NotEqualBool(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "neq_bool")
            }
            Opcode::NotEqualStr(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "neq_str")
            }
            Opcode::NotEqualChar(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "neq_char")
            }

            Opcode::LessThanInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "lt_int")
            }
            Opcode::LessThanFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "lt_float")
            }
            Opcode::LessThanStr(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "lt_str")
            }
            Opcode::LessThanChar(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "lt_char")
            }

            Opcode::LessEqInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "leq_int")
            }
            Opcode::LessEqFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "leq_float")
            }
            Opcode::LessEqStr(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "leq_str")
            }
            Opcode::LessEqChar(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "leq_char")
            }

            Opcode::GreaterThanInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "gt_int")
            }
            Opcode::GreaterThanFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "gt_float")
            }
            Opcode::GreaterThanStr(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "gt_str")
            }
            Opcode::GreaterThanChar(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "gt_char")
            }

            Opcode::GreaterEqInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "geq_int")
            }
            Opcode::GreaterEqFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "geq_float")
            }
            Opcode::GreaterEqStr(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "geq_str")
            }
            Opcode::GreaterEqChar(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "geq_char")
            }

            Opcode::AddInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "add_int")
            }
            Opcode::AddFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "add_float")
            }
            Opcode::AddStr(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "add_str")
            }

            Opcode::SubInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "sub_int")
            }
            Opcode::SubFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "sub_float")
            }

            Opcode::MulInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "mul_int")
            }
            Opcode::MulFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "mul_float")
            }
            Opcode::MulStr(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "mul_str")
            }

            Opcode::DivInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "div_int")
            }
            Opcode::DivFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "div_float")
            }

            Opcode::ModInt(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "mod_int")
            }
            Opcode::ModFloat(lhs, rhs, dst) => {
                write!(f, "{:<padding$} {lhs:<3} {rhs:<3} {dst:<3}", "mod_float")
            }

            Opcode::NegInt(operand, dst) => {
                write!(f, "{:<padding$} {operand:<3} {dst:<3}", "neg_int")
            }
            Opcode::NegFloat(operand, dst) => {
                write!(f, "{:<padding$} {operand:<3} {dst:<3}", "neg_float")
            }
            Opcode::NegBool(operand, dst) => {
                write!(f, "{:<padding$} {operand:<3} {dst:<3}", "neg_bool")
            }

            Opcode::LoadConst(reg, idx) => write!(f, "{:<padding$} {reg:<3} {idx}", "ldconst"),
            Opcode::LoadNum(reg, val) => write!(f, "{:<padding$} {reg:<3} {val}", "ldnum"),
            Opcode::LoadBool(reg, val) => write!(f, "{:<padding$} {reg:<3} {val}", "ldbool"),
            Opcode::Copy(src, dst) => write!(f, "{:<padding$} {src:<3} {dst:<3}", "copy"),

            Opcode::Save(reg) => write!(f, "{:<padding$} {reg:<3}", "save"),
            Opcode::Jump(amount) => write!(f, "{:<padding$} {amount}", "jump"),
            Opcode::JumpCond(operand, amount) => {
                write!(f, "{:<padding$} {operand:<3} {amount}", "jumpcond")
            }
            Opcode::Error => write!(f, "{:<padding$}", "error"),

            // For WIP only
            Opcode::LoadInt(reg, val) => write!(f, "{:<padding$} {reg:<3} {val}", "wip_loadint"),
            Opcode::LoadFloat(reg, val) => {
                write!(f, "{:<padding$} {reg:<3} {val}", "wip_loadfloat")
            }
            Opcode::LoadStr(reg, val) => write!(f, "{:<padding$} {reg:<3} {val}", "wip_loadstr"),
            Opcode::LoadChar(reg, val) => write!(f, "{:<padding$} {reg:<3} {val}", "wip_loadchar"),
            Opcode::Print(reg) => write!(f, "{:<padding$} {reg:<3}", "wip_print"),
        }
    }
}
