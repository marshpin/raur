extern crate clap;

use clap::{Command, ArgGroup, arg};
use raur::pkg::{sync, remove};

fn main() {
    let raur = Command::new("raur")
        .args(&[
            arg!(-S --sync <PACKAGE> "Syncs AUR repository to system."),
            arg!(-R --remove <PACKAGE> "Removes synced package from system.")
        ])
        .groups(&[
            ArgGroup::multiple(ArgGroup::new("sync_args").args(["sync"]), true),
            ArgGroup::multiple(ArgGroup::new("remove_args").args(["remove"]), true)
        ])
    .get_matches();

    if let Some(pkg_name) = raur.get_one::<String>("sync") {
        sync(pkg_name.to_string());
    } else if let Some(pkg_name) = raur.get_one::<String>("remove") {
        remove(pkg_name.to_string());
    }
}
