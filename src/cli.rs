pub mod cli {
    extern crate clap;
    use clap::{App, Arg};

    pub fn get_matches() {
        let matches = App::new("rustler")
            .version("1.0")
            .about("Rustles files for TODO and FIXME comments")
            // .arg(
            // Arg::with_name("config")
            // .short("c")
            // .long("config")
            // .value_name("FILE")
            // .help("Sets a custom config file")
            // .takes_value(true),
            // )
            .arg(
                Arg::with_name("filename")
                    .help("Sets the input file to rustle")
                    .required(true)
                    .index(1),
            )
            // .arg(
            // Arg::with_name("v")
            // .short("v")
            // .multiple(true)
            // .help("Sets the level of verbosity"),
            // )
            .get_matches();

        // Gets a value for config if supplied by user, or defaults to "default.conf"
        let config = matches.value_of("config").unwrap_or("default.conf");
        println!("Value for config: {}", config);

        // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
        // required we could have used an 'if let' to conditionally get the value)
        println!(
            "Using input file: {}",
            matches.value_of("filename").unwrap()
        );
    }
}
