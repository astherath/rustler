mod cli;
mod parser;

fn main() {
    cli::cli::get_matches();
    // let filename = String::from("example.txt");
    // parser::parser::read_file_data(filename);
}
