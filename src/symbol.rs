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

/**
 * tests 모듈은 symbol 모듈 안에 중첩되어 있어서, 바깥에 있는 get_functions 함수에 접근하려면 super::를 붙여야 합니다.
 *
 * 비유하자면 파일 시스템의 ../와 비슷합니다:
 *
 * super → 부모 모듈 (..)
 * self → 현재 모듈 (.)
 * crate → 크레이트 루트 (/)
 */
#[cfg(test)]
mod tests {
    use std::fs;
    // use super::*;
    #[test]
    fn test_get_functions_returns_sorted() {
        let data = fs::read("elf-Linux-x86-bash").unwrap(); // location 시작 기준은 cargo project
        let elf = goblin::elf::Elf::parse(&data).unwrap();
        let funcs = super::get_functions(&elf, 5);

        for i in 1..funcs.len() {
            assert!(funcs[i - 1].size >= funcs[i].size);
        }
    }
}