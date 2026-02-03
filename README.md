# ELF analyzer

```bash
$ cargo add goblin clap colored
# 수동으로 Cargo.toml 직접 편집하는 것과 동일
```

goblin: ELF 바이너리 파싱 (헤더, 섹션, 심볼 등)  
clap: 커맨드라인 옵션 처리 (--help, 인자 파싱)

sample elf
```bash
git clone https://github.com/JonathanSalwan/binary-samples
cargo run -- binary-samples/elf-Linux-x64-bash
```