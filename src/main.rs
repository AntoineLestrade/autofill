extern crate getopts;
extern crate rand;

use getopts::Options;

use rand::Rng;
use std::env;
use std::fs;
use std::path::PathBuf;

fn list_files(
    path: &std::path::PathBuf,
    mut files_list: &mut Vec<PathBuf>) {
    let metadata = fs::metadata(path).unwrap();
    if metadata.is_file() {
        files_list.push(
            path.clone()
        );
    } else if metadata.is_dir() {
        for entry in fs::read_dir(path).unwrap() {
            let p = entry.unwrap().path();
            list_files(&p, &mut files_list);
        }
    }
}

fn print_usage(program: &str, opts: &Options) {
    let brief = format!("Usage: {} [options] SIZE SOURCE_FOLDER [SOURCE_FOLDER_2 [SOURCE_FOLDER_3 ...]] DESTINATION_FOLDER", program);
    print!("{}", opts.usage(&brief));
}
pub fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "verbose", "print more output");
    //opts.optmulti("f", "filter", "Regular expression that filters inculded files", "REGEX");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    if matches.opt_present("h") || matches.free.len() < 3 {
        print_usage(&program, &opts);
        return;
    }
    let size_limit: u64 = match matches.free[0].parse() {
        Ok(u) => { u }
        Err(f) => { panic!(f.to_string()) }
    };


    let verbose: bool = matches.opt_present("v");
    let target = std::path::PathBuf::from(&matches.free[&matches.free.len()-1]);
    let mut files_list = Vec::<PathBuf>::new();
    for index in 1..&matches.free.len() - 1 {
        if verbose {
            println!("Listing files of folder {}.", &matches.free[index]);
        }
        list_files(&std::path::PathBuf::from(&matches.free[index]), &mut files_list);
    }
    if verbose {
        println!("Files selected as potential candidates: {}", files_list.len());
    }

    let mut remain = size_limit;

    let mut generator = rand::thread_rng();

    while files_list.len() > 0 {
        let index = generator.gen_range(0, files_list.len() - 1);
        let path: PathBuf = files_list.remove(index);

        if let Ok(metadata) = fs::metadata(&path) {
            if !metadata.is_file() {
                continue;
            }
            else if metadata.len() > remain {
                println!("Target full. Selected file: {:?} (size: {})", path, metadata.len());
                return;
            }
            else if let Some(file_name) = path.file_name() {
                let target_file = target.join(std::path::Path::new(file_name));
                if fs::metadata(&target_file).is_err() {
                    // if not error, that means the target file already exists;

                    match fs::copy(&path, target_file) {
                        Ok(len) => remain -= len,
                        Err(err) => println!("ERROR Copying file: {}", err)
                    };
                }
            }
        }
    }
    println!("WARN: All files copied, the target size was not reached.");
}
