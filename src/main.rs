extern crate clap_markdown;
extern crate version_manager_rs;

version_manager_rs::cli_struct_and_helpers!(
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_AUTHORS"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_DESCRIPTION")
);

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = {
        let mut _args = Cli::parse();
        config_from_file(&mut _args)?.unwrap_or_else(|| _args)
    };
    if args.markdown_help {
        clap_markdown::print_help_markdown::<Cli>();
        return Ok(());
    }
    match &args.command {
        Commands::Ls {} => {
            default_ls_command(&args)?;
        }
        _ => panic!("No command given"),
    }
    if should_write_to_config(&args) {
        config_file_write(&args)?;
    }
    Ok(())
}
