pub struct Line {
    pub number: usize,
    pub content: String,
    pub is_special: bool,
}

impl Line {
    pub fn new(raw_content: &str, number: usize, is_special: bool) -> Self {
        Self {
            number,
            content: raw_content.to_string(),
            is_special,
        }
    }
}
