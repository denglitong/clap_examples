pub mod a {
    use clap::{App, Arg};

    pub fn main() {
        let matches = App::new("myapp")
            .arg(
                Arg::with_name("seq")
                    .help("A sequence of whole positive numbers, i.e. 20 25 30")
                    .short("s")
                    .long("sequence")
                    .takes_value(true)
                    .multiple(true), // takes multiple value
            )
            .arg(
                Arg::with_name("len")
                    .short("l")
                    .long("len")
                    .help("A length to use, defaults to 10 when omitted")
                    .takes_value(true),
            )
            .get_matches();

        let len = value_t!(matches, "len", u32).unwrap_or(10);
        println!("len {}", len);

        let sequence = values_t!(matches, "seq", u32).unwrap_or_else(|e| e.exit());
        for v in sequence {
            println!("Sequence part: {}", v);
        }
    }
}
