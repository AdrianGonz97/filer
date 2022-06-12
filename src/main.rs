use std::env;
use std::process;

use file_rename::Config;

fn main() {
    // specify the type as it tells collect what type of collection we want
    let args: Vec<String> = env::args().collect();

    file_rename::test();

    // let config = Config::new(&args).unwrap_or_else(|err| {
    //     eprintln!("Problem parsing arguments: {}", err);
    //     process::exit(1);
    // });

    // if let Err(e) = file_rename::run(config) {
    //     eprintln!("Application error: {}", e);
    //     process::exit(1);
    // }
}
