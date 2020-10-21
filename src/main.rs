mod cli;
use cli::cli::CommandLineArgs;
use lines::lines::CodePatch;
mod printer;
use printer::printer::ConsolePrinter;
mod parser;
use ansi_term::{self, Colour};

fn setup_ansi_colors() {
    #[cfg(target_os = "windows")]
    ansi_term::enable_ansi_support();
}

fn main() {
    setup_ansi_colors();

    let cli_args = CommandLineArgs::new();
    let file_lines = parser::parser::read_file_data(cli_args.filename);
    let code_patch_vec = CodePatch::unpack_lines(file_lines, cli_args.context);
    let special_colour = Colour::Purple;
    let printer = ConsolePrinter::new(special_colour);
    printer.print_all_lines(code_patch_vec);
}
