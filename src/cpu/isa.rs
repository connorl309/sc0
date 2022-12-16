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
    3, // add
    3, // sub
    3, // mul
    3, // div
    2, // mov
    3, // and
    3, // or
    3, // xor
    2, // not
    3, // lshf
    3, // rhsf
    2, // lea
    2, // ldi
    3, // ldb
    3, // ldw
    3, // ldd
    2, // sti
    3, // stb
    3, // stw
    3, // std
    1, // jmp
    1, // call
    1, // syscall
    2, // branch
    2, // cmp
    1, // push
    1, // pop
    1, // orig
    1, // fill
    1, // string
    0, // end
    0 // error
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
    for exec in &p.instructions {
        if exec.opc == Instruction::Error {
            return false;
        }
        else {
            if exec.opc == Instruction::ORIG {
                if exec.args[0].starts_with("0x") {
                    let temp = exec.args[0].trim_start_matches("0x");
                    p.start_pc = u32::from_str_radix(temp, 16).unwrap();
                } else if exec.args[0].starts_with("#") {
                    let temp = exec.args[0].trim_start_matches("#");
                    p.start_pc = u32::from_str_radix(temp, 10).unwrap();
                }
            }
            
            if INSTR_LUT[exec.opc as usize] != exec.args.len() {
                println!("Invalid argument lengths detected for {:?}", exec.opc);
                return false;
            }
        }
    }
    return true;
}