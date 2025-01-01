use std::{
    fs::File, 
    io::{self, BufRead, BufReader}
};
use clap::{ command, Parser };

use crate::MyResult;

#[derive(Parser, Debug)]
#[command(name = "minicat")]
#[command(author = "gitkeniwo")]
#[command(version = "0.1")]
#[command(about = "A lesser cat utility written in rust. Concatenate and print files", long_about = None)]
pub struct Config {
    /// Number the output lines, starting at 1.
    #[arg(default_value = "-")]
    files: Vec<String>,

    #[arg(short = 'n', long, )]
    number_lines: bool,

    /// Number the non-blank output lines, starting at 1.
    #[arg(short = 'b', long, )]
    number_nonblank_lines: bool,    
}

impl Config {
    pub fn print_filenames(&self) {
        for filename in &self.files {
            match open_file(filename) {
                Err(e) => eprintln!("Failed to open {filename}: {e}"),
                Ok(reader) => {
                    println!("Opened {}:", filename); 
                    for line in reader.lines() {
                        match line {
                            Ok(l) => println!("{l}"),
                            Err(e) => println!("Readline Error: {e}")
                        }
                    }
                 },
            }
        }
    }
}

// Any std::io structs that implememts BufRead https://doc.rust-lang.org/1.82.0/std/io/trait.BufRead.html
fn open_file(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(
            File::open(filename)?
        ))),
    }
}