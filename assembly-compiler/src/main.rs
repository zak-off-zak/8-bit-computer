use std::process;

use assembly_compiler::Config;
use clap::Parser;

fn main() {
    let config = Config::parse();

    if let Err(e) = assembly_compiler::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
