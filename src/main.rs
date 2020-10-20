mod cli;
use cli::cli::CommandLineArgs;
use lines::lines::CodePatch;
mod parser;

fn main() {
    let cli_args = CommandLineArgs::new();
    let file_lines = parser::parser::read_file_data(cli_args.filename);
    let _code_patch_vec = CodePatch::unpack_lines(file_lines, cli_args.context);
}
