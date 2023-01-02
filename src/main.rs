use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::io::{BufRead, BufReader, Lines};
use std::{fs::File, path::Path};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let mut files: Vec<(f64, String)> = Vec::new();
    let mut filename: Option<String> = None;
    let mut line_string = "".to_string();

    for line in read_lines(file_path) {
        line_string = line.unwrap();
        let mut line_str = line_string.as_str();
        line_str = trim(line_str);

        if line_str == "" {
            continue;
        }

        if filename == None {
            filename = Some(line_str.to_string());
            continue;
        }

        let duration = get_duration(line_str);
        files.push((duration, filename.unwrap()));
        filename = None;
    }

    drop(filename);
    drop(line_string);

    files.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let files_length = files.len();
    let slowest_files = &files[0..if files_length < 100 { files_length } else { 100 }];

    println!("Slowest files");
    for file in slowest_files.iter() {
        println!("{:.2} seconds: {}", file.0, file.1);
    }
}

// SEE: https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html
fn read_lines<P>(filename: P) -> Lines<BufReader<File>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
}

// Imitate PHP's trim.
// SEE: https://www.php.net/manual/en/function.trim.php
fn trim(line_str: &str) -> &str {
    let chars_to_trim: &[char] = &[' ', '\t', '\n', '\r', '\0', '\u{000B}'];
    line_str.trim_matches(chars_to_trim)
}

fn get_duration(line_str: &str) -> f64 {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"took ([\d.]+) s").unwrap();
    }
    let cap = RE.captures(line_str).unwrap();
    cap.get(1).unwrap().as_str().parse::<f64>().unwrap()
}
