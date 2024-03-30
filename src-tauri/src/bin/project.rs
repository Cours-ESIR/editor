#[path = "../fonts.rs"]
mod fonts;
#[path = "../packages.rs"]
mod packages;
#[path = "../world.rs"]
mod world;

use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Path to the file to compile (.typ)
    file: PathBuf,

    /// Debug mode [info,debug]
    #[arg(short, long, default_value_t = log::LevelFilter::Debug)]
    debug: log::LevelFilter,
}

fn main() {
    let cli = Cli::parse();
    env_logger::builder().filter_level(cli.debug).init();
    let file = &cli.file;
}