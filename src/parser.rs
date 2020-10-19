pub mod parser {
    use ansi_term::{self, Colour};
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    fn setup_ansi_colors() {
        #[cfg(target_os = "windows")]
        ansi_term::enable_ansi_support();
    }

    pub fn read_file_data(filename: String) {
        let mut file_op = FileParser::new(filename);
        file_op.print_all_lines()
    }

    fn check_line_todo(line: &String) -> bool {
        line.to_lowercase().contains("todo")
    }

    fn get_lines_from_file(filename: &String) -> std::io::Lines<std::io::BufReader<std::fs::File>> {
        let path = Path::new(filename);
        let file = match File::open(&path) {
            Err(why) => panic!(
                "cannot open file \"{}\": {}",
                filename,
                Colour::Red.paint(why.to_string())
            ),
            Ok(file) => file,
        };
        let lines = io::BufReader::new(file).lines();
        lines
    }

    struct FileParser {
        filename: String,
        current_line: u32,
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
        /// The filename is NOT checked to exist at this point. However the file will be checked for completion on read so no double check needed.
        fn new(filename: String) -> FileParser {
            let response = FileParser {
                filename,
                current_line: 1,
            };
            response
        }

        fn inc_line_number(&mut self) {
            self.current_line += 1
        }

        fn process_special_line(&mut self, line: &String) {
            self.inc_special_line_count();
            self.add_special_line();
        }
    }
}
