extern crate clap;
extern crate elf;

use std::path::PathBuf;
use clap::Parser;

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
        for phdr in elf_file.phdrs {
            println!("{}", phdr);
        }
    }

    if args.section_headers {
        for s in elf_file.sections {
            println!("{}", s.shdr);
        }
    }
}
