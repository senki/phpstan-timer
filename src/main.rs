#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::cargo)]

use regex::Regex;
use simple_eyre::eyre;
use std::env;
use std::fs;
use std::io::{self, BufRead};

fn main() -> eyre::Result<()> {
    simple_eyre::install()?;

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eyre::bail!("Missing file path argument.");
    }
    let file_path = &args[1];

    let mut files: Vec<(f64, String)> = Vec::new();
    let mut filename: Option<String> = None;
    let mut line_string: String;

    for (index, line) in read_lines(file_path)?.enumerate() {
        line_string = match line {
            Ok(line) => line,
            Err(error) => eyre::bail!("Could not read line {index} >> {error}"),
        };
        let line_str = trim(line_string.as_str());

        if line_str.is_empty() {
            continue;
        }

        if filename.is_none() {
            filename = Some(line_str.to_string());
            continue;
        }

        let duration = match get_duration(line_str) {
            Ok(duration) => duration,
            Err(error) => eyre::bail!("Could not get `duration` from line {index} >> {error}"),
        };
        if let Some(name) = filename {
            files.push((duration, name));
        }
        filename = None;
    }

    files.sort_by(|a, b| b.0.total_cmp(&a.0));

    let files_length = files.len();
    let slowest_files = &files[0..if files_length < 100 { files_length } else { 100 }];

    println!("Slowest files:");
    for file in slowest_files.iter() {
        println!("{:.2} seconds: {}", file.0, file.1);
    }

    Ok(())
}

/// Read `file_path` one line at a time.
///
/// SEE: <https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html>
fn read_lines(file_path: &String) -> eyre::Result<io::Lines<io::BufReader<fs::File>>> {
    let file = match fs::File::open(file_path) {
        Ok(file) => file,
        Err(error) => eyre::bail!("Could not open `{file_path}` >> {error}"),
    };
    Ok(io::BufReader::new(file).lines())
}

/// Strip whitespace from the beginning and end of a `&str`.
/// Uses the same character list as PHP's `trim()` by default.
///
/// SEE: <https://www.php.net/manual/en/function.trim.php>
fn trim(line_str: &str) -> &str {
    let chars_to_trim: &[char] = &[' ', '\t', '\n', '\r', '\0', '\u{000B}'];
    line_str.trim_matches(chars_to_trim)
}

/// Get the duration as `f64` from a `&str`.
/// Returns `0.0` if no duration is found.
fn get_duration(line_str: &str) -> eyre::Result<f64> {
    let r = Regex::new(r"took ([\d.]+) s")?;

    if let Some(captures) = r.captures(line_str) {
        if let Some(matches) = captures.get(1) {
            return match matches.as_str().parse::<f64>() {
                Ok(duration) => Ok(duration),
                Err(error) => eyre::bail!("`{}` not a number >> {}", matches.as_str(), error),
            };
        }
    }
    Ok(0.0)
}
