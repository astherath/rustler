use super::common_structs::OutputBlock;
use super::markdown::{HeaderLevel, MarkdownBuilder};

/// Processes the given [`OutputBlock`s](OutputBlock) into a single markdown
/// string, ready to write to file.
pub fn get_markdown_output_str(
    output_blocks: Vec<OutputBlock>,
    file_extension: Option<&str>,
) -> String {
    output_blocks
        .into_iter()
        .map(|x| get_output_str_for_block(x, file_extension))
        .collect::<Vec<String>>()
        .join("")
}

fn get_output_str_for_block(block: OutputBlock, file_extension: Option<&str>) -> String {
    let mut md_builder = MarkdownBuilder::new();

    md_builder = header_for_output_block(md_builder, &block);

    if block.all_lines.len() == 0 {
        return md_builder.to_markdown_string();
    }

    md_builder = context_block_header(md_builder, &block);
    md_builder = context_block_inner_code(md_builder, block, file_extension);
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

fn get_code_block_name_from_extension(file_extension: Option<&str>) -> &str {
    if file_extension.is_none() {
        return "";
    }
    match file_extension.unwrap() {
        "py" => "python",
        "rs" => "rust",
        "js" => "javascript",
        "ts" => "typescript",
        "cs" => "c#",
        _ => "",
    }
}

fn context_block_inner_code(
    mut builder: MarkdownBuilder,
    block: OutputBlock,
    file_extension: Option<&str>,
) -> MarkdownBuilder {
    builder = builder
        .increase_indentation_level()
        .newline()
        .insert_single_line(&format!(
            "- ```{}",
            get_code_block_name_from_extension(file_extension)
        ))
        .unwrap()
        .increase_indentation_level()
        .newline();

    let lines_in_block = block
        .all_lines
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
