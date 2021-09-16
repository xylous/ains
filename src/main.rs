use clap::{App, Arg};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct InstallOptions {
    username: Option<String>,
    hostname: Option<String>,
    #[serde(default = "default_editor")]
    editor: String,
    #[serde(default = "default_shell")]
    shell: String,
    #[serde(default = "default_bootloader")]
    bootloader: String,
    #[serde(default = "default_partitions")]
    partitions: Vec<Partition>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Partition {
    #[serde(default = "default_partition_format")]
    format: String,
    #[serde(default = "default_partition_disk")]
    disk: String,
    #[serde(default = "default_partition_size")]
    size: String,
    #[serde(default = "default_partition_mount")]
    mount: String,
}

impl Partition {
    fn new(format: String, disk: String, size: String, mount: String) -> Partition {
        Partition {
            format,
            disk,
            size,
            mount,
        }
    }
}

fn default_editor() -> String { "vim".to_string() }
fn default_shell() -> String { "bash".to_string() }
fn default_bootloader() -> String { "grub".to_string() }
fn default_partition_format() -> String { "ext4".to_string() }
fn default_partition_disk() -> String { "/dev/nvme0n1".to_string() }
fn default_partition_size() -> String { "".to_string() }
fn default_partition_mount() -> String { "".to_string() }

fn default_partitions() -> Vec<Partition> {
    vec![
        Partition::new(default_partition_format(),
                        default_partition_disk(),
                        default_partition_size(),
                        default_partition_mount()),
    ]
}

fn file_exists(path: &str) -> bool {
    std::path::Path::new(path).exists()
}

fn file_to_string(path: &str) -> String
{
    std::fs::read_to_string(path).expect("failed to read file")
}

fn main() {
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("xylous, xylous.e@gmail.com")
        .about("Install Arch from a config file")
        .arg(Arg::new("FILE")
            .required(true)
            .about("input file"))
        .get_matches();

    let input_file = cli_args.value_of("FILE").unwrap();
    if ! file_exists(input_file) {
        eprintln!("error: file does not exist");
        return;
    }

    let contents = file_to_string(input_file);
    let _iopt: InstallOptions = serde_yaml::from_str(&contents).unwrap();
}
