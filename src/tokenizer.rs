use std::str::Chars;

use crate::token::{Token, SourceLocation};

type TokResult = (Result<Token, String>, SourceLocation);
pub struct Tokenizer<'a>
{
    source : &'a str,
    chars  : Chars<'a>,
}
impl<'a> Tokenizer<'a>
{
    pub fn new(source: &'a str) -> Self 
    { 
        Self { source, chars: source.chars() } 
    }

    pub fn next_token(&mut self) -> TokResult
    {
        self.skip_whitespaces();
        let offset = self.compute_offset();
        if let Some(c) = self.chars.next()
        {
            let mut len = 1;

            use Token::*;
            let tok = match c
            {
                // Single char token
                '(' => LeftParen,
                ')' => RightParen,
                '{' => LeftBrace,
                '}' => RightBrace,
                ',' => Comma,
                '.' => Dot,
                '-' => Minus,
                '+' => Plus,
                ';' => Semicolon,
                '*' => Star,

                // One or two chars token
                '!' => if self.consume_on('=') { len = 2; BangEqual }    else { Bang },
                '=' => if self.consume_on('=') { len = 2; EqualEqual }   else { Equal },
                '<' => if self.consume_on('=') { len = 2; LessEqual }    else { Less },
                '>' => if self.consume_on('=') { len = 2; GreaterEqual } else { Greater },

                '/' => if self.consume_on('/') 
                { // Line comment
                    self.consume_while(|c| c != '\n');
                    return self.next_token();  // I'm not fully satisfied with this way of handling it but at least it is simple
                }
                else { Slash },

                '"' =>
                {
                    self.consume_while(|c| c != '"');
                    if self.chars.next() != Some('"') 
                    { // self.chars.next() was None denoting the end of the source
                        return (Err(format!("Unterminated string")), SourceLocation {offset, len: self.compute_offset() - offset}); 
                    }

                    let offset_end = self.compute_offset();
                    len = offset_end - offset;
                    String(self.source[(offset + 1)..(offset_end - 1)].to_string())
                },

                c if Self::is_digit(c) =>
                {
                    self.consume_while(Self::is_digit);
                    if matches!(self.peek_first_two(), 
                        (Some('.'), Some(c)) if Self::is_digit(c))
                    {
                        self.chars.next(); // Consume '.'
                        self.consume_while(Self::is_digit);
                    }

                    let offset_end = self.compute_offset();
                    len = offset_end - offset;
                    let number_res = self.source[(offset)..(offset_end)].parse::<f64>();
                    match number_res
                    {
                        Ok(x)  => Number(x),
                        Err(e) => return (Err(format!("Parsing error: {e:?}")), SourceLocation {offset, len}),
                    }
                },

                _ => return (Err(format!("Unexpected character: {c:?}")), SourceLocation {offset, len}),
            };
            (Ok(tok), SourceLocation {offset, len})
        }
        else { (Ok(Token::Eof), SourceLocation {offset, len: 0}) }
    }

    fn peek_first(&self) -> Option<char> { self.chars.clone().next() }
    fn peek_first_two(&self) -> (Option<char>, Option<char>) { let mut peek = self.chars.clone(); (peek.next(), peek.next()) }
    fn consume_while(&mut self, mut predicate: impl FnMut(char) -> bool)
    {
        while self.peek_first().map_or(false, |c| predicate(c))
        {
            self.chars.next();
        }
    }
    fn skip_whitespaces(&mut self) { self.consume_while(char::is_whitespace) }
    fn consume_if(&mut self, predicate: impl FnOnce(char) -> bool) -> bool
    {
        let consumed = self.peek_first().map_or(false, |c| predicate(c));
        if consumed { self.chars.next(); }
        consumed
    }
    fn consume_on(&mut self, expected: char) -> bool { self.consume_if(|c| c == expected) }
    fn compute_offset(&self) -> usize { self.source.len() - self.chars.as_str().len() }

    fn is_digit(c: char) -> bool { c.is_ascii_digit() }
}

impl<'a> Iterator for Tokenizer<'a>
{
    type Item = TokResult;
    fn next(&mut self) -> Option<Self::Item> 
    {
        match self.next_token() 
        {
            (Ok(Token::Eof), _) => None,
            tok_res             => Some(tok_res),
        }
    }
}
