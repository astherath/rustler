mod cli;
use cli::cli::CommandLineArgs;
mod parser;

fn main() {
    let cli_args = CommandLineArgs::new();
    parser::parser::read_file_data(cli_args.filename);
}
