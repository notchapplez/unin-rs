use crate::tools::{
    //imports the functions from the tools module
    find_files_because_the_user_is_too_lazy,
    install_to_bin,
};
use colored::Colorize; //other imports
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
use std::fmt::Write;
use std::io::BufRead;
use std::{
    env, fs,
    path::PathBuf,
    process::{Command, Stdio},
};
use std::path::Path;
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

    let pkg_count = cargo_pkg_count(&directory);

    let pb = ProgressBar::new(pkg_count as u64);
    pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({eta})",
        )
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| {
            write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        })
        .progress_chars("▰ ▱"),
    );

    let mut child = Command::new("cargo")
        .args([
            "build",
            "--release",
            "--color",
            "always",
            "--message-format",
            "json",
        ])
        .current_dir(&directory)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to compile");

    let stdout = child.stdout.take().unwrap();
    let reader = std::io::BufReader::new(stdout);
    for line in reader.lines() {
        if let Ok(line_content) = line {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line_content) {
                if let Some(reason) = json.get("reason").and_then(|r| r.as_str()) {
                    if reason == "compiler-artifact" {
                        pb.inc(1);
                    }
                }
            }
        }
    }
    pb.finish_with_message("Finished compiling Rust project.");

    println!();
    let releases_folder = PathBuf::from(format!("{}/target/release", directory.to_str().unwrap()));
    if !releases_folder.exists() {
        fs::create_dir_all(releases_folder.clone()).expect("Failed to create directory");
    }

    let binaries = find_files_because_the_user_is_too_lazy(directory); //finds the binaries in the path

    if noinstall {
        //skips installation if user
        println!("Skipping the installation as --noinstall was given.");
        return;
    }

    install_to_bin(binaries.clone()); //installs the dropped binaries
    println!(
        //prints an end message
        "{}",
        "The compilation and installation is finished. No error reported.".green()
    );

    for binary in binaries.clone() {
        let absolute_path = binary.absolutize().unwrap();
        let temp_binary: UninPackage = UninPackage { name: binary.to_str().unwrap().split('/').collect::<Vec<&str>>().last().unwrap().to_string(), paths: vec![PathBuf::from(absolute_path.as_ref())], change_date: String::from(time_create()), updated: false };
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

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines: Vec<&str> = stdout.lines().collect();

    lines.len() - 1
}
