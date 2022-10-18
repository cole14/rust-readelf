extern crate clap;
extern crate comfy_table;
extern crate elf;

use clap::Parser;
use comfy_table::{Cell, Table};
use elf::section::SectionTable;
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

fn print_section_table(sections: &SectionTable, strtab: &StringTable) {
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
    for s in sections.iter() {
        let name = match strtab.get(s.shdr.sh_name as usize) {
            Ok(name) => name,
            Err(e) => panic!("Error: {:?}", e),
        };
        let cells: Vec<Cell> = vec![
            name.into(),
            s.shdr.sh_type.into(),
            s.shdr.sh_flags.into(),
            s.shdr.sh_addr.into(),
            s.shdr.sh_offset.into(),
            s.shdr.sh_size.into(),
            s.shdr.sh_link.into(),
            s.shdr.sh_info.into(),
            s.shdr.sh_addralign.into(),
            s.shdr.sh_entsize.into(),
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
        let name = match strtab.get(sym.st_name as usize) {
            Ok(name) => name,
            Err(e) => panic!("Error: {:?}", e),
        };
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
    let mut io = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let mut elf_file = match elf::File::open_stream(&mut io) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    if args.file_header {
        let ehdr = &elf_file.ehdr;
        println!("{ehdr}");
    }

    if args.program_headers {
        let mut phdrs = match elf_file.segments() {
            Ok(phdrs) => phdrs,
            Err(e) => panic!("Error: {:?}", e),
        };
        print_program_headers(&mut phdrs);
    }

    if args.section_headers {
        let strtab = match elf_file.section_strtab() {
            Ok(strtab) => strtab,
            Err(e) => panic!("Error: {:?}", e),
        };
        let sections = match elf_file.sections() {
            Ok(sections) => sections,
            Err(e) => panic!("Error: {:?}", e),
        };
        print_section_table(&sections, &strtab);
    }

    if args.symbols || args.dynamic_symbols {
        let tables: Option<(elf::symbol::SymbolTable, elf::string_table::StringTable)>;
        if args.symbols {
            tables = match elf_file.symbol_table() {
                Ok(tables) => tables,
                Err(e) => panic!("Error: {:?}", e),
            }
        } else {
            tables = match elf_file.dynamic_symbol_table() {
                Ok(tables) => tables,
                Err(e) => panic!("Error: {:?}", e),
            }
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
