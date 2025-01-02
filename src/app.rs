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
pub struct Opts {
    /// Number the output lines, starting at 1.
    #[arg(default_value = "-")]
    pub files: Vec<String>,

    #[arg(short = 'n', long,)]
    pub number_lines: bool,

    /// Number the non-blank output lines, starting at 1.
    #[arg(short = 'b', long,)]
    pub number_nonblank_lines: bool,    
}

impl Opts {
    pub fn print_filenames(&self) {
        for filename in &self.files {
            match open_file(filename) {
                Err(e) => eprintln!("Failed to open {filename}: {e}"),
                Ok(reader) => {
                    // println!("Opened {}:", filename); 
                    let mut last_num = 0;
                    for (line_number, line) in reader.lines().enumerate() {
                        match line {
                            Ok(l) => {
                                if self.number_lines {
                                    println!("{:>6}\t{}", line_number+1, l)
                                    //":" indicates the start of formatting options
                                    //">" specifies right-alignment
                                    //"6" sets the minimum width to 6 characters
                                } else if self.number_nonblank_lines {
                                    if l.is_empty() {
                                        println!("{}", l)
                                    } else {
                                        last_num += 1;     
                                        println!("{:>6}\t{}", last_num, l)
                                    }
                                } else {
                                    println!("{}", l)
                                }
                            }, 
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