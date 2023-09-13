extern crate clap;

pub mod pkg;
pub mod git;

use clap::{Command, ArgGroup, arg};
use pkg::{sync, remove, get_source};

fn main() {
    let raur = Command::new("raur")
        .args(&[
            arg!(-S --sync <PACKAGE> "Syncs AUR repository to system."),
            arg!(-R --remove <PACKAGE> "Removes synced package from system."),
            arg!(-T --test <TEST> "Test command")
        ])
        .groups(&[
            ArgGroup::multiple(ArgGroup::new("sync_args").args(["sync"]), true),
            ArgGroup::multiple(ArgGroup::new("remove_args").args(["remove"]), true),
            ArgGroup::multiple(ArgGroup::new("test_args").args(["test"]), true)
        ])
    .get_matches();

    if let Some(pkg_name) = raur.get_one::<String>("sync") {
        sync(pkg_name.to_string());
    } else if let Some(pkg_name) = raur.get_one::<String>("remove") {
        remove(pkg_name.to_string());
    } else if let Some(_pkg_name) = raur.get_one::<String>("test") {
        println!("{}", get_source(&String::from("PKGBUILD")));
    }
}
