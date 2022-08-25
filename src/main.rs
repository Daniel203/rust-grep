use std::env::{self, current_dir};
use std::fs::read_to_string;

fn main() {
    let word: Option<String> = env::args().nth(1);
    let path_or_filename: Option<String> = env::args().nth(2);

    if word.is_none() || path_or_filename.is_none() {
        println!("Usage: {} <word> <path>", env::args().nth(0).unwrap());
        return;
    } else {
        let word = word.unwrap();
        let path_or_filename = path_or_filename.unwrap();

        let path: String = resolve_path(path_or_filename);
        let data: String = read_file_content(path);
        let occurrences: Vec<String> = find_words_in_string(data, word);

        for line in occurrences {
            println!("{}", line);
        }
    }
}

// if the path_or_filename is an absolute path than return it as it is
// otherwise convert it to a relative path (relative from where the user is
// calling the function)
fn resolve_path(path_or_filename: String) -> String {
    let path: String;

    if path_or_filename.starts_with("/") {
        path = path_or_filename;
    } else {
        let current_dir: String = current_dir().unwrap().to_str().unwrap().to_string();
        path = [current_dir, path_or_filename].join("/");
    }

    path
}

// open the file and read the content
fn read_file_content(path: String) -> String {
    let data: String = read_to_string(path).expect("Unable to read the file!");

    data
}

// find all the lines where "word" appear
fn find_words_in_string(data: String, word: String) -> Vec<String> {
    let mut occurrences: Vec<String> = Vec::new();

    for line in data.lines() {
        if line.contains(&word) {
            occurrences.push(line.to_string());
        }
    }

    occurrences
}
