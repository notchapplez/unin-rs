use std::{path::PathBuf, process::exit};
use std::io::{BufRead, Write};
use colored::Colorize;
use dialoguer::console::strip_ansi_codes;

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
	for line in reader.lines() {
		if let Ok(mut content) = line { //if the line is fine
			if content.contains("CC") || content.contains("LD") || content.contains("LINK") {
				content = strip_ansi_codes(&content.to_string().to_owned()).to_string();
				content = content.trim().to_string();
				let mut content_vec = content.split_whitespace().map(String::from).collect::<Vec<String>>();
				content_vec[0] = content_vec[0].blue().bold().to_string();
				content_vec[1..].iter_mut().for_each(|s| {
					*s = s.purple().bold().to_string();
				});
				content = content_vec.join(" ");
				print!("\r\x1B[K{}", content.trim_end());
				std::io::stdout().flush().unwrap();
			}
		}
	}


    exit(0)
	//make install --prefix=/usr/local
}
