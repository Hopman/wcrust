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
 * Simple struct for what to count.
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
            bytes: false, // Do not count bytes as default
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
 * Simple count struct for all the possible counts.
 * When count is None, it will not be processed.
 */
#[derive(Debug)]
struct Count {
    lines: Option<usize>,
    words: Option<usize>,
    chars: Option<usize>,
    bytes: Option<usize>,
}

impl Count {
    // Initiate new Count with Nones
    fn new() -> Count {
        Count {
            lines: None,
            words: None,
            chars: None,
            bytes: None,
        }
    }
    // Initiate new Count with all zeroes
    fn zeros() -> Count {
        Count {
            lines: Some(0),
            words: Some(0),
            chars: Some(0),
            bytes: Some(0),
        }
    }
    // add function for counting totals
    fn add(&mut self, count: &Count, wtc: &WhatToCount) {
        if wtc.lines {
            self.lines = Some(self.lines.unwrap() + count.lines.unwrap());
        }
        if wtc.words {
            self.words = Some(self.words.unwrap() + count.words.unwrap());
        }
        if wtc.chars {
            self.chars = Some(self.chars.unwrap() + count.chars.unwrap());
        }
    }
}

/*
 * CountResult:
 * This will be in the result vector, printing either the count or the
 * error from reading the file.
 * (No other errors will be handled for now).
 */
#[derive(Debug)]
struct CountResult {
    count_result: Result<Count, io::Error>,
    path: PathBuf,
}

// MAIN
// Runs the actual main function (run) and exits on it's return.
fn main() {
    ::std::process::exit(run());
}

// RUN
// Does the actual work, returns exit status.
fn run() -> i32 {
    // Set exit status to 0
    let mut exit_status = 0;

    // Get arguments
    let opt = Opt::from_args();

    // Get what to count from args or default
    let wtc = if opt.lines || opt.words || opt.chars || opt.bytes {
        WhatToCount::from_args(&opt)
    } else {
        WhatToCount::default()
    };


    // If no files are given, read from stdin
    if opt.files.is_empty() {
        println!("TODO: Get input");//TODO
    } else {
        // Results vector
        let mut results = Vec::new();

        for path in &opt.files {
            // Push CountResult with path into result vector
            results.push(CountResult {
                    count_result: count(path, &wtc),
                    path: path.to_path_buf(),
            });
        }

        // TODO: Get max width from results
        let max_w = 8;

        let mut totals = Count::zeros();

        // Print all the results
        for result in &results {
            match &result.count_result {
                Ok(count)  => {
                    fancy_print(&count, &wtc, max_w, result.path.to_str().unwrap());
                    totals.add(&count, &wtc);
                },
                Err(error) => {
                    // Print to error to stderr and zeros to stdout
                    eprintln!("wcrust: {}: {}", result.path.to_str().unwrap(), error);
                    fancy_print(&Count::zeros(), &wtc, max_w, result.path.to_str().unwrap()); exit_status = 1;
                },
            }
        }
        // Print totals
        fancy_print(&totals, &wtc, max_w, "total");
    }

    // Return exit status
    return exit_status
}

fn count(path: &PathBuf, wtc: &WhatToCount) -> Result<Count, io::Error> {
    let mut string = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut string)?;
    Ok(count_string(string, wtc))
}

// Count the string (only count requested values)
fn count_string(string: String, wtc: &WhatToCount) -> Count {
    let mut count = Count::new();
    if wtc.lines {
        count.lines = Some(string.lines().count());
    }
    if wtc.words {
        count.words = Some(string.split_whitespace().count());
    }
    if wtc.chars {
        count.chars = Some(string.len());
    }
    // TODO bytes not working (yet)
    if wtc.bytes {
        println!("Bytes not yet supported.");
        ::std::process::exit(1);
    }
    return count
}

fn fancy_print(c: &Count, wtc: &WhatToCount, max_w: usize, path: &str) {
    let mut print_string = String::new();
    if wtc.lines {
        print_string.push_str(&format!("{:>1$?}", c.lines.unwrap(), max_w));
    }
    if wtc.words {
        print_string.push_str(&format!("{:>1$?}", c.words.unwrap(), max_w));
    }
    if wtc.chars {
        print_string.push_str(&format!("{:>1$?}", c.chars.unwrap(), max_w));
    }
    if wtc.bytes {
        print_string.push_str(&format!("{:>1$?}", c.bytes.unwrap(), max_w));
    }
    print_string.push_str(&format!(" {}", path));
    println!("{}", print_string);
}
