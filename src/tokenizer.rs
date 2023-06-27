use std::{str::Chars, collections::HashMap};

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
        if let Some(c) = self.consume()
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
                    if self.consume() != Some('"') 
                    { // self.consume() was None denoting the end of the source
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
                        self.consume(); // Consume '.'
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

                c if Self::is_alpha(c) =>
                {
                    self.consume_while(Self::is_alphanum);

                    let offset_end = self.compute_offset();
                    len = offset_end - offset;

                    Self::ident_or_keyword_from_str(&self.source[(offset)..(offset_end)])
                },

                _ => return (Err(format!("Unexpected character: {c:?}")), SourceLocation {offset, len}),
            };
            (Ok(tok), SourceLocation {offset, len})
        }
        else { (Ok(Token::Eof), SourceLocation {offset, len: 0}) }
    }

    fn peek_first(&self) -> Option<char> { self.chars.clone().next() }
    fn peek_first_two(&self) -> (Option<char>, Option<char>) { let mut peek = self.chars.clone(); (peek.next(), peek.next()) }
    fn consume(&mut self) -> Option<char> { self.chars.next() }
    fn consume_while(&mut self, mut predicate: impl FnMut(char) -> bool)
    {
        while self.peek_first().map_or(false, |c| predicate(c))
        {
            self.consume();
        }
    }
    fn skip_whitespaces(&mut self) { self.consume_while(char::is_whitespace) }
    fn consume_if(&mut self, predicate: impl FnOnce(char) -> bool) -> bool
    {
        let consumed = self.peek_first().map_or(false, |c| predicate(c));
        if consumed { self.consume(); }
        consumed
    }
    fn consume_on(&mut self, expected: char) -> bool { self.consume_if(|c| c == expected) }
    fn compute_offset(&self) -> usize { self.source.len() - self.chars.as_str().len() }

    fn is_digit(c: char) -> bool { c.is_ascii_digit() }
    fn is_alpha(c: char) -> bool { c == '_' || c.is_alphabetic() }
    fn is_alphanum(c: char) -> bool { Self::is_alpha(c) || Self::is_digit(c) }

    fn ident_or_keyword_from_str(name: &str) -> Token
    { 
        // O(n) implementation. Could be speeded up by using a static map but that would require static initialization of non-const value which in turn requires unsafe code.
        // There are some crates that provide safe wrappers but in order to keep this project not using external dependency, I currently let it be like that.
        use Token::*;
        match name
        {
            "and"    => And,
            "class"  => Class,
            "else"   => Else,
            "false"  => False,
            "fun"    => Fun,
            "for"    => For,
            "if"     => If,
            "nil"    => Nil,
            "or"     => Or,
            "print"  => Print,
            "return" => Return,
            "super"  => Super,
            "this"   => This,
            "true"   => True,
            "var"    => Var,
            "while"  => While,

            _ => Identifier(name.to_string())
        }
    }
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
