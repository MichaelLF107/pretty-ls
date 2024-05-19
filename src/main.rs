use std::env;
use std::fs;

mod utils;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut path = if args.len() > 1 { &args[1] } else { "" };
    let default_path = &String::from("./");

    if path.is_empty() {
        path = default_path;
    }

    let files = fs::read_dir(path).unwrap();

    utils::print_files(files, path);
}
