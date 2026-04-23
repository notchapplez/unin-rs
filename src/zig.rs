use colored::Colorize;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::Command;

fn build_zig(directory: PathBuf, noinstall: bool) {
    let mut zig_build_process = Command::new("zig")
        .current_dir(&directory)
        .arg("build")
        .arg("-Doptimize=ReleaseFast")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn()
        .expect("Couldn't start the zig build process.");

    let waiter = &zig_build_process
        .wait()
        .expect("Couldn't wait for the zig build process.");
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

    let out_dir = format!("{}/zig-out", directory.to_str().unwrap());
}
