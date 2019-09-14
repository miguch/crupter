use crate::utils::errors::MustacheError;
use lazy_static;
use regex::Regex;
use std::collections::HashMap;

/// Basic components of mustache expression
#[derive(Debug)]
pub enum Token {
    /// Token is a literal string
    Chars(String),
    /// Variable token with name
    Var(String),
}

impl Token {
    pub fn is_chars(&self) -> bool {
        match *self {
            Token::Chars(_) => true,
            _ => false,
        }
    }
    pub fn is_var(&self) -> bool {
        match *self {
            Token::Var(_) => true,
            _ => false,
        }
    }
}

pub type MustacheExp = Vec<Token>;

pub fn compile_mustache(exp: &str, allow_no_var: bool) -> Result<MustacheExp, failure::Error> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\{\{\s*[\w\.]+\s*\}\}").unwrap();
    }
    let matches = RE.find_iter(exp);
    let mut last_index = 0;
    let mut expression = MustacheExp::new();
    for m in matches {
        if m.start() > last_index {
            let part = exp[last_index..m.start()].to_owned();
            expression.push(Token::Chars(part));
        }
        let variable = exp[m.start() + 2..m.end() - 2].trim().to_owned();
        expression.push(Token::Var(variable));
        last_index = m.end();
    }
    if last_index < exp.len() {
        expression.push(Token::Chars(exp.chars().skip(last_index).collect()))
    }
    if !allow_no_var {
        if expression.iter().all(Token::is_chars) {
            Err(MustacheError::CompileError {
                msg: "expression contains no variable".to_owned(),
            })?;
        }
    }
    Ok(expression)
}

pub fn render(exp: &MustacheExp, data: &HashMap<&str, String>) -> Result<String, failure::Error> {
    let mut result = String::new();
    for token in exp {
        result.push_str(match token {
            Token::Chars(s) => s,
            Token::Var(name) => match data.get::<str>(name.as_str()) {
                Some(part) => part,
                None => Err(MustacheError::DataNotFoundError {
                    missing_field: name.to_owned(),
                })?,
            },
        })
    }
    Ok(result)
}
