extern crate reqwest;
use crate::errors::MariaDbVmError;

pub async fn download(
    version: &str,
    target_dir: &std::ffi::OsString,
    force: bool,
) -> Result<Option<std::ffi::OsString>, MariaDbVmError> {
    // TODO: mirrors, checksum
    let (url, filename) = {
        let _parts = if cfg!(windows) {
            (
                "https://archive.mariadb.org//mariadb-",
                version,
                "/winx64-packages/",
                "mariadb-",
                version,
                "-winx64.zip",
            )
        } else if cfg!(linux) {
            (
                "https://archive.mariadb.org/mariadb-",
                version,
                "/bintar-linux-systemd-x86_64/",
                "mariadb-",
                version,
                "-preview-linux-systemd-x86_64.tar.gz",
            )
        } else {
            (
                "https://archive.mariadb.org/mariadb-",
                version,
                "/bintar-freebsd130-x86_64/",
                "mariadb-",
                version,
                "-freebsd13.0-x86_64.tar.gz",
            )
        };
        (
            reqwest::Url::parse(&format!(
                "{}{}{}{}{}{}",
                _parts.0, _parts.1, _parts.2, _parts.3, _parts.4, _parts.5
            ))?,
            format!("{}{}{}", _parts.3, _parts.4, _parts.5),
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
    let body = reqwest::get(url).await?.bytes().await?;
    std::fs::write(&target_file, body)?;
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

#[derive(serde::Deserialize, Debug)]
// #[allow(dead_code)]
pub struct MajorReleases {
    pub release_id: String,
    pub release_name: String,
    pub release_status: String,
    pub release_support_type: String,

    #[serde(default)]
    #[serde(with = "date_format_month_year_day")]
    pub release_eol_date: Option<chrono::NaiveDate>,
}

pub async fn versions_from_remote() -> Result<Vec<MajorReleases>, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://downloads.mariadb.org/rest-api/mariadb/").await?;
    let list_of_major_and_minor_releases: ListOfMajorAndMinorReleases = response.json().await?;
    Ok(list_of_major_and_minor_releases.major_releases)
}
