use std::{fs::read_to_string, path::PathBuf};

// if the path_or_filename is an absolute path than return it as it is
// otherwise convert it to a relative path (relative from where the user is
// calling the function)
pub fn resolve_path(path_or_filename: &Vec<PathBuf>) -> Vec<String> {
    let mut path_resolved: Vec<String> = Vec::new();

    for file_path in path_or_filename {
        let path: Result<PathBuf, std::io::Error> = file_path.canonicalize();

        if path.is_ok() {
            path_resolved.push(path.unwrap().to_string_lossy().to_string());
        }
    }

    if path_resolved.len() > 0 {
        path_resolved
    } else {
        eprintln!("File does not exists!");
        return Vec::new();
    }
}

// open the file and read the content
pub fn read_file_content(paths: &Vec<String>) -> Vec<String> {
    let mut data: Vec<String> = Vec::new();

    for path in paths {
        let content: String = read_to_string(path).unwrap();
        data.push(content);
    }

    data
}

// find all the lines where "word" appear
pub fn find_words_in_string(
    data: Vec<String>,
    word: &String,
    ignore_case: bool,
) -> Vec<Vec<String>> {
    let mut occurrences: Vec<Vec<String>> = Vec::new();

    for datum in data {
        let mut occurrences_file: Vec<String> = Vec::new();
        if ignore_case {
            // case insensitive
            for line in datum.lines() {
                if line.to_lowercase().contains(&word.to_lowercase()) {
                    occurrences_file.push(line.to_string());
                }
            }
        } else {
            // case sensitive
            for line in datum.lines() {
                if line.contains(word) {
                    occurrences_file.push(line.to_string());
                }
            }
        }

        occurrences.push(occurrences_file);
    }

    occurrences
}
