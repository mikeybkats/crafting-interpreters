#![allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq)]
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

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Num(f64),
    Bool(bool),
    Nil,
}

impl Literal {
    /// # is_truthy
    /// returns whether the literal value is true or false in lox
    pub fn is_truthy(&self) -> bool {
        match self {
            Literal::Bool(b) => *b,
            Literal::Nil => false,
            _ => true, // All other values (Str, Num) are truthy
        }
    }

    /// # instance_of
    /// returns true if the type being checked  matches the type of self
    pub fn instance_of(&self, type_check: &Literal) -> bool {
        match (self, type_check) {
            (Literal::Str(_), Literal::Str(_)) => true,
            (Literal::Num(_), Literal::Num(_)) => true,
            (Literal::Bool(_), Literal::Bool(_)) => true,
            (Literal::Nil, Literal::Nil) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Literal>,
        line: usize,
    ) -> Self {
        // Using a literal enum with a processed literal would allow a simpler object, but
        // what is done when there is a none value?
        // process ahead of time or before use?
        // let literal = match literal {
        //     Some(Literal::Str(s)) => Literal::Str(s),
        //     Some(Literal::Num(n)) => Literal::Num(n),
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
        format!("{:?} {} {:?}", self.token_type, self.lexeme, self.literal)
    }
}
