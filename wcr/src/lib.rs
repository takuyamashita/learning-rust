use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    files: Vec<String>,
    lines: bool,
    words: bool,
    bytes: bool,
    chars: bool,
}

#[derive(Debug, PartialEq)]
pub struct FileInfo {
    num_lines: usize,
    num_words: usize,
    num_bytes: usize,
    num_chars: usize,
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("wcr")
        .version("0.1.0")
        .author("https://github.com/takuyamashita")
        .about("wc")
        .args(&[
            Arg::with_name("files")
                .value_name("FILE")
                .multiple(true)
                .default_value("-")
                .help("Input file(s)"),
            Arg::with_name("lines")
                .value_name("LINE")
                .short("l")
                .long("lines")
                .takes_value(false)
                .help("Show line count"),
            Arg::with_name("words")
                .value_name("WORD")
                .short("w")
                .long("words")
                .takes_value(false)
                .help("Show word count"),
            Arg::with_name("bytes")
                .value_name("BYTE")
                .short("c")
                .long("bytes")
                .takes_value(false)
                .help("Show byte count"),
            Arg::with_name("chars")
                .value_name("CHAR")
                .short("m")
                .long("chars")
                .takes_value(false)
                .conflicts_with("bytes")
                .help("Show line count"),
        ])
        .get_matches();

    let mut lines = matches.is_present("lines");
    let mut words = matches.is_present("words");
    let mut bytes = matches.is_present("bytes");
    let chars = matches.is_present("chars");

    if [lines, words, bytes, chars].iter().all(|v| v == &false) {
        lines = true;
        words = true;
        bytes = true;
    }

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        lines,
        words,
        bytes,
        chars,
    })
}

pub fn run(config: Config) -> MyResult<()> {
    let mut total_lines = 0;
    let mut total_words = 0;
    let mut total_bytes = 0;
    let mut total_chars = 0;

    for filename in &config.files {
        match open(filename) {
            Err(err) => eprintln!("{}: {}", filename, err),
            Ok(file) => {
                if let Ok(file) = count(file) {
                    println!(
                        "{}{}{}{}{}",
                        format_field(file.num_lines, config.lines),
                        format_field(file.num_words, config.words),
                        format_field(file.num_bytes, config.bytes),
                        format_field(file.num_chars, config.chars),
                        if filename == &"-".to_string() {
                            "".to_string()
                        } else {
                            format!(" {}", filename)
                        }
                    );

                    total_lines += file.num_lines;
                    total_words += file.num_words;
                    total_bytes += file.num_bytes;
                    total_chars += file.num_chars;
                }
            }
        }
    }

    if config.files.len() > 1 {
        println!(
            "{}{}{}{} total",
            format_field(total_lines, config.lines),
            format_field(total_words, config.words),
            format_field(total_bytes, config.bytes),
            format_field(total_chars, config.chars),
        )
    }

    Ok(())
}

fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}

pub fn count(mut file: impl BufRead) -> MyResult<FileInfo> {
    let mut num_lines = 0;
    let mut num_words = 0;
    let mut num_bytes = 0;
    let mut num_chars = 0;

    let mut line = String::new();

    loop {
        let read_bytes = file.read_line(&mut line)?;

        if read_bytes == 0 {
            break;
        }

        num_lines += 1;
        num_words += line.split_whitespace().count();
        num_bytes += read_bytes;
        num_chars += line.chars().count();

        line.clear();
    }

    Ok(FileInfo {
        num_lines,
        num_words,
        num_bytes,
        num_chars,
    })
}

fn format_field(value: usize, show: bool) -> String {
    if show {
        format!("{:>8}", value)
    } else {
        "".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::{count, format_field, FileInfo};
    use std::io::Cursor;

    #[test]
    fn test_count() {
        let text = "I don't want the world. I just want your half.\r\n";
        let info = count(Cursor::new(text));
        assert!(info.is_ok());
        let expected = FileInfo {
            num_lines: 1,
            num_words: 10,
            num_chars: 48,
            num_bytes: 48,
        };

        assert_eq!(info.unwrap(), expected);
    }

    #[test]
    fn test_format_filed() {
        assert_eq!(format_field(1, false), "");
        assert_eq!(format_field(3, true), "       3");
        assert_eq!(format_field(10, true), "      10");
    }
}
