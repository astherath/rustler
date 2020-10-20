pub mod printer {
    use ansi_term::Colour;
    use lines::lines::{CodeLine, CodePatch};

    fn print_separator() {
        let separator = {
            let mut _str = String::new();
            let sep_char = '-';
            for _ in 0..80 {
                _str.push(sep_char);
            }
            _str
        };
        println!("{}", separator);
    }

    pub struct ConsolePrinter {
        special_colour: Colour,
    }

    impl ConsolePrinter {
        pub fn new(special_colour: Colour) -> ConsolePrinter {
            ConsolePrinter { special_colour }
        }

        pub fn print(&self, code_patches: Vec<CodePatch>) {
            for patch in code_patches {
                print_separator();
                for line in patch.lines {
                    self.print_line(line);
                }
            }
            print_separator();
        }

        fn print_line(&self, line: CodeLine) {
            let printed_string = {
                if line.is_special {
                    self.special_colour.paint(&line.content).to_string()
                } else {
                    line.content
                }
            };
            println!("{}\t{}", &line.number, printed_string);
        }
    }
}
