use goblin::elf::Elf;
use std::env;
use std::fs;

fn main() {
    // // TEST args 1
    // let args_iterator: std::env::Args = env::args();
    // println!("{:?}", args_iterator);
    // // TEST args 2
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <elf-file>", args[0]);
        std::process::exit(1);
    }
}
