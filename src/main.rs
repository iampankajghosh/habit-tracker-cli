use clap::Parser;
use habit::cli::commands::{Cli, run};

fn main() {
    let cli = Cli::parse();
    if let Err(err) = run(cli) {
        eprintln!("❌ Error: {}", err);
        std::process::exit(1);
    }
}
