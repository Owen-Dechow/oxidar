mod error;

use super::TemplateVar;
pub use error::TemplateParsingError;

struct CharStream {
    idx: usize,
    chars: Vec<char>,
}

impl CharStream {
    fn new(txt: String) -> CharStream {
        CharStream {
            idx: 0,
            chars: txt.chars().collect(),
        }
    }

    fn get(&mut self) -> Option<&char> {
        let c = self.chars.get(self.idx);
        self.idx += 1;
        return c;
    }
}

struct TokenStream<'a> {
    idx: usize,
    tokens: Vec<(TemplateToken<'a>, usize)>,
}

impl<'a> TokenStream<'a> {
    fn new(tokens: Vec<(TemplateToken<'a>, usize)>) -> TokenStream<'a> {
        TokenStream { idx: 0, tokens }
    }

    fn get(&mut self) -> Option<&(TemplateToken<'a>, usize)> {
        let token = match self.tokens.get(self.idx) {
            Some(token) => Some(token),
            None => None,
        };

        self.idx += 1;
        return token;
    }

    fn next_is_html(&self) -> bool {
        if let Some((TemplateToken::Html(_), _)) = self.tokens.get(self.idx) {
            return true;
        }

        return false;
    }
}

#[derive(Debug)]
enum TemplateToken<'a> {
    If,
    End,
    Else,
    For,
    In,
    Safe,
    Block,
    Let,
    Add,
    Subtract,
    Eq,
    NotEq,
    Not,
    Less,
    Greater,
    LessEq,
    GreaterEq,
    Set,
    And,
    Or,
    Value(TemplateVar),
    ValueRef(&'a TemplateVar),
    Html(String),
}

#[derive(Debug)]
enum ParserState {
    Html,
    Str(char, bool),
    KeyWordOperator,
    Num(bool),
    None,
}

fn finalize_block_token<'a>(
    output: &mut Vec<(TemplateToken<'a>, usize)>,
    token: &mut String,
    data: &'a TemplateVar,
    char_stream: CharStream,
) -> Result<CharStream, TemplateParsingError> {
    let block_token = match token.as_str() {
        "if" => (true, TemplateToken::If),
        "end" => (true, TemplateToken::End),
        "else" => (true, TemplateToken::Else),
        "for" => (true, TemplateToken::For),
        "in" => (true, TemplateToken::In),
        "block" => (true, TemplateToken::Block),
        "let" => (false, TemplateToken::Let),
        "safe" => (true, TemplateToken::Let),
        "+" => (false, TemplateToken::Add),
        "-" => (false, TemplateToken::Subtract),
        "==" => (true, TemplateToken::Eq),
        "!=" => (true, TemplateToken::NotEq),
        "!" => (true, TemplateToken::Not),
        "<" => (true, TemplateToken::Less),
        ">" => (true, TemplateToken::Greater),
        "<=" => (true, TemplateToken::LessEq),
        ">=" => (true, TemplateToken::GreaterEq),
        "=" => (false, TemplateToken::Set),
        "||" => (false, TemplateToken::Or),
        "&&" => (false, TemplateToken::And),
        "true" => (true, TemplateToken::Value(TemplateVar::Bool(true))),
        "false" => (true, TemplateToken::Value(TemplateVar::Bool(false))),
        "None" => (true, TemplateToken::Value(TemplateVar::None)),
        _ => (true, TemplateToken::ValueRef(data.resolve(token))),
    };

    if !block_token.0 {
        return Err(TemplateParsingError::from_charstream(
            &format!(
                "{block_token:?} {} {}",
                "is only partially implimented. Because it will be implimented in",
                "the future versions you can not use it here as a variable name."
            ),
            char_stream,
        ));
    }

    output.push((block_token.1, char_stream.idx));
    token.clear();
    return Ok(char_stream);
}

fn handle_char<'a>(
    output: &mut Vec<(TemplateToken<'a>, usize)>,
    c: char,
    state: &mut ParserState,
    current_content: &mut String,
    mut char_stream: CharStream,
    data: &'a TemplateVar,
) -> Result<CharStream, TemplateParsingError> {
    match state {
        ParserState::Html => match c {
            '{' => {
                if let Some('{') = char_stream.get() {
                    current_content.push('{');
                } else {
                    char_stream.idx -= 1;
                    output.push((
                        TemplateToken::Html(current_content.clone()),
                        char_stream.idx,
                    ));
                    current_content.clear();
                    *state = ParserState::None;
                }
            }
            _ => current_content.push(c),
        },
        ParserState::Str(quote_char, escaped) => {
            if *escaped {
                let c = match c {
                    '"' => '"',
                    '\'' => '\'',
                    'n' => '\n',
                    't' => '\t',
                    'r' => '\r',
                    '\\' => '\\',
                    '\0' => '\0',
                    _ => {
                        return Err(TemplateParsingError::from_charstream(
                            &format!("\"\\{c}\" is not a valid escape sequence."),
                            char_stream,
                        ))
                    }
                };

                *escaped = false;
                current_content.push(c);
            } else {
                if c == *quote_char {
                    output.push((
                        TemplateToken::Value(TemplateVar::Str(current_content.clone())),
                        char_stream.idx,
                    ));

                    current_content.clear();
                    *state = ParserState::None;
                } else if c == '\'' {
                    *escaped = true;
                } else {
                    current_content.push(c);
                }
            }
        }
        ParserState::KeyWordOperator => {
            if c.is_whitespace() {
                char_stream = finalize_block_token(output, current_content, data, char_stream)?;
                *state = ParserState::None;
            } else if c == '}' {
                char_stream = finalize_block_token(output, current_content, data, char_stream)?;
                *state = ParserState::Html;
            } else {
                current_content.push(c);
            }
        }
        ParserState::Num(is_dec) => {
            if c.is_ascii_digit() {
                current_content.push(c);
            } else if c == '.' {
                match is_dec {
                    false => {
                        current_content.push('.');
                        *is_dec = true
                    }
                    true => {
                        return Err(TemplateParsingError::from_charstream(
                            &format!(
                                "The number \"{current_content}.\" {}",
                                "already has a decimal in it. It may not have more than one."
                            ),
                            char_stream,
                        ));
                    }
                }
            } else if c.is_whitespace() || c == '}' {
                output.push((
                    TemplateToken::Value(TemplateVar::Num(match current_content.parse() {
                        Ok(num) => num,
                        Err(err) => {
                            return Err(TemplateParsingError::from_charstream(
                                &format!("Error parsing number: {err}"),
                                char_stream,
                            ))
                        }
                    })),
                    char_stream.idx,
                ));

                *state = match c {
                    '}' => ParserState::Html,
                    _ => ParserState::None,
                };
                current_content.clear();
            } else {
                return Err(TemplateParsingError::from_charstream(
                    &format!("\"{c}\" is an invalid character for a number."),
                    char_stream,
                ));
            }
        }
        ParserState::None => {
            if c.is_whitespace() {
                return Ok(char_stream);
            } else if c.is_numeric() {
                current_content.push(c);
                *state = ParserState::Num(false)
            } else if "\"'".contains(c) {
                *state = ParserState::Str(c, false)
            } else if c == '}' {
                *state = ParserState::Html;
            } else {
                *state = ParserState::KeyWordOperator;
                current_content.push(c);
            }
        }
    }

    return Ok(char_stream);
}

fn lex(
    initial: String,
    data: &TemplateVar,
) -> Result<(Vec<(TemplateToken, usize)>, Vec<char>), TemplateParsingError> {
    let mut output = Vec::new();
    let mut state = ParserState::Html;
    let mut current_content = String::new();
    let mut char_stream = CharStream::new(initial);
    while let Some(c) = char_stream.get() {
        char_stream = handle_char(
            &mut output,
            *c,
            &mut state,
            &mut current_content,
            char_stream,
            data,
        )?;
    }

    if let ParserState::Html = state {
        output.push((TemplateToken::Html(current_content), char_stream.idx));
    } else {
        return Err(TemplateParsingError::from_charstream(
            "Template block not closed.",
            char_stream,
        ));
    }

    return Ok((output, char_stream.chars));
}

fn resolve_value_led_block(
    value: TemplateVar,
    idx: usize,
    token_stream: &TokenStream,
    output: &mut String,
    chars: Vec<char>,
) -> Result<Vec<char>, TemplateParsingError> {
    println!("{value:?}");

    if token_stream.next_is_html() {
        output.push_str(&value.string());
    } else {
        return Err(TemplateParsingError::err(
            format!(
                "If a value is the leading token in a template block, {}",
                "it is expected that is the only token in the block."
            ),
            idx,
            chars,
        ));
    }

    return Ok(chars);
}

fn resolve_block(
    token_stream: &mut TokenStream,
    output: &mut String,
    mut chars: Vec<char>,
) -> Result<Vec<char>, TemplateParsingError> {
    if let Some((token, idx)) = token_stream.get() {
        match token {
            TemplateToken::If => todo!(),
            TemplateToken::End => todo!(),
            TemplateToken::Else => todo!(),
            TemplateToken::For => todo!(),
            TemplateToken::In => todo!(),
            TemplateToken::Block => todo!(),
            TemplateToken::Not => todo!(),
            TemplateToken::Value(template_var) => {
                chars = resolve_value_led_block(
                    template_var.clone(),
                    *idx,
                    token_stream,
                    output,
                    chars,
                )?;
            }
            TemplateToken::ValueRef(template_var) => {
                chars = resolve_value_led_block(
                    (*template_var).clone(),
                    *idx,
                    token_stream,
                    output,
                    chars,
                )?;
            }
            _ => {
                return Err(TemplateParsingError::err(
                    format!("Invalid leading token \"{:?}\".", token),
                    *idx,
                    chars,
                ))
            }
        }
    }

    return Ok(chars);
}

pub fn parse(initial: String, data: &TemplateVar) -> Result<String, TemplateParsingError> {
    let (token_list, mut chars) = lex(initial, data)?;
    let mut token_stream = TokenStream::new(token_list);

    let mut output = String::new();
    while let Some((TemplateToken::Html(html), _idx)) = token_stream.get() {
        output.push_str(html);
        chars = resolve_block(&mut token_stream, &mut output, chars)?;
    }

    return Ok(output);
}
