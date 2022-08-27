use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;
use std::process::exit;

#[derive(Debug, Parser)]
struct Args {
    /// Word that you need to search
    #[clap()]
    word: String,

    /// path of the file where you need to search
    #[clap()]
    path: std::path::PathBuf,

    /// search for occurrences in both uppercase and lowercase
    #[clap(short = 'i', long = "ignore-case")]
    ignore_case: bool,
}

fn main() {
    let args = Args::parse();

    let path: String = resolve_path(args.path);
    let data: String = read_file_content(path);

    let occurrences: Vec<String> = find_words_in_string(data, args.word, args.ignore_case);

    for line in occurrences {
        println!("{}", line);
    }
}

// if the path_or_filename is an absolute path than return it as it is
// otherwise convert it to a relative path (relative from where the user is
// calling the function)
fn resolve_path(path_or_filename: PathBuf) -> String {
    let path: Result<PathBuf, std::io::Error> = path_or_filename.canonicalize();

    if path.is_ok() {
        path.unwrap().to_string_lossy().to_string()
    } else {
        eprintln!("File does not exists!");
        exit(1)
    }
}

// open the file and read the content
fn read_file_content(path: String) -> String {
    let data: String = read_to_string(path).expect("Unable to read the file!");

    data
}

// find all the lines where "word" appear
fn find_words_in_string(data: String, word: String, ignore_case: bool) -> Vec<String> {
    let mut occurrences: Vec<String> = Vec::new();

    if ignore_case {
        // case insensitive
        for line in data.lines() {
            if line.to_lowercase().contains(&word.to_lowercase()) {
                occurrences.push(line.to_string());
            }
        }
    } else {
        // case sensitive
        for line in data.lines() {
            if line.contains(&word) {
                occurrences.push(line.to_string());
            }
        }
    }

    occurrences
}
