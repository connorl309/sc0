#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]

/**
 * This file contains all definitions and funcs needed for
 * the "hardware" of the SC0
 * 
 * Register file, a (basic) datapath (which might not even be implemented initially...),
 * memory, and "I/O" (software stdin/stdio)
 */

use crate::helpers::program::Program;
use super::assemble::assemble;
use super::isa::{Instruction, check_args};
use std::fs::File;
use std::io::Write;
use std::ptr::null_mut;
use std::num::Wrapping;

// List of syscall definitions
pub const HALT: u16 = 0x1F;
pub const PRINT: u16 = 0x2F;
pub const DISPLAY: u16 = 0x3F;
pub const INPUT: u16 = 0x4F;

// List of registers
pub const R0: usize = 0; pub const R3: usize = 3;
pub const R1: usize = 1; pub const R4: usize = 4;
pub const R2: usize = 2; pub const R5: usize = 5;
pub const R6: usize = 6; pub const R7: usize = 7;
pub const R8: usize = 8; pub const R9: usize = 9;
pub const R10: usize = 10; pub const R11: usize = 11;
pub const R12: usize = 12; pub const R13_SP: usize = 13;
pub const R14_PC: usize = 14; pub const R15_PSR: usize = 15;

// "ALU", for math operations
// add, sub, mul, div, shifts, xor, and, or, not
// I probably dont need an impl for this,
// but i want to get practice with it.
struct ALU {
    input1: u32,
    input2: u32,
    output: u32
}
impl ALU {
    fn flush(&mut self) {
        self.input1 = 0;
        self.input1 = 0;
        self.output = 0;
    }
    fn add(&mut self) -> u32 {
        self.output = Wrapping(Wrapping(self.input1 as i32) + Wrapping(self.input2 as i32)).0.0 as u32;
        return self.output;
    }
    fn sub(&mut self) -> u32 {
        self.output = Wrapping(Wrapping(self.input1 as i32) - Wrapping(self.input2 as i32)).0.0 as u32;
        return self.output;
    }
    fn mul(&mut self) -> u32 {
        self.output = Wrapping(Wrapping(self.input1 as i32) * Wrapping(self.input2 as i32)).0.0 as u32;
        return self.output;
    }
    fn div(&mut self) -> u32 {
        self.output = Wrapping(Wrapping(self.input1 as i32) / Wrapping(self.input2 as i32)).0.0 as u32;
        return self.output;
    }
    fn lshf(&mut self) -> u32 {
        self.output = self.input1 << self.input2;
        return self.output;
    }
    fn rshf(&mut self) -> u32 {
        self.output = self.input1 >> self.input2;
        return self.output;
    }
    fn xor(&mut self) -> u32 {
        self.output = self.input1 ^ self.input2;
        return self.output;
    }
    fn and(&mut self) -> u32 {
        self.output = self.input1 & self.input2;
        return self.output;
    }
    fn or(&mut self) -> u32 {
        self.output = self.input1 | self.input2;
        return self.output;
    }
    fn not(&mut self) -> u32 {
        self.output = !self.input1; // rust uses ! for bitwise not. funny.
        return self.output;
    }
}

// Initialization of hardware object
pub fn initialize(limit: u32) -> Sc0Hardware {
    let mut regs = [0; 16];
    regs[R13_SP] = limit - 1; // stack pointer; initialize to point at last memory address
    regs[R15_PSR] = 0b010; // initial CC is only Z
    return Sc0Hardware { register_file: regs, memory: vec![0; limit as usize], 
        alu: ALU{input1: 0, input2: 0, output: 0}, mem_limit: limit,
        user_progs: Vec::new(), selected: String::new(), // this gotta be checked !!
    }
}

pub struct Sc0Hardware {
    pub register_file: [u32; 16], // note that [13-15] are somewhat reserved
    pub memory: Vec<u32>,
    alu: ALU,
    mem_limit: u32,
    user_progs: Vec<Program>,
    pub selected: String,
}
// related funcs
impl Sc0Hardware {
    // higher level stuff, program related
    pub fn add_prog(&mut self, p: Program) {
        // p passed in from load_program() in main.rs
        // this returns a program obj
        self.user_progs.push(p);
        // want to verify for every program added
        let mut objectVec: Vec<u32> = Vec::new();
        if check_args(self.user_progs.last_mut().unwrap()) && assemble(self.user_progs.last_mut().unwrap(), &mut objectVec) {
            let prog: &Program = self.user_progs.last().unwrap();
            let mut ctr = 0;
            objectVec.remove(0); // remove the .orig start.
            for value in objectVec.iter() {
                self.memory[(prog.start_pc + ctr) as usize] = *value;
                ctr += 1;
            }
            println!("Successfully loaded user program '{}'.", self.user_progs.last().unwrap().name.clone());
        } else {
            println!("User program '{}' failed to load -- instruction check failed!", self.user_progs.last().unwrap().name.clone());
            // remove failed program
            self.user_progs.pop();
        }
    }
    
    // return program reference for given name
    pub fn get_prog(&self, n: String) -> Option<&Program> {
        for p in &self.user_progs {
            if p.name == n {
                return Some(p);
            }
        }
        return None;
    }
    // Register manipulation
    pub fn get_reg(&self, idx: usize) -> u32 {
        return self.register_file[idx];
    }
    pub fn set_reg(&mut self, idx: usize, val: u32) {
        self.register_file[idx] = val;
    }
    // Memory reads
    // dword, word, byte
    pub fn get_mem_dw(&self, addr: u32) -> u32 {
        if addr >= self.mem_limit {
            panic!("Memory access of address {:#04X?} is out of bounds!", addr);
        }
        return self.memory[addr as usize];
    }
    pub fn get_mem_w(&self, addr: u32) -> u32 {
        if addr >= self.mem_limit {
            panic!("Memory access of address {:#04X?} is out of bounds!", addr);
        }
        return self.memory[addr as usize] & 0xFFFF;
    }
    pub fn get_mem_b(&self, addr: u32) -> u32 {
        if addr >= self.mem_limit {
            panic!("Memory access of address {:#04X?} is out of bounds!", addr);
        }
        return self.memory[addr as usize] & 0xFF;
    }
    // Memory writes
    // dword, word, byte
    pub fn set_mem_dw(&mut self, addr: u32, val: u32) {
        if addr >= self.mem_limit {
            panic!("Memory access of address {:#04X?} is out of bounds!", addr);
        }
        self.memory[addr as usize] = val;
    }
    pub fn set_mem_w(&mut self, addr: u32, val: u16) {
        if addr >= self.mem_limit {
            panic!("Memory access of address {:#04X?} is out of bounds!", addr);
        }
        self.memory[addr as usize] &= 0xFFFF0000;
        self.memory[addr as usize] |= ((val as u16) & 0xFFFF) as u32;
    }
    pub fn set_mem_b(&mut self, addr: u32, val: u8) {
        if addr >= self.mem_limit {
            panic!("Memory access of address {:#04X?} is out of bounds!", addr);
        }
        self.memory[addr as usize] &= 0xFFFFFF00;
        self.memory[addr as usize] |= ((val as u8) & 0xFF) as u32;
    }
    // ALU support
    // This updates the internal state of the ALU
    fn set_alu(&mut self, arg1: u32, arg2: u32) {
        self.alu.input1 = arg1;
        self.alu.input2 = arg2;
    }
    pub fn alu_op(&mut self, instr: Instruction, arg1: u32, arg2: u32) -> u32 {
        self.set_alu(arg1, arg2);
        let mut ret_val: u32 = 0;
        match instr {
            Instruction::Add => ret_val = self.alu.add(),
            Instruction::Sub => ret_val = self.alu.sub(),
            Instruction::Mul => ret_val = self.alu.mul(),
            Instruction::Div => ret_val = self.alu.div(),
            Instruction::Lshf => ret_val = self.alu.lshf(),
            Instruction::Rshf => ret_val = self.alu.rshf(),
            Instruction::Xor => ret_val = self.alu.xor(),
            Instruction::And => ret_val = self.alu.and(),
            Instruction::Or => ret_val = self.alu.or(),
            Instruction::Not => ret_val = self.alu.not(),
            _ => panic!("Error: Invalid ALU operation {:?}!", instr)
        }
        return ret_val;
    }
}

// Debug functions
pub fn __debug_memdump_all(hw: &Sc0Hardware) {
    let mut f = File::create("memdump.out").expect("Could not create memory dump debug file!");
    for (addr, val) in hw.memory.iter().enumerate() {
        writeln!(&mut f, "Address {:#06X} = {:#06X}", addr, val).unwrap();
    }
    println!("\n================Debug memory dump finished================\n");
}