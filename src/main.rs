pub mod cmake;
pub mod installer;
mod rust;
pub mod tools;
pub mod make;

use crate::tools::*;
use clap::{Parser, ValueEnum};
use std::{
    path::PathBuf,
};
use colored::Colorize;

#[derive(Parser, Debug)]
#[command(name = "unin", version = "0.1.0", author = "notchapplez")]
struct Cli {
    #[arg(
        long,
        value_enum,
        help = "set up the necessary tools for the compilation"
    )]
    setup: Option<SetupMode>,

    #[arg(long,
        value_enum,
        help = "Clean artefacts built")]
    clean: Option<PathBuf>,

    #[arg(
        value_name = "PATH",
        default_value = ".",
        help = "Path to the directory to compile"
    )]
    path: PathBuf,

    #[arg(long, default_value = "false", help = "Skip the install step")]
    noinstall: bool,


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
        println!("Cleaning {}", cli.clean.clone().unwrap().to_str().unwrap().yellow());
        detect_clean(cli.clean.unwrap().to_str().unwrap().to_owned());
    }

    //Set up the languages
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

    /*
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let args_string = format!("Current arguments given are {:?}", args);
    println!("{}", args_string.yellow());
    if args.is_empty() {
        let cwd = env::current_dir().unwrap();
        args = vec![cwd.into_os_string().into_string().unwrap()];
        println!("Fixed empty args to {:?}\n", args);
    }
    let argument = &args[0];
    detect(argument.clone()) */
}

//meow!
