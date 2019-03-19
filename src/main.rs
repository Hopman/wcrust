// Extern crates
#[macro_use]
extern crate structopt;

// STD
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::PathBuf;
use std::thread;

// EXT
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
struct Count {
    lines: i64,
    words: i64,
    chars: i64,
}

// Main function
fn main() {
    // Get arguments
    let opt = Opt::from_args();

    //  what to count
    let mut wtc =  

    // Run program
    run(&opt);
}

fn run(opt: &Opt) {

    // Create vector for results
    let mut result = 0;
    let mut total = Count {lines: 0, words: 0, chars: 0};
    // Check all paths in FILES
    for path in &opt.files {
        // Check if path exists
        if ! path.exists() {
            eprintln!("wcrust: {:?}: No such file or directory.", &path);
        } else if path.is_dir() && ! opt.directories {
            eprintln!("wcrust: {:?}: Is a directory.", &path);
        // If file
        } else if path.is_file() {
            // Read file to string
            let content = read_file(&path); 
            //                              // TODO: Rework unwrap here
            let count = count_string(content.unwrap(), &opt);
            // Push results into vec
            println!("{:<8?} {:?}", count, &path);
            // Add to totals
            result += 1;
            total.lines += count.lines;
            total.words += count.words;
            total.chars += count.chars;
        }
    }
    if opt.files.len() > 1 {
        println!("{:<8?} total", total);
    }
}
            
fn read_file(path: &PathBuf) -> Result<String, io::Error> {
    
    // Read contents of file to string  
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // Return content
    Ok(content)
}

// Counting
fn count_string(string: String, opt: &Opt) -> Count {
    let mut lin: i64 = 0;
    let mut wrd: i64 = 0;

    for line in string.lines() {
        lin += 1;
        wrd += line.split_whitespace().count() as i64;
    }
    let chr = string.len() as i64;
    return Count { lines: lin, words: wrd, chars: chr };
}
