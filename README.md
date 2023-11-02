# Inline Crate

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
  -f, --force
          Force writing, even if the file exists

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```

## License

[BSD 3-Clause License](./LICENSE)
