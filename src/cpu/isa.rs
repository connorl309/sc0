/**
 * This file contains definitions for the SC0
 * ISA (Instruction Set Architecture)
 * 
 * It is a somewhat loose definition and will likely be changing as
 * this project evolves.
 */
#[derive(Debug)]
pub enum Instruction {
    Add,
    Sub,
    Mul,
    Div,
    Mov,
    And,
    Or,
    Not,
    Xor,
    Lshf,
    Rshf,
    Lea,
    Ldi,
    Ldb,
    Ldw,
    Ldd,
    Sti,
    Stb,
    Stw,
    Std,
    Jmp,
    Call,
    Syscall,
    Branch,
    Cmp,
    Push,
    Pop,
    Error
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
        _ => Instruction::Error
    }
}