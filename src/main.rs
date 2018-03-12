extern crate getopts;
extern crate rand;

use getopts::Options;

use rand::distributions::{Sample, Range};

use std::env;
use std::fs;
use std::path::PathBuf;

fn list_files(
    path: &std::path::PathBuf,
    mut files_list: &mut Vec<PathBuf>) {
    let metadata = fs::metadata(path).unwrap();
    if metadata.is_file() {
        files_list.push(path.clone());
    } else if metadata.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let p = entry.unwrap().path();
            list_files(&p, &mut files_list);
        }
    }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] SIZE SOURCE_FOLDER DESTINATION_FOLDER", program);
    print!("{}", opts.usage(&brief));
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    //opts.optopt("o", "", "set output file name", "NAME");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") || matches.free.len() != 3 {
        print_usage(&program, &opts);
        return;
    }
    let size_limit: u64 = match matches.free[0].parse() {
        Ok(u) => { u }
        Err(f) => { panic!(f.to_string()) }
    };


    let path = std::path::PathBuf::from(&matches.free[1]);
    let target = std::path::PathBuf::from(&matches.free[2]);
    let mut files_list = Vec::<PathBuf>::new();
    list_files(&path, &mut files_list);

    let mut remain = size_limit;
    let mut full = false;

    let mut generator = rand::thread_rng();
    let mut range = Range::new(0, files_list.len());

    while !full {
        let index = range.sample(&mut generator);
        let path = &files_list[index];

        if let Ok(metadata) = fs::metadata(&path) {
            if metadata.is_file() {
                if metadata.len() > remain {
                    full = true;
                } else if let Some(file_name) = path.file_name() {
                    if fs::copy(path, target.join(std::path::Path::new(file_name))).is_ok() {
                        remain -= metadata.len();
                    }
                }
            }
        }
    }
}
