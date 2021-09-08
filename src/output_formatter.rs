use super::common_structs::CommentType;
use super::file_io::OutputBlock;
use super::markdown::{HeaderLevel, MarkdownBuilder};

pub fn get_header_str_for_block_type(block_type: &CommentType) -> MarkdownBuilder {
    let header_title = match block_type {
        CommentType::Todo => "TODO's",
        CommentType::Fixme => "FIXME's",
        CommentType::Note => "NOTE's",
        CommentType::XXX => "XXX's",
        CommentType::Other => "OTHER",
    };

    MarkdownBuilder::new()
        .header(HeaderLevel::H2)
        .insert_single_line(header_title)
        .unwrap()
        .newline()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn header_created_correctly() {
        let expected_string = "## TODO's\n";
        let header_string = get_header_str_for_block_type(&CommentType::Todo).to_markdown_string();

        assert_eq!(expected_string, header_string);
    }
}

// pub fn get_output_str_for_block_md(block: OutputBlock) -> String {
pub fn get_output_str_for_block(block: OutputBlock) -> String {
    let mut md_builder = MarkdownBuilder::new()
        .checkbox()
        .header(HeaderLevel::H4)
        .insert_single_line(&block.special_line.tokenized_line.join(" "))
        .unwrap()
        .indent()
        .newline();

    if block.context_lines.len() == 0 {
        return md_builder.to_markdown_string();
    }

    md_builder = md_builder
        .indent()
        .newline()
        .insert_single_line("- ")
        .unwrap()
        .header(HeaderLevel::H5)
        .insert_single_line(&format!(
            "Context for TODO in line #{}",
            block.special_line.line_number
        ))
        .unwrap()
        .newline();

    md_builder = md_builder
        .indent()
        .newline()
        .insert_single_line("- ```")
        .unwrap()
        .indent()
        .newline();

    let lines_in_block = block
        .context_lines
        .into_iter()
        .filter(|x| x.tokenized_line.len() > 1)
        .map(|x| x.tokenized_line.join(" "))
        .collect::<Vec<String>>();

    for line in lines_in_block {
        md_builder = md_builder.insert_single_line(&line).unwrap().newline()
    }

    md_builder
        .insert_single_line("```")
        .unwrap()
        .newline()
        .to_markdown_string()
}

pub fn _get_output_str_for_block(block: OutputBlock) -> String {
    let mut output_str = String::new();

    // MD format for a checkbox and a leading h4
    output_str.push_str("- [ ] #### ");

    // pushes the words together into a single string
    // (this gives us a chance to sanitize the words one by one)
    for word in block.special_line.tokenized_line {
        output_str.push_str(&format!("{} ", &word));
    }
    output_str.push_str("\n");

    // if there are no valid context lines, return early
    if block.context_lines.len() == 0 {
        return output_str;
    }

    // FIXME: the first part of the string is hardcoded, should be reliant on the CommentType for the block

    // pushes the context header string
    output_str.push_str(&format!(
        "\n\t - ##### Context for TODO in line #{}\n\n",
        block.special_line.line_number
    ));

    // builds the lines of context word by word (again, sanitizing)
    output_str.push_str("\t\t- ```\n");
    for line in block.context_lines {
        if line.tokenized_line.len() == 1 {
            continue;
        }
        output_str.push_str("\t\t\t");
        for word in line.tokenized_line {
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
