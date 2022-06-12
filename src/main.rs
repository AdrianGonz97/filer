use std::env;
use std::process;

fn main() {
    file_rename::run();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // if let Err(e) = file_rename::run(config) {
    //     eprintln!("Application error: {}", e);
    //     process::exit(1);
    // }
}
