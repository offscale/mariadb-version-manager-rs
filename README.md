mariadb-version-manager-rs
==========================
[![License](https://img.shields.io/badge/license-Apache--2.0%20OR%20MIT-blue.svg)](https://opensource.org/licenses/Apache-2.0)

[MariaDB](https://mariadb.org) version manager. Installs any supported version on any OS.

# Command-Line Help for `mariadb-version-manager-rs`

This document contains the help content for the `mariadb-version-manager-rs` command-line program.

**Command Overview:**

* [`mariadb-version-manager-rs`↴](#mariadb-version-manager-rs)
* [`mariadb-version-manager-rs download`↴](#mariadb-version-manager-rs-download)
* [`mariadb-version-manager-rs env`↴](#mariadb-version-manager-rs-env)
* [`mariadb-version-manager-rs install`↴](#mariadb-version-manager-rs-install)
* [`mariadb-version-manager-rs ls`↴](#mariadb-version-manager-rs-ls)
* [`mariadb-version-manager-rs ls-remote`↴](#mariadb-version-manager-rs-ls-remote)
* [`mariadb-version-manager-rs reload`↴](#mariadb-version-manager-rs-reload)
* [`mariadb-version-manager-rs start`↴](#mariadb-version-manager-rs-start)
* [`mariadb-version-manager-rs stop`↴](#mariadb-version-manager-rs-stop)
* [`mariadb-version-manager-rs uri`↴](#mariadb-version-manager-rs-uri)
* [`mariadb-version-manager-rs install-service`↴](#mariadb-version-manager-rs-install-service)
* [`mariadb-version-manager-rs install-service open-rc`↴](#mariadb-version-manager-rs-install-service-open-rc)
* [`mariadb-version-manager-rs install-service systemd`↴](#mariadb-version-manager-rs-install-service-systemd)
* [`mariadb-version-manager-rs install-service windows-service`↴](#mariadb-version-manager-rs-install-service-windows-service)

## `mariadb-version-manager-rs`



**Usage:** `mariadb-version-manager-rs [OPTIONS] --port <PORT> <COMMAND>`

###### **Subcommands:**

* `download` — Download specified version
* `env` — Print out associated environment variables
* `install` — Install specified version
* `ls` — List what versions are installed
* `ls-remote` — List what versions are available
* `reload` — Reload specified version
* `start` — Start specified version
* `stop` — Stop specified version
* `uri` — Print out database connection string
* `install-service` — Install service (daemon), e.g., systemd, OpenRC, windows-service

###### **Options:**

* `--vms-config <VMS_CONFIG>` — Config file to read from. If provided used as new default (before env and argv res)

  Default value: `$HOME/version-managers/mariadb-version-manager-rs/vms-config.json`
* `--config-read` — Whether to read from config file. If vms_config provided, this defaults to `true`

  Default value: `false`
* `--config-write` — Whether to write to config file

  Default value: `true`
* `--app-version <APP_VERSION>` — Desired version of application

  Default value: `latest`
* `--vm-root <VM_ROOT>` — root directory for all version-managers. For download cache and interdependencies

  Default value: `$HOME/version-managers`
* `--root <ROOT>` — Root directory. By default all paths are relative to this one

  Default value: `$HOME/version-managers/mariadb-version-manager-rs`
* `--hostname <HOSTNAME>` — Hostname of server

  Default value: `localhost`
* `-p`, `--port <PORT>` — Port for server to listen on
* `--database <DATABASE>` — Database name

  Default value: `database`
* `--runtime-path <RUNTIME_PATH>` — Runtime path. This is where PID files and/or similar temporary files are stored

  Default value: `$HOME/version-managers/mariadb-version-manager-rs/mariadb-version-manager-rs/$APP_VERSION/run`
* `--data-path <DATA_PATH>` — Data path. This is where the actual data is stored, e.g., the .db and WAL files

  Default value: `$HOME/version-managers/mariadb-version-manager-rs/mariadb-version-manager-rs/$APP_VERSION/data`
* `--bin-path <BIN_PATH>` — Binary path. Where the executable binary are located. Sometimes called PREFIX

  Default value: `$HOME/version-managers/mariadb-version-manager-rs/mariadb-version-manager-rs/$APP_VERSION/bin`
* `--logs-path <LOGS_PATH>` — Logs path. Where the log files are to be stored

  Default value: `$HOME/version-managers/mariadb-version-manager-rs/mariadb-version-manager-rs/$APP_VERSION/logs`
* `--locale <LOCALE>` — Locale to use

  Default value: `en_US.UTF-8`
* `--markdown-help` — Markdown help generator. Only really used to generate replacement README.md files



## `mariadb-version-manager-rs download`

Download specified version

**Usage:** `mariadb-version-manager-rs download [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `mariadb-version-manager-rs env`

Print out associated environment variables

**Usage:** `mariadb-version-manager-rs env`



## `mariadb-version-manager-rs install`

Install specified version

**Usage:** `mariadb-version-manager-rs install [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `mariadb-version-manager-rs ls`

List what versions are installed

**Usage:** `mariadb-version-manager-rs ls`



## `mariadb-version-manager-rs ls-remote`

List what versions are available

**Usage:** `mariadb-version-manager-rs ls-remote`



## `mariadb-version-manager-rs reload`

Reload specified version

**Usage:** `mariadb-version-manager-rs reload [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `mariadb-version-manager-rs start`

Start specified version

**Usage:** `mariadb-version-manager-rs start [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `mariadb-version-manager-rs stop`

Stop specified version

**Usage:** `mariadb-version-manager-rs stop [VERSION]`

###### **Arguments:**

* `<VERSION>`



## `mariadb-version-manager-rs uri`

Print out database connection string

**Usage:** `mariadb-version-manager-rs uri`



## `mariadb-version-manager-rs install-service`

Install service (daemon), e.g., systemd, OpenRC, windows-service

**Usage:** `mariadb-version-manager-rs install-service
install-service <COMMAND>`

###### **Subcommands:**

* `open-rc` — Install OpenRC service
* `systemd` — Install systemd service
* `windows-service` — Install Windows Service



## `mariadb-version-manager-rs install-service open-rc`

Install OpenRC service

**Usage:** `mariadb-version-manager-rs install-service open-rc [OPTIONS]`

###### **Options:**

* `--group <GROUP>`

  Default value: `mariadb-version-manager-rs`
* `--config-install-path <CONFIG_INSTALL_PATH>`

  Default value: `/etc/conf.d/mariadb-version-manager-rs`
* `--service-install-path <SERVICE_INSTALL_PATH>`

  Default value: `/etc/init.d/mariadb-version-manager-rs`
* `--user <USER>`

  Default value: `mariadb-version-manager-rs`



## `mariadb-version-manager-rs install-service systemd`

Install systemd service

**Usage:** `mariadb-version-manager-rs install-service systemd [OPTIONS]`

###### **Options:**

* `--group <GROUP>`

  Default value: `mariadb-version-manager-rs`
* `--service-install-path <SERVICE_INSTALL_PATH>`

  Default value: `/etc/systemd/system/mariadb-version-manager-rs.service`
* `--user <USER>`

  Default value: `mariadb-version-manager-rs`



## `mariadb-version-manager-rs install-service windows-service`

Install Windows Service

**Usage:** `mariadb-version-manager-rs install-service windows-service [OPTIONS]`

###### **Options:**

* `--service-name <SERVICE_NAME>`

  Default value: `mariadb-version-manager-rs`
* `--service-description <SERVICE_DESCRIPTION>`

  Default value: ``


<hr/>

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <https://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <https://opensource.org/licenses/MIT>)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
