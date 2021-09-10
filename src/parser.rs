use ansi_term::Colour;
use std::fs::{self, DirEntry, File};
use std::io::{self, BufRead};
use std::path::Path;

struct ParsedDirectory {
    directory_path: String,
    files: Vec<ParsedFile>,
}

struct ParsedFile {
    filename: String,
    lines: Vec<String>,
}

pub fn read_directory_data_recursive(directory_path: &str) -> io::Result<ParsedDirectory> {
    let files = vec![];

    Ok(ParsedDirectory {
        directory_path: directory_path.to_string(),
        files,
    })
}

/// Top-level function to mask the usage of the FileParser for a simple interface
pub fn read_file_data(file_path: &str) -> io::Result<ParsedFile> {
    let lines = get_lines_from_file(file_path)?;
    Ok(ParsedFile {
        filename: file_path.to_string(),
        lines,
    })
}

fn parse_all_(directory_path: &str) -> io::Result<ParsedDirectory> {
    struct UncheckedFileData {
        filename: String,
        unchecked_lines: io::Result<Vec<String>>,
    }
    let mut unchecked_files: Vec<UncheckedFileData> = vec![];
    let process = |dir: &DirEntry| {
        let path = dir.path().as_os_str().to_str().unwrap();
        let lines = get_lines_from_file(&path);
        let parsed_file = UncheckedFileData {
            filename: path.to_string(),
            unchecked_lines: lines,
        };
        unchecked_files.push(parsed_file);
    };
    visit_dirs(&Path::new(directory_path), &process)?;

    let files = vec![];
    for file_result in unchecked_files {
        files.push(ParsedFile {
            filename: file_result.filename,
            lines: file_result.unchecked_lines?,
        });
    }

    Ok(ParsedDirectory {
        directory_path: directory_path.to_string(),
        files,
    })
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &dyn FnMut(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

/// Reads and returns an iterator with the line data for the given filename
///
/// # Arguments
///
/// * `filename` - A string reference of the filename
///
/// # Returns
///
/// `Vec<String>` of all of the lines in the file
fn get_lines_from_file(filename: &str) -> io::Result<Vec<String>> {
    let path = Path::new(filename);
    let file = File::open(&path)?;

    // unpack line iterator into vector
    let lines_iter = io::BufReader::new(file).lines();
    let mut lines = Vec::new();

    for line_result in lines_iter {
        match line_result {
            Err(why) => panic!("cannot read line {}", Colour::Red.paint(why.to_string())),
            Ok(line) => {
                if !line.is_empty() {
                    lines.push(line)
                }
            }
        }
    }
    Ok(lines)
}
