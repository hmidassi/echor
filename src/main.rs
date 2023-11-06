use clap::{Arg, Command};

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("Hedy Midassi")
        .about("Echo made with Rust")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .help("Do not print newline")
                .num_args(0),
        )
        .get_matches();

    let text: Vec<&str> = matches
        .get_many("text")
        .unwrap()
        .map(String::as_str)
        .collect();

    let omit_newline: bool = *matches.get_one("omit_newline").unwrap();

    let ending =if omit_newline {" "} else {"\n"};


    print!("{}{}", text.join(" "), ending);
}
