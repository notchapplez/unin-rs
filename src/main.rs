use clap::{Parser, command, ValueEnum};
use colored::Colorize;
use std::os::unix::fs::PermissionsExt;
use std::ptr::copy_nonoverlapping;
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};
use std::time::Duration;
use indicatif::ProgressBar;
use os_release::OsRelease;
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

    if let Some(mode) = cli.setup {
        match mode {
            SetupMode::Full => setup_files_full(),
            SetupMode::Rust => setup_files_lang("rust".to_owned()),
            SetupMode::Cmake => setup_files_lang("cmake".to_owned()),
            SetupMode::Go => setup_files_lang("go".to_owned()),
            SetupMode::Zig => setup_files_lang("zig".to_owned()),
            SetupMode::Swift => setup_files_lang("swift".to_owned()),
            SetupMode::Haskell => setup_files_lang("haskell".to_owned()),
            SetupMode::D => setup_files_lang("d".to_owned()),
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

fn setup_files_full() {
    let pm = detect_package_manager();

    if pm == "UNKNOWN" {
        println!("Unknown package manager: {}", pm);
        return;
    }

    let packages = get_full_packages(pm);

    println!("Installing all langs for {}...", pm.yellow());
    for package in packages {
        println!("Installing {}...", package.cyan());
        install_package(pm, package);
    }

    println!("{}", "All packages installed successfully!".green());
}

fn setup_files_lang(lang: String) {
    let pm = detect_package_manager();

    if pm == "UNKNOWN" {
        println!("Unknown package manager: {}", pm);
        return;
    }

    let package = match lang.as_str() {
        "rust" => "rustup",
        "cmake" => "cmake",
        "go" => get_go_package(pm),
        "zig" => "zig",
        "swift" => "swift",
        "haskell" => "ghc",
        "d" => "dmd",
        _ => {
            println!("Language not supported yet");
            return;
        }
    };

    install_package(pm, package);
}

fn detect_package_manager() -> &'static str {
    let candidates = ["apt", "dnf", "yum", "zypper", "pacman", "apk"];

    for pm in candidates {
        if Command::new(pm).arg("--version").output().is_ok() {
            return pm;
        }
    }

    "UNKNOWN"
}

fn get_full_packages(pm: &str) -> Vec<&'static str> {
    match pm {
        "apt" => vec![
            "rustup",
            "golang-go",
            "zig",
            "swift",
            "ghc",
            "dmd",
            "cmake",
            "make",
            "gcc",
            "git",
            "pkg-config",
        ],
        "dnf" | "yum" => vec![
            "rustup",
            "golang",
            "zig",
            "swift",
            "ghc",
            "dmd",
            "cmake",
            "make",
            "gcc",
            "git",
            "pkgconfig",
        ],
        "zypper" => vec![
            "rustup",
            "go",
            "zig",
            "swift",
            "ghc",
            "dmd",
            "cmake",
            "make",
            "gcc",
            "git",
            "pkg-config",
        ],
        "pacman" => vec![
            "rustup",
            "go",
            "zig",
            "swift",
            "ghc",
            "dmd",
            "cmake",
            "make",
            "gcc",
            "git",
            "pkg-config",
        ],
        "apk" => vec![
            "rustup",
            "go",
            "zig",
            "swift",
            "ghc",
            "dmd",
            "cmake",
            "make",
            "gcc",
            "git",
            "pkgconfig",
        ],
        _ => {
            println!("Unknown package manager: {}", pm);
            vec![]
        }
    }
}

fn get_go_package(pm: &str) -> &'static str {
    match pm {
        "apt" => "golang-go",
        _ => "go",
    }
}

fn install_package(pm: &str, package: &str) {
    let (cmd, args): (&str, Vec<&str>) = match pm {
        "apt" => ("sudo", vec!["apt", "install", "-y", package]),
        "dnf" => ("sudo", vec!["dnf", "install", "-y", package]),
        "yum" => ("sudo", vec!["yum", "install", "-y", package]),
        "zypper" => ("sudo", vec!["zypper", "install", "-y", package]),
        "pacman" => ("sudo", vec!["pacman", "-S", "--noconfirm", package]),
        "apk" => ("sudo", vec!["apk", "add", package]),
        _ => {
            println!("Unknown package manager: {}", pm);
            return;
        }
    };

    Command::new(cmd)
        .args(args)
        .status()
        .ok();
}


fn detect(path: String, noinstall: bool) {
    let mut new_path: PathBuf = PathBuf::new();
    if path.is_empty() {
        println!("Path is empty");
        new_path = env::current_dir().unwrap()
    } else {
        new_path.clear();
        new_path.push(&path);
    }
    for matching_file in fs::read_dir(&new_path).unwrap() {
        let entry = matching_file.unwrap();
        let os_filename = entry.file_name();
        let filename = os_filename.into_string().unwrap();
        match filename.as_str() {
            "Cargo.toml" => compile_rust(PathBuf::from(&path), noinstall),
            "CMakeLists.txt" => todo!(),
            _ => {}
        }
    }
}
fn compile_rust(directory: PathBuf, noinstall: bool) {
    println!("Now compiling {}", directory.to_str().unwrap().yellow());

    let pb = ProgressBar::new_spinner();
    pb.set_message("Compiling...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));

    let mut child = Command::new("cargo") //compile the stuff
        .args(["build", "--release"])
        .current_dir(&directory)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("Failed to compile");

    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                pb.finish_with_message("Compilation successful!");
                break;
            }
            Ok(None) => {
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => {
                pb.finish_with_message(format!("Error: {}", e));
                break;
            }
        }
    }

    let binaries = find_files_because_the_user_is_too_lazy(directory);

    if noinstall {
        println!("Skipping installation as --noinstall was given.");
        return;
    }

    if !sudo() {
        println!(
            "{}",
            "The command wasn't run with superuser privileges. Retrying...".yellow()
        );
    }

    install_to_bin_as_sudo_because_the_fucking_user_didnt_supply_sudo(binaries);
    println!(
        "{}",
        "The compilation and installation is finished. No error reported.".green()
    )
}
fn find_files_because_the_user_is_too_lazy(directory: PathBuf) -> Vec<PathBuf> {
    let releases_folder = format!("{}/target/release", directory.to_str().unwrap());
    let mut paths: Vec<PathBuf> = vec![];
    for file in fs::read_dir(PathBuf::from(releases_folder)).unwrap() {
        let file_path = file.unwrap().path();
        paths.push(file_path);
    }

    //println!("{:?} found", paths);

    find_executable_file_in_the_goddamn_end_folder(paths.clone())
}

fn install_to_bin_as_sudo_because_the_fucking_user_didnt_supply_sudo(executables: Vec<PathBuf>) {
    for binary in executables {
        let filename = binary.file_name().unwrap().to_str().unwrap().to_owned();
        let destination = format!("/usr/local/bin/{}", filename);

        let clean = Command::new("sudo")
            .arg("rm")
            .arg(&destination)
            .arg("-f")
            .output()
            .unwrap()
            .status;
        if clean.success() {
            println!("{}", "Removal of the old version is complete!".green())
        } else {
            println!("{}", "skipping old version removal as not present".green())
        }
        let status = Command::new("sudo")
            .arg("cp")
            .arg(&binary)
            .arg(&destination)
            .status()
            .expect("failed to execute process");

        if !status.success() {
            println!("Failed to copy binaries ");
        } else {
            Command::new("sudo")
                .args(["chmod", "+x", &destination])
                .status()
                .ok();
            println!("{}", "Installation to /usr/bin/ is complete!".green())
        }
    }
}
fn sudo() -> bool {
    unsafe { libc::geteuid() == 0 }
}
fn find_executable_file_in_the_goddamn_end_folder(files: Vec<PathBuf>) -> Vec<PathBuf> {
    files
        .into_iter()
        .filter(|path| {
            if let Ok(metadata) = fs::metadata(path) {
                metadata.is_file() && (metadata.permissions().mode() & 0o111 != 0)
            } else {
                false
            }
        })
        .collect()
}
