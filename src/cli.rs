// cli: Handle command line parsing
#![forbid(unsafe_code)]
#![forbid(missing_docs)]
use super::products::PRODUCTS_LIST;
use clap::{
    crate_description,
    crate_name,
    crate_version,
    App,
    AppSettings,
    Arg,
    ArgMatches,
};
use std::env;
use std::ffi::{
    OsStr,
    OsString,
};
use std::path::Path;

#[cfg(feature = "shell_completion")]
use clap::Shell;

#[cfg(feature = "shell_completion")]
use std::io;

#[cfg(feature = "shell_completion")]
use std::str::FromStr;

#[cfg(target_arch = "arm")]
pub const DEFAULT_ARCH: &str = "arm";

#[cfg(target_arch = "x86")]
pub const DEFAULT_ARCH: &str = "386";

#[cfg(target_arch = "x86_64")]
pub const DEFAULT_ARCH: &str = "amd64";

const VALID_ARCH: &[&str] = &[
    "386",
    "amd64",
    "arm",
];

#[cfg(target_os = "freebsd")]
pub const DEFAULT_OS: &str = "freebsd";

#[cfg(target_os = "linux")]
pub const DEFAULT_OS: &str = "linux";

#[cfg(target_os = "macos")]
pub const DEFAULT_OS: &str = "darwin";

#[cfg(target_os = "openbsd")]
pub const DEFAULT_OS: &str = "openbsd";

#[cfg(target_os = "solaris")]
pub const DEFAULT_OS: &str = "solaris";

#[cfg(target_os = "windows")]
pub const DEFAULT_OS: &str = "windows";

const VALID_OS: &[&str] = &[
    "darwin",
    "freebsd",
    "linux",
    "openbsd",
    "solaris",
    "windows",
];

const DEFAULT_VERSION: &str = "latest";
const NO_COLOR: &str = "NO_COLOR";

// Checks the environment to see if NO_COLOR is in use.
pub fn no_color() -> bool {
    env::var_os(NO_COLOR).is_some()
}

// Ensure that the installation dir exists and is a directory.
fn is_valid_install_dir(s: &OsStr) -> Result<(), OsString> {
    let path = Path::new(&s);

    if !path.exists() {
        return Err("install-dir does not exist".into());
    }

    if !path.is_dir() {
        return Err("install-dir is not a directory".into());
    }

    Ok(())
}

fn create_app<'a, 'b>() -> App<'a, 'b> {
    let mut app = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        // Flags
        .arg(
            Arg::with_name("CHECK")
                .long("check")
                .help("Check for the latest version and exit without downloading.")
                .takes_value(false)
                .conflicts_with_all(&[
                    "BUILD",
                    "QUIET",
                ])
        )
        .arg(
            Arg::with_name("DOWNLOAD_ONLY")
                .long("download-only")
                .short("D")
                .help("Only download the product, do not install it. Implies --keep.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("KEEP")
                .long("keep")
                .short("k")
                .help("Keep downloaded zipfile after install.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("LIST_PRODUCTS")
                .long("list-products")
                .short("l")
                .help("List all available HashiCorp products.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("NO_VERIFY_SIGNATURE")
                .long("no-verify-signature")
                .help("Disable GPG signature verification.")
                .takes_value(false)
        )
        .arg(
            Arg::with_name("QUIET")
                .long("quiet")
                .short("q")
                .help("Silence all non-error output")
                .takes_value(false)
        )
        // Options
        .arg(
            Arg::with_name("ARCH")
                .long("arch")
                .short("a")
                .help("Specify product architecture to download.")
                .default_value(DEFAULT_ARCH)
                .possible_values(VALID_ARCH)
        )
        .arg(
            Arg::with_name("BUILD")
                .long("build")
                .short("b")
                .help("Specify product build version to download.")
                .default_value(DEFAULT_VERSION)
                .value_name("VERSION")
        )
        .arg(
            Arg::with_name("INSTALL_DIR")
                .long("install-dir")
                .short("d")
                .help("Specify directory to install product to.")
                .takes_value(true)
                .value_name("DIR")
                .validator_os(is_valid_install_dir)
        )
        .arg(
            Arg::with_name("OS")
                .long("os")
                .short("o")
                .help("Specify product OS family to download.")
                .default_value(DEFAULT_OS)
                .possible_values(VALID_OS)
        )
        // Positional
        .arg(
            Arg::with_name("PRODUCT")
                .help("Name of the Hashicorp product to download.")
                .index(1)
                .takes_value(true)
                .possible_values(PRODUCTS_LIST)
                .required_unless_one(&[
                    "COMPLETIONS",
                    "LIST_PRODUCTS",
                ])
        );

    if no_color() {
        app = app.setting(AppSettings::ColorNever);
    }

    #[cfg(feature = "shell_completion")]
    {
        app = app.arg(
            Arg::with_name("COMPLETIONS")
                .long("completions")
                .help("Generate shell completions for the given shell")
                .takes_value(true)
                .value_name("SHELL")
                .possible_values(&Shell::variants())
        );
    }


    app
}

pub fn parse_args<'a>() -> ArgMatches<'a> {
    create_app().get_matches()
}

#[cfg(feature = "shell_completion")]
pub fn gen_completions(shell: &str) {
    // Value of shell was checked during CLI parsing.
    let shell = Shell::from_str(shell).unwrap();

    create_app().gen_completions_to(crate_name!(), shell, &mut io::stdout());
}
