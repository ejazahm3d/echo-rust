use clap::{App, Arg};

fn main() {
    let matches = App::new("Echor")
        .version("0.1.0")
        .author("Ejaz Ahmed")
        .about("Rust Echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input Text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .help("Do not print newline")
                .takes_value(false)
                .short("n"),
        )
        .get_matches();

    let text = matches.values_of_lossy("text").unwrap();

    match matches.is_present("omit_newline") {
        true => print!("{}", text.join(" ")),
        false => println!("{}", text.join(" ")),
    };
}
