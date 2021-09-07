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

    pub fn checkbox(&mut self) -> &mut Self {
        let md_checkbox = "- [ ] ";
        self.contents.push_str(md_checkbox);
        self
    }

    pub fn newline(&mut self) -> &mut Self {
        self.contents.push('\n');
        self
    }

    pub fn insert(&mut self, text: &str) -> &mut Self {}
}
