pub mod helpers;
use crate::helpers::*;
fn main() {
    println!("Welcome to the SC0. Please enter a command, or type '?' for a list of commands.");
    helpers::cli::commands();
}
