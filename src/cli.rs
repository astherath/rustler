pub mod cli {
    extern crate ansi_term;
    extern crate clap;
    use ansi_term::Colour;
    use clap::{App, Arg};
    use lines::lines::CodePatchType;
    use std::{self, path::Path};

    pub struct CommandLineArgs {
        pub filename: String,
        pub context: usize,
        pub display_type: CodePatchType,
        pub output_filename: Option<String>,
        pub output_file_flag: bool,
    }

    fn make_error_msg(message: &str, usage: &str) -> String {
        let red_error_str = Colour::Red.paint("ERROR".to_string()).to_string();
        let green_usage_str = Colour::White.paint(usage);

        let error_msg = format!("{}: {}\n{}", red_error_str, message, green_usage_str);
        error_msg
    }

    impl CommandLineArgs {
        pub fn new() -> CommandLineArgs {
            CommandLineArgs::get_args()
        }

        fn get_args() -> CommandLineArgs {
            let matches = App::new("rustler")
                .version("1.0")
                .about("Rustles files for TODO and FIXME comments")
                // Required filename arg that eventually gets checked for existence
                .arg(
                    Arg::with_name("filename")
                    .help("Sets the input file to rustle")
                    .required(true)
                )
                // Sets context lines opt
                .arg(
                    Arg::with_name("context")
                    .help("Tells rustler how many files of surrounding context to return for special lines")
                    .required(true)
                )
                // Sets the wanted type of display returned
                .arg(
                    Arg::with_name("type")
                    .help("Selects what type of special lines get displayed [default: all]")
                    .required(false)
                    .possible_values(&["todo", "fixme", "note", "xxx", "all"])
                )
                // Handles setting the output filename (if one given)
                .arg(
                    Arg::with_name("out")
                    .help("If set, a Markdown version of the special lines will be written to this file")
                    .required(false)
                    .long("output-filename")
                    .require_equals(true)
                    .takes_value(true)
                )
                .get_matches();

            // check that filename is valid and exists
            let filename = matches.value_of("filename").unwrap().to_string();

            let exists = Path::new(&filename).exists();
            if !exists {
                let usage = matches.usage.expect("Usage not generated by CLI app!");
                let error_msg = make_error_msg("File not found!", &usage);
                let err = clap::Error {
                    kind: clap::ErrorKind::InvalidValue,
                    message: error_msg,
                    info: None,
                };
                err.exit();
            }

            // gets the type of the display wanted
            let display_type_arg = matches.value_of("type").unwrap_or("all").to_string();
            let display_type = CodePatchType::get_display_type(&display_type_arg);

            // sets output filename if one given
            let output_file_flag;
            let output_filename = {
                match matches.value_of("out") {
                    Some(file_str) => {
                        output_file_flag = true;
                        Some(file_str.to_string())
                    }
                    None => {
                        output_file_flag = false;
                        None
                    }
                }
            };

            // context needs to be unwrapped from the cli then atoi'd into a usize
            let context: usize = matches.value_of("context").unwrap_or("0").parse().unwrap();
            CommandLineArgs {
                filename,
                context,
                display_type,
                output_filename,
                output_file_flag,
            }
        }
    }
}
