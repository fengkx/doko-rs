# doko-rs

Rust implementation of https://github.com/egoist/doko

# Why

For Rust learning and pratices and more custom options like image tag.

This is my first rust project so it might contain some bad code, please raise an issue if you find any room for improvment.

# Usage

In most case it is the same as [doko](https://github.com/egoist/doko).

```plain
doko v0.1
A docker-based development dependency manager

USAGE:
    doko-rs [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    disable
    enable     enable service (help subcommand to list all available services)
    help       Prints this message or the help of the given subcommand(s)
```

```plain
doko-rs-enable
enable service

USAGE:
    doko-rs enable [SUBCOMMAND]

FLAGS:
    -h, --help    Prints help information

SUBCOMMANDS:
    chrome      Headless Chrome docker service
    mysql       MySQL docker service
    postgres    PostgreSQL docker service
    redis       Redis docker service
```
