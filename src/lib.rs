#![allow(dead_code, unused_imports)]

//define git commands as functions to use
pub mod git {
    use std::process::Command as cmd;

    pub fn clone(url: &String){
        cmd::new("git").arg("clone").arg(&url).output().expect("Unable to clone git repository!");
    }
}

//package building utilities
pub mod pkg {
    use crate::git;
    use std::process::Command as cmd;
    use std::path::PathBuf;
    use std::fs;

    //read the source=() field from the PKGBUILD file
    pub fn get_source(input: &String) -> String {
        let PKGBUILD = fs::read_to_string(&input).expect("Unable to find or read PKGBUILD file!");

        let mut source: String = Default::default();
        let mut source_count = 0;

        for line in PKGBUILD.lines() {

            if line.starts_with("source=") {
                if ! line.ends_with(")") {
                    source.push_str(&line);
                    source_count += 1;
                }
            } else if source_count > 0 {
                if line.ends_with(")") {
                    source_count = 0
                }
                source.push_str(&line);
            }

        }

        source
    }

    //filter package argument into a url if it isn't already formatted as one
    pub fn filter_input(input: &String) -> String {
        //assumes that if the input does not start with an AUR repo link, the package name is bare
        //filter will be updated later to be better
        if ! input.starts_with("https://aur.archlinux.org/") {
            format!("{}{}", &"https://aur.archlinux.org/", &input)
        } else {
            input.to_string()
        }
    }

    //git clone package to system
    pub fn clone_pkg(input: &String) {
        git::clone(input);
    }

    //execute makepkg -si in PKGBUILD directory
    pub fn make_pkg(input: &String) {
        cmd::new("makepkg").current_dir(&input).arg("-si").status().expect("Unable to execute makepkg!");
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
        clone_pkg(&filter_input(&input));
        println!("Repository successfully cloned!");
        println!("Making package...");
        make_pkg(&input);
        cleanup(&input);
        println!("Cleaning up...")
    }
}

//TODO: add aur mod to search pkgs

pub mod aur {

}