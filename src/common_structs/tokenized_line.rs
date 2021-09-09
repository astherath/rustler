pub struct TokenizedLine {
    pub line_number: usize,
    pub tokenized_line: Vec<String>,
}

impl TokenizedLine {
    pub fn from_code_line(line: String, line_number: usize) -> Self {
        let tokenized_line = line
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        Self {
            line_number,
            tokenized_line,
        }
    }
}
