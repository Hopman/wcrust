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
// Starts the actual run function, where the actual work happens and exits with run's return value
//
fn main() {
    ::std::process::exit(run());
}

// RUN
// Does the actual work, returns exit status
fn run() -> i32 {
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

    if opt.files.is_empty() {
        // No files given, so reading from stdin
        let path = "";
        let mut input = String::new();
        let input_result = io::stdin().read_to_string(&mut input);

        match input_result {
            Ok(_) => {
                let count = count_string(input, &wtc);
                let max_w = count.max_w();
                fancy_print(
                    &count,
                    &wtc,
                    max_w,
                    path,
                );
            }
            Err(error) => {
                eprintln!("wcrust: {}", error);
                let count = Count::new();
                let max_w = 2;
                fancy_print(
                    &count,
                    &wtc,
                    max_w,
                    path,
                );
            }
        }

    } else {
        // Multiple results; create vector
        let mut results = Vec::new();

        // Total Count
        let mut totals = Count::new();

        // Iterate over paths
        for path in &opt.files {
            // Get count from path
            let count_result = count_file(path, &wtc, &mut totals);
            // Push with path into result vector
            results.push((
                    count_result,
                    path.to_path_buf(),
            ));
        }

        // Totals will always have the biggest numbers (yes, biggest)
        let max_w = totals.max_w();

        // Print all the results
        for result in &results {
            match &result.0 {
                Ok(count)  => {
                    fancy_print(
                        &count,
                        &wtc,
                        max_w,
                        result.1.to_str().unwrap()
                    );
                },
                Err(error) => {
                    // Print to error to stderr and a zero-Count to stdout, then continue
                    eprintln!("wcrust: {}: {}", result.1.to_str().unwrap(), error); // Unclean
                    fancy_print(
                        &Count::new(),
                        &wtc,
                        max_w,
                        result.1.to_str().unwrap()
                    );
                    // Ran into an error; exit status should reflect this
                    exit_status = 1;
                },
            }
        }

        // Print totals for more than 1 result
        if results.len() > 1 {
            fancy_print(&totals, &wtc, max_w, "total");
        }

    }

    // Return exit status
    return exit_status
}

// Read file and count contents
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
fn fancy_print(count: &Count, wtc: &WhatToCount, max_w: usize, path: &str) {
    let mut print_string = String::new();
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
    print_string.push_str(&format!(" {}", path));
    println!("{}", print_string);
}
