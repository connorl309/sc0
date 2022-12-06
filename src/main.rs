pub mod helpers;
use std::{borrow::Borrow, clone};

use crate::helpers::{cli::*};
fn main() {
    println!("Welcome to the SC0. Please enter a command, or type '?' for a list of commands.");
    let mut input: Inputs = Inputs::NULL;

    // how do i fix this lol
    while input != Inputs::Exit {
        input = poll_input();
        match input {
            Inputs::Help => commands(),
            Inputs::Load(name) => load(String::from(name)),
            Inputs::Select(name) => select(String::from(name)),
            Inputs::Memdump(start, end) => memdump(start, end),
            Inputs::Regdump => regdump(),
            Inputs::Execute => execute(),
            Inputs::Run(count) => run(count),
            Inputs::Exit => quit(),
            Inputs::NULL => error(),
            Inputs::Error => error(),
        }
    }
}
