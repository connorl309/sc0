pub mod helpers;
pub mod cpu;

use std::process::exit;

use crate::cpu::hardware::{initialize, __debug_memdump};
use crate::helpers::{cli::*};
use crate::cpu::*;

fn main() {
    println!("Welcome to the SC0. Please enter a command, or type '?' for a list of commands.");
    /*
    let mut sc0 = initialize(0xFFFF + 1);
    __debug_memdump(&sc0);
    */
    let mut close = false;
    // how do i fix this lol
    while !close {
        match poll_input() {
            Inputs::Help => commands(),
            Inputs::Load(name) => load(name.clone()),
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
