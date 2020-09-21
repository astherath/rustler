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

        fn print_special_line(&self, line: &String) {
            println!("{}\t{}", self.current_line, Colour::Purple.paint(line))
        }

        fn add_special_line(&mut self) {
            self.special_line_numbers.push(self.current_line);
        }

        fn process_special_line(&mut self, line: &String) {
            self.print_special_line(line);
            self.inc_special_line_count();
            self.add_special_line();
        }

        fn print_all_lines(&mut self) {
            let lines = get_lines_from_file(&self.filename);

            for line in lines {
                if let Ok(txt) = line {
                    if check_line_todo(&txt) {
                        self.process_special_line(&txt)
                    } else {
                        println!("{}\t{}", self.current_line, txt);
                    }
                    self.inc_line_number();
                }
            }

            // print post-process file data
            self.print_file_data()
        }

        fn line_data_to_string(&self) -> String {
            let mut line_str = String::from("[");
            for num in &self.special_line_numbers {
                line_str.push_str(&num.to_string());
                line_str.push_str(", ");
            }
            line_str.push_str("]");
            line_str
        }

        fn print_file_data(&self) {
            println!(
                "\"{}\" had {} TODO's at lines: {}",
                self.filename,
                self.special_line_count,
                self.line_data_to_string()
            );
        }
    }
}
