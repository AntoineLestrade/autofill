use std::fs;
use std::path::PathBuf;

use humanize_rs::bytes::{Bytes};
use rand::Rng;
use structopt::StructOpt;


fn parse_size(src: &str) -> Result<Bytes<u64>, humanize_rs::ParseError> {
    return src.parse::<Bytes<u64>>();
}

#[derive(Debug, StructOpt)]
struct Params {
    #[structopt(short = "v", long="verbose")]
    verbose: bool,
    #[structopt(parse(try_from_str = "parse_size"))]
    size_limit: Bytes<u64>,
    #[structopt(parse(from_os_str))]
    dest_folder: std::path::PathBuf,
    #[structopt()]
    source_folders: Vec<String>,
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

pub fn main() {
    let params = Params::from_args();

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

    let mut remain: u64 = params.size_limit.size();

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
