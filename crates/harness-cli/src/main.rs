use clap::Parser;

fn main() {
    let cli = harness_cli::interface::Cli::parse();
    if let Err(error) = harness_cli::interface::run(cli) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}
