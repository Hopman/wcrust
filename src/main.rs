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

/*
 * WhatToCount:
 * Simple struct for what to count
 */
#[derive(Debug)]
struct WhatToCount {
    lines: bool,
    words: bool,
    chars: bool,
    bytes: bool,
}

// Default and argument parsing for WhatToCount
impl WhatToCount {
    // Default settings
    fn default() -> WhatToCount {
        WhatToCount {
            lines: true,
            words: true,
            chars: true,
            bytes: false, // Do not count bytes by default
        }
    }

    // Read from arguments
    fn from_args(opt: &Opt) -> WhatToCount {
        WhatToCount {
            lines: opt.lines,
            words: opt.words,
            chars: opt.chars,
            bytes: opt.bytes,
        }
    }
}

/*
 * Count:
 * Simple count struct for all the possible counts
 */
#[derive(Debug)]
struct Count {
    lines: usize,
    words: usize,
    chars: usize,
    bytes: usize,
}

impl Count {
    // New zero-filled count
    fn new() -> Count {
        Count {
            lines: 0,
            words: 0,
            chars: 0,
            bytes: 0,
        }
    }

    // 'add' function for counting totals
    fn add(&mut self, count: &Count, wtc: &WhatToCount) {
        if wtc.lines {
            self.lines += count.lines;
        }
        if wtc.words {
            self.words += count.words;
        }
        if wtc.chars {
            self.chars += count.chars;
        }
        if wtc.bytes {
            self.bytes += count.bytes;
        }
    }

    // Get maximum digit width: used in printing results
    // (Unclean)
    fn max_w(&self) -> usize {
        let max_w = vec![
            self.lines.to_string().len(),
            self.words.to_string().len(),
            self.chars.to_string().len(),
            self.bytes.to_string().len(),
        ].iter().max().unwrap().to_owned(); // (Unclean)

        // Add one to make column 1 line wider than maxmimum width
        return max_w + 1
    }
}

//
// MAIN
//
fn main() {
    // Set exit status to 0
    let mut exit_status = 0;

    // Get arguments
    let opt = Opt::from_args();

    // If no flags for counting have been given, use standard
    let wtc = if opt.lines || opt.words || opt.chars || opt.bytes {
        WhatToCount::from_args(&opt)
    } else {
        WhatToCount::default()
    };

    // Create empty vector and totals
    let mut result_vector = Vec::new();
    let mut totals = Count::new();

    // If no files are given, read from stdin
    if opt.files.is_empty() {
        let count_result = count_stdin(&wtc, &mut totals);
        result_vector.push((count_result, PathBuf::new()))
    } else {
        // Iterate over files
        for path in &opt.files {
            let count_result = count_file(path, &wtc, &mut totals);
            result_vector.push((count_result, path.to_path_buf()));
        }
    }

    // Get maximum width from totals
    let max_w = totals.max_w();

    // Iterate over results and print counts or errors
    for result in &result_vector {
        // result.0 = Result<Count, io::Error>
        // result.1 = path
        match &result.0 {
            Ok(count) =>  {
                // Print in columns
                fancy_print(&count, &wtc, max_w, &result.1);
            },
            Err(error) => {
                // Exit status on error
                exit_status = 1;
                // Print error to stderr
                eprintln!("wcrust: {:?}: {}", result.1, error);
                // Print count (zeros+path) to stdout
                fancy_print(&Count::new(), &wtc, max_w, &result.1);
            },
        }
    }

    // For more than one file print totals
    if result_vector.len() > 1 {
        fancy_print(&totals, &wtc, max_w, &PathBuf::from("total"));
    }

    // Exit with exit status
    ::std::process::exit(exit_status);
}

// Read file to a string, count the string, increment totals
// Todo: read non-UTF-8 files
fn count_file(path: &PathBuf, wtc: &WhatToCount, totals: &mut Count) -> Result<Count, io::Error> {
    // Open and read file
    let mut content = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut content)?;

    // Count string
    let count = count_string(content, wtc);

    // Increment the totals
    totals.add(&count, wtc);

    // Return successful count
    Ok(count)
}

// Read stdin, count the string, increment totals
// Todo: read non-UTF8
fn count_stdin(wtc: &WhatToCount, totals: &mut Count) -> Result<Count, io::Error> {
    // Read stdin
    let mut content = String::new();
    io::stdin().read_to_string(&mut content)?;

    // Count
    let count = count_string(content, wtc);

    // Increment totals
    totals.add(&count, wtc);

    // Return successful count
    Ok(count)
}

// Count the string (only count requested values)
fn count_string(string: String, wtc: &WhatToCount) -> Count {
    // Initiate Count (zeros)
    let mut count = Count::new();

    // Actual couting for requested values
    if wtc.lines {
        count.lines = string.lines().count();
    }
    if wtc.words {
        count.words = string.split_whitespace().count();
    }
    if wtc.chars {
        count.chars = string.len();
    }
    // Todo: bytes not working; exit immediately
    if wtc.bytes {
        eprintln!("Bytes not yet supported.");
        ::std::process::exit(1);
    }

    // Return Count
    return count
}

// Print results in columns
fn fancy_print(count: &Count, wtc: &WhatToCount, max_w: usize, path: &PathBuf) {
    // Create empty string
    let mut print_string = String::new();

    // Check what to count and push to string
    if wtc.lines {
        print_string.push_str(&format!("{:>1$?}", count.lines, max_w));
    }
    if wtc.words {
        print_string.push_str(&format!("{:>1$?}", count.words, max_w));
    }
    if wtc.chars {
        print_string.push_str(&format!("{:>1$?}", count.chars, max_w));
    }
    if wtc.bytes {
        print_string.push_str(&format!("{:>1$?}", count.bytes, max_w));
    }
    print_string.push_str(&format!(" {}", path.to_string_lossy()));

    // PRINT
    println!("{}", print_string);
}
