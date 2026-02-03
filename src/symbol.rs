use goblin::elf::Elf;
use goblin::elf::sym::STT_FUNC;

pub struct FuncSymbol {
    pub name: String,
    pub size: u64,
    pub addr: u64,
}

pub fn get_functions(elf: &Elf, top_n: usize) -> Vec<FuncSymbol> {
    let mut funcs: Vec<_> = elf
        .syms
        .iter()
        .filter(|s| s.st_type() == STT_FUNC && s.st_size > 0)
        .map(|s| FuncSymbol {
            name: elf.strtab.get_at(s.st_name).unwrap_or("???").to_string(),
            size: s.st_size,
            addr: s.st_value,
        })
        .collect();

    funcs.sort_by(|a, b| b.size.cmp(&a.size));
    funcs.truncate(top_n);
    funcs
}
