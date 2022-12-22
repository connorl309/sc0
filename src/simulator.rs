#![allow(rustdoc::all)]
use std::io::{self, Write};

/**
 * This file is the actual simulator for a program
 * we just pass in an Sc0 ref because it (should) contain all info we need
 * 
 * While it's easier to not assemble the user program at all, and just
 * simulate it based on the program text, that is not true to what something like
 * the LC3 would do -- and we lose access to resources in a memory space if we wanted to use it.
 * 
 * Additionally, syscalls are implemented in this file.
 */
use crate::cpu::{hardware::*, isa::*};

pub fn simulate(computer: &mut Sc0Hardware) {
    let program_selected = computer.get_prog(computer.selected.clone()).unwrap();
    computer.set_reg(R14_PC, program_selected.start_pc); // initialize program counter

    let mut can_run = true;
    while can_run {
        let mem_entry = computer.get_mem_dw(computer.get_reg(R14_PC as u8) as u32);

        let dest = get_bitrange(mem_entry, 20, 23);
        let possible_src1 = get_bitrange(mem_entry, 16, 19);
        let possible_src2 = get_bitrange(mem_entry, 0, 3);
        let constant = get_constant(mem_entry);
        let imm_flag = ((mem_entry & (1 << 24)) >> 24) & 1;
        let instr = get_opcode(mem_entry);

        // Match instruction
        match get_opcode(mem_entry) {
            // all math operations
            Instruction::Add | Instruction::Sub | Instruction::Mul | Instruction::Div | Instruction::And |
            Instruction::Or | Instruction::Xor | Instruction::Not | Instruction::Lshf | Instruction::Rshf => {
                math(imm_flag, instr, computer, dest, possible_src1, possible_src2, constant); // CC handled inside
            },
            Instruction::Mov => {
                if imm_flag == 1 { // constant
                    computer.set_reg(dest as usize, constant as u32);
                } else {
                    computer.set_reg(dest as usize, computer.get_reg(possible_src1 as u8) as u32);
                }
            },
            Instruction::Lea => {
                computer.set_reg(dest as usize, ((computer.get_reg(R14_PC as u8) as i32) + (constant as i32)) as u32);
            }
            Instruction::Ldi => {
                let val = computer.get_mem_dw(computer.get_mem_dw(possible_src1));
                computer.set_reg(dest as usize, val);
                set_cc(computer, val as i32);
            }
            Instruction::Ldb => {
                if imm_flag == 1 { // constant offset
                    let mut addr = computer.get_reg(possible_src1 as u8);
                    addr += constant as i32;
                    computer.set_reg(dest as usize, computer.get_mem_b(addr as u32));
                } else { // mem load offset in register
                    let mut addr = computer.get_reg(possible_src1 as u8);
                    addr += computer.get_reg(possible_src2 as u8);
                    computer.set_reg(dest as usize, computer.get_mem_b(addr as u32));
                }
            }
            Instruction::Ldw => {
                if imm_flag == 1 { // constant offset
                    let mut addr = computer.get_reg(possible_src1 as u8);
                    addr += constant as i32;
                    computer.set_reg(dest as usize, computer.get_mem_w(addr as u32));
                } else { // mem load offset in register
                    let mut addr = computer.get_reg(possible_src1 as u8);
                    addr += computer.get_reg(possible_src2 as u8);
                    computer.set_reg(dest as usize, computer.get_mem_w(addr as u32));
                }
            }
            Instruction::Ldd => {
                if imm_flag == 1 { // constant offset
                    let mut addr = computer.get_reg(possible_src1 as u8);
                    addr += constant as i32;
                    computer.set_reg(dest as usize, computer.get_mem_dw(addr as u32));
                } else { // mem load offset in register
                    let mut addr = computer.get_reg(possible_src1 as u8);
                    addr += computer.get_reg(possible_src2 as u8);
                    computer.set_reg(dest as usize, computer.get_mem_dw(addr as u32));
                }
            }
            Instruction::Sti => {
                let addr = computer.get_mem_dw(computer.get_mem_dw(computer.get_reg(dest as u8) as u32));
                computer.set_mem_dw(addr, computer.get_reg(possible_src1 as u8) as u32);
            }
            Instruction::Stb => {
                if imm_flag == 1 {
                    let addr = computer.get_reg(possible_src1 as u8) + (constant as i32);
                    computer.set_mem_b(addr as u32, computer.get_reg(dest as u8) as u8);
                } else {
                    let addr = computer.get_reg(possible_src1 as u8) + computer.get_reg(possible_src2 as u8);
                    computer.set_mem_b(addr as u32, computer.get_reg(dest as u8) as u8);
                }
            }
            Instruction::Stw => {
                if imm_flag == 1 {
                    let addr = computer.get_reg(possible_src1 as u8) + (constant as i32);
                    computer.set_mem_w(addr as u32, computer.get_reg(dest as u8) as u16);
                } else {
                    let addr = computer.get_reg(possible_src1 as u8) + computer.get_reg(possible_src2 as u8);
                    computer.set_mem_w(addr as u32, computer.get_reg(dest as u8) as u16);
                }
            }
            Instruction::Std => {
                if imm_flag == 1 {
                    let addr = computer.get_reg(possible_src1 as u8) + (constant as i32);
                    computer.set_mem_dw(addr as u32, computer.get_reg(dest as u8) as u32);
                } else {
                    let addr = computer.get_reg(possible_src1 as u8) + computer.get_reg(possible_src2 as u8);
                    computer.set_mem_dw(addr as u32, computer.get_reg(dest as u8) as u32);
                }
            }
            Instruction::Jmp => {
                if (mem_entry & 0xF0000) != 0 { // flag for register specified jump
                    computer.set_reg(R14_PC, computer.get_reg(
                        possible_src2 as u8 // reg jmp is [3:0]
                    ) as u32);
                } else {
                    computer.set_reg(R14_PC, constant as u32);
                }
            }
            Instruction::Call => {
                // save next instr...
                computer.set_reg(R12, (computer.get_reg(R14_PC as u8) as u32) + 1);
                if (mem_entry & 0xF0000) != 0 { // flag for register specified jump
                    computer.set_reg(R14_PC, computer.get_reg(
                        possible_src2 as u8 // reg jmp is [3:0]
                    ) as u32);
                } else {
                    computer.set_reg(R14_PC, constant as u32);
                }
            }
            Instruction::Syscall => {
                let call = get_bitrange(mem_entry, 0, 15);
                match call as u16 {
                    HALT => can_run = syscall_halt(),
                    DISPLAY => syscall_display(computer),
                    INPUT => syscall_input(computer),
                    PRINT => syscall_print(computer),
                    _ => {
                        println!("Invalid syscall 0x{:04X}! Aborting simulation.", call);
                        return;
                    }
                }
            }
            Instruction::Branch => {
                let spec_cc = get_bitrange(mem_entry, 16, 18);
                let curr_cc = get_bitrange(computer.get_reg(R15_PSR as u8) as u32, 0, 2);
                if (spec_cc & curr_cc) != 0 {
                    let new_addr = computer.get_reg(R14_PC as u8) + (constant as i32);
                    computer.set_reg(R14_PC, new_addr as u32);
                }
            }
            Instruction::Cmp => {
                if (mem_entry & (1 << 20)) != 0 { // const value for #2 arg
                    let val = computer.get_reg(dest as u8) - (constant as i32);
                    set_cc(computer, val);
                } else {
                    let val = computer.get_reg(dest as u8) - computer.get_reg(possible_src2 as u8);
                    set_cc(computer, val);
                }
            }
            Instruction::Push => {
                // 0x100000; // bit 20 flag for imm
                if (mem_entry & 0x100000) != 0 {
                    computer.set_mem_dw(computer.get_reg(R13_SP as u8) as u32, constant as u32);
                } else { // mama mia thats a lot of casts
                    computer.set_mem_dw(computer.get_reg(R13_SP as u8) as u32, 
                                        computer.get_reg(possible_src2 as u8) as u32
                    );
                }
                computer.set_reg(R13_SP, (computer.get_reg(R13_SP as u8) as u32) - 1);
            }
            Instruction::Pop => {
                computer.set_reg(R13_SP, (computer.get_reg(R13_SP as u8) as u32) + 1);
                computer.set_reg(possible_src2 as usize, computer.get_mem_dw(
                    computer.get_reg(R13_SP as u8) as u32
                ));
                set_cc(computer, computer.get_reg(possible_src2 as u8));
            }
            Instruction::END | Instruction::Error | Instruction::ORIG | Instruction::FILL | Instruction::STRING => {
                println!("Read an invalid opcode {:?}. Please check program or assembler/simulator code.", get_opcode(mem_entry));
            }
        }

        // Finally increment PC
        computer.set_reg(R14_PC, (computer.get_reg(R14_PC as u8) as u32) + 1);
    }
    computer.set_reg(R14_PC, (computer.get_reg(R14_PC as u8) as u32) - 1); // adjust for loop error
}

// highly repeated code
fn set_cc(computer: &mut Sc0Hardware, val: i32) {
    let mut currentCC = computer.get_reg(R15_PSR as u8);
    if val == 0 {
        currentCC &= !5;
        currentCC |= 2;
    } else if val > 0 {
        currentCC &= !6;
        currentCC |= 1;
    } else {
        currentCC &= !4;
        currentCC |= 4;
    }
    computer.set_reg(R15_PSR, currentCC as u32);
}
fn math(imm_flag: u32, instr: Instruction, computer: &mut Sc0Hardware, dest: u32, possible_src1: u32, possible_src2: u32, constant: i16) {
    let mut val = 0;
    if imm_flag == 1 {
        val = computer.alu_op(instr, computer.get_reg(possible_src1 as u8), constant as i32);
        computer.set_reg(dest as usize, val as u32);
    } else {
        val = computer.alu_op(instr, computer.get_reg(possible_src1 as u8), computer.get_reg(possible_src2 as u8));
        computer.set_reg(dest as usize, val as u32);
    }
    println!("[DBG] set register {} to {}", dest, val);
    set_cc(computer, val);
}

// Extract opcode from a memory entry
fn get_opcode(mem_entry: u32) -> Instruction {
    let val = ((mem_entry >> 27) & 0x1F) as u8;
    let res: Instruction = unsafe {std::mem::transmute(val)}; // mama mia
    res
}
// Extract bit range from a memory entry
fn get_bitrange(mem_entry: u32, start: u8, stop: u8) -> u32 { // can(and will) get demoted as necessary
    let mask = ((1 << (stop - start)) - 1) << start; // this is scuffed
    return (mem_entry & mask) >> start;
}

// Extract a constant
fn get_constant(mem_entry: u32) -> i16 {
    let val: u16 = (mem_entry & 0xFFFF) as u16;
    return val as i16;
}

// Syscalls
fn syscall_halt() -> bool {
    println!("Syscall [HALT] executed! Ending simulation of current program.");
    false
}
fn syscall_print(computer: &mut Sc0Hardware) { // might need to revamp this...
    println!("DBG: Syscall [PRINT] executed! (i need to fix strings!)");
}
fn syscall_display(computer: &mut Sc0Hardware) {
    print!("{}", computer.get_reg(R0 as u8));
}
fn syscall_input(computer: &mut Sc0Hardware) {
    print!("> ");
    let _ = io::stdout().flush(); // so formatting is nice
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read user input!");
    if input.len() > 1 {
        println!("Please only input 1 character. Taking first character...");
    }
    computer.set_reg(R0, input.remove(0) as u32);
}