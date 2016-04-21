extern crate elf;
extern crate argparse;

use std::path::PathBuf;
use argparse::{ArgumentParser, StoreTrue, Store};

fn main() {
    let mut file_header = false;
    let mut program_headers = false;
    let mut section_headers = false;
    let mut filename = "".to_string();
    {
        let mut ap = ArgumentParser::new();
        ap.set_description(
            "Display information about the contents of ELF files");
        ap.refer(&mut file_header)
            .add_option(&["-h", "--file-header"], StoreTrue,
                        "Display the ELF file header");
        ap.refer(&mut program_headers)
            .add_option(&["-l", "--program-headers"], StoreTrue,
                        "Display the program headers");
        ap.refer(&mut section_headers)
            .add_option(&["-S", "--section-headers"], StoreTrue,
                        "Display the section headers");
        ap.refer(&mut filename)
            .add_option(&["-f", "--file-name"], Store,
                        "ELF file to inspect");
        ap.parse_args_or_exit();
    }

    let path: PathBuf = From::from(filename);
    let file = match elf::File::open_path(&path) {
        Ok(f) => f,
        Err(e) => panic!("Error: {:?}", e),
    };

    if file_header {
        println!("{}", file.ehdr);
    }
    if program_headers {
        for phdr in file.phdrs {
            println!("{}", phdr);
        }
    }
    if section_headers {
        for s in file.sections {
            println!("{}", s.shdr);
        }
    }
}
