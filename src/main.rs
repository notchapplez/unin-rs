use colored::Colorize;
use spinners::Spinner;
use spinners::Spinners::Dots;
use std::os::unix::fs::PermissionsExt;
use std::thread::sleep;
use std::time::Duration;
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};

fn main() {
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
    detect(argument.clone())
}

fn detect(path: String) {
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
            "Cargo.toml" => compile_rust(PathBuf::from(&path)),
            "CMakeLists.txt" => todo!(),
            _ => {}
        }
    }
}
fn compile_rust(directory: PathBuf) {
    println!("Now compiling {}", directory.to_str().unwrap().yellow());

    let mut sp = Spinner::new(Dots, "Compiling... Hang on, this can take up to 10 minutes depending on the hardware and complexity of the application being compiled.".into());

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
                sp.stop_with_newline();
                if status.success() {
                    println!("{}", "Compilation successful!".green());
                } else {
                    eprintln!("Process failed with code: {:?}", status.code());
                }
                break;
            }
            Ok(None) => {
                sleep(Duration::from_millis(100));
            }
            Err(e) => {
                sp.stop();
                eprintln!("Error waiting for process: {}", e);
                break;
            }
        }
    }

    let binaries = find_files_because_the_user_is_too_lazy(directory);

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
        let destination = format!("/usr/bin/{}", filename);

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
