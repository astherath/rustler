pub mod cli {
    extern crate clap;
    use clap::{App, Arg};

    pub struct CommandLineArgs {
        pub filename: String,
        pub config_filename: String,
        pub context: usize,
    }

    impl CommandLineArgs {
        pub fn new() -> CommandLineArgs {
            CommandLineArgs::get_args()
        }

        fn get_args() -> CommandLineArgs {
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
                )
                .arg(
                    Arg::with_name("context")
                        .help("Tells rustler how many files of surrounding context to return for special lines")
                        .required(true)
                )
                // .arg(
                // Arg::with_name("v")
                // .short("v")
                // .multiple(true)
                // .help("Sets the level of verbosity"),
                // )
                .get_matches();

            // get values from matches
            let filename = matches.value_of("filename").unwrap().to_string();
            // Gets a value for config if supplied by user, or defaults to "default.conf"
            let config_filename = matches
                .value_of("config")
                .unwrap_or("default.conf")
                .to_string();

            // parse context arg
            let context: usize = matches.value_of("context").unwrap_or("0").parse().unwrap();

            let response = CommandLineArgs {
                filename,
                config_filename,
                context,
            };
            response
        }
    }
}
