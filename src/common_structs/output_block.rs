use super::{CommentType, MarkedSection, TokenizedLine};

pub struct OutputBlock {
    pub block_type: CommentType,
    pub special_line: TokenizedLine,
    pub context_lines: Vec<TokenizedLine>,
}

impl OutputBlock {
    /// Process a single [`MarkedSection`](MarkedSection) into an [`OutputBlock`](Self)
    pub fn from_marked_section(marked_section: MarkedSection) -> Self {
        let mut special_line = None;
        let mut context_lines = Vec::new();

        for line in marked_section.lines {
            let output_line = TokenizedLine::from_code_line(line.content, line.number);
            if line.is_special {
                special_line = Some(output_line);
            } else {
                context_lines.push(output_line);
            }
        }

        Self {
            // should never be none
            special_line: special_line.unwrap(),
            block_type: marked_section.comment_type,
            context_lines,
        }
    }
}
