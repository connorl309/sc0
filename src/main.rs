pub mod helpers;
pub mod cpu;

use crate::cpu::hardware::*;
use crate::helpers::program::*;
use crate::helpers::{cli::*};

#[allow(non_snake_case)]
fn main() {
    println!("Welcome to the SC0. Please enter a command, or type '?' for a list of commands.");
    
    let mut sc0 = initialize(0xFFFF + 1);
    //__debug_memdump(&sc0);
    let mut close = false;
    // how do i fix this lol
    while !close {
        match poll_input() {
            Inputs::Help => commands(),
            Inputs::Load(name) => {
                sc0.add_prog(load_prog(name));
            },
            Inputs::Select(name) => select(name.clone()),
            Inputs::Memdump(start, end) => memdump(start, end),
            Inputs::Regdump => regdump(),
            Inputs::Execute => execute(),
            Inputs::Run(_count) => {
            },
            Inputs::Debug(name) => {
                if let Some(p) = sc0.get_prog(name) {
                    __debug_progdump(p);
                } else {
                    println!("Could not find specified program to dump!");
                }
                __debug_memdump(&sc0);
            }
            Inputs::Exit => close = true,
            Inputs::NULL => error(),
            Inputs::Error => error(),
        }
    }
}
