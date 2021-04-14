use std::env;
use std::fs;

fn charAt(string: String, index: usize) -> char {
    let bytes = string.into_bytes();
    let c: char = bytes[index] as char;
    return c;
}

fn readFile(file: String) -> Vec<String> {
    let data = fs::read_to_string(file).expect("Unable to read file");
    let split = data.split("\n");
    let mut vec: Vec<String> = Vec::new();
    for s in split {
        vec.push(s.to_string());
    }
    return vec;
}

fn sortFiles(files:Vec<String>, reverse:bool){
    for file in files {
        let mut vec:Vec<String> = readFile(file);
        vec.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));
        if reverse == true {
            vec.reverse();
        }
        for lines in vec {
            println!("{}", lines);
        }
    }
}

fn main() {
    let arg = env::args()
        .skip(1)
        .collect::<Vec<String>>();

}
