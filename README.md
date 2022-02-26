# Caesar cipher

A small program that take a plaintext and apply the caesar cipher. Do the inverse operation too.

## Why ?
To learn rust. And maybe with test unit.

## Usage

```
$ caesar -h
rust-caesar 0.1.0

USAGE:
    rust-caesar.exe [OPTIONS]

OPTIONS:
    -h, --help                   Print help information
    -k, --key <KEY>              number applied to the rotation, by default 13 [default: 13]
    -t, --text <TEXT>            text (cipher or plaintext). If not use, will look on the stdin
    -T, --textfile <TEXTFILE>    file containing the text
    -V, --version                Print version information
```