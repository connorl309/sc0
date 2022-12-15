use crate::helpers::program::Program;
use crate::cpu::isa::{ExecStatement, Instruction};
use std::fs::File;
use std::io::Write;

// Some useful return errors
const IMM_ERROR: u32 = 0xAFFFFFFF;
const REG_ERROR: u8 = 255;

/**
 * We do all the actual instruction checking and "assembly" here.
 * Big ass file. Not really clean. Oh well...
 */
pub fn assemble(p: &mut Program) -> bool {
    let mut objectFile = File::open(String::from(p.name.clone() + ".object")).unwrap();
    for (pos, statement) in p.instructions.iter_mut().enumerate() {
        // in this function we are GUARANTEED that every instruction has the correct num of args.
        // see instruction list if need (the .pdf)
        match statement.opc {
            Instruction::Add => {
                
            }
            Instruction::Sub => {
                
            }
            Instruction::Mul => {
                
            }
            Instruction::Div => {
                
            }
            Instruction::Mov => {
                
            }
            Instruction::And => {
                
            }
            Instruction::Or => {
                
            }
            Instruction::Xor => {
                
            }
            Instruction::Not => {
                
            }
            Instruction::Lshf => {
                
            }
            Instruction::Rshf => {
                
            }
            Instruction::Lea => {
                
            }
            Instruction::Ldi => {
                
            }
            Instruction::Ldb => {
                
            }
            Instruction::Ldw => {
                
            }
            Instruction::Ldd => {
                
            }
            Instruction::Sti => {
                
            }
            Instruction::Stb => {
                
            }
            Instruction::Stw => {
                
            }
            Instruction::Std => {
                
            }
            Instruction::Jmp => {
                
            }
            Instruction::Call => {
                
            }
            Instruction::Syscall => {
                
            }
            Instruction::Branch => {
                
            }
            Instruction::Cmp => {
                
            }
            Instruction::Push => {
                
            }
            Instruction::Pop => {
                
            }
            Instruction::ORIG => {
                
            }
            Instruction::FILL => {
                
            }
            Instruction::STRING => {
                
            }
            Instruction::END => {
                
            }
            Instruction::Error => {
                
            }
        }
    }
    return true;
}

// Extract out the register number from a register string
// An error occurs on result = 255
fn reg_num(arg: &String) -> u8 {
    if !(arg.starts_with('r')) {
        println!("Invalid operand {}, expected a register! Cannot assemble.", &arg);
        return 255;
    } else {
        return match arg.as_str() {
            "r0" => 0,
            "r1" => 1,
            "r2" => 2,
            "r3" => 3,
            "r4" => 4,
            "r5" => 5,
            "r6" => 6,
            "r7" => 7,
            "r8" => 8,
            "r9" => 9,
            "r10" => 10,
            "r11" => 11,
            "r12" => 12,
            "r13" | "sp" => 13,
            "r14" | "pc" => 14,
            "r15" | "psr" => 15,
            _ => REG_ERROR
        }
    }
}

// Extract out the numerical value of a string
// Error check -> i need to define the encodings.....
// I'm *pretty* sure constants will at most take up 16 bits. But I don't know.
// Note: need to cast as u32 so that formatting is normal
// When program is EXECUTED, is when we need to cast as i32.
fn imm_val(arg: &String) -> u32 {
    if arg.starts_with("0x") {
        let temp = arg.trim_start_matches("0x");
        match u32::from_str_radix(temp, 16) {
            Ok(_num) => {
                if _num > 65535 {
                    println!("Hex argument {} takes up more than 16 bits! Stopping assembly.", arg);
                    return IMM_ERROR;
                }
                return _num;
            },
            Err(_err) => {
                println!("Could not parse hex argument {}! Stopping assembly.", arg);
                return IMM_ERROR;
            }
        }
    } else if arg.starts_with("#") {
        let temp = arg.trim_start_matches("#");
        match u32::from_str_radix(temp, 10) {
            Ok(_num) => {
                if _num > 65535 {
                    println!("Decimal argument {} takes up more than 16 bits! Stopping assembly.", arg);
                    return IMM_ERROR;
                }
                return _num;
            },
            Err(_) => {
                println!("Could not parse decimal argument {}! Stopping assembly.", arg);
                return IMM_ERROR;
            }
        }
    } else {
        println!("Invalid immediate '{}'! Cannot parse, stopping assembly.", arg);
        return IMM_ERROR;
    }
}