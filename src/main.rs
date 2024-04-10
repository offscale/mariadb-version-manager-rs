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

const AOT_MARIADB_VERSIONS_LEN: usize = 9;
type AotMariadbVersionsArrType = [&'static str; AOT_MARIADB_VERSIONS_LEN];

// From https://dlm.mariadb.com/rest/releases/mariadb_server/ 2024-04-06
const AOT_MARIADB_VERSIONS_ARR: AotMariadbVersionsArrType = [
    "10.4.33", "10.5.24", "10.6.17", "10.11.7", "11.0.5", "11.1.4", "11.2.3", "11.3.2", "11.4.1",
];

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
        Commands::Ls {} | Commands::Env {} => {}
        _ => {
            // TODO: Update cache here so that latest versions are available
            mariadb_vm::versions_from_remote().await?;
            // .iter().map(|release| format!("{}", release.release_id)).collect::<Vec<String>>();
        }
    }

    match &args.command {
        Commands::Download { version } => {
            let ver: &str = resolve_version(
                match version {
                    Some(v) => v,
                    None => "latest",
                },
                AOT_MARIADB_VERSIONS_ARR,
            );
            match mariadb_vm::download(ver, &args.vm_root, false).await? {
                Some(filepath) => println!("Downloaded: {:?}", filepath),
                None => {}
            }
        }
        Commands::Ls {} => command::default_ls_command(&args)?,
        Commands::LsRemote {} => {
            for ver in AOT_MARIADB_VERSIONS_ARR.iter() {
                println!("{}", ver)
            }
        }
        _ => command::default_command(&args)?,
    }
    config::maybe_config_file_write(&args)?;
    Ok(())
}

fn resolve_version(version: &str, aot_versions: AotMariadbVersionsArrType) -> &str {
    // Future work:
    // Semver range queries
    match version {
        "latest" => aot_versions[AOT_MARIADB_VERSIONS_LEN - 1],
        s => s,
    }
}
