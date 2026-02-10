use std::{env, fs};

mod lexer;
mod parser;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];

    let infile = fs::read_to_string(file_path).expect("Unable to read file.");
    let mut lexer = lexer::Lexer::new(&infile);
    let tokens = lexer.lex();

    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    println!("{:?}", ast);
}
