#[macro_use]
extern crate lazy_static;

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

/*
# To support old versions for testing purposes which are not on dlm.mariadb.com
case ${mariadb_server_version} in
    *10.1[0-9]*) ;; # everything >= 10.10 is on the new dlm for sure
    *10.0*) ;& # 10.0 releases are on the old download server only
    *10.1*) ;& # same for 10.1
    *10.2*) ;& # same for 10.2
    *10.3.[0-2]*|*10.3.[0-9]) ;& # 10.3: old server has <= .32, dlm has >= .29, we switch at .30
    *10.4.[0-1]*|*10.4.[0-9]) ;& # 10.4: dlm has >= 10.4.20
    *10.5.[0-9])     # 10.5: dlm has >= 10.5.10
    url_base="downloads.mariadb.com"
    url_mariadb_repo="https://${url_base}/MariaDB"
    mariadb_server_version_real=$mariadb_server_version
    ;;
esac
*/

// From https://dlm.mariadb.com/rest/releases/mariadb_server/ 2024-04-06
lazy_static! {
    static ref AOT_MARIADB_VERSIONS_VEC: Vec<&'static str> = vec![
        "10.4.33", "10.5.24", "10.6.17", "10.11.7", "11.0.5",
        "11.1.4", "11.2.3", "11.3.2", "11.4.1"];
    static ref AOT_MARIADB_VERSIONS: std::collections::HashSet<&'static str> = std::collections::HashSet::from(
        AOT_MARIADB_VERSIONS_VEC.copy()
    );
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = {
        let mut _args = Cli::parse();
        config::maybe_config_from_file(&mut _args)?.unwrap_or_else(|| {
            config::resolve_config_vars(&mut _args);
            _args
        })
    };
    if args.markdown_help {
        clap_markdown::print_help_markdown::<Cli>();
        return Ok(());
    }
    println!("args.root: {:?}\n", args.root);
    match &args.command {
        Commands::Download { version } => {
            // let ver: String = resolve_version(version, AOT_MARIADB_VERSIONS);
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
        Commands::LsRemote {} => for ver in &AOT_MARIADB_VERSIONS_VEC {
            println!("{:?}", ver)
        },
        _ => command::default_command(&args)?,
    }
    config::maybe_config_file_write(&args)?;
    Ok(())
}

/// Resolve `latest`, `lts`, `stable` and other such keywords using
fn resolve_version(version: &Option<String>, aot_versions: AOT_MARIADB_VERSIONS) -> String {
    // Future work:
    // Semver range queries
    todo!()
}
