use std::error::Error;

use clap::Parser;

mod app;

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    let args = app::Opts::parse();
    args.print_filenames();
    Ok(())
}
