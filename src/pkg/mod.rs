use std::process::Command as cmd;
use std::fs;
use std::process::Stdio;

use crate::git;

//filter package argument into a url if it isn't already formatted as one
pub fn filter_input(input: &String) -> String {
    //assumes that if the input does not start with an AUR repo link, the package name is bare
    let mut string = input.to_owned();
    if ! input.starts_with("https://aur.archlinux.org/") {
        if ! input.ends_with(".git") {
            string = format!("{}{}", &string, &".git");
        }
        string = format!("{}{}", &"https://aur.archlinux.org/", &string);
    }
    string
}

//execute makepkg -si in PKGBUILD directory
pub fn make_pkg(input: &String) {
    cmd::new("makepkg").current_dir(&input).arg("-i").output().expect("Failed to execute makepkg!");
}

pub fn install_pkg(input: &String) {
    cmd::new("sudo").current_dir(&input).arg("pacman").arg("-U").arg("*.tar.gz").status().expect("Failed to execute pacman!");
}

//cleanup build dir
pub fn cleanup(input: &String) {
    fs::remove_dir_all(&input).expect("Unable to cleanup repo!");
}

//remove package from system (-R)
pub fn remove(input: String) {
    cmd::new("sudo").arg("pacman").arg("-R").arg(&input).status().expect("Unable to execute sudo!");
}

//sync package to system (-S)
pub fn sync(input: String) {
    println!("Cloning repository...");
    match git::clone(&filter_input(&input)) {
        Ok(t) => println!("{}", t),
        Err(e) => panic!("{}", e)
    };
    println!("Making package...");
    make_pkg(&input);
    install_pkg(&input);
    cleanup(&input);
    println!("Cleaning up...")
}