use std::process;

fn main() {
    if let Err(e) = filer::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
