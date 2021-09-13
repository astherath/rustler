use super::common_structs::{ParseData, ParsedDirectory, ParsedFile};
use ansi_term::Colour;
use std::fs::{self, DirEntry, File};
use std::io::{self, BufRead};
use std::path::Path;

/// Note: this can either be a single file or an actual directory
pub fn read_data_for_path(path: &str) -> io::Result<ParseData> {
    if Path::new(path).is_dir() {
        Ok(ParseData::Directory(read_directory_data_recursive(path)?))
    } else {
        Ok(ParseData::File(read_file_data(path)?))
    }
}

pub fn read_directory_data_recursive(directory_path: &str) -> io::Result<ParsedDirectory> {
    let files = get_parsed_files_for_dir_rec(directory_path)?;

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

fn get_parsed_files_for_dir_rec(directory_path: &str) -> io::Result<Vec<ParsedFile>> {
    let unchecked_files = get_unchecked_files_for_dir_rec(directory_path)?;

    let mut files = vec![];
    for file_result in unchecked_files {
        files.push(ParsedFile {
            filename: file_result.0,
            lines: file_result.1?,
        });
    }

    Ok(files)
}

fn get_unchecked_files_for_dir_rec(
    directory_path: &str,
) -> io::Result<Vec<(String, io::Result<Vec<String>>)>> {
    type UncheckedFileData = (String, io::Result<Vec<String>>);
    let mut unchecked_files: Vec<UncheckedFileData> = vec![];
    let mut process = |dir: &DirEntry| {
        let dir_path = dir.path();
        let path = dir_path.as_os_str().to_str().unwrap();
        let lines = get_lines_from_file(&path);
        unchecked_files.push((path.to_string(), lines));
    };
    visit_dirs(&Path::new(directory_path), &mut process)?;
    Ok(unchecked_files)
}

// one possible implementation of walking a directory only visiting files
fn visit_dirs(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
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
