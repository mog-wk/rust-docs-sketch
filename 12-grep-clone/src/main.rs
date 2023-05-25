// std::env::args_os >>> StringOs
use std::env;
use std::process;

use grepclone::Config;

fn main() {
    let args: Vec<String> = env::args().collect(); // env::args() return an iterator .collect() turns iter into vec

    let config: Config = grepclone::Config::new(&args).unwrap_or_else(|err| {
        eprintln!("problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = grepclone::run(config) {
        eprintln!("Application error {}", e);
        process::exit(1);
    }
}

