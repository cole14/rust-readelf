extern crate clap;
extern crate comfy_table;
extern crate elf;

use clap::Parser;
use comfy_table::{Cell, Table};
use elf::endian::AnyEndian;
use elf::note::Note;
use elf::relocation::{RelIterator, RelaIterator};
use elf::ElfStream;
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
    relocations: bool,

    #[arg(long)]
    notes: bool,
}

//  _____ _ _      _   _                _
// |  ___(_) | ___| | | | ___  __ _  __| | ___ _ __
// | |_  | | |/ _ \ |_| |/ _ \/ _` |/ _` |/ _ \ '__|
// |  _| | | |  __/  _  |  __/ (_| | (_| |  __/ |
// |_|   |_|_|\___|_| |_|\___|\__,_|\__,_|\___|_|
//

fn print_file_header(ehdr: &elf::file::FileHeader) {
    println!(
        "File Header for {} {} Elf {} for {} {}",
        ehdr.class,
        ehdr.ei_data,
        elf::to_str::e_type_to_string(ehdr.e_type),
        elf::to_str::e_osabi_to_string(ehdr.osabi),
        elf::to_str::e_machine_to_string(ehdr.e_machine)
    );
}

//  ____                                      _   _                _
// |  _ \ _ __ ___   __ _ _ __ __ _ _ __ ___ | | | | ___  __ _  __| | ___ _ __ ___
// | |_) | '__/ _ \ / _` | '__/ _` | '_ ` _ \| |_| |/ _ \/ _` |/ _` |/ _ \ '__/ __|
// |  __/| | | (_) | (_| | | | (_| | | | | | |  _  |  __/ (_| | (_| |  __/ |  \__ \
// |_|   |_|  \___/ \__, |_|  \__,_|_| |_| |_|_| |_|\___|\__,_|\__,_|\___|_|  |___/
//

fn print_program_headers(elf_file: &mut ElfStream<AnyEndian, std::fs::File>) {
    let phdrs = match elf_file.segments().expect("Failed to parse Segment Table") {
        Some(phdrs) => phdrs,
        None => {
            return;
        }
    };

    let mut table = Table::new();
    table.set_header([
        "p_type", "p_offset", "p_vaddr", "p_paddr", "p_filesz", "p_memsz", "p_align", "p_flags",
    ]);
    for phdr in phdrs {
        let cells: Vec<Cell> = vec![
            elf::to_str::p_type_to_string(phdr.p_type).into(),
            format!("{:#x}", phdr.p_offset).into(),
            format!("{:#x}", phdr.p_vaddr).into(),
            format!("{:#x}", phdr.p_paddr).into(),
            format!("{:#x}", phdr.p_filesz).into(),
            format!("{:#x}", phdr.p_memsz).into(),
            phdr.p_align.into(),
            elf::to_str::p_flags_to_string(phdr.p_flags).into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

//  ____            _   _             _   _                _
// / ___|  ___  ___| |_(_) ___  _ __ | | | | ___  __ _  __| | ___ _ __ ___
// \___ \ / _ \/ __| __| |/ _ \| '_ \| |_| |/ _ \/ _` |/ _` |/ _ \ '__/ __|
//  ___) |  __/ (__| |_| | (_) | | | |  _  |  __/ (_| | (_| |  __/ |  \__ \
// |____/ \___|\___|\__|_|\___/|_| |_|_| |_|\___|\__,_|\__,_|\___|_|  |___/
//

fn print_section_table(elf_file: &mut ElfStream<AnyEndian, std::fs::File>) {
    let (shdrs, strtab) = elf_file
        .section_headers_with_strtab()
        .expect("Failed to read section table and string table");
    let mut table = Table::new();
    table.set_header([
        "index",
        "name",
        "sh_type",
        "sh_addr",
        "sh_offset",
        "sh_size",
        "sh_entsize",
        "sh_flags",
        "sh_link",
        "sh_info",
        "sh_addralign",
    ]);
    for (ndx, shdr) in shdrs.iter().enumerate() {
        let name = strtab
            .get(shdr.sh_name as usize)
            .expect("Failed to get name from string table");
        let cells: Vec<Cell> = vec![
            ndx.into(),
            name.into(),
            elf::to_str::sh_type_to_string(shdr.sh_type).into(),
            format!("{:#x}", shdr.sh_addr).into(),
            format!("{:#x}", shdr.sh_offset).into(),
            format!("{:#x}", shdr.sh_size).into(),
            shdr.sh_entsize.into(),
            format!("{:#x}", shdr.sh_flags).into(),
            shdr.sh_link.into(),
            shdr.sh_info.into(),
            shdr.sh_addralign.into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

//  ____                  _           _ _____     _     _
// / ___| _   _ _ __ ___ | |__   ___ | |_   _|_ _| |__ | | ___
// \___ \| | | | '_ ` _ \| '_ \ / _ \| | | |/ _` | '_ \| |/ _ \
//  ___) | |_| | | | | | | |_) | (_) | | | | (_| | |_) | |  __/
// |____/ \__, |_| |_| |_|_.__/ \___/|_| |_|\__,_|_.__/|_|\___|
//        |___/
//

fn print_symbol_table(elf_file: &mut ElfStream<AnyEndian, std::fs::File>) {
    let (symtab, strtab) = match elf_file
        .symbol_table()
        .expect("Failed to get .symtab and string table")
    {
        Some(tables) => tables,
        None => {
            return;
        }
    };

    let mut table = Table::new();
    table.set_header([
        "ndx",
        "value",
        "size",
        "type",
        "bind",
        "visibility",
        "shndx",
        "name",
    ]);
    for (ndx, sym) in symtab.iter().enumerate() {
        let name = strtab
            .get(sym.st_name as usize)
            .expect("Failed to get name from string table");
        let cells: Vec<Cell> = vec![
            ndx.into(),
            format!("{:#x}", sym.st_value).into(),
            sym.st_size.into(),
            elf::to_str::st_symtype_to_string(sym.st_symtype()).into(),
            elf::to_str::st_bind_to_string(sym.st_bind()).into(),
            elf::to_str::st_vis_to_string(sym.st_vis()).into(),
            sym.st_shndx.into(),
            name.into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

//  ____              ____
// |  _ \ _   _ _ __ / ___| _   _ _ __ ___  ___
// | | | | | | | '_ \\___ \| | | | '_ ` _ \/ __|
// | |_| | |_| | | | |___) | |_| | | | | | \__ \
// |____/ \__, |_| |_|____/ \__, |_| |_| |_|___/
//        |___/             |___/

fn print_dynamic_symbol_table(elf_file: &mut ElfStream<AnyEndian, std::fs::File>) {
    // Get the .dynsym table. If this file doesn't have one, then we're done
    let (dynsyms, dynstrs) = match elf_file
        .dynamic_symbol_table()
        .expect("Failed to get .dynsym and string table")
    {
        Some(tables) => tables,
        None => {
            return;
        }
    };

    // Parse out all the symbols so that we can look up versions for them if needed.
    let symbols: Vec<(String, elf::symbol::Symbol)> = dynsyms
        .iter()
        .map(|sym| {
            (
                dynstrs
                    .get(sym.st_name as usize)
                    .expect("Failed to get symbol name")
                    .to_string(),
                sym,
            )
        })
        .collect();

    let vertab = elf_file
        .symbol_version_table()
        .expect("Failed to parse GNU symbol versions");

    let mut table = Table::new();
    table.set_header([
        "ndx",
        "value",
        "size",
        "type",
        "bind",
        "visibility",
        "shndx",
        "needs version",
        "name",
    ]);
    for (sym_idx, (sym_name, sym)) in symbols.iter().enumerate() {
        let needs_name = match &vertab {
            Some(vertab) => {
                if sym.is_undefined() {
                    match vertab
                        .get_requirement(sym_idx)
                        .expect("Failed to parse symbol requirement")
                    {
                        Some(req) => req.name,
                        None => "None",
                    }
                } else {
                    "None"
                }
            }
            None => "None",
        };
        let cells: Vec<Cell> = vec![
            sym_idx.into(),
            format!("{:#x}", sym.st_value).into(),
            sym.st_size.into(),
            elf::to_str::st_symtype_to_string(sym.st_symtype()).into(),
            elf::to_str::st_bind_to_string(sym.st_bind()).into(),
            elf::to_str::st_vis_to_string(sym.st_vis()).into(),
            sym.st_shndx.into(),
            needs_name.into(),
            sym_name.into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

//          _                             _
//       __| |_   _ _ __   __ _ _ __ ___ (_) ___
//      / _` | | | | '_ \ / _` | '_ ` _ \| |/ __|
//  _  | (_| | |_| | | | | (_| | | | | | | | (__
// (_)  \__,_|\__, |_| |_|\__,_|_| |_| |_|_|\___|
//            |___/
//

fn print_dynamic(elf_file: &mut ElfStream<AnyEndian, std::fs::File>) {
    let dyns = match elf_file.dynamic_section().expect("Failed to get .dynamic") {
        Some(dyns) => dyns,
        None => {
            return;
        }
    };

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

//           _
//  _ __ ___| | ___   ___ ___
// | '__/ _ \ |/ _ \ / __/ __|
// | | |  __/ | (_) | (__\__ \
// |_|  \___|_|\___/ \___|___/
//

fn print_rels(rels: RelIterator<AnyEndian>) {
    let mut table = Table::new();
    table.set_header(["r_type", "r_sym", "r_offset"]);
    for r in rels {
        let cells: Vec<Cell> = vec![
            format!("{:#X?}", r.r_type).into(),
            format!("{:#X?}", r.r_sym).into(),
            format!("{:#X?}", r.r_offset).into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

fn print_relas(relas: RelaIterator<AnyEndian>) {
    let mut table = Table::new();
    table.set_header(["r_type", "r_sym", "r_offset", "r_addend"]);
    for r in relas {
        let cells: Vec<Cell> = vec![
            format!("{:#X?}", r.r_type).into(),
            format!("{:#X?}", r.r_sym).into(),
            format!("{:#X?}", r.r_offset).into(),
            format!("{:#X?}", r.r_addend).into(),
        ];
        table.add_row(cells);
    }
    println!("{table}");
}

fn print_relocations(elf_file: &mut ElfStream<AnyEndian, std::fs::File>) {
    let shdrs: Vec<elf::section::SectionHeader> = elf_file
        .section_headers()
        .iter()
        .filter(|shdr| shdr.sh_type == elf::abi::SHT_REL || shdr.sh_type == elf::abi::SHT_RELA)
        .map(|shdr| *shdr)
        .collect();

    for ref shdr in shdrs {
        if shdr.sh_type == elf::abi::SHT_REL {
            let rels = elf_file
                .section_data_as_rels(shdr)
                .expect("Failed to read notes section");
            print_rels(rels);
        } else if shdr.sh_type == elf::abi::SHT_RELA {
            let relas = elf_file
                .section_data_as_relas(shdr)
                .expect("Failed to read notes section");
            print_relas(relas);
        }
    }
}

//              _
//  _ __   ___ | |_ ___  ___
// | '_ \ / _ \| __/ _ \/ __|
// | | | | (_) | ||  __/\__ \
// |_| |_|\___/ \__\___||___/
//

fn print_notes(elf_file: &mut ElfStream<AnyEndian, std::fs::File>) {
    let (shdrs, strtab) = elf_file
        .section_headers_with_strtab()
        .expect("strtab should be readable");

    let shdrs_with_names: Vec<_> = shdrs
        .iter()
        .filter(|shdr| shdr.sh_type == elf::abi::SHT_NOTE)
        .map(|shdr| {
            let name = strtab
                .get(shdr.sh_name as usize)
                .expect("section name should parse");
            (name.to_string(), *shdr)
        })
        .collect();

    for (sh_name, shdr) in shdrs_with_names {
        let notes = elf_file
            .section_data_as_notes(&shdr)
            .expect("Failed to read notes section");

        println!("Displaying notes found in: {sh_name}");
        for note in notes {
            match note {
                Note::GnuAbiTag(abi) => {
                    let os_str = elf::to_str::note_abi_tag_os_to_str(abi.os)
                        .map_or(format!("{}", abi.os), |val| val.to_string());
                    println!(
                        "    OS: {os_str}, ABI: {}.{}.{}",
                        abi.major, abi.minor, abi.subminor
                    );
                }
                Note::GnuBuildId(build_id) => {
                    print!("    Build ID: ");
                    for byte in build_id.0 {
                        print!("{byte:02x}");
                    }
                    println!("");
                }
                Note::Unknown(any) => {
                    let mut table = Table::new();
                    table.set_header(["type", "name", "desc"]);
                    let cells: Vec<Cell> = vec![
                        any.n_type.into(),
                        any.name.into(),
                        format!("{:02X?}", any.desc).into(),
                    ];
                    table.add_row(cells);
                    println!("{table}");
                }
            }
        }
    }
}

fn main() {
    let args = Args::parse();

    let path: PathBuf = From::from(args.file_name);
    let io = std::fs::File::open(path).expect("Could not open file");

    let mut elf_file =
        ElfStream::<AnyEndian, _>::open_stream(io).expect("Failed to open ELF stream");

    if args.file_header {
        print_file_header(&elf_file.ehdr);
    }

    if args.program_headers {
        print_program_headers(&mut elf_file);
    }

    if args.section_headers {
        print_section_table(&mut elf_file);
    }

    if args.symbols {
        print_symbol_table(&mut elf_file);
    }

    if args.dynamic_symbols {
        print_dynamic_symbol_table(&mut elf_file);
    }

    if args.dynamic {
        print_dynamic(&mut elf_file);
    }

    if args.notes {
        print_notes(&mut elf_file);
    }

    if args.relocations {
        print_relocations(&mut elf_file);
    }
}
