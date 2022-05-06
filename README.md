# temper-cli

A CLI-based tool that performs searches on temp directories or application directories based on an inputted string of text.

_Disclaimer: This is intended for Windows machines ONLY._

## Building

To build an executable from the code, run `cargo build` on the command line. Use `cargo build --release` instead if you wish to create a more optimized binary.

## Running

You can run the code without interacting with the binary with `cargo run`, but you must supply a valid argument.

Examples:
`cargo run --quiet -- --search "discord"`
`cargo run -- --help`

You may also run it directly by using the path (either absolute or relative) of the binary like so:

`C:\Users\USERNAME\YOUR_PATH\temper.exe --search "python"`
`./temper.exe -h`
