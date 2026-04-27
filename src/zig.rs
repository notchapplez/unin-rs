use colored::Colorize;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::{exit, Command};
use crate::tools::find_files_because_the_user_is_too_lazy;

pub fn build_zig(directory: PathBuf, noinstall: bool) {
    let mut zig_build_process = Command::new("zig")
        .current_dir(&directory)
        .arg("build")
        .arg("-Doptimize=ReleaseFast")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Couldn't start the zig build process.");

    if let Some(stdout) = zig_build_process.stdout.take() {
        let buf_reader = std::io::BufReader::new(stdout);

        for line in buf_reader.lines() {
            match line {
                Ok(content) => {
                    let content = content.replace('\r', "");
                    println!("{}", content.bold().purple());
                }
                Err(e) => println!("Error reading stdout: {}", e),
            }
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
    let _  = executables.iter().for_each(|executable| {println!("Found {}", executable.to_str().unwrap().green());});


}
