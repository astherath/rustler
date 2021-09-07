use std::fmt;

pub type BuilderResult<T> = Result<T, BuilderError>;

#[derive(Debug)]
pub struct MarkdownBuilder {
    contents: String,
    indentation_level: u8,
}

impl MarkdownBuilder {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
            indentation_level: 0,
        }
    }

    // Monadic builder pattern for markdown creation

    pub fn checkbox(mut self) -> Self {
        let md_checkbox = "- [ ] ";
        self.contents.push_str(md_checkbox);
        self
    }

    pub fn newline(mut self) -> Self {
        let indent = "\t".repeat(self.indentation_level as usize);
        self.contents.push_str(&indent);
        self.contents.push('\n');
        self
    }

    pub fn indent(mut self) -> Self {
        self.indentation_level += 1;
        self
    }

    pub fn unindent(mut self) -> Self {
        if self.indentation_level > 0 {
            self.indentation_level -= 1;
        }

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

    /// Adds a MD header.
    ///
    /// Note: the header level must be (0, 6]
    pub fn header(mut self, level: u8) -> BuilderResult<Self> {
        match level {
            0..=6 => {
                self.contents.push_str(&"#".repeat(level as usize));
                Ok(self)
            }
            _ => Err(BuilderError::HeaderOutOfBounds),
        }
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
    HeaderOutOfBounds,
    TextInsertError(String),
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::HeaderOutOfBounds => "Header level invalid, must be (0, 6]".to_string(),
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
            let export_string = builder.indent().newline().to_markdown_string();

            let expected_string = "\t\n".to_string();

            assert_eq!(
                export_string, expected_string,
                "newline with indent should be present in empty builder export"
            );
        }

        #[test]
        fn mutlitple_newlines_adhere_to_single_indent_level() {
            let mut builder = get_empty_builder().indent();

            let newlines_to_append = 3;

            for _ in 0..newlines_to_append {
                builder = builder.newline();
            }

            let export_string = builder.to_markdown_string();

            let expected_string = "\t\n".repeat(newlines_to_append);

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
        fn header_checks_bounds() {
            let builder = get_empty_builder();
            let upper_out_of_bounds_header_level = 7;

            // no need to check for lower bound where n < 0 since datatype is u8.
            let result = builder.header(upper_out_of_bounds_header_level);

            assert!(
                result.is_err(),
                "header addition should return err if our of bounds"
            );
            assert_eq!(
                result.unwrap_err(),
                BuilderError::HeaderOutOfBounds,
                "error type should be 'HeaderOutOfBounds'"
            );
        }

        #[test]
        fn header_builds_correctly() {
            let builder = get_empty_builder();
            let header_level = 4;

            let export_string = builder
                .header(header_level)
                .expect("header level is valid and should not return Err")
                .to_markdown_string();

            let expected_string = "#".repeat(header_level as usize).to_string();
            assert_eq!(
                export_string, expected_string,
                "header should be appended if level is within bounds"
            );
        }
    }
}
