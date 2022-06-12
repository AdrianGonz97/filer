use std::process;

fn main() {
    if let Err(e) = file_rename::run() {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}
