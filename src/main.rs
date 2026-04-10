use std::{
    path::{PathBuf,},
    env,
    fs
};
use std::fmt::{format, Debug};
use colored::{Color, ColoredString, Colorize};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    println!(" Current args are {:?}", args);
    if args.len() == 0 {
        let cwd = env::current_dir().unwrap();
        args = vec![cwd.into_os_string().into_string().unwrap()];
        println!("Fixed empty args to {:?}", args);
    }
    let argument = &args[0];
    detect(argument.clone())
}


fn detect(path: String) {
    let mut new_path: PathBuf = PathBuf::new();
    if path.is_empty() {
        println!("Path is empty");
        new_path = env::current_dir().unwrap()
    } else {
        new_path.clear();
        new_path.push(path);
    }
    let lsdir = fs::read_dir(&new_path).unwrap();
    let file = String::new();
    for file in lsdir {
        //file type finder logic goes here
        let ind_file: String = file.unwrap().file_name().into_string().unwrap().to_owned(); //colorize ts
        println!("{}", ind_file); //aww
    }
}