use std::{io::{self, BufRead, BufReader}, fs::File};
use crate::cpu::isa::{ExecStatement, get_instr};

pub struct Program {
    name: String,
    instructions: Vec<ExecStatement>,
    start_pc: u32
}

pub fn load_prog(iname: String) -> Program {
    let mut i_list: Vec<ExecStatement> = Vec::new();
    let mut pc: u32 = 0;
    let name = iname.clone();

    let infile = File::open(iname).expect("Could not open user program file!");
    for line in BufReader::new(infile).lines() {
        let mut splite: Vec<String> = line.unwrap().split(&[',', ' ', '\t']).map(|s| s.to_string()).collect();
        let _opc = get_instr(splite[0].as_str());
        splite.remove(0); // get rid of first statement
        let exec = ExecStatement { opc: _opc, args: splite};
        i_list.push(exec);
    }
    return Program { name: name, instructions: i_list, start_pc: pc }
}

fn verify_prog(list: &Vec<ExecStatement>) -> bool {
    return true;
}