use core::str;
use std::process::Command;

pub fn list_files() {
    let output = Command::new("sh")
        .arg("-c")
        .arg("ls")
        .output()
        .expect("failed to execute process");

    let hello = str::from_utf8(&output.stdout).unwrap();
}
