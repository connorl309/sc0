/**
 * This file contains all functions and relevant info for the CLI
 * of the SC0 simulator.
 * 
 */
use std::{io::{self, Write}, process::exit};
use crate::cpu::hardware::Sc0Hardware;

use super::program::{Program, load_prog};

#[derive(PartialEq)]
#[derive(Clone)]
pub enum Inputs {
    Help,
    Load(String),
    Select(String),
    Memdump(u32, u32),
    Regdump,
    Execute,
    Run(u16),
    Debug,
    Exit,
    NULL,
    Error
}

/**
 * Kind of a nasty function, but it's a "parser". It's bound to be ugly.
 * Reads in user input.
 */
pub fn poll_input() -> Inputs {
    print!("[SC0 Sim] > ");
    let _ = io::stdout().flush(); // so formatting is nice
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read user input!");
    input = input.replace(&['\r', '\n'], ""); // remove any delimiters
    let input_split = input.split(" ").collect::<Vec<_>>(); // split input (just in case.)

    // Need error checking for inputs, just in case
    return match input_split[0] {
        "exit" | "e" | "quit" | "q" => Inputs::Exit,
        "?" | "help" | "commands" | "h" => Inputs::Help,
        "load" => {
            if input_split.len() != 2 {
                println!("\nPlease specify a program to load in the format 'load <program name here>'.");
                return Inputs::Error;
            }
            Inputs::Load(String::from(input_split[1]))
        },
        "select" => {
            if input_split.len() != 2 {
                println!("\nPlease select a program in the format 'select <program name here>'.");
                return Inputs::Error;
            }
            Inputs::Select(String::from(input_split[1]))
        },
        "memdump" => {
            if input_split.len() != 3 {
                println!("\nPlease specify a memory range in the format 'memdump START END'.");
                return Inputs::Error;
            }
            let start = input_split[1].parse::<u32>().unwrap();
            let end = input_split[2].parse::<u32>().unwrap();
            Inputs::Memdump(start, end)
        },
        "regdump" => Inputs::Regdump,
        "execute" => Inputs::Execute,
        "run" => {
            if input_split.len() != 2 {
                println!("\nPlease specify the number of instructions to run for in the format 'run NUMBER'.");
                return Inputs::Error;
            }
            let count = input_split[1].parse::<u16>().unwrap();
            Inputs::Run(count)
        },
        "debug" => Inputs::Debug,
        _ => Inputs::Error
    };
}
/**
 * CLI commands
 * 
 * These will eventually be implemented and 
 * indirectly access data such as program memory
 */
pub fn commands() {
    println!("\nCommand List");
    println!("?                      - displays this list");
    println!("exit                   - quit the SC0");
    println!("load <program>         - load the specified program into memory");
    println!("select <program>       - selects the specified program as target");
    println!("memdump <mem1> <mem2>  - dump memory in the range specified by mem1 and mem2");
    println!("regdump                - dump all register and flag information");
    println!("debug                  - dump all memory contents to file");
    println!("execute                - execute the currently selected program until halt");
    println!("run <n>                - runs the currently selected program for N instructions");
}
pub fn quit() {
    exit(0);
}
pub fn load(_pname: String) -> Program {
    return load_prog(_pname);
}
pub fn select(hw: &mut Sc0Hardware, _pname: String) {
    let copy = _pname.clone();
    match hw.get_prog(_pname) {
        Some(resulting_prog) => {
            hw.selected = resulting_prog.name.clone();
            println!("Selected program '{}'!", copy);
        }
        None => {
            println!("Program '{}' does not exist / hasn't been loaded into the SC0! Cannot select.", copy);
            hw.selected = "".to_string();
        }
    }
}
pub fn memdump(hw: &Sc0Hardware, _start: u32, _end: u32) {
    let mut start = _start;
    let mut f = std::fs::File::create("memdump.out").expect("Could not create memory dump debug file!");
    while start <= _end {
        writeln!(f, "ADDR [0x{:04X}] - 0x{:08X}", start, hw.get_mem_dw(start)).unwrap();
        start += 1;
    }
    println!("\n================Finished dump================\n");
}
pub fn regdump(hw: &Sc0Hardware) {
    println!("===========Register Dump===========");
    for (reg, val) in hw.register_file.iter().enumerate() {
        // special formatting checks
        if reg == 13 {
            println!("R13/Stack Pointer:\t0x{:08X}", val);
        }
        if reg == 14 {
            println!("R14/Program Counter:\t0x{:06X}", val);
        }
        if reg == 15 { // check if 32 is right or not
            println!("R15/Program Status Register:\t0b{:#032b}", val);
        } else {
            println!("R{}:\t0x{:08X}", reg as u8, val);
        }
    }
}
pub fn execute(hw: &Sc0Hardware) {
    let prog_ref = hw.get_prog(hw.selected.clone()).unwrap();
    run_hidden(&prog_ref, 0xFFFF);
}
pub fn run(hw: &Sc0Hardware, _p: String, _count: u16) {
    let prog_ref = hw.get_prog(hw.selected.clone()).unwrap();
    run_hidden(&prog_ref, _count);
}
// this is the actual run function
fn run_hidden(prog: &Program, limit: u16) {
    println!("RUN is not yet implemented");
}
// EOF