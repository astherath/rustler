use std::fmt;

/// Represents the different types of breadcrumb comments that
/// can be detected by the parser.
#[derive(PartialEq, Clone)]
pub enum CommentType {
    Todo,
    Fixme,
    Note,
    XXX,
    Other,
}

impl CommentType {
    /// Factory method in-place of a constructor for creating CommentType instances for display args
    ///
    /// # Arguments
    ///
    /// * `type_opt` - A borrowed String with the CLI arg passed in for wanted type
    ///
    /// # Returns
    ///
    /// A [`CommentType`](Self) instance
    ///
    /// # Note
    ///
    /// If no match is found, will return [`CommentType::Other`](Self::Other).
    pub fn get_display_type(type_opt: &str) -> Self {
        if type_opt == "todo" {
            Self::Todo
        } else if type_opt == "fixme" {
            Self::Fixme
        } else if type_opt == "note" {
            Self::Note
        } else if type_opt == "xxx" {
            Self::XXX
        } else {
            Self::Other
        }
    }

    /// Returns the type of special line based on the string contents
    pub fn get_special_line_type(line_str: &str) -> Self {
        let lower_line = line_str.to_lowercase();
        if lower_line.contains("todo") {
            Self::Todo
        } else if lower_line.contains("fixme") {
            Self::Fixme
        } else if lower_line.contains("note") {
            Self::Note
        } else if lower_line.contains("xxx") {
            Self::XXX
        } else {
            Self::Other
        }
    }

    pub fn check_line_special(line: &str) -> bool {
        let is_special = Self::get_special_line_type(line) != Self::Other;
        let is_comment = is_comment(line);

        is_special && is_comment
    }
}

impl fmt::Display for CommentType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let comment_type = match self {
            Self::Todo => "Todo",
            Self::Fixme => "Fixme",
            Self::Note => "Note",
            Self::XXX => "XXX",
            Self::Other => "Other",
        };
        write!(f, "{}", comment_type)?;
        Ok(())
    }
}

fn is_comment(line: &str) -> bool {
    const COMMENT_SYMBOLS: [&str; 5] = ["//", "/***", "#", r#"""""#, "/**"];
    COMMENT_SYMBOLS.iter().map(|x| line.contains(x)).any(|x| x)
}
