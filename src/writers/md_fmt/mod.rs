use crate::writers::file_writer::OutputBlock;
use lines::lines::CodePatchType;

pub fn get_header_str_for_block_type(block_type: &CodePatchType) -> String {
    match block_type {
        CodePatchType::Todo => "## TODO's (+ context)\n\n",
        CodePatchType::Fixme => "## TODO's (+ context)\n\n",
        CodePatchType::Note => "## TODO's (+ context)\n\n",
        CodePatchType::XXX => "## TODO's (+ context)\n\n",
        CodePatchType::Other => "",
    }
    .to_string()
}

pub fn get_output_str_for_block(block: OutputBlock) -> String {
    let mut output_str = String::new();

    output_str.push_str("- [ ] #### ");
    for word in block.special_line.full_word_vec {
        output_str.push_str(&format!("{} ", &word));
    }
    output_str.push_str("\n");

    if block.context_lines.len() == 0 {
        return output_str;
    }
    // XXX: the TODO is hardcoded, should be reliant on the CodePatchType for the block
    output_str.push_str(&format!(
        "\n\t - ##### Context for TODO in line #{}\n\n",
        block.special_line.line_number
    ));
    for line in block.context_lines {
        output_str.push_str("\t\t- ");
        for word in line.full_word_vec {
            output_str.push_str(&format!("{} ", &word));
        }
        output_str.push_str("\n");
    }
    output_str
}
