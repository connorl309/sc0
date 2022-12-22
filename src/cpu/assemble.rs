#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

/**
 * We do all the actual instruction checking and "assembly" here.
 * Big ass file. Not really clean. Oh well...
 * 
 * Can probably clean this up *a lot*. If you have any suggestions, please
 * recommend them to me as a Github issue or PR.
 */

use crate::helpers::program::Program;
use crate::cpu::hardware::{HALT,PRINT,DISPLAY,INPUT};
use crate::cpu::isa::{Instruction};
use std::path::Path;

use super::hardware::Sc0Hardware;

// Some useful return errors
const IMM_ERROR: u32 = 0xAFFFFFFF;
const REG_ERROR: u8 = 255;

// Syscall constants (see hardware.rs)
pub fn assemble(p: &mut Program, outputter: &mut Vec<u32>) -> bool {
    // all program assembly stored into memory
    for (pos, statement) in p.instructions.iter().enumerate() {
        let mut outputHexLine: u32 = 0;
        // in this function we are GUARANTEED that every instruction has the correct num of args.
        // see instruction list if need (the .pdf)
        let opc_val = (statement.opc as u32) << 27;
        outputHexLine += opc_val;
        match statement.opc {
            Instruction::Add => { // bit 24 = 1 if immediate, 0 if not
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: ADD destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on ADD! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Sub => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: SUB destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on SUB! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Mul => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: MUL destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on MUL! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Div => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: DIV destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on DIV! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Mov => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src = try_reg_or_imm_or_label(&p, &statement.args[1]);
                if dest.1 != ResType::reg {
                    println!("Error: MOV destination invalid! Aborting assembly.");
                    
                    return false;
                }
                if src.1 == ResType::reg {
                    outputHexLine += src.0 << 16;
                }
                else if src.1 == ResType::constant {
                    outputHexLine += src.0 & 0xFFFF;
                    outputHexLine |= (1 << 24);
                    outputHexLine |= 0x1000000;
                } else {
                    println!("Argument error on MOV! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::And => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: AND destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on AND! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Or => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: OR destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on OR! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Xor => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: XOR destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on XOR! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Not => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src = try_reg_or_imm_or_label(&p, &statement.args[1]);
                if dest.1 != ResType::reg || src.1 != ResType::reg {
                    println!("Error: NOT destination/src invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 12;
                outputHexLine += src.0;
            }
            Instruction::Lshf => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: LSHF destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on LSHF! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Rshf => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: RSHF destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on RSHF! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Lea => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src = try_reg_or_imm_or_label(&p, &statement.args[1]);
                if dest.1 != ResType::reg || src.1 != ResType::label {
                    println!("Error: LEA destination/src invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += ((src.0 as i32) & 0xFFFF) as u32;
            }
            Instruction::Ldi => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src = try_reg_or_imm_or_label(&p, &statement.args[1]);
                if dest.1 != ResType::reg || src.1 != ResType::reg {
                    println!("Error: LDI destination/src invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src.0;
            }
            Instruction::Ldb => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: LDB destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on LDB! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Ldw => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: LDW destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on LDW! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Ldd => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: LDD destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on LDD! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Sti => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src = try_reg_or_imm_or_label(&p, &statement.args[1]);
                if dest.1 != ResType::reg || src.1 != ResType::reg {
                    println!("Error: STI destination/src invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src.0;
            }
            Instruction::Stb => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: STB destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on STB! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Stw => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: STW destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on STW! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Std => {
                let dest = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[2]);
                if dest.1 != ResType::reg || src1.1 != ResType::reg {
                    println!("Error: STD destination/src1 invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += dest.0 << 20;
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else if src2.1 == ResType::constant {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x1000000; // set bit 24
                } else {
                    println!("Argument error on STD! Aborting assembly.");
                    
                    return false;
                }
            }
            Instruction::Jmp => {
                let goto = try_reg_or_imm_or_label(&p, &statement.args[0]);
                if goto.1 == ResType::constant {
                    println!("Error: JMP destination invalid! Aborting assembly.");
                    
                    return false;
                }
                if goto.1 == ResType::reg {
                    outputHexLine += goto.0;
                    outputHexLine |= 0xF0000; // flag for register specified jump
                } else {
                    outputHexLine += ((goto.0 as i32) & 0xFFFF) as u32;
                }
            }
            Instruction::Call => {
                let goto = try_reg_or_imm_or_label(&p, &statement.args[0]);
                if goto.1 == ResType::constant {
                    println!("Error: CALL destination invalid! Aborting assembly.");
                    
                    return false;
                }
                if goto.1 == ResType::reg {
                    outputHexLine += goto.0;
                    outputHexLine |= 0xF0000; // flag for register specified jump
                } else {
                    outputHexLine += ((goto.0 as i32) & 0xFFFF) as u32;
                }
            }
            Instruction::Syscall => {
                // see hardware.rs for these constants
                match statement.args[0].as_str() {
                    "halt" => outputHexLine += (HALT as u32) & 0xFFFF,
                    "print" => outputHexLine += (PRINT as u32) & 0xFFFF,
                    "display" => outputHexLine += (DISPLAY as u32) & 0xFFFF,
                    "input" => outputHexLine += (INPUT as u32) & 0xFFFF,
                    _ => {
                        println!("Invalid syscall! Aborting assembly.");
                        
                        return false;
                    }
                }
            }
            Instruction::Branch => {
                // args[0] of branch is condition codes
                let cc = &statement.args[0];
                let label = try_reg_or_imm_or_label(&p, &statement.args[1]);
                if label.1 != ResType::label {
                    println!("BRANCH argument is not a label! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += ((label.0 as i32) & 0xFFFF) as u32;
                // set CC bits (18:16)
                if cc.contains('n') {
                    outputHexLine |= 0x40000;
                }
                if cc.contains('z') {
                    outputHexLine |= 0x20000;
                }
                if cc.contains('p') {
                    outputHexLine |= 0x10000;
                }
            }
            Instruction::Cmp => {
                let src1 = try_reg_or_imm_or_label(&p, &statement.args[0]);
                let src2 = try_reg_or_imm_or_label(&p, &statement.args[1]);
                if src1.1 != ResType::reg || src2.1 == ResType::label {
                    println!("CMP arguments are invalid! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += src1.0 << 16;
                if src2.1 == ResType::reg {
                    outputHexLine += src2.0;
                } else {
                    outputHexLine += ((src2.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x100000; // bit 20 flag for imm
                }
            }
            Instruction::Push => {
                let toPush = try_reg_or_imm_or_label(&p, &statement.args[0]);
                if toPush.1 == ResType::label {
                    println!("PUSH argument cannot be a label! Aborting assembly.");
                    
                    return false;
                }
                if toPush.1 == ResType::reg {
                    outputHexLine += toPush.0 << 16;
                }
                else {
                    outputHexLine += ((toPush.0 as i32) & 0xFFFF) as u32;
                    outputHexLine |= 0x100000; // bit 20 flag for imm
                }
            }
            Instruction::Pop => {
                let intoReg = try_reg_or_imm_or_label(&p, &statement.args[0]);
                if intoReg.1 != ResType::reg {
                    println!("POP argument MUST be a register! Aborting assembly.");
                    
                    return false;
                }
                outputHexLine += intoReg.0;
            }
            Instruction::ORIG => {
                let (val, result) = try_reg_or_imm_or_label(&p, &statement.args[0]); // we only want
                if (result == ResType::bad) { return false; }
                outputHexLine = val;
            }
            Instruction::FILL => {
                let mut fillVal: u32 = 0;
                let arg = &statement.args[0];
                if arg.starts_with("0x") {
                    let temp = arg.trim_start_matches("0x");
                    fillVal = u32::from_str_radix(temp, 16).unwrap();
                } else if arg.starts_with("#") {
                    let temp = arg.trim_start_matches("#");
                    fillVal = u32::from_str_radix(temp, 10).unwrap();
                }
                outputHexLine = fillVal;
            }
            Instruction::STRING => {
                // this is going to be a bit tricky
                // we have to separate the whole string into chunks
                // of 4 bytes each
                let mut count = 0;
                let mut collect: u32 = 0;
                for c in statement.args[0].as_bytes() {
                    count += 1;
                    collect <<= 8;
                    collect += *c as u32;
                    if count%4 == 0 {
                        outputter.push(collect);
                        collect = 0;
                    }
                }
                // write whatever is remaining
                // this is scuffed and honestly idk if it works
                // BUT. maybe.
                collect <<= (count%4) * 8;
                outputter.push(collect);
                continue;
            }
            Instruction::END => {
                break;
            }
            Instruction::Error => {
                println!("\nHow did an invalid instruction slip through the checks... check project code!!!\n");
                panic!();
            }
        }
        // for now... assume the write is successful...
        outputter.push(outputHexLine);
        p.scratchwork += 1;
    }
    return true;
}

// Adjusts label addresses once we actually calculate the .ORIG value
// unsure if i need this...?
pub fn adjust_labels(p: &mut Program, adjustment: u32) {
    for label in p.sym_table.iter_mut() {
        label.addr += adjustment;
    }
}

// Extract either register, or immediate, value from an arg string
// Kind of nasty, but oh well.
#[derive(PartialEq)]
enum ResType {
    reg,
    constant,
    label,
    bad
}
fn try_reg_or_imm_or_label(p: &&mut Program, arg: &String) -> (u32, ResType) {
    // label
    if p.labelExists(arg.to_string()) {
        return (p.labelOffset(arg.to_string(), p.scratchwork) as u32, ResType::label);
    }
    // registers
    else if arg.starts_with('r') {
        return match arg.as_str() {
            "r0" => (0, ResType::reg),
            "r1" => (1, ResType::reg),
            "r2" => (2, ResType::reg),
            "r3" => (3, ResType::reg),
            "r4" => (4, ResType::reg),
            "r5" => (5, ResType::reg),
            "r6" => (6, ResType::reg),
            "r7" => (7, ResType::reg),
            "r8" => (8, ResType::reg),
            "r9" => (9, ResType::reg),
            "r10" => (10, ResType::reg),
            "r11" => (11, ResType::reg),
            "r12" => (12, ResType::reg),
            "r13" | "sp" => (13, ResType::reg),
            "r14" | "pc" => (14, ResType::reg),
            "r15" | "psr" => (15, ResType::reg),
            _ => {
                println!("Invalid register '{}' specified. Please fix.", arg);
                return (IMM_ERROR, ResType::bad);
            }
        }
    } 
    // numbers
    else if arg.starts_with("0x") {
        let temp = arg.trim_start_matches("0x");
        match u32::from_str_radix(temp, 16) {
            Ok(_num) => {
                if _num > 65535 {
                    println!("Hex argument {} takes up more than 16 bits! Stopping assembly.", arg);
                    return (IMM_ERROR, ResType::bad);;
                }
                return (_num, ResType::constant);
            },
            Err(_err) => {
                println!("Could not parse hex argument {}! Stopping assembly.", arg);
                return (IMM_ERROR, ResType::bad);
            }
        }
    } else if arg.starts_with("#") {
        let temp = arg.trim_start_matches("#");
        match u32::from_str_radix(temp, 10) {
            Ok(_num) => {
                if _num > 65535 {
                    println!("Decimal argument {} takes up more than 16 bits! Stopping assembly.", arg);
                    return (IMM_ERROR, ResType::bad);
                }
                return (_num, ResType::constant);
            },
            Err(_) => {
                println!("Could not parse decimal argument {}! Stopping assembly.", arg);
                return (IMM_ERROR, ResType::bad);
            }
        }
    } else {
        println!("Invalid argument '{}'! Cannot parse, stopping assembly.", arg);
        return (IMM_ERROR, ResType::bad);
    }
}