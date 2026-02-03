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

    // Read file
    let path = &args[1];
    let data = fs::read(path).expect("Can not read file");

    // ELF parsing
    let elf = Elf::parse(&data).expect("Fail to parsing ELF");

    // Header information
    println!("=== ELF Header ===");
    println!("Entry point: 0x{:x}", elf.entry);
    println!("Machine: {:?}", elf.header.e_machine);
    println!("Type: {:?}", elf.header.e_type);
    println!("Endian: {}", if elf.little_endian {"Little"} else {"Big"});

    // Section
    println!("\n=== Sections ===");
    println!("{:<20} {:>12} {:>12}", "Name", "Size", "Addr");
    println!("{}", "-".repeat(64));

    for sh in &elf.section_headers {
        let name = elf.shdr_strtab.get_at(sh.sh_name).unwrap_or("???");
        println!("{:<20} {:>12} 0x{:08x}", name, sh.sh_size, sh.sh_addr);
    }
    
}
