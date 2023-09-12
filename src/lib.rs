
//define git commands as functions to use
pub mod git {
    use std::process::Command as cmd;

    pub fn clone(url: &String){
        cmd::new("git").arg("clone").arg(url).output().expect("Unable to clone git repository!");
    }
}

//package building utilities
pub mod pkg {
    use crate::git;
    use std::process::Command as cmd;
    use std::fs;

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

    pub fn cleanup(input: &String) {
        fs::remove_dir_all(&input).expect("Unable to cleanup repo!");
    }

    pub fn sync(input: String) {
        clone_pkg(&filter_input(&input));
        make_pkg(&input);
        cleanup(&input);
    }
}