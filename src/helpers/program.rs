#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use std::{io::{BufRead, BufReader}, fs::File, process::exit, ascii::AsciiExt};
use crate::cpu::isa::{ExecStatement, get_instr, Instruction};

pub struct Symbol {
    name: String,
    addr: u32
}
pub struct Program {
    pub name: String,
    pub obj_name: String,
    pub instructions: Vec<ExecStatement>,
    pub start_pc: u32,
    pub sym_table: Vec<Symbol>,
}
impl Program {
    pub fn labelOffset(&self, labName: String, pc: u32) -> u32 {
        for i in &self.sym_table {
            if i.name == labName {
                return i.addr - pc;
            }
        }
        return u32::MAX;
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
        // special check for .string
        
        if l.to_ascii_lowercase().contains(".string") {
            let tempOp = Instruction::STRING;
            let mut oneArg: Vec<String> = Vec::new();
            oneArg.push(l[8..].to_string());
            println!("{}", oneArg[0]);
            i_list.push(ExecStatement {opc: tempOp, args: oneArg});
            continue;
        }
        let mut splite: Vec<String> = l
            .to_ascii_lowercase()
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
        // special check for branches
        if _opc == Instruction::Branch {
            // we can pop the first two chars from the line string
            // and the remainder is the CC
            let mut temp = splite[0].chars();
            temp.next(); temp.next(); // remove "br"
            splite.insert(1, String::from(temp.as_str()));
        }
        splite.remove(0); // get rid of instruction
        let exec = ExecStatement { opc: _opc, args: splite};
        i_list.push(exec);
    }
    return Program { name: name, obj_name: String::new(), instructions: i_list, start_pc: pc, sym_table: Vec::new() }
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
pub fn run_program(p: &mut Program, _count: u16) {
    // do things...
}