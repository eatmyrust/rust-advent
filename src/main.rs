use std::env;
use std::process;

use rust_advent;
use rust_advent::CLIParams;

fn main() {
    let cli_params = CLIParams::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    rust_advent::run_advent_day(&cli_params);
}
