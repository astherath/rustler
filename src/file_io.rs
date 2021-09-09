use super::common_structs::{MarkedSection, OutputBlock};
use super::output_formatter;
use std::fs;
use std::io;

/// top-level function for outputting to a markdown file
pub fn export_marked_sections_to_markdown_file(
    marked_sections: Vec<MarkedSection>,
    file_extension: Option<&str>,
    filename: &String,
) -> io::Result<()> {
    let markdown_output_str = output_formatter::get_markdown_output_str(
        marked_sections
            .into_iter()
            .map(|x| OutputBlock::from_marked_section(x))
            .collect::<Vec<OutputBlock>>(),
        file_extension,
    );

    fs::write(filename, &markdown_output_str.as_bytes())?;
    Ok(())
}
