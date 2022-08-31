use std::path::PathBuf;

mod arg_parser;
mod parse;

fn main() {
    let args = arg_parser::parse();

    let word: &String = &args.word;
    let path = args.path;

    let resoleved_path: Vec<String> = parse::resolve_path(&path);
    let data: Vec<String> = parse::read_file_content(&resoleved_path);

    let occurrences: Vec<Vec<String>> = parse::find_words_in_string(data, &word, args.ignore_case);

    print_result(&occurrences, &path);
}

// print the output of the grep process
fn print_result(occurrences: &Vec<Vec<String>>, path: &Vec<PathBuf>) {
    for i in 0..occurrences.len() {
        let occurrence: Vec<String> = occurrences.get(i).unwrap().to_vec();
        for line in occurrence {
            println!(
                "{}: {}",
                path.get(i).unwrap().to_string_lossy().to_string(),
                line
            );
        }
    }
}
