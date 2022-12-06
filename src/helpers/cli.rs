pub fn pollInput() {
    
}

pub fn commands() {
    println!("\nCommand List");
    println!("?                      - displays this list");
    println!("load <program>         - load the specified program into memory");
    println!("select <program>       - selects the specified program as target");
    println!("memdump <mem1> <mem2>  - dump memory in the range specified by mem1 and mem2");
    println!("regdump                - dump all register and flag information");
    println!("execute                - execute the currently selected program until halt");
    println!("run <n>                - runs the currently selected program for N instructions");
    println!("\n");
}