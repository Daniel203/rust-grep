# rust-grep
Program in rust that simulates the behavior of the grep command in bash.

```$ grep word path```

#### For example
``` 
$ rust-grep main src/main.rs 
fn main() {
fn resolve_path(path_or_filename: String) -> String {
fn read_file_content(path: String) -> String {
fn find_words_in_string(data: String, word: String) -> Vec<String> {
```

Returns the lines of the file in the "path" where the word "word" is present

#### Avaialable cli arguments
| Short | Long | Description  |
|---|---|---|
| -i | --ignore-case | Ignore case distinctions in patterns and data |
| -v | --invert-match | Select non maching lines |
| -m | --max-count | Stop after NUM solected lines |
