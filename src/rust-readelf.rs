extern crate clap;
extern crate comfy_table;
extern crate elf;

use clap::Parser;
use comfy_table::{Cell, Table};
use elf::segment::SegmentIterator;
use elf::string_table::StringTable;
use elf::symbol::SymbolTable;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file_name: String,

    #[arg(long)]
    file_header: bool,

    #[arg(long)]
    program_headers: bool,

    #[arg(long)]
    section_headers: bool,

    #[arg(long)]
    symbols: bool,

    #[arg(long)]
    dynamic_symbols: bool,
}

fn print_program_headers(phdrs: &mut SegmentIterator) {
    let mut table = Table::new();
    table.set_header([
        "p_type", "p_offset", "p_vaddr", "p_paddr", "p_align", "p_filesz", "p_memsz", "p_flags",
    ]);
    for phdr in phdrs {
        let cells: Vec<Cell> = vec![
            phdr.p_type.into(),
            phdr.p_offset.into(),
            phdr.p_vaddr.into(),
            phdr.p_paddr.into(),
            phdr.p_align.into(),
            phdr.p_filesz.into(),
            phdr.p_memsz.into(),
            phdr.p_flags.into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

fn print_section_table(sections: Vec<elf::section::SectionHeader>, strtab: &StringTable) {
    let mut table = Table::new();
    table.set_header([
        "name",
        "sh_type",
        "sh_flags",
        "sh_addr",
        "sh_offset",
        "sh_size",
        "sh_link",
        "sh_info",
        "sh_addralign",
        "sh_entsize",
    ]);
    for shdr in sections.iter() {
        let name = strtab
            .get(shdr.sh_name as usize)
            .expect("Failed to get name from string table");
        let cells: Vec<Cell> = vec![
            name.into(),
            shdr.sh_type.into(),
            shdr.sh_flags.into(),
            shdr.sh_addr.into(),
            shdr.sh_offset.into(),
            shdr.sh_size.into(),
            shdr.sh_link.into(),
            shdr.sh_info.into(),
            shdr.sh_addralign.into(),
            shdr.sh_entsize.into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

fn print_symbol_table(symtab: &SymbolTable, strtab: &StringTable) {
    let mut table = Table::new();
    table.set_header([
        "name",
        "value",
        "size",
        "type",
        "bind",
        "visibility",
        "shndx",
    ]);
    for sym in symtab.iter() {
        let name = strtab
            .get(sym.st_name as usize)
            .expect("Failed to get name from string table");
        let cells: Vec<Cell> = vec![
            name.into(),
            sym.st_value.into(),
            sym.st_size.into(),
            sym.st_symtype().into(),
            sym.st_bind().into(),
            sym.st_vis().into(),
            sym.st_shndx.into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

fn main() {
    let args = Args::parse();

    let path: PathBuf = From::from(args.file_name);
    let io = std::fs::File::open(path).expect("Could not open file.");
    let mut c_io = elf::CachedReadBytes::new(io);

    let mut elf_file = elf::File::open_stream(&mut c_io).expect("Failed to open ELF stream");

    if args.file_header {
        let ehdr = &elf_file.ehdr;
        println!("{ehdr}");
    }

    if args.program_headers {
        let mut phdrs = elf_file.segments().expect("Failed to parse Segment Table");
        print_program_headers(&mut phdrs);
    }

    if args.section_headers {
        let shdrs: Vec<elf::section::SectionHeader> = elf_file
            .section_headers()
            .expect("Failed to parse Section Table")
            .collect();
        let strtab = elf_file
            .section_strtab()
            .expect("Failed to get section string table");
        print_section_table(shdrs, &strtab);
    }

    if args.symbols || args.dynamic_symbols {
        let tables: Option<(elf::symbol::SymbolTable, elf::string_table::StringTable)>;
        if args.symbols {
            tables = elf_file
                .symbol_table()
                .expect("Failed to get .symtab string table");
        } else {
            tables = elf_file
                .dynamic_symbol_table()
                .expect("Failed to get .dynsym string table");
        }

        match tables {
            Some(tables) => {
                let (symtab, strtab) = tables;
                print_symbol_table(&symtab, &strtab);
            }
            None => (),
        }
    }
}
