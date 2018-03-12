extern crate getopts;
extern crate rand;

use getopts::Options;

use rand::Rng;
use std::env;
use std::fs;
use std::path::PathBuf;

struct Params {
    program_name: String,
    show_help: bool,
    verbose: bool,
    size_limit: u64,
    source_folders: Vec<String>,

    dest_folder: std::path::PathBuf
}

fn parse_arguments() -> Result<(Params, Options), getopts::Fail> {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optflag("h", "help", "print this help menu");
    opts.optflag("v", "verbose", "print more output");
    let matches = opts.parse(&args[1..])?;

    let size_limit = match matches.free[0].parse() {
        Ok(v) => v,
        Err(_) => return Err(getopts::Fail::UnexpectedArgument(format!("Cannot parse size {} into number", matches.free[0])))
    };

    Ok((Params {
        program_name: args[0].clone(),
        show_help: matches.opt_present("h"),
        verbose: matches.opt_present("v"),
        size_limit,

        source_folders: matches.free[1..matches.free.len() - 1].to_vec(),
        dest_folder: std::path::PathBuf::from(matches.free[matches.free.len() - 1].clone())
    }, opts))

}

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
    let params = match parse_arguments() {
        Ok((p, opts)) => {
            if p.show_help {
                print_usage(&p.program_name,&opts);
                return;
            }
            p
        },
        Err(e) => {
            panic!("Error: {}.
Please refer to the help for correct usage.", e);
        }
    };

    let mut files_list = Vec::<PathBuf>::new();
    for p in params.source_folders {
        if params.verbose {
            println!("Listing files of folder {}.", p);
        }
        list_files(&PathBuf::from(p), &mut files_list);
    }
    if params.verbose {
        println!("Files selected as potential candidates: {}", files_list.len());
    }

    let mut remain = params.size_limit;

    let mut generator = rand::thread_rng();

    while !files_list.is_empty() {
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
                let target_file = params.dest_folder.join(std::path::Path::new(file_name));
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
