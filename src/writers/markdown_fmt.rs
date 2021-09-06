use super::file_io::OutputBlock;
use lines::lines::CodePatchType;

pub fn get_header_str_for_block_type(block_type: &CodePatchType) -> String {
    match block_type {
        CodePatchType::Todo => "## TODO's\n\n",
        CodePatchType::Fixme => "## TODO's\n\n",
        CodePatchType::Note => "## TODO's\n\n",
        CodePatchType::XXX => "## TODO's\n\n",
        CodePatchType::Other => "",
    }
    .to_string()
}

pub fn get_output_str_for_block(block: OutputBlock) -> String {
    let mut output_str = String::new();

    // MD format for a checkbox and a leading h4
    output_str.push_str("- [ ] #### ");

    // pushes the words together into a single string
    // (this gives us a chance to sanitize the words one by one)
    for word in block.special_line.full_word_vec {
        output_str.push_str(&format!("{} ", &word));
    }
    output_str.push_str("\n");

    // if there are no valid context lines, return early
    if block.context_lines.len() == 0 {
        return output_str;
    }

    // FIXME: the first part of the string is hardcoded, should be reliant on the CodePatchType for the block

    // pushes the context header string
    output_str.push_str(&format!(
        "\n\t - ##### Context for TODO in line #{}\n\n",
        block.special_line.line_number
    ));

    // builds the lines of context word by word (again, sanitizing)
    output_str.push_str("\t\t- ```\n");
    for line in block.context_lines {
        if line.full_word_vec.len() == 1 {
            continue;
        }
        output_str.push_str("\t\t\t");
        for word in line.full_word_vec {
            // check so as to avoid single char lines
            output_str.push_str(&format!("{} ", &word));
        }

        // pops the trailing space and adds a newline
        output_str.pop();
        output_str.push_str("\n");
    }
    output_str.push_str("\t\t\t```\n");
    output_str
}
