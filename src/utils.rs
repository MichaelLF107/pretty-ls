use colored::Colorize;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path;

fn print_line(width: usize) {
    println!("{:─^width$}", "", width = width);
}

fn print_file(file: DirEntry, count: i32, pad: usize, longest_str: usize) {
    let path = file.path();
    let file_name = path.file_name().unwrap();
    let file_name = file_name.to_str().unwrap();
    let file_type = file.file_type().unwrap();
    let metadata = file.metadata().unwrap();
    let file_size_kb = metadata.len();
    let file_size_mb = file_size_kb as f64 / 1024.0;
    let file_size_gb = file_size_kb as f64 / 1024.0 / 1024.0;
    match file_type.is_dir() {
        true => {
            println!(
                "|{:0>width$}| {} {:<file_width$}|",
                count,
                "\u{f115}".blue(),
                format!(
                    "{}{}",
                    file_name.blue(),
                    match file_size_kb {
                        0..=1023 => format!(" ({:.2} KB)", file_size_kb),
                        1024..=1048576 => format!(" ({:.2} MB)", file_size_mb),
                        _ => format!(" ({:.2} GB)", file_size_gb),
                    }
                ),
                width = pad,
                file_width = longest_str + 2
            );
        }
        false => {
            println!(
                "|{:0>width$}| {} {:<file_width$}|",
                count,
                "\u{f15b}".purple(),
                format!(
                    "{}{}",
                    file_name.purple(),
                    match file_size_kb {
                        0..=1023 => format!(" ({:.2} KB)", file_size_kb),
                        1024..=1048576 => format!(" ({:.2} MB)", file_size_mb),
                        _ => format!(" ({:.2} GB)", file_size_gb),
                    }
                ),
                width = pad,
                file_width = longest_str + 2
            );
        }
    }
}

pub fn print_files(files: fs::ReadDir, path_arg: &str) {
    println!("Path: {}", path_arg);
    let mut count = 1;
    let mut longest_str = 0;

    let files = files.collect::<Vec<_>>();
    let file_count = files.len();

    let pad = file_count.to_string().len();

    let current_dir = env::current_dir();
    let mut path_name: String;

    if (path_arg == "./") || (path_arg == ".") {
        match current_dir {
            Ok(dir) => {
                path_name = dir.to_str().unwrap().to_string();
                if path_name.len() > 30 {
                    let mut path = path::Path::new(&path_name);
                    let mut path_str = String::new();
                    while path != path.parent().unwrap() {
                        let path_name = path.file_name().unwrap().to_str().unwrap();
                        path_str = format!("{}/{}", path_name, path_str);
                        path = path.parent().unwrap();
                    }
                    path_name = format!("...{}", path_str);
                }
            }
            Err(_) => {
                path_name = "Unknown".to_string();
            }
        }
    } else {
        path_name = path_arg.to_string();
    }

    for file in &files {
        let path = file.as_ref().unwrap().path();
        let file_name = path.file_name().unwrap();
        let file_name = file_name.to_str().unwrap();
        let metadata = file.as_ref().unwrap().metadata().unwrap();
        let file_size_kb = metadata.len();
        let file_size_mb = file_size_kb as f64 / 1024.0;
        let file_size_gb = file_size_kb as f64 / 1024.0 / 1024.0;
        let length = file_name.len() + match file_size_kb {
            0..=1023 => file_size_kb.to_string().len() + 9,
            1024..=1048576 => file_size_mb.to_string().len() + 9,
            _ => file_size_gb.to_string().len() + 9,
        };
        if length > longest_str {
            longest_str = length;
        }
    }

    if path_name.len() > longest_str {
        longest_str = path_name.len();
    }

    print_line(longest_str + 1);
    println!(
        "|{:^path_width$}|",
        path_name.bold().bright_cyan(),
        path_width = longest_str - 1
    );
    print_line(longest_str + 1);

    for file in files {
        match file {
            Ok(file) => {
                print_file(file, count, pad, longest_str);
                count += 1;
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    print_line(longest_str + 1);
}
