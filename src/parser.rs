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
    // ///
    // ///
    // ///
    // ///
    pub fn read_file_data(filename: String) {
        let file_op = FileParser::new(&filename);
        // file_op.print_all_lines()
        let resp = file_op.unpack_lines_to_code_patch();
        let mut i = 1;
        for l in resp {
            println!("line #{} is: {}", i, l.line);
            i += 1;
        }
    }

    struct FileParser<'a> {
        filename: &'a str,
        lines: Vec<String>,
        current_line: u32,
    }

    impl<'a> FileParser<'a> {
        /// Instantiates FileParser for a given filename
        ///
        /// # Arguments
        ///
        /// * `filename` - A string with the filename of the file to be parsed
        ///
        /// # Notes
        ///
        /// The filename is NOT checked to exist at this point. However the file will be checked for completion on read so no double check needed.
        fn new(filename: &'a str) -> FileParser<'a> {
            let lines = FileParser::get_lines_from_file(filename);
            let response = FileParser {
                filename,
                lines,
                current_line: 1,
            };
            response
        }

        /// Returns a line iterator for the given filename
        ///
        /// # Arguments
        ///
        /// * `filename` - A string reference of the filename
        ///
        /// # Returns
        ///
        /// `io::Lines` iterator from the file
        fn get_lines_from_file(filename: &str) -> Vec<String> {
            let path = Path::new(filename);
            let file = match File::open(&path) {
                Err(why) => panic!(
                    "cannot open file \"{}\": {}",
                    filename,
                    Colour::Red.paint(why.to_string())
                ),
                Ok(file) => file,
            };
            let lines_iter = io::BufReader::new(file).lines();
            let mut lines = Vec::new();

            // unpack line iterator to vector
            for line_result in lines_iter {
                match line_result {
                    Err(why) => panic!("cannot read line {}", Colour::Red.paint(why.to_string())),
                    Ok(line) => lines.push(line),
                }
            }
            lines
        }

        /// Reads the file line-by-line into a context-driven `lines::CodePatch` struct
        fn unpack_lines_to_code_patch(&self) -> Vec<CodePatch> {
            let lines = &self.lines;
            let mut code_patch_vec = Vec::new();
            for line in lines {
                code_patch_vec.push(CodePatch::new(line.to_string()));
                // match line_result {
                // Err(why) => panic!("cannot read line {}", Colour::Red.paint(why.to_string())),
                // Ok(line) => code_patch_vec.push(CodePatch::new(line)),
                // }
            }
            code_patch_vec
        }

        fn check_line_todo(line: &String) -> bool {
            line.to_lowercase().contains("todo")
        }

        // fn inc_line_number(&mut self) {
        // self.current_line += 1
        // }

        // fn process_special_line(&mut self, line: &String) {
        // self.inc_special_line_count();
        // self.add_special_line();
        // }
    }
}
