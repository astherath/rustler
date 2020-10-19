pub mod parser {
    use ansi_term::{self, Colour};
    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    pub fn read_file_data(filename: String) {
        let mut file_op = FileOperatorUtil::new(filename);
        file_op.print_all_lines()
    }

    fn setup_ansi_colors() {
        #[cfg(target_os = "windows")]
        ansi_term::enable_ansi_support();
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

    struct FileOperatorUtil {
        filename: String,
        current_line: u32,
        special_line_count: u16,
        special_line_numbers: Vec<u32>,
    }

    impl FileOperatorUtil {
        fn new(filename: String) -> FileOperatorUtil {
            let response = FileOperatorUtil {
                filename,
                current_line: 1,
                special_line_count: 0,
                special_line_numbers: Vec::new(),
            };
            response
        }

        fn inc_line_number(&mut self) {
            self.current_line += 1
        }

        fn inc_special_line_count(&mut self) {
            self.special_line_count += 1
        }

        fn add_special_line(&mut self) {
            self.special_line_numbers.push(self.current_line);
        }

        fn process_special_line(&mut self, line: &String) {
            self.inc_special_line_count();
            self.add_special_line();
        }
    }
}
