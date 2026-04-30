use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::Command;
use colored::Colorize;

fn start_meson(directory: PathBuf, noinstall: bool) {
    let setup = Command::new("meson")
        .args(&["setup", "build"]) //build is the path to the build directory!
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .current_dir(directory.clone())
        .spawn();

    let stdout = setup.unwrap().stdout.unwrap();
    let reader = BufReader::new(stdout);
	let mut full_content = String::new();
	let mut has_error = false;
    for line in reader.lines() {
        if let Ok(content) = line {
            let raw_content = content.clone();
            let shc = content.clone();
            if shc.contains("Compiler")
                || shc.contains("dependency")
                || shc.contains("Project")
                || shc.contains("Build targets")
                || shc.contains("header")
            {
				println!("{}", shc);
				full_content.push_str(&raw_content.clone());
				continue;
			} else if shc.contains("error") {
				has_error = true;
				full_content.push_str(&raw_content.clone());
				continue;
			}
        }
    }
	if has_error {
		println!("{} The full error will be printed here.", "\nAn error occurred while configuring the project.".red().bold());
		println!("{}", full_content.trim_end().replace("error:", &"error:".red()));
	}
}
