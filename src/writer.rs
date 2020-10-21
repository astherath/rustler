pub mod writer {

    use lines::lines::{CodePatch, CodePatchType};
    use std::fs::File;
    use std::io::prelude::*;

    pub fn write_todos_to_file(code_patches: &Vec<CodePatch>, filename: &String) {
        let mut file = File::create(filename).unwrap();
        let mut output_str = String::new();
        for patch in code_patches {
            if patch.patch_type != CodePatchType::Todo {
                continue;
            }
            for line in &patch.lines {
                output_str.push_str(&line.content);
            }
        }
        file.write_all(&output_str.as_bytes()).unwrap();
    }
}
