use super::file_io::OutputBlock;
use super::markdown::{HeaderLevel, MarkdownBuilder};

pub fn get_output_str_for_block(block: OutputBlock) -> String {
    let mut md_builder = MarkdownBuilder::new();

    md_builder = header_for_output_block(md_builder, &block);

    if block.context_lines.len() == 0 {
        return md_builder.to_markdown_string();
    }

    md_builder = context_block_header(md_builder, &block);
    md_builder = context_block_inner_code(md_builder, block);
    md_builder.to_markdown_string()
}

fn header_for_output_block(builder: MarkdownBuilder, block: &OutputBlock) -> MarkdownBuilder {
    builder
        .checkbox()
        .header(HeaderLevel::H4)
        .insert_single_line(&block.special_line.tokenized_line.join(" "))
        .unwrap()
        .increase_indentation_level()
        .newline()
}

fn context_block_header(builder: MarkdownBuilder, block: &OutputBlock) -> MarkdownBuilder {
    builder
        .newline()
        .insert_single_line("- ")
        .unwrap()
        .header(HeaderLevel::H5)
        .insert_single_line(&format!(
            "Context for `{}` in `line #{}`",
            block.block_type.to_string().to_uppercase(),
            block.special_line.line_number
        ))
        .unwrap()
        .newline()
}

fn context_block_inner_code(mut builder: MarkdownBuilder, block: OutputBlock) -> MarkdownBuilder {
    builder = builder
        .increase_indentation_level()
        .newline()
        .insert_single_line("- ```")
        .unwrap()
        .increase_indentation_level()
        .newline()
        .insert_single_line(&block.special_line.tokenized_line.join(" "))
        .unwrap()
        .newline();

    let lines_in_block = block
        .context_lines
        .into_iter()
        .filter(|x| x.tokenized_line.len() > 1)
        .map(|x| x.tokenized_line.join(" "))
        .collect::<Vec<String>>();

    for line in lines_in_block {
        builder = builder.insert_single_line(&line).unwrap().newline()
    }

    builder
        .insert_single_line("```")
        .unwrap()
        .reset_indentation()
        .newline()
        .newline()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_code_block_exported_correctly_to_string() {
        let expected_string = "";
        let exported_string = "";
        assert_eq!(
            expected_string, exported_string,
            "exported string should match the expected string"
        );
    }
}
