use std::{io::{BufRead, BufReader}, fs::File, process::exit};
use crate::cpu::isa::{ExecStatement, get_instr, Instruction};

pub struct Symbol {
    name: String,
    addr: u32
}
pub struct Program {
    pub name: String,
    pub instructions: Vec<ExecStatement>,
    pub start_pc: u32,
    pub sym_table: Vec<Symbol>,
}

pub fn load_prog(iname: String) -> Program {
    let mut i_list: Vec<ExecStatement> = Vec::new();
    let pc: u32 = 0; // handled later
    let name = iname.clone();

    let infile = File::open(iname).expect("Could not open user program file!");
    for line in BufReader::new(infile).lines() {
        // rust pls
        if line.as_ref().unwrap().len() == 0 {
            continue; // skip empty lines
        }
        
        // remove any comments
        let mut l = line.unwrap();
        if let Some((m, _)) = l.split_once("#") {
            l = String::from(m);
        }

        let mut splite: Vec<String> = l
            .split(&[',', ' ', '\t'])
            .filter(|&s| !s.is_empty())
            .map(|s| s.to_string()).collect();
        if splite.len() == 0 {
            continue;
        }
        let _opc = get_instr(splite[0].as_str());
        // todo: handle this error better, probably don't just quit.
        if _opc == Instruction::Error {
            println!("Error: invalid instruction {:?}. Exiting SC0!", splite[0]);
            exit(-1);
        }
        splite.remove(0); // get rid of instruction
        let exec = ExecStatement { opc: _opc, args: splite};
        i_list.push(exec);
    }
    return Program { name: name, instructions: i_list, start_pc: pc, sym_table: Vec::new() }
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
pub fn run_program(p: &mut Program, _count: u16) {

}