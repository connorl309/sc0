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
    3,
    3,
    3,
    3,
    2,
    3,
    3,
    2,
    3,
    3,
    3,
    2,
    2,
    3,
    3,
    3,
    2,
    3,
    3,
    3,
    2,
    2,
    1,
    1,
    2,
    1, // pop
    1, // pseudo ops
    1,
    1,
    0,
    0
];

#[derive(Debug, Copy, Clone, PartialEq)]
// Note that this is used for indices !!
pub enum Instruction {
    Add,
    Sub ,
    Mul ,
    Div ,
    Mov ,
    And ,
    Or ,
    Xor ,
    Not ,
    Lshf ,
    Rshf ,
    Lea ,
    Ldi ,
    Ldb ,
    Ldw ,
    Ldd ,
    Sti ,
    Stb ,
    Stw ,
    Std ,
    Jmp ,
    Call ,
    Syscall ,
    Branch ,
    Cmp ,
    Push ,
    Pop ,
    // pseudo-ops
    ORIG ,
    FILL ,
    STRING ,
    END ,
    Error ,
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
pub fn check_args(p: &mut Program) -> bool {
    for exec in &mut p.instructions {
        if exec.opc == Instruction::Error {
            return false;
        }
        else {
            if exec.opc == Instruction::ORIG {
                p.start_pc = exec.args[0].parse::<u32>().unwrap();
                continue;
            } 
            
            if INSTR_LUT[exec.opc as usize] != exec.args.len() {
                println!("Invalid argument lengths detected for {:?}", exec.opc);
                return false;
            }
        }
    }
    return true;
}