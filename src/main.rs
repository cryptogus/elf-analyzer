mod symbol;

use goblin::elf::Elf;
use std::env;
use std::fs;
use colored::*;

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

    // Read file
    let path = &args[1];
    let data = fs::read(path).expect("Can not read file");

    // ELF parsing
    let elf = Elf::parse(&data).expect("Fail to parsing ELF");

    // Header information
    println!("{}","=== ELF Header ===".cyan().bold());
    println!("Entry point: {}", format!("0x{:x}", elf.entry).yellow());
    println!("Machine: {:?}", elf.header.e_machine);
    println!("Type: {:?}", elf.header.e_type);
    println!(
        "Endian: {}",
        if elf.little_endian { "Little" } else { "Big" }
    );

    // Section
    println!("\n=== Sections ===");
    println!("{:<20} {:>12} {:>12}", "Name".bold(), "Size".bold(), "Addr".bold());
    println!("{}", "-".repeat(64));

    for sh in &elf.section_headers {
        let name = elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("???");
        println!("{:<20} {:>12} 0x{:08x}", name, sh.sh_size, sh.sh_addr);
    }

    // Symbol
    const TOP_NUM: usize = 10;
    let elf = Elf::parse(&data).unwrap();
    let funcs = symbol::get_functions(&elf, TOP_NUM);

    println!("\n=== Top {} Functions ===", TOP_NUM);
    println!("{:40} {:>10} {:>12}", "Name", "Size", "Addr");
    println!("{}", "-".repeat(64));

    for f in &funcs {
        let name = if f.name.len() > 38 {
            format!("{}...", &f.name[..35]) // Create new String
        } else {
            f.name.clone() // Copy String
        };
        println!("{:<40} {:>10} 0x{:08x}", name, f.size, f.addr);
    }

    // Memory Usage
    println!("\n{}", "=== Memory Usage ===".cyan().bold());
    let sections = [".text", ".rodata", ".data", ".bss"];
    for name in sections {
        if let Some(sh) = elf.section_headers.iter()
            .find(|s| elf.shdr_strtab.get_at(s.sh_name) == Some(name)) 
        {
            let bar_len = (sh.sh_size as usize / 1024).min(40).max(1);
            let bar = "█".repeat(bar_len);
            let colored_bar: ColoredString = match name {
                ".text" => bar.blue(),
                ".rodata" => bar.cyan(),
                ".data" => bar.green(),
                ".bss" => bar.yellow(),
                _ => bar.normal(),
            };
            println!("{:<10} {:>8} bytes {}", name, sh.sh_size, colored_bar);
        }
    }
}
