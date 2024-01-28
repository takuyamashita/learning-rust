use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr").version("0.1.0").args(&[
        Arg::with_name("files")
            .value_name("FILE")
            .multiple(true)
            .default_value("-")
            .help("Files to read"),
        Arg::with_name("number_lines")
            .short("n")
            .long("number")
            .takes_value(false)
            .help("Number lines"),
        Arg::with_name("number_nonblank_lines")
            .short("b")
            .long("number-nonblank")
            .takes_value(false)
            .conflicts_with("number_lines")
            .help("Number non-blank lines"),
    ]);

    let matches = matches.get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number_lines"),
        number_nonblank_lines: matches.is_present("number_nonblank_lines"),
    })
}

pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        match open(&filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                let mut non_blank_line_num = 0;

                for (line_num, line) in file.lines().enumerate() {
                    let line = line?;

                    if config.number_lines {
                        println!("{:>6}\t{}", line_num + 1, line);
                    } else if config.number_nonblank_lines {
                        if line.is_empty() {
                            println!("{}", line);
                        } else {
                            non_blank_line_num += 1;
                            println!("{:>6}\t{}", non_blank_line_num, line);
                        }
                    } else {
                        println!("{}", line);
                    }
                }
            }
        }
    }
    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
