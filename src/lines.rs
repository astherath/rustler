pub mod lines {

    pub struct CodePatch {
        pub line: String,
    }

    impl CodePatch {
        pub fn new(line: String) -> CodePatch {
            CodePatch { line }
        }
    }
}
