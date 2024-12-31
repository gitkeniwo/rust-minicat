fn main() {
    if let Err(e) = rust_minicat::run() {
        eprintln!("{}", e); 
        std::process::exit(1); 
    }
}
