#![allow(unused)]

mod app;
mod args;
mod filter;

use clap::Parser;

fn main() -> std::io::Result<()> {
    let args = args::Args::parse();

    app::run(args)?;

    Ok(())
}
