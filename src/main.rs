use rust_minicat::run;

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e); 
        std::process::exit(1); 
    }
}
