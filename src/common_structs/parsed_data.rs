pub struct ParsedDirectory {
    pub directory_path: String,
    pub files: Vec<ParsedFile>,
}

pub struct ParsedFile {
    pub filename: String,
    pub lines: Vec<String>,
}

pub enum ParseData {
    Directory(ParsedDirectory),
    File(ParsedFile),
}
