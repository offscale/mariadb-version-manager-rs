// #[macro_use]
extern crate clap_markdown;
extern crate version_manager_rs;

version_manager_rs::cli_struct!(
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_AUTHORS"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_DESCRIPTION")
);

fn main() {
    let args = Cli::parse();
    if args.markdown_help {
        clap_markdown::print_help_markdown::<Cli>();
    }
}
