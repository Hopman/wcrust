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

// Main function
fn main() -> std::io::Result<()> {
    // Get arguments
    let opt = Opt::from_args();

    // Run program
    let result = run(&opt)?;

    // Print results
    for r in &result {
        println!("{}", r);
    }
    
    // Clean exit
    Ok(())
}

fn run(opt: &Opt) -> Result<Vec<String>, io::Error> {

    // Create vector for results
    let mut result = Vec::new();

    // Check all paths in FILES
    for path in &opt.files {
        
        // Check if path exists
        if ! path.exists() {
            result.push(format!("wcrust: {:?}: No such file or directory.", &path));

        // If file
        } else if path.is_file() {
            let content = read_file(&path)?;
            let file_count = count_string(content, &opt);
            result.push(format!("{:<40} {:?}", file_count, &path));

        // If directory
        } else if path.is_dir() && ! opt.directories {
            result.push(format!("wcrust: {:?}: is a directory.", &path));
        }
    }
    Ok(result)
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
fn count_string(string: String, opt: &Opt) -> String {
    let mut lin: i64 = 0;
    let mut wrd: i64 = 0;
    let mut chr: i64 = 0;

    for line in string.lines() {
        lin += 1;
        for word in line.split_whitespace() {
            wrd += 1;
            chr += word.len() as i64;
        }
    }
    return format!("{:>10} {:>10} {:>10}", lin, wrd, chr)
}
