mod application;
mod domain;
mod infrastructure;
mod interface;

use clap::Parser;

fn main() {
    let cli = interface::Cli::parse();
    if let Err(error) = interface::run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
