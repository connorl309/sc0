use super::isa::Instruction;

/**
 * This file contains all definitions and funcs needed for
 * the "hardware" of the SC0
 * 
 * Register file, a (basic) datapath (which might not even be implemented initially...),
 * memory, and "I/O" (software stdin/stdio)
 */

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

pub struct SC0_Hardware {
    register_file: [i32; 16], // note that [13-15] are explicitly reserved
    memory: Vec<u32>,
    alu: ALU,
    mem_limit: u32
}

impl SC0_Hardware {
    // Initialization
    pub fn initialize(&mut self, limit: u32) {
        self.mem_limit = limit;
        self.memory = vec![0; self.mem_limit as usize];
        self.set_alu(0, 0);
        self.register_file = [0; 16];
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