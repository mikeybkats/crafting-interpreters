#![allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum TokenType {
    // Single-character tokens.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    Eof,
}

#[derive(Debug)]
pub enum StringOrNumber {
    Str(String),
    Num(f64),
}

// Another way to do this:
// enum Literal {
//     Str(String),
//     Num(i32),
//     None,
// }

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<StringOrNumber>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<StringOrNumber>,
        line: usize,
    ) -> Self {
        // Using a literal enum with a processed literal would allow a simpler object, but
        // what is done when there is a none value?
        // process ahead of time or before use?
        // let literal = match literal {
        //     Some(StringOrNumber::Str(s)) => Literal::Str(s),
        //     Some(StringOrNumber::Num(n)) => Literal::Num(n),
        //     None => Literal::None,
        // };

        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    fn to_string(&self) -> String {
        format!(
            "{:?} {} {:?}",
            self.token_type,
            self.lexeme,
            self.literal.as_ref().unwrap()
        )
    }
}
