extern crate clap;
extern crate comfy_table;
extern crate elf;

use clap::Parser;
use comfy_table::{Cell, Table};
use elf::dynamic::DynIterator;
use elf::note::NoteIterator;
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

    #[arg(long)]
    dynamic: bool,

    #[arg(long)]
    notes: bool,
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

fn print_section_table(sections: elf::section::SectionHeaderIterator, strtab: StringTable) {
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
    for shdr in sections {
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

fn print_dynamic(dyns: DynIterator) {
    let mut table = Table::new();
    table.set_header(["d_tag", "d_ptr/d_val"]);
    for d in dyns {
        let cells: Vec<Cell> = vec![
            format!("{:#X?}", d.d_tag).into(),
            format!("{:#X?}", d.d_val()).into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

fn print_notes(notes: NoteIterator) {
    let mut table = Table::new();
    table.set_header(["type", "name", "desc"]);
    for note in notes {
        let cells: Vec<Cell> = vec![
            note.n_type.into(),
            note.name.into(),
            format!("{:02X?}", note.desc).into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

fn main() {
    let args = Args::parse();

    let path: PathBuf = From::from(args.file_name);
    let io = std::fs::File::open(path).expect("Could not open file.");

    let mut elf_file =
        elf::File::open_stream(elf::CachedReadBytes::new(io)).expect("Failed to open ELF stream");

    if args.file_header {
        let ehdr = &elf_file.ehdr;
        println!("{ehdr}");
    }

    if args.program_headers {
        let mut phdrs = elf_file.segments().expect("Failed to parse Segment Table");
        print_program_headers(&mut phdrs);
    }

    if args.section_headers {
        let (shdrs, strtab) = elf_file
            .section_headers_with_strtab()
            .expect("Failed to read section table and string table");
        print_section_table(shdrs, strtab);
    }

    if args.symbols {
        let tables = elf_file
            .symbol_table()
            .expect("Failed to get .symtab and string table");
        match tables {
            Some(tables) => {
                let (symtab, strtab) = tables;
                print_symbol_table(&symtab, &strtab);
            }
            None => (),
        }
    }

    if args.dynamic_symbols {
        let tables = elf_file
            .dynamic_symbol_table()
            .expect("Failed to get .dynsym and string table");
        match tables {
            Some(tables) => {
                let (symtab, strtab) = tables;
                print_symbol_table(&symtab, &strtab);
            }
            None => (),
        }
    }

    if args.dynamic {
        let dyns = elf_file.dynamic_section().expect("Failed to get .dynamic");
        match dyns {
            Some(dyns) => {
                print_dynamic(dyns);
            }
            None => (),
        }
    }

    if args.notes {
        let shdrs: Vec<elf::section::SectionHeader> = elf_file
            .section_headers()
            .expect("Failed to parse section headers")
            .collect();
        for ref shdr in shdrs {
            if shdr.sh_type != elf::gabi::SHT_NOTE {
                continue;
            }
            let notes = elf_file
                .section_data_as_notes(shdr)
                .expect("Failed to read notes section");
            print_notes(notes);
        }
    }
}
