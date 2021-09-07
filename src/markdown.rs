use std::fmt;

#[derive(Debug)]
struct MarkdownBuilder {
    contents: String,
}

impl MarkdownBuilder {
    pub fn new() -> Self {
        Self {
            contents: String::new(),
        }
    }

    // Monadic builder pattern for markdown creation

    pub fn checkbox(mut self) -> Self {
        let md_checkbox = "- [ ] ";
        self.contents.push_str(md_checkbox);
        self
    }

    pub fn newline(mut self) -> Self {
        self.contents.push('\n');
        self
    }

    /// Inserts text at the current position in the cursor
    pub fn insert(mut self, text: &str) -> Self {
        self.contents.push_str(text);
        self
    }

    /// Adds a MD header.
    ///
    /// Note: the header level must be (0, 6]
    pub fn header(mut self, level: u8) -> Result<Self, BuilderError> {
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
}

#[derive(Debug, PartialEq, Eq)]
enum BuilderError {
    HeaderOutOfBounds,
    Other(String),
}

impl fmt::Display for BuilderError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Self::HeaderOutOfBounds => "Header level invalid, must be (0, 6]".to_string(),
            Self::Other(reason) => format!("Could not build markdown string. Reason: {}", reason),
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

            let export_string = builder.header(header_level).unwrap().to_markdown_string();

            let expected_string = "#".repeat(header_level as usize).to_string();
            assert_eq!(export_string, expected_string);
        }
    }
}
