[![](https://img.shields.io/crates/v/readelf.svg)](https://crates.io/crates/readelf)

# rust-readelf
Pure-Rust implementation of the utility readelf using the `elf` crate.

## Options Usage:
```sh
$ rust-readelf --help
A pure-rust implementation of the utility readelf

Usage: rust-readelf [OPTIONS] --file-name <FILE_NAME>

Options:
  -f, --file-name <FILE_NAME>
      --file-header
      --program-headers
      --section-headers
      --symbols
      --dynamic-symbols
      --dynamic
      --relocations
      --notes
  -h, --help                   Print help information
  -V, --version                Print version information
```

