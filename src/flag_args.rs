use clap::{App, Arg};

pub fn main() {
    let matches = App::new("MyApp")
        .arg(
            Arg::with_name("config")
                .takes_value(true)
                .short("c")
                .long("config"),
        )
        .arg(
            Arg::with_name("output")
                .takes_value(true)
                .short("o")
                .long("output"),
        )
        .arg(
            Arg::with_name("awesome")
                .help("turns up the awesome")
                .short("a")
                .long("awesome")
                .multiple(true)
                .requires("config") // says, if the user uses -a, they
                // MUST also use 'config' arg too
                .conflicts_with("output"), // opposite of requires(), if user use -a, they CANNOT
                                           // use 'output' arg
        )
        .get_matches();

    if matches.is_present("awesome") {
        println!("Awesome is turned on");
    }

    match matches.occurrences_of("awesome") {
        0 => println!("Nothing is awesome"),
        1 => println!("Some things are awesome"),
        2 => println!("Lots of things are awesome"),
        _ => println!("EVERYTHING is awesome!"),
    }
}
