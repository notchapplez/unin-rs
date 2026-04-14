pub mod cmake;
pub mod installer;
mod rust;
pub mod tools;

use crate::tools::*;
use clap::{Parser, ValueEnum};
use std::{
    path::PathBuf,
};
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

    #[arg(long)]
    noinstall: bool,
}

#[derive(Clone, Debug, ValueEnum)]
enum SetupMode {
    Full,
    Rust,
    Cmake,
    Go,
    Zig,
    Swift,
    Haskell,
    D,
}

fn main() {
    let cli = Cli::parse();

    //Set up the languages
    if let Some(mode) = cli.setup {
        match mode {
            SetupMode::Full => installer::setup_files_full(),
            SetupMode::Rust => installer::setup_files_lang("rust".to_owned()),
            SetupMode::Cmake => installer::setup_files_lang("cmake".to_owned()),
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
