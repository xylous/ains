use std::process::{Command, Stdio};

use crate::Partition;

impl Partition {
    pub fn create(&self) -> Result<(), std::io::Error> {
        let echo_args = format!(
            // n: create new partition
            // use partition number specified in instance
            // next line: default first sector
            // use partition size specified in instance
            "n\n\
            {}\n\
            \n\
            {}\n",
            &self.number,
            &self.size,
        );

        let echo_cmd = Command::new("echo")
            .arg("-n")
            .args(echo_args.split("\n"))
            .stdout(Stdio::piped())
            .spawn()?;

        // now, run fdisk
        Command::new("fdisk")
            .arg(&self.disk)
            .stdin(echo_cmd.stdout.unwrap())
            .spawn()?;

        // and format the newly created partiton
        let mut mkfs_cmd: String = String::from("mkfs.");
        let mut mkfs_args = vec![self.disk_partition()];
        match self.format.as_str() {
            "ext4" | "ext3" | "ext2" => mkfs_cmd += &self.format,
            "fat32" | "fat" => { mkfs_cmd += "fat"; mkfs_args.insert(0, String::from("-F32")) }
            _ => ()
        }

        Command::new(mkfs_cmd)
            .args(mkfs_args)
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
