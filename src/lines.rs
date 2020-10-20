pub mod lines {
    use std::cmp;
    use std::convert::TryInto;

    pub struct CodeLine {
        pub number: u32,
        pub content: String,
        pub is_special: bool,
    }

    impl CodeLine {
        fn new(content: String, number: u32, is_special: bool) -> CodeLine {
            CodeLine {
                number,
                content,
                is_special,
            }
        }
    }

    pub struct CodePatch {
        pub lines: Vec<CodeLine>,
    }

    impl CodePatch {
        pub fn new(lines: Vec<CodeLine>) -> CodePatch {
            CodePatch { lines }
        }

        fn check_line_special(line: &String) -> bool {
            line.to_lowercase().contains("todo")
        }

        /// Reads the file line-by-line into a context-driven `lines::CodePatch` struct
        ///
        /// # Arguments
        ///
        /// * `lines` - A String vector with all of the non-empty lines from the file
        ///
        /// * `context` - The amount of context lines surrounding the special lines
        ///
        /// # Returns
        ///
        /// * `Vec<CodePatch>` - A vec containing all of the context-aware special lines
        ///
        /// # Notes
        ///
        /// If `context` passed in is > `lines.len()` then it will count context until EOF.
        pub fn unpack_lines(lines: Vec<String>, context: usize) -> Vec<CodePatch> {
            // code_patch vector to be returned
            let mut code_patch_vec = Vec::new();

            // loop vars
            let lines_len = lines.len();
            let mut i = 0;

            while i < lines_len {
                if CodePatch::check_line_special(&lines[i]) {
                    // re-cyclable vec to hold the CodeLines for a single CodePatch
                    let mut code_lines_vec = Vec::new();

                    // add context # of lines behind
                    let back_range = i.checked_sub(context).unwrap_or(0);
                    for j in back_range..i {
                        code_lines_vec.push(CodeLine::new(
                            lines[j].trim().to_string(),
                            j.try_into().unwrap(),
                            false,
                        ));
                    }

                    // add current line
                    code_lines_vec.push(CodeLine::new(
                        lines[i].trim().to_string(),
                        i.try_into().unwrap(),
                        true,
                    ));

                    // add context # of line ahead
                    let front_range = cmp::min(lines_len, i + context + 1);
                    for j in (i + 1)..front_range {
                        code_lines_vec.push(CodeLine::new(
                            lines[j].trim().to_string(),
                            j.try_into().unwrap(),
                            false,
                        ));
                    }

                    code_patch_vec.push(CodePatch::new(code_lines_vec));
                }
                i += 1;
            }
            code_patch_vec
        }
    }
}
