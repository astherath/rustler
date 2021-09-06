use ansi_term::Colour;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Top-level function to mask the usage of the FileParser for a simple interface
pub fn read_file_data(filename: String) -> Vec<String> {
    let file_op = FileParser::new(filename);
    file_op.lines
}

struct FileParser {
    lines: Vec<String>,
}

impl FileParser {
    /// Instantiates FileParser for a given filename
    ///
    /// # Arguments
    ///
    /// * `filename` - A string with the filename of the file to be parsed
    ///
    /// # Notes
    ///
    /// The filename is NOT checked to exist at this point.
    /// However the file will be checked for completion on read so no double check needed.
    fn new(filename: String) -> FileParser {
        let lines = FileParser::get_lines_from_file(&filename);
        FileParser { lines }
    }

    /// Reads and returns an iterator with the line data for the given filename
    ///
    /// # Arguments
    ///
    /// * `filename` - A string reference of the filename
    ///
    /// # Returns
    ///
    /// `Vec<String>` of all of the lines in the file
    fn get_lines_from_file(filename: &String) -> Vec<String> {
        let path = Path::new(filename);
        let file = match File::open(&path) {
            Err(why) => panic!(
                "cannot open file \"{}\": {}",
                filename,
                Colour::Red.paint(why.to_string())
            ),
            Ok(file) => file,
        };

        // unpack line iterator into vector
        let lines_iter = io::BufReader::new(file).lines();
        let mut lines = Vec::new();

        for line_result in lines_iter {
            match line_result {
                Err(why) => panic!("cannot read line {}", Colour::Red.paint(why.to_string())),
                Ok(line) => {
                    if !line.is_empty() {
                        lines.push(line)
                    }
                }
            }
        }
        lines
    }
}
