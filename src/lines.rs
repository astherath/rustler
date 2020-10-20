pub mod lines {
    use std::cmp;

    pub struct CodePatch {
        pub line: String,
    }

    impl CodePatch {
        pub fn new(line: String) -> CodePatch {
            CodePatch { line }
        }

        fn check_line_special(line: &String) -> bool {
            line.to_lowercase().contains("todo")
        }

        // fn print_all_todos(&self) {}

        /// Reads the file line-by-line into a context-driven `lines::CodePatch` struct
        pub fn unpack_lines_to_code_patch_vec(
            lines: Vec<String>,
            context: usize,
        ) -> Vec<CodePatch> {
            let mut code_patch_vec = Vec::new();
            let mut i = 0;
            let lines_len = lines.len();
            while i < lines_len {
                if CodePatch::check_line_special(&lines[i]) {
                    let back_range = i.checked_sub(context).unwrap_or(0);
                    let mut full_context = String::new();

                    // add context # of lines behind
                    for j in back_range..i {
                        full_context.push_str(&lines[j].trim());
                        full_context.push_str("\n");
                    }

                    // add current line
                    full_context.push_str(&lines[i].trim());
                    full_context.push_str("\n");

                    // add context # of line ahead
                    let front_range = cmp::min(lines_len, i + context) + 1;
                    for j in (i + 1)..front_range {
                        full_context.push_str(&lines[j].trim());
                        full_context.push_str("\n");
                    }
                    println!("{}------------------------------", &full_context);
                    // XXX: debug
                }
                i += 1;
            }
            code_patch_vec
        }

        pub fn print_if_special(&self) {
            if CodePatch::check_line_special(&self.line) {
                println!("{}", &self.line);
            }
        }
    }
}
