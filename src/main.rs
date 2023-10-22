extern crate clap_markdown;
extern crate version_manager_rs;

use mariadb_version_manager_rs::mariadb_vm::download;

version_manager_rs::cli_struct_and_helpers!(
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_AUTHORS"),
    env!("CARGO_PKG_VERSION"),
    env!("CARGO_PKG_DESCRIPTION"),
    3306_u16 // default port
);

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = {
        let mut _args = Cli::parse();
        config::maybe_config_from_file(&mut _args)?.unwrap_or_else(|| _args)
    };
    if args.markdown_help {
        clap_markdown::print_help_markdown::<Cli>();
        return Ok(());
    }
    match &args.command {
        Commands::Download { version } => {
            match download(
                version.as_ref().unwrap_or(&args.app_version),
                &args.vm_root,
                false,
            )
            .await?
            {
                Some(filepath) => println!("Downloaded: {:?}", filepath),
                None => {}
            }
        }
        Commands::Ls {} => command::default_ls_command(&args)?,
        _ => {
            let _ = Cli::command().print_help();
        }
    }
    config::maybe_config_file_write(&args)?;
    Ok(())
}
