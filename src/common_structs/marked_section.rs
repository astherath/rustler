use super::{CommentType, Line, ParseData, ParsedDirectory, ParsedFile};
use std::cmp;

pub struct MarkedSection {
    pub lines: Vec<Line>,
    pub comment_type: CommentType,
    pub title_xxx: String,
}

impl MarkedSection {
    fn unpack_lines_for_directory(dir_data: ParsedDirectory, context: usize) -> Vec<Self> {
        dir_data
            .files
            .into_iter()
            .map(|x| Self::unpack_lines_for_file(x, context).into_iter())
            .flatten()
            .collect()
    }

    fn unpack_lines_for_file(file_data: ParsedFile, context: usize) -> Vec<Self> {
        let lines = file_data.lines;
        let mut marked_sections = Vec::new();

        let lines_len = lines.len();
        let mut i = 0;

        while i < lines_len {
            if CommentType::check_line_special(&lines[i]) {
                let mut current_lines = Vec::new();

                // add context # of lines behind
                let range_start = i.checked_sub(context).unwrap_or(0);
                for j in range_start..i {
                    current_lines.push(Line::new(&lines[j], j, false));
                }

                // add special line
                current_lines.push(Line::new(&lines[i], i, true));

                // add context # of line ahead
                let range_end = cmp::min(lines_len, i + context + 1);
                for j in (i + 1)..range_end {
                    current_lines.push(Line::new(&lines[j], j, false));
                }

                // get type of special line
                let comment_type = CommentType::get_special_line_type(&lines[i]);

                marked_sections.push(Self {
                    lines: current_lines,
                    title_xxx: file_data.filename.clone(),
                    comment_type,
                });
            }
            i += 1;
        }
        marked_sections
    }

    /// Reads the file line-by-line into a context-driven [`MarkedSection`](Self) struct
    ///
    /// # Arguments
    ///
    /// * `parsed_data` - The file data for a single file or a directory
    ///
    /// * `context` - The amount of context lines surrounding the special lines
    ///
    /// # Returns
    ///
    /// * `Vec<MarkedSection>` - All of the context-aware special lines
    ///
    /// # Notes
    ///
    /// If `context` passed in is > `lines.len()` then it will count context until EOF.
    pub fn unpack_lines(parsed_data: ParseData, context: usize) -> Vec<Self> {
        match parsed_data {
            ParseData::Directory(dir_data) => Self::unpack_lines_for_directory(dir_data, context),
            ParseData::File(file_data) => Self::unpack_lines_for_file(file_data, context),
        }
    }
}
