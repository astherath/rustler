use std::fmt;

pub type BuilderResult<T> = Result<T, BuilderError>;

#[derive(Debug)]
pub struct MarkdownBuilder {
    contents: String,
    pub indentation_level: u8,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub enum HeaderLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl HeaderLevel {
    fn to_numeric(self) -> usize {
        match self {
            Self::H1 => 1,
            Self::H2 => 2,
            Self::H3 => 3,
            Self::H4 => 4,
            Self::H5 => 5,
            Self::H6 => 6,
        }
    }
}

impl MarkdownBuilder {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
            indentation_level: 0,
        }
    }

    // Monadic builder pattern for markdown creation

    /// Adds a MD header
    pub fn header(mut self, level: HeaderLevel) -> Self {
        let header = format!("{} ", "#".repeat(level.to_numeric()));
        self.contents.push_str(&header);
        self
    }

    pub fn newline(mut self) -> Self {
        self.contents.push('\n');
        for _ in 0..self.indentation_level {
            self.contents.push('\t');
        }
        // let indent = "\t".repeat(self.indentation_level as usize);
        // self.contents.push_str(&indent);
        self
    }

    pub fn increase_indentation_level(mut self) -> Self {
        self.indentation_level += 1;
        self
    }

    pub fn reset_indentation(mut self) -> Self {
        self.indentation_level = 0;
        self
    }

    pub fn checkbox(mut self) -> Self {
        let md_checkbox = "- [ ] ";
        self.contents.push_str(md_checkbox);
        self
    }

    /// Inserts text at the current position in the cursor.
    ///
    /// Will return an [`Err`](BuilderError) if the contents include a newline or a tab/indent character
    ///
    /// Note: Use the [`newline()`](Self.newline()) method if multi-line text is necessary.
    pub fn insert_single_line(mut self, text: &str) -> BuilderResult<Self> {
        if let Err(error) = Self::check_single_line_text_input_for_forbidden_characters(text) {
            return Err(error);
        }
        self.contents.push_str(text);
        Ok(self)
    }

    /// Finishes the builder pattern by consuming `Self` and returning the final string
    pub fn to_markdown_string(self) -> String {
        self.contents
    }

    fn check_single_line_text_input_for_forbidden_characters(text: &str) -> BuilderResult<()> {
        if text.contains('\n') {
            let err_reason = "newline character found in input.";
            Err(BuilderError::TextInsertError(err_reason.to_string()))
        } else if text.contains('\t') {
            let err_reason = "indentation character found in input.";
            Err(BuilderError::TextInsertError(err_reason.to_string()))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum BuilderError {
    TextInsertError(String),
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::TextInsertError(reason) => format!(
                "Text to be inserted contains invalid characters: {}",
                reason
            ),
        };
        write!(f, "{}", message)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_empty_builder() -> MarkdownBuilder {
        MarkdownBuilder::new()
    }

    mod insert {
        use super::*;

        #[test]
        fn single_line_text_inserted_correctly() {
            let builder = get_empty_builder();
            let text_to_insert = "test text";
            let export_string = builder
                .insert_single_line(&text_to_insert)
                .expect("valid text should not return Err")
                .to_markdown_string();

            assert_eq!(
                export_string, text_to_insert,
                "text inserted should match exported string on empty builder"
            );
        }

        #[test]
        fn multi_line_text_returns_error() {
            let builder = get_empty_builder();
            let text_to_insert = "multiline \n text";
            let insert_result = builder.insert_single_line(&text_to_insert);

            assert!(
                insert_result.is_err(),
                "insert text with newline present should return error"
            );

            let expected_error_message = "newline character found in input.".to_string();
            assert_eq!(
                insert_result.unwrap_err(),
                BuilderError::TextInsertError(expected_error_message),
                "invalid insert error should be of type 'TextInsertError`, with correct error message"
            );
        }

        #[test]
        fn indented_text_returns_error() {
            let builder = get_empty_builder();
            let text_to_insert = "indented \t text";
            let insert_result = builder.insert_single_line(&text_to_insert);

            assert!(
                insert_result.is_err(),
                "insert text with tab present should return error"
            );

            let expected_error_message = "indentation character found in input.".to_string();
            assert_eq!(
                insert_result.unwrap_err(),
                BuilderError::TextInsertError(expected_error_message),
                "invalid insert error should be of type 'TextInsertError`, with correct error message"
            );
        }
    }

    mod newline {
        use super::*;

        #[test]
        fn newline_appended_correctly() {
            let builder = get_empty_builder();
            let export_string = builder.newline().to_markdown_string();

            let expected_string = "\n".to_string();

            assert_eq!(
                export_string, expected_string,
                "newline should be present in empty builder export"
            );
        }

        #[test]
        fn single_newline_appended_with_single_indent_correctly() {
            let builder = get_empty_builder();
            let export_string = builder
                .increase_indentation_level()
                .newline()
                .to_markdown_string();

            let expected_string = "\n\t".to_string();

            assert_eq!(
                export_string, expected_string,
                "newline with indent should be present in empty builder export"
            );
        }

        #[test]
        fn mutlitple_newlines_adhere_to_single_indent_level() {
            let mut builder = get_empty_builder().increase_indentation_level();

            let newlines_to_append = 3;

            for _ in 0..newlines_to_append {
                builder = builder.newline();
            }

            let export_string = builder.to_markdown_string();

            let expected_string = "\n\t".repeat(newlines_to_append);

            assert_eq!(
                export_string, expected_string,
                "multiple newlines should adhere to the same indent level"
            );
        }

        #[test]
        fn single_newline_adheres_to_multiple_indents() {
            let indent_level = 3;
            let mut builder = get_empty_builder();
            for _ in 0..indent_level {
                builder = builder.increase_indentation_level();
            }

            let newlines_to_append = 3;

            for _ in 0..newlines_to_append {
                builder = builder.newline();
            }

            let export_string = builder.to_markdown_string();

            let indent_string = "\t".repeat(indent_level);
            let expected_string = format!("\n{}", &indent_string).repeat(newlines_to_append);

            assert_eq!(
                export_string, expected_string,
                "multiple newlines should adhere to the same indent level"
            );
        }

        #[test]
        fn multiple_newline_adheres_to_multiple_indents() {
            let indent_level = 5;
            let mut builder = get_empty_builder();
            for _ in 0..indent_level {
                builder = builder.increase_indentation_level();
            }

            let newlines_to_append = 6;

            for _ in 0..newlines_to_append {
                builder = builder.newline();
            }

            let export_string = builder.to_markdown_string();

            let indent_string = "\t".repeat(indent_level);
            let expected_string = format!("\n{}", &indent_string).repeat(newlines_to_append);

            assert_eq!(
                export_string, expected_string,
                "multiple newlines should adhere to the same indent level"
            );
        }
    }

    mod checkbox {
        use super::*;

        #[test]
        fn checkbox_created_correctly() {
            let builder = get_empty_builder();
            let export_string = builder.checkbox().to_markdown_string();

            let expected_string = "- [ ] ".to_string();

            assert_eq!(
                export_string, expected_string,
                "checkbox should be present in empty builder export"
            );
        }
    }

    mod headers {
        use super::*;

        #[test]
        fn header_builds_correctly() {
            let builder = get_empty_builder();
            let header_level = HeaderLevel::H4;

            let export_string = builder.header(HeaderLevel::H4).to_markdown_string();

            let expected_string = format!("{} ", "#".repeat(header_level.to_numeric()));
            assert_eq!(
                export_string, expected_string,
                "header should be appended if level is within bounds"
            );
        }
    }

    mod indentation {
        use super::*;

        #[test]
        fn indent_level_is_added_correctly() {
            let mut builder = get_empty_builder();
            assert_eq!(
                builder.indentation_level, 0,
                "should start with 0 indentation"
            );

            let indent_level = 6;
            for _ in 0..indent_level {
                builder = builder.increase_indentation_level();
            }

            assert_eq!(
                indent_level, builder.indentation_level,
                "should match indentation level of builder"
            );
        }

        #[test]
        fn indent_level_is_reset_correctly() {
            let mut builder = get_empty_builder();
            assert_eq!(
                builder.indentation_level, 0,
                "should start with 0 indentation"
            );

            let indent_level = 6;
            for _ in 0..indent_level {
                builder = builder.increase_indentation_level();
            }

            assert_eq!(
                indent_level, builder.indentation_level,
                "should match indentation level of builder"
            );

            builder = builder.reset_indentation();
            assert_eq!(
                builder.indentation_level, 0,
                "indent level should be 0 post-reset"
            );
        }
    }
}
