use std::process::{Command, Stdio};

use crate::Partition;

impl Partition {
    pub fn create(&self) -> Result<(), std::io::Error> {
        let echo_args = format!(
            // n: create new partition
            // next line: default partition number
            // next line: default first sector
            // use partition size specified in struct
            "n\n\
            \n\
            \n\
            {}\n",
            &self.size,
        );

        let echo_cmd = Command::new("echo")
            .arg("-n")
            .args(echo_args.split("\n"))
            .stdout(Stdio::piped())
            .spawn()?;

        // now, run fdisk
        Command::new("fdisk")
            .arg(format!("{}", self.disk))
            .stdin(echo_cmd.stdout.unwrap())
            .spawn()?;
        Ok(())
    }

    pub fn disk_partition(&self) -> String {
        format!("{}p{}", self.disk, self.number)
    }

    pub fn mount(&self) -> Result<(), std::io::Error> {
        let mount_args = format!(
            "mount {} {}",
            self.mount,
            self.disk_partition()
        );
        exec_cmd(&mount_args.split_whitespace().collect())
    }
}

pub fn install(packages: Vec<&str>) -> Result<(), std::io::Error> {
    let cmd: Vec<&str> = vec![vec!["pacman", "-S"], packages].into_iter().flatten().collect();
    exec_cmd(&cmd)
}

fn exec_cmd(args: &Vec<&str>) -> Result<(), std::io::Error> {
    Command::new(args[0])
        .args(&args[1..])
        .spawn()?;

    Ok(())
}
