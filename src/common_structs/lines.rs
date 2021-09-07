pub struct Line {
    pub number: usize,
    pub content: String,
    pub is_special: bool,
}

impl Line {
    pub fn new(raw_content: &str, number: usize, is_special: bool) -> Self {
        let content = raw_content.trim().to_string();
        Self {
            number,
            content,
            is_special,
        }
    }
}
