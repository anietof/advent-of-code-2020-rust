use std::fs;

pub fn get_default_file_input() -> String {
    return get_file_input("input/input.txt")
}

pub fn get_file_input(path: &str) -> String {
    return fs::read_to_string(path)
        .expect("Something went wrong reading the file");
}