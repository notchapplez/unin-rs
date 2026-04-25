use crate::cmake::compile_cmake;
use crate::rust::compile_rust;
use colored::Colorize;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};
use path_absolutize::Absolutize;

#[derive(Debug)]
struct CustomError(Vec<String>);
impl std::fmt::Display for CustomError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Errors: {:?}", self.0)
    }
}
impl std::error::Error for CustomError {}

pub fn absolutize_path(path: String) -> PathBuf {
    let path = PathBuf::from(path);
    let absolutized_path = path.absolutize().unwrap();
    absolutized_path.to_path_buf()
}

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
            "build.meson" => todo!(),
            "configure" => todo!(),
            "build.zig" => todo!(),
            _ => {}
        }
    }
}
pub fn detect_clean(directory: String) {
    let mut path = PathBuf::new();
    if directory.is_empty() {
        println!("Path is empty, nothing to clean");
        path = env::current_dir().unwrap()
    } else {
        path.clear();
        path.push(&directory);
    }
    for got_file in fs::read_dir(&path).unwrap() {
        let entry = got_file.unwrap();
        let os_filename = entry.file_name();
        let filename = os_filename.into_string().unwrap();
        match filename.as_str() {
            "Cargo.toml" => crate::rust::clean(path.clone()),
            "CMakeLists.txt" => crate::cmake::clean(path.clone()),
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

pub fn install_to_bin(executables: Vec<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let mut errors: Vec<String> = Vec::new();
    for binary in executables {
        let filename = binary.file_name().unwrap().to_str().unwrap().to_owned();
        let destination = format!("/usr/local/bin/{}", filename);

        let clean = Command::new("sudo")
            .arg("rm")
            .arg(&destination)
            .arg("-f")
            .output()?
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
            errors.push(format!("{}", binary.to_str().unwrap()));
        } else {
            if Command::new("sudo")
                .args(["chmod", "+x", &destination])
                .status()
                .is_ok() {
                println!("Copied {} to {}", binary.to_str().unwrap(), destination.green());
            } else {
                println!("Failed to copy {} {}", binary.to_str().unwrap(), destination.green());
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(Box::new(CustomError(errors)))
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
