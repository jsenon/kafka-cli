# Simple Kafka CLI

Note: **Currently in DRAFT**

## Prerequisite

## Build

Build App:

```sh
RUSTFLAGS='-C link-args=-lzstd'  cargo build --release
```

## Usage

```sh
Kafka Cli 1.0
Julien Senon <julien.senon@gmail.com>
Producer and consumer kafka command line

USAGE:
    kafka-cli [FLAGS] [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -v               Sets the level of verbosity
    -V, --version    Prints version information

SUBCOMMANDS:
    consumer    Launch Consumer
    help        Prints this message or the help of the given subcommand(s)
    producer    Launch Consumer
```