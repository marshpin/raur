extern crate clap;

use clap::{Command, ArgGroup, arg};
use raur::pkg::sync;

fn main() {
    let raur = Command::new("raur")
        .args(&[
            arg!(-s --sync <PACKAGE> "Syncs AUR repository to system."),
            arg!(-r --remove <PACKAGE> "Removes synced package from system.")
        ])
        .groups(&[
            ArgGroup::multiple(ArgGroup::new("syncargs").args(["sync"]), true),
            ArgGroup::multiple(ArgGroup::new("removeargs").args(["remove"]), true)
        ])
    .get_matches();

    if let Some(pkg_name) = raur.get_one::<String>("sync") {
        sync(pkg_name.to_string());
    }
}
