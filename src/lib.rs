use chrono::offset::Local;
use clap::{arg, ArgAction, Parser};
use dateparser::parse_with_timezone;
use filetime::{set_file_atime, set_file_mtime, set_file_times, FileTime};
use std::{ffi::OsString, fs::File, path::Path, process, time::SystemTime};

const DESCRIPTION: &str = "touch - change file timestamps\n\nUpdate the access and modification times of each FILE to the current time.\nA FILE argument that does not exist is created empty, unless -c is supplied.\nMandatory arguments to long options are mandatory for short options too.";

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

fn create_file(path: &Path) -> File {
    File::create(path).unwrap_or_else(|err| {
        eprintln!("touch: can't create file: {err}");
        process::exit(1)
    })
}

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
