mod cli;
mod parser;

fn main() {
    cli::cli::get_args();
    let filename = String::from("example.txt");
    parser::parser::read_file_data(filename);
}
