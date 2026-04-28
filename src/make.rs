use crate::tools::{find_files_because_the_user_is_too_lazy, install_to_bin};
use colored::Colorize;
use dialoguer::console::strip_ansi_codes;
use std::{
    io::{BufRead, Write},
    path::PathBuf,
    process::exit,
};

pub fn build_make(directory: PathBuf, noinstall: bool) {
    let num_cpus = num_cpus::get();
    let mut make_process = std::process::Command::new("make")
        .args(["-j", &num_cpus.to_string()])
        .current_dir(&directory)
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("Couldn't start the make process.");

    let stderr = make_process.stderr.take().unwrap();
    let reader = std::io::BufReader::new(stderr);
    let mut full_content = String::new();
    let mut has_error = false;
    for line in reader.lines() {
        if let Ok(mut content) = line {
            let raw_content = content.clone();
            if content.contains("CC")
                || content.contains("LD")
                || content.contains("LINK")
                || content.contains("AR")
                || content.contains("checking")
            {
                content = strip_ansi_codes(&content.to_string().to_owned()).to_string();
                content = content.trim().to_string();
                let mut content_vec = content
                    .split_whitespace()
                    .map(String::from)
                    .collect::<Vec<String>>();
                content_vec[0] = content_vec[0].blue().bold().to_string();
                content_vec[1..].iter_mut().for_each(|s| {
                    *s = s.purple().bold().to_string();
                });
                content = content_vec.join(" ");
                print!("\r\x1B[K{}", content.trim_end());
                std::io::stdout().flush().unwrap();
            } else if content.contains("error:") {
                has_error = true;
                full_content.push_str(&raw_content.clone());
                full_content.push('\n');
            }
            full_content.push_str(&raw_content.clone());
            full_content.push('\n');
        }
    }
    let _waiter = make_process
        .wait()
        .unwrap_or_else(|_| panic!("Couldn't wait for the make process to finish."));

    if has_error {
        println!("{}", "\nAn error occurred while compiling.".red().bold());
        println!("{}", "This might be due to a missing dependency or unset environment variables. Check README.md for exact building information.".red().bold());
        println!(
            "Full error: \n{}",
            full_content.trim_end().replace("error:", &"error".red())
        );
    }

    let binaries = find_files_because_the_user_is_too_lazy(directory.clone());
    println!("DEbug binaries: {:?}", binaries);
    if noinstall {
        println!();
        binaries.iter().for_each(|binary| {
            let bin = binary.to_string_lossy().to_string();
            if bin.trim().is_empty() {
                panic!("No binaries found.")
            }
            let test = bin.split("/").last().unwrap();
            println!("File {} dropped.", test.yellow().underline());
        });
        exit(0)
    }
    install_to_bin(binaries);

    exit(0)
    //make install --prefix=/usr/local
}

pub fn clean(directory: PathBuf) {
    let mut clean_process = std::process::Command::new("make")
        .args(["clean"])
        .current_dir(&directory)
        .stderr(std::process::Stdio::piped())
        .stdout(std::process::Stdio::null())
        .spawn()
        .expect("Couldn't start the make process.");
    let waiter = clean_process
        .wait()
        .unwrap_or_else(|_| panic!("Couldn't wait for the make process to finish."));

    if waiter.success() {
        println!("Cleaned successfully.");
    } else {
        println!("Cleaning failed.");
        println!(
            "If you want, you can try to clean manually by running \"make clean\" in the project root."
        )
    }
}
