mod cli;
mod common_structs;
mod file_io;
mod markdown;
mod output_formatter;
mod parser;
mod printer;

use ansi_term::{self, Colour};
use std::ffi::OsStr;
use std::path::Path;

use common_structs::MarkedSection;
use printer::ConsolePrinter;

fn setup_ansi_colors() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support();
}

fn main() {
    // windows only setup
    setup_ansi_colors();

    // the `cli_args` struct returned here has all of the pre-validated
    // CLI args, opts, inputs, etc.
    let cli_args = cli::CommandLineArgs::new();

    // `file_lines` a vec of all of the non-empty lines (Strings) in the file
    let file_lines = parser::read_file_data(&cli_args.input_path);

    // from `file_lines` we make the vec of context-aware "code patches" here
    let code_patch = MarkedSection::unpack_lines(file_lines, cli_args.context);

    // creating printer and consuming it to display terminal output
    let printer = ConsolePrinter::new(Colour::Purple);
    printer.print_all_lines(&code_patch, cli_args.display_type);

    // output to markdown if export flag is set
    if cli_args.markdown_output_flag {
        let file_extension = Path::new(&cli_args.input_path)
            .extension()
            .and_then(OsStr::to_str);
        let output_filename = cli_args.output_filename.unwrap();
        if let Err(error) = file_io::export_marked_sections_to_markdown_file(
            code_patch,
            file_extension,
            &output_filename,
        ) {
            clap::Error::with_description(
                &format!("Error exporting markdown to file: {}", error),
                clap::ErrorKind::Io,
            )
            .exit()
        }
    }
}
