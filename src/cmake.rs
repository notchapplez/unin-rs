use colored::Colorize;
use dialoguer::Input;
use indicatif::ProgressBar;
use regex::Regex;
use std::{
    fs as filesystem, path::{Path, PathBuf}, process as commands, process::Stdio, thread as sleeping,
    time::Duration,
};
use std::io::{stdout, BufRead, BufReader};

pub fn compile_cmake(directory: PathBuf, noinstall: bool) {
    let build_dir: PathBuf = PathBuf::from(format!("{}/build", directory.to_str().unwrap()));
    println!("Now configuring {}", directory.to_str().unwrap().yellow());
    let cmake_lists_path = format!("{}/CMakeLists.txt", directory.to_str().unwrap());

    let cmake_lists: PathBuf = PathBuf::from(&cmake_lists_path);
    let opened_file = std::fs::read_to_string(cmake_lists).unwrap();
    println!(
        "{} | {} | {}",
        "Option".bold().red(),
        "Description".bold().red(),
        "Default value".bold().red()
    );
    sleeping::sleep(Duration::from_millis(500));
    let re = Regex::new(r#""[^"]*"|\S+"#).unwrap();

    for line in opened_file.lines() {
        if line.contains("option(") {
            line.split("(");
                let linecontentfiltered =
                    format!("{}", line.replace("option(", "").replace(")", ""));


                let result: Vec<&str> = re
                    .find_iter(linecontentfiltered.as_str())
                    .map(|m| m.as_str())
                    .collect();

                let result_string = result.join(" ");
                println!("{}", result_string.bold().green());
                sleeping::sleep(Duration::from_millis(10));

        }
    }
    let arguments_history: PathBuf =
        PathBuf::from(format!("{}/.unin_arguments", directory.to_str().unwrap()));

    if arguments_history.exists() {
        let old_argument_read = filesystem::read_to_string(&arguments_history).unwrap().replace("-DCMAKE_INSTALL_PREFIX=/usr/local", "").replace("-Wno-dev", "");
        println!("Following arguments were found as they were present in the .unin_arguments file: {}", old_argument_read.bold().yellow().underline());

        let check_user_continue_old_args: String = Input::new()
            .with_prompt("Do you want to use the already used, cached arguments? (y/n)")
            .interact_text()
            .unwrap();
        if check_user_continue_old_args.trim() == "y" {
            let old_args = filesystem::read_to_string(&arguments_history).unwrap();
            configure(old_args.split(" ").collect(), &directory);

            make(directory,build_dir, noinstall);
        } else {
            let input: String = Input::new()
                .allow_empty(true)
                .with_prompt("Add Arguments now. Prefix your project arguments with -D and use a space for separation, for example -DBUILD_SHARED_LIBS=ON. Other arguments will also be used, like warning flags.")
                .interact_text()
                .unwrap();

            println!();

            let full_cmake_input = format!("{} -DCMAKE_INSTALL_PREFIX=/usr/local",&input);
            let input_vec: Vec<&str> = input.split(' ').collect();
            configure(input_vec, &directory);
            filesystem::remove_file(&arguments_history).unwrap();
            filesystem::write(arguments_history, full_cmake_input.clone()).unwrap();

            make(directory, build_dir, noinstall);
        }
    } else {
        let input: String = Input::new()
            .allow_empty(true)
            .with_prompt("Add Arguments now. Prefix your project arguments with -D and use a space for separation, for example -DBUILD_SHARED_LIBS=ON. Other arguments will also be used, like warning flags.")
            .interact_text()
            .unwrap();
        println!();

        println!("{}", input);
        let full_cmake_input = format!("{} -DCMAKE_INSTALL_PREFIX=/usr/local -Wno-dev",&input);
        let input_vec: Vec<&str> = full_cmake_input.split(" ").collect();
        filesystem::write(arguments_history, full_cmake_input.clone()).unwrap();
        println!("{:?}", input_vec);
        configure(input_vec, &directory);

        make(directory, build_dir, noinstall);
    }
}

fn configure(input_vec: Vec<&str>, directory: &Path) {
    println!();
    filesystem::create_dir_all(format!("{}/build", directory.to_str().unwrap())).unwrap();

    let build_dir = format!("{}/build", directory.to_str().unwrap());
    let mut configure_cmake = commands::Command::new("cmake")
        .current_dir(build_dir)
        .arg("..")
        .arg("-Wno-dev")
        .args(input_vec)
        .output()
        .expect("Failed to configure");

    print!("{}", String::from_utf8_lossy(&configure_cmake.stdout).yellow());
    eprint!("{}", String::from_utf8_lossy(&configure_cmake.stderr).red());

    if !configure_cmake.status.success() {
        eprintln!("CMake configure failed. See the output above for more information.");
        std::process::exit(1);
    }
}
fn make(directory: PathBuf, build_directory: PathBuf, noinstall: bool) {

    println!("{}", "Starting to compile in three seconds. To cancel, press Ctrl+C".blue());
    sleeping::sleep(Duration::from_secs(3));

    let cores = num_cpus::get();
    println!("Now compiling {}", directory.to_str().unwrap().yellow());

    if noinstall == true {
        println!("Skipping install step.");
        let mut make_process = commands::Command::new("make")
            .current_dir(build_directory)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start make");

        if let Some(stdout) = make_process.stdout.take() {
            let buf_reader = std::io::BufReader::new(stdout);

            for line in buf_reader.lines() {
                match line {
                    Ok(content) => {
                        let content = content.replace('\r', "");
                        println!("{}", content.bold().green());
                    }
                    Err(e) => println!("Error reading stdout: {}", e),
                }
            }
        }

        if let Some(stderr) = make_process.stderr.take() {
            let buf_reader = std::io::BufReader::new(stderr);

            for line in buf_reader.lines() {
                match line {
                    Ok(content) => {
                        let content = content.replace('\r', "");
                        eprintln!("{}", content.red());
                    }
                    Err(e) => println!("Error reading stderr: {}", e),
                }
            }
        }

        let _ = make_process.wait().expect("Command isn't running.");
    } else {
        let mut make_process = commands::Command::new("cmake")
            .arg("--build")
            .current_dir(&build_directory)
            .arg(".")
            .arg("-j")
            .arg(cores.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start cmake build");

        if let Some(stdout) = make_process.stdout.take() {
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

        if let Some(stderr) = make_process.stderr.take() {
            let buf_reader = std::io::BufReader::new(stderr);

            for line in buf_reader.lines() {
                match line {
                    Ok(content) => {
                        let content = content.replace('\r', "");
                        eprintln!("{}", content.red());
                    }
                    Err(e) => println!("Error reading stderr: {}", e),
                }
            }
        }

        let _ = make_process.wait().expect("Command isn't running.");

        let _make_install_process = commands::Command::new("sudo")
            .current_dir(build_directory)
            .arg("cmake")
            .arg("--install")
            .arg(".")
            .spawn()
            .unwrap();
    }
}
