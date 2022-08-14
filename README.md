# crates.io-management
Cargo subcommand to management crates.io

## Install

```bash
cargo install --git https://github.com:lidongjies/crates.io-management.git --path . --force
```

## Usage

```
$ cargo-csm --help

cargo-csm 0.1.0 (dd79306 2022-08-14)
Jaye Lee <lidongjies@gmail.com>
A Cargo subcommand for management crates.io sources

USAGE:
    cargo-csm <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    add        Add one custom source
    current    Show current source name
    del        Delete one custom source
    help       Print this message or the help of the given subcommand(s)
    list       List all the sources
    use        Change crates-io replace-with to custom source
```
