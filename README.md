# Numtimes
Numtimes is a simple command-line tool written in Rust that prints out the multiplication table of a number.

## Whats New?
The latest release (1.1) adds the following:
- **A new argument (--answer-only):** This argument will only print the result of every multiplication without the operation

A full list of changes for this release and all others before can be found [here](https://github.com/Terminnus95/numtimes/blob/main/CHANGELOG.md)

## Installation

### Through crates.io
Numtimes is available on [crates.io](https://crates.io/crates/numtimes), you can install it like this:
```
$ cargo install numtimes
```

### Install it From Source
To install it from source, you have to clone the repository and then build it with `cargo build`
```
$ git clone https://github.com/Terminnus95/numtimes.git
$ cargo build --release #Execute this in the directory where the cloned repo is stored
```
Then you can copy the binary into a folder in `$PATH` (usr/bin, ~/.local/bin, etc) to be able to execute it like any normal command

```
$ cp /path/to/repo/target/release/numtimes /your/preferred/path
```

### Pre-Built Binaries
You can download binaries for Linux and Windows on x86_64 [here](https://github.com/Terminnus95/numtimes/releases)

## Usage
```
numtimes [OPTIONS] <BASE> [LENGTH]
```

### Arguments
- **Base:** The base number of the multiplication table
- **Length:** Number of times the base number will be multiplied (default: 12)

### Options
- **-r, --reversed:** Reverse the order of the numbers («result = base×length» instead of «base×length = result»)
- **-a, --answer-only:** Only print out the answer
- **-h, --help:** Show the help screen
- **-V, --version:** Print version

