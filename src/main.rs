use std::fmt;
use std::fs;
use thiserror::Error;

enum Expr<'a> {
    Atom(&'a str),
    List(Vec<Expr<'a>>),
}

impl<'a> fmt::Debug for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Expr::Atom(ref s) => write!(f, "Atom({})", s),
            Expr::List(ref l) => write!(f, "List({:?})", l),
        }
    }
}

#[derive(Error, Debug)]
enum LexError {
    #[error("Failed to open file: {0}")]
    FileNotFound(String),
    #[error("Mismatched parentheses, please make sure your parentheses are paired")]
    UnpairedParentheses(),
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
    if start < s.len() {
        tokens.push(&s[start..]);
    }
    tokens
}
fn parser(v: Vec<&str>) -> Result<Vec<Expr<'_>>, LexError> {
    let mut stack = Vec::new();
    let mut current_list = Vec::new();

    for &value in v.iter() {
        if value == "(" {
            stack.push(current_list);
            current_list = Vec::new();
        } else if value == ")" {
            let list = current_list;
            if let Some(prev_list) = stack.pop() {
                current_list = prev_list;
                current_list.push(Expr::List(list)); 
            } else {
                return Err(LexError::UnpairedParentheses());
            }
        } else {
            current_list.push(Expr::Atom(value));
        }
    }

    if !stack.is_empty() {
        return Err(LexError::UnpairedParentheses());
    }

    Ok(current_list)
}

fn main() {
    let input = "src/test.txt";
    let strings = match file_reader(input) {
        Ok(out) => out,
        Err(e) => return println!("Error: {e}"),
    };
    let tokens = tokeniser(&strings);
    println!("{:?}", tokeniser(&strings));
    let lexed = match parser(tokens) {
        Ok(n) => n,
        Err(e) => return println!("Error: {e}"),
    };
    println!("{:?}", lexed);
}
