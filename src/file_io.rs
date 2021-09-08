use super::common_structs::{CommentType, MarkedSection};
use super::output_formatter;
use std::fs::{self, File};
use std::io::prelude::*;

pub struct TokenizedLine {
    pub line_number: usize,
    pub tokenized_line: Vec<String>,
}

impl TokenizedLine {
    pub fn from_code_line(line: String, line_number: usize) -> Self {
        let tokenized_line = line
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        Self {
            line_number,
            tokenized_line,
        }
    }
}

pub struct OutputBlock {
    pub block_type: CommentType,
    pub special_line: TokenizedLine,
    pub context_lines: Vec<TokenizedLine>,
}

impl OutputBlock {
    /// Process a single [`MarkedSection`](MarkedSection) into an [`OutputBlock`](Self)
    fn from_marked_section(marked_section: MarkedSection) -> Self {
        let mut special_line = None;
        let mut context_lines = Vec::new();

        for line in marked_section.lines {
            let output_line = TokenizedLine::from_code_line(line.content, line.number);
            if line.is_special {
                special_line = Some(output_line);
            } else {
                context_lines.push(output_line);
            }
        }

        Self {
            // should never be none
            special_line: special_line.unwrap(),
            block_type: marked_section.comment_type,
            context_lines,
        }
    }

    fn get_markdown_output_str(output_blocks: Vec<OutputBlock>) -> String {
        output_blocks
            .into_iter()
            .map(|x| output_formatter::get_output_str_for_block(x))
            .collect::<Vec<String>>()
            .join("")
    }
}

// top-level function for outputting to a markdown file
pub fn export_to_markdown(marked_sections: Vec<MarkedSection>, filename: &String) {
    let markdown_output_str = OutputBlock::get_markdown_output_str(
        marked_sections
            .into_iter()
            .map(|x| OutputBlock::from_marked_section(x))
            .collect::<Vec<OutputBlock>>(),
    );

    // let mut file = File::create(filename).unwrap();
    // file.write_all(&markdown_output_str.as_bytes()).unwrap();

    // println!("md output: {}", &markdown_output_str);

    fs::write(filename, &markdown_output_str.as_bytes()).unwrap();
}
