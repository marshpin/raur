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