use std::env;
use std::error::Error;
use std::process;

use rust_advent::{run_advent_day, CLIParams};

fn main() -> Result<(), Box<dyn Error>> {
    let cli_params = CLIParams::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    run_advent_day(&cli_params)?;

    Ok(())
}
