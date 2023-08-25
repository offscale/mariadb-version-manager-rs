// #[macro_use]
extern crate version_manager_rs;
use clap::Subcommand;

version_manager_rs::cli_struct!(
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_AUTHORS"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_DESCRIPTION")
);

fn main() {
    let _ = Cli::parse();
}
