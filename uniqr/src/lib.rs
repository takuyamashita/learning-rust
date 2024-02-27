use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    in_file: String,
    out_file: Option<String>,
    count: bool,
}

pub fn run(config: Config) -> MyResult<()> {
    println!("{:?}", config);
    Ok(())
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("uniqr")
        .version("0.0.1")
        .author("github.com/takuyamashita")
        .args(&[
            Arg::with_name("in_file")
                .value_name("IN_FILE")
                .default_value("-"),
            Arg::with_name("out_file").value_name("OUT_FILE"),
            Arg::with_name("count")
                .takes_value(false)
                .short("c")
                .long("count"),
        ])
        .get_matches();

    Ok(Config {
        in_file: matches.value_of_lossy("in_file").map(Into::into).unwrap(),
        out_file: matches.value_of_lossy("out_file").map(Into::into),
        count: matches.is_present("count"),
    })
}
