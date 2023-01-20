use chrono::offset::Local;
use clap::{arg, ArgAction, Parser};
use dateparser::parse_with_timezone;
use filetime::{set_file_atime, set_file_mtime, set_file_times, FileTime};
use std::{ffi::OsString, fs::File, path::Path, process, time::SystemTime};

const DESCRIPTION: &str = "touch - change file timestamps\n\nUpdate the access and modification times of each FILE to the current time.\nA FILE argument that does not exist is created empty, unless -c is supplied.\nMandatory arguments to long options are mandatory for short options too.";

/// Arguments received from command line.
///
/// To create new, use `Args::parse()` with `parse` imported from `clap::Parser`
#[derive(Parser, Debug)]
#[command(version, about = DESCRIPTION)]
pub struct Args {
    #[arg(short, action = ArgAction::SetTrue, help = "Change only the access time")]
    access: bool,

    #[arg(short = 'c', long = "no-create", action = ArgAction::SetTrue, help = "Do not create any files")]
    do_not_create: bool,

    #[arg(short, long, help = "Parse DATE and use it instead of current time")]
    date: Option<OsString>,

    #[arg(short, action = ArgAction::SetTrue, help = "Change only the modification time")]
    modification: bool,

    pub file_paths: Vec<OsString>,
}

/// Start the program with args received from command line.
///
/// Before running this function, check if `args.file_paths` is not empty.
///
/// # Error handling
///
/// If creating file or parsing given date failed, if exits with printing
/// error message to standard error output.
///
/// If setting date to file failed, it prints error to standard error
/// output and continues to next file.
pub fn run(args: Args) {
    for path in &args.file_paths {
        let path = Path::new(path);
        if !path.exists() {
            if !args.do_not_create {
                create_file(path);
            } else {
                continue;
            }
        }
    }

    if args.date.is_some() {
        let date_str = args.date.as_ref().unwrap().to_str().unwrap_or_else(|| {
            eprintln!("touch: error parsing date");
            process::exit(1);
        });

        let datetime = parse_with_timezone(date_str, &Local).unwrap_or_else(|_| {
            eprintln!("touch: error parsing date");
            process::exit(1);
        });
        let time = FileTime::from_system_time(SystemTime::from(datetime));

        set_files_time(&args, time);
    } else {
        let time_now = FileTime::from_system_time(SystemTime::now());

        set_files_time(&args, time_now);
    }
}

/// Creates a file from a received file path.
///
/// Returns the file it created.
///
/// # Error handling
///
/// If `create_file` is unable to create file, it exits with error message.
fn create_file(path: &Path) -> File {
    File::create(path).unwrap_or_else(|err| {
        eprintln!("touch: can't create file: {err}");
        process::exit(1)
    })
}

/// Sets files modification and access time from received args and time.
///
/// This functions looks up given args for `args.access` and `args.modification`.
/// If none of them is received (which means they are `false`), it sets both modification and access time.
///
/// Else if any of those args is received, it sets them.
///
/// # Error handling
///
/// If file path does not exists, or setting modification/access failed, it prints error
/// to standard error output, and continues to next given file path.
fn set_files_time(args: &Args, time: FileTime) {
    if !args.access && !args.modification {
        for path in &args.file_paths {
            let path = Path::new(path);
            if path.exists() {
                set_file_times(path, time, time)
                    .unwrap_or_else(|err| eprintln!("touch: unable to set file times: {err}"));
            }
        }
    }

    if args.access {
        for path in &args.file_paths {
            let path = Path::new(path);
            if path.exists() {
                set_file_atime(path, time).unwrap_or_else(|err| {
                    eprintln!("touch: unable to set file access time: {err}")
                });
            }
        }
    }

    if args.modification {
        for path in &args.file_paths {
            let path = Path::new(path);
            if path.exists() {
                set_file_mtime(path, time).unwrap_or_else(|err| {
                    eprintln!("touch: unable to set file modification time: {err}")
                });
            }
        }
    }
}
