use std::{process::Command, time::Duration, path::PathBuf};
use colored::Colorize;
use indicatif::ProgressBar;

fn build(directory: PathBuf, noinstall: bool) {
    let file_content = std::fs::read_to_string(format!("{}/Makefile", directory.to_str().unwrap())).unwrap();
    for line in file_content.lines() {
        println!("{line}")
    }

}
