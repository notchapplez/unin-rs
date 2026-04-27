use colored::Colorize;
use std::io::{BufRead, Write};
use std::path::PathBuf;
use std::process::{exit, Command};
use crate::tools::{find_files_because_the_user_is_too_lazy, install_to_bin};

pub fn build_zig(directory: PathBuf, noinstall: bool) {
    let mut zig_build_process = Command::new("zig")
        .current_dir(&directory)
        .arg("build")
        .arg("-Doptimize=ReleaseFast")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("Couldn't start the zig build process.");

    let stderr = zig_build_process.stderr.take().unwrap();
    let reader = std::io::BufReader::new(stderr);
    for line in reader.lines() {
        if let Ok(actual_line) = line {
            print!("\r\x1B[K{}", actual_line.bold().purple());
        }
    }

    let _waiter = &zig_build_process
        .wait()
        .expect("Couldn't wait for the zig build process.");


    let out_dir = PathBuf::from(format!("{}/zig-out/bin", directory.to_str().unwrap()));
    if noinstall {
        println!("{}", "Skipping installation of zig binaries".yellow());
        println!("You can find the binaries in {}", out_dir.to_str().unwrap().yellow());
        exit(0)
    }
    println!("Installing zig binaries");
    let executables = find_files_because_the_user_is_too_lazy(out_dir.clone());
    let _ = install_to_bin(executables);

    exit(0)
}
