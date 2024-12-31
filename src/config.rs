use clap::{ command, Parser };

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
            println!("{}", filename);
        }
    }
}