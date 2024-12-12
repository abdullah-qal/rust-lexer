use std::fs;
use thiserror::Error;
// enum Expr<'a> {
//     Atom(&'a str),
//     List(Vec<Expr<'a>>),
// }

#[derive(Error, Debug)]
enum LexError {
    #[error("Failed to open file: {0}")]
    FileNotFound(String),
}

fn file_parser(input: &str) -> Result<String, LexError> {
    let output = fs::read_to_string(input)  
        .map_err(|_| LexError::FileNotFound(input.to_string()))?;
    Ok(output)
}

fn lex_string(s: &str) {
    println!("{s}");
}

fn main() {
    match file_parser("src/test.txt") {
        Ok(text) => lex_string(&text),
        Err(error) => eprintln!("Error: {}", error),

    };}
