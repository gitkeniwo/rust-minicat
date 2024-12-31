use std::error::Error;

use clap::Parser;

mod config;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let args = config::Config::parse();
    dbg!(&args);
    args.print_filenames();
    Ok(())
}
