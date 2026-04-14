use crate::cmake::compile_cmake;
use crate::rust::compile_rust;
use colored::Colorize;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

pub fn detect(path: String, noinstall: bool) {
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
            "CMakeLists.txt" => compile_cmake(PathBuf::from(&path), noinstall),
            "Makefile" => todo!(),
            _ => {}
        }
    }
}
pub fn find_files_because_the_user_is_too_lazy(directory: PathBuf) -> Vec<PathBuf> {
    let releases_folder = format!("{}/target/release", directory.to_str().unwrap());
    let mut paths: Vec<PathBuf> = vec![];
    for file in fs::read_dir(PathBuf::from(releases_folder)).unwrap() {
        let file_path = file.unwrap().path();
        paths.push(file_path);
    }

    //println!("{:?} found", paths);

    find_executable_file_in_the_goddamn_end_folder(paths.clone())
}

pub fn install_to_bin_as_sudo_because_the_fucking_user_didnt_supply_sudo(
    executables: Vec<PathBuf>,
) {
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
            println!("{}", "Installation to /usr/local/bin is complete!".green())
        }
    }
}
pub fn sudo() -> bool {
    unsafe { libc::geteuid() == 0 }
}
pub fn find_executable_file_in_the_goddamn_end_folder(files: Vec<PathBuf>) -> Vec<PathBuf> {
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
