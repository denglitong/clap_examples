pub mod a {
    use clap::{App, Arg};

    // create an app with 5 possible arguments (2 auto generated) and 2 subcommands (1 auto generated)
    // -c --config filename
    // -d --debug
    // -h --help
    // -V --version
    // subcommand
    //  test
    //      -l list
    //      -h --help
    //      -V --version
    //  help
    pub fn main() {
        let matches = App::new("MyApp")
            .version("1.0")
            .author("denglitong litongdeng@gmail.com")
            .about("My First Clap App")
            .arg(
                Arg::with_name("config")
                    .short("c")
                    .long("config")
                    .value_name("FILE")
                    .help("Sets a custom config file")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("output")
                    .short("o")
                    .long("output")
                    .help("Sets an optional output file")
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("debug")
                    .short("d")
                    .long("debug")
                    .help("Turn debugging information on")
                    .multiple(true),
            )
            .subcommand(
                App::new("test").about("does something things").arg(
                    Arg::with_name("list")
                        .short("l")
                        .long("list")
                        .help("lists test values"),
                ),
            )
            .get_matches();

        // check the value provided by positional arguments, or option arguments
        if let Some(o) = matches.value_of("output") {
            println!("Value for output: {}", o);
        }
        if let Some(c) = matches.value_of("config") {
            println!("Value for config: {}", c);
        }

        // see how many times a particular flag or argument occurred, only flags can have multiple occurrences
        match matches.occurrences_of("debug") {
            0 => println!("Debug mode is off"),
            1 => println!("Debug mode is kind of on"),
            2 => println!("Debug mode is on"),
            3 | _ => println!("Don't be crazy"),
        }

        if let Some(ref matches) = matches.subcommand_matches("test") {
            if matches.is_present("list") {
                println!("Printing testing lists...");
            } else {
                println!("Not printing testing lists...");
            }
        }
    }
}
