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

const AOT_MARIADB_VERSIONS_LEN: usize = 10;

fn parse_from_year_month_date(date_str: &str) -> chrono::ParseResult<chrono::NaiveDate> {
    chrono::NaiveDate::parse_from_str(date_str, "%Y-%m-%d")
}

// From https://dlm.mariadb.com/rest/releases/mariadb_server/ 2024-04-06
fn offline_major_releases() -> Vec<mariadb_vm::MajorReleases> {
    let mut releases = std::vec::Vec::with_capacity(AOT_MARIADB_VERSIONS_LEN);
    releases[0] = mariadb_vm::MajorReleases {
        release_id: String::from("11.5"),
        release_name: String::from("MariaDB Server 11.5"),
        release_status: String::from("Alpha"),
        release_support_type: String::from("Short Term Support"),
        release_eol_date: None,
    };
    releases[1] = mariadb_vm::MajorReleases {
        release_id: String::from("11.4"),
        release_name: String::from("MariaDB Server 11.4"),
        release_status: String::from("RC"),
        release_support_type: String::from("Long Term Support"),
        release_eol_date: None,
    };
    releases[2] = mariadb_vm::MajorReleases {
        release_id: String::from("11.3"),
        release_name: String::from("MariaDB Server 11.3"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Short Term Support"),
        release_eol_date: None,
    };
    releases[3] = mariadb_vm::MajorReleases {
        release_id: String::from("11.2"),
        release_name: String::from("MariaDB Server 11.2"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Short Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2024-11-21").unwrap()),
    };
    releases[4] = mariadb_vm::MajorReleases {
        release_id: String::from("11.1"),
        release_name: String::from("MariaDB Server 11.1"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Short Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2024-08-21").unwrap()),
    };
    releases[5] = mariadb_vm::MajorReleases {
        release_id: String::from("11.0"),
        release_name: String::from("MariaDB Server 11.0"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Short Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2024-06-06").unwrap()),
    };
    releases[6] = mariadb_vm::MajorReleases {
        release_id: String::from("10.11"),
        release_name: String::from("MariaDB Server 10.11"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Long Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2028-02-16").unwrap()),
    };
    releases[7] = mariadb_vm::MajorReleases {
        release_id: String::from("10.6"),
        release_name: String::from("MariaDB Server 10.6"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Long Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2026-07-06").unwrap()),
    };
    releases[8] = mariadb_vm::MajorReleases {
        release_id: String::from("10.5"),
        release_name: String::from("MariaDB Server 10.5"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Long Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2025-06-24").unwrap()),
    };
    releases[9] = mariadb_vm::MajorReleases {
        release_id: String::from("10.4"),
        release_name: String::from("MariaDB Server 10.4"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Long Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2024-06-18").unwrap()),
    };
    releases
}

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

    let releases = match &args.command {
        Commands::Ls {}// | Commands::Env {}
        => Vec::with_capacity(0),
        _ => {
            match std::env::var("SKIP_REMOTE_VERSION_REFRESH") {
                Ok(_) => offline_major_releases(),
                Err(_) => {
                    mariadb_vm::versions_from_remote().await?
                }
            }
        }
    };

    match &args.command {
        Commands::Download { version } => {
            let ver: String = resolve_version(
                match version {
                    Some(v) => v,
                    None => "latest",
                },
                releases,
            );
            match mariadb_vm::download(&ver, &args.vm_root, false).await? {
                Some(filepath) => println!("Downloaded: {:?}", filepath),
                None => {}
            }
        }
        Commands::Ls {} => command::default_ls_command(&args)?,
        Commands::LsRemote {} => {
            let mut release_ids = releases
                .iter()
                .map(|rel| rel.release_id.clone())
                .collect::<Vec<String>>();
            release_ids.sort();
            for release_id in release_ids.iter() {
                println!("{}", release_id)
            }
        }
        _ => command::default_command(&args)?,
    }
    config::maybe_config_file_write(&args)?;
    Ok(())
}

fn resolve_version(version: &str, aot_versions: Vec<mariadb_vm::MajorReleases>) -> String {
    // Future work:
    // Semver range queries
    match version {
        "latest" => aot_versions[aot_versions.len() - 1].release_id.clone(),
        s => s.to_string(),
    }
}
