use std::io::prelude::*;
use std::process::{Command, Stdio};

fn exec_cmd(cmd: &str) -> Result<(), std::io::Error> {
    let args: Vec<&str> = cmd.split_whitespace().collect();

    Command::new(&args[0])
        .args(&args[1..])
        .spawn()?;

    Ok(())
}
