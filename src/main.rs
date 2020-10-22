mod cli;
use lines::lines::CodePatch;
mod parser;
mod printer;
mod writers;
use ansi_term::{self, Colour};
use writers::file_writer;

fn setup_ansi_colors() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support();
}

fn main() {
    // windows only setup
    setup_ansi_colors();

    // the `cli_args` struct returned here has all of the pre-validated
    // CLI args, opts, inputs, etc.
    let cli_args = cli::cli::CommandLineArgs::new();

    // `file_lines` a vec of all of the non-empty lines (Strings) in the file
    let file_lines = parser::parser::read_file_data(cli_args.filename);

    // from `file_lines` we make the vec of context-aware "code patches" here
    let code_patch_vec = CodePatch::unpack_lines(file_lines, cli_args.context);

    // creating printer and consuming it to display terminal output
    let special_colour = Colour::Purple;
    let printer = printer::printer::ConsolePrinter::new(special_colour);
    printer.print_all_lines(&code_patch_vec, cli_args.display_type);

    // file output operations handled by the writer methods IF the output flag is set and valid
    if cli_args.output_file_flag {
        let output_filename = cli_args.output_filename.unwrap();
        file_writer::export_to_markdown(code_patch_vec, &output_filename);
    }
}
