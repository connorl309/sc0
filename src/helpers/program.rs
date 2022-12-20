#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
/**
 * This file contains all definitions and declarations for
 * a program object - filename, list of instructions, the starting PC value,
 * the corresponding symbol table, etc.
 */
use std::{io::{BufRead, BufReader}, fs::File, process::exit};
use crate::cpu::isa::{ExecStatement, get_instr, Instruction};

pub struct Symbol {
    name: String,
    pub addr: u32
}

pub struct Program {
    pub name: String,
    pub instructions: Vec<ExecStatement>,
    pub start_pc: u32,
    pub sym_table: Vec<Symbol>,
    pub scratchwork: u32,
}
impl Program {
    pub fn labelOffset(&self, labName: String, pc: u32) -> i32 {
        for i in &self.sym_table {
            if i.name == labName {
                return (i.addr as i32) - (pc as i32);
            }
        }
        return i32::MAX;
    }
    pub fn labelExists(&self, labName: String) -> bool {
        for elem in &self.sym_table {
            if elem.name == labName {
                return true;
            }
        }
        return false;
    }
}

pub fn load_prog(iname: String) -> Program {
    let mut i_list: Vec<ExecStatement> = Vec::new();
    let mut pc: u32 = 0; // handled later
    let mut syms: Vec<Symbol> = Vec::new();
    let mut labelPC: u32 = 0;
    let name = iname.clone();

    let infile = File::open(iname).expect("Could not open user program file!");
    for line in BufReader::new(infile).lines() {
        // rust pls
        if line.as_ref().unwrap().len() == 0 {
            continue; // skip empty lines
        }
        
        // remove any comments
        let mut l = line.unwrap().trim().to_string();
        if let Some((m, _)) = l.split_once(";") {
            l = String::from(m);
        }
        // rust pls (part two)
        if l.len() == 0 {
            continue; // skip empty lines
        }
        labelPC += 1;

        // first, formatting
        let fit_line = l.to_ascii_lowercase();
        let mut full_split: Vec<String> = fit_line.split(&[',', ' ', '\t',]).filter(|&s| !s.is_empty()).map(|s| s.to_string()).collect();
        // check if .orig
        if fit_line.contains(".orig") {
            full_split.remove(0); // remove the .ORIG part
            if full_split[0].starts_with("0x") {
                let temp = full_split[0].trim_start_matches("0x");
                pc = u32::from_str_radix(temp, 16).unwrap();
                labelPC = pc;
            } else if full_split[0].starts_with("#") {
                let temp = full_split[0].trim_start_matches("#");
                pc = u32::from_str_radix(temp, 10).unwrap();
                labelPC = pc;
            }
            if pc < 0xFF {
                println!("Error: program origin MUST be at or above 0xFF! Aborting...");
                exit(-1);
            }
            i_list.push(ExecStatement {
                opc: Instruction::ORIG,
                args: full_split,
            });
            continue;
        }
        // check if label(s)
        if full_split[0].contains(':') {
            full_split[0].pop(); // remove the :
            syms.push( Symbol {
               addr: labelPC, // unlike LC3, PC increments at the END of execution
               name: full_split[0].clone() 
            });
            full_split.remove(0); // remove label
        }
        // check .string
        if full_split[0].contains(".string") {
            let mut oneArg: Vec<String> = Vec::new();
            oneArg.push(fit_line[8..].to_string());
            i_list.push(ExecStatement {
                 opc: Instruction::STRING, args: oneArg
            });
            continue;
        }

        let opcode = get_instr(&full_split[0]);
        // check for errors (somehow)
        if opcode == Instruction::Error {
            println!("Invalid opcode '{}'! Exiting the SC0.", full_split[0]);
            // actual "error" handling: how to do....
            exit(-1);
        }
        // branch check, and get CC in right spot
        if opcode == Instruction::Branch {
            let mut temp = full_split[0].chars();
            temp.next(); temp.next();
            full_split.insert(1, temp.as_str().to_string());
        }
        // otherwise we are *probably* fine... do instructions as normal
        full_split.remove(0); // remove instruction (already stored in 'opcode')
        //println!("program.rs dbg: {:?}", full_split);
        i_list.push(ExecStatement {
            opc: opcode,
            args: full_split
        });
    }
    return Program { name: name, instructions: i_list, start_pc: pc, sym_table: syms, scratchwork: pc }
}

// debug info
pub fn __debug_progdump(p: &Program) {
    let mut ctr = 0;
    for exec in &p.instructions {
        ctr += 1;
        print!("LINE [{}]: ", ctr);
        for arg in &exec.args {
            print!("{:?} ", arg);
        }
        println!();
    }
}

// TODO
// Open "./objects/<progname>.object" and run
// man. pain.
pub fn run_program(p: &mut Program, _count: u16) {
    // do things...
}