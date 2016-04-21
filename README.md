[![Build Status](https://travis-ci.org/cole14/rust-readelf.svg?branch=master)](https://travis-ci.org/cole14/rust-readelf)

# rust-readelf
Pure-Rust implementation of the binutils utility readelf

## Options Usage:
```$ rust-readelf --help
Usage:
    ./target/debug/rust-readelf [OPTIONS]

Display information about the contents of ELF files

optional arguments:
  -h,--help             show this help message and exit
  -h,--file-header      Display the ELF file header
  -l,--program-headers  Display the program headers
  -S,--section-headers  Display the section headers
  -f,--file-name FILE_NAME
                        ELF file to inspect
```

