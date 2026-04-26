use crate::tools::{
    //imports the functions from the tools module
    find_files_because_the_user_is_too_lazy,
    install_to_bin,
};
use colored::Colorize; //other imports
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::io::{BufRead, BufReader};
use std::io::Write; // Import `Write` for `flush()` method
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};
use std::path::Path;
use cargo_metadata::{CargoOpt, MetadataCommand, TargetKind};
use libc::{__c_anonymous_sockaddr_can_can_addr, mq_notify};
use path_absolutize::Absolutize;
use unin::{registry_write, time_create, UninPackage};

pub fn compile_rust(directory: PathBuf, noinstall: bool) {
    let mut full_path = String::new();
    //defines the function
    if directory == PathBuf::from(".") {
        let directory = env::current_dir().unwrap();
        full_path = String::from(directory.to_str().unwrap());
    }
    println!("Now compiling {}", full_path.yellow()); //prints a start message

    let mut child = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--color",
            "always",
        ])
        .current_dir(&directory)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to compile");

    let stderr = child.stderr.take().unwrap();
    let reader = std::io::BufReader::new(stderr);
    for line in reader.lines() { 
        match line {
            //matches the line
            Ok(content) => {
                //if the line is fine
                if content.contains("Compiling") {
                    let soos = content.trim().clone().to_string();
                    print!("\r\x1B[K{}", soos.bold().purple());
                    let _ = std::io::stdout().flush().unwrap();
                }
            }
            Err(e) => println!("Error reading stdout: {}", e), //if there is an error, print the error
        }
    }
    
    println!();
    let releases_folder = PathBuf::from(format!("{}/target/release", directory.to_str().unwrap()));
    if !releases_folder.exists() {
        fs::create_dir_all(releases_folder.clone()).expect("Failed to create directory");
    }

    let binaries = find_files_because_the_user_is_too_lazy(releases_folder); //finds the binaries in the path

    if noinstall {
        //skips installation if user
        println!("Skipping the installation as --noinstall was given.");
        return;
    }

    let _ = install_to_bin(binaries.clone()); //installs the dropped binaries
    println!(
        //prints an end message
        "{}",
        "The compilation and installation is finished. No error reported.".green()
    );

    for binary in binaries.clone() {
        let last_item_binary = binary.to_str().unwrap().split("/").collect::<Vec<&str>>().last().unwrap().to_string();
        let installed_absolute_path = format!("/usr/local/bin/{}", last_item_binary);
        println!("Found {} in {}", installed_absolute_path, installed_absolute_path.green());
        let temp_binary: UninPackage = UninPackage { name: binary.to_str().unwrap().split('/').collect::<Vec<&str>>().last().unwrap().to_string(), paths: vec![PathBuf::from(installed_absolute_path)], change_date: String::from(time_create()), updated: false };
        registry_write(&temp_binary);
        println!("Writing\n {} to registry", temp_binary);
    }
}
pub fn clean(directory: PathBuf) {
    let clean_process_cargo = Command::new("cargo")
        .args(["clean"])
        .current_dir(&directory)
        .output();
    println!("{}", String::from_utf8_lossy(&clean_process_cargo.unwrap().stderr).trim().clone());
}

fn cargo_pkg_count(directory: &PathBuf) -> usize {
    let output = Command::new("cargo")
        .current_dir(&directory)
        .args(&[
            "tree",
            "--prefix",
            "none",
            "-e",
            "normal",
            "--workspace",
            "--all-features",
        ])
        .output()
        .expect("Failed to execute cargo tree");

    let mut stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines: Vec<&str> = Vec::new();
    for line in stdout.lines() {
        if !line.is_empty() {
            lines.push(line);
        } else {
            continue;
        }
    }
    let mut final_lines: Vec<&str> = Vec::new();
    lines.dedup();
    for line in lines {
        if !line.contains("(proc-macro)") {
            final_lines.push(line);
        } else {
            continue;
        }
    }
    let lines_count = final_lines.len() + 1;
    println!("{} packages found", lines_count);
    return lines_count
}
