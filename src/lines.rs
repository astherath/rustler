pub mod lines {
    use std::cmp;
    use std::convert::TryInto;

    #[derive(PartialEq)]
    pub enum CodePatchType {
        Todo,
        Fixme,
        Note,
        XXX,
        Other,
    }

    impl CodePatchType {
        /// Factory method in-place of a constructor for creating CodePatchTypes
        ///
        /// # Arguments
        ///
        /// * `line_str` - A borrowed String with the line contents
        ///
        /// # Returns
        ///
        /// A CodePatchType instance
        ///
        /// # Note
        ///
        /// This function should only be called if the line has been proven to be "special" already.
        /// If not, it will silently return a `CodePatchType::Other` which may have unintended effects.
        fn get_special_line_type(line_str: &String) -> CodePatchType {
            let lower_line = line_str.to_lowercase();
            if lower_line.contains("todo") {
                CodePatchType::Todo
            } else if lower_line.contains("fixme") {
                CodePatchType::Fixme
            } else if lower_line.contains("note") {
                CodePatchType::Note
            } else if lower_line.contains("xxx") {
                CodePatchType::XXX
            } else {
                CodePatchType::Other
            }
        }

        /// Factory method in-place of a constructor for creating CodePatchType instances for display args
        ///
        /// # Arguments
        ///
        /// * `type_opt` - A borrowed String with the CLI arg passed in for wanted type
        ///
        /// # Returns
        ///
        /// A CodePatchType instance
        ///
        /// # Note
        ///
        /// If no match is found, will return `CodePatchType::All`.
        pub fn get_display_type(type_opt: &String) -> CodePatchType {
            if type_opt == "todo" {
                CodePatchType::Todo
            } else if type_opt == "fixme" {
                CodePatchType::Fixme
            } else if type_opt == "note" {
                CodePatchType::Note
            } else if type_opt == "xxx" {
                CodePatchType::XXX
            } else {
                CodePatchType::Other
            }
        }

        fn check_line_special(line: &String) -> bool {
            CodePatchType::get_special_line_type(line) != CodePatchType::Other
        }
    }

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
        pub patch_type: CodePatchType,
    }

    impl CodePatch {
        pub fn new(lines: Vec<CodeLine>, patch_type: CodePatchType) -> CodePatch {
            CodePatch { lines, patch_type }
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
                if CodePatchType::check_line_special(&lines[i]) {
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

                    // get actual type of special line
                    let patch_type = CodePatchType::get_special_line_type(&lines[i]);

                    code_patch_vec.push(CodePatch::new(code_lines_vec, patch_type));
                }
                i += 1;
            }
            code_patch_vec
        }
    }
}
