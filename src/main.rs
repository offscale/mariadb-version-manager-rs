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
        return;
    } else if !args.vms_config.is_empty() && std::path::Path::new( &args.vms_config).components().next() == Some(std::path::Component::Normal(std::ffi::OsStr::new("$HOME"))) {
        println!("got a config file {:?}", args.vms_config)
    }
    match &args.command {
        Commands::Ls {} => {
            unimplemented!("TODO")
        },
        _ => panic!("No command given"),
    }
}
