use std::{fs::File, path::Path, process, time::SystemTime};

use chrono::offset::Local;
use clap::{arg, ArgAction, Parser};
use dateparser::parse_with_timezone;
use filetime::{set_file_atime, set_file_mtime, set_file_times, FileTime};

const DESCRIPTION: &str = "touch - change file timestamps\n\nUpdate the access and modification times of each FILE to the current time.\nA FILE argument that does not exist is created empty, unless -c is supplied.\nMandatory arguments to long options are mandatory for short options too.";

#[derive(Parser, Debug)]
#[command(version, about = DESCRIPTION)]
pub struct Args {
    #[arg(short, action = ArgAction::SetTrue, help = "Change only the access time")]
    access: bool,

    #[arg(short = 'c', long = "no-create", action = ArgAction::SetTrue, help = "Do not create any files")]
    do_not_create: bool,

    #[arg(short, long, help = "Parse DATE and use it instead of current time")]
    date: Option<String>,

    #[arg(short, action = ArgAction::SetTrue, help = "Change only the modification time")]
    modification: bool,

    pub file_paths: Vec<String>,
}

pub fn run(args: Args) {
    for path in &args.file_paths {
        if !Path::new(&path).exists() {
            if !args.do_not_create {
                create_file(path);
            } else {
                continue;
            }
        }

        if args.date.is_some() {
            let date_string = args.date.as_ref().unwrap();

            let time = parse_with_timezone(date_string, &Local).unwrap_or_else(|_| {
                eprintln!("touch: error parsing date");
                process::exit(1);
            });

            let time = FileTime::from_system_time(SystemTime::from(time));
            set_file_time(&args, path, time, time);
        } else {
            let time_now = FileTime::from_system_time(SystemTime::now());

            set_file_time(&args, path, time_now, time_now);
        }
    }
}

pub fn create_file(path: &String) -> File {
    File::create(path).unwrap_or_else(|err| {
        eprintln!("touch: can't create file {path}: {err}");
        process::exit(1)
    })
}

pub fn set_file_time(args: &Args, path: &String, atime: FileTime, mtime: FileTime) {
    if !args.access && !args.modification {
        set_file_times(path, atime, mtime)
            .unwrap_or_else(|err| eprintln!("touch: unable to set file times: {err}"));
    }

    if args.access {
        set_file_atime(path, atime)
            .unwrap_or_else(|err| eprintln!("touch: unable to set file access time: {err}"));
    }

    if args.modification {
        set_file_mtime(path, mtime)
            .unwrap_or_else(|err| eprintln!("touch: unable to set file modification time: {err}"))
    }
}