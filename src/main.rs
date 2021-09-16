use clap::{App, Arg};

fn main() {
    let cli_args = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author("xylous, xylous.e@gmail.com")
        .about("Install Arch from a config file")
        .arg(Arg::new("FILE")
            .required(true)
            .about("input file"))
        .get_matches();

    let input_file = cli_args.value_of("INPUT").unwrap();
}
