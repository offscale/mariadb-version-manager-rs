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
