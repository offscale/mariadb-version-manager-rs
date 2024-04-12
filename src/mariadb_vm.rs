extern crate reqwest;

use crate::errors::MariaDbVmError;

pub async fn download(
    version: &str,
    target_dir: &std::ffi::OsString,
    force: bool,
) -> Result<Option<std::ffi::OsString>, MariaDbVmError> {
    // TODO: mirrors, checksum
    let (url, filename) = {
        let _parts = match std::env::consts::OS {
            "linux" => (
                format!(
                    "https://downloads.mariadb.org/rest-api/mariadb/{version}/",
                    version = version
                ),
                format!(
                    "mariadb-{version}-linux-systemd-{arch}.tar.gz",
                    version = version,
                    arch = std::env::consts::ARCH
                ),
            ),
            "freebsd" => (
                format!(
                    "https://archive.mariadb.org/mariadb-{version}/bintar-freebsd130-{arch}/",
                    version = version,
                    arch = std::env::consts::ARCH
                ),
                format!(
                    "mariadb-{version}-freebsd13.0-{arch}.tar.gz",
                    version = version,
                    arch = std::env::consts::ARCH
                ),
            ),
            "windows" => (
                format!(
                    "http://downloads.mariadb.org/rest-api/mariadb/{version}/",
                    version = version
                ),
                format!(
                    "mariadb-{version}-win{arch}.zip",
                    version = version,
                    arch = match std::env::consts::ARCH {
                        "x86" => "x32",
                        "x86_64" => "x64",
                        a => a,
                    }
                ),
            ),
            _ => unimplemented!("Build from source"),
        };

        (
            reqwest::Url::parse(&format!("{}{}", _parts.0, _parts.1))?,
            _parts.1,
        )
    };
    let target_dir = std::path::Path::new(target_dir.as_os_str())
        .join("downloads")
        .join("mariadb");
    let target_file = target_dir.join(filename);
    if !force && target_file.is_file() {
        return Ok(Some(target_file.into_os_string()));
    } else if !target_dir.is_dir() {
        std::fs::create_dir_all(target_dir)?;
    }
    let response = reqwest::get(url).await?;
    std::fs::write(&target_file, response.bytes().await?)?;
    Ok(Some(std::ffi::OsString::from(target_file)))
}

mod date_format_month_year_day {
    use chrono::NaiveDate;
    use serde::{Deserialize, Deserializer, Serializer};

    const FORMAT: &'static str = "%Y-%m-%d";

    #[allow(dead_code)]
    pub fn serialize<S>(date: &Option<NaiveDate>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match date {
            Some(d) => s.serialize_str(&d.format(FORMAT).to_string()),
            None => s.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::deserialize(deserializer)? {
            Some(s) => Ok(Some(
                NaiveDate::parse_from_str(s, FORMAT).map_err(serde::de::Error::custom)?,
            )),
            None => Ok(None),
        }
    }
}

/// List of major & minor releases
/// https://mariadb.org/downloads-rest-api/#list-of-major-minor-releases
#[derive(serde::Deserialize, Debug)]
#[allow(dead_code)]
struct ListOfMajorAndMinorReleases {
    major_releases: Vec<MajorReleases>,
}

#[derive(Clone, serde::Deserialize, Debug)]
pub struct MajorReleases {
    pub release_id: String,
    pub release_name: String,
    pub release_status: String,
    pub release_support_type: String,

    #[serde(default)]
    #[serde(with = "date_format_month_year_day")]
    pub release_eol_date: Option<chrono::NaiveDate>,
}

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
pub fn offline_major_releases() -> Vec<MajorReleases> {
    let mut releases: Vec<MajorReleases> = Vec::with_capacity(AOT_MARIADB_VERSIONS_LEN);

    releases[0] = MajorReleases {
        release_id: String::from("10.4"),
        release_name: String::from("MariaDB Server 10.4"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Long Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2024-06-18").unwrap()),
    };
    releases[1] = MajorReleases {
        release_id: String::from("10.5"),
        release_name: String::from("MariaDB Server 10.5"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Long Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2025-06-24").unwrap()),
    };
    releases[2] = MajorReleases {
        release_id: String::from("10.6"),
        release_name: String::from("MariaDB Server 10.6"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Long Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2026-07-06").unwrap()),
    };
    releases[3] = MajorReleases {
        release_id: String::from("10.11"),
        release_name: String::from("MariaDB Server 10.11"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Long Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2028-02-16").unwrap()),
    };
    releases[4] = MajorReleases {
        release_id: String::from("11.0"),
        release_name: String::from("MariaDB Server 11.0"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Short Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2024-06-06").unwrap()),
    };
    releases[5] = MajorReleases {
        release_id: String::from("11.1"),
        release_name: String::from("MariaDB Server 11.1"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Short Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2024-08-21").unwrap()),
    };
    releases[6] = MajorReleases {
        release_id: String::from("11.2"),
        release_name: String::from("MariaDB Server 11.2"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Short Term Support"),
        release_eol_date: Some(parse_from_year_month_date("2024-11-21").unwrap()),
    };
    releases[7] = MajorReleases {
        release_id: String::from("11.3"),
        release_name: String::from("MariaDB Server 11.3"),
        release_status: String::from("Stable"),
        release_support_type: String::from("Short Term Support"),
        release_eol_date: None,
    };
    releases[8] = MajorReleases {
        release_id: String::from("11.4"),
        release_name: String::from("MariaDB Server 11.4"),
        release_status: String::from("RC"),
        release_support_type: String::from("Long Term Support"),
        release_eol_date: None,
    };
    releases[9] = MajorReleases {
        release_id: String::from("11.5"),
        release_name: String::from("MariaDB Server 11.5"),
        release_status: String::from("Alpha"),
        release_support_type: String::from("Short Term Support"),
        release_eol_date: None,
    };

    releases
}

pub async fn versions_from_remote() -> Result<Vec<MajorReleases>, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://downloads.mariadb.org/rest-api/mariadb/").await?;
    let mut list_of_major_and_minor_releases: ListOfMajorAndMinorReleases = response.json().await?;
    list_of_major_and_minor_releases
        .major_releases
        .sort_by(|rel0, rel1| rel0.release_id.cmp(&rel1.release_id));
    Ok(list_of_major_and_minor_releases.major_releases)
}

/// Given a version string resolve various version names to their numerical meanings
pub fn resolve_version(version: &str, releases: &Vec<MajorReleases>) -> String {
    // Future work:
    // - Semver range queries
    match version {
        "alpha" => {
            releases
                .iter()
                .rev()
                .find(|rel| rel.release_status == "Alpha")
                .cloned()
                .unwrap()
                .release_id
        }
        "latest" => {
            releases
                .iter()
                .rev()
                .find(|rel| rel.release_status == "Stable")
                .cloned()
                .unwrap()
                .release_id
        }
        "LTS" | "lts" => {
            releases
                .iter()
                .rev()
                .find(|rel| rel.release_support_type == "Long Term Support")
                .cloned()
                .unwrap()
                .release_id
        }
        "RC" | "rc" => {
            releases
                .iter()
                .filter(|rel| rel.release_status == "RC")
                .cloned()
                .next()
                .unwrap()
                .release_id
        }
        s => s.to_string(),
    }
}
