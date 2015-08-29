extern crate getopts;
extern crate rand;

use getopts::Options;

use rand::distributions::{Sample, Range};

use std::env;
use std::fs;
use std::path::PathBuf;

fn list_files (path: &std::path::PathBuf) -> Result<Vec<PathBuf>, std::io::Error> {

    let mut result : Vec<PathBuf> = Vec::<PathBuf>::new();

    let metadata = try!(fs::metadata(path));
    if metadata.is_file() {
        result.push(path.clone());
    }
    else if metadata.is_dir() {
        for entry in try!(fs::read_dir(path)) {
            let entry = try!(entry);
            let p = entry.path();
            let list = try!(list_files(&p));
            for f in list {
                result.push(f);
            }
        }
    }

    Ok(result)
}

fn print_usage(program: &str, opts: Options) {
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
        print_usage(&program, opts);
        return;
    }
    let size_limit : u64 = match matches.free[0].parse() {
        Ok(u) => { u }
        Err(f) => { panic!(f.to_string()) }
    };
    
    
    let path = std::path::PathBuf::from(&matches.free[1]);
    let target = std::path::PathBuf::from(&matches.free[2]);
    let files = list_files(&path);
    if let Ok(list) = files {
//        for f in list {
//            println!("{}", match f.to_str() {
//                    Some(p) => { p }
//                    None => { "Empty path?" }
//                });
//        }
        
        let mut remain = size_limit;
        let mut full = false;
        
        let mut generator = rand::thread_rng();
        let mut range = Range::new(0, list.len());
        
        while !full {
            let index = range.sample(&mut generator);
            let path = &list[index];
            
            if let Ok(metadata) = fs::metadata(&path) {
                if metadata.is_file() {
                    if metadata.len() > remain {
                        full = true;
                    }
                    else if let Some(file_name) = path.file_name() {
                        if let Ok(_) = fs::copy(path, target.join(std::path::Path::new(file_name))) {
                            remain -= metadata.len();
                        }
                    }
                }
            }
        }
    }
}
