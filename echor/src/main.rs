use clap::{App, Arg};

fn main() {
    let matches = App::new("MyApp")
        .version("1.0")
        .author("Me")
        .about("Does awesome things")
        .arg(
            Arg::with_name("text")
                .help("The text to print")
                .required(true)
                .index(1)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .help("Don't print a newline at the end")
                .short("n")
                .long("no-newline")
                .takes_value(false),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();
    let omit_newline = matches.is_present("omit_newline");
    let ending = if omit_newline { "" } else { "\n" };

    print!("{}{}", text.join(" "), ending);
}
