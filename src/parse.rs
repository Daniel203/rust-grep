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
    invert_match: bool,
    max_count: u32,
) -> Vec<Vec<String>> {
    let mut occurrences: Vec<Vec<String>> = Vec::new();
    let mut count_lines: u32;

    for datum in data {
        count_lines = 0;
        let mut occurrences_file: Vec<String> = Vec::new();
        for line in datum.lines() {
            let fixed_line: String;
            let fixed_word: String;
            if ignore_case {
                // case insensitive
                fixed_line = line.to_lowercase();
                fixed_word = word.to_lowercase();
            } else {
                //case sensitive
                fixed_line = line.to_string();
                fixed_word = word.to_string();
            }

            if fixed_line.contains(&fixed_word) && !invert_match {
                occurrences_file.push(line.to_string());
            } else if invert_match {
                occurrences_file.push(line.to_string());
            }

            count_lines += 1;
            if count_lines == max_count {
                break;
            }
        }

        occurrences.push(occurrences_file);
    }

    occurrences
}
