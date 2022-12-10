pub mod helpers;
pub mod cpu;
use crate::cpu::hardware::{initialize, __debug_memdump};
use crate::helpers::program::Program;
use crate::helpers::{cli::*};

fn main() {
    println!("Welcome to the SC0. Please enter a command, or type '?' for a list of commands.");
    
    let sc0 = initialize(0xFFFF + 1);
    let mut userProg: Program;
    __debug_memdump(&sc0);
    let mut close = false;
    // how do i fix this lol
    while !close {
        match poll_input() {
            Inputs::Help => commands(),
            Inputs::Load(name) => {
                userProg = load(name.clone());
            },
            Inputs::Select(name) => select(name.clone()),
            Inputs::Memdump(start, end) => memdump(start, end),
            Inputs::Regdump => regdump(),
            Inputs::Execute => execute(),
            Inputs::Run(count) => run(count),
            Inputs::Exit => close = true,
            Inputs::NULL => error(),
            Inputs::Error => error(),
        }
    }
}
