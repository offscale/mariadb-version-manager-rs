extern crate reqwest;

use std::cmp::Ordering;

use rand::Rng;
use serde::{Deserialize, Serialize};
use sha2::Digest;

use crate::errors::MariaDbVmError;

#[derive(Clone, Deserialize, Serialize)]
struct Checksum {
    pub md5sum: Option<String>,
    pub sha1sum: Option<String>,
    pub sha256sum: Option<String>,
    pub sha512sum: Option<String>,
}

/* Start https://mariadb.org/downloads-rest-api/#list-file-checksums */

#[derive(Serialize, Deserialize)]
struct Response {
    pub checksum: Checksum,
}

#[derive(Serialize, Deserialize)]
struct ListOfFileChecksumsRoot {
    pub response: Response,
}

/* End https://mariadb.org/downloads-rest-api/#list-file-checksums */

/* Begin https://mariadb.org/downloads-rest-api/#list-of-point-releases-and-files */
#[derive(Clone, Serialize, Deserialize)]
struct Files {
    pub file_id: i64,
    pub file_name: String,
    pub package_type: Option<String>,
    pub os: Option<String>,
    pub cpu: Option<String>,
    pub checksum: Checksum,
    pub signature: Option<String>,
    pub checksum_url: String,
    pub signature_url: String,
    pub file_download_url: String,
}

#[derive(Clone, Serialize, Deserialize)]
struct Release {
    pub release_id: String,
    pub release_name: String,
    pub date_of_release: String,
    pub release_notes_url: String,
    pub change_log: String,
    pub files: Vec<Files>,
}
#[derive(Serialize, Deserialize)]
struct ListOfPointReleasesAndFilesRoot {
    pub releases: std::collections::BTreeMap<String, Release>,
}

/* End https://mariadb.org/downloads-rest-api/#list-of-point-releases-and-files */

/* Start https://mariadb.org/downloads-rest-api/#list-available-mirrors */

#[derive(Serialize, Deserialize)]
struct Mirror {
    pub mirror_id: String,
    pub mirror_name: String,
}

#[derive(Serialize, Deserialize)]
struct ListAvailableMirrorsRoot {
    pub mirror_list: std::collections::HashMap<String, Vec<Mirror>>,
}

/* End https://mariadb.org/downloads-rest-api/#list-available-mirrors */

const API_BASE: &'static str = "https://downloads.mariadb.org/rest-api/mariadb";

/// This download function makes up-to 3 requests: get exact version, checksum, mirror, archive
pub async fn download(
    version: &str,
    target_dir: &std::ffi::OsString,
    force: bool,
    mirror: &Option<String>,
) -> Result<Option<std::ffi::OsString>, MariaDbVmError> {
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::limited(1))
        .build()?;

    let mut checksum: Option<Checksum> = None;
    let mut file_id: Option<i64> = None;

    let major_minor_patch: std::borrow::Cow<str> =
        if version.chars().filter(|c| *c == '.').count() > 1 {
            std::borrow::Cow::from(version)
        } else {
            // https://mariadb.org/downloads-rest-api/#list-of-point-releases-and-files
            let list_of_point_releases_and_files_url = reqwest::Url::parse(&format!(
                "{API_BASE}/{version}/",
                API_BASE = API_BASE,
                version = version
            ))?;
            println!("GET {}", list_of_point_releases_and_files_url);
            let list_of_point_releases_and_files_root: ListOfPointReleasesAndFilesRoot = client
                .get(list_of_point_releases_and_files_url)
                .send()
                .await?
                .json()
                .await?;
            // Last element is the newest version
            let release = list_of_point_releases_and_files_root
                .releases
                .values()
                .rev()
                .cloned()
                .next()
                .unwrap();

            let os = match std::env::consts::OS {
                "linux" => "Linux",
                "windows" => "Windows",
                // "freebsd" => "FreeBSD", // actually I don't think this ever appears
                _ => "Source",
            };

            if let Some(file) = release
                .files
                .iter()
                .find(|file| file.os == Some(String::from(os)))
            {
                file_id = Some(file.file_id);
                checksum = Some(file.checksum.clone());
            };
            std::borrow::Cow::from(release.release_id)
        };

    let (download_url, filename) = {
        let (base_url, filename) = match std::env::consts::OS {
            "linux" => (
                format!(
                    "{API_BASE}/{major_minor_patch}/",
                    API_BASE = API_BASE,
                    major_minor_patch = major_minor_patch
                ),
                format!(
                    "mariadb-{major_minor_patch}-linux-systemd-{arch}.tar.gz",
                    major_minor_patch = major_minor_patch,
                    arch = std::env::consts::ARCH
                ),
            ),
            "freebsd" => (
                format!(
                    "https://archive.mariadb.org/mariadb-{major_minor_patch}/bintar-freebsd130-{arch}/",
                    major_minor_patch = major_minor_patch,
                    arch = std::env::consts::ARCH
                ),
                format!(
                    "mariadb-{major_minor_patch}-freebsd13.0-{arch}.tar.gz",
                    major_minor_patch = major_minor_patch,
                    arch = std::env::consts::ARCH
                ),
            ),
            "windows" => (
                format!(
                    "{API_BASE}/{major_minor_patch}/",
                    API_BASE = API_BASE,
                    major_minor_patch = major_minor_patch
                ),
                format!(
                    "mariadb-{major_minor_patch}-win{arch}.zip",
                    major_minor_patch = major_minor_patch,
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
            reqwest::Url::parse(&format!("{}{}", base_url, filename))?,
            filename,
        )
    };

    if checksum.is_none() {
        let checksum_url = reqwest::Url::parse(&format!(
            "{API_BASE}/{major_minor_patch}/{filename}/checksum",
            API_BASE = API_BASE,
            major_minor_patch = major_minor_patch,
            filename = filename
        ))?;
        println!("GET {}", checksum_url);
        let checksum_response = client.get(checksum_url).send().await?;
        let checksum_root: ListOfFileChecksumsRoot = checksum_response.json().await?;
        checksum = Some(checksum_root.response.checksum);
    }

    let target_dir = std::path::Path::new(target_dir.as_os_str())
        .join("downloads")
        .join("mariadb");
    let target_file = target_dir.join(filename);

    let checksum_func = |sh256: &String| -> Result<bool, MariaDbVmError> {
        let mut hasher = sha2::Sha256::new();
        let mut file = std::fs::File::open(&target_file)?;
        let bytes_written = std::io::copy(&mut file, &mut hasher)?;
        assert!(bytes_written > 0);
        let hash_bytes = hasher.finalize();
        if sh256.as_bytes().cmp(&hash_bytes[..]) != Ordering::Equal {
            eprintln!("{:x?}\n\t!=\n{:x?}", sh256.as_bytes(), &hash_bytes[..]);
            Err(MariaDbVmError::from(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "SHA256 verification failed",
            )))
        } else {
            Ok(true)
        }
    };
    let check_sum = checksum.unwrap();

    let checksum_sha256 = check_sum.sha256sum.unwrap();
    // could check other checksums here if sha256 isn't defined

    if !force && target_file.is_file() {
        match checksum_func(&checksum_sha256) {
            Ok(passed) => {
                if passed {
                    return Ok(Some(target_file.into_os_string()));
                } else {
                }
            }
            Err(_) => {}
        }
        // Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "Checksum not found"));
    } else if !target_dir.is_dir() {
        std::fs::create_dir_all(target_dir)?;
    }

    let mirror_id: String = if std::env::consts::OS == "freebsd" {
        String::from("")
    } else if let Some(id) = mirror {
        String::from(id)
    } else {
        let mirrors_url = reqwest::Url::parse("https://downloads.mariadb.org/rest-api/mirrors")?;
        println!("GET {}", mirrors_url);
        let mirrors_response = client.get(mirrors_url).send().await?;
        let list_available_mirrors_root: ListAvailableMirrorsRoot = mirrors_response.json().await?;

        // Choose a random mirror. In reality, you may want to choose one closer to your IP? GeoIP?
        let mut rng = rand::thread_rng();
        let rand_i: usize = rng.gen_range(0..list_available_mirrors_root.mirror_list.len());
        let mirrors = list_available_mirrors_root
            .mirror_list
            .values()
            .skip(rand_i)
            .next()
            .unwrap();
        let rand_j: usize = rng.gen_range(0..mirrors.len());
        String::from(mirrors.get(rand_j).unwrap().mirror_id.as_str())
    };

    let response = if std::env::consts::OS == "freebsd" || file_id.is_none() {
        println!("GET {}", download_url);
        client.get(download_url).send().await?
    } else {
        // This alternative URL allows a provided mirror
        let dl_url = reqwest::Url::parse_with_params(
            &format!(
                "{API_BASE}/{major_minor_patch}/{file_id}",
                API_BASE = API_BASE,
                major_minor_patch = major_minor_patch,
                file_id = file_id.unwrap()
            ),
            &[("mirror", mirror_id)],
        )?;
        println!("GET {}", dl_url);
        client.get(dl_url).send().await?
    };
    std::fs::write(&target_file, response.bytes().await?)?;
    checksum_func(&checksum_sha256)?;
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
    let response = reqwest::get(format!("{API_BASE}/", API_BASE = API_BASE)).await?;
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
