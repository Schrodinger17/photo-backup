#![allow(unused)]

mod app;
mod args;

use clap::Parser;

fn main() -> std::io::Result<()> {
    let args = args::Args::parse();

    app::run(args)?;

    Ok(())
}
