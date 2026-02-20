use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;
use std::{env, fs, io};

#[path = "src/args.rs"]
mod commands;
use commands::Args;

fn main() -> io::Result<()> {
    fs::create_dir_all("target/dist/completions")?;

    for &shell in Shell::value_variants() {
        clap_complete::generate_to(
            shell,
            &mut Args::command(),
            env!("CARGO_PKG_NAME"),
            "target/dist/completions",
        )?;
    }

    Ok(())
}
