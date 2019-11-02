use clap::{App, Arg};

pub fn main() {
    let matches = App::new("MyApp")
        .arg(
            Arg::with_name("input")
                .help("the input file to use")
                .required(true) // by default this argument must be present
                // if you set to false, this is option arg
                .takes_value(true) // take a value for arg
                .short("i")
                .long("input"),
        )
        .get_matches();

    if matches.is_present("input") {
        println!("An input file was specified");
    }
}
