# Numtimes
Numtimes is a simple command line tool written in Rust that prints out the multiplication table of a number.

## Installation

### Trough crates.io
Numtimes is available on crates.io, you can install it like this:
```
$ cargo install numtimes
```

### Install it From Source
To install it from source, you have to clone the repository and then build it with `cargo build`
```
$ git clone https://github.com/Terminnus95/numtimes.git
$ cargo build --release #Execute this on the directory the cloned repo is stored
```
Then you can copy the binary into a folder in `$PATH` (usr/bin, ~/.local/bin, etc...) to be able to execute it like any normal command

```
$ cp /path/to/repo/target/release/numtimes /your/preferred/path
```

### Pre-Built Binaries
Pre-built binaries are coming soon, stay tuned!

## Usage
```
numtimes [OPTIONS] <BASE> [LENGTH]
```

### Arguments:
- **BASE:** The base number of the multiplication table
- **LENGTH:** Number of times the base number will be multiplied (default: 12)

### Options:
- **-r, --reversed:** Reverse the order of the numbers («result = base×length» instead of «base×length = result»)
- **-h, --help:** Show the help screen
- **-V, --version:** Print version

