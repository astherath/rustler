use super::common_structs::{CommentType, MarkedSection};
use super::markdown_fmt;
use std::fs::File;
use std::io::prelude::*;

pub struct OutputLine {
    pub line_number: usize,
    pub full_word_vec: Vec<String>,
}

impl OutputLine {
    fn new(full_word_vec: Vec<String>, line_number: usize) -> OutputLine {
        OutputLine {
            line_number,
            full_word_vec,
        }
    }

    fn from_code_line(line: String, line_number: usize) -> OutputLine {
        // populates a String vec with the words from the code line (split by " ")
        let split_line = {
            let mut split_line_vec = Vec::new();
            let line_iter = line.split(" ");
            for line_str in line_iter {
                split_line_vec.push(line_str.to_string());
            }
            split_line_vec
        };

        OutputLine::new(split_line, line_number)
    }
}

pub struct OutputBlock {
    pub block_type: CommentType,
    pub special_line: OutputLine,
    pub context_lines: Vec<OutputLine>,
}

impl OutputBlock {
    fn new(
        block_type: CommentType,
        special_line: OutputLine,
        context_lines: Vec<OutputLine>,
    ) -> OutputBlock {
        OutputBlock {
            block_type,
            special_line,
            context_lines,
        }
    }

    // Faux-construction
    // processes a single MarkedSection into an output-ready OutputBlock
    fn from_code_patch(code_patch: MarkedSection) -> OutputBlock {
        let mut special_line = None;
        let mut context_lines_vec = Vec::new();

        // process all of the lines, saving all but one as context
        // the single "special" line is saved separately
        for line in code_patch.lines {
            let output_line = OutputLine::from_code_line(line.content, line.number);
            if line.is_special {
                special_line = Some(output_line);
            } else {
                context_lines_vec.push(output_line);
            }
        }
        OutputBlock::new(
            code_patch.comment_type,
            special_line.unwrap(),
            context_lines_vec,
        )
    }

    // Calls the markdown_fmt module to get final formatted Markdown string to write to file
    fn get_markdown_output_str(output_blocks: Vec<OutputBlock>) -> String {
        // TODO:    this function can eventually be re-used if the markdown_fmt methods are swapped out
        //          (i.e. the block handling is the same regardless)

        let mut output_str = String::new();
        let mut last_block_type = CommentType::Other;

        // iterates over the block vec, appending the markdown output_str for each block
        // also pushes a new the header if needed (for new block_type)
        for block in output_blocks {
            if last_block_type != block.block_type {
                let header_str = markdown_fmt::get_header_str_for_block_type(&block.block_type);
                output_str.push_str(&header_str);
                last_block_type = block.block_type.clone();
            }

            output_str.push_str(&markdown_fmt::get_output_str_for_block(block));
        }
        output_str
    }
}

// top-level function for outputting to a markdown file
pub fn export_to_markdown(code_patches: Vec<MarkedSection>, filename: &String) {
    let mut todo_output_blocks = Vec::new();

    // iterates over the patch vec and populates the output_block vec with the data
    for patch in code_patches {
        if patch.comment_type == CommentType::Todo {
            todo_output_blocks.push(OutputBlock::from_code_patch(patch));
        }
    }

    // this calls the top-level markdown formatting/processing output function
    // in the future, this could be a generic handler and be passed in a format enum
    let output_str = OutputBlock::get_markdown_output_str(todo_output_blocks);

    let mut file = File::create(filename).unwrap();
    file.write_all(&output_str.as_bytes()).unwrap();
}
