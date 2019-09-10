use lazy_static;
use regex::Regex;

/// Basic components of mustache expression
#[derive(Debug)]
pub enum Token {
    /// Token is a literal string
    Chars(String),
    /// Variable token with name
    Var(String),
}

pub fn compile_mustache(exp: &str) -> Vec<Token> {
    lazy_static! {
        static ref RE: Regex = Regex::new("{{\\s*[\\w\\.]+\\s*}}").unwrap();
    }
    RE.is_match(exp);
    vec![]
}
