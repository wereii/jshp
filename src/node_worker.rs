use std::error::Error;
use std::{fs, io};
use mktemp::Temp;
use std::process::Command;

// todo: maybe also "stream" this into node:vm
pub fn node_check(code: &str) -> io::Result<bool> {
    let file = Temp::new_file()?;
    fs::write(file.as_path(), code)?;
    Ok(Command::new("node").arg("--check").arg(file.as_path()).output()?.status.success())
}
