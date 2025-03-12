use super::CharStream;
use std::fmt::Display;

#[derive(Debug)]
pub struct TemplateParsingError {
    msg: String,
    idx: usize,
    chars: Vec<char>,
}

impl TemplateParsingError {
    pub fn from_charstream(msg: &str, char_stream: CharStream) -> TemplateParsingError {
        TemplateParsingError {
            msg: msg.to_string(),
            idx: char_stream.idx,
            chars: char_stream.chars,
        }
    }

    pub fn err(msg: String, idx: usize, chars: Vec<char>) -> TemplateParsingError {
        TemplateParsingError { msg, idx, chars }
    }
}

impl Display for TemplateParsingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "TemplateParsingError: {}", self.msg)
    }
}

impl std::error::Error for TemplateParsingError {}
