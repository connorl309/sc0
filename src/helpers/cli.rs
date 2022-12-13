use std::{io::{self, Write}, process::exit};
use super::program::{Program, load_prog, run_program};

#[derive(PartialEq)]
#[derive(Clone)]
pub enum Inputs {
    Help,
    Load(String),
    Select(String),
    Memdump(u16, u16),
    Regdump,
    Execute,
    Run(u16),
    Debug(String),
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
        "exit" => Inputs::Exit,
        "?" => Inputs::Help,
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
            let start = input_split[1].parse::<u16>().unwrap();
            let end = input_split[2].parse::<u16>().unwrap();
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
        "debug" => {
            if input_split.len() != 2 {
                println!("Please enter only one/a user program name!");
                return Inputs::Error;
            }
            Inputs::Debug(String::from(input_split[1]))
        },
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
    println!("execute                - execute the currently selected program until halt");
    println!("run <n>                - runs the currently selected program for N instructions");
}
pub fn quit() {
    exit(0);
}
pub fn load(_pname: String) -> Program {
    return load_prog(_pname);
}
pub fn select(_pname: String) {
    println!("<select> command not implemented!");
}
pub fn memdump(_start: u16, _end: u16) {
    println!("<memdump> command not implemented!");
}
pub fn regdump() {
    println!("<regdump> command not implemented!");
}
pub fn execute() {
    println!("<execute> command not implemented!");
}
pub fn run(p: &mut Program, _count: u16) {
    println!("<run> command not implemented!");
}
pub fn error() {
    println!("A parse error occurred. I don't know what happened!");
}

// EOF