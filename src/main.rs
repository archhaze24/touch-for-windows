use clap::Parser;
use std::process;

fn main() {
    let args = touch_for_windows::Args::parse();

    if args.file_paths.is_empty() {
        println!("touch: missing file operand\nTry 'touch --help' for more information");
        process::exit(0);
    }

    touch_for_windows::run(args);
}
