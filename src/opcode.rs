#![allow(dead_code)]

pub const LOAD: u8 = 0x00;
pub const ADD: u8 = 0x01;
pub const SUB: u8 = 0x02;
pub const MUL: u8 = 0x03;
pub const DIV: u8 = 0x04;

#[derive(Debug)]
pub enum Opcode {
    Or(u8, u8, u8),  // lhs idx, rhs idx, result idx
    And(u8, u8, u8), // lhs idx, rhs idx, result idx

    EqualInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    EqualUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    EqualFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    EqualBool(u8, u8, u8),  // lhs idx, rhs idx, result idx
    EqualStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    EqualChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    NotEqualInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    NotEqualUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    NotEqualFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    NotEqualBool(u8, u8, u8),  // lhs idx, rhs idx, result idx
    NotEqualStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    NotEqualChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    LessThanInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    LessThanUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    LessThanFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    LessThanStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    LessThanChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    LessEqInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    LessEqUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    LessEqFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    LessEqStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    LessEqChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    GreaterThanInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    GreaterThanUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    GreaterThanFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    GreaterThanStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    GreaterThanChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    GreaterEqInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    GreaterEqUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    GreaterEqFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    GreaterEqStr(u8, u8, u8),   // lhs idx, rhs idx, result idx
    GreaterEqChar(u8, u8, u8),  // lhs idx, rhs idx, result idx

    AddInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    AddUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    AddFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    AddStr(u8, u8, u8),   // lhs idx, rhs idx, result idx

    SubInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    SubUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    SubFloat(u8, u8, u8), // lhs idx, rhs idx, result idx

    MulInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    MulUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    MulFloat(u8, u8, u8), // lhs idx, rhs idx, result idx
    MulStr(u8, u8, u8),   // lhs idx, rhs idx, result idx

    DivInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    DivUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    DivFloat(u8, u8, u8), // lhs idx, rhs idx, result idx

    ModInt(u8, u8, u8),   // lhs idx, rhs idx, result idx
    ModUint(u8, u8, u8),  // lhs idx, rhs idx, result idx
    ModFloat(u8, u8, u8), // lhs idx, rhs idx, result idx

    LoadConst(u8, u16), // target register, constant pool idx
    LoadNum(u8, u16),   // target register, small numeric constant value
    Copy(u8, u8),       // src idx, dst idx

    Save(u8), // Save content of target register as thread return value
    Error,    // Malformed bytecode

    // For WIP only
    LoadInt(u8, i64),
    LoadUint(u8, u64),
    LoadFloat(u8, f64),
    LoadBool(u8, bool),
    LoadStr(u8, Box<String>),
    LoadChar(u8, char),
    Print(u8), // argument idx
}
