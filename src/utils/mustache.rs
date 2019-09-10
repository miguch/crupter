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
    pub fn isChars(&self) -> bool {
        match *self {
            Token::Chars(_) => true,
            _ => false,
        }
    }
    pub fn isVar(&self) -> bool {
        match *self {
            Token::Var(_) => true,
            _ => false,
        }
    }
}

pub type MustacheExp = Vec<Token>;

pub enum MustacheError {
    CompileError(String),
    DataNotFoundError(String),
}

impl std::fmt::Display for MustacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MustacheError::CompileError(msg) => write!(f, "{}", msg),
            MustacheError::DataNotFoundError(field) => write!(f, "cannot find field {}", field),
        }
    }
}

pub fn compile_mustache(exp: &str, allow_no_var: bool) -> Result<MustacheExp, MustacheError> {
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
        let variable = exp[m.start() + 2..m.end() - 2].to_owned();
        expression.push(Token::Var(variable));
        last_index = m.end();
    }
    if last_index < exp.len() {
        expression.push(Token::Chars(exp.chars().skip(last_index).collect()))
    }
    if !allow_no_var {
        if expression.iter().all(Token::isChars) {
            return Err(MustacheError::CompileError(
                "expression contains no variable".to_owned(),
            ));
        }
    }
    Ok(expression)
}

pub fn render(exp: &MustacheExp, data: HashMap<&str, String>) -> Result<String, MustacheError> {
    unimplemented!()
}
