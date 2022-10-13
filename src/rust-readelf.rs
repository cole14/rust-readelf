extern crate clap;
extern crate elf;

use clap::Parser;
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

fn main() {
    let args = Args::parse();

    let path: PathBuf = From::from(args.file_name);
    let mut io = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    let elf_file = match elf::File::open_stream(&mut io) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    if args.file_header {
        println!("{}", elf_file.ehdr);
    }

    if args.program_headers {
        for phdr in &elf_file.phdrs {
            println!("{}", phdr);
        }
    }

    if args.section_headers {
        for s in elf_file.sections.iter() {
            println!("{}", s.shdr);
        }
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
                for (idx, sym) in symtab.iter().enumerate() {
                    let name = match strtab.get(sym.st_name as usize) {
                        Ok(name) => name,
                        Err(e) => panic!("Error: {:?}", e),
                    };
                    println!("{idx}: {:?} {}", sym, name);
                }
            }
            None => (),
        }
    }
}
