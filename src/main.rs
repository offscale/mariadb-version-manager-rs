extern crate clap_markdown;
extern crate version_manager_rs;
use mariadb_version_manager_rs::mariadb_vm;

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
        if _args.markdown_help {
            clap_markdown::print_help_markdown::<Cli>();
            return Ok(());
        }
        config::maybe_config_from_file(&mut _args)?.unwrap_or_else(|| {
            config::resolve_config_vars(&mut _args);
            _args
        })
    };
    println!("args.root: {:?}\n", args.root);

    let releases: Vec<mariadb_vm::MajorReleases> = match &args.command {
        Commands::Ls {} | Commands::Env {} => Vec::with_capacity(0),
        _ => match std::env::var("SKIP_REMOTE_VERSION_REFRESH") {
            Ok(_) => mariadb_vm::offline_major_releases(),
            Err(_) => mariadb_vm::versions_from_remote().await?,
        },
    };

    match &args.command {
        Commands::Download { version, mirror } => {
            let release_id: String = mariadb_vm::resolve_version(
                match version {
                    Some(v) => v,
                    None => "latest",
                },
                &releases,
            );
            match mariadb_vm::download(&release_id, &args.vm_root, false, mirror).await? {
                Some(filepath) => println!("Downloaded: {:?}", filepath),
                None => {}
            }
        }
        Commands::Ls {} => command::default_ls_command(&args)?,
        Commands::LsRemote {} => {
            for release in releases {
                println!("{}", release.release_id)
            }
        }
        _ => command::default_command(&args)?,
    }
    config::maybe_config_file_write(&args)?;
    Ok(())
}
