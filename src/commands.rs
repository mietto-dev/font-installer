use core::str;
use std::fmt::{Display, Formatter, Result};
use std::process::Command;

pub fn exec(cmd: &AllowedCommands, args: &str) -> String {
    let cmd_args = format!("{} {}", cmd.to_string(), args);

    let output = Command::new("sh")
        .arg("-c")
        .arg(cmd_args)
        .output()
        .expect("failed to execute process");

    let result = str::from_utf8(&output.stdout).unwrap();
    return result.to_string();
}

// TODO: properly model command execution, for better reusability (maybe on a separate lib / crate)
// Example usage: `let cmd = AllowedCommands::ChangeFileMod(READ_WRITE_GROUP, "./path/to/files")`

// TODO: add tuples or custom implementations for each command args
pub enum AllowedCommands {
    ChangeFileMod,
    ChangeOwnership,
    Copy,
    FCCache,
    List,
    Move,
    MakeDir,
    RestoreCon,
}

impl Display for AllowedCommands {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "{}",
            match self {
                Self::ChangeFileMod => "chmod",
                Self::ChangeOwnership => "chown",
                Self::Copy => "cp",
                Self::FCCache => "fc-cache",
                Self::List => "ls",
                Self::Move => "mv",
                Self::MakeDir => "mkdir",
                Self::RestoreCon => "restorecon",
            }
        )
    }
}
