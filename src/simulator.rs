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
        let imm_flag = get_bitrange(mem_entry, 24, 24);
        let instr = get_opcode(mem_entry);

        // Match instruction
        match get_opcode(mem_entry) {
            // all math operations
            Instruction::Add | Instruction::Sub | Instruction::Mul | Instruction::Div | Instruction::And |
            Instruction::Or | Instruction::Xor | Instruction::Not | Instruction::Lshf | Instruction::Rshf => {
                math(imm_flag, instr, computer, dest, possible_src1, possible_src2, constant); // CC handled inside
            },
            Instruction::Mov => {
                if possible_src1 != 0 {
                    computer.set_reg(dest as usize, computer.get_reg(possible_src1 as u8) as u32);
                } else {
                    computer.set_reg(dest as usize, constant as u32);
                }
            },
            Instruction::Lea => {
                computer.set_reg(dest as usize, ((computer.get_reg(R14_PC as u8) as i32) + (constant as i32)) as u32);
            }
            Instruction::Ldi => todo!(),
            Instruction::Ldb => todo!(),
            Instruction::Ldw => todo!(),
            Instruction::Ldd => todo!(),
            Instruction::Sti => todo!(),
            Instruction::Stb => todo!(),
            Instruction::Stw => todo!(),
            Instruction::Std => todo!(),
            Instruction::Jmp => todo!(),
            Instruction::Call => todo!(),
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
            Instruction::Branch => todo!(),
            Instruction::Cmp => todo!(),
            Instruction::Push => todo!(),
            Instruction::Pop => todo!(),
            Instruction::END | Instruction::Error | Instruction::ORIG | Instruction::FILL | Instruction::STRING => {
                println!("Read an invalid opcode {:?}. Please check program or assembler/simulator code.", get_opcode(mem_entry));
            }
        }

        // Finally increment PC
        computer.set_reg(R14_PC, (computer.get_reg(R14_PC as u8) as u32) + 1);
    }
}

// highly repeated code
fn math(imm_flag: u32, instr: Instruction, computer: &mut Sc0Hardware, dest: u32, possible_src1: u32, possible_src2: u32, constant: i16) {
    let mut val = 0;
    if imm_flag == 1 {
        val = computer.alu_op(instr, computer.get_reg(possible_src1 as u8), constant as i32);
        computer.set_reg(dest as usize, val as u32);
    } else {
        val = computer.alu_op(instr, computer.get_reg(possible_src1 as u8), computer.get_reg(possible_src2 as u8));
        computer.set_reg(dest as usize, val as u32);
    }
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