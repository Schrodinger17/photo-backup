#![allow(unused)]

mod app;
mod args;
mod filter;
mod memory;
mod stats;

use app::App;

use clap::Parser;

fn main() -> std::io::Result<()> {
    let args = args::Args::parse();

    App::new().run(args)?;

    Ok(())
}
