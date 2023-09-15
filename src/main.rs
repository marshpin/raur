extern crate clap;

pub mod pkg;
pub mod git;

use std::{
    env,
    collections::HashMap
};

//use clap::{Command, ArgGroup, arg};
//use pkg::{sync, remove, get_source};

fn get_args() -> Vec<String> {
    env::args().collect()
}

fn collect_args(raw_args: Vec<String>) -> HashMap<String, String> {
    let mut args: HashMap<String, String> = HashMap::new();
    // OPERATION : FIRST KEY //
    args.insert(raw_args[1], String::from("None"));

    args
}

fn main() {
    /*let raur = Command::new("raur")
        .args(&[
            // OPERATIONS //
            arg!(-S --sync <PACKAGE> "Syncs AUR repository to system."),
            arg!(-R --remove <PACKAGE> "Removes synced package from system."),
            arg!(-T --test <TEST> "Test command"),
            // OPERATION FLAGS //
            arg!(-n --noconfirm "Skips pacman [y/N] confirmation.")
        ])
        .groups(&[
            ArgGroup::multiple(ArgGroup::new("sync_args").args(["sync", "noconfirm"]), true),
            ArgGroup::multiple(ArgGroup::new("remove_args").args(["remove", "noconfirm"]), true),
            ArgGroup::multiple(ArgGroup::new("test_args").args(["test"]), true)
        ])
    .get_matches();



    if let Some(pkg_name) = raur.get_one::<String>("sync") {
        sync(pkg_name.to_string());
    } else if let Some(pkg_name) = raur.get_one::<String>("remove") {
        remove(pkg_name.to_string());
    } else if let Some(_pkg_name) = raur.get_one::<String>("test") {
        println!("{}", get_source(&String::from("PKGBUILD")));
    }*/

    let raw_args = get_args();

    dbg!(collect_args(raw_args));
}