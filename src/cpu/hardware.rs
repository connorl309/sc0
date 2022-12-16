#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(unused)]
use crate::helpers::program::Program;
use super::assemble::assemble;
use super::isa::{Instruction, check_args};
use std::fs::File;
use std::io::Write;

/**
 * This file contains all definitions and funcs needed for
 * the "hardware" of the SC0
 * 
 * Register file, a (basic) datapath (which might not even be implemented initially...),
 * memory, and "I/O" (software stdin/stdio)
 */

// List of syscall definitions
pub const HALT: i32 = 0xAAAA;
pub const PRINT: i32 = 0xBBBB;
pub const DISPLAY: i32 = 0xCCCC;
pub const DELAY: i32 = 0xDDDD;
pub const INPUT: i32 = 0xEEEE;

// "ALU", for math operations
// add, sub, mul, div, shifts, xor, and, or, not
// I probably dont need an impl for this,
// but i want to get practice with it.
struct ALU {
    input1: i32,
    input2: i32,
    output: i32
}
impl ALU {
    fn flush(&mut self) {
        self.input1 = 0;
        self.input1 = 0;
        self.output = 0;
    }
    fn add(&mut self) -> i32 {
        self.output = self.input1 + self.input2;
        return self.output;
    }
    fn sub(&mut self) -> i32 {
        self.output = self.input1 - self.input2;
        return self.output;
    }
    fn mul(&mut self) -> i32 {
        self.output = self.input1 * self.input2;
        return self.output;
    }
    fn div(&mut self) -> i32 {
        self.output = self.input1 / self.input2;
        return self.output;
    }
    fn lshf(&mut self) -> i32 {
        self.output = self.input1 << self.input2;
        return self.output;
    }
    fn rshf(&mut self) -> i32 {
        self.output = self.input1 >> self.input2;
        return self.output;
    }
    fn xor(&mut self) -> i32 {
        self.output = self.input1 ^ self.input2;
        return self.output;
    }
    fn and(&mut self) -> i32 {
        self.output = self.input1 & self.input2;
        return self.output;
    }
    fn or(&mut self) -> i32 {
        self.output = self.input1 | self.input2;
        return self.output;
    }
    fn not(&mut self) -> i32 {
        self.output = !self.input1; // rust uses ! for bitwise not. funny.
        return self.output;
    }
}

// Initialization of hardware object
pub fn initialize(limit: u32) -> Sc0Hardware {
    return Sc0Hardware { register_file: [0; 16], memory: vec![0; limit as usize], 
        alu: ALU{input1: 0, input2: 0, output: 0}, mem_limit: limit,
        user_progs: Vec::new() 
    }
}

pub struct Sc0Hardware {
    register_file: [i32; 16], // note that [13-15] are somewhat reserved
    memory: Vec<u32>,
    alu: ALU,
    mem_limit: u32,
    user_progs: Vec<Program>,
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
    pub fn get_reg(&self, idx: u8) -> i32 {
        return self.register_file[idx as usize];
    }
    pub fn set_reg(&mut self, idx: usize, val: i32) {
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
        self.memory[addr as usize] = (val as u32) & 0xFFFF;
    }
    pub fn set_mem_b(&mut self, addr: u32, val: u8) {
        if addr >= self.mem_limit {
            panic!("Memory access of address {:#04X?} is out of bounds!", addr);
        }
        self.memory[addr as usize] = (val as u32) & 0xFF;
    }
    // ALU support
    // This updates the internal state of the ALU
    pub fn set_alu(&mut self, arg1: i32, arg2: i32) {
        self.alu.input1 = arg1;
        self.alu.input2 = arg2;
    }
    pub fn alu_op(&mut self, instr: Instruction, arg1: i32, arg2: i32) -> i32 {
        self.set_alu(arg1, arg2);
        match instr {
            Instruction::Add => self.alu.add(),
            Instruction::Sub => self.alu.sub(),
            Instruction::Mul => self.alu.mul(),
            Instruction::Div => self.alu.div(),
            Instruction::Lshf => self.alu.lshf(),
            Instruction::Rshf => self.alu.rshf(),
            Instruction::Xor => self.alu.xor(),
            Instruction::And => self.alu.and(),
            Instruction::Or => self.alu.or(),
            Instruction::Not => self.alu.not(),
            _ => panic!("Error: Invalid ALU operation {:?}!", instr)
        }
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