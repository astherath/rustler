use super::common_structs::{ParseData, ParsedDirectory, ParsedFile};
use std::fs::{self, DirEntry, File};
use std::io::{self, BufRead, ErrorKind};
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
    let lines = get_lines_from_file(Path::new(file_path)).unwrap()?;
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
        if let Some(lines) = get_lines_from_file(&dir.path()) {
            unchecked_files.push((path_to_str(&dir.path()).to_string(), lines));
        }
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
            if path.is_dir() && !path_should_be_skipped(&path) {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn path_to_str(path: &Path) -> &str {
    path.as_os_str().to_str().unwrap()
}

fn path_should_be_skipped(path: &Path) -> bool {
    const FORBIDDEN_PATH_ARRAY: [&str; 4] = [".git", "target", ".config", "~"];
    FORBIDDEN_PATH_ARRAY
        .iter()
        .map(|x| path_to_str(path).contains(x))
        .any(|x| x)
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
fn get_lines_from_file(file_path: &Path) -> Option<io::Result<Vec<String>>> {
    if !file_path.is_file() {
        return None;
    }

    let file = File::open(&file_path).ok()?;

    let mut lines = Vec::new();
    for line_result in io::BufReader::new(file).lines() {
        match line_result {
            Err(io_error) => {
                if io_error.kind() == ErrorKind::InvalidData {
                    return None;
                } else {
                    return Some(Err(io_error));
                }
            }
            Ok(line) => {
                if !line.is_empty() {
                    lines.push(line)
                }
            }
        }
    }
    Some(Ok(lines))
}
