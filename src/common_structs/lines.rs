use std::{cmp, convert::TryInto};

/// Represents the different types of breadcrumb comments that
/// can be detected by the parser.
#[derive(PartialEq, Clone)]
pub enum CommentType {
    Todo,
    Fixme,
    Note,
    XXX,
    Other,
}

impl CommentType {
    /// Factory method in-place of a constructor for creating CommentTypes
    ///
    /// # Arguments
    ///
    /// * `line_str` - A borrowed String with the line contents
    ///
    /// # Returns
    ///
    /// A CommentType instance
    ///
    /// # Note
    ///
    /// This function should only be called if the line has been proven to be "special" already.
    /// If not, it will silently return a `CommentType::Other` which may have unintended effects.
    fn get_special_line_type(line_str: &String) -> CommentType {
        let lower_line = line_str.to_lowercase();
        if lower_line.contains("todo") {
            CommentType::Todo
        } else if lower_line.contains("fixme") {
            CommentType::Fixme
        } else if lower_line.contains("note") {
            CommentType::Note
        } else if lower_line.contains("xxx") {
            CommentType::XXX
        } else {
            CommentType::Other
        }
    }

    /// Factory method in-place of a constructor for creating CommentType instances for display args
    ///
    /// # Arguments
    ///
    /// * `type_opt` - A borrowed String with the CLI arg passed in for wanted type
    ///
    /// # Returns
    ///
    /// A [`CommentType`](CommentType) instance
    ///
    /// # Note
    ///
    /// If no match is found, will return [`CommentType::All`](CommentType::All).
    pub fn get_display_type(type_opt: &String) -> CommentType {
        if type_opt == "todo" {
            CommentType::Todo
        } else if type_opt == "fixme" {
            CommentType::Fixme
        } else if type_opt == "note" {
            CommentType::Note
        } else if type_opt == "xxx" {
            CommentType::XXX
        } else {
            CommentType::Other
        }
    }

    fn check_line_special(line: &String) -> bool {
        CommentType::get_special_line_type(line) != CommentType::Other
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
    pub patch_type: CommentType,
}

impl CodePatch {
    pub fn new(lines: Vec<CodeLine>, patch_type: CommentType) -> CodePatch {
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
            if CommentType::check_line_special(&lines[i]) {
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
                let patch_type = CommentType::get_special_line_type(&lines[i]);

                code_patch_vec.push(CodePatch::new(code_lines_vec, patch_type));
            }
            i += 1;
        }
        code_patch_vec
    }
}
