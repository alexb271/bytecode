#![allow(dead_code)]

pub const LOAD: u8 = 0x00;
pub const ADD: u8 = 0x01;
pub const SUB: u8 = 0x02;
pub const MUL: u8 = 0x03;
pub const DIV: u8 = 0x04;

pub type Word = f64;

#[derive(Debug)]
pub enum Opcode {
    Load(u8, Word),  // idx, value
    Add(u8, u8, u8), // lhs idx, rhs idx, result idx
    Sub(u8, u8, u8), // lhs idx, rhs idx, result idx
    Mul(u8, u8, u8), // lhs idx, rhs idx, result idx
    Div(u8, u8, u8), // lhs idx, rhs idx, result idx
    Copy(u8, u8),    // src idx, dst idx
    Print(u8),       // argument idx
}
