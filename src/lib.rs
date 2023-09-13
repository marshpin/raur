//define git commands as functions to use
pub mod git {
    use std::process::Command as cmd;

    pub fn clone(url: &String) -> Result<&str, &str> {
        let output = cmd::new("git").arg("clone").arg("--depth").arg("1").arg(&url).output().expect("Unable to clone git repository!");
        if ! output.status.success() {
            println!("{}", String::from_utf8_lossy(&output.stderr));
            Err("Failed to clone repository! Exiting...")
        } else {
            Ok("Cloned successfully!")
        }
    }
}

//package building utilities
pub mod pkg {
    use crate::git;
    use std::process::Command as cmd;
    use std::fs;

    //read the source=() field from the PKGBUILD file
    pub fn get_source(input: &String) -> String {
        let pkgbuild = fs::read_to_string(&input).expect("Unable to find or read PKGBUILD file!");

        let mut source: String = Default::default();
        let mut source_count = 0;

        for line in pkgbuild.lines() {

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
    // TODO: replace git clone system to use AUR search api instead of... whatever this is
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
        match git::clone(&filter_input(&input)) {
            Ok(t) => println!("{}", t),
            Err(e) => panic!("{}", e)
        };
        println!("Making package...");
        make_pkg(&input);
        cleanup(&input);
        println!("Cleaning up...")
    }
}

//TODO: add aur mod to search pkgs

pub mod aur {

}