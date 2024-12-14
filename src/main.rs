use std::fs;
use thiserror::Error;
enum Expr<'a> {
    Atom(&'a str),
    List(Vec<Expr<'a>>),
}

#[derive(Error, Debug)]
enum LexError {
    #[error("Failed to open file: {0}")]
    FileNotFound(String),
}

fn file_reader(input: &str) -> Result<String, LexError> {
    let output =
        fs::read_to_string(input).map_err(|_| LexError::FileNotFound(input.to_string()))?;
    Ok(output)
}

fn tokeniser(s: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut start = 0;

    for (i, c) in s.char_indices() {
        if c == '(' || c == ')' {
            if start < i {
                tokens.push(&s[start..i]);
            }
            tokens.push(&s[i..i + 1]);
            start = i + 1;
        } else if c.is_whitespace() {
            if start < i {
                tokens.push(&s[start..i]);
            }
            start = i + 1;
        }
    }
    if start < s.len() {tokens.push(&s[start..]);}
    tokens
}

fn main() {
    let input = "src/test.txt";
    let strings = match file_reader(input) {
        Ok(out) => out,
        Err(e) => {
            println!("Error: {e}");
            String::new()
        }
    };
   println!("{:?}", tokeniser(&strings));

}
