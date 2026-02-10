use std::{env, fs};

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let infile = fs::read_to_string(file_path).expect("Unable to read file.");
    let mut lexer = lexer::Lexer::new(&infile);
    let tokens = lexer.lex();
    println!("{:#?}", tokens);
}
