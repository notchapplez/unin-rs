use std::path::PathBuf;
use colored::Colorize;
use regex::Regex;

pub fn compile_cmake(directory: PathBuf, noinstall: bool) {
    println!("Now compiling {}", directory.to_str().unwrap().yellow());
    let CMakeListsPath = format!("{}/CMakeLists.txt", directory.to_str().unwrap());
    println!("{}", CMakeListsPath);

    let CMakeLists: PathBuf = PathBuf::from(&CMakeListsPath);
    let opened_file = std::fs::read_to_string(CMakeLists).unwrap();
    let mut resulting_string: String = String::new();
    for line in opened_file.lines() {
        if line.contains("option(") {
            line.split("(");
            let mut linecontentfiltered = String::new();
            if line.contains("option(") {
                line.split("(");
                linecontentfiltered = format!("{}", line.replace("option(", "").replace(")", ""));

                let re = Regex::new(r#""[^"]*"|\S+"#).unwrap();
                let result: Vec<&str> = re.find_iter(linecontentfiltered.as_str())
                    .map(|m| m.as_str())
                    .collect();

                let result_string = result.join(" ");
                println!("{}", result_string.bold().green());
                //so put selection using numbers or smth here idk user needs to select
            }
        }
    }

}
