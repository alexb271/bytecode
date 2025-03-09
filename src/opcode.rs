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

    NegInt(u8, u8), // operand idx, result idx
    NegFloat(u8, u8), // operand idx, result idx
    NegBool(u8, u8), // operand idx, result idx

    LoadConst(u8, u16), // target register, constant pool idx
    LoadNum(u8, u16),   // target register, small numeric constant value
    LoadBool(u8, bool), // target register, bool constant
    Copy(u8, u8),       // src idx, dst idx

    Save(u8), // Save content of target register as thread return value
    Error,    // Malformed bytecode

    // For WIP only
    LoadInt(u8, i64),
    LoadFloat(u8, f64),
    LoadStr(u8, Box<String>),
    LoadChar(u8, char),
    Print(u8), // argument idx
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Opcode::Or(lhs, rhs, dst) => write!(f, "or {lhs} {rhs} {dst}"),
            Opcode::And(lhs, rhs, dst) => write!(f, "and {lhs} {rhs} {dst}"),

            Opcode::EqualInt(lhs, rhs, dst) => write!(f, "eq_int {lhs} {rhs} {dst}"),
            Opcode::EqualFloat(lhs, rhs, dst) => write!(f, "eq_float {lhs} {rhs} {dst}"),
            Opcode::EqualBool(lhs, rhs, dst) => write!(f, "eq_bool {lhs} {rhs} {dst}"),
            Opcode::EqualStr(lhs, rhs, dst) => write!(f, "eq_str {lhs} {rhs} {dst}"),
            Opcode::EqualChar(lhs, rhs, dst) => write!(f, "eq_char {lhs} {rhs} {dst}"),

            Opcode::NotEqualInt(lhs, rhs, dst) => write!(f, "neq_int {lhs} {rhs} {dst}"),
            Opcode::NotEqualFloat(lhs, rhs, dst) => write!(f, "neq_float {lhs} {rhs} {dst}"),
            Opcode::NotEqualBool(lhs, rhs, dst) => write!(f, "neq_bool {lhs} {rhs} {dst}"),
            Opcode::NotEqualStr(lhs, rhs, dst) => write!(f, "neq_str {lhs} {rhs} {dst}"),
            Opcode::NotEqualChar(lhs, rhs, dst) => write!(f, "neq_char {lhs} {rhs} {dst}"),

            Opcode::LessThanInt(lhs, rhs, dst) => write!(f, "lt_int {lhs} {rhs} {dst}"),
            Opcode::LessThanFloat(lhs, rhs, dst) => write!(f, "lt_float {lhs} {rhs} {dst}"),
            Opcode::LessThanStr(lhs, rhs, dst) => write!(f, "lt_str {lhs} {rhs} {dst}"),
            Opcode::LessThanChar(lhs, rhs, dst) => write!(f, "lt_char {lhs} {rhs} {dst}"),

            Opcode::LessEqInt(lhs, rhs, dst) => write!(f, "leq_int {lhs} {rhs} {dst}"),
            Opcode::LessEqFloat(lhs, rhs, dst) => write!(f, "leq_float {lhs} {rhs} {dst}"),
            Opcode::LessEqStr(lhs, rhs, dst) => write!(f, "leq_str {lhs} {rhs} {dst}"),
            Opcode::LessEqChar(lhs, rhs, dst) => write!(f, "leq_char {lhs} {rhs} {dst}"),

            Opcode::GreaterThanInt(lhs, rhs, dst) => write!(f, "gt_int {lhs} {rhs} {dst}"),
            Opcode::GreaterThanFloat(lhs, rhs, dst) => write!(f, "gt_float {lhs} {rhs} {dst}"),
            Opcode::GreaterThanStr(lhs, rhs, dst) => write!(f, "gt_str {lhs} {rhs} {dst}"),
            Opcode::GreaterThanChar(lhs, rhs, dst) => write!(f, "gt_char {lhs} {rhs} {dst}"),

            Opcode::GreaterEqInt(lhs, rhs, dst) => write!(f, "geq_int {lhs} {rhs} {dst}"),
            Opcode::GreaterEqFloat(lhs, rhs, dst) => write!(f, "geq_float {lhs} {rhs} {dst}"),
            Opcode::GreaterEqStr(lhs, rhs, dst) => write!(f, "geq_str {lhs} {rhs} {dst}"),
            Opcode::GreaterEqChar(lhs, rhs, dst) => write!(f, "geq_char {lhs} {rhs} {dst}"),

            Opcode::AddInt(lhs, rhs, dst) => write!(f, "add_int {lhs} {rhs} {dst}"),
            Opcode::AddFloat(lhs, rhs, dst) => write!(f, "add_float {lhs} {rhs} {dst}"),
            Opcode::AddStr(lhs, rhs, dst) => write!(f, "add_str {lhs} {rhs} {dst}"),

            Opcode::SubInt(lhs, rhs, dst) => write!(f, "sub_int {lhs} {rhs} {dst}"),
            Opcode::SubFloat(lhs, rhs, dst) => write!(f, "sub_float {lhs} {rhs} {dst}"),

            Opcode::MulInt(lhs, rhs, dst) => write!(f, "mul_int {lhs} {rhs} {dst}"),
            Opcode::MulFloat(lhs, rhs, dst) => write!(f, "mul_float {lhs} {rhs} {dst}"),
            Opcode::MulStr(lhs, rhs, dst) => write!(f, "mul_str {lhs} {rhs} {dst}"),

            Opcode::DivInt(lhs, rhs, dst) => write!(f, "div_int {lhs} {rhs} {dst}"),
            Opcode::DivFloat(lhs, rhs, dst) => write!(f, "div_float {lhs} {rhs} {dst}"),

            Opcode::ModInt(lhs, rhs, dst) => write!(f, "mod_int {lhs} {rhs} {dst}"),
            Opcode::ModFloat(lhs, rhs, dst) => write!(f, "mod_float {lhs} {rhs} {dst}"),

            Opcode::NegInt(operand, dst) => write!(f, "neg_int {operand} {dst}"),
            Opcode::NegFloat(operand, dst) => write!(f, "neg_float {operand} {dst}"),
            Opcode::NegBool(operand, dst) => write!(f, "neg_bool {operand} {dst}"),

            Opcode::LoadConst(reg, idx) => write!(f, "ldconst {reg} {idx}"),
            Opcode::LoadNum(reg, val) => write!(f, "ldnum {reg} {val}"),
            Opcode::LoadBool(reg, val) => write!(f, "ldbool {reg} {val}"),
            Opcode::Copy(src, dst) => write!(f, "copy {src} {dst}"),

            Opcode::Save(reg) => write!(f, "save {reg}"),
            Opcode::Error => write!(f, "error"),

            // For WIP only
            Opcode::LoadInt(reg, val) => write!(f, "wip_loadint {reg} {val}"),
            Opcode::LoadFloat(reg, val) => write!(f, "wip_loadfloat {reg} {val}"),
            Opcode::LoadStr(reg, val) => write!(f, "wip_loadstr {reg} {val}"),
            Opcode::LoadChar(reg, val) => write!(f, "wip_loadchar {reg} {val}"),
            Opcode::Print(reg) => write!(f, "wip_print {reg}"),
       }
    }
}