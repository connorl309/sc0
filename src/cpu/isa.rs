use std::process::exit;

use crate::helpers::program::Program;

/**
 * This file contains definitions for the SC0
 * ISA (Instruction Set Architecture)
 * 
 * It is a somewhat loose definition and will likely be changing as
 * this project evolves.
 */

// LUT for instruction arg validity
const INSTR_LUT: &'static[usize] = &[
    3, /* add */ 3, /* sub */ 3, /* mul */
    3, /* div */ 2, /* mov */ 3, /* and */
    3, /* or */ 2, /* not */ 3, /* xor */
    3, 3, /* shifts */ 2, /* lea */
    2, /* ldi */ 3, 3, 3, /* ldb thru ldd */
    2, /* sti */ 3, 3, 3, /* stb thru std */
    1, /* jmp */ 1, /* call */ 1, /* syscall */
    1, /* brnzp */ 2, /* cmp */ 1, 1 /* push and pop */
];

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Instruction {
    Add = 1,
    Sub = 2,
    Mul = 3,
    Div = 4,
    Mov = 5,
    And = 6,
    Or = 7,
    Xor = 8,
    Not = 9,
    Lshf = 10,
    Rshf = 11,
    Lea = 12,
    Ldi = 13,
    Ldb = 14,
    Ldw = 15,
    Ldd = 16,
    Sti = 17,
    Stb = 18,
    Stw = 19,
    Std = 20,
    Jmp = 21,
    Call = 22,
    Syscall = 23,
    Branch = 0,
    Cmp = 24,
    Push = 25,
    Pop = 26,
    // pseudo-ops
    ORIG = 27,
    FILL = 28,
    STRING = 29,
    END = 30,
    Error = 31,
}
pub struct ExecStatement {
    pub opc: Instruction,
    pub args: Vec<String>,
}

// Get instruction from a string slice (i.e., one loaded from program)
pub fn get_instr(op: &str) -> Instruction {
    match op.to_lowercase().as_str() {
        "add" => Instruction::Add,
        "sub" => Instruction::Sub,
        "mul" => Instruction::Mul,
        "div" => Instruction::Div,
        "mov" => Instruction::Mov,
        "and" => Instruction::And,
        "or" => Instruction::Or,
        "not" => Instruction::Not,
        "xor" => Instruction::Xor,
        "lshf" => Instruction::Lshf,
        "rshf" => Instruction::Rshf,
        "lea" => Instruction::Lea,
        "ldi" => Instruction::Ldi,
        "ldb" => Instruction::Ldb,
        "ldw" => Instruction::Ldw,
        "ldd" => Instruction::Ldd,
        "sti" => Instruction::Sti,
        "stb" => Instruction::Stb,
        "stw" => Instruction::Stw,
        "std" => Instruction::Std,
        "jmp" => Instruction::Jmp,
        "call" => Instruction::Call,
        "syscall" => Instruction::Syscall,
        "br" | "brnzp" | "brnz" | "brnp" | "brn" | "brz" | "brp" | "brzp" => Instruction::Branch,
        "cmp" => Instruction::Cmp,
        "push" => Instruction::Push,
        "pop" => Instruction::Pop,
        ".orig" => Instruction::ORIG,
        ".end" => Instruction::END,
        ".string" => Instruction::STRING,
        ".fill" => Instruction::FILL,
        _ => Instruction::Error
    }
}

// validate # of arguments for every type of instruction
// i could maybe do this a better way but nah
pub fn check_args(p: &Program) -> bool {
    for exec in &p.instructions {
        if exec.opc == Instruction::Error {
            return false;
        }
        else {
            if INSTR_LUT[exec.opc as usize] != exec.args.len() {
                println!("Invalid argument lengths detected for {:?}", exec.opc);
                return false;
            }
        }
    }
    return true;
}