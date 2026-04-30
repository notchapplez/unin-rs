use colored::Colorize;
use rand:: RngExt;
use std::io::{BufRead, BufReader, Read, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio, exit};

pub fn start_meson(directory: PathBuf, noinstall: bool) {
    let mut setup = Command::new("meson")
        .args(&["setup", "build"]) //build is the path to the build directory!
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .current_dir(directory.clone())
        .spawn();

    let stdout = setup.as_mut().unwrap().stdout.take().unwrap();
    let reader = BufReader::new(stdout);
    let mut full_content = String::new();
    let mut has_error = false;
    for line in reader.lines() {
        if let Ok(content) = line {
            let raw_content = content.clone();
            let shc = content.clone();
            if shc.contains("ERROR") {
                has_error = true;
                print!("\r\x1B[K{}", shc.red().underline().bold());
                std::io::stdout().flush().unwrap();
                full_content.push_str(format!("{}\n", &raw_content.clone()).as_str());
                continue;
            }
            print!("\r\x1B[K{}", content.purple().bold());
            std::io::stdout().flush().unwrap();
            let mut rng = rand::rng();
            let random_delay: u64 = rng.random_range(30..=60);
            std::thread::sleep(std::time::Duration::from_millis(random_delay));
            full_content.push_str(format!("{}\n", &raw_content.clone()).as_str());
        }
    }
    let waiter = setup.unwrap().wait().unwrap();

    if !waiter.success() {
        println!("\nConfiguration failed.");
    }
    if has_error {
        println!(
            "{} The full error will be printed here.\n",
            "\nAn error occurred while configuring the project."
                .red()
                .bold()
        );
        println!(
            "{}",
            full_content.trim_end().replace("error:", &"error:".red())
        );
        exit(0);
    }

    println!("\nConfiguration finished successfully.");
    let mut child = Command::new("ninja")
        .args(&["-C", "build"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn();

    let childed = child.as_mut().unwrap().stdout.take().unwrap();

    let stdout = childed;
    let reader = BufReader::new(stdout);
    let mut content_full = String::new();
    let mut has_error = false;
    for line in reader.lines() {
        if let Ok(content) = line {
            print!("\r\x1B[K{}", content.green().bold());
            std::io::stdout().flush().unwrap();
            if content.contains("error:") {
                has_error = true;
                content_full.push_str(content.as_str());
                continue;
            }
            content_full.push_str(content.as_str());
            continue;
        }
    }
    let _waiter = child.unwrap().wait().unwrap();
    if has_error {
        println!(
            "Build using ninja failed. If you want, I can show you the output of the build process. I don't care if you want to, here it is:"
        );
        full_content.split("\n").for_each(|line| println!("{}", line));
    }
    if noinstall {
        println!("Build finished successfully.");
        println!("I have no idea where the binaries are. They are somewhere, go find them")
    }
    //i need to implement the install command
    //exactly as i did with make, i need to check if the file provides a build : install rule
    //if it does, i need to run the install command
    //if it doesn't, i need to FUCK OFF
    //and i need to refine the logic for make prolly, i guess, as
    //i checked and this is weird:
    // build install: phony meson-internal__install
    // build meson-internal__install: CUSTOM_COMMAND PHONY | all
    //  COMMAND = /usr/bin/meson install --no-rebuild
    // build uninstall: phony meson-internal__uninstall
    // build meson-internal__uninstall: CUSTOM_COMMAND PHONY


}
