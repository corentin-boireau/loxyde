#[derive(Debug)]
pub enum Token 
{
    // Single-character tokens.
    LeftParen,    // (
    RightParen,   // )
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,
    Dot,          // .
    Minus,        // -
    Plus,         // +
    Semicolon,    // ;
    Slash,        // /
    Star,         // *

    // One or two character tokens.
    Bang,          // !
    BangEqual,     // !=
    Equal,         // =
    EqualEqual,    // ==
    Greater,       // >
    GreaterEqual,  // >=
    Less,          // <
    LessEqual,     // <=

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, While,

    Eof
}

#[derive(Debug)]
pub struct SourceLocation
{
    pub offset : usize,
    pub len    : usize,
}