//#[macro_use]
extern crate structopt;

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;

use structopt::StructOpt;

// Opt: Arguments struct
#[derive(Debug, StructOpt)]
#[structopt(name="wcrust", about="Prints newline, word and character counts for each file")]
struct Opt {
    #[structopt(short="c", long="bytes")]
    /// Prints the byte counts
    bytes: bool,

    #[structopt(short="m", long="chars")]
    /// Prints the character counts
    chars: bool,

    #[structopt(short="l", long="lines")]
    /// Prints the newline counts
    lines: bool,

    #[structopt(short="w", long="words")]
    /// Prints the word counts
    words: bool,

    #[structopt(short="D", long="skip-directories")]
    /// Skip directories
    directories: bool,

    #[structopt(name="FILE", parse(from_os_str))]
    ///Files to process
    files: Vec<PathBuf>,
}

#[derive(Debug)]
struct WhatToCount {
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool,
}

// Main function
fn main() -> Result<(), io::Error> {
    // Get arguments
    let opt = Opt::from_args();
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_chars = 0;
    let mut total_bytes = 0;

    let mut wtc = WhatToCount {lines: false, words: false, chars: false, bytes: false};

    if ! opt.bytes && ! opt.chars && ! opt.lines && ! opt.words {
        wtc.lines = true;
        wtc.words = true;
        wtc.chars = true;
    } else {
        wtc.lines = opt.lines;
        wtc.words = opt.words;
        wtc.chars = opt.chars;
        wtc.bytes = opt.bytes;
    }

    // Check all paths in FILES
    for path in &opt.files {
        // Check if path exists
        if ! path.exists() {
            eprintln!("wcrust: {:?}: No such file or directory.", &path);
        } else if path.is_dir() && ! opt.directories {
            eprintln!("wcrust: {:?}: Is a directory.", &path);
        // If file
        } else if path.is_file() {
            // Read contents of file to string
            let mut file = File::open(path)?;
            let mut content = String::new();
            let bts = match file.read_to_string(&mut content) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("wcrust: {:?}: Could not read to string:\n\t{}", &path, e);
                    0
                }
            };

            let mut lin = 0;
            let mut wrd = 0;
            let mut chr = 0;

            let mut print_string = String::new();

            // Count
            if wtc.lines {
                lin = content.lines().count();
                print_string.push_str(&format!("{:<8?}", lin));
            }
            if wtc.words {
                wrd = content.split_whitespace().count();
                print_string.push_str(&format!("{:<8?}", wrd));
            }
            if wtc.chars {
                chr = content.len();
                print_string.push_str(&format!("{:<8?}", chr));
            }
            if wtc.bytes {
                print_string.push_str(&format!("{:<8?}", bts));
            }

            println!("{} {:>8}", print_string, path.to_str().unwrap());

            // Add to totals
            total_lines += lin;
            total_words += wrd;
            total_chars += chr;
            total_bytes += bts;
        }
    }
    if opt.files.len() > 1 {
            let mut total_string = String::new();
            // Format
            if wtc.lines {
                total_string.push_str(&format!("{:>8?}", total_lines));
            }
            if wtc.words {
                total_string.push_str(&format!("{:>8?}", total_words));
            }
            if wtc.chars {
                total_string.push_str(&format!("{:>8?}", total_chars));
            }
            if wtc.bytes {
                total_string.push_str(&format!("{:>8?}", total_bytes));
            }
            total_string.push_str(" total");
            println!("{}", total_string);
    }
    Ok(())
}
