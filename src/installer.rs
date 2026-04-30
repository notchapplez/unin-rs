use colored::Colorize;
use std::process::Command;

pub fn setup_files_full() {
    let pm = detect_package_manager();

    if pm == "UNKNOWN" {
        println!("Unknown package manager: {}", pm);
        return;
    }

    let packages = get_full_packages(pm);

    println!("Installing all langs for {}...", pm.yellow());
    for package in packages {
        println!("Installing {}...", package.cyan());
        install_package(pm, package);
    }

    println!("{}", "All packages installed successfully!".green());
}

pub fn setup_files_lang(lang: String) {
    let pm = detect_package_manager();

    if pm == "UNKNOWN" {
        println!("Unknown package manager: {}", pm);
        return;
    }

    let package = match lang.as_str() {
        "rust" => "rustup",
        "cmake" => "cmake",
        "go" => get_go_package(pm),
        "zig" => "zig",
        "swift" => get_swift_package(pm),
        "haskell" => "ghc",
        "d" => "dmd",
        _ => {
            println!("Language not supported yet");
            return;
        }
    };

    install_package(pm, package);
}

pub fn get_swift_package(pm: &str) -> &'static str {
    match pm {
        "pacman" => "swift-bin",
        _ => "swift",
    }
}
pub fn detect_package_manager() -> &'static str {
    let candidates = ["apt", "dnf", "yum", "zypper", "pacman", "apk"];

    for pm in candidates {
        if Command::new(pm).arg("--version").output().is_ok() {
            return pm;
        }
    }

    "UNKNOWN"
}

pub fn get_full_packages(pm: &str) -> Vec<&'static str> {
    match pm {
        "apt" => vec![
            "rustup",
            "golang-go",
            "zig",
            "swift",
            "ghc",
            "dmd",
            "cmake",
            "make",
            "gcc",
            "git",
            "pkg-config",
        ],
        "dnf" | "yum" => vec![
            "rustup",
            "golang",
            "zig",
            "swift",
            "ghc",
            "dmd",
            "cmake",
            "make",
            "gcc",
            "git",
            "pkgconfig",
        ],
        "zypper" => vec![
            "rustup",
            "go",
            "zig",
            "swift-bin",
            "ghc",
            "dmd",
            "cmake",
            "make",
            "gcc",
            "git",
            "pkg-config",
        ],
        "pacman" => vec![
            "rustup",
            "go",
            "zig",
            "swift-bin",
            "ghc",
            "dmd",
            "cmake",
            "make",
            "gcc",
            "git",
            "pkg-config",
        ],
        "apk" => vec![
            "rustup",
            "go",
            "zig",
            "swift",
            "ghc",
            "dmd",
            "cmake",
            "make",
            "gcc",
            "git",
            "pkgconfig",
        ],
        _ => {
            println!("Unknown package manager: {}", pm);
            vec![]
        }
    }
}

pub fn get_go_package(pm: &str) -> &'static str {
    match pm {
        "apt" => "golang-go",
        _ => "go",
    }
}

pub fn install_package(pm: &str, package: &str) {
    let (cmd, args): (&str, Vec<&str>) = match pm {
        "apt" => ("sudo", vec!["apt", "install", "-y", package]),
        "dnf" => ("sudo", vec!["dnf", "install", "-y", package]),
        "yum" => ("sudo", vec!["yum", "install", "-y", package]),
        "zypper" => ("sudo", vec!["zypper", "install", "-y", package]),
        "pacman" => ("sudo", vec!["yay", "-S", "--noconfirm", package]),
        "apk" => ("sudo", vec!["apk", "add", package]),
        _ => {
            println!("Unknown package manager: {}", pm);
            return;
        }
    };

    Command::new(cmd).args(args).status().ok();
}
