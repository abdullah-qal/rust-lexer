use std::fs;
use thiserror::Error;
use std::fmt;

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
    let mut output: Vec<Expr<'_>> = Vec::new();
    let mut stack = Vec::new();
    let mut in_list = false;
    for (index, &value) in v.iter().enumerate() {
        if value == "(" {
            stack.push(index);
            in_list = true;
        } else if value == ")" {
            let open_index = stack.pop().ok_or_else(|| LexError::UnpairedParentheses())?;
            output.push(Expr::List(
                v[open_index + 1..index]
                    .iter()
                    .map(|s| Expr::Atom(*s))
                    .collect(),
            ));
            in_list = false;
        } else if !in_list {
            output.push(Expr::Atom(value))
        }
    }
    if !stack.is_empty() {
        return Err(LexError::UnpairedParentheses());
    }
    Ok(output)
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
    let tokens = tokeniser(&strings);
    println!("{:?}", tokeniser(&strings));
    let lexed = match parser(tokens) {
        Ok(n) => n,
        Err(e) => return println!("Error: {e}"),
    };
    println!("{:?}", lexed);
}
