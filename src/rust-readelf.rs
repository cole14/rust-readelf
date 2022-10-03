extern crate clap;
extern crate elf;

use std::path::PathBuf;
use clap::Parser;

/// Simple program to greet a person
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
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    if args.file_header {
        println!("{}", file.ehdr);
    }

    if args.program_headers {
        for phdr in file.phdrs {
            println!("{}", phdr);
        }
    }

    if args.section_headers {
        for s in file.sections {
            println!("{}", s.shdr);
        }
    }
}
