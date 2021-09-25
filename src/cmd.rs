use std::process::{Command, Stdio};

pub fn install(packages: Vec<&str>) -> Result<(), std::io::Error> {
    let cmd: Vec<&str> = vec![vec!["pacman", "-S"], packages].into_iter().flatten().collect();
    exec_cmd(&cmd)?;
    Ok(())
}

fn exec_cmd(args: &Vec<&str>) -> Result<(), std::io::Error> {
    Command::new(args[0])
        .args(&args[1..])
        .spawn()?;

    Ok(())
}
