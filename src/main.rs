pub mod helpers;
pub mod cpu;
pub mod simulator;

use crate::cpu::hardware::*;
use crate::helpers::program::*;
use crate::helpers::{cli::*};

#[allow(non_snake_case)]
fn main() {
    println!("Welcome to the SC0. Please enter a command, or type '?' for a list of commands.");
    
    let mut sc0 = initialize(0xFFFF + 1);
    let mut close = false;
    // how do i fix this lol
    while !close {
        match poll_input() {
            Inputs::Help => commands(),
            Inputs::Load(name) => {
                sc0.add_prog(load_prog(name));
            },
            Inputs::Select(name) => select(&mut sc0, name.clone()),
            Inputs::Memdump(start, end) => memdump(&sc0, start, end),
            Inputs::Regdump => regdump(&sc0),
            Inputs::Execute => {
                if sc0.selected.len() == 0 {
                    println!("Please select a loaded program first!");
                    continue;
                }
                execute(&mut sc0);
            }
            Inputs::Debug => {
                println!("Executing debug total-memory dump...");
                __debug_memdump_all(&sc0);
            }
            Inputs::Exit => close = true,
            Inputs::NULL => println!("Invalid command!"),
            Inputs::Error => println!("Invalid command!")
        }
    }
}
