pub mod parser {
    use ansi_term::{self, Colour};
    use lines::lines::CodePatch;
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    // TODO: fix this to be called eventually somewhere
    fn _setup_ansi_colors() {
        #[cfg(target_os = "windows")]
        ansi_term::enable_ansi_support();
    }

    // /// Top-level function to mask the usage of the FileParser for a simple interface. Returns
    pub fn read_file_data(filename: String) {
        let file_op = FileParser::new(filename);
        let code_patch_vec = CodePatch::unpack_lines_to_code_patch_vec(file_op.lines, 2);

        for code_patch in code_patch_vec {
            code_patch.print_if_special();
        }
    }

    struct FileParser {
        filename: String,
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
            FileParser { filename, lines }
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
}
