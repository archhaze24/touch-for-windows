# touch-for-windows
Analogue of Linux `touch` command for Windows, written in Rust.

## Installation:

### [Scoop](https://scoop.sh/):
```
scoop install https://raw.githubusercontent.com/archhaze24/touch-for-windows/master/touch.json
```

### Manually:
Download the latest executable from releases page (or build it manually), place it where you want, and add folder containing it to Path.


## Usage: 
```
touch - change file timestamps

Update the access and modification times of each FILE to the current time.
A FILE argument that does not exist is created empty, unless -c is supplied.
Mandatory arguments to long options are mandatory for short options too.

Usage: touch.exe [OPTIONS] [FILE_PATHS]...

Arguments:
[FILE_PATHS]...

Options:
-a                 Change only the access time
-c, --no-create    Do not create any files
-d, --date <DATE>  Parse DATE and use it instead of current time
-m                 Change only the modification time
-h, --help         Print help
-V, --version      Print version
```

## Building:
Prerequisites: [Rust toolchain](https://rustup.rs/).

Building from source is simple: just clone this project, and then run `cargo build --release`. For convenience, you can rename the file to `touch.exe`.

## Contributing:
Contributions are welcome - both issues and pull requests!
