pub mod cmake;
pub mod installer;
pub mod make;
mod rust;
pub mod tools;
pub mod zig;
pub mod gradraw;
pub use gradraw::*;

use crate::tools::*;
use clap::{Parser, ValueEnum};
use colored::Colorize;
use std::io;
use std::io::Write;
use std::path::PathBuf;
use std::process::exit;
use unin::registry;

#[derive(Parser, Debug)]
#[command(name = "unin", version = "0.1.0", author = "notchapplez")]
struct Cli {
    #[arg(
        long,
        value_enum,
        help = "set up the necessary tools for the compilation"
    )]
    setup: Option<SetupMode>,

    #[arg(
        value_name = "PATH",
        default_value = ".",
        help = "Path to the directory to compile"
    )]
    path: PathBuf,

    #[arg(long, default_value = "false", help = "Skip the install step")]
    noinstall: bool,

    #[arg(
        long,
        default_value = "false",
        help = "debug purposes",
        required = false
    )]
    test: bool,

    #[arg(
    long,
    value_name = "PATH",
    num_args = 0..=1,
    default_missing_value = ".",
    help = "Clean artefacts built",
    required = false,
    conflicts_with_all = ["setup", "noinstall", "test", "path"]
    )]
    clean: Option<PathBuf>,
}

#[derive(Clone, Debug, ValueEnum)]
enum SetupMode {
    Full,
    Rust,
    Cmake,
    Make,
    Go,
    Zig,
    Swift,
    Haskell,
    D,
}

fn main() {
    let cli = Cli::parse();

    if cli.clean.is_some() {
        println!(
            "Cleaning {}",
            cli.clean.clone().unwrap().to_str().unwrap().yellow()
        );
        let _ = io::stdout().flush();
        detect_clean(cli.clean.unwrap().to_str().unwrap().to_owned());
        exit(0);
    }
    if cli.test {
        registry::temp_test();
        exit(0)
    }

    if let Some(mode) = cli.setup {
        match mode {
            SetupMode::Full => installer::setup_files_full(),
            SetupMode::Rust => installer::setup_files_lang("rust".to_owned()),
            SetupMode::Cmake => installer::setup_files_lang("cmake".to_owned()),
            SetupMode::Make => installer::setup_files_lang("make".to_owned()),
            SetupMode::Go => installer::setup_files_lang("go".to_owned()),
            SetupMode::Zig => installer::setup_files_lang("zig".to_owned()),
            SetupMode::Swift => installer::setup_files_lang("swift".to_owned()),
            SetupMode::Haskell => installer::setup_files_lang("haskell".to_owned()),
            SetupMode::D => installer::setup_files_lang("d".to_owned()),
        }
    }

    detect(cli.path.to_str().unwrap().to_owned(), cli.noinstall);
}

//meow!
