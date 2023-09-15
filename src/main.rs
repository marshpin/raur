pub mod pkg;
pub mod git;

use std::{
    env,
    process
};

fn get_args() -> Vec<String> {
    env::args().collect()
}

fn process_args(raw_args: Vec<String>) {
    if raw_args[1] != String::from("-S") && raw_args[1] != String::from("-h") && raw_args[1] != String::from("--sync") && raw_args[1] != String::from("--help") {
        println!(r#"Invalid argument! Please enter "raur -h" for help."#);
        process::exit(1);
    } else if raw_args[1] == String::from("-S") || raw_args[1] == String::from("--sync") && raw_args.len() == 3 {
        pkg::sync(raw_args[2].to_owned());
    } else if raw_args[1] == String::from("-h") || raw_args[1] == String::from("--help") && raw_args.len() == 2 {
        println!(r#"usage: raur <OPERATION> <ARGUMENT>
  options:
    -S, --sync <PACKAGE> Syncs AUR package to system.
    -h, --help           Shows help text.
    "#);
    } else {    
        println!(r#"Not enough or too many arguments! Please enter "raur -h" for help."#);
        process::exit(1);
    }
}

fn main() {
    let raw_args = get_args();
    process_args(raw_args);
}