# Inline Crate

[<img src="https://img.shields.io/badge/inline--crate-blue?logo=rust&label=crates.io" alt="crates.io">](https://crates.io/crates/inline-crate)
[<img src="https://img.shields.io/badge/BSD_3_Clause-blue?logo=opensourceinitiative&label=License" alt="license">](https://github.com/jaybosamiya/inline-crate/blob/main/LICENSE)

A small helper tool to inline an entire Rust crate into a single file

## Install

``` sh
cargo install inline-crate --locked
```

## Usage

``` console
$ inline-crate --help
Inline an entire crate into a single file

Aimed at making it easy to distribute reproducers, or run minimizers.

If an output path is passed, then outputs to the file, otherwise outputs to stdout.

Usage: inline-crate [OPTIONS] <CRATE_ROOT> [OUTPUT_FILE]

Arguments:
  <CRATE_ROOT>
          The input crate root

  [OUTPUT_FILE]
          The output file (default: stdout)

Options:
  -v, --verbose
          Print verbose output

      --ignore-missing
          Ignore missing modules

  -f, --force
          Force writing, even if the file exists

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## License

[BSD 3-Clause License](./LICENSE)
